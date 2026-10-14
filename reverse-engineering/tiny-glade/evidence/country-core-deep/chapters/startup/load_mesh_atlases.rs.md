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
