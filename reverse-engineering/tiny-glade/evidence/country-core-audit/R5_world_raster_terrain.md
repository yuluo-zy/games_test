# R5：C 包（世界栅格 / 地形 CPU 侧）落码

日期：2026-10-14。依据：`country-core-deep/terrain-water/DIGEST.md` §2–§3，**但每一项都用 B EXE 字节/汇编重新核对**（`/tmp/dis.py`：PE 节映射 + `objdump -D`；IAT 解析 `floorf/fmaf/ceilf`）。落点：`tiny-glade/crates/country-core`。标注 O/I/U。

## 落码
| 项 | 位置 | 级别 |
|---|---|---|
| `DirtyTiles`（统一） | `utils/dirty_tiles.rs`（原版 `country_core::utils::dirty_tiles`）；`recovered_building::dirty_tiles` 改为转发（`DirtyTiles64` 别名保留） | O |
| `WorldRaster<512>` 本体 | `resources/world_raster.rs`：`new` / `clear_buffer` / `clear_all` / `copy_composite_to_scratch` / `commit_scratch_to_composite` / `copy_to_staging` / `subscribe` / `write_composite` | O |
| `TileHasher` / `build_tile_hasher` / `DirtyTilePublisher` | 同上 | O（`unique`）/ I（`shared`、`secondary`） |
| 6 个命名栅格 + `AllWorldRasterWrite` | `resources/world_raster_defs.rs`：`ordered_names` / `write` | O |
| `BrushSizes` | `resources/brush_sizes.rs`：6 个方法 | O |
| `ActiveRasterStroke` / `InProgressRasterStroke` / `ActiveRasterStrokesStorage` | `resources/active_raster_stroke.rs`：`start_new` / `add_point` / `last` / `solidify_fluid_into` / `resample_and_smooth` / `index_mut` | O |
| `TerrainHeightsData<T>` | `resources/terrain_height.rs`：`load_index_unchecked` / `sample_at_world_xz` / `copy_from_gpu` / `intersect_ray` | O |

`country-core` 新增依赖 `tabloid`（工作区内已有；`Cargo.lock` 仅多一行依赖边）。

## 比 DIGEST 更正的事实（EXE 为准）
1. **`0x07954dbb` 的语义已解**：它是**全零 8×8 瓦片的哈希**。`copy_to_staging` 的闭包（`DirtyTiles::retain`，0x140adfcb0）对瓦片 64 字节做 ahash 回退实现的 `write`（`buffer = (k2+64)*0x5851f42d4c957f2d`，先尾部 16 B 再 0/16/32 处三块，每块 `rol((buffer+k3) ^ folded_multiply(lo^k0, hi^k1), 23)`，末尾 `rol(folded_multiply(buffer,k3), buffer&63)` 取低 32 位）。四个密钥就是 `build_tile_hasher` 写的四个 u64，编译器把 `(k2+64)*M + k3` 折成立即数 `0x6b229c9255586cd1`（测试复核）。独立的 Python 转写给出全零瓦片哈希 = `0x07954dbb`，与 `WorldRaster::new` 的填充值一致——所以“从未上传过”等价于“全空”，空瓦片不会被重复写出。DIGEST 把它记为 U。
2. **`WorldRaster` 的布局**：+0x60 是 56 字节的 `DirtyTilePublisher`；**+0x98 的 0x200 B 就是 `DirtyTiles` 本体**（不是“内部 staging 缓冲”）；+0x48 的 `Vec<u8>`（0x40 B，零填）是 `copy_to_staging` 的瓦片暂存；+0x30 的 `Vec<u32>` 是 4096 个瓦片哈希。`new` 对 +0x98..+0x29c 整段 `memset(0, 0x204)`，所以 +0x298..+0x29b 四个标志初值全 0。
3. **`copy_to_staging` 的实际流程**：①`+0x299` 置位则 `memset(out, 0, len)` 并清标志；②写 `+0x298`；③`retain`：逐个脏瓦片从栅格取 8×8（8 个 u64，行距 512）算哈希，与状态表相同→**清位**，不同→写 `out[idx*64..+64]`（`idx = y*64+x`，**瓦片主序**，暂存区共 4096×64 = 0x40000 B）、更新哈希、保留位；④发布保留下来的位图；⑤`+0x29a = 1`。`DirtyTilePublisher` 的判别式：`cap - 0x7fffffffffffffff` 取 0/1/2 = Unique / Shared / Secondary；`subscribe` 对 Secondary panic；`copy_to_staging` 对 Shared/Secondary 先抢 parking_lot 锁再发布。
4. **`clear_buffer` 是整块 `memset(0)`**（DIGEST 写“0x200 或全 0x40000”），不动位图；`copy_composite_to_scratch`/`commit_scratch_to_composite` 只做 `copy_from_slice`，同样不置脏。`clear_all` 另会位图 `memset(0xff)`、`+0x299 = 1`、哈希表复位。
5. **`TerrainHeightsData::sample_at_world_xz` 返回 f32，不是 u16**：4 个角各自 `u16/65535` 解码成 `v*max + (1-v)*min`（0x1408b54c0），再沿 z、再沿 x 做 `a + (b-a)*t`（0x140c904d0）。居中/向量两条采样路径由 `*0x14362c308` 所指的字节选择（居中路径 `-0.5` 偏移 + 钳到 511；向量路径 `*512` + 钳到 510，上边界 frac 可达 2），与 `utils::float_grid` 是同一套；这里直接复用 `FloatGridSampler`。
6. **`intersect_ray` 是 32 步行进 + 7 次二分 + 末次选择**（DIGEST 写 3 次）：AABB = x/z∈[-65,65]、y∈[min-0.001, max+0.001]；`step = (t1 - t0) * 0.03125`；命中判据 `height > y`；二分 `hi = mid if y < h`、`lo = mid if !(h > y)`；末次 `t = (y < h) ? lo : mid`。随后与 **y = 0 平面**求交，仅当 `t <= t_max`（`ucomiss; jb`，NaN 也跳过）且命中点在 ±65 之外才算数；与地形命中取较近者，**平面不比地形远（含 NaN）就取平面**。
7. **`copy_from_gpu`**：位图先清零；`min/max` 与默认值（-2.5 / 24.0，NaN 也算不等）任一不等 → 复位并 `memset(0xff)`、标记已变化；随后按 64×8×64 个 16 字节块比较，不同则拷贝并置位；**有变化才向 +0x18 的发布器发布**。
8. **`ActiveRasterStroke` 的字段**：+0x00/+0x08 是 `fluid_started`/`fluid_cursor`（`start_new` 只清前者）；+0x68 是 u32、**+0x6c 是 f32**（`movss`，DIGEST 记成 u32 计数器）；**+0x70 与 +0x270 是两份 `DirtyTiles`**（`draw_raster` 0x142260ae0 里 `clear(+0x270)` → `rasterize_world_space_stroke` → 两次 `extend` → 512 B 互换）；共 6 个槽位（`drop_in_place<ActiveRasterStrokesStorage>` 的偏移序列）。
9. **`add_point` 的状态机**（DIGEST 没写全）：输入为空直接入队；否则 `length += |last-p|`（距离平方按 `(dy²+dx²)+dz²`）；`length > 1.0`（NaN 不触发）时——`fluid_started && cursor >= 2` → `solidify_fluid_into(&input[..cursor], &mut solid)`、`input.drain(..cursor-1)`、`cursor = input.len()`（**不清零 length**）；否则 **同一个点再入队一次**、`fluid_started = true`、`cursor = input.len()`、`length = 0`。
10. **`solidify_fluid_into`**（DIGEST 记为 U）：去相邻重复点 → 不足 2 点原样 / 否则 `Curve::try_new_from_points(..).unwrap().crashy_resample(0.5)` → 目标为空整段追加，否则跳过首点追加。`resample_and_smooth`：`resampled = solid`，再固化全部 `input` 追加，最后 `smooth_in_place(10, false)`（0x141465260，与 `utils::curve::smooth_in_place` 逐指令一致）。
11. **`BrushSizes`**：路径下限的位模式是 **`0x3ea3d70b`**，比字面量 `0.32_f32`（`0x3ea3d70a`）大 1 ULP——必须按位写。`set_*` 是 `minss(上限, maxss(下限, x))`，NaN 会穿过两次钳制。

## 测试（净增 59）
`dirty_tiles` 7、`world_raster` 16（该文件共 17 项，另 1 项为原有）、`world_raster_defs` 5、`brush_sizes` 5、`active_raster_stroke` 14、`terrain_height` 12，合计 59。关键向量：
- 瓦片哈希向量（`0..63` → `0x44fdb575`、全 `0xff` → `0x5170679f`、全零 → `0x07954dbb`）由与 Rust 无关的 Python 按汇编转写得到；
- 折叠立即数 `0x6b229c9255586cd1` 与密钥的关系；
- `add_point` 两次越过阈值的完整序列、`smooth` 十遍用测试内独立公式手算；
- `intersect_ray` 的 NaN `t_max`、平面兜底、平面命中在盒内被忽略。
全量：默认与 `native-physics` 均 **1284 项通过**（基线 1225）；严格 clippy（`--no-deps -D warnings`）、`cargo fmt --check`、`verify_inventory.py`、`verify_foundation_links.py` 通过；`tabloid`、`tg-building-runtime` 测试不受影响。

## 未做 / U
- `rasterize_stroke`（0x140a16960）：依赖 `zeno`（锁文件里有，本机未缓存）的三个 blend 闭包；`rasterize_world_space_stroke`（0x140ade620）依赖 `conservative_line_raster`（锁文件里没有）。**未落码**。
- `WorldRaster` 的 serde（`Expected raster ` / `End of stream while expecting raster `）、`subscribe` 的订阅记录消费方。
- `+0x29b`（`reupload`）的置位点仍 U（只提供读写口）。
- `DirtyTilePublisher::shared/secondary` 在原版内联、无独立符号，构造方式按使用面还原（I）。
- `TerrainHeightsData` 的构造点、+0x00..0x58 里除已知字段外的部分、`TerrainRockMask`、`mouse_terrain_raycast`、`read_mouse_depth_value`、`cell_from_world_space`（0x140960b20：三次 `fmaf`、四通道输出，参数布局没对上）、`draw_raster` 状态机、`ui_terrain_focus_mode`。
- 工作区里 `recovered_building::wall_collection::TerrainHeightGrid512` 与本包的 `TerrainHeightsData::sample_at_world_xz` 是同一套数学的两份实现（前者带显式 `sampling_fix` 与 `Arc<[u16]>`）；这里没有合并，后续可让前者委托 `FloatGridSampler`。
- 渲染 pass 里的 `upload_world_raster::{PathRaster,…}`（测试用的旗标替身）与本包 `world_raster_defs` 的同名类型没有接线。
