# RESTORE — systems/timberframe + systems/roof wave3 梁写路径/屋顶写路径（restorer-wave3-timber-roof，2026-10-17）

> 硬标准（真实逻辑非形状）：每业务函数 **fresh 反编译**（Ghidra shared worker，`work/wave3-timber-roof/evidence/batch1-decomp.json` / `batch2-utils.json`，get_metadata 身份 = tiny-glade.exe patched B）→ 重建完整控制流 → Rust body 逐语句对照。
> 总账条目：`planning/2026-10-17-unimplemented-ledger.md` §state/timberframe（construct 梁写路径/裙层环数/屋顶环）＋ §state/roof（simulate 完整/calculate_roof_trigger_volumes/highlight_roof/gpu_flush_buffers/switch_roof_type mesh ops）。

## 逐函数判定

| 函数（VA） | 判定 | 说明 |
|---|---|---|
| `construct_timberframe_rectangle`（0x141a41ba0） | **重建 + 梁写路径落库**（结构/计数判等；写路径按 core WoodenPlankSsbo） | §1 |
| `construct_timberframe_circle`（0x1421b67a0） | **重建 + 梁写路径落库**（同上） | §2 |
| `timberframe_beam_count_change_audio`（0x1421b6610） | 判等（主体，不变） | 无改动 |
| `simulate_roof_tile_for_audio`（0x1421a7090） | **判等（修正 angle' 三处）** | §3 |
| `calculate_roof_trigger_volumes`（0x1421a3220） | **未实现（诚实）+ 细分计数登记** | §4 |
| `highlight_roof`（0x141a284b0/0x141a31280） | **重建**（事件→RoofHighlight/EditingHighlight 数据流落位） | §5 |
| `gpu_flush_buffers`（0x140de50a0） | **结构落位 + gpu_flush GAP（RoofTileSsbo 非 SparseSsbo）** | §6 |
| `switch_roof_type`（0x1421ac4f0） | 部分重建（不变；mesh ops 渲染边界 GAP） | §7 |
| `add_timberframe`（0x140630a20）/`timberframe_on_glade_unload` | glue（不变） | — |

## 编译/测试

- `cargo check -p timberframe`：**0 错误**（最终复跑 `Checking rhapsody/country-core/timberframe … Finished`；验证窗口内 country-core/rhapsody 他组 WIP 瞬态错误曾阻断两次，均已由其组闭合）。
- `cargo check -p system-roof`：**0 错误**，2 警告（delete_roof/update_roof_spatial 预存 unused-import，非本批）。
- `cargo test -p system-roof`：**2/2 通过**（roof_replay_history）；`cargo test -p timberframe`：编译通过（无测试）。

## 跨域改动（core，登记）

- `TimberframeConstructionParamsCache`（`country-core/src/systems/wall/shapes/calc_shape_height.rs`）：`{_gap: u64}` 8B 占位扩展为 `{_gap u64@0, skirt_offset f32@8, main_height f32@0xc}`（16B，对齐原布局）。依据：fresh 0x141a41ba0/0x1421b67a0 读 cache+8/+0xc 两 f32。仅 timberframe 两构造使用（grep 无其他消费），低风险。

## §1 矩形（0x141a41ba0）——重建 + 梁写路径

**已重建（判等）**：相位结构与顺序（槽空建 allocation → 二次查询+互斥 → writer.clear → 事件迭代 → 垂直梁 → 等距梁 → 裙层 → 屋脊 → 每事件写回）、`n=⌊d/1.51⌋`+`t=i/n` 等距算法、截面 0.2/0.18、裙层 `split_height_into_skirts(cache+0xc)` + {0.0, cache+0xc} 两固定环（环序 O：split 表在前、固定环在后）、`count += min(lenA,lenB)` 屋脊 + `perimeter>0.5 → +1`、`BeamCount.count`（i64 @+8）每事件写回、`event[0x1c]` 门控（= `cmd.shape[24]`，payload 自 event+4 → event[0x1c]=payload[0x18]=shape[24]）的存在与位置。

**梁写路径（落库）**：经 `AssetSsboLibrary::get_mut<WoodenPlankSsbo>` 取 `SparseInstanceBuffer`；槽空 `new_allocation()` 存 `TimberframeInstanceBuffer`；`writer().clear()` + 逐梁 `push(WoodenPlankSsbo {position, size, rotation})`。推入失败（NoFreeChunks）置标志、事件末尾 log「Timberframe constructor ran out of memory」（O uVar24 bit0）。

**分歧点（GAP/I 级）**：
- 库面：原 `Res<AssetSsboLibrary>`（frozen 内部可变图）写；core 占位 `RwLock<HashMap>` 需 `&mut` → 本系统 `ResMut<AssetSsboLibrary>`（登记同 world_raster/rasterize_buildings 写库面）。
- 朝向：`orientation_mat3`（0x140c99570）= 本地精确镜像（`Mat3::from_cols((axis.x,0,axis.y), Y, (-axis.y,0,axis.x))`）；裙/屋脊 `rotation_from_normal`（0x140c90a40）utils 未提供 → `beam_rotation(edge)`（`Quat::from_mat3(orientation_mat3(edge))`）等价旋转（I 级，utils 补位后换回）。双覆盖符号连续性 I 级。
- `RenderInstance` 中间记录（`impl$1::from` 输入面）字段序 I 未落库 → 梁实例按 core `WoodenPlankSsbo {position,size,rotation}` 直写。
- 事件 bit0 门控（rect 处理 bit0=1）：core 契约未暴露事件首字节 → `get_shape()` 形状过滤代理（I）。
- `as_curve3` 高度实参（寄存器面 I）：本实现以 0.0 复用一调用点（只影响梁 y 位、不影响点数/计数面等价，restorer 已登记）。
- 屋脊 `RoofRidgeDims::calc` 的 `ridge_dir` 字节 U；`roof_height` 作梁位 y（`RoofRidgeDims::to_world_space` 待接入，I）。

## §2 圆形（0x1421b67a0）——重建 + 梁写路径

**已重建（判等）**：`count=max(6,⌊perimeter/1.5⌋)`、`u=i/(count-1)` 采样、相位结构（槽门控 panic 面 → 主环 → 裙层 → 屋顶环 → 每事件写回）、裙层 `split_height_into_skirts` + {0.0, cache+0xc} 两固定环、屋顶环 `event[0x1c]` 门控（= shape[24]）、空槽 panic（非 return）、`BeamCount.count` 每事件写回。

**梁写路径（落库）**：同 rect（`get_mut<WoodenPlankSsbo>` + 既有 allocation + writer 逐梁 push）。

**分歧点（GAP/I 级）**：同 rect 库面/朝向/RenderInstance/bit0。**主环朝向 GAP**：解压主环从 `impl$1::from()` 输入面取（`{0, cache+0xc, 0, 0}` 初始态，RenderInstance 字段序 U）→ 本实现以 `Quat::IDENTITY` 常值占位（主环梁朝向待 RenderInstance 合同）。屋顶环高度 = `roof_height_ws_from_wall_shape + cache+8 + cache+0xc`（O）。裙层采样 `u=i/(count-1)` 相邻点法向（O）。

## §3 音频（0x1421b6610）——判等（主体，不变）

无改动。既有 `判等（主体）`（frame 读 +0x2c 裸读、`(Δframe==1)&&(last_count!=count)`、kind=6、回写序）维持；`get_building_size_01`/PlayTimberframeSound size/id 字段 GAP 转 core（总账已登记）。

## §4 calculate_roof_trigger_volumes（0x1421a3220）——未实现（诚实）+ 细分计数登记

fresh 印证真实逻辑全描述：细分 `(is_flat^1)*5+4`（平顶 4 / 坡顶 9）；矩形分支 `get_roof_expanded_top_and_bottom`→`as_rectangle`→`is_flat`(+0.4)→`eave_length_ws`(−0.56)→`Rectangle2d::{new_centered_axis_aligned,new_axis_aligned,orientation_mat2}`→`as_curve2`×2→`get_coord_at_u<Vec2>` 采样→`height()` 混顶→`for q in 0..4` 子三角→`Triangle::normal`→UV `0x40000000/0x40066666`；圆形分支 `as_circle`(+0.13)→`profile_curve_ws`/`roof_tip_offset_xz`/`Circle2d::new_rotated(0.05)`→15 段 `get_pos_at_u`+四元数法向（6.2831855）；逐顶点 `apply_roof_noise`（先减 roof+0x4c）→`set_triangles/set_uvs/set_normals`+tick；panic `Couldn't recalcualte roof volume`（0x142ff6ae8）。

**本轮处置**：几何算术（1580 行/12048B 的逐顶点 lerp 链、圆扇四元数法向、UV 常量、噪声尺度）**未达到逐语句判等** → 按硬标准标 `未实现`，现体内不伪造三角化产物；仅登记可判等骨架（事件消费、墙→屋顶表查找 + panic、`is_flat` 细分计数 `profile_subdivisions`）。分歧点：细分计数（已判等登记）、剖面采样/檐口/UV/噪声尺度（未重建）。

## §5 highlight_roof（0x141a284b0/0x141a31280）——重建

**签名（O）**：`(ResMut<EditingHighlight>, EventReader<HighlightRoofCmd>, ResMut<RoofHighlight>)`。

**数据流（O，0x141a31280 存储迭代权威）**：`RoofHighlight.tag = 0`（资源写前清 tag）→ 逐事件（0x18 步长：`wall u64@+8`、`color u8@+0x10`）：`RoofHighlight.value ← event+8`（wall u64 低 u32）、`RoofHighlight.tag ← event+0x10`（color 判别字节）、`HighlightColor::to_rgb()` → `EditingHighlight.color`（Vec3）。changed tick 逐事件写（Rust `&mut` 自动等价）。

**分歧（I 级）**：`*puVar6 = uVar7` 写 8 字节覆盖 `{value u32, pad 4B}`——pad 不可表示；0x141a284b0（iterator）与 0x141a31280（存储迭代）对 RoofHighlight 源字段读出不同，以存储迭代为准（0x141a31280 与发射方 ui_edit_roof 写 `tag=2@+0x10` 自洽）。`RoofHighlight.value = wall 低 u32` 的语义（墙 id 作高亮值）U，按解压数据流登记。

## §6 gpu_flush_buffers（0x140de50a0）——结构落位 + gpu_flush GAP

**签名（O，run_unsafe mangled 名）**：`(Res<GraphicsDeviceResource>, ResMut<AssetSsboLibrary>)`。
**控制流（O）**：`AssetSsboLibrary::get_mut<RoofTileSsbo>()` → None → `unwrap_failed` → `GraphicsDeviceResource::deref()` → `SparseInstanceBuffer<RoofTileSsbo>::gpu_flush` + tick（core 占位 tick 自动）。
**GAP（core-type，诚实）**：core `RoofTileSsbo`（`compute_passes::roof_tiles_gravity`）仍为 `TypedStorageBuffer` typed-gap 占位（非 `SparseSsbo`）→ `SparseInstanceBuffer::gpu_flush` 调用面无法落码；core 补 `RoofTileSsbo` 的 `SparseSsbo`（CHUNK_COUNT=0x2000，sparse_instance_buffer 文档）后本函数照此 5 行补 gpu_flush。

## §7 switch_roof_type（0x1421ac4f0）——部分重建（不变；mesh ops 渲染边界 GAP）

非 mesh 部分（RoofSound kind0/6 + `flat_surface=roof[0x54]`、ridge==1→WallGableChanged、`source_flag==1 && height!=1.6`→MAXSS(1.6,h)/MINSS(14,·)→update_wall_state_height、RecalculateWallBricks、`Roof::ty()`==target assert）已按机器码（上轮判等）。**mesh ops 未实现（渲染边界）**：`Commands::entity(roof).insert(NaniBundle<StoneFloorInstanceData>)`（0x88B 命令、kind 9、白 1,1,1,1）、`remove(DrawableMeshBundle)`、`format!("roof")`→`AssetMeshLibrary::try_get_handle_by_name`→命中→`TransientMeshRegistry::deferred_delete_mesh`、恒有 `remove(Handle<Vao>)`（0x18 命令）——`NaniBundle`/`DrawableMeshBundle`/`TransientMeshRegistry`/`Handle<Vao>` 组件均未在 core 提供。本轮未新增这些类型（core 渲染组在途）。

## 交付差异摘要

- **重建 + 写路径落库**：construct_timberframe_rectangle、construct_timberframe_circle（SSBO 推入 + 槽/互斥/clear + 计数写回）。
- **判等（修正）**：simulate_roof_tile_for_audio（angle' 三处：panic 判据、角度钳制、`angle<0` 累加器门控改用新角）。
- **重建**：highlight_roof（事件→RoofHighlight/EditingHighlight 数据流）。
- **结构落位 + GAP**：gpu_flush_buffers（RoofTileSsbo 非 SparseSsbo 阻塞 gpu_flush）。
- **未实现（诚实，细分计数登记）**：calculate_roof_trigger_volumes（几何算术未逐语句重建）。
- **不变**：switch_roof_type（mesh ops 渲染边界）、timberframe_beam_count_change_audio（判等主体）、add_timberframe（glue）。

## 证据索引

- 本仓 fresh：`work/wave3-timber-roof/evidence/batch1-decomp.json`（7 函数 signature+decompiled_code）、`batch2-utils.json`（0x140c90a40/0x140c99570）、`0x141a41ba0_*.c`/`0x1421b67a0_*.c`/`0x1421a7090_*.c`/`0x1421a3220_*.c`/`0x141a31280_*.c`/`0x140de50a0_*.c`/`0x1421ac4f0_*.c`。
- 身份：get_metadata 每批首步（tiny-glade.exe，B patched cf048e27…）。
- 上轮交叉：`reports/state/timberframe/{RESTORE,REVIEW}.md`、`reports/state/roof/{RESTORE,REVIEW}.md`、coregap-1 `roof_buffers.rs`/`device.rs`/`commands.rs` 落库合同。

## 转集成 owner（跨域交接）

1. **utils**：补 `rotation_from_normal`（0x140c90a40）、`Rectangle2d::orientation_mat3`（0x140c99570）→ timberframe 裙/屋脊梁朝向按解压逐语句换回（现 beam_rotation 等价旋转为 I 级）。
2. **country-core（render）**：补 `RoofTileSsbo` 的 `SparseSsbo`（CHUNK_COUNT=0x2000）→ gpu_flush_buffers 落 gpu_flush 5 行；`NaniBundle`/`DrawableMeshBundle`/`TransientMeshRegistry`/`Handle<Vao>` 组件 → switch_roof_type mesh ops。
3. **country-core（resources）**：`ConstructTimberframeCmd` 暴露事件首字节 bit0（rect 处理 bit0=1/circle bit0=0）与 `+0x1c`（现以 `shape[24]` 读取，契约化待宿主）；`PlayTimberframeSound` 补 size/id 字段 + `get_building_size_01`（audio size 写）。
4. **RenderInstance 记录**（`impl$1::from` 中间态字段序）：落库后梁实例/主环朝向按解压直写。

## 注册面说明

`highlight_roof`/`gpu_flush_buffers` 已实现但**未入 `add_roof_systems`**——reviewer-state-roof FIX-4 判原 0x14062fcb0 不注册未实现族为正确；两者真实注册面（render 调度 / UI 调度）待集成 owner 按原 `add_roof_systems` 之外的宿主插件接线（未证，登记）。
