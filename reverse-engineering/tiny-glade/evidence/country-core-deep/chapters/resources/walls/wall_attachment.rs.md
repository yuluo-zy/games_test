# resources/walls/wall_attachment.rs

> 文件作用：墙锚类型：WallAttachmentAnchor（platform WallId/wall_height_relative_y/anchor_ws[3]/corner/is_bottom_door/side）、WallAttachmentSide 0/1、WallCornerAttachment（0=None, 1..=4=矩形角 id+1）；serde 兼容层。

函数总计 22：业务 7 / 内联无独立符号 1 / 派生胶水 14

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>` | 0x1416fa8d0 | 6 | 序列化 | U |  |
| `country_core::resources::walls::wall_attachment::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x1400eed40 | 6 | 序列化 | U |  |
| `country_core::resources::walls::wall_attachment::_::impl$0::serialize<serde_json::value::ser::Serializer>` | 0x1416fa3a0 | 6 | 序列化 | U |  |
| `country_core::resources::walls::wall_attachment::_::impl$0::deserialize<enum2$<serde_json::value::Value> >` | 0x1416fa9e0 | 32 | 序列化 | U |  |
| `country_core::resources::walls::wall_attachment::WallCornerAttachment::is_attached` | 0x140b256b0 | 55 | WallCornerAttachment::is_attached（字节!=0） | O | wall\decomp\decomp-0x140b256b0.txt |
| `country_core::resources::walls::wall_attachment::WallCornerAttachment::from_rectangle_corner_id` | 0x140b256c0 | 58 | WallCornerAttachment::from_rectangle_corner_id（id<4 否则 panic） | O | wall\decomp\decomp-0x140b256c0.txt |
| `country_core::resources::walls::wall_attachment::WallCornerAttachment::to_rectangle_corner_id` | 0x140b256f0 | 64 | WallCornerAttachment::to_rectangle_corner_id（{present, id-1}） | O | wall\decomp\decomp-0x140b256f0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::walls::wall_attachment::is_default`（行 40）— 谓词/校验

## 派生胶水（derive/宏生成；不计独立业务）

`default`、`deserialize`、`eq`、`expecting`、`fmt`、`serialize`、`visit_enum`、`visit_map`、`visit_seq`、`visit_str` 等 14 项
## 实现方式（算法级）

### WallCornerAttachment（O，DIGEST §1.4 深化）

- `is_attached` 0x140b256b0：字节 != 0。
- `from_rectangle_corner_id` 0x140b256c0：`if (id < 4)` → 返回 `低字节 = id+1`（Option<u32> 编码，0=None、1..=4=角 id+1）；`id >= 4` → panic_str "assertion failed: id < 4"（0x142ae9ae8, line 0x18=24, file 0x142ae9b40 = wall_attachment.rs）。
- `to_rectangle_corner_id` 0x140b256f0（手解）：`xor eax,eax; test cl,cl; setne al; movzx edx,cl; dec rdx; ret` → 返回 {present: self!=0, id: self-1}（Option<u32>）。

### WallAttachmentAnchor / WallAttachmentSide（U 结构）

- 字段：platform WallId / wall_height_relative_y / anchor_ws[3] / corner / is_bottom_door / side（serde 兼容层 0x1416fa8d0/0x1400eed40 等为 derive 胶水）。
- WallAttachmentSide 0/1 判别（I）。

### 新常量表

| 串 | 地址 | 语义 |
|---|---|---|
| "assertion failed: id < 4" | 0x142ae9ae8 | from_rectangle_corner_id（line 0x18） |

### 跨模块调用

panic_str FUN_1428d9430。WallCornerAttachment 供装饰角吸附/门窗锚定位（decorator_archivist/trigger 域消费）。

### 遗留 U

- WallAttachmentAnchor 字段级布局（serde 名表已有，二进制偏移 U）。
- is_default（内联行 40）语义 I。
