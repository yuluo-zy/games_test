# WP9 屋顶资源合同 — roof_tiles（屋瓦实例资源）

Target: B。PDB 骨架：`skeleton/B/crates/country-core/src/resources/roofs/roof_tiles.rs`、
`roof_tile_sim.rs`；消费者 `systems/render_loop/compute_passes/roof_tiles_gravity.rs`。
标的：`resources::roofs::roof_tiles::RoofTiles` —— 被 `roof_tiles_gravity_compute` 读取的
每屋顶屋瓦实例资源（64 字节 RoofShingle 记录）。标注 O/I/U 同 roof-shape-sim.md。

## 1. RoofTiles（O，Component）

- 源文件 **8 行**（line_blocks=2，注册/克隆方法在 line 8），极小型组件。
- **类型**：ECS **组件**（非 Resource）。PDB 证据：`QueryState<(&RoofTiles, &RoofTileSimulation,
  &Roof, &RoofAnimation)>` 迭代类型；`register_required_components`（0x140903f40，
  `#[derive(Component)]` 生成）+ `clone_behavior`（0x140002370，Component derive 的 Clone 方法，
  全游戏 183 处复用）。
- **内容**（I）：在 `roof_tiles_gravity_compute` 的 `from_iter` 闭包中，每个 query 项
  flat_map 一个 `Option<SparseInstanceBufferAllocation<RoofTileSsbo>>`，再对该 allocation
  的 `IntoIter<InstancedDraw>` 逐条 map 生成 `PushConstants`。
  → RoofTiles 持有该屋顶在共享 `RoofTileSsbo` 稀疏缓冲中的**分配句柄**
  `SparseInstanceBufferAllocation<RoofTileSsbo>`（单分配；行 stride 64B）。
  8 行源码形态 I：`#[derive(Component, Clone, Default)] pub struct RoofTiles(...)` 或
  `Option<...>` 包装；`#[require(...)]` 具体成员 U（create_roof 已显式 spawn 全套）。
- `create_roof`（0x1421aaea0）spawn bundle 确认：屋顶实体组件集合 =
  `(Roof, RoofTileSimulation, RoofTriggerVolume, RoofAnimation, RoofTiles,
   RoofSpire, RoofSupportBeams, NaniBundle<StoneFloorInstanceData>)`。O。
- 生产者：`system_roof::visual::assemble_roof_tiles::assemble_roof_tiles`（0x141a36700）
  → `assemble_circular_roof`（0x1421b3120）/`assemble_rectangular_roof`（0x1421aeef0）
  → 记录 closure（0x1421b2ca0/0x1421b4510）→ `SparseInstanceBufferWriter::push<RoofTileSsbo>`
  （0x141a3c070，读 64B 记录写 SSBO）。**prior work `reconstruction/roof/tiles.rs` 已恢复**
  圆形/矩形铺瓦数值核心（见 evidence/roof-tiles/findings.md，机器码验证）。

## 2. RoofShingle / RoofTileSsbo（O，64B 实例记录）

GPU 结构名（shader `roof/roof_tile.inc.hlsl` + SPIR-V OpMemberName）与 CPU 记录逐字节一致：

| 偏移 | 类型 | shader 名 |
|---:|---|---|
| 0 | f32×3 | base_position |
| 12 | f32 | current_height |
| 16 | u32×2 | clipping_plane__ |
| 24 | u32×2 | scale_t（half2 打包：尺寸×、t） |
| 32 | u32×2 | rotation_quat__（half4） |
| 40 | u32 | roof_id |
| 44 | u32 | velocity_rotation（half2x16） |
| 48 | f32 | animation_t |
| 52 | i32 | seed |
| 56 | i32 | is_circle |
| 60 | i32 | is_main_roof_body |

- CPU 记录打包规则：`reconstruction/roof/tiles.rs::record()`（half=half2.4.1 `f16::from_f32`，
  quat=glam `Quat::from_mat3`，随机=fastrand 1.8.0）——机器码验证（圆形 11467 条记录零差异）。
- RoofTileSsbo = RoofShingle（64B）；`RoofSpireSsbo`（spire 顶点，布局 U）；`RoofSupportBeamSsbo`
  在 roof_buffers.rs（布局 U）；`WoodenPlankSsbo`（U）。

## 3. roof_tiles_gravity_compute（O，消费系统，W19）

`systems::render_loop::compute_passes::roof_tiles_gravity`，0x140a29b10，rva 0xA29B10。
- 结构（O，本次反编译 + wp5 compute-passes.md W19 合同）：
  1. World/资源解析（`Components::get_resource_id`、`AssetSsboLibrary::get` 取
     `RoofTileSsbo` BufferState，名称断言长度 0x35=53 字符串；`BufferState::get`）。
  2. `QueryState::new::<(&RoofTiles, &RoofTileSimulation, &Roof, &RoofAnimation)>`。
  3. 每 query 项：`RoofTiles` 的 tile 分配 → 逐 `InstancedDraw` → **`PushConstants`**
     （见下），`Vec::from_iter(flat_map(...))`。
  4. `ComputePass::new` + `read_storage_buffer<RoofTileSsbo>` + `uniform<f32>`
     （closure$0 写 wall_height_delta）+ `dispatch_with(closure$1)`（dispatch_1d，
     组数=PushConstants 数）。
  5. 失败分支：资源缺→`unwrap_failed`；query world 校验失败→`panic_mismatched`。
- **RoofPushConstants**（shader `roof/roof_tile.inc.hlsl`，20B）：
  `{u32 first_instance, u32 instance_count, f32 wall_height_delta, f32 roof_pivot_y,
   f32 animation_t}`。
  - 生产者闭包（I）：per-InstancedDraw：first_instance/instance_count 来自 InstancedDraw，
    `wall_height_delta` 来自 `RoofTileSimulation`（off4 候选），`roof_pivot_y` 来自
    `Roof::pivot_3d().y`（base_y），`animation_t` 来自 `RoofAnimation`。
  - `write_uniform<f32>` 的 f32 = wall_height_delta（O：closure_env_0<f32>）。
- **GPU 侧**（shader `roof_tiles_gravity.cs.hlsl`，已恢复）：每 thread 更新一条 RoofShingle：
  `velocity += (-98.1·lerp(1,0.4,t))·dt`；`new_height=pivot_y+(below_roof?0:h)+v·dt`；
  `t=floor(half(scale_t.y高位)·50)·0.02`；settle ease `1-exp(-dt·5·lerp(0,2,1-t⁴)·(rand01(hash(seed))·t>0.3?1:1000))`；
  速度 clamp max 15；`rot=(1-exp(-δ³))·-1.57`；打包 `velocity_rotation`；`animation_t=push`。
- **RoofTileSimulation off4 语义**（I）：gravity 统一的 `wall_height_delta`（负=墙在长高，
  shader 用它判 wall_clip）。off0 候选初始位移。**typed gap**（字段名 U）。

## 4. instanced_roof 族 draw 消费（O，`draw_passes/draw_shaders/instanced_roof.rs`）

| 函数 | VA | 绑定/读 | SSBO |
|---|---|---|---|
| `draw_instanced_roof_shader_nani` | 0x1409d9070 | 2×TexturePreset + 4×TextureFromDisk + 2×uniform<i32> | DormerHoleSsbo, DormerHoleCountPerCellSsbo, RoofColorIdSsbo, **RoofTileSsbo** |
| `draw_instanced_roof_spire_shader` | 0x1409d96a0 | draw_instanced_w_gl_draw_flag | **RoofSpireSsbo** |
| `draw_instanced_roof_support_beam_shader` | 0x1409d98a0 | 2×TexturePreset + draw_instanced_w_gl_draw_flag | **RoofSupportBeamSsbo** |

- shader 映射：`nani_instanced_roof.raster.hlsl`、`instanced_roof_spire.raster.hlsl`、
  `instanced_roof_beams.raster.hlsl`（`reconstruction/shader-source/shaders/`）。
- 纹理库按名 hash（0x9311cf2688353878 资源 + 4 个 TextureFromDisk id），id→路径 U。

## 关键数值/边界裁定（机器码校验）

- 64B 记录布局 + half 打包 + 随机相位/分区（evidence/roof-tiles 圆形/矩形全记录零差异）。
- gravity 常量：`-98.1`（重力）、`15.0`（max fall）、`-1.57`（落地转）、`-13.3`（audio ease）、
  `-45.126003`（audio gravity）、`0.166`（wall_height_delta 速度换算）、`0.3`（noise 幅度）。

## 复用裁定（prior work）

- `evidence/roof-tiles/*`（surface/tiles 数值核心、findings.md）→ RoofShingle 生产者，**复用**。
- wp5 W19 合同 → gravity compute 结构，**复用**（本次反编译逐点确认）。
- `reconstruction/roof/tiles.rs` → RoofTiles 生产核心，**复用**。

## typed gaps（U）

RoofTiles 具体字段形态与 `#[require]` 成员、RoofSpireSsbo/RoofSupportBeamSsbo/WoodenPlankSsbo
布局、InstancedDraw→PushConstants 的精确映射（field 名）、纹理 id→路径、gravity dispatch 的
instance_count 组数公式。
