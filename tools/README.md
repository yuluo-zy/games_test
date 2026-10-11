# 逆向工具链（Tiny Glade / Windows Rust 游戏）

安装位置：`/Users/yuluo/Desktop/game/tools`

## 已安装

| 组件 | 版本 | 位置 | 用途 |
|---|---|---|---|
| Node.js | 24.11.0 | `tools/node` | REA 运行时 |
| REA (`rea-agents`) | 6.3.0 | `tools/node/bin/rea` | 逆向编排 MCP + skill |
| Ghidra | 12.1.4 PUBLIC | `tools/ghidra_12.1.4_PUBLIC` | 反汇编/反编译（PE + PDB） |
| ghidra-mcp | 7.0.0 | `tools/mcp-servers/ghidra-mcp` | Ghidra MCP 扩展 + bridge |
| radare2 | 6.2.4 | `/opt/homebrew/bin/r2` | CLI 二进制分析 |
| radare2-mcp (`r2mcp`) | - | `r2pm -r r2mcp` | radare2 MCP |
| ReVa skills | 7.3.1 | `~/.codex/skills/*` | 逆向技能集 |
| Frida | 17.23.3 | `tools/venv/bin/frida` | 动态插桩（**当前用不上**：目标为 Windows PE，本机无 VM/Wine）|
| rustfilt | 0.2.1 | `~/.cargo/bin/rustfilt` | Rust 符号 demangle CLI |
| kaitai-struct-compiler | 0.11 | `/opt/homebrew/bin/kaitai-struct-compiler` | 自定义格式解析（支持生成 Rust/Python）|
| pyghidra | 3.1.0 | `tools/venv` | 无头 Python 驱动 Ghidra |
| Python 依赖 | - | `tools/venv` | lief / pefile / capstone |

JDK 21：`/Users/yuluo/Library/Java/JavaVirtualMachines/corretto-21.0.5`

## 环境变量

```bash
export GHIDRA_INSTALL_DIR=/Users/yuluo/Desktop/game/tools/ghidra_12.1.4_PUBLIC
export JAVA_HOME=$(/usr/libexec/java_home -v 21)
export PATH=/Users/yuluo/Desktop/game/tools/node/bin:$PATH
```

## Codex MCP 注册（~/.codex/config.toml）

- `rea`    → `tools/node/bin/rea mcp`
- `ghidra` → `uv run --directory tools/mcp-servers/ghidra-mcp bridge-mcp-ghidra`
- `radare2`→ `r2pm -r r2mcp`

## 已验证

目标：`ljxsj_92385/Tiny Glade/tiny-glade.exe`（PE32+ x64, Rust, 56MB）+ `tiny_glade.pdb`(262MB)
Cargo.lock SHA256 与项目记忆基线一致：`9e0cb4a0...7184bd`

- radare2：识别 PE64 / windows / `dbg_file=tiny_glade.pdb`
- Ghidra headless：导入成功，PDB Universal + Rust Demangler 生效
  结果：**131,517 函数 / 991,680 符号 / 56,861 个 Rust `::` 符号**
  样例：`country_core::systems::wall::adjust_wall_height::...`、`system_decorator::...`

## 常用命令

```bash
# 重新导入并完整分析（分析较慢，建议 ≥1h 或直接在 GUI 中打开）
"$GHIDRA_INSTALL_DIR/support/analyzeHeadless" tools/ghidra-projects TinyGlade \
  -import "ljxsj_92385/Tiny Glade/tiny-glade.exe" -overwrite

# 打开 GUI（GhidraMCP 插件随项目窗口启动，MCP 在 127.0.0.1:8089）
"$GHIDRA_INSTALL_DIR/ghidraRun"

# radare2 快速检索
r2 -q -c 'aaa; afl~wall' "ljxsj_92385/Tiny Glade/tiny-glade.exe"
```

## 未安装 / 已移除

- ImHex：Homebrew cask 因 Gatekeeper 被禁用
- IDA Pro：付费，未安装
- ReVa 扩展本体：官方最新版针对 Ghidra 12.1.3，本机为 12.1.4，仅安装了其 skills
- spirv-cross / spirv-tools：**已卸载**——实测游戏 404 个 `compiled-shaders/*.bin` 魔数为 `02000000`/`01000000`，非 SPIR-V，工具不适用
- aria2：**已卸载**——原为解决直连慢，现有 7897 代理，多余

## 下载注意

GitHub release 直连为 0 B/s，必须走代理：

```bash
export https_proxy=http://127.0.0.1:7897 http_proxy=http://127.0.0.1:7897
```
