# game 工作区成果归档

本仓库保留 Tiny Garden 工程与 Tiny Glade 逆向研究，并归档 `game` 工作区中的 Blender 素材库、工具脚本和技能文档。原始游戏文件仅在本地保存，不上传 GitHub。

## 已纳入的内容

| 路径 | 内容 |
|---|---|
| `tiny-garden/` | 原创 Rust / Bevy 项目、资产、美术源工程和文档 |
| `reverse-engineering/tiny-glade/` | 逆向研究、证据、进度、分析脚本及还原出的 Rust 源码 |
| `TinyGlade_Blender_Library/` | Blender 插件、构建/验证脚本、可编辑 `.blend` 工程、预览、渲染、报告和解码后的贴图 |
| `tools/README.md` | 本地逆向工具链安装和使用说明 |
| `tools/ghidra-scripts/` | 项目分析脚本 |
| `tools/blender/` | Blender 安装说明、启用插件脚本、启动入口和连接验证记录 |
| `.agents/skills/` | 工作区内已有的逆向技能及参考文档 |
| `.mcp.json` | 无凭证的本地 IDE MCP 连接配置 |
| `TinyGlade_类手游_*.md`、`TinyGlade_美术架构资产/` | 需求、架构、计划和设计资产 |

## 仅保留在本地的内容

- 原始游戏目录 `Tiny Glade/`、`ljxsj_92385/`、`tiny.glade/`，以及其中的安装包、可执行文件、PDB 和原始发行资源。
- `TinyGlade_Blender_Library/source/`：未经转换的网格、贴图、配置和 foliage 原始资源归档。
- `reverse-engineering/tiny-glade/workspace/`：原始二进制副本、PDB、Ghidra 数据库和本地验证运行器。
- 第三方工具安装目录、克隆的工具仓库、虚拟环境和 Ghidra 项目数据库。
- Rust `target/`、Python 缓存、Blender 自动备份、IDE 元数据和本地助手配置。

以上排除项由 `.gitignore` 控制，不删除本地文件。逆向还原出的 Rust 源码不属于排除项。

## Blender 素材库说明

`TinyGlade_Blender_Library/` 是工作区同名目录的发布快照，原目录保持不变。详见 [素材库说明](TinyGlade_Blender_Library/README.md)。

可编辑工程、解码后的贴图及预览可用于浏览和编辑；由于没有发布 `source/` 原始资源归档，涉及原始资源的重建脚本或精确归档验证仍需要本地原文件。报告中的来源路径和哈希保留用于溯源，不代表这些原文件已经随仓库发布。

工具文档中的绝对路径和版本记录描述本地安装环境；本仓库不包含工具可执行文件或完整安装包。
