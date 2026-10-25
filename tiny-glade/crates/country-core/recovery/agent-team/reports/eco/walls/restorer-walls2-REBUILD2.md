# REBUILD2 — walls 批 2 的 8 个「未实现-需重做」（restorer-walls2，2026-10-17）

> 协议：`planning/2026-10-16-logic-verify-protocol.md`（含分歧模式检查表 8 条）。
> 只以本轮 **fresh 反编译/反汇编** 为准：`reverse-operator.py ghidra --worker shared`，
> 8 函数一次批（fields=decompiled_code,callees）+ 4 函数 disasm + 常量内存读取。
> 证据落盘：`work/restorer-walls2/decomp/batch1-result.json`、`disasm2-result.json`、
> `batch2-result.json`（caller 0x1408dec70）、`batch3-result.json`（Easing apply + 常量）、
> `mem-result.json`。

## 结论概览

| # | VA | 函数 | verdict | 本轮闭合的分歧 |
|---|---|---|---|---|
| 1 | 0x14097e560 | create_circle | **判等-注意** | height_01 公式（u 不 clamp + SineInOut tag2 + 末 clamp）与 roof 载荷 |
| 2 | 0x14165bed0 | ui_create_circle | **判等-注意** | 输入动作码 3/4/<2 + 状态跳表 code2/3/4 + 累加/置位/钳制重算 |
| 3 | 0x140a6ad80 | snapping_to_ends | **判等** | snap 点 = 端点 u∈{0,1} 3D；补 cursor flags bit0 门 |
| 4 | 0x14086ad70 | telegraph_walls_are_merged | **判等-注意** | easing 修正为 tag0 CircularInOut；计时不重置 |
| 5 | 0x14097f970 | shape_placement_params | **判等-注意** | 查询半宽 ±2.25（已改）；raycast 跳表跨域登记 |
| 6 | 0x1408dfc60 | preview_snapping_line | **判等-注意** | 返回扩展曲线投影点（切片中点），y=0；freehand 分支 |
| 7 | 0x1409de190 | wall_eraser_bulldoze | **判等-注意** | 归属纠正为 ray-pick UI 生产者；缓冲桥拆为独立 handler |
| 8 | 0x141660880 | show_full_circle_signifier | **判等-注意/跨域** | 建立库层对应函数（format + AppendOnlyVec push 记录） |

---

## 1. create_circle 0x14097e560 — 判等-注意

fresh（615 行 decomp + disasm）逐段：

- `Circle2d::as_points2`(0x140c97bc0，XMM2=max_step) → `from_iter`(0x141304650) →
  `try_new_from_points`(0x141469430) → `new_wall_w_params`(0x14085a370)。
- **`_try_update_state<update_elevation>`**(0x140aaaac0，closure 值 = 输入事件 `+0x24`) →
  **`update_wall_state_enclosure`**(0x140aa9610) → `get_building_and_roof_colors`(0x140a94880) →
  **`update_style`**(0x140aaa350)。
- 高度：`14097ebf7 ADDSS XMM7,[0x142aa508c]`（−0.9）→ `DIVSS XMM7,XMM10`（7.1，
  内存 0x142aa5090=0x40e33333）→ **无 clamp** → `EasingFunction::apply`（RCX=0x142aa5938，
  内存 tag=2 = SineInOut）→ `(10e+4(1−e))/12` → `MAXSS 0 / MINSS 1`。
  `0x142925870=0.5`、`0x142925904=1.0` 已内存核实。
- CreateRoofCmd 0x40B：profile 0.5、height_01、ridge/eave 0.2；`ev_new_wall` + `History::add` ×2。

本轮改动 `create_circle.rs`：**删除 u 的 clamp01**（native 仅在最终 height clamp），
测试改为 `height(0.9)=1/3`、`height(0.0)≈0.35290`（u<0 时 SineInOut 对称回升，
此前误写 1/3）、`height(8.0)=5/6`。

登记（未编造）：
1. `update_elevation` 的 closure 值 = 输入事件实例 `+0x24`（0x30 stride 事件布局未定案；
   Rust `CreateCircleCmd` 只有 center/radius）；Rust 以 `new_wall_w_params` 的 elevation
   参数承载，独立 update 调用未接线。
2. `get_building_and_roof_colors` → `update_style`：颜色库跨域；style 传 0（与
   create_rectangle 一致）。
3. 原生另写 2 个事件 + 条件 tag-4 屋顶历史（颜色链非零时）为宿主接线。
4. `as_points2/from_iter/try_new_from_points` ↔ `as_curve2(0.2)` 表示差异（点集等价）。

## 2. ui_create_circle 0x14165bed0 — 判等-注意

fresh（1391 行 decomp + 619 指令 disasm）：

- 门：`*mode == 6 && mode.sub == 1` 否则清 preview 4 字节并 return。
- `UiState::add("wall_circle_canvas"/"cursor_circle_")`；`60.0` 阈值分支写 AlignKind 记录。
- **输入动作码 @state+0x19**：
  - `3`：写预览圆心/半径（canvas +0x10/+0x14/+0x18/+0x20）；
  - `4`：`radius = 旧半径 + SQRT(dx²+dz²)`（输入资源 `+0x140` 两 f32），
    `4.0 < radius` → 置位钳制字节 `+0xc`（**不截断半径**）；
  - `<2`：按下/释放事件记录（宿主）。
- **状态跳表** `bVar25=(code−2)∈{0,1,2}`：code2/3 写圆参数记录（tag 0/2 + area/inverse_lerp），
  **code4 且钳制位置位** → `radius = clamp(圆心到游标投影距离, 0.9(0x3f666667), 8.0(0x41000000))`
  并记录 `|Δradius|`。
- 后续每帧 `circle_signifier` + `AppendOnlyVec::push`；case7 走 `as_curve2` +
  `update_wall_state_curve/enclosure` 编辑链。

本轮改动 `ui_create_circle.rs`：`CircleCanvasInput` 增加 `action_code`，按上述两段
状态机实现；code4 累加**不再** `min(4.0)`，改为置位 + 同帧 clamp 距离重算
（`cursor.delta.xz` 代理 `+0x140`）。测试 2 条（累加不截断；>4 切换 clamp 距离/下限 0.9）。

登记：GladeBorder::is_shape_inside；circle 编辑链 case7（需编辑 wall id）；
PreferredNodeAlignment/AlignKind 记录；DebugLine/puffin；60.0 阈值分支；
相机矩阵投影（in_R9）以世界 XZ 距离代理。`CursorTerrainRaycast.tag` 门为库层输入代理
（native 无同名门，登记）。

## 3. snapping_to_ends 0x140a6ad80 — 判等

fresh 733 行 decomp + 471 指令 disasm（前轮已定案，本轮复核）：

- `mode != 1 → return` ✓；`in_progress.wall` unwrap_failed（Rust return 登记）。
- **cursor flags bit0 门**（`140a6ae01 TEST byte[RAX],0x1; JZ → {0,0}`）本轮补入；
  tag&1 时参考点 = (pos.x,pos.z)。
- snapped==0：仅 `tag==2 && wall!=current` 参与；snapped==1：全部 tag==2（含 current）。
- 当前墙 → 首 raw 点 + flags；其余 → 最近端点（`SQRT(d_start)<SQRT(d_end)`，
  起点选中 end_flag=1/u=0，否则 end_flag=0/u=1）。
- 最小距离 `<1.5` 才写；**snap 点 = `get_coord_at_u(curve,u∈{0,1})` → outline 3D 按 t 插值**
  （`140a6b302 MOVAPS XMM1,XMM6`）；写 mode=2 / snapped 字节 / Vec3 / wall / end。

Rust 与之逐条对应（`state.assert_valid()` 覆盖 native 的 len<2/length≤0 panic）。
登记：`in_progress.wall None → return` vs unwrap_failed panic（系统输入边）；
mode.snapped 写 bool vs 原生原始字节（0/1 等价）。

## 4. telegraph_walls_are_merged 0x14086ad70 — 判等-注意

fresh 836 行 decomp + 922 指令 disasm：

- 每 public 段：`seg+0x40 += dt` → 写回 → `t = /0.75` → clamp01。
- `EasingFunction::apply` 的枚举指针 `0x142a96d84`，内存 **tag=0 = CircularInOut**
  （Rust enum 序 CircularInOut=0；此前误用 SineInOut，本轮修正）。
- `u_hi = 2e`、`u_lo = 2e−1`（先 `maxps 0/minps 1` clamp）→
  `|u_hi−u_lo| < 2·(0.025/curve_len)` 时 `u_hi+=eps, u_lo−=eps` 再 clamp
  （eps 用 `seg+0x30` 曲线长）。
- `t==1.0` → 旧实体 `EntityCommands::despawn`，累加器**保持饱和不重置**
  （本轮修正：Rust 不再 remove timer，避免预览重播）；随后
  `slice_into<Vec3,3D>` → `try_new_from_points` 等实体链。

登记：`despawn/update_curve_mesh/SignifierEntityBuilder::build/insert<LineAcrossWallSignifier,
DashedLineLength,DrawTopology>` 为 render-host 实体链；计时键为 WallId（原生 per-segment
`+0x40`）；曲线切片用简化 2D + outline 插值表示；signifier 载荷简化。

## 5. shape_placement_params 0x14097f970 — 判等-注意

fresh 1005 行 decomp：

- 门：`AppMode tag6/sub{1,2}`；cursor flags bit0（tag&1 才取 pos）。
- **`query_with_aabb_clamped` 的 AABB = center ±2.25**（`14097fc1` 段 `+−2.25`）；
  Rust 已从 ±8.0 改为 2.25 ✓。
- `from_iter<SnapProposal,filter_map<closest_segment_to_snap_to::closure_env_0>>` →
  `sort_by`（<0x15 smallsort insert_tail / 否则 driftsort）→ 取首个（最近）。
  Rust 的 `query_with_aabb_clamped` 已排序 + 同墙去重，取最近等价。
- 无候选 → mode=0 unsnapped；有候选 → 墙形状判别（circle normal / rect fields /
  freehand tangent）+ `get_approx_u_from_pos`×2 + `1.5` 门 + roof_entity 查询。
- **Rapier::raycast(0x1408cdc60) 命中类型跳表**：case 0/9 墙（PublicWalls::get +
  normal + 1.5 + roof ridge）、case 4/5/… 其它目标；miss 才落回 query 路径。

登记（跨域）：Rapier raycast + 命中类型跳表（collision 域）、`snap_to_roof_ridge`
（roof 域 `Roof::pivot_3d`/`RoofRidgeDims::calc`）、DebugSphere/DebugLine、屋顶实体
Query、mode{0,1,2} 多载荷记录为宿主接线。本域 query 路径与 native 一致。

## 6. preview_snapping_line 0x1408dfc60 — 判等-注意

fresh 240 行 decomp + 210 指令 disasm + caller 0x1408dec70 复核：

- 参数：`(out, state, position: Vec3, marker(R9), line_len=4.5)`（caller 栈参
  `0x40900000`=4.5）。
- tag≤1：circle `radius+0.315` / rect dims `+0.63` → `as_curve3(max_step=0.5,height=0)`
  （rect 另 `crashy_resample_per_segment`）。
- `u = get_approx_u_from_pos(curve3, position)`；`half = clamp(4.5/len,0,1)·0.5`
  （`MULSS [0x142925870]`，内存核实 **0.5**）；tag==2 开曲线 clamp[0,1] + slice_into，
  tag>2 `fmod` + circular_slice；随后 `get_coord_at_u(slice, 0.5)`（同一 0.5 常量）
  **取切片中点 = 投影点**，3D 点按 segment/t 插值（premade y=0）。
- tag>1：同流程走 state 的 3D 曲线（+0x38）。

本轮重写 `shape_placement_signifiers.rs`：`preview_snapping_line(state, position)`，
premade 返回扩展形状最近点（y=0，此前误用 `calculated_max_y`），freehand 返回
prepared path / 曲线在切片中点处的 3D 点；调用方 `snapping_maintain` /
`shape_placement_signifiers` 改为传参考点。新增投影 helper 单测。

登记：`as_curve3` 弧长参数 vs 形状自身参数化；返回载荷 `+0xc/+0x14 + 2×qword`；
freehand anyhow/CurveError 分支（宿主）。

## 7. wall_eraser_bulldoze 0x1409de190 — 判等-注意

fresh 430 行 decomp：

- `UiState::add` + `Rapier::raycast_simple`(0x1408ce180) + 命中类型跳表
  （0x11 demo → "cursor_hammer"；case 0/1/2/6/7/8/9 墙；case 3/4/5/10–14 其它动作）。
- 逐墙段（0x98 stride）：命中点距离、`delete_tree` signifier + `UiState::add<String>`、
  距离门（`threshold ≤ dist && dist != threshold`）→ 设置动作 kind；
  末 `local_100` 间接跳表分派目标 handler。
- **归属结论：本 VA 是 UI 生产者，不是命令缓冲桥。** Rust 已拆为
  `wall_eraser_bulldoze`（ray-pick → signifier + AnimateWallBulldozeCmd）与
  `enqueue_bulldoze_command`（事件→buffer，native 跳表目标 handler 的独立 VA）。

登记：命中类型间接跳表目标 VA 未在本函数体内（host 动作表）；逐段距离门与
low/high 擦除范围载荷；`Rapier::raycast_simple` 为 collision 域（`BulldozerRayPick` 代理）；
DebugViz。当前默认全墙 `low=0/high=1` 为登记近似。

## 8. show_full_circle_signifier 0x141660880 — 判等-注意/跨域

fresh 73 行 decomp：

- `format_inner`：`resources::walls::impl$7::fmt` + `geometry::circle::impl$3::fmt`
  + 两个 `core::fmt::float::impl$5::fmt` → 分配字符串 → `memcpy` →
  `AppendOnlyVec::push<UiAppendSignifier>`（记录 `qword[0]=1,qword[1]=1`,
  字符串 ptr/len, `+0x20/+0x24` 两 f32）。

Rust 现于 `ui_edit_circle.rs` 提供 `show_full_circle_signifier(stream, center, radius)`
写 `UiAppendSignifier`（tag1/kind1/position/scale）。登记：格式化字符串句柄
（`name` 语义 U）、记录两 f32 槽位、`AppendOnlyVec` 为 render-host 存储；
调用点在 ui_edit_circle/ui_create_circle 的输入动作状态机内（宿主接线）。

---

## 自检

- `cargo check -p country-core --lib` → **exit 0**（最终运行 19:11；期间曾被
  `systems/ui/signifiers`、`systems/time_of_day` 他组 WIP 短暂阻断，等其落盘后复跑通过，
  未代改他域代码）。
- `cargo test -p country-core --lib` → **1879 passed / 0 failed**。
- `cargo test -p country-core --lib wall` → **99 passed / 0 failed**
  （含 create_circle 2、ui_create_circle 2、preview_snapping_line 2、
  telegraph 1、ui_bulldozer 2、snapping 8、valid_enclosure 6）。
- 改动文件（均在 walls 域）：
  `systems/wall/shapes/circle/create_circle.rs`、
  `systems/wall/shapes/circle/ui_create_circle.rs`、
  `systems/wall/snapping/snapping_to_ends.rs`、
  `systems/wall/snapping/telegraph_walls_are_merged.rs`、
  `systems/wall/shapes/shape_placement_signifiers.rs`、
  `systems/wall/snapping/snapping_maintain.rs`（仅调用点）。
- 只读 `--worker shared`，未改数据库；未触碰他域业务。
- 未采信前轮 restorer/verifier 描述，全部以本轮 fresh decomp/disasm/内存为准。
