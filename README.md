# GrokSearch-rs（personal-compat）

> **本仓库**：[`ZJ-zhangcn/GrokSearch-rs`](https://github.com/ZJ-zhangcn/GrokSearch-rs)  
> **分支**：`personal-compat`  
> **npm**：[`@zj-zhangcn/grok-search-rs`](https://www.npmjs.com/package/@zj-zhangcn/grok-search-rs)（发版后）  
> **上游**：[`Episkey-G/GrokSearch-rs`](https://github.com/Episkey-G/GrokSearch-rs)  
> **Python 对照**：[`ZJ-zhangcn/GrokSearch@personal-responses`](https://github.com/ZJ-zhangcn/GrokSearch/tree/personal-responses)

轻量 **Rust MCP**：Grok 搜索 + Tavily fetch/map（可选 Firecrawl）。  
比 Python/uvx 更省内存、启动更快。

---

## 推荐：不 clone，一套 MCP 配置（npx 本 fork）

```bash
npx -y @zj-zhangcn/grok-search-rs@latest
```

> ⚠️ **不要**用上游 `grok-search-rs@latest`（卡在 0.1.17，无 personal-compat 补丁）。  
> 发包步骤见 [docs/PUBLISH.md](docs/PUBLISH.md)。发版前可先用本地 `target/release/grok-search-rs`。

### Hermes

```yaml
mcp_servers:
  grok-search:
    command: npx
    args:
      - -y
      - "@zj-zhangcn/grok-search-rs@latest"
    env:
      GROK_API_KEY: ${MCP_GROK_API_KEY}
      GROK_API_URL: ${MCP_GROK_API_URL}
      GROK_MODEL: ${MCP_GROK_MODEL}
      GROK_API_MODE: ${MCP_GROK_API_MODE}
      # grok-4.5 自带搜索；勿默认 true，否则易 400/429
      GROK_SEARCH_WEB_SEARCH: "false"
      TAVILY_API_KEY: ${MCP_TAVILY_API_KEY}
      TAVILY_API_URL: ${MCP_TAVILY_API_URL}
    timeout: 180
    connect_timeout: 120
```

`GROK_API_*` 与 Python fork 同名；也兼容 `GROK_SEARCH_*`。

### 通用 MCP JSON

```json
{
  "grok-search": {
    "command": "npx",
    "args": ["-y", "@zj-zhangcn/grok-search-rs@latest"],
    "env": {
      "GROK_API_KEY": "sk-...",
      "GROK_API_URL": "https://api.x.ai/v1",
      "GROK_MODEL": "grok-4.5",
      "GROK_SEARCH_WEB_SEARCH": "false",
      "TAVILY_API_KEY": "tvly-..."
    }
  }
}
```

---

## personal-compat 相对上游

| 项 | 说明 |
|---|---|
| **`GROK_API_*` 别名** | 与 Python GuDaStudio 命名一致；`GROK_SEARCH_*` 仍优先 |
| **`GROK_API_MODE`** | `auto`/`responses` → Responses；`chat` → Chat Completions |
| **今天/时间注入** | 「今天/today/最新…」自动附本机日期 |
| **默认不塞原生 web_search tool** | `grok-4.5` 自带搜索；再传 `tools:[{"type":"web_search"}]` 易 400/429 |

| Python / 本 fork | 上游原生 | 用途 |
|---|---|---|
| `GROK_API_KEY` | `GROK_SEARCH_API_KEY` | API Key |
| `GROK_API_URL` | `GROK_SEARCH_URL` | base（会规范到 `/v1`） |
| `GROK_MODEL` | `GROK_SEARCH_MODEL` | 模型名 |
| `GROK_API_MODE` | （无） | `auto`/`chat`/`responses` |

---

## 开发：本地 release（发版前 / 改代码）

```bash
git clone https://github.com/ZJ-zhangcn/GrokSearch-rs.git
cd GrokSearch-rs && git checkout personal-compat
cargo build --release
# 二进制：target/release/grok-search-rs
```

发 npm 多平台包：打 `v*` tag → Actions **Release**（需 secret `NPM_TOKEN`）。详见 [docs/PUBLISH.md](docs/PUBLISH.md)。

---

## 工具（5）

`web_search` · `get_sources` · `web_fetch` · `web_map` · `doctor`

上游精简面，无 Python 的 `plan_*` / `switch_model`。

---

## 环境变量摘要

见 [docs/CONFIGURATION.md](docs/CONFIGURATION.md)。布尔：`1`/`true`/`yes` = 开。

---

## 致谢

- 上游：[Episkey-G/GrokSearch-rs](https://github.com/Episkey-G/GrokSearch-rs)
- Python 生态：[GuDaStudio/GrokSearch](https://github.com/GuDaStudio/GrokSearch) / 本用户 `personal-responses` fork
