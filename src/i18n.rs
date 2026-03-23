//! Internationalization with 7 languages (en/zh/ja/ko/es/pt/ru).
//!
//! Provides two translation functions:
//! - `t(key)` -- static translation lookup, returns `&'static str`
//! - `tf(key, args)` -- template translation with positional `{0}`, `{1}` placeholders
//!
//! The active language is stored in an `AtomicU8` static for zero-cost switching.
//! Call `set_lang("zh")` to change; unknown codes default to English.
//!
//! Used throughout the wizard for all user-facing text. The render pipeline
//! does not use i18n (statusline output is language-independent).

use std::sync::atomic::{AtomicU8, Ordering};

/// Supported language codes.
pub const SUPPORTED_LANGS: &[&str] = &["en", "zh", "ja", "ko", "es", "pt", "ru"];

/// 0 = en, 1 = zh, 2 = ja, 3 = ko, 4 = es, 5 = pt, 6 = ru
static LANG: AtomicU8 = AtomicU8::new(0);

/// Set the current language. Defaults to en for unknown values.
pub fn set_lang(lang: &str) {
    let code = match lang {
        "zh" => 1,
        "ja" => 2,
        "ko" => 3,
        "es" => 4,
        "pt" => 5,
        "ru" => 6,
        _ => 0,
    };
    LANG.store(code, Ordering::Relaxed);
}

/// Get the current language code.
#[allow(dead_code)]
pub fn get_lang() -> &'static str {
    match LANG.load(Ordering::Relaxed) {
        1 => "zh",
        2 => "ja",
        3 => "ko",
        4 => "es",
        5 => "pt",
        6 => "ru",
        _ => "en",
    }
}

/// Look up a static translation by key.
/// Returns the key itself if not found.
pub fn t(key: &str) -> &'static str {
    match LANG.load(Ordering::Relaxed) {
        1 => t_zh(key),
        2 => t_ja(key),
        3 => t_ko(key),
        4 => t_es(key),
        5 => t_pt(key),
        6 => t_ru(key),
        _ => t_en(key),
    }
}

/// Look up a dynamic translation by key, replacing positional placeholders with `args`.
///
/// Placeholders are `{0}`, `{1}`, etc., corresponding to the index in `args`.
/// Returns the key itself (no substitution) if the key is not found.
pub fn tf(key: &str, args: &[&str]) -> String {
    let template = match LANG.load(Ordering::Relaxed) {
        1 => tf_zh(key),
        2 => tf_ja(key),
        3 => tf_ko(key),
        4 => tf_es(key),
        5 => tf_pt(key),
        6 => tf_ru(key),
        _ => tf_en(key),
    };
    match template {
        Some(tmpl) => substitute(tmpl, args),
        None => key.to_string(),
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn substitute(template: &str, args: &[&str]) -> String {
    let mut result = template.to_string();
    for (i, arg) in args.iter().enumerate() {
        let placeholder = format!("{{{}}}", i);
        result = result.replace(&placeholder, arg);
    }
    result
}

// ---------------------------------------------------------------------------
// English — static keys
// ---------------------------------------------------------------------------

fn t_en(key: &str) -> &'static str {
    match key {
        // Language selector (always bilingual)
        "lang.prompt" => "Language / 语言",

        // Steps
        "step.start" => "Start",
        "step.segments" => "1/4 Segments",
        "step.reorder" => "3/4 Reorder",
        "step.confirm" => "4/4 Confirm",

        // Mode selection
        "mode.prompt" => "Config mode",
        "mode.presets" => "Recommended presets",
        "mode.presetsHint" => "Pick a curated setup, ready to go",
        "mode.defaults" => "Use defaults",
        "mode.defaultsHint" => "Save directly, just press Enter",
        "mode.custom" => "Customize",
        "mode.customHint" => "Adjust styles and options step by step",
        "mode.existing" => "Use existing config",
        "mode.existingHint" => "Keep your current setup",
        "prompt.saveExisting" => "Re-apply existing config?",

        // Presets
        "preset.minimal" => "Minimal",
        "preset.minimalHint" => "Model + Cost + Context bar",
        "preset.developer" => "Developer",
        "preset.developerHint" => "Model + Cost + Path + Git + Context",
        "preset.dashboard" => "Full Dashboard",
        "preset.dashboardHint" => "Everything enabled, all the data",
        "preset.rainbow" => "Rainbow",
        "preset.rainbowHint" => "Maximum color, rainbow everywhere",
        "preset.crypto" => "Crypto",
        "preset.cryptoHint" => "Model + Cost + BTC/ETH/SOL + Context",

        // Segments
        "seg.model" => "Model",
        "seg.cost" => "Cost",
        "seg.usage" => "5h Limit",
        "seg.usage7d" => "7d Limit",
        "seg.path" => "Path",
        "seg.git" => "Git",
        "seg.context" => "Context",
        "seg.crypto" => "Crypto",

        // Sub-parts
        "part.bar" => "Progress bar",
        "part.percent" => "Percentage",
        "part.size" => "Capacity",
        "part.reset" => "Reset timer",
        "part.dirty" => "Dirty marker",
        "part.remote" => "Remote diff",
        "part.branch" => "branch",

        // Prompts
        "prompt.selectSegments" => "Select segments to enable",
        "prompt.showParts" => "Show which parts",
        "prompt.barStyle" => "Bar style",
        "prompt.barChar" => "Bar character",
        "prompt.barLength" => "Bar length",
        "prompt.textStyle" => "Text style",
        "prompt.style" => "Style",
        "prompt.selectCoins" => "Select coins (max 3)",
        "prompt.refreshInterval" => "Refresh interval",
        "prompt.maxLength" => "Max length (shows dir name if exceeded)",
        "prompt.saveDefaults" => "Save and apply defaults?",
        "prompt.save" => "Save and apply?",

        // Styles
        "style.rainbow" => "Ultrathink (Rainbow)",
        "style.cyan" => "Cyan",
        "style.green" => "Green",
        "style.blue" => "Blue",
        "style.yellow" => "Yellow",
        "style.magenta" => "Magenta",
        "style.red" => "Red",
        "style.white" => "White",
        "style.orange" => "Orange",
        "style.pink" => "Pink",
        "style.purple" => "Purple",

        // Bar styles
        "barStyle.gradient" => "Ultrathink Gradient (Rainbow)",
        "barStyle.trafficLight" => "Traffic light (G→Y→R)",
        "barStyle.trafficLightHint" => "\u{2264}30% green / \u{2264}60% yellow / >60% red",

        // Bar characters
        "char.shade" => "▓░ Shade",
        "char.fullBlock" => "█░ Full block",
        "char.rectangle" => "▬ Rectangle",

        // Units
        "unit.chars" => "chars",
        "unit.seconds" => "s",

        // Messages
        "msg.preview" => "Preview:",
        "msg.cancelled" => "Cancelled, config not saved",
        "msg.cancelledShort" => "Cancelled",
        "msg.saving" => "Saving config...",
        "msg.saved" => "Config saved",
        "msg.saveFailed" => "Save failed",
        "msg.restart" => "Restart Claude Code or wait for next refresh",
        "msg.unknownError" => "Unknown error",
        "msg.noClaudeCode" => "Claude Code not detected (~/.claude does not exist)",
        "msg.installClaudeCode" => "Please install: https://claude.ai/code",

        // Fallback: return the key itself
        _ => key_to_static(key),
    }
}

// ---------------------------------------------------------------------------
// Chinese — static keys
// ---------------------------------------------------------------------------

fn t_zh(key: &str) -> &'static str {
    match key {
        "lang.prompt" => "Language / 语言",

        "step.start" => "开始",
        "step.segments" => "1/4 选择段落",
        "step.reorder" => "3/4 排列顺序",
        "step.confirm" => "4/4 确认",

        "mode.prompt" => "配置模式",
        "mode.presets" => "推荐预设",
        "mode.presetsHint" => "精选配置方案，即选即用",
        "mode.defaults" => "使用默认配置",
        "mode.defaultsHint" => "直接保存，按回车即可",
        "mode.custom" => "自定义配置",
        "mode.customHint" => "逐项调整样式和选项",
        "mode.existing" => "使用已有配置",
        "mode.existingHint" => "保持当前配置不变",
        "prompt.saveExisting" => "重新应用已有配置？",

        "preset.minimal" => "极简",
        "preset.minimalHint" => "模型 + 费用 + 上下文进度条",
        "preset.developer" => "开发者",
        "preset.developerHint" => "模型 + 费用 + 目录 + Git + 上下文",
        "preset.dashboard" => "全部仪表盘",
        "preset.dashboardHint" => "全部段落开启，信息最大化",
        "preset.rainbow" => "彩虹",
        "preset.rainbowHint" => "全彩虹渐变，满屏闪烁",
        "preset.crypto" => "币圈",
        "preset.cryptoHint" => "模型 + 费用 + 三大币价 + 上下文",

        "seg.model" => "模型",
        "seg.cost" => "费用",
        "seg.usage" => "5h限额",
        "seg.usage7d" => "7d限额",
        "seg.path" => "目录",
        "seg.git" => "Git状态",
        "seg.context" => "上下文",
        "seg.crypto" => "加密货币",

        "part.bar" => "进度条",
        "part.percent" => "百分比",
        "part.size" => "容量",
        "part.reset" => "重置倒计时",
        "part.dirty" => "脏状态标记",
        "part.remote" => "远程差异",
        "part.branch" => "分支",

        "prompt.selectSegments" => "选择要启用的段落",
        "prompt.showParts" => "显示哪些部分",
        "prompt.barStyle" => "进度条样式",
        "prompt.barChar" => "进度条字符",
        "prompt.barLength" => "进度条长度",
        "prompt.textStyle" => "文字样式",
        "prompt.style" => "样式",
        "prompt.selectCoins" => "选择币种 (最多3个)",
        "prompt.refreshInterval" => "刷新间隔",
        "prompt.maxLength" => "最大长度 (超出只显示目录名)",
        "prompt.saveDefaults" => "保存并应用默认配置？",
        "prompt.save" => "保存并应用？",

        "style.rainbow" => "Ultrathink (彩虹)",
        "style.cyan" => "Cyan",
        "style.green" => "Green",
        "style.blue" => "Blue",
        "style.yellow" => "Yellow",
        "style.magenta" => "Magenta",
        "style.red" => "Red",
        "style.white" => "White",
        "style.orange" => "Orange",
        "style.pink" => "Pink",
        "style.purple" => "Purple",

        "barStyle.gradient" => "Ultrathink 渐变 (彩虹)",
        "barStyle.trafficLight" => "红绿灯 (绿→黄→红)",
        "barStyle.trafficLightHint" => "\u{2264}30% 绿 / \u{2264}60% 黄 / >60% 红",

        "char.shade" => "▓░ 阴影",
        "char.fullBlock" => "█░ 全块",
        "char.rectangle" => "▬ 水平矩形",

        "unit.chars" => "字符",
        "unit.seconds" => "秒",

        "msg.preview" => "预览:",
        "msg.cancelled" => "已取消，配置未保存",
        "msg.cancelledShort" => "已取消",
        "msg.saving" => "保存配置...",
        "msg.saved" => "配置已保存",
        "msg.saveFailed" => "保存失败",
        "msg.restart" => "重启 Claude Code 或等待下次刷新即可生效",
        "msg.unknownError" => "未知错误",
        "msg.noClaudeCode" => "未检测到 Claude Code (~/.claude 不存在)",
        "msg.installClaudeCode" => "请先安装: https://claude.ai/code",

        _ => key_to_static(key),
    }
}

// ---------------------------------------------------------------------------
// Japanese — static keys
// ---------------------------------------------------------------------------

fn t_ja(key: &str) -> &'static str {
    match key {
        "lang.prompt" => "Language / 言語",

        "step.start" => "開始",
        "step.segments" => "1/4 セグメント選択",
        "step.reorder" => "3/4 並べ替え",
        "step.confirm" => "4/4 確認",

        "mode.prompt" => "設定モード",
        "mode.presets" => "おすすめプリセット",
        "mode.presetsHint" => "厳選された設定、すぐ使える",
        "mode.defaults" => "デフォルトを使用",
        "mode.defaultsHint" => "そのまま保存、Enterを押すだけ",
        "mode.custom" => "カスタマイズ",
        "mode.customHint" => "スタイルとオプションを段階的に調整",
        "mode.existing" => "既存の設定を使用",
        "mode.existingHint" => "現在の設定をそのまま維持",
        "prompt.saveExisting" => "既存の設定を再適用しますか？",

        "preset.minimal" => "ミニマル",
        "preset.minimalHint" => "モデル + コスト + コンテキストバー",
        "preset.developer" => "デベロッパー",
        "preset.developerHint" => "モデル + コスト + パス + Git + コンテキスト",
        "preset.dashboard" => "フルダッシュボード",
        "preset.dashboardHint" => "全セグメント有効、最大情報量",
        "preset.rainbow" => "レインボー",
        "preset.rainbowHint" => "全色カラー、レインボー全開",
        "preset.crypto" => "クリプト",
        "preset.cryptoHint" => "モデル + コスト + BTC/ETH/SOL + コンテキスト",

        "seg.model" => "モデル",
        "seg.cost" => "コスト",
        "seg.usage" => "5h制限",
        "seg.usage7d" => "7d制限",
        "seg.path" => "パス",
        "seg.git" => "Git",
        "seg.context" => "コンテキスト",
        "seg.crypto" => "暗号通貨",

        "part.bar" => "プログレスバー",
        "part.percent" => "パーセント",
        "part.size" => "容量",
        "part.reset" => "リセットタイマー",
        "part.dirty" => "変更マーカー",
        "part.remote" => "リモート差分",
        "part.branch" => "ブランチ",

        "prompt.selectSegments" => "有効にするセグメントを選択",
        "prompt.showParts" => "表示する項目",
        "prompt.barStyle" => "バースタイル",
        "prompt.barChar" => "バー文字",
        "prompt.barLength" => "バーの長さ",
        "prompt.textStyle" => "テキストスタイル",
        "prompt.style" => "スタイル",
        "prompt.selectCoins" => "コインを選択 (最大3)",
        "prompt.refreshInterval" => "更新間隔",
        "prompt.maxLength" => "最大長 (超過時はディレクトリ名のみ表示)",
        "prompt.saveDefaults" => "デフォルト設定を保存して適用しますか？",
        "prompt.save" => "保存して適用しますか？",

        "style.rainbow" => "Ultrathink (レインボー)",
        "style.cyan" => "Cyan",
        "style.green" => "Green",
        "style.blue" => "Blue",
        "style.yellow" => "Yellow",
        "style.magenta" => "Magenta",
        "style.red" => "Red",
        "style.white" => "White",
        "style.orange" => "Orange",
        "style.pink" => "Pink",
        "style.purple" => "Purple",

        "barStyle.gradient" => "Ultrathink グラデーション (レインボー)",
        "barStyle.trafficLight" => "信号灯 (緑→黄→赤)",
        "barStyle.trafficLightHint" => "\u{2264}30% 緑 / \u{2264}60% 黄 / >60% 赤",

        "char.shade" => "▓░ Shade",
        "char.fullBlock" => "█░ Full block",
        "char.rectangle" => "▬ Rectangle",

        "unit.chars" => "文字",
        "unit.seconds" => "秒",

        "msg.preview" => "プレビュー:",
        "msg.cancelled" => "キャンセルしました。設定は保存されていません",
        "msg.cancelledShort" => "キャンセル",
        "msg.saving" => "設定を保存中...",
        "msg.saved" => "設定を保存しました",
        "msg.saveFailed" => "保存に失敗しました",
        "msg.restart" => "Claude Codeを再起動するか次の更新をお待ちください",
        "msg.unknownError" => "不明なエラー",
        "msg.noClaudeCode" => "Claude Codeが検出されません (~/.claude が存在しません)",
        "msg.installClaudeCode" => "インストール: https://claude.ai/code",

        _ => key_to_static(key),
    }
}

// ---------------------------------------------------------------------------
// Korean — static keys
// ---------------------------------------------------------------------------

fn t_ko(key: &str) -> &'static str {
    match key {
        "lang.prompt" => "Language / 언어",

        "step.start" => "시작",
        "step.segments" => "1/4 세그먼트 선택",
        "step.reorder" => "3/4 순서 변경",
        "step.confirm" => "4/4 확인",

        "mode.prompt" => "설정 모드",
        "mode.presets" => "추천 프리셋",
        "mode.presetsHint" => "엄선된 설정, 바로 사용",
        "mode.defaults" => "기본값 사용",
        "mode.defaultsHint" => "바로 저장, Enter만 누르세요",
        "mode.custom" => "사용자 정의",
        "mode.customHint" => "스타일과 옵션을 단계별로 조정",
        "mode.existing" => "기존 설정 사용",
        "mode.existingHint" => "현재 설정 유지",
        "prompt.saveExisting" => "기존 설정을 다시 적용하시겠습니까?",

        "preset.minimal" => "미니멀",
        "preset.minimalHint" => "모델 + 비용 + 컨텍스트 바",
        "preset.developer" => "개발자",
        "preset.developerHint" => "모델 + 비용 + 경로 + Git + 컨텍스트",
        "preset.dashboard" => "풀 대시보드",
        "preset.dashboardHint" => "전체 세그먼트 활성화",
        "preset.rainbow" => "레인보우",
        "preset.rainbowHint" => "풀 컬러, 레인보우 전개",
        "preset.crypto" => "크립토",
        "preset.cryptoHint" => "모델 + 비용 + BTC/ETH/SOL + 컨텍스트",

        "seg.model" => "모델",
        "seg.cost" => "비용",
        "seg.usage" => "5h 한도",
        "seg.usage7d" => "7d 한도",
        "seg.path" => "경로",
        "seg.git" => "Git",
        "seg.context" => "컨텍스트",
        "seg.crypto" => "암호화폐",

        "part.bar" => "진행률 바",
        "part.percent" => "퍼센트",
        "part.size" => "용량",
        "part.reset" => "리셋 타이머",
        "part.dirty" => "변경 표시",
        "part.remote" => "원격 차이",
        "part.branch" => "브랜치",

        "prompt.selectSegments" => "활성화할 세그먼트 선택",
        "prompt.showParts" => "표시할 항목",
        "prompt.barStyle" => "바 스타일",
        "prompt.barChar" => "바 문자",
        "prompt.barLength" => "바 길이",
        "prompt.textStyle" => "텍스트 스타일",
        "prompt.style" => "스타일",
        "prompt.selectCoins" => "코인 선택 (최대 3개)",
        "prompt.refreshInterval" => "새로고침 간격",
        "prompt.maxLength" => "최대 길이 (초과시 디렉토리 이름만 표시)",
        "prompt.saveDefaults" => "기본 설정을 저장하고 적용하시겠습니까?",
        "prompt.save" => "저장하고 적용하시겠습니까?",

        "style.rainbow" => "Ultrathink (레인보우)",
        "style.cyan" => "Cyan",
        "style.green" => "Green",
        "style.blue" => "Blue",
        "style.yellow" => "Yellow",
        "style.magenta" => "Magenta",
        "style.red" => "Red",
        "style.white" => "White",
        "style.orange" => "Orange",
        "style.pink" => "Pink",
        "style.purple" => "Purple",

        "barStyle.gradient" => "Ultrathink 그라데이션 (레인보우)",
        "barStyle.trafficLight" => "신호등 (초록→노랑→빨강)",
        "barStyle.trafficLightHint" => "\u{2264}30% 초록 / \u{2264}60% 노랑 / >60% 빨강",

        "char.shade" => "▓░ Shade",
        "char.fullBlock" => "█░ Full block",
        "char.rectangle" => "▬ Rectangle",

        "unit.chars" => "자",
        "unit.seconds" => "초",

        "msg.preview" => "미리보기:",
        "msg.cancelled" => "취소되었습니다. 설정이 저장되지 않았습니다",
        "msg.cancelledShort" => "취소됨",
        "msg.saving" => "설정 저장 중...",
        "msg.saved" => "설정이 저장되었습니다",
        "msg.saveFailed" => "저장 실패",
        "msg.restart" => "Claude Code를 재시작하거나 다음 새로고침을 기다리세요",
        "msg.unknownError" => "알 수 없는 오류",
        "msg.noClaudeCode" => "Claude Code가 감지되지 않습니다 (~/.claude가 존재하지 않습니다)",
        "msg.installClaudeCode" => "설치: https://claude.ai/code",

        _ => key_to_static(key),
    }
}

// ---------------------------------------------------------------------------
// Spanish — static keys
// ---------------------------------------------------------------------------

fn t_es(key: &str) -> &'static str {
    match key {
        "lang.prompt" => "Language / Idioma",

        "step.start" => "Inicio",
        "step.segments" => "1/4 Segmentos",
        "step.reorder" => "3/4 Reordenar",
        "step.confirm" => "4/4 Confirmar",

        "mode.prompt" => "Modo de configuración",
        "mode.presets" => "Presets recomendados",
        "mode.presetsHint" => "Configuraciones seleccionadas, listas para usar",
        "mode.defaults" => "Usar valores predeterminados",
        "mode.defaultsHint" => "Guardar directamente, solo presiona Enter",
        "mode.custom" => "Personalizar",
        "mode.customHint" => "Ajustar estilos y opciones paso a paso",
        "mode.existing" => "Usar configuración existente",
        "mode.existingHint" => "Mantener tu configuración actual",
        "prompt.saveExisting" => "¿Reaplicar la configuración existente?",

        "preset.minimal" => "Minimalista",
        "preset.minimalHint" => "Modelo + Costo + Barra de contexto",
        "preset.developer" => "Desarrollador",
        "preset.developerHint" => "Modelo + Costo + Ruta + Git + Contexto",
        "preset.dashboard" => "Panel completo",
        "preset.dashboardHint" => "Todo habilitado, máxima información",
        "preset.rainbow" => "Arcoíris",
        "preset.rainbowHint" => "Color máximo, arcoíris por todas partes",
        "preset.crypto" => "Cripto",
        "preset.cryptoHint" => "Modelo + Costo + BTC/ETH/SOL + Contexto",

        "seg.model" => "Modelo",
        "seg.cost" => "Costo",
        "seg.usage" => "Límite 5h",
        "seg.usage7d" => "Límite 7d",
        "seg.path" => "Ruta",
        "seg.git" => "Git",
        "seg.context" => "Contexto",
        "seg.crypto" => "Criptomonedas",

        "part.bar" => "Barra de progreso",
        "part.percent" => "Porcentaje",
        "part.size" => "Capacidad",
        "part.reset" => "Temporizador de reinicio",
        "part.dirty" => "Marcador de cambios",
        "part.remote" => "Diferencia remota",
        "part.branch" => "rama",

        "prompt.selectSegments" => "Seleccionar segmentos a habilitar",
        "prompt.showParts" => "Mostrar qué partes",
        "prompt.barStyle" => "Estilo de barra",
        "prompt.barChar" => "Carácter de barra",
        "prompt.barLength" => "Longitud de barra",
        "prompt.textStyle" => "Estilo de texto",
        "prompt.style" => "Estilo",
        "prompt.selectCoins" => "Seleccionar monedas (máx. 3)",
        "prompt.refreshInterval" => "Intervalo de actualización",
        "prompt.maxLength" => "Longitud máxima (muestra solo nombre del directorio si se excede)",
        "prompt.saveDefaults" => "¿Guardar y aplicar valores predeterminados?",
        "prompt.save" => "¿Guardar y aplicar?",

        "style.rainbow" => "Ultrathink (Arcoíris)",
        "style.cyan" => "Cyan",
        "style.green" => "Green",
        "style.blue" => "Blue",
        "style.yellow" => "Yellow",
        "style.magenta" => "Magenta",
        "style.red" => "Red",
        "style.white" => "White",
        "style.orange" => "Orange",
        "style.pink" => "Pink",
        "style.purple" => "Purple",

        "barStyle.gradient" => "Ultrathink Degradado (Arcoíris)",
        "barStyle.trafficLight" => "Semáforo (V→A→R)",
        "barStyle.trafficLightHint" => "\u{2264}30% verde / \u{2264}60% amarillo / >60% rojo",

        "char.shade" => "▓░ Shade",
        "char.fullBlock" => "█░ Full block",
        "char.rectangle" => "▬ Rectangle",

        "unit.chars" => "car.",
        "unit.seconds" => "s",

        "msg.preview" => "Vista previa:",
        "msg.cancelled" => "Cancelado, configuración no guardada",
        "msg.cancelledShort" => "Cancelado",
        "msg.saving" => "Guardando configuración...",
        "msg.saved" => "Configuración guardada",
        "msg.saveFailed" => "Error al guardar",
        "msg.restart" => "Reinicia Claude Code o espera la próxima actualización",
        "msg.unknownError" => "Error desconocido",
        "msg.noClaudeCode" => "Claude Code no detectado (~/.claude no existe)",
        "msg.installClaudeCode" => "Instalar: https://claude.ai/code",

        _ => key_to_static(key),
    }
}

// ---------------------------------------------------------------------------
// Portuguese — static keys
// ---------------------------------------------------------------------------

fn t_pt(key: &str) -> &'static str {
    match key {
        "lang.prompt" => "Language / Idioma",

        "step.start" => "Início",
        "step.segments" => "1/4 Segmentos",
        "step.reorder" => "3/4 Reordenar",
        "step.confirm" => "4/4 Confirmar",

        "mode.prompt" => "Modo de configuração",
        "mode.presets" => "Presets recomendados",
        "mode.presetsHint" => "Configurações selecionadas, prontas para usar",
        "mode.defaults" => "Usar padrões",
        "mode.defaultsHint" => "Salvar diretamente, apenas pressione Enter",
        "mode.custom" => "Personalizar",
        "mode.customHint" => "Ajustar estilos e opções passo a passo",
        "mode.existing" => "Usar configuração existente",
        "mode.existingHint" => "Manter sua configuração atual",
        "prompt.saveExisting" => "Reaplicar a configuração existente?",

        "preset.minimal" => "Minimalista",
        "preset.minimalHint" => "Modelo + Custo + Barra de contexto",
        "preset.developer" => "Desenvolvedor",
        "preset.developerHint" => "Modelo + Custo + Caminho + Git + Contexto",
        "preset.dashboard" => "Painel completo",
        "preset.dashboardHint" => "Tudo habilitado, máxima informação",
        "preset.rainbow" => "Arco-íris",
        "preset.rainbowHint" => "Cor máxima, arco-íris em todo lugar",
        "preset.crypto" => "Cripto",
        "preset.cryptoHint" => "Modelo + Custo + BTC/ETH/SOL + Contexto",

        "seg.model" => "Modelo",
        "seg.cost" => "Custo",
        "seg.usage" => "Limite 5h",
        "seg.usage7d" => "Limite 7d",
        "seg.path" => "Caminho",
        "seg.git" => "Git",
        "seg.context" => "Contexto",
        "seg.crypto" => "Criptomoedas",

        "part.bar" => "Barra de progresso",
        "part.percent" => "Porcentagem",
        "part.size" => "Capacidade",
        "part.reset" => "Temporizador de reinício",
        "part.dirty" => "Marcador de alterações",
        "part.remote" => "Diferença remota",
        "part.branch" => "branch",

        "prompt.selectSegments" => "Selecionar segmentos para ativar",
        "prompt.showParts" => "Mostrar quais partes",
        "prompt.barStyle" => "Estilo da barra",
        "prompt.barChar" => "Caractere da barra",
        "prompt.barLength" => "Comprimento da barra",
        "prompt.textStyle" => "Estilo do texto",
        "prompt.style" => "Estilo",
        "prompt.selectCoins" => "Selecionar moedas (máx. 3)",
        "prompt.refreshInterval" => "Intervalo de atualização",
        "prompt.maxLength" => "Comprimento máximo (mostra apenas nome do diretório se excedido)",
        "prompt.saveDefaults" => "Salvar e aplicar padrões?",
        "prompt.save" => "Salvar e aplicar?",

        "style.rainbow" => "Ultrathink (Arco-íris)",
        "style.cyan" => "Cyan",
        "style.green" => "Green",
        "style.blue" => "Blue",
        "style.yellow" => "Yellow",
        "style.magenta" => "Magenta",
        "style.red" => "Red",
        "style.white" => "White",
        "style.orange" => "Orange",
        "style.pink" => "Pink",
        "style.purple" => "Purple",

        "barStyle.gradient" => "Ultrathink Gradiente (Arco-íris)",
        "barStyle.trafficLight" => "Semáforo (V→A→V)",
        "barStyle.trafficLightHint" => "\u{2264}30% verde / \u{2264}60% amarelo / >60% vermelho",

        "char.shade" => "▓░ Shade",
        "char.fullBlock" => "█░ Full block",
        "char.rectangle" => "▬ Rectangle",

        "unit.chars" => "car.",
        "unit.seconds" => "s",

        "msg.preview" => "Prévia:",
        "msg.cancelled" => "Cancelado, configuração não salva",
        "msg.cancelledShort" => "Cancelado",
        "msg.saving" => "Salvando configuração...",
        "msg.saved" => "Configuração salva",
        "msg.saveFailed" => "Falha ao salvar",
        "msg.restart" => "Reinicie o Claude Code ou aguarde a próxima atualização",
        "msg.unknownError" => "Erro desconhecido",
        "msg.noClaudeCode" => "Claude Code não detectado (~/.claude não existe)",
        "msg.installClaudeCode" => "Instalar: https://claude.ai/code",

        _ => key_to_static(key),
    }
}

// ---------------------------------------------------------------------------
// Russian — static keys
// ---------------------------------------------------------------------------

fn t_ru(key: &str) -> &'static str {
    match key {
        "lang.prompt" => "Language / Язык",

        "step.start" => "Начало",
        "step.segments" => "1/4 Сегменты",
        "step.reorder" => "3/4 Порядок",
        "step.confirm" => "4/4 Подтверждение",

        "mode.prompt" => "Режим настройки",
        "mode.presets" => "Рекомендуемые пресеты",
        "mode.presetsHint" => "Готовые конфигурации на выбор",
        "mode.defaults" => "Использовать по умолчанию",
        "mode.defaultsHint" => "Сохранить сразу, просто нажмите Enter",
        "mode.custom" => "Настроить",
        "mode.customHint" => "Настроить стили и параметры пошагово",
        "mode.existing" => "Использовать существующие настройки",
        "mode.existingHint" => "Сохранить текущую конфигурацию",
        "prompt.saveExisting" => "Применить существующие настройки заново?",

        "preset.minimal" => "Минимум",
        "preset.minimalHint" => "Модель + Стоимость + Контекст",
        "preset.developer" => "Разработчик",
        "preset.developerHint" => "Модель + Стоимость + Путь + Git + Контекст",
        "preset.dashboard" => "Полная панель",
        "preset.dashboardHint" => "Все сегменты включены",
        "preset.rainbow" => "Радуга",
        "preset.rainbowHint" => "Максимум цвета, радуга везде",
        "preset.crypto" => "Крипто",
        "preset.cryptoHint" => "Модель + Стоимость + BTC/ETH/SOL + Контекст",

        "seg.model" => "Модель",
        "seg.cost" => "Стоимость",
        "seg.usage" => "Лимит 5ч",
        "seg.usage7d" => "Лимит 7д",
        "seg.path" => "Путь",
        "seg.git" => "Git",
        "seg.context" => "Контекст",
        "seg.crypto" => "Криптовалюта",

        "part.bar" => "Прогресс-бар",
        "part.percent" => "Процент",
        "part.size" => "Объём",
        "part.reset" => "Таймер сброса",
        "part.dirty" => "Маркер изменений",
        "part.remote" => "Разница с удалённым",
        "part.branch" => "ветка",

        "prompt.selectSegments" => "Выберите сегменты для включения",
        "prompt.showParts" => "Какие части показывать",
        "prompt.barStyle" => "Стиль бара",
        "prompt.barChar" => "Символ бара",
        "prompt.barLength" => "Длина бара",
        "prompt.textStyle" => "Стиль текста",
        "prompt.style" => "Стиль",
        "prompt.selectCoins" => "Выберите монеты (макс. 3)",
        "prompt.refreshInterval" => "Интервал обновления",
        "prompt.maxLength" => "Макс. длина (при превышении показывается только имя каталога)",
        "prompt.saveDefaults" => "Сохранить и применить настройки по умолчанию?",
        "prompt.save" => "Сохранить и применить?",

        "style.rainbow" => "Ultrathink (Радуга)",
        "style.cyan" => "Cyan",
        "style.green" => "Green",
        "style.blue" => "Blue",
        "style.yellow" => "Yellow",
        "style.magenta" => "Magenta",
        "style.red" => "Red",
        "style.white" => "White",
        "style.orange" => "Orange",
        "style.pink" => "Pink",
        "style.purple" => "Purple",

        "barStyle.gradient" => "Ultrathink Градиент (Радуга)",
        "barStyle.trafficLight" => "Светофор (З→Ж→К)",
        "barStyle.trafficLightHint" => "\u{2264}30% зелёный / \u{2264}60% жёлтый / >60% красный",

        "char.shade" => "▓░ Shade",
        "char.fullBlock" => "█░ Full block",
        "char.rectangle" => "▬ Rectangle",

        "unit.chars" => "симв.",
        "unit.seconds" => "с",

        "msg.preview" => "Предпросмотр:",
        "msg.cancelled" => "Отменено, настройки не сохранены",
        "msg.cancelledShort" => "Отменено",
        "msg.saving" => "Сохранение настроек...",
        "msg.saved" => "Настройки сохранены",
        "msg.saveFailed" => "Ошибка сохранения",
        "msg.restart" => "Перезапустите Claude Code или дождитесь следующего обновления",
        "msg.unknownError" => "Неизвестная ошибка",
        "msg.noClaudeCode" => "Claude Code не обнаружен (~/.claude не существует)",
        "msg.installClaudeCode" => "Установить: https://claude.ai/code",

        _ => key_to_static(key),
    }
}

// ---------------------------------------------------------------------------
// English — dynamic (template) keys
// ---------------------------------------------------------------------------

fn tf_en(key: &str) -> Option<&'static str> {
    match key {
        "step.configSegment" => Some("2/4 Configure ({0}/{1})"),
        "step.reorderN" => Some("3/4 Reorder ({0}/{1})"),
        "msg.missingDeps" => Some("Missing dependencies: {0}"),
        "msg.installDeps" => Some("Please install: brew install {0}"),
        "prompt.currentOrder" => Some("Current order: {0}, adjust?"),
        "prompt.pickN" => Some("Position {0}"),
        "done.segments" => Some("Segments: {0}"),
        "done.coins" => Some("Coins: {0}"),
        "done.refresh" => Some("Refresh: {0}s"),
        "done.length" => Some("Length: {0}"),
        "done.text" => Some("Text: {0}"),
        "done.order" => Some("Order: {0}"),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Chinese — dynamic (template) keys
// ---------------------------------------------------------------------------

fn tf_zh(key: &str) -> Option<&'static str> {
    match key {
        "step.configSegment" => Some("2/4 配置段落 ({0}/{1})"),
        "step.reorderN" => Some("3/4 排列顺序 ({0}/{1})"),
        "msg.missingDeps" => Some("缺少必要依赖: {0}"),
        "msg.installDeps" => Some("请先安装: brew install {0}"),
        "prompt.currentOrder" => Some("当前顺序: {0}，要调整吗？"),
        "prompt.pickN" => Some("第 {0} 个段落"),
        "done.segments" => Some("段落: {0}"),
        "done.coins" => Some("币种={0}"),
        "done.refresh" => Some("刷新={0}s"),
        "done.length" => Some("长度={0}"),
        "done.text" => Some("文字={0}"),
        "done.order" => Some("顺序: {0}"),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Japanese — dynamic (template) keys
// ---------------------------------------------------------------------------

fn tf_ja(key: &str) -> Option<&'static str> {
    match key {
        "step.configSegment" => Some("2/4 設定 ({0}/{1})"),
        "step.reorderN" => Some("3/4 並べ替え ({0}/{1})"),
        "msg.missingDeps" => Some("依存関係が不足: {0}"),
        "msg.installDeps" => Some("インストール: brew install {0}"),
        "prompt.currentOrder" => Some("現在の順序: {0}、変更しますか？"),
        "prompt.pickN" => Some("{0}番目を選択"),
        "done.segments" => Some("セグメント: {0}"),
        "done.coins" => Some("コイン: {0}"),
        "done.refresh" => Some("更新: {0}秒"),
        "done.length" => Some("長さ: {0}"),
        "done.text" => Some("テキスト: {0}"),
        "done.order" => Some("順序: {0}"),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Korean — dynamic (template) keys
// ---------------------------------------------------------------------------

fn tf_ko(key: &str) -> Option<&'static str> {
    match key {
        "step.configSegment" => Some("2/4 설정 ({0}/{1})"),
        "step.reorderN" => Some("3/4 순서 변경 ({0}/{1})"),
        "msg.missingDeps" => Some("누락된 의존성: {0}"),
        "msg.installDeps" => Some("설치: brew install {0}"),
        "prompt.currentOrder" => Some("현재 순서: {0}, 변경하시겠습니까?"),
        "prompt.pickN" => Some("{0}번째 선택"),
        "done.segments" => Some("세그먼트: {0}"),
        "done.coins" => Some("코인: {0}"),
        "done.refresh" => Some("새로고침: {0}초"),
        "done.length" => Some("길이: {0}"),
        "done.text" => Some("텍스트: {0}"),
        "done.order" => Some("순서: {0}"),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Spanish — dynamic (template) keys
// ---------------------------------------------------------------------------

fn tf_es(key: &str) -> Option<&'static str> {
    match key {
        "step.configSegment" => Some("2/4 Configurar ({0}/{1})"),
        "step.reorderN" => Some("3/4 Reordenar ({0}/{1})"),
        "msg.missingDeps" => Some("Dependencias faltantes: {0}"),
        "msg.installDeps" => Some("Instalar: brew install {0}"),
        "prompt.currentOrder" => Some("Orden actual: {0}, ¿ajustar?"),
        "prompt.pickN" => Some("Posición {0}"),
        "done.segments" => Some("Segmentos: {0}"),
        "done.coins" => Some("Monedas: {0}"),
        "done.refresh" => Some("Actualizar: {0}s"),
        "done.length" => Some("Longitud: {0}"),
        "done.text" => Some("Texto: {0}"),
        "done.order" => Some("Orden: {0}"),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Portuguese — dynamic (template) keys
// ---------------------------------------------------------------------------

fn tf_pt(key: &str) -> Option<&'static str> {
    match key {
        "step.configSegment" => Some("2/4 Configurar ({0}/{1})"),
        "step.reorderN" => Some("3/4 Reordenar ({0}/{1})"),
        "msg.missingDeps" => Some("Dependências ausentes: {0}"),
        "msg.installDeps" => Some("Instalar: brew install {0}"),
        "prompt.currentOrder" => Some("Ordem atual: {0}, ajustar?"),
        "prompt.pickN" => Some("Posição {0}"),
        "done.segments" => Some("Segmentos: {0}"),
        "done.coins" => Some("Moedas: {0}"),
        "done.refresh" => Some("Atualização: {0}s"),
        "done.length" => Some("Comprimento: {0}"),
        "done.text" => Some("Texto: {0}"),
        "done.order" => Some("Ordem: {0}"),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Russian — dynamic (template) keys
// ---------------------------------------------------------------------------

fn tf_ru(key: &str) -> Option<&'static str> {
    match key {
        "step.configSegment" => Some("2/4 Настройка ({0}/{1})"),
        "step.reorderN" => Some("3/4 Порядок ({0}/{1})"),
        "msg.missingDeps" => Some("Отсутствуют зависимости: {0}"),
        "msg.installDeps" => Some("Установить: brew install {0}"),
        "prompt.currentOrder" => Some("Текущий порядок: {0}, изменить?"),
        "prompt.pickN" => Some("Позиция {0}"),
        "done.segments" => Some("Сегменты: {0}"),
        "done.coins" => Some("Монеты: {0}"),
        "done.refresh" => Some("Обновление: {0}с"),
        "done.length" => Some("Длина: {0}"),
        "done.text" => Some("Текст: {0}"),
        "done.order" => Some("Порядок: {0}"),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Leak the key string to produce a &'static str fallback.
// This is intentional: unknown keys are rare, and the leaked memory is tiny.
// ---------------------------------------------------------------------------

fn key_to_static(key: &str) -> &'static str {
    Box::leak(key.to_string().into_boxed_str())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_lang_is_en() {
        set_lang("en");
        assert_eq!(get_lang(), "en");
    }

    #[test]
    fn test_set_lang_zh() {
        set_lang("zh");
        assert_eq!(get_lang(), "zh");
        set_lang("en");
    }

    #[test]
    fn test_set_lang_unknown_falls_back_to_en() {
        set_lang("fr");
        assert_eq!(get_lang(), "en");
        set_lang("en");
    }

    #[test]
    fn test_t_en() {
        set_lang("en");
        assert_eq!(t("step.start"), "Start");
        assert_eq!(t("seg.model"), "Model");
        assert_eq!(t("msg.saved"), "Config saved");
    }

    #[test]
    fn test_t_zh() {
        set_lang("zh");
        assert_eq!(t("step.start"), "开始");
        assert_eq!(t("seg.model"), "模型");
        set_lang("en");
    }

    #[test]
    fn test_t_unknown_key_returns_key() {
        set_lang("en");
        assert_eq!(t("nonexistent.key"), "nonexistent.key");
    }

    #[test]
    fn test_tf_en() {
        set_lang("en");
        assert_eq!(tf("step.configSegment", &["2", "4"]), "2/4 Configure (2/4)");
        assert_eq!(tf("done.refresh", &["30"]), "Refresh: 30s");
        assert_eq!(
            tf("msg.missingDeps", &["jq perl"]),
            "Missing dependencies: jq perl"
        );
    }

    #[test]
    fn test_tf_zh() {
        set_lang("zh");
        assert_eq!(
            tf("step.configSegment", &["2", "4"]),
            "2/4 配置段落 (2/4)"
        );
        assert_eq!(tf("done.refresh", &["30"]), "刷新=30s");
        set_lang("en");
    }

    #[test]
    fn test_tf_unknown_key_returns_key() {
        set_lang("en");
        assert_eq!(tf("nonexistent.key", &["arg"]), "nonexistent.key");
    }

    #[test]
    fn test_supported_langs() {
        assert_eq!(SUPPORTED_LANGS, &["en", "zh", "ja", "ko", "es", "pt", "ru"]);
    }

    #[test]
    fn test_lang_prompt_starts_with_language() {
        for &lang in SUPPORTED_LANGS {
            set_lang(lang);
            let prompt = t("lang.prompt");
            assert!(
                prompt.starts_with("Language"),
                "lang.prompt for '{}' should start with 'Language', got: {}",
                lang,
                prompt
            );
        }
        set_lang("en");
    }

    #[test]
    fn test_style_names_same_in_all() {
        let style_keys = &[
            "style.cyan",
            "style.green",
            "style.blue",
            "style.yellow",
            "style.magenta",
            "style.red",
            "style.white",
            "style.orange",
            "style.pink",
            "style.purple",
        ];
        set_lang("en");
        for name in style_keys {
            let en_val = t(name);
            for &lang in SUPPORTED_LANGS {
                set_lang(lang);
                let val = t(name);
                assert_eq!(
                    en_val, val,
                    "style key '{}' should be identical in en and {}", name, lang
                );
            }
        }
        set_lang("en");
    }

    #[test]
    fn test_t_ja() {
        set_lang("ja");
        assert_eq!(t("step.start"), "開始");
        assert_eq!(t("seg.model"), "モデル");
        set_lang("en");
    }

    #[test]
    fn test_tf_ja() {
        set_lang("ja");
        assert_eq!(tf("step.configSegment", &["2", "4"]), "2/4 設定 (2/4)");
        set_lang("en");
    }
}
