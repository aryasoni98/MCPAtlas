# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Argument extraction helpers** (`tools/args.rs`): `parse_string_arg`, `parse_optional_str`, `parse_usize_arg`, `parse_optional_u64` to reduce boilerplate in tool handlers
- **Hybrid search module** (`tools/hybrid.rs`): Reciprocal Rank Fusion (RRF) extracted for reuse; `reciprocal_rank_fusion` and `RRF_K` constant
- **Per-method dispatch** (`tools/dispatch.rs`): Dedicated handlers for each MCP JSON-RPC method (`handle_initialize`, `handle_tools_call`, etc.)
- **Codebase refactoring report** (`docs/CODEBASE_REFACTORING_REPORT.md`): Analysis of unused code, duplication, modularization, and optimization opportunities
- **Landing site** (`site/`): Vite + React + TypeScript + Tailwind + Framer Motion; Hero, Problem, Solution, Features, Architecture, Use cases, Roadmap, CTA
- **2026 AI-native landing redesign**: New hero with two-column layout, MCP flow diagram (Client→Transport→Core→Tools→Storage), microterminal block with line-by-line reveal and cursor blink; shared motion system (`sectionReveal`, `staggerContainer`/`staggerItem`) with reduced-motion fallbacks; design tokens (`--accent-brand`, `--accent-glow`, `shadow-glow`); Success metrics table in Roadmap (from ROADMAP.md); scroll-linked navbar glass (stronger background after 80px)
- **In-app docs** at `/docs`: Markdown content in `site/src/content/docs/` (introduction, getting-started, configuration, architecture, deployment, roadmap, contributing, tools-reference); sidebar layout and `react-markdown` + `remark-gfm` rendering
- **Vercel deployment**: `site/vercel.json` with build command, output directory, and SPA rewrites; set Vercel root to `site` for one-click deploy
- **404.html** in site for SPA fallback on GitHub Pages
- **Release verification script** `scripts/verify-release.sh` (install, build site, serve instructions)
- **CI site job**: build and lint the Vite site on every push/PR

### Changed

- **Tools module structure**: Split `tools/mod.rs` into `args`, `completion`, `dispatch`, `hybrid`, `resources`, `schemas`; extracted tool definitions to `schemas.rs`, resource logic to `resources.rs`, completion to `completion.rs`
- **Search handlers**: Use `args::` helpers and `hybrid::reciprocal_rank_fusion`; `handle_jsonrpc` delegates to `dispatch::` handlers
- **Lock safety**: Replaced `unwrap()` on `RwLock`/`Mutex` with `expect("...")` in `server.rs` and `tools/mod.rs` for clearer panic messages
- **Workspace**: Added `mcp-atlas-graph-surrealdb` to `Cargo.toml` members
- **Error module**: `McpError` annotated with `#[allow(dead_code)]` and doc for future structured error handling
- **Hero**: Replaced floating-icons hero with MCP flow diagram + terminal block; headline "The CNCF Landscape, in your AI assistant."; gradient + grid background with optional parallax
- **Sections**: Problem, Solution, Features, Architecture, Use cases, Roadmap, CTA use shared motion variants and design tokens; Features/UseCases/Roadmap use stagger-on-scroll; Architecture includes MCPFlowDiagram and tokenized pre block
- **Roadmap**: Phase cards show "Phase N of 4"; Success metrics table (3/6/12 mo) added below phases
- **Navbar**: Scroll-based glass (opacity increase after 80px); tokenized link colors
- **Docs delivery**: Documentation is served from the Vite app at `/docs` (in-app routes; markdown in `site/src/content/docs/`)
- **Pages workflow** (`.github/workflows/pages.yml`): Builds and deploys the Vite site (landing + in-app docs)
- **Navbar**: "Docs" links to in-app `/docs`; internal links use React Router `<Link>`
- **LAUNCH_READINESS_REPORT** and **RELEASE_EXECUTION_REPORT**: Updated for in-app docs and Vercel

### Removed

- **GlowingEffect component** and **motion** dependency from site (unused; reduces bundle size)

### Fixed

### Security

---

## [0.1.0] - TBD

### Added

- MCP server for CNCF Landscape (2,400+ projects) over STDIO, SSE/HTTP, and Streamable HTTP
- Tools: `search_projects`, `get_project`, `compare_projects`, `list_categories`, `get_stats`, `find_alternatives`, `get_health_score`, `suggest_stack`, `analyze_trends`, `get_relationships`, `find_path`, `get_graph_stats`, `get_good_first_issues`, `get_migration_path`
- Prompts: `evaluate_tool`, `plan_migration`, `review_stack`, `onboard_contributor`
- Resources: `cncf://landscape/overview`, `cncf://categories/all`, `cncf://projects/{name}`, `cncf://categories/{category}`
- Full-text search (Tantivy), knowledge graph engine, local JSON cache with configurable TTL
- CLI: `sync`, `validate` for landscape data
- HTTP transport: CORS, rate limiting, `/metrics`, streamable sessions, request cancellation
- Multi-arch release binaries and container image (ghcr.io)

[Unreleased]: https://github.com/mcp-atlas/mcp-atlas/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/mcp-atlas/mcp-atlas/releases/tag/v0.1.0
