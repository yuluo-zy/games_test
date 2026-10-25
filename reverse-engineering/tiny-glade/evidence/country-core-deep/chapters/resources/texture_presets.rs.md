# resources/texture_presets.rs

> 文件作用：**纹理预设绑定类型（TexturePreset 数值槽/TextureFromDisk 名字哈希）。**

函数总计 6：业务 1 / 内联无独立符号 0 / 派生胶水 5

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::resources::texture_presets::impl$1::from | 0x140a59890 | 104 | `impl From<TexturePreset> for TextureDesc`（switch 0..0x1f：new_2d/new_2d_array→sampled→storage/color/depth/mip；48 B desc，+0x20 嵌 NameHash） | O（2026-10-14 反编译） | render-gap\decomp\decomp-0x140a59890.txt |

> 纠错（render-gap §5.4）：0xa59890 **不是** `From<u8> for TexturePreset`（该 From 无机器码证据）；reconstruction 的归因错位。
> `usage()` 判 11/12 为 depth 有误——机器码 0xb/0xc 是普通 `new_2d(0x200,0x200, 0x63/0x4c)+storage`；仅 0xa=depth_stencil_attachment、0xf/0x10=color_attachment、8=mip_levels(5)、0x11/0x12=2D array+depth。
> 预设表（尺寸×格式+hash）：0:64²/0x53/0xe3624a5fa0363c58；1-5,7,9,0xb,0xd-0x16:512²（0x4c/0x61/0x46/0x7c/0x63/0x53/9）；6,0xe,0xf,0x13,0x14:512²/0x61；8:512²/0x7a+mip5/0x09fa1678d41f0c18；0xc:256²/0x4c；0x10:512²/9+color；0x11/0x12:2D array(3,0x7e)；0x17:225²/0x4c；0x18:256²/0x25；0x19..0x1f:复杂路径（U）。
> **round3 闭合（2026-10-15，见 `round3/RENDER-U-CLOSURE.md` §7）**：key = `xxh3_64(预设名)`（const 折叠，名串不落二进制）；
> 已还原 11/25 名字：0=brdf_lut、5=paths_exclusion_mask、7=dirt_mask、8=terrain_lighting、9=terrain_heightmap_texture、
> 0xa=visual_terrain_heightmap_texture、0xe=rocky_terrain、0xf=ObjectGrassPush、0x11=ShadowMap、0x13=water_mask、0x14=garden_mask；
> 其余 14 名字 U（全部 SPIR-V 标识符/二进制串/结构化候选穷尽）。0x19..0x1f = **死变体**（表条目指向邻接函数指令中部，0xa5a020 `dof_prepare` 之后的 0x748 帧函数），`From` 只覆盖 0..0x18。

## 实现方式（算法级）（O，render-gap §5.4 + round3 §7 闭合）

**impl From<TexturePreset> for TextureDesc @0x140a59890**：
- `switch 0..0x1f`：`new_2d`/`new_2d_array` → sampled → storage/color/depth/mip 分派；
  产出 48B desc，+0x20 嵌 NameHash。（O）
- 0xb/0xc = 普通 `new_2d(0x200,0x200,0x63/0x4c)+storage`（**不是 depth**，纠错 §5.4）；
  仅 0xa=depth_stencil_attachment、0xf/0x10=color_attachment、8=mip_levels(5)、
  0x11/0x12=2D array+depth。（O）
- **round3 闭合**：key = `xxh3_64(预设名)`（const 折叠，名串不落二进制）；已还原 11/25 名
  （0=brdf_lut、5=paths_exclusion_mask、7=dirt_mask、8=terrain_lighting、9=terrain_heightmap_
  texture、0xa=visual_terrain_heightmap_texture、0xe=rocky_terrain、0xf=ObjectGrassPush、
  0x11=ShadowMap、0x13=water_mask、0x14=garden_mask），其余 14 名字 U。（O）
- 0x19..0x1f = **死变体**（表条目指向邻接函数指令中部，0xa5a020 dof_prepare 之后的 0x748
  帧函数），From 只覆盖 0..0x18。（O）
- `impl$1::from` 非 `From<u8>`（无机器码证据，reconstruction 归因错位）。O

## 派生胶水（derive/宏生成；不计独立业务）

`eq`、`fmt`、`hash`、`next` 等 5 项
