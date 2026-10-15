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
## 实现方式（算法级，2026-10-15 深挖）

- **ev_new_wall 0x140a83300**：断言 `new_wall.curve.length_ws() != 0.0`
  （wall_history.rs，@0x142ad7d48）；构建 0x108B WallHistoryEdit（载荷）+ FUN_141696f50
  校验 → `History::add({tag 7, records=&组表, count}, file)`；records = 组数组
  （16B {入口指针数组, 组数}），add 侧按 niche 分变体做曲线/None 校验（见 mod.rs
  算法节）。
- **ev_height_changed 0x140a83f80**：栈 24B 缓冲 → `EditType::new_height_changed(buf,
  wall, h0, h1)`（0x140a83fb0：写 {tag 7, records=&0x108B 载荷, count} + 载荷
  {1, 1, niche 0x8000000000000002=变体2, wall, h0, h1} + 0x108B 拷贝 + 校验）→
  `History::add(self, &buf, &静态文件串 0x142ae7818)`（track-caller）。

