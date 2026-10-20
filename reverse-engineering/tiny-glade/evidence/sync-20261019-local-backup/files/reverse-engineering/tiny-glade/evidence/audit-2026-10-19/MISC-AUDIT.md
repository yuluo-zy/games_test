# country-core B 版逐文件还原度复核（只读审计）

- 日期：2026-10-19
- 代码：`D:/game/tiny-glade/crates/country-core`（67 个在册文件，13,314 行）
- 骨架：`D:/game/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src`
- 机械统计：`evidence/audit-2026-10-19/fn_audit.json`（2,380 条符号；本范围有骨架符号的文件 41 个）
- 基线：`recovery/STUBS-2026-10-19.md`（只作线索，本文以源码与骨架符号为准）
- 限制：未运行 cargo、未做原侧 runtime oracle、未修改任何源码；"无对应函数"按全 workspace `*.rs` 名称/语义检索判定，仍可能有改名等价物，标"需人工"。

## 0. 结论摘要

1. 在册 67 文件全部有实质内容：**范围内 `todo!`/`unimplemented!` = 0，空函数体 = 0**。范围外（render/pass_builder、world_raster 等）仍有 5 处 todo/panic 占位，不计入本审计。
2. 真正的水分不在空壳，而在**缺失文件 + 未接线 + 契约占位**：
   - 骨架 startup 下 17 个文件在重建树中不存在（clearing、debug、themes×2、load_audio、load_glade_grading、panic_reporter_startup、spawn_brush_previews、spawn_human_ref、spawn_preplaced_trees、startup_critters/meadow/roof/sheep/sky_and_sun/time_of_day/walls）。
   - `game_key_bindings.rs` 整文件缺失；仅 `GameKeyBindings` 数据类型并入 `game_profile.rs`，7 个绑定 UI/查询函数无实现。
   - `startup_water`、`startup_ivy`、`startup_garden_brush` 有真实实现但宿主未接线；`add_game_startup_systems` 全 workspace 无调用者。
   - `rhapsody_contract`/`rhapsody_types` 大量编译桥仍在 panic/占位。
3. 机械可追踪率（骨架符号 in_file+moved+va_only）：**130/238 = 54.6%**；扣除闭包/泛型单态化/derive 后，业务函数存在率约 **89%（212/238）**，但后者含 18 个仅名字/部分实现，且语义验收未做。
4. 文件存在率：重建 67 / 原始同范围约 85 = **约 79%**。
5. 综合完成度：**约 78%–82%（中置信度）**。依据、分解口径与置信度见 §9。

## 1. 文件级状态表

性质：声明 / 数据合同 / 真实行为 / 混合 / 数据清单 / 测试。状态：完整、基本完整、部分、未接线。

### 根目录（11 个在册；`game_key_bindings.rs` 缺失）

| 文件 | 行 | 性质 | 状态与说明 |
|---|---:|---|---|
| `app_loading_state.rs` | 13 | 数据合同 | 完整：三态 State，真实 Bevy 身份 |
| `game_config.rs` | 240 | 混合 | 基本完整：serde 布局+Default+upgrade；`border_signifier_enabled` 是登记 seam |
| `game_profile.rs` | 905 | 混合 | 基本完整：档案 serde/默认/存盘真实；ron 形状与 hint 区命名仍待取证 |
| `lib.rs` | 65 | 声明 | 完整模块树；`system` 子模块声明，无行为 |
| `meta_commander.rs` | 451 | 真实行为 | 基本完整：7 字节载体、fence、update 分派真实；profiler/截图类命令显式拒绝 |
| `name_hash.rs` | 32 | 真实行为 | 完整：复用 xxh3 |
| `reused.rs` | 20 | 声明 | 完整：转发表 |
| `rhapsody_contract.rs` | 435 | 混合 | 部分：设备真实转发；句柄/PassBuilder/RenderGraph 为 panic 桥 |
| `rhapsody_types.rs` | 485 | 数据合同/占位 | 部分：TextureDesc/ViewDesc 真实；Texture/Pipeline/ImportedBuffer 占位 |
| `frozen_map.rs` | 89 | 真实行为 | 完整：RwLock 实现真实契约（模块注释称"占位"，易误读） |
| `utils_contract.rs` | 4 | 声明 | 完整：共享 utils 类型转发 |

### startup（22 个在册 + 17 个骨架文件缺失）

| 文件 | 行 | 性质 | 状态与说明 |
|---|---:|---|---|
| `init_color_library.rs` | 19 | 真实行为 | 完整（薄）：默认调色板上传+配置监听 |
| `init_photomode_state.rs` | 94 | 真实行为/数据 | 完整：构造与进入重置 |
| `init_populate_minimal_startup_ssbo_library.rs` | 59 | 真实行为 | 完整：loading App 两项 SSBO |
| `init_populate_ssbo_library.rs` | 252 | 真实行为 | 完整：72 项顺序/构造器/游标事件 |
| `load_glade_audio.rs` | 125 | 真实行为 | 完整：主题曲库/参数；但 ECS 包装 `configure_glade_audio` 全 workspace 无调用者 |
| `load_glade_tod_and_weather.rs` | 29 | 真实行为 | 部分：TOD/天气替换真实；`load_snow_particles` 缺失 |
| `load_mesh_atlases.rs` | 297 | 真实行为 | 基本完整：RON/预制件/5 族程序网格/重载 |
| `load_prefabs.rs` | 555 | 真实行为 | 基本完整：扫描/热更/RT/audio/canopy 处理 |
| `load_shaders.rs` | 79 | 真实行为 | 完整：UI 6 + 通用 13 注册 |
| `load_textures.rs` | 95 | 真实行为（谓词） | 部分：只有 `should_load`；纹理映射加载缺失 |
| `mod.rs` | 77 | 混合 | 部分：注册器只装 ivy+garden_brush；`add_game_startup_systems` 无调用者 |
| `run_game.rs` | 195 | 真实行为 | 部分：核心资源/session/update 装配真实，非完整原 Startup 元组 |
| `startup_blit.rs` | 26 | 真实行为 | 完整（薄）：3 个 blit shader；宿主已接 |
| `startup_deferred_raster.rs` | 92 | 真实行为 | 完整：4 材质槽保序；宿主已接 |
| `startup_garden_brush.rs` | 148 | 混合 | 部分：ECS 入口只注册网格；shader/实体走 Host trait，未接线 |
| `startup_grass.rs` | 147 | 真实行为 | 完整：27 索引草叶+两 drawable；宿主 OnEnter(Game) 已接 |
| `startup_ivy.rs` | 19 | 真实行为 | 完整（薄）；**未接线** |
| `startup_nani.rs` | 239 | 真实行为 | 完整：27 shader/subset/visibility；宿主已接 |
| `startup_session.rs` | 381 | 真实行为 | 完整：起始场景/TOD/会话发布 |
| `startup_terrain.rs` | 532 | 真实行为 | 完整：grid/floor/rocky/stairs/terrain 实体；宿主已接 |
| `startup_water.rs` | 399 | 真实行为 | 完整但**未接线**：水/冰/季节植被全实现，workspace 无调用 |
| `tests.rs` | 466 | 测试 | 仅测试，不计业务 |

### input（5）

| 文件 | 行 | 性质 | 状态 |
|---|---:|---|---|
| `action_set.rs` | 437 | 真实行为 | 完整：40 数字/6 模拟槽、12B 协议、serde |
| `input_stream.rs` | 283 | 真实行为 | 基本完整：快照/裁剪/结构校验；`add_frame` 未同名，疑并入 `record_frame`（需人工） |
| `mod.rs` | 168 | 混合 | 完整：模式/冷却/niche |
| `panic_repro_state.rs` | 28 | 真实行为 | 完整（薄）：仅判别字节 |
| `recording.rs` | 252 | 真实行为 | 基本完整：v2 文件协议；原完整协议/旧格式兼容需人工 |

### components（5）

| 文件 | 行 | 性质 | 状态 |
|---|---:|---|---|
| `drawable.rs` | 132 | 数据合同 | 基本完整：Bundle/枚举/布局；注释自认 `cache_prev_object_transforms` 与完整材质组件族未恢复 |
| `mod.rs` | 29 | 声明 | 完整：导出/标记 |
| `registry.rs` | 421 | 数据清单 | 完整但不含运行时：57 组件登记表 |
| `rendering.rs` | 41 | 数据合同 | 完整：7 个标记/参数组件 |
| `trees.rs` | 103 | 数据合同+行为 | 完整：树组件与 `TreeAnimation::advance` |
| `ui.rs` | 51 | 数据合同 | 完整：5 个 UI 组件 |

### geometry（6 + 5 测试）

| 文件 | 行 | 性质 | 状态/使用 |
|---|---:|---|---|
| `cube.rs` | 101 | 真实行为 | 完整；`systems/prefab.rs:340` 使用 |
| `instanced_wall.rs` | 283 | 真实行为 | 完整；wall 系统与 SSBO 生产者使用 |
| `mod.rs` | 6 | 声明 | 完整 |
| `plane.rs` | 36 | 真实行为 | 完整；**crate 内无消费者**（仅测试） |
| `sphere.rs` | 129 | 真实行为 | 完整；crate 内仅 `DebugSphere` 参数使用，网格无消费者 |
| `runtime.rs` | 104 | 真实行为 | 完整；`asset_libraries/runtime.rs` 注册 |
| `tests/{cube,instanced_wall,plane,runtime,sphere}.rs` | 118/292/45/269/99 | 测试 | 仅测试 |

### utils（12）

| 文件 | 行 | 性质 | 状态 |
|---|---:|---|---|
| `audio_utils.rs` | 303 | 真实行为 | 完整：环形音效状态机 |
| `child_wall.rs` | 68 | 真实行为 | 完整 |
| `components/transform.rs` | 140 | 真实行为/合同 | 完整：布局/TRS/look_at |
| `conservative_line_raster.rs` | 262 | 真实行为 | 完整：有界保守栅格 |
| `debug_viz.rs` | 530 | 真实行为 | 完整：DebugSphere/Rectangle/Line→signifier |
| `dirty_tiles.rs` | 416 | 真实行为 | 完整：64×64 位图/笔画/tabloid |
| `float_grid.rs` | 119 | 声明/转发 | 完整：委托共享 utils |
| `geo.rs` | 164 | 真实行为 | 完整：折线/闭合曲线改接 |
| `index_condenser.rs` | 120 | 真实行为 | 完整：委托共享 condenser |
| `load_json.rs` | 291 | 真实行为 | 完整：JSON 网格解析/校验 |
| `mod.rs` | 113 | 混合 | 完整：资源/事件/坐标工具 + 模块导出 |
| `project_cursor_onto_ray.rs` | 297 | 真实行为 | 完整：屏幕→射线投影 |

## 2. 空壳与水分（文件:行）

### 2.1 范围内 todo/空体

- `todo!`/`unimplemented!`：**0**。
- 空函数体：**0**（正则扫描 `fn ... {}`；范围内无命中）。
- 仅测试文件：`startup/tests.rs`、`geometry/tests/*`（不计业务）。

### 2.2 明确的占位/panic 桥

| 位置 | 内容 | 性质 |
|---|---|---|
| `src/rhapsody_contract.rs:3` | "其他尚未完成的 ABI 兼容边界…" | 自述缺口 |
| `src/rhapsody_contract.rs:186-192` | `full_barrier_required` 只 `log::warn` 并返回 `false` | 登记缺口 §4.3，非真实实现 |
| `src/rhapsody_contract.rs:239-241` | `Handle::get` → `panic!("Managed handle backend is not connected")` | 编译桥 |
| `src/rhapsody_contract.rs:261-263` | `ReadOnlyHandle::get` → panic | 编译桥 |
| `src/rhapsody_contract.rs:331-336` | `bind()` 生成占位绑定（view=None） | 游戏侧声明占位 |
| `src/rhapsody_contract.rs:349-355` | `PassBuilder::add_resource_access` → panic | detached builder |
| `src/rhapsody_contract.rs:357-359` | `PassBuilder::resolve_resource` → panic | typed gap 待取证 |
| `src/rhapsody_contract.rs:361-364` | `PassBuilder::render` → panic | 未接 recorder |
| `src/rhapsody_types.rs:1-8` | 自述"唯一占位类型" | 占位总说明 |
| `src/rhapsody_types.rs:199-213` | `Texture { id,width,height,format }`，`handle()` 返回 id 低 32 位（U） | 占位 |
| `src/rhapsody_types.rs:219-221` | `CompiledShaderPipeline(Vec<u8>)` | 字节容器占位 |
| `src/rhapsody_types.rs:223-229` | `RasterPipelineDesc` 字段 U | 仅供比较 |
| `src/rhapsody_types.rs:239-250` | `RasterPipeline` / `ComputePipeline` 内容 U | 占位 |
| `src/rhapsody_types.rs:260-266` | `ImportedBuffer` 槽表未恢复 | 编译桥 |
| `src/rhapsody_types.rs:268-336` | `RenderGraph` 无 backend 时 test-only / 生产 panic | 适配桥 |
| `src/components/drawable.rs:7-8` | "仍未…恢复 cache_prev_object_transforms 系统和完整的材质组件族" | 明确未恢复 |
| `src/game_profile.rs:448` | "ron 序列化形状与原版一致性待取证" | 待取证 |
| `src/meta_commander.rs:96-98` | profiler sinks 未接真实消费者 | 登记缺口 |
| `src/meta_commander.rs:145-154` | 8/9/12-16/≥20 命令保留并报错 | 有意的未支持分支 |
| `src/components/registry.rs:4` | "不是运行时注册器" | 数据清单，非运行行为 |
| `src/frozen_map.rs:1` | 注释称"调用面占位" | **误标**：实现为真实契约 |
| `src/utils/mod.rs:1-2` | "共享数学、采样和索引算法委托 utils crate" | 委托，不是空壳 |
| `src/startup/README.md:46` | "网格、AABB 和部分音频字段仍是未接入的恢复占位" | 文档登记 |
| `src/startup/grass.rs:64` | "legal watcher no-op" | 合法空操作，不是缺口 |

### 2.3 "薄但真实"

`startup_ivy.rs`（19 行）、`startup_blit.rs`（26 行）、`init_color_library.rs`（19 行）、`load_glade_tod_and_weather.rs`（29 行）、`panic_repro_state.rs`（28 行）都只有单一真实动作，不是签名壳；但前两者未接线。

## 3. 对照骨架：no_trace 分类

判定口径：`in_file`=名字+地址在重建代码；`moved`=地址出现在其它类型/模块；`name_only`=只有名字；`no_trace`=全 crate 名字和地址都没有。下面把 no_trace 再分为"业务缺失 / 仅泛型单态化或派生胶水 / 已改名搬移"。

### 3.1 startup 24 个 no_trace

| 符号 | 地址 | 判定 | 证据 |
|---|---|---|---|
| `init_clearing` | 0x140a78aa0 | **真缺失** | `asset_libraries/audio_trigger_library.rs:81` 仅注释；无函数 |
| `load_audio` | 0x140b0e0f0 | **真缺失** | 与主题音乐 `load_glade_audio` 不同；全 workspace 无同名 |
| `load_glade_grading` | 0x1408bf9c0 | **真缺失** | 只有 `resources/grading_lut.rs` LUT 资源，无启动加载器 |
| `load_snow_particles` | 0x140ae7d90 | **真缺失** | 仅 `draw_lists.rs:934` 的 shader 名 |
| `load_glade_texture_mappings` | 0x1409e6b60 | **真缺失** | `load_textures.rs` 只恢复 `should_load` |
| `spawn_brush_previews` | 0x140924b40 | **真缺失** | `systems/brush_preview.rs` 只有更新/隐藏 |
| `spawn_human_ref` | 0x14089d5a0 | **真缺失** | 全 workspace 无 `human_ref` |
| `startup_critters` | 0x1409d49a0 | **真缺失** | critter SSBO/可见性存在，无启动 spawn |
| `init_meadow` | 0x1408639f0 | **真缺失** | "meadow" 仅 shader 字符串 |
| `startup_roof` | 0x1414547d0 | **真缺失** | roofs 资源存在，无启动生产者 |
| `startup_sky_and_sun` | 0x140864430 | **真缺失** | 只有 `systems/time_of_day.rs` 太阳计算 |
| `startup_flowery` | 0x141553120 | **真缺失** | 只有 GladeType/颜色库枚举 |
| `startup_olden` | 0x1415539d0 | **真缺失** | 同上 |
| `glade_startup_walls` | 0x140a92670 | **已搬移** | `systems/wall/wall_state/publish_public_wall_state.rs:106 once_startup_walls`，宿主已接 |
| `Error>` ×2 | 0x140a14620 / 0x140a2f670 | 派生/泛型 | serde/lang 类型参数 |
| `Error>` | 0x1415f8c40 | 派生 | enum visitor |
| `closure_env$0/1>` | 0x1415f7190 / 0x1415f5b60 | 内联闭包 | 由 `load_nani_meshes_if` 等实现 |
| `closure_env$1/2>` | 0x140a08680 / 0x140a08d30 | 内联闭包 | `process_prefab_mesh`/filter |
| `closure$0` | 0x140922830 | 内联闭包 | prefab 依赖回调 |
| `String>` | 0x140b40210 | 泛型 | shader 名类型参数 |
| `load_shader_into_library_replace_previous<str>` | 0x140b405e0 | 合并 | `startup_water::install_shader(..., replace=true)` 等价 |
| `String>` 另计 | — | — | — |

补充 name_only（有名字无地址/无体）：`uninit_clearing`、`debug`、`spawn_preplaced_trees`、`startup_sheep`、`startup_time_of_day` 均**无实现**；`startup/mod.rs::build` 为泛型胶水。

### 3.2 components 18 个 no_trace

| 组 | 数量 | 判定 |
|---|---:|---|
| `drawable.rs` 的 `*InstanceData>` / `DrawableMeshBundle>` / `DesignatedRenderPass>` / `PrevTransform>` | 15 | **泛型单态化/派生**。对应类型均在 `render/nani/instance_data.rs`（宏生成）、`draw_passes/draw_lists.rs`、`draw_callbacks_a_views.rs`；无业务函数缺失 |
| `mod.rs Global>` | 1 | 派生/分配器类型参数 |
| `transient_mesh.rs closure_env$0>`、`deferred_delete_mesh` | 2 | **搬移**：`asset_libraries/transient_mesh.rs:84 delete_dropped_transient_meshes`；组件本身在 `asset_libraries/transient_mesh.rs:18` |

结论：components 18 个 no_trace **0 个真缺失**。但 `components/drawable.rs:7-8` 的材质组件族/cache_prev 系统仍是真实缺口（不在该 18 条内）。

### 3.3 utils 20 个 no_trace

| 符号组 | 数量 | 判定 |
|---|---:|---|
| `debug_viz::closure$0/$1` | 2 | 内联于 `DebugLine::try_show`，已实现 |
| `dirty_tiles` 的 5 个 `*PathFilter>`/`closure_env$0/$2>` | 6 | `rasterize_world_space_stroke` 的泛型单态化；filter 类型在 `systems/history/replay_path`（范围外），非 dirty_tiles 缺失 |
| `dirty_tiles::closure_env$1>` | 1 | 内联闭包 |
| `intersects_world_space_rect` | 1 | **改名**：`utils/dirty_tiles.rs:219 intersects_world_rect` |
| `geo Global>`/`Vec3>`/`closure_env$1>` | 3 | 泛型/闭包胶水 |
| `load_json String>`/`PathBuf>`/`load_json_as_mesh<str>` | 3 | 泛型单态化；函数在 `utils/load_json.rs:92/97` |
| `utils/mod OsCursor*/CustomCursor*/OsWindowCmd>` | 4 | **搬移**：`resources/mouse_capture.rs:24-46` |

结论：utils 20 个 no_trace **0 个真缺失**。

### 3.4 game_key_bindings（实际 9 个 no_trace + 1 name_only）

任务书写"6 个"，`fn_audit.json` 实际为 **9 no_trace + 1 name_only**。

| 符号 | 地址 | 判定 |
|---|---|---|
| `action_desc` | 0x14089cb60 | **真缺失**：无按键动作本地化描述 |
| `mouse_button_to_string` | 0x14089c970 | **真缺失** |
| `assign_mouse_button` | 0x14089b280 | **真缺失** |
| `assign_key` | 0x14089b2a0 | **真缺失** |
| `get_mouse_button` | 0x14089c880 | **真缺失** |
| `get_keycode` | 0x14089d300 | **真缺失** |
| `keycode_to_str` | 0x14089d490 | **真缺失** |
| `Error>`/`String>` | 0x140366a40/0x140831740 | 派生/泛型 |
| `default`（name_only） | 0x14089b110 | 默认值并入 `game_profile.rs:30 GameKeyBindings::default` |

`game_key_bindings.rs` 文件本身不存在；`game_profile.rs` 只恢复了持久化 schema，绑定 UI 层未恢复。

## 4. 抽读 12 个最大文件结论

阅读文件（按行数）：`game_profile.rs` 905、`load_prefabs.rs` 555、`startup_terrain.rs` 532、`debug_viz.rs` 530、`rhapsody_types.rs` 485、`startup/tests.rs` 466、`meta_commander.rs` 451、`action_set.rs` 437、`rhapsody_contract.rs` 435、`components/registry.rs` 421、`dirty_tiles.rs` 416、`startup_water.rs` 399。

1. **`game_profile.rs` 是真实现，不是签名壳。** 40 个具名键位双向 serde、`desktop_keycode_index` 194 项表、load/save 临时文件、18 路 tutorial 标志与 reset/set-all 均有字节/地址注释和测试（如 623-765、878-904）。
2. **`load_prefabs.rs` 是真实现。** 扫描→JSON mesh→canopy/trunk 顶点色过滤→quad 法线重算→RT/audio 独立 bounds→库提交→watcher 注册完整（160-292、391-500）。
3. **`startup_terrain.rs` 是真实启动生产者。** 90×90 floor、demo overlay、rocky SSBO/资源、stairs flags 0/1、terrain/rocks/billboard 实体都真实 spawn（186-337），测试验证共享 mesh 与实体数。
4. **`debug_viz.rs` 是真实现。** 不只颜色表：`DebugLine::try_show` 校验→哈希身份→signifier factory→真实 spawn/TransientMesh/DashedLineLength（192-307）。
5. **`meta_commander.rs` 是真实现，且有明确未支持面。** MetaCommand 编解码/fence/session load/update 分派真实；profiler/screenshot 等保留并返回错误（136-197），不是静默吞掉。
6. **`action_set.rs` 是真实现。** 40/6 槽、12B 记录、serde 命名映射与重复/越界校验完整（114-247）；测试覆盖重构与非法输入。
7. **`dirty_tiles.rs` 是真实现。** 64×64 位图、区域/笔画栅格、tabloid delta OR 合并均真实（156-257）；`intersects_world_space_rect` 仅改名。
8. **`startup_water.rs` 是真实现但未接线。** 水/冰/折射/雪/睡莲/芦苇/花/石全按季节 spawn（95-214）；workspace 内无调用者（`startup/mod.rs` 只声明模块）。
9. **`components/registry.rs` 不是行为实现。** 57 条组件证据表 + 空 required 集合，模块自述"不是运行时注册器"（4）；不能按 57 个具名条目计完成。
10. **`rhapsody_types.rs` 是混合体。** TextureDesc/ViewDesc 的构造器与字节布局有地址依据（16-197），但 Texture/Pipeline/ImportedBuffer/RenderGraph 是编译桥或 panic（199-336）。
11. **`rhapsody_contract.rs` 的设备包装真实，句柄/图是桥。** 10+ 个 GraphicsDevice 方法与 create_buffer/texture 转发真实（45-180）；Handle/PassBuilder/RenderGraph 不可用（184-381）。
12. **`startup/tests.rs` 是测试，不是业务。** 466 行覆盖初始化/重载/fence/audio/grid 等，不能作为完成度分子。

## 5. 启动装配：哪些真、哪些仅签名/未接

### 真实且已接线（宿主 `tiny-glade` 调用）

| 功能 | 接线点 |
|---|---|
| 72 项 SSBO 生产者 | `gameplay_startup.rs:55`（PreStartup Inputs） |
| loading App 两项 SSBO | `loading_graph.rs:51` |
| prefab/atlas/Nani | `gameplay_startup.rs:64-82` |
| terrain / grass | `world_terrain.rs:455 / 301` |
| 通用/UI/blit shader | `production_renderer.rs:62-64` |
| deferred raster | `world_lighting.rs:251` |
| 调色板 / 纹理白名单 | `loading_graph.rs:99/111` |
| TOD/天气 | `run_game.rs:455` |
| photomode 状态 | `systems/backend/photomode.rs:47-48` |
| 主题音乐 | `systems/audio/music/load_glade_audio.rs:22-23` 有 ECS 入口，但 `configure_glade_audio` 无调用者 |
| session/profile/meta/walls | `run_game.rs:246/385/444/515/606` |

### 真实但未接线

- `startup_water::startup_water`：全 workspace 无调用（只有注释）。
- `systems/audio/music/load_glade_audio::configure_glade_audio`：真实包装，但全 workspace 无调用（主题音乐 ECS 入口未接）。
- `startup_ivy::startup_ivy`、`startup_garden_brush::startup_garden_brush`：仅被 `startup/mod.rs:70-76` 注册；`add_game_startup_systems` 本身无调用者（`tiny-glade/src/game_loop.rs:262` 只是注释）。
- `startup/mod.rs::add_game_startup_systems`：真实函数、零调用。

### 仅签名/部分

- `load_textures.rs`：只有 `should_load` 谓词；原 `load_textures`/`load_glade_texture_mappings` 加载器未恢复。
- `load_glade_tod_and_weather.rs`：只有 TOD/天气替换；`load_snow_particles` 未恢复。
- `startup/mod.rs::load_shader_into_library_replace_previous`：未单独存在，等价 replace 标志合并在各 install_shader。
- `run_game::register_startup_systems`：只装 session list 刷新 + 音乐初始化，不是原 Startup 的完整 8 项启动元组。
- `add_game_startup_systems` 的注册表明显缩水：骨架 `startup/mod.rs:1077` 的闭包环境类型显示原元组含 `PhotomodeState`、`TodSequence/Weather/GladeSettings/ActiveSession`、terrain/Nani/纹理等多组系统；重建 `startup/mod.rs:70-76` 只有 ivy + garden_brush，其余被分散到宿主或未接。

### 完全缺失（文件级）

`clearing`、`debug`、`glade_themes/{flowery,olden}`、`load_audio`、`load_glade_grading`、`panic_reporter_startup`、`spawn_brush_previews`、`spawn_human_ref`、`spawn_preplaced_trees`、`startup_critters`、`startup_meadow`、`startup_roof`、`startup_sheep`、`startup_sky_and_sun`、`startup_time_of_day`、`startup_walls`（函数已搬移）。

## 6. 组件/几何是否真的被使用

- **组件**：`DrawableMeshBundle` 被 terrain/water/grass/debug_viz 等 10 处使用；`KeepOnLevelUnload` 6 处；rendering 标记被 draw pass 消费；trees/ui 组件被树与 UI 系统消费。`registry.rs` 不被运行时使用。
- **几何**：
  - `cube_mesh` → `systems/prefab.rs:340` 真实消费。
  - `InstancedWall`/`BrickTransformSsbo` → `systems/wall/construct_wall_bricks.rs:209`、`geometry/runtime.rs`、`init_populate_ssbo_library` 真实消费。
  - `plane_mesh`、`Sphere::to_mesh` → **crate 内无消费者**（仅自身测试）；可能由宿主/原版其它模块使用，需人工确认。
- **Nani 实例类型**：宏定义在 `render/nani/instance_data.rs`，`startup_nani` 与 draw callback 真实使用；components 的 15 个 no_trace 只是实例化名。

## 7. 根契约类型哪些是占位

| 类型/函数 | 状态 |
|---|---|
| `GraphicsDevice` 包装及 10+ 方法 | 真实转发 |
| `TextureDesc` / `TextureViewDesc` | 真实 builder + 字节布局，部分字段 U |
| `Texture` / `RasterPipeline` / `ComputePipeline` / `CompiledShaderPipeline` | 占位/字节容器 |
| `RasterPipelineDesc` / `RasterPipelineKey` | 可比较键；字段语义部分 U |
| `ImportedBuffer` / `BufferGraphBackend` / `RenderGraph` | 编译桥；无 backend 时生产 panic |
| `ManagedHandle` / `Handle::get` / `ReadOnlyHandle::get` | 桥；get 必 panic |
| `ResourceHandle` / `RenderPassDesc` / `RenderPassAttachmentDesc` | 编译桥/占位 |
| `PassResourceSlot` / `RenderPassBinding` / `bind` | 游戏侧声明占位 |
| `PassBuilder` | detached；三个方法 panic |
| `full_barrier_required` | 登记缺口，恒 false + warn |
| rhapsody 真实类型 re-export | 真实（`rhapsody::raw/rg` 转发） |

## 8. 仍未复现清单（≥12，带证据）

判定为"未复现"指：在 `country-core` 及上层 `tiny-glade`/`systems` crates 中按原名或明显等价函数未找到。宿主可能有替代路径（例如纹理加载在 `loading_graph.rs`），逐条仍需人工确认。

1. `startup/clearing.rs::init_clearing` 0x140a78aa0 — 无函数；仅 `asset_libraries/audio_trigger_library.rs:81` 等注释。
2. `startup/clearing.rs::uninit_clearing` 0x140a7a5b0（name_only）— 无函数；`render/nani/gpu_world.rs:602`、`resources/rt.rs:266` 只描述语义。
3. `startup/load_audio.rs::load_audio` 0x140b0e0f0 — 无函数；与 `load_glade_audio` 不同。
4. `startup/load_glade_grading.rs::load_glade_grading` 0x1408bf9c0 — 无函数；只有 `resources/grading_lut.rs` 资源。
5. `startup/load_glade_tod_and_weather.rs::load_snow_particles` 0x140ae7d90 — 无函数；仅 shader 名。
6. `startup/load_textures.rs::load_glade_texture_mappings` 0x1409e6b60 — 无函数；文件只有 `should_load`。
7. `startup/spawn_brush_previews.rs::spawn_brush_previews` 0x140924b40 — 无函数；`systems/brush_preview.rs` 只更新/隐藏。
8. `startup/spawn_human_ref.rs::spawn_human_ref` 0x14089d5a0 — 全 workspace 无。
9. `startup/spawn_preplaced_trees.rs::spawn_preplaced_trees` 0x140862270（name_only）— 无函数；只有 `TreeAnimation::preplaced` 常量。
10. `startup/startup_critters.rs::startup_critters` 0x1409d49a0 — 无函数；critter SSBO 只在 72 项生产者与可见性查询。
11. `startup/startup_meadow.rs::init_meadow` 0x1408639f0 — 无函数；"meadow" 仅 `meadow_flower` shader。
12. `startup/startup_roof.rs::startup_roof` 0x1414547d0 — 无函数；roofs 资源已恢复但无启动生产者。
13. `startup/startup_sheep.rs::startup_sheep` 0x140a093e0（name_only）— 无函数；sheep 控制/音频存在。
14. `startup/startup_sky_and_sun.rs::startup_sky_and_sun` 0x140864430 — 无函数；只有太阳位置计算。
15. `startup/startup_time_of_day.rs::startup_time_of_day` 0x14089d6f0（name_only）— 无函数；`resources/time_of_day.rs:9/26` 注释指该模块但不存在。
16. `startup/glade_themes/{flowery,olden}.rs::startup_flowery/startup_olden` 0x141553120/0x1415539d0 — 无文件/函数；只有 GladeType 与颜色库名字。
17. `startup/debug.rs::debug` 0x1408efd40（name_only）— 无函数/文件。
18. `game_key_bindings.rs` 7 个函数：`action_desc` 0x14089cb60、`mouse_button_to_string` 0x14089c970、`assign_key` 0x14089b2a0、`assign_mouse_button` 0x14089b280、`get_keycode` 0x14089d300、`get_mouse_button` 0x14089c880、`keycode_to_str` 0x14089d490 — 全无实现。
19. `components/drawable.rs:7-8` 的 `cache_prev_object_transforms` 与完整材质组件族 — 自述未恢复；`PrevTransform` 只有 draw callback `*View`（`draw_callbacks_a_views.rs:78`、`draw_callbacks_b.rs:215`）。
20. `startup_water` 未接线 — 399 行真实实现但无宿主调用；`startup_ivy`/`startup_garden_brush`/`add_game_startup_systems`/`configure_glade_audio` 同样零调用。
21. `lib.rs::ui_adjust_wall_height` 0x140b2fc00 — 无同名实现；`systems/wall/adjust_wall_height.rs:39 adjust_wall_height` 只覆盖命令算法，UI 模式函数需人工确认是否另有实现。
22. `startup/mod.rs::load_shader_into_library_replace_previous` 0x140b405e0 — 无独立函数（replace 标志已合并，算部分）。

## 9. 完成度估计

| 口径 | 数值 | 依据 | 置信度 |
|---|---:|---|---|
| 文件存在率 | 67/约 85 = **79%** | 骨架同范围 17 个 startup 文件 + `game_key_bindings.rs` 缺失 | 高 |
| 骨架符号严格可追踪 | 130/238 = **54.6%** | `fn_audit`：in_file 84 + moved 42 + va_only 4 | 高（机械）；存在假阳性，见下 |
| 业务函数存在率（扣除非业务 no_trace） | 约 **89%**（212/238） | 84 个 no_trace 中约 57 为闭包/泛型/derive/搬移；26 个业务函数缺失 | 中 |
| 语义验收 | **未完成** | `recovery/inventory.json` 的 `semantic_acceptance:false`；本轮未跑原侧 oracle | 高（未做） |
| **综合完成度** | **约 78%–82%** | 文件缺口 + 26 个业务函数缺失 + 3 处未接线/2 处部分，扣分后落在文件存在率附近 | 中 |

补充：如果把 `components/registry.rs` 的 57 条证据表或测试文件计入，会虚高；本文均不计业务完成度。启动目录是全范围最弱环节（骨架 60 个符号中 24 no_trace、6 name_only，且 17 个文件缺失）。

`fn_audit` 的 `in_file` 不是逐函数体验证。反例：`components/drawable.rs::cache_prev_object_transforms` 被标 `in_file`（0x14082bd80），但源码只有 `drawable.rs:7` 的"尚未恢复"注释；本文将其计入未复现，故 54.6% 应视为上界，真实可追踪率可能更低。类似假阳性需逐条复核。

## 10. 需人工确认

1. `input_stream::add_frame` 是否等价于重建的 `record_frame`（骨架有直接调用 `0x949660`，重建无同名）。
2. `startup_time_of_day::startup_time_of_day` 是否与 `load_glade_tod_and_weather` 合并，或确为独立默认 TOD 加载器。
3. `spawn_brush_previews`、`spawn_human_ref`、`startup_critters` 是否被有意推迟到上层 `systems/*`/宿主；当前全 workspace 未找到。
4. `plane_mesh`/`Sphere::to_mesh` 的宿主消费者（crate 内无）。
5. `startup_water` 是否由尚未提交的宿主代码接入；当前无调用。
6. `ui_adjust_wall_height` 的 UI 模式函数是否在 `systems/wall` 其它文件以别名存在。
7. `game_profile` 的 ron 序列化形状与 `TutorialCooldowns` 字段命名（注释自标待取证）。
8. `recording.rs` v2 协议与原版完整录制协议的一致性。
