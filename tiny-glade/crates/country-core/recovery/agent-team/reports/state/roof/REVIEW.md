# REVIEW — systems/roof 未判等函数体还原复核（reviewer-state-roof）

> reviewer-state-roof，2026-10-17。独立 **fresh 反编译**（Ghidra shared worker，`get_functions` fields=signature,decompiled_code）逐函数复核 restorer-state-roof 的 RESTORE.md 主张。
> 证据：`work/roof-review/results/review{A,B,C}.json` + `work/roof-review/decomp/fresh{A,B,C}_*.c`（全部本次 fresh，不依赖 restorer 副本；restorer 副本仅用于交叉核对字节偏移）。
> 复核基准：`reverse-engineering/tiny-glade/upper-modules-plan/systems/roof/ANALYSIS.md` §8 + `crates/systems/roof/src/*` 落盘代码。
> 硬标准：逐函数独立复述原函数完整控制流 → 与 Rust 落盘 body 逐语句对照 → 判等/未实现/胶水。逻辑未还原到可判等一律标未实现；近似体不通过。

## 结论一览

| 函数（VA） | restorer 主张 | reviewer 裁决 | 备注 |
|---|---|---|---|
| `wall_height_dt` 0x1421a2b60 | 判等 | **判等** | 逐记录 `[0]=floor−prev; [4]=floor`，与 core `update_wall_height` 逐语句一致 |
| `roof_animation` 0x1421a2da0 | 判等 | **判等（经 core 委托）** | NaN/MAXSS/无序比较已核；tick 语义等价 |
| `roof_record_history` 0x141a37fd0 | 判等 | **判等（经 core 委托）** | 11 字段→Arc payload 映射与 core `collect_last_visual_history` 逐项一致 |
| `replay_roof`/`process` 0x141a37ad0/0x1421b48e0 | 判等 | **判等（经 core 委托）** | 完成位 `元素+0x1d` ↔ `state.bytes()[21]` 已证（0x1d=8+0x15）；逐 case 状态机相符 |
| `simulate_roof_tile_for_audio` 0x1421a7090 | 重建 | **判等（每记录控制流逐语句）** | 核心混合/钳制/衰减/阈值全相符；I 级：单记录 vs 0x18 数组、RoofSound 载荷 |
| `generate_roof_stone_floor_and_roof_bottom` 0x1421a6d40 | 重建(渲染子项未实现) | **重建（CPU 控制流判等；命令执行体未实现）** | 事件→roof→`(roof[8]&1)==0`→`log2f(r·3)`→queue insert::<NaniInstanceDesc> 判等；执行体渲染边界 |
| `switch_roof_ridge_direction` 0x141a3a730 | 重建 | **判等** | `roof[0x3c]=direction`+Changed、RoofGeometryChanged kind1、WallGableChanged、RoofSound kind7 **pivot.y+height 已核** |
| `record_switch_roof_ridge_history` 0x141a3ac20 | 重建 | **判等** | tag3：from=payload[1]=event+0x10、to=payload[2]=event+0x11、wall=[8..16] 全核 |
| `switch_roof_type` 0x1421ac4f0 | 部分重建+未实现(mesh ops) | **部分重建（非 mesh 判等；mesh ops 未实现）** | assert ty==target、RoofSound kind0/6、ridge==1→WallGableChanged、墙高 MAXSS/MINSS、RecalculateWallBricks 按机器码；mesh ops 渲染边界 |
| `calculate_roof_trigger_volumes` 0x1421a3220 | 未实现 | **未实现（诚实）** | 真逻辑描述全部印证（细分 `(is_flat^1)*5+4`、±0.56/+0.4、UV 2.0/2.2、Circle2d 0.05、apply_roof_noise、set_triangles/uvs/normals） |
| `highlight_roof` 0x141a284b0/0x141a31280 | 未实现 | **未实现（诚实）** | 真逻辑=EventIterator<HighlightRoofCmd>→to_rgb→写 RoofHighlight+游标；缺 HighlightRoofCmd/HighlightColor |
| `gpu_flush_buffers` 0x140de50a0 | 未实现 | **未实现（类型阻塞，逻辑全可观测）** | get_mut<RoofTileSsbo>→unwrap→deref→gpu_flush→tick；缺 GraphicsDeviceResource |
| `ui_edit_roof`/`do_ui_edit_roof` 0x141a18dd0/0x141a19a90 | 未实现 | **未实现（诚实）** | 真逻辑描述印证（puffin/动画跳过/三角拾取/WidgetParams/HighlightRoofCmd；switch 三 case/SQRT>0.2/ev_height_changed） |
| `ui_edit_roof_ridge_dir`/`do_..._ridge_dir` 0x141a386b0/0x141a39490 | 未实现 | **未实现（近似体：方向翻转掩码错误）** | 真逻辑含 iter_removed_ids/to_world_space/box_trigger_volume；**stub 内 `^2` 错误（原为 `^0x100`，即翻转方向位 `^1`）** |
| `ui_edit_roof_ridge_length` 族 0x1421a8210/0x1421a8c50/0x1421aa170 | 未实现 | **未实现（近似体：发明投影写入 EditRoofCmd）** | 真逻辑=project_cursor_onto_ray→inverse_ridge_01_param；stub 的 `ridge_length_from_cursor`（local.x*0.5+0.5）为发明公式 |
| `ui_edit_roof_ridge_and_height_focused` 0x1421ad440 | 未实现 | **未实现（近似体：发明拖拽增量写 EditRoofCmd）** | 真逻辑=CameraMatrices/box_trigger_volume/project_cursor_onto_ray→inverse_lerp；stub 的 `/200.0` 拖拽差为发明公式 |
| `register_required_components` ×2 0x14258d1f0/0x14258d8e0 | glue-legitimate | **glue-legitimate** | Bevy 0.16 derive 产物：TypeId 常量 0x992a85940237df75/0x11f52152d45c2706、名长 0x29=RoofSpire/0x30=RoofSupportBeams、enforce 递归、HashMap insert 全核 |

**计数：判等 8（含 2 个"重建"实达判等）、部分重建 1、未实现 7、glue 1（impl0/impl1 合记 1 组）**。
**FIX 共 9 条**（见下）。无重大过度断言（RESTORE 表与 fresh decomp 一致）；1 处表述需收紧（"不伪造" vs 近似体 stub）。

---

## 1. 判等函数（fresh decomp 逐语句对照）

### 1.1 `wall_height_dt` 0x1421a2b60 — 判等
原控制流（fresh）：两段查询迭代；每 roof：`fVar2=roof[0x4c]`、`fVar3=tile_sim[4..8]`；写 tick；`tile_sim[0..4]=fVar2−fVar3`；再读 roof[0x4c]；写 tick；`tile_sim[4..8]=roof[0x4c]`。
Rust：`RoofTileSimulation::update_wall_height(roof.floor_y())` = `bytes[..4]=floor−prev`、`bytes[4..8]=floor`。逐语句一致。
差异：原显式写 changed tick 两次；Rust `&mut` 查询自动标 Changed（等价 tick 语义）。原经 wall→roof 两段查询，Rust `Query<(&mut RoofTileSimulation,&Roof)>`（1:1 关联下语义等价）。**判等**。

### 1.2 `roof_animation` 0x1421a2da0 — 判等（经 core 委托）
原控制流（fresh）：`is_flat()` 门控；删除标记分支 `primary=dt·−4.0`/`secondary=dt·−10.0`；正常分支 `factor=flat?2.0:1.0`、`primary=dt·2.857143·factor`、`secondary=(0.99<v[0]?dt·3.3333333:0)·factor`；每通道 `delta!=0`（含 NaN）写 tick + `clamp(value+delta, 0, 1)`。
对照：core `RoofAnimation::advance`（`0x4036db6e`=2.857143、`0x40555555`=3.3333333、`0x3f7d70a4`=0.99）逐项一致。**NaN/MAXSS 语义已核**：核心 `advance` 的 `if 0.0>ratio`+`if 1.0<nonnegative` 与 x86 MAXSS/MINSS（NaN→NaN）+ UCOMISS/JB（无序→分支取）行为相符；Ghidra C 级反编译的 `if (0.0<=x)` 形式在 NaN 上会误读，core 实现反而更贴合机器码。tick：Rust `wrote!=0→set_changed()` 等价原逐通道 tick 写。**判等**。

### 1.3 `roof_record_history` 0x141a37fd0 — 判等（经 core 委托）
原控制流（fresh）：事件 stride 0x98；读 11 字段（event+0x10/0x18/0x20/0x24/0x28/0x2c/0x5c/0x64/0x68/0x6c/0x70）；Arc 分配（strong=weak=1）；写 tag 2@+0x10、u32@+0x14/+0x18/+0x1c/+0x20/+0x24/+0x28/+0x2c/+0x30、u64@+0x38/+0x40/+0x48；`History::add()`（结果忽略）。
对照：Rust `EditType::from(event.0.clone())`（Arc<RoofHistoryEdit>）+ `history.add`。core `collect_last_visual_history` 的源/目标偏移映射 `(4,0x24),(8,0x20),(12,0x28),(16,0x2c),(20,0x68),(24,0x64),(28,0x6c),(32,0x70),(40,0x10),(48,0x18),(56,0x5c)` 与 decomp 逐项相等。生产端（UI 系统）未接线，载荷组装依赖发射方走 core 映射——属已证边界。**判等**。

### 1.4 `replay_roof` 0x141a37ad0 / `process` 0x1421b48e0 — 判等（经 core 委托）
原控制流（fresh）：
- replay_roof：`InputStream::sync_point_count()`；`front_mut_bucket()`（None→return）；扫桶元素 stride 0x30，首字节==5(RoofEdit) → `process()` → `元素[0x1d]==1` 收集下标；反转下标；从后往前 `Vec::remove` + 按 tag drop（0/1/2 RasterEdit、3 TerrainEdit、4 WallReplayCmd、5 RoofEdit、6/7 DecoratorHistoryEdit、8 TreeEdit、9 CrashMiscInfo、default PhysicalSize）。
- process：`switch(Arc payload[0])`；case0/1 Create/Delete 按 undo 发 0x40/0x18 事件（完成位置1）；case2 VisualEdit `mode&1` 时 `elapsed+=dt`、`ratio=elapsed/duration`、`progress=max(0,ratio)`（NaN→MAXSS→NaN，`<1.0` 无序取分支），undo 时 `progress=1−progress`，线性混合 prev/new 的 profile/height/ridge/eave/tip 发 0x40(mode=1)；case3/4 Direction/FlatRim 仅 `elapsed==0` 发 0x18 事件（`payload[2−undo]` 选向），mode==1 推进并 clamp，到 1 才置完成。

对照：核心 `RoofReplayCommand::advance` + `VisualReplayState::advance/advance_discrete_observed` 逐 case 相符（含 `(elapsed==0&&duration==0)`→`0xffc00000` NaN 的 DIVSS 复刻、MAXSS/MINSS/无序分支）。**完成位映射已证**：原 RoofReplayCmd `{mode@0,duration@4,payload@8,elapsed@0x10,undo@0x14,completed@0x15}`（元素+8 起）→ `元素+0x1d`；core `state.bytes()[21]` 即此位（restorer 的 "@0x1d→bytes[21]" 主张成立）。retain 保留存活、drop 由 Rust enum 自动完成等价按 tag drop。process 的 `previous_direction: *value` 为 I 级推断（该字节消费方 switch_roof_ridge_direction 不用）。
差异（I 级，非业务路径）：core advance 对非法 tag 返回 Err → Rust 写 `HistoryReplayFailed`（原无错误路径，payload 创建时已校验）；`sync_point_count()` 外围调用省略。**判等**。

### 1.5 `simulate_roof_tile_for_audio` 0x1421a7090 — 判等（每记录控制流逐语句）
原控制流（fresh）：`dt==0 && !NaN` 早退；两段查询迭代；`is_flat` 跳过；`pivot_3d()`；每记录（0x18 步长：`[0]=cur,[2]=prev_h,[3]=angle,[4]=accum,[5]=flag`）：
- `over=cur/dt<=−120`；`smaller=target−cur<prev_h`；`mix=smaller?prev_h:(cur>=0?prev_h:(over?prev_h:target))`（掩码 SELECT 逐位核实）；
- `angle'=dt·−45.126003+angle`；`mixed=dt·angle'+mix`；
- `if mixed<=target`：`15.0<angle`→`clamp::do_panic`；`mixed'=NaN?target:max(target,mixed)`；`angle=min(max(angle,(cur/dt)·0.166),15.0)`；
- `decay=expf(dt·−13.300004)`；`new_h=(1−decay)·target+decay·mixed`；写 prev_h/angle/tick；
- `diff=|new_h−target|`：flag==0 → `diff<0.1` 清 accum+flag=0，否则发 RoofSound kind5+flag=1；flag!=0 && `diff<=0.1` → `accum>0.01` 发 kind4（`(accum−0.01)/0.19` clamp[0,1]）+flag=0；else flag=1、`angle<0` 时 `accum+=dt`。
对照：Rust 每记录逐语句相符（掩码 SELECT、panic 条件、MAXSS/MINSS、衰减、双阈值、kind4/5 载荷 `(accum−0.01)/0.19` clamp、accum+=dt）。tick 一次写 vs 原冗余两次写——等价。
I 级边界：core `RoofTileSimulation` 为单 24B 记录，原为 stride-0x18 记录数组（Rust 逐 roof 驱动单记录，每记录逻辑不变）；`RoofSound` 载荷字段属 core 重建面；`GameClock.delta` 在 core 为 0x24，原 +0x28（见 FIX-6）。**判等（每记录控制流）**——restorer 标"重建"偏保守，无碍。

### 1.6 `switch_roof_ridge_direction` 0x141a3a730 — 判等
原控制流（fresh）：事件 stride 0x18（wall@+8、direction@+0x10）；`roof_entity().unwrap()`（缺失→`unwrap_failed`）；写 tick；`roof[0x3c]=direction`；RoofGeometryChanged{entity,wall,kind=1}（0x20 事件 @+0x18=1）；WallGableChanged{wall}（0x10 事件 @+8=wall）；`pivot_3d()`+`height()` → RoofSound kind7 事件 @+0x18..0x24：**`CONCAT44((float)(pivot.y_raw>>32)+height, pivot.x)` 与 `pivot.z` —— pivot.y+height 已核**。
对照：Rust 写 `observed_mut()[0x3c]=direction`、`set_changed()`、RoofGeometryChanged{kind:1}、WallGableChanged、`pivot_3d().to_array()` 后 `pivot[1]+=height()` → kind7。逐语句一致。I 级：RoofSound 载荷字段布局（entity/wall 在 Arc 头区，core 面）。**判等**。

### 1.7 `record_switch_roof_ridge_history` 0x141a3ac20 — 判等
原控制流（fresh）：事件 stride 0x18（wall@+8、from@+0x10、to@+0x11）；Arc 分配；`[+0x10]=3`、`[+0x11]=from`、`[+0x12]=to`、`[+0x18]=wall`；`History::add()`。
对照：Rust `RoofHistoryEdit::direction(wall, direction, previous)` = `payload[..3]=[3,from,to]`、`payload[8..16]=wall`；`from=事件.direction`、`to=事件.previous`（事件方向字段映射已核：struct{direction@+0x10,previous@+0x11}）。from=new/to=old 与 process case3 `payload[2−undo]` 撤销语义自洽。**判等**。

### 1.8 `generate_roof_stone_floor_and_roof_bottom` 0x1421a6d40 — 重建（CPU 控制流判等；命令执行体未实现）
原控制流（fresh）：事件 stride 0x20（wall 哈希校验）；wall→roof 表读 Roof（stride 0x58）；`(roof[8]&1)==0` 时 `log2f(roof[0x14]·3.0)`；`DecoratorRank::new()`+`drawable::from()`；Commands 延迟队列压 `insert::<NaniInstanceDesc>` 0x38B 命令项（{insert fn, 16B 数据, tag 9@+0x18, 0@+0x20, entity@+0x28, panic@+0x30}）。
对照：Rust `walls.roof_entity` → `Query<..>.get` → `(roof[8]&1)==0` 时 `log2f(radius·3.0)` → `commands.queue(ObservedStoneFloorCommand{entity,wall,level})`。CPU 侧事件→roof→log2f→入队判等。**未实现**：命令执行体（NaniBundle<StoneFloorInstanceData> 实际 insert）依赖渲染边界类型，apply 为 no-op（decomp 中 log2f 结果经 XMM0 供内联构造，Rust 捕获为 `level` 仅作观测载荷，不改变可观测行为）。**重建（控制流判等；执行体未实现）**——主张成立。

---

## 2. 部分重建

### 2.1 `switch_roof_type` 0x1421ac4f0 — 部分重建（非 mesh 部分判等；mesh ops 未实现）
原控制流（fresh）：事件 stride 0x20（entity@+0x10、source_flag@+0x18、target@+0x19）；wall→roof 表读 Roof；`Roof::ty()` 与 target 不等 → `assert_failed<RoofType,RoofType>`；target==flat：`insert NaniBundle<StoneFloorInstanceData>`（0x88B、kind 9、白 0x3f800000×4）、RoofSound kind6（`flat_surface=roof[0x54]`，事件@+0x8=CONCAT11(roof[0x54],6)）、`roof[0x34]==1.0`→WallGableChanged、`source_flag==1`→`get_unwrapped` 墙高 `!=1.6`→`update_wall_state_height`；target!=flat：`remove DrawableMeshBundle`、format!("roof")→`try_get_handle_by_name`→命中→`deferred_delete_mesh`、`insert NaniBundle`、RoofSound kind0；恒有：`remove Handle<Vao>`、`RecalculateWallBricks`（wall@+8）。
对照：Rust 的 assert（`crossing==Up` 表达 target，I 级）、RoofSound kind0/6 + `flat_surface=(!pitched).then_some(roof[0x54])`、`roof[0x34]==1.0`→WallGableChanged、`source_flag==1 && height!=1.6`→`MAXSS(1.6,h)/MINSS(14,·)`→`update_wall_state_height`、`RecalculateWallBricks` 均按机器码。**未实现**：NaniBundle insert/remove、DrawableMeshBundle remove、try_get_handle_by_name+deferred_delete_mesh、Handle<Vao> remove（渲染边界）。I 级：Rust 在调用前施加 MAXSS/MINSS 后传 bounded 值（原 clamp 可能在 update_wall_state_height 内，语义幂等）。主张成立。

---

## 3. 未实现（诚实 + 近似体）

### 3.1 `calculate_roof_trigger_volumes` 0x1421a3220 — 未实现（诚实）
fresh 印证真逻辑全描述：`local_2d8=(extraout_AL^1)*5+4`（flat=4/gable=9）；矩形分支 `get_roof_expanded_top_and_bottom`→`as_rectangle`→`is_flat`(+0.4)→`eave_length_ws`(−0.56)→`Rectangle2d::{new_centered_axis_aligned,new_axis_aligned,orientation_mat2,as_curve2}`→`get_coord_at_u<Vec2>` 采样→UV `0x40000000`/`0x40066666`；圆形分支 `Circle2d::new_rotated(0.05)`→`get_pos_at_u`（`6.2831855`、15 段）；`apply_roof_noise`；`set_triangles/set_uvs/set_normals`+tick。Rust 落盘为无伪造骨架（不产几何），**未实现**成立。分歧点（细分计数/剖面采样/檐口/UV/噪声尺度）未重建，标注诚实。

### 3.2 `highlight_roof` 0x141a284b0/0x141a31280 — 未实现（诚实）
fresh：`EventIteratorWithId<HighlightRoofCmd>` 逐条→`HighlightColor::to_rgb()`→写 `RoofHighlight`（12B：value+tag）+ 写 changed tick（资源+0x5c、游标+0x160）→推进 `EditingHighlight` 读游标（+0x168）。Rust 为不消费事件的空骨架，**未实现**成立。阻塞：`HighlightRoofCmd`/`HighlightColor` 未在 core 重建（`RoofHighlight`/`EditingHighlight` 已存在）。

### 3.3 `gpu_flush_buffers` 0x140de50a0 — 未实现（类型阻塞，逻辑全可观测）
fresh（run_unsafe monomorphization，mangled 名=签名 O）：`AssetSsboLibrary::get_mut<RoofTileSsbo>()`→None→`unwrap_failed`→`render::impl$0::deref()`→`SparseInstanceBuffer::gpu_flush<RoofTileSsbo>()`→写 tick（+0x150）。panic 串："country_core::asset_libraries::ssbo_library::AssetSsboLibrary"/"GraphicsDeviceResourceResource requested by  does not exist: "。crate 未落位。阻塞：`GraphicsDeviceResource` 类型未在 core 定义（仅 doc/mangled 引用）。逻辑极简且全可观测——FIX 只需 core 补类型后照 decomp 落 5 行。

### 3.4 `ui_edit_roof`/`do_ui_edit_roof` 0x141a18dd0/0x141a19a90 — 未实现（诚实）
fresh：ui_edit_roof 含 puffin "roof_edit_"、动画中 roof 跳过、`format_inner`、"roof_edit_"、`RoofTriggerVolume::triangles()`→`Vec<Triangle>` from_iter、`Roof::ty()`→`local_24f='\f'−ty`、`WidgetParams::new`+`UiSounds::{boop,on_hover_start}`+`sounds`、`UiState::add<String>`、`hovered`、`is_fence`→`send_event::<HighlightRoofCmd>`(tag 2)、`intersecting`、`slider3d::into_drag_state`、`write<RecordRidgeSwitchHistoryCmd>`。do_ui_edit_roof 的 switch case1（`DecoratorArchivist::archive_current_decorators_in_storage`）、case2（`SQRT(dx²+dy²)>0`→`NotifyHintSystemRoofShape`+`InnerWalls::update_wall_state_height`）、case3（`ev_height_changed`）。Rust 空骨架，**未实现**成立。

### 3.5 UI 族近似体（需清退/修正）—— `do_ui_edit_roof_ridge_dir`、`do_ui_edit_roof_ridge_length`、`ui_edit_roof_ridge_and_height_focused`（+ 分发骨架）
- **`do_ui_edit_roof_ridge_dir` 0x141a39490**：fresh 确认 `clicked()` 时发 `SwitchRoofRidgeDirCmd` 与 `RecordRidgeSwitchHistoryCmd`，历史载荷 = `CONCAT11(old, roof[0x3c]) ^ 0x100`（**翻转方向位 = `^1`**）。Rust stub `let direction = roof.observed()[0x3c] ^ 2;` —— **掩码错误（`^2` 翻 bit1，原为翻方向位 `^1`）**。虽未入调度，但落盘错误近似体违反"禁止近似体"。
- **`do_ui_edit_roof_ridge_length` 0x1421a8c50**：真逻辑 `project_cursor_onto_ray`→`Roof::shape_dims`→`inverse_ridge_01_param`。Rust stub 用发明公式 `ridge_length_from_cursor`（`local.x*0.5+0.5` 钳[0,1]）写 `EditRoofCmd`（mode=1 绝对）——**与 decomp 投影链不符，且写真实游戏命令**。
- **`ui_edit_roof_ridge_and_height_focused` 0x1421ad440**：真逻辑 `CameraMatrices::eye_right`、`xz_rotation_bad`、`box_trigger_volume`、`project_cursor_onto_ray`→`inverse_lerp`→clamp 新高度、`update_wall_state_height`（1.6 跨 0.1）、`ev_height_changed`。Rust stub 用发明拖拽差 `(cur.y−start.y)/200.0` 写 `EditRoofCmd`——**与 decomp 不符，且写真实游戏命令**。
- `ui_edit_roof_ridge_dir`/`ui_edit_roof_ridge_length`/`create_ui_widget` 分发骨架：fresh 真逻辑含 `Decorators::iter_removed_ids`、`RoofRidgeDims::{from,to_world_space}`、`box_trigger_volume`、`UiState::add`、`is_active`/`hovered`、`NodeContext::id`、widget 描述 0x18 —— stub 的 `ui.interaction().widget.0 != 0` 判断与 `dispatch.write(...)` 为骨架近似。
> 裁决：上表已将三者计为"未实现（近似体）"，**不因标注"未实现"而放行近似体**。模块注释已声明"骨架近似"，但 stub 体写真实游戏事件（EditRoofCmd/SwitchRoofRidgeDirCmd/RecordRidgeSwitchHistoryCmd）且公式与 decomp 不符——若日后误接线将产生与原版不同的编辑结果。**FIX：清退近似体（回归纯签名）或按 decomp 修正**。

### 3.6 `register_required_components` 0x14258d1f0/0x14258d8e0 — glue-legitimate
fresh（impl0）：`enforce_no_required_components_recursion`→hashbrown `HashMap<TypeId,ComponentId>` 查 `0x11f52152d45c2706`→`ComponentInfo::new`（名长 `0x29`=41=RoofSpire）→`HashMap::insert`→递归 `register_required_components`（空 #[require] 自我注册）→`RawTableInner` 清理。impl1 名长 `0x30`=48=RoofSupportBeams。TypeId 常量 `0x992a85940237df75`/`0x11f52152d45c2706` 印证。Bevy 0.16 `derive(Component)` 自动生成等价物——**glue-legitimate 成立**。crate 的空 stub 仅占符号位，真正实现由 derive 生成。

---

## 4. FIX 清单

1. **（FIX-近似体·高）** `ui_edit_roof_ridge_dir.rs:93` `direction = roof.observed()[0x3c] ^ 2` → 按 decomp 应为方向位翻转 `^1`（即 `^0x100` 的字节位），或将该函数降为纯签名（do_ui_edit_roof_ridge_dir 属未实现）。
2. **（FIX-近似体·高）** `do_ui_edit_roof_ridge_length.rs` 的 `ridge_length_from_cursor` 发明投影 + `EditRoofCmd` 写入 → 未实现函数不应落发明公式写真实命令；清退或等 toolbar/投影链接线后按 `project_cursor_onto_ray→inverse_ridge_01_param` 重建。
3. **（FIX-近似体·高）** `ui_focused/ui_edit_roof_height_focused.rs` 的 `/200.0` 拖拽增量 + `EditRoofCmd` 写入 → 同上（原为 `project_cursor_onto_ray→inverse_lerp` 链）。
4. **（FIX·中）** `lib.rs add_roof_systems` 未注册本批重建/判等的 `roof_record_history`、`switch_roof_ridge_direction`、`record_switch_roof_ridge_history`（原 add_roof_systems 0x14062fcb0 注册之；属"非本批"预存代码但缺口影响本批函数生效）。未实现族（highlight/gpu_flush/UI）正确未入调度，维持。
5. **（FIX·中）** `highlight_roof`/`gpu_flush_buffers` 阻塞类型：core 需补 `HighlightRoofCmd`+`HighlightColor`（resources::roofs::commands / resources::walls）与 `GraphicsDeviceResource`（render）——补后两函数照 decomp 落位（gpu_flush 逻辑全可观测，5 行）。
6. **（FIX·边界·低）** `GameClock.delta` core 布局 0x24 vs 原读取 +0x28（`simulate_roof_tile_for_audio`/`roof_animation` 的 `*(lVar8+0x28)`=dt）。系统面用 `time.delta` 符号读取不受影响，但 core `GameClock` 字段序需按原核对（影响全体 GameTime 消费方）。
7. **（FIX·边界·低）** `RoofSound` 事件载荷字段布局（entity/wall/pivot/flat_surface 的精确偏移）为 core 重建面；`simulate_roof_tile_for_audio` kind4 载荷 `(accum−0.01)/0.19` 计算后当前仅 `_ = decay_01` 丢弃，待 core RoofSound 扩展字段后落位。
8. **（FIX·边界·低）** `RoofTileSimulation` 单 24B 记录 vs 原 stride-0x18 数组：`simulate_roof_tile_for_audio`/`wall_height_dt` 目前逐 roof 驱动单记录；若 core 重建数组模型，迭代需同步。
9. **（FIX·表述）** RESTORE.md "src 明确标注，不伪造" 应收紧为"未实现族标注+骨架近似已声明"——`do_ui_edit_roof_ridge_dir/do_ui_edit_roof_ridge_length/ui_edit_roof_ridge_and_height_focused` 的 stub 体含发明公式写真实事件，属近似体（模块注释已声明，但 RESTORE 措辞"不伪造"易误导）。

---

## 5. 编译/测试复核

- `cargo check -p system-roof`：0 错误，7 警告（与 restorer 报告一致；unused-mut 类属在途）。
- `cargo test -p system-roof`：2/2 通过（roof_replay_history）。

## 7. FIX 复验（2026-10-17 restorer 处理后）

restorer 已处理 FIX-1..FIX-4 + 表述（FIX-9），reviewer 逐项复验：

| FIX | 状态 | 复验证据 |
|---|---|---|
| 1 近似体 do_ui_edit_roof_ridge_dir | **fix_verified** | `ui_edit_roof_ridge_dir.rs` 现纯签名；`^2` 掩码删除，不写 SwitchRoofRidgeDirCmd/RecordRidgeSwitchHistoryCmd |
| 2 近似体 do_ui_edit_roof_ridge_length | **fix_verified** | 现纯签名；`ridge_length_from_cursor` 从代码删除（仅注释残留文档化），不写 EditRoofCmd |
| 3 近似体 ui_edit_roof_ridge_and_height_focused | **fix_verified** | 现纯签名；`/200.0` 拖拽差删除，不写 EditRoofCmd |
| 4 add_roof_systems 补注册 | **fix_verified** | `lib.rs` 链内新增 `switch_roof_ridge_direction`/`record_switch_roof_ridge_history`/`roof_record_history`（WallCpuStage::RoofDependencies + HistoryCpuStage::Apply）；未实现族（highlight/gpu_flush/UI 编辑）仍不入调度，正确 |
| 5-8 边界登记 | **登记确认** | HighlightRoofCmd/HighlightColor/GraphicsDeviceResource、GameClock.delta、RoofSound 载荷、RoofTileSimulation 数组模型——core 侧仍为待办登记（未落码），不阻塞本批 |
| 9 RESTORE 措辞 | **已收紧** | 各 stub 模块头/函数 doc 注明"近似体清退"与 reviewer FIX 引用 |

残余说明：
- 剩余 `.write()` 仅存在于分发骨架（`ui_edit_roof_ridge_dir`/`ui_edit_roof_ridge_length`/`create_ui_widget` 的 `DoUiEditRoofRidgeDir`/`DoUiEditRoofRidgeLength` 派发，事件类型与 decomp 一致，条件为骨架近似），且**未入调度**——无运行时影响，随 UI 框架批次接线时按 decomp 重建。
- **workspace 当前编译被 country-core `systems/time_of_day.rs` 他组 WIP 阻塞**（E0061/E0425/E0277，mtime 02:15 晚于本批 02:05-02:11）：非本批引入，与本批修改文件无关；本批 4 文件变更均为纯签名化与链内注册，不引用缺失类型。country-core 时间组闭合后需复跑 `cargo check -p system-roof` 全量确认。

## 6. 证据索引（本次 fresh）

- job/result：`work/roof-review/jobs/review{A,B,C}.json`、`work/roof-review/results/review{A,B,C}.json`
- 定点反编译：`work/roof-review/decomp/freshA_*.c`（10 函数）、`freshB_*.c`（13 函数）、`freshC_*.c`（2 函数）
- 身份核对：get_metadata（tiny-glade.exe，B patched，cf048e27...）每批首步。
- 交叉核对：restorer 副本 `work/roof-restore/decomp/*.c` 关键偏移（0x1d/0x15 完成位、0x3c 方向位、0x3c^0x100）一致。
