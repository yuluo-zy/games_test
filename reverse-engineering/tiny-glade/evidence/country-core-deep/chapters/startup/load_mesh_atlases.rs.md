# startup/load_mesh_atlases.rs

> 文件作用：**网格图集（mesh atlas）生产者：生成程序化图集 mesh parts（stone_floor/plaster/stairs_floor/smoke）+ 载入 nani 网格（assets/nani_meshes.ron）+ 地形网格。是渲染 pass 的网格资产输入。**

函数总计 19：业务 5 / 内联无独立符号 4 / 派生胶水 10

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >` | 0x1415f8c40 | 53 | `visit_str<ron::Error>`：ron 枚举字符串反序列化胶水。 | **O** | decomp-0x1415f8c40.txt |
| `country_core::startup::load_mesh_atlases::load_mesh_atlases` | 0x14129f400 | 69 | `load_mesh_atlases`：生成图集 parts — `Vertex_Position`+`stone_floor_`、`plaster`/`plaster_strip`、`stairs_floor_chunk_`、`smoke`；调 grid_gen_cw/ccw（0x140928100/350）、load_nani_meshes（0x1415f7190）。 | **O** | decomp-0x14129f400.txt |
| `country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>` | 0x1415f7190 | 102 | `load_nani_meshes<closure0>`：打开+deserialize `assets/nani_meshes.ron`，注册 nani 网格并建文件 watcher（`Failed to watch file`）。 | **O** | decomp-0x1415f7190.txt |
| `country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>` | 0x1415f5b60 | 102 | `load_nani_meshes<closure1>`：reload 路径变体（同 RON 解析 + 图集重建）。 | **O** | decomp-0x1415f5b60.txt |
| `country_core::startup::load_mesh_atlases::reload_mesh_atlases` | 0x1412a0820 | 277 | `reload_mesh_atlases`：重建图集（销毁旧 part、重新生成并注册），含 `load_prefabs.rs` 跨模块引用；反转字符串常量疑为调试/计数标记。 | **O** | decomp-0x1412a0820.txt |

## 内联/无独立符号函数

- `country_core::startup::load_mesh_atlases::generate_roof_floor_meshes`（行 159）— **I**：`generate_roof_floor_meshes` — 屋顶/地板网格生成器（内联）。
- `country_core::startup::load_mesh_atlases::generate_curved_halftimber_meshes`（行 197）— **I**：`generate_curved_halftimber_meshes` — 弧形半木构网格生成器（内联）。
- `country_core::startup::load_mesh_atlases::generate_plaster_mesh`（行 234）— **I**：`generate_plaster_mesh` — plaster 网格生成器（内联）。
- `country_core::startup::load_mesh_atlases::generate_stairs_floor_mesh`（行 255）— **I**：`generate_stairs_floor_mesh` — 楼梯地板网格生成器（内联）。

## 派生胶水

- 10 项：closure$0, closure$1, default, deserialize, visit_enum, visit_map, visit_str 等（derive 直接复用，不计业务）。

## 实现方式（算法级，2026-10-15 深挖）

- **load_mesh_atlases 0x14129f400（O）** 顺序：
  1. 旧 parts 处理（FUN_140a37760）
  2. load_nani_meshes<closure0>（FUN_1415f7190）：'assets/nani_meshes.ron'（0x142c7d28b, 22）
     → 读（panic 'open nani_meshes.ron' 行 20）→ RON deserialize（panic 'deserialize nani_meshes.ron' 行 27）
     → 每 entry 注册 FUN_140a371d0(reg, key&0xff, name, len, mesh) + watcher（FUN_14091d540）
  3. 程序化 parts（FUN_140a371d0 注册，key 常量）：
     - `stone_floor_`（0x142c2a380）：grid_gen_cw(0,1,…) 'grid cw'（0x142aaf2ea）→ 属性 'Vertex_Position'(15)（FUN_140a34c90）→ **key 9**
     - `timber_`（0x142c2a398）：grid_gen_ccw(0,6,…) 'grid ccw'（0x142aaf3a0）→ **key 0x18=24**
     - `plaster`（0x142c2a3a8, 7）：32 quad 手写索引 {i|0x10000, i, i+1, i+1, i+0x10001, i|0x10000} → **key 0x13=19**
     - `stairs_floor`+序号（0x142c2a450, 12 + 1 字符序号 0x142c2a470）：**5 chunks（三角数 4→8→16→32→64 翻倍，<0x41 停）**，每 chunk 索引 {3n,3n+1,3n+2} → **key 0x1a=26**
     - `smoke`（0x142c2a35c, 5）：grid_gen_cw(0,0xc,0x32,0xc) → **key 0x15=21**
   > **key 语义（round3 闭合，O）**：5 个 key = `GpuWorldSubsetKind`（27 变体 u8）判别值，非哈希/任意索引——
   > stone_floor_=9=RoofFloor、timber_=0x18=24=CurvedHalfTimber、plaster=0x13=19=Plaster、
   > stairs_floor=0x1a=26=StairsFloor、smoke=0x15=21=Smoke。FUN_140a371d0 按 key 注册进
   > `MeshAtlasLibrary.world_subsets: HashMap<GpuWorldSubsetKind, WorldSubsetMeshAtlas>`；消费端
   > nani 各 `nani_update_*` 系统 `owned_subset_mut(kind)` 同 key 索引（详见 `input-geometry/CLOSURE-ROUND3.md` §5）。
  4. FUN_140ae6b90 收尾 hook（与 reload 同）
- **reload_mesh_atlases 0x1412a0820（49KB，O）**：锁守卫 → 迭代旧 parts（0x98-stride 条目，FUN_1415265f0 逐个销毁）→ 重新生成（FUN_140a37760）→ load_nani_meshes<closure1>（FUN_1415f5b60）→ changed_world_subsets 收集（0x38-stride 条目 push，grow FUN_1406319d0）→ FUN_140ae6b90。含 load_prefabs.rs 行 43 unwrap 引用（0x142c2aae0/0x142c2a048）。
