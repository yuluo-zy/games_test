# WP18 契约 — draw_bricks.rs

版本 **B**。数据源：skeleton bundle + Ghidra 只读反编译（b1）+ shader-uri-list.json。
标注：**O**=反编译直接支持；**I**=组合；**U**=未恢复。

## 类型（O）

| 项 | 值 |
|---|---|
| 编译单元 | 487 |
| `indirect_draw_bricks<0>` | 0x1408a53d0，size 1387，行 19（LOD0） |
| `indirect_draw_bricks<1>` | 0x1408a4ca0，size 1417，行 19（LOD1） |

const 泛型 `<0>/<1>` = 墙砖 LOD 变体（I：与 compiled `_wall_wall_brick_lod{0,1}`
对应）。

## 业务逻辑（O，两版一致，差异标注）

1. `+0xf0` 状态字节：`<0>` >1 → `discard`；`<1>` `&7` 取变体，case 3/4 直接
   `multi_draw_indexed_indirect`。
2. 读间接参数（u64：低 32 位 = 间接缓冲索引，高 32 位 = 期望键）→
   `unwrap_failed<Result>`；`+4` == -1 → discard 路径。
3. `UnsafeEntityCell::get<WallBrickInstancedDrawProxy>`（空 → `unwrap_failed`）；
   proxy 标记字节==1 → `discard`。
4. 置 `+0xa8` cull：`<0>` `|=4`；`<1>` `&=0xf3`（LOD1 关闭/不同 cull，I）。
5. 资源 `0x8226cc786e679355`（U：ssbo library）unwrap → `FrozenMap::get<SsboId,
   Box<BufferState>, NoHashHasher>` + `assert_failed<str,str>`（58 字节名字）+
   vtable+0x18 取 buffer 指针（空 → unwrap）。
6. `bind_external_storage_buffer` + `read_storage_buffer<BrickDrawListSsbo>`。
7. 资源 `0x850dbd51eafef756`（U）：buffer 空且标记 → `discard`；否则
   `bind_brick_erase_highlight`（0x14090be10）+ `bind_wall_color_customization`
   （0x14090bb70）+ `TextureFromDisk(0x7db9e7483fa24078)`（砖纹理）+ 
   `read_storage_buffer<RoofColorIdSsbo>` + `write_uniform<i32>`。
8. `multi_draw_indexed_indirect`（间接参数源 = W21/W27 `_wall_generate_draw_lists`
   写的 `DrawIndexedIndirectBuffer`，I）。

## Pass 组合 / shader URI（O）

| LOD | URI | compiled |
|---|---|---|
| 0 | `/wall/wall_brick_lod0.raster.hlsl` | `_wall_wall_brick_lod0.raster.hlsl_{b903…,f0ad…}` |
| 1 | `/wall/wall_brick_lod1.raster.hlsl` | `_wall_wall_brick_lod1.raster.hlsl_{b903…,f0ad…}` |

## typed gap（U）

- `0x8226cc786e679355`/`0x850dbd51eafef756` 资源 type-id 语义；FrozenMap 58 字节
  断言名字；`<1>` 变体开关 case 3/4 语义；`TextureFromDisk(0x7db9e7483fa24078)`
  纹理名。
- `bind_brick_erase_highlight`/`bind_wall_color_customization` O 为
  `DrawBuilder::bind_*` 方法；共享 crate 建模在 `CommonDrawParams`（集成裁决）。

## 交付

`src/.../draw_shaders/draw_bricks.rs`（`indirect_draw_bricks<const LOD: u8>`，
`unimplemented!()` 骨架）。
