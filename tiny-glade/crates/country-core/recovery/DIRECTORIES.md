# country-core 逐目录源码核对

由 tools/audit_core_foundation.py 生成；B 为唯一目标。原目录以 B PDB 文件索引为准，编译导出取自 rustdoc。
有正式映射不等于完整实现；未映射不等于算法不存在（可能仍在 recovered_building 等投影中）。
完成验收必须另外核对状态所有者、消费者、失败分支、编辑/删除/历史传播和原版算法。

B 索引文件 539；其中有章节 529；编译可达文件 268；旧纠错条目 277。不据此计算业务完成率。
旧库存另有 74 个路径未出现在 B PDB 文件表，单列 historical_only_paths，不混入原目录完成分母。

| 原目录 | 原文件数 | 有正式源码映射 | 本轮逐文件合同 | 状态 |
|---|---:|---:|---:|---|
| `./` | 6 | 1 | 0 | 待逐文件复核 |
| `asset_libraries/` | 12 | 12 | 0 | 待逐文件复核 |
| `components/` | 3 | 1 | 0 | 待逐文件复核 |
| `geometry/` | 4 | 0 | 0 | 待逐文件复核 |
| `input/` | 4 | 0 | 0 | 待逐文件复核 |
| `render/` | 21 | 17 | 0 | 待逐文件复核 |
| `render/nani/` | 8 | 8 | 0 | 待逐文件复核 |
| `render/pass_builder/` | 10 | 10 | 0 | 待逐文件复核 |
| `resources/` | 60 | 18 | 6 | 恢复/核验中 |
| `resources/critters/` | 1 | 0 | 0 | 待逐文件复核 |
| `resources/critters/sheep/` | 1 | 0 | 0 | 待逐文件复核 |
| `resources/history/` | 12 | 1 | 1 | 恢复/核验中 |
| `resources/ivy/` | 4 | 0 | 0 | 待逐文件复核 |
| `resources/render/` | 3 | 3 | 0 | 待逐文件复核 |
| `resources/roofs/` | 11 | 1 | 0 | 待逐文件复核 |
| `resources/rt/` | 3 | 0 | 0 | 待逐文件复核 |
| `resources/stairs/` | 7 | 0 | 0 | 待逐文件复核 |
| `resources/walls/` | 25 | 4 | 3 | 恢复/核验中 |
| `startup/` | 33 | 2 | 0 | 待逐文件复核 |
| `startup/glade_themes/` | 2 | 0 | 0 | 待逐文件复核 |
| `systems/` | 29 | 2 | 0 | 待逐文件复核 |
| `systems/audio/` | 31 | 0 | 0 | 待逐文件复核 |
| `systems/audio/music/` | 3 | 0 | 0 | 待逐文件复核 |
| `systems/backend/` | 12 | 0 | 0 | 待逐文件复核 |
| `systems/backend/input/` | 7 | 0 | 0 | 待逐文件复核 |
| `systems/camera_rig/` | 5 | 0 | 0 | 待逐文件复核 |
| `systems/collision/` | 3 | 0 | 0 | 待逐文件复核 |
| `systems/debug/` | 3 | 0 | 0 | 待逐文件复核 |
| `systems/garden/` | 2 | 0 | 0 | 待逐文件复核 |
| `systems/history/` | 15 | 0 | 0 | 待逐文件复核 |
| `systems/ivy/` | 9 | 0 | 0 | 待逐文件复核 |
| `systems/paths/` | 4 | 0 | 0 | 待逐文件复核 |
| `systems/render_loop/` | 5 | 5 | 1 | 恢复/核验中 |
| `systems/render_loop/compute_passes/` | 41 | 41 | 0 | 待逐文件复核 |
| `systems/render_loop/draw_passes/` | 16 | 16 | 0 | 待逐文件复核 |
| `systems/render_loop/draw_passes/draw_shaders/` | 16 | 16 | 0 | 待逐文件复核 |
| `systems/render_loop/painterly/` | 1 | 0 | 0 | 待逐文件复核 |
| `systems/render_loop/resources/` | 1 | 1 | 0 | 待逐文件复核 |
| `systems/render_loop/session_selection/` | 1 | 1 | 0 | 待逐文件复核 |
| `systems/save/` | 11 | 0 | 0 | 待逐文件复核 |
| `systems/steam/` | 2 | 0 | 0 | 待逐文件复核 |
| `systems/terrain/` | 9 | 0 | 0 | 待逐文件复核 |
| `systems/ui/` | 4 | 0 | 0 | 待逐文件复核 |
| `systems/ui/resources/` | 3 | 0 | 0 | 待逐文件复核 |
| `systems/ui/signifiers/` | 4 | 0 | 0 | 待逐文件复核 |
| `systems/ui_clutter/` | 7 | 0 | 0 | 待逐文件复核 |
| `systems/wall/` | 13 | 1 | 1 | 恢复/核验中 |
| `systems/wall/adjust_wall_height/` | 5 | 0 | 0 | 待逐文件复核 |
| `systems/wall/eraser/` | 4 | 0 | 0 | 待逐文件复核 |
| `systems/wall/shapes/` | 4 | 0 | 0 | 待逐文件复核 |
| `systems/wall/shapes/circle/` | 4 | 0 | 0 | 待逐文件复核 |
| `systems/wall/shapes/circle/signifiers/` | 2 | 0 | 0 | 待逐文件复核 |
| `systems/wall/shapes/rectangle/` | 8 | 0 | 0 | 待逐文件复核 |
| `systems/wall/shapes/rectangle/ui_modes/` | 2 | 0 | 0 | 待逐文件复核 |
| `systems/wall/shapes/utils/` | 1 | 0 | 0 | 待逐文件复核 |
| `systems/wall/snapping/` | 7 | 0 | 0 | 待逐文件复核 |
| `systems/wall/valid_enclosure/` | 1 | 0 | 0 | 待逐文件复核 |
| `systems/wall/wall_state/` | 6 | 1 | 1 | 恢复/核验中 |
| `utils/` | 8 | 7 | 3 | 恢复/核验中 |

## 文件与证据

每条列出正式源码、章节及原调用者数量；具体调用者、符号和 FIX 行见同目录 inventory.json。

### ./

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [app_loading_state.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/app_loading_state.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [game_config.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/game_config.rs.md) | - | 缺正式映射，待查现有投影 | 24 |
| [game_key_bindings.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/game_key_bindings.rs.md) | - | 缺正式映射，待查现有投影 | 14 |
| [game_profile.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/game_profile.rs.md) | - | 缺正式映射，待查现有投影 | 6 |
| `lib.rs`（无章节） | [lib.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/lib.rs:1) | 已有代码，语义待复核 | 6 |
| [meta_commander.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/meta_commander.rs.md) | - | 缺正式映射，待查现有投影 | 12 |

### asset_libraries/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [audio_trigger_library.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/asset_libraries/audio_trigger_library.rs.md) | [audio_trigger_library.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/asset_libraries/audio_trigger_library.rs:1) | 已有代码，语义待复核 | 8 |
| [color_library.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/asset_libraries/color_library.rs.md) | [color_library.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/asset_libraries/color_library.rs:1) | 已有代码，语义待复核 | 6 |
| [configwatch.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/asset_libraries/configwatch.rs.md) | [configwatch.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/asset_libraries/configwatch.rs:1) | 已有代码，语义待复核 | 2 |
| [mesh_library.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/asset_libraries/mesh_library.rs.md) | [mesh_library.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/asset_libraries/mesh_library.rs:1) | 已有代码，语义待复核 | 35 |
| [meshwatch.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/asset_libraries/meshwatch.rs.md) | [meshwatch.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/asset_libraries/meshwatch.rs:1) | 已有代码，语义待复核 | 5 |
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/asset_libraries/mod.rs.md) | [mod.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/asset_libraries/mod.rs:2) | 已有代码，语义待复核 | 18 |
| [placement_data_library.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/asset_libraries/placement_data_library.rs.md) | [placement_data_library.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/asset_libraries/placement_data_library.rs:1) | 已有代码，语义待复核 | 8 |
| [shader_library.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/asset_libraries/shader_library.rs.md) | [shader_library.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/asset_libraries/shader_library.rs:1) | 已有代码，语义待复核 | 60 |
| [ssbo_library.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/asset_libraries/ssbo_library.rs.md) | [ssbo_library.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/asset_libraries/ssbo_library.rs:1) | 已有代码，语义待复核 | 402 |
| [texture_library.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/asset_libraries/texture_library.rs.md) | [texture_library.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/asset_libraries/texture_library.rs:1) | 已有代码，语义待复核 | 37 |
| [texture_watch.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/asset_libraries/texture_watch.rs.md) | [texture_watch.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/asset_libraries/texture_watch.rs:1) | 已有代码，语义待复核 | 2 |
| [vao_library.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/asset_libraries/vao_library.rs.md) | [vao_library.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/asset_libraries/vao_library.rs:1) | 已有代码，语义待复核 | 7 |

### components/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [drawable.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/components/drawable.rs.md) | - | 缺正式映射，待查现有投影 | 9 |
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/components/mod.rs.md) | [components.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/components.rs:1) | 已有代码，语义待复核 | 19 |
| [transient_mesh.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/components/transient_mesh.rs.md) | - | 缺正式映射，待查现有投影 | 4 |

### geometry/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| `cube.rs`（无章节） | - | 缺正式映射，待查现有投影 | 6 |
| [instanced_wall.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/geometry/instanced_wall.rs.md) | - | 缺正式映射，待查现有投影 | 22 |
| `plane.rs`（无章节） | - | 缺正式映射，待查现有投影 | 1 |
| [sphere.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/geometry/sphere.rs.md) | - | 缺正式映射，待查现有投影 | 4 |

### input/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [action_set.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/input/action_set.rs.md) | - | 缺正式映射，待查现有投影 | 57 |
| [input_stream.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/input/input_stream.rs.md) | - | 缺正式映射，待查现有投影 | 18 |
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/input/mod.rs.md) | - | 缺正式映射，待查现有投影 | 6 |
| [panic_repro_state.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/input/panic_repro_state.rs.md) | - | 缺正式映射，待查现有投影 | 0 |

### render/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [adaptive_render_state.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/adaptive_render_state.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [atmosphere.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/atmosphere.rs.md) | [atmosphere.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/atmosphere.rs:1) | 已有代码，语义待复核 | 3 |
| `debug_info_buffers.rs`（无章节） | - | 缺正式映射，待查现有投影 | 0 |
| [directional_light.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/directional_light.rs.md) | [directional_light.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/directional_light.rs:1) | 已有代码，语义待复核 | 4 |
| [dirty_index_range.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/dirty_index_range.rs.md) | [dirty_index_range.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/dirty_index_range.rs:1) | 已有代码，语义待复核 | 21 |
| [frame_constants.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/frame_constants.rs.md) | [frame_constants.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/frame_constants.rs:1) | 已有代码，语义待复核 | 5 |
| [fsr_plugin.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/fsr_plugin.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [indirect.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/indirect.rs.md) | [indirect.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/indirect.rs:1) | 已有代码，语义待复核 | 0 |
| [load_texture.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/load_texture.rs.md) | [load_texture.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/load_texture.rs:1) | 已有代码，语义待复核 | 1 |
| [mesh.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/mesh.rs.md) | [mesh.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/mesh.rs:1) | 已有代码，语义待复核 | 70 |
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/mod.rs.md) | [mod.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/mod.rs:1) | 已有代码，语义待复核 | 72 |
| [nani.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/nani.rs.md) | [mod.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/nani/mod.rs:1) | 已有代码，语义待复核 | 3 |
| [raster_single.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/raster_single.rs.md) | [raster_single.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/raster_single.rs:1) | 已有代码，语义待复核 | 3 |
| [render_config.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/render_config.rs.md) | [render_config.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/render_config.rs:1) | 已有代码，语义待复核 | 20 |
| [shader.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/shader.rs.md) | [shader.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/shader.rs:1) | 已有代码，语义待复核 | 19 |
| [shadowed_buffer.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/shadowed_buffer.rs.md) | [shadowed_buffer.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/shadowed_buffer.rs:1) | 已有代码，语义待复核 | 45 |
| [sparse_instance_buffer.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/sparse_instance_buffer.rs.md) | [sparse_instance_buffer.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/sparse_instance_buffer.rs:1) | 已有代码，语义待复核 | 94 |
| [ssbo.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/ssbo.rs.md) | [ssbo.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/ssbo.rs:1) | 已有代码，语义待复核 | 302 |
| [uniform.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/uniform.rs.md) | - | 缺正式映射，待查现有投影 | 21 |
| [vao.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/vao.rs.md) | [vao.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/vao.rs:1) | 已有代码，语义待复核 | 3 |
| [view_constants.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/view_constants.rs.md) | [view_constants.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/view_constants.rs:1) | 已有代码，语义待复核 | 3 |

### render/nani/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [buffers.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/nani/buffers.rs.md) | [buffers.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/nani/buffers.rs:1) | 已有代码，语义待复核 | 9 |
| `cull_shaders.rs`（无章节） | [cull_shaders.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/nani/cull_shaders.rs:1) | 已有代码，语义待复核 | 0 |
| [draw_ext.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/nani/draw_ext.rs.md) | [draw_ext.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/nani/draw_ext.rs:1) | 已有代码，语义待复核 | 4 |
| [gpu_world.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/nani/gpu_world.rs.md) | [gpu_world.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/nani/gpu_world.rs:1) | 已有代码，语义待复核 | 60 |
| [instance_data.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/nani/instance_data.rs.md) | [instance_data.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/nani/instance_data.rs:1) | 已有代码，语义待复核 | 11 |
| [mesh_atlas.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/nani/mesh_atlas.rs.md) | [mesh_atlas.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/nani/mesh_atlas.rs:1) | 已有代码，语义待复核 | 4 |
| [systems.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/nani/systems.rs.md) | [instance_updates.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/nani/instance_updates.rs:1); [systems.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/nani/systems.rs:1) | 已有代码，语义待复核 | 0 |
| [trimesh.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/nani/trimesh.rs.md) | [trimesh.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/nani/trimesh.rs:1) | 已有代码，语义待复核 | 3 |

### render/pass_builder/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [bind_resources.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/pass_builder/bind_resources.rs.md) | [bind_resources.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/pass_builder/bind_resources.rs:1) | 已有代码，语义待复核 | 24 |
| [compute_pass_builder.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/pass_builder/compute_pass_builder.rs.md) | [compute_pass_builder.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/pass_builder/compute_pass_builder.rs:1) | 已有代码，语义待复核 | 155 |
| [draw_builder.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/pass_builder/draw_builder.rs.md) | [draw_builder.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/pass_builder/draw_builder.rs:1) | 已有代码，语义待复核 | 144 |
| [draw_list.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/pass_builder/draw_list.rs.md) | [draw_list.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/pass_builder/draw_list.rs:1) | 已有代码，语义待复核 | 12 |
| [draw_stream.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/pass_builder/draw_stream.rs.md) | [draw_stream.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/pass_builder/draw_stream.rs:1) | 已有代码，语义待复核 | 46 |
| [framebuffer_binding.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/pass_builder/framebuffer_binding.rs.md) | [framebuffer_binding.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/pass_builder/framebuffer_binding.rs:1) | 已有代码，语义待复核 | 7 |
| [pass_resource_access.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/pass_builder/pass_resource_access.rs.md) | [pass_resource_access.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/pass_builder/pass_resource_access.rs:1) | 已有代码，语义待复核 | 183 |
| [pass_resources.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/pass_builder/pass_resources.rs.md) | [pass_resources.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/pass_builder/pass_resources.rs:1) | 已有代码，语义待复核 | 16 |
| [shader_uri.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/pass_builder/shader_uri.rs.md) | [shader_uri.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/pass_builder/shader_uri.rs:1) | 已有代码，语义待复核 | 54 |
| [uniform_binder.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render/pass_builder/uniform_binder.rs.md) | [uniform_binder.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/render/pass_builder/uniform_binder.rs:1) | 已有代码，语义待复核 | 28 |

### resources/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [active_raster_stroke.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/active_raster_stroke.rs.md) | [active_raster_stroke.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/resources/active_raster_stroke.rs:1) | 已有代码，语义待复核 | 17 |
| [audio_resources.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/audio_resources.rs.md) | - | 缺正式映射，待查现有投影 | 95 |
| [audio_settings.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/audio_settings.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [brush_sizes.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/brush_sizes.rs.md) | [brush_sizes.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/resources/brush_sizes.rs:1) | 已有代码，语义待复核 | 3 |
| [clutter_meshes.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/clutter_meshes.rs.md) | - | 缺正式映射，待查现有投影 | 10 |
| [color_icon.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/color_icon.rs.md) | [color_icon.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/resources/color_icon.rs:1) | 已修复亮度表；图标组件与消费仍缺 | 2 |
| [color_id.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/color_id.rs.md) | [color_id.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/resources/color_id.rs:1) | 已有代码，语义待复核 | 10 |
| [color_id_roof.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/color_id_roof.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [critters.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/critters.rs.md) | - | 缺正式映射，待查现有投影 | 8 |
| [cursor_terrain_raycast.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/cursor_terrain_raycast.rs.md) | [cursor_terrain_raycast.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/resources/cursor_terrain_raycast.rs:1) | 已有代码，语义待复核 | 62 |
| [daily.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/daily.rs.md) | - | 缺正式映射，待查现有投影 | 5 |
| [drawing_input_smoother.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/drawing_input_smoother.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [editing_highlight.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/editing_highlight.rs.md) | [editing_highlight.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/resources/editing_highlight.rs:1) | clear/墙高亮写入/统一 draw 消费已核验 | 1 |
| [feedback_report_resources.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/feedback_report_resources.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [garden_mask.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/garden_mask.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [glade_border.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/glade_border.rs.md) | - | 缺正式映射，待查现有投影 | 17 |
| [glade_color_customization.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/glade_color_customization.rs.md) | - | 缺正式映射，待查现有投影 | 9 |
| [glade_settings.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/glade_settings.rs.md) | - | 缺正式映射，待查现有投影 | 6 |
| [gothic_window_profile.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/gothic_window_profile.rs.md) | [gothic_window_profile.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/resources/gothic_window_profile.rs:1) | 已有代码，语义待复核 | 8 |
| [gpu_lut.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/gpu_lut.rs.md) | - | 缺正式映射，待查现有投影 | 4 |
| [grading_lut.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/grading_lut.rs.md) | [grading_lut.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/resources/grading_lut.rs:1) | 已有代码，语义待复核 | 17 |
| [gui.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/gui.rs.md) | [gui.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/resources/gui.rs:1) | 菜单拥有状态/推进、缩略图算法、调试标签已实现；GUI 其余资源仍缺 | 37 |
| [image_formation.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/image_formation.rs.md) | [image_formation.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/resources/image_formation.rs:1) | 已有代码，语义待复核 | 16 |
| [in_progress_curve.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/in_progress_curve.rs.md) | - | 缺正式映射，待查现有投影 | 4 |
| [loading_screen.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/loading_screen.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [logo_animation.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/logo_animation.rs.md) | - | 缺正式映射，待查现有投影 | 3 |
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/mod.rs.md) | [mod.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/resources/mod.rs:1) | 已有代码，语义待复核 | 1 |
| [mouse_capture.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/mouse_capture.rs.md) | - | 缺正式映射，待查现有投影 | 3 |
| [mouse_terrain_raycast.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/mouse_terrain_raycast.rs.md) | - | 缺正式映射，待查现有投影 | 11 |
| [onboarding.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/onboarding.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [photo_mode.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/photo_mode.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [placement_data.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/placement_data.rs.md) | - | 缺正式映射，待查现有投影 | 17 |
| [prefab.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/prefab.rs.md) | - | 缺正式映射，待查现有投影 | 7 |
| [preplaced_autoclutter.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/preplaced_autoclutter.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [raster_stroke.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/raster_stroke.rs.md) | - | 缺正式映射，待查现有投影 | 22 |
| [read_mouse_depth_value.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/read_mouse_depth_value.rs.md) | - | 缺正式映射，待查现有投影 | 3 |
| [rocky_terrain.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/rocky_terrain.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [session.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/session.rs.md) | [session.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/resources/session.rs:1) | 补齐菜单所需 SessionId；完整会话待恢复 | 86 |
| [shader_refactoring_state.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/shader_refactoring_state.rs.md) | - | 缺正式映射，待查现有投影 | 3 |
| [shape_intersections.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/shape_intersections.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [shutdown.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/shutdown.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| `sparkle.rs`（无章节） | - | 缺正式映射，待查现有投影 | 0 |
| [stairs_walls.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/stairs_walls.rs.md) | - | 缺正式映射，待查现有投影 | 4 |
| [steam.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/steam.rs.md) | - | 缺正式映射，待查现有投影 | 19 |
| [terrain_editing.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/terrain_editing.rs.md) | - | 缺正式映射，待查现有投影 | 11 |
| [terrain_height.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/terrain_height.rs.md) | [terrain_height.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/resources/terrain_height.rs:1) | 已有代码，语义待复核 | 36 |
| [terrain_rock_mask.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/terrain_rock_mask.rs.md) | - | 缺正式映射，待查现有投影 | 9 |
| [terrain_stroke.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/terrain_stroke.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [texture_presets.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/texture_presets.rs.md) | [texture_presets.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/resources/texture_presets.rs:1) | 已有代码，语义待复核 | 7 |
| [time.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/time.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [time_of_day.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/time_of_day.rs.md) | - | 缺正式映射，待查现有投影 | 9 |
| [toolbar.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/toolbar.rs.md) | - | 缺正式映射，待查现有投影 | 23 |
| [tree_storage.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/tree_storage.rs.md) | - | 缺正式映射，待查现有投影 | 11 |
| [ui_settings.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/ui_settings.rs.md) | [ui_settings.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/resources/ui_settings.rs:1) | 已修复并核验：7 字段/default/JSON；持久化消费者待接 | 5 |
| [water_exclude_clutter_mask.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/water_exclude_clutter_mask.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [water_mask.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/water_mask.rs.md) | [water_mask.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/resources/water_mask.rs:1) | 已有代码，语义待复核 | 0 |
| [weather.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/weather.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [window.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/window.rs.md) | [window.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/resources/window.rs:1) | 已修复并核验：尺寸/转换/比例；启动消费者待接 | 32 |
| [window_lights.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/window_lights.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [world_raster.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/world_raster.rs.md) | [world_raster.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/resources/world_raster.rs:1) | 已有代码，语义待复核 | 74 |

### resources/critters/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [sheep.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/critters/sheep.rs.md) | - | 缺正式映射，待查现有投影 | 3 |

### resources/critters/sheep/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [control.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/critters/sheep/control.rs.md) | - | 缺正式映射，待查现有投影 | 7 |

### resources/history/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [auto_clutter_edit.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/history/auto_clutter_edit.rs.md) | [clutter.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/recovered_systems/clutter.rs:1) | 已存在 Delete serde 合同投影；完整历史链未接 | 0 |
| [color_edit.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/history/color_edit.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [decorator_edit.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/history/decorator_edit.rs.md) | - | 缺正式映射，待查现有投影 | 11 |
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/history/mod.rs.md) | - | 缺正式映射，待查现有投影 | 107 |
| [path_edit.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/history/path_edit.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [replay_cmd_buffer.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/history/replay_cmd_buffer.rs.md) | - | 缺正式映射，待查现有投影 | 23 |
| [roof_edit.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/history/roof_edit.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [stairs_history_edit.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/history/stairs_history_edit.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [terrain_edit.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/history/terrain_edit.rs.md) | - | 缺正式映射，待查现有投影 | 6 |
| [tree_edit.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/history/tree_edit.rs.md) | - | 缺正式映射，待查现有投影 | 5 |
| [wall_compound_edit.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/history/wall_compound_edit.rs.md) | - | 缺正式映射，待查现有投影 | 27 |
| [wall_history.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/history/wall_history.rs.md) | - | 缺正式映射，待查现有投影 | 6 |

### resources/ivy/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [ivy_direction_proposer.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/ivy/ivy_direction_proposer.rs.md) | - | 缺正式映射，待查现有投影 | 3 |
| [ivy_leaf_storage.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/ivy/ivy_leaf_storage.rs.md) | - | 缺正式映射，待查现有投影 | 6 |
| [ivy_storage.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/ivy/ivy_storage.rs.md) | - | 缺正式映射，待查现有投影 | 16 |
| [wall_coord.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/ivy/wall_coord.rs.md) | - | 缺正式映射，待查现有投影 | 4 |

### resources/render/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [mesh_atlas_library.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/render/mesh_atlas_library.rs.md) | [mesh_atlas_library.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/resources/render/mesh_atlas_library.rs:1) | 已有代码，语义待复核 | 12 |
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/render/mod.rs.md) | [mod.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/resources/render/mod.rs:1) | 已有代码，语义待复核 | 35 |
| [spatial_resolve_offsets.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/render/spatial_resolve_offsets.rs.md) | [spatial_resolve_offsets.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/resources/render/spatial_resolve_offsets.rs:1) | 已有代码，语义待复核 | 1 |

### resources/roofs/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [commands.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/roofs/commands.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [dormer_tiles.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/roofs/dormer_tiles.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [roof_animation.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/roofs/roof_animation.rs.md) | - | 缺正式映射，待查现有投影 | 4 |
| `roof_buffers.rs`（无章节） | - | 缺正式映射，待查现有投影 | 0 |
| [roof_noise.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/roofs/roof_noise.rs.md) | - | 缺正式映射，待查现有投影 | 6 |
| [roof_ridge.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/roofs/roof_ridge.rs.md) | - | 缺正式映射，待查现有投影 | 16 |
| [roof_shape.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/roofs/roof_shape.rs.md) | [roof_shape.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/resources/roofs/roof_shape.rs:1) | 已有代码，语义待复核 | 108 |
| [roof_tile_sim.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/roofs/roof_tile_sim.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [roof_tiles.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/roofs/roof_tiles.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [roof_trigger_volume.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/roofs/roof_trigger_volume.rs.md) | - | 缺正式映射，待查现有投影 | 10 |
| [roof_visual_shape.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/roofs/roof_visual_shape.rs.md) | - | 缺正式映射，待查现有投影 | 16 |

### resources/rt/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [bvh.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/rt/bvh.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [bvh_builder.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/rt/bvh_builder.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/rt/mod.rs.md) | - | 缺正式映射，待查现有投影 | 23 |

### resources/stairs/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [invalidate.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/stairs/invalidate.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [stairs_assembly_params.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/stairs/stairs_assembly_params.rs.md) | - | 缺正式映射，待查现有投影 | 28 |
| [stairs_collider.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/stairs/stairs_collider.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [stairs_removed_supports.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/stairs/stairs_removed_supports.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [stairs_state.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/stairs/stairs_state.rs.md) | - | 缺正式映射，待查现有投影 | 72 |
| [structural_graph.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/stairs/structural_graph.rs.md) | - | 缺正式映射，待查现有投影 | 4 |
| [visual.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/stairs/visual.rs.md) | - | 缺正式映射，待查现有投影 | 7 |

### resources/walls/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [arches.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/walls/arches.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [cached_decorator_transforms.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/walls/cached_decorator_transforms.rs.md) | - | 缺正式映射，待查现有投影 | 61 |
| [curved_halftimber.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/walls/curved_halftimber.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [decorator.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/walls/decorator.rs.md) | - | 缺正式映射，待查现有投影 | 137 |
| [decorator_archivist.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/walls/decorator_archivist.rs.md) | - | 缺正式映射，待查现有投影 | 58 |
| [decorator_collision.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/walls/decorator_collision.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [decorator_outline.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/walls/decorator_outline.rs.md) | - | 缺正式映射，待查现有投影 | 6 |
| [decorator_storage.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/walls/decorator_storage.rs.md) | - | 缺正式映射，待查现有投影 | 159 |
| [decorator_subtype.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/walls/decorator_subtype.rs.md) | - | 缺正式映射，待查现有投影 | 75 |
| [dormers.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/walls/dormers.rs.md) | - | 缺正式映射，待查现有投影 | 9 |
| [elevation_support_stats.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/walls/elevation_support_stats.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [in_progress_wall.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/walls/in_progress_wall.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [inner_wall_state.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/walls/inner_wall_state.rs.md) | [inner_wall_state.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/resources/walls/inner_wall_state.rs:1) | 已接入墙权威链；serde/完整消费者未闭合 | 35 |
| [inner_walls.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/walls/inner_walls.rs.md) | [inner_walls.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/resources/walls/inner_walls.rs:1) | 已接入墙权威链；完整消费者未闭合 | 67 |
| [material_stats.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/walls/material_stats.rs.md) | - | 缺正式映射，待查现有投影 | 10 |
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/walls/mod.rs.md) | [mod.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/resources/walls/mod.rs:1) | 已有代码，语义待复核 | 44 |
| [plaster.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/walls/plaster.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [public_wall_state.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/walls/public_wall_state.rs.md) | - | 缺正式映射，待查现有投影 | 145 |
| [public_walls.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/walls/public_walls.rs.md) | [public_walls.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/resources/walls/public_walls.rs:1) | WallEntities 查询已接；完整 PublicWalls 仍缺 | 171 |
| [wall_attachment.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/walls/wall_attachment.rs.md) | - | 缺正式映射，待查现有投影 | 12 |
| [wall_holes.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/walls/wall_holes.rs.md) | - | 缺正式映射，待查现有投影 | 39 |
| [wall_path_segment.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/walls/wall_path_segment.rs.md) | - | 缺正式映射，待查现有投影 | 4 |
| [wall_stone_columns.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/walls/wall_stone_columns.rs.md) | - | 缺正式映射，待查现有投影 | 5 |
| [wall_trigger_volume.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/walls/wall_trigger_volume.rs.md) | - | 缺正式映射，待查现有投影 | 13 |
| [wooden_gates.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/resources/walls/wooden_gates.rs.md) | - | 缺正式映射，待查现有投影 | 0 |

### startup/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [clearing.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/clearing.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [debug.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/debug.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [init_color_library.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/init_color_library.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [init_photomode_state.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/init_photomode_state.rs.md) | - | 缺正式映射，待查现有投影 | 4 |
| [load_audio.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/load_audio.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [load_glade_audio.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/load_glade_audio.rs.md) | - | 缺正式映射，待查现有投影 | 3 |
| [load_glade_grading.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/load_glade_grading.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [load_glade_tod_and_weather.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/load_glade_tod_and_weather.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [load_mesh_atlases.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/load_mesh_atlases.rs.md) | - | 缺正式映射，待查现有投影 | 4 |
| [load_prefabs.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/load_prefabs.rs.md) | - | 缺正式映射，待查现有投影 | 11 |
| [load_shaders.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/load_shaders.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [load_textures.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/load_textures.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/mod.rs.md) | [mod.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/startup/mod.rs:1) | 已有代码，语义待复核 | 37 |
| [panic_reporter_startup.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/panic_reporter_startup.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [spawn_brush_previews.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/spawn_brush_previews.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [spawn_human_ref.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/spawn_human_ref.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [spawn_preplaced_trees.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/spawn_preplaced_trees.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [startup_blit.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/startup_blit.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [startup_critters.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/startup_critters.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [startup_deferred_raster.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/startup_deferred_raster.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [startup_garden_brush.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/startup_garden_brush.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [startup_grass.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/startup_grass.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [startup_ivy.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/startup_ivy.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [startup_meadow.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/startup_meadow.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [startup_nani.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/startup_nani.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [startup_roof.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/startup_roof.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [startup_session.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/startup_session.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [startup_sheep.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/startup_sheep.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [startup_sky_and_sun.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/startup_sky_and_sun.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [startup_terrain.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/startup_terrain.rs.md) | [startup_terrain.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/startup/startup_terrain.rs:1) | 已有代码，语义待复核 | 5 |
| [startup_time_of_day.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/startup_time_of_day.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [startup_walls.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/startup_walls.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [startup_water.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/startup_water.rs.md) | - | 缺正式映射，待查现有投影 | 0 |

### startup/glade_themes/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [flowery.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/glade_themes/flowery.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [olden.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/startup/glade_themes/olden.rs.md) | - | 缺正式映射，待查现有投影 | 1 |

### systems/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [arrow_gizmo_animation.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/arrow_gizmo_animation.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [brush_preview.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/brush_preview.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [camera_rig.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/camera_rig.rs.md) | [mod.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/camera_rig/mod.rs:1) | 已有代码，语义待复核 | 59 |
| [clear_canvas.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/clear_canvas.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [collision.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/collision.rs.md) | - | 缺正式映射，待查现有投影 | 3 |
| [despawn.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/despawn.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [garden.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/garden.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [look_at_camera.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/look_at_camera.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [main_camera.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/main_camera.rs.md) | [main_camera.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/camera_rig/main_camera.rs:1) | 已有代码，语义待复核 | 6 |
| [mesh_follow_terrain.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/mesh_follow_terrain.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/mod.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [mod_key.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/mod_key.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [mode_manager.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/mode_manager.rs.md) | - | 缺正式映射，待查现有投影 | 26 |
| [prefab.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/prefab.rs.md) | - | 缺正式映射，待查现有投影 | 5 |
| [time_of_day.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/time_of_day.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [time_update.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/time_update.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [timelapse_replay.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/timelapse_replay.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [ui_2d_follow_mouse.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ui_2d_follow_mouse.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [ui_camera_mode_gizmos.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ui_camera_mode_gizmos.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [ui_edit_focus.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ui_edit_focus.rs.md) | - | 缺正式映射，待查现有投影 | 7 |
| [ui_edit_freehand_walls.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ui_edit_freehand_walls.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [ui_focus_mode_shape_height.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ui_focus_mode_shape_height.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [ui_move_freehand_walls.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ui_move_freehand_walls.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [ui_move_shapes.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ui_move_shapes.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [ui_raise_shape.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ui_raise_shape.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [utils.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/utils.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [utils_intersect.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/utils_intersect.rs.md) | - | 缺正式映射，待查现有投影 | 3 |
| [weather_system.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/weather_system.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [world_raster.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/world_raster.rs.md) | - | 缺正式映射，待查现有投影 | 5 |

### systems/audio/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [audio_update.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/audio_update.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [building_audio.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/building_audio.rs.md) | - | 缺正式映射，待查现有投影 | 5 |
| [clutter_audio.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/clutter_audio.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [decorator_audio.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/decorator_audio.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [elevation_supports_audio.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/elevation_supports_audio.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [firstperson_steps.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/firstperson_steps.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [forest_audio.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/forest_audio.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [game_loading_audio.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/game_loading_audio.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [garden_audio.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/garden_audio.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [ivy_audio.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/ivy_audio.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [menu_audio.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/menu_audio.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/mod.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [path_audio.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/path_audio.rs.md) | - | 缺正式映射，待查现有投影 | 7 |
| [playermade_stairs_audio.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/playermade_stairs_audio.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [replay_audio.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/replay_audio.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [roof_audio.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/roof_audio.rs.md) | - | 缺正式映射，待查现有投影 | 7 |
| [sheep_audio.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/sheep_audio.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [signifier_audio.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/signifier_audio.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [terrain_audio.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/terrain_audio.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [terrain_stairs_audio.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/terrain_stairs_audio.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [timberframe_audio.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/timberframe_audio.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [tree_audio.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/tree_audio.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [trigger_audio.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/trigger_audio.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [wall_audio_fmod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/wall_audio_fmod.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [wall_audio_orchestrator.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/wall_audio_orchestrator.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [wall_audio_path.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/wall_audio_path.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [wall_audio_ui.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/wall_audio_ui.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [wall_audio_utils.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/wall_audio_utils.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [water_ambience_audio.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/water_ambience_audio.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [water_audio.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/water_audio.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [weather_audio.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/weather_audio.rs.md) | - | 缺正式映射，待查现有投影 | 0 |

### systems/audio/music/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [music_debug_gui.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/music/music_debug_gui.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [music_player.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/music/music_player.rs.md) | - | 缺正式映射，待查现有投影 | 7 |
| [music_system.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/audio/music/music_system.rs.md) | - | 缺正式映射，待查现有投影 | 1 |

### systems/backend/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [configwatch.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/backend/configwatch.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [flush_screen_log.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/backend/flush_screen_log.rs.md) | - | 缺正式映射，待查现有投影 | 5 |
| [game_frame_index.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/backend/game_frame_index.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [grow_grass.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/backend/grow_grass.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [icc.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/backend/icc.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [meshwatch.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/backend/meshwatch.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/backend/mod.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [panic_reporter_fetch_active_session.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/backend/panic_reporter_fetch_active_session.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [prepare_frame_constants.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/backend/prepare_frame_constants.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [texture_watch.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/backend/texture_watch.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [transient_mesh.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/backend/transient_mesh.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [vao_update.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/backend/vao_update.rs.md) | - | 缺正式映射，待查现有投影 | 0 |

### systems/backend/input/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [calculate_input_delta.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/backend/input/calculate_input_delta.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [input_record_replay.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/backend/input/input_record_replay.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [input_replay_for_trailers.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/backend/input/input_replay_for_trailers.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [reset_input_state.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/backend/input/reset_input_state.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [translate_input.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/backend/input/translate_input.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [translate_input_to_cursor_moved.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/backend/input/translate_input_to_cursor_moved.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [update_cursor_position.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/backend/input/update_cursor_position.rs.md) | - | 缺正式映射，待查现有投影 | 0 |

### systems/camera_rig/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [butterfly.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/camera_rig/butterfly.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [camera_body.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/camera_rig/camera_body.rs.md) | - | 缺正式映射，待查现有投影 | 4 |
| [free.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/camera_rig/free.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [normal.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/camera_rig/normal.rs.md) | - | 缺正式映射，待查现有投影 | 4 |
| [walk.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/camera_rig/walk.rs.md) | - | 缺正式映射，待查现有投影 | 3 |

### systems/collision/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [intersect_meshes.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/collision/intersect_meshes.rs.md) | - | 缺正式映射，待查现有投影 | 5 |
| [visualize_colliders.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/collision/visualize_colliders.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [world.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/collision/world.rs.md) | - | 缺正式映射，待查现有投影 | 173 |

### systems/debug/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [human_scale_ref.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/debug/human_scale_ref.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [lazy_grid_debug.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/debug/lazy_grid_debug.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [sheep_controls.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/debug/sheep_controls.rs.md) | - | 缺正式映射，待查现有投影 | 3 |

### systems/garden/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [draw_garden.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/garden/draw_garden.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [record_garden_history.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/garden/record_garden_history.rs.md) | - | 缺正式映射，待查现有投影 | 0 |

### systems/history/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [finish_replay.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/history/finish_replay.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [history_process.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/history/history_process.rs.md) | - | 缺正式映射，待查现有投影 | 9 |
| [history_recording.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/history/history_recording.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/history/mod.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [replay_auto_clutter.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/history/replay_auto_clutter.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [replay_garden.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/history/replay_garden.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [replay_mode.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/history/replay_mode.rs.md) | - | 缺正式映射，待查现有投影 | 3 |
| [replay_path.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/history/replay_path.rs.md) | - | 缺正式映射，待查现有投影 | 5 |
| [replay_pop_bucket.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/history/replay_pop_bucket.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [replay_raster.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/history/replay_raster.rs.md) | - | 缺正式映射，待查现有投影 | 15 |
| [replay_stairs.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/history/replay_stairs.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [replay_terrain.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/history/replay_terrain.rs.md) | - | 缺正式映射，待查现有投影 | 7 |
| [replay_wall.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/history/replay_wall.rs.md) | - | 缺正式映射，待查现有投影 | 9 |
| [replay_water.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/history/replay_water.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [signifier_replay_active.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/history/signifier_replay_active.rs.md) | - | 缺正式映射，待查现有投影 | 1 |

### systems/ivy/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [ivy_branch_thickness_animation.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ivy/ivy_branch_thickness_animation.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [ivy_grower.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ivy/ivy_grower.rs.md) | - | 缺正式映射，待查现有投影 | 6 |
| [ivy_leaf_animation.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ivy/ivy_leaf_animation.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [ivy_leaf_spawner.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ivy/ivy_leaf_spawner.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [ivy_pruner.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ivy/ivy_pruner.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [ivy_send_to_gpu_leaf.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ivy/ivy_send_to_gpu_leaf.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [ivy_send_to_gpu_segments.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ivy/ivy_send_to_gpu_segments.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [ivy_spawner.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ivy/ivy_spawner.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ivy/mod.rs.md) | - | 缺正式映射，待查现有投影 | 1 |

### systems/paths/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [draw_path.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/paths/draw_path.rs.md) | - | 缺正式映射，待查现有投影 | 24 |
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/paths/mod.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [path_brush_preview_size.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/paths/path_brush_preview_size.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [record_path_history.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/paths/record_path_history.rs.md) | - | 缺正式映射，待查现有投影 | 0 |

### systems/render_loop/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| `indirect_params.rs`（无章节） | [indirect_params.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/indirect_params.rs:1) | 已有代码，语义待复核 | 0 |
| [lut_gallery.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/lut_gallery.rs.md) | [lut_gallery.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/lut_gallery.rs:1) | 菜单与缩略图消费者已修正；GPU/名字格式仍待恢复 | 0 |
| [menu_background.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/menu_background.rs.md) | [menu_background.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/menu_background.rs:1) | 已有代码，语义待复核 | 1 |
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/mod.rs.md) | [mod.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/mod.rs:1) | 已有代码，语义待复核 | 7 |
| [vermeer.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/vermeer.rs.md) | [vermeer.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/vermeer.rs:1) | 已有代码，语义待复核 | 11 |

### systems/render_loop/compute_passes/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [apply_screen_shadows.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/apply_screen_shadows.rs.md) | [apply_screen_shadows.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/apply_screen_shadows.rs:1) | 已有代码，语义待复核 | 1 |
| [apply_volumetrics.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/apply_volumetrics.rs.md) | [apply_volumetrics.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/apply_volumetrics.rs:1) | 已有代码，语义待复核 | 1 |
| [bin_lights.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/bin_lights.rs.md) | [bin_lights.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/bin_lights.rs:1) | 已有代码，语义待复核 | 1 |
| [blur_pyramid.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/blur_pyramid.rs.md) | [blur_pyramid.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/blur_pyramid.rs:1) | 已有代码，语义待复核 | 3 |
| [compute_garden.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/compute_garden.rs.md) | [compute_garden.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/compute_garden.rs:1) | 已有代码，语义待复核 | 1 |
| [compute_path_masks.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/compute_path_masks.rs.md) | [compute_path_masks.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/compute_path_masks.rs:1) | 已有代码，语义待复核 | 2 |
| [compute_water.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/compute_water.rs.md) | [compute_water.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/compute_water.rs:1) | 已有代码，语义待复核 | 1 |
| [displace_rocky_terrain.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/displace_rocky_terrain.rs.md) | [displace_rocky_terrain.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/displace_rocky_terrain.rs:1) | 已有代码，语义待复核 | 1 |
| [dof.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/dof.rs.md) | [dof.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/dof.rs:1) | 已有代码，语义待复核 | 2 |
| [dof_autofocus.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/dof_autofocus.rs.md) | [dof_autofocus.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/dof_autofocus.rs:1) | 已有代码，语义待复核 | 1 |
| [eval_gi_screen_radiance.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/eval_gi_screen_radiance.rs.md) | [eval_gi_screen_radiance.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/eval_gi_screen_radiance.rs:1) | 已有代码，语义待复核 | 1 |
| [generate_hzb.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/generate_hzb.rs.md) | [generate_hzb.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/generate_hzb.rs:1) | 已有代码，语义待复核 | 1 |
| [generate_mips.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/generate_mips.rs.md) | [generate_mips.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/generate_mips.rs:1) | 已有代码，语义待复核 | 2 |
| [gpu_generate_draw_lists.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/gpu_generate_draw_lists.rs.md) | [gpu_generate_draw_lists.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/gpu_generate_draw_lists.rs:1) | 已有代码，语义待复核 | 7 |
| [grass.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/grass.rs.md) | [grass.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/grass.rs:1) | 已有代码，语义待复核 | 2 |
| [half_res.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/half_res.rs.md) | [half_res.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/half_res.rs:1) | 已有代码，语义待复核 | 1 |
| [highlight_active_terrain_stroke.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/highlight_active_terrain_stroke.rs.md) | [highlight_active_terrain_stroke.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/highlight_active_terrain_stroke.rs:1) | 已有代码，语义待复核 | 1 |
| [leaf_particles.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/leaf_particles.rs.md) | [leaf_particles.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/leaf_particles.rs:1) | 已有代码，语义待复核 | 1 |
| [mouse_depth_value.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/mouse_depth_value.rs.md) | [mouse_depth_value.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/mouse_depth_value.rs:1) | 已有代码，语义待复核 | 1 |
| [mouse_read_terrain_stroke_index.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/mouse_read_terrain_stroke_index.rs.md) | [mouse_read_terrain_stroke_index.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/mouse_read_terrain_stroke_index.rs:1) | 已有代码，语义待复核 | 1 |
| [path_pebbles.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/path_pebbles.rs.md) | [path_pebbles.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/path_pebbles.rs:1) | 已有代码，语义待复核 | 1 |
| [prefix_scan.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/prefix_scan.rs.md) | [prefix_scan.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/prefix_scan.rs:1) | 已有代码，语义待复核 | 1 |
| [primary_rt_debug.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/primary_rt_debug.rs.md) | [primary_rt_debug.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/primary_rt_debug.rs:1) | 已有代码，语义待复核 | 0 |
| [rasterize_buildings.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/rasterize_buildings.rs.md) | [rasterize_buildings.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/rasterize_buildings.rs:1) | 已有代码，语义待复核 | 1 |
| [rasterize_terrain_stroke.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/rasterize_terrain_stroke.rs.md) | [rasterize_terrain_stroke.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/rasterize_terrain_stroke.rs:1) | 已有代码，语义待复核 | 3 |
| [ray_trace_gi.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/ray_trace_gi.rs.md) | [ray_trace_gi.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/ray_trace_gi.rs:1) | 已有代码，语义待复核 | 1 |
| [read_bush_positions_for_cpu.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/read_bush_positions_for_cpu.rs.md) | [read_bush_positions_for_cpu.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/read_bush_positions_for_cpu.rs:1) | 已有代码，语义待复核 | 1 |
| [read_terrain_data.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/read_terrain_data.rs.md) | [read_terrain_data.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/read_terrain_data.rs:1) | 已有代码，语义待复核 | 2 |
| [reprojection.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/reprojection.rs.md) | [reprojection.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/reprojection.rs:1) | 已有代码，语义待复核 | 1 |
| [reset_indirect_args.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/reset_indirect_args.rs.md) | [reset_indirect_args.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/reset_indirect_args.rs:1) | 已有代码，语义待复核 | 0 |
| [restir_di.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/restir_di.rs.md) | [restir_di.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/restir_di.rs:1) | 已有代码，语义待复核 | 1 |
| [roof_tiles_gravity.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/roof_tiles_gravity.rs.md) | [roof_tiles_gravity.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/roof_tiles_gravity.rs:1) | 已有代码，语义待复核 | 1 |
| [shader_refactoring.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/shader_refactoring.rs.md) | [shader_refactoring.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/shader_refactoring.rs:1) | 已有代码，语义待复核 | 1 |
| `spawn_boats.rs`（无章节） | [spawn_boats.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/spawn_boats.rs:1) | 已有代码，语义待复核 | 0 |
| [spawn_vegetation_around_buildings.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/spawn_vegetation_around_buildings.rs.md) | [spawn_vegetation_around_buildings.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/spawn_vegetation_around_buildings.rs:1) | 已有代码，语义待复核 | 1 |
| [ssgi.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/ssgi.rs.md) | [ssgi.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/ssgi.rs:1) | 已有代码，语义待复核 | 1 |
| [stairs.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/stairs.rs.md) | [stairs.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/stairs.rs:1) | 已有代码，语义待复核 | 1 |
| [taa.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/taa.rs.md) | [taa.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/taa.rs:1) | 已有代码，语义待复核 | 1 |
| [terrain_lighting_texture.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/terrain_lighting_texture.rs.md) | [terrain_lighting_texture.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/terrain_lighting_texture.rs:1) | 已有代码，语义待复核 | 1 |
| [upload_world_raster.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/upload_world_raster.rs.md) | [upload_world_raster.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs:1) | 已有代码，语义待复核 | 1 |
| [visibility_queries.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/compute_passes/visibility_queries.rs.md) | [visibility_queries.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/visibility_queries.rs:1) | 已有代码，语义待复核 | 1 |

### systems/render_loop/draw_passes/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [common_draw_params.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/common_draw_params.rs.md) | [common_draw_params.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs:1) | 已有代码，语义待复核 | 24 |
| [custom_cursor_pass.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/custom_cursor_pass.rs.md) | [custom_cursor_pass.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/custom_cursor_pass.rs:1) | 已有代码，语义待复核 | 0 |
| [deferred_raster.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/deferred_raster.rs.md) | [deferred_raster.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/deferred_raster.rs:1) | 已有代码，语义待复核 | 2 |
| [draw_lists.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/draw_lists.rs.md) | [draw_lists.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/draw_lists.rs:1) | 已有代码，语义待复核 | 5 |
| [feedback_report_bg.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/feedback_report_bg.rs.md) | [feedback_report_bg.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/feedback_report_bg.rs:1) | 已有代码，语义待复核 | 0 |
| [final_post.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/final_post.rs.md) | [final_post.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/final_post.rs:1) | 已有代码，语义待复核 | 2 |
| [foliage_exclusion_mask.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/foliage_exclusion_mask.rs.md) | [foliage_exclusion_mask.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/foliage_exclusion_mask.rs:1) | 已有代码，语义待复核 | 1 |
| [logo_overlay.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/logo_overlay.rs.md) | [logo_overlay.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/logo_overlay.rs:1) | 已有代码，语义待复核 | 0 |
| [main_pass.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/main_pass.rs.md) | [main_pass.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/main_pass.rs:1) | 已有代码，语义待复核 | 4 |
| [material_id.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/material_id.rs.md) | [material_id.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/material_id.rs:1) | 已有代码，语义待复核 | 0 |
| [object_grass_push.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/object_grass_push.rs.md) | [object_grass_push.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/object_grass_push.rs:1) | 已有代码，语义待复核 | 1 |
| [shadow_pass.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/shadow_pass.rs.md) | [shadow_pass.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/shadow_pass.rs:1) | 已有代码，语义待复核 | 1 |
| [transparency_pass.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/transparency_pass.rs.md) | [transparency_pass.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/transparency_pass.rs:1) | 已有代码，语义待复核 | 1 |
| [ui_2d_pass.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/ui_2d_pass.rs.md) | [ui_2d_pass.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs:1) | 已有代码，语义待复核 | 1 |
| [ui_pass.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/ui_pass.rs.md) | [ui_pass.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/ui_pass.rs:1) | 已有代码，语义待复核 | 0 |
| [wireframe_pass.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/wireframe_pass.rs.md) | [wireframe_pass.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/wireframe_pass.rs:1) | 已有代码，语义待复核 | 0 |

### systems/render_loop/draw_passes/draw_shaders/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [billboard_forest.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/draw_shaders/billboard_forest.rs.md) | [billboard_forest.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/billboard_forest.rs:1) | 已有代码，语义待复核 | 0 |
| [clearing_shaders.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/draw_shaders/clearing_shaders.rs.md) | [clearing_shaders.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/clearing_shaders.rs:1) | 已有代码，语义待复核 | 0 |
| [clouds.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/draw_shaders/clouds.rs.md) | [clouds.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/clouds.rs:1) | 已有代码，语义待复核 | 0 |
| [common_includes.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/draw_shaders/common_includes.rs.md) | [common_includes.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs:1) | 已有代码，语义待复核 | 51 |
| [draw_bricks.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/draw_shaders/draw_bricks.rs.md) | [draw_bricks.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/draw_bricks.rs:1) | 已有代码，语义待复核 | 0 |
| [floor_shader.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/draw_shaders/floor_shader.rs.md) | [floor_shader.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/floor_shader.rs:1) | 已有代码，语义待复核 | 2 |
| [grass_shader.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/draw_shaders/grass_shader.rs.md) | [grass_shader.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/grass_shader.rs:1) | 已有代码，语义待复核 | 2 |
| [instanced_critters.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/draw_shaders/instanced_critters.rs.md) | [instanced_critters.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_critters.rs:1) | 已有代码，语义待复核 | 0 |
| [instanced_ivy.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/draw_shaders/instanced_ivy.rs.md) | [instanced_ivy.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_ivy.rs:1) | 已有代码，语义待复核 | 0 |
| [instanced_roof.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/draw_shaders/instanced_roof.rs.md) | [instanced_roof.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_roof.rs:1) | 已有代码，语义待复核 | 0 |
| [leaf_particle_draw.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/draw_shaders/leaf_particle_draw.rs.md) | [leaf_particle_draw.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/leaf_particle_draw.rs:1) | 已有代码，语义待复核 | 0 |
| [path_pebble_shader.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/draw_shaders/path_pebble_shader.rs.md) | [path_pebble_shader.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/path_pebble_shader.rs:1) | 已有代码，语义待复核 | 0 |
| [rocky_terrain_draw.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/draw_shaders/rocky_terrain_draw.rs.md) | [rocky_terrain_draw.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/rocky_terrain_draw.rs:1) | 已有代码，语义待复核 | 2 |
| [stairs_draw.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/draw_shaders/stairs_draw.rs.md) | [stairs_draw.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/stairs_draw.rs:1) | 已有代码，语义待复核 | 0 |
| [stars.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/draw_shaders/stars.rs.md) | [stars.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/stars.rs:1) | 已有代码，语义待复核 | 0 |
| [water_planes_draw.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/draw_passes/draw_shaders/water_planes_draw.rs.md) | [water_planes_draw.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/water_planes_draw.rs:1) | 已有代码，语义待复核 | 0 |

### systems/render_loop/painterly/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/painterly/mod.rs.md) | - | 缺正式映射，待查现有投影 | 1 |

### systems/render_loop/resources/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| `critters.rs`（无章节） | [critters.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/resources/critters.rs:1) | 已有代码，语义待复核 | 0 |

### systems/render_loop/session_selection/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/render_loop/session_selection/mod.rs.md) | [mod.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/render_loop/session_selection/mod.rs:1) | 已有代码，语义待复核 | 4 |

### systems/save/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [load_completion_checker.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/save/load_completion_checker.rs.md) | - | 缺正式映射，待查现有投影 | 5 |
| [load_game.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/save/load_game.rs.md) | - | 缺正式映射，待查现有投影 | 5 |
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/save/mod.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [save_backup_list.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/save/save_backup_list.rs.md) | - | 缺正式映射，待查现有投影 | 7 |
| [save_backup_metronome.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/save/save_backup_metronome.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [save_file_identity.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/save/save_file_identity.rs.md) | - | 缺正式映射，待查现有投影 | 3 |
| [save_game.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/save/save_game.rs.md) | - | 缺正式映射，待查现有投影 | 10 |
| [save_thread.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/save/save_thread.rs.md) | - | 缺正式映射，待查现有投影 | 7 |
| [shadow_file.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/save/shadow_file.rs.md) | - | 缺正式映射，待查现有投影 | 14 |
| [snapshot.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/save/snapshot.rs.md) | - | 缺正式映射，待查现有投影 | 5 |
| [snapshot_gc.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/save/snapshot_gc.rs.md) | - | 缺正式映射，待查现有投影 | 1 |

### systems/steam/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/steam/mod.rs.md) | - | 缺正式映射，待查现有投影 | 10 |
| [startup_steam.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/steam/startup_steam.rs.md) | - | 缺正式映射，待查现有投影 | 4 |

### systems/terrain/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [deferred_terrain_history.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/terrain/deferred_terrain_history.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [delayed_terrain_changed_msg.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/terrain/delayed_terrain_changed_msg.rs.md) | - | 缺正式映射，待查现有投影 | 8 |
| [draw_terrain_stroke.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/terrain/draw_terrain_stroke.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [edit_terrain_stroke.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/terrain/edit_terrain_stroke.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/terrain/mod.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [terrain_hover_idx.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/terrain/terrain_hover_idx.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [ui_draw_terrain.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/terrain/ui_draw_terrain.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [ui_edit_terrain.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/terrain/ui_edit_terrain.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [ui_terrain_focus_mode.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/terrain/ui_terrain_focus_mode.rs.md) | - | 缺正式映射，待查现有投影 | 1 |

### systems/ui/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [debug_visualize_volumes.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ui/debug_visualize_volumes.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [hover_intersection.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ui/hover_intersection.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ui/mod.rs.md) | - | 缺正式映射，待查现有投影 | 4 |
| [ui_update.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ui/ui_update.rs.md) | - | 缺正式映射，待查现有投影 | 1 |

### systems/ui/resources/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [slider3d.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ui/resources/slider3d.rs.md) | - | 缺正式映射，待查现有投影 | 9 |
| [ui_state.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ui/resources/ui_state.rs.md) | - | 缺正式映射，待查现有投影 | 93 |
| [widget_params.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ui/resources/widget_params.rs.md) | - | 缺正式映射，待查现有投影 | 69 |

### systems/ui/signifiers/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [animation.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ui/signifiers/animation.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ui/signifiers/mod.rs.md) | - | 缺正式映射，待查现有投影 | 154 |
| [signifier_create.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ui/signifiers/signifier_create.rs.md) | - | 缺正式映射，待查现有投影 | 57 |
| [signifier_mark_for_delete.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ui/signifiers/signifier_mark_for_delete.rs.md) | - | 缺正式映射，待查现有投影 | 0 |

### systems/ui_clutter/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [debug_spawn_clutter.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ui_clutter/debug_spawn_clutter.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [decoclutter_show_pins.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ui_clutter/decoclutter_show_pins.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [defered_autoclutter_history.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ui_clutter/defered_autoclutter_history.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ui_clutter/mod.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [rmb_autoclutter.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ui_clutter/rmb_autoclutter.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [ui_autoclutter.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ui_clutter/ui_autoclutter.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [ui_focus_decoclutter.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/ui_clutter/ui_focus_decoclutter.rs.md) | - | 缺正式映射，待查现有投影 | 1 |

### systems/wall/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [calc_wall_bend.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/calc_wall_bend.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [deferred_wall_history.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/deferred_wall_history.rs.md) | - | 缺正式映射，待查现有投影 | 3 |
| [delete_wall.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/delete_wall.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [draw_wall.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/draw_wall.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [finish_wall.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/finish_wall.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [merge_walls.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/merge_walls.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [set_wall_style.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/set_wall_style.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [structure_outline.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/structure_outline.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [ui_continue_wall.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/ui_continue_wall.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [ui_draw_wall.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/ui_draw_wall.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [upload_instanced_wall_ssbo.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/upload_instanced_wall_ssbo.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [wall_highlight.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/wall_highlight.rs.md) | [wall_highlight.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/wall/wall_highlight.rs:1) | 原单系统与真实 ECS 事件/资源接线已核验 | 0 |
| [wall_splitter.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/wall_splitter.rs.md) | - | 缺正式映射，待查现有投影 | 0 |

### systems/wall/adjust_wall_height/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [adjust_wall_height_impl.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/adjust_wall_height/adjust_wall_height_impl.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [adjust_wall_ui_mode.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs.md) | - | 缺正式映射，待查现有投影 | 5 |
| [signifier_animation.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/adjust_wall_height/signifier_animation.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [ui_adjust_wall_height.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/adjust_wall_height/ui_adjust_wall_height.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [ui_adjust_wall_height_mode.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/adjust_wall_height/ui_adjust_wall_height_mode.rs.md) | - | 缺正式映射，待查现有投影 | 4 |

### systems/wall/eraser/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [animate_wall_bulldoze.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/eraser/animate_wall_bulldoze.rs.md) | - | 缺正式映射，待查现有投影 | 6 |
| [archivist.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/eraser/archivist.rs.md) | - | 缺正式映射，待查现有投影 | 6 |
| [ui_bulldozer.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/eraser/ui_bulldozer.rs.md) | - | 缺正式映射，待查现有投影 | 3 |
| [wall_eraser.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/eraser/wall_eraser.rs.md) | - | 缺正式映射，待查现有投影 | 1 |

### systems/wall/shapes/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [calc_shape_height.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/shapes/calc_shape_height.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/shapes/mod.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [shape_placement.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/shapes/shape_placement.rs.md) | - | 缺正式映射，待查现有投影 | 6 |
| [shape_placement_signifiers.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/shapes/shape_placement_signifiers.rs.md) | - | 缺正式映射，待查现有投影 | 2 |

### systems/wall/shapes/circle/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [create_circle.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/shapes/circle/create_circle.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [ui_create_circle.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/shapes/circle/ui_create_circle.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [ui_edit_circle.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/shapes/circle/ui_edit_circle.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [ui_rotate_circle.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/shapes/circle/ui_rotate_circle.rs.md) | - | 缺正式映射，待查现有投影 | 0 |

### systems/wall/shapes/circle/signifiers/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [edit_arrow.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/shapes/circle/signifiers/edit_arrow.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [edit_line.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/shapes/circle/signifiers/edit_line.rs.md) | - | 缺正式映射，待查现有投影 | 0 |

### systems/wall/shapes/rectangle/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [create_rectangle.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/shapes/rectangle/create_rectangle.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [edit_rectangle.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/shapes/rectangle/edit_rectangle.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [finish_edit_rectangle.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/shapes/rectangle/finish_edit_rectangle.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/shapes/rectangle/mod.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [signifier_edit_dims_line.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/shapes/rectangle/signifier_edit_dims_line.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [ui_create_rectangle.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/shapes/rectangle/ui_create_rectangle.rs.md) | - | 缺正式映射，待查现有投影 | 3 |
| [ui_edit_rectangle_dims.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/shapes/rectangle/ui_edit_rectangle_dims.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [ui_edit_rectangle_orientation.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/shapes/rectangle/ui_edit_rectangle_orientation.rs.md) | - | 缺正式映射，待查现有投影 | 16 |

### systems/wall/shapes/rectangle/ui_modes/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [edit_dims.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/shapes/rectangle/ui_modes/edit_dims.rs.md) | - | 缺正式映射，待查现有投影 | 4 |
| [edit_orientation.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/shapes/rectangle/ui_modes/edit_orientation.rs.md) | - | 缺正式映射，待查现有投影 | 3 |

### systems/wall/shapes/utils/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [shape_signifier_lerp.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/shapes/utils/shape_signifier_lerp.rs.md) | - | 缺正式映射，待查现有投影 | 2 |

### systems/wall/snapping/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/snapping/mod.rs.md) | - | 缺正式映射，待查现有投影 | 2 |
| [snapping_audio.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/snapping/snapping_audio.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [snapping_init.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/snapping/snapping_init.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [snapping_maintain.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/snapping/snapping_maintain.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [snapping_to_ends.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/snapping/snapping_to_ends.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [snapping_to_segments.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/snapping/snapping_to_segments.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [telegraph_walls_are_merged.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/snapping/telegraph_walls_are_merged.rs.md) | - | 缺正式映射，待查现有投影 | 0 |

### systems/wall/valid_enclosure/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/valid_enclosure/mod.rs.md) | - | 缺正式映射，待查现有投影 | 52 |

### systems/wall/wall_state/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [debug_viz_threshold_curve.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/wall_state/debug_viz_threshold_curve.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [publish_public_wall_state.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/wall_state/publish_public_wall_state.rs.md) | [publish_public_wall_state.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/systems/wall/wall_state/publish_public_wall_state.rs:1) | 限定 CPU 发布链；原完整发布系统未恢复 | 4 |
| [spatial_hash_debug.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/wall_state/spatial_hash_debug.rs.md) | - | 缺正式映射，待查现有投影 | 1 |
| [spatial_hash_update.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/wall_state/spatial_hash_update.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [wall_roof_hashmap.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/wall_state/wall_roof_hashmap.rs.md) | - | 缺正式映射，待查现有投影 | 0 |
| [wall_spatial_hash.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/systems/wall/wall_state/wall_spatial_hash.rs.md) | - | 缺正式映射，待查现有投影 | 48 |

### utils/

| 原文件 | 当前源码 | 当前判定 | 原调用者 |
|---|---|---|---:|
| [audio_utils.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/utils/audio_utils.rs.md) | - | 缺正式映射，待查现有投影 | 45 |
| [child_wall.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/utils/child_wall.rs.md) | [child_wall.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/utils/child_wall.rs:1) | 已有代码，语义待复核 | 4 |
| [debug_viz.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/utils/debug_viz.rs.md) | [debug_viz.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/utils/debug_viz.rs:1) | 已有代码，语义待复核 | 40 |
| [dirty_tiles.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/utils/dirty_tiles.rs.md) | [dirty_tiles.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/utils/dirty_tiles.rs:1) | 补齐 cell 转换；既有位图/栅格链继续复核 | 67 |
| [geo.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/utils/geo.rs.md) | [geo.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/utils/geo.rs:1) | 已有代码，语义待复核 | 7 |
| [load_json.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/utils/load_json.rs.md) | [load_json.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/utils/load_json.rs:1) | 已有代码，语义待复核 | 10 |
| [mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/utils/mod.rs.md) | [mod.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/utils/mod.rs:1) | 依赖胶水已接；部分内联工具待取证 | 97 |
| [project_cursor_onto_ray.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/utils/project_cursor_onto_ray.rs.md) | [project_cursor_onto_ray.rs](/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/src/utils/project_cursor_onto_ray.rs:1) | 已修复尺寸比例边界；投影其余原差分待验 | 14 |

## 实际依赖

| 包 | 锁定要求 | 处理 |
|---|---|---|
| `anyhow` | `=1.0.86` | 复用原依赖 |
| `ash` | `=0.38.0` | 复用原依赖 |
| `bevy_ecs` | `=0.16.0` | 复用原依赖 |
| `bytemuck` | `=1.16.0` | 复用原依赖 |
| `dolly` | `=0.6.0` | 复用原依赖 |
| `fastrand` | `=1.8.0` | 复用原依赖 |
| `fnv` | `=1.0.7` | 复用原依赖 |
| `glam` | `=0.29.3` | 复用原依赖 |
| `half` | `=2.4.1` | 复用原依赖 |
| `hashbrown` | `=0.14.5` | 复用原依赖 |
| `hotwatch` | `*` | 复用原依赖 |
| `itertools` | `=0.14.0` | 复用原依赖 |
| `log` | `=0.4.27` | 复用原依赖 |
| `nohash-hasher` | `=0.2.0` | 复用原依赖 |
| `parking_lot` | `=0.12.1` | 复用原依赖 |
| `perchance` | `=0.4.0` | 复用原依赖 |
| `rapier3d` | `*` | 复用原依赖；可选 |
| `relative-path` | `=1.9.0` | 复用原依赖 |
| `serde` | `=1.0.200` | 复用原依赖 |
| `serde_json` | `=1.0.85` | 复用原依赖 |
| `slotmap` | `=1.0.6` | 复用原依赖 |
| `system-roof` | `*` | 已有 workspace 实现 |
| `tabloid` | `*` | 已有 workspace 实现 |
| `tg-history` | `*` | 已有 workspace 实现 |
| `utils` | `*` | 已有 workspace 实现 |
| `uuid` | `=1.15.1` | 复用原依赖 |
| `xxhash-rust` | `=0.8.10` | 复用原依赖 |
| `zeno` | `=0.2.2` | 复用原依赖 |
