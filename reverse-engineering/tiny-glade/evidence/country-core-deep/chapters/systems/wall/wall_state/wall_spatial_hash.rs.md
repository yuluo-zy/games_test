# systems/wall/wall_state/wall_spatial_hash.rs

> 文件作用：墙空间哈希结构：固定 64×64 格（walls-audit）；add_segment 逐格填 0x38B 成员 {WallId, 段数据}（格条目 0x30B 链表）；remove_wall_id；get_cells_from_aabb/circle/rectangle2d/segment 栅格化查询（断言 RESAMPLING<cell_size 与 bbx 越界）。

函数总计 54：业务 23 / 内联无独立符号 10 / 派生胶水 21

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment::terrain_height_closest_to_ws_pos` | 0x140aa50e0 | 39 | WallSegment::terrain_height_closest_to_ws_pos（f64 距离加权地形高） | O | wall\decomp\decomp-0x140aa50e0.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::new` | 0x140aa5150 | 109 | 构造/初始化 | I | wall\decomp\decomp-0x140aa5150.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::remove_wall_id` | 0x140aa5360 | 120 | WallSpatialHash::remove_wall_id（hashbrown erase） | O | wall\decomp\decomp-0x140aa5360.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query` | 0x140aa5730 | 130 | 查询/计算 | I | wall\decomp\decomp-0x140aa5730.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_segment` | 0x140aa5850 | 146 | 查询/计算 | I | wall\decomp\decomp-0x140aa5850.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle` | 0x140aa5940 | 162 | 查询/计算 | I | wall\decomp\decomp-0x140aa5940.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<forest::manage_tree_prefabs::manage_tree_prefabs::closure_env$0>` | 0x1422785d0 | 179 | WallSpatialHash::query_with_circle_callback 闭包实例（圆查询：f64 网格坐标换算 + 半径过滤 + 逐格成员距离校验 + bbx_min/RESAMPLING 断言） | O | wall\decomp\decomp-0x1422785d0.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<system_tree::convert_preplaced_trees::convert_preplaced_trees::closure_env$0>` | 0x1425defd0 | 179 | WallSpatialHash::query_with_circle_callback 闭包实例（同 0x1422785d0） | O | wall\decomp\decomp-0x1425defd0.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<system_tree::hide_trees::hide_trees::closure_env$0>` | 0x1425df320 | 179 | WallSpatialHash::query_with_circle_callback 闭包实例（同 0x1422785d0，含 bbx_max 断言） | O | wall\decomp\decomp-0x1425df320.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_aabb_clamped` | 0x140aa5a10 | 195 | 查询/计算 | I | wall\decomp\decomp-0x140aa5a10.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_rectangle2d` | 0x140aa5ba0 | 217 | 查询/计算 | I | wall\decomp\decomp-0x140aa5ba0.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_rectangle2d<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_rectangle2d::closure_env$0>` | 0x1409ef210 | 232 | WallSpatialHash::new（bbx 越界断言族） | O | wall\decomp\decomp-0x1409ef210.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::add_segment::closure_env$0>` | 0x1409ef870 | 273 | 累加/追加 | I | wall\decomp\decomp-0x1409ef870.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_segment::closure_env$0>` | 0x1409eff80 | 273 | 访问器/转换 | I | wall\decomp\decomp-0x1409eff80.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_circle<country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug::closure_env$2>` | 0x1409f0e80 | 381 | 访问器/转换 | I | wall\decomp\decomp-0x1409f0e80.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_circle<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_circle::closure_env$0>` | 0x1409f08f0 | 381 | 访问器/转换 | I | wall\decomp\decomp-0x1409f08f0.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_center` | 0x140aa5c70 | 409 | WallSpatialHash 查询/换算方法（[+0x48] 分支，0x110 帧） | I | wall\decomp\decomp-0x140aa5c70.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_aabb<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_aabb_clamped::closure_env$0>` | 0x1409f12e0 | 423 | 访问器/转换 | I | wall\decomp\decomp-0x1409f12e0.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::add_segment` | 0x140aa5dd0 | 454 | WallSpatialHash::add_segment（逐格 0x38B 成员 push） | O | wall\decomp\decomp-0x140aa5dd0.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::world_space_to_grid_coord_unclamped` | 0x140aa6520 | 497 | world_space_to_grid_coord 浮点换算（[+0x50]/[+0x54] + 常量） | O | wall\decomp\decomp-0x140aa6520.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::grid_coord_to_cell_index` | 0x140aa65d0 | 521 | grid_coord_to_cell_index（gy×dims.x+gx：imul [rcx+0x48]） | O | wall\decomp\decomp-0x140aa65d0.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_size` | 0x140aa65e0 | 538 | 浮点网格坐标换算（cvtsi2ss [+0x48]/[+0x50]/[+0x54]） | O | wall\decomp\decomp-0x140aa65e0.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::bounds_ws` | 0x140aa6600 | 545 | cell 中心 f64 换算（movddup [+0x50] × 常量） | O | wall\decomp\decomp-0x140aa6600.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::wall_state::wall_spatial_hash::GridCell::new`（行 64）— 构造
- `country_core::systems::wall::wall_state::wall_spatial_hash::GridCell::add`（行 71）— 构造/注册/生成
- `country_core::systems::wall::wall_state::wall_spatial_hash::GridCell::remove_segments`（行 79）— 删除/清理
- `country_core::systems::wall::wall_state::wall_spatial_hash::GridCell::walls`（行 84）— 待推断
- `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_circle`（行 381）— 查询/计算
- `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_aabb`（行 423）— 查询/计算
- `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::world_space_to_grid_coord`（行 505）— 待推断
- `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_index_to_grid_coord`（行 527）— 待推断
- `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cell_at_index`（行 534）— 查询/计算
- `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::dims`（行 552）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`closure` 等 21 项

## 实现方式（算法级）

### WallSpatialHash 结构布局（O，new=0x140aa5150 + 换算族机器码）

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x00 / +0x08 / +0x10 | cells Vec cap-first {cap,ptr,len}；cap=len=0x1000(4096=64×64) | new 行 60–84 |
| +0x18 / +0x20 / +0x28 / +0x30 | hashbrown map `wall_id → 每墙格列表`：ctrl / mask u64 / data / items；空表 ctrl=静态 0x142ADB0C4 | new 行 87–93；remove/add_segment 探针 |
| +0x38 / +0x40 | hash 种子 u64（new 里全局静态种子生成器 `FUN_140d4d740`/`FUN_140d64e00` 递增产生） | new 行 65–94；remove 行 44 |
| +0x48 / +0x4c | dims u32 = {64, 64}（`0x4000000040`） | new 行 95；grid_coord_to_cell_index imul |
| +0x50 / +0x54 | extent f32 = {130.0, 130.0}（0x43020000）→ cell_size = {130/64≈2.03125} | new 行 96；cell_size 机器码 |
| +0x58 | 段版本计数器 u32（每 add_segment +1） | add_segment 行 93–102 |

**new 0x140aa5150**（O）：`malloc(0x30000)`=4096×0x30B 格缓冲；cells Vec {cap 0x1000, ptr, len 0x1000}（`FUN_1412f3f10` 初始化）；种子经全局 `FUN_140d4d740()` 静态 0x18B 记录 + `FUN_140d64e00()` 生成、+1 递增；dims/extent/版本如上。malloc 失败 panic `FUN_1428d8fe3(8,0x30000,&UNK_142adce50)`（文件 = wall_spatial_hash.rs）。**64×64 先验证实**（O，非仅合同）。

**GridCell 0x30B**：{member Vec cap@0,ptr@8,len@0x10} + {wall_id Vec cap@0x18,ptr@0x20,len@0x28}（add_segment 行 150–190）。
**WallSegment 成员 0x38B**：{raw a Vec2@0, raw b Vec2@8, height_a@0x10, height_b@0x14, snapped_a Vec2@0x18, snapped_b Vec2@0x20, wall_id u64@0x28, version u32@0x30}（add_segment 行 179–189，字段值来自 spatial_hash_update 传参）。

### add_segment 0x140aa5dd0（O）
1. `*(u32*)(self+0x58) += 1`（版本号）。
2. `FUN_1409ef870(param_10=RESAMPLING 0.28f, start/end 几何…, &result_vec)` = **get_cells_from_segment<add_segment closure>** → 命中格索引 Vec（cap-first）。
3. 逐格索引 uVar15：
   - 越界 `*(self+0x10) <= idx` → panic `FUN_1428d9518(idx,count,&UNK_142adce80)`（wall_spatial_hash.rs）。
   - 格记录 = `cells_ptr + idx*0x30`；**墙去重**：扫格内 wall_id Vec（8B 元素，ptr@+0x20，len@+0x28），已含本墙则不重复加墙；新墙 push（cap 判等 → 扩容 `FUN_140562870(plVar16,&UNK_142adce20)`，8B 元素）。
   - **push 成员**：len==cap（格 +0x00）→ 扩容 `FUN_1406319d0(plVar14,&UNK_142adce38)`（0x38B 元素）；写 0x38B 成员（布局见上）。
   - **map 维护**：`*(self+0x30)!=0`（map 非空）时 hash 探针（0x38 步长，key=wall_id）——命中则 `FUN_141524fe0(entry_value_ptr, cell_idx)` 追加格；未命中经 `FUN_141527f60(&map, wall_id)` get-or-create（初值静态默认记录 `_UNK_142c6bf00..18`）后追加。
4. 结果 Vec 释放 `func_0x000140613c20(ptr, len<<3, 8)`。

### remove_wall_id 0x140aa5360（O）
1. hash = `FUN_1405b0d70(self+0x38, &wall_id)`；`FUN_140b768c0(&entry, self+0x18, hash, &key)` = **hashbrown erase**，返回条目（值 = 该墙的格索引列表 Vec<u64>）。
2. 逐格（值 Vec 元素 = 格索引 uVar13）：越界 panic（UNK_142adce68）；对该格：
   - **清成员**（行 118–165）：扫 0x38B 成员，`wall_id@+0x28` 匹配者通过 0x38B 块前移（`pauVar17` 循环 memmove）删除，`len(+0x10) -= 删除数`。
   - **清 wall_id 列表**（行 166–225）：8B 元素右移删除（`plVar18` 循环），`len(+0x28) -= 删除数`。
3. 释放值 Vec 缓冲 `func_0x000140613c20`（元素 0x10B）。

### 坐标换算族（O，机器码手解）
- **grid_coord_to_cell_index 0x140aa65d0**（Ghidra NotFound，原字节）：`imul r8d,[rcx+0x48]; add r8d,edx; movsxd rax,r8d; ret` → **i = y*dims.x + x**（{x=edx, y=r8d}）。
- **cell_size 0x140aa65e0**：`cvtsi2ss(+0x48); movss(+0x50); divss; cvtsi2ss(+0x4c); movss(+0x54); divss` → **{+0x50/dims.x, +0x54/dims.y}**。
- **bounds_ws 0x140aa6600**：`mov rax,rcx; movddup xmm0,[rdx+0x50]; mulps xmm0,[rip+0x142AA5D60]; movups [rcx],xmm0` → 返回 16B **{-extent.x, -extent.y, +extent.x, +extent.y}**（乘数常量 = {−1,−1,+1,+1}）——名义边界 ±extent，实际有效范围 ±extent/2（换算 + clamp 双重保证）。
- **world_space_to_grid_coord_unclamped 0x140aa6520**：`grid = floor((ws + 0.5*extent) / cell_size)`（`_UNK_142925870`=0.5f 乘 +0x50/+0x54）；`func_0x000142923ee0` = **floor 导入 jmp thunk**；饱和 f32→i32 cast：`> 0x4effffff(=2147483520f, 最大可表示 <2^31) → 0x7fffffff; NaN → 0`，`cvttss2si`。
- **cell_center 0x140aa5c70**：dims.x==0 → panic（`FUN_1428d9a20(&UNK_142adce98)`，行）；`param_2==i32::MIN && dims==-1` 分支防除零；`y=floor(index/dims.x)`（饱和 cast）+ `mod` 拆分 x/y；divps `{+0x50, mod,0,0}/{dims,dims,0,0}` 与 `{+0x54, y,0,0}/{dimy,...}` 算中心（= min + (frac+0.5)*cell_size，min=−0.5*extent；Ghidra 因未初始 XMM 输入丢部分向量，语义经闭包用法确认：低 32 位=center.x、高=center.y）。

### 查询族（O）
- **query(point) 0x140aa5730**：单点换算 → 格索引 → 越界则返回空（值 8），否则返回**该格 member 列表指针**（`cells_ptr + idx*0x30 + 8`）。
- **query_with_segment/circle/aabb_clamped/rectangle2d 闭包**（0x140aa5850/5940/5a10/5ba0）：调对应 `get_cells_from_*`（0x1409eff80/9f08f0/9f12e0/9ef210）→ 收集成员指针列表（cap-first Vec {ptr@+8,len@+0x10}）→ **按 version@+0x30 去重**：相邻成员 version 相等（同墙相邻格）保留首个（`FUN_141338a90` <0x15 栈内 / `FUN_1409583c0` 大表排序去重）。
- **query_with_circle_callback 三闭包实例**（0x1422785d0 / 0x1425defd0 = 同算法、仅 panic 串地址不同；0x1425df320 = hide_trees 完整成员遍历）：
  1. `bounds_ws`（−extent/+extent）与圆 AABB 取 `maxps/minps` 夹取；断言 `bbx_min.x >= grid_min.x && bbx_min.y >= grid_min.y`（UNK_1430374c0，行 0x44=68）、`bbx_max…`（UNK_143037590，行 68）、`RESAMPLING < self.cell_size().x`（UNK_1430375f0，行 0x31=49）。
  2. 夹取后 `world_space_to_grid_coord_unclamped` 换算 x/y 格界，clamp 到 `[0, dims-1]`。
  3. 逐格：`grid_coord_to_cell_index` → `cell_center` → **半径过滤**：`dist(center, query_center) < radius + cell_size.x*0.5*√2`（`_UNK_14292e6f4`=1.4142135f，半格对角线膨胀）；格越界 panic（UNK_143037640）。
  4. hide_trees 变体（0x1425df320）：命中格再逐 0x38B 成员：`member.wall_id(+0x28)==排除墙` 时做**点到段距离**：`t = clamp(dot/|b-a|², 0, 1.0)`（`_UNK_142a7baa0`=eps、`_UNK_142925904`=1.0），`SQRT(dist²) < _UNK_142925970(0.4f)` → 返回命中；manage_tree_prefabs 变体仅判格内有成员即命中（布尔"墙存在"查询）。

### 常量表（位型 + 数值 + 含义）
| 地址 | 位型 | 数值 | 含义 |
|---|---|---|---|
| 0x142925870 | f32 | 0.5 | 换算半程系数 / AABB clamp |
| 0x1429258e0/8e4 | f32 | +0.5 | 换算 +0.5×extent |
| 0x1429258e8/8ec | f32 | 0.0 | 换算零填充 |
| 0x142a171f0/1f4 | f32 | −0.5 | 网格 min 角系数（get_cells_from_rectangle2d） |
| 0x14292e6f4 | f32 | √2 | 圆查询半径膨胀（半格对角线） |
| 0x142925970 | f32 | 0.4 | hide_trees 段碰撞半径 |
| 0x142a7baa0 | f32 | 2.5e-20 | 段长² eps |
| 0x142925980 | f32 | f32::MAX | 最近点初值 |
| 0x142925904 | f32 | 1.0 | t clamp 上界 / RESAMPLING 判据 |
| 0x1429a49bc | f32 | 0x4effffff | 饱和 cast 上界（2147483520f） |
| 0x142AA5D60 | 4×f32 | {−1,−1,+1,+1} | bounds_ws 乘数 |
| 0x14302…(new) | u64 | 0x4000000040 | dims {64,64} |
| 0x1429a3c70 | 2×f32 | {130.0,130.0} | extent |

**跨模块 callee**：FUN_1409ef870/1409eff80/1409f08f0/1409f12e0/1409ef210（get_cells_from_*，wall 域自身）、FUN_141524fe0/FUN_141527f60（格列表/表项维护，wall 域）、FUN_1406319d0/FUN_140562870（cap-first Vec 扩容 0x38B/8B）、FUN_140b768c0（hashbrown erase）、FUN_1405b0d70（hash）、FUN_141338a90/FUN_1409583c0（切片去重）、func_0x000142923ee0（floor 导入 thunk）。

### 遗留 U
- map@+0x18 值记录 0x30B 的字段语义（初值静态 `_UNK_142c6bf00` 的 0x42c6bef0≈99.37f/u32 1 含义）。
- `get_cells_from_segment` 0x1409ef870 内部 RESAMPLING 步进细节（仅见接口与 0x28f RESAMPLING 常量）。
- cell_center 的精确浮点表达式（Ghidra 向量退化，语义经闭包用法确认）。
