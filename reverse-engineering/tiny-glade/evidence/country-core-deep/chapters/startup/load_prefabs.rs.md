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

## 实现方式（算法级，2026-10-15 深挖）

- **load_prefabs_impl 0x1412a1df0（154KB，O）** 算法：
  1. `PathBuf::from("assets/prefabs")`（FUN_140d61b00，len 0xe=14 = **字符串长度**，非条目数）
  2. read_dir/open（FUN_140d76ed0 → FUN_14200a1b0 结果检查）；失败 → `.unwrap()` panic 行 43
  3. 每条目：`format!("assets/meshes/{name}.json")`（前缀串 0x142c2a8e0，FUN_1415244d0）
     → 打开（FUN_140d76ed0）→ 读入（FUN_140b0fe60，unwrap 行 43）
  4. 内联 load_json_as_mesh（render/mesh.rs）：Mesh::new/is_valid/has_attribute/vertex_count
     + Inheritable::as_concrete（PDB inline 证实）
  5. 属性查找 FUN_1405b1a70(mesh, name, len)（hash + hashbrown probe；**第三参 = 名字符串长度**）：
     `"id"`(2, 0x142c2a850 串合并显示 idappear_pos — 实为 'id'+'appear_pos' 合并)、
     `appear_pos`(10)、`age`(3)、`prim_center`(0xb)、`Vertex_Position`(0xf)、`Vertex_Normal`(13)；
     prim_center/Vertex_Normal 要求属性类型判别 == 3，不符 → panic 'incorrect attribute type'
     （render/mesh.rs：0x142ac7018 / 0x142ac7040）；顶点数据按 12B(Vec3) 拷入（FUN_1412e7c80）
  6. MeshProcessorConfig 分发：未知 code → panic `Unknown mesh code processor {code:?}`
     （0x142c2a828 尾随空格）；Option 缺失 → panic `Sholud not exist`（0x142c2a928/938）
  7. filter_mesh_by_vertex_color 两 closure（0x140a08680/0x140a08d30；串 'Vertex_Color' 0x142ac6590、'No ' 0x142ac69e0）
  8. 注册：库 HashMap 索引写 → `no entry found for key` panic（0x142c29ee8，行 22，文件 0x142c2a960）
  9. PrefabWatcher::watch_prefab 注册 → `Failed to watch file {path:?}!`（0x142c2aa90）
- **reload_prefabs 0x1412a1a60（O）**：Mutex 守卫（@+0x30 的 +0x10 标志，LOCK/UNLOCK）；
  已置位 → 重入跳过；置位 → 调 load_prefabs_impl（0x1412a1df0 唯一调用点）。
- **PrefabWatcher::new 0x1412a8e00**：notify watcher（debounce 100,000,000 ns）+ channel；
  `watch_atlas` 0x1412a8fa0 = Arc clone + notify watch 注册（FUN_14091d540）。
- 纠错：章节旧述 "idappear_pos 提取" 修正为 **"id"(2)**（串合并假象，见 ALGORITHMS 6.2）。
