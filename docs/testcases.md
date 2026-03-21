# Integration Test Cases

> For AI agents: run these test cases after any wizard or distribution changes.
> Use tmux for all interactive testing. Wait 0.5s after sending keys before capture.

## Prerequisites

```bash
# Download latest binary (macOS ARM example)
VERSION=2.0.4
curl -sL "https://github.com/LokiQ0713/cc-statusline/releases/download/v${VERSION}/cc-statusline-aarch64-apple-darwin.tar.gz" | tar xz -C /tmp/
chmod +x /tmp/cc-statusline
BINARY=/tmp/cc-statusline
```

## Environment cleanup

```bash
rm -rf ~/.claude/statusline/bin ~/.claude/statusline/config.json
```

---

## Suite 1: Language Selection

### TC-1.1: Fresh install shows language picker
- **Precondition**: No config.json
- **Steps**: Launch wizard
- **Expected**: 7 language options (English, 中文, 日本語, 한국어, Español, Português, Русский)
- **Verify**: English is default selected (●)

### TC-1.2: Existing config skips language picker
- **Precondition**: config.json exists with `lang: "en"`
- **Steps**: Launch wizard
- **Expected**: Skips directly to mode selection

---

## Suite 2: Mode Selection

### TC-2.1: Fresh install — 6 options (no "existing")
- **Precondition**: No config.json
- **Steps**: Select language → mode selection
- **Expected**: 6 options — Minimal, Developer, Full Dashboard, Rainbow, Crypto, Customize
- **Verify**: No "Use existing config" option

### TC-2.2: With existing config — 7 options (includes "existing")
- **Precondition**: config.json exists
- **Steps**: Launch wizard → mode selection
- **Expected**: 7 options — 5 presets + Use existing config + Customize

### TC-2.3: Default selection is Developer
- **Verify**: Developer option has ● marker on initial render

### TC-2.4: Preview shows Developer preset on initial load
- **Verify**: Preview bar shows Developer config (Model + Cost + Path + Git + Context)
- **Verify**: No Usage, no Crypto in preview

---

## Suite 3: Preview Switching

### TC-3.1: Navigate to Minimal — preview updates
- **Steps**: At mode selection, press Up to Minimal
- **Expected**: Preview shows only `Opus4.6 $0.42 ▓▓▓░░ 60%`

### TC-3.2: Navigate to Rainbow — preview updates
- **Steps**: Press Down to Rainbow
- **Expected**: Preview shows full-block bars (████), ultrathink rainbow colors

### TC-3.3: Navigate to "Use existing" — preview shows saved config
- **Precondition**: Existing config with specific settings (e.g., crypto disabled)
- **Steps**: Navigate to "Use existing config"
- **Expected**: Preview switches to show saved config, not default

### TC-3.4: Navigate to "Customize" — preview shows defaults
- **Steps**: Navigate to Customize
- **Expected**: Preview shows Config::default() (all segments, default styles)

---

## Suite 4: Preset Selection + Save

### TC-4.1: Select Minimal → confirm → verify config
- **Steps**: Select Minimal → Y to confirm
- **Expected config.json**:
  - `order: ["model", "cost", "context"]`
  - model.enabled=true, cost.enabled=true, context.enabled=true
  - All others disabled

### TC-4.2: Select Developer → confirm → verify config
- **Expected config.json**:
  - `order: ["model", "cost", "path", "git", "context"]`
  - model.icon="" (no icon), model.style="cyan"
  - path.max_length=20, git.show_dirty=true, git.show_remote=true

### TC-4.3: Select Full Dashboard → verify all 7 segments enabled
- **Expected**: All segments enabled, crypto has ["BTC"]

### TC-4.4: Select Rainbow → verify ultrathink styles
- **Expected**: usage.bar_char="full-block", context.bar_char="full-block"
- All color styles are "ultrathink" or "ultrathink-gradient"

### TC-4.5: Select Crypto → verify coin list
- **Expected**: crypto.coins=["BTC","ETH","SOL"], crypto.refresh_interval=30

---

## Suite 5: Existing Config Flow

### TC-5.1: "Use existing" → confirm → saves existing config
- **Precondition**: config.json with known content
- **Steps**: Select "Use existing config" → Y
- **Expected**: Config file unchanged (same content re-saved)

### TC-5.2: "Use existing" → Back → returns to mode selection
- **Steps**: Select "Use existing config" → press Left/N
- **Expected**: Returns to mode selection with all options

---

## Suite 6: Customize Flow

### TC-6.1: "Customize" starts from defaults
- **Precondition**: config.json exists with custom settings
- **Steps**: Select "Customize"
- **Expected**: Step 1 shows all 7 segments enabled (Config::default())

### TC-6.2: Full custom wizard flow
- **Steps**: Customize → select segments → configure → reorder → confirm
- **Expected**: Each step renders correctly with ◆ header and pending footer

### TC-6.3: Back navigation through wizard steps
- **Steps**: Go to Step 2 → press Left → back to Step 1
- **Expected**: Config restored from snapshot, preview updated

---

## Suite 7: On-change + Back Restoration

### TC-7.1: Model icon — navigate then Back
- **Steps**: At model icon selection, navigate to 🐢 → press Left (Back)
- **Expected**: Preview restores to previous icon (not 🐢)

### TC-7.2: Usage bar style — navigate then Back
- **Steps**: At usage bar style, navigate to Blue → press Left
- **Expected**: Bar style restored, preview shows original style

### TC-7.3: Step 0 multiselect — toggle then Back
- **Steps**: At segment selection, uncheck Model → press Left
- **Expected**: All segments re-checked (snapshot restored)

---

## Suite 8: Render Mode

### TC-8.1: --render with valid JSON
```bash
echo '{"model":{"id":"claude-opus-4-6"},"context_window":{"context_window_size":1000000,"used_percentage":0.6},"cost":{"total_cost_usd":"0.42"}}' | $BINARY --render
```
- **Expected**: Non-empty output with ANSI color codes

### TC-8.2: --render with empty JSON
```bash
echo '{}' | $BINARY --render
```
- **Expected**: Exits 0, may output partial segments

### TC-8.3: --render with no stdin
```bash
echo '' | $BINARY --render
```
- **Expected**: Exits 0, no crash

---

## Suite 9: Distribution Channels

### TC-9.1: npm install + postinstall downloads binary
```bash
rm -rf ~/.claude/statusline/bin
npm install -g cc-statusline@latest
ls ~/.claude/statusline/bin/cc-statusline
```
- **Expected**: Binary exists, is executable

### TC-9.2: brew tap + install
```bash
brew tap LokiQ0713/cc-statusline
brew install cc-statusline
which cc-statusline
```
- **Expected**: Binary available in PATH

---

## Suite 10: Settings.json Integration

### TC-10.1: Save updates settings.json
- **Steps**: Complete any wizard flow (preset or custom) with confirm
- **Expected**: `~/.claude/settings.json` has `statusLine.command` = `~/.claude/statusline/bin/cc-statusline --render`

### TC-10.2: Re-save preserves other settings
- **Precondition**: settings.json has other keys (e.g., `theme`)
- **Steps**: Run wizard and save
- **Expected**: Other keys preserved, only `statusLine` updated
