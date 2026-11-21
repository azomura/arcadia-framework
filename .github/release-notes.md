# Arcadia Framework v{{VERSION}}

## English

Arcadia Framework is a modular backend framework for Rust inspired by the
architecture and developer experience of NestJS.

### Included in this release

- modular application composition and dependency injection;
- controllers, providers, lifecycle hooks, guards, pipes, interceptors, and filters;
- HTTP request context, testing helpers, WebSockets, and transport adapters;
- OpenAPI metadata and explicitly enabled Swagger UI;
- customizable tracing logger with colored console output and file logging;
- procedural macros for modules, controllers, routes, and injectable providers.
- reusable `BaseConfig`, `ProjectPaths`, and environment-aware configuration;
- `.env.<environment>` → `.env` precedence with critical startup errors;
- public validators for booleans, choices, email, IP, ports, ranges, lists,
  optional text, and URLs;
- logger initialization before configuration loading and tests for the new APIs.

Full changelog: https://github.com/azomura/arcadia-framework/blob/main/CHANGELOG.md

## Русская версия

Arcadia Framework — модульный backend-фреймворк на Rust, вдохновлённый
архитектурой и подходом NestJS к разработке приложений.

### Что входит в этот релиз

- модульная композиция приложения и dependency injection;
- controllers, providers, lifecycle hooks, guards, pipes, interceptors и filters;
- HTTP-контекст запросов, testing helpers, WebSockets и транспортные адаптеры;
- OpenAPI metadata и Swagger UI, подключаемый явно;
- настраиваемый tracing-логгер с цветным выводом и записью в файлы;
- procedural macros для модулей, controllers, маршрутов и injectable providers.
- переиспользуемые `BaseConfig`, `ProjectPaths` и конфигурация с учётом окружения;
- порядок `.env.<окружение>` → `.env` с критической ошибкой при запуске;
- публичные validators для boolean, choices, email, IP, портов, диапазонов,
  списков, optional text и URL;
- инициализация логгера до загрузки конфигурации и тесты новых API.

Полный changelog: https://github.com/azomura/arcadia-framework/blob/main/CHANGELOG.md
