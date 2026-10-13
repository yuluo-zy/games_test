# 合同：resources::ivy（常春藤生长状态族）

源 SHA256（B skeleton）：`resources/ivy/{ivy_storage,ivy_leaf_storage,ivy_direction_proposer,wall_coord,mod}.rs`
PDB 实体：`Ivy` / `IvyPoint` / `IvyStorage` / `IvyLeaf` / `IvyLeafRng` / `IvyLeafStorage` / `IvyDirectionProposer` / `IvySpawnNoise` / `WallCoord`
Rust：`src/resources/ivy.rs`

标注：**O**=反编译/PDB 直接支持；**I**=推断；**U**=未恢复（typed gap）。

## 已观察（O）

- **`IvyPoint` = 40 字节**（`Ivy::add_point` 0x140869460 写 stride 0x28）：
  `{ point_id: u64 @0x0, position: Vec3 @0x8, direction: Vec3 @0x14, reserved: u32 @0x20=0 }`。
- **`Ivy`**（B 构建）`points: VecDeque<IvyPoint>`（cap@0x0 / data-ptr@0x8 / head@0x10 /
  len@0x18）+ `next_point_id: u64 @0x58`。`add_point` push_back 新点（id=`next+1`）；
  `last_point`（0x1408694f0）空表 unwrap 失败；`last_segment_point_ids`
  （0x140869540）返回末两段端点 id（len<2 expect 失败，段回环用 id 相等判定）；
  `remove_last_point` / `remove_root` / `set_last_point_position` / `point_count` /
  `get_point_id` 均为 grower/pruner/leaf/draw 系统内联。
- **`IvyStorage`**（`spawn_ivy` 0x1408696f0）：`HashMap<usize, Ivy>`（hashbrown，
  `insert<usize,Ivy>`）+ `next_ivy_id`（+0x40 自增）+ `PerchanceContext` RNG
  （`usize_less_than` + `uniform_f32`）。`new` 内联于 `insert_ivy_resources`
  （0x1409440c0）。`iter/iter_mut/iter/roots/get/remove` 由
  grower/send_to_gpu_segments/spawner/pruner/leaf_spawner 内联。
- **`IvyDirectionProposer`**（`impl$0::default` 0x1412343b0）：两个
  `bracket_noise::FastNoise`（seeded + `set_fractal_octaves/gain/seed`）+ 常量
  `1.0`(0x3f800000) / `2.0`(0x40000000) / `1.3`(0x3fa66666)。
  `get_direction`（0x141234590）：3×`get_noise3d` + 3×`uniform_f32` 混合 →
  `θ = ((v*80 − (1−v)*80) * DEG2RAD * 0.5)`（v 归一到 [0,1]）→ 返回 `(cos θ, sin θ)`。
- **`WallCoord`**：`into_world_space`（0x140b199e0）读 `PublicWallState` 曲线总长
  （+0x30）、曲线点表（+8 ptr / +0x10 len，每点 Vec2），`u/总长` 采样
  `Curve::get_coord_at_u` 得墙点，沿墙面法线偏移 `0.63 * 0.5`（O 常量），按侧
  （bool）取 ±。
- **`IvyLeafStorage`**（`add` 0x1408fe280 / `all_leaves(_mut)` / `delete_all_leaves` /
  `delete_affected_leaves`）：叶集合；`IvyLeaf`/`IvyLeafRng` 布局未恢复（U）。

## 输入 / 输出 / 未知

- 输入：`IvyStorage` / `IvyLeafStorage` / `IvyDirectionProposer` 均为 ECS 资源；
  spawn/grow 由 ivy 系统（见 misc-systems 合同 §ivy）驱动。
- 输出：折线点列供 GPU 段/叶上传；`as_float` 采样由绘制消费。
- 未知（U）：`Ivy` 0x20..0x58 区间字段；`IvyLeaf`/`IvyLeafRng` 布局；
  `get_direction` 噪声采样坐标映射；`IvySpawnNoise` 具体参数。

## 复用边界

- `bracket_noise::FastNoise`（bracket-noise 0.8.2，registry）→ `reused::noise`。
- `perchance::PerchanceContext`（perchance 0.4.0，registry）→ `reused::rng`。
- `VecDeque` 内部布局为 std 实现细节，U（用 std `VecDeque` 承载）。
