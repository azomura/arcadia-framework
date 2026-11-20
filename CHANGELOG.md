# Changelog

All notable Arcadia Framework changes are documented here.

## 0.1.8

### English

- expanded integration coverage for the container, scoped tokens, lifecycle,
  application bootstrap, and listen entrypoint;
- covered configuration edge cases, invalid environment files, transport
  message parsing, GraphQL defaults, and WebSocket room behavior;
- covered all module registration groups and controller OpenAPI metadata in
  procedural macro integration tests;
- verified formatting, workspace tests, Clippy, and LLVM source coverage.

### Русский

- расширено интеграционное покрытие контейнера, scoped-токенов, lifecycle,
  bootstrap приложения и входа `listen`;
- добавлены проверки edge cases конфигурации, некорректных env-файлов,
  transport-сообщений, GraphQL defaults и работы WebSocket-комнат;
- покрыты все группы регистрации модулей и OpenAPI metadata контроллеров в
  интеграционных тестах procedural macros;
- проверены форматирование, workspace-тесты, Clippy и LLVM source coverage.

## 0.1.7

### English

- split validation, release, Cargo, and GHCR workflows;
- automatic synchronized versioning for both crates and README installation examples;
- cached Cargo dependencies and build artifacts in CI;
- bilingual GitHub release notes and automated crates.io/GHCR publishing;
- modular runtime with dependency injection, HTTP, WebSockets, transports,
  OpenAPI, testing helpers, and customizable logging.

### Русский

- разделены workflow для validation, release, Cargo и GHCR;
- автоматическая синхронизация версий обоих crates и примеров установки в README;
- кеширование Cargo-зависимостей и артефактов сборки в CI;
- двуязычные GitHub Release notes и автоматическая публикация в crates.io/GHCR;
- модульный runtime с dependency injection, HTTP, WebSockets, transports,
  OpenAPI, testing helpers и настраиваемым логированием.

## 0.1.4

### English

- introduced reusable release orchestration with automatic tags and GitHub Releases;
- added Cargo and GHCR package publication workflows.

### Русский

- добавлена reusable release orchestration с автоматическими tag и GitHub Releases;
- добавлены workflow публикации Cargo-пакетов и GHCR-образа.

## Links

- Website: <https://frameworks.ilnazik.dev/arcadia>
- Repository: <https://github.com/azomura/arcadia-framework>
- crates.io runtime: <https://crates.io/crates/arcadia-framework>
- crates.io macros: <https://crates.io/crates/arcadia-macros>
