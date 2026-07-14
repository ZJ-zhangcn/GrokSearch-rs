# 发包流程（personal-compat → npm）

**不需要创建 npm 组织。** 使用**无 scope** 包名，个人账号即可发布。

| 包 | 用途 |
|---|---|
| `grok-search-rs-pc` | 主包（`npx` 入口） |
| `grok-search-rs-pc-darwin-universal` | macOS 二进制 |
| `grok-search-rs-pc-win32-x64` | Windows x64 二进制 |

与上游 `grok-search-rs` **名字不同**，不会冲突。

## 目标平台

- macOS universal（Intel + Apple Silicon 一条包）
- Windows x64  

不构建：Linux、Windows ARM。

## 一次性准备

### 1. npm 登录（个人账号即可）

```bash
npm login
npm whoami
```

无需 Organization。

### 2. Automation Token（CI 用）

1. https://www.npmjs.com/settings/~/tokens  
2. Generate New Token → **Granular** 或 classic **Automation**  
3. Granular 需：
   - Packages: **Read and write**
   - **Bypass two-factor authentication (2FA)** = 开（若账号开了 2FA）  
4. 复制 token

### 3. GitHub Secret

仓库 `ZJ-zhangcn/GrokSearch-rs` → Settings → Secrets → Actions：

| Name | Value |
|---|---|
| `NPM_TOKEN` | 上一步 token |

## 发版

```bash
git checkout personal-compat && git pull
# 改完代码后
git tag -a v0.2.0 -m "v0.2.0"
git push origin v0.2.0
```

或 Actions → **Bump Version**。

打 `v*` tag 后 **Release** workflow 会：编译 mac+win → GitHub Release → `npm publish` 平台包 → 主包。

## Hermes

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
```

## 验证

```bash
npx -y grok-search-rs-pc@latest
# 或
npm view grok-search-rs-pc version
```

## 常见失败

| 现象 | 处理 |
|---|---|
| `403 … bypass 2fa` | token 开 Bypass 2FA 或用 Automation |
| `ENEEDAUTH` | 检查 `NPM_TOKEN` secret |
| `Scope not found` | 旧配置用了 `@zj-zhangcn/*`；已改为无 scope，重发即可 |
| 装到上游 0.1.17 | 包名应是 `grok-search-rs-pc`，不是 `grok-search-rs` |

重跑失败的 publish（构建已成功时）：

```bash
gh run rerun <run-id> -R ZJ-zhangcn/GrokSearch-rs --failed
```
