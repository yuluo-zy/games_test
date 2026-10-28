# 形状编辑 UX：原调度树与宿主注册合同（M01，只读）

证据（全部磁盘实读，未改源码、未跑 cargo）：
- `reports/module-recovery-2026-10-23/shape-registration-contract-native.json`（原 `140945f50` 的真实构造/注册截获；
  `tree`/`set`/`run_if`/`events`/`resources`/`original_register`）。
- `reports/module-recovery-2026-10-23/shape-registration-contract-review.md`（合同复核）。
- 当前实现：`crates/tiny-glade/src/run_game.rs:1134 install_toolbar_shape_dependencies`、
  `crates/tiny-glade/src/app_systems.rs:158 add_wall_events`、
  `crates/country-core/src/systems/wall/registration.rs`、
  `crates/country-core/src/startup/run_game.rs:191`、`crates/tiny-glade/src/startup_event_runtime.rs:58-63`。

---

## 1. 原 `add_wall_shape_ux`（入口 `0x140945f50`）调度树（精确）

> **术语说明**：本节 `a` / `b` / `c` 是原函数 `add_wall_shape_ux` 里的**局部 tuple 变量名**（`let a = (...)` / `let b = (...)` / `let c = (...)`），与游戏版本 A/B/C 无关；本项目**只重建 B 版**。下文统一称"注册树的 a/b/c tuple 分组"。

`shape-registration-contract-native.json.tree` 逐字：

```text
(a, c).chain_ignore_deferred()
      .in_set(<Set 变体 17>)
      .run_if(country_core::systems::is_gameplay_active)

a = (                                   // .chain_ignore_deferred()
    shape_placement_params,
    shape_placement_signifiers,
    calc_shape_height,
    ui_edit_circle,
    signifier_edit_circle_arrow,
    signifier_edit_circle_line,
    ui_rotate_circle,
    ui_create_circle,
    create_circle,
)

c = (                                   // .chain_ignore_deferred()
    ui_move_shapes,
    ui_move_freehand_walls,
    ui_edit_freehand_walls,
    b,                                  // 见下，作为单元素嵌入 c
    ui_focus_mode_shape_height,
    impl_raise_shape,
    ui_create_rectangle,
    signifier_edit_dims,
    create_rectangle,
    edit_rectangle,
    finish_edit_rectangle,
)

b = (                                   // .chain()
    (                                   //   普通并列 tuple（组1）
        ui_edit_rectangle_dims,
        ui_edit_rectangle_rotate,
        ui_raise_shape,
        ui_adjust_wall_height,
    ),
    (                                   //   普通并列 tuple（组2）
        do_ui_edit_rectangle_dims,
        do_ui_edit_rectangle_rotate,
    ),
)
```

要点（易错，逐条）：
- **`a` 与 `c` 各用 `chain_ignore_deferred`；`b` 用 `chain()`**（`b` 两组之间保留 deferred 同步语义）。不要把 `b` 展平成 6 项 chain，也不要给组内成员加先后约束。
- **最外层是 `(a, c).chain_ignore_deferred()`**——`a` 全链在 `c` 全链之前，且跨 tuple 不强制 flush deferred。
- `b` 是 `c` 的**单个元素**（嵌套 tuple），位置在 `ui_edit_freehand_walls` 之后、`ui_focus_mode_shape_height` 之前。
- 原 `141450870` 输出两个配置的 Vec（len/cap=2），不是第三个 boxed 系统；反编译里的 `Box` 是 `IntoScheduleConfigs` 目标泛型类型，与 tuple 成员不同层级。

### 1.1 Set 与 run_if
- `set = 17`（`native.original_register` 的 `tree.set`）。当前宿主 `GameSystemSet(17)` 与此一致。
- `run_if = country_core::systems::is_gameplay_active`，原 VA `0x140a90e00`，name VA `0x142c5fc6c` len `0x29`，
  字节 `48 8b 01 80 78 30 01 75 0a 48 8b 02 80 38 01 0f 94 c0 c3 31 c0 c3`。
- **两个真实来源**：predicate = `GameTime +0x30 (ran:bool) == 1 && State<AppLoadingState>.0 (byte) == 1`。
  即 `Res<GameTime>` 的 `ran` 与 `Res<State<AppLoadingState>>` 当前变体（Game）。**不能**简化为"仅 LoadingState=Game"，
  也不能用默认值代替 `GameTime.ran`。
- 当前 `run_game.rs:1178 is_gameplay_active(time: Res<GameTime>, state: Res<State<AppLoadingState>>)`
  实现 `time.ran && *state.get() == AppLoadingState::Game` —— 与原文等价，**保留**（不要重复定义第二个条件）。

### 1.2 每个系统在树中的位置
| 系统 | 位置 |
|---|---|
| shape_placement_params / shape_placement_signifiers / calc_shape_height | `a[0..3]` |
| ui_edit_circle / signifier_edit_circle_arrow / signifier_edit_circle_line / ui_rotate_circle | `a[3..7]` |
| ui_create_circle / create_circle | `a[7..9]` |
| ui_move_shapes / ui_move_freehand_walls / ui_edit_freehand_walls | `c[0..3]` |
| ui_edit_rectangle_dims / ui_edit_rectangle_rotate / ui_raise_shape / ui_adjust_wall_height | `b` 组1 |
| do_ui_edit_rectangle_dims / do_ui_edit_rectangle_rotate | `b` 组2 |
| ui_focus_mode_shape_height / impl_raise_shape / ui_create_rectangle | `c`（`b` 之后） |
| signifier_edit_dims | `c`（ui_create_rectangle 之后） |
| create_rectangle / edit_rectangle / finish_edit_rectangle | `c` 末尾 |

---

## 2. 原 `140945f50` 显式注册：8 事件 + 9 资源（原始顺序，按 call_site 排序）

原 register 的事件在缺失时才创建 `Events`（两空 Vec、计数 0）；已存在不清空。资源用 typed `insert_resource`。
**顺序即原调用顺序**（`call_site` 递增）：

| # | call_site | 动作 | 类型 | 显式默认写入（原 ABI） |
|---|---|---|---|---|
| 1 | 0x140946168 | insert | `rectangle::WipRectangle` | `+0x0 u32 = 0`（Idle 判别） |
| 2 | 0x1409461bb | insert | `shape_placement::ShapePlacement` | `+0 u32=0, +8 u32=0, +0x2c byte=2, +0x30..0x4f=0` |
| 3 | 0x1409461c5 | insert | `shape_placement::SnapMode` | `byte@0 = 0` |
| 4 | 0x14094622b | register_event | `rectangle::CreateRectangleCmd` | — |
| 5 | 0x140946287 | register_event | `rectangle::EditRectangleCmd` | — |
| 6 | 0x1409462e3 | register_event | `rectangle::FinishEditRectangleCmd` | — |
| 7 | 0x14094633f | register_event | `circle::create_circle::CreateCircleCmd` | — |
| 8 | 0x14094639b | register_event | `shape_placement::DontRecalculateShapePlacement` | — |
| 9 | 0x1409463f7 | register_event | `ui_raise_shape::HandleRaiseShapeUiInteraction` | — |
| 10 | 0x140946453 | register_event | `rectangle::ui_edit_rectangle_dims::DoUiEditRectangleDims` | — |
| 11 | 0x1409464af | register_event | `rectangle::ui_edit_rectangle_orientation::DoUiEditRectangleRotate` | — |
| 12 | 0x1409464c5 | insert | `circle::ui_edit_circle::AnimatedCircleSignifierResource` | `+0x38 byte = 2`（size 64） |
| 13 | 0x1409464e5 | insert | `rectangle::ui_edit_rectangle_dims::UiEditRectangleActiveWidgetResource` | `+0x0 qword = 0x8000000000000000`（size 80） |
| 14 | 0x140946527 | insert | `rectangle::ui_create_rectangle::PreviewRectangle` | `+0 u32=0; +0x10..1f f32=[1,0,0,0]; +0x20..27 f32=[6,6]; +0x28 f32=0`（size 44） |
| 15 | 0x140946558 | insert | `circle::ui_create_circle::PreviewCircle` | `+0 u32=0; +0x10..1f f32=[0,0,3,0]; +0x20 f32=0`（size 36） |
| 16 | 0x14094657d | insert | `rectangle::ui_create_rectangle::ShapesDragMouseMoved` | `+0..0xc = 0`（size 16） |
| 17 | 0x1409465a1 | insert | `calc_shape_height::TimberframeConstructionParamsCache` | `+0 u32=0; +8..0xf=0`（size 16） |

> **重要**：原 `140945f50` **不注册** `EditCircleCmd` / `FinishEditCircleCmd` / `RotateCircleCmd`。
> 这三个是当前仓库自创，必须从 `register_shape_resources` 与 `startup_event_runtime` 删除（见 §4）。
> `PreviewRectangle`/`PreviewCircle` 的默认值不得用 `derive(Default)`（尺寸 6、圆半径 3、朝向 [1,0] 不是零）。

---

## 3. 宿主需要新增 / 修改的内容

### 3.1 `crates/tiny-glade/src/run_game.rs:1134 install_toolbar_shape_dependencies`
现状只装了两条 5 系统链（`a` 的 5 个 + `c` 的 4 个），**缺** `a` 中的 `ui_edit_circle`、`signifier_edit_circle_arrow`、
`signifier_edit_circle_line`、`ui_rotate_circle`，缺整个 `b` 组，缺 `c` 的 `ui_move_shapes`、`ui_move_freehand_walls`、
`ui_edit_freehand_walls`、`signifier_edit_dims`、`edit_rectangle`、`finish_edit_rectangle`。

改造目标：按 §1 原树一次性安装（`GameSystemSet(17)` + `is_gameplay_active`），并补齐：
- **新增事件**：`DoUiEditRectangleDims`、`DoUiEditRectangleRotate`（`add_event`）。
- **新增资源**：`WipRectangle`、`AnimatedCircleSignifierResource`、`UiEditRectangleActiveWidgetResource`（`init_resource`）。
- **保留已有**：`ShapePlacement`、`SnapMode`、`PreviewCircle`、`PreviewRectangle`、
  `TimberframeConstructionParamsCache`、`ShapesDragMouseMoved`、`CreateCircleCmd`、`CreateRectangleCmd`、
  `DontRecalculateShapePlacement`、`HandleRaiseShapeUiInteraction`、`EditRectangleCmd`、`FinishEditRectangleCmd`。
  注意 `CreateCircleCmd` 在 PDB 的规范路径是 `circle::create_circle::CreateCircleCmd`，
  当前 `shapes::mod.rs` 顶层同名 re-export 需确认与之同型（当前 `shapes::CreateCircleCmd` 已被 create_circle 使用）。
- **删除**：`EditCircleCmd`、`FinishEditCircleCmd`、`RotateCircleCmd` 的 `add_event`（不在原合同）。
- **顺序**：资源 1-3 → 事件 4-11 → 资源 12-17（§2 表）。`register_event` 的缺失即创建语义由 Bevy `add_event` 等价提供。

### 3.2 `crates/tiny-glade/src/app_systems.rs:158 add_wall_events`
该函数对应原 `0x1404936b0`（全局墙事件/资源注册），**不是**形状 UX 入口。需要新增（当前仅在注释里点名、无类型定义）：
- `OnShapeMoved`、`OnFreehandWallEdited`、`OnCircleRotated`、`OnRectangleEdited`（`add_event`）。
- `RecalculateRoofCmd`（`add_event`；PDB 路径 `resources::roofs::commands`）。
- 已定位可注册的：`HideTreesNextToWallCmd`、`HideTreesinAreaCmd`、`ShowPinnedDecoClutter`、`PlayUiSound`、
  `NotifyHintSystemShapeEditWasPerformed`、`CmdCheckForSheepInsideBuilding`、`AdjustWallHeightUi`、
  `FinishAdjWallHeightMode`、`InitAdjWallHeightMode`（后两者已注册）。
- **原始 `0x1404936b0` 的逐调用注册顺序尚未整体还原**（`add_wall_events` 注释亦承认）；新增项应尽量按
  原 call_site 顺序插入。证据缺口：`0x1404936b0` 的完整事件调用序（`shape-registration-contract` 只覆盖 `140945f50`）。

### 3.3 当前 `add_wall_events` 已注册（保持）
`FinishWall`、`RecordMouseInputIntoWall`、`RoofIsInvalid`、`RecalculateWallHoles`、`WallBendChangedEvent`、
`OnWallDeleted`、`OnWallChanged`、`AdjustWallHeightCmd`、`StoneColumnsChanged`、`RecalculatePlasterCmd`、
`HighlightWallCmd`、`OutlineWallCmd`、`OutlineStairsCmd`、`AdjustWallHeightUi`、`FinishAdjWallHeightMode`、
`MergeWallsCmd`、`DoUiArchControls`，及一批资源（`ContinueWallOriginalState` 等）。

---

## 4. 需要删除 / 重构的旧注册点（逐条）

| 位置 | 现状 | 处置 |
|---|---|---|
| `country-core/src/systems/wall/shapes/mod.rs:110-146` | 自创 `EditCircleCmd`/`FinishEditCircleCmd`/`RotateCircleCmd` 三事件 | **删除三事件定义**（原合同无） |
| `shapes/mod.rs:170-183 register_shape_resources` | 初始化上述三事件 + `RectangleDimsEditInput`/`RectangleOrientationEditInput`/`CircleEditState`/`RectangleEditUiState` | 改为原始 8 事件 + 9 资源；删除自创 Resources |
| `tiny-glade/src/startup_event_runtime.rs:59,61,63` | `add_event::<EditCircleCmd/FinishEditCircleCmd/RotateCircleCmd>` | **删除三行** |
| `shapes/circle/ui_edit_circle.rs` | 事件驱动壳 + `CircleEditState` | 重写为 30 参系统（无事件） |
| `shapes/circle/ui_rotate_circle.rs` | `RotateCircleCmd` 壳 + 未注册 | 重写为 21 参系统 |
| `shapes/rectangle/ui_edit_rectangle_dims.rs` | `RectangleDimsEditInput` + `RectangleEditUiState` | 改 `WipRectangle`/`UiEditRectangleActiveWidgetResource`/`DoUiEditRectangleDims`/`Slider3d` |
| `shapes/rectangle/ui_edit_rectangle_orientation.rs` | `RectangleOrientationEditInput` + 错误 `ACOS_COEFFS` | 改 `ClosestCornerCache`/`DoUiEditRectangleRotate`；修正系数与 `fast_acos` |
| `shapes/rectangle/finish_edit_rectangle.rs` | `RectangleEditUiState([u8;80])` | 改 `WipRectangle`；补 9 参全链 |
| `shapes/rectangle/edit_decision.rs` | 自创判据封装 | 合并回 `finish_edit_rectangle` 或按原字段表核验后保留 |
| `shapes/rectangle/signifier_edit_dims_line.rs` | 4 边 + `pack2`，数据源 `PreviewRectangle` | 改 Query + `UiEditRectangleActiveWidgetResource` |
| `shapes/circle/signifiers/edit_arrow.rs` / `edit_line.rs` | `PublicWalls`/`PreviewCircle` + `pack2` | 改 Query + `AnimatedCircleSignifierResource` |
| `country-core/src/systems/wall/registration.rs:38-46,111,229-245` | 本地 CPU schedule 注册 `calc_shape_height/create_circle/create_rectangle/edit_rectangle/ui_create_*` 与 `register_shape_resources` | 与宿主 set(17) 树**重复**：同一系统会在两处运行。必须二选一（推荐生产只保留原 `a/b/c` tuple 分组树；本地 schedule 仅保留纯 CPU 非 UX 者，或明确其为测试装配）。`edit_rectangle` 尤其会双重消费 `EditRectangleCmd`。 |
| `country-core/src/startup/run_game.rs:126/191` + `tiny-glade/src/startup_construction_runtime.rs:14` | `register_wall_cpu_resources`/`add_wall_cpu_systems` | 同上：核对是否与 set(17) 树重复注册同一系统。 |
| `shapes/mod.rs:84-108` | `CreateCircleCmd`/`CreateRectangleCmd` 顶层 re-export | 核对与 PDB 路径（`create_circle::CreateCircleCmd` / `rectangle::CreateRectangleCmd`）是否同型；同型可保留别名。 |

---

## 5. 验收顺序（实现 agent）
1. 先做 §3.1 的资源/事件补齐（不改系统），保证 `cargo check`。
2. 按 §4 删除三自创事件 + 自创 Resources，纠正 `register_shape_resources`。
3. 逐叶替换系统（顺序见 `shape-edit-spec.md` 附录 A）。
4. 最后一次性按 §1 树安装到 `GameSystemSet(17)`，并解决 §4 的重复注册（本地 schedule vs 宿主树）。

## 6. 未决 / 需补证据
- `0x1404936b0 add_wall_events` 的**完整逐调用注册顺序**（仅 `140945f50` 有精确顺序）。
- `WipRectangle`/`UiEditRectangleActiveWidgetResource`/`AnimatedCircleSignifierResource` 的字段名（仅默认写入字节已知）。
- `ShapePlacement`/`SnapMode` 的字段语义归属（合同只给原写入，未给字段名）。
- 本地 CPU schedule 与宿主 set(17) 树的职责划分（属工程决策，需用户确认；不属原版 ABI）。
