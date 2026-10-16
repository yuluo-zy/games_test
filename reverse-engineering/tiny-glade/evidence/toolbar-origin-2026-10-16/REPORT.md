# 原版 toolbar 的开源来源核查（B，2026-10-16）

结论：目前不能把整个 toolbar 归为某个开源 GUI 项目的修改版。原作呈现的是 **游戏自有 toolbar/mei UI 实现 + 原版 Taffy 0.3.13 布局依赖 + tg-text 文字/图形桥接**。如果问工具栏的开源布局底座，明确答案是 [DioxusLabs/taffy](https://github.com/DioxusLabs/taffy)；如果问整个工具栏从哪个开源项目 fork 而来，现有证据未找到这样的上游。私有源码路径和自有命名只能证明实现归属，不能证明历史上没有移植或借鉴。

## 1. 身份与方法

只研究 `D:/game/ljxsj_92385/Tiny Glade` 的 B 原发行文件，不研究 tiny-garden，不将 A/C 等同 B。

| 输入 | SHA-256 |
|---|---|
| tiny-glade.rne（原指令 oracle） | f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03 |
| tiny-glade.exe（MCP 数据库输入） | cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323 |
| tiny_glade.pdb | 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911 |
| build-info/Cargo.lock | 9e0cb4a0fa173122fa7cd552aa639f5ee977abc22c74aec92398808c707184bd |

从本机原 PDB 重新运行 LLVM `dump -files`，并独立解析 PDB compiled-procedure 记录。依照 PDB 范围选取 8 个函数，逐个确认原 RNE 与 patched EXE 的完整范围字节相同，再在专属 Windows MCP worker 定点准备和反编译。没有修改原发行文件、原数据库、游戏实现或 Rust 依赖。

## 2. 最强来源证据：Taffy 没有出现源码修改

原锁文件记录：

```text
taffy 0.3.13
source = registry+https://github.com/rust-lang/crates.io-index
checksum = 82870da09c57a8a5a50f830ce8993a6637b9a4932f69257f12aea3fa68f588c9
```

下载 [crates.io 原发布包](https://static.crates.io/crates/taffy/taffy-0.3.13.crate)，包 SHA-256 与原锁完全一致。随后比较本轮重新导出的 PDB 文件校验和：

- PDB 记录的 31 个 Taffy 源文件，各自 SHA-256 全部与发布包对应文件一致。
- 包含 `src/node.rs`、`src/compute/flexbox.rs`、`src/compute/grid/*`、`src/style/*`、`src/layout.rs` 等。
- 发布包总计 39 个 `.rs` 文件；上述结论仅覆盖 PDB 有记录的 31 个，不把另 8 个也计为已经核实。
- PDB 来源路径为 `.cargo/registry/src/.../taffy-0.3.13/...`，与锁中的 registry 来源互相支持。

因此应复用原版 Taffy 0.3.13，不能把 toolbar 的适配代码叫作“Taffy 分叉改动”。这些证据支持已记录编译来源文件未经修改；它们不验证未记录源码、编译选项或完整构建环境。

## 3. 机器码调用说明 Taffy 负责什么

本轮定点反编译与原 PDB 名称对应如下（完整调用表见 `resolved-callees.json`）：

| 原作地址 | 归属/行为 | 来源判断 |
|---|---|---|
| `0x141e4c650` | toolbar `TaffyLayout::style` → `0x140fe7cb0` Taffy `style`，另有 Result unwrap | 游戏薄封装调用开源库 |
| `0x141e4c6c0` | toolbar `TaffyLayout::perform_layout` → 游戏 `taffy_mut_style`、Taffy `compute_layout`/`layout`、游戏 `make_layout_absolute` | 开源布局计算 + 游戏锚点/坐标策略 |
| `0x140fe7e40` | `taffy::node::Taffy::compute_layout` → `0x141e84140` `taffy::compute::compute_layout` | 库实现，不恢复替代算法 |
| `0x142374e10` | 游戏 `make_layout_absolute<Taffy>`：移动节点坐标、dirty 标记、经 child_count/child 递归处理子节点 | 游戏适配层；调用库内部状态 API |
| `0x140683ec0` | `MeiContext::reuse_or_create_node`：按 GlobalId 查找/回收自有 Node，调用 hashbrown | 自有节点模型；容器仍复用库 |
| `0x140fe0e10` | `mei::layout::LayoutStyle::compute_min_size_with_children`：读取子尺寸，进行自身 gap/轴向聚合 | 自有简化布局；该函数没有 Taffy 调用 |
| `0x141e9df60` | toolbar `render_text_fragments` → `0x140b91120` `tg_text::TextFragment::render` | 文字交给 tg-text，而非 egui/iced 控件渲染 |

注意 Ghidra 默认函数名为 `FUN_*`，真实名称按 PDB 过程地址解析；Rust 泛型地址合并时保留全部别名。没有把 PDB 的 `void()` 当作真实 ABI，也没有仅因 decompiler 完成就认定参数和局部变量类型已恢复。

## 4. toolbar 里面实际有两类 UI 机制

1. **Taffy/ECS 工具栏布局**：`resouces_and_components/taffy_layout.rs`、`toolbar_layout.rs`、`toolbar_systems.rs`、`ui_interaction.rs` 等负责布局、工具分类、对齐、自动隐藏、焦点、点击/拖拽、动画及声音。Taffy 提供布局计算，游戏层决定节点、参数与操作反馈。
2. **mei 节点框架**：PDB 明确把 `core.rs`、`layout.rs`、`config.rs`、`widgets.rs`、`widgets/tweaker.rs`、`widgets/decorate.rs`、`audio.rs` 归到 `crates/systems/toolbar/src/mei/`。已有原反编译中可见 GlobalId 节点复用、跨帧节点表、内容 trait、输入处理、导航、模板和控件。菜单消费这一层；它自带简化布局，不能一概说成 Taffy 的实现。

原发行 `assets/ui/menu/default.ron` 也直接给出 `Vertical`、`Horizontal`、`Expand`、`Align`、`XAlign`、`Padding`、`Gap`、`slot` 和 `Image(... scaling: Nine)` 等模板内容，与 mei 配置/模板代码吻合。这是原资源证据，不是根据现有重建代码倒推。

旧 `upper-modules-plan/systems/toolbar/ANALYSIS.md` 中“mei 最小尺寸计算(taffy)”的概括应收窄：本轮确认 `0x140fe0e10` 是 `toolbar::mei::layout` 自有计算，不能将它标为 Taffy 库算法。两套机制的存在也不表示它们在所有页面完全隔离。

## 5. 哪些开源项目确实出现了 fork

| 组件 | 原锁来源 | 与 toolbar 的关系 |
|---|---|---|
| Taffy 0.3.13 | crates.io registry，31 个已记录源文件逐字节匹配 | 直接布局依赖；无已发现源码改动 |
| cosmic-text 0.11.2 | `PounceLight/cosmic-text`，commit `3da2a14ea3d063875fdc55695431e74405302b0f` | `toolbar → tg-text → cosmic-text`；文本子系统的真实分叉来源，不能升级为整个 toolbar 的来源 |
| etagere 0.2.10 | crates.io registry | tg-text 的图集分配依赖 |
| iced 0.12.1 | `PounceLight/iced`，commit `2bccf7317e42422cb3b98c0de4f291d8c0865380` | 直接消费者为 crash-reporter-gui、tg-gpu-selector、tg-iced、tg-integrity-report；toolbar 无直接依赖边 |
| egui 0.27.2 | crates.io registry | country-core、tiny-glade 等其他组件使用；toolbar 无直接依赖边，不能据程序出现 egui 就判定工具栏基于 egui |

[Taffy 项目](https://github.com/DioxusLabs/taffy)自述为布局库，[cosmic-text 上游](https://github.com/pop-os/cosmic-text)是文字处理项目；[PounceLight 官方 GitHub 组织页](https://github.com/PounceLight)列出 iced 为 iced-rs/iced 的 fork。确切版本和 commit 仍以本地原锁为准，不使用公开仓库当前 HEAD 替换历史构建。

## 6. 能回答与尚不能回答的问题

- **已回答**：工具栏可明确确认的开源布局底座是 Taffy；已记录 Taffy 源文件与 0.3.13 发布包全部一致；游戏侧还具有独立 mei、交互/模板及 tg-text 接线。
- **已回答**：cosmic-text 和 iced 的 PounceLight Git 分叉在原锁有准确身份，但分属文字子系统与辅助 GUI 链，不能把二者当作整套 toolbar 的上游。
- **部分回答**：实现可按游戏自有框架与开源依赖分开恢复；尚无证据把 mei 对应到一个完整公开 GUI 项目。
- **未能证明**：mei 从零原创、从未借鉴/移植任何开源代码，或所有非直接依赖都绝不参与 toolbar。有限符号/依赖查询不能证明这些否定命题。
- **范围外**：完整 cosmic-text/iced fork diff、全部 UI 行为还原、A/C、运行画面、GPU 与全游戏验收。本轮无游戏代码修改或完成度增长。

## 7. 可复查证据

- `provenance.json`：原文件身份、原锁条目、来源路径、31 份源码哈希对照和直接依赖消费者。
- `original-pdb-files.txt`：本轮原 PDB 的 LLVM 文件表；`procs.tsv`：本轮独立 PDB 过程导出。
- `toolbar-procedures.json`：542 条 toolbar 命名空间过程记录（未去重/分类，不能和旧 509 条 scoped 清单当成同一分母，不是进度）。
- `bounded-procedures.json`：8 个 PDB 代码范围、原 RNE 字节 SHA、patched EXE 相等断言。
- `bounded-query.json` / `bounded-result.json` / `bounded-result.artifacts/`：真实 MCP SDK 请求、完整结果、SHA 校验后的 stdout；`0x*.c` 为本轮定点伪码。
- `resolved-callees.json`：原 PDB 名称解析后的调用边；未知 ABI/间接调用边界仍保留。
- `mcp-result.json`：初始 seed 的 1 字节函数缺口，不能当作反编译证据；随后 bounded preparation 才取得有效函数体。
- Worker：`toolbar-origin-20261016`；独立库 `D:/game/tools/reverse-runtime/toolbar-origin-20261016/project/TinyGlade.gpr`，只进行了限定函数范围准备和本地 checkpoint，未同步历史数据库。
- `reopen-query.json` / `reopen-result.json`：正常停止本 worker 后重开独立库，重新确认程序身份与三个已准备函数的代码范围/调用边仍保留。
- `collect.py` / `prepare.py`：来源采集和定点 job 的复现脚本；脚本本身不算业务还原。
