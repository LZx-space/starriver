# StarRiver

## Intro

StarRiver is a practice project for learning Rust, with a focus on Domain-Driven Design (DDD).

- Rust-lang practice: Hands-on use of Rust's safety, concurrency, and performance features.
- DDD: Structuring the project using DDD concepts to build a scalable application.

## Installation

To get started:

1. Install Rust and Cargo from [rust-lang.org](https://www.rust-lang.org/tools/install).
2. Clone the repository: `git clone <url>`.
3. Navigate to the project: `cd starriver`.
4. Build the project: `cargo build`.
5. Run tests: `cargo test`.
6. Initialize DB: `cd script && wsl_local_db_pg_init.bat`

## Architecture

Hexagonal + Clean Architecture, DDD tactical patterns.

### Layered Structure

adapter → application → domain

| Layer | Ability | Depends on |
|---|---|---|
| **Domain**      | Aggregates, value objects, domain services, repository traits | Nothing |
| **Application** | Use cases, DTOs, port interfaces, error mapping               | Domain |
| **Adapter**     | HTTP handlers, Domain & Application IDP Traits Impl           | Application + Domain |

All ports are traits defined in domain or application. All implementations live in adapter. Dependency only points inward.

### Shared Kernel

| Crate | What it holds |
|---|---|
| `shared-base`      | `Patterns`, IO traits... — zero external deps |
| `shared-framework` | `ApiError`, `AuthenticatedUser`, middleware, extractors... |

### Bounded Contexts

| Context | crates |
|---|---|
| Identity | `starriver-identity/*` — users, authentication, security events |
| Blogging | `starriver-blogging/*` — posts, categories, attachments |

Contexts isolated; cross-context shared only through `shared-base`.

## Run

Start the server:

```sh
cargo run
```

The logging level is controlled via `RUST_LOG` (defaults to `info` if unset):

```sh
# Console output (default)
cargo run

# Debug-level console output
RUST_LOG=debug cargo run
```

Output destination is configured in `config.toml` under `[file_logging]`:
- **`file_enabled = true`** — JSON lines written to daily-rotated files in `file_directory`
- **`file_enabled = false`** (default) — colored text printed to stdout
