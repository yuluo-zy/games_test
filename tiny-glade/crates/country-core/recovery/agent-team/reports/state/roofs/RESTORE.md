# RESTORE — resources/roofs + systems/roof（卡A3，组 state，roofs 收尾核对）

> restorer-state，2026-10-16。核对现状 + 缺口登记；未做大件=诚实预算判定（同卡A2/A5 原则）。

## 现状核对（inventory ↔ src ↔ roofs-rt 证据）

| inventory 文件 | src 现状 | 覆盖 | 缺口 |
|---|---|---|---|
| resources/roofs/commands.rs | **EXISTS**（68L：CreateRoofCmd/DeleteRoofCmd/SwitchRoofDirectionCmd/SetFlatRoof*Cmd/EditRoofCmd…） | 命令类型/事件全 | — |
| resources/roofs/roof_shape.rs（61 函数/32 业务） | **EXISTS**（105L：Roof 0x58 投影 + 谓词 is_pitched/flat/gable/gable_sides + pivot_3d/shape_dims + 委托 system_roof surface/parameters） | new/pivot_3d/height/is_flat/is_flat_w_no_rim/is_gable/gable_sides/shape_dims；几何族委托 system_roof | **缺口**：is_stepped/is_overhang/pattern_id/is_wooden/profile_curve_ws/roof_uv_at_ws_pos/roof_tip_offset_xz/rotation_yaw_radians/height_ws/eave_length_ws/from_wall_shape/roof_shape_at_intersection_y/ridge_length_01/roof_uv_and_point_at_ws_pos（多数几何委托 system_roof，逐函数核对待续） |
| resources/roofs/roof_animation.rs（5 业务） | **EXISTS**（23L：RoofAnimation/RoofMarkedForDeletion） | 组件/删除标记 | 动画推进细节待核 |
| resources/roofs/roof_tile_sim.rs（3） | **EXISTS**（29L：RoofTileSimulation） | new + required-components | — |
| resources/roofs/roof_trigger_volume.rs（11/8 业务） | **EXISTS**（62L：三 Vec + set_* + get_position_from_mesh_uv） | new/triangles/set_*/get_position_from_mesh_uv（断言 len 相等） | — |
| **resources/roofs/roof_noise.rs（8 业务）** | **新增**（本批）：apply_roof_noise/revert_roof_noise/noise/hash1/hash_combine2/hash2/uint_to_u01_float/lerp | 机器码常量（尺度 0.5/振幅 0.3/smoothstep (3−2s)s²；hash 常量 0x19660d/0x8001/0xae135d3；角种子表 e10/e14=0x0ae135d3） | — |
| resources/roofs/roof_ridge.rs（8/7 业务） | **MISSING** | — | **缺**：to_world_space/get_inner/max_dim/from/calc/calc_roof_tip_center_ls/inverse_ridge_01_param（ridge 几何；reconstruction/roof ridge.rs 验证性恢复，可复用） |
| resources/roofs/roof_visual_shape.rs（6 业务） | **MISSING** | — | **缺**：get_roof_shape/get_roof_shape_from_pos_ws_clamped/get_height_01_unclamped_from_ws（消费 roof_noise）/get_roof_expanded_top_and_bottom/as_circle/as_rectangle |
| resources/roofs/roof_tiles.rs / dormer_tiles.rs / roof_buffers.rs | **MISSING** | — | 缺（tiles/dormer 组件胶水 + buffers 资源；roofs-rt HIGHLIGHTS #9：register_required_components 为标准 Bevy 胶水不计业务） |
| resources/history/roof_edit.rs（22/6 业务） | 半实现（RoofEdit 在 resources/roofs/commands.rs + recovered_building/building_history_transactions.rs） | 操作枚举（FlatRoofRimStyleChange/SwitchRoofRidgeDir…） | serde 0x140259e30/0x140b38b80 对齐待核（卡C 登记） |
| systems/roof/（editing/lifecycle/registration/replay） | **EXISTS**（~600L） | 创建/删除/类型切换/回放链 + register_roof_events（18 事件） | 视觉 bundle/碰撞构造/全局注册序（README 已披露） |
| startup/startup_roof.rs（0 函数） | **MISSING** | — | 缺（注册胶水，随卡G 域注册并入） |

## recovered_building 算法复核边界

- **roof_shape 28 业务全部澄清**（roofs-rt HIGHLIGHTS #1：24 O；Roof 几何族由
  reconstruction/roof ridge.rs/surface.rs/roof_state.rs 验证性恢复，不重挖）；
  `Roof::get_roof_shape` = surface.rs（3,004 案例 0 差异）。
- **Roof 0x58 布局关键偏移已证实**（HIGHLIGHTS #2：+8 tag/+0x24/28 tip/+0x2c profile/
  +0x30 height/+0x34 ridge/+0x38 eave/+0x3c ridge_dir/+0x3d rim/+0x4c floor/+0x54）；
  src `new_observed` 与逐偏移谓词一致。
- **RoofTriggerVolume 三 Vec 布局**（HIGHLIGHTS #3：stride 0x24/0x24/0x18 + len 断言）；
  与 src 一致。
- 复核边界：system_roof crate（共享）承载 surface/parameters/ridge/tiles/edit_rules/
  pipeline/rectangular_*；country-core 的 roof_shape 只保留组件投影与谓词，几何委托共享
  crate——**不重复实现依赖数学**。

## 本批新增

- `resources/roofs/roof_noise.rs`（+2 测试：noise [0,1] 确定性、apply/revert 强度尺度）。

## 编译/测试状态

- `cargo check -p country-core --offline`：我的文件零错误零警告。
- 全库测试目标编译被 audio 组 `systems/audio/audio_update.rs`（卡F 在途）瞬时打断；
  绿窗口期内基线 1629/1（唯一失败=pre-existing geometry/sphere）。

## 合同/状态建议

- inventory：roofs → `partial_implemented`（创建/删除/类型切换/回放链 + Roof 谓词 +
  噪声链已核；roof_ridge/roof_visual_shape/roof_tiles/dormer/buffers/startup_roof 缺登记）。
- roofs-rt 域证据（ALGORITHMS/DIGEST）已重锚引用（HIGHLIGHTS #1/#2/#3）。
- DISPATCH：卡A3 交 reviewer 终验。
