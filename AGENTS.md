# Arcadia Framework

Arcadia is a Rust backend framework inspired by NestJS. The current temporary
implementation is a single crate; its public architecture is organized around
modules, controllers, services, constructor injection, and application bootstrap.

## Development rules

- Keep HTTP concerns in controllers and business logic in services.
- Prefer structs with private fields, constructors, composition, and traits.
- Add functional modules under `src/modules/<name>/`.
- Format with `cargo fmt --all` and validate with `cargo test --workspace`.
- Use Conventional Commits: `type(scope): description`.
- A commit may contain no more than five files.
- Commit messages are one concise lowercase English line, under 100 characters,
  with no trailing period.

## Repository layout

- `src/application.rs`: application bootstrap and dependency container.
- `src/routing.rs`: module and controller contracts.
- `src/http.rs`: request/response abstractions.

