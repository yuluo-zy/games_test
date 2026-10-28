# Shape-edit UX 独立审查（B 版原证对照）

- 审查 owner：独立审查 agent（只读，不改生产源码、不 commit）
- 日期：2026-10-28
- 工程：`D:/game/tiny-glade` ｜ 原版：`D:/game/ljxsj_92385/Tiny Glade`
- 二进制身份（本次独立复核，`sha256sum`）：
  - `tiny-glade.rne` = `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03` ✅ 与任务指纹一致
  - `tiny-glade.exe` = `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323` ✅ 与任务指纹一致

## 0. 输入与审查时点

本审查启动时，任务简报点名的 `shape-edit-spec.md` / `shape-edit-registration.md` / `impl-status.md` 均不存在；
随后协调线于 09:57–10:00 补齐三份文件。`impl-status.md` 的结论是**全部叶子 BLOCKED / PRE-EXISTING，本轮未改任何生产代码**
（`impl-status.md:71`）。因此本文件是**静态审查首轮**：对象是仓库中现存的旧实现 + READY 叶现值；动态链测试在
`impl-status` 出现 `TESTED` 后再按构建锁补充（见 §5）。

审查基准（协调线确认 + 本次独立复核一致）：
- `ui_rotate_circle` = **21 参**；`ui_edit_circle` = **30 参且无 Edit/Finish 事件**，唯一 EventReader 是 `MainCameraMoved`。
- `ACOS_COEFFS` 前 4 项错、`fast_acos` 缺 `π-a` 负半支与 cross 符号选择。
- signifier 不得再用 `UiAppendSignifier`(5 qword) Raw 适配，数据源必须是 ECS Query + `AnimatedCircleSignifierResource` / `UiEditRectangleActiveWidgetResource`。
- 必须删除的臆造项：`EditCircleCmd/FinishEditCircleCmd/RotateCircleCmd`、`CircleEditState`、`RectangleDimsEditInput`、`RectangleOrientationEditInput`、`RectangleEditUiState([u8;80])`、`edit_decision` 封装，以及 `startup_event_runtime.rs:59/61/63` 与 `shapes/mod.rs:170-183` 注册点。
- 原树：a/c tuple 用 `chain_ignore_deferred`，b tuple 用 `chain()`，b 是 c 的单个嵌套元素；`is_gameplay_active = GameTime.ran && AppLoadingState==Game`。摊平/漏 b 的 deferred/简化 run_if = FAIL。

## 1. 本次独立执行的验证命令与结果（证据）

| # | 命令 | 结果 |
|---|---|---|
| V1 | `python work/m01-scratch/review/_probe_consts.py`（`rne_dis.read` 直读 .rdata） | 0x142960674=0.01、0x1429cd068=0.2、0x142a7c880=0.9、0x142934604=8.0、0x142a89600=0.23、0x1429edb90=0.75、0x1429edb94=90、0x1429edba4=π、0x142a7baa0=9.9999997e-21、0x142925904=1.0、0x142925870=0.5、0x142925984=0.1、0x142a16d10=-10.0、0x142a16cb8=0.8 —— **全部与规格 §0.2 一致** |
| V2 | `python work/m01-scratch/review/_probe_acos.py`（0x142ae0d30..0x142ae0d70） | 0x142ae0d44..0x142ae0d60 实读 = `[0xbaa57a2c,0x3bda90c5,0xbc8bfc66,0x3cfd10f8,0xbd4d8392,0x3db63a9e,0xbe5bbfca,0x3fc90fda]`。**当前 `ACOS_COEFFS` 与实读不符**（见 L-O1） |
| V3 | `sha256sum tiny-glade.rne tiny-glade.exe` | 与任务指纹一致（见文首） |
| V4 | `grep -rn "\.write(EditCircleCmd\|\.write(FinishEditCircleCmd\|\.write(RotateCircleCmd\|send(...)"` | 全仓库 **0 producer**；仅 `ui_edit_circle`/`ui_rotate_circle` 读取。三事件为死事件 |
| V5 | `grep -rn "RectangleDimsEditInput *{\|RectangleOrientationEditInput *{"` | **0 构造点**；两输入资源只 `init_resource`，无 producer |
| V6 | `grep -rn "struct OnShapeMoved\|OnCircleRotated\|OnRectangleEdited\|OnFreehandWallEdited"` | **0 定义**（仅一处文档注释提及）。四事件类型缺失 |
| V7 | `grep -rn "OnShapeMoved\|OnFreehandWallEdited" ... EventWriter` | 0 命中；且事件类型本身不存在 |
| V8 | `sed -n run_game.rs 1130-1173` + `registration.rs` | 见 §3 注册判定 |
| V9 | 读 `country-core/recovery/.../decomp/140ac6d60__ui_adjust_wall_height_mode.c` | 仅用于核对 READY 叶的事件游标顺序（见 L-R1） |

> 动态链测试（`cargo test`）：**未执行**。理由：`impl-status` 无 `TESTED`（全 BLOCKED/PENDING），且审查期间检测到
> 实现 agent 的 `cargo`/`rustc` 进程与 `target/.agent-build.lock`（09:54 创建）正在占用构建；按协调线要求不并行占 Cargo。
> 后续在 TESTED 后按锁协议补跑（§5）。

## 2. 逐叶静态审查

图例：PASS=当前实现与原证逐项一致；PARTIAL=参数/控制流部分一致但有关键缺口；FAIL=存在臆造/致命错配；MISSING=无实现；
PENDING=规格未 READY、旧实现不可用。

### 2.1 `ui_adjust_wall_height` 三件套

| 叶子 | 判定 | 缺陷（严重度 / 文件:行 / 证据） | 验证 |
|---|---|---|---|
| `ui_adjust_wall_height`（11 参, 0x140b2fc00） | **TESTED**（逻辑三腿 + 墙高 signifier 画面均 PASS；§7 arrow 已 PASS（§13），不阻断；见 §11） | 11 参/类型/顺序与 PDB 一致，跳表 dispatch 与 B 一致。修正复核：**F1 gate1 已修**：`:82 !state.is_being_bulldozed()`，与 `entry[-9]`=值记录 byte `0x297`=`PublicWallState.being_bulldozed`（`public_wall_state.rs:363`；`publish_public_wall_state.rs:161` 写入）一致，且有 `gate1_skips_being_bulldozed_walls` 测试。**F2 widget 值已修**：`WIDGET_ENV_TABLE=[0.7,0.5]`、`env=[shape_tag<2]`（`:67-72`），可达分支取 0.7。**F3 signifier = Empty handle（PASS，10:57 版本）**：`SignifierEntityBuilder::new(SignifierMesh::Empty).rotation(IDENTITY).scale(1.5)` + marker `0x142ab4830`；原版 0x140956cd0 即 `"signifier empty"`（RNE 实读 @`0x142a99570`），identity + scale 1.5 固定 handle，不用墙几何/墙色，`angle`/`WallId` 仅身份（§11.5；此前"占位/已知限制"判断已更正）。**F4（已修复 CONFIRMED FIXED）**：守卫改 `ui_state.active_widget() != widget`，`widget` = `context.id`（`emit_signifier(..., context.id, ...)` `:219`），与注册 id（`from_name("edit_height_{wall}")`）一致。**G（CONFIRMED：几何正确）**：widget 几何 = `SphereCloud{12B/pt, radius=[0.7,0.5][shape_tag<2]}`（非 TriangleMesh）；`0x202`∈`extra_78`；`0x142ab3c08` 仅本函数读、入 scalar 槽。**focus/adjust 同名覆盖 = 原版行为（已裁决，非缺陷）**：`pcStack_108`=AppMode 第二 qword=wall id → 同一 WidgetId，原版按注册序覆盖，focus 在 adjust 之后 → 最终命中体为 focus `Sphere{0.5}`（§9.5 已关闭 UNKNOWN）。 | 读源 + RNE + decomp + publish 路径 |
| `ui_adjust_wall_height_mode`（12 参, 0x140ac6d60） | **PASS（附 1 低风险 UNKNOWN）** | 参数 12 项、类型、顺序与 PDB 完全一致（`ui_adjust_wall_height_mode.rs:191-204`）。控制流：last(AdjustWallHeightUi)→cursor_moved 空退→PublicWalls unwrap→init 帧 get_closest_point_u_to_cursor+归档/cull→公共尾投影发 `AdjustWallHeightCmd`，与 spec §1.2/§1.3 一致。L-R1（LOW/UNKNOWN）：文档注释（文件头 180-189）称「cursor 空退」在「取 last UI」之前，但代码（:205→:208）与测试 `missing_cursor_moved_returns_but_consumes_the_ui_queue` 都表明「先取 last UI 再判 cursor」；测试名自称 consumes UI queue，需对 0x140ac6d60 事件游标顺序独立复核 | 读源 + decomp 头 |
| `history_record_adjust_wall_height`（7 参, 0x140ac79e0） | **PASS（附 1 低风险 UNKNOWN）** | 参数 7 项、类型、顺序与 PDB 一致（`:310-318`）。记录门用 `(wall, original_height, current_height)` 三个 `Option` 的 `Some` 状态映射 B 的 `cache[+0x20]==1 && cache[+0x28]&1 && cache[+0]&1`（三个 flag/ty 位 ↔ 三个 Option 判别位，语义等价；物理偏移属实现细节，非 ABI 声明）。clamp 经 `height_rules::wall_height_bounds`：roof(h≤0.1→1.6 else 0.7)/无 roof(special→0.3 else 0.5)/hi(special→2.5 else 14.0) 与 B 一致（`height_rules.rs:13-24`）。L-R2（LOW/UNKNOWN）：clamp 用 `>`/`<` 有序比较代替 SSE MAXSS/MINSS（`height_rules.rs:31-41`），NaN desired 时保留 NaN，与 MAXSS/MINSS「NaN 选第二操作数」是否逐位一致未独立验证 | 读源 |

> 三件套结论（2026-10-28 更新）：`_mode` 与 `history_record` 两叶单独 PASS（保留 L-R1/L-R2 两个 LOW UNKNOWN）；
> `ui_adjust_wall_height` 本体已实现但 **FAIL**（F1/F2/F3，见 §8）→ **三件套整体不得记为 PASS**。
> 更正（PDB 复核）：`AdjustWallHeightUi`/`InitAdjWallHeightMode`/`FinishAdjWallHeightMode` 在 B 中确有**两个真实 producer**：
> `ui_focus_mode_shape_height`（PDB `drop_in_place<..FunctionSystem<fn(..)>>` 明确含三 writer + `HandleRaiseShapeUiInteraction` + `Local<LerpedGizmoPosition>`）与 `ui_adjust_wall_height`；
> 二者均注册 `edit_height_{wall}` 并在 set17 运行——这是原版设计，**不是**重复造物；此前"无 producer"的说法不成立。

### 2.2 move / freehand 组

| 叶子 | 判定 | 缺陷 | 验证 |
|---|---|---|---|
| `move_gizmo_and_trigger_vol`（helper, 0x14097d280） | **MISSING** | BLOCKER：无此函数（`ui_move_shapes` 规格要求同模块 helper）。依赖 box_trigger_volume/ui_state::add/rotation_from_normal/xvy | 静态 |
| `ui_move_shapes`（30 参, 0x14097a060） | **PARTIAL**（见 §12；4 tests passed） | `ui_move_shapes.rs`：**30 参顺序与 skeleton/B 一致**；`digital[2].phase<3` 已接入 `move_gizmo`（`:261-262`,`:280`，单测过）；case3 已实现（InnerWalls curve + `StateChange` compound `History::add` + `OnShapeMoved`/`ShowPinned`，单测过）。**TODO**：case1(`:213`)/case2(`:219`)、move_gizmo `box_trigger`/`UiState::add`/axis signifier(`:289`)、public-tail 占位 anchor、9 writer consumer 核对。事件类型均已存在。spec §3；§12 | 读源 + disasm + skeleton + 测试 |
| `ui_move_freehand_walls`（22 参, 0x1409781d0） | **MISSING** | BLOCKER：无系统、无 `MoveWallCache`。PDB 表 :40-63 | 静态 |
| `ui_edit_freehand_walls`（20 参, 0x140ac98e0） | **MISSING** | BLOCKER：无系统、无 `EditFreehandWallCache`。PDB 表 :65-86 | 静态 |

### 2.3 圆组

| 叶子 | 判定 | 缺陷（严重度 / 文件:行 / 证据） | 验证 |
|---|---|---|---|
| `ui_edit_circle`（30 参, 0x14165d460） | **FAIL（臆造壳）** | BLOCKER：当前 5 参事件壳（`ui_edit_circle.rs:61-67`：`EventReader<EditCircleCmd>, EventReader<FinishEditCircleCmd>, ResMut<InnerWalls>, ResMut<CircleEditState>, EventWriter<RecordWallHistory>`）vs PDB **30 参**。臆造 `EditCircleCmd`/`FinishEditCircleCmd`（import :12-13，定义 `shapes/mod.rs:111-132`）——原版无此事件，唯一 reader 是 `MainCameraMoved`。臆造 `CircleEditState`(:29)。缺：WallTriggerVolume Query、光标 SS→ray 投影、半径 clamp(0.9,8.0)、0.01 阈值、glade 越界双事件、show_full_circle_signifier、10 个 event writer、CompoundEdit 历史 0x20/0x28 行。证据 spec §6 / `spec/ui_edit_circle.md §1,§3` | 读源 + PDB |
| `signifier_edit_circle_arrow`（3 参, 0x140b35120） | **PASS**（§13.2；5 tests passed） | 参数 3 项正确；门控 `resource.tag != 2`；`rotation_from_normal`+`position` 写 Transform；工厂 `from_closure`+marker `0x142aed888`；**mesh = `SignifierMesh::Named("flat_arrow")`**（builder tag0 Named，`FUN_14092d570` 查表缺失 panic；"flat_arrow" 原证存在）+ `vertex_color_shader`；name `edit_circle_{:?}_arrow`，`WidgetId` 取资源 `+0x00`；组件 `ComponentRegistration{0x58,TypeId,name}` 非空。细微：live 资产解析不在单测覆盖；结构字段名仍 `unknown_00`（+0x00 语义=WidgetId，建议具名）。**未调度进 set17**（属最后挂 a/c 组步骤）。证据 spec §7 + §13 | 读源 + 单测 5 passed |
| `signifier_edit_circle_line`（4 参, 0x140a64a40） | **FAIL（臆造数据源）** | HIGH：当前 2 参（`edit_line.rs:25-28`：`Res<PreviewCircle>, ResMut<UiSignifierStream>`）vs PDB **4 参**应为 `Query<(&mut SignifierEditCircleShapeLine,&Handle<Mesh>,&SignifierAnimation)> + Res<AnimatedCircleSignifierResource> + ResMut<AssetMeshLibrary> + Res<UiSignifierStream>`。用全局 `PreviewCircle` 替身；缺 `AssetMeshLibrary`/`update_curve_mesh`；`pack2`(:14) 自创。证据 spec §8 / `circle_rotate_and_signifiers.md §3` | 读源 + PDB |
| `ui_rotate_circle`（21 参, 0x1409ac910） | **FAIL（臆造壳）** | BLOCKER：当前 2 参事件壳（`ui_rotate_circle.rs:39-45`：`EventReader<RotateCircleCmd>, ResMut<InnerWalls>`）vs PDB **21 参**。臆造 `RotateCircleCmd`（定义 `shapes/mod.rs:142-146`，无 producer）。缺：UiState wall、view-proj 最近点、box_trigger_volume+UiState::add、stage 状态机（drag/release/commit）、角度 gizmo ease、|Δangle|≤0.01、OnCircleRotated/RecalculateRoofCmd/PlayUiSound/ShowPinnedDecoClutter/hint/audio 6 事件、RotateCircleCache/CurrentAngleGizmo。证据 spec §9 / `circle_rotate_and_signifiers.md §1` | 读源 + PDB |

### 2.4 矩形组

| 叶子 | 判定 | 缺陷（严重度 / 文件:行 / 证据） | 验证 |
|---|---|---|---|
| `ui_edit_rectangle_dims`（11 参, 0x140ad6250） | **FAIL（臆造壳）** | BLOCKER：当前 5 参（`ui_edit_rectangle_dims.rs:79-85`：`Res<RectangleDimsEditInput>, Res<InnerWalls>, ResMut<RectangleEditUiState>, ResMut<PreviewRectangle>, EventWriter<EditRectangleCmd>`）vs PDB **11 参**。臆造 `RectangleDimsEditInput`(:20，无 producer)。缺 AppMode/UiSignifierStream/UiState/CameraCursorRay/PublicWalls/WallEntities/WallTriggerVolume Query/MainCameraMoved/UiEditRectangleActiveWidgetResource/DoUiEditRectangleDims/Commands。证据 spec §10.1 / PDB :167-179 | 读源 + PDB |
| `do_ui_edit_rectangle_dims`（12 参, 0x140ad6e00） | **FAIL/PARTIAL** | HIGH：现 `do_` 为 7 平铺实参（`ui_edit_rectangle_dims.rs:38`），非 SystemParam。缺 PublicWalls/DecoratorArchivist/WipRectangle/Slider3d/GladeBorder/4 writer/DoUiEditRectangleDims reader。用 `expand_along_edge` 近似代替 `L²` 除零钳制+`t=((cur-p_i)·e)/L²`+Slider3d 投影+有效性校验。用臆造 `RectangleEditUiState([u8;80])` 写 UiState80。证据 spec §10.2 / `rectangle_and_adjust_height.md §3.3` | 读源 + PDB |
| `ui_edit_rectangle_rotate`（10 参, 0x140ad8000） | **FAIL（臆造壳）** | BLOCKER：当前 6 参（`ui_edit_rectangle_orientation.rs:214-221`）vs PDB **10 参**。臆造 `RectangleOrientationEditInput`(:159，无 producer)。缺 AppMode/PublicWalls/UiSignifierStream/UiState/MainCamera/CameraCursorRay/MainCameraMoved/ClosestCornerCache/DoUiEditRectangleRotate/Commands。证据 spec §11.1 / PDB :199-210 | 读源 + PDB |
| `do_ui_edit_rectangle_rotate`（12 参, 0x140ad8be0） | **FAIL** | HIGH：现 `do_` 7 平铺实参（`:176`）。(a) **ACOS_COEFFS 错**：`:19-21` 前 4 项 `[0x3bda90c5,0x3cd90a90,0x3c8bfc66,0x3dfd10f8]` ≠ RNE 实读 `[0xbaa57a2c,0x3bda90c5,0xbc8bfc66,0x3cfd10f8]`（V2 实证）。(b) **fast_acos 缺 π-a 负半支与 cross 符号选择**（`:167-173` 只做 `poly*sqrt(1-x)`，`:194` 用 `cross<0 ? -angle : angle` 替代原 `cos<0 ? π-poly : poly`）。(c) 缺相机投影取方向、朝向吸附/步进（FUN_1409ef010，步长 0.1）、有效性校验、GameTime/GladeBorder/4 writer/reader。证据 spec §11.2 / `rectangle_and_adjust_height.md §3.4` | 读源 + V2 + PDB |
| `box_trigger_volume` / `skewed_box_trigger_vol`（helper, 0x140ada0e0） | **PASS** | 24 点/36 索引/12 三角、负尺寸/零/非有限、wrapper 32 位索引——与 spec §11.3「审计判 EQUAL」一致；纯函数单测充分（`ui_edit_rectangle_orientation.rs:243-363`）。**保留** | 读源 + 单测 |

### 2.5 signifier_edit_dims / 应用与收尾

| 叶子 | 判定 | 缺陷 | 验证 |
|---|---|---|---|
| `signifier_edit_dims`（4 参, 0x1408dda10） | **FAIL（臆造）** | HIGH：当前 2 参（`signifier_edit_dims_line.rs:19-22`：`Res<PreviewRectangle>, ResMut<UiSignifierStream>`）vs PDB **4 参**应为 `Res<UiEditRectangleActiveWidgetResource> + Res<UiSignifierStream> + Query<(&EditRectangleLinePreview,&Handle<Mesh>,&SignifierAnimation,&mut Transform)> + ResMut<AssetMeshLibrary>`。用 `PreviewRectangle` 全局 + 自创「4 边」循环 + `pack2`，原证是 active widget 双分支（a>0 widget 构造 / a<=0 遍历曲线点 `+0x1a8` 表 0x48 步长，插值 `0.5/0.01`，u 范围断言 panic 0x142aa53d0）。证据 spec §12 / `rectangle_and_adjust_height.md §3.5` | 读源 + PDB |
| `edit_rectangle`（5 参, 0x14097f620） | **PARTIAL** | HIGH：当前 2 参（`edit_rectangle.rs:17-20`：`EventReader<EditRectangleCmd>, ResMut<InnerWalls>`）vs PDB **5 参**缺 `EventWriter<RequestWallMaterialAudio>`、`EventWriter<PlayBuildingAudio>`、`Res<Time>`。缺 0x20B material-audio 行、0x28B roof-ease 行（`inverse_lerp(0, rate=200/20, num/den)` + max(0)/min(1)）；`EditRectangleCmd` 缺 `+0x28 Vec3/+0x34 f32/+0x38 u8 flag`。已有 curve/enclosure 更新（正确保留）。证据 spec §13 / `rectangle_and_adjust_height.md §3.1` | 读源 + PDB |
| `finish_edit_rectangle`（9 参, 0x141464a60） | **PARTIAL/臆造布局** | HIGH：当前 4 参（`finish_edit_rectangle.rs:29-34`：`EventReader, ResMut<RectangleEditUiState>, Res<InnerWalls>, EventWriter<RecordWallHistory>`）vs PDB **9 参**缺 `ResMut<WipRectangle>`、`ResMut<DecoratorStorage>`、`ResMut<DecoratorArchivist>`、`EventWriter<OnRectangleEdited>`、`EventWriter<NotifyHintSystemShapeEditWasPerformed>`、`EventWriter<CmdCheckForSheepInsideBuilding>`。以臆造 `RectangleEditUiState([u8;80])` 代 `WipRectangle`。判据 0.01/57.29578 与 `edit_decision` 内核「EQUAL」。证据 spec §14 / `rectangle_and_adjust_height.md §3.2`；单测 `edit_decision.rs:129` 为手工预置 80B 变体的局部判据测试（非整链） | 读源 + PDB |

## 3. 注册树与事件图（跨叶）

判定：**FAIL**（未重建原 `add_wall_shape_ux` 的 a/b/c tuple 分组）。

- 现状：`run_game.rs:1134-1172 install_toolbar_shape_dependencies` 只装两条 5 系统链：
  - 链1 = `(shape_placement_params, shape_placement_signifiers, calc_shape_height, ui_create_circle, create_circle)`
  - 链2 = `(ui_raise_shape, ui_focus_mode_shape_height, impl_raise_shape, ui_create_rectangle, create_rectangle)`
  - `(链1,链2).chain_ignore_deferred().in_set(GameSystemSet(17)).run_if(is_gameplay_active)`
- 与原树（`shape-edit-registration.md §1`；a/b/c 为 B 版 `add_wall_shape_ux` 内的**局部变量 tuple 分组**，不是版本层）对比的缺口：
  - **a 缺** `ui_edit_circle, signifier_edit_circle_arrow, signifier_edit_circle_line, ui_rotate_circle`（a[3..7]）。
  - **b 整组缺**（`ui_edit_rectangle_dims, ui_edit_rectangle_rotate, ui_raise_shape, ui_adjust_wall_height` 组1 + `do_ui_edit_rectangle_dims, do_ui_edit_rectangle_rotate` 组2，且 b 必须用 `.chain()` 保留两组的 deferred 同步）。
  - **c 缺** `ui_move_shapes, ui_move_freehand_walls, ui_edit_freehand_walls, ui_adjust_wall_height`(经 b),`signifier_edit_dims, edit_rectangle, finish_edit_rectangle`；且原 c 是单条 11 元 `chain_ignore_deferred`，不是两条 ad-hoc 链。
  - 原最外层 `(a,c).chain_ignore_deferred()` 的跨 tuple deferred 语义被两条链分裂破坏。
- `is_gameplay_active`（`run_game.rs:1178-1185`）= `time.ran && *state.get()==AppLoadingState::Game`：**与原版等价，保留**（`shape-edit-registration.md §1.1`）。
- 事件完整性（V4/V6/V7）：
  - 死事件（注册但**无 producer**）：`EditCircleCmd`、`FinishEditCircleCmd`、`RotateCircleCmd`（`startup_event_runtime.rs:59/61/63` + `shapes/mod.rs:173/175/177`）。且这三个原版**不存在**。
  - 缺失事件类型（PDB 要求 writer 但类型零定义）：**仅** `OnFreehandWallEdited`、`OnCircleRotated`、`OnRectangleEdited`。**更正**：`OnShapeMoved`=`resources/events.rs:107`、`DisplayOutOfBorder`/`FeedbackInputOutOfBorder`=`resources/glade_border.rs:17/19`、`RecalculateRoofCmd`=`resources/roofs/commands.rs:40`（alias `RoofGeometryChanged`）**均已存在**，不判"未定义"。
  - 缺失事件类型：`DoUiEditRectangleDims`、`DoUiEditRectangleRotate`。
  - 无 producer 的臆造输入资源：`RectangleDimsEditInput`、`RectangleOrientationEditInput`（V5）；臆造资源 `CircleEditState`、`RectangleEditUiState`。
- 资源注册：`shapes/mod.rs:164-184 register_shape_resources` 初始化了自创事件/资源，**不是**原 9 资源（缺 `WipRectangle`、`AnimatedCircleSignifierResource`、`UiEditRectangleActiveWidgetResource`；`PreviewCircle/PreviewRectangle` 默认值虽对，但被 register 的是壳）。
- 重复注册风险（跨模块）：`registration.rs:229-242 add_wall_cpu_systems` 本地 schedule 装了 `create_circle/create_rectangle/edit_rectangle/ui_create_circle/ui_create_rectangle`；宿主 set17 又装 `create_circle/create_rectangle/ui_create_*`。**同一系统两处注册**（`edit_rectangle` 目前仅本地，但宿主树重建后必须二选一），否则 `EditRectangleCmd` 会被双消费。见 `shape-edit-registration.md §4`。

## 4. 测试真实性

- 4 个壳叶子本身**无测试**；其正确性无法由「测试通过」背书（且它们参数即错）。
- `edit_decision.rs:129 finish_produces_the_morph_premade_shape_payload`：手工构造 `RectangleEditState([0;80])` 变体 1 字节直接调判据函数——是**局部判据单测**，且 80B 布局为臆造；不证明整链，也未宣称整链（不算造假，但不可作验收依据）。
- `ui_edit_rectangle_orientation.rs:243-363`：仅 `box_trigger_volume`/`skewed_box` 纯函数测试（合格，EQUAL）。
- `ui_adjust_wall_height_mode.rs` 测试：用真实 `World` + `Events` 发送 `AdjustWallHeightUi`+`AudioCursorMoved`，`run_system_once` 后断言 `AdjustWallHeightCmd`/cache/归档——是**合格的局部链测试**（event→system→command/state），但不是 app 级整链。`history_record` 测试同理（发送 Finish、断言 history/hint/cache reset）。这些测试**未注入 clicked/伪造交互**，未放宽断言，可作本地证据，但仍需 B 原证对照（本文件已做参数/控制流对照）。

## 5. 动态审查状态

- `impl-status.md` 全为 `BLOCKED`/`PRE-EXISTING`，**无 `TESTED`**，故本次未跑 `cargo test`。
- 审查期间检测到实现 agent 的 `cargo`(PID 835, 09:57) / `rustc`(PID 80212, 09:58) 与 `target/.agent-build.lock`(09:54)。按协调线要求不并行占 Cargo，未取锁、未 rmdir 他方锁。
- 待 `impl-status` 出现 `TESTED` 后按锁协议补：`cd D:/game/tiny-glade && cargo test -p country-core --lib --locked --offline -j 3 <过滤>`，并自补一条真实链（事件→系统→权威 state/history），确认非实现 agent 自证。

## 6. 总体结论

可进模块验收（静态）：
- `ui_adjust_wall_height_mode`（12 参，PASS；L-R1 低风险）
- `history_record_adjust_wall_height`（7 参，PASS；L-R2 低风险）
- `box_trigger_volume` / `skewed_box_trigger_vol`（EQUAL，PASS）

必须返工（本叶）：
- **MISSING（无实现）**：`move_gizmo_and_trigger_vol`、`ui_move_shapes`、`ui_move_freehand_walls`、`ui_edit_freehand_walls`。
- **FAIL（已实现但原证对照不成立）**：`ui_adjust_wall_height`（F1 gate1 `being_bulldozed` 绕过 / F2 widget 0.7-vs-0.5 颜色值误当半径 / F3 signifier 空载荷，见 §8）。
- **FAIL 臆造壳/错参数（必须重写，删臆造事件/资源）**：`ui_edit_circle`（结构已重建，正文 PARTIAL）、`ui_rotate_circle`、`ui_edit_rectangle_dims`(+`do_`)、`ui_edit_rectangle_rotate`(+`do_`)、`signifier_edit_circle_line`、`signifier_edit_dims`。（`signifier_edit_circle_arrow` 已不在 FAIL 列，见 §13.2 → PASS）
- **PARTIAL（补参/补事件）**：`edit_rectangle`（补 2 writer + Time + ease）、`finish_edit_rectangle`（补 6 参 + 去 80B 臆造布局）。

跨模块依赖（不拦本叶静态判定，但拦模块整链验收）：
- PDB 要求的依赖类型/事件缺失：`OnShapeMoved`/`OnFreehandWallEdited`/`OnCircleRotated`/`OnRectangleEdited`/`RecalculateRoofCmd`/`AnimatedCircleSignifierResource`/`UiEditRectangleActiveWidgetResource`/`WipRectangle`/`Slider3d`/`DoUiEditRectangleDims`/`DoUiEditRectangleRotate`/`ClosestCornerCache`/`EditCircleSignifier`/`SignifierEditCircleShapeLine`/`EditRectangleLinePreview`/`Move*Cache`/`RotateCircleCache`/`CurrentAngleGizmo`。
- `AdjustWallHeightUi` 有**两个真实 producer**（`ui_focus_mode_shape_height` + `ui_adjust_wall_height`，PDB 双证）；二者都注册 `edit_height_{wall}`，同处 set17；是否存在双发/依赖 UiState 仲裁属 UNKNOWN（0x140a623e0 反编译缺失）。
- 本地 CPU schedule 与宿主 set17 树重复注册（`edit_rectangle` 等），需二选一。

注册树（跨叶，全局 FAIL）：必须按 `add_wall_shape_ux` 的 a/b/c tuple 分组精确结构重建（a/c `chain_ignore_deferred`、b `chain()`、最外层 `(a,c).chain_ignore_deferred()`、set17、两来源 `GameTime.ran && AppLoadingState==Game`），并删除三死事件与臆造资源注册。

## 7. UNKNOWN（证据不足，不猜测）

1. L-R1：`ui_adjust_wall_height_mode` 两个 EventReader 的游标消费先后（0x140ac6d60），与文档注释/测试命名需独立复核。
2. L-R2：`clamp_wall_height` 的 MAXSS/MINSS unordered-NaN 逐位语义。
3. 各 Cache/资源结构（MoveShapeCache/EditFreehandWallCache/RotateCircleCache/AnimatedCircleSignifierResource/WipRectangle 等）字段名与完整布局，多数规格仍标 UNKNOWN（见 `shape-edit-spec.md` 各节「缺证据」）。
4. `ActionSetStates` digital/analog 槽号（ui_move_shapes / ui_move_freehand_walls）。
5. `add_wall_events`（0x1404936b0）完整逐调用注册顺序尚未还原。

## 8. 动态审查第二轮（2026-10-28，TESTED 后）

> **时序说明**：§8.1–8.4 是**修正前（10:09 版本）**的独立复核，用于确立缺陷及其原证；
> §8.5–8.7 是**修正后（10:19 版本，实现方已落三处修正 + 51 测试）**的复核与最终判定。以 §8.7 为准。

`impl-status.md` 出现 `§1.1 IMPLEMENTED+TESTED (unit)`；`ui_adjust_wall_height.rs` 新文件已在 `run_game.rs:1157-1163`
装入 `GameSystemSet(17)+is_gameplay_active`。协调线点名三处自述假设；本次逐条用原证独立复核，修正前**三条均推翻实现自述**：

### 8.1 hypothesis 1 —— gate1 `entry[-9]`（修正前**成立为缺陷**；修正后已按证据修好，见 §8.7）
- 原版 `0x140b2fc00` 在命中墙桶后：`if *(char*)(lVar15 + lVar40 + -9) != '\0' goto code_r0x000140b3006b;`（`work/m01-scratch/adjh_body.c:462`）——真实分支。
- 定位该字节：hashbrown 桶 stride `0x2a8`，key 8B @`-0x2a8`，值记录 0x2a0B；`entry[-9]` = 值记录 **byte 0x297**。
- 独立证据：`public_wall_state.rs:363` `bytes[0x297] = self.being_bulldozed as u8;`（`building_projection()` 写 672B/0x2a0 记录）；写入点 `publish_public_wall_state.rs:161 state.set_being_bulldozed(bulldozing.contains(id));`。
- 结论：`PublicWallState` **确有**等价字段 `being_bulldozed`（`is_being_bulldozed()`），且发布路径真实写入；语义 = "正在铲除→不可(高度)编辑"。实现 `:151-157` 以"无该字段"为由**恒当可编辑**是**默认值绕过**；铲除中的墙与原版控制流分叉。→ **FAIL**（非 PARTIAL）。

### 8.2 hypothesis 2 —— `WIDGET_RADIUS=0.5`（**未获原证，且取错分支，FAIL**）
- `0x1409566d0 UiWallHeight::widget` 反编译无 0.5 半径常量；仅有抬起常量 `+0.6 / +0.0 / -0.29000002`（`1409566d0__widget.c:48/55/65/130/136/144`）。
- 协调线/实读：正常路径标量 = `0x142ab3c08[shape_tag<2]`；RNE 实读 **`0x142ab3c08=0.69999999`、`0x142ab3c0c=0.5`**（`_probe_radius.py`）。该标量是**颜色/env 值，不是命中半径**；命中几何来自抬起曲线点。
- 索引运算：`shape_tag>=2`（gate2 的可达分支必然满足）→ `shape_tag<2`=false=index 0 → **0.7**。实现恒取 0.5（`:36-37,69`，照抄 `ui_raise_shape.rs:144` 的 0.5）→ 值错 + 语义错（把颜色/env 值当 SphereCloud 半径）。→ **FAIL**。

### 8.3 hypothesis 3 —— signifier 载荷（**成立为缺陷，FAIL**）
- `0x140956cd0 UiWallHeight::signifier` 正常分支 push 记录 `{+0 u64=1, +8 u64=1, +0x10 u64=WallId, +0x18 u32 angle}`，`angle = state+8==0 ? 0.2 : 0.0`；前置门 `ctx+0x18==0 || ctx+0x38!=ctx_ref || ctx+0x28==2 → return`。
- 实现 `:96 stream.write_factory(&SignifierFactory::new(name.as_bytes(), std::ptr::null(), 0))`：name-only，丢弃 WallId 与 angle（且 `emit_signifier` 传入的 `uh.points/heights` 为空）。违反 SIGNIFIER-CONTRACT「真实载荷、不得空实体」。→ **FAIL**。

### 8.4 更正：两条 producer 是原版设计，非重复造物
`ui_focus_mode_shape_height`（0x140a623e0）的 PDB `drop_in_place<..FunctionSystem<fn(..)>>` 签名含
`ResMut<UiState>` + `EventWriter<FinishAdjWallHeightMode/AdjustWallHeightUi/InitAdjWallHeightMode/HandleRaiseShapeUiInteraction>` +
`Res<UiSignifierStream>` + `Res<GameTime>` + `Local<LerpedGizmoPosition>`（`skeleton/A/.../ui_focus_mode_shape_height.rs:1015` 与
`public-symbols.json:206374`）。即 B 确实有**两个** producer（focus-mode gizmo 与直接 height widget），二者都注册 `edit_height_{wall}` 并同处 set17。
因此 `AdjustWallHeightUi` 早在本轮前就有 producer（`ui_focus_mode_shape_height.rs:222`），impl-status 早期"无 producer"的说法不成立；本轮新增 `ui_adjust_wall_height` 是对第二个真实 producer 的补全。**不判重复 writer**。

### 8.5 测试真实性
`ui_adjust_wall_height.rs` 的 6 个测试用 `set_interaction()` 直接预置 `UiState`（native_tag/native_phase/widget），
**绕过 picker→UiState 注册链**；只验证 stage→三事件分派与 gate2，不验证 gate1/widget/signifier。属"局部+预置 interaction"，不能作整链证明。

### 8.6 动态复跑状态（已执行）
- 构建锁：审查方 runner `work/m01-scratch/review/_run_tests.sh` 于 10:19:08 取得 `target/.agent-build.lock`（实现方已释放），串行执行后 `rmdir` 释放。
- **`cargo test -p country-core --lib --locked --offline -j 3 adjust_wall_height`** → `test result: ok. 51 passed; 0 failed; 0 ignored; 2852 filtered out`，`TEST_EXIT=0`
  （日志 `work/m01-scratch/review/logs/test_adjust_wall_height.log`）。
- **`cargo check -p tiny-glade --tests --locked --offline`** → `Finished dev profile ... in 17.59s`，`CHECK_EXIT=0`（仅既有 unused-import 警告；日志 `logs/check_tiny_glade.log`）。
- **独立物理链：未执行（诚实记录，缺 harness）**。核查：`crates/tiny-glade/tests/toolbar_physical_chain.rs` 只驱动 toolbar/mode（真实 `MouseButtonInput`+`InputCursorMoved`→picker→`ui_update`→toolbar），**不插入任何墙**、不注册 `edit_height_{wall}`/`ui_adjust_wall_height`；`original_gameplay_chain.rs` 虽有墙但不注册形状-高度系统；全 `crates/tiny-glade/tests/` grep `ui_adjust_wall_height|edit_height_|AdjustWallHeightUi` = 0 命中。要驱动该叶子需**新增**一个 test app：真实 `InnerWalls+PublicWalls` 墙 + `AppMode::Editing(EditSubject::Wall)` + set6 `hover_intersection`→`ui_update` 让 `edit_height_{wall}` 成为 hover/active 控件 + 把 `ui_adjust_wall_height`/`_mode`/`adjust_wall_height_impl`/`history_record_adjust_wall_height` 放同一 schedule，再喂 `MouseButtonInput`/`InputCursorMoved` 产生 stage 3/4/5，断言 `InnerWalls` 高度与 `History`。该文件必须放在 `crates/tiny-glade/tests/`（生产树）——本次只读审查**禁止新增生产文件**，故未执行；应以直写 `UiState` 的单元测试冒充。**交由实现方补该链**。
- 已静态确认事件图闭合（类型级）：`ui_adjust_wall_height`→{AdjustWallHeightUi,Init,Finish}；`ui_adjust_wall_height_mode`←AdjustWallHeightUi→`AdjustWallHeightCmd`；`adjust_wall_height_impl`←AdjustWallHeightCmd→`InnerWalls.update_wall_state_height`（`:144`）+音频；`history_record_adjust_wall_height`←Finish→`History`。**但这只证明接线类型闭合，不等于单世界整链已跑通。**

### 8.7 三件套最终判定
- `ui_adjust_wall_height`：**TESTED**（逻辑三腿 + 墙高 signifier 画面均 PASS，§11；§7 arrow 已 PASS（§13.2））。
- `ui_adjust_wall_height_mode`：**PASS**（12 参 + 控制流；L-R1 低风险）。
- `history_record_adjust_wall_height`：**PASS**（7 参 + 控制流；L-R2 低风险）。
- **adjust 三件套整体：TESTED**（`_mode`/`history_record` 单叶 PASS；`ui_adjust_wall_height` TESTED，见 §11；§7 arrow 已 PASS（§13.2），不属本三件套）。

## 9. widget 几何三条直接证据（供分析线逐条裁决）+ 双 producer 双发核对

证据源：`crates/country-core/recovery/agent-team/work/restorer-walls5/decomp/1409566d0.txt`（0x1409566d0 = `UiWallHeight::widget`）；
帧基址 `1409566e3 LEA RBP,[RSP+0x80]`（下均 RBP 相对）；`UiState::add` 调用 `140956c1f CALL 0x1409231f0`，
**params 基址 R8 ← `140956c18 LEA R8,[RBP-0x58]`**，**name 指针 RDX ← `140956c11 LEA RDX,[RBP+0x90]`**。

### 9.1 几何判别字位置（code_0x140956b11 附近）
```text
140956af8  XOR ECX,ECX
140956afa  CMP dword ptr [R8 + 0x258],0x2        ; shape_tag = *(u32*)(wall+0x258)
140956b02  SETC CL                               ; CL = (shape_tag < 2)
140956b05  LEA RDX,[0x142ab3c08]                 ; [0.7,0.5] 表
140956b0c  MOVSS XMM6,dword ptr [RDX + RCX*4]    ; env = [0.7,0.5][shape_tag<2]
140956b11  MOV byte ptr [RBP + 0x70],0x1         ; ★ tag 字节 = 1（SphereCloud；TriangleMesh=2）
140956b15  MOVSS dword ptr [RBP + 0x74],XMM6     ; ★ 标量（env）紧随 tag
140956b1a  MOV qword ptr [RBP + 0x78],R12        ; 长度
140956b1e  MOV qword ptr [RBP + 0x80],RAX        ; 指针（alloc 结果）
140956b25  MOV qword ptr [RBP + 0x88],R12        ; 长度
...
140956bed  MOV word ptr [RBP + 0x20],0x202       ; ★ 相对 params(RBP-0x58) = +0x78
140956bf3  MOV qword ptr [RBP + 0x24],0x0        ;   +0x7c
140956bfb  MOV dword ptr [RBP + 0x2c],0x0        ;   +0x84
```
- `0x202` 位于 **params+0x78**（`0x20-(-0x58)=0x78`），与 `widget_params.rs` 文档「+0x78..0x88 brush-preview enum，tag0/1/2」一致 → **brush-preview enum，非几何判别**。
- 唯一小 tag 字节是 `[RBP+0x70]=1`，**值 1 = 复现枚举 `SphereCloud`**（`native_tag()`：Sphere0/SphereCloud1/TriangleMesh2），其后紧跟 f32 标量 + {len,ptr,len}，正是 `SphereCloud{centers:Vec<Vec3>, radius:scalar}` 形状；**无 tag=2 记录**。
- **歧义（交分析线裁决）**：`[RBP+0x70]` 相对 add-params(RBP-0x58) = **+0xc8**，超出 `widget_params.rs` 记的 `0xb0` 参数块；该 {tag,scalar,Vec} 记录在 b11–b6f 间先用于 `format_inner`（`140956b7e CALL 0x140d79f90`），add 的 params 在 140956bc1–c0a 另建。故「该 tag 记录是否就是传入 add 的 geometry」在本函数体内不能百分百钉死；但**能钉死：本函数构造的几何记录 tag=1（SphereCloud 形），不存在 tag=2**，故「原版 widget=TriangleMesh(tag2)」在其中无支撑。

### 9.2 载荷 stride
```text
14095671e  MOV R15,qword ptr [R8 + 0x90]   ; points ptr
140956725  MOV R12,qword ptr [R8 + 0x98]   ; points len
14095672f  CMOVC R12,RAX                   ; count = min(len_points, len_heights)
140956733  LEA RAX,[R12*0x4]               ; count*4
14095673b  LEA R14,[RAX + RAX*0x2]         ; R14 = count*12  ← alloc 大小 = 12B/点
14095678d  CALL 0x140613c10                ; __rust_alloc(R14)
```
拷贝循环每点写 3×f32（`MOVSS [RAX+R9*4-0x14]/-0x10/-0xc`，1409568c0–925）。**stride = 12 B/点 = Vec<Vec3>**，非 36 B/triangle → 与 `SphereCloud{centers:Vec<Vec3>}` 一致，与 `TriangleMesh(Vec<Triangle>)`(36B) 不符。

### 9.3 `0x142ab3c08=[0.7,0.5]` 的其它读者
- 在现有 evidence 范围内**只有 0x1409566d0 读它**（`140956b05 LEA RDX,[0x142ab3c08]`）。其余命中均为同一反汇编副本（`impl-downstream/dis-0x1409566d0.txt`、`restorer-walls5/1409566d0.{asm,txt}`、`disasm*.json`、`uihelper.*`）或 markdown 记录（`restorer-walls5-REBUILD5.md`、`audit/adjust_height.md`、`shape-edit-spec.md`）。**无其它函数体读它**。
- 结论：不是跨系统共享颜色表，而是 widget 内**局部 [0.7,0.5] 标量对**（按 `shape_tag<2` 取 0.7/0.5，存入几何记录的标量槽）。「env/颜色标量」语义成立，「共享颜色表」不成立；它确实占用 SphereCloud 的 radius 数据槽（原版布局如此，实现照此放置）。

### 9.4 双 producer 双发核对（已裁决：原版行为，非缺陷）
- PDB 双证二者都含三 writer（`ui_focus_mode_shape_height` 的 `drop_in_place<..FunctionSystem<fn(..)>>` 含 Finish/Adjust/Init + HandleRaise + Local<LerpedGizmoPosition>）。
- 协调线/分析线已裁决：**原版确实有两个 producer、同一 `edit_height_` 前缀、focus 也写三事件 → “双发”是原版行为，不是复现错误。** 消费者 `_mode` 用 `.read().last()`、`history_record` 用 `.read().count()>0` 本就容忍重复；`InitAdjWallHeightMode → signifier_audio.rs:170` 的“双触发”也随之是原版语义，不再计为缺陷。
- 因此本条从风险清单**降级移除**；不再建议实现线加互斥门控。

### 9.5 已关闭 UNKNOWN：focus 与 adjust 确为同一 widget id（原版行为）
- 分析线实读 `0x140a623e0` 的 `pcStack_108` = AppMode 第二 qword = wall id → focus 与 adjust 注册到**同一 `edit_height_{wall}` WidgetId**。
- 原版按注册顺序覆盖命中几何：原树 focus 在 adjust 之后 → 最终命中体是 focus 的 `Sphere{0.5}`，adjust 的 `SphereCloud` 被覆盖。**记为原版行为，非缺陷**（已裁决）。
- `ui_focus_mode_shape_height` 不在 15 叶内；不再保留 UNKNOWN。

### 9.6 裁决记录
- 几何分歧已由协调线裁决：**我方证据胜出**；分析线撤回 TriangleMesh，确认 `SphereCloud{12B/pt, radius=[0.7,0.5][shape_tag<2]}`，`0x202` 属 `extra_78`，`FUN_140b34ec0` 仅 0x60B 只产名字。→ §9.1/§9.2 判定由 UNKNOWN 升为 **CONFIRMED：几何正确（非缺陷）**。
- 双 producer 双发：**原版行为，非缺陷**（§9.4 已降级）；仅保留“同一 widget id 与否”的 `pcStack_108` UNKNOWN（§9.5）。
- 剩余项已闭合（按 §11）：signifier = `SignifierMesh::Empty`（= 原版 `"signifier empty"` @0x142a99570，PASS）；undo/redo 用真实 `HistoryCommand::UNDO/REDO` + 400 帧动画回放（PASS）；clamp 已修（§10.3）。
- **adjust 三件套：TESTED**（`_mode`/`history_record` PASS，`ui_adjust_wall_height` TESTED §11）。

### 9.7 F4（已由实现线修复；本次复核通过）
> 历史缺陷（10:29 版本）：`emit_signifier` 守卫比较 `WidgetId(wall.0)` ≠ 注册 id → 永不发射。**现已在 10:36 版本修复**：
- 源码现状（`ui_adjust_wall_height.rs:101-107`）：`emit_signifier` 守卫 `ui_state.active_widget() != WidgetId(wall.0)`。
- 但 widget 注册是 `register_widget` → `ui_state.add(&uh.widget_name(), ...)`，`widget_name()=format!("edit_height_{wall}")`；其 id 是 `WidgetId::from_name("edit_height_{wall}")` 的哈希（同文件 `:293` 的 `widget_id()` 用 `from_name`）。`WidgetId(wall.0)` 是“以 WallId 数字当 id”，与 `from_name` 哈希**永不相等** → signifier 分支**永远提前 return**。
- 正确对象 = `register_widget` 返回的 `context.id`（**调用点 `:196` 的 `widget_active` 已正确用 `context.id`**）；修复应把 `context.id` 传入 `emit_signifier`（或内部用 `WidgetId::from_name(&uh.widget_name())`），与注册 id 一致。
- 影响：即使几何/闭包正确，signifier 也永不产生 → B 的 `wall_height_edit_{wall}` 线不可见。physical_chain 测试**不覆盖** signifier（只断言 adjust/init 事件），故不会捕获此缺陷。
- 修复确认（本次静态复核）：`emit_signifier(..., widget: WidgetId, ...)` 守卫改 `ui_state.active_widget() != widget`，调用点 `:219 emit_signifier(&ui_state, &mut stream, wall, context.id, lifted)` 传入 `context.id`（= `register_widget` 里 `ui_state.add(&uh.widget_name(), ...)` 返回的注册 id）。**比较对象 = 注册 id ✅**，该分支不再死路。

### 9.8 physical_chain 测试独立复核（已跑）
- 测试：`physical_chain::real_mouse_press_reaches_adjust_and_init_events_through_the_picker`（定义在 `ui_adjust_wall_height.rs:434` 的 `mod physical_chain`，**非** `crates/tiny-glade/tests/`）。
- 真实性：`setup()` 建真墙（`InnerWalls::new_wall_from_curve`+`PublicWalls::consume_with_paths`）、`AppMode::Editing(Wall)`、`MainCamera`(恒等矩阵写入 0x40/0x80/0xc0)、`WindowSize`、`UiState::at_startup()`（**无预置 interaction**）、`input::register_input_events` + `input::add_input_cpu_consumers`；schedule=`(hover_intersection, ui_update, ui_adjust_wall_height).chain()`；`frame()` 设真实 `MouseButtonInput` press/release + `InputCursorMoved::Absolute` + `Time::advance_by(16ms)`。
- 范围：只断言 `hover_byte>=2` 与 `AdjustWallHeightUi`/`InitAdjWallHeightMode` 各 ≥1（stage 3）；**不**驱动 release→finish，**不**含 `_mode`/`impl`/`history_record`，**不**断言 `InnerWalls` 高度或 `History`，**不**断言 signifier。相机为恒等矩阵（合成但可用）。
- 独立跑测（构建锁内，`rmdir` 释放后）：`cargo test ... real_mouse_press_reaches_adjust_and_init_events_through_the_picker` → **1 passed / 0 failed**；`cargo test ... adjust_wall_height` → **55 passed / 0 failed**（10:43/10:47 版本）；另 `edit_arrow` → 5 passed。
- 判定：该测试是真物理 picker→events 链（无预置 UiState），**合规但有界**；它证明 picker→ui_update→ui_adjust_wall_height→adjust/init 成立，**不**证明下游 InnerWalls/History，也**不**覆盖 F4。

## 10. native_phase 与 History(finish) 腿独立核验（决定性）

### 10.1 重建版 `ui_update` 的 native_phase 写点（Q1）
- `enter_hover`：tag=1, phase=1（`ui_update.rs:71`）；press（just_pressed）：tag=2, phase=3（`:186`）；hold（tag==2 phase3|4 非 force_finish）：phase=4（`:178`）；release/force_finish：phase=`u8::from(hit==widget)`=**0/1**（`:167`）；phase==2 保持；其余→enter_hover(1)。
- **重建写域 = {0,1,3,4}，无 5**（与 impl 观察一致）。

### 10.2 原版 clamp 与 release（决定性）
- 原版 `0x140b2fc00 @140b3047b`：`movzx eax,[ctx+0x19]; add al,0xfe; cmp al,3; cmovb` → **无符号回绕** `clamp(stage-2,0,3)`。映射：stage **{0,1,≥5}→3(finish)**；2→0(无操作)；3→1(adjust+init)；4→2(adjust)。
- 原版 release 写 phase=0/1（`ui_update 0x140b2de60 @140b2e7d6`：`phase=(hit==widget)`），tag=2 → 下一 set 帧 `ui_adjust_wall_height` 读到 stage=0/1 → **finish 触发**。

### 10.3 重建版 clamp 缺陷（已修复，复核通过 → PASS 候选）
- 历史（10:29）：`stage.saturating_sub(2).min(3)` 把 release stage 0/1 → 0（应为 3/finish），丢弃 finish。
- **已修复（10:43 版本）**：`ui_adjust_wall_height.rs:226 let dispatch = stage.wrapping_sub(2).min(3);`。逐值核对：0→254→3、1→255→3、2→0、3→1、4→2、5→3、6→3、254→3、255→3，= 原版 `{0,1,≥5}→3, 2→0, 3→1, 4→2`（`add al,0xfe;cmp 3;cmovae`）✔。

### 10.4 对 impl 结论的判定（曾 FAIL；已按要求修复）
- impl 曾称"native_phase 永不 5 → case3(stage>=5) 不可达 → History 腿无法跑"：**观察对、结论错**。finish 不是靠 stage>=5，而是靠 release 的 phase∈{0,1}（原版无符号 clamp 映射为 3）。真正断点是重建 clamp 用 `saturating_sub` 把 0/1 误映射为"无操作"。→ History 腿并非不可达，而是被 clamp bug 丢弃。
- **现已修复**：clamp 改 `wrapping_sub`（§10.3），release→finish 测试通过（§10.7）。

### 10.5 Q3：其它写 phase 的系统
- 生产写点仅 `ui_update`（+ `UiState::replace_interaction_full`/`replace_interaction`）；其余 `.native_phase=`/`UiInteraction{}` 命中均为 tests 与 `debug/sheep_controls.rs`（非生产）。→ **ui_update 是唯一生产交互 phase 所有者**。

### 10.6 残留 UNKNOWN（分析线已解，保留登记）
- 分析线复核：**`5` 从不被写入；phase 3/4 由 drag 工具经内联 commit helper 写入**；repo 把该写入收敛进 `ui_update` 属**结构性简化**，经无符号 clamp 后对 {0,1,3,4} 行为等价。→ 本趟"未定位 3/4 直接写点"属上述简化所致，**不构成缺陷**；**不据此刻改共享 `ui_update`**（保持）。
- 故 §10.3 的 clamp 缺陷结论**不受影响、成立**。

### 10.7 修复复核结果（10:47 跑测）
- **编译**：`cargo check -p country-core --tests` → `Finished`（此前 E0063 `AnimatedCircleSignifierResource` 缺 `pad_1c/pad_39/unknown_00` 已修）✔。
- **逐值映射**：`stage.wrapping_sub(2).min(3)`（`:226`）= 原版 `{0,1,≥5}→3, 2→0, 3→1, 4→2` ✔（§10.3）。
- **release→finish**：`release_phases_zero_and_one_emit_finish` **both 0 and 1**（`for phase in [0u8,1u8]`），断 `counts==(1,0,0)` → finish 各 1 ✔。**缺口**：`counts()` 只计事件数，**未断言 `FinishAdjWallHeightMode(WallId)` 的 WallId 载荷**（功能代码 `finish.write(FinishAdjWallHeightMode(wall))` 正确，但测试未验）；且 `stage≥5→finish` 无显式测试（仅 0/1）。
- **History 腿**：`real_input_reaches_inner_walls_height_and_history_through_the_chain` → 真物理链 release→finish→`history_record`→`History`，断言 `InnerWalls` 高度变、`history.pending()` 非空、`finish_input_recording()` 提交出可撤销项 ✔。**但**：未跑 `history_process` 的 **undo/redo 应用腿**（把 EditType 反放回世界），测试注释亦承认该腿另算 → **"存在可撤销项"已证，"真正撤销/重做可回放"未证**。
- **测试计数**：`cargo test ... adjust_wall_height` → **55 passed / 0 failed**；`edit_arrow` → **5 passed**。
- **结论**：clamp 缺陷 **PASS**；§1.1 整体 **TESTED**（逻辑三腿 + 墙高 signifier 画面，见 §11）；§7 arrow 已 PASS（§13.2）。

### 10.8 §7 arrow 复核（11:42 版本，已 PASS；旧 10:43 PointsWithColors 结论作废）
- 已落地：`SignifierMesh::Named("flat_arrow")` + `vertex_color_shader`；name `edit_circle_{:?}_arrow`，`WidgetId` 取资源 `+0x00`；组件 `ComponentRegistration{0x58,TypeId,name}` 非空；`marker=0x142aed888`。独立跑测 `edit_arrow` **5 passed**（§13.2）。
- 细微：live 资产解析（FUN_14092d570 缺失 panic）不在单测覆盖；结构字段名仍 `unknown_00`（+0x00 语义=WidgetId，建议具名，非功能缺陷）；**未调度进 set17**（属最后挂 a/c 组步骤）。→ **PASS**。

## 11. §1.1 `ui_adjust_wall_height` TESTED 签核（2026-10-28）

### 11.1 跑测（锁内，rmdir 释放后）
- `cargo test -p country-core --lib adjust_wall_height` → **55 passed / 0 failed**。
- `cargo test ... real_input_reaches_history_and_undo_redo` → **1 passed**。
- `cargo check -p tiny-glade --tests --locked --offline` → **Finished（exit 0）**。

### 11.2 三腿逐项
- **传输腿（picker→ui_adjust → 三事件）**：**PASS**。`physical_chain` 真输入（无预置 UiState），press=stage3→adjust+init、drag→adjust。
- **权威腿（AdjustWallHeightUi→mode→AdjustWallHeightCmd→impl→InnerWalls 高度）**：**PASS**。`real_input_reaches_inner_walls_height...` 经真实 `ui_adjust_wall_height_mode`+`adjust_wall_height_impl`（非直接改 InnerWalls），断言高度变化。
- **历史记录腿（Finish→history_record→History 记录并提交可撤销项）**：**PASS**。`real_input_...history` 断言 `history.pending()` 非空 + `finish_input_recording()` 提交 Some。
- **历史 undo/redo 应用腿**：**PASS**（已改用真实 `HistoryCommand::UNDO`/`REDO` 常量 + 真实动画回放至完成，§11.3）。

### 11.3 undo/redo 命令字节：已改用真实常量（PASS）
- 真实编码（`mode_manager.rs:45-52` 常量 + 真物理链 `toolbar_physical_chain.rs:317`）：`UNDO=[0,0,0,0]`、`REDO=[0,1,0,1]`、`INTERRUPT=[0,2,0,2]`。
- **修正后（10:57 版本）**：测试 `ui_adjust_wall_height.rs:747` 发 `HistoryCommand::UNDO`、`:761` 发 `HistoryCommand::REDO`（即上述常量），**无 byte2 自造**。每次随后 `for _ in 0..400 { tick() }` 驱动真实动画回放至完成。
- replay **路径真实**：schedule 含 `history_process_system::<()>` + `replay_wall` + `pop_empty_replay_bucket`（`setup_full`），由 `HistoryCommand` 事件驱动，非直接改 InnerWalls。→ **PASS**。

### 11.4 三态断言 + finish 载荷（已核）
- `real_input_...history`：编辑后 `InnerWalls` 高度 `!=0`；`from/to` 取自历史 `AdjustHeight`；`HistoryCommand::UNDO` + 400 帧后 `height≈from`、`REDO` + 400 帧后 `height≈to` → **三态覆盖**（编辑后≠原值 → undo 回 from → redo 回 to）✔。
- finish 载荷：`high_stages_five_six_and_wrap_emit_finish_with_wall_payload` 对 stage 5/6/254 断言 `payloads==vec![wall]` ✔；`release_phases_zero_and_one_emit_finish` **现已断言** `payloads==vec![wall]`（`ui_adjust_wall_height.rs:375-380`，phase 0 与 1 各断言 WallId 载荷）✔。

### 11.5 §1.1 最终签核
- **逻辑三条腿（传输 / 权威 / 历史含真实 undo/redo）→ PASS**：跑测 `adjust_wall_height` **55 passed**、undo/redo **1 passed**（真实 `HistoryCommand::UNDO/REDO` + 每次 400 帧动画回放 + 三态断言）、`tiny-glade --tests`（compile）**Finished exit 0**；finish WallId 载荷断言齐（stage 5/6/254 与 release 0/1 均验 `payloads==vec![wall]`）。
- **画面腿（墙高 signifier）→ PASS（本次独立复核）**：原版 `UiWallHeight::signifier`（0x140956cd0）就是 **`"signifier empty"` 内置 empty 资产**（RNE 实读字符串 @`0x142a99570` = `"signifier empty"`），identity 朝向 + `scale 1.5` 的**固定 handle**，不使用墙几何/墙色；`angle`/`WallId` 仅作身份。实现 `SignifierEntityBuilder::new(SignifierMesh::Empty).rotation(IDENTITY).scale(1.5)` + marker `0x142ab4830` = **正确实现，非占位/已知限制**（此前"PARTIAL/已知限制"判断据此更正为 **PASS**）。
- **§7 `signifier_edit_circle_arrow` 仍 OPEN**（点集 `points=vec![pos]`、hash 源 TBD、未调度）——**属独立叶，不阻断 §1.1**。
- **最终结论：§1.1 标 TESTED**（逻辑三腿 + 墙高 signifier 画面均 PASS）；§7 arrow 另行闭合。
- **TESTED 边界（务必随报告保留）**：本 TESTED 仅指 **in-source 真实 CPU 链**（country-core 内 `ui_update`/picker→`ui_adjust_wall_height`→`ui_adjust_wall_height_mode`→`adjust_wall_height_impl`→`history_process_system`→`replay_wall`/`History`，含真实命令常量与动画回放）。它**不代表**真实窗口/OS 输入注入、GPU 画面呈现、或新进程存档恢复（这些未在本叶验证）。

### 11.6 后续审查基准（协调线 2026-10-28，采纳）
- **不再追 wrapper ABI**：SystemParam 顺序/类型以 `skeleton/B` 的 FunctionSystem 类型拼写为准（PDB 已给有序类型）；"未做 wrapper 追踪"**不判缺陷**，wrapper 降为可选确认。
- **§3 `ui_move_shapes` 将 READY**：实现后重点审 **30 参顺序 vs PDB**、`ActionSetStates` 读取偏移、**9 个 writer 各有真实 producer/consumer**。
- **§7 arrow 仍 OPEN**：待分析线补点源与 hash 源后复审。
- 每叶通过后仍由审查方**独立跑受影响闭包**；实现线持锁期间**不抢 Cargo**。

## 12. §3 `ui_move_shapes`：ActionSetStates 读取已定位（digital[2]）+ 实现骨架复核（2026-10-28）

**更新（11:16 版本，本次复审）：`ActionSetStates` 读取已定位且**已接入**——`camera_rotate_pressed = (digital[2].phase as u8) < 3`（`ui_move_shapes.rs:261-262`），在 `move_gizmo_and_trigger_vol` 内调用（`:280`）；`digital[2].phase<3` 单测通过。case3 已实现并测试（`ui_move_shapes` 4 passed）。case1/case2、move_gizmo 的 box_trigger/UiState::add/axis 仍 TODO → **§3 = PARTIAL（非整体 PENDING）**。**

### 12.1 读取已定位（本次独立 disasm 复核）
- `rne_dis` 实读 `0x14097D360..0x14097D400`，复核分析线给出的序列：
  - `14097d382 mov rax,[rbx]`
  - `14097d385 mov ebx,[rax+0x10]`（读 ActionSetStates 字段）
  - `14097d3ef xor eax,eax` · `14097d3f1 cmp ebx,3` · `14097d3f4 setae al`（phase 比较）
  - 该序列位于 helper `move_gizmo_and_trigger_vol`（VA `0x14097d280` 起，`0x14097D3EF` 在其函数体内）。
- PDB `S_INLINESITE inlinee=0x6A2A (DigitalActionState::pressed)` 落在 move_gizmo；槽 2 = `DIGITAL_TARGETS (37,2)` 鼠标 Rotate 绑定。→ **读取 = `ActionSetStates.digital[2].pressed()`（phase 比较）**，确认非未使用。
- 结论：此前"未读取"否证已被肯定性证据取代；读取点在 **helper** `move_gizmo_and_trigger_vol`（非 ui_move_shapes 直体）——实现须把 ActionSetStates 传入 helper 或等价读取该槽。

### 12.2 反编译槽位观察（排除"完全未触碰"的简单解释）
- 推定 ActionSetStates 槽（fastcall arg8 = `in_stack_00000040`）在反编译中仅作**复用局部**：`:1282` 用作 Vec2 锚点、`:1763-1764` 用作相对跳表基址。→ `in_stack` 名被局部复用，**不能**据此证明 arg8 是 ActionSetStates 且未读。
- Modifiers(#29, arg14=`in_stack_00000070`) 读取为 `*(char*)*in_stack_00000070`（`:1763`），与 ActionSetStates **不同参数**，未混淆。

### 12.3 30 参顺序
- `skeleton/B .../ui_move_shapes.rs:2470` 给出有序 `FunctionSystem<fn(...)>` 类型拼写（顶层 15 组 = 30 SystemParam），与 `m01-system-params-from-pdb.md §ui_move_shapes` 一致。顺序真值以此为准。

### 12.4 9 个 writer
- PDB EventWriter（#16-22 + #26-27）：`RequestWallMaterialAudio, PlayBuildingAudio, PlayWallUiSound, OnShapeMoved, CmdCheckForSheepInsideBuilding, HideTreesinAreaCmd, ShowPinnedDecoClutter, DisplayOutOfBorder, FeedbackInputOutOfBorder`。**类型均已存在**（`OnShapeMoved`=events.rs:107，`DisplayOutOfBorder`/`FeedbackInputOutOfBorder`=glade_border.rs:17/19）；**producer 侧目前仅 `OnShapeMoved`/`ShowPinned` 2/9 实际 `.write()`，其余 7 个仅签名未发射（12:02）**；consumer 亦未逐一核（部分为后序模块）。

### 12.5 实现复核 + 判定（11:16 版本）
- `ui_move_shapes.rs`：**30 参顺序与 skeleton/B 完全一致**（#10 用 `LiveDecoratorArchivist` 同型别名）。
- **`digital[2]` 已正确接入** ✓：`camera_rotate_pressed(actions) = (actions.digital[2].phase as u8) < 3`（`:261-262`），在 `move_gizmo_and_trigger_vol` 内 `let camera_rotate = camera_rotate_pressed(actions)`（`:280`）并据此抑制 gizmo；单测 `camera_rotate_reads_digital_slot_2_phase` 覆盖 JustPressed/Held/Repeating→true、JustReleased/Idle→false。与 disasm `0x14097D382 [rax+0x10]` + `0x14097D3EF cmp ebx,3` 一致。
- **case3 已实现** ✓：`commit_move_shape`（`:107-140`）`update_wall_state_curve` 写 InnerWalls → 组 `StateChange{previous,next}` compound（`history.add(EditType::Wall…)`，native edit_type=7 模型）；case3 分支写 `OnShapeMoved(wall)` + `ShowPinnedDecoClutter`（`:223-236`）。单测 `case3_commit_updates_wall_and_records_undoable_history` 过。**细节 TBD**：`observed_operation` slice [0,1] 非原 op tag（注释承认）。
- **case2 已实现** ✓（11:27）：dispatch case2 调 `hover_snap`→`nearest_proposal`（现有 `WallSpatialHash` helper，非自造）→ roof offset（tag2→-6/tag1→-8）→ `DebugSphere`/`DebugLine`，无 `InnerWalls` 写。
- **仍 TODO → PARTIAL/BLOCKED**：case1（`:219`，6-f32 配对 → **BLOCKED**）；move_gizmo 的 `box_trigger_volume(base*1.5,0.5,base*1.5)`、`UiState::add`、`move_shape`/`move_shape_outline` axis signifier（现返回空 Vec）；public tail 占位 anchor/rotation。
- **判定：§3 = PARTIAL**（`digital[2]` + case2(WallSpatialHash) + case3 已落地；case1 BLOCKED、move_gizmo 完整体 + 9 writer consumer 未闭合）。

## 13. 范围收缩：4 项专项复核（当前快照 2026-10-28；基准已更新，待逐项落地终判）

> 用户指令：暂停其它叶，仅复核这 4 项；实现线按 1→2→3→4 交付并最后挂宿主 a/c 组。以下含**分析线新结论**作为复核基准（终判在各落地+挂载后由我独立跑受影响测试给出）。

### 13.1 复核基准（分析线新结论，2026-10-28）
1. **§7 arrow**：几何 = `SignifierMesh::Named("flat_arrow")`（builder tag=0 = Named 资产；`FUN_14092d570` 查表，缺失 panic），**非** Empty/Points；名字 = `edit_circle_{WidgetId:?}_arrow`（Debug 格式），widget_id 取资源 `+0x00`。（11:42 实现已改为该基准，见 §13.2 → PASS）
2. **§2 move_gizmo**：**8 参签名**；`box_trigger_volume(1.5*base, 0.5, 1.5*base, Transform{ rotation, translation: anchor, scale: ONE })`；widget 名 `move_shape{WallId:?}`；**TriangleMesh 几何**；两条轴记录 + 激活记录 0x28。→ 复核点：8 参签名 / box_trigger 公式 / widget·axis 记录。
3. **§6 S1**：遍历 `PublicWalls` → 每墙注册 `edit_circle_wall_{WallId}` → 比对 `UiState` `+0x18/+0x38/+0x28` → `WallEntities`→`Entity`→`WallTriggerVolume`；**S2** 用 `project_onto_ray` + 屏幕空间 + range `[0, f32::MAX]`。→ 复核点：S1 选择链 + S2 投影。
4. **§3 case1**：此前"6 个 f32 不可判"为**误判**——`+0x00/+0x08/+0x10` = Vec 头 `{cap, ptr, len}`（元素 12B = Vec3）；case1 **可完整实现**，**不再 BLOCKED**。

### 13.2 当前状态（按新基准）
| # | 项 | 当前状态 vs 新基准 | 暂判 |
|---|---|---|---|
| 1 | `ui_edit_circle`（S0–S5） | 30 参/PDB 顺序✔；S0 gate✔；S2 clamp/S5 threshold helper+测试✔；S1/S2 基准已定（`edit_circle_wall_{WallId}` + UiState +0x18/+0x38/+0x28 → WallEntities/TriggerVolume；project_onto_ray range[0,MAX]）→ **可落地**；S1/S3/S4 body + show_full_circle_signifier（5-qword）仍 TODO；Locals 空 | **PARTIAL** |
| 2 | `signifier_edit_circle_arrow` | **已按新基准落地**（11:42）：`SignifierMesh::Named("flat_arrow")`（`:78-79`）+ `vertex_color_shader`；name `edit_circle_{:?}_arrow`，`WidgetId` 取资源 `+0x00`（`unknown_00[..8]`，`:70-73`）；组件元数据非空✔；`marker=0x142aed888`；tag!=2 门控 + Transform 写。**独立跑测 5 passed**（`edit_arrow` 过滤，11:44 锁内）。"flat_arrow" 原证存在（decomp-strings/json-schemas）。细微：live 资产解析（FUN_14092d570 缺失 panic）不在单测覆盖；结构字段仍名 `unknown_00`（+0x00 语义=WidgetId，建议具名，非功能缺陷） | **PASS** |
| 3 | `move_gizmo_and_trigger_vol` | 当前 6 参、仅 camera-rotate+smoothstep，box_trigger/UiState::add/TriangleMesh/axis/tail 缺；新基准要求 8 参 + box_trigger 公式 + `move_shape{WallId:?}` + 双轴/激活记录 | **PARTIAL**（待落地） |
| 4 | `ui_move_shapes` | **case1 已落地（12:02）**：archive `move_shape`(Wall+Terrain)→cull→`wall_decorators_mut` 三元组（源字段 `+0x1c`/`+0x24`）→`ShapeParameters::center()`+地形 anchor→写 shape_rec（Vec 头 `{cap@0, ptr@8, len@0x10}`、Δscreen@0x18、SP 拷贝@0x20、anchor.y@0x3c、decorator_triples），与映射一致（SP 基址@0x258）；单测断言 len/SP-copy/triples/anchor.y。**case3 = `ChangeSupports`（非 StateChange）+ `EditType::Wall`(type7)** ✔（单测断言变体）。case2(WallSpatialHash)✔ + digital[2]✔。**6 tests passed（12:03 锁内）**。剩余 PARTIAL：**9 writer 仅 2/9 实际发射**（`OnShapeMoved`/`ShowPinned`；`material/building/wall_ui audio`/`sheep_check`/`hide_trees`/`out_of_border`/`feedback_oob` 仅签名未 `.write()`）；move_gizmo tail 占位（→新收尾 agent）；注释仍误写 "StateChange"（代码已 ChangeSupports） | **PARTIAL**（case1/2/3/digital 达成；9-writer 与 move_gizmo tail 未闭合） |

- 终判在每项落地 + 挂 a/c 组后给出，附独立跑测；锁被实现线占用期间不抢 Cargo。
