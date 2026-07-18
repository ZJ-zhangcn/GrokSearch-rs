# GrokSearch-rs（personal-compat）

中文 | [English](https://github.com/ZJ-zhangcn/GrokSearch-rs/blob/main/README.en.md)


> **本仓库**：[`ZJ-zhangcn/GrokSearch-rs`](https://github.com/ZJ-zhangcn/GrokSearch-rs)  
> **分支**：`personal-compat`  
> **npm**：`grok-search-rs-pc`（**无需组织**）  
> **上游**：[`Episkey-G/GrokSearch-rs`](https://github.com/Episkey-G/GrokSearch-rs)

轻量 Rust MCP。比 Python/uvx 更省内存。

---

## 推荐：npx（不 clone）

```bash
npx -y grok-search-rs-pc@latest
```

> 不要用上游 `grok-search-rs@latest`（0.1.17，无本 fork 补丁）。

### Hermes

```yaml
mcp_servers:
  grok-search:
    command: npx
    args:
      - -y
      - grok-search-rs-pc@latest
    env:
      GROK_API_KEY: ${MCP_GROK_API_KEY}
      GROK_API_URL: ${MCP_GROK_API_URL}
      GROK_MODEL: ${MCP_GROK_MODEL}
      GROK_API_MODE: ${MCP_GROK_API_MODE}
      GROK_SEARCH_WEB_SEARCH: "false"
      TAVILY_API_KEY: ${MCP_TAVILY_API_KEY}
      TAVILY_API_URL: ${MCP_TAVILY_API_URL}
    timeout: 180
    connect_timeout: 120
```

发包说明：[docs/PUBLISH.md](docs/PUBLISH.md)

---

## personal-compat 补丁

| 项 | 说明 |
|---|---|
| `GROK_API_*` 别名 | 与 Python 同名 |
| 今天/时间注入 | 减少二次搜索 |
| 默认不塞 `tools: web_search` | 避免 grok-4.5 双工具 400/429 |

平台：macOS universal + Windows x64。

---

## 本地开发

```bash
cargo build --release
# target/release/grok-search-rs
```

---

## 工具

`web_search` · `get_sources` · `web_fetch` · `web_map` · `doctor`
