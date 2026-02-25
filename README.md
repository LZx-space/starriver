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

## DDD Overview

This project follows DDD principles:

- **Adapter Layer**: Web controllers (REST API endpoints), Authentication and authorization, etc.
- **Application Layer**: Handles use cases and orchestrates domain logic, **`Provides implementations for repositories`**.
- **Domain Layer**: Contains entities, value objects, aggregates, and domain services.
- **Infrastructure Layer**: Common model and services, Security implementations, Shared models and error types, etc.

This structure promotes separation of concerns and testability.

## Contributing

We welcome contributions!

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/new-feature`).
3. Commit your changes (`git commit -am 'Add new feature'`).
4. Push to the branch (`git push origin feature/new-feature`).
5. Open a Pull Request.

Please ensure your code adheres to Rust best practices and passes all tests.
