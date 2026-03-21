# cc-statusline

[![CI](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/ci.yml/badge.svg)](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/ci.yml)
[![Release](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/release.yml/badge.svg)](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/release.yml)
[![npm](https://img.shields.io/npm/v/cc-statusline-tui)](https://www.npmjs.com/package/cc-statusline-tui)
[![crates.io](https://img.shields.io/crates/v/cc-statusline-tui)](https://crates.io/crates/cc-statusline-tui)

> Статусная строка Claude Code скучная. Давайте это исправим.

[English](README.md) | [中文](README.zh.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Español](README.es.md) | [Português](README.pt.md)

![statusline preview](preview.png)

Одна команда. Семь языков. Ноль сожалений.

## Установка

Выбирай на вкус:

### npm

```bash
npx cc-statusline-tui
```

### Cargo

```bash
cargo install cc-statusline-tui
```

### Homebrew

```bash
brew tap LokiQ0713/cc-statusline-tui
brew install cc-statusline
```

## Что происходит?

Появляется TUI-мастер. Выбираешь штуки. Получается красиво. Всё.

```
1/4 Сегменты      → Что показывать
2/4 Настройка     → Как это выглядит (живой предпросмотр, мгновенный отклик)
3/4 Порядок       → Что куда поставить
4/4 Подтверждение → Поехали!
```

Лень настраивать? Выбери «По умолчанию» и нажми Enter дважды. Мы не осуждаем.

## Сегменты

| Сегмент | Выглядит так | Что делает | Можно настроить |
|---------|-------------|------------|----------------|
| Model | `🔥 Opus4.6` | Показывает, какой мозг ты сейчас жжёшь | Иконка (🔥🦊🐢🐰), цвет |
| Cost | `$0.42` | Уровень боли твоего кошелька | Цвет |
| Usage | `██░░ 25% 1h43m` | Лимит 5ч: полоска + % + обратный отсчёт | Стиль/символ/длина полоски, цвет, обновление |
| Path | `~/project` | Где ты находишься | Цвет, макс. длина |
| Git | `main* ↑2↓1` | Ветка + незакоммиченное + ahead/behind | Компоненты, цвет |
| Context | `▓▓▓▓░░░ 60% 600K/1M` | Сколько контекстного окна ты уже съел | Стиль/символ/длина полоски, цвет |
| Crypto | `BTC:$73748` | А почему бы не глянуть курс, пока кодишь? | Монеты (BTC/ETH/BNB/SOL), цвет, обновление |

## Цветовые стили

| Стиль | Атмосфера |
|-------|-----------|
| **Ultrathink Rainbow** | Каждый символ — свой цвет. Да, оно переливается. |
| **Ultrathink Gradient** | Плавная радуга по всей полоске. Шедевр. |
| **Traffic Light** | Зелёный (≤30%) → Жёлтый (≤60%) → Красный (>60%). Твоё контекстное окно — это пробка. |
| Cyan / Green / Blue / Yellow | Разумный выбор |
| Magenta / Red / Orange / Pink / Purple / White | Для тех, кто хочет самовыражения |

## Языки

English, 中文, 日本語, 한국어, Español, Português, Русский.

Выбирается при первом запуске. Статусная строка говорит на твоём языке.

## Как это работает

1. Мастер сохраняет конфиг в `~/.claude/statusline/config.json`
2. Бинарник копируется в `~/.claude/statusline/bin/`
3. `~/.claude/settings.json` обновляется автоматически
4. Перезапусти Claude Code. Полюбуйся новой строкой. Похвастайся коллегам.

При повторном запуске загружается существующий конфиг как значения по умолчанию. Ничего не ломает. Обещаем.

## Требования

- Установлен Claude Code (существует `~/.claude/`)
- Node.js ≥ 18 (только для установки через `npx` — в рантайме не нужен)

## Безопасность и конфиденциальность

Твои данные мы не трогаем. Честно:

- **Crypto-сегмент** обращается к публичному API Binance (`api.binance.com`) — без аутентификации, без отправки личных данных
- **Usage-сегмент** читает OAuth-токен из связки ключей macOS (`Claude Code-credentials`) для запроса к API использования Anthropic
- **npm postinstall** скачивает бинарник для твоей платформы из GitHub Releases — больше ничего не скачивает
- Все данные кэшируются локально в файлах `/tmp/claude-statusline-*`
- Никакой телеметрии, никакой аналитики, данные никуда больше не отправляются
- Подробности в [SECURITY.md](SECURITY.md)

## Удаление

```bash
# Удалить конфиг и бинарник
rm -rf ~/.claude/statusline/

# Удалить статусную строку из настроек Claude Code
# Отредактируй ~/.claude/settings.json и удали ключ "statusLine"

# Удалить кэш
rm -f /tmp/claude-statusline-*

# Удалить через пакетный менеджер
npm uninstall -g cc-statusline-tui
# или: cargo uninstall cc-statusline-tui
# или: brew uninstall cc-statusline
```

## Решение проблем

| Проблема | Решение |
|----------|---------|
| "Binary not found" | Запусти `npx cc-statusline-tui` ещё раз для повторной загрузки |
| Ошибка "Is a directory" | Проверь, что `~/.claude/statusline/bin/cc-statusline` — это файл, а не директория |
| Crypto не отображается | Удали директорию `/tmp/claude-statusline-crypto-lock`, если она существует (устаревшая блокировка) |
| Изменения не видны | Перезапусти Claude Code после сохранения настроек |

## Участие в проекте

Нашёл баг? Хочешь фичу? [Создай issue](https://github.com/LokiQ0713/cc-statusline-tui/issues). PR приветствуются.

## License

MIT
