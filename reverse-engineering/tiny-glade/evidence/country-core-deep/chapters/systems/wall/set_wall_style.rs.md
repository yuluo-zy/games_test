# systems/wall/set_wall_style.rs

> 文件作用：墙样式命令系统：三事件读器（SetWallStyleCmd 0x18B/SetFreehandHeightTypeCmd 0x20B/SetPlasterPeelAmountCmd 0x18B）；style 到 update_style + CompoundEditBuilder 历史；freehand ty 到 update_freehand_ty+update_arch_type；plaster peel 到 update_plaster_peel；样式改变后写 0x20B RequestWallMaterialAudio {tag=0x10, style, pos}。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::set_wall_style::set_wall_style` | 0x1409a2290 | 44 | set_wall_style（三读器 + CompoundEditBuilder + RequestWallMaterialAudio 0x20B） | O | wall\decomp\decomp-0x1409a2290.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项

## 实现方式（算法级）（O，decomp-0x1409a2290）

**处理链**：7 参 `(3×EventReader, ResMut<InnerWalls>, Res<PublicWalls>, ResMut<CompoundEditBuilder>,
EventWriter<RequestWallMaterialAudio>)`。

**行宽核实（纠错）**：param_1 事件 **0x18B**（SetFreehandHeightTypeCmd）、param_2 **0x18B**
（SetWallStyleCmd）、param_3 **0x20B**（SetPlasterPeelAmountCmd）。章节头猜测
「SetFreehandHeightTypeCmd 0x20B」不符 —— freehand 行 0x18B、plaster 行 0x20B。

**三个读器**：
1. reader1：`FUN_140aa99e0`（update_freehand_ty，写 WallState+0x60）+ `FUN_140aa9eb0`
   （update_arch_type，+0x64）；行字节@0x11=freehand_ty、@0x12=arch_type 标志。
2. reader2：`FUN_140aaa530` = **_try_update_state<update_style> 实例**；行字节@0x18=标志。
3. reader3：`FUN_140aaa350` = **_try_update_state<update_plaster_peel> 实例**。

**CompoundEditBuilder 历史（0x140a82b40 ×4）**，niche 变体：`-0x7ffffffffffffff6`
（0x800000000000000A → **变体 11**）freehand_ty、`-0x7ffffffffffffff9`（→ **8**）arch_type、
`-0x7ffffffffffffff7`（→ **10**）style、`-0x7ffffffffffffff8`（→ **9**）plaster_peel；
`FUN_141696f50`（finish）×3、`FUN_140a84ec0`（commit）×3。

**样式改变后音频**（循环收尾，末次事件墙）：PublicWalls 查表（0x2a8 步长）：退化 → panic
`w.simplified_curve_2d.is_valid()`（**file set_wall_style.rs** 0x142abc958，行 50）；曲线 u 插值
`FUN_141467ce0` → 段索引+t，12B 点 `{x@0,y@4,z@8}` 三通道 lerp，越界 → FUN_1428d9518；写
**RequestWallMaterialAudio**：param_7 EventWriter，**0x20B 行** `{wall_id u64@0, u8 tag=0x10@8,
u8 flag@9, {f32,f32} pos@0x10, f32 z@0x18}`。
