# 合同：resources::walls 余量（arches / dormers / wall_holes / wall_stone_columns / wooden_gates / decorator_collision / decorator_outline / decorator_subtype / elevation_support_stats）

版本 B（EXE cf048e27…，PDB 2d9686b8…）。源 SHA256 见各模块。方法：业务逻辑分析（PDB
enriched/structures + Ghidra 只读反编译 + 调用链）→ 整理（O/I/U）→ 骨架对齐 Rust。
机器码差分仅作裁决，不构成实现手段。

标注：**O**=反编译/PDB 直接支持；**I**=推断；**U**=未恢复（typed gap）。

## arches.rs（O：仅标记类型）

PDB 实体：`ArchSegment` / `ArchSegments` / `BuildingHadArchesLastFrame` / `LintelSegment` /
`LintelSegments`。无独立编译函数（`impl$3/5/7::default` 全内联）→ 纯标记资源。
业务上下文：`system_wall_constructor::construct_arches` 产出 96B `WallBrickInstance`
拱砖 + `ConstructGateCmd`（证据 `evidence/building-arches/findings.md`）；本模块是被该
系统写回的**跨帧拱段状态**。布局 U。

## dormers.rs（O：DormerHoleSsbo 48B 构造器）

- **`DormerHoleSsbo`** = 48 字节 GPU 记录。
  - `dummy`（0x141286660）：清零 48B。
  - `gothic`（0x141286680）：`{ +0x0: Vec3, +0xc: f32(in_EDX), +0x10: Vec3, +0x1c: f32=0,
    +0x20..0x30: 4×f32 }` —— 两个 Vec3 点位 + 尺寸/偏移。
  - `cottage`（0x1412866e0）：同 gothic 但 `+0x28=1.0`、`+0x2c=−50.0`
    （0xc24800003f800000）。
- **`DormerHoleCountPerCellSsbo`**：计数 SSBO（布局 U）。
- 消费方：渲染 pass 写 dormer 墙洞（GPU 网格化）。语义：每个记录是一个 dormer 洞的
  中心/尺寸/几何常数（字段名 U，按偏移 O）。

## wall_holes.rs（O：完整存储族；复用 building-holes 先验）

PDB 实体：`WallHoles` / `WallHoleStorage` / `InProgressHoleStorage` / `FinalizedWallHoles` /
`FinalizedHoleStorage` / `PrevWallHoles` / `HoleChecksum` / `HoleOrigin` / `HoleType`。

- **`WallHoles`**：ECS 资源 `HashMap<WallId, WallHoleStorage>`（0xf86500 `add` /
  0xf867c0 `clear_for_next_frame` / 0x12ba9f0 `extend`，entry stride 0x68）。
- **`WallHoleStorage`** = 0x68 字节：`phase`（+0x0，0=WIP/1=Finalized）、`raw: Vec<RawWallHole>`
  （ptr/cap/len @+0x8/+0x10/+0x18）、`finalized: Vec<FinalizedWallHole>`
  （@+0x20/+0x28/+0x30）、fingerprint sum/xor（@+0x38/+0x40）、previous sum/xor
  （@+0x48/+0x50）、`wall_length: f32`（+0x58）、`closed: bool`（+0x5c）。
  - `RawWallHole` = 20B：`{ center: Vec2, dimensions: Vec2, kind: u8 }`（`InProgressHoleStorage::add`
    0xf87860 按 0x14 步长 push；指纹 `utils::calculate_hash<(array<u32,2>,array<u32,2>,
    HoleType)>` 累加 `sum += h, xor ^= h` @+0x38/+0x40）。
  - `FinalizedWallHole` = 20B：`{ aabb: Aabb2(16B), kind: u8, origin: u8 }`；`finalize`
    （0xf87300）：raw→Aabb2，`sort_by(maxX)`（<0x15 insertion，否则 driftsort），closed 时对
    跨 seam 洞写 ±wall_length 平移副本、origin=0/1。`finalize` 不推进 previous 指纹；
    `advance_frame` 才推进（building-holes 证据）。
  - `as_finalized`/`as_wip_mut` 断言 phase（panic on wrong）。
  - `clone_and_reopen`（0xf86f50）：深拷贝 raw/finalized，phase 复位 0。
- **`PrevWallHoles`**：`HashMap<WallId, FinalizedWallHoles>`（0xf86d00 查含）。
- **`FinalizedWallHoles`**：`{ holes: Vec<FinalizedWallHole>, wall_length: f32, closed: bool }`
  （holes_no_duplicates/wall_length getter；`iter_holes_with_padding<closure>`
  0x123fc70 供 wall_constructor normalize_holes 按比例缩放孔并补 padding 副本）。
- **`HoleType`** / **`HoleOrigin`** / **`HoleChecksum`**：枚举/哈希类型
  （HoleType=u8 判别；HoleOrigin=0 主/1 平移副本；HoleChecksum{sum,xor}，O）。

先验复用：`reconstruction/building/wall_holes.rs`（building-holes 已恢复生产/存储/finalize
整链，B 768 例原码验证）；本模块为 country-core 资源级骨架对齐。

## wall_stone_columns.rs（O：clear_and_get_mut 0xa78880）

- **`StoneColumnCollisionStorage`**：`{ cleared: Vec<WallId>（本帧已清）, map: HashMap<WallId,
  Vec<StoneColumn>> }`。`clear_and_get_mut(wall_id)`：把 wall_id 记入 cleared，然后
  `map.entry(id).or_default().clear()` 并返回 `&mut Vec<StoneColumn>`（插入默认 cap=4）。
- **`StoneColumn`**：单根石柱碰撞数据（布局 U）。**`StoneColumnsChanged`**：变更事件（U）。

## wooden_gates.rs（O：Gates::clear 0xaaae80）

- **`Gates`**：`HashMap<WallId, Vec<u64>>`（entry stride 0x20；值元素 8B=实体引用）。
  由 `system_wall_constructor::construct_gates` 维护（证据 building-gates）。`clear`：
  遍历释放各 Vec 缓冲并清空 map（control 字节置 0xff、len=0、cap 重算）。
- 与 `components::drawable` 无关；`WoodenGate` 组件 spawn 载荷 136B 见
  building-gates findings（本模块只管 `Gates` 资源）。

## decorator_collision.rs（O：标记资源）

- **`DecoratorPhysicsColliders`**：装饰物理碰撞体集合（rapier/parry 碰撞世界边界）。
  无独立编译函数。布局 U。

## decorator_outline.rs（O：display 0xb33920 / 0xb33b00）

- **`DecoratorCollisionOutline`**：判别枚举 `{ Lines(Vec<Vec3>), Sphere }`；`display`
  （0xb33920）把顶点行按 3×3 矩阵+平移变换（stride 0xc）后经
  `DebugLine::show` / `DebugSphere::show` 绘制（debug_viz，U 消费方）。
- **`DecoratorOutline`**：带位置/尺寸的调试描边枚举；`display`（0xb33b00）读
  `+0x20/+0x24/+0x28`（尺寸）、`+0x30/+0x34/+0x38`（位置），flag 时 `位置 += 尺寸*0.18`，
  格式化 Debug 串后按判别 jumptable 显示。

## decorator_subtype.rs（O：库面 + 方法面）

PDB 实体：`DecoratorSubtype` / `DerivedDecoratorSubtypes` / `DerivedDecoratorInfo` /
`DerivedDecoratorInfoResources` / `DecoratorMeshLibrary` / `DecoratorCollision` /
`DecoratorLibKey`。

- **`DecoratorLibKey`**：复合键（首字节 subtype 判别，`'\x1a'`(26)=空；次字节 variant）。
- **`DecoratorMeshLibrary`**：4 个 `HashMap<DecoratorLibKey, …>`（collision /
  interaction / outline / collision_outline）。`insert`/`insert_collision_and_interaction_as_same`
  （0xb262a0 / 0xb25ef0）：克隆 `Vec<Arc<RaycastMeshData>>`（0x24B 元素）后依次插入 4 表。
  `get_outline`（0xb26690，map3 @+0x60..）/`get_collision_outline`（0xb267f0，map4
  @+0x90..）按键查（首字节≠0x1a 且匹配）；未找到 panic。
- **`DecoratorCollision`**：碰撞数据载体（`Arc<RaycastMeshData>` 列表 + parry3d
  `SharedShape`/`IsoTransform`；`empty` 0xb25ca0 建空碰撞）。parry3d=0.17.0 复用边界。
- **`DerivedDecoratorSubtypes`**：`HashMap<u32, Arc<DerivedDecoratorInfo>>`（0xb25b60
  `get` 未命中 panic）。**`DerivedDecoratorInfo`**：`{ subtype, collision_mesh /
  interaction_mesh / bbx_ls / dims_wall_space … }`（0x1fdc7b0 `bbx_ls<DecoratorSubtype>`
  按 DecoratorLibKey 查 collision 得 Aabb）。**`DecoratorSubtype::can_be_on_corner`**
  （0xb25b50）：字节标志。
- 业务上下文：`system_decorator` 把墙窗/门装饰按 subtype 索引 mesh 库；`derive_wall_subtype`
  （reconstruction/building/decorator_subtype.rs）是系统侧，本模块是其资源容器。

## elevation_support_stats.rs（O：标记资源）

- **`ElevationSupportStats`** / **`ElevationSupports`**：升降支撑统计（无独立编译函数）。
  语义：墙高调整的升降支撑计数/统计（U）。音频系统 `elevation_supports_audio` 消费（共享 crate）。

## 输入 / 输出 / 未知汇总

- 输入：wall_constructor / decorator_visual / construct_gates 等系统写回；`OnWallChanged`
  → recalc（building-holes）。
- 输出：dormer 孔 SSBO、墙洞 finalize 几何、装饰 mesh/outline 库、石柱碰撞。
- 未知（U）：各 struct 内部未列出字段名；`StoneColumn` 布局；`DecoratorPhysicsColliders`
  容器；`DecoratorCollision` 0x90B 详布局；arches 标记类型布局；DormerHoleSsbo 字段语义名。

## 复用边界

- hashbrown/`RandomState`（std）容器 → std `HashMap`（Rust 1.86 RandomState=foldhash）。
- parry3d 0.17.0（SharedShape/cuboid）、rapier（碰撞世界）→ 被复用依赖，typed gap。
- `utils::geometry::aabb::Aabb2`、`Rectangle3d`、`calculate_hash` 游戏自有 utils → 骨架内声明。
