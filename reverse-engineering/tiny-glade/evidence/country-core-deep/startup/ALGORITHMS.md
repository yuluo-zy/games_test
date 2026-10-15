# startup 模块算法级总览（ALGORITHMS.md，2026-10-15）

范围：`country_core::startup/**` 全部 35 章升级为算法级描述。证据：expansion-b2 decomp +
新增定点反编译（decomp/ 目录，dbid tg_startup_a1）+ rne-read 串/常量 + B skeleton PDB 内联注解。
所有函数地址 = B patched EXE。判定：O = 机器码/反编译证实；I = 按名/消费方推断；U = 未定。

---

## 1. 启动顺序图（谁先谁后）

### 1.1 注册骨架：`add_game_startup_systems`（GameStartupPlugin::build，0x140941110，行 107）

```
FUN_140a97110()                       # 构建 28-系统 tuple（0x1c0B = 28×16B）
  ├─ 含 AssetTextureLibrary::create_preset_textures（串 0x142adaf38，91 字符）
  ├─ 4 个子组构建器：FUN_140ce4b10 / FUN_140a2bf40 / FUN_1415f4750 / FUN_1409c8600
  └─ 0x1c0/8 = 56 qword = 28 × {fn_ptr, cfg}
FUN_140b67620(app, tuple)             # App::add_systems(Startup, …)（bevy_app-0.16.0 app.rs:301，PDB inline）
FUN_141740a20(tuple)                  # 子插件 build 链
FUN_141452870(&list, tuple)
FUN_140b66830(app, 1, &list)          # add_systems（state=1 组）
FUN_140b3f550(tuple)
FUN_140b66480(app, 1, tuple)          # add_systems（state=1 组）
FUN_140b67b50(app)                    # 插件注册
（内联）add_audio_startup_systems      # systems/audio/mod.rs:144（PDB inline_direct，注册音频启动系统）
（内联）closure$0（行 164）            # 引用 ScheduledTerrainReadback::on_terrain_modified → 排程 wiring
尾部：Hash 0x8e1bd251962ba784 查表（FUN_1400dd6f0），缺 → 格式化 panic（0x142af2cc0/0x142af2df0，行 39）
```

要点：启动系统经 **4 次 add_systems** 注册（1 个 28 系统 tuple + 2 个 state=1 组 + 音频组）；
`chain_ignore_deferred` + `in_set_inner` 排程（reconstruction mod.rs 注）。28 系统逐名 = U（Bevy
泛型 tuple 不暴露名字；PDB 仅证 Audio 组 + create_preset_textures）。

### 1.2 数据依赖顺序（每函数内部，O）

```
load_game_session (0x14166e7d0)          # 最先：会话身份 → 起始 build 选择
  ├─ 读 SessionLoadAttempt（*param_1+0x10 空 → theme=3；否则 attempt 字段 → theme byte）
  ├─ 主题 jump-table 分发（0x142c84d24 表，uVar20 = niche ^ 0x8000…；case0 → unreachable panic）
  ├─ case 1/2：继续会话 → 'Latest game session '（有效，FUN_140a9fa80 判真）或
  │   'Continuing game session ' → push 会话 Vec（cap-first grow FUN_140562870，elem 8B，计数 +0x40）
  │   → 拼 `<会话目录>/history.json`
  └─ 新会话：'Starting a new game session' → FUN_140c102e0 生成会话 id → 拼路径
      → create_dir（'Creating folder for game session' panic，行 32）
      → 写 meta.json（'Writing new session meta.json' panic，行 29）
      → push 会话 Vec → 起始 build：demo flag(0x143634d90) ? 'assets/starting-builds/demo'(0x142c849cb)
        : 'assets/starting-builds/full'(0x142c849b0，len 0x1b=27)
      → 内联 copy_dir_all（walk 目录；条目名 "intro"(5 字符,0x72746e69+o) 跳过；排序后复制）
      → theme 写入 meta（'summer/olden/flowery/autumn/winter' 小写串表 0x142c84cc0 区）

load_mesh_atlases (0x14129f400)           # 图集生产者
  ├─ 1. 旧 parts 处理（FUN_140a37760）
  ├─ 2. load_nani_meshes<closure0>（FUN_1415f7190）— assets/nani_meshes.ron 先载
  ├─ 3. 程序化 parts（顺序）：stone_floor_ → timber_ → plaster → stairs_floor×5 → smoke
  └─ 4. FUN_140ae6b90 收尾 hook

load_prefabs_impl (0x1412a1df0)           # 网格资产生产者（依赖 nani/图集已建）
  ├─ assets/prefabs read_dir（.unwrap() 行 43）
  ├─ 每条目 → 'assets/meshes/{name}.json' → 读 → 解析（内联 load_json_as_mesh）
  ├─ 属性提取（'id'/'appear_pos'/'age'/'prim_center'/'Vertex_Position'/'Vertex_Normal'）
  ├─ mesh_code 处理器分发（Unknown → panic）→ filter_mesh_by_vertex_color
  └─ 库 HashMap 注册（'no entry found for key' 行 22）+ watcher 注册

startup_terrain (0x140926bf0)             # 依赖 prefabs 网格库可写
  ├─ grid_gen_cw(0,90,90,90) 90×90 地形网格
  └─ 8 网格 + 6 shader（嵌套 Option 链，任一失败整体中止）→ 每对压 spawn 记录（fn-ptr + mesh + shader + scale 1.0）

init_clearing (0x140a78aa0)               # 依赖主题资产
  ├─ clearing 网格（名 + 文件两式）
  ├─ 直接调用 startup_flowery (0x141553120) + startup_olden (0x1415539d0)  ← 跨模块关键边
  └─ deserialize trees.json

其他单资产生产者（startup_sheep/walls/water/…）各自独立，均在 28 系统 tuple 内。
reload 链（热重载，与启动并行）：reload_prefabs (0x1412a1a60) → load_prefabs_impl；
reload_mesh_atlases (0x1412a0820) → load_nani_meshes<closure1> (0x1415f5b60)。
```

---

## 2. 入库链（输入资产 → 解析 → 注册 → 可消费）

### 2.1 mesh 入库链（共享 loader，O）
```
load_mesh_into_library (0x140b1aca0，87 调用方)
  = MeshWatch::watch(name) + AssetMeshLibrary::add(name, 0x90B mesh payload)（reconstruction 注 + 调用面）
文件式变体 FUN_140b0eeb0（json 路径入参，如 'assets/data/rocky_terrain.json'/'assets/meshes/…json'）
  = 打开 + 解析 + 注册（多数 startup 网格走此）
```

### 2.2 shader 入库链（共享 loader，O）
```
load_shader_into_library<&str> (0x140b3fe40，97 调用方) / <String> (0x140b40210)
  = clone name + compile_shader_variants() + AssetShaderLibrary::add_raster()
load_shader_into_library_replace_previous (0x140b405e0) = 同上，replace 语义
```

### 2.3 prefab 链（load_prefabs.rs，O）
```
assets/prefabs/（read_dir，0xe = 名字符串长度 "assets/prefabs"，非条目数）
  → 每条目名 → format!("assets/meshes/{name}.json")（前缀串 0x142c2a8e0）
  → File 读（FUN_140d76ed0 + FUN_140b0fe60，unwrap 行 43）
  → 内联 load_json_as_mesh（render/mesh.rs）：
      Mesh::new / is_valid / vertex_count / has_attribute（PDB inline）+ Inheritable::as_concrete（asset::inheritable reflect）
      → 属性提取 FUN_1405b1a70(mesh, name, len)（hash + hashbrown probe）：
        'id'(2,0x142c2a850) 'appear_pos'(10,0x142c2a852) 'age'(3,0x142c2a878)
        'prim_center'(0xb,0x142c2a950) 'Vertex_Position'(0xf,0x142c2a361) 'Vertex_Normal'(13,0x142c2a978)
        —— 类型判别检查：prim_center/Vertex_Normal 要求 type==3（dword @entry-0x20），不符
        → panic 'incorrect attribute type'（render/mesh.rs）
        —— 顶点数据按 12B（Vec3）元素拷入 mesh 属性 Vec（FUN_1412e7c80；grow FUN_1416a83a0）
  → MeshProcessorConfig 分发：已知 mesh_code → 处理；未知 → panic 'Unknown mesh code processor {code:?}'
    （0x142c2a828 尾随空格；格式化参数 &ppauStack_528）
    Option 缺失分支 → panic 'Sholud not exist'（0x142c2a928/938）
  → filter_mesh_by_vertex_color（closure 1/2：0x140a08680 / 0x140a08d30；串 'Vertex_Color' 0x142ac6590 / 'No ' 0x142ac69e0）
  → 注册：库 HashMap 索引写（'no entry found for key' panic 行 22，0x142c29ee8/0x142c2a960）
  → PrefabWatcher::watch_prefab（'Failed to watch file {path:?}!' 0x142c2aa90）
可消费：render/nani mesh_atlas / 建筑系统经 PrefabLibrary::iter（PDB inline）
```

### 2.4 网格图集链（load_mesh_atlases.rs，O）
```
A. nani 链：'assets/nani_meshes.ron'（0x142c7d28b，22）
   → 读（'open nani_meshes.ron' panic 行 20）→ RON deserialize（'deserialize nani_meshes.ron' panic 行 27）
   → 每 entry：FUN_140a371d0(reg, key&0xff, name, len, mesh) 注册
   → watcher 注册（FUN_14091d540）
B. 程序化 parts（FUN_140a371d0 注册，key 为字节常量）：
   | part 名 | 生成算法 | 索引模式 | key |
   |---|---|---|---|
   | 'stone_floor_' (0x142c2a380) | grid_gen_cw(0,1,…,1) 'grid cw'(0x142aaf2ea) | 6 idx/quad | 9 |
   | 'timber_' (0x142c2a398) | grid_gen_ccw(0,6,…,6) 'grid ccw'(0x142aaf3a0) | 6 idx/quad | 0x18=24 |
   | 'plaster' (0x142c2a3a8,7) | 32 quad 手写索引 | {i|0x10000, i, i+1, i+1, i+0x10001, i|0x10000} | 0x13=19 |
   | 'stairs_floor'+序号 (0x142c2a450,12 + 1 字符序号 0x142c2a470) | 5 chunks：三角数 4→8→16→32→64（翻倍，<0x41 停） | {3n,3n+1,3n+2} | 0x1a=26 |
   | 'smoke' (0x142c2a35c,5) | grid_gen_cw(0,0xc,0x32,0xc) | 6 idx/quad | 0x15=21 |
   → 属性 'Vertex_Position'(15) 经 FUN_140a34c90 挂到 parts
C. FUN_140ae6b90 收尾（同 reload）
可消费：渲染 pass 网格输入（load_json_as_mesh 68 调用方上游）
```

### 2.5 会话链（startup_session.rs，O）→ 见 §1.2；复制链 copy_dir_all (0x141740c00)
```
递归 walk（FUN_140d59e80 next / FUN_140d59f10 path / FUN_140d62e10 is_dir / FUN_140d61e00 打开）
→ 条目 Vec（elem 4×qword，grow FUN_140562c30）
→ "intro" 子目录跳过（名 5 字符且 == "intro"）
→ 排序（FUN_1413320d0 / 大表 FUN_140b4c390）→ 逐项复制
```

### 2.6 主题链（glade_themes/* + clearing + glade_*，O）
```
flowery (0x141553120)：2 名网格 + 2 文件网格（meadow_lowpoly_flowers、plants/lavender.json）
  + 2 shader（FUN_140937aa0：'billboard_plant_shader' 0x142c6c862、'meadow_flower' 0x142c6c8f3）
  + 树配置 trees/{rocks,clearing_edge_rocks,stumps,clearing_edge_tree_stumps}.json + 2 记录
olden (0x1415539d0)：trees/{rocks,clearing_edge_rocks,stumps,clearing_edge_tree_stumps,logs,
  clearing_edge_tree_logs}.json（串 0x142c6c918 区，6 配置）+ 'assets/meshes/clover.json' + 2 记录
init_clearing (0x140a78aa0)：clearing 网格 + 2 文件网格（'assets/meshes/ice_cracks.json'、
  'assets/meshes/distant_billboard.json' 0x142ad6e71）+ 直接调 flowery/olden + deserialize trees.json
  （'deserialize trees' 0x142ad6e18）+ 记录（FUN_1412dd460 等）
uninit_clearing (0x140a7a5b0)：恢复 FMOD 'snapshot:/Loading'（0x142ad6ef0）
glade 音频（0x1408be360）：theme byte@+0x58 ×8 索引 → GladeAudio::load (0x1408bf460)
  → 'Reading {path}/audio.json' 日志（0x142aa0d44）→ 4 事件 event:/Music/Halloween/
  {Single Leaf,Tiny Olden Glade,Forgotten Ruins,Enchanted Forest}（串 0x142aa0c33 区）
  → FUN_140a54440 事件句柄 ×2 + 参数 'GladeTypeIsHalloween'(9,0x142aa0ce8)/'IsHalloween'(0xb,0x142aa0cf1)
glade 分级（0x1408bf9c0）：theme 路径（'assets/gladestate' 0x142aef51a 族）+ FUN_141405ec0 注册
glade ToD/天气（0x140ae7810）：theme 路径 + 'assets/tod/default.json'（0x142a99540,23）→ FUN_1414051e0 载入
雪粒子（0x140ae7d90）：RNG 状态（FUN_140d4d740/FUN_140d64e00）+ 记录 FUN_1412fa8e0 + assert panic
startup_time_of_day (0x14089d6f0) = 与 glade ToD 共用：FUN_140d59140('assets/tod/default.json') → FUN_1414051e0
```

### 2.7 纹理链（load_textures.rs，O）
```
should_load (0x1409e6950) 前缀谓词（串 0x142ac4780 区，顺序）：
  logo / brush / loading / menu/pounce_light_logo / menu/heart / menu/noknok_logo / menu/fmod_logo
  / blue_noise / default_loading_background / default_loading_background_demo
  末两项受 demo flag 0x143634d90 门控
load_glade_texture_mappings (0x1409e6b60)：读 'textures.json' → 语义键 glade:{ivy_leaf_color,
  canopy_flowers_depth, leaf_particle_colors, terrain_texture, grass_patch, lilypad_flower,
  canopy_color, bark_color, bark_color_naked} → 失败 panic 'Couldn't load '（0x142ac47xx 区）
```

### 2.8 资产注册记录（spawn 记录表，startup 共用机制，O）
```
每网格+shader 对 → FUN_140d146a0(&list, ctx) 取表 → 压 {spawn_fn_ptr(每类型不同), mesh, shader,
scale 1.0(0x3f800000), …}（cap-first Vec，grow FUN_1428a9a60）
spawn fn-ptr 族（类型标识；2026-10-15 复验模式）：FUN_1412d7aa0(0x10B) 1412d7c70(0x78B) 1412d8b00(0x18B) 1412dc9f0(0x88B)
  1412da440 1412d9790 1412dbb00 1412db2a0 1412dd190(0x88B) 1412d6640 1412d6d80 1412d9de0 1412dabb0
  1412d9400 1412d9270 1412dd460 1412dc470 1412fa8e0 1412fa010 1412e6ad0 1412e68e0 1412dfbb0 1412e23e0 1412e0fa0 …
  实测（capstone）：每闭包 `add qword [r8], 增量`（表长递增；FUN_1412d7aa0 增 8 / 1412d7c70 增 0x70 /
  1412d8b00 增 0x10 / 1412dc9f0 增 0x80——注记 (0x..B) = 增量+8，即含 8B fn-ptr 槽）→ 从 ctx（rcx 各偏移
  0x40/0x50/0x60/0x70 等）构建 spawn 记录 → 调 mesh-spawn helper（0x14205e980/0x140cf81b0/0x142058f60 等）。
  每 mesh 类型一个，类型身份按消费端/上下文 I 级。
消费方：渲染/backend spawn 系统（render-loop 域）
```

---

## 3. panic/日志串清单（地址 = B EXE）

| 串 | 地址（fat=胖指针） | 出处 | 行 |
|---|---|---|---|
| `Unknown mesh code processor ` | 0x142c2a828 (fat) | load_prefabs_impl（格式化，{code:?}） | load_prefabs.rs |
| `Sholud not exist` | 0x142c2a928 (fat) | load_prefabs_impl | load_prefabs.rs |
| `incorrect attribute type` | 0x142ac7018 (fat)；file 0x142ac7040 | load_prefabs_impl 内联 render/mesh.rs | mesh.rs |
| `Failed to watch file {path:?}!` | 0x142c2aa90 (cstr) | PrefabWatcher::watch_atlas | load_prefabs.rs |
| `no entry found for key` | 0x142c29ee8 (cstr) | load_prefabs_impl 注册 | :22 |
| `called \`Result::unwrap()\` on an \`Err\` value` | 0x142c2a018 / 0x142aae510 / 0x142ac6360 / 0x142ad0600 / 0x142aa03b8 | 各 loader | 多为 :43 |
| `open nani_meshes.ron` | 0x142c7d2a1 (cstr) | load_nani_meshes | :20 |
| `deserialize nani_meshes.ron` | 0x142c7d308 (cstr) | load_nani_meshes | :27 |
| `Creating folder for game session` | 0x142c84940 (cstr) | load_game_session | :32 |
| `Writing new session meta.json` | 0x142c84978 (cstr) | load_game_session | :29 |
| `internal error: entered unreachable code` | 0x142c84625 (cstr) | load_game_session 主题 case0 | :40 |
| `should not have been consumed earlier` | 0x142c84830 | load_game_session | |
| `Failed to load session meta. Using default. Error: ` | 0x142c84c40 | load_game_session | |
| `Error deleting glade: ` | 0x142c84cc0 区 | session（glade 删除） | |
| `deserialize trees` | 0x142ad6e18 | clearing / spawn_preplaced_trees | |
| `deserialize plants` | 0x142a96308 | init_meadow | :18 |
| `Couldn't load ` | 0x142ac47xx 区 | load_glade_texture_mappings | |
| `Continuing game session ` | 0x142c84730 (fat)；0x142c846f0 (cstr+模块名) | load_game_session 日志 | |
| `Latest game session ` | 0x142c846b8 (fat) | load_game_session 日志 | |
| `Loading a pre-parsed game session` | 0x142c84798 (fat) | load_game_session 日志 | |
| `Starting a new game session` | 0x142c84918 (fat) | load_game_session 日志 | |
| `snapshot:/Loading` | 0x142ad6ef0 | uninit_clearing（FMOD） | |

常量资产串：`assets/prefabs` 0x142c2a590（14）；`assets/meshes/` 0x142c2a8e0；
`assets/colors.json` 0x142a920a6；`assets/tod/default.json` 0x142a99540（23）；
`assets/gladestate` 0x142aef51a；`assets/starting-builds/full` 0x142c849b0（27）/`demo` 0x142c849cb；
`assets/nani_meshes.ron` 0x142c7d28b（22）；`history.json` 0x142c845e7（12）；`snapshots` 0x142c848b8；
主题小写名 `summer/olden/flowery/autumn/winter` 0x142c84cc0 区；`grid cw` 0x142aaf2ea / `grid ccw` 0x142aaf3a0；
`Vertex_Color` 0x142ac6590；`No ` 0x142ac69e0；`Pitch` 0x142ac6c48（sheep FMOD 参数）；
`GladeTypeIsHalloween` 0x142aa0ce8 / `IsHalloween` 0x142aa0cf1；
`event:/Music/Halloween/*` 0x142aa0c33 区；`event:/SFX/Sheep/Baah`（sheep）；`event:/SFX/Butterfly/Shoosh`（critters）。

---

## 4. 跨模块依赖

| 边 | 证据 |
|---|---|
| init_clearing → startup_flowery/startup_olden | 直接调用 0x141553120 / 0x1415539d0（init_clearing decomp） |
| load_mesh_atlases → load_nani_meshes → grid_gen_cw/ccw | FUN_1415f7190 / FUN_140928100(0,1) / 140928350(0,6) |
| load_prefabs_impl → render/mesh.rs load_json_as_mesh（内联） | Mesh::new/is_valid/has_attribute/vertex_count PDB inline + 'incorrect attribute type' 串 |
| load_prefabs_impl → filter_mesh_by_vertex_color | 直接调用 0x140a08680 / 0x140a08d30 |
| load_prefabs_impl → startup_terrain::generate_grid_cw（内联） | PDB inline（canopy quad 重算） |
| reload_prefabs → load_prefabs_impl；reload_mesh_atlases → load_nani_meshes<closure1> | 调用面 |
| load_game_session → copy_dir_all；→ session.rs 资源（meta/history/snapshots/screenshot.jpg） | 串 + 调用面 |
| startup_time_of_day / load_glade_tod_and_weather 共用 ToD 载入器 | FUN_140d59140 + FUN_1414051e0 同调用面 |
| startup 资产 → 渲染/backend spawn 记录表 | FUN_140d146a0 + FUN_141564d30 三件套（§2.8） |
| startup_terrain grid_gen → 地形渲染网格；atlas parts → nani mesh_atlas | 调用面 |
| startup_nani/reload_nani ↔ render/nani/gpu_world.rs | 串 0x142aba4d8 + unwrap :43 |
| add_game_startup_systems → systems/audio::add_audio_startup_systems | PDB inline（audio/mod.rs:144） |

---

## 5. 共享 helper 身份表（调用面 + 上下文判定，I/O 标注）

| helper | 判定 | 说明 |
|---|---|---|
| FUN_140b1aca0 | O | load_mesh_into_library（名+数据；87 调用方） |
| FUN_140b3fe40 / FUN_140b40210 / FUN_140b405e0 | O | load_shader_into_library<&str/String>/replace（97 调用方） |
| FUN_140b0eeb0 | I | 文件式网格载入（json 路径 → 解析 → 注册） |
| FUN_140937aa0 | I | shader 载入变体（名+len 次序不同，terrain/sheep 主题用） |
| FUN_1405b1a70(obj, name, len) | O | mesh 属性查找（hash + hashbrown probe；len = 名长） |
| FUN_140a34c90(&attr, name, len, data) | I | 属性挂载（add_attribute） |
| FUN_140a371d0(reg, key, name, len, mesh) | O | 图集 part 注册（key 为 u8 常量） |
| FUN_140d146a0(&list, ctx) | I | spawn 记录表获取 |
| FUN_1428a9a60 / FUN_1416a83a0 / FUN_1406319d0 / FUN_140562870 / FUN_140562c30 | O | cap-first Vec grow（元素 4B/4B/8B/8B/16B 等） |
| FUN_141564d30 等 24+ | I（族模式 O） | spawn 记录 fn-ptr（每 mesh 类型一个）；§2.8 已复验：`add qword [r8], 增量` + ctx 建记录 + spawn helper（个别类型身份按消费端待定） |
| FUN_140d76ed0(path) | I | 文件系统打开/读取（目录与文件两语境） |
| FUN_140d62810(out, h, base, str, len) | O | 路径 append/拼接 |
| FUN_140d79f90 / FUN_140d61b00 | O | String→PathBuf / PathBuf::from(str,len) |
| FUN_140d59140(&out, str, len) | I | gladestate/tod 资源路径载入 |
| FUN_1414051e0 / FUN_141405ec0 | I | ToD / 分级 配置载入注册 |
| FUN_140ae6b90 | I | 图集重建收尾 hook（load/reload 均调） |
| FUN_14091d540 | I | notify watcher 路径注册（watch_atlas 调用面） |
| 全局 0x14362c3a8 | O | 日志级别门控 `if (2 < …)` = info 级 |
| 全局 0x143634d90 | O | demo 模式 flag（should_load/会话 build/photomode FOV 共用） |

---

## 6. 纠错 / 新证（相对章节一句话版）

1. `load_prefabs_impl` 的 "0xe" = **"assets/prefabs" 名字符串长度（14）**，不是条目数。
2. 属性查找第三参 = **名字符串长度**（"id"=2、appear_pos=10、age=3、prim_center=0xb、
   Vertex_Position=0xf、Vertex_Normal=0xd）；`UNK_142c2a850` 因串合并显示 'idappear_pos'，
   实为 'id'(2) + 'appear_pos'(10) 两串合并——**真实查找名是 "id" 而非 "idappear_pos"**。
3. 'incorrect attribute type' panic 归属 **render/mesh.rs**（内联 load_json_as_mesh），不是
   load_prefabs.rs；prim_center/Vertex_Normal 要求属性类型判别 == 3。
4. load_game_session 的 theme 值在磁盘为**小写**（summer/olden/flowery/autumn/winter）；
   'Starting a new game session' 日志实际拼写 = 'Starting a new game session'（fat 串）。
5. startup_terrain 网格库键名含 `stairs_step_mesh`/`path_pebble_mesh`/`stairs_pebble_mesh`
   （16/16/18 字符），rocky_terrain 走文件式（assets/data/rocky_terrain.json）。
6. spawn_brush_previews 实际 24 次网格载入（13 名 + 11 文件），非章节"~13"。
7. 图集 parts key 常量（**本轮闭合 = `GpuWorldSubsetKind` 判别值，非哈希**，详见 `CLOSURE-ROUND3.md` §5）：
   stone_floor_=9（RoofFloor，nani_update_stone_floor kind 9）、timber_=0x18=24（CurvedHalfTimber）、
   plaster=0x13=19（Plaster）、stairs_floor=0x1a=26（StairsFloor）、smoke=0x15=21（Smoke）。
   证据：FUN_140a371d0 按 key 字节注册进 `MeshAtlasLibrary.world_subsets: HashMap<GpuWorldSubsetKind,
   WorldSubsetMeshAtlas>`；消费端 nani 各 `nani_update_*` 系统 `owned_subset_mut(kind)` 同 key 索引
   （reconstruction render/nani/gpu_world.rs 枚举判别 + systems.rs kind 立即数均 O）。
8. on_enable_photomode 机器码（2026-10-15 复验，rne-read+capstone）：demo flag(0x143634d90) ?
   FOV=1.0 (0x142925904=0x3f800000) : FOV=3.0 (0x1429cd078=0x40400000)；字段 [0]=fov、[4]=-1、
   [8]=0、[0xa]=0、[0xc]=0。邻接第二实例（纠错：原记 0x140870590 应为 0x1408705b0）写
   word[0xc]=0x100、qword[4]=-1（字段宽不同，另一结构/impl）。证据 startup/decomp/decomp-0x140870570.txt。
9. reload_prefabs = Mutex 守卫（@+0x30 的 +0x10 标志，重入跳过）+ 调 load_prefabs_impl。
10. bevy_ecs-0.16.0 证实（function_system.rs 串 0x142ada938 等，'bevy_ecs-0.16.0/src/system/function_system.rs'）。

## 7. 证据落盘
- 新增定点反编译：`startup/decomp/decomp-0x140a97110.txt`、`decomp-0x140b67620.txt`、
  `decomp-0x140b66480.txt`、`decomp-0x140b66830.txt`（dbid tg_startup_a1，B 只读会话）。
- 新增机器码手解（替代 tg_b2 NotFound）：`startup/decomp/decomp-0x140a14620.txt`（clearing
  visit_str）、`decomp-0x140870570.txt`（on_enable_photomode）、`decomp-0x140b1ac90.txt`
  （impl$1::build）——2026-10-15，rne-read --hex + capstone。
- 既有证据：expansion-b2/decomp/decomp-0x1412a1df0.txt 等 55 份 + expansion-b2/decomp-strings.txt。
