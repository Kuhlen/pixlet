# Pixlet - Image Converter

A client-side image converter web app built with [Leptos](https://github.com/leptos-rs/leptos) and [Trunk](https://github.com/trunk-rs/trunk). All image processing runs entirely in the browser via WebAssembly - no server required.

## Features

- Convert images between **JPG, PNG, WebP, GIF, BMP, TIFF**
- Adjustable quality for JPEG output
- Drag & drop or click to upload
- Image preview before conversion
- Automatic file download after conversion
- Fully client-side - your images never leave your browser

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- `wasm32-unknown-unknown` target
- [Trunk](https://trunkrs.dev/) - downloads the Tailwind CLI itself, no Node.js needed

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk
```

## Development

```sh
trunk serve --port 3000 --open
```

Opens the app at `http://localhost:3000`.

## Test and lint

```sh
cargo test                                                    # core rules
cargo clippy --target wasm32-unknown-unknown --all-targets    # wasm lint
```

## Build for Production

```sh
trunk build --release
```

Output files are in the `dist/` folder, ready to be served by any static file host.

## Project Structure

Follows the shared Leptos rules in `../CLAUDE.md`, CSR variant.

```
style/tailwind.css      Tailwind v4 entry, @source points at src/
src/
  main.rs               mount_to_body(App)
  app.rs                <App/>, route table, path consts. Only place URLs are written
  error.rs              AppError, one error type for the whole app
  core/                 pure Rust: formats, decode/encode, stage, filename rules. Unit tested
  api/                  browser I/O: read File, object URLs, download
  pages/                one folder or file per route, owns state and calls core/api
  components/layout/    AppShell (ParentRoute + Outlet), Navbar, Footer
  components/ui/        Card, Badge, Button, Icon, FeatureCard. Data only via props
```

## Tech Stack

- **Leptos 0.8** (CSR) - Rust reactive web framework
- **image crate** - Image decoding/encoding compiled to WASM
- **Tailwind CSS 4** - Styling
- **Trunk** - WASM build tool & dev server
