# startup/mod.rs

> 文件作用：**startup 模块总入口：`add_game_startup_systems`（GameStartup 插件，注册全部启动系统）+ shader/mesh 入库 loader（load_shader_into_library、load_mesh_into_library）。**

函数总计 9：业务 6 / 内联无独立符号 1 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::impl$0::add_game_startup_systems` | 0x140941110 | 107 | `impl$0::add_game_startup_systems`：GameStartup 插件 build — 依次注册 startup 系统（FUN_140b67620/FUN_140b66830/FUN_140b66480 等 Bevy app.add_systems），缺注册 panic。 | **O** | decomp-0x140941110.txt |
| `country_core::startup::impl$1::build` | 0x140b1ac90 | 206 | `impl$1::build`：`mov rcx,rdx; jmp 0x140b65f80` 尾调用共享 build（子插件 build 委托；目标 = Hash 0x8e1bd251962ba784 查表族，同 add_game_startup_systems 尾部）。 | **O** | hex 手解（rne-read+capstone，8B）→ startup/decomp/decomp-0x140b1ac90.txt；原 tg_b2 NotFound |
| `country_core::startup::load_shader_into_library<alloc::string::String>` | 0x140b40210 | 220 | `load_shader_into_library<String>`：载入 shader（源码读取 + 编译）入库，返回句柄。 | **O** | decomp-0x140b40210.txt |
| `country_core::startup::load_shader_into_library<ref$<str$> >` | 0x140b3fe40 | 220 | `load_shader_into_library<&str>`：&str 变体（多数 startup 载入走此）。 | **O** | decomp-0x140b3fe40.txt |
| `country_core::startup::load_shader_into_library_replace_previous<ref$<str$> >` | 0x140b405e0 | 244 | `load_shader_into_library_replace_previous<&str>`：载入并替换旧 shader 的变体。 | **O** | decomp-0x140b405e0.txt |
| `country_core::startup::load_mesh_into_library` | 0x140b1aca0 | 268 | `load_mesh_into_library`：载入网格入库（JSON 网格解析 + 注册；多个 startup 载入者共用）。 | **O** | decomp-0x140b1aca0.txt |

## 内联/无独立符号函数

- `country_core::startup::load_shader_into_library`（行 220）— **I**：`load_shader_into_library` — 泛型分派（内联）。

## 派生胶水

- 2 项：closure$0, closure$1 等（derive 直接复用，不计业务）。

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
