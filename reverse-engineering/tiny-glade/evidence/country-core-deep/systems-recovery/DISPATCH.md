# crates/systems 复现分派台账（DISPATCH）

> 中央智能体维护。模式沿用 `tiny-glade/crates/country-core/recovery/agent-team/`（CHARTER 3 人组
> 结构：restorer 复写 / reviewer 审查 / re-analyst 深挖），目标：`tiny-glade/crates/systems/**`
> 13 个游戏自有 crate 全部按原二进制恢复并落库。
> 状态：queued / in_progress(restorer) / in_review / needs_reanalysis / fix_pending /
> review_passed(限定) / blocked。日期与更新者随行记录。

## 0. 依赖层次（恢复顺序，箭头 = 数据前置）

```
0  country-core（已章节化 3,118 业务，全 crate 覆盖闭合）
   │
   ├── 1a 独立小系统（无系统间依赖）：system-water / forest / system-glade-border /
   │        timberframe / system-tree-tool / system-color（55 业务）   ← 组 A
   ├── 1b 内容系统（消费 core 权威状态）：system-critters / system-clutter /
   │        system-onboarding（148 业务）                             ← 组 B
   ├── 1c 构造系统（消费 core 墙/屋顶权威）：system-wall-constructor /
   │        system-roof（130 业务；roof 大部分已恢复于 reconstruction/roof）← 组 C
   ├── 2  system-decorator（189 业务，唯一 ★★★★，依赖 core decorator_storage）← 批次 2
   ├── 3  toolbar（500 业务，自绘 UI 框架；布局复用 taffy 0.3.13，难度 ★★★☆）← 批次 3
   └── 4  system-onboarding → toolbar（onboarding 依赖 toolbar，故排组 B 尾部/批次 2）
```

## 1. 源码地图统计（skeleton/B/crates/systems，13 crate / 297 文件 / 1,025 业务）

| crate | 文件 | 总符号 | 业务 | 内联 | 胶水 | 依赖 | 难度 |
|---|---|---|---|---|---|---|---|
| system-water | 4 | 16 | 10 | 1 | 5 | core | ★★ |
| forest | 3 | 10 | 4 | 2 | 4 | core | ★ |
| system-glade-border | 3 | 5 | 2 | 1 | 2 | core | ★ |
| timberframe | 4 | 10 | 4 | 1 | 5 | core | ★★ |
| system-tree-tool | 11 | 24 | 13 | 3 | 8 | core | ★★ |
| system-color | 17 | 77 | 26 | 11 | 40 | core | ★★ |
| system-critters | 24 | 121 | 51 | 14 | 56 | core | ★★★ |
| system-clutter | 20 | 140 | 37 | 37 | 66 | core | ★★★ |
| system-onboarding | 20 | 115 | 60 | 18 | 37 | core+toolbar | ★★★ |
| system-wall-constructor | 53 | 348 | 97 | 52 | 199 | core | ★★★☆ |
| system-roof | 26 | 71 | 33 | 7 | 31 | core | ★★☆ |
| system-decorator | 74 | 644 | 189 | 80 | 375 | core | ★★★★ |
| toolbar | 38 | 914 | 500 | 150 | 264 | core | ★★★☆ |

数据源：`skeleton/B/crates/systems/**` PDB 编译单元元数据；胶水 = derive/serde/Bevy 注册
（按 AGENTS.md 不计独立业务）。机器码量级估计 1–1.5 MiB（toolbar 单 crate 914 符号）。

## 2. 分组与角色

### 组 A（快速通道：独立小系统）— restorer-A / reviewer-A / reanalyst-A
| 模块 | 状态 | 验收门槛（coordinator） |
|---|---|---|
| system-water（10）+ forest（4）+ glade-border（2）+ timberframe（4）+ tree-tool（13）+ color（26）| in_progress | **原函数闭合**：各文件业务函数逐一对齐（见骨架章）；**无系统间依赖**；consumer：core 资源（water_mask/color_id 等）+ render_loop 各 pass。证据：`evidence/country-core-deep/chapters/{water*,color*,tree*}*` + fresh 反编译。区分实现层面（库内可编译）与验收层面（consumer 闭合）。 |

### 组 B（内容系统）— restorer-B / reviewer-B / reanalyst-B
| 模块 | 状态 | 验收门槛 |
|---|---|---|
| system-critters（51）+ system-clutter（37）+ system-onboarding（60）| in_progress | **critters**：boids/羊/蝴蝶/鸭/鱼/萤火虫状态机（对照 reconstruction/critters 既有源码）；**clutter**：autoclutter 散布链（对照 reconstruction/clutter）；**onboarding**：20 hint_* 触发+显示（碎而浅）。consumer：core critters/clutter_meshes + render_loop。证据：既有 reconstruction/{critters,clutter} + building-* 证据 + 章节。 |

### 组 C（构造系统）— restorer-C / reviewer-C / reanalyst-C
| 模块 | 状态 | 验收门槛 |
|---|---|---|
| system-wall-constructor（97）+ system-roof（33）| in_progress | **wall-constructor**：拱/檐口/围栏/门/阶梯/半木构 13 子模块（对照 building-* 证据 + reconstruction/wall）；**roof**：create/edit/delete/replay（对照 reconstruction/roof 11 文件已恢复，重锚映射不重挖）。consumer：core PublicWalls/Roof + history。证据：`evidence/building-*` + `reconstruction/{roof,wall}` + `evidence/country-core-deep/roofs-rt/ALGORITHMS.md`。 |

## 3. 批次 2/3（组 A/B/C 完成后）：
- 批次 2：system-decorator（189）+ system-onboarding→toolbar 消费
- 批次 3：toolbar（500，Content 族归并 ~15 族）

## 4. 协作协议（同 country-core CHARTER）
1. 分派以本台账为准；restorer 交模块 → reviewer 独立审查 → FIX 回 restorer → re-analyst 补挖 → reviewer 复验。
2. 只读共享 Ghidra/ReVa shared；写库用各自唯一 --write worker。
3. **src 互斥**：同一时间一个模块只由一个 restorer 编辑；三组负责不同 crate 天然隔离。
4. 验收门槛见上；`review_passed` 前不算完成；**不把编译通过/局部测试当整模块完成**。
5. 实现落点：`tiny-glade/crates/systems/<crate>/`（workspace 已挂 roof/terrain 两个）；
   证据落点：`evidence/country-core-deep/systems-recovery/<组>/<模块>/`。
6. **接口契约**：restorer 必须按原逆向接口契约（章节 O 事实 + 真实 consumer）实现调用与参数，
   不得自造签名；reviewer 重点核对签名/参数面与机器码一致。

## 5. 当前断点（2026-10-16）
三组已启动（2026-10-16 中央，9 agent）：
- 组 A（快速通道 6 crate）：restorer=a6fb5271 / reviewer=a7ce3166 / reanalyst=ae85ce45
- 组 B（内容系统 3 crate）：restorer=a97b4a99 / reviewer=affac0bd / reanalyst=ac813dd0
- 组 C（构造系统 2 crate）：restorer=aee834b4 / reviewer=afe1f351 / reanalyst=a7efbba5
流转：restorer 交付 → main 通知 reviewer 独立审查 → FIX 回 restorer → reanalyst 并行补证据 → main 逐模块复核。

## 6. 组 A 审查准备（reviewer-A a7ce3166，2026-10-16）
- BASELINE-function-list.md（6 crate 符号+VA 缺失对照表）+ BASELINE-signature-surface.md（签名核对基准）
- 57 份 fresh RE-MCP 反编译（全部 business VA；color_menu 0x142291230 与 show_color_options 0x14229a7f0 超时待重试）
- 契约风险已转 restorer-A：ColorMenuBrightnessValue 三变体 {0.25,1.0,4.0}（旧 {Default,Custom(f32)} 错）；屋顶默认 ColorId=1 非 0；zeno/rhapsody 实例按依赖过滤不计缺

## 7. 组 B 审查准备（reviewer-B affac0bd，2026-10-16）
- 预置 REVIEW.md（148 业务逐项核对表）+ REVIEW_BASELINE.md（R1–R20 风险）
- 独立 RNE 抽验：firefly bias=+0.5/-0.5（文档"校正 -1.0"错、草稿对）；wren/fish/butterfly 常量吻合草稿；being_petted>0.01
- 最严重：① firefly 证据陷阱（文档 vs 机器码矛盾）② reconstruction/clutter 假实现不可复用 ③ legacy_spherecast 10 槽序契约
- 已转 restorer-B

## 8. 组 C 首轮审查（reviewer-C afe1f351，2026-10-16）
- roof 复用旧实现（reconstruction/roof 11 文件）：**O×11 零语义不一致**（edit_rules vs 0x1421ab7c0 全分支、surface/ridge/tiles 20+ 常量 rne-read 实测、replay 四 case）——可直接作 roof crate 基础
- 证据纠错：ALGORITHMS.md `_UNK_142934604`=16.0 错（实测 8.0），章节/实现正确，已转 re-analyst-C 登记
- BLOCKER：restorer-C 实现未落盘（ECS 面无法核对）+ 复用源不可编译（replay.rs 模块路径错、无 lib.rs/Cargo.toml）
- MAJOR：resolved_update 恒 true 与机器码不符（16B 通知仅 gable/!A 时推，ridge/tip 不推）
- wall-constructor：97 业务核对矩阵固化（半木构/高程支撑水分区须原码差分）；FIX 六项已转 restorer-C

## 39. toolbar 开源来源核查（2026-10-16，证据 evidence/toolbar-origin-2026-10-16/REPORT.md）

**结论**：不能把整个 toolbar 归为某开源 GUI 项目的修改版 = **游戏自有 toolbar/mei UI 实现 + 原版 Taffy 0.3.13 布局依赖 + tg-text 文字/图形桥接**。

- **布局底座 = Taffy 0.3.13**（registry；PDB 记录 31 个源文件 SHA-256 与 crates.io 发布包逐字节一致——无源码修改，直接复用，不恢复替代算法）
- **文本链 = cosmic-text 0.11.2**（PounceLight fork commit 3da2a14e；toolbar→tg-text→cosmic-text）——真实 fork 在文字子系统，不是 toolbar 整体
- **iced 0.12.1**（PounceLight fork 2bccf731）消费者为 crash-reporter-gui/tg-gpu-selector/tg-iced 等，**toolbar 无直接依赖边**
- **egui 0.27.2**（registry）country-core/tiny-glade 使用，**toolbar 无直接依赖边**（mei/widgets 的 egui 引用属于 tiny_glade::egui_backend 侧，非 toolbar 本体）
- **toolbar 内两类 UI 机制**：① Taffy/ECS 工具栏布局（taffy_layout/toolbar_layout/systems/ui_interaction）② mei 节点框架（core/layout/config/widgets/audio——自带简化布局，0x140fe0e10 是 mei 自有计算非 Taffy 库算法；旧 ANALYSIS"mei 最小尺寸=taffy"概括收窄）
- **逆向策略**：toolbar 自有框架（节点模型/交互/模板/tg-text 接线）按游戏业务恢复；Taffy/cosmic-text/etagere 按依赖复用
- 证据：provenance.json（31 源码哈希对照）/toolbar-procedures.json（542 过程，未去重不作分母）/bounded-*（8 函数 RNE=EXE 字节断言）/resolved-callees.json
