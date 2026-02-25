# Outline MCP Server

[![CI](https://github.com/nizovtsevnv/outline-mcp-rs/workflows/CI/badge.svg)](https://github.com/nizovtsevnv/outline-mcp-rs/actions/workflows/ci.yml)
[![Release Build](https://github.com/nizovtsevnv/outline-mcp-rs/workflows/Release%20Build/badge.svg)](https://github.com/nizovtsevnv/outline-mcp-rs/actions/workflows/release.yml)
[![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/nizovtsevnv/outline-mcp-rs?sort=semver)](https://github.com/nizovtsevnv/outline-mcp-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)

MCP (Model Context Protocol) сервер для взаимодействия с API Outline с фокусом на **простоту**, **производительность** и **надёжность**.

Поддерживает два транспортных режима:
- **STDIO** — однопользовательский, для прямой интеграции с MCP-клиентами (Cursor IDE, Claude Desktop и др.)
- **HTTP** — многопользовательский Streamable HTTP транспорт (спецификация MCP 2025-03-26) с аутентификацией, rate limiting, сессиями, SSE и CORS

## Быстрый старт

### 1. Получите API ключ Outline
- **Outline.com**: https://app.outline.com/settings/api-and-apps
- **Собственный хостинг**: https://your-instance.com/settings/api-and-apps

### 2. Скачайте и установите

Выберите один из способов установки:

#### Вариант 1: Скачивание готового бинарника (Рекомендуемый)
Скачайте с [GitHub Releases](https://github.com/nizovtsevnv/outline-mcp-rs/releases)

**После извлечения архива:**
- **Linux/macOS**: При необходимости сделайте исполняемым: `chmod +x outline-mcp`
- **Windows**: Поскольку релиз не подписан цифровой подписью, Windows Defender может заблокировать выполнение. Вам потребуется:
  1. Разрешить исполняемый файл через Windows Defender/антивирус
  2. Добавить папку в исключения Windows Defender, или
  3. ПКМ по файлу > Свойства > «Разблокировать», если скачано из интернета

#### Вариант 2: Установка из crates.io
```bash
cargo install outline-mcp-rs
```
*Требует Rust toolchain. Бинарник будет установлен в `~/.cargo/bin/outline-mcp`*

#### Вариант 3: Сборка из исходного кода
```bash
git clone https://github.com/nizovtsevnv/outline-mcp-rs.git
cd outline-mcp-rs
cargo build --release
# Бинарник будет в target/release/outline-mcp
```

#### Вариант 4: Nix (с воспроизводимой средой)
```bash
nix run github:nizovtsevnv/outline-mcp-rs
```

### 3. Настройте ваш AI-агент (режим STDIO)

JSON-конфигурация для Cursor IDE, Gemini CLI:
```json
{
  "mcpServers": {
    "Outline knowledge base": {
      "command": "outline-mcp",
      "env": {
        "OUTLINE_API_KEY": "ваш-api-ключ-здесь",
        "OUTLINE_API_URL": "https://app.getoutline.com/api"
      }
    }
  }
}
```

> **Примечания о путях:**
> - **cargo install**: Используйте `"outline-mcp"` (автоматически добавляется в PATH)
> - **Скачанный бинарник**: Используйте полный путь, например `"/path/to/outline-mcp"`
> - **Собранный из исходников**: Используйте `"/path/to/outline-mcp-rs/target/release/outline-mcp"`

**Требования к путям:**
- **Используйте абсолютные пути** — относительные могут работать некорректно
- **Без пробелов** в пути к исполняемому файлу (используйте подчёркивания или дефисы)
- **Только ASCII символы** — избегайте кириллицы и других не-латинских символов в путях
- **Пользователи Windows**: Используйте двойной обратный слеш `\\` в путях (например, `"C:\\tools\\outline-mcp.exe"`)

## Режим HTTP (многопользовательский)

HTTP-режим запускает Streamable HTTP транспорт, к которому могут подключаться несколько пользователей, каждый со своим API-ключом Outline. Сам сервер защищён MCP-токенами доступа.

### Архитектура

```
                    +--------------------------+
  Клиент A -------->|                          |-------> Outline API
  [X-MCP-Token]     |    MCP HTTP сервер       |  (ключ пользователя A)
  [Authorization]   |                          |
                    |  - Аутентификация         |
  Клиент B -------->|  - Rate limiting по IP    |-------> Outline API
  [X-MCP-Token]     |  - Сессии (UUID v4)       |  (ключ пользователя B)
  [Authorization]   |  - SSE стриминг           |
                    |  - CORS                   |
  Клиент C -------->|                          |-------> Outline API
                    +--------------------------+  (ключ пользователя C)
```

### Настройка

```bash
# Обязательно: задайте разрешённые MCP-токены доступа (через запятую)
export MCP_AUTH_TOKENS="токен-для-алисы,токен-для-боба"

# Опционально: URL Outline API (по умолчанию: https://app.getoutline.com/api)
export OUTLINE_API_URL="https://your-outline.example.com/api"

# Запустите сервер
./outline-mcp --http
```

### Клиентские запросы

Каждый запрос должен содержать два вида аутентификации:
- **`X-MCP-Token`** — токен доступа к серверу (одно из значений `MCP_AUTH_TOKENS`)
- **`Authorization: Bearer <ключ>`** — собственный API-ключ Outline пользователя

```bash
# Проверка здоровья (без аутентификации)
curl http://127.0.0.1:3000/health

# Инициализация сессии
curl -X POST http://127.0.0.1:3000/mcp \
  -H "Content-Type: application/json" \
  -H "X-MCP-Token: токен-для-алисы" \
  -H "Authorization: Bearer ol_api_xxxxx" \
  -d '{"jsonrpc":"2.0","method":"initialize","params":{},"id":1}'
# Ответ содержит заголовок Mcp-Session-Id

# Список инструментов (с сессией)
curl -X POST http://127.0.0.1:3000/mcp \
  -H "Content-Type: application/json" \
  -H "X-MCP-Token: токен-для-алисы" \
  -H "Authorization: Bearer ol_api_xxxxx" \
  -H "Mcp-Session-Id: <id-из-инициализации>" \
  -d '{"jsonrpc":"2.0","method":"tools/list","params":{},"id":2}'

# Удаление сессии
curl -X DELETE http://127.0.0.1:3000/mcp \
  -H "X-MCP-Token: токен-для-алисы" \
  -H "Mcp-Session-Id: <id-из-инициализации>"
```

### HTTP-эндпоинты

| Метод | Путь | Аутентификация | Описание |
|-------|------|----------------|----------|
| GET | /health | Нет | Проверка здоровья (`{"status":"ok","version":"..."}`) |
| POST | /mcp | Да | Обработка MCP JSON-RPC запроса |
| GET | /mcp | Да | Открытие SSE-потока (требует `Mcp-Session-Id`) |
| DELETE | /mcp | Да | Завершение сессии (требует `Mcp-Session-Id`) |
| OPTIONS | * | Нет | CORS preflight |

### HTTP-ответы об ошибках

| Статус | Значение |
|--------|----------|
| 401 | Отсутствует или недействителен заголовок `X-MCP-Token` / `Authorization` |
| 413 | Тело запроса превышает `HTTP_MAX_BODY_SIZE` |
| 415 | `Content-Type` не является `application/json` |
| 429 | Превышен лимит запросов для данного IP |

## Конфигурация

### Переменные окружения

| Переменная | Режим | Обязательна | По умолчанию | Описание |
|------------|-------|-------------|--------------|----------|
| `OUTLINE_API_KEY` | STDIO | Да | — | API-ключ Outline |
| `OUTLINE_API_URL` | Оба | Нет | `https://app.getoutline.com/api` | URL Outline |
| `MCP_AUTH_TOKENS` | HTTP | Да | — | MCP-токены доступа через запятую |
| `HTTP_HOST` | HTTP | Нет | `127.0.0.1` | Адрес привязки |
| `HTTP_PORT` | HTTP | Нет | `3000` | Номер порта (>= 1024) |
| `HTTP_RATE_LIMIT` | HTTP | Нет | `60` | Макс. запросов/мин на IP |
| `HTTP_SESSION_TIMEOUT` | HTTP | Нет | `1800` | TTL сессии в секундах (30 мин) |
| `HTTP_MAX_BODY_SIZE` | HTTP | Нет | `1048576` | Макс. размер тела запроса в байтах (1 МБ) |
| `RUST_LOG` | Оба | Нет | `error` (STDIO) / `info` (HTTP) | Уровень логирования |

### Режим STDIO (по умолчанию)
```bash
export OUTLINE_API_KEY="ваш-ключ"
./outline-mcp
```

### Режим HTTP
```bash
export MCP_AUTH_TOKENS="секретный-токен"
./outline-mcp --http
```

## Поддерживаемые инструменты (25)

Полное покрытие функциональности Outline API:

### Операции с документами (12)
- `create_document` — Создать новый документ
- `get_document` — Получить документ по ID
- `update_document` — Обновить существующий документ
- `delete_document` — Удалить документ
- `list_documents` — Список документов с фильтрацией
- `search_documents` — Поиск документов по запросу
- `archive_document` — Архивировать документ
- `restore_document` — Восстановить документ из корзины
- `unarchive_document` — Разархивировать документ
- `move_document` — Переместить документ между коллекциями
- `list_drafts` — Список черновиков документов
- `create_template_from_document` — Создать шаблон из документа

### Управление коллекциями (6)
- `create_collection` — Создать новую коллекцию
- `get_collection` — Получить детали коллекции
- `update_collection` — Обновить метаданные коллекции
- `delete_collection` — Удалить коллекцию
- `list_collections` — Список всех коллекций
- `get_collection_documents` — Получить структуру документов коллекции

### Комментарии и совместная работа (5)
- `create_comment` — Добавить комментарий к документу
- `get_comment` — Получить комментарий по ID
- `update_comment` — Изменить существующий комментарий
- `delete_comment` — Удалить комментарий
- `list_document_comments` — Список комментариев к документу

### Управление пользователями (2)
- `list_users` — Список участников команды
- `get_user` — Получить данные пользователя по ID

## Архитектура

```
┌─────────────────┐    ┌──────────────────────────────┐    ┌─────────────────┐
│   MCP-клиент    │────│      Транспортный слой        │────│  Outline API    │
│ (Claude/Cursor) │    │  STDIO | Streamable HTTP      │    │   (REST/JSON)   │
└─────────────────┘    └──────────────────────────────┘    └─────────────────┘
```

### Структура исходного кода

```
src/
├── main.rs          # Точка входа, инициализация логирования
├── lib.rs           # run_stdio(), run_http()
├── cli.rs           # Разбор аргументов командной строки
├── config.rs        # Конфигурация из переменных окружения
├── error.rs         # Централизованные типы ошибок
├── mcp.rs           # Обработчик протокола MCP JSON-RPC 2.0
├── outline.rs       # HTTP-клиент Outline API
├── tools/           # Реализации MCP-инструментов
│   ├── mod.rs       # Реестр и диспетчер инструментов
│   ├── common.rs    # Общие утилиты инструментов
│   ├── documents.rs # Операции с документами (12 инструментов)
│   ├── collections.rs # Операции с коллекциями (6 инструментов)
│   ├── comments.rs  # Операции с комментариями (5 инструментов)
│   └── users.rs     # Операции с пользователями (2 инструмента)
└── http/            # Streamable HTTP транспорт
    ├── mod.rs       # Перечисление HttpBody, объявления модулей
    ├── server.rs    # HttpServer, AppState, graceful shutdown
    ├── router.rs    # Маршрутизация запросов по методу и пути
    ├── handler.rs   # Обработчики MCP POST/GET/DELETE
    ├── auth.rs      # Валидация токенов + rate limiting
    ├── session.rs   # Управление сессиями UUID v4 с TTL
    ├── sse.rs       # SSE body + keepalive
    ├── cors.rs      # Управление CORS-заголовками
    ├── request.rs   # Валидация запросов (Content-Type, размер)
    ├── response.rs  # Построители HTTP-ответов (коды статусов)
    └── health.rs    # Эндпоинт GET /health
```

## Принципы проекта

### Производительность
- **Статическая сборка** с musl/glibc — один файл без зависимостей
- **< 5 МБ бинарник** с полной функциональностью
- **< 10 мс запуск** до готового состояния
- **< 10 МБ потребление памяти**

### Надёжность
- **Нулевые зависимости** во время выполнения (статическая линковка)
- **Явная обработка ошибок** — никаких паник в продакшене
- **Безопасность типов** — использование системы владения Rust
- **Комплексное тестирование** — юнит и интеграционные тесты

### Простота
- **Минимальный код** — только необходимая функциональность
- **Ясная архитектура** — легко понять и модифицировать
- **Один бинарник** — простое развёртывание
- **Конфигурация через переменные окружения** — без конфигурационных файлов

## Разработка

### Требования
- **Nix** (рекомендуется) — автоматически обрабатывает все зависимости
- **ИЛИ вручную**: Rust 1.75+, библиотеки разработки OpenSSL

### Команды сборки

```bash
# Окружения разработки
nix develop              # Нативная разработка с инструментами
nix develop .#musl       # musl окружение статической сборки
nix develop .#windows    # Windows кросс-компиляция
nix develop .#macos      # macOS разработка (только Darwin)

# Сборка пакетов
nix build                # Нативная сборка (Linux/macOS авто-определение)
nix build .#musl         # Статическая musl сборка (портативный Linux)
nix build .#glibc-optimized # Оптимизированная glibc сборка
nix build .#windows      # Windows кросс-компиляция
nix build .#macos-x86_64 # macOS Intel
nix build .#macos-arm64  # macOS Apple Silicon
```

### Тестирование

```bash
# Запуск всех тестов
nix develop -c cargo test

# Интеграционные тесты
nix develop -c cargo test --test http_transport_tests
nix develop -c cargo test --test mcp_tests
```

### Рабочий процесс разработки

```bash
nix develop
cargo fmt
cargo clippy -- -D warnings
cargo test
cargo build --release
```

## Вклад в проект

1. **Форкните** репозиторий
2. **Создайте** ветку функции (`git checkout -b feature/amazing-feature`)
3. **Внесите** изменения с тестами
4. **Убедитесь**, что все проверки проходят: `cargo fmt && cargo clippy -- -D warnings && cargo test`
5. **Отправьте** pull request

## Лицензия

MIT License — смотрите файл [LICENSE](LICENSE) для деталей.

## Благодарности

- Команде **Outline** за отличную документацию API
- **Anthropic** за спецификацию протокола MCP
- Сообществу **Rust** за выдающиеся инструменты и библиотеки
