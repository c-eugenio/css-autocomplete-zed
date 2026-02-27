mod context;
mod frameworks;
mod scanner;

use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use url::Url;

#[derive(Clone)]
struct Backend {
    client: Client,
    file_classes: Arc<RwLock<HashMap<Url, HashSet<String>>>>,
    documents: Arc<RwLock<HashMap<Url, String>>>,
}

impl Backend {
    fn new(client: Client) -> Self {
        Backend {
            client,
            file_classes: Arc::new(RwLock::new(HashMap::new())),
            documents: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn all_classes(&self) -> BTreeSet<String> {
        let mut classes: BTreeSet<String> =
            frameworks::bootstrap5_classes().into_iter().collect();
        let map = self.file_classes.read().await;
        classes.extend(map.values().flatten().cloned());
        classes
    }

    async fn rescan_css_file(&self, path: PathBuf) {
        if let Ok(content) = tokio::fs::read_to_string(&path).await {
            if let Ok(url) = Url::from_file_path(&path) {
                let classes = scanner::extract_classes(&content);
                self.file_classes.write().await.insert(url, classes);
            }
        }
    }

    fn is_css_url(url: &Url) -> bool {
        url.path()
            .rsplit('.')
            .next()
            .map(|ext| matches!(ext, "css" | "scss" | "sass" | "less"))
            .unwrap_or(false)
    }

    /// Get document text from cache or fall back to reading from disk.
    /// This handles URI normalization mismatches (e.g. /private/... vs /Users/... on macOS).
    async fn get_document_text(&self, uri: &Url) -> Option<String> {
        if let Some(text) = self.documents.read().await.get(uri).cloned() {
            return Some(text);
        }

        eprintln!("[css-class-lsp] cache miss for {uri}, reading from disk");
        let path = uri.to_file_path().ok()?;
        let text = tokio::fs::read_to_string(&path).await.ok()?;

        // Warm the cache for next time
        self.documents
            .write()
            .await
            .insert(uri.clone(), text.clone());
        Some(text)
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        let roots: Vec<PathBuf> = {
            let mut r = Vec::new();
            if let Some(folders) = params.workspace_folders {
                for f in folders {
                    if let Ok(path) = f.uri.to_file_path() {
                        r.push(path);
                    }
                }
            }
            if r.is_empty() {
                #[allow(deprecated)]
                if let Some(uri) = params.root_uri {
                    if let Ok(path) = uri.to_file_path() {
                        r.push(path);
                    }
                }
            }
            r
        };

        eprintln!("[css-class-lsp] initialize, roots={:?}", roots);

        let file_classes = self.file_classes.clone();
        tokio::task::spawn_blocking(move || {
            for root in &roots {
                let scanned = scanner::scan_directory(root);
                eprintln!(
                    "[css-class-lsp] scanned {:?}: {} CSS file(s)",
                    root,
                    scanned.len()
                );
                file_classes.blocking_write().extend(scanned);
            }
        });

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![
                        " ".to_string(), "\"".to_string(), "'".to_string(), "-".to_string(),
                        "a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(),
                        "e".to_string(), "f".to_string(), "g".to_string(), "h".to_string(),
                        "i".to_string(), "j".to_string(), "k".to_string(), "l".to_string(),
                        "m".to_string(), "n".to_string(), "o".to_string(), "p".to_string(),
                        "q".to_string(), "r".to_string(), "s".to_string(), "t".to_string(),
                        "u".to_string(), "v".to_string(), "w".to_string(), "x".to_string(),
                        "y".to_string(), "z".to_string(),
                        "0".to_string(), "1".to_string(), "2".to_string(), "3".to_string(),
                        "4".to_string(), "5".to_string(), "6".to_string(), "7".to_string(),
                        "8".to_string(), "9".to_string(),
                    ]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "css-class-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        eprintln!("[css-class-lsp] initialized");
        let watchers = vec![FileSystemWatcher {
            glob_pattern: GlobPattern::String("**/*.{css,scss,sass,less}".to_string()),
            kind: None,
        }];
        let registration = Registration {
            id: "css-file-watcher".to_string(),
            method: "workspace/didChangeWatchedFiles".to_string(),
            register_options: Some(
                serde_json::to_value(DidChangeWatchedFilesRegistrationOptions { watchers })
                    .unwrap(),
            ),
        };
        if let Err(e) = self.client.register_capability(vec![registration]).await {
            eprintln!("[css-class-lsp] watcher registration failed: {e}");
        }
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        eprintln!("[css-class-lsp] did_open: {uri}");
        if Self::is_css_url(&uri) {
            let classes = scanner::extract_classes(&text);
            eprintln!("[css-class-lsp] CSS file opened, {} classes", classes.len());
            self.file_classes.write().await.insert(uri, classes);
        } else {
            self.documents.write().await.insert(uri, text);
        }
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(change) = params.content_changes.into_iter().last() {
            let text = change.text;
            if Self::is_css_url(&uri) {
                let classes = scanner::extract_classes(&text);
                self.file_classes.write().await.insert(uri, classes);
            } else {
                self.documents.write().await.insert(uri, text);
            }
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.documents
            .write()
            .await
            .remove(&params.text_document.uri);
    }

    async fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {
        for change in params.changes {
            match change.typ {
                FileChangeType::CREATED | FileChangeType::CHANGED => {
                    if let Ok(path) = change.uri.to_file_path() {
                        self.rescan_css_file(path).await;
                    }
                }
                FileChangeType::DELETED => {
                    self.file_classes.write().await.remove(&change.uri);
                }
                _ => {}
            }
        }
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri.clone();
        let pos = params.text_document_position.position;

        eprintln!(
            "[css-class-lsp] completion: line={} char={}",
            pos.line, pos.character
        );

        let text = match self.get_document_text(&uri).await {
            Some(t) => t,
            None => {
                eprintln!("[css-class-lsp] document not found (even on disk)");
                return Ok(None);
            }
        };

        // Normalize line endings so position math is always correct
        let text = text.replace("\r\n", "\n").replace('\r', "\n");

        if !context::is_in_class_attribute(&text, pos) {
            eprintln!("[css-class-lsp] not in class attribute");
            return Ok(None);
        }

        let prefix = context::get_word_prefix(&text, pos);
        eprintln!("[css-class-lsp] prefix={:?}", prefix);

        let all = self.all_classes().await;
        let items: Vec<CompletionItem> = all
            .iter()
            .filter(|c| c.starts_with(&prefix))
            .map(|c| CompletionItem {
                label: c.clone(),
                kind: Some(CompletionItemKind::VALUE),
                ..Default::default()
            })
            .collect();

        eprintln!("[css-class-lsp] returning {} items", items.len());
        Ok(Some(CompletionResponse::Array(items)))
    }
}

#[tokio::main]
async fn main() {
    eprintln!("[css-class-lsp] starting");
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
