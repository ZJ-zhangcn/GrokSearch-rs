# @zj-zhangcn/grok-search-rs

personal-compat build of [GrokSearch-rs](https://github.com/ZJ-zhangcn/GrokSearch-rs) (fork of Episkey-G/GrokSearch-rs).

## Install / run

```bash
npx -y @zj-zhangcn/grok-search-rs@latest
# or
npm i -g @zj-zhangcn/grok-search-rs
```

## Hermes MCP

```yaml
command: npx
args: [-y, "@zj-zhangcn/grok-search-rs@latest"]
env:
  GROK_API_KEY: ${MCP_GROK_API_KEY}
  GROK_API_URL: ${MCP_GROK_API_URL}
  GROK_MODEL: ${MCP_GROK_MODEL}
  GROK_SEARCH_WEB_SEARCH: "false"
```

See repo README for personal-compat features (time inject, no double web_search tool).
