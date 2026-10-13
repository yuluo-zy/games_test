# nani 实例数据契约 — instance_data.rs（6 procs + 23 类型实体）

标注：**O**=反编译/PDB 直接支持；**I**=推断；**U**=未恢复。源 SHA `222856`（gzip 文件元，PDB 模块同 gpu_world）。这些结构体是 `_nani_cull_and_bucket{...}` 与 `nani_uber` 光栅 shader 消费的 per-class GPU 实例布局（见 evidence/render-passes/INDEX.md §3）。

## 1. 类型清单与 GPU 元素尺寸（O）

尺寸证据：`impl$14::copy<T>`（gpu_world.rs 0x1415724f0/5b0/660/760）按类型复制元素尺寸；`update_instance_data<T>`/`create_instances<T>` 同尺寸。

| 类型 | 尺寸 | copy 入口 | 备注 |
|---|---|---|---|
| `UberInstanceData` | 0x40 = 64B | 0x1415724f0 | `register_required_components` 有专属 impl；`update_instance_data<Uber>` 0x14156db40 |
| `SolidVertexColorInstanceData` | 0x40 | 0x1415724f0 | |
| `SolidVertexColorBricksInstanceData` | 0x40 | 0x1415724f0 | |
| `SolidVertexColorWindowInstanceData` | 0x40 | 0x1415724f0 | |
| `SolidVertexColorWindowGlassInstanceData` | 0x40 | 0x1415724f0 | 关联 `WindowGlassFlags`（modular_bitfield） |
| `SolidAlbedoMapInstanceData` | 0x40 | 0x1415724f0 | |
| `SolidAlbedoMapEmissiveInstanceData` | 0x40 | 0x1415724f0 | |
| `LanternEmissiveInstanceData` | 0x40 | 0x1415724f0 | |
| `BalconyDoorInstanceData` | 0x40 | 0x1415724f0 | |
| `SmokeInstanceData` | 0x40 | 0x1415724f0 | |
| `StoneFloorInstanceData` | 0x40 | 0x1415724f0 | |
| `CurvedHalfTimberInstanceData` | 0x40 = 64B | 0x1415724f0 | 见 §2 构造器；关联 `CurvedHalfTimberCaps` |
| `ClothInstanceData` | 0x40 | 0x1415724f0 | |
| `PreplacedFlowerInstanceData` | 0x40 | 0x1415724f0 | |
| `PlantInstanceData` | 0x30 = 48B | 0x1415725b0 | `PlantInstanceData::marker` |
| `StairsFloorInstanceData` | 0x30 | 0x1415725b0 | |
| `PlasterInstanceData` | 0x30 | 0x1415725b0 | |
| `PlacementDataSsbo` | 0x30 | 0x1415725b0 | WP3 之外类型（prefab），仅记录尺寸 |
| `WrenInstanceData` | 0x70 = 112B | 0x141572660 | |
| `GothicWindowBricksInstanceData` | 0x70 | 0x141572660 | `update_instance_data<Gothic>` 0x14156ebd0 写 0x70 |
| `VelocityOnlyInstanceData` | 0x60 = 96B | 0x141572760 | `register_required_components` 有专属 impl |

bitfield/flag 类型（U 字段布局，仅名称 O）：`NaniUberInstanceFlags`（Uber 的位域，modular_bitfield 0.11.2 依赖证据）、`WindowGlassFlags`（窗玻璃位域）、`CurvedHalfTimberCaps`（弯曲半木端帽）。

## 2. `CurvedHalfTimberInstanceData` 构造器（O，64B）

### `new_curved_xy`（0x14098f690，行 343）
```
fn new_curved_xy(curved: &CurvedHalfTimber, mid: f32, half_len: f32, start: f32, end: f32, color/count: u32) -> Self
```
1. `fVar5 = mid + half_len * 0.5`（中心+半长/2 = 端点参数）；`fVar4 = ceil((ABS(end - start) * fVar5 + 1.0) / 0.6)` → 细分段数。
2. `count = next_power_of_two(fVar4)`（u32，clz 循环；`core::num::int_log10` 非正 panic 分支）。
3. 写入 64B 结构：
   - +0x00..+0x0b：复制来源前 12B（color_id / atlas 引用）
   - +0x0c u32=0；+0x10 f32=`start - half_len*0.5`；+0x14 f32=`fVar5`
   - +0x18 f32=`start`；+0x1c f32=`end`
   - +0x20 u64=0；+0x28 u32=`in_stack_00000028`（细分 count / 色彩）；+0x2c u32=0
   - +0x30 u32=`count`（power-of-two 段数）；+0x34 u64=0；+0x3c u32=0

### `new_curved_xz`（0x14098f800，行 375）
对称构造（xz 平面弯曲），363B，结构同 64B 布局（O）。

### `new_straight`（0x14098f970，行 407）
```
+0x00..0x0b 复制；+0x0c=0
+0x10 f32=start；+0x14 f32=end；+0x18 f32=h_start；+0x1c f32=h_end
+0x20 i32 = caps_count*2 + (has_cap ? 2 : 1)   // 端帽数（in_stack_00000040=帽数, in_stack_00000038=布尔）
+0x24 u64 = 0x3dc28f5c00000001   // {u32=1, u32=0x3dc28f5c(f32≈0.0951)}
+0x2c u64 = 0x100000000          // {u32=0, u32=1}
+0x34 u64=0；+0x3c u32=0
```
数值 `0x3dc28f5c`/`0x100000000` 的语义 **U**（推断为打包的 mesh atlas part / sort 相关字段）。

## 3. 其他函数

- `impl$122::register_required_components`（0x140990a80，行 284）：为 instance_data 各类型注册 bevy component（`InstanceDataMarker<T>`）。
- `impl$122::clone_behavior`（0x140990af0，13B）：Component::clone_behavior 薄封装。
- `PlantInstanceData::marker`（0x14003bb80，1B，与多个 `*_marker` 共址）：`pub const fn marker()` 常量（O，符号证据）。

## 4. 字段级布局

除 `CurvedHalfTimberInstanceData` 构造器外，各类型内部字段名/语义 **U**（PDB 无 Rust struct field 记录；`name_and_relations_only`）。GPU 侧解释按 shader（cull_and_bucket 各变体）与元素尺寸对齐；字段精确顺序需后续 shader 反射或运行时回读确认（typed gap）。

## 5. 消费者接线（O）

- `systems::nani_update_<T>`：从 ECS 组件（Transform/ColorId/材质等）计算 `T`，调 `OwnedGpuWorldSubset::update_instance_data<T>`。
- `gpu_generate_draw_lists`（0x140987dd0）按 kind 分派 `_nani_cull_and_bucket{,_bush,_curved_halftimber,_flowers,_plaster,_roof_tile,_smoke,_stairs_floor,_wooden_plank}`（见 INDEX.md §3 与 pass-declaration.md §2.1）。
