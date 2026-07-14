# GrokSearch-rs（personal-compat）

> **本仓库**：[`ZJ-zhangcn/GrokSearch-rs`](https://github.com/ZJ-zhangcn/GrokSearch-rs)  
> **分支**：`personal-compat`  
> **上游**：[`Episkey-G/GrokSearch-rs`](https://github.com/Episkey-G/GrokSearch-rs)  
> **Python 对照**：[`ZJ-zhangcn/GrokSearch@personal-responses`](https://github.com/ZJ-zhangcn/GrokSearch/tree/personal-responses)

轻量 **Rust MCP**：Grok 搜索 + Tavily fetch/map（可选 Firecrawl）。  
比 Python/uvx 更省内存、启动更快。

---

## 推荐：不 clone，一套 MCP 配置（npx）

Rust 不能像 Python 那样 `uvx --from git+https://...` 即拉即跑。  
**零本地源码**的等价做法是用上游 **npm 预编译二进制**（`npx -y`，无需 `npm i -g`、无需 clone/cargo）：

### Hermes

```yaml
mcp_servers:
  grok-search:
    command: npx
    args:
      - -y
      - grok-search-rs@latest
    env:
      # 继续用你已有的 MCP_GROK_* / MCP_TAVILY_*，在这里映射成 rs 原生名
      GROK_SEARCH_API_KEY: ${MCP_GROK_API_KEY}
      GROK_SEARCH_URL: ${MCP_GROK_API_URL}
      GROK_SEARCH_MODEL: ${MCP_GROK_MODEL}
      GROK_SEARCH_WEB_SEARCH: "true"
      GROK_SEARCH_X_SEARCH: "false"
      TAVILY_API_KEY: ${MCP_TAVILY_API_KEY}
      TAVILY_API_URL: ${MCP_TAVILY_API_URL}
    timeout: 180
    connect_timeout: 120
```

有 `GROK_SEARCH_API_KEY` 时走 **Responses**：`POST {GROK_SEARCH_URL}/responses`。

### 通用 MCP JSON

```json
{
  "grok-search": {
    "command": "npx",
    "args": ["-y", "grok-search-rs@latest"],
    "env": {
      "GROK_SEARCH_API_KEY": "sk-...",
      "GROK_SEARCH_URL": "https://newapi.example/v1",
      "GROK_SEARCH_MODEL": "grok-4.5",
      "TAVILY_API_KEY": "th-...",
      "TAVILY_API_URL": "https://api.tavily.com"
    }
  }
}
```

### 校验

```bash
hermes mcp test grok-search
# 或客户端里 call doctor → transport 应为 Responses / grok_responses
```

| 方式 | 要不要 clone | 说明 |
|---|---|---|
| **`npx -y grok-search-rs`** | 否 | **推荐日常**；预编译，省内存 |
| Python `uvx --from git+...` | 否 | 一套配置但 Python 更重 |
| 本地 `cargo build --release` | 要 | 开发本 fork / 打补丁时用 |

---

## 本 fork（personal-compat）额外提供什么

当你 **从源码跑本仓库** 时，除了上游全部能力，还支持：

| 能力 | 说明 |
|---|---|
| **`GROK_API_*` 别名** | 与 Python GuDaStudio 命名一致；`GROK_SEARCH_*` 仍优先 |
| **`GROK_API_MODE`** | `auto`/`responses` → Responses；`chat` → Chat Completions |

| Python / 本 fork | 上游原生 | 用途 |
|---|---|---|
| `GROK_API_KEY` | `GROK_SEARCH_API_KEY` | Bearer |
| `GROK_API_URL` | `GROK_SEARCH_URL` | base URL |
| `GROK_MODEL` | `GROK_SEARCH_MODEL` | 模型 |
| `GROK_API_MODE` | （无） | `auto` / `responses` / `chat` |

**日常用 npx 时**：在 MCP 配置里把 `MCP_GROK_*` **映射成 `GROK_SEARCH_*`** 即可，不必 clone 本 fork。  
**要改代码 / 用别名直传 `GROK_API_*`**：再 clone 本分支。

### 本地开发本 fork

```bash
git clone -b personal-compat https://github.com/ZJ-zhangcn/GrokSearch-rs.git
cd GrokSearch-rs
cargo build --release

# 仅开发机：command 指向 target/release/grok-search-rs
# env 可直接用 GROK_API_KEY / GROK_API_URL / GROK_MODEL（与 Python 同名）
```

---

## 功能概览

- 🔎 `web_search` + 缓存 `get_sources`
- 📏 响应预算（控制上下文体积）
- 🧩 `web_fetch` 专用站解析 + Tavily→Firecrawl
- 🔀 Responses 或 Chat Completions
- 🩺 `doctor`

| Tool | 用途 |
|---|---|
| `web_search` | 检索摘要 |
| `get_sources` | 按 session 取来源 |
| `web_fetch` | 抽 URL 正文 |
| `web_map` | 站点映射 |
| `doctor` | 诊断 |

完整变量 / OAuth / config.toml：见 [docs/CONFIGURATION.md](docs/CONFIGURATION.md)。

---

## 与 Python fork 对照

| | Python `@personal-responses` | Rust（npx 或本 fork） |
|---|---|---|
| 零 clone | `uvx --from git+...` | **`npx -y grok-search-rs`** |
| 内存 | 高 | **低** |
| 一套 env | `GROK_API_*` / `MCP_GROK_*` | 映射到 `GROK_SEARCH_*`（或本 fork 源码跑时用 `GROK_API_*`） |
| Responses | 支持 | 原生（有 key 即 Responses） |
| 工具数 | 更多（plan_* 等） | 5 个核心工具 |

---

## Acknowledgements

- 上游：[Episkey-G/GrokSearch-rs](https://github.com/Episkey-G/GrokSearch-rs)
- Python：[GuDaStudio/GrokSearch](https://github.com/GuDaStudio/GrokSearch) · [ZJ-zhangcn/GrokSearch@personal-responses](https://github.com/ZJ-zhangcn/GrokSearch/tree/personal-responses)

## License

MIT — 见 [LICENSE](LICENSE)。
