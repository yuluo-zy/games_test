# systems/wall/shapes/shape_placement.rs

> 文件作用：形状放置参数：光标地形位到 ShapePlacement::new_unsnapped；closest_segment_to_snap_to（WallSpatialHash 查询 + distance_segment_point）；snap_to_roof_ridge（Roof::pivot_3d/RoofRidgeDims::calc）。

函数总计 12：业务 2 / 内联无独立符号 2 / 派生胶水 8

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge` | 0x140980db0 | 0 | snap_to_roof_ridge（Roof::pivot_3d/RoofRidgeDims::calc） | O | wall\decomp\decomp-0x140980db0.txt |
| `country_core::systems::wall::shapes::shape_placement::shape_placement_params` | 0x14097f970 | 86 | shape_placement_params（光标位→new_unsnapped + 吸附） | O | wall\decomp\decomp-0x14097f970.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::shapes::shape_placement::ShapePlacement::new_unsnapped`（行 70）— 构造
- `country_core::systems::wall::shapes::shape_placement::closest_segment_to_snap_to`（行 456）— 查询/计算

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 8 项

## 实现方式（算法级）

**shape_placement_params（O，0x14097f970）**：14 参系统。光标地形位 → 生成 `ShapePlacement`（是否吸附、目标点、法向）。处理链（按反编译逐分支）：

1. **光标取位**：`FUN_1408cdc60(&uStack_148, ..., *param_11, 0)`（光标→地形坐标，uStack_148 为 2D 点）。
2. **模式分派**：以形状模式字节（uStack_148 & 0xff，取自 shape 选择资源）走跳转表（case 0/9 与 case 2 为主要路径；default → 返回）。
3. **case 0/9（未吸附）**：
   - `FUN_140a91ef0(*param_3, lVar29, &UNK_142ab6fe8)`（PublicWalls 查墙）→ 命中则：
   - 距离钳制 `min(fVar52, 60.0f)`（0x42700000@0x1429d18cc）；
   - `FUN_141469950(wall+0x38, ...)` 与 `FUN_141469a20(wall, ...)`（approximate_curve_u 双路）→ u 夹取 [0,1]；
   - 预置形状判别 `*(u32*)(wall+600) < 2`（tag@+0x258）：tag==0（circle）→ `FUN_140c978e0(wall+0x25c)`（圆 get_normal_at_u，payload 基 = +0x25c）；tag==1（rect）→ 读 `+0x25c/+0x260` 两 f32（cos/sin）；tag>=2（freehand）→ `FUN_141468ee0`（自由曲线法向）；
   - `if (fVar52 < 1.5f)`（0x3fc00000@0x1429258d0）→ `FUN_1414671d0`（吸附到曲线段）。
4. **case 2（吸附）**：
   - `FUN_140a91ef0`（PublicWalls 查墙，路径 UNK_142ab6f40）；`tag@+600 <= 1` 才继续（仅预置形状）；
   - u 计算 + 法向（circle→FUN_140c978e0()，rect→读 +0x25c/+0x260 qword）；
   - **形状类型断言**：`*piVar20 != 6 || ty!=2` 且 `*piVar20 != 6 || ty!=1` → panic_str（`FUN_1428d9430(&UNK_142ab6841, 0x28, &UNK_142ab6fd0)`，shape_placement.rs:40）。rect 预览半径偏移 -6.0（0xc0c00000@0x142aab4d4），circle -8.0（0xc1000000@0x1429cd074）；
   - `FUN_140a922b0(pfStack_150, lVar29)`（PublicWalls::roof_entity 查屋顶）；无屋顶则跳过；
   - `FUN_1409767a0(&uStack_170, ...)`（**closest_segment_to_snap_to**，屋顶版）结果 ==3 才继续 → `FUN_140980db0(&uStack_170, ..., 1.0f)`（**snap_to_roof_ridge**）；
   - 按结果再分：result==1 → **new_unsnapped**（置 {0.0, 0.3f, ...}，`FUN_140b02520` 构造）+ `FUN_1409766a0()`（**closest_segment_to_snap_to** 墙版）+ `func_0x000140c9cde0`（曲线）+ `FUN_140b04f70(&uStack_148, uVar23)`（**WallSpatialHash 查询**）→ 状态字节 = 2（吸附到墙）；else 分支 → 另一套：法向插值（`fVar36 = 1.0 - fVar47` 混合，偏移 +0.2 常量族 0x142a89ef0..fc）→ 同套 closest_segment + 空间哈希查询 → 状态 2。
   - `fVar52 += fVar36 + 2.5f`（0x40200000@0x1429cd064）。
5. **结果写回**：构造 ShapePlacement 记录 {模式字节 u8@+0（0 未吸附/2 吸附）、snapped u8@+1、位置 Vec2、目标点、法向}（具体字段序 U，见 `code_r0x00014097ff79` 的 0x30B 记录布局：+0 模式、+0x2d 位置、+0xd 目标/法向）。

**snap_to_roof_ridge（O，0x140980db0）**：`Roof::pivot_3d` 族 + `RoofRidgeDims::calc`（屋顶脊吸附点计算），输入 u=1.0（_UNK_142925870），输出 {0=无/1=新建/其他} 判别 + 吸附点。**新常量**：距离钳制 60.0 @0x1429d18cc、吸附阈值 1.5 @0x1429258d0、preview 偏移 rect -6.0 @0x142aab4d4 / circle -8.0 @0x1429cd074、+2.5 @0x1429cd064、0.2 偏移族 @0x142a89ef0..fc。
**边界**：形状类型不符 → panic（shape_placement.rs:40）；命中 roof 但 wall 非预置 → 跳过。
