# Navi

> Local code intelligence for multi-repository codebases.

Navi indexes one or more repositories and builds a local, structured model of
code, relationships, and Git history.

## What is Navi?

Navi turns a codebase into a queryable graph of:

- Files and symbols
- Calls and references
- Imports and dependencies
- API and service relationships
- Cross-repository dependencies
- Git history and changes

The goal is to make it easier to understand how a system is structured and
how changes propagate through it.

## Key Capabilities

- 🔍 Symbol and code search
- 🕸️ Code dependency graph
- 🔗 Cross-repository relationships
- 💥 Change impact analysis
- 🌳 Git-aware code history
- ⚡ Incremental indexing
- 🧠 Optional local semantic search / LLM
- 🔒 Fully local operation

## Example

Given:

    frontend
        │
        │ HTTP
        ▼
    backend
        │
        │ gRPC
        ▼
    identity

Navi can represent this as:

    frontend::UserClient
            │
        HTTP_CALL
            ▼
    backend::UserController
            │
          CALLS
            ▼
    backend::UserService
            │
        GRPC_CALL
            ▼
    identity::UserService

And answer queries such as:

    navi callers UserService::update
    navi callees UserService::update
    navi graph UserService::update
    navi impact UserService::update

## Multi-Repository

Navi treats multiple repositories as a workspace:

    workspace/
    ├── frontend/
    ├── backend/
    ├── payments/
    ├── identity/
    └── shared/

Repositories remain independently represented while relationships between
them are included in the workspace graph.

## Quick Start

    git clone <repository>
    cd navi

    cargo build --release

    navi index /path/to/workspace

## Architecture

    Repositories
          │
          ▼
       Indexer
          │
       ┌──┴─────────────┐
       ▼                ▼
    Parser             Git
       │                │
       └───────┬────────┘
               ▼
          Navi Model
               │
       ┌───────┼────────┐
       ▼       ▼        ▼
     Graph   Search   History
       │       │        │
       └───────┼────────┘
               ▼
             CLI

See [`docs/`](docs/) for architecture and implementation details.

## Tech Stack

- Rust
- Tree-sitter
- SQLite
- Tantivy
- Git
- Local embedding models

## Roadmap

- [ ] Rust parsing and symbol indexing
- [ ] Code relationship graph
- [ ] Incremental indexing
- [ ] Impact analysis
- [ ] Git integration
- [ ] Multi-repository analysis
- [ ] Semantic search
- [ ] Graph visualization
- [ ] Local LLM integration

## Development

    cargo test
    cargo fmt
    cargo clippy

## License

TBD
