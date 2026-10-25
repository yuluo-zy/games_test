# game 工作区成果归档

本工作区保存 Tiny Glade 的 Rust 还原工程、逆向研究与 Blender 素材库，并归档工具脚本和技能文档。原始游戏文件仅在本地保存，不随仓库发布。

> 下表路径均相对于 `game/` 工作区根目录。入口索引见 [`../README.md`](../README.md)；逆向证据域见 [`../reverse-engineering/README.md`](../reverse-engineering/README.md)。

## 已纳入的内容

| 路径 | 内容 |
|---|---|
| `tiny-glade/` | Rust workspace：还原源码、415 个可读 HLSL、`provenance/` 审计与清单、`reports/`、`tools/` |
| `reverse-engineering/tiny-glade/` | 逆向研究、证据、A/B/C 代码骨架、恢复脚本、计划与工具文档 |
| `美术资源 Blender 版本/` | Blender 可编辑素材库：`library/`（blends、插件、预览、贴图、渲染）、`pipeline/scripts/`、`reports/`、`source/`、`docs/art-direction/` |
| `tools/README.md` | 本地逆向工具链安装和使用说明 |
| `tools/ghidra-scripts/` | 项目分析脚本 |
| `tools/blender/` | Blender 安装说明、启用插件脚本、启动入口和连接验证记录 |
| `.agents/skills/` | 工作区内已有的逆向技能及参考文档 |
| `.mcp.json`、`AGENTS.md` | 无凭证的本地 IDE MCP 配置与工作区约定 |

`tiny-glade/reports/` 按领域分为 `foundation/`、`building/`、`systems/`、`runtime/`、`business/`、`ui-input/`，入口索引见 [reports/README.md](reports/README.md)；被取代的早期迭代和原始转储在 `reports/_archive/`。

美术方向概念板与生成记录已归入 `美术资源 Blender 版本/docs/art-direction/`，不再单独占一个顶层目录。

## 仅保留在本地的内容

- 原始游戏三版：`A版本内容/`、`B版本内容/`、`C 版本内容最新/`，包含可执行文件、PDB、`assets/`、`compiled-assets/` 等发行资源。三版已删除广告、安装器、第三方修改器等无关文件，并去掉多余嵌套目录；原声与 B 版的 RUNE 残留按要求保留。
- `美术资源 Blender 版本/source/`：未经转换的网格、贴图、配置和 foliage 原始资源归档。
- `reverse-engineering/tiny-glade/workspace/`：原始二进制副本、PDB、Ghidra 数据库和本地验证运行器。
- `tools/` 下的工具安装目录、克隆仓库、虚拟环境和 Ghidra 项目数据库。
- Rust `target/`、Python 缓存、Blender 自动备份、IDE 元数据和本地助手配置。

以上排除项由 `.gitignore` 控制，不删除本地文件。逆向还原出的 Rust 源码不属于排除项。

## Blender 素材库说明

`美术资源 Blender 版本/` 是素材库的唯一规范目录，早期并列的 `TinyGlade_Blender_Library/` 嵌套副本已删除。入口为 `美术资源 Blender 版本/library/blends/00_StartHere.blend`，也可双击 `打开素材库.command`。详见 [素材库说明](../美术资源%20Blender%20版本/README.md)。

可编辑工程、解码后的贴图及预览可用于浏览和编辑；`source/` 原始资源归档只保留在本地，涉及原始资源的重建脚本或精确归档验证仍需要本地原文件。报告中的来源路径和哈希保留用于溯源，不代表这些原文件已经随仓库发布。

工具文档中的绝对路径和版本记录描述本地安装环境；本工作区不包含工具可执行文件或完整安装包。

> 历史说明：以下四节记录早期仓库的推送范围与快照。当前工作区副本不含 `.git`，其中的推送、远端和备份引用仅作历史留存。

## 大型分析数据快照（2026-10-12）

新增全部还原候选、骨架、证据、文档和工具脚本。骨架中的原始PDB副本沿用“原始游戏文件仅本地保存”规则。超过GitHub普通单文件上限的`metadata-stage.pickle`与`index.sqlite3`以同目录`.gz`保留无损快照，原文件和快照均留在本地、不随本次交付推送；路径、大小与SHA256见`reverse-engineering/tiny-glade/tracking/git-large-file-snapshots.json`。

本地需要时，可用`gzip -dk 文件.gz`恢复原文件，再按清单核对SHA256。索引恢复后可用于已有task_queue；元数据快照用于既有分析脚本。源码候选仍为implemented_unverified，本次提交不构成编译、原码或业务验收。

## 本次最终推送范围

按用户最终选择，提交全部已还原源码、工作流/工具脚本、文档和有界交付证据。保留`abc-tools-52`与`abc-building-35`的共享Rust候选、A/B/C映射、中文注释、冻结清单、合同及独立审查报告。

`skeleton-enriched`副本已于 2026-10-24 删除（内容由 `skeleton/` 覆盖）。全量PDB/符号/汇编/依赖dump、数据库、全量函数/文件/任务索引、自动队列快照与大数据压缩快照留在本地。原依赖Cargo锁及必要源码工作目录随交付保留。文档中的原始游戏、全量生成输入和绝对路径用于本地溯源，不表示这些输入文件随本次推送发布。完整全量提交另有本地Git备份`refs/codex-backups/full-generated-20261012`，未推送远端。

## A/B/C 版本代码补充推送

按用户后续明确要求，`reverse-engineering/tiny-glade/skeleton/A/`、`B/`、`C/`三个版本的全部3,667个`.rs`代码骨架、对应Cargo清单/锁文件、说明文档及代码形式的Shader示例也纳入Git。保持原版本和模块路径，未重新生成或改写原代码。

这些版本文件包含PDB符号、接口、字段与原指令观察的注释骨架；不能称为完整可运行的三版游戏。87组共享Rust候选仍在`skeleton/task-index/work`中保留单份实现与ABC映射。三版旁路`.json`/`.gz`元数据、原PDB、全量索引、缓存继续仅保留本地；`skeleton-enriched` 重复副本已于 2026-10-24 全部删除。

## 2026-10-24 工作区整理

- Blender 素材库由 `TinyGlade_Blender_Library/` 更名为 `美术资源 Blender 版本/`：运行时文件归入 `library/`，脚本归入 `pipeline/scripts/`，概念板归入 `docs/art-direction/`。删除与上层逐字节相同的嵌套副本，约 0.45 GB。
- 原始游戏目录整理为 `A版本内容/`、`B版本内容/`、`C 版本内容最新/`：删除广告、启动器、卸载器、VC 运行库、修改器、临时日志与冗余 Crack 副本约 50 MB，并将 `B版本内容/Tiny Glade/`、`C 版本内容最新/XJ07036/Tiny Glade v1.16.0/` 展平。
- `美术资源 Blender 版本/pipeline/scripts/prepare_sources.py` 的 `VERSIONS` 已同步为三个新根路径，脚本依赖的 `assets/meshes`、`compiled-assets/textures`、`compiled-assets/foliage-package` 均已核对存在。
- `reverse-engineering/tiny-glade/` 清理：删除全部 `target/` 构建目录、临时目录与缓存约 21.4 GB；经逐项核对，`skeleton-enriched/` 的 3,292 个 `.rs` 与 3,184 个增强区段均已被 `skeleton/` 覆盖，该目录已整体删除。目录由 39 GB 降至 10 GB。
- 修正 `AI_REVERSE_TOOLS.md`、`MULTI_BASELINE.md`、`THREE_VERSION_RUST_PLAN.md` 与根 `AGENTS.md` 的失效路径；历史文档已于同日清理。
- 二次清理（2026-10-24）：删除 `docs/archive/`、`rounds/`、`upper-modules-plan/`、`checkpoints/`、`recovery/`、`skeleton-enriched/`、`workspace/integration-backup-2026-10-13/` 与 `evidence/sync-20261019-local-backup/`。保留 `evidence/`、`skeleton/`（含 `task-index/work`）、`reconstruction/`、`docs/current/`、`docs/reference/`、脚本与工具。
