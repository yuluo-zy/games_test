# systems/ui_clutter/debug_spawn_clutter.rs

> 文件作用：调试批量生成 clutter：消费 DebugSpawnClutterCmd/DebugSpawnAllClutterCmd 事件批量生成。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui_clutter::debug_spawn_clutter::debug_spawn_all_clutter` | 0x1409a1f50 | 30 | 系统:事件处理（debug_spawn_all_clutter） | O：消费两命令批量 spawn（wp20 合同复核） | ui-input\decomp\decomp-0x1409a1f50.txt |

## 实现方式（算法级）

- `debug_spawn_all_clutter` @0x1409a1f50（O，本域手解）：消费 `DebugSpawnClutterCmd`/`DebugSpawnAllClutterCmd` 双事件队列（bevy VecDeque，元素 stride 0x10）。
  - 单条命令：推 0x48B spawn 事件（`tag=2`、`subtype=3 @+0x29`、目标 entity@+0x40）。
  - 批量命令：螺旋分布 0x2a=42 个——`r = sqrt(n)`、`θ = n * _UNK_142abbcec`，`cos/sin` 定位（`func_0x000142923e80`/`func_0x000142923fd0`），逐项类型字节 `(&UNK_142aa0013)[n]` + `FUN_1409e9640`，推 0x48B 事件（@+0x30/0x38 = 位置 Vec2，@+0x2c 旋转）。
  - 用途：调试把 clutter 铺满场景的确定性 scatter（O）。
