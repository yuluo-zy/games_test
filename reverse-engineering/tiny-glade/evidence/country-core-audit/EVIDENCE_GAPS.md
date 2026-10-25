# country-core 还缺证据的项（交给分析）

整理日期：2026-10-15。范围：R5–R11 这几轮里"没有落码"或"落码了但依据不足"的项。
地址是 B patched EXE（sha256 前缀 cf048e27acdc266b）的 VA。"需要什么"一栏是解除阻塞所需的分析结论，不是实现方案。
级别沿用 O（机器码直接核对）/ I（推断）/ U（未知）。

## A. 已落码，但依据弱（建议优先补验证）

| 项 | 地址 | 弱在哪 | 需要什么 |
|---|---|---|---|
| `conservative_plot_line`（`utils/conservative_line_raster.rs`） | 0x140b1e460 / 0x142820fb0（逐字节同体）；0x1402582d0（楼梯实例，回调不同） | 算法读自汇编，但没有对原函数做逐格差分；测试只验证性质 | 一组真实输入输出：在原版里取几条线段（含水平、垂直、对角、负斜率、半径 0、坐标越界），记下被置位的瓦片集合；或一个能跑 x86 的模拟器环境（`unicorn`/`capstone` 目前都没有） |
| `assert_delta` 的 panic 文本 | 0x140b84e50 / 0x140b84ec0；格式串 0x142af6f30 | 调用点的分支条件已排除触发，所以没复刻 | 确认是否真的不可达；若可达，读出完整格式串 |
| `store_parameters` 的日志级别 | 0x140911710 | 由"门是 `*max_level != 0`"推出是 Error 级（I） | 确认 `0x140d4a330` 里 Record 的 level 字段 |
| `CameraRigParameters` 字段名 | 0x140911710 | +0x14 / +0x1c / +0x20 的名字取自会话存档字段名（I），值固定写 0 | 与 `0x1403b4ca0` 的 ron 反序列化字段逐项对上 |
| `grid_gen_*` 的 `start > end` panic | 0x1428d9310，位置 `startup_terrain.rs:290:27` | 0x1428d9310 的身份取自另一处同形调用（I），消息按 `Option::unwrap` 写 | 确认该函数确为 `unwrap_failed`，或读出其格式串 |
| `ChildWall` 的 `id` 字段 | 0x140a24000 | `+0` 的 8 字节只是被原样带到结果里，"父墙 id"是猜的（I） | 找到 `ChildWall::new` / 调用方，确认含义 |
| `DebugSphere` 等 `color` 默认值 | 0x140b02490 / 0x140b02e10 / 0x140b04e70 | 构造器只写首个 dword，其余 12 字节在 EXE 里未初始化（取 0） | 确认类型是否为带 niche 的枚举 |
| `DebugRectangle3d::rect` | 0x140b02e10 | 32 字节布局未解 | `show`（0x140b02ed0）里 `FUN_140c99600` 展开成 4 角点的方式 |
| kind ↔ 系统 ↔ 实例类型对应 | 0x142070ac0 … 0x14207fcc0 | 取自 DIGEST（B 阶段 O），本轮没有逐个系统重核；"StoneFloor=9"在工作区枚举里是 `RoofFloor` | 逐系统确认传给 `update_instance_data` 的 kind 常量 |
| `FreeCamera::match_normal` 的 `Smooth` 初值 | 0x14090e6f0 | 第三个 driver 的 0x40 字节镜像与 dolly 的 `Smooth::new_position_rotation(1,1)` 对得上，但没有逐字段核对 | 对一遍 `0x142aece40…` 周围的初始化字节 |

## B. 完全没落码（缺证据或缺依赖）

### B1. 相机（`systems::camera_rig` / `main_camera`）
| 项 | 地址 | 卡在哪 |
|---|---|---|
| `MainCamera::new` | 0x14090ec70（853 行） | 六个 body 的默认值与类型化字段；需要逐 body 对出字段表 |
| `main_camera_update` 的按模式跳表 | 0x140912c20；表 0x142aabd04 / 0x142aabd30 | 各分支反编译有损坏；需要手解每个跳转目标 |
| `paused_main_camera_update` 的 case 6 / 11 | 0x1409145a0 | "bad instruction data"；case 7–10 的 fov 对已记录 |
| `MainCamera::reset` | 0x140911280 | 只有前 64 字节手解 |
| `immediately_finish_interpolating` | 0x140911b50 | 106 KB 粘连函数，需要切分 |
| `chase_butterfly` | 0x140911520 | 未落码；`ActiveRotation` 28 字节元素的字段名 |
| `FreeCamera::translate_screen_space` | 0x1409938b0 | 依赖地形采样 `FUN_1409e9640`（返回两个 f32，语义未解） |
| `WalkCamera::constrain_to_be_above_terrain` / `teleport` | 0x140994320 / 0x140994130 | 两段 spherecast、`ground_hit` 的 `CastHit`；`teleport` 里的方向→欧拉转换 |
| `NormalCamera::update/constrain/translate` | 0x140949fe0 / 0x140949dc0 / 0x140949e70 | body 字段偏移与 DIGEST 的 yaw/pitch 有矛盾（已按 `store_parameters` 取 pitch +0x150 / yaw +0x154），需要整体重核 |
| `CameraSnapshot` 插值 | 0x140911e40 | PDB 名与行为错位（实为按模式快照分发） |
| `CameraMode` 6–12 的名字 | — | 只知道复用 6 个 body 与各自 fov 槽；`next` 没定位 |

### B2. nani
| 项 | 说明 |
|---|---|
| 19 个 gap 组件的字段布局 | `Wren`、`Plaster`、`Smoke`、`ClothlineCloth`、`NormalMap`、`StairsFloorMeshChunk`、`GothicWindowBrickDeformation`、`Roof`/`Floor`/`RoofAnimation`、`LanternLightParams`、`BrickColor`、`NaniSolidTexture`、`WindowLight`、`AlbedoMap`、`PrevTransform`……目前 `build_*` 只接收原始字 |
| `circle_tangent_u0` | `Circle2d::get_tangent_at_u(0.0)`，0x140c97660，未恢复（当前由调用方传入） |
| `terrain_rock_mask_words` | `TerrainRockMaskSsbo` 的访问器；"有行时未注册"原版 unwrap panic 的路径 |
| `startup_nani` / `reload_nani` / `load_mesh_atlases` / `reload_mesh_src` | 0x140a398b0（`reload_nani`，1458 B）等 |
| `RemovedComponents` 里 "generation==0 跳过" | 在 bevy 0.16 的 `Entity` 上没有对应物，需要确认原版里是怎么表达的 |
| External subset 的 `draw_count` | gpu_world 侧 U 项 |

### B3. 世界栅格 / 地形
| 项 | 地址 | 卡在哪 |
|---|---|---|
| `draw_raster` 三个实例 | 0x142260ae0（水）/ 0x140aba770 / 0x140ab4750（路径） | 672 行；含 SSBO 名称校验、路径字符串处理、事件队列；需要先拆出"状态机"与"胶水" |
| `cell_from_world_space` | 0x140960b20 | 三次 `fmaf`、四通道输出，参数/返回布局没对上 |
| `WorldRaster` 的 `+0x29b`（reupload）置位点 | — | setter 未知 |
| `mouse_terrain_raycast`、`TerrainRockMask` | — | 未做 |
| `WallColorIdSsbo` 默认表 | `color_id_roof::default_value` 0x14099ca50 | 未读 |
| 渲染 pass 里的栅格旗标替身 | `systems/render_loop/compute_passes/upload_world_raster.rs` | 与新的 `world_raster_defs` 类型未接线（工程问题，不是证据问题） |

### B4. startup（约 60 个函数 / 158 KB）
全部是 ECS 系统或资产库调用链。大块：`load_prefabs`（含 `reload_prefabs` 0x1412a1a60、`PrefabWatcher`）、`startup_terrain` 本体（0x140926bf0，载入 8 个网格 + 6 个着色器 + 生成实体）、`startup_walls` 0x140a92670、`startup_roof` 0x1414547d0、`startup_ivy` 0x140a21a30、`startup_grass` 0x140a21620、`startup_garden_brush` 0x1415f87c0、`glade_themes::flowery` 0x141553120、`load_glade_*`、`load_audio`、`load_shaders`、`load_textures`、`startup_session`、`clearing`。
需要的前置：组件层（H 包）的字段布局，以及 `Commands` / `Assets` 的使用面。

### B5. utils
| 项 | 说明 |
|---|---|
| `get_last_event` / `drain_events` | 0x1402b4910 / 0x1402b47f0：看起来是 bevy `World` 资源访问 + `Events<T>::drain`，需要确认并决定落在哪层 |
| `audio_utils`（`AudioLoop` 等） | 0x140a59220 / 0x140a593f0 / 0x140a59430 / 0x140a596b0；依赖 kira |
| `debug_viz::*::show` 及闭包 | 0x140b02520 / 0x140b02ed0 / 0x140b04640 / 0x140b04f70；`Commands` + `AssetMeshLibrary` / `AssetShaderLibrary` |
| `DebugRectangle2d` | 没有构造器符号 |

### B6. 非渲染大块（尚未开始，按未覆盖字节排序）
`systems::wall` 约 333 KB、`systems::save` 约 183 KB、`systems::audio` 约 151 KB、`resources::walls` 约 106 KB、`systems::collision` 约 98 KB、`systems::ui` 约 46 KB、`systems::ivy` 约 39 KB、`systems::backend` 约 34 KB、`systems::ui_clutter` 约 30 KB、`systems::terrain` 约 29 KB、`systems::history` 约 29 KB、`systems::steam` 约 27 KB、`resources::stairs` 约 24 KB。
（数字来自本会话开头的自上而下统计，按"被工作区引用"的字节算，是下限；建筑相关代码在 `recovered_building` 里，引用格式不同，会被低估。）

## C. 分析文档本身的错误（已核对，供你回写 DIGEST）
| 位置 | 文档说 | 实际 |
|---|---|---|
| utils DIGEST §3.3 | 0x141405930 是"自定义 JSON 解析" | 是 `serde_json::de::from_str`（R6） |
| utils DIGEST §3.8 `ChildWall::slice` | 两值各自 `f32::clamp` | helper 0x140c904d0 是 `a + (b-a)*t`（lerp），结果检查 [0,1]（R7） |
| utils DIGEST §3.7 | `get_last_event` 是"自定义 EventRegistry" | 是 bevy `World` 资源访问 + `Events::drain`（R7，I） |
| camera-time DIGEST §2.1 Normal body | yaw +0x50 / pitch +0x54 | `store_parameters` 与 `match_free_camera_to_normal` 都证明 pitch +0x150、yaw +0x154（R9） |
| camera-time DIGEST §2.3 `previous` | 8 项表 | 移位按低 6 位取模，模式 8–12 回绕到 `mode & 7`（R9） |
| 旧 wp6 `render/nani/systems.rs` | `nani_maintain_instances` 等 | 是带 `todo!()` 的骨架，不是恢复结果（R8） |
| render-nani DIGEST | StoneFloor = kind 9 | 工作区枚举里 9 是 `RoofFloor`（名字待核） |

## D. 流程上还悬着的
* `render-source-map.json` 里 5 个早于本轮的陈旧哈希，未处理。
* `tg-history` 在 clippy 1.96 下有一条 lint。
* 本机 cargo 镜像（USTC）配置在 `/tmp/m-ustc.toml`，没有持久化。
