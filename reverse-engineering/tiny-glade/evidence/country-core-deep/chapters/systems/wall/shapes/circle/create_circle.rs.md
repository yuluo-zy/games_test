# systems/wall/shapes/circle/create_circle.rs

> 文件作用：圆形墙创建：CreateCircleCmd 到 Circle2d::as_curve2 到 crashy_new_from_points、enclosure=circle、style/elevation、RoofShapeParams::from_wall_shape + roof_height_01_from_wall_shape 到 CreateRoofCmd、历史。

函数总计 3：业务 1 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::circle::create_circle::create_circle` | 0x14097e560 | 38 | create_circle（曲线+enclosure+roof+历史） | O | wall\decomp\decomp-0x14097e560.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 2 项

## 实现方式（算法级）

**create_circle（O，0x14097e560）**：15 参系统；9 个 ResMut（param_2/3/4/5/6/7/8/9/15 +0x1c tick 回写）。遍历 `EventReader<CreateCircleCmd>`（48B/行：{cmd u64×2, WallId u64×2, u32@+0x1c, u32@+0x24, …}）。每行处理链：

1. **写待建墙命令**（0x30B/条 push）：`{tag u32@+0=0(圆), ShapeParameters 载荷 24B@+4（center qword@+4、+0xc…），f32@+0x20=4.0(海拔), u8@+0x24=1, u64@+0x28}`（Vec 扩容 FUN_1405629f0）。
2. **建曲线**：`FUN_140c97bc0`（事件→Circle2d 参数）→ `FUN_141304650(&lStack_318, &uStack_1c8, &UNK_142ab66b8)`（Circle2d::as_curve2，内部 `.collect().unwrap()`，panic 路径 = `/rustc/...library/core/src/iter/traits/iterator.rs` @0x142ab66b8）→ `FUN_141469430(&uStack_1c8, &lStack_318, 0)`（2D→3D 投影）。`.unwrap()` 失败 → **fmt panic** `FUN_1428d9760(&UNK_142c63340, 0x2b, ..., &UNK_142ab6ea0)`（"called `Result::unwrap()` on an `Err` value" + `/home/h3/.../crates/utils/src/curve/mod.rs` + "Curve"）。
3. **建墙**：`FUN_14085a370(plStack_108, &curve, id, 1, ...)`（InnerWalls::new_wall，返回 WallId）→ `FUN_140aaaac0 / 140aa9610 / 140aaa350`（_try_update_state 三连：update_curve / update_height / update_style）。
4. **颜色**：`FUN_140a94880`（get_building_and_roof_colors）。
5. **墙事件**：push 2×0x18B `{u64, WallId, u32}` 行。
6. **高度**：`fVar36 = 全局常量读（FUN_140c95ef0 跳转表，全局 tag=2@0x142aa5938）`；`fVar37 = (f36·f9 + (f8−f36)·f10)/f11` 插值 → clamp[0, fVar8]；**屋顶高度钳制**：hashbrown（0x78 步长）查该墙屋顶，命中且 `roof_height <= 0.1f`（0x3dcccccd@0x142925984）时用屋顶高。
7. **发 CreateRoofCmd**（0x40B/行 push）：`{u64, WallId, u32, u64, u32, u64=0, f32@+0x28=1.0, f32@+0x2c=height, u64@+0x30, u8@+0x38=0, u8@+0x39=style, u8@+0x3c=0}`。
8. **历史**：hashbrown（0x78 步长）取 WallState（未命中 → panic `FUN_1428d9310(&UNK_142ab6eb8)`）→ 拷贝 curve Vec（8B）+ 高度 Vec（4B）→ `FUN_140a83300`（历史记录墙创建）→ 构建 **0x50B 历史记录** `{1,1,u8,u32,u8,u64=0,f32=1.0,f32=height,u64,u8,u8,WallId}` → `FUN_140a84ec0(tag=8, &UNK_142ab6ed0)`（**CompoundEditBuilder::record，tag 8 = 墙创建**，路径 create_circle.rs）。若 style 有值（`*(i32*)(lStack_128+0x38)!=0`）→ 再建 **0x30B 历史记录**（`{1,1,u8=4,WallId,u32=0,u32=roof_ty}`）→ `FUN_140a84ec0(tag=9, &UNK_142ab6ee8)`（**tag 9 = 屋顶创建**）。
9. **事件**：push 2×0x10B 行 `{u64, u8=7}`、`{u64, WallId}`。

**数据结构**：CreateCircleCmd 48B；待建墙命令 0x30B（tag0=圆/1=矩形，含 ShapeParameters）；CreateRoofCmd 0x40B；历史记录 0x50B(tag8)/0x30B(tag9)。
**常量**：海拔默认 4.0（0x40800000 立即数）、roof 高阈值 0.1 @0x142925984、u 上限 1.0 @0x142925904、哨兵 u64::MAX @0x142925990。
**边界/panic**：curve unwrap 失败、墙缺失（i64::MIN 哈希哨兵 @0x142925990 比较）、屋顶记录越界（0xffffffff 哨兵）。
