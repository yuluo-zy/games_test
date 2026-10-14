# startup/load_prefabs.rs

> 文件作用：**预制体（prefab）入库：网格资产生产者。读 `assets/prefabs` 目录，经 `load_json_as_mesh` 处理顶点属性（appear_pos/prim_center/prim_normals/Vertex_Normal/Vertex_Position）注册进 mesh 库；含 PrefabWatcher（热重载）与插件 build。**

函数总计 27：业务 7 / 内联无独立符号 5 / 派生胶水 15

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::load_prefabs::reload_prefabs` | 0x1412a1a60 | 39 | `reload_prefabs`：重载触发器 — Mutex 守卫（LOCK/UNLOCK +0x10 标志）+ 重入检查，触发重新载入。 | **O** | decomp-0x1412a1a60.txt |
| `country_core::startup::load_prefabs::load_prefabs_impl` | 0x1412a1df0 | 61 | **核心生产者** `load_prefabs_impl`：枚举 `assets/prefabs`（0xe），对每个 prefab 用 mesh 属性名查找（FUN_1405b1a70）提取 `appear_pos`/`idappear_pos`/`prim_center`/`prim_normals`/`Vertex_Normal`/`Vertex_Position{stone_floor_}`，`Unknown mesh code processor`/`Sholud not exist` panic 分支，注册进 mesh 库（load_json_as_mesh 消费链）。 | **O** | decomp-0x1412a1df0.txt |
| `country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$1>` | 0x140a08680 | 267 | `filter_mesh_by_vertex_color<closure1>`：按顶点颜色过滤网格（遍历 float 顶点数据，判定 vertex color 存在性）。 | **O** | decomp-0x140a08680.txt |
| `country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$2>` | 0x140a08d30 | 267 | `filter_mesh_by_vertex_color<closure2>`：同族第二实例。 | **O** | decomp-0x140a08d30.txt |
| `country_core::startup::load_prefabs::impl$0::build` | 0x1408e4b00 | 354 | `impl$0::build`（PrefabWatcher 插件 build）：查 TypeId 注册系统（缺失时 FUN_14150c420/FUN_141519680 添加），创建 PrefabWatcher（0x1412a8e00），注册进 app。 | **O** | decomp-0x1408e4b00.txt |
| `country_core::startup::load_prefabs::PrefabWatcher::new` | 0x1412a8e00 | 387 | `PrefabWatcher::new`：创建 notify 文件 watcher（debounce 100,000,000 ns）+ channel；失败 panic。 | **O** | decomp-0x1412a8e00.txt |
| `country_core::startup::load_prefabs::PrefabWatcher::watch_atlas` | 0x1412a8fa0 | 417 | `PrefabWatcher::watch_atlas`：Arc clone + 注册 atlas 路径监听（FUN_14091d540），失败 panic `Failed to watch file {path:?}!`。 | **O** | decomp-0x1412a8fa0.txt |

## 内联/无独立符号函数

- `country_core::startup::load_prefabs::load_prefabs`（行 32）— **I**：`load_prefabs` — 启动入口包装（内联）。
- `country_core::startup::load_prefabs::load_prefab_mesh`（行 187）— **I**：`load_prefab_mesh` — 单个 prefab 网格载入（内联）。
- `country_core::startup::load_prefabs::recalculate_canopy_quad_attribs`（行 290）— **I**：`recalculate_canopy_quad_attribs` — 树冠 quad 属性重算（内联）。
- `country_core::startup::load_prefabs::PrefabWatcher::watch_prefab`（行 400）— **I**：`watch_prefab` — 监听单个 prefab 路径（内联）。
- `country_core::startup::load_prefabs::PrefabWatcher::changed_world_subsets`（行 435）— **I**：`changed_world_subsets` — 变更的世界子集收集（内联）。

## 派生胶水

- 15 项：closure$0, closure$1, closure$2, closure$3, closure$4, closure$5 等（derive 直接复用，不计业务）。
