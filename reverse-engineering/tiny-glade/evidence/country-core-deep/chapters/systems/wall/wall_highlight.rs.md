# systems/wall/wall_highlight.rs

> 文件作用：墙高亮系统：消费 HighlightWallCmd，WallEntities 查实体，写 WallHighlight{entity, wall, ty}；EditingHighlight rgba 按 ty 四常量选择。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::wall_highlight::wall_highlight` | 0x140981390 | 17 | wall_highlight（WallEntities 查表 + WallHighlight 写入 + ty→rgba 四常量） | O | wall\decomp\decomp-0x140981390.txt |

## 实现方式（算法级）（O，decomp-0x140981390）

**处理链**：4 参 `(ResMut<WallHighlight>, ResMut<EditingHighlight>, EventReader<HighlightWallCmd>,
ResMut<WallEntities>)`。

1. **每 tick 先清** WallHighlight **ty 字节 @+0x10 = 0**。
2. 事件行 **0x18B** `{u64@0, wall_id u64@8, ty u8@0x10}`。
3. **WallEntities 查表**（param_4 deref，map@+0x20，条目步长 **0x10** = `{key u64@0, value Entity u64@8}`）：
   value = `{index u32@0, generation u32@4}`；**高 32 位（generation）== 0 → 直接 return**（无高亮）——
   与 delete_wall 相同「空实体=0」编码。
4. 写 **WallHighlight** `{entity 8B@0, wall_id 8B@8, ty u8@0x10}`（param_1 回写）。
5. **EditingHighlight rgb**（param_2 回写，`u64@0` = r,g 打包 + `u32@8` = b）：
   - `ty==1` → `(0.10, 0.03, 0.00)`：u64 @0x142ab6650 = `{0.10f, 0.03f}`，b=0；
   - `ty!=1` → `(0.07, 0.08, 0.07)`：u64 @0x142ab6640 = `{0.07f, 0.08f}`，b=`0x142a7e354` = `0.07f`。
   重建 HighlightColor::V0/V1 数值完全一致 ✓（实际仅两色；「四常量」第四 = b 通道 0.07f）。
