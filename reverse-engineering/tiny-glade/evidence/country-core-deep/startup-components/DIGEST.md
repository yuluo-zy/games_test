# startup-components 域 DIGEST（2026-10-14，owner tg_sc1）

域范围：`country_core::components/**`（mod / drawable / transient_mesh）+ `country_core::startup/**`
（load_* / spawn_* / startup_* / glade_themes）。MODULE_GAP_ANALYSIS §1 判定的"深度证据 0%"最高优先缺口。
本域结论：**组件 = Bevy 注册胶水 + 少量业务系统；startup = 资产库生产者（网格/shader/纹理/音频/主题）**。
证据源：expansion-b1/b2 decomp（138 业务函数全部已有反编译证据，本轮逐一核对 + rne-read 字符串/机器码补证）。

## 1. 模块清单与业务过程数

| 模块 | 业务函数 | 结论 | 说明 |
|---|---|---|---|
| components/mod.rs | 54 | 45× register_required_components = Bevy 注册胶水；9× clone_behavior；1× serde serialize 胶水 | 组件类型清单见 reconstruction mod.rs（WP6） |
| components/drawable.rs | 19 | 13× register_required_components 胶水；4× clone_behavior 胶水；1× From<MeshAtlasPart>；1× **cache_prev_object_transforms 业务系统** | |
| components/transient_mesh.rs | 5 | **drain/deferred_delete/new_with_registry = 真实业务**；register/clone_behavior 胶水 | |
| startup/clearing.rs | 3 | init_clearing / uninit_clearing = 业务；visit_str 胶水 | |
| startup/debug.rs | 1 | debug = 业务（debug 网格载入） | |
| startup/glade_themes/{flowery,olden}.rs | 2 | 主题资产载入 = 业务 | |
| startup/init_color_library.rs | 1 | 业务（colors.json 调色板） | |
| startup/init_photomode_state.rs | 1 | 业务（PhotonModeState 启用初始化） | |
| startup/load_audio.rs | 1 | 业务（audio 资源容器） | |
| startup/load_glade_audio.rs | 2 | 业务（theme 音乐事件） | |
| startup/load_glade_grading.rs | 1 | 业务（theme 分级 LUT） | |
| startup/load_glade_tod_and_weather.rs | 2 | 业务（theme ToD/天气 + 雪粒子） | |
| startup/load_mesh_atlases.rs | 5 | 业务（图集生成 + nani RON + reload） | |
| startup/load_prefabs.rs | 7 | 业务（**网格资产生产者** + watcher） | |
| startup/load_shaders.rs | 2 | 业务（UI/主渲染 shader 入库清单） | |
| startup/load_textures.rs | 3 | 业务（should_load 过滤 + texture mappings） | |
| startup/mod.rs | 6 | 业务（插件注册 + shader/mesh 入库 loader） | |
| startup/panic_reporter_startup.rs | 0 业务 | 1× inline panic hook | |
| startup/spawn_brush_previews.rs | 1 | 业务（brush 预览实体） | |
| startup/spawn_human_ref.rs | 1 | 业务（human_ref_cube 参考） | |
| startup/spawn_preplaced_trees.rs | 1 | 业务（trees.json 预置树） | |
| startup/startup_blit.rs | 1 | 业务（blit shader） | |
| startup/startup_critters.rs | 1 | 业务（butterfly/duck/splash） | |
| startup/startup_deferred_raster.rs | 1 | 业务（deferred shader） | |
| startup/startup_garden_brush.rs | 1 | 业务 | |
| startup/startup_grass.rs | 1 | 业务 | |
| startup/startup_ivy.rs | 1 | 业务 | |
| startup/startup_meadow.rs | 1 | 业务（plants.json） | |
| startup/startup_nani.rs | 2 | 业务（系统入口 + reload_nani） | |
| startup/startup_roof.rs | 1 | 业务 | |
| startup/startup_session.rs | 2 | 业务（**会话启动** + copy_dir） | |
| startup/startup_sheep.rs | 1 | 业务 | |
| startup/startup_sky_and_sun.rs | 1 | 业务 | |
| startup/startup_terrain.rs | 3 | 业务（地形网格 + grid_gen） | |
| startup/startup_time_of_day.rs | 1 | 业务 | |
| startup/startup_walls.rs | 1 | 业务（wall LOD shader + half_cube） | |
| startup/startup_water.rs | 1 | 业务 | |

合计 38 文件 / 138 业务函数全部 O（带证据），23 个内联函数 I（按名/消费方推断）。

## 2. 关键 O 事实

### 组件（components/）
- **register_required_components 家族（~58 个）**：O — Bevy 0.16 required-components 注册胶水。均 18KB
  单态化模板（bevy_ecs component.rs 源码串 + 每 impl 独立组件类型名/类型表），无独立业务逻辑。
  组件类型清单即 reconstruction `mod.rs`/`drawable.rs` 的 struct 集（含 typed gap）。
- **clone_behavior 两族**：
  - 共享默认 0x140002370 = `xor eax,eax; ret`（返回 0 = 无克隆）。mod.rs/drawable/transient_mesh 的 impl$23 共用。
  - 每组件变体（0x14082cac0、0x14084cc00、0x140831030 等）= `lea rdx,[类型表]; mov eax,2; ret`（返回行为=2）。
- **drawable `impl$1::from` 0x14082bd60**：O — `NaniInstanceMeshDesc::from<MeshAtlasPart>`，机器码
  `mov [rcx+4],eax; mov dword[rcx],1` → 判别 1 + 值 0 = `AtlasPart(0)`。与 reconstruction 一致。
- **drawable `impl$2::build` 0x14082bd70 / startup/mod `impl$1::build` 0x140b1ac90**：O — `mov rcx,rdx; jmp` 尾调用共享 build。
- **drawable `cache_prev_object_transforms` 0x14082bd80**：O — **唯一真实组件域系统**。遍历 sparse
  instance buffer（+0xf8 指针 / +0x100 计数 / +0x108 标志；实例行 stride 0xa0、0x48 属性、0x30 变换），
  把当前变换拷贝写入 `PrevTransform`（供 velocity_only/wren）。与渲染 nani 系统族接口。
- **transient_mesh 三函数**：O —
  - `deferred_delete_mesh` 0x140aa8070：锁注册表 Mutex（@+0x0）→ push 句柄进 Vec@+0x8（**cap-first**
    {cap@8,ptr@0x10,len@0x18}，元素 8B，grow FUN_140562870）。
  - `drain_meshes_to_delete` 0x1412e9e70：锁 + 排空，对每个句柄调闭包（闭包内两次删除
    FUN_14092d750 / FUN_1409247a0），清 len。backend `delete_dropped_transient_meshes` 每帧消费。
  - `new_with_registry` 0x140aa8200：入口 = **Arc clone**（`lock inc [ptr]` + 溢出检查）→ 构造组件。
- **components serde 0x14036a4f0**：O — serde_json CompactFormatter 整数序列化（/10000 迭代数字化，UNK_1429c6964 数字表）。

### startup 资产载入（每个 = 一个资产库生产者）
- **load_prefabs_impl 0x1412a1df0（154KB，核心生产者）**：读 `assets/prefabs` 目录，对每个 prefab 用
  mesh 属性名查找（FUN_1405b1a70）提取 `appear_pos` / `idappear_pos` / `prim_center` / `prim_normals` /
  `Vertex_Normal` / `Vertex_Position{stone_floor_}`，panic 分支 `Unknown mesh code processor` / `Sholud not exist`，
  注册进 mesh 库 = **68 调用方 load_json_as_mesh 的消费链源头**。类型：MeshProcessorConfig / PrefabRenderConfig / AudioTriggerConfig。
- **PrefabWatcher**：`new` 0x1412a8e00 = notify watcher（debounce 100ms）+ channel；`watch_atlas` 0x1412a8fa0 =
  Arc clone + 注册路径监听（失败 panic `Failed to watch file {path:?}!`）；`impl$0::build` 0x1408e4b00 = 插件注册。
  `reload_prefabs` 0x1412a1a60 = Mutex 守卫重载触发器。
- **load_mesh_atlases**：0x14129f400 生成图集 parts（`Vertex_Position`+`stone_floor_`、`plaster`/`plaster_strip`、
  `stairs_floor_chunk_`、`smoke`）+ 调 grid_gen_cw/ccw + load_nani_meshes；`load_nani_meshes` 0x1415f7190/0x1415f5b60 =
  打开+deserialize `assets/nani_meshes.ron` 注册 nani 网格 + watcher；`reload_mesh_atlases` 0x1412a0820 = 重建图集。
- **load_textures**：`should_load` 0x1409e6950 = 前缀过滤谓词（logo/brush/loading/menu-*/blue_noise/
  default_loading_background*，末两项受全局标志门控）；`load_glade_texture_mappings` 0x1409e6b60 = 从
  `textures.json` 建 `glade:{ivy_leaf_color,canopy_flowers_depth,leaf_particle_colors,terrain_texture,grass_patch,
  lilypad_flower,canopy_color,bark_color,bark_color_naked}` 语义映射。
- **load_shaders / startup_blit / startup_deferred_raster**：O — 明确 shader 清单（见各章节表）。
- **startup_session**：`load_game_session` 0x14166e7d0（78KB）= 起始 build（`assets/starting-builds/{full,demo}`+intro）、
  创建会话目录、读/写 `meta.json`（theme: Summer/Olden/Flowery/Autumn/Winter）、载入 `history.json`/`snapshots`/`screenshot.jpg`；
  `copy_dir_all` 0x141740c00 = 递归目录复制。
- **startup_terrain**：0x140926bf0 载入 terrain/rocky_terrain/stairs_step/path_pebble/stairs_pebble/billboard_plants 网格
  + 6 shader + spawn 实体；`grid_gen_cw` 0x140928100 / `grid_gen_ccw` 0x140928350 = **递归生成地形 quad 三角索引**
  （cap-first Vec，元素 4B，`grid cw`/`grid ccw` 命名）。
- **clearing**：`init_clearing` 0x140a78aa0 载入 clearing 网格（tree/bush、ice_cracks、distant_billboard）+ shader +
  deserialize `trees.json`；`uninit_clearing` 0x140a7a5b0 恢复 FMOD `snapshot:/Loading`。
- **主题/小动物/羊**：flowery/olden 载主题树配置 + 花；critters 载 butterfly/duck/splash + `event:/SFX/Butterfly/Shoosh`；
  sheep 载 sheep_animation + `event:/SFX/Sheep/Baah`。均为网格+shader+音效三件套。
- **startup/mod.rs**：`add_game_startup_systems` 0x140941110 = GameStartup 插件（注册全部 startup 系统）；
  `load_shader_into_library{<String>,<&str>,replace}` 0x140b40210/0x140b3fe40/0x140b405e0 + `load_mesh_into_library` 0x140b1aca0 =
  所有 startup 共用的 shader/mesh 入库 loader。

## 3. 内联函数（I，按名/消费方推断）
见各章节"内联/无独立符号函数"。共 23 个：Bundle 派生胶水（component_ids/get_components/build）、
generate_{roof_floor,curved_halftimber,plaster,stairs_floor}_meshes、load_prefab_mesh、
recalculate_canopy_quad_attribs、watch_prefab、changed_world_subsets、runtime_check_limits、
create_world_subset、load_session_meta、MaybeExists::get、SessionLoadAttempt::from_session_id、
generate_grid_cw/ccw、once_startup_walls、create_half_cube_mesh、load_default_tod、load_textures、
GladeTextureMappings::create_mappings、reload_color_library、init_photomode_state、panic_reporter_startup。

## 4. panic/日志串清单
| 串 | 地址 | 位置 |
|---|---|---|
| `Failed to watch file {path:?}!` | 0x142c2aa90 | load_prefabs |
| `Unknown mesh code processor ` | 0x142c2a828 | load_prefabs_impl |
| `Sholud not exist` | 0x142c2a928 | load_prefabs_impl |
| `called \`Result::unwrap()\` on an \`Err\` value` | 0x142c2a018 | 多处 |
| `Continuing game session ` | 0x142c846f0 | startup_session |
| `should not have been consumed earlier` | 0x142c84830 | startup_session |
| `Creating folder for game session` | 0x142c84940 | startup_session |
| `Writing new session meta.json` | 0x142c84978 | startup_session |
| `internal error: entered unreachable code` | 0x142c84625 | startup_session |
| `deserialize trees` | 0x142ad6e18 | clearing / spawn_preplaced_trees |
| `deserialize plants` | 0x142a96308 | startup_meadow |
| `open nani_meshes.ron` / `deserialize nani_meshes.ron` | 0x142c7d2a1 / 0x142c7d308 | load_mesh_atlases |
| `Couldn't load ` | 0x142ac4c30 区 | load_textures |
| `incorrect attribute type`（render/mesh.rs） | 0x142ac7018 | load_prefabs_impl |
| `snapshot:/Loading` | 0x142ad6ef0 | uninit_clearing |
| `assets/colors.json` | 0x142a920a6 | init_color_library |
| `assets/gladestate`（session.rs 路径） | 0x142aef51a | load_glade_audio/grading/tod / session |

## 5. 与现有 reconstruction 代码的差异
- **transient_mesh.rs**：reconstruction `TransientMeshRegistry` 写 `pending_delete: Mutex<Vec<u64>>`（Mutex 包 Vec）。
  二进制证据：注册表 = `{ Mutex@+0x0, Vec@+0x8 }`（Mutex 与 Vec 分离，Mutex 只作锁）。行为等价，结构不同 → 建议
  改为 `{ lock: Mutex<()>, pending: Vec<u64> }`（或注明语义等价）。`new_with_registry` 二进制入口为 Arc clone
  （入参 &Arc → 内部 clone），reconstruction 直接 move Arc，签名可对齐为 `&Arc<...>`。
- **drawable.rs `NaniInstanceMeshDesc::from`**：机器码 `AtlasPart(0)` 与 reconstruction 一致（O）。
- **components/mod.rs 组件集**：PDB 行号（至 439）与 reconstruction 行号（202）不对应 — reconstruction 为部分骨架，
  组件清单以 PDB 命名实体 + register_required_components 类型表为准（U 项：每 impl 对应组件名的精确映射未逐条机器码核对）。
- **load_glade_audio**：reconstruction `GladeAudio` 结构未覆盖 4 主题音乐事件表（event:/Music/Halloween/*）→ 契约缺该表。
- **load_textures should_load**：reconstruction 无该过滤谓词 → 契约缺（启动只载入这些前缀纹理）。

## 6. 跨领域线索
- **渲染/GPU**：drawable `cache_prev_object_transforms` ↔ render/nani sparse_instance_buffer（0xa0/0x48/0x30 stride）。
  load_mesh_atlases/load_prefabs 的网格 → render/nani mesh_atlas 消费（load_json_as_mesh 68 调用方）。
  grid_gen_cw/ccw → 地形渲染网格。
- **audio-misc 域**：FMOD 事件串（Music/Halloween、SFX/Sheep/Baah、SFX/Butterfly/Shoosh、snapshot:/Loading）。
- **session-save 域**：startup_session 的 meta.json/history.json/snapshots 与 session.rs 资源接口（0x142aef51a 共享）。
- **ui-input 域**：spawn_brush_previews 的 brush 网格 + Ui2dImage::new 组件。
- **terrain-water 域**：startup_water 网格（water_plane/lilypad/reed）+ startup_terrain（rocky_terrain）。

## 7. 错位/纠错
- PDB `impl$N::register_required_components` 名与行为一致（确为 Bevy 注册），但**体量大**（18KB）——不是简单注册，
  是 Bevy required-components 单态化模板；不可按方法名简化成一行。
- `clone_behavior` 有两个实现族：默认 0x140002370（返回 0）vs 每组件 0x1408xxxx（`mov eax,2; ret`）。PDB 只给方法名，
  机器码区分。**勿把它们当同一函数**。
- drawable `impl$1::from` 地址 0x14082bd60 在 Ghidra 无函数（NotFound），机器码手解确认语义。
- `load_prefabs_impl` 与 `PrefabWatcher` 的反编译落入同一 CU 字符串区（`Failed to watch file` 在 impl 的字符串表中）——
  PDB 函数边界按 CU 聚合，字符串引用不作为单函数归属的唯一依据。
