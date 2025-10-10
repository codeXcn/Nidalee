# 签名密钥与安全（维护者）

本文档解释如何生成/配置 Tauri 更新签名密钥、公钥与 CI Secrets，并给出安全建议。

> 面向维护者；普通用户无需操作本页面内容。

## 生成密钥对（推荐）

在本地执行（确保安装了 Tauri CLI）：

```bash
pnpm exec tauri signer generate
```

输出示例（两行）：

```text
untrusted comment: minisign public key: XXXXXXXX
RWQraoGQhjHTGHgqKhjI0WKxWjDUNc3XWldb5RPIJWDf5+VYhY04+6Di
```

- 仅将第二行 Base64 文本作为公钥，粘贴到 `src-tauri/tauri.conf.json` → `plugins.updater.pubkey`。
- 第一行 `untrusted comment: ...` 是注释，不要粘贴到配置里。

## 配置私钥（CI/本地）

将私钥保存到安全位置，并配置：

- GitHub 仓库 Secrets：
  - `TAURI_SIGNING_PRIVATE_KEY`
  - `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`（若创建时设置密码）
- 本地构建时可临时导出环境变量（注意不要提交到仓库）。

## 校验公钥有效性

- 公钥应为 Base64，解码后长度为 32 字节（ed25519）。
- 可用 Node 快速验证：

```bash
node -e "const s='PASTE_BASE64_HERE';console.log(Buffer.from(s,'base64').length)"
# 输出 32 即正确
```

## 更换密钥对的影响

- 更换密钥对会导致旧版本无法通过应用内更新（验签失败）。
- 如必须更换：
  - 在 Release 说明与 README 明确提示需要下载最新安装包手动更新；
  - 在一段时间内同时提供过渡说明，减少用户困惑。

## 安全建议

- 私钥仅存于 CI Secrets 与维护者本地，切勿提交到仓库或泄露。
- 公钥为只读配置，不涉及秘密；但避免无意义修改，以免用户端缓存/对比混乱。
- 发布流程尽量通过 Tag 触发，保持可追踪与可回溯。
