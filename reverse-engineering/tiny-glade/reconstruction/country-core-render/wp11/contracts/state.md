# WP11 墙状态资源族契约 — `resources/walls` state types

标注：**O**=反编译/PDB/机器码直接支持；**I**=推断；**U**=未恢复（typed gap）。
B-only；地址均为 B patched EXE VA。源骨架：`skeleton/B/crates/country-core/src/resources/walls/`。
本契约覆盖**状态资源**（system/构造器消费方属 WP12，只列证据侧）。

## 1. `mod.rs` — WallId / Floor / WallHighlight / HighlightColor（O 主体）

| 类型 | 角色 | 证据 | 布局 |
|---|---|---|---|
| `WallId` | 全局墙 ID（newtype u64） | O：`WallId::get_inner` 0x140885d0 返回内值；`is_valid` 0x140b34f20 机器码 `cmp`/`setne` 测 u64!=0；serialize `_::impl$0::serialize` 0x140b35980 十进制写 Vec<u8>（JSON 数字串）；PDB `std::collections::hash::map::Keys<WallId, PublicWallState>` | **O**：`WallId(pub u64)`；Hash/Eq/Clone/Copy/Debug |
| `Floor` | 楼板（组件） | O：ECS `Option<&Floor>`；`nani_update_stone_floor` 读 +8 字节 bit0（WP6） | I：≥0x28B；+0x8 bit0 控制屋檐切线路径 |
| `WallHighlight` | 墙高亮（组件） | O：命名实体（PDB）；render 高亮消费 | U |
| `HighlightColor` | 高亮色枚举（字节判别） | O：`to_rgb` 0x140b34f30：判别字节 ==1 时 RGB=(0.1,0.03,0)；否则 (0.07,0.08,0.07)（机器码 xmm 常量） | O：字节判别 2 分支；变体名 U |
| `split_height_into_skirts`（模块级函数） | 墙"裙边"横带高度 | O：0x140b34f80：height<5.9 → 空；否则边界 4.5、+3.5 递增，条件 `height > f+1.4` | O：`fn(height:f32)->Vec<f32>` |

## 2. `public_wall_state.rs` — `PublicWallState`（O 主体，0x2a0 记录）

存储于 `PublicWalls: HashMap<WallId, PublicWallState>`（O，PDB hashbrown 泛型证据）。
先验布局 `reconstruction/building/wall_state.rs`（sha256 3ac4555841…）与本轮反编译一致：

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x10 | raw Curve2 点计数 u64 | O：get_wall_attachment_height 越界检查读 +0x10 |
| +0x30 | raw Curve2 长度 f32 | O：同上 `u * *(float*)(+0x30)` |
| +0xe8 | 另一数组长度 u64（mapped 3D outline?） | O：同上第二组越界检查（U 名称） |
| +0x108/+0x10c | max_y 缓存存在位/f32 | O：max_y 0x140b2a620 `(+0x108)&1` 否则 panic_fmt；get_wall_attachment_height 同 |
| +0x110/+0x114 | gable max_y 缓存位/f32 | O：max_y_gable 0x140b2a680 |
| +0x258..0x274 | 形状记录 28B（tag u32+24B 联合） | O：get_wall_shape 0x140b2a5d0 `*(u32*)(+600)<2` 否则 unwrap_failed；is_rectangle/is_circle 复制 +0x25c/0x264/0x26c |
| +0x274/+0x278/+0x27c | min/max position y / min relative height f32 | O（先验 wall_publication.rs） |
| +0x284 | constrained_extra f32 | O（先验 publication_metrics） |
| +0x28c | published raw Curve2 长度 f32 | O（先验 roof sync；wall_space 周期/末端） |
| +0x299 | style（WallStyle 字节） | O：`>=2` = halftimbered（is_halftimbered 0x1408c5670 机器码 `cmp byte,2; setae`） |

### PublicWallState 方法面（PDB 骨架 + 机器码）
| 方法 | RVA | 业务 |
|---|---|---|
| `is_premade_shape()` | inline（源码行113） | 形状判别 <2（Circle/Rectangle） |
| `is_enclosed()` | inline（行118） | valid_enclosure 判定 |
| `get_wall_shape()` | 0xb2a5d0 | 返回 &WallShape；判别 >=2（Freehand）→ unwrap_failed panic |
| `try_get_wall_shape()` | 0xb2a600 | 返回 Option<&WallShape>（判别>=2 → None） |
| `max_y()` | 0xb2a620 | +0x108 位0 存在→返回+0x10c；否则 panic_fmt |
| `max_y_gable()` | 0xb2a680 | +0x110 位0 存在→返回+0x114 |
| `flat_roof_y()` | 0xb2a6e0 | 调 max_y − 0.79（先验 wall_height_rules::flat_roof_y，bits 0xbf4a3d71） |
| `wall_space()` | 0xb2a740 | 墙可用 u 区间（调 is_enclosed；装饰/烟囱/窗/旗 消费方见附档） |
| `wall_space_length()` | 0xb2a760 | wall_space 长度（half_timber/wooden_floor_borders 消费） |
| `u_space()` | 0xb2a770 | 墙 u 参数区间（arches/lintels/segment 消费） |
| `is_rectangle()` | 0xb2a790 | 判别==1→复制 24B 返回 Option<Rectangle> |
| `is_circle()` | 0xb2a7d0 | 判别==0→复制 16B 返回 Option<Circle> |
| `corner_pos_and_normal_xz()` | 0xb2a800 | 仅 Rectangle：Rectangle2d::get_point/get_corner_normal；返回 {present u32, corner Vec2, normal Vec2} |
| `get_wall_attachment_height()` | 0xb2a890 | 先 max_y 缓存；Freehand：u×raw length→clamp 0..1→Curve 投影（装饰锚高） |
| `max_ivy_grow_height()` | inline（行~140） | 常春藤生长上限（命名实体证据） |
| `corner_normal_xz()` | inline（行~215） | 角法线（corner_pos_and_normal_xz 前段） |

消费方（enriched 引用方，节选）：get_wall_shape → construct_walls/construct_floors/segmentation/elevation_supports/fish/roof_outline；max_y → 71 直调（construct_walls、split_wall_into_sides、maintain_plaster、structure_outline、ui_move_shapes、ui_focus_mode_shape_height、wall_splitter、wall_eraser、ui_edit_rectangle_*）；flat_roof_y → stairs_assemble、find_stair_placement_candidate、ui_place_decorator、ui_change_stair_height、ui_dispatch_decorator_affordance。

## 3. `inner_wall_state.rs` — WallState + 枚举族（部分 O）

| 类型 | 角色 | 证据 |
|---|---|---|
| `WallState` | 内部权威编辑状态（InnerWalls 值） | O 方法面：`new`(行174)、`height()`(行195)、`is_premade_shape`(行202)、`get_enclosure_params`(行211, RVA 0x8c56b0)、`is_enclosed`(行219)、`_deprecated_empty_w_height`(行239)。布局 U；先验 `wall_height_authority.rs`：源行 +0x54=可编辑高度、+0x58=terrain floor（O） |
| `WallStyle` | 墙材质枚举（字节） | O：`convert_to_stone_version`(行62)、`is_halftimbered`(行69/70, RVA 0x8c5670：机器码 `cmp byte,2; setae` → >=2)。变体 O：Stone=0、StoneWithSkirts=1、HalfTimbered=2（deserialize_wall_style 0x85ebc0 字符串比对 "Stone"/"StoneWithSkirts"/"HalfTimbered"）；其余变体 U |
| `StructureType` | 结构类型枚举（字节） | O：`is_fence`(行156/157, RVA 0x8c5680 机器码 `movzx byte[rcx];ret`=判别读取，比较在调用方内联)、`is_wall`(行161, RVA 0x833230)。变体值 U（先验 wall_production.rs：StructureType tag 在 24B 辅助块 +20；tag2=plain） |
| `ArchType` | 拱类型枚举（字节） | O 命名实体；deserialize/serialize（行27）。变体 U |
| `FreehandHeightType` | 徒手高度类型枚举 | O：`toggle`(行46, RVA 0x8c5660：机器码 `*self ^= 1`，字节枚举) |
| `PlasterPeelAmount` | 石膏剥离量枚举 | O 命名实体；clone(行84)。变体 U |
| `deserialize_wall_style` | WallStyle 兼容反序列化 | O：0x85ebc0；visit_bool(行270)/visit_str(行280)；含旧 bool→style 兼容 |

WallState 记录先验来源：`reconstruction/building/wall_height_authority.rs`（+0x54 height、+0x58 floor）、`wall_height_command.rs`（40B payload 命令）、`wall_authoring.rs`（创建参数）。

## 4. `inner_walls.rs` / `public_walls.rs` — 资源

| 类型 | 角色 | 证据 |
|---|---|---|
| `InnerWalls`（Resource） | `HashMap<WallId, WallState>` 内部状态 | O：命名实体；WP12 system 签名 `ResMut<InnerWalls>`（adjust_wall_height_impl 等） |
| `PublicWalls`（Resource） | `HashMap<WallId, PublicWallState>` 发布状态 | O：命名实体；`nani_update_curved_halftimber/plaster` 读；hashbrown 泛型 PDB 证据 |
| `WallEntities`（Resource） | `HashMap<WallId, Entity>` 墙实体 | O 命名实体；布局 U |
| `WallPlasters`（Resource） | `HashMap<WallId, Entity>` 石膏实体 | O 命名实体；布局 U |

内部字段（dirty 集合、roof hashmap 关联）为 typed gap；`wall_roof_hashmap.rs` 的 Recalculate* 事件属 WP12。

## 5. 旁证复用
- `reconstruction/building/wall_state.rs` — PublicWallState 投影（0x2a0）；`wall_publication.rs` — 发布度量与 +0x274..+0x284 字段；`wall_height_rules.rs`（`reconstruction/wall/`）— clamp/max_y/gable/flat 数值策略（6624 案例）；`wall_height_authority.rs` — 内部源行 +0x54/+0x58；`wall_cache.rs` — regular wall height cache（0x14078b865..0x14078bc96）。
- 上述先验全部 B-only，未重推导；本包以其为基础组织骨架对齐 Rust。
