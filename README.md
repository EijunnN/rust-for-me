# Rust for Me

Interactive desktop app for learning Rust from scratch. Built entirely in Rust with Tauri v2 + Leptos 0.8.

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
![Tauri](https://img.shields.io/badge/Tauri_v2-24C8D8?style=flat&logo=tauri&logoColor=white)
![Leptos](https://img.shields.io/badge/Leptos_0.8-EF4444?style=flat)
![License](https://img.shields.io/badge/License-MIT-blue.svg)

## What is this?

A desktop application that teaches Rust interactively through theory lessons, hands-on exercises, and guided projects. Write and run Rust code directly in the app — no terminal needed.

### Key Features

- **Theory Modules** — Structured lessons covering Rust fundamentals with code examples
- **Practice Exercises** — Write code, fix bugs, and predict output with instant feedback
- **Guided Projects** — Step-by-step projects (calculator, and more coming)
- **Rust Playground** — Global code editor drawer (Ctrl+Shift+P) with CodeMirror 6, syntax highlighting, and hybrid compilation
- **Hybrid Compilation** — Uses Rust Playground API remotely; falls back to local `rustc` if offline
- **Clippy Analysis** — Run Clippy on your code directly from the playground
- **Error Explainer** — Beginner-friendly explanations for common Rust compiler errors (E0382, E0502, E0308, etc.)
- **Bilingual** — Full Spanish and English support
- **Progress Tracking** — SQLite-backed progress persistence

## Architecture

```
rust-for-me/
├── src/                    # Leptos 0.8 frontend (WASM)
│   ├── components/         # UI components
│   │   ├── editor/         # Code editor, output panel, run button
│   │   ├── layout/         # App shell, sidebar, top bar
│   │   ├── lesson/         # Theory blocks, code examples, quizzes
│   │   └── playground/     # Global playground drawer + CodeMirror
│   ├── pages/              # Route pages (dashboard, theory, practice, projects, settings)
│   ├── services/           # Tauri command wrappers (compiler, content, progress)
│   ├── i18n/               # Internationalization (ES/EN)
│   └── models/             # Data structures
├── src-tauri/              # Tauri v2 backend (native)
│   └── src/
│       ├── commands/       # Tauri commands (compiler, filesystem, progress)
│       └── db/             # SQLite database setup and migrations
├── content/                # Educational content (TOML)
│   ├── theory/             # Lesson modules
│   ├── exercises/          # Practice exercises
│   └── projects/           # Guided projects
├── assets/js/              # CodeMirror 6 bundle (esbuild)
└── index.html              # Trunk entry point
```

**Frontend**: Leptos 0.8 in CSR mode, compiled to WASM by Trunk. Tailwind CSS via CDN.

**Backend**: Tauri v2 with rusqlite for progress, tokio for async compilation, reqwest for Playground API.

**Editor**: CodeMirror 6 bundled locally with esbuild (Rust syntax + One Dark theme), exposed to WASM via `window.CMBridge`.

## Prerequisites

- **Rust** (stable, MSVC toolchain on Windows)
- **WASM target**: `rustup target add wasm32-unknown-unknown`
- **Trunk**: `cargo install trunk`
- **Tauri CLI**: `cargo install tauri-cli`
- **Node.js** (only needed to rebuild the CodeMirror bundle)

## Getting Started

```bash
# Clone
git clone https://github.com/EijunnN/rust-for-me.git
cd rust-for-me

# Run in development mode
cargo tauri dev
```

This starts Trunk (serves the Leptos frontend on port 1420) and the Tauri window.

### Build for production

```bash
cargo tauri build
```

### Rebuild CodeMirror bundle (optional)

Only needed if you modify `cm-entry.js` or update CodeMirror versions:

```bash
npm install
npx esbuild cm-entry.js --bundle --format=iife --minify --outfile=assets/js/codemirror-bundle.js
```

## Compilation Modes

The app compiles user code using a hybrid strategy:

| Priority | Method | When |
|----------|--------|------|
| 1 | Rust Playground API | Default — works without local Rust installation |
| 2 | Local `rustc` | Fallback when offline |
| 3 | Error message | Neither available — suggests installing Rust |

This means **beginners don't need Rust installed** to start learning.

## Roadmap

- [x] Phase 1 — Scaffold, routing, i18n, all pages and components
- [ ] Phase 2 — TOML content parser, CodeMirror integration, Module 1 rendering
- [ ] Phase 3 — Compilation engine, exercise validation
- [ ] Phase 4 — SQLite progress tracking, real dashboard stats
- [ ] Phase 5 — Dark mode toggle, animations, more content modules

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Frontend | Leptos 0.8 (Rust → WASM) |
| Backend | Tauri v2 |
| Database | SQLite (rusqlite) |
| Editor | CodeMirror 6 |
| Styling | Tailwind CSS |
| Bundler | Trunk (WASM) + esbuild (JS) |
| Compiler | Rust Playground API + local rustc |

## License

MIT
