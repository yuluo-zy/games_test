# utils/dirty_tiles.rs

> 文件作用：**64×64 世界脏瓦片位图（512 B）+ tabloid State/StateDelta 协议**。`DirtyTiles` 同时是
> 共享 `tabloid` crate 的 `State` 与 `StateDelta`（`apply_to` = 逐字 OR 累积）；地形/栅格系统把笔刷
> 笔触、矩形区域光栅成脏格，经 tabloid Publisher 发布，订阅者（wall 惰性更新、world_raster
> copy_to_staging、autoclutter、stairs 传播）按 OR 累积批次消费。单元 = 32 世界单位
> （`rasterize_world_space_stroke` 常量 `{32/65, 1/2600, 32.0}` @0x142925874 → 64×32=2048×2048 地图）。
> 证据：clear/new_clear memset 0x200、extend OR、iter_world_space_rect_cells 位循环、tabloid 镜像测试
> `shared-foundation/crates/tabloid/tests/dirty_tiles.rs`。

函数总计 39：业务 27 / 内联无独立符号 7 / 派生胶水 5

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::utils::dirty_tiles::DirtyTiles::new_clear` | 0x140960ac0 | 26 | 全零新建 | **O** | 原字节 `mov r8d,0x200; xor edx,edx; jmp memset` |
| `country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<Map<Iter<Vec3>,iter_points_xz×2>>` | 0x140ade620 | 32 | 笔触→脏格保守线光栅 | **O** | 世界→cell 缩放(0x142925874 常量) + FUN_140b1e460 保守线（复用 conservative_line_raster） |
| `…rasterize_world_space_stroke<replay_raster ×7>` | 0x140ade620 | 32 | 历史回放笔触→脏格 | **O** | 同地址单例（replay_path/replay_water 闭包实例化） |
| `DirtyTiles::iter_world_space_rect_cells<intersects closure>` | 0x140adec50 | 87 | 矩形范围遍历（碰撞闭包） | **O** | 闭包置 bool |
| `DirtyTiles::iter_world_space_rect_cells<rasterize closure>` | 0x140ade8e0 | 87 | 矩形范围遍历（置位闭包） | **O** | `word[y]|=1<<(x&0x3f)`；越界 panic UNK_142ae2060 |
| `DirtyTiles::iter_world_space_rect_cells<autoclutter ×4>` | 0x141cf3540/3900/3c00/3ee0 | 87 | autoclutter 订阅遍历 | **O** | system_clutter 闭包实例化 |
| `DirtyTiles::rasterize_world_space_rect` | 0x140960ae0 | 119 | 世界 AABB 置位 | **O** | tail-jmp 到 rasterize 闭包变体 |
| `DirtyTiles::intersects_world_space_rect` | 0x140960af0 | 127 | 世界 AABB 相交 | **O** | 栈 bool + 闭包变体，返 byte |
| `DirtyTiles::cell_from_world_space` | 0x140960b20 | 141 | 世界→(x,y) cell | **O** | 双 fmaf + cvttss2si，`<0x40` 有效，`0x7fffffff` 哨兵 |
| `DirtyTiles::clear` | 0x140960c20 | 156 | 清空 | **O** | memset 0x200 |
| `DirtyTiles::iter_tiles<wall ×2>` | 0x140adeea0 / 0x140adf4b0 | 160 | 逐格闭包（wall lazy update） | **O** | publish_public_wall_state::gather_lazy_update_sources |
| `DirtyTiles::iter_tiles<stairs>` | 0x1412945c0 | 160 | 逐格闭包（stairs 传播） | **O** | decorator_visual::stairs_assemble |
| `DirtyTiles::retain<copy_to_staging>` | 0x140adfcb0 | 172 | 谓词保留 | **O** | world_raster::copy_to_staging 订阅 |
| `DirtyTiles::extend` | 0x140960c30 | 189 | 逐字 OR 合并 | **O** | 32×movups+orps |
| `impl$2::apply_to` | 0x140960ef0 | 310 | tabloid StateDelta::apply_to | **O** | 16B orps（= 镜像测试语义） |
| `impl$3::iter_dirty_tiles<wall ×2>` | 0x141458b20 / 0x141458a50 | 320 | 脏 delta 遍历 | **O** | gather_lazy_update_sources |

## 内联/无独立符号函数

- `DirtyTiles::iter_world_space_stroke_cells`（行 43）— **O**（逐位展开迭代器，重建实现合理）
- `DirtyTiles::iter_tiles`（行 160）— **O**（collect 各格）
- `DirtyTiles::set`（行 204）— **O**（位循环置位，机器码一致）
- `DirtyTiles::is_set`（行 215）— **O**（谓词，机器码一致）
- `DirtyTiles::bitmap_set`（行 221）— **I**（位图辅助，语义同 set）
- `DirtyTiles::bitmap_intersects`（行 238）— **I**（位图相交）
- `DirtyTiles::dilate`（行 243）— **I**（无独立符号；名称推断膨胀）

## 派生胶水（5 项：clone、closure 等）
