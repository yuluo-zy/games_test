# REBUILD5 — walls 批 3 剩余 24 项复核（restorer-walls5，2026-10-17）

> 协议：`planning/2026-10-16-logic-verify-protocol.md`（分歧检查表 8 条）、
> `planning/2026-10-17-recovery-sop.md`。方法：**只以本轮 fresh 反编译/反汇编为准**。
> fresh 证据落盘 `work/restorer-walls5/decomp/`：
> - `batch1.result.json`（20 函数，fields=signature,decompiled_code,callees）、
>   `batch2.result.json`（5 函数）；
> - `disasm1.result.json`（get_closest / UiWallHeight widget+signifier+closure /
>   spatial_hash_debug / debug_viz_threshold_curve / wall_roof_hashmap）、
>   `disasm_finish.result.json`（finish_edit_rectangle，362 指令）；
> - helper 反编译/反汇编：`aspect_ratio` 0x140c96e60、`utils::lerp` 0x140c904d0、
>   `mouse_ss_to_cs` 0x140c906d0、`get_coord_at_u` 0x141467ce0、
>   `project_onto_ray` 0x140ac84a0、`UiState::add<String>` 0x1409231f0、
>   `walls::impl$0::fmt` 0x140b34ec0；
> - 常量/字符串经 `read_memory` 核实：1e-20 @0x142a7baa0、1e-10 @0x142abaa00、
>   FLT_MAX @0x142925980、1.0 @0x142925904、0.6 @0x142925908、-0.3 @0x142925900、
>   0.7/0.5 @0x142ab3c08、`"Tried to init CurveU from an invalid u: "` @0x142aa5398、
>   `"assertion failed: !u.is_nan()"` @0x142aa53d0、
>   `"assertion failed: !curve.points.is_empty()"` @0x142adf9e0、
>   WallId Display pieces `"WallId("`/`")"` @0x142aed7c8/0x142aed7cf。

## 0. 概览

| 类别 | 数量 |
|---|---|
| 本轮 closed（fresh 重建 + 数值断言） | **1**（get_closest_point_u_to_cursor） |
| 本轮新增 seam（仅接口/host 接线） | 0 |
| 仍登记（REBUILD3 §2 的 25 行，含 1 个闭包） | **24** |

`cargo check -p country-core --lib` **exit 0**；`cargo test -p country-core --lib wall`
**154 passed / 0 failed**（含本轮新增 6 条 cursor 测试）；
全量 lib **2024 passed / 0 failed**（并发改动稳定后复跑）。

## 1. closed：get_closest_point_u_to_cursor（0x140ac7400）

Rust 位置 `src/systems/wall/adjust_wall_height.rs`（原生符号
`ui_adjust_wall_height_mode::get_closest_point_u_to_cursor`）。

fresh 反编译 1494 B + 301 指令反汇编，逐语句判等后重建：

- **入参**：`camera: &MainCamera`、`points: &[Vec3]`（原生 slice ptr@+8/len@+0x10，
  步长 0xc）、`u_values: &[f32]`（ptr@+0x20/len@+0x28，步长 4）、
  `cursor_canvas: Vec2`（原生 pair 序 `(screen_y, screen_x·aspect)`，
  `UNPCKLPS XMM2,XMM1` @0x140ac7567；caller 0x140ac705f 传
  `(mouse_ss_to_cs.y, mouse_ss_to_cs.x·aspect)`）、`height_offset: f32`（caller 取
  PublicWallState +0x1c）。
- **相机矩阵**：fresh 证明原生 `CameraMatrices` 块内 projection@+0x00、view@+0x80
  （`aspect_ratio` 0x140c96e60 = `[+0x14]/[+0x00]`；`eye_position` 0x140c96d40 读
  inverse_view@+0xc0）。本实现经 `MainCamera::read_matrix(0x00/0x80)` 取两块，
  新增常量 `CAMERA_PROJECTION_OFFSET`/`CAMERA_VIEW_OFFSET`。
- **循环**：空 `points` → panic
  `assertion failed: !curve.points.is_empty()`；`points.len()<2` → 返回零结果；
  段数 = `min(u_values.len()-1, points.len()-1)`（0x140ac749c..0x140ac74bb 的
  `min(len_points,len_u)` / `min(len_u-1,len_points-1)` 双界）。
- **每段**：只把 y（第二分量）加 `height_offset`；`view` 再 `projection`；NDC
  `x/w·aspect`、`y/w`；2D 最近点 `t=clamp01(dot/MAXSS(len²,1e-20))`
  （MAXSS/MINSS 有序语义，NaN 透传）；`UCOMISS best,dist; JBE` = 有序 `<`。
- **命中更新**：透视校正比例
  `f = clamp01( lerp(0/w0, 1/w1, t) / MAXSS(lerp(1/w0,1/w1,t), 1e-10) )`
  （`utils::lerp` = `a+(b-a)t`；负分母被 MAXSS 拉到 1e-10——本轮测试即据此校正）；
  `u = lerp(u_i,u_i+1,f)`（同 lerp 公式）；点 `p0·(1-f)+p1·f`。
- **收尾断言**（顺序一致）：u 越界（含 NaN，`UCOMISS; JC`）→
  `Tried to init CurveU from an invalid u: {}`（f32 Display）→
  `assert!(!u.is_nan())`（机器码保留，NaN 已被前一门捕获）。

新增 6 条数值断言：单点零结果、最近段 u/点插值、y 偏移、透视比例(1/3)、
空点 panic、u 越界 panic。

## 2. 跨域解锁核对（本轮）

| 依赖 | 现状 | 结论 |
|---|---|---|
| camera_rig / `CameraMatrices` | `utils::camera::CameraMatrices`（aspect_ratio/eye_*）+ `MainCamera` raw 0x4e0（view@0x80、projection@0x00） | **已解锁**（本轮消费：get_closest） |
| collision pair | `OffsetColliderTransform` + `RaycastMesh` 已在 `systems/collision/world.rs` 落库 | **已解锁** |
| history 类型 | `CompoundEditBuilder`/`ConcurrentEdits`（`recovered_building/wall_history_edit.rs`）、`WindowFocusEvent`、`WriteSettingsFileCmd`、`History::add`、`add_decorator_diff`、`finish_input_recording` 均存在 | **已解锁** |
| UiState / WidgetParams | `UiState::add(&str,WidgetParams)` 与原生 `add<String>` 0x1409231f0 同构（hash(name)+Arc<0x c0>插入）；`WidgetParams` 0xb0 已落库 | **已解锁**（但宿主类型未落库，见 §3） |
| autoclutter | 原生查询类型是 `system_clutter::autoclutter::AutoClutterObject`（`crates/systems/clutter`），该 crate **依赖 country-core** | **未解锁（依赖方向倒置）** |
| PreferredNodeAlignment | 原生类型 `system_decorator::decorator_visual::stairs::PreferredNodeAlignment`（`crates/systems/decorator`），该 crate **依赖 country-core** | **未解锁（依赖方向倒置）** |
| AlignKind | 原生位于 `country_core::resources::stairs::stairs_assembly_params::AlignKind`；crate 只有 20 B 的 `ObservedAlignKind`，与 fresh 记录写的 24 B 值布局不符 | **未解锁（类型未落库）** |

## 3. 逐项状态

| # | 项（VA） | 状态 | 依据 / 还缺什么 |
|---|---|---|---|
| 1 | edit_rectangle 0x14097f620 | 仍登记 | 曲线/enclosure 核已判等；缺 `Vec<(DecoratorId, Vec<PreferredNodeAlignment>)>`（stride 0x20，类型在下游 system_decorator）+ `indexmap::Bucket<(DecoratorId,DecoratorId),AlignKind>`（stride 0x28）。fresh 解出 AlignKind 值面：bucket `{hash@0, key 2×id@8, value@0x10}`，value = `{qword@+0, f32@+8, f32@+0xc, u8 tag@+0x10}`（tag=0/1），共 24 B；PreferredNodeAlignment 元素写 `{q0,q1,q2,u32,u16=4}`。**crate 依赖方向倒置**（原生 country_core→system_decorator），需集成 owner 决定类型下沉/反转载。 |
| 2 | finish_edit_rectangle 0x141464a60 | 仍登记 | 决策核已判等；collision pair、CompoundEditBuilder、WindowFocusEvent、WriteSettingsFileCmd 均已解锁。**剩余**：① history 的 `WallHistoryEdit::MorphPremadeShape` 仍为 `UnresolvedWallEditPayload`；fresh 已解出载荷：`{wall@+8, previous=ShapeParameters@+0x10, next=ShapeParameters@+0x2c}`，两 shape 均 tag=1（rect：axis@+4,center@+0xc,size@+0x14），previous 取事件 `0x20/0x24/0x28/0x2c/0x30/0x34`，next 取 InnerWalls 记录 `+0x44..+0x5c`；② profiler 输出缓冲 push（`gpu_profiler::...ProfilerOutputBuffer`，0x58 B/事件，render 插桩）。 |
| 3 | spatial_hash_debug 0x1408ec0e0 | 仍登记 | 门控 `DebugGui+0xf1==1`；矩形 `DebugRectangle2d::show`、网格线、圆 `get_cells_from_circle<closure_env$2>`、`Circle2d::as_curve3` + `DebugLine::show`、`query_with_circle` 全部落点均为 debug_viz ECS 胶水；`DebugRectangle2d/DebugLine::show` 未落库，closure_env$2 单态体未取证。debug-only。 |
| 4 | debug_viz_threshold_curve 0x140a8b0c0 | 仍登记 | 门控 `DebugGui+0xf3==1`；PublicWalls(0x2a8) 逐记录取 4 个 Vec 长度（+0xa0/+0xd8/+0xf0/+0x108）最小者，逐元素 4×`DebugSphere::show`；`DebugSphere::show` 为 debug_viz ECS 胶水未落库。debug-only。 |
| 5 | wall_roof_hashmap 0x140a94cd0 | 仍登记 | 核心是 `HashMap<u64,&AutoClutterObject>` 每墙单屋顶（panic 文案已恢复）；`WindowFocusEvent`/puffin 已可接。**类型在 `crates/systems/clutter`（依赖 country-core）**，country-core 不能反向依赖。 |
| 6 | get_closest_point_u_to_cursor 0x140ac7400 | **closed** | §1。 |
| 7 | history_record_adjust_wall_height 0x140ac79e0 | 仍登记 | 公式已判等：门控 `mode==1 && flags@0x28&1 && id@0&1`；`new = min(max(base, from), cap)`，base = `roof? (pitch<=0.1?1.6:0.7) : (b29e?0.3:0.5)`，cap = `b29e?2.5:14.0`；`from!=to` → `EditType::new_height_changed` + `History::add` +（记录 count<2）`WriteSettingsFileCmd`。`PublicWallState.authority.byte_0x29e` 与 `PublicWalls::roof_component` seam 已可用。**剩余**：`UiAdjustWallHeightMode` 事件类型（0x30 B：+0 GlobalId/flag、+8 wall、+0x20 mode、+0x24 from、+0x28 flags、+0x2c to；+0x10/+0x18 未读）在 crate 无所有者，生产方（ui_adjust_wall_height_mode）也未落库。 |
| 8 | history_finish_recording 0x1409dd790 | 仍登记（他域在岗） | 符号属 `systems::history::history_recording`；`work/impl-history-remainder` 正在岗。本轮未改 history 文件（红线）。fresh 体：4 路事件游标推进 + `DecoratorArchivist::extract_diff` + `StairsState::extract_diff_and_make_backup` + `History::add_decorator_diff` + `finish_input_recording` + 置 `+0x1b0=1`。 |
| 9 | UiWallHeight::widget 0x1409566d0 | 仍登记 | 体已解：points/heights 逐点 `(x, h+off, z)`，off = `+9==0 ? 0.6 : (stackflag? -0.29 : 0.0)`；名 `format!("edit_height_{}", WallId Display)`；params 为 0xb0 块（字段写 +0x18=3b, +0x30=2b, +0x48=3b, +0x60=3b, +0x78 word 0x202, +0xa8=!(arg+8), +0xa9=1，b=!(R9&1)），尾部 `UiState::add<String>`（已解锁），另有 `+0x258<2 → 0.5 else 0.7` 常量。**剩余**：`UiWallHeight` 宿主类型（+0x90/+0x98 点 Vec、+0xe0/+0xe8 高 Vec）、`UiState80` widget 上下文（+8/+9 字节、+0x258）与 params 其余字段（sounds/volume）来源未落库。 |
| 10 | UiWallHeight::signifier 0x140956cd0 | 仍登记 | 两分支：① `+9==1`：目标记录校验（byte+0x18、qword+0x38==id、dword+0x28!=2）后名 `format!("wall_height_edit_{}", WallId Display)`，40 B 记录 env `{1,1,wall_id, f32(flag?0:0.2)}`；② else：points/heights 逐点 `+0.6` 后名 `format!("wall_height_edit_{}{}", Display, num)`、env `{1,1,len,ptr,len}`；均 `AppendOnlyVec::push<UiAppendSignifier>`。**剩余**：宿主 `UiWallHeight` 布局 + `ui_state::impl$15` Display 未落库。 |
| 11 | impl$0::signifier::closure$1 0x1409576d0 | 仍登记 | 与 #10 同一 signifier 宿主闭包；随 #10。 |
| 12 | ui_adjust_wall_height_signifier_animation 0x14085bb50 | 仍登记 | 3414 B，SignifierEntityBuilder/animation 实体链；`build` 组件面（DrawTopology/UiPass/MeshFollowTerrain）未落库（REBUILD4 已登记 seam）。 |
| 13 | ui_adjust_wall_height 0x140b2fc00 | 仍登记 | 2647 B，UI 输入/widget 链；依赖 #9/#10 与 `UiState` widget 上下文；`grow_one` 仅见 WindowFocusEvent/WriteSettingsFileCmd。 |
| 14 | ui_adjust_wall_height_mode 0x140ac6d60 | 仍登记 | 1680 B，UI 模式/事件链；消费 #7 的事件类型（0x30 B）与其生产方均未落库。 |
| 15 | signifier_edit_circle_line 0x140a64a40 | 仍登记 | 体已解：门控 `shape+0x38 != 2 && !(bit0)`；预览 `Circle2d::new/get_pos_at_u(0.75)-0.75` + 名 `edit_circle_line_` + `ui_state::impl$15` Display；随后遍历墙密集表（0xa0/0x48/0x30 步长、每墙高度 0x1c 步长）算 `0.75-(0.01+0.1·h01)` / `0.75+…`，CurveU 范围断言，`as_curve2_range` + `update_curve_mesh<Map<..,closure_env$1>>`。**剩余**：`ui_state::impl$15` Display、密集表与 Mesh 宿主。现有 `edit_line.rs` 为近似壳（40 B 记录打包），f新证不符，待解锁后重写。 |
| 16 | signifier_edit_dims 0x1408dda10 | 仍登记 | 同族：`crashy_slice` + `update_curve_mesh<closure_env$1>`（`utils::geo` 已落库）+ `ui_state::impl$15` Display + signifier/mesh 宿主。 |
| 17 | ui_create_rectangle 0x14155d410 | 仍登记 | 3134 B；同 #1 的 PreferredNodeAlignment Vec + AlignKind bucket（多处写面）、`GladeBorder::is_shape_inside`、eye_right、`wall_rectangle_canvas` 输入、WriteSettingsFileCmd/iced 事件。 |
| 18 | ui_edit_rectangle_dims 0x140ad6250 | 仍登记 | 2608 B；Rapier raycast + widget/signifier + puffin/command buffer；现有 `ui_edit_rectangle_dims.rs` 为本地输入代理壳，非判等。 |
| 19 | ui_edit_rectangle_rotate 0x140ad8000 | 仍登记 | 2645 B；eye_position（camera 已解锁）+ intersect_ray/is_hit + widget/signifier/command buffer。 |
| 20 | do_ui_edit_rectangle_dims 0x140ad6e00 | 仍登记 | 1649 B；事件跳表 case 2/3/4 + edge normal/point/expand_along_edge；UiState80 多写面未落库。 |
| 21 | do_ui_edit_rectangle_rotate 0x140ad8be0 | 仍登记 | 1565 B；同 #20 + fast acos；事件无 unwrap 门控面未定案。 |
| 22 | UiRectangleDimsEdit::signifier 0x14155eba0 | 仍登记 | 2028 B；`Rectangle2d::get_point`×2 + get_edge_normal + 名 `dims_edit_…` + `UiAppendSignifier` + 3 点曲线 `try_new_from_points<Vec2>`；缺 `UiRectangleDimsEdit` 宿主与 `ui_state::impl$15` Display。 |
| 23 | UiRectangleDimsEdit::widget 0x14155fc70 | 仍登记 | 2527 B；widget 宿主/实体链未落库。 |
| 24 | UiRectangleOrientationEdit::widget 0x14083d3f0 | 仍登记 | 1560 B；widget 宿主/实体链未落库。 |
| 25 | UiRectangleOrientationEdit::signifier 0x14083da10 | 仍登记 | 1105 B；signifier 实体链 + 宿主未落库。 |

（上表 25 行 = REBUILD3 §2 全部登记行；closure$1 与 #10 同宿主。）

## 4. 本轮 fresh 证据对其他域的交付

1. **对齐记录布局（→ stairs/decorator owner）**
   - `indexmap::Bucket<(DecoratorId,DecoratorId), AlignKind>` stride 0x28：
     `{hash u64@0, key 2×id@8, value@0x10}`；写值 = `{qword@+0, f32@+8, f32@+0xc,
     u8 tag@+0x10}`（tag 0/1），即 **AlignKind 值 24 B**，不是现有
     `ObservedAlignKind` 的 20 B。
   - `Vec<(DecoratorId, Vec<PreferredNodeAlignment>)>` 元素 stride 0x20；写
     `{q0,q1,q2,u32@0x18,u16@0x1c=4}`。`PreferredNodeAlignment` 在原生
     `system_decorator`（country_core 的依赖），本 workspace 依赖方向相反。
2. **finish_edit_rectangle → history**：`MorphPremadeShape` 载荷已解（见 #2）。
3. **CameraMatrices 原生块（→ camera/render）**：projection@0x00、view@0x80、
   inverse_view@0xc0；`aspect_ratio` = `projection.y_axis.y / projection.x_axis.x`；
   `utils::camera::CameraMatrices` 目前只建模 projection+inverse_view，缺 view 槽。
4. **WallId Display（→ walls UI）**：`format!("WallId({})", self.0)`（0x140b34ec0）。
5. **UiState::add**：原生 `add<String>(state, name: String, params: &WidgetParams)`
   等价现有 `UiState::add(&self, &str, WidgetParams)`（hash(name) → Arc<WidgetParams> 插表）。

## 5. 改动文件（全部 walls 域）

- `src/systems/wall/adjust_wall_height.rs`：新增 `get_closest_point_u_to_cursor`
  + `WallCursorHit` + `CAMERA_PROJECTION_OFFSET`/`CAMERA_VIEW_OFFSET` + 6 条测试。
- 未改数据库；只读 `--worker shared`；未触碰他域业务。

## 6. 测试

- `cargo check -p country-core --lib`：exit 0。
- `cargo test -p country-core --lib wall`：**154 passed / 0 failed**
  （本轮新增 6 条位于 `cursor_tests`）。
- 全量 lib：**2024 passed / 0 failed**（并发改动稳定后复跑）。
