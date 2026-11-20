# Arcadia Framework

Arcadia is a Rust web framework inspired by NestJS architecture. It provides
modules, constructor injection, provider scopes, request context, guards,
interceptors, pipes, filters, lifecycle hooks, dynamic modules and a testing module.

The workspace contains the runtime crate (`arcadia-framework`) and procedural
macros (`arcadia-macros`). The generator in `tools/generate.py` creates and wires
application modules without embedding project-specific code.

```toml
[dependencies]
arcadia-framework = { git = "https://github.com/azomura/arcadia-framework" }
arcadia-macros = { git = "https://github.com/azomura/arcadia-framework" }
```

The repository is currently intended to remain private. crates.io does not
support private packages; publication must use an authenticated private Cargo
registry configured in `.cargo/config.toml`.

