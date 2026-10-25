# systems/wall/shapes/rectangle/create_rectangle.rs

> 文件作用：矩形墙创建：CreateRectangleCmd 到 Rectangle2d::as_curve2 + smoothstep 角 + ridge_length_01_from_rectangle_ratio + roof_height_01_from_wall_shape 到 CreateRoofCmd、历史。

函数总计 6：业务 2 / 内联无独立符号 0 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::rectangle::create_rectangle::create_rectangle` | 0x140a93790 | 26 | create_rectangle（smoothstep 角 + roof + 历史） | O | wall\decomp\decomp-0x140a93790.txt |
| `country_core::systems::wall::shapes::rectangle::create_rectangle::get_building_and_roof_colors` | 0x140a94880 | 173 | 访问器/转换 | I | wall\decomp\decomp-0x140a94880.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 4 项

## 实现方式（算法级）

**create_rectangle（O，0x140a93790）**：与 create_circle 同构（共享 wall/roof/history 管线），形状部分走矩形。遍历 `EventReader<CreateRectangleCmd>`（**0x70B/行**：{cmd u64×2@+0, center qword@+0x20, dims qword@+0x30, orientation u64@+0x40, u32@+0x58, …}）。每行处理链：

1. **写待建墙命令**（0x30B/条）：`{tag u32@+0=1(矩形), ShapeParameters 载荷@+4（center qword@+4、dims qword@+0xc、orientation qword@+0x14），f32@+0x20=4.0(海拔), u8@+0x24=1, u64@+0x28}`。
2. **建曲线**：`FUN_140c98cf0(&iStack_248, &uStack_338)`（**Rectangle2d::as_curve2 = smoothstep 圆角采样**，见 valid_enclosure/utils 几何）→ Curve2。
3. **建墙**：`FUN_14085a370`（InnerWalls::new_wall）→ `FUN_140aaaac0/140aa9610/140aaa350`（update curve/enclosure/height/style 三连）。
4. `FUN_140a94880`（get_building_and_roof_colors）。
5. 墙事件 2×0x18B。
6. **屋顶脊长比例**（ridge_length_01_from_rectangle_ratio）：`fVar41 = dims.x/dims.y`、`fVar48 = dims.y/dims.x`；`fVar49 = max(ratio, 1/ratio)`；`max_dim + (-2.25)`（0xc0100000@0x142aa5084）→ `/fVar16` → **clamp01** → 与 `FUN_140c95ef0`（跳转表分派，全局 tag=2@0x142aa5938）读出的插值混合（`(e·f11 + (1−e)·f12)/f13`）→ clamp01。
7. **屋顶高度钳制**：hashbrown（0x78 步长）查屋顶，`roof_height <= 0.1f`（@0x142925984）时用屋顶高。
8. **CreateRoofCmd**（0x40B）：布局同 create_circle（+0x28=1.0、+0x2c=height、+0x39=style）。
9. **历史**：取 WallState（0x78 hashbrown，缺失 → panic `FUN_1428d9310(&UNK_142ada390)`）→ 拷贝曲线 → `FUN_140a83300` → 0x50B 记录 tag 8（`FUN_140a84ec0(..., &UNK_142ada3a8)`）+ 屋顶 tag 9（`&UNK_142ada3c0`）。style 有值时再发第二历史。
10. **事件** 2×0x10B。

**get_building_and_roof_colors（I，0x140a94880）**：从事件/样式记录取 building/roof 颜色（create_circle 同用）。
**常量**：海拔 4.0、roof 高阈值 0.1 @0x142925984、脊长比例 -2.25 @0x142aa5084、全局 tag 2 @0x142aa5938、哨兵 u64::MAX @0x142925990。
**跨模块**：`FUN_140c98cf0` = utils/geometry 矩形 as_curve2；`FUN_140c95ef0` = 共享 enum 分派（跳转表 @0x142b2e028）。
**边界**：屋顶记录越界（0xffffffff 哨兵）、墙缺失 panic。
