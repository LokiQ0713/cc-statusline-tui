# cc-statusline

[![CI](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/ci.yml/badge.svg)](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/ci.yml)
[![Release](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/release.yml/badge.svg)](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/release.yml)
[![npm](https://img.shields.io/npm/v/cc-statusline-tui)](https://www.npmjs.com/package/cc-statusline-tui)
[![crates.io](https://img.shields.io/crates/v/cc-statusline-tui)](https://crates.io/crates/cc-statusline-tui)

> Claude Code 상태표시줄, 너무 밋밋하지 않나요? 한번 꾸며봅시다.

[English](README.md) | [中文](README.zh.md) | [日本語](README.ja.md) | [Español](README.es.md) | [Português](README.pt.md) | [Русский](README.ru.md)

![statusline preview](preview.png)

명령어 하나. 7개 언어. 후회 제로.

## 설치

취향대로 골라주세요:

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

## 뭐가 되는 건데?

TUI 마법사가 뜹니다. 이것저것 고르면 됩니다. 멋져집니다. 끝.

```
1/4 세그먼트      → 뭘 보여줄지
2/4 설정          → 어떻게 보일지 (실시간 미리보기, 즉각 반영)
3/4 순서 정하기    → 어디에 배치할지
4/4 확인          → 적용!
```

커스터마이즈 귀찮으시다고요? "기본값 사용" 선택하고 Enter 두 번이면 끝. 아무도 뭐라 안 합니다.

## 세그먼트

| 세그먼트 | 이렇게 보여요 | 하는 일 | 바꿀 수 있는 것 |
|---------|-------------|--------|---------------|
| Model | `🔥 Opus4.6` | 지금 쓰고 있는 모델 표시 | 아이콘 (🔥🦊🐢🐰), 색상 |
| Cost | `$0.42` | 지갑이 얼마나 아픈지 실시간 확인 | 색상 |
| Usage | `██░░ 25% 1h43m` | 5시간 사용량 제한: 바 + % + 리셋 카운트다운 | 바 스타일/문자/길이, 색상 |
| Path | `~/project` | 현재 위치 | 색상, 최대 길이 |
| Git | `main* ↑2↓1` | 브랜치 + 변경사항 + ahead/behind | 표시 항목, 색상 |
| Context | `▓▓▓▓░░░ 60% 600K/1M` | 컨텍스트 윈도우 얼마나 먹었는지 | 바 스타일/문자/길이, 색상 |
| Crypto | `BTC:$73748` | 코딩하다가 코인 시세도 확인. 왜냐고요? 그냥요. | 코인 (BTC/ETH/BNB/SOL), 색상, 갱신주기 |

## 색상 스타일

| 스타일 | 느낌 |
|-------|------|
| **Ultrathink Rainbow** | 글자마다 색이 다름. 네, 반짝반짝합니다. |
| **Ultrathink Gradient** | 바 전체에 부드러운 무지개 그라데이션. 감동 그 자체. |
| **Traffic Light** | 초록 (≤30%) → 노랑 (≤60%) → 빨강 (>60%). 컨텍스트 윈도우가 막히고 있어요. |
| Cyan / Green / Blue / Yellow | 무난한 선택 |
| Magenta / Red / Orange / Pink / Purple / White | 개성 넘치는 선택 |

## 지원 언어

English, 中文, 日本語, 한국어, Español, Português, Русский.

첫 실행 시 선택. 상태표시줄이 당신의 언어로 말합니다.

## 작동 원리

1. 마법사가 설정을 `~/.claude/statusline/config.json`에 저장
2. 바이너리가 `~/.claude/statusline/bin/`에 복사됨
3. `~/.claude/settings.json`이 자동으로 업데이트됨
4. Claude Code 재시작. 새 상태표시줄 감상. 동료한테 자랑.

다시 실행하면 기존 설정이 기본값으로 불러와집니다. 기존 설정 안 날아갑니다. 약속.

## 요구사항

- Claude Code 설치됨 (`~/.claude/`가 존재해야 함)
- Node.js ≥ 18 (`npx` 설치 시에만 필요 — 실행 시에는 불필요)

## 보안 및 개인정보

여러분의 데이터, 건드리지 않습니다. 안심하세요:

- **Crypto 세그먼트**는 Binance 공개 API(`api.binance.com`)에 요청합니다 — 인증 불필요, 개인 데이터 전송 없음
- **Usage 세그먼트**는 Claude Code가 stdin으로 전달하는 JSON 데이터(네이티브 `rate_limits` 필드)를 직접 읽습니다 — 외부 API 호출이나 키체인 접근 불필요
- **npm postinstall**은 GitHub Releases에서 플랫폼별 바이너리를 다운로드합니다 — 그 외 외부 다운로드 없음
- 모든 데이터는 `/tmp/claude-statusline-*` 파일에 로컬 캐시됩니다
- 텔레메트리 없음, 분석 없음, 다른 곳으로 데이터 전송 없음
- 자세한 내용은 [SECURITY.md](SECURITY.md)를 참고하세요

## 제거

```bash
# 설정과 바이너리 삭제
rm -rf ~/.claude/statusline/

# Claude Code 설정에서 상태표시줄 제거
# ~/.claude/settings.json 을 편집하고 "statusLine" 키를 삭제

# 캐시 삭제
rm -f /tmp/claude-statusline-*

# 패키지 매니저에서 제거
npm uninstall -g cc-statusline-tui
# 또는: cargo uninstall cc-statusline-tui
# 또는: brew uninstall cc-statusline
```

## 문제 해결

| 문제 | 해결 방법 |
|------|----------|
| "Binary not found" | `npx cc-statusline-tui`를 다시 실행해서 재다운로드 |
| "Is a directory" 오류 | `~/.claude/statusline/bin/cc-statusline`가 디렉토리가 아닌 파일인지 확인 |
| 암호화폐가 안 보임 | `/tmp/claude-statusline-crypto-lock` 디렉토리가 있으면 삭제 (오래된 잠금) |
| 변경사항이 안 보임 | 설정 저장 후 Claude Code 재시작 |

## 기여하기

버그 발견? 기능 제안? [이슈를 열어주세요](https://github.com/LokiQ0713/cc-statusline-tui/issues). PR 환영합니다.

## License

MIT
