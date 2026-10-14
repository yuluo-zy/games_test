# R3：render-gap 证据落码（reset_indirect_args、预设纹理表）

日期：2026-10-14。依据：`evidence/country-core-deep/render-gap/DIGEST.md`（提交 `26fe29d5`）。标注 O/I/U。
落点：`tiny-glade/crates/country-core`。验证口径见 `render-source-map.json` 的 `verification.r1_r2_real_workspace`。

## 落码
1. `reset_indirect_args`：0x1e0 B 暂存区改为 `draw_indexed_init()`（24 条 `DrawIndexedIndirectCommand`，仅 `index_count` 非零）。
   原判"无写入"是错的：0x14090a870 里有一处 `memcpy(staging, 0x142aaaa78, 0x1e0)`。
2. `TexturePreset::texture()`：`From<TexturePreset>`（0x140a59890）25 个预设全部落表（尺寸、格式、用途位、嵌入的 64 位哈希）。
   新增 `TextureDesc::{color_attachment, depth_stencil_attachment, new_2d_array}`（0x140bd53d0 / 0x140bd53f0 / 0x140bd52e0）。
   `create_preset_textures(device)` 直接用真实表；旧的带注入参数的版本改名 `create_preset_textures_with`（测试注入点）。

## 用 EXE 字节/汇编核对后纠正的分析错误
| 分析里的说法 | 实际（O） |
|---|---|
| 24 条命令的第 13 项 `index_count` = `0x10b` | EXE 字节 `01 0b 00 00`，小端 = `0xb01`（2817）。已在 DIGEST 就地更正 |
| 预设共 26 个（0..=0x19） | **25 个**。`enum_iterator` 折叠函数 0x141474b90 的状态 0..=24 产出预设，状态 25 是结束标记；`From` 跳转表里 0x19 的目标 0x140a5ab83 落在指令 0x140a5ab82 中间，不是代码入口 |
| 0xa59890 是 `From<u8> for TexturePreset`（WP21 归因） | 是 `From<TexturePreset>` → 预设纹理描述；`From<u8>` 没有机器码证据（保留为平凡新类型转换） |
| `usage()` 把预设 11/12 判为 depth | 11/12 是普通 storage；depth-stencil 附件是 10、0x11、0x12；color 附件是 0xf、0x10；只有 8 带 mip（5 级） |
| `load_json_as_mesh` 用"自定义 JSON 解析器（非 serde_json）"（utils-input-config DIGEST §3.3） | 被调函数 0x141405930 的 PDB 名是 `serde_json::de::from_str<Value>`，就是 serde_json——按项目规则直接复用，后续 E 包不重写解析器（**未落码，仅记录**） |

## 未做 / U
- 预设表里每个 64 位哈希应是预设名的 `NameHash`（xxh3），预设名本身没有还原（I）。
- `TextureDesc` 仍不建模 `+0x1e` 维度码（2D=2、2D 数组=3、3D=4）与 `+0x1d`。
- `+0x29b` 等其它 U 项不在本包范围。
