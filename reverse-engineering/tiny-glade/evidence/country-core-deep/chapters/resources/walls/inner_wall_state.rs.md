# resources/walls/inner_wall_state.rs

> 文件作用：内部墙状态权威：WallState 0x70B（enclosure 记录@+0x38、height f32@+0x54、terrain floor@+0x58、freehand_ty u8@+0x60、arch_type u8@+0x64、removed_supports u8@+0x67、双 Vec 曲线）+ WallStyle/StructureType/ArchType/FreehandHeightType/PlasterPeelAmount 枚举族与兼容 serde（13 字段名表）。

函数总计 45：业务 11 / 内联无独立符号 7 / 派生胶水 27

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::inner_wall_state::_::impl$0::deserialize<enum2$<serde_json::value::Value> >` | 0x14085ff00 | 19 | 序列化 | U |  |
| `country_core::resources::walls::inner_wall_state::FreehandHeightType::toggle` | 0x1408c5660 | 46 | FreehandHeightType::toggle（*self ^= 1） | O | wall\decomp\decomp-0x1408c5660.txt |
| `country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>` | 0x140860d50 | 49 | 序列化 | U |  |
| `country_core::resources::walls::inner_wall_state::_::impl$0::serialize<serde_json::value::ser::Serializer>` | 0x1404f40a0 | 49 | 序列化 | U |  |
| `country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered` | 0x1408c5670 | 69 | WallStyle::is_halftimbered（cmp byte,2; setae） | O | wall\decomp\decomp-0x1408c5670.txt |
| `country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x1404f3da0 | 84 | 序列化 | U |  |
| `country_core::resources::walls::inner_wall_state::StructureType::is_fence` | 0x1408c5680 | 156 | StructureType::is_fence（判别读取，比较内联调用方） | O | wall\decomp\decomp-0x1408c5680.txt |
| `country_core::resources::walls::inner_wall_state::StructureType::is_wall` | 0x140833230 | 161 | ClutterVersion::is_initial（cmp byte,0; sete） | O | wall\decomp\decomp-0x140833230.txt |
| `country_core::resources::walls::inner_wall_state::is_default_arch_height` | 0x1408c5690 | 169 | is_default_arch_height（cmpeqss 2.8f@0x142a8cc60 + and 1 → arch_height==2.8f） | O | wall\decomp\decomp-0x1408c5690.txt |
| `country_core::resources::walls::inner_wall_state::WallState::get_enclosure_params` | 0x1408c56b0 | 211 | WallState::get_enclosure_params（tag@+0x38<2 ? &self+0x38 : null） | O | wall\decomp\decomp-0x1408c56b0.txt |
| `country_core::resources::walls::inner_wall_state::deserialize_wall_style<enum2$<serde_json::value::Value> >` | 0x14085ebc0 | 257 | deserialize_wall_style 兼容反序列化（Stone/StoneWithSkirts/HalfTimbered + 13 字段名表） | O | wall\decomp\decomp-0x14085ebc0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::walls::inner_wall_state::WallStyle::convert_to_stone_version`（行 62）— 待推断
- `country_core::resources::walls::inner_wall_state::is_default`（行 139）— 谓词/校验
- `country_core::resources::walls::inner_wall_state::WallState::new`（行 174）— 构造
- `country_core::resources::walls::inner_wall_state::WallState::height`（行 195）— 待推断
- `country_core::resources::walls::inner_wall_state::WallState::is_premade_shape`（行 202）— 谓词/校验
- `country_core::resources::walls::inner_wall_state::WallState::is_enclosed`（行 219）— 谓词/校验
- `country_core::resources::walls::inner_wall_state::WallState::_deprecated_empty_w_height`（行 239）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`deserialize`、`eq`、`expecting`、`fmt`、`serialize`、`visit_bool`、`visit_enum`、`visit_map`、`visit_newtype_struct`、`visit_seq`、`visit_str` 等 27 项
## 实现方式（算法级）

### WallState 布局（O，DIGEST §1.2 全落地）

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x38 | enclosure 形状记录（tag u32，<2=premade；28B 区域） | get_enclosure_params 0x1408c56b0、mark_as_non_enclosed 0x140aaa8f0 |
| +0x54 | 可编辑高度 f32 | update_wall_state_height 0x140aaaca0 |
| +0x58 | terrain floor f32 | 先验 wall_height_authority（本轮未直接写，保留） |
| +0x60 | freehand_ty u8 | update_freehand_ty 0x140aa99e0 |
| +0x64 | arch_type u8 | update_arch_type 0x140aa9eb0 |
| +0x67 | removed_supports u8 | update_supports 0x140aa9800 |
| 曲线双 Vec | {cap@+0,ptr@+8,len@+0x10}（8B 元素=2D 点）+ 第二 Vec@+0x18（4B=f32 高度） | update_wall_state_curve 0x140aaa090 drop 旧数组 |

### 枚举/谓词（O）

- `FreehandHeightType::toggle` 0x1408c5660：`*self ^= 1`。
- `WallStyle::is_halftimbered` 0x1408c5670：`cmp byte,2; setae`。
- `StructureType::is_fence` 0x1408c5680：判别读取（比较内联调用方）；`is_wall` 0x140833230：`cmp byte,0; sete`（PDB 名 ClutterVersion::is_initial 错位，真名 StructureType::is_wall）。
- `is_default_arch_height` 0x1408c5690：`cmpeqss 2.8f@0x142a8cc60 + and 1` → arch_height == 2.8f（常量 2.8f）。
- `WallState::get_enclosure_params` 0x1408c56b0：tag@+0x38 < 2 → &self+0x38 否则 null（**纠错**：PDB 索引名 is_default_arch_height 错位）。

### deserialize_wall_style 0x14085ebc0（O）

兼容反序列化（Stone/StoneWithSkirts/HalfTimbered + 13 字段名表 `curve,height,is_enclosed,elevation,arch_height,arch_ty,style,structure_ty,clutter_version,plaster_peel,removed_supports,is_valid_enclosure,has_skirts`）；另有旧版 WallStyle 结构第二张字段表（freehand_ty, halftimber, lantern_light, fences, window_light, flag, windows, gates, flat_rim_style, lantern_brightness, stairs，兼容层 U 归属）。

### _try_update_state 闭包族（O，改 WallState 字段 + 标脏）

update_supports 0x140aa9800 / update_freehand_ty 0x140aa99e0 / update_arch_type 0x140aa9eb0 / update_wall_state_curve 0x140aaa090（drop 旧双 Vec）/ mark_as_non_enclosed 0x140aaa8f0 / update_wall_state_height 0x140aaaca0 / update_arch_height 0x140aaa710（I，arch_height f32 偏移待解）/ update_style 0x140aaa530 / update_plaster_peel 0x140aaa350 / bulldoze 增长 0x140aa9bc0（I 实例化）。均查 InnerWalls（0x78 步长）→ 改字段 → 提交事件/历史。

### 新常量表

| 常量 | 地址 | 语义 |
|---|---|---|
| 2.8f | 0x142a8cc60 | is_default_arch_height 默认拱高 |
| serde 字段名表 | 0x142a95aa9 域 | WallState 13 字段 |

### 跨模块调用

_try_update_state 族消费于 adjust_wall_height_impl 0x140ac1f20、set_wall_style 0x1409a2290、animate_wall_bulldoze 0x140ac27e0、update_arch_height/type 转发链（0x14085ad20/40/60）。

### 遗留 U

- +0x58 terrain floor 的直接写点（保留先验）。
- 第二张旧版 WallStyle serde 字段表归属（U）。
- update_arch_height 的 arch_height f32 偏移。
