# Outline MCP Server

[![CI](https://github.com/nizovtsevnv/outline-mcp-rs/workflows/CI/badge.svg)](https://github.com/nizovtsevnv/outline-mcp-rs/actions/workflows/ci.yml)
[![Release Build](https://github.com/nizovtsevnv/outline-mcp-rs/workflows/Release%20Build/badge.svg)](https://github.com/nizovtsevnv/outline-mcp-rs/actions/workflows/release.yml)
[![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/nizovtsevnv/outline-mcp-rs?sort=semver)](https://github.com/nizovtsevnv/outline-mcp-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)

MCP (Model Context Protocol) сервер для взаимодействия с API Outline с фокусом на **простоту**, **производительность** и **надежность**.

## 🚀 Быстрый старт

### 1. Получите ваш API ключ Outline
- **Outline.com**: https://app.outline.com/settings/api-and-apps
- **Собственный хостинг**: https://your-instance.com/settings/api-and-apps

### 2. Скачайте и установите

Выберите один из способов установки:

#### 🔄 Вариант 1: Скачивание готового бинарника (Рекомендуемый)
Скачайте с [GitHub Releases](https://github.com/nizovtsevnv/outline-mcp-rs/releases)

**После извлечения архива:**
- **Linux/macOS**: При необходимости сделайте исполняемым: `chmod +x outline-mcp`
- **Windows**: Поскольку релиз не подписан цифровой подписью, 🛡️ Windows Defender может заблокировать выполнение. Вам потребуется:
  1. Разрешить исполняемый файл через Windows Defender/антивирус
  2. Добавить папку в исключения Windows Defender, или
  3. Щелкнуть правой кнопкой мыши на файле → Свойства → "Разблокировать" если скачано из интернета

#### 📦 Вариант 2: Установка из crates.io
```bash
cargo install outline-mcp-rs
```
*Требует Rust toolchain. Бинарник будет установлен в `~/.cargo/bin/outline-mcp`*

#### 🔨 Вариант 3: Сборка из исходного кода
```bash
git clone https://github.com/nizovtsevnv/outline-mcp-rs.git
cd outline-mcp-rs
cargo build --release
# Бинарник будет в target/release/outline-mcp
```

#### ❄️ Вариант 4: Nix (с воспроизводимой средой)
```bash
nix run github:nizovtsevnv/outline-mcp-rs
```

### 3. Настройте ваш AI агент

JSON конфигурация для Cursor IDE, Gemini CLI:
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

> **💡 Примечания о путях:**
> - **cargo install**: Используйте `"outline-mcp"` (автоматически добавляется в PATH)  
> - **Скачанный бинарник**: Используйте полный путь вроде `"/path/to/outline-mcp"`
> - **Собранный из исходников**: Используйте `"/path/to/outline-mcp-rs/target/release/outline-mcp"`

**⚠️ Важные требования к путям:**
- **Используйте абсолютные пути** - относительные пути могут работать некорректно
- **Без пробелов** в пути к исполняемому файлу (используйте подчеркивания или дефисы)
- **Только ASCII символы** - избегайте кириллицы и других не-латинских символов в путях
- **Пользователи Windows**: Используйте двойной обратный слеш `\\` в путях (например, `"C:\\tools\\outline-mcp.exe"`)

**✅ Хорошие примеры:**
- Linux/macOS: `"/usr/local/bin/outline-mcp"` или `"/home/user/bin/outline-mcp"`
- Windows: `"C:\\tools\\outline-mcp.exe"` или `"C:\\Users\\YourName\\bin\\outline-mcp.exe"`

**❌ Избегайте:**
- `"./outline-mcp"` (относительный путь)
- `"/path with spaces/outline-mcp"` (пробелы в пути)
- `"/путь/outline-mcp"` (не-латинские символы)
- `"C:\tools\outline-mcp.exe"` (одинарный обратный слеш в Windows)

## 🛠️ Поддерживаемые инструменты

Полное покрытие функциональности Outline API:

### 📄 Операции с документами
- `create_document` - Создать новый документ
- `get_document` - Получить документ по ID
- `update_document` - Обновить существующий документ
- `delete_document` - Удалить документ
- `list_documents` - Список документов с фильтрацией
- `search_documents` - Поиск документов по запросу
- `archive_document` - Архивировать документ
- `move_document` - Переместить документ между коллекциями

### 📁 Управление коллекциями
- `create_collection` - Создать новую коллекцию
- `get_collection` - Получить детали коллекции
- `update_collection` - Обновить метаданные коллекции
- `list_collections` - Список всех коллекций

### 💬 Комментарии и совместная работа
- `create_comment` - Добавить комментарий к документу
- `update_comment` - Изменить существующий комментарий
- `delete_comment` - Удалить комментарий

### 🔍 Расширенные функции
- `create_template_from_document` - Создать шаблоны для повторного использования
- `list_users` - Управление пользователями

## 🎯 Принципы проекта

### ⚡ Производительность
- **Статическая сборка** с musl - один файл без зависимостей
- **< 5MB бинарный файл** с полной функциональностью
- **< 10мс время запуска** до готового состояния
- **< 10MB использование памяти**

### 🛡️ Надежность
- **Нулевые зависимости** во время выполнения (статическая линковка)
- **Явная обработка ошибок** - никаких паник в продакшене
- **Безопасность типов** - использование системы владения Rust
- **Комплексное тестирование** - юнит и интеграционные тесты

### 🔧 Простота
- **Минимальный код** - только основная функциональность
- **Ясная архитектура** - легко понять и модифицировать
- **Один бинарный файл** - простое развертывание
- **Конфигурация через переменные окружения** - без конфигурационных файлов

## 📋 Требования для разработки
- **Nix** (рекомендуется) - автоматически обрабатывает все зависимости
- **ИЛИ вручную**: Rust 1.75+, библиотеки разработки OpenSSL

## 🏗️ Архитектура

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   MCP Клиент    │────│  Транспортный    │────│  Outline API    │
│   (Claude/др.)  │    │  слой (STDIO/HTTP)│    │   (REST/JSON)   │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

### Основные компоненты
- **Транспортный слой**: Адаптеры STDIO и HTTP
- **MCP Протокол**: Реализация JSON-RPC 2.0
- **Outline Клиент**: Обертка HTTP API
- **Реестр инструментов**: Динамическое обнаружение и выполнение инструментов

#### Быстрые команды сборки:
```bash
# Linux/Unix системы
nix build                # Linux нативная
nix build .#musl         # Linux статическая (портативная)
nix build .#windows      # Windows кросс-компиляция

# macOS системы (требует Nix на macOS)  
nix build                # Авто-определение Intel/ARM
nix build .#macos-x86_64 # Intel цель
nix build .#macos-arm64  # ARM цель
```

#### Настройка разработки для macOS:
```bash
# Установка Nix на macOS
curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install

# Включение флаков
echo "experimental-features = nix-command flakes" >> ~/.config/nix/nix.conf

# Клонирование и сборка
git clone https://github.com/nizovtsevnv/outline-mcp-rs
cd outline-mcp-rs
nix build
```

## 🧪 Тестирование

```bash
# Запуск всех тестов
nix develop -c cargo test

# Запуск с покрытием
nix develop -c cargo test --coverage

# Интеграционные тесты с живым API (установите OUTLINE_API_KEY)
nix develop -c cargo test --test integration
```

## 🔧 Конфигурация

### Режим STDIO (по умолчанию)
```bash
export OUTLINE_API_KEY="ваш-ключ-здесь"
./outline-mcp
```

### Режим HTTP
```bash
export OUTLINE_API_KEY="ваш-ключ-здесь"
export HTTP_HOST="0.0.0.0"
export HTTP_PORT="8080"
./outline-mcp --http
```

## 🔧 Оптимизированная конфигурация Nix

Наш `flake.nix` был тщательно оптимизирован для устранения дублирования и улучшения поддерживаемости:

### 🏗️ Улучшения архитектуры

- **📦 Синхронизация метаданных**: Информация о пакете ссылается на значения `Cargo.toml` с комментариями
- **🔄 Переиспользуемый строитель оболочки**: Функция `mkDevShell` устраняет дублирование кода
- **🎯 Единообразные хуки оболочки**: Унифицированная функция `mkShellHook` для всех окружений  
- **⚡ Базовые входные данные сборки**: Общие зависимости для всех оболочек разработки
- **🧪 Автоматизированные проверки**: Встроенные рабочие процессы форматирования, линтинга и тестирования

### 📋 Доступные команды

```bash
# Окружения разработки
nix develop              # Нативная разработка с инструментами
nix develop .#musl       # musl окружение статической сборки  
nix develop .#windows    # Windows кросс-компиляция
nix develop .#macos      # macOS разработка (только Darwin)

# Сборка пакетов
nix build                # Нативная сборка (Linux/macOS авто-определение)
nix build .#musl         # Статическая musl сборка (портативный Linux)
nix build .#windows      # Windows кросс-компиляция
nix build .#macos-x86_64 # macOS Intel (требует macOS или CI)
nix build .#macos-arm64  # macOS Apple Silicon (требует macOS или CI)

# Альтернатива: Использование dev окружения для сборки
nix develop -c cargo build --release                              # Нативная
nix develop .#musl -c cargo build --target x86_64-unknown-linux-musl --release    # musl
nix develop .#windows -c cargo build --target x86_64-pc-windows-gnu --release     # Windows

# macOS цели (только macOS)
nix develop -c cargo build --target x86_64-apple-darwin --release   # Intel Mac
nix develop -c cargo build --target aarch64-apple-darwin --release  # Apple Silicon
```

## 🤝 Вклад в проект

1. **Форкните** репозиторий
2. **Создайте** ветку функции (`git checkout -b feature/amazing-feature`)
3. **Внесите** изменения с тестами
4. **Убедитесь**, что все проверки проходят: `cargo test && cargo clippy`
5. **Отправьте** pull request

### Рабочий процесс разработки
```bash
# Настройка окружения разработки
nix develop

# Форматирование кода
cargo fmt

# Линтинг
cargo clippy

# Тестирование
cargo test

# Кросс-платформенное тестирование
nix develop .#musl --command cargo test --target x86_64-unknown-linux-musl
nix develop .#windows --command cargo check --target x86_64-pc-windows-gnu
```

## 📄 Лицензия

MIT License - смотрите файл [LICENSE](LICENSE) для деталей.

## 🙏 Благодарности

- Команде **Outline** за отличную документацию API
- **Anthropic** за спецификацию протокола MCP
- Сообществу **Rust** за выдающиеся инструменты и библиотеки 