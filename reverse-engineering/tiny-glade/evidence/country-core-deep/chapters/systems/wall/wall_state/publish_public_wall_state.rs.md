# systems/wall/wall_state/publish_public_wall_state.rs

> 文件作用：公共墙状态发布（权威流）：14 系统参数；收集 lazy 更新源到逐墙重建 public 记录（源曲线点/形状/style halftimber/material_stats/SparseInstanceBuffer::allocate/哈希段维护）；AnimateWallBulldozeBuffer::contains 保留动画墙；发 OnWallChanged/OnWallDeleted（16B 行）；断言 !is_adding 与 w.simplified_curve_2d.is_valid()。

函数总计 12：业务 1 / 内联无独立符号 2 / 派生胶水 9

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::wall_state::publish_public_wall_state::publish_public_wall_state` | 0x140a8b530 | 51 | publish_public_wall_state（14 参；lazy 源→重建 public→SparseInstanceBuffer::allocate→OnWallChanged/Deleted） | O | wall\decomp\decomp-0x140a8b530.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources`（行 206）— 待推断
- `country_core::systems::wall::wall_state::publish_public_wall_state::process`（行 309）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 9 项

## 实现方式（算法级）

### publish_public_wall_state 0x140a8b530（14 系统参数，101KB）

**处理链**（O，全反编译）
1. **puffin scope**：`FUN_140a87c80(&UNK_142ad8bd1)`（guard），name 串 = `country_core::systems::wall::wall_state::publish_public_wall_state::publish_public_wall_state`（0x142ad8bd1，各系统名连续无 NUL 存储）；前置 UTF-8/`\\`(0x5c)/`/`(0x2f) 路径裁剪扫描（行 369–453），文件串 `crates/country-core/src/systems/wall/wall_state/publish_public_wall_state.rs`（0x142ad9268）。puffin 关闭（`func_0x000140c16a00()`=0）整段跳过。
2. **EventReader**：`func_0x000140c18130()` + `FUN_1428f87d0(rd,0)`/`FUN_140c16d40`（行 455–476）；事件行 16B；读失败路径 `FUN_1428d9250(&UNK_142c3ab70)`（puffin）/`FUN_1428a9a00`（Arc overflow）panic。
3. **lazy 更新源收集（两遍 hashbrown SIMD ctrl 组探针）**：
   - 第一遍（行 555–583）：**PublicWalls 0x2a8 步长 map**（ctrl@`*pauStack_100`，mask@`+8`，探针 `hash & mask` 定位 ctrl 槽，数据槽 = ctrl-0x2a8；条目 = {key WallId 8B, value 0x2a0}）。逐满槽取 `key@-0x2a8`，推 16B 行 `{uVar12@0 = 资源(+0x40) 计数游标, WallId@8}` 入 cap-first Vec（cap@+0x20/ptr@+0x28/len@+0x30，元素 16B，扩容 `FUN_140562630(&vec,&UNK_142c3a148)` = bevy events collections）；每次 +0x40 计数 +1。
   - 第二遍（行 751–892）：**0x78 步长实体键 map**（键比较 `ctrl+(idx&mask)*-0x78`，数据槽 -0x78），取条目 +8 值入同一 Vec。
   - 行 587–589：`uStack_168 = CONCAT44(..., *(u32*)(param_6+0x1c))`（tick 回写）→ **param_6 = ResMut 事件存储（EventWriter<OnWallChanged>）**，EventInstance 16B = {id u64@0, WallId@8}，与 spatial_hash_update 行 +8 读法一致。
4. **逐墙处理主循环**：
   - `FUN_141458b20(*param_10, &wall)`（行 900）+ `FUN_141458a50(*param_10+8, …)`（行 917）= 曲线/形状重建；param_10 = ResMut（+0x1c 回写行 895）。重建临时记录在栈上装配（行 2100–2280：克隆曲线点 Vec<8B>、高度 Vec<4B>；形状 tag 选择 `uVar24=count-2` clamp≤2 → `iStack_6f0=2/3`（圆/矩形/自由）读 `pauStack_108[-3]/[-4]+0xc` 几何参数；`fStack_6c4/fVar78` 默认 `_UNK_1429cd064`(2.5f)/`_UNK_1429258d8`(0.3f)，空记录路径换 `_UNK_142a7de00`/`_UNK_142925870`(0.5f)；若干空 Vec 头 `{0,4,0}`）。
   - PublicWalls 0x2a8 二次查表（行 1041）：`value+0x10 < 2 || value+0x30 <= 0.0` → **panic `assertion failed: w.simplified_curve_2d.is_valid()`**（`FUN_1428d9430(&UNK_142ada193,0x32,&UNK_142ad9308)`；行 0x32=50；串 = `…is_valid()` 与 `public_walls.rs` 拼接存储）。此检查=点计数≥2 且长度>0。
   - `FUN_140b772b0(&cmd, *param_6, hash, &wall)`（行 1155/1354）= **AnimateWallBulldozeBuffer::contains(wall_id)**：动画中墙 → 跳过重建（保留）。
   - `func_0x000142923c00(dst,&src,0x2a0)` memcpy（行 1159/1364/1369）搬 PublicWallState 记录。
   - **panic `assertion failed: !is_adding`**（`FUN_1428d9430(&UNK_142ad92d0,0x1c,&UNK_142ad92f0)`，行 0x1c=28，行 1371）：源记录 i64::MIN 哨兵分支（行 1356/1368 `alStack_e88[0]=0x8000000000000000`，`uStack_120`/is_adding=1）且 `pauStack_110` 非零时触发——墙仍"添加中"却到达安定写路径。
   - **8 个 Vec 释放**（`func_0x000140613c20(ptr,count,elem)`，元素 8/4/12/4/4/12/4/4B，行 1390–1411）+ **7×`FUN_14087f390`**（行 1413–1419）栈上临时值 drop 胶水 + 另 3 个 `func_0x000140613c20`（行 1421–1427，元素 4/4/8B）。
   - **地形采样** `FUN_1409e9640(uStack_168, pos…)`（行 1612/1717，terrain 域）：`new_height[i] = height[i] - terrain_at(pos)`（行 1716–1718，12B 步长源、4B 结果）。
   - **SparseInstanceBuffer::allocate ×2**：`FUN_140969720(uStack_230=*param_11, &UNK_142c2e2b8)`（行 1725）+ `&UNK_142c2e360`（行 1890），静态类型 id（0x142c2e2b8/0x142c2e360 各 16B）；返回缓冲后校验元素类型名 = ssbo_library 注册表串：`&UNK_142a9d7ff`（`country_core::geometry::instanced_wall::BrickTransformSsbo`，len 0x3a=58）/`&UNK_142a9de39`（`…render::nani::instance_data::UberInstanceData`，len 0x3d=61），`func_0x000142923bf0` strncmp 不匹配 → `FUN_1428cab41` panic；分配 0x20B handle `{0,4,0,0}` + **Arc 克隆**（`LOCK;inc;UNLOCK`，-1/负数溢出 → `Arc counter overflow`，行 1814/1832）。
   - `FUN_140b04f70(&cmd, *param_12)`（行 2338）：见纠错节。
   - 收尾：`func_0x000142923c00(&pauStack_608,&stack_0x2a0,0x2a0)` 回写 map 值（行 2351）→ `FUN_14152a360`（行 2353）→ `FUN_140895600`（行 2356/2362）。
5. **三个 EventWriter 刷新**（行 2382–2418）：param_1[3]/[6]/[9] 三组 `{ptr,len}`；`func_0x000142923c20(data,0xff,uVar40+0x11)` memset 0xff 初始化新槽、`(n+1 & ~7)-((n+1)>>3)` 缩容；尾部 `func_0x000140c18130()+FUN_140c170f0` 提交（行 2422–2437）。

**数据结构**：ResMut tick 回写点 = param_2(+0x1c,480)、param_5(+0x1c,1119)、param_6(+0x1c,1221)、param_10(+0x1c,895)、param_1[1]+0xc(482)。

**常量**：`_UNK_1429cd064`=2.5f、`_UNK_14292c76c`=100.0f、`_UNK_1429258d8`=0.3f、`_UNK_142925870`=0.5f、`_UNK_142934600`=24.0f（行 1123/1227/2154 加载）；`_UNK_142c6bf00..18` 32B 静态默认记录（首 f32 0x42c6bef0≈99.37 + u32 1 + 零填充）。

**遗留 U**：inner→public 逐字段复制映射的具体字段名（重建 0x2a0 记录在栈装配，行 2100–2280 仅骨架）；param_12 资源身份（FUN_140b04f70 参数）。

**2026-10-30 类型纠正与木分配生命周期**：上文将 `0x142a9de39` 标为 Uber 错误；按 f507 PE 映射直接读取 61 字节是 `country_core::resources::roofs::roof_buffers::WoodenPlankSsbo`，与 `gpu_clear_wall_gate_pieces@140773c00` 的类型串相同。`140a8d0f5–d165` 查此资源，`140a8d18a–d200` 建空分配并持其共享 owner，`140a8d9c8–d9dd` 写新记录 `rbp+0x628/+0x630`；基址 `rbp+0x500`，即 PublicWallState `+0x128/+0x130`。`140a8dd06–dd3d` 搬入并插入新行。有效更新先取旧行，`140a8ca12–ca19` drop 旧 `+0x128`，`14087f3a5` 检查该对的 `+8` 并退役旧 `+0x130`；成功重发布建立新 owner，跳过的行保持旧 owner。`140773e20–e2b` 在消费变更时缺分配直接 panic。此结论来自静态字节，未执行原程序。

**2026-10-30 通用砖 allocation 独立复核**：f507 SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03` 的 `140a8ce76–cf52` 以 `0x142c2e2b8` 查库，校验 `0x142a9d7ff` 长度 `0x3a` 的 `BrickTransformSsbo` 类型串，虚调用 `+0x18` 后分配空 32B 句柄并取得共享回收 owner；`140a8d9ac–d9c1` 写栈记录 `rbp+0x618/+0x620`，相对记录基址 `rbp+0x500` 即 `+0x118/+0x120`。旧行基址 `rbp+0x840`，`140a8ca06–ca0d` 将旧 `+0x118` 对交给 `14087f390`，该 helper 对非空 `+8` 句柄加入回收队列。成功重发布重建，跳过的行不替换。

`construct_arches@14075fe39–ffa3` 以 0x2a8 桶查墙，Public 值基址为控制指针减 `0x2a0`，读取控制指针减 `0x180` 即 Public `+0x120`，空句柄走失败分支并锁定后清理。`construct_walls@140759844–98cc` 在创建墙实体时另从同一砖 SSBO 新分配句柄并传入 `InstancedWall::new@140858800`；实体独立持有其分配，`delete_wall@140aac1e0` 通过 despawn 清理实体。因分配调用与各自持有者不同，不得把主体分配与公共墙通用拱／门／柱分配合并。当前 sidecar 仅适配该生命周期，不声称复现原私有 ABI。以上为原指令静态核对，未运行原版，未构成 GPU 或完整交互验收。

### 纠错
- **0x1409ef210 不是 new**：decomp PDB 名 = `impl$4::query_with_rectangle2d::closure$0`（get_cells_from_rectangle2d 闭包，bbx/RESAMPLING 断言族）。`WallSpatialHash::new` 真身在 **0x140aa5150**（行 109）。
- **FUN_140b04f70 应为 gizmo 绘制命令而非 WallSpatialHash 查询**：spatial_hash_debug（0x1408ec0e0）中同函数以 `{2,2,i64::MIN,4,1.0/3.0,1}` 命令结构 + 线/矩形数据调用，确定为绘制。DIGEST 对 shape_placement_params "FUN_140b04f70 ×3（I: WallSpatialHash 查询）" 及本行 2338 需按绘制命令复核。
