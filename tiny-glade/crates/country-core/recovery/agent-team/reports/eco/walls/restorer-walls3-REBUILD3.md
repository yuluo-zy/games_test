# REBUILD3 — walls 批 3 的 49 项「未实现-需重做」（restorer-walls3，2026-10-17）

> 协议：`planning/2026-10-16-logic-verify-protocol.md`（含分歧检查表 8 条）、
> `planning/2026-10-17-recovery-sop.md`。方法：**只以本轮 fresh 反编译/反汇编为准**。
> fresh 证据落盘 `work/restorer-walls3/decomp/`：
> `batch1.result.json`（20 函数，fields=signature,decompiled_code,callees）、
> `batch1b-disasm.result.json`（snap_to_roof_ridge / edit_arrow / segment&rectangle cells /
> load_state / 3 个 4–7B 谓词 / terrain_height）、
> `batch1c-disasm.result.json`（extract_before_after_impl / add_segment）、
> `consts/mem2..mem5.result.json`（14 个常量 + panic 文案）、
> `batch2.result.json`（20 函数，剩余 UI/历史面）。未采信 LOGIC3 的结论本身，
> 逐函数重新取证；发现 2 处 LOGIC3 与 fresh 机器码不符（见 §4）。

## 0. 概览

| 类别 | 数量 | 说明 |
|---|---|---|
| 本批 closed（代码重建/补齐，逐语句或结构判等） | 25 | §1 |
| 登记-跨域/缺类型（不改或只改门控，接口语义写明） | 24 | §2 |
| 其中按 fresh 推翻 LOGIC3 结论 | 2 组 | §4 |

`cargo check -p country-core --lib` 通过；`cargo test -p country-core --lib wall`
**122 passed / 0 failed**；全量 lib **1934 passed / 1 failed**（失败项
`resources::history::decorator_edit::tests::convert_nan_is_a_change_and_uses_after_value`
属 history 域、当轮他组在写，不代改；20:0x 时不复现/复现反复，归属 history）。

## 1. closed（本轮重建/补齐）

| VA | 函数 | Rust 位置 | verdict / 本轮闭合点 |
|---|---|---|---|
| 0x14085ad80 | InnerWalls::load_state | inner_walls.rs | **判等-注意**：3 前置 panic（walls/updated/removed items；**不查 created**）；空记录仅 return（max_id 不变）；逐记录 id 插 created；`HashMap::with_capacity(len)` 重建 walls；`max_id = 记录 id 最大值`。登记：记录内 Curve2 点列是堆指针，0x70B 字节不可重建点列（`WallState::from_record_bytes` 保留 length，点列空） |
| 0x14085a200 | InnerWalls::new_wall_from_curve | inner_walls.rs | **判等**：max_id+1 → insert（旧值 drop）→ created insert；构造由调用方完成（与 new_wall_w_params 同构） |
| 0x1408c5660 | FreehandHeightType::toggle | inner_wall_state.rs | **判等**：`*self ^= 1`（新增类型） |
| 0x1408c5670 | WallStyle::is_halftimbered | inner_wall_state.rs | **判等**：`CMP byte,2; SETNC` = `>= 2`（新增类型） |
| 0x140833230 | StructureType::is_wall / FreehandHeightType::is_follow_terrain | inner_wall_state.rs | **判等**：`CMP byte,0; SETZ` = `== 0`（同址单态合并） |
| 0x14085ebc0 / 0x140860d50 | deserialize_wall_style / visit_str | inner_wall_state.rs | **判等**：`"Stone"→0`/`"StoneWithSkirts"→1`/`"HalfTimbered"→2`/`"WattleAndDaub"→2`，其余 `unknown_variant`；`serde::Deserialize` 实现 |
| 0x140b2a680 | PublicWallState::max_y_gable | public_wall_state.rs | **判等**：flag@+0x110 未置位 → **panic**（消息 `max_wall_y_gable` 未计算，0x142aeaec8，161B，逐字恢复）；删除 `unwrap_or_else(max_y)` 回退 |
| 0x1409ef210 | get_cells_from_rectangle2d | wall_spatial_hash.rs | **判等**：查询矩形 `aabb` + 区间 `min=max(min,0)`/`max=min(max,dims-1)`（不反向钳制）+ **逐格格矩形 aabb × Aabb2::intersects**；原先只铺格，已补筛 |
| 0x1409ef870 | get_cells_from_segment\<add_segment\> | wall_spatial_index.rs `segment_cells` | **判等（LOGIC3 更正）**：两 `is_inside_bbx` 失败 → **直接 return（无格）**；`length==0` → 单点 grid 格（有界检查）。原实现无需改，见 §4.1 |
| 0x1409eff80 | get_cells_from_segment\<query_with_segment\> | 同上 | **判等（LOGIC3 更正）**：同几何，仅 out 收集闭包不同 |
| 0x140aa50e0 | WallSegment::terrain_height_closest_to_ws_pos | wall_spatial_hash.rs | **判等-注意**：`t=clamp01(dot(p−a,b−a)/max(|ab|²,1e-20))`，`(1−t)h0+t h1`；SSE 有序语义。登记：原生读 +0x18 pair，本结构按已证布局映射 `relative_heights`（PDB 名 terrain 与 +0x18 语义待 add_segment 形参顺序定案） |
| 0x140b35120 | signifier_edit_circle_arrow | edit_arrow.rs | **判等-注意**：门控修正 `tag != 2`（圆+矩形都发；0x142aed850 两片段 `edit_circle_`+`_arrow`；rotation_from_normal@subject+0x2c）。ECS Query 形态/逐段列写入登记 |
| 0x140980db0 | snap_to_roof_ridge | shape_placement.rs | **判等**：完整重建（见 §1.1） |
| 0x140b2bb00 | DecoratorArchivist::all_decorators | decorator_archivist.rs | **判等**：walls + special1 + special2 三元链 |
| 0x140b2bb80 | all_storages | decorator_archivist.rs | **判等**：`(addr,&桶)` 三元链（niche 编码以显式枚举表达） |
| 0x140b2bc00 | decorator_cull_allow_wall | decorator_archivist.rs | **判等**：walls 探针命中 → 桶内 id 任一 allowed；miss false |
| 0x140b2bd30 | decorator_cull_allow_stairs | decorator_archivist.rs | **判等**：special2 桶 fold |
| 0x140b2be70 | extract_before_after_impl | decorator_archivist.rs | **判等-注意**：补 **preview panic**——`CMP byte [entry+0x4f],0; JNZ panic`（entry={id@0,info@8..0x50} ⇒ **info.state(+0x47) != 0**）；消息 0x142aeb4f8 `"Found a preview decorator while trying to extract history diff. "` + id/info 两参数。另一 `'after' state exists` panic 的触发 flag 未定案，登记 |
| 0x140b2c4b0 | extract_and_restore_to_storage | decorator_archivist.rs + decorator_storage.rs | **判等**：清 collision_a/b → take special1/2 → 三路 chain → `get_or_insert_default` + `tracked_clone_from<DecoMoveToStorage>`；新增 `DecoratorStorage::tracked_clone_into`（字段拆分借用，d2s 重指向） |
| 0x140b2cb30 | extract_diff | decorator_archivist.rs | **判等-注意**：walls 之外 **special1/special2 也产 Terrain/Stair diff**；配对闭包仍为既有登记 U 语义 |
| 0x14123b5c0 | archive_current_decorators_in_storage（大单态） | decorator_archivist.rs | **判等-注意**：逐墙/terrain/stairs 归档与 `DecoratorStorage::get` 面已由既有实现覆盖；collision `get_collider/get_collider_mesh + insert + drop_slow` 属 collision 域，登记 |
| 0x140ac1f20 | adjust_wall_height_impl | adjust_wall_height.rs | **判等-注意**：mode0 跳过、`!=1 panic`、walls miss **panic**（原发事件）；PublicWalls 探针 + `clamp_wall_height` 断言（count>1/length>0）。仍登记：PreferredNodeAlignment Vec + `(DecoratorId,DecoratorId)→AlignKind` indexmap 两条记录（缺跨域类型） |
| 0x140ac7f30 | clamp_wall_height | adjust_wall_height.rs | **判等-注意**：PublicWalls 探针 miss/invalid → panic（`validate_public_wall_read` 同式 count>1 且 length>0）→ roof component |
| 0x1408dec70 | shape_placement_signifiers | shape_placement_signifiers.rs | **判等-注意**：补 `SelectedWallShape tag==6 && ty∈{1,2}` + 公共墙 `tag != 2` 门控；`mode==2`/else 两分支的 `"snap_signifier_"` / `"snap_signifier_curve_"` 名字句柄与载荷为 render-host 登记 |

### 1.1 snap_to_roof_ridge 0x140980db0 重建要点（fresh decomp + 316 指令 disasm）

- 圆（`roof[8]&1==0`）：平顶 `h01<=0.1` → Point`(center.x, position.y, center.z)`；
  否则 `u=((handle+(-0.90000004))/7.1)/0.4` → t=clamp01 → `smooth=(3-2t)t²` →
  `limit=smooth·12+(1-smooth)·8`（`assert!(0<=limit)`，原生 clamp 运行时 panic）→
  `h=min(limit,max(0,(1-h01)·0.4+h01·12))` → Point`(0·h+center.x, h+floor, center.z+0)`。
  距离 < threshold 才返回。
- 矩形：`ridge_dims_calc` 同构尺寸对 `(f3,f2)`（dir 决定轴）→ `f4=f3<=f2?f2:0.01`、
  `f3'=f2<f3?f3:0.01`（disasm `CMPLTSS/ANDPS/ANDNPS` 选择）；矩形
  `{axis=roof+0xc, center=roof+0x14, size=(f3',f4)}`；角点 0/2 经 `xvy` 取两点，
  y=平顶?position.y : floor+ridge_height（高度缓动低值 9.0、比值 `(max(dims)-2.25)/12.75/0.4`）。
  线段最近点 `a(1-t)+bt`；命中 → Segment，t 缺省 0.5，若 `max(dims)>0.5` 按
  `0.3/len` 边界：`boundary>t→0`，`t>1-boundary→1`，否则 t（UCOMISS/JA/JBE 有序语义）。
- 常量全部 `read_memory` 核实：1e-20 / 0.3 / 0.4 / 12.0 / 8.0 / 9.0 / -2.25 / 12.75 /
  -0.90000004 / 7.1 / 0.1 / 0.01 / 3.0。
- 返回类型改为 `RoofRidgeSnap::{None,Point,Segment{t}}`（原生 sret 判别 0/1/2 载荷）。
- 测试 3 条（平顶圆、斜圆、矩形段）对齐原生几何。

## 2. 登记-跨域/缺类型（不改或只改门控）

| VA | 函数 | 阻塞 / 解锁条件 |
|---|---|---|
| 0x14097f620 | edit_rectangle | 核心 `as_curve2 + update_wall_state_curve/enclosure` 已判等；缺 `PreferredNodeAlignment` Vec（stride 0x20，system_decorator 域）与 `indexmap::Bucket<(DecoratorId,DecoratorId),AlignKind>`（0x28，stairs 域）两条记录写。解锁：system_decorator 提供 PreferredNodeAlignment + stairs 提供 AlignKind 实体类型 |
| 0x141464a60 | finish_edit_rectangle | 决策核（0.01 双阈值/acosf/EnclosureChange history）已判等；缺 `DecoratorArchivist<C>` 具体 C（collision pair 未落库）与 WindowFocusEvent/WriteSettingsFileCmd/profiler 事件。解锁：collision 提供 `(OffsetColliderTransform,RaycastMesh)` 资源类型 |
| 0x1408ec0e0 | spatial_hash_debug | DebugLine/DebugSphere + 每格数据绘制；render-host。解锁：debug 绘制事件类型 |
| 0x140a8b0c0 | debug_viz_threshold_curve | debug 字节@+0xf3 门控 + 曲线绘制；render-host |
| 0x140a94cd0 | wall_roof_hashmap | `system_clutter::autoclutter::AutoClutterObject` 查询 + 每墙单屋顶 map（panic 文案已恢复：`"Only one roof should be associated..."`/`"Tried deleting roof during Replay"`）+ WindowsFocus 事件 + puffin。解锁：autoclutter 类型 |
| 0x140ac7400 | get_closest_point_u_to_cursor | `CameraMatrices::aspect_ratio` 投影 + 屏幕空间最近段；缺相机矩阵资源 |
| 0x140ac79e0 | history_record_adjust_wall_height | 公式已恢复：门控 `mode==1 && flag28&1 && flag0&1`；`new=min(max(roof? (flat?1.6:0.7):(b29e?0.3:0.5), to), b29e?2.5:14.0)`；`from!=new` → `EditType::new_height_changed` → History::add + WriteSettingsFileCmd。缺：`UiAdjustWallHeightMode` 记录类型 + PublicWallState +0x29e 快照字节（crate 无该字段） |
| 0x1409dd790 | history_finish_recording | `History::add_decorator_diff + finish_input_recording`，属 history 域 |
| 0x1409566d0 | UiWallHeight::widget | UiState80 widget/ECS 实体链（1445B） |
| 0x140956cd0 | UiWallHeight::signifier | signifier 实体链 |
| 0x1409576d0 | impl$0::signifier::closure$1 | 同一 signifier 闭包 |
| 0x14085bb50 | ui_adjust_wall_height_signifier_animation | 3414B，动画/signifier 实体 |
| 0x140b2fc00 | ui_adjust_wall_height | 2647B，UI 输入/widget 链 |
| 0x140ac6d60 | ui_adjust_wall_height_mode | 1680B，UI 模式/事件链 |
| 0x140a64a40 | signifier_edit_circle_line | 核心圆几何已建模；门控 `tag!=2 && tag&1==0` 的 subject 组件（0x140b35120 同族，0x38 tag）与 ECS Query 未落库，`PreviewCircle` 仅为代理 |
| 0x1408dda10 | signifier_edit_dims | `crashy_slice + update_curve_mesh` + signifier 实体（mesh/host） |
| 0x14155d410 | ui_create_rectangle | `rectangle_canvas` UiState + is_shape_inside + PreferredNodeAlignment/AlignKind + eye_right |
| 0x140ad6250 | ui_edit_rectangle_dims | Rapier raycast + widget/signifier + puffin/command buffer |
| 0x140ad8000 | ui_edit_rectangle_rotate | eye_position + intersect_ray/is_hit + widget/signifier |
| 0x140ad6e00 | do_ui_edit_rectangle_dims | 事件跳表（case 2/3/4）+ edge normal/point/expand_along_edge；核心 `expand_along_edge` 已在 Rust，但事件载荷/UiState80 多写面未落库 |
| 0x140ad8be0 | do_ui_edit_rectangle_rotate | 同上 + fast acos 已建模；事件无 unwrap 门控面未定案 |
| 0x14155eba0 | UiRectangleDimsEdit::signifier | ECS 实体链 |
| 0x14155fc70 | UiRectangleDimsEdit::widget | ECS 控件（2527B） |
| 0x14083d3f0 | UiRectangleOrientationEdit::widget | ECS 控件 |
| 0x14083da10 | UiRectangleOrientationEdit::signifier | ECS 实体链 |

## 3. 测试

新增 19 条数值断言（walls 域）：
- `load_state`：map/created/max_id、空记录保持 max_id、3 个前置 panic、`new_wall_from_curve` 序列（7）。
- `WallStyle` 四串映射/未知变体、`is_halftimbered`、`FreehandHeightType::toggle`、
  `StructureType::is_wall`、0x70B 记录偏移解码（4）。
- `terrain_height_closest_to_ws_pos` 插值/夹取/零长（1）；`rectangle_cells` 世界外空/轴对齐 31..32 四格（2）。
- `all_decorators/all_storages` 三元链、cull wall/stairs、`extract_and_restore_to_storage`、
  preview panic、`extract_diff` Terrain/Stair（4）。
- `snap_to_roof_ridge` 平顶圆/斜圆/矩形段（3）。

`cargo check -p country-core --lib` exit 0；`cargo test -p country-core --lib wall` **122/122**；
全量 1934 passed / 1 failed（history 域 `convert_nan_is_a_change_and_uses_after_value`，非本批文件）。

## 4. 对 LOGIC3 的 fresh 更正（机器码证据）

1. **§3.4「端点越界 → 推中点格」不成立**。0x1409ef870 disasm：
   `CALL is_inside_bbx ×2`（1409ef90a/1409ef944）；第二次 `TEST AL,AL; JZ 0x1409efd08`
   （= 函数 epilogue，直接 return）。中点/单点格代码（1409ef996–1409efa7f）只在
   `UCOMISS length,0; JNZ/JP main` 的 **length==0** 落空路径执行，即**零长段单格**，
   不是越界分支。`segment_cells` 现有「越界空 + 零长单格 + 有界检查」与之等价，未改逻辑。
   同理 `get_cells_from_rectangle2d` 的逐格 intersects 是**对查询矩形 AABB**（两者轴对齐），
   数学上等于区间铺格；本轮仍按原操作序列补上逐格筛与「不反向钳制」语义。
2. **preview 断言字节是 `DecoratorInfo+0x47`（state），不是 +0xf**。0x140b2be70 disasm：
   入口基址 `R11 = RCX + R9 - 0x50`，检查 `CMP byte [RCX + R9 - 1],0`（= entry+0x4f），
   entry = {DecoratorId@0, DecoratorInfo@8..0x50} ⇒ info+0x47；`JNZ 0x140b2c2ff` 进入
   0x142aeb540 文案的 `panic_fmt`。实现按 `info.0[0x47] != 0` panic。
3. **add_segment WallSegment 偏移**（0x140aa5dd0 disasm）：`{p0@p1 @0, pair@0x10,
   pair@0x18, pair@0x20, WallId@0x28, u32 key@0x30}`（stride 0x38）。本结构把
   +0x18 记为 `relative_heights`，与 ivy 已证锚一致；PDB 名 terrain 的语义待形参顺序定案（已登记）。
4. `max_y_gable` panic 文案 161B 已逐字恢复（`read_memory 0x142aeaec8`）。

## 5. 改动文件（全部 walls 域）

`src/resources/walls/{inner_wall_state,inner_walls,public_wall_state,decorator_archivist,decorator_storage}.rs`、
`src/systems/wall/wall_state/wall_spatial_hash.rs`、
`src/systems/wall/shapes/{shape_placement,shape_placement_signifiers}.rs`、
`src/systems/wall/shapes/circle/signifiers/edit_arrow.rs`、
`src/systems/wall/adjust_wall_height.rs`。
只读 `--worker shared`，未改数据库；未触碰他域业务。

## 6. 剩余

本批 49 项：closed 25、登记 24。剩余按解锁条件由对应域提供类型后即可继续
（最高收益：PreferredNodeAlignment/AlignKind → edit_rectangle + ui_create_rectangle；
collision pair → finish_edit_rectangle；autoclutter → wall_roof_hashmap；
camera matrices → get_closest_point_u_to_cursor；UiState80/widget 类型 → 6 个 UI 系统）。
