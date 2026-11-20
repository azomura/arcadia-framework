# Arcadia Framework

Arcadia is a private Rust backend framework inspired by NestJS. The repository
contains a runtime crate and a procedural-macro crate. The runtime owns HTTP,
DI, routing, configuration, lifecycle, and testing abstractions; macros provide
declarative module, controller, route, and injectable definitions.

## Architecture

- `runtime/src/application`: application factory, container, scopes, lifecycle.
- `runtime/src/config`: environment and typed configuration primitives.
- `runtime/src/http`: request execution, pipeline, guards, pipes, filters,
  interceptors, middleware, and validation.
- `runtime/src/routing`: modules, controllers, dynamic modules, and request logs.
- `runtime/src/testing`: framework testing helpers.
- `macros/src`: procedural macros and their expansion support.
- `tools` is intentionally excluded from this repository migration.

Controllers own transport concerns. Services and providers own business logic.
Dependencies are created by constructors and resolved by `Container`.

## Rules

- Keep fields private and expose behavior through constructors and methods.
- Prefer composition and traits over global state and manual registration.
- Use `cargo fmt --all -- --check`, `cargo test --workspace`, and clippy before
  completing a functional change.
- Do not use `println!` for backend logging; use `tracing`.
- Use Conventional Commits: `type(scope): description`.
- Allowed types: `feat`, `fix`, `refactor`, `docs`, `test`, `build`, `ci`,
  `chore`, `perf`, `revert`.
- Each commit contains at most five files and exactly one concise English line.
- Commit messages use lowercase imperative English, stay under 100 characters,
  and do not end with a period.
