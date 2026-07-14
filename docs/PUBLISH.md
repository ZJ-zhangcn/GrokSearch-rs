# 发包流程（personal-compat → npm）

包名：

| 包 | 用途 |
|---|---|
| `@zj-zhangcn/grok-search-rs` | 主包（`npx` 入口） |
| `@zj-zhangcn/grok-search-rs-darwin-universal` | macOS 二进制 |
| `@zj-zhangcn/grok-search-rs-linux-x64` | Linux x64 |
| `@zj-zhangcn/grok-search-rs-linux-arm64` | Linux arm64 |
| `@zj-zhangcn/grok-search-rs-win32-x64` | Windows x64 |
| `@zj-zhangcn/grok-search-rs-win32-arm64` | Windows arm64 |

与上游 `grok-search-rs@0.1.17` **同名空间不同**，不会互相覆盖。

## 一次性准备

### 1. npm 账号与 scope

1. 注册/登录 [npmjs.com](https://www.npmjs.com/)
2. 用户名或组织需能发布 **`@zj-zhangcn/*`**  
   - 若 npm 用户名不是 `zj-zhangcn`：要么建 org `zj-zhangcn` 并把账号加进去，要么改本仓库所有 `package.json` 里的 scope 为你的用户名
3. 本地验证：

```bash
npm login
npm whoami
# 可选：试探 scope 是否可用
npm access list packages @zj-zhangcn 2>/dev/null || true
```

### 2. Automation Token

1. npm → Access Tokens → **Generate New Token**
2. 选 **Granular Access Token**（推荐）或 classic **Automation**（专门给 CI）
3. Granular 必勾：
   - **Packages and scopes**：Read and write（至少能 publish `@zj-zhangcn/*`）
   - **Bypass two-factor authentication (2FA)** / “Bypass 2FA for automation” = **开**  
     （不开会 403：`Two-factor authentication or granular access token with bypass 2fa enabled is required`）
4. classic 则选类型 **Automation**（非 Publish/Read-only）
5. 复制 token（只显示一次）

> v0.2.0 构建与 GitHub Release 已成功；若只因 token 失败，修好 secret 后执行：  
> `gh run rerun 29306039113 -R ZJ-zhangcn/GrokSearch-rs --failed`  
> 无需重打 tag。

### 3. 写入 GitHub Secrets

仓库：`ZJ-zhangcn/GrokSearch-rs` → Settings → Secrets and variables → Actions

| Name | Value |
|---|---|
| `NPM_TOKEN` | 上一步的 npm token |

（Release workflow 用 `NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}`）

### 4. 默认分支 / 权限

- 日常开发：`personal-compat`
- Settings → Actions → General：Workflow permissions 允许 **Read and write**（打 tag 后回写版本）

## 发一版（推荐）

在 `personal-compat` 上改完代码并 push 后：

### 方式 A：GitHub Actions「Bump Version」

1. Actions → **Bump Version** → Run workflow  
2. `version`：例如 `0.2.1`（不要带 `v`）  
3. `dry_run`：先 `true` 看 diff，再 `false` 真正 commit + tag + push  
4. 推送 tag `v0.2.1` 会触发 **Release** workflow：  
   - 多平台 `cargo build`  
   - GitHub Release 附件  
   - 依次 `npm publish` 各 platform 包 + 主包  

### 方式 B：本地打 tag

```bash
cd ~/Desktop/projects/GrokSearch-rs
git checkout personal-compat
git pull

# 已改代码后 bump 版本（或靠 Actions Bump）
# 当前仓库版本见 Cargo.toml / npm/*/package.json

git tag -a v0.2.0 -m "v0.2.0 personal-compat first npm"
git push origin personal-compat
git push origin v0.2.0
```

然后打开 Actions → **Release** 看是否全绿。

## 验证

```bash
npx -y @zj-zhangcn/grok-search-rs@0.2.0 --version
# 或
npx -y @zj-zhangcn/grok-search-rs@latest
```

Hermes：

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
      GROK_SEARCH_WEB_SEARCH: "false"
      TAVILY_API_KEY: ${MCP_TAVILY_API_KEY}
      TAVILY_API_URL: ${MCP_TAVILY_API_URL}
```

## 常见失败

| 现象 | 处理 |
|---|---|
| `ENEEDAUTH` / 403 | `NPM_TOKEN` 未设或过期；需 **Automation** 或 granular + **Bypass 2FA** |
| `403 … Two-factor authentication or granular … bypass 2fa` | 账号开了 2FA，token 未勾 Bypass 2FA；重建 token 后 `gh run rerun <id> --failed` |
| `402` / scope 不存在 | npm 上无 `@zj-zhangcn` 组织/用户 |
| platform publish 成功、主包失败 | 看 optionalDependencies 版本是否与 tag 一致 |
| Release 没跑 | tag 必须是 `v*`（如 `v0.2.0`） |
| 仍装到上游 0.1.17 | 包名写错成 `grok-search-rs` 了，应用 `@zj-zhangcn/grok-search-rs` |

## 本地仅 mac 试发（可选，不推荐生产）

多平台二进制应走 CI。若只想本地测发布流程：

```bash
# 需要本机已 cargo build --release
mkdir -p npm/platforms/darwin-universal/bin
cp target/release/grok-search-rs npm/platforms/darwin-universal/bin/
cd npm/platforms/darwin-universal && npm publish --access public
cd ../../grok-search-rs && npm publish --access public
```

注意：只发 darwin 时，linux/win 用户 `npx` 会缺 optional binary。
