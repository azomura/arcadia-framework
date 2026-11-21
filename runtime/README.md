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

### Configuration

The runtime provides `BaseConfig`, `ProjectPaths`, environment selection
(`dev`, `release`, `build`, `test`), typed `ConfigSection` loading, and public
validators. It checks `.env.<environment>` first and then `.env`; missing files
return a critical configuration error.

```rust
use arcadia_framework::config::{validate_email, BaseConfig, ConfigField};

let config = BaseConfig::from_env()?;
validate_email(&ConfigField::new("ADMIN_EMAIL", "administrator email"),
               config.optional("ADMIN_EMAIL")?.as_deref())?;
```

Custom sections implement `ConfigSection`, and custom validators can be kept
in application modules while reusing `arcadia_framework::config::validators`.

### Install from crates.io

```toml
[dependencies]
arcadia-framework = "0.1.8"
arcadia-macros = "0.1.8"
```

```bash
cargo add arcadia-framework@0.1.7
cargo add arcadia-macros@0.1.7
```

### Git release

```toml
[dependencies]
arcadia-framework = { git = "https://github.com/azomura/arcadia-framework.git", tag = "v0.1.8" }
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

### Конфигурация

Runtime предоставляет `BaseConfig`, `ProjectPaths`, выбор окружения (`dev`,
`release`, `build`, `test`), загрузку типизированных `ConfigSection` и публичные
validators. Сначала проверяется `.env.<окружение>`, затем `.env`; при отсутствии
файлов возвращается критическая ошибка конфигурации.

Собственные секции реализуются через `ConfigSection`, а свои validators можно
хранить в модулях приложения и переиспользовать функции из
`arcadia_framework::config::validators`.

### Установка из crates.io

```toml
[dependencies]
arcadia-framework = "0.1.8"
arcadia-macros = "0.1.8"
```

```bash
cargo add arcadia-framework@0.1.7
cargo add arcadia-macros@0.1.7
```

### Git-релиз

```toml
[dependencies]
arcadia-framework = { git = "https://github.com/azomura/arcadia-framework.git", tag = "v0.1.8" }
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
