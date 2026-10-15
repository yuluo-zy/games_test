# resources/terrain_height.rs

> 文件作用：**地形高度场：TerrainHeightsData<T> 泛型资源双实例（TerrainHeightSsbo 权威 / VisualTerrainHeightSsbo 视觉）。u16 512×512 网格@+0x08，min/max f32@+0x50/54（默认 -2.5/24.0）；世界采样/射线求交/GPU 回拷差分。全地形系统的数据底座。**

函数总计 15：业务 5 / 内联无独立符号 7 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>` | 0x1409e9640 | 118 | (x/130+0.5) clamp[0,1] → idx=u*512-0.5+1 clamp[0,511] → 返回 u16 网格值；debug 全局字节 *0x14362c308（round3 round2 修正：==0 才走 +0x201 偏移视觉网格变体，原“≠0”方向反） | O | terrain-water\decomp\decomp-0x1409e9640.txt
| `country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::VisualTerrainHeightSsbo>` | 0x140257960 | 118 | 视觉高度场实例 sample_at_world_xz：双线性变体（4 角 load_index_unchecked + 3 lerp，u16→f32 解码用 +0x50/54 min/max） | O | terrain-water\decomp\decomp-0x141246fb0.txt
| `country_core::resources::terrain_height::TerrainHeightsData::intersect_ray<country_core::resources::terrain_height::TerrainHeightSsbo>` | 0x1409e9a20 | 126 | 高度场 AABB（min/max@+0x50/54）进出 t → 32 步行进 + 每步 3 次 0.5 二分逼近交点，逐步采样 | O | terrain-water\decomp\decomp-0x1409e9a20.txt
| `country_core::resources::terrain_height::TerrainHeightsData::copy_from_gpu<country_core::resources::terrain_height::TerrainHeightSsbo>` | 0x1409ea020 | 247 | min/max 非默认→复位 [-2.5,24.0]；8×8 瓦片粒度 16B 块差分拷贝（不同才拷）并置 64 位脏位图位 | O | terrain-water\decomp\decomp-0x1409ea020.txt
| `country_core::resources::terrain_height::impl$7::load_index_unchecked` | 0x1408b54c0 | 347 | 按索引直接装载网格单元（u16→f32 解码含 min/max；视觉双线性实例 0x141246fb0 内 4 次调用） | O | terrain-water\decomp\decomp-0x1408b54c0.txt

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz`（行 118）— 世界 XZ→高度采样（另见业务行：视觉实例双线性变体）
- `country_core::resources::terrain_height::TerrainHeightsData::heightfield_intersect_ray`（行 161）— 高度场求交核心（intersect_ray 内部）
- `country_core::resources::terrain_height::TerrainHeightsData::subscribe`（行 243）— 订阅地形变化（construct_elevation_supports 消费）
- `country_core::resources::terrain_height::TerrainHeightsData::as_raw`（行 321）— 原始字节视图（save snapshot load）
- `country_core::resources::terrain_height::TerrainHeightsData::as_mut_raw`（行 325）— 原始可变字节视图
- `country_core::resources::terrain_height::TerrainHeightsData::as_bytes`（行 330）— 字节视图（save snapshot capture）
- `country_core::resources::terrain_height::TerrainHeightsData::height_range`（行 334）— 高度范围 [min_y, max_y]

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 3 项

## 实现方式（算法级）

### 双实例（O）

`TerrainHeightsData<T>` 泛型：`TerrainHeightSsbo`（权威 0x1409e9640/0x1409e9a20/0x1409ea020）、`VisualTerrainHeightSsbo`（视觉双线性 0x141246fb0 / 视觉 intersect 0x1412773f0）。布局：+0x08 u16 512×512 网格（512KB）；+0x50/+0x54 min/max（默认 -2.5/24.0 @0x142a9f3f0）。

### 采样

```
sample_at_world_xz 权威（0x9e9640）：
  u = (world.xz/130) + 0.5
  *0x14362c308==0：u=clamp01(u) → idx = fptoui_sat(u*512) → grid[(z<<9|x) + 0x201]  ← +0x201 为正常分支（本轮修正 DIGEST）
  否则：idx 域 clamp → grid[z<<9|x]（无偏移）
sample_at_world_xz 视觉双线性（0x141246fb0）：
  load_index_unchecked ×4（0x1408b54c0，u16→f32 解码含 min/max）+ 3×lerp（func_0x000140c904d0）
```

### intersect_ray（0x1409e9a20）

```
AABB（xz ±65、y=[min,max]）slab 求进出 t（FUN_141240860）→ 无 hit miss
t_near=max(t_near,0)；t_far=min(t_max,t_far)
32 步行进（step=(t_far−t_near)/32，0x142ac4cc0）：ray.y(t) < height(t) → 命中
每命中步 3 次 0.5 二分精化（0x142925870）+ 尾次合并 → t_hit
与 y=min 平面交点（FUN_1412803f0）比距取近
out = {tag, y, x, z, t}
```

### copy_from_gpu（0x1409ea020）

```
memset(局部脏位图[64×u64],0)；min/max 非默认 → 复位 {-2.5,24.0}
for 64×64 个 8×8px 瓦片（行=8×512px unroll）:
  逐 16B 块（8×u16）diff：不同 → copy 16B + dirty[行] |= 1<<列
```

### panic / 调试

- debug 全局字节 *0x14362c308（0 走 +0x201 分支；+0x201 语义 U——halo/边际行缓冲候选）
- copy_from_gpu 未发现 panic（长度按全网格固定）
