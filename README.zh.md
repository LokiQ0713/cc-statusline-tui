# cc-statusline

[![CI](https://github.com/LokiQ0713/cc-statusline/actions/workflows/ci.yml/badge.svg)](https://github.com/LokiQ0713/cc-statusline/actions/workflows/ci.yml)
[![Release](https://github.com/LokiQ0713/cc-statusline/actions/workflows/release.yml/badge.svg)](https://github.com/LokiQ0713/cc-statusline/actions/workflows/release.yml)
[![npm](https://img.shields.io/npm/v/cc-statusline)](https://www.npmjs.com/package/cc-statusline)

> Claude Code 的状态栏太素了？给它整点花活。

[English](README.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Español](README.es.md) | [Português](README.pt.md) | [Русский](README.ru.md)

![statusline preview](preview.png)

一行命令，七种语言，零后悔。

## 安装

选一个你喜欢的：

### npm

```bash
npx cc-statusline
```

### Homebrew

```bash
brew tap LokiQ0713/cc-statusline
brew install cc-statusline
```

## 它干了啥

弹出一个 TUI 向导，你选一选，状态栏就好看了。就这么简单。

```
1/4 选择段落      → 想看啥
2/4 配置段落      → 想咋看（实时预览，所见即所得）
3/4 排列顺序      → 放哪里
4/4 确认          → 搞定收工
```

懒得折腾？选「使用默认配置」然后按两下回车。我们不评判。

## 能配什么

| 段落 | 长啥样 | 干啥的 | 可以调 |
|------|--------|--------|--------|
| 模型 | `🔥 Opus4.6` | 你正在烧哪个大脑 | 图标（🔥🦊🐢🐰）、颜色 |
| 费用 | `$0.42` | 你钱包的痛苦指数 | 颜色 |
| 5h限额 | `██░░ 25% 1h43m` | 限额进度条 + 百分比 + 重置倒计时 | 进度条样式/字符/长度、颜色、刷新间隔 |
| 目录 | `~/project` | 你在哪 | 颜色、最大长度 |
| Git | `main* ↑2↓1` | 分支 + 脏状态 + 远程差异 | 子组件、颜色 |
| 上下文 | `▓▓▓▓░░░ 60% 600K/1M` | 上下文窗口吃了多少 | 进度条样式/字符/长度、颜色 |
| 加密货币 | `BTC:$73748` | 写代码的时候顺便看看币价，懂的都懂 | 币种（BTC/ETH/BNB/SOL）、颜色、刷新 |

## 颜色风格

| 风格 | 氛围 |
|------|------|
| **Ultrathink 彩虹** | 每个字符颜色都不一样，还会闪。是的，就是这么骚。 |
| **Ultrathink 渐变** | 进度条上的平滑彩虹渐变。厨师之吻。 |
| **红绿灯** | 绿（≤30%）→ 黄（≤60%）→ 红（>60%）。你的上下文窗口变成了交通灯。 |
| Cyan / Green / Blue / Yellow | 稳重之选 |
| Magenta / Red / Orange / Pink / Purple / White | 张扬之选 |

## 多语言

English、中文、日本語、한국어、Español、Português、Русский。

首次运行时选择，状态栏说你的语言。

## 工作原理

1. 向导保存配置到 `~/.claude/statusline/config.json`
2. 二进制文件放到 `~/.claude/statusline/bin/`
3. 自动更新 `~/.claude/settings.json`
4. 重启 Claude Code，欣赏你的新状态栏，跟同事炫耀一下

再次运行会读取已有配置作为默认值。不会覆盖你的心血。放心。

## 前提

- Claude Code 已安装（`~/.claude/` 存在）
- Node.js ≥ 18（仅 `npx` 安装时需要，运行时不需要）

## 安全与隐私

你的数据我们不碰，放一百个心：

- **加密货币段落**请求的是 Binance 公开 API（`api.binance.com`）——不需要登录，不发送任何个人数据
- **5h限额段落**从 macOS 钥匙串读取 OAuth 令牌（`Claude Code-credentials`）来查询 Anthropic 的使用量 API
- **npm postinstall** 从 GitHub Releases 下载平台对应的二进制文件——除此之外不会下载任何东西
- 所有数据缓存在本地 `/tmp/claude-statusline-*` 文件里
- 没有遥测，没有分析，不往任何地方发数据
- 详情请看 [SECURITY.md](SECURITY.md)

## 卸载

不想用了？干干净净帮你删：

```bash
# 删除配置和二进制文件
rm -rf ~/.claude/statusline/

# 从 Claude Code 设置中移除状态栏
# 编辑 ~/.claude/settings.json，删掉 "statusLine" 那一行

# 清理缓存
rm -f /tmp/claude-statusline-*

# 从包管理器卸载
npm uninstall -g cc-statusline
# 或者：brew uninstall cc-statusline
```

## 常见问题

| 问题 | 解决办法 |
|------|---------|
| "Binary not found" | 重新跑一遍 `npx cc-statusline` 重新下载 |
| "Is a directory" 错误 | 检查 `~/.claude/statusline/bin/cc-statusline` 是文件不是目录 |
| 加密货币不显示 | 删掉 `/tmp/claude-statusline-crypto-lock` 目录（可能是过期的锁） |
| 改了配置没生效 | 保存后重启 Claude Code |

## 贡献

发现 bug？想要新功能？[提 Issue](https://github.com/LokiQ0713/cc-statusline/issues)。PR 欢迎。

## 许可

MIT
