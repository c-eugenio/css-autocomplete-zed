use zed_extension_api::{
    self as zed, Architecture, DownloadedFileType, LanguageServerId, Os, Result,
};

const BINARY_NAME: &str = "css-class-lsp";

struct CssClassAutocomplete {
    cached_binary_path: Option<String>,
}

impl zed::Extension for CssClassAutocomplete {
    fn new() -> Self {
        CssClassAutocomplete {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary = self.binary_path(language_server_id, worktree)?;
        Ok(zed::Command {
            command: binary,
            args: vec![],
            env: vec![],
        })
    }
}

impl CssClassAutocomplete {
    fn binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        // 1. User's PATH (lets developers use their own build)
        if let Some(path) = worktree.which(BINARY_NAME) {
            return Ok(path);
        }

        // 2. ~/.cargo/bin (manual cargo install)
        let home = std::env::var("HOME").unwrap_or_default();
        let cargo_bin = format!("{home}/.cargo/bin/{BINARY_NAME}");
        if std::fs::metadata(&cargo_bin).map_or(false, |m| m.is_file()) {
            return Ok(cargo_bin);
        }

        // 3. Already downloaded in a previous session
        if let Some(path) = &self.cached_binary_path {
            if std::fs::metadata(path).map_or(false, |m| m.is_file()) {
                return Ok(path.clone());
            }
        }

        // 4. Download from GitHub releases
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );

        let version = env!("CARGO_PKG_VERSION");
        let (os, arch) = zed::current_platform();

        let target = format!(
            "{}-{}",
            match arch {
                Architecture::Aarch64 => "aarch64",
                Architecture::X8664 | Architecture::X86 => "x86_64",
            },
            match os {
                Os::Mac => "apple-darwin",
                Os::Linux => "unknown-linux-gnu",
                Os::Windows => "pc-windows-msvc",
            }
        );

        let url = format!(
            "https://github.com/c-eugenio/css-autocomplete-zed/releases/download/v{version}/{BINARY_NAME}-{target}.tar.gz"
        );

        // download_file extracts into a directory named after the second argument
        zed::download_file(&url, BINARY_NAME, DownloadedFileType::GzipTar)
            .map_err(|e| format!("failed to download {BINARY_NAME}: {e}"))?;

        let binary_path = format!("{BINARY_NAME}/{BINARY_NAME}");

        zed::make_file_executable(&binary_path)
            .map_err(|e| format!("failed to make {BINARY_NAME} executable: {e}"))?;

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

zed::register_extension!(CssClassAutocomplete);
