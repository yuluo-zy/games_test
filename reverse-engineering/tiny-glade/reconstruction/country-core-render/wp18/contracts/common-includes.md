# WP18 契约 — common_includes.rs

版本 **B**。数据源：skeleton bundle + Ghidra 只读反编译（b3）+ wp7 water 证据。
标注：**O**=反编译直接支持；**I**=组合；**U**=未恢复。

## 类型（O）

| 项 | 值 |
|---|---|
| 编译单元 | 495/497/498/502/514/521/528/529/530/531/532/536/538/540/542/545/547/550/551/552/589/1615/1632/1638（多 pass 共用） |
| `impl$0::bind_terrain_heightmap` | 行 13；无独立过程（内联进各 pass） |
| `impl$1::bind_terrain_lighting` | 行 30；内联 |
| `impl$2::bind_water_object_push<ComputePass>` | 0x14160e330，size 584，行 47 |
| `impl$2::bind_water_object_push<DrawBuilder>` | 0x141631f90，size 224，行 47 |
| `impl$3::bind_garden_exclusion<ComputePass>` | 0x14160e630，size 344，行 65 |

## 绑定逻辑（O）

- **bind_water_object_push<DrawBuilder>**（0x141631f90）：`TexturePreset(0xf)` →
  `read_storage_buffer<RippleSourceSsbo>` → `TexturePreset(0x18)`。= wp7
  `water_planes_draw.rs` 引用的 `common_includes::bind_water_object_push`
  （水植被 push 常量，`water_object_push.inc.hlsl` 嵌入 lilypad/reed raster，O）。
- **bind_water_object_push<ComputePass>**（0x14160e330）：同序（compute 侧
  `bind_texture_to_compute_pass` + `read_access_buffer<RippleSourceSsbo>`）+
  `assert_failed<SetIndex>`（set==0）+ `grow_one<Range<ResourceAccessIndex>>`。
- **bind_garden_exclusion<ComputePass>**（0x14160e630）：`TexturePreset(0xe)` →
  `0x13` → `5` → `0x10`（garden 排斥 mask 四槽，I：foliage/water/path 排斥）。
- **bind_terrain_heightmap / bind_terrain_lighting**（内联）：deps `read_texture`
  （terrain heightmap / lighting 纹理读；grass/水/阴影等大量 pass 复用，O）。

## Pass 组合 / shader URI（I）

无独立 URI（helper 层）。`water_object_push.inc.hlsl` 嵌入水植被 raster；
terrain heightmap/lighting 纹理被 `_grass*`/`_water_*`/`_rocky_terrain*` 读取。

## typed gap（U）

- 泛型 T 的精确约束（ComputePass/DrawBuilder 双实例 O，约束体 U）。
- preset 槽 `0xf`/`0x18`/`0xe`/`0x13`/`5`/`0x10` 的语义名。
- 本束泛型签名 `fn bind_*<T>(pass: T) -> T`（按值返回）为 I；集成时可按
  `&mut T` 或 consume 语义调整。

## 交付

`src/.../draw_shaders/common_includes.rs`（4 个 `bind_*<T>` 泛型 helper，
`unimplemented!()` 骨架）。
