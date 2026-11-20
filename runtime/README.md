# arcadia-framework

Website: <https://frameworks.ilnazik.dev/arcadia><br>
Repository: <https://github.com/azomura/arcadia-framework><br>
Crates.io: <https://crates.io/crates/arcadia-framework><br>
License: [MIT](https://github.com/azomura/arcadia-framework/blob/main/LICENSE)

## English

`arcadia-framework` is the runtime crate for a modular Rust backend framework
inspired by NestJS. It provides application composition, dependency injection,
controllers, request context, lifecycle hooks, middleware, guards, pipes,
interceptors, filters, WebSockets, transports, OpenAPI, testing helpers, and a
customizable tracing logger.

### Install from crates.io

```toml
[dependencies]
arcadia-framework = "0.1.7"
arcadia-macros = "0.1.7"
```

```bash
cargo add arcadia-framework@0.1.7
cargo add arcadia-macros@0.1.7
```

### Git release

```toml
[dependencies]
arcadia-framework = { git = "https://github.com/azomura/arcadia-framework.git", tag = "v0.1.7" }
```

```bash
cargo add arcadia-framework --git https://github.com/azomura/arcadia-framework.git --tag v0.1.7
```

### Development branch

```toml
[dependencies]
arcadia-framework = { git = "https://github.com/azomura/arcadia-framework.git", branch = "dev" }
```

```bash
cargo add arcadia-framework --git https://github.com/azomura/arcadia-framework.git --branch dev
```

### Local clone

```toml
[dependencies]
arcadia-framework = { path = "../arcadia-framework/runtime" }
```

## Русский

`arcadia-framework` — runtime-crate модульного backend-фреймворка на Rust,
вдохновлённого NestJS. Он содержит композицию приложений, dependency injection,
controllers, request context, lifecycle hooks, middleware, guards, pipes,
interceptors, filters, WebSockets, transports, OpenAPI, testing helpers и
настраиваемый tracing-логгер.

### Установка из crates.io

```toml
[dependencies]
arcadia-framework = "0.1.7"
arcadia-macros = "0.1.7"
```

```bash
cargo add arcadia-framework@0.1.7
cargo add arcadia-macros@0.1.7
```

### Git-релиз

```toml
[dependencies]
arcadia-framework = { git = "https://github.com/azomura/arcadia-framework.git", tag = "v0.1.7" }
```

```bash
cargo add arcadia-framework --git https://github.com/azomura/arcadia-framework.git --tag v0.1.7
```

### Development-ветка

```toml
[dependencies]
arcadia-framework = { git = "https://github.com/azomura/arcadia-framework.git", branch = "dev" }
```

```bash
cargo add arcadia-framework --git https://github.com/azomura/arcadia-framework.git --branch dev
```

### Локальный clone

```toml
[dependencies]
arcadia-framework = { path = "../arcadia-framework/runtime" }
```

### Лицензия

Проект распространяется по лицензии [MIT](https://github.com/azomura/arcadia-framework/blob/main/LICENSE). Полный текст находится
в файле `LICENSE`.
