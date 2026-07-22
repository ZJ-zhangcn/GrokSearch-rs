# GrokSearch-rs（personal-compat）

轻量 **Rust MCP** 服务：Grok / OpenAI 兼容网关网页搜索，附 Tavily fetch/map 与 Firecrawl 回退。  
Fork 自 [`Episkey-G/GrokSearch-rs`](https://github.com/Episkey-G/GrokSearch-rs)，**默认分支 = `personal-compat`**。

| 项 | 值 |
|---|---|
| 仓库 | https://github.com/ZJ-zhangcn/GrokSearch-rs |
| 分支 | `personal-compat` |
| 上游 npm | `grok-search-rs`（**不要用**，缺本分支补丁） |
| 本 fork npm | **`grok-search-rs-pc`**（unscoped，无需组织） |
| 上游 | https://github.com/Episkey-G/GrokSearch-rs |

## 与上游的主要区别

| 点 | 上游 | 本分支 |
|---|---|---|
| npm 包名 | `grok-search-rs` / 组织包 | **`grok-search-rs-pc`**，一键 `npx` |
| 环境变量 | 主要 `GROK_SEARCH_*` | 额外接受 **`GROK_API_*` / `GROK_MODEL` / `GROK_API_MODE`** 等 Python 同名别名 |
| 时间 | 无 | 查询含「今天/today」等时 **注入本地时间**，减少二次搜时间 |
| `GROK_SEARCH_WEB_SEARCH` | 默认更偏 `true` | **默认 `false`**，避免 grok-4.5 双工具 400/429 |
| 发布矩阵 | 多平台 | **macOS universal + Windows x64**（去掉 Win ARM 等） |
| 文档 | 英文长文 | 本中文操作说明 + `docs/PUBLISH.md` |

MCP 工具集与上游一致：`web_search` · `get_sources` · `web_fetch` · `web_map` · `doctor`。

## 部署 / 接入方式

### 方式 A：npx（推荐，不 clone）

**No local build needed.** Every release ships ready‑to‑run server artifacts for Linux
`x86_64` + `aarch64` (static musl) — ideal for low‑RAM/small‑disk boards where a native
`cargo build` would OOM or fill the disk:

- **Prebuilt binary** — download `grok-search-rs-http_Linux_<arch>.tar.gz` from the
  [latest release](https://github.com/Episkey-G/GrokSearch-rs/releases/latest) and run
  `GROK_MCP_BIND=0.0.0.0:8080 ./grok-search-rs --http`. (The plain `grok-search-rs_…`
  assets are **stdio‑only**: the HTTP transport is compile‑time gated and not in them.)
- **Docker image** — multi‑arch `amd64`/`arm64`:
  `docker pull ghcr.io/episkey-g/grok-search-rs:latest` (serves on `:8080`).

Building from source instead:

```bash
npx -y grok-search-rs-pc@latest
```

在 MCP 客户端里配置 command/args 即可，二进制由 npm 按平台拉取。

#### Hermes 示例

```yaml
mcp_servers:
  grok-search:
    command: npx
    args: ["-y", "grok-search-rs-pc@latest"]
    env:
      # 本分支别名（也可用 GROK_SEARCH_*）
      GROK_API_KEY: ${MCP_GROK_API_KEY}
      GROK_API_URL: ${MCP_GROK_API_URL}
      GROK_MODEL: ${MCP_GROK_MODEL}
      GROK_API_MODE: ${MCP_GROK_API_MODE}
      GROK_SEARCH_WEB_SEARCH: "false"
      TAVILY_API_KEY: ${MCP_TAVILY_API_KEY}
      TAVILY_API_URL: ${MCP_TAVILY_API_URL}
      # FIRECRAWL_API_KEY: optional
    timeout: 180
    connect_timeout: 120
```

#### 通用 JSON 示例

```json
{
  "grok-search": {
    "command": "npx",
    "args": ["-y", "grok-search-rs-pc@latest"],
    "env": {
      "GROK_SEARCH_API_KEY": "",
      "GROK_SEARCH_URL": "https://api.x.ai",
      "GROK_SEARCH_MODEL": "grok-4.20-fast",
      "GROK_SEARCH_WEB_SEARCH": "false",
      "TAVILY_API_KEY": "",
      "TAVILY_API_URL": "https://api.tavily.com"
    }
  }
}
```

### 方式 B：全局安装

```bash
npm install -g grok-search-rs-pc
# MCP command 改为: grok-search-rs
```

### 方式 C：源码编译

```bash
git clone -b personal-compat https://github.com/ZJ-zhangcn/GrokSearch-rs.git
cd GrokSearch-rs
cargo build --release
# 客户端 command 指向 target/release/grok-search-rs
```

### 方式 D：全局配置文件

```bash
grok-search-rs --init
${EDITOR:-nano} ~/.config/grok-search-rs/config.toml
```

环境变量模板可参考仓库 `.env.example`（注意本分支默认 `WEB_SEARCH=false` 的行为以运行时/补丁为准）。

## 传输模式

| 模式 | 条件 |
|---|---|
| xAI Responses | 配置 `GROK_SEARCH_*` / `GROK_API_*` 指向 Responses 兼容网关 |
| OpenAI Chat Completions | 未设 Grok Key，而配置了 `OPENAI_COMPATIBLE_API_URL/KEY/MODEL` |

可选 OAuth：`grok-search-rs login|status|logout`。

## 验证

在助手里调用 **`doctor`**，确认各上游 `reachable` 与 `transport`。

## 发包（维护者）

见 [`docs/PUBLISH.md`](docs/PUBLISH.md)。Release 走 tag + Actions；版本回写 `personal-compat`。

## 上游

完整英文说明、高级参数：上游仓库。日常接入请始终用 **`grok-search-rs-pc`**。

## 远程 HTTP（上游 0.1.18，可选）

上游新增 **Streamable HTTP** 多租户 BYO-key 模式（`--features http`，默认 stdio 不受影响）。本 fork 的 npm 包仍以 **stdio + personal-compat 补丁** 为主。

```bash
cargo build --profile release-http --features http
GROK_MCP_BIND=127.0.0.1:8080 target/release-http/grok-search-rs --http
```

详情见上游文档与仓库内 `Dockerfile`、`docker-compose.yml`、`Caddyfile`。Hermes 日常仍用：

```bash
npx -y grok-search-rs-pc@latest
```
