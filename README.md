# GrokSearch-rs（personal-compat）

> **本仓库**：[`ZJ-zhangcn/GrokSearch-rs`](https://github.com/ZJ-zhangcn/GrokSearch-rs)  
> **分支**：`personal-compat`  
> **上游**：[`Episkey-G/GrokSearch-rs`](https://github.com/Episkey-G/GrokSearch-rs)（Rust 重写）  
> **Python 对照**：[`ZJ-zhangcn/GrokSearch@personal-responses`](https://github.com/ZJ-zhangcn/GrokSearch/tree/personal-responses)

轻量 **Rust MCP** 服务：Grok 搜索 + Tavily fetch/map（可选 Firecrawl）。  
相对 Python 版占用更低、冷启动更快；本 fork 额外对齐 **Python / GuDaStudio 的 `GROK_API_*` 环境变量**，Hermes 等客户端可以继续用**同一套 env**，不必改成 `GROK_SEARCH_*`。

---

## 本 fork 相对上游改了什么

| 能力 | 说明 |
|---|---|
| **`GROK_API_*` 别名** | 与 Python `GrokSearch` 一致；`GROK_SEARCH_*` 仍优先 |
| **`GROK_API_MODE`** | `auto` / `responses` → Responses；`chat` → Chat Completions |
| **一套 MCP 配置** | 推荐本地 `cargo build --release`，`command` 指二进制路径；**无需** `npm i -g` |
| 默认传输 | 有 Grok key 时走 **`POST {URL}/responses`**（适配 NewAPI `grok-4.5` 等） |

| 变量（Python / 本 fork） | 等价上游变量 | 用途 |
|---|---|---|
| `GROK_API_KEY` | `GROK_SEARCH_API_KEY` | Bearer token |
| `GROK_API_URL` | `GROK_SEARCH_URL` | base，如 `https://newapi.example/v1` |
| `GROK_MODEL` | `GROK_SEARCH_MODEL` | 模型名，如 `grok-4.5` |
| `GROK_API_MODE` | （上游无） | `auto` / `responses` / `chat` |
| `TAVILY_API_KEY` / `TAVILY_API_URL` | 同名 | 抓取 / map |

优先级：`GROK_SEARCH_*` **>** `GROK_API_*` **>** 默认值。

传输选择（简化）：

1. `GROK_SEARCH_AUTH_MODE=oauth` → Responses（本地 token）
2. 存在 `GROK_SEARCH_API_KEY` 或 `GROK_API_KEY`，且 `GROK_API_MODE` 不是 chat → **Responses**
3. `GROK_API_MODE=chat` → Chat Completions（若未设 `OPENAI_COMPATIBLE_*`，会复用 `GROK_API_*`）
4. 仅有 `OPENAI_COMPATIBLE_API_URL` + `OPENAI_COMPATIBLE_API_KEY` → Chat Completions

---

## 推荐：源码构建 + 一套 MCP 配置

```bash
git clone -b personal-compat https://github.com/ZJ-zhangcn/GrokSearch-rs.git
cd GrokSearch-rs
cargo build --release
# 二进制：target/release/grok-search-rs
```

### Hermes（与 Python fork 相同 env 名）

```yaml
mcp_servers:
  grok-search:
    command: /绝对路径/GrokSearch-rs/target/release/grok-search-rs
    args: []
    env:
      GROK_API_KEY: ${MCP_GROK_API_KEY}
      GROK_API_URL: ${MCP_GROK_API_URL}
      GROK_MODEL: ${MCP_GROK_MODEL}
      GROK_API_MODE: ${MCP_GROK_API_MODE}
      TAVILY_API_KEY: ${MCP_TAVILY_API_KEY}
      TAVILY_API_URL: ${MCP_TAVILY_API_URL}
    timeout: 180
    connect_timeout: 120
```

`.env` 示例：

```text
MCP_GROK_API_KEY=sk-...
MCP_GROK_API_URL=https://newapi.example/v1
MCP_GROK_MODEL=grok-4.5
MCP_GROK_API_MODE=auto
MCP_TAVILY_API_KEY=th-...
MCP_TAVILY_API_URL=https://api.tavily.com
```

`auto` + `grok-4.5` → 请求 **`{MCP_GROK_API_URL}/responses`**。

### 通用 MCP JSON

```json
{
  "grok-search": {
    "command": "/绝对路径/GrokSearch-rs/target/release/grok-search-rs",
    "args": [],
    "env": {
      "GROK_API_KEY": "sk-...",
      "GROK_API_URL": "https://newapi.example/v1",
      "GROK_MODEL": "grok-4.5",
      "GROK_API_MODE": "auto",
      "TAVILY_API_KEY": "th-...",
      "TAVILY_API_URL": "https://api.tavily.com"
    }
  }
}
```

### 校验

```bash
# 仅用 GROK_API_* 时 doctor 应显示 transport/provider = grok_responses
export GROK_API_KEY=... GROK_API_URL=... GROK_MODEL=grok-4.5 GROK_API_MODE=auto
export TAVILY_API_KEY=... TAVILY_API_URL=...
# 客户端内调用 doctor，或：
hermes mcp test grok-search
```

拉代码后需重新编译：

```bash
git pull && cargo build --release
```

---

## 功能概览（继承上游）

- 🔎 `web_search`：带引用的实时搜索，结果缓存供 `get_sources`
- 📏 响应预算：控制 inline sources / 总字符，避免撑爆 agent 上下文
- 🧩 `web_fetch`：GitHub / SE / arXiv / Wikipedia 专用解析 + Tavily→Firecrawl 兜底
- 🔀 双传输：`/v1/responses` 或 `/v1/chat/completions`
- 📥 Tavily map/extract；可选多 key 轮转
- 🩺 `doctor`：连通性 + 脱敏配置

工具一览：

| Tool | 用途 |
|---|---|
| `web_search` | 主题检索与摘要 |
| `get_sources` | 按 `session_id` 取缓存来源 |
| `web_fetch` | 指定 URL 抽正文 |
| `web_map` | 站点 URL 发现 |
| `doctor` | 诊断 |

完整上游变量（`GROK_SEARCH_*` / OAuth / Firecrawl / 全局 config.toml）见 [docs/CONFIGURATION.md](docs/CONFIGURATION.md) 与上游 README。

---

## 可选：npm 全局包（上游官方）

若不想本地编译，仍可用上游预编译包（**没有**本 fork 的 `GROK_API_*` 别名，需写 `GROK_SEARCH_*`）：

```bash
npm install -g grok-search-rs
```

本 fork 推荐路径仍是 **release 二进制 + `GROK_API_*`**。

---

## 开发

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

文档：

- [Configuration](docs/CONFIGURATION.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Testing](docs/TESTING.md)

---

## 与 Python fork 的取舍

| | Python `GrokSearch@personal-responses` | 本仓库 `GrokSearch-rs@personal-compat` |
|---|---|---|
| 安装 | `uvx --from git+...` 一套配置 | 一次 `cargo build --release`，之后一套配置 |
| 内存 / 启动 | 较高 | 更低 / 更快 |
| env | `GROK_API_*` | **同名兼容** |
| Responses | 支持 | 原生默认 |
| 工具数量 | 更多（含 plan_* 等） | 5 个核心工具 |

---

## Acknowledgements

- 上游：[Episkey-G/GrokSearch-rs](https://github.com/Episkey-G/GrokSearch-rs)
- Python 先驱：[GuDaStudio/GrokSearch](https://github.com/GuDaStudio/GrokSearch)
- 个人 Responses 兼容参考：[ZJ-zhangcn/GrokSearch@personal-responses](https://github.com/ZJ-zhangcn/GrokSearch/tree/personal-responses)

## License

MIT — 见 [LICENSE](LICENSE)。
