#!/usr/bin/env python3
"""Regenerate startup-components domain chapters with owner analysis (2026-10-14).

Reads SOURCE_MAP-extra.json, applies per-file purpose + per-function conclusions,
rewrites chapters/components/** and chapters/startup/** in place.
Evidence: expansion-b1/b2 decomp + rne-read string/machine-code evidence (owner tg_sc1).
"""
import json, os

BASE = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep"
SRC = os.path.join(BASE, "SOURCE_MAP-extra.json")
CH = os.path.join(BASE, "chapters")

d = json.load(open(SRC, encoding="utf-8"))

# ---------------------------------------------------------------------------
# Per-file purpose + function conclusions.
# fn_notes key: (va, name) -> (conclusion, action_desc). conclusion in O/I/U.
# inline_notes key: line -> (conclusion, action_desc)
# ---------------------------------------------------------------------------
A = {}

# ============================ components ============================
reg = ("O", "Bevy `register_required_components` 注册胶水（monomorphized，bevy_ecs component.rs；18KB 哈希/类型表 + 每 impl 独立组件类型名表）。required-components 策略按类型合同恢复，无独立业务。")
clone_bh = ("O", "共享 `clone_behavior` 0x140002370 = `xor eax,eax; ret`（返回 0 = 无克隆行为）。Bevy 默认胶水。")
clone_bh2 = ("O", "`clone_behavior` = `lea rdx,[组件类型表]; mov eax,2; ret`（返回行为=2，克隆行为枚举变体；每 impl 独立类型表）。Bevy required-components 胶水。")

A["components/mod.rs"] = {
    "purpose": "游戏组件集总模块：~45 个 ECS 组件类型（marker / 参数组件）。本文件主要业务面 = 每个组件的 `register_required_components`（Bevy required-components 接线）。组件类型清单已由 reconstruction mod.rs 按 WP6 恢复（含 typed gap）。",
    "fn_notes": {
        ("0x140844f20",): reg, ("0x140845610",): reg, ("0x140845d00",): reg,
        ("0x1408463f0",): reg, ("0x140846ae0",): reg, ("0x1408471d0",): reg,
        ("0x1408478c0",): reg, ("0x140847fb0",): reg, ("0x1408486a0",): reg,
        ("0x140848d90",): reg, ("0x140849480",): reg, ("0x140849b70",): reg,
        ("0x14084a260",): reg, ("0x14084a950",): reg, ("0x14084b040",): reg,
        ("0x14084b730",): reg, ("0x14084be20",): reg, ("0x14084c510",): reg,
        ("0x14084cc10",): reg, ("0x14084d310",): reg, ("0x14084da10",): reg,
        ("0x14084e100",): reg, ("0x14084e7f0",): reg, ("0x14084eee0",): reg,
        ("0x14084f5d0",): reg, ("0x14084fcc0",): reg, ("0x1408503b0",): reg,
        ("0x140850aa0",): reg, ("0x140851190",): reg, ("0x140851880",): reg,
        ("0x140851f80",): reg, ("0x140852670",): reg, ("0x140852d60",): reg,
        ("0x140853450",): reg, ("0x140853b40",): reg, ("0x140854230",): reg,
        ("0x140854930",): reg, ("0x140855030",): reg, ("0x140855730",): reg,
        ("0x140855e20",): reg, ("0x140856510",): reg, ("0x140856c10",): reg,
        ("0x140857300",): reg, ("0x1408579f0",): reg,
        ("0x140002370",): clone_bh,
        ("0x14036a4f0",): ("O", "serde_json CompactFormatter 整数序列化（迭代 /10000 数字化，UNK_1429c6964 数字表）。derive 胶水。"),
        ("0x14084cc00",): clone_bh2, ("0x14084d300",): clone_bh2, ("0x14084da00",): clone_bh2,
        ("0x140851f70",): clone_bh2, ("0x140854920",): clone_bh2, ("0x140855020",): clone_bh2,
        ("0x140855720",): clone_bh2, ("0x140856c00",): clone_bh2,
    },
    "inline_notes": {
        "229": ("I", "`Ui2dImage::new`（UI 2D 图片组件构造）— 内联无独立符号，功能由消费方（UI/工具栏构造）推断。"),
    },
}

A["components/drawable.rs"] = {
    "purpose": "nani 可绘制组件族：`NaniInstanceMeshDesc`（命名/图集 part 选择）、`NaniActiveInstance`(=InstanceKey)、`NaniSolidTexture`/`PrevTransform`/`AlbedoMap`/`AlphaMap`、`DrawableMeshBundle`。业务面 = 组件注册 + `cache_prev_object_transforms` 系统 + `From<MeshAtlasPart>`。",
    "fn_notes": {
        ("0x14082c3d0",): reg, ("0x14082cad0",): reg, ("0x14082d1c0",): reg,
        ("0x14082d8b0",): reg, ("0x14082dfa0",): reg, ("0x14082e690",): reg,
        ("0x14082ed80",): reg, ("0x14082f470",): reg, ("0x14082fb60",): reg,
        ("0x140830250",): reg, ("0x140830940",): reg, ("0x140831040",): reg,
        ("0x14082cac0",): clone_bh2, ("0x140002370",): clone_bh,
        ("0x140831030",): clone_bh2, ("0x140831730",): clone_bh2,
        ("0x14082bd60",): ("O", "`NaniInstanceMeshDesc::from<MeshAtlasPart>`（机器码 `mov [rcx+4],eax; mov dword[rcx],1` → 判别=1 + 值=0 = `AtlasPart(0)`）。与 reconstruction 一致。"),
        ("0x14082bd70",): ("O", "`impl$2::build` = `mov rcx,rdx; jmp` 尾调用共享 Bundle build（DrawableMeshBundle 构造委托）。"),
        ("0x14082bd80",): ("O", "**业务系统** `cache_prev_object_transforms`：逐帧遍历 sparse instance buffer（+0xf8/+0x100/+0x108 字段、0xa0/0x48/0x30 行 stride），把实例当前变换拷贝写入 `PrevTransform`（velocity_only/wren 消费）。"),
    },
    "inline_notes": {
        "11": ("I", "`impl$3/impl$5` = Bundle 派生 `component_ids`/`get_components`（DrawableMeshBundle/含 KeepOnLevelUnload/DesignatedRenderPass 变体）。"),
        "43": ("I", "`impl$0::from` = `NaniInstanceMeshDesc::from` 另一变体（内联）。"),
        "61": ("I", "`impl$13::component_ids`/`impl$15::get_components` = 另一 Bundle 变体的派生胶水（同 line 11 族）。"),
    },
}

A["components/transient_mesh.rs"] = {
    "purpose": "瞬时网格组件与注册表：`TransientMeshRegistry`（Mutex+待删 Vec）、`TransientMesh` 组件（持 Arc 注册表，Drop 时登记删除）。backend `delete_dropped_transient_meshes` 每帧排空释放 GPU 网格。",
    "fn_notes": {
        ("0x1412e9e70",): ("O", "`drain_meshes_to_delete<F>`：锁 + 遍历待删 Vec（cap-first {cap@+8,ptr@+0x10,len@+0x18}，元素 8B），对每个句柄调闭包（闭包内两次删除 FUN_14092d750/FUN_1409247a0），清空 len。"),
        ("0x140aa8070",): ("O", "`deferred_delete_mesh`：锁注册表 Mutex（@+0x0），push 句柄进 Vec@+0x8（cap-first，grow FUN_140562870 元素 8B）。"),
        ("0x140002370",): clone_bh,
        ("0x140aac640",): reg,
        ("0x140aa8200",): ("O", "`new_with_registry`：入口 = Arc clone（`lock inc [ptr]` + 溢出检查）→ 构造 TransientMesh（存 Arc + 句柄）。机器码 0x140aa8200。"),
    },
    "inline_notes": {},
}

# ============================ startup ============================
A["startup/clearing.rs"] = {
    "purpose": "clearing（林间空地）初始化/反初始化：载入 clearing 网格与 shader、反初始化时恢复音频 snapshot。",
    "fn_notes": {
        ("0x140a78aa0",): ("O", "`init_clearing`：载入 clearing 网格 `assets/meshes/{ice_cracks,distant_billboard}.json` + `clearing/tree/bush`，注册 `ice_cracks/water/ice_cracks.raster.hlsl` shader，deserialize `trees.json`（clearing 外围树）。"),
        ("0x140a7a5b0",): ("O", "`uninit_clearing`：恢复 FMOD `snapshot:/Loading`（音频快照），卸载 clearing 状态。"),
        ("0x140a14620",): ("O", "`visit_str<serde_json::Error>`：serde 枚举字符串反序列化胶水（长度 switch + 跳表）。"),
    },
    "inline_notes": {
        "41": ("I", "`util_add_clearing_shader` — 添加 clearing shader 的内联 helper。"),
    },
}

A["startup/debug.rs"] = {
    "purpose": "debug 启动系统：载入调试可视化/测试网格（debug 球体、decorator 测试网格、stair_step、particle_plane）。",
    "fn_notes": {
        ("0x1408efd40",): ("O", "`debug`：载入 `assets/meshes/{decorator_delete_scribble,decorator_test,decorator_test_turret,decorator_test_door,decorator_test_cupola,decorator_test_dormer,decorator_outline,stair_step,particle_plane}.json` + debug_sphere_{white,red,blue,green,grey} 等网格。"),
    },
    "inline_notes": {},
}

A["startup/init_color_library.rs"] = {
    "purpose": "颜色库初始化：从 assets/colors.json 载入默认调色板。",
    "fn_notes": {
        ("0x14082c000",): ("O", "`init_color_library`：构造 0x12B 颜色条目（color 值 + 名字），注册进 color library 资源（FUN_1408ba290）；路径 `assets/colors.json`。"),
    },
    "inline_notes": {
        "29": ("I", "`reload_color_library` — 重载颜色库 helper（内联）。"),
    },
}

A["startup/init_photomode_state.rs"] = {
    "purpose": "照片模式状态初始化与启用。",
    "fn_notes": {
        ("0x140870570",): ("O", "`PhotomodeState::on_enable_photomode`：机器码 — 写 float 到字段0、字段+4=-1、+8/+0xa/+0xc 清零 = 初始化 PhotomodeState 默认值（视全局标志分支）。"),
    },
    "inline_notes": {
        "15": ("I", "`init_photomode_state` — 注册 PhotomodeState 资源/初始化（内联）。"),
    },
}

A["startup/load_audio.rs"] = {
    "purpose": "音频系统启动：注册 audio 源/资源容器。",
    "fn_notes": {
        ("0x140b0e0f0",): ("O", "`load_audio`：构造 3 个音频对象（0x1c/0xc/8B 记录）注册进 audio 资源；读配置（全局 gate `*puRam+0x...c3a8`）。细节 U（音频句柄类型）。"),
    },
    "inline_notes": {},
}

A["startup/load_glade_audio.rs"] = {
    "purpose": "glade 音频载入：按 glade theme 载入对应 FMOD 音乐事件与音效。",
    "fn_notes": {
        ("0x1408be360",): ("O", "`load_glade_audio`：读 gladestate 配置（theme byte@+0x58 ×8 索引），载入 GladeAudio；构造 4 个音频对象，事件路径 `event:/Music/Halloween/{Single Leaf,Tiny Olden Glade,Forgotten Ruins,Enchanted Forest}`（对应 4 theme）。"),
        ("0x1408bf460",): ("O", "`GladeAudio::load`：载入单个 GladeAudio（音频事件配置）。"),
    },
    "inline_notes": {
        "81": ("I", "`stop_and_play_new` — 停止旧音频并播放新（内联）。"),
        "119": ("I", "`GladeAudio::try_load` — 尝试载入（内联）。"),
    },
}

A["startup/load_glade_grading.rs"] = {
    "purpose": "glade 分级 LUT 载入（按 theme）。",
    "fn_notes": {
        ("0x1408bf9c0",): ("O", "`load_glade_grading`：按 theme（byte@+0x58 ×8）从 gladestate 载入分级配置/LUT（`assets/gladestate` 路径族）。"),
    },
    "inline_notes": {},
}

A["startup/load_glade_tod_and_weather.rs"] = {
    "purpose": "glade ToD/天气 与雪粒子载入（按 theme）。",
    "fn_notes": {
        ("0x140ae7810",): ("O", "`load_glade_tod_and_weather`：按 theme（byte@+0x58 ×8）载入 `assets/gladestate/<theme>` 的 ToD/weather 配置（FUN_140d76ed0 打开路径）。"),
        ("0x140ae7d90",): ("O", "`load_snow_particles`：载入雪粒子配置/资源。"),
    },
    "inline_notes": {},
}

A["startup/load_mesh_atlases.rs"] = {
    "purpose": "网格图集（mesh atlas）生产者：生成程序化图集 mesh parts（stone_floor/plaster/stairs_floor/smoke）+ 载入 nani 网格（assets/nani_meshes.ron）+ 地形网格。是渲染 pass 的网格资产输入。",
    "fn_notes": {
        ("0x14129f400",): ("O", "`load_mesh_atlases`：生成图集 parts — `Vertex_Position`+`stone_floor_`、`plaster`/`plaster_strip`、`stairs_floor_chunk_`、`smoke`；调 grid_gen_cw/ccw（0x140928100/350）、load_nani_meshes（0x1415f7190）。"),
        ("0x1412a0820",): ("O", "`reload_mesh_atlases`：重建图集（销毁旧 part、重新生成并注册），含 `load_prefabs.rs` 跨模块引用；反转字符串常量疑为调试/计数标记。"),
        ("0x1415f7190",): ("O", "`load_nani_meshes<closure0>`：打开+deserialize `assets/nani_meshes.ron`，注册 nani 网格并建文件 watcher（`Failed to watch file`）。"),
        ("0x1415f5b60",): ("O", "`load_nani_meshes<closure1>`：reload 路径变体（同 RON 解析 + 图集重建）。"),
        ("0x1415f8c40",): ("O", "`visit_str<ron::Error>`：ron 枚举字符串反序列化胶水。"),
    },
    "inline_notes": {
        "159": ("I", "`generate_roof_floor_meshes` — 屋顶/地板网格生成器（内联）。"),
        "197": ("I", "`generate_curved_halftimber_meshes` — 弧形半木构网格生成器（内联）。"),
        "234": ("I", "`generate_plaster_mesh` — plaster 网格生成器（内联）。"),
        "255": ("I", "`generate_stairs_floor_mesh` — 楼梯地板网格生成器（内联）。"),
    },
}

A["startup/load_prefabs.rs"] = {
    "purpose": "预制体（prefab）入库：网格资产生产者。读 `assets/prefabs` 目录，经 `load_json_as_mesh` 处理顶点属性（appear_pos/prim_center/prim_normals/Vertex_Normal/Vertex_Position）注册进 mesh 库；含 PrefabWatcher（热重载）与插件 build。",
    "fn_notes": {
        ("0x1412a1df0",): ("O", "**核心生产者** `load_prefabs_impl`：枚举 `assets/prefabs`（0xe），对每个 prefab 用 mesh 属性名查找（FUN_1405b1a70）提取 `appear_pos`/`idappear_pos`/`prim_center`/`prim_normals`/`Vertex_Normal`/`Vertex_Position{stone_floor_}`，`Unknown mesh code processor`/`Sholud not exist` panic 分支，注册进 mesh 库（load_json_as_mesh 消费链）。"),
        ("0x1412a1a60",): ("O", "`reload_prefabs`：重载触发器 — Mutex 守卫（LOCK/UNLOCK +0x10 标志）+ 重入检查，触发重新载入。"),
        ("0x140a08680",): ("O", "`filter_mesh_by_vertex_color<closure1>`：按顶点颜色过滤网格（遍历 float 顶点数据，判定 vertex color 存在性）。"),
        ("0x140a08d30",): ("O", "`filter_mesh_by_vertex_color<closure2>`：同族第二实例。"),
        ("0x1408e4b00",): ("O", "`impl$0::build`（PrefabWatcher 插件 build）：查 TypeId 注册系统（缺失时 FUN_14150c420/FUN_141519680 添加），创建 PrefabWatcher（0x1412a8e00），注册进 app。"),
        ("0x1412a8e00",): ("O", "`PrefabWatcher::new`：创建 notify 文件 watcher（debounce 100,000,000 ns）+ channel；失败 panic。"),
        ("0x1412a8fa0",): ("O", "`PrefabWatcher::watch_atlas`：Arc clone + 注册 atlas 路径监听（FUN_14091d540），失败 panic `Failed to watch file {path:?}!`。"),
    },
    "inline_notes": {
        "32": ("I", "`load_prefabs` — 启动入口包装（内联）。"),
        "187": ("I", "`load_prefab_mesh` — 单个 prefab 网格载入（内联）。"),
        "290": ("I", "`recalculate_canopy_quad_attribs` — 树冠 quad 属性重算（内联）。"),
        "400": ("I", "`watch_prefab` — 监听单个 prefab 路径（内联）。"),
        "435": ("I", "`changed_world_subsets` — 变更的世界子集收集（内联）。"),
    },
}

A["startup/load_shaders.rs"] = {
    "purpose": "shader 载入：UI shader 与主渲染 shader 入库（经 startup/mod.rs 的 load_shader_into_library）。",
    "fn_notes": {
        ("0x140aaaf80",): ("O", "`load_ui_shaders`：载入 6 个 UI shader — ui_2d, ui_2d_ws, ui_2d_color_button, text, mei_shadow, custom_cursor（`.raster.hlsl`）。"),
        ("0x140aab0c0",): ("O", "`load_shaders`：载入 ~13 个渲染 shader — meadow_flower, vertex_color_shader, dashed_line, timer_circle, final_post, egui, solid_vertex_color{, _albedo_map,_wooden_gate}, flag_{roof_cloth,wall_cloth,small}, stairs_foliage_exclusion。"),
    },
    "inline_notes": {},
}

A["startup/load_textures.rs"] = {
    "purpose": "纹理载入：过滤（should_load）并载入启动纹理（logo/brush/loading/menu 相关），构建 GladeTextureMappings（textures.json 语义键→纹理文件映射）。",
    "fn_notes": {
        ("0x1409e6950",): ("O", "`impl$0::should_load`：纹理过滤谓词 — 前缀匹配 {logo,brush,loading,menu/pounce_light_logo,menu/heart,menu/noknok_logo,menu/fmod_logo,blue_noise,default_loading_background{, _demo}}，末两项受全局标志 `bRam+0x3634d90` 门控。"),
        ("0x1409e6b60",): ("O", "`load_glade_texture_mappings`：读 `textures.json`，构建 GladeTextureMappings — 语义键 `glade:{ivy_leaf_color,canopy_flowers_depth,leaf_particle_colors,terrain_texture,grass_patch,lilypad_flower,canopy_color,bark_color,bark_color_naked}`。`Couldn't load` panic。"),
        ("0x140a2f670",): ("O", "`visit_str<serde_json::Error>`：serde 枚举字符串反序列化胶水（长度 switch 按首字节匹配，映射到判别值）。"),
    },
    "inline_notes": {
        "53": ("I", "`load_textures` — 启动纹理总入口（内联）。"),
        "155": ("I", "`GladeTextureMappings::create_mappings` — 建映射（内联）。"),
    },
}

A["startup/mod.rs"] = {
    "purpose": "startup 模块总入口：`add_game_startup_systems`（GameStartup 插件，注册全部启动系统）+ shader/mesh 入库 loader（load_shader_into_library、load_mesh_into_library）。",
    "fn_notes": {
        ("0x140941110",): ("O", "`impl$0::add_game_startup_systems`：GameStartup 插件 build — 依次注册 startup 系统（FUN_140b67620/FUN_140b66830/FUN_140b66480 等 Bevy app.add_systems），缺注册 panic。"),
        ("0x140b1ac90",): ("O", "`impl$1::build`：`mov rcx,rdx; jmp` 尾调用共享 build（子插件 build 委托）。"),
        ("0x140b40210",): ("O", "`load_shader_into_library<String>`：载入 shader（源码读取 + 编译）入库，返回句柄。"),
        ("0x140b3fe40",): ("O", "`load_shader_into_library<&str>`：&str 变体（多数 startup 载入走此）。"),
        ("0x140b405e0",): ("O", "`load_shader_into_library_replace_previous<&str>`：载入并替换旧 shader 的变体。"),
        ("0x140b1aca0",): ("O", "`load_mesh_into_library`：载入网格入库（JSON 网格解析 + 注册；多个 startup 载入者共用）。"),
    },
    "inline_notes": {
        "220": ("I", "`load_shader_into_library` — 泛型分派（内联）。"),
    },
}

A["startup/panic_reporter_startup.rs"] = {
    "purpose": "panic 报告器启动：设置 Rust panic hook（上报到反馈系统）。",
    "fn_notes": {},
    "inline_notes": {
        "22": ("I", "`panic_reporter_startup` — 设置 panic 报告器（内联，功能由模块名+消费方推断；无反编译符号）。"),
    },
}

A["startup/spawn_brush_previews.rs"] = {
    "purpose": "画笔预览实体生成：载入 ~13 个 brush 预览网格并 spawn 预览实体（鼠标光标笔刷）。",
    "fn_notes": {
        ("0x140924b40",): ("O", "`spawn_brush_previews`：载入 brush_arrow, brush_circle{, _cross,_scribble}, brush_4_arrows, brush_2_arrows, brush_5_replay, brush_arrow_flat{, _single_up,_single_foward,_outline}, flat_arrow, corner_arrow_{outline,switch_ridge_dir}, move_shape{, _outline}, hand(temp_sphere), dashed_lines(snapping_line/circle) 等（13× load_mesh_into_library）+ spawn 预览实体（17×）。"),
    },
    "inline_notes": {},
}

A["startup/spawn_human_ref.rs"] = {
    "purpose": "人类尺寸参考实体：spawn 一个参考立方体（human_ref_cube）。",
    "fn_notes": {
        ("0x14089d5a0",): ("O", "`spawn_human_ref`：载入/引用 `human_ref_cube` 网格（vertex_color_shader），spawn 参考实体。"),
    },
    "inline_notes": {},
}

A["startup/spawn_preplaced_trees.rs"] = {
    "purpose": "预置树生成：deserialize trees.json，spawn clearing 外围预置树。",
    "fn_notes": {
        ("0x140862270",): ("O", "`spawn_preplaced_trees`：载入 `trees.json`（deserialize trees）+ `clearing/tree/bush` 网格，spawn 预置树实体。"),
    },
    "inline_notes": {},
}

A["startup/startup_blit.rs"] = {
    "purpose": "blit 阶段 shader 载入（交换链 resolve/srgb decode）。",
    "fn_notes": {
        ("0x140901430",): ("O", "`startup_blit`：载入 3 个 blit shader — resolve_to_swapchain, resolve_msaa_to_swapchain, blit_stretch_decode_srgb（`/blit/*.raster.hlsl`）。"),
    },
    "inline_notes": {},
}

A["startup/startup_critters.rs"] = {
    "purpose": "小动物（critters）资源载入：butterfly/duck/splash 网格 + shader + 音效。",
    "fn_notes": {
        ("0x1409d49a0",): ("O", "`startup_critters`：载入 butterfly, duck_flying, duck_heart, splash 网格；shader — firefly_glow, duck_animated, splash_particle；音效 `event:/SFX/Butterfly/Shoosh`。"),
    },
    "inline_notes": {},
}

A["startup/startup_deferred_raster.rs"] = {
    "purpose": "deferred raster 阶段 shader 载入。",
    "fn_notes": {
        ("0x140a97a30",): ("O", "`startup_deferred_raster`：载入 4 个 shader 并注册句柄（param_4 数组记录 {key,handle}）。"),
    },
    "inline_notes": {},
}

A["startup/startup_garden_brush.rs"] = {
    "purpose": "花园笔刷资源载入：garden_stone + 低模花。",
    "fn_notes": {
        ("0x1415f87c0",): ("O", "`startup_garden_brush`：载入 `assets/meshes/garden_stone.json` + `solid_vertex_color_garden_stone.raster.hlsl` shader + garden_lowpoly_flower。"),
    },
    "inline_notes": {},
}

A["startup/startup_grass.rs"] = {
    "purpose": "草地资源载入：grass blade 网格 + grass shader。",
    "fn_notes": {
        ("0x140a21620",): ("O", "`startup_grass`：载入 `grass blade`/`grass_blade` 网格 + `grass_object_push`/`grass` shader（`/grass_object_push.raster.hlsl`、`/grass.raster.hlsl`）。"),
    },
    "inline_notes": {},
}

A["startup/startup_ivy.rs"] = {
    "purpose": "常春藤资源载入：ivy 分支/叶/花网格 + instanced shader。",
    "fn_notes": {
        ("0x140a21a30",): ("O", "`startup_ivy`：载入 ivy_branch, ivy_leaf, ivy_flower 网格 + `instanced_ivy_branch`/`instanced_ivy_leaf` shader + ivy_debug_cube。"),
    },
    "inline_notes": {},
}

A["startup/startup_meadow.rs"] = {
    "purpose": "草地/meadow 植物载入：plants.json + 低模花 + meadow_plant_shader。",
    "fn_notes": {
        ("0x1408639f0",): ("O", "`init_meadow`：deserialize `plants.json`（`deserialize plants` panic），载入 meadow 植物网格 + `meadow_plant_shader`。"),
    },
    "inline_notes": {},
}

A["startup/startup_nani.rs"] = {
    "purpose": "nani（GPU 实例渲染后端）启动与热重载：创建世界子集、检查运行时限制。",
    "fn_notes": {
        ("0x140a38610",): ("O", "`startup_nani`：Bevy 系统入口（间接跳转分发到实际系统函数）。"),
        ("0x140a398b0",): ("O", "`reload_nani`：重载 nani 配置（遍历实例/world subset 重建；内联 runtime_check_limits / create_world_subset）。"),
    },
    "inline_notes": {
        "153": ("I", "`runtime_check_limits` — 运行时限制检查（内联）。"),
        "166": ("I", "`create_world_subset` — 创建 nani 世界子集（内联）。"),
    },
}

A["startup/startup_roof.rs"] = {
    "purpose": "屋顶资源载入：roof_spire/support_beam 网格 + shader + 删除笔刷。",
    "fn_notes": {
        ("0x1414547d0",): ("O", "`startup_roof`：载入 roof_spire, roof_support_beam, roof_delete_scribble 网格 + `instanced_roof_spire` shader + roof_button。"),
    },
    "inline_notes": {},
}

A["startup/startup_session.rs"] = {
    "purpose": "游戏会话启动：起始 build 选择（full/demo/intro）、会话目录创建、meta.json 读写、历史/快照载入。",
    "fn_notes": {
        ("0x14166e7d0",): ("O", "**核心** `load_game_session`：起始 build（`assets/starting-builds/{full,demo}`+intro）、创建会话文件夹（`Creating folder for game session`）、读/写 `meta.json`（theme: Summer/Olden/Flowery/Autumn/Winter）、载入 `history.json`/`snapshots`/`screenshot.jpg`；`Continuing game session` 日志。78KB。"),
        ("0x141740c00",): ("O", "`copy_dir_all<PathBuf>`：递归复制目录（std fs walk）。"),
    },
    "inline_notes": {
        "39": ("I", "`MaybeExists::get` — 路径存在性查询（内联）。"),
        "79": ("I", "`SessionLoadAttempt::from_session_id` — 由 session id 构造载入尝试（内联）。"),
        "438": ("I", "`load_session_meta` — 载入 meta.json（内联）。"),
    },
}

A["startup/startup_sheep.rs"] = {
    "purpose": "羊资源载入：羊动画网格 + 附属物（heart/umbrella/floatie）+ shader + 羊叫音效。",
    "fn_notes": {
        ("0x140a093e0",): ("O", "`startup_sheep`：载入 sheep_animation/{1,delighted}, sheep_heart, umbrella, floatie 网格 + `sheep_shader`(solid_vertex_color) + `event:/SFX/Sheep/Baah` + Pitch。47.8KB。"),
    },
    "inline_notes": {},
}

A["startup/startup_sky_and_sun.rs"] = {
    "purpose": "天空/太阳资源载入：skydome mesh + shader。",
    "fn_notes": {
        ("0x140864430",): ("O", "`startup_sky_and_sun`：载入 skydome（`skydome_shader`，来自 systems/backend/vao_update.rs 的 VAO 构建路径）。"),
    },
    "inline_notes": {},
}

A["startup/startup_terrain.rs"] = {
    "purpose": "地形启动：载入地形/岩石/楼梯/鹅卵石/植被网格与 shader，生成地形网格（cw/ccw），spawn 地形实体。",
    "fn_notes": {
        ("0x140926bf0",): ("O", "`startup_terrain`：载入 terrain, rocky_terrain, stairs_step, path_pebble, stairs_pebble, billboard_plants 网格（8× load_mesh_into_library）+ 6 shader（demo_terrain_overlay, rocky_terrain, stairs_step, path_pebble, solid_vertex_color{albedo_map}, billboard_plant）+ 调 grid_gen_cw（`grid cw`）+ spawn 地形实体。"),
        ("0x140928100",): ("O", "`grid_gen_cw`：递归生成地形 quad 网格三角形索引（CW 缠绕；param_1 = cap-first Vec，元素 4B，FUN_1416a83a0 扩容）。"),
        ("0x140928350",): ("O", "`grid_gen_ccw`：同族 CCW 缠绕变体。"),
    },
    "inline_notes": {
        "264": ("I", "`generate_grid_cw` — 生成函数包装（内联）。"),
        "311": ("I", "`generate_grid_ccw` — 生成函数包装（内联）。"),
    },
}

A["startup/startup_time_of_day.rs"] = {
    "purpose": "ToD 启动：载入默认时刻。",
    "fn_notes": {
        ("0x14089d6f0",): ("O", "`startup_time_of_day`：启动时初始化时间（load_default_tod 内联）。"),
    },
    "inline_notes": {
        "10": ("I", "`load_default_tod` — 载入默认 ToD（内联）。"),
    },
}

A["startup/startup_walls.rs"] = {
    "purpose": "墙启动：载入 wall brick LOD shader，生成 half_cube 网格。",
    "fn_notes": {
        ("0x140a92670",): ("O", "`glade_startup_walls`：载入 `wall_brick_lod0`/`wall_brick_lod1` shader（`/wall/wall_brick_lod*.raster.hlsl`）+ 生成 `half_cube`（brick half cube）网格（字节常量 0x01-0x07 为三角索引）。"),
    },
    "inline_notes": {
        "19": ("I", "`once_startup_walls` — 一次性墙初始化（内联）。"),
        "120": ("I", "`create_half_cube_mesh` — 半立方体网格生成器（内联）。"),
    },
}

A["startup/startup_water.rs"] = {
    "purpose": "水面资源载入：water_plane 网格 + 水 shader 族 + 睡莲/芦苇/水石网格。",
    "fn_notes": {
        ("0x1408f1a20",): ("O", "`startup_water`：载入 water_plane, lilypad, lilypad_flower, reed{, _autumn}, water_stones, fallen_leaves, water_grass_winter 网格；shader — water_plane, ice_plane, water_plane_refraction_copy, water_snow_plane, lilypad, lilypad_flower, reed, water_x_paths。"),
    },
    "inline_notes": {},
}

A["startup/glade_themes/flowery.rs"] = {
    "purpose": "Flowery 主题资源载入：主题植物（lavender/低模花）+ 树配置。",
    "fn_notes": {
        ("0x141553120",): ("O", "`startup_flowery`：载入 billboard_plants_2, plants/lavender.json, meadow_lowpoly_flowers 网格 + trees/{rocks,clearing_edge_rocks,stumps,clearing_edge_tree_stumps}.json 树配置。"),
    },
    "inline_notes": {},
}

A["startup/glade_themes/olden.rs"] = {
    "purpose": "Olden 主题资源载入：主题树配置 + clover 花。",
    "fn_notes": {
        ("0x1415539d0",): ("O", "`startup_olden`：载入 trees/{rocks,clearing_edge_rocks,stumps,clearing_edge_tree_stumps,logs}.json 树配置 + clover_flowers 网格。"),
    },
    "inline_notes": {},
}

# ---------------------------------------------------------------------------
# Renderer
# ---------------------------------------------------------------------------
def conf_for(va, name, f):
    if "clone_behavior" in name:
        return ("O", "共享 `clone_behavior` 0x140002370（返回 None，无克隆）。Bevy 胶水。")
    if "register_required_components" in name:
        return reg
    if "visit_" in name or "deserialize" in name or "expecting" in name:
        return ("O", "serde 反序列化胶水（visit/expect 族）。")
    if "serialize" in name or "fmt" in name:
        return ("O", "derive serde/fmt 胶水。")
    key = tuple(x for x in (va,) if x) or None
    # fallback to per-VA notes
    notes = A[f]["fn_notes"]
    for k, v in notes.items():
        if va in k:
            return v
    return ("U", "待分类（无独立反编译证据覆盖）。")

for f, meta in A.items():
    if f not in d:
        continue
    entry = d[f]
    funcs = entry["functions"]
    biz = [x for x in funcs if x["kind"] == "business"]
    inline = [x for x in funcs if x["kind"] == "inline"]
    glue = [x for x in funcs if x["kind"] == "glue"]

    lines = []
    lines.append(f"# {f}")
    lines.append("")
    lines.append(f"> 文件作用：**{meta['purpose']}**")
    lines.append("")
    lines.append(f"函数总计 {len(funcs)}：业务 {len(biz)} / 内联无独立符号 {len(inline)} / 派生胶水 {len(glue)}")
    lines.append("")
    lines.append("## 业务函数")
    lines.append("")
    lines.append("| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |")
    lines.append("|---|---|---|---|---|---|")
    for x in biz:
        name = x["name"]
        va = x.get("va", ["-"])
        va_s = ", ".join(va)
        line = x.get("line", "")
        if va_s == "-" or not va_s:
            va_s = "-"
        # find fn note
        note = ("U", "待分类")
        if va and va[0]:
            for k, v in meta["fn_notes"].items():
                if va[0] in k:
                    note = v
                    break
        conf, desc = note
        # evidence refs
        lines.append(f"| `{name}` | {va_s} | {line} | {desc} | **{conf}** | decomp-{va[0] if va and va[0] else ''}.txt |")
    lines.append("")
    lines.append("## 内联/无独立符号函数")
    lines.append("")
    if inline:
        for x in inline:
            name = x["name"]
            line = str(x.get("line", ""))
            note = meta["inline_notes"].get(line, ("I", "待推断（内联无独立符号）。"))
            conf, desc = note
            lines.append(f"- `{name}`（行 {line}）— **{conf}**：{desc}")
    else:
        lines.append("- （无）")
    lines.append("")
    lines.append("## 派生胶水")
    lines.append("")
    lines.append(f"- {len(glue)} 项：{', '.join(sorted(set(g['name'].split('::')[-1] for g in glue))[:12])} 等（derive 直接复用，不计业务）。")
    lines.append("")

    outp = os.path.join(CH, f.replace("/", os.sep) + ".md")
    os.makedirs(os.path.dirname(outp), exist_ok=True)
    with open(outp, "w", encoding="utf-8") as fh:
        fh.write("\n".join(lines))
    print("wrote", outp)

print("done")
