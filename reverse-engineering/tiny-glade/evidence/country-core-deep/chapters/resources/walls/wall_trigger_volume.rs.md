# resources/walls/wall_trigger_volume.rs

> 文件作用：墙触发体积：quad 数组 {ptr@+8,len@+0x10}（元素 0x60B）到 get_tris 输出 0x48B 三角形；recompute_quads 重算网格（panic !triangles/!mesh_uvs/!normals.is_empty）；形状编辑拖拽触发体。

函数总计 29：业务 9 / 内联无独立符号 9 / 派生胶水 11

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::recompute_quads` | 0x1408eae40 | 0 | recompute_quads（27KB；panic !mesh_uvs/!normals.is_empty()） | O | wall\decomp\decomp-0x1408eae40.txt |
| `bevy_ecs::system::schedule_system::impl$1::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)> >` | 0x140002370 | 116 | edit_arrow clone_behavior（xor eax,eax 返回 0） | O | audio-misc\decomp\decomp-0x140002370.txt, camera-time\decomp\decomp-0x140002370.txt, collision-eco\decomp\decomp-0x140002370.txt |
| `country_core::resources::walls::wall_trigger_volume::impl$5::clone_behavior` | 0x140002370 | 116 | edit_arrow clone_behavior（xor eax,eax 返回 0） | O | audio-misc\decomp\decomp-0x140002370.txt, camera-time\decomp\decomp-0x140002370.txt, collision-eco\decomp\decomp-0x140002370.txt |
| `country_core::resources::walls::wall_trigger_volume::impl$5::register_required_components` | 0x1408ecd50 | 116 | 构造/注册/生成 | U |  |
| `country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::default_tris` | 0x1408e80e0 | 201 | register_required_components（返回 {+0x20,+0x28} 对） | O | wall\decomp\decomp-0x1408e80e0.txt |
| `country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_tris` | 0x1408e80f0 | 207 | get_tris（quad 0x60B → tri 0x48B 输出；panic !triangles.is_empty()） | O | wall\decomp\decomp-0x1408e80f0.txt |
| `country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo` | 0x1408e8d70 | 219 | 访问器/转换 | I | wall\decomp\decomp-0x1408e8d70.txt |
| `country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::recompute` | 0x1408e9ed0 | 310 | 重算 | I | wall\decomp\decomp-0x1408e9ed0.txt |
| `country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_position_from_mesh_uv` | 0x1408ec0a0 | 552 | 访问器/转换 | I | wall\decomp\decomp-0x1408ec0a0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::walls::wall_trigger_volume::Quad::apply_params`（行 30）— 待推断
- `country_core::resources::walls::wall_trigger_volume::Quad::as_triangles_w_normals_n_uvs`（行 78）— 访问器/转换
- `country_core::resources::walls::wall_trigger_volume::WallTriggerVolumeParams::new`（行 145）— 构造
- `country_core::resources::walls::wall_trigger_volume::WallTriggerVolumeParams::edit_shape_default`（行 156）— 待推断
- `country_core::resources::walls::wall_trigger_volume::WallTriggerVolumeParams::offset_from_the_top_ws`（行 176）— 待推断
- `country_core::resources::walls::wall_trigger_volume::WallTriggerVolumeParams::u_range`（行 182）— 待推断
- `country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::is_valid`（行 196）— 谓词/校验
- `country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo_callback`（行 242）— 查询/计算
- `country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::recompute_tris`（行 522）— 重算

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 11 项
