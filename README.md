# Pixlet - Image Converter

A client-side image converter web app built with [Leptos](https://github.com/leptos-rs/leptos) and [Trunk](https://github.com/trunk-rs/trunk). All image processing runs entirely in the browser via WebAssembly — no server required.

## Features

- Convert images between **JPG, PNG, WebP, GIF, BMP, TIFF**
- Adjustable quality for JPEG output
- Drag & drop or click to upload
- Image preview before conversion
- Automatic file download after conversion
- Fully client-side — your images never leave your browser

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- `wasm32-unknown-unknown` target
- [Trunk](https://trunkrs.dev/)
- [Node.js](https://nodejs.org/) (for Tailwind CSS)

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk
```

## Development

```sh
trunk serve --port 3000 --open
```

Opens the app at `http://localhost:3000`.

## Build for Production

```sh
trunk build --release
```

Output files are in the `dist/` folder, ready to be served by any static file host.

## Tech Stack

- **Leptos 0.8** (CSR) — Rust reactive web framework
- **image crate** — Image decoding/encoding compiled to WASM
- **Tailwind CSS 4** — Styling
- **Trunk** — WASM build tool & dev server
