# Arcadia Framework

Website: <https://frameworks.ilnazik.dev/arcadia><br>
Repository: <https://github.com/azomura/arcadia-framework><br>
License: [MIT](LICENSE)

## English

Arcadia Framework is a modular Rust backend framework inspired by NestJS.

The workspace contains `arcadia-framework` (runtime, HTTP, WebSockets,
transports, OpenAPI, testing, and logging) and `arcadia-macros` (procedural
macros for modules, controllers, routes, and injectable providers).

### Features

- modules, imports, exports, providers, async providers, and dependency injection;
- controllers, request context, lifecycle hooks, middleware, guards, pipes,
  interceptors, filters, and custom decorators;
- WebSockets, microservice transports, GraphQL primitives, and OpenAPI;
- customizable colored console and file logging;
- testing modules and integration testing utilities.

### Install from crates.io

```toml
[dependencies]
arcadia-framework = "0.1.7"
arcadia-macros = "0.1.7"
```

### Install a Git release

```toml
[dependencies]
arcadia-framework = { git = "https://github.com/azomura/arcadia-framework.git", tag = "v0.1.7" }
arcadia-macros = { git = "https://github.com/azomura/arcadia-framework.git", tag = "v0.1.7" }
```

### Install the development branch

```toml
[dependencies]
arcadia-framework = { git = "https://github.com/azomura/arcadia-framework.git", branch = "dev" }
arcadia-macros = { git = "https://github.com/azomura/arcadia-framework.git", branch = "dev" }
```

### Clone and build locally

```bash
git clone https://github.com/azomura/arcadia-framework.git
cd arcadia-framework
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

For a local application, use path dependencies:

```toml
[dependencies]
arcadia-framework = { path = "../arcadia-framework/runtime" }
arcadia-macros = { path = "../arcadia-framework/macros" }
```

## Русский

Arcadia Framework — модульный backend-фреймворк на Rust, вдохновлённый NestJS.

В workspace входят `arcadia-framework` (runtime, HTTP, WebSockets, transports,
OpenAPI, testing и логирование) и `arcadia-macros` (procedural macros для
модулей, controllers, маршрутов и injectable providers).

### Возможности

- modules, imports, exports, providers, async providers и dependency injection;
- controllers, request context, lifecycle hooks, middleware, guards, pipes,
  interceptors, filters и custom decorators;
- WebSockets, microservice transports, GraphQL primitives и OpenAPI;
- настраиваемый цветной логгер для консоли и файлов;
- testing modules и интеграционные тесты.

### Установка из crates.io

```toml
[dependencies]
arcadia-framework = "0.1.7"
arcadia-macros = "0.1.7"
```

### Установка Git-релиза

```toml
[dependencies]
arcadia-framework = { git = "https://github.com/azomura/arcadia-framework.git", tag = "v0.1.7" }
arcadia-macros = { git = "https://github.com/azomura/arcadia-framework.git", tag = "v0.1.7" }
```

### Установка development-ветки

```toml
[dependencies]
arcadia-framework = { git = "https://github.com/azomura/arcadia-framework.git", branch = "dev" }
arcadia-macros = { git = "https://github.com/azomura/arcadia-framework.git", branch = "dev" }
```

### Лицензия

Проект распространяется по лицензии [MIT](LICENSE). Полный текст лицензии
находится в файле `LICENSE`.
