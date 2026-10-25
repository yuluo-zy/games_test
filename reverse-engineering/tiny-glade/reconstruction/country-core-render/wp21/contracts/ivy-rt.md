# 合同：resources::ivy + resources::rt 余量（ivy_storage / ivy_leaf_storage / ivy_direction_proposer / wall_coord + rt bvh / bvh_builder / mod）

版本 B。本包把 WP16 合并的 `resources/{ivy,rt}.rs` **扩展到骨架子路径粒度**；wp16 已恢复
的 O 逻辑重述于此并补新证据。

## ivy_storage.rs / ivy_direction_proposer.rs / wall_coord.rs（扩展；wp16 已覆盖）

- **`Ivy`**（40B 点，VecDeque 折线）+ `add_point` 0x140869460、`last_point` 0x1408694f0、
  `last_segment_point_ids` 0x140869540、`remove_last_point`/`remove_root`/
  `set_last_point_position`/`point_count`/`get_point_id`（grower/pruner/leaf 内联，O）。
- **`IvyStorage`**：`HashMap<usize, Ivy>` + `next_ivy_id` + PerchanceContext RNG；
  `spawn_ivy` 0x1408696f0、`iter/iter_mut/roots/get/remove/total_segment_count`（O）。
  复用边界：perchance 0.4.0（RNG）。
- **`IvyDirectionProposer`**：两个 FastNoise + 常量 1.0/2.0/1.3；`get_direction`
  0x141234590（θ 混合 → (cosθ,sinθ)）。**`IvySpawnNoise::new`**（O 名）。复用：
  bracket-noise 0.8.2（typed gap，缓存无）。
- **`WallCoord`**：`from_wall_u` + `into_world_space` 0x140b199e0（沿 PublicWallState 曲线
  采样，法向偏移 0.63*0.5，按侧±）。`PublicWallState::max_ivy_grow_height` 内联引用。

## ivy_leaf_storage.rs（新增：IvyLeaf 104B 布局 O）

- **`IvyLeafStorage`** = `HashMap<usize, Vec<IvyLeaf>>`（键=ivy id）。
- **`IvyLeaf`** = 0x68 = 104 字节（`add` 0x1408fe280 写全字段）：
  `{ +0x0: u64, +0x8: u64, +0x10: u64(ivy id/键), +0x18: u64, +0x20: Vec3(位置),
    +0x2c: Vec2?, +0x38: u64, +0x40: f32, +0x44: f32, +0x48: u64=0, +0x50: f32,
    +0x54: f32, +0x58: u64, +0x60: f32 }`（GPU 叶实例参数；字段名 I/U）。
- **`IvyLeafRng`**：叶 RNG（布局 U）。
- 方法：`add`（0x8fe280，push 104B 叶到对应 ivy）、`all_leaves(_mut)`、`new`、
  `delete_all_leaves`、`delete_affected_leaves`（按受影响 ivy id 删，O 名）。
- 消费：ivy_leaf_spawner / ivy_send_to_gpu_leaf。

## rt/mod.rs + rt/bvh.rs + rt/bvh_builder.rs（扩展；wp16 已覆盖主体）

- **`RtWorld`**：`HashMap<RtMeshHandle, DirtyCow<RtMesh>>` + 移除/脏计数簿记。
  `get_mut<RtMeshHandle>`（0x141562aa0 / 0x1400cce50）：**缺省建默认 RtMesh**
  （ArcInner 0x60B：strong/weak=1，矩阵单位、`+0x38 Vec2{1,1}`、`vertices Vec` cap 4），
  DirtyCow::get_mut 置脏并按 Arc 强计数克隆；`get`（0x141562cf0）同查不置脏。
  `update_rt_world` 0x1408bc650、`upload_rt_world` 0x1408bcff0、`RtBvhBuilderGlueResource::
  new/is_idle`、`generate_handle/reset/remove/is_dirty/consume`（wp16，O）。
- **`DirtyCow<T>`**：`{ arc: Arc<T>, dirty: bool @+0x8 }`；`get_mut` make_mut 语义（0x15626f0）。
- **`GpuBlBvh`**：BVH 节点 SSBO 记录（80B/项；wp16）。
- **`build_gpu_bvh`**（0x140b0da50，bvh.rs）：输入三角形（3×16B 顶点行 [x,y,z,z]）→
  `obvhs::cwbvh::build_cwbvh_from_tris`（复用库，typed gap）→ 叶序三角形 + u32 索引 →
  `GpuBlBvh` + 三角形重排。
- **`RtBvhBuilderGlue`**（bvh_builder.rs）：三角形列表/元/BVH 输出 + busy/idle 标志；
  `bvh_builder_worker`（0x140b24d50）：后台线程状态机收集脏网格 → build_gpu_bvh → 写回。
  **`InputTriangle`** 40B（3 顶点+法向），**`RtBvhBuilderOutput`**。

## 输入 / 输出 / 未知

- ivy：叶系统生产 IvyLeaf 104B → GPU；rt：墙/装饰/楼梯构造写 RtMesh，worker 重建 BVH →
  3 个 RT SSBO（GI/水 pass）。
- 未知（U）：IvyLeaf 字段名/语义；IvyLeafRng；RtMesh 单位矩阵字段语义（矩阵/尺寸分块）；
  glue 内部 Vec 槽位；obvhs/bracket-noise 库体。

## 复用边界

- std `HashMap`/`VecDeque`/`Arc`（RandomState=foldhash）。
- perchance 0.4.0（缓存有）、bracket-noise 0.8.2、obvhs（cwbvh）、frozen/nohash-hasher →
  复用依赖（缓存无者 typed gap）。
