# cc-statusline

[![CI](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/ci.yml/badge.svg)](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/ci.yml)
[![Release](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/release.yml/badge.svg)](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/release.yml)
[![npm](https://img.shields.io/npm/v/cc-statusline-tui)](https://www.npmjs.com/package/cc-statusline-tui)
[![crates.io](https://img.shields.io/crates/v/cc-statusline-tui)](https://crates.io/crates/cc-statusline-tui)

> Tu barra de estado de Claude Code es aburrida. Vamos a arreglar eso.

[English](README.md) | [中文](README.zh.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md)

![statusline preview](preview.png)

Un comando. Siete idiomas. Cero arrepentimientos.

## Instalación

Elige tu veneno:

### npm

```bash
npx cc-statusline-tui@latest
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

## ¿Qué pasa?

Aparece un asistente TUI. Eliges cosas. Queda bonito. Fin.

```
1/4 Segmentos     → Qué mostrar
2/4 Configurar    → Cómo se ve (vista previa en vivo, feedback instantáneo)
3/4 Reordenar     → Dónde va cada cosa
4/4 Confirmar     → ¡A producción!
```

¿Te da pereza personalizar? Elige "Usar valores por defecto" y pulsa Enter dos veces. Aquí no juzgamos a nadie.

## Segmentos

| Segmento | Se ve así | Qué hace | Puedes ajustar |
|----------|----------|----------|----------------|
| Model | `🔥 Opus4.6` | Muestra qué cerebro estás quemando | Icono (🔥🦊🐢🐰), color |
| Cost | `$0.42` | El nivel de dolor de tu cartera | Color |
| Usage | `██░░ 25% 1h43m` | Límite de 5h: barra + % + cuenta atrás | Estilo/carácter/longitud de barra, color |
| Path | `~/project` | Dónde estás | Color, longitud máxima |
| Git | `main* ↑2↓1` | Rama + cambios sin commit + ahead/behind | Componentes, color |
| Context | `▓▓▓▓░░░ 60% 600K/1M` | Cuánta ventana de contexto te has comido | Estilo/carácter/longitud de barra, color |
| Crypto | `BTC:$73748` | Porque ¿por qué no mirar precios mientras programas? | Monedas (BTC/ETH/BNB/SOL), color, actualización |

## Estilos de color

| Estilo | La onda |
|--------|---------|
| **Ultrathink Rainbow** | Cada carácter es de un color diferente. Sí, brilla. |
| **Ultrathink Gradient** | Arcoíris suave a lo largo de la barra. Obra de arte. |
| **Traffic Light** | Verde (≤30%) → Amarillo (≤60%) → Rojo (>60%). Tu ventana de contexto es un atasco. |
| Cyan / Green / Blue / Yellow | Las opciones sensatas |
| Magenta / Red / Orange / Pink / Purple / White | Las opciones expresivas |

## Idiomas

English, 中文, 日本語, 한국어, Español, Português, Русский.

Se selecciona en la primera ejecución. Tu barra de estado habla tu idioma.

## Cómo funciona

1. El asistente guarda la configuración en `~/.claude/statusline/config.json`
2. El binario va a `~/.claude/statusline/bin/`
3. `~/.claude/settings.json` se actualiza automáticamente
4. Reinicia Claude Code. Admira tu nueva barra de estado. Cuéntaselo a tus compañeros.

Si lo ejecutas de nuevo, carga tu configuración existente como valores por defecto. No destruye nada. Prometido.

## Requisitos

- Claude Code instalado (que exista `~/.claude/`)
- Node.js ≥ 18 (solo para instalar con `npx` — no se necesita en tiempo de ejecución)

## Seguridad y Privacidad

Tus datos no los tocamos. Tranquilo:

- El **segmento de crypto** hace peticiones a la API pública de Binance (`api.binance.com`) — sin autenticación, sin datos personales enviados
- El **segmento de usage** lee directamente los datos JSON que Claude Code pasa por stdin (campo nativo `rate_limits`) — sin llamadas a APIs externas ni acceso al llavero
- El **npm postinstall** descarga un binario específico para tu plataforma desde GitHub Releases — nada más se descarga
- Todos los datos se cachean localmente en archivos `/tmp/claude-statusline-*`
- Sin telemetría, sin analytics, sin envío de datos a ningún otro lugar
- Para más detalles, consulta [SECURITY.md](SECURITY.md)

## Desinstalar

```bash
# Eliminar configuración y binario
rm -rf ~/.claude/statusline/

# Eliminar la línea de estado de la configuración de Claude Code
# Edita ~/.claude/settings.json y borra la clave "statusLine"

# Eliminar caché
rm -f /tmp/claude-statusline-*

# Desinstalar del gestor de paquetes
npm uninstall -g cc-statusline-tui
# o: cargo uninstall cc-statusline-tui
# o: brew uninstall cc-statusline
```

## Solución de problemas

| Problema | Solución |
|----------|----------|
| "Binary not found" | Ejecuta `npx cc-statusline-tui@latest` de nuevo para volver a descargar |
| Error "Is a directory" | Verifica que `~/.claude/statusline/bin/cc-statusline` sea un archivo, no un directorio |
| Crypto no se muestra | Elimina el directorio `/tmp/claude-statusline-crypto-lock` si existe (bloqueo obsoleto) |
| Los cambios no se ven | Reinicia Claude Code después de guardar la configuración |

## Contribuir

¿Encontraste un bug? ¿Quieres una feature? [Abre un issue](https://github.com/LokiQ0713/cc-statusline-tui/issues). Los PRs son bienvenidos.

## License

MIT
