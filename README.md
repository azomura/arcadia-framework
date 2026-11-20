# Arcadia Framework

Arcadia is a private Rust backend framework inspired by NestJS. It provides
modular application composition, constructor-based dependency injection,
provider scopes, request context, guards, interceptors, pipes, filters,
lifecycle hooks, dynamic modules, configuration, and testing helpers.

## Workspace

- `runtime` — framework runtime and public API.
- `macros` — procedural macros for modules, controllers, routes, and injection.

The `tools` directory from the legacy project is intentionally not part of this
repository. The framework runtime and macro crates are prepared for crates.io.
