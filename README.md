# starriver

## Intro

starriver is a practice project for learning Rust, with a focus on Domain-Driven Design (DDD).

- Rust-lang practice: Hands-on use of Rust's safety, concurrency, and performance features.
- DDD: Structuring the project using DDD concepts to build a scalable application.

## Architecture

Hexagonal + Clean Architecture, DDD tactical patterns.

- Layered Structure

adapter → application → domain

| Layer           | Ability                                                       | Depends on           |
| --------------- | ------------------------------------------------------------- | -------------------- |
| **Adapter**     | HTTP handlers, IDP traits impl, framework                     | Application + Domain |
| **Application** | use cases, DTOs, port interfaces, error mapping               | Domain               |
| **Domain**      | aggregates, value objects, domain services, repository traits | Nothing              |

All ports are traits defined in domain or application. All implementations live in adapter. Dependency only points inward.

- Shared Kernel

| Crate              | What it holds                               |
| ------------------ | ------------------------------------------- |
| `shared-base`      | `Patterns`, `IO traits`... — zero framework |
| `shared-framework` | `ApiError`, `middleware`, `extractors`...   |

- Bounded Contexts

| Context  | crates                                                          |
| -------- | --------------------------------------------------------------- |
| Identity | `starriver-identity/*` — users, authentication, security events |
| Blogging | `starriver-blogging/*` — posts, categories, attachments         |

Contexts isolated; cross-context shared through `shared-base` & `shared-framework`.

## Installation

To get started:

1. Install Rust and Cargo from [rust-lang.org](https://www.rust-lang.org/tools/install).
2. Clone the repository: `git clone <url>`.
3. Navigate to the project: `cd starriver`.
4. Build the project: `cargo build`.
5. Run tests: `cargo test`.
6. Install Postgres-18 And Extension PGroonga (version 4.0.6).
7. Initialize DB: `./script/wsl_local_db_pg_init.bat`.

## Run

Start the server:

```sh
cargo run
```
