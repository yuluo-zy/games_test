# -*- coding: utf-8 -*-
"""Append algorithm-level sections to startup chapter files (in-place upgrade)."""
import io, os

BASE = r'D:\game\reverse-engineering\tiny-glade\evidence\country-core-deep\chapters\startup'

SECTIONS = {}

SECTIONS['load_prefabs.rs.md'] = '''
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
'''

SECTIONS['load_mesh_atlases.rs.md'] = '''
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
  4. FUN_140ae6b90 收尾 hook（与 reload 同）
- **reload_mesh_atlases 0x1412a0820（49KB，O）**：锁守卫 → 迭代旧 parts（0x98-stride 条目，FUN_1415265f0 逐个销毁）→ 重新生成（FUN_140a37760）→ load_nani_meshes<closure1>（FUN_1415f5b60）→ changed_world_subsets 收集（0x38-stride 条目 push，grow FUN_1406319d0）→ FUN_140ae6b90。含 load_prefabs.rs 行 43 unwrap 引用（0x142c2aae0/0x142c2a048）。
'''

SECTIONS['mod.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **add_game_startup_systems 0x140941110（O）** 注册顺序：
  1. FUN_140a97110 构建 28-系统 tuple（0x1c0B = 28x16B；含 `AssetTextureLibrary::create_preset_textures`
     串 0x142adaf38（91 字符）+ 4 子组构建器 FUN_140ce4b10/140a2bf40/1415f4750/1409c8600）
  2. FUN_140b67620 = App::add_systems(Startup, tuple)（bevy_app-0.16.0 app.rs:301 PDB inline）
  3. FUN_141740a20 / FUN_141452870（子插件 build 链）
  4. FUN_140b66830(app, 1, …) = add_systems（state=1 组）
  5. FUN_140b3f550 → FUN_140b66480(app, 1, …) = add_systems（state=1 组）
  6. FUN_140b67b50 = 插件注册
  7. 内联 add_audio_startup_systems（systems/audio/mod.rs:144 PDB inline）+ closure$0（行 164，
     引用 ScheduledTerrainReadback::on_terrain_modified = 排程 wiring）
  8. 尾部 Hash 0x8e1bd251962ba784 查表（FUN_1400dd6f0）缺 → 格式化 panic（0x142af2cc0，行 39）
- **load_shader_into_library**（0x140b3fe40<&str> / 0x140b40210<String> / 0x140b405e0 replace，O）：
  clone name + compile_shader_variants() + AssetShaderLibrary::add_raster()（97 调用方）
- **load_mesh_into_library 0x140b1aca0（O）**：MeshWatch::watch(name) + AssetMeshLibrary::add(name, 0x90B payload)（87 调用方）
- 补充证据：bevy_ecs-0.16.0 确认（function_system.rs 串 0x142ada938 区）。
'''

SECTIONS['clearing.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **init_clearing 0x140a78aa0（26KB，O）** 顺序：
  1. 名式网格 2x（load_mesh_into_library：tree/bush）+ 文件式 2x（'assets/meshes/ice_cracks.json'、
     'assets/meshes/distant_billboard.json' 0x142ad6e71）
  2. **直接调用 startup_flowery（0x141553120）+ startup_olden（0x1415539d0）** ← 跨模块主题装载边
  3. clearing shader 注册（ice_cracks/water + ice_cracks.raster.hlsl 族，util_add_clearing_shader 内联）
  4. deserialize trees.json（'deserialize trees' 0x142ad6e18）→ 注册记录（FUN_1412dd460 等记录 fn-ptr）
  5. 6x unwrap panic（'called Result::unwrap…'，0x142ad69a0 区）
- **uninit_clearing 0x140a7a5b0（O）**：恢复 FMOD `snapshot:/Loading`（0x142ad6ef0）+ 卸载状态。
'''

SECTIONS['debug.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **debug 0x1408efd40（36KB，O）**：**21x 名式网格（FUN_140b1aca0）+ 14x 文件式（FUN_140b0eeb0）**，
  共 35 次网格载入（章节旧述 9 个为欠计）+ 8x FUN_140a07680 + 5x FUN_140a06b00（调试网格后处理/记录）。
  已证名：decorator_trigger_volume、cube、cube_grey、stair_step（0x142aa78d0 区）、leaf_particle_mesh、
  leaf_particle、texture_test_shader、debug_sphere_{white,red,blue,green,grey}（0x142aa79b5 区）、
  particle_plane（'assets/meshes/particle_plane.json' 0x142aa7914）、'assets/meshes/stair_step.json'（0x142aa7895）。
  15x unwrap panic（0x142aa6a30/0x142aa6a10 区）。
'''

SECTIONS['init_color_library.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **init_color_library 0x14082c000（O）**：读 `assets/colors.json`（0x142a920a6, 16 字符；失败 OOM abort）
  → 构造 0x12B 个颜色条目（color 值 + 名字串）→ FUN_1408ba290 注册进 color library 资源；
  条目经 cap-first Vec push（FUN_1428a9a60）+ FUN_1412bbcf0/FUN_1412d8420（条目构造/登记）。
- reload_color_library（内联，行 29）：重建同一注册。
'''

SECTIONS['init_photomode_state.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖；机器码手解）

- **on_enable_photomode 0x140870570（O，机器码）**：
  `cmp byte [0x143634d90], 0`（demo flag）→ demo：`movss xmm0, [0x142925904]`（**1.0**）/
  非 demo：`movss xmm0, [0x1429cd078]`（**3.0**）→ `[rcx+0]=fov; [rcx+4]=-1; [rcx+8]=0; [rcx+0xa]=0; [rcx+0xc]=0; ret`。
  即 PhotomodeState 默认：{ fov: demo?1.0:3.0, +4: -1(哨兵), +8/+0xa/+0xc: 0 }。
- 邻接函数 0x140870590（同一文件第二实例）：`word[+0xc]=0x100`、`qword[+4]=-1`（fov 语义同族）。
- 注：本地址在 tg_b2 数据库反编译为 NotFound（小函数），机器码手解为准（rne-read --hex 0x140870570）。
'''

SECTIONS['load_audio.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **load_audio 0x140b0e0f0（O）**：FUN_140d635e0 取全局音频系统句柄 → FUN_140a57100 注册
  → 构造 3 个音频对象（0x1c/0xc/8B 记录）入 audio 资源容器（cap-first Vec push x2）；
  `if (2 < *puRam000000014362c3a8)` 门控 info 日志（FUN_140d63910/FUN_140d8a550，'Reading …' 族）。
  细节 U：音频句柄类型（audio-misc 域覆盖）。
'''

SECTIONS['load_glade_audio.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **load_glade_audio 0x1408be360（O）**：theme byte @ gladestate+0x58 x8 索引 →
  GladeAudio::load（0x1408bf460，4 主题各一次）→ 每主题：
  'Reading {path}/audio.json' 日志（'audio.json' 0x142aa0d44）→ 4 事件
  `event:/Music/Halloween/{Single Leaf, Tiny Olden Glade, Forgotten Ruins, Enchanted Forest}`
  （串 0x142aa0c33 区）→ FUN_140a54440 事件句柄 x2 + FUN_14089a280（容器注册）
  → FMOD 参数 `GladeTypeIsHalloween`(9, 0x142aa0ce8) / `IsHalloween`(0xb, 0x142aa0cf1)
  （FUN_140a53db0 设置，行 371/386）→ stop_and_play_new（内联，行 81）。
  5x unwrap panic 行 43（0x142ad0600/0x142ad05c0/0x142ad2bc0/0x142ad2c20）。
- **GladeAudio::load 0x1408bf460（O）**：单主题音频事件配置载入（见上）。
- 契约缺口：reconstruction GladeAudio 无 4 主题音乐事件表（HIGHLIGHTS 7 既有结论保持）。
'''

SECTIONS['load_glade_grading.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **load_glade_grading 0x1408bf9c0（O）**：按 theme（byte@+0x58 x8）选 `assets/gladestate/<theme>` 路径
  （0x142aef51a 区）→ 2x 打开（FUN_140d76ed0 + FUN_140d62810 拼接）→ 分级配置经 FUN_141405ec0
  注册（2 变体：初载/reload 路径各一）→ FUN_140d7bd90 清理 x4 + 1x unwrap。
'''

SECTIONS['load_glade_tod_and_weather.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **load_glade_tod_and_weather 0x140ae7810（O）**：按 theme 路径（assets/gladestate/<theme>，0x142aef51a 区）
  → 2x 打开 + 拼接（FUN_140d76ed0/FUN_140d62810）→ **共用 ToD 载入器 FUN_140d59140 +
  FUN_1414051e0**（2 变体：theme 目录与 'assets/tod/default.json' 0x142a99540(23)）→ 3x unwrap panic。
- **load_snow_particles 0x140ae7d90（O）**：全局生成器/RNG 状态（FUN_140d4d740/FUN_140d64e00）
  + 记录注册（FUN_1412fa8e0 + FUN_1412dc470）+ assert panic（FUN_1428d9310）。
- 跨模块：startup_time_of_day 复用同一 ToD 载入器（FUN_140d59140 + FUN_1414051e0）。
'''

SECTIONS['load_shaders.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **load_ui_shaders 0x140aaaf80（O）**：6 个 UI shader 依次经 load_shader_into_library<&str>（FUN_140b3fe40）：
  ui_2d、ui_2d_ws、ui_2d_color_button、text、mei_shadow、custom_cursor（`.raster.hlsl`）。
- **load_shaders 0x140aab0c0（O）**：~13 个渲染 shader（串表 0x142add604 区证实 10 个路径+名对）：
  meadow_flower、vertex_color_shader、dashed_line、timer_circle、final_post、egui、
  solid_vertex_color{,_albedo_map,_wooden_gate}、flag_{roof_cloth,wall_cloth,small}、stairs_foliage_exclusion；
  每对 = '{/…}.raster.hlsl' 路径 + 名称键。
'''

SECTIONS['load_textures.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **should_load 0x1409e6950（O）**：前缀匹配谓词，串表 0x142ac4780 区（顺序）：
  logo / brush / loading / menu/pounce_light_logo / menu/heart / menu/noknok_logo / menu/fmod_logo
  / blue_noise / default_loading_background / default_loading_background_demo；
  **末两项受 demo flag 0x143634d90 门控**（与 session build/photomode 共用同一全局）。
- **load_glade_texture_mappings 0x1409e6b60（O）**：读 'textures.json' → 语义键
  glade:{ivy_leaf_color, canopy_flowers_depth, leaf_particle_colors, terrain_texture, grass_patch,
  lilypad_flower, canopy_color, bark_color, bark_color_naked} → 失败 panic `Couldn't load `（0x142ac47xx 区）。
- load_textures（内联，行 53）= should_load 过滤后逐纹理载入 + create_mappings（内联）。
'''

SECTIONS['panic_reporter_startup.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- panic_reporter_startup（行 22，内联，无反编译符号，**I**）：模块语义 = 设置 Rust panic hook
  上报反馈系统（消费方 = feedback_report 域）。无反编译符号，无机器码证据；保持 I。
'''

SECTIONS['spawn_brush_previews.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **spawn_brush_previews 0x140924b40（51KB，O）**：**24 次网格载入（13x 名式 FUN_140b1aca0 +
  11x 文件式 FUN_140b0eeb0）**（章节旧述 ~13 为欠计）+ **17 个 spawn 记录**（17x
  FUN_1428a9a60/FUN_141564d30/FUN_140d146a0 三件套 = 记录表 push）+ 13x unwrap panic。
  名族（0x142aaed1c 区等）：brush_arrow、brush_circle{,_cross,_scribble}、brush_4_arrows、
  brush_2_arrows、brush_5_replay、brush_arrow_flat{,_single_up,_single_foward,_outline}、
  flat_arrow{,_outline}、corner_arrow{,_outline,_switch_ridge_dir}、move_shape{,_outline}、
  vertex_color_shader（0x142aaece6 区）。
- 消费：spawn 记录 → 渲染/backend 预览实体（Ui2dImage::new 组件，ui-input 域线索）。
'''

SECTIONS['spawn_human_ref.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **spawn_human_ref 0x14089d5a0（O）**：load_mesh_into_library('human_ref_cube'，串 0x142a9947e)
  → shader 'vertex_color_shader'（0x142a9948c）→ 1 个 spawn 记录（FUN_1412d6d80）+ assert panic
  （FUN_1428d9310）。人类尺寸参考立方体实体。
'''

SECTIONS['spawn_preplaced_trees.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **spawn_preplaced_trees 0x140862270（27KB，O）**：3x 文件操作（FUN_140d76ed0 + FUN_140d62810，
  读 trees.json + clearing/tree/bush 网格）+ deserialize trees（'deserialize trees' 0x142ad6e18）
  → 4 个 spawn 记录（FUN_1428a9a60/FUN_141564d30 三件套）+ 3x unwrap。
'''

SECTIONS['startup_blit.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **startup_blit 0x140901430（O）**：3x load_shader_into_library<&str>（FUN_140b3fe40）：
  resolve_to_swapchain、resolve_msaa_to_swapchain、blit_stretch_decode_srgb（`/blit/*.raster.hlsl`）。
'''

SECTIONS['startup_critters.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **startup_critters 0x1409d49a0（21KB，O）**：6x 名式网格（butterfly、duck_flying、duck_heart、
  splash 等）+ 5x shader（firefly_glow、duck_animated、splash_particle 等）+ 4 个 spawn 记录
  + FMOD `event:/SFX/Butterfly/Shoosh` + 6x unwrap panic。
'''

SECTIONS['startup_deferred_raster.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **startup_deferred_raster 0x140a97a30（O）**：4x load_shader_into_library<&str>（FUN_140b3fe40）
  → 句柄对 {key, handle} 写入 param_4 数组（每项 2x8B，供 deferred raster pass 消费）。
'''

SECTIONS['startup_garden_brush.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **startup_garden_brush 0x1415f87c0（O）**：2x 名式网格（garden_lowpoly_flower 等）+ 2x 文件式
  （'assets/meshes/garden_stone.json'）+ 2x shader（solid_vertex_color_garden_stone.raster.hlsl 等）
  + 2 个 spawn 记录 + 2x unwrap。
'''

SECTIONS['startup_grass.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **startup_grass 0x140a21620（O）**：grass blade/grass_blade 网格（load_mesh_into_library）
  + 3x shader（grass_object_push、grass，'/grass_object_push.raster.hlsl'、'/grass.raster.hlsl'）
  + 2 个 spawn 记录（FUN_1412dc470）+ 特殊注册 FUN_1412fff70（grass 实例化后端接线）。
'''

SECTIONS['startup_ivy.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **startup_ivy 0x140a21a30（O）**：4x 名式网格（ivy_branch、ivy_leaf、ivy_flower、ivy_debug_cube）
  + instanced_ivy_branch/instanced_ivy_leaf shader + 3 个 spawn 记录（FUN_1412dd460）+ 3x unwrap。
'''

SECTIONS['startup_meadow.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **init_meadow 0x1408639f0（O）**：3x 文件操作（FUN_140d76ed0 + FUN_140d62810：plants.json 与
  meadow 植物网格路径）→ deserialize plants（panic `deserialize plants` 0x142a96308，行 18，
  文件 0x142a95ec0/0x142a96320）→ 1 个 spawn 记录 + meadow_plant_shader 注册。
'''

SECTIONS['startup_nani.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **startup_nani 0x140a38610（O）**：Bevy 系统入口（间接跳转分发到实际系统函数）。
- **reload_nani 0x140a398b0（O）**：遍历 nani 实例/world subset 重建 —
  FUN_140ae6750（subset 重建，索引 &0xffffffff）→ FUN_140beefa0（重建结果接线）→
  记录 push（FUN_140b724f0/FUN_140b761c0/FUN_1428c08b0）→ `.unwrap()` panic 行 43，
  文件 `crates/country-core/src/render/nani/gpu_world.rs`（0x142aba4d8）。
  内联 runtime_check_limits（行 153）/ create_world_subset（行 166）。
'''

SECTIONS['startup_roof.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **startup_roof 0x1414547d0（O）**：4x 名式网格 + 4x 文件式（roof_spire、roof_support_beam、
  'assets/meshes/roof_delete_scribble.json' 0x142c61753）→ 2 个 spawn 记录（FUN_1412dd460）
  + instanced_roof_spire shader + roof_button + 4x unwrap。
'''

SECTIONS['startup_session.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **load_game_session 0x14166e7d0（78KB，O）** 完整流程：
  1. 读 SessionLoadAttempt（*param_1+0x10 空 → theme 默认 3；否则 attempt → theme byte @+6）
  2. 主题 jump-table（0x142c84d24：uVar20 = niche ^ 0x8000…；case0 → `internal error: entered
     unreachable code` panic 行 40；case 1/2 → 继续/新会话路径）
  3. 继续会话：FUN_140a9fa80 判有效 → 日志 'Latest game session ' / 否则 'Continuing game session '
     → 会话 Vec push（cap-first grow FUN_140562870，8B 元素，计数 @+0x40）→ 拼 `<会话>/history.json`
     （'history.json' 0x142c845e7, 12）
  4. 新会话：'Starting a new game session' 日志 → FUN_140c102e0 生成会话 id → create_dir
     （panic 'Creating folder for game session' 行 32）→ 写 meta.json（panic 'Writing new session
     meta.json' 行 29；theme 小写 summer/olden/flowery/autumn/winter，串 0x142c84cc0 区）
     → 起始 build：demo flag 0x143634d90 ? 'assets/starting-builds/demo'(0x142c849cb)
     : 'assets/starting-builds/full'(0x142c849b0, 27) → 内联 copy_dir_all
  5. 内联 copy_dir_all：递归 walk（FUN_140d59e80/59f10/62e10/61e00）→ 条目 Vec（4xqword，grow
     FUN_140562c30）→ **"intro" 子目录跳过**（名 5 字符 == "intro" 0x72746e69+o）→ 排序
     （FUN_1413320d0 / FUN_140b4c390）→ 逐项复制
  6. 日志/串：'Loading a pre-parsed game session'（0x142c84798）、'should not have been consumed
     earlier'（0x142c84830）、'Failed to load session meta. Using default. Error: '（0x142c84c40）、
     'Error deleting glade: '（0x142c84cc0 区）、session 文件族 meta.json/history.json/
     snapshots(0x142c848b8)/screenshot.jpg
- **copy_dir_all 0x141740c00（O）**：递归目录复制（std fs walk；&str 变体内联于 load_game_session）。
'''

SECTIONS['startup_sheep.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **startup_sheep 0x140a093e0（47.8KB，O）**：
  1. 5x 文件式网格（FUN_140b0eeb0）：'assets/meshes/sheep_animation/1.json'（0x142ac6a23 区）、
     sheep_animation/delighted.json、sheep_heart、umbrella、floatie
  2. 4x 名式网格（FUN_140b1aca0）：sheep_mesh（0x142ac6a18）等
  3. 5x 属性提取（FUN_1405b1a70：'Vertex_Position'(15, 0x142ac6553) / 'Vertex_Normal'(13, 0x142ac6562)）
     — 网格后处理（动画顶点数据）
  4. 注册链 FUN_14130efe0/FUN_1412e23e0/FUN_1416f0ea0（动画网格注册）
  5. 音频：FMOD `event:/SFX/Sheep/Baah` + 参数 'Pitch'（0x142ac6c48）
  6. 6x unwrap panic（0x142ac6360 区，行 43 族）
'''

SECTIONS['startup_sky_and_sun.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **startup_sky_and_sun 0x140864430（O）**：skydome 网格（1x 名式 + 1x 文件式）+ skydome_shader
  （load_shader_into_library）+ 1 个 spawn 记录（FUN_1412dd460）+ 1x unwrap。
  网格消费：systems/backend/vao_update.rs 的 VAO 构建路径。
'''

SECTIONS['startup_terrain.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **startup_terrain 0x140926bf0（28KB，O）**：
  1. grid_gen_cw(Vec, 0, 0x5a, 0x5a, 0x5a) — **90x90 地形 quad 索引网格（cw）**（行 104）
  2. 8 网格 + 6 shader 以**嵌套 Option 链**顺序载入（任一失败整体中止；每对成功 →
     压 spawn 记录 {fn-ptr 类型标识, mesh, shader, scale 1.0 (0x3f800000)}，cap-first grow FUN_1428a9a60）：
     floor(5) + floor_shader → demo_text_floor(15) + demo_terrain_overlay →
     rocky_terrain（**文件式** 'assets/data/rocky_terrain.json'，FUN_140b0eeb0）+ rocky_terrain shader →
     stairs_step_mesh(16) + stairs_step → path_pebble_mesh(16) + path_pebble →
     stairs_pebble_mesh(18) → terrain(7) + solid_vertex_color{albedo_map} → billboard_plants(16)
     + billboard_plant_shader（串 0x142aaef28–0x142aaf2d4 区）
  3. spawn 记录 fn-ptr 族：FUN_1412d7aa0(0x10B) 1412d7c70(0x78B) 1412d8b00(0x18B) 1412dc9f0(0x88B)
     1412da440 1412d9790 1412dbb00 1412db2a0 1412dd190(0x88B) 1412d6640
  4. 7x unwrap panic 行 43（0x142aae510/0x142aae4f0）
- **grid_gen_cw 0x140928100（O）**：递归生成三角索引 — `while (5 < (to+1)-from)`: 分 <=4 列块递归；
  每 quad 6 索引 {a, b, a+1, a+1, b, c}（a=from+c+r*stride，b=a+stride+1，c=a+stride+2 族）；
  每行 param_4 行 x (to-from) quad；cap-first Vec<u32>（grow FUN_1416a83a0，4B）；from>to →
  assert panic（0x142aaf2f8 区）。**grid_gen_ccw 0x140928350 = 同族 CCW 缠绕变体**。
  复用：load_mesh_atlases 的 stone_floor_/timber_/smoke parts 也用它。
- 跨模块：load_prefabs_impl 内联引用 generate_grid_cw（canopy quad 重算，PDB inline）。
'''

SECTIONS['startup_time_of_day.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **startup_time_of_day 0x14089d6f0（O）**：load_default_tod（内联，行 10）=
  FUN_140d59140(&path, 'assets/tod/default.json' 0x142a99540, 23) → FUN_1414051e0 载入 ToD 配置
  → 2x unwrap panic。**与 load_glade_tod_and_weather 共用同一 ToD 载入器**（跨模块）。
'''

SECTIONS['startup_walls.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **glade_startup_walls 0x140a92670（O）**：
  1. 2x load_shader_into_library<&str>（FUN_140b3fe40）：'/wall/wall_brick_lod0.raster.hlsl' +
     'wall_brick_lod0'、'/wall/wall_brick_lod1.raster.hlsl' + 'wall_brick_lod1'（串 0x142ada29e 区）
  2. 2x load_mesh_into_library（FUN_140b1aca0）：'half_cube'（0x142ada295）/'brick half cube'（0x142ada2fe）
  3. **7x 顶点/索引构造调用**（FUN_1428a9a60/FUN_141564d30/FUN_140d146a0 三件套 x7 =
     create_half_cube_mesh 内联：字节常量 0x01–0x07 三角索引族）
- 消费：wall_brick LOD shader → 墙渲染 LOD（systems/wall 域）。
'''

SECTIONS['startup_water.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **startup_water 0x1408f1a20（17.6KB，O）**：6x 文件式网格（water_plane、lilypad、lilypad_flower、
  reed{,_autumn}、water_stones、fallen_leaves、water_grass_winter 族）+ 6x shader
  （water_plane、ice_plane、water_plane_refraction_copy、water_snow_plane、lilypad、lilypad_flower、
  reed、water_x_paths 族）+ **7 个 spawn 记录**（FUN_1428a9a60/FUN_141564d30/FUN_140d146a0 三件套
  x7）+ 6x unwrap panic。
- 消费：水面渲染（terrain-water 域跨模块线索）。
'''

SECTIONS['glade_themes/flowery.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **startup_flowery 0x141553120（O）**：2x 名式网格 + 2x 文件式网格（meadow_lowpoly_flowers、
  'plants/lavender.json'）+ 2x shader（FUN_140937aa0：'billboard_plant_shader' 0x142c6c862、
  'meadow_flower' 0x142c6c8f3）+ 树配置 trees/{rocks,clearing_edge_rocks,stumps,
  clearing_edge_tree_stumps}.json（FUN_140d62810 拼接）+ 2 个注册记录（FUN_1412dabb0/1412d9de0）
  + 2x unwrap。
- 跨模块：init_clearing 直接调用本函数（0x140a78aa0 → 0x141553120）。
'''

SECTIONS['glade_themes/olden.rs.md'] = '''
## 实现方式（算法级，2026-10-15 深挖）

- **startup_olden 0x1415539d0（O）**：2x 名式网格 + 2x 文件式网格（'assets/meshes/clover.json'）+
  2x shader + **6 份树配置** trees/{rocks,clearing_edge_rocks,stumps,clearing_edge_tree_stumps,
  logs,clearing_edge_tree_logs}.json（串 0x142c6c918 区）+ 2 个注册记录（FUN_1412d9400/1412d9270）
  + 2x unwrap。
- 跨模块：init_clearing 直接调用本函数（0x140a78aa0 → 0x1415539d0）。
'''

updated = 0
for fn, sec in SECTIONS.items():
    p = os.path.join(BASE, fn)
    with io.open(p, 'a', encoding='utf-8') as f:
        f.write('\n' + sec.lstrip('\n').rstrip() + '\n')
    updated += 1
    print('updated', fn)
print('total', updated)
