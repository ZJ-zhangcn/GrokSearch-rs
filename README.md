# GrokSearch-rs（personal-compat）

轻量 Rust MCP：Grok/兼容网关网页搜索 + Tavily/Firecrawl。

| 项 | 值 |
|---|---|
| 仓库 | https://github.com/ZJ-zhangcn/GrokSearch-rs |
| 分支 | `personal-compat` |
| npm | **`grok-search-rs-pc`**（无需组织 scope） |
| 上游 | https://github.com/Episkey-G/GrokSearch-rs |

## 推荐运行

```bash
npx -y grok-search-rs-pc@latest
```

不要用上游 `grok-search-rs@latest`（缺本分支补丁）。

## Hermes 示例

```yaml
mcp_servers:
  grok-search:
    command: npx
    args: ["-y", "grok-search-rs-pc@latest"]
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

## 本分支补丁

| 项 | 说明 |
|---|---|
| `GROK_API_*` 别名 | 与常见 Python 配置同名 |
| 今天/时间注入 | 减少二次搜时间 |
| 默认不塞 `tools: web_search` | 降低 grok-4.5 双工具 400/429 |

## 工具

`web_search` · `get_sources` · `web_fetch` · `web_map` · `doctor`

## 本地编译

```bash
cargo build --release
# target/release/grok-search-rs
```

平台产物：macOS universal + Windows x64。发包见 `docs/PUBLISH.md`（若有）。
