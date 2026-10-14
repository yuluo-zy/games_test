# render/nani/instance_data.rs

> 文件作用：每类 nani 实例的 GPU 布局类型：元素尺寸 O（0x30/0x40/0x60/0x70 分组），字段布局 U（归 nani_update_* 与 shader 侧）；CurvedHalfTimberInstanceData 三个构造器逐字节 O；WindowGlassFlags / NaniUberInstanceFlags / CurvedHalfTimberCaps 位域。

函数总计 22：业务 6 / 内联无独立符号 15 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::render::nani::instance_data::UberInstanceData::marker` | 0x14003bb80 | 28 | pub const fn marker()（1B ret，与其它 ZST 函数 ICF 合并）返回 InstanceDataMarker<Uber…> | O | B_n_world §1（0x14003bb80） |
| `country_core::render::nani::instance_data::impl$122::clone_behavior` | 0x140990af0 | 284 | Component Clone 派生薄封装（13B） | O | B_n_world §1（0x140990af0） |
| `country_core::render::nani::instance_data::impl$122::register_required_components` | 0x140990a80 | 284 | InstanceDataMarker<T> 组件注册（bevy 派生；行 284） | O | B_n_world §1（0x140990a80） |
| `country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::new_curved_xy` | 0x14098f690 | 343 | 0x40B 逐偏移写入：+0x10=start-half_len*0.5、+0x14=mid+half_len*0.5、+0x18=start、+0x1c=end、+0x28=细分计数值、+0x30=next_power_of_two(ceil((|y1-y0|*x1+1)/0.6))（clz 循环）；+0x30 细分等级 ilog2(next_pow2(…)).clamp(2,5) | O | B_n_world §1/§2.10（0x14098f690） |
| `country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::new_curved_xz` | 0x14098f800 | 375 | xz 平面对称构造（363B，同 0x40B 布局） | O | B_n_world §1（0x14098f800） |
| `country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::new_straight` | 0x14098f970 | 407 | +0x20 = caps_count*2 + (has_cap?2:1)；+0x24 = 0x3dc28f5c00000001；+0x2c = 0x100000000（打包字段语义 U） | O | B_n_world §1/§2.10（0x14098f970） |

## 内联/无独立符号函数

- `country_core::render::nani::instance_data::WindowGlassFlags::color_id_or_err`（行 59）— modular_bitfield 派生（color_id 位域读取；bit31 越界 unwrap 文案 O）—— I
- `country_core::render::nani::instance_data::WindowGlassFlags::set_color_id`（行 59）— 位域写 —— I
- `country_core::render::nani::instance_data::WindowGlassFlags::set_color_id_checked`（行 59）— 位域检查写 —— I
- `country_core::render::nani::instance_data::WindowGlassFlags::with_color_id`（行 59）— 位域 builder —— I
- `country_core::render::nani::instance_data::WindowGlassFlags::is_semitransparent_or_err`（行 60）— semi 位读取 —— I
- `country_core::render::nani::instance_data::WindowGlassFlags::set_is_semitransparent`（行 60）— 位域写 —— I
- `country_core::render::nani::instance_data::WindowGlassFlags::set_is_semitransparent_checked`（行 60）— 检查写 —— I
- `country_core::render::nani::instance_data::WindowGlassFlags::with_is_semitransparent`（行 60）— builder —— I
- `country_core::render::nani::instance_data::NaniUberInstanceFlags::has_albedo_texture_or_err`（行 208）— albedo 位读取 —— I
- `country_core::render::nani::instance_data::NaniUberInstanceFlags::is_semitransparent_or_err`（行 209）— semi 位读取 —— I
- `country_core::render::nani::instance_data::NaniUberInstanceFlags::with_is_semitransparent`（行 209）— builder —— I
- `country_core::render::nani::instance_data::NaniUberInstanceFlags::receive_contact_shadows_or_err`（行 210）— contact shadow 位读取 —— I
- `country_core::render::nani::instance_data::NaniUberInstanceFlags::pad_or_err`（行 211）— pad 位读取 —— I
- `country_core::render::nani::instance_data::CurvedHalfTimberCaps::to_u32`（行 322）— 端帽位域打包（行 322）—— I
- `country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::curved_halftimber_mesh_atlas_part_idx`（行 443）— PDB 仅 LF_FUNC_ID（内联残留）；**无机器码依据，reconstruction 已删除**—— U

## 派生胶水（1 项：clone 等）

1 项 glue = impl$118::clone（Component Clone 派生）。

## 证据注记（tg_rn1）
- **元素尺寸表**（O，update_instance_data<T> 最大写偏移 + impl$14::copy 步长核对）：0x40 = Uber/SolidVertexColor/Bricks/Window/WindowGlass/SolidAlbedoMap/SolidAlbedoMapEmissive/LanternEmissive/BalconyDoor/Smoke/StoneFloor/Cloth/CurvedHalfTimber；0x30 = Plant/Plaster/StairsFloor/**PreplacedFlower**（旧骨架 0x40 错）；0x60 = VelocityOnly；0x70 = Wren/GothicWindowBricks。
- **CurvedHalfTimber 构造器**（O，0x40B 逐字节）：`new_curved_xy/xz` 细分段数 = next_power_of_two(ceil((|y1-y0|*x1+1)/0.6))（clz 循环），细分等级 = ilog2(...).clamp(2,5)；`new_straight` 的 +0x24=0x3dc28f5c00000001、+0x2c=0x100000000 打包字段语义 U。
- **位域**：WindowGlassFlags（color_id bit31 → unwrap "value out of bounds for field WindowGlassFlags.color_id"，O）、NaniUberInstanceFlags（flags=albedo*1+semi*2+4）、CurvedHalfTimberCaps —— modular_bitfield 0.11.2 派生，字段布局 U。
- 纠错：`curved_halftimber_mesh_atlas_part_idx` 只有 PDB LF_FUNC_ID（内联残留），无机器码依据，reconstruction 已删除；`PlantInstanceData::marker()` 删除（真实 marker 只有 Uber）。
