# resources/history/wall_history.rs

> 文件作用：墙历史事件接口：History 的 WallHistory trait 实现（ev_new_wall 0x140a83300：断言 curve 长度非零、构建 0x108B WallHistoryEdit + FUN_141696f50 校验 → History::add(tag 7)；ev_height_changed 0x140a83f80：EditType::new_height_changed（0x140a83fb0，niche+2 判别）→ History::add）。

函数总计 3：业务 3 / 内联无独立符号 0 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::history::wall_history::impl$0::ev_new_wall` | 0x140a83300 | 25 | 断言 curve 长度非零；构建 0x108B WallHistoryEdit+FUN_141696f50 校验 → History::add(tag 7) | O | decomp-0x140a83300.txt |
| `country_core::resources::history::wall_history::impl$0::ev_height_changed` | 0x140a83f80 | 75 | new_height_changed(buf,wall,h0,h1) → History::add（track-caller） | O | 原字节 0x140a83f80 |
| `country_core::resources::history::EditType::new_height_changed` | 0x140a83fb0 | 81 | 写 24B 记录 {1,1,niche+2@+0x10,wall,h0,h1}+0x108B 拷贝+FUN_141696f50 | O | decomp-0x140a83fb0.txt |

<!-- 未覆盖函数数：0 -->
