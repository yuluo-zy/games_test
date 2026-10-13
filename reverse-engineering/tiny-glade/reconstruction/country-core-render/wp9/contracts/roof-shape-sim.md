# WP9 屋顶资源合同 — roof shape / simulation CPU 状态

Target: B（ljxsj_92385，EXE cf048e27…，PDB 2d9686b8…）。PDB 骨架证据包：
`skeleton/B/crates/country-core/src/resources/roofs/*.rs.*`（roof_shape/roof_tile_sim/
roof_animation/roof_ridge/roof_visual_shape/roof_trigger_volume/roof_noise/roof_buffers/
roof_tiles/dormer_tiles/commands）。本合同覆盖**形状/模拟 CPU 状态**；屋瓦实例见
`roof-tiles.md`；系统见 `roof-systems.md`。

标注：**O**=Observed（反编译/机器码直接证据），**I**=Inferred（调用链/PDB 命名推导），
**U**=Unknown（typed gap，不臆造）。

## 1. Roof（O，`resources::roofs::roof_shape`）

88 字节 ECS **组件**（`Query<&Roof>` / `Option<&Roof>`；stone_floor 行 stride 0x58 证据）。
`Roof::new`（0x1408e31c0）确认 88 字节布局（与 `reconstruction/roof/roof_state.rs` 的
`new_observed` 逐字节一致）：

| 偏移 | 类型 | 语义 | 证据 |
|---:|---|---|---|
| 0x00 | u64 | roof id（tiles.rs 读低 32 位作 roof_id） | O |
| 0x08 | byte bit0 | **type bit**：0=圆 roof，1=矩形 roof（pivot_3d/rotation_yaw/get_roof_shape 分支） | O |
| 0x0c..0x24 | [f32;7] | 截面几何：center_xz(0x0c,0x10)、圆半径/矩形长边(0x14)、圆旋转字/矩形(0x18)、矩形 dims(0x1c,0x20) | O |
| 0x24..0x40 | 28B | RoofShapeParams（见 §2） | O |
| 0x4c | f32 | **base_y**（墙 max_y；update_roof_spatial 写 0x1408e*：`roof+0x4c = max_y`） | O |
| 0x50 | f32 | base 高度参量（new 时从参数写；Roof::height 用作 roof 底） | O |
| 0x54 | byte | special_mode / 半木标记（update_roof_spatial 比较 is_halftimbered 并写回） | O |
| 0x58 | 0 | 组件总大小（0x55..0x58 未写，见 new_observed 注） | I |

Roof 方法（地址来自 enriched compiled_instances；正文来自本次 Ghidra 反编译 +
`reconstruction/roof/{surface,tiles,parameters,roof_state}.rs` 机器码验证）：

- `ty()`（0x1408e3250）→ RoofType：读 `height_01(0x30)`，`0.1 < v || v.is_nan()` → Gable，否则 Flat（roof_state.rs `ty_observed`，O）。
- `height()`（0x1408e3260，line 186）→ f32：**完整语义见 surface.rs `height`**：
  - 门：`height_01(0x30) > 0.1` 才计算；否则返回 0.0（`!h.is_nan() && h<=0.1 → 0.0`）。
  - size：圆 `t=(radius(0x14)-0.9)/7.1`、base 8.0；矩形 `t=(max(dim(0x1c),dim(0x20))-2.25)/12.75`、base 9.0（NaN dim → 取另一维，O）。
  - `u=clamp01(t/0.4)`；`smooth=(3-2u)·u²`；`cap=smooth·12+(1-smooth)·base`；断言 `cap>=0`（`f32::clamp` do_panic）。
  - `wanted=h·12+(1-h)·0.4`；返回 `min(cap, max(0,wanted))`（Ghidra 在断言后截断，最终 min 由机器码验证）。
- `is_flat()`（0x1408e3380）= `!ty()`，O；`is_gable()`（0x1408e33b0）= `ty() && ridge_length_01(0x34)==1.0`（`is_gable_observed`），O；
  `is_flat_w_no_rim()`（0x1408e3390）= flat 且 FlatRoofRimStyle==None（rim 字节 0x25 语义 U）。
- `pivot_3d()`（0x1408e3220）→ [f32;3]：`tag=u32(roof[8..12])`；`[roof[0x0c+tag·8], roof[0x4c], roof[0x10+tag·8]]`（= edit_rules.rs `pivot`），O。
- `shape_dims()`（0x1408e3800）：`if (roof[8]&1)==0 return circle-dims`；矩形 dims=roof[0x1c..0x24]（[f32;2]），O。
- `gable_sides()`（0x1408e33e0）→ `Option<[u8;2]>`：`(0x34==1.0 && 0x30>0.1)` 时 `Some([roof[0x3c], roof[0x3c]|2])`，否则 None，O。
- `roof_tip_offset_xz()`（0x1408e3780）→ [f32;2]：圆 `[(r+r)·tip_off[0]·0.5, (r+r)·tip_off[1]·0.5]`（tip_off=roof[0x24..0x28]）；矩形经 `Rectangle2d::orientation_mat2` 转世界（tiles.rs `tip`），O。
- `rotation_yaw_radians()`（0x1408e3b10）：矩形 `atan2f(-roof[0x10], roof[0x0c])`；圆用 `utils::circular_f32::CircularF32::as_f32()`，O。
- `profile_curve_normalized()`（0x1408e3420）→ utils Curve2：读 profile_01(0x2c)，`exp=profile·1.5+(1-profile)·0.5`，20 点 `[u, u^exp]`（u 常量 0,0.05263158…0.94736844，末点[1,1]），`Curve::try_new_from_points<Vec2,CurveCoordSemantic2D>()` 并 unwrap；= surface.rs `profile_curve_normalized`，O。
- `get_roof_shape(t)`（0x1408e4920）→ 28B 截面：圆 `{tag=0, center, r=blend·0.2+(1-blend)·r_interp, word0x18}`；矩形 `{tag=1, center, dims, top_xz}`，`blend=t^(1/exp)`，expanded=eave_ws 项；= surface.rs `get_roof_shape`，O。
- `roof_shape_at_intersection_y(y)`（0x1408e3820）：先算 §height 的完整 `h=min(cap,max(0,wanted))`；`if y==roof[0x4c]`（base）→ 直接复制 roof[0x8..0x24] 为截面，tag=0/1；否则 `t=inverse_lerp(0x4c, 0x4c+h, y)`，`u=t^(1/exp)`，圆→`Circle2d::new_rotated`(tag=0)/矩形→`Rectangle2d::new`(tag=1)；`y<base || y>base+h` → tag=2（无截面）。O（见下 get_roof_shape_from_pos_ws_clamped 链）。
- `get_roof_shape_from_pos_ws_clamped()`（0x1408e4750）= `get_height_01_unclamped_from_ws()` + `get_roof_shape()`，O。
- `get_height_01_unclamped_from_ws()`（0x1408e4790）：height 计算 + `utils::inverse_lerp`（把 ws 高度反演为 height_01，unclamped），O。
- `roof_uv_at_ws_pos(pos)`（0x1408e4040）→ `Option<…>`：调用 roof_uv_and_point_at_ws_pos 后提取 bool+u32/f32，O。
- `roof_uv_and_point_at_ws_pos(pos)`（0x1408e4090）：世界位置→屋顶 UV。内部：GlobalId/collider 哈希查找 + rapier3d 射线 + `IsoTransform::from_translation_rotation` + 局部坐标系矩阵，返回 `{tag=0, uv}` 或错误码 tag（0x301=碰撞失败、1=未命中、4=…），U（依赖整个碰撞域，纯 typed gap）。
- `register_required_components`/`clone_behavior`（impl$37，0x1408e6400/0x1408e6af0）：bevy Component derive 胶水；`#[require(...)]` 具体成员 U（create_roof 显式 spawn 全套组件）。

## 2. RoofShapeParams（O，roof_shape 模块内 26 字节）

PDB observed_game_structures：`[f32;2]@0, f32@8, f32@12, f32@16, f32@20, byte@24(ridge_dir), byte@25(rim)`。
= `reconstruction/roof/parameters.rs` `RoofParameterBlock`（Roof[0x24..0x40] 28B，含 2B 未用）：
tip_offset_01[2]、profile_01、height_01、ridge_length_01、eave_length_01、ridge_dir（Width=0/Length=1）、rim。

- `eave_length_ws()`（0x1408e3c30，line 435）= `(sqrt(profile)+(1-sqrt(profile))·0.5)·(1.5·eave+(1-eave)·0.0)`，O（surface.rs `eave_length_ws` 已机器码验证，VA 0x1408e3c30 一致）。
- `height_ws()`（line ~440）：I（方法存在，rva 为空——inline；与 roof_height_ws_from_wall_shape 复用 EasingFunction::apply）。
- `from_wall_shape()`：rva 空（inline），wall→RoofShapeParams 映射，I/U。

## 3. 枚举（O，roof_shape 模块）

- `RoofType { Gable, Flat }`（ty() 判别），O。
- `GableRoofStyle`：`is_stepped`（0x140833230）、`is_overhang`（0x1408c5680），O（枚举值 0/1/… U）。
- `FlatRoofStyle`：`is_wooden`（0x1408e31a0）、`pattern_id`（U）。
- `FlatRoofRimStyle`、`FlatRoofMaterial`：命名实体，判别/字段 U。
- `RoofRidgeDirection { Width=0, Length=1 }`（参数块 byte@24，parameters.rs `direction`），O。
- serialize/deserialize：serde_json CompactFormatter + Value 分发（impl$0），业务语义=保存文件格式，U。

## 4. RoofRidgeDims（O，`roof_ridge.rs`，脊线维度）

- `get_inner(roof)`（0x1400885d0，callers=283）：脊线内尺寸，O（正文简，返回 [f32;2] 类）。
- `from(roof)`（0x1408e2690）：`if roof[8] != 1（非矩形）return`；`if roof[0x3c] != 0（ridge_dir≠Width）return` → 只有 Width 向矩形脊线有 dims，O。
- `calc(...)`（0x1408e2700）：in_DL 布尔门；O。
- `to_world_space(...)`（0x1408e2750）：脊线局部→世界。`max_dim<=0.1` → 矩形 `orientation_mat2`+closure_0，tag=0（无脊）；否则（圆 panic）矩形 `ridge_dir==1` 分支用 `roof[0x14]`（axis dim）、`orientation_mat2`+`swizzlers::x0y`，完整 height（cap/wanted）参与，输出 28B `{tag=1, start, end}`，O。
- `calc_roof_tip_center_ls()`（0x1408e30b0）、`inverse_ridge_01_param(value, top, bottom, flag)`（0x1408e30f0）：`flag==0 → bottom=top`；`(x-0.1)/(bottom-0.1)` clamp01，`bottom==0.1` panic、越界/NaN panic，O。
- `RoofRidgeDims::max_dim`：inline，U。

## 5. RoofBoundsDims（O，`roof_visual_shape.rs`）

Tagged 20B：`{u32 tag@0, union{circle:[f32;2]@4(bottom,top=0.2), rect:[f32;2]@4(bottom),[f32;2]@0xc(top)}}`。
- `get_roof_expanded_top_and_bottom(roof)`（0x140a880c0）：= tiles.rs `expanded_bounds_observed`（机器码验证）。矩形 bottom=roof[0x1c,0x20]+expanded、top=0.1 或 dim·ridge+inset、tag=1；圆 bottom=(expanded·1.2+0.28)·0.5+radius、top=0.2、tag=0。O。
- `as_circle()`（0x140a88200）：断言 tag 位0==0，返回底部圆半径，O。
- `as_rectangle()`（0x140a88240）：断言 tag 位0==1，返回 `(self[4..12], self[0xc..0x14])`，O。

## 6. RoofTileSimulation（O，`roof_tile_sim.rs`）

16B ECS **组件**（Query `&RoofTileSimulation`；`create_roof` spawn 时 `RoofTileSimulation::new`）。
PDB OBS 字段 off=4 f32。
- `new()`（0x140939390）：`out[0..4]=0`，`out[4]=in_XMM1`（传入 f32），`out[8..16]=0`，O。
  - off0 f32（初 0）、off4 f32（传入）、off8 u64（0）。语义 I：off4 疑为 `wall_height_delta`（gravity 统一参数，见 roof-tiles.md 消费）；off0 疑为初始高度/标记。
- `register_required_components`（impl$2，0x140939470）、`Default`（impl$3）：bevy derive 胶水，`#[require]` 成员 U。
- source 行：`new` line 14，register line 3 → 文件约 14 行（小型）。

## 7. RoofAnimation / RoofMarkedForDeletion（O，`roof_animation.rs`）

- `RoofAnimation`：ECS **组件**（`Option<&RoofAnimation>`），WP6 已恢复为 ~8B gap。
  `is_being_animated()`（0x1408dbc20）：读标记位，O（正文简）；`new()`（0x1408dbc40）：初始化，O。
- `RoofMarkedForDeletion`：**Resource**（WP6 O），`mark_roof_for_deletion` 写。
- impl$2/impl$4 `register_required_components`（0x1408e06d0/0x1408e0740）：bevy 胶水，成员 U。

## 8. RoofTriggerVolume（O，`roof_trigger_volume.rs`）

72B ECS **组件**（`create_roof` spawn `RoofTriggerVolume::default`）。3 个集合：
`triangles`（36B/三角）、`normals`、`uvs`，各自 {ptr,len,cap}（位置见 decomp：+0x00/+0x08..0x18、+0x20..0x30、+0x38..0x48）。
- `default()`（0x1408e4520）：三集合初值各 `{0,4,0}`（“4”为集合容量标记，U 容器细节），O。
- `triangles()`（0x1408e4560）→ Range：`(ptr, ptr+len·0x24)`，空 panic，O。
- `set_triangles/set_normals/set_uvs`（0x1408e45b0/45f0/4660）：替换集合，`set_normals/set_uvs` 断言长度==三角数，O。
- `get_position_from_mesh_uv(uv)`（0x1408e46d0）→ `Option<[f32;3]>`：`utils::geometry::position_from_mesh_uvs::get_position_from_mesh_uv`，命中写位置，O/U（网格 UV 几何工具链 U）。
- 消费者：`systems/roof/src/roof_trigger_volume.rs::calculate_roof_trigger_volumes`（0x1421a3220，见 roof-systems.md）。

## 9. roof_noise（O，`roof_noise.rs`）

- `gpunoise::{hash1, hash2, hash_combine2, uint_to_u01_float, lerp}`：inline GPU 风格哈希，U（正文未展开）。
- `gpunoise::noise(x, y)`（0x140a152c0）：`floorf(x), floorf(y)` 起点，O。
- `apply_roof_noise(roof_or_dst, x)`（0x1412348f0）：`dst[+4] += noise()·0.3·smoothstep(clamp01(x))`，O（`smoothstep=(3-2t)t²`）。

## 10. roof_buffers（O，SSBO 元素命名，布局 U）

- `RoofSupportBeamSsbo`、`WoodenPlankSsbo`：命名实体，布局 U。
- `impl$15::fmt` 的 consts（LESS_SNOWY / NO_ERASER_HIGHLIGHT / TIMBEFRAME_PREVIEW / WALL_ID_IS_COLOR_ID）：格式化常量（shader 绑定开关），U。

## 11. dormer_tiles / commands（O 命名）

- `DormerTiles`（`dormer_tiles.rs`）：`Default`（impl$1），组件/资源未定，U。
- `commands.rs` 命令枚举：`CreateRoofCmd, DeleteRoofCmd, EditRoofCmd, HighlightRoofCmd, RecalculateRoofCmd, SetFlatRoofRimStyleCmd, SwitchRoofTypeCmd`（+ `fmt` impl），值布局 U（system_roof 命令与 country-core 命令的桥接语义）。

## 关键数值/边界裁定（机器码校验）

- height：圆 base 8.0/矩形 9.0、t 归一 `(size-0.9)/7.1` 与 `(max-2.25)/12.75`、`u=t/0.4`、smoothstep、`lerp(base,12)`；`wanted=h·12+(1-h)·0.4` 与 `min(cap,max(0,wanted))`（surface.rs 全函数机器码验证通过）。
- gable：`height>0.1 (NaN→true)` 且 `ridge==1.0`。
- Roof::new 8 参数写入 88B（roof_state.rs `new_observed` 逐字节验证）。
- RoofShapeParams eave_length_ws 公式（parameters.rs，62B 原函数验证）。
- 脊线 `inverse_ridge_01_param` 的 `0.1` 下限与 clamp/panic。

## 复用裁定（prior work）

| 已恢复文件 | 对应骨架 | 裁定 |
|---|---|---|
| `reconstruction/roof/surface.rs`（Curve2/profile/height/eave/get_roof_shape） | roof_shape.rs 方法 | **复用**；本次反编译逐函数确认 VA/公式一致，不重推导 |
| `reconstruction/roof/parameters.rs`（RoofParameterBlock） | RoofShapeParams | **复用**；PDB OBS 26B 与参数块一致 |
| `reconstruction/roof/roof_state.rs`（new/ty/gable） | Roof::new/ty/is_gable | **复用**；本次反编译确认 |
| `reconstruction/roof/tiles.rs`（expanded_bounds/surface/roof_tip_offset） | roof_visual_shape/roof_shape | **复用**；0x140a880c0/0x1408e3780 反编译一致 |
| `reconstruction/roof/edit_rules.rs`（RoofEditEvent 64B/apply_edit） | edit_roof 系统核心 | **复用**；本次 0x1421ab7c0 反编译字段/事件 tag 完全一致 |

## typed gaps（U）

RoofRidgeDims 内部字段名、RoofTriggerVolume 集合容器细节、gpunoise 哈希正文、
RoofType/GableRoofStyle/FlatRoofStyle 枚举判别编号、RoofAnimation 布局、RoofTileSimulation
off0 语义、`#[require]` 成员、roof_buffers SSBO 布局、roof_uv 碰撞链。
