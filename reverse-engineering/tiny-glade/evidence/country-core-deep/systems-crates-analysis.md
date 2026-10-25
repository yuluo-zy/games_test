# crates/systems 家族逆向分析（2026-10-14）

## 0. 总览

`skeleton/B/crates/systems/` 下 13 个游戏自有 crate（`system-*` 命名族 + toolbar/timberframe/forest），
**297 文件 / 4,427 函数符号 = 1,025 业务 + 445 内联 + 2,957 派生胶水**。这是 country-core
之外的**最大游戏自有代码块**，机器码体量估计 1–1.5 MiB（toolbar 单 crate 就 914 符号）。

| crate | 文件 | 业务 | 内联 | 胶水 | 难度级 | 性质 |
|---|---|---|---|---|---|---|
| toolbar | 38 | 500 | 150 | 264 | ★★★☆ | 完整 UI 框架（**布局复用 taffy 0.3.13 公开 crate**，自有业务=Content 族/生命周期/菜单/拍照/MEI） |
| decorator | 74 | 189 | 80 | 375 | ★★★★ | 装饰物生命周期/视觉/UI/碰撞（22 文件 decorator_visual + 16 ui_systems） |
| wall-constructor | 53 | 97 | 52 | 199 | ★★★☆ | 墙构造算法族（拱/檐口/围栏/门/阶梯/半木构 13 子模块） |
| clutter | 20 | 37 | 37 | 66 | ★★★ | autoclutter 散布/檐口垛/缝合砖 |
| critters | 24 | 51 | 14 | 56 | ★★★ | boids/羊/蝴蝶/鸭/鱼/萤火虫 AI |
| onboarding | 20 | 60 | 18 | 37 | ★★★ | 新手引导提示系统（20 个 hint_* 文件） |
| roof | 26 | 33 | 7 | 31 | ★★☆ | create/edit/delete/replay + 视觉（**大部分已恢复**） |
| color | 17 | 26 | 11 | 40 | ★★ | 颜色菜单/屋顶墙门装饰着色 |
| tree-tool | 11 | 13 | 3 | 8 | ★★ | 树木工具 |
| water | 4 | 10 | 1 | 5 | ★★ | 水面系统 |
| forest | 3 | 4 | 2 | 4 | ★ | 树林 |
| timberframe | 4 | 4 | 1 | 5 | ★★ | 木构架 |
| glade-border | 3 | 2 | 1 | 2 | ★ | 地块边界 |

## 1. 已有覆盖（不用重挖的部分）

| 领域 | 既有恢复 | 对应 systems crate |
|---|---|---|
| 屋顶 | `reconstruction/roof/` 11 文件（edit_rules/parameters/pipeline/replay/tiles/ridge/surface…4514+ 案例验证） | system-roof 大部分 |
| 装饰物 | `evidence/building-decorator-info/`（B 资源类型/消费者/生产者多包） | system-decorator 信息模型层 |
| 墙体 | `evidence/building-*`（wall-state/lifecycle 已深度） | system-wall-constructor 的墙状态侧 |
| 生物 | `reconstruction/critters/`（firefly/sheep_pet/butterfly/duck 已有源码） | system-critters 部分 |
| autoclutter | `reconstruction/clutter/`（add_rules/manage/populate/process 已恢复） | system-clutter 部分 |
| 屋顶触发/垛/塔尖 | `evidence/building-roof-{trigger,spires}/` | system-clutter inter_roof_merlons |

## 2. 难度评估

### ★★★★ 最难：toolbar（500 业务）、decorator（189 业务）

**toolbar**：不是简单 UI——布局引擎**复用 taffy 0.3.13**（原锁 registry 依赖、toolbar 17 文件引用但 PDB 调用面仅 5 个 API 点：points/from_points/layout_mut/mark_dirty；taffy_update.rs 三函数 = 游戏侧接线层，非自研布局），含 `mei/`（相机编辑器）、
`photomode/`、`menu/`（10 文件）、`resouces_and_components/`（5 文件）、`demo_stuff/`。
函数名模式显示它是**自绘 UI 组件框架**：`ScrollContent/LabelContent/ImageContent/FrameContent/
TweakerContent/TodCircleContent/ThumbnailScrollContent/TextEditContent/TabSwitcherContent/
PillButtonContent/MenuButtonContent` 等 12+ 内容类型 × `create_node_content/ui/prepare_layout/
render/on_input/process_input_action` 生命周期方法。这是**全套 egui 风格的即时模式 UI 框架**
（作者自研，非 egui——egui 是另一依赖）。难度：布局算法 + 输入命中 + 主题化 + 无障碍，且
大量泛型单态化（每个 Content<T> 独立实例）。**唯一 500 业务单 crate**，需单独一个多轮专项。

**decorator**：74 文件最多，含 `decorator_visual/`（22 文件——装饰物网格/变形/动画）、
`ui_systems/`（16 文件）、`apply_decorator_op`（装饰操作应用）、`backup/blueprint/
collision_detection/cull_oob_decorators/chimney_smoke_anim_thickness`。装饰物是游戏最复杂
实体（墙附着/窗口/门/灯/烟囱 + 历史 op 系统），与 country-core 的
`resources::walls::decorator_storage/archivist` 直接对接。难度：几何变形 + 附着 + 历史。

### ★★★ 中难：wall-constructor（97）、critters（51）、onboarding（60）、clutter（37）

**wall-constructor**：构造算法（拱 walker/檐口支撑 13 文件/围栏/门/阶梯/半木构），几何密集，
但**已有 building-* 证据基础**（wall-state/lifecycle/arches 已深挖）——恢复路径清晰。

**critters**：boids 群体 AI + 各生物状态机（羊 11 文件最多）。已有 reconstruction/critters 部分
源码可对照。AI 逻辑（目标/避让/状态机）是自有业务，难度中等。

**onboarding**：20 个 hint_* 文件 = 新手引导提示（相机控制/进度/焦点编辑/吸附禁用…），
逻辑简单（触发条件 + 显示状态），但**文件多而碎**，适合批量章节化。

**clutter**：autoclutter 散布（候选→区域→实体管理）+ 垛/缝合。已有 reconstruction/clutter
基础。散布算法（窗/门/屋顶周围规则）是核心业务。

### ★★ 低中：roof（33）、color（26）、tree-tool（13）、water（10）、timberframe（4）

roof 大部分已恢复（create/edit/delete/replay + 视觉）；color 是简单着色命令；tree-tool/water/
timberframe 体量小。这些是**快速通道**。

### ★ 低：forest（4）、glade-border（2）

几乎纯胶水，数日可完。

## 3. 依赖关系（决定顺序）

```
toolbar（UI 宿主） ──► color（着色菜单） ──► decorator（装饰 UI）
                              ▲
wall-constructor ──► decorator（门/窗附着） ──► clutter（垛/缝合）
                              │
critters / tree-tool / water / forest ──► 独立内容系统（可并行）
```

建议推进顺序：
1. **快速通道**（水/树/木构/地块边界/color，约 55 业务）——1 个 agent 一轮闭合
2. **clutter + onboarding**（97 业务）——1 个 agent（clutter 有基础、onboarding 碎而浅）
3. **critters 补全**（51 业务，对照已有源码）——1 个 agent
4. **wall-constructor**（97 业务，几何密集、有 building-* 基础）——2 个 agent 分块
5. **decorator**（189 业务，最大实体域）——3–4 个 agent 按子模块分
6. **toolbar**（500 业务，自绘 UI 框架）——单独多轮专项（先框架骨架→内容类型族→布局）

## 4. 风险点

- **toolbar 开源来源已核查**（2026-10-16，evidence/toolbar-origin-2026-10-16/REPORT.md）：整个 toolbar
  = 游戏自有框架 + **Taffy 0.3.13 布局依赖**（PDB 31 源文件与发布包逐字节一致，无改动直接复用）+
  **cosmic-text 0.11.2**（PounceLight fork，文字链）+ tg-text 桥接；iced/egui 均无 toolbar 直接依赖边；
  逆向只恢复自有框架（mei 节点模型/交互/模板/接线），布局/文字库按依赖复用。
- **decorator 与 country-core 的边界**：decorator_storage/archivist 在 country-core
  （已章节化），system-decorator 消费它——接线证据需跨 crate 交叉。
- **泛型单态化规模**：toolbar 每个 Content<T> 独立实例（类似 ssbo_library 12 族——可用
  族归并法，预计 toolbar 可归并为 ~15 族）。
- 机器码体量：13 crate 合计估计 1–1.5 MiB，是 country-core（约 3.9 MiB）的 1/3 量级，
  但**逻辑密度更高**（UI/构造算法/AI 全是自有业务，胶水占比 67% 与 core 相近）。

## 5. 结论

`crates/systems` 是 country-core 之后**最大的游戏自有源码块**（1,025 业务函数）。
不是"难"，而是"大而专"：decorator（装饰物全生命周期）是唯一 ★★★★ 级专项；toolbar（自绘 UI 框架，布局复用 taffy）为 ★★★☆，需要多 agent 分轮；roof/water/tree/forest/color 等约 88 业务可快速闭合；
wall-constructor/critters/clutter/onboarding 有既有恢复基础，属中量级。按 §3 顺序推进，
预计与 country-core 同等规模的 4–6 轮可完成章节化 + 实现。
