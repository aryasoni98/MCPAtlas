# Project structure

This page describes the repository layout and key files, so you can find where the server, site, and docs live.

## Project root

The repo root is the workspace root. From here you run `cargo` for the Rust crates and `npm` (in `site/`) for the landing and in-app docs.

## Key directories

| Directory | Purpose |
|-----------|---------|
| `crates/` | Rust workspace crates: core server, data pipeline, search, graph, plugins, CLI. |
| `site/` | Landing page and in-app docs (Vite + React). Build with `npm ci && npm run build` in `site/`. |
| `book/` | Built mdBook output (static HTML docs). Optional; in-app docs live in `site/src/content/docs/`. |
| `deploy/` | Docker, Helm, and deployment manifests (e.g. `deploy/helm/`, `deploy/docker/`). |
| `docs/` | Design and task docs (e.g. Blueprint, release reports), not the user-facing doc content. |
| `data/` | Optional local landscape file (e.g. `landscape.yml`) for offline or custom data. |
| `scripts/` | Release, verification, and automation scripts. |

## Key files

| File | Purpose |
|------|---------|
| `Cargo.toml` | Workspace definition: members (core, data, search, graph, vector, plugins, cli), shared deps, release profile. |
| `crates/mcp-atlas-core/Cargo.toml` | Main server crate; binary name `mcp-atlas`. |
| `crates/mcp-atlas-cli/Cargo.toml` | CLI crate; binary name `mcp-atlas-cli` (sync, validate). |
| `site/package.json` | Site scripts: `dev`, `build`, `preview`, `lint`. Entry for Vite dev server and production build. |
| `site/vite.config.ts` | Vite config; base URL and doc routing. |
| `data/landscape.yml` | Optional local copy of CNCF landscape; use with `--landscape-file data/landscape.yml`. |
| `.env` | Never committed; use for `GITHUB_TOKEN` or other secrets when developing. |

## Binaries and outputs

After `cargo build --release`:

- **`target/release/mcp-atlas`**  MCP server (STDIO or HTTP).
- **`target/release/mcp-atlas-cli`**  CLI for `sync` and `validate`.

After `npm run build` in `site/`:

- **`site/dist/`**  Static site (landing + docs) for deployment (e.g. GitHub Pages, Vercel).

## Docs: site vs book

- **In-app docs** (what you see at `/docs` on the site) are Markdown files in **`site/src/content/docs/`**. Edits there update the live docs after rebuilding the site.
- **`book/`** is the output of mdBook (if used). The single source of truth for user-facing docs in this project is `site/src/content/docs/`. The table on [Introduction](/docs/introduction) links to each doc.

## Command line entrypoints

- **Run server (STDIO):** `cargo run -p mcp-atlas-core -- --transport stdio --skip-github`
- **Run server (HTTP):** `cargo run -p mcp-atlas-core -- --transport sse --port 3000`
- **Run site dev server:** `cd site && npm run dev`
- **Build site:** `cd site && npm run build`
- **Sync landscape:** `cargo run -p mcp-atlas-cli -- sync`
- **Validate landscape:** `cargo run -p mcp-atlas-cli -- validate data/landscape.yml`
