# cc-statusline

[![CI](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/ci.yml/badge.svg)](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/ci.yml)
[![Release](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/release.yml/badge.svg)](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/release.yml)
[![npm](https://img.shields.io/npm/v/cc-statusline-tui)](https://www.npmjs.com/package/cc-statusline-tui)
[![crates.io](https://img.shields.io/crates/v/cc-statusline-tui)](https://crates.io/crates/cc-statusline-tui)

> A barra de status do seu Claude Code tá sem graça. Bora dar um trato nela.

[English](README.md) | [中文](README.zh.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Español](README.es.md) | [Русский](README.ru.md)

![statusline preview](preview.png)

Um comando. Sete idiomas. Zero arrependimentos.

## Instalação

Escolhe teu método:

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

## O que acontece?

Um wizard TUI aparece. Você escolhe umas coisas. Fica bonito. É isso.

```
1/4 Segmentos     → O que mostrar
2/4 Configurar    → Como vai ficar (preview ao vivo, feedback instantâneo)
3/4 Reordenar     → Onde cada coisa fica
4/4 Confirmar     → Manda ver!
```

Tá com preguiça de customizar? Escolhe "Usar padrões" e aperta Enter duas vezes. A gente não julga.

## Segmentos

| Segmento | Aparência | O que faz | Dá pra ajustar |
|----------|----------|-----------|----------------|
| Model | `🔥 Opus4.6` | Mostra qual cérebro você tá usando | Ícone (🔥🦊🐢🐰), cor |
| Cost | `$0.42` | O nível de sofrimento da sua carteira | Cor |
| Usage | `██░░ 25% 1h43m` | Limite de 5h: barra + % + contagem regressiva | Estilo/caractere/tamanho da barra, cor |
| Path | `~/project` | Onde você tá | Cor, tamanho máximo |
| Git | `main* ↑2↓1` | Branch + alterações + ahead/behind | Componentes, cor |
| Context | `▓▓▓▓░░░ 60% 600K/1M` | Quanto da janela de contexto você já comeu | Estilo/caractere/tamanho da barra, cor |
| Crypto | `BTC:$73748` | Porque por que não checar preços enquanto programa? | Moedas (BTC/ETH/BNB/SOL), cor, atualização |

## Estilos de cor

| Estilo | A vibe |
|--------|--------|
| **Ultrathink Rainbow** | Cada caractere é de uma cor diferente. Sim, ele brilha. |
| **Ultrathink Gradient** | Arco-íris suave ao longo da barra. De cair o queixo. |
| **Traffic Light** | Verde (≤30%) → Amarelo (≤60%) → Vermelho (>60%). Sua janela de contexto virou um congestionamento. |
| Cyan / Green / Blue / Yellow | As escolhas sensatas |
| Magenta / Red / Orange / Pink / Purple / White | As escolhas expressivas |

## Idiomas

English, 中文, 日本語, 한국어, Español, Português, Русский.

Selecionado na primeira execução. Sua barra de status fala a sua língua.

## Como funciona

1. O wizard salva a configuração em `~/.claude/statusline/config.json`
2. O binário vai pra `~/.claude/statusline/bin/`
3. `~/.claude/settings.json` é atualizado automaticamente
4. Reinicie o Claude Code. Admire sua nova barra de status. Conte pros colegas.

Rodar de novo carrega sua configuração existente como padrão. Não destrói nada. Promessa.

## Requisitos

- Claude Code instalado (`~/.claude/` precisa existir)
- Node.js ≥ 18 (só pra instalar via `npx` — não precisa no runtime)

## Segurança e Privacidade

Seus dados a gente não mexe. Pode ficar tranquilo:

- O **segmento de crypto** faz requisições para a API pública da Binance (`api.binance.com`) — sem autenticação, sem dados pessoais enviados
- O **segmento de usage** lê diretamente os dados JSON que o Claude Code passa via stdin (campo nativo `rate_limits`) — sem chamadas a APIs externas e sem acesso ao chaveiro
- O **npm postinstall** baixa um binário específico para sua plataforma do GitHub Releases — nenhum outro download externo
- Todos os dados são cacheados localmente em arquivos `/tmp/claude-statusline-*`
- Sem telemetria, sem analytics, sem envio de dados pra nenhum outro lugar
- Para mais detalhes veja [SECURITY.md](SECURITY.md)

## Desinstalar

```bash
# Remover configuração e binário
rm -rf ~/.claude/statusline/

# Remover a statusline das configurações do Claude Code
# Edite ~/.claude/settings.json e delete a chave "statusLine"

# Remover cache
rm -f /tmp/claude-statusline-*

# Desinstalar do gerenciador de pacotes
npm uninstall -g cc-statusline-tui
# ou: cargo uninstall cc-statusline-tui
# ou: brew uninstall cc-statusline
```

## Solução de problemas

| Problema | Solução |
|----------|---------|
| "Binary not found" | Rode `npx cc-statusline-tui` de novo pra baixar novamente |
| Erro "Is a directory" | Verifique que `~/.claude/statusline/bin/cc-statusline` é um arquivo, não um diretório |
| Crypto não aparece | Delete o diretório `/tmp/claude-statusline-crypto-lock` se existir (lock travado) |
| Mudanças não aparecem | Reinicie o Claude Code depois de salvar a configuração |

## Contribuindo

Achou um bug? Quer uma feature? [Abra uma issue](https://github.com/LokiQ0713/cc-statusline-tui/issues). PRs são bem-vindos.

## License

MIT
