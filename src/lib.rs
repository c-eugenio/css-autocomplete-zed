use zed_extension_api::{self as zed, LanguageServerId, Result};

struct CssClassAutocomplete;

impl zed::Extension for CssClassAutocomplete {
    fn new() -> Self {
        CssClassAutocomplete
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // Try PATH first (works when ~/.cargo/bin is in Zed's PATH)
        let binary = "css-class-lsp";
        let path = worktree.which(binary).unwrap_or_else(|| {
            // Fallback: macOS apps launched from Dock don't inherit shell PATH,
            // so ~/.cargo/bin may not be visible. Try the common install location.
            let home = std::env::var("HOME").unwrap_or_default();
            format!("{home}/.cargo/bin/{binary}")
        });

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: vec![],
        })
    }
}

zed::register_extension!(CssClassAutocomplete);
