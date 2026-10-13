# WP11 墙附着/装饰附着契约 — `resources/walls/wall_attachment.rs` + `decorator.rs`

标注：**O**=反编译/PDB/机器码直接支持；**I**=推断；**U**=未恢复（typed gap）。
B-only。源骨架：`skeleton/B/crates/country-core/src/resources/walls/{wall_attachment,decorator}.rs`。
S09 backlog：装饰物→墙锚点（anchor）与目的地（destination）类型。

## 1. `wall_attachment.rs` — 锚类型（O 主体）

| 类型 | 角色 | 证据 |
|---|---|---|
| `WallAttachmentSide` | 墙侧/壁龛枚举（字节 0/1） | O：`try_dst_as_wall` = destination side/niche <2（先验 decorator-info）；字段名 `side`（anchor 序列化器 1416fa690） |
| `WallCornerAttachment` | 角附着（字节枚举） | O：`is_attached` 0x140b256b0 = 字节!=0；`from_rectangle_corner_id` 0x140b256c0 = id<4 否则 panic；`to_rectangle_corner_id` 0x140b256f0。编码 I：0=None，1..=4=矩形角 id+1 |
| `WallAttachmentAnchor` | 墙锚（Info+8 起） | O（anchor 序列化器 1416fa690 字段名 + 先验 decorator-info 布局） |

### WallAttachmentAnchor 布局（O，anchor 序列化字段名 + B Info 偏移）

| Info 偏移 | 字段 | 类型 |
|---|---|---|
| +0x08 | `platform` | WallId u64（`WallId::is_valid` 0x140b34f20 测 !=0） |
| +0x10 | `wall_height_relative_y` | f32 |
| +0x14/+0x18/+0x1c | `anchor_ws` | [f32;3]（世界空间锚点） |
| +0x20 | `corner` | WallCornerAttachment（字节） |
| +0x21 | `is_bottom_door` | bool |
| +0x22 | `side` | WallAttachmentSide（0/1，兼 destination 判别 niche） |

序列化：`is_default`(行40) 内联 14 次（逐字段默认比较）；`serialize` 0x14016fa690/0x14016fa3a0（12 serialize_field）；`deserialize` 0x14016fa9e0 + visit_str 0x14016fa8d0（Compat：JSON 字符串/数字均接受，行32/44）。源行：6(derive)、32(serde)、40(is_default)、44(deserialize)、55/58/64(方法)。

## 2. `decorator.rs` — 装饰附着/信息类型（部分 O）

`DecoratorInfo` 72B 记录（先验 `reconstruction/building/decorator_info.rs` + `evidence/building-decorator-info/findings.md`）：

| Info 偏移 | 字段 | 证据 |
|---|---|---|
| +0x00/+0x04 | `coord`（WallSpaceCoord 两 f32） | O：WallAttachment 序列化 0x140a4b2a0 字段 `coord`（首 8B） |
| +0x08 | `anchor`（WallAttachmentAnchor） | O：同上字段 `anchor`（+8 起） |
| +0x2c | `id`（DecoratorId u32） | O：实际插入键（先验 findings） |
| +0x44 | `rank`（DecoratorRank u8） | O：`is_atomic/is_compound`（1..3 有效） |
| +0x46 | `type`（DecoratorType u8） | O：window cottage 0x38/gothic 0x39 等 |
| +0x47 | 调用者 R8 字节 | O：存在；语义 U |

### 类型面（PDB 骨架方法）
| 类型 | 方法 |
|---|---|
| `DecoratorId` | `new`、`get_inner`（u32 newtype，先验 NativeDecoratorId u32） |
| `DecoratorIdGen` | `generate`、`reserve`（ID 生成器） |
| `DecoratorInfo` | `dst_as_wall/try_dst_as_wall/dst_as_wall_mut/dst_as_roof/dst_as_stair/dst_as_terrain` |
| `DecoratorDst` | `as_wall/as_wall_mut/as_roof/is_wall`（目的地联合：Wall/Roof/Terrain/Stair） |
| `DecoratorExtraInfo` | `as_chimney/as_flag/as_lantern/as_stair/as_window` + `_mut` + `try_`（附加信息联合） |
| `DecoratorRank` | `new/x/is_atomic/is_compound` |
| `DecoratorRotationXZ` | `from_quat/from_xz_forward_vector/offset/to_projective_xz_mat2` |
| `DecoratorLocator` | `new` |
| `WallAttachment` | 顶层附着 `{coord: Vec2, anchor: WallAttachmentAnchor}`（装饰→墙） |
| `RoofAttachment`/`TerrainAttachment`/`StairAttachment` | 其它目的地附着（布局 U） |
| `ChimneyDecoratorInfo`/`FlagDecoratorInfo`/`LanternDecoratorInfo`/`StairDecoratorInfo`/`WindowDecoratorInfo` | 附加信息（U） |
| `ClutterVersion` | `is_initial`(0x140833230 机器码 `cmp byte,0; sete`)、`latest` |
| `StairRailingType`/`StairSupportType` | 楼梯附件枚举（U） |
| `DecoratorOpEvent`/`DecoratorOpEventKind`/`DeferredDecoratorOpEvents` | 装饰操作事件（U） |
| `TransientDecoratorIdGen` | 瞬态 ID 生成（U） |

## 3. 消费方与生产方（enriched 引用方节选）
- `PublicWallState::get_wall_attachment_height` 0xb2a890 → 消费方：ui_focus_stairs、decorator_on_wall_height_changed、move_decorators_following_anchors、handle_moving_out_wall_decorator_part（锚高=max_y 或 Freehand 投影）。
- `move_decorators_following_anchors` 0x14081c790 / `cache_decorator_transforms` 0x1408153d0 直接读 anchor/coord（先验 B-anchor-consumers）。
- 先验 `reconstruction/building/decorator_info.rs`（DecoratorInfo72 解码 + wall_attachment_height）、`decorator_placement.rs`、`decorator_subtype.rs`（wall_opening_family）、`decorator_archivist.rs` 对应本包装饰域。

## 4. 与 WP6/WP12 边界
- WP6 已登记 `resources/walls/{curved_halftimber,plaster,public_walls}` 消费装饰索引；本包提供 WallId/WallAttachment 类型供其引用。
- `decorator_storage.rs`（DecoratorStorage/DecoratorStorageAddr/Snapshot）、`decorator_archivist.rs`（DecoratorArchivist/DecoratorArchivalResources）、`decorator_collision.rs`（DecoratorPhysicsColliders）、`decorator_outline.rs`（DecoratorOutline）为装饰**存储/归档**资源，随 WP13+ 装饰系统恢复；本包只列类型名（U）。
