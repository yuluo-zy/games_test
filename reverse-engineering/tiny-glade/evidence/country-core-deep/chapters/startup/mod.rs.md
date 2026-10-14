# startup/mod.rs

> 文件作用：**startup 模块总入口：`add_game_startup_systems`（GameStartup 插件，注册全部启动系统）+ shader/mesh 入库 loader（load_shader_into_library、load_mesh_into_library）。**

函数总计 9：业务 6 / 内联无独立符号 1 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::impl$0::add_game_startup_systems` | 0x140941110 | 107 | `impl$0::add_game_startup_systems`：GameStartup 插件 build — 依次注册 startup 系统（FUN_140b67620/FUN_140b66830/FUN_140b66480 等 Bevy app.add_systems），缺注册 panic。 | **O** | decomp-0x140941110.txt |
| `country_core::startup::impl$1::build` | 0x140b1ac90 | 206 | `impl$1::build`：`mov rcx,rdx; jmp` 尾调用共享 build（子插件 build 委托）。 | **O** | decomp-0x140b1ac90.txt |
| `country_core::startup::load_shader_into_library<alloc::string::String>` | 0x140b40210 | 220 | `load_shader_into_library<String>`：载入 shader（源码读取 + 编译）入库，返回句柄。 | **O** | decomp-0x140b40210.txt |
| `country_core::startup::load_shader_into_library<ref$<str$> >` | 0x140b3fe40 | 220 | `load_shader_into_library<&str>`：&str 变体（多数 startup 载入走此）。 | **O** | decomp-0x140b3fe40.txt |
| `country_core::startup::load_shader_into_library_replace_previous<ref$<str$> >` | 0x140b405e0 | 244 | `load_shader_into_library_replace_previous<&str>`：载入并替换旧 shader 的变体。 | **O** | decomp-0x140b405e0.txt |
| `country_core::startup::load_mesh_into_library` | 0x140b1aca0 | 268 | `load_mesh_into_library`：载入网格入库（JSON 网格解析 + 注册；多个 startup 载入者共用）。 | **O** | decomp-0x140b1aca0.txt |

## 内联/无独立符号函数

- `country_core::startup::load_shader_into_library`（行 220）— **I**：`load_shader_into_library` — 泛型分派（内联）。

## 派生胶水

- 2 项：closure$0, closure$1 等（derive 直接复用，不计业务）。
