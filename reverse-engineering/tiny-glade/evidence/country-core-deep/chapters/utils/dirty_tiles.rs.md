# utils/dirty_tiles.rs

> 文件作用：**64×64 世界脏瓦片位图（512 B = 64×u64）+ tabloid State/StateDelta 协议实现**。
> 覆盖世界范围 **±65 单位（130×130，与 float_grid/terrain_rock_mask 同界）**，单元
> **65/32 = 2.03125 世界单位**（⚠️ 旧版 DIGEST §2.1 把常量 `{32/65, 1/2600, 32.0}` 误读为
> "32 世界单位 / 2048 地图"——`32.0` 是 fmaf 的**偏移**（cell 32 = 网格中心 = 65×32/65），
> 不是单元尺寸；纠错见 `utils/ALGORITHMS.md` §4）。`DirtyTiles` 同时是共享 `tabloid` crate
> 的 `State`（`Clone` + `empty()`）与 `StateDelta`（`apply_to` = 逐字 OR 累积）；地形/栅格
> 系统把笔刷笔触、矩形区域光栅成脏格，经 `Publisher::publish` 发布，订阅者（wall 惰性更新
> `gather_lazy_update_sources`、world_raster `copy_to_staging`、autoclutter ×4、stairs 传播）
> 按 OR 累积批次消费（镜像测试 `shared-foundation/crates/tabloid/tests/dirty_tiles.rs` 已证
> 语义一致：慢订阅者收到多批 OR 快照）。

## 核心算法

```
world→cell：  cell = (i32)trunc( fmaf(world, 32/65, 32.0) )   // fmaf = CRT 导入 @0x142923ef0
              // w=+65 → 64（越界哨兵 0x7fffffff 判定 < 0x40 有效）；w=-65 → 0
位图：         bits[y] = 第 y 行 64 位；set(x,y) = bits[y] |= 1 << (x & 0x3f)
              （cap-first 无关：裸 [u64;64]）
clear/new_clear： memset(self, 0, 0x200)（原字节 tail-jmp memset）
extend/apply_to： 512 B = 32×16 B movups+orps 向量 OR（apply_to = tabloid StateDelta 逐字 OR）
rasterize_world_space_stroke(points, radius)：
  每段 (p1,p2)：cell1 = cell(p1.xz), cell2 = cell(p2.xz)
  r_cells = radius*(32/65) + 1/2600        // 半径换算 + 1/2600 ε
  FUN_140b1e460 = conservative_plot_line(cell1, cell2, r_cells, &self, &early_stop)
  // 复用游戏自有 crate conservative_line_raster（reconstruction reused.rs 同名 stub）
  // early_stop 标志置位后停止后续段
iter_world_space_rect_cells(min, max)：
  4× fmaf 求 min/max 四角 cell → 遍历 x∈[minx..maxx], y∈[miny..maxy]
  <rasterize 闭包>：bits[y] |= 1<<(x&0x3f)；越界 panic（UNK_142ae2060 → dirty_tiles.rs）
  <intersects 闭包>：置栈 bool
rasterize_world_space_rect / intersects_world_space_rect：tail-jmp 到上述闭包变体
retain(pred)：逐格谓词保留（world_raster copy_to_staging 订阅消费）
iter_tiles / iter_dirty_tiles：枚举置位格（逐位展开，内联宿主）
```

函数总计 39：业务 27 / 内联无独立符号 7 / 派生胶水 5

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `DirtyTiles::new_clear` | 0x140960ac0 | 26 | 全零新建 | **O** | 原字节 `mov r8d,0x200; xor edx,edx; jmp memset` |
| `…rasterize_world_space_stroke<Map<Iter<Vec3>,iter_points_xz×2>>` | 0x140ade620 | 32 | 笔触→脏格保守线光栅 | **O** | fmaf 常量 `{32/65, 1/2600, 32.0}` @0x142925874（首两常量 = 32/65、1/2600）+ FUN_140b1e460 |
| `…rasterize_world_space_stroke<replay_raster ×7>` | 0x140ade620 | 32 | 历史回放笔触→脏格 | **O** | 同地址单例（replay_path/replay_water 闭包实例化） |
| `DirtyTiles::iter_world_space_rect_cells<intersects closure>` | 0x140adec50 | 87 | 矩形范围遍历（碰撞闭包） | **O** | 闭包置 bool |
| `DirtyTiles::iter_world_space_rect_cells<rasterize closure>` | 0x140ade8e0 | 87 | 矩形范围遍历（置位闭包） | **O** | `word[y]|=1<<(x&0x3f)`；越界 panic UNK_142ae2060 |
| `DirtyTiles::iter_world_space_rect_cells<autoclutter ×4>` | 0x141cf3540/3900/3c00/3ee0 | 87 | autoclutter 订阅遍历 | **O** | system_clutter 闭包实例化 |
| `DirtyTiles::rasterize_world_space_rect` | 0x140960ae0 | 119 | 世界 AABB 置位 | **O** | tail-jmp 到 rasterize 闭包变体 |
| `DirtyTiles::intersects_world_space_rect` | 0x140960af0 | 127 | 世界 AABB 相交 | **O** | 栈 bool + 闭包变体，返 byte |
| `DirtyTiles::cell_from_world_space` | 0x140960b20 | 141 | 世界→(x,y) cell | **O** | 双 fmaf + cvttss2si；`<0x40` 有效，`0x7fffffff` 哨兵 |
| `DirtyTiles::clear` | 0x140960c20 | 156 | 清空 | **O** | memset 0x200 |
| `DirtyTiles::iter_tiles<wall ×2>` | 0x140adeea0 / 0x140adf4b0 | 160 | 逐格闭包（wall lazy update） | **O** | publish_public_wall_state::gather_lazy_update_sources |
| `DirtyTiles::iter_tiles<stairs>` | 0x1412945c0 | 160 | 逐格闭包（stairs 传播） | **O** | decorator_visual::stairs_assemble |
| `DirtyTiles::retain<copy_to_staging>` | 0x140adfcb0 | 172 | 谓词保留 | **O** | world_raster::copy_to_staging 订阅 |
| `DirtyTiles::extend` | 0x140960c30 | 189 | 逐字 OR 合并 | **O** | 32×movups+orps（512 B/16 B） |
| `impl$2::apply_to` | 0x140960ef0 | 310 | tabloid StateDelta::apply_to | **O** | 16B orps（= 镜像测试语义） |
| `impl$3::iter_dirty_tiles<wall ×2>` | 0x141458b20 / 0x141458a50 | 320 | 脏 delta 遍历 | **O** | gather_lazy_update_sources |

## 内联/无独立符号函数

- `iter_world_space_stroke_cells`（行 43）— **O**（逐位展开迭代器，重建实现合理）
- `iter_tiles`（行 160）— **O**（collect 各格）
- `set`（行 204）— **O**（位循环置位，机器码一致）
- `is_set`（行 215）— **O**（谓词，机器码一致）
- `bitmap_set`（行 221）— **I**（位图辅助，语义同 set）
- `bitmap_intersects`（行 238）— **I**（位图相交）
- `dilate`（行 243）— **I**（无独立符号；名称推断膨胀）

## 派生胶水（5 项：clone、closure 等）

- `impl$4::clone`（行 13）— tabloid `State: Clone` 约束的实现面。
