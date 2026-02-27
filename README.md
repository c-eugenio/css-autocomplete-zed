# CSS Class Autocomplete — Zed Extension

A [Zed](https://zed.dev) extension that provides CSS class name autocompletion in HTML, PHP, JS and TSX files.

- Suggests classes from **Bootstrap 5.3** out of the box (no local CSS file needed)
- Also scans your workspace for any `.css`, `.scss`, `.sass`, or `.less` files and suggests those classes too
- Completions only appear inside `class="..."` or `className="..."` attributes — not everywhere

---

## Requirements

- [Zed](https://zed.dev) editor
- [Rust](https://rustup.rs) — to build the LSP binary

---

## Installation

### 1. Build and install the LSP binary

```sh
git clone https://github.com/username/css-autocomplete-zed
cd css-autocomplete-zed/lsp
cargo install --path .
```

This places the `css-class-lsp` binary in `~/.cargo/bin/`.

### 2. Load the extension in Zed

1. Open Zed
2. Open the Command Palette (`Cmd+Shift+P`)
3. Run **Extensions: Install Dev Extension**
4. Select the root folder of this repository

Zed will compile the WASM extension automatically.

### 3. Verify it's running

Open any `.php` or `.html` file, click inside a `class="..."` attribute and start typing. The autocomplete popup should appear.

If it does not appear, restart the language server: `Cmd+Shift+P` → **language server: restart**.

---

## How it works

The extension is split into two parts:

```
css-autocomplete-zed/
├── extension.toml        # Zed extension manifest
├── Cargo.toml            # WASM extension crate (cdylib)
├── src/
│   └── lib.rs            # Finds css-class-lsp binary and tells Zed how to launch it
└── lsp/
    ├── Cargo.toml
    └── src/
        ├── main.rs       # tower-lsp server
        ├── scanner.rs    # Walks workspace, extracts class names from CSS files
        ├── context.rs    # Detects when cursor is inside class="..." attribute
        └── frameworks.rs # Bundled Bootstrap 5.3 class list
```

**WASM extension** (`src/lib.rs`) — a thin shim that implements the `zed_extension_api::Extension` trait. Its only job is to locate the `css-class-lsp` binary and return the command to Zed.

**Native LSP binary** (`lsp/`) — a standard [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) server built with `tower-lsp`. On startup it scans the workspace for CSS files and extracts class selectors. On every completion request it checks whether the cursor is inside a `class="..."` attribute, extracts the current word prefix, and filters the combined class list (Bootstrap built-ins + scanned classes) to return matches.

---

## Updating after LSP changes

After any change to the `lsp/` source:

```sh
cd lsp && cargo install --path .
```

Then in Zed: `Cmd+Shift+P` → **language server: restart**.

---

## Credits & References

| Project                                                         | Use                                                                                                                                                  |
| --------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Zed](https://zed.dev)                                          | Editor this extension targets                                                                                                                        |
| [zed_extension_api](https://crates.io/crates/zed-extension-api) | Rust crate used to implement the WASM extension shim                                                                                                 |
| [zed-extensions](https://github.com/zed-industries/extensions)  | Reference implementations — the `phpcs` extension's `extension.toml` structure was used as a reference for the correct `language_ids` mapping format |
| [tower-lsp](https://github.com/ebkalderon/tower-lsp)            | LSP server framework used by the native binary                                                                                                       |
| [Bootstrap 5.3](https://getbootstrap.com)                       | The bundled class list is extracted from Bootstrap 5.3.3 (`bootstrap.min.css` via jsDelivr CDN). Bootstrap is MIT licensed.                          |
| [walkdir](https://github.com/BurntSushi/walkdir)                | Used to walk the workspace directory tree when scanning for CSS files                                                                                |

---

## Author

César Eugénio — [c-eugenio](https://github.com/c-eugenio)

---

## License

[MIT](LICENSE)
