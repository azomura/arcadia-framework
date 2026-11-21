# Changelog

All notable Arcadia Framework changes are documented here.

## 0.2.0

### English

- added reusable `BaseConfig`, `ProjectPaths`, and environment-aware loading;
- added `.env.<environment>` → `.env` precedence with critical startup errors;
- added public validators for booleans, choices, email, IP, ports, ranges,
  lists, optional text, and URLs;
- initialized the logger before configuration loading and covered the new APIs.

### Русский

- добавлены переиспользуемые `BaseConfig`, `ProjectPaths` и загрузка с учётом окружения;
- добавлен порядок `.env.<окружение>` → `.env` с критической ошибкой при запуске;
- добавлены публичные validators для boolean, choices, email, IP, портов,
  диапазонов, списков, optional text и URL;
- логгер инициализируется до загрузки конфигурации, новые API покрыты тестами.

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
