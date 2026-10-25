# resources/grading_lut.rs

> 文件作用：**颜色分级 LUT：完整 .cube 解析器 + assets/luts 目录装载。**

函数总计 22：业务 8 / 内联无独立符号 2 / 派生胶水 12

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::resources::grading_lut::impl$2::load_cube::parse_rgb | 0x140b01cc0 | 0 | 0x140b01cc0：空白切分→3×f32 解析；失败 Err | O | audio-misc\decomp\decomp-0x140b01cc0.txt |
| country_core::resources::grading_lut::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error> | 0x1400ef340 | 24 | serde 反序列化派生（ColorGradingState visit_str） | O（serde 派生） |  |
| country_core::resources::grading_lut::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > | 0x1400ee2c0 | 24 | serde json 序列化派生 | O（serde 派生） |  |
| country_core::resources::grading_lut::_::impl$0::serialize<serde_json::value::ser::Serializer> | 0x140af00e0 | 24 | serde value 序列化派生 | O（serde 派生） | audio-misc\decomp\decomp-0x140af00e0.txt |
| country_core::resources::grading_lut::ColorGradingState::flattened | 0x140b00b20 | 50 | 0x140b00b20：{name,4×f32,byte} 扁平化→final_post/prepare_frame_constants 消费 | O |  |
| country_core::resources::grading_lut::GradingLut::load_unwrapped | 0x140b00bc0 | 60 | 0x140b00bc0：展平 PNG→RGBA8→3D 纹理；路径前缀 "assets/luts/" | O |  |
| country_core::resources::grading_lut::GradingLut::load_cube | 0x140b01050 | 86 | 0x140b01050（PDB 名 asset_name 错位）：逐行解析 TITLE 跳过/#注释跳过/DOMAIN_MIN 必须(0,0,0)/DOMAIN_MAX 必须(1,1,1)/LUT_1D_SIZE→错 "1D LUTs are not supported"/LUT_3D_SIZE 手写 u64 解析(±号/≤17位/溢出检查)→size³ 数组(×4B)；数据行 parse_rgb→RGB9E5 打包→3D 纹理(名 "grading lut")；超 size³→错 "Too many entries in the LUT…" | O | audio-misc\decomp\decomp-0x140b01050.txt |
| country_core::resources::grading_lut::GradingLuts::load | 0x140b01df0 | 172 | 0x140b01df0：readdir "assets/luts"→逐 DirEntry 闭包(0x140ae5bb0：DirEntry 空 panic "read_dir entry"、按扩展名分派 cube/unwrapped)→双 Vec(stride 0x28/0x38 cap-first) | O | audio-misc\decomp\decomp-0x140b01df0.txt |

## 实现方式（算法级）（O 全量，ALGORITHMS §1.4）

**GradingLut::load_cube @0x140b01050（PDB 名 asset_name 错位，完整 `.cube` 解析器）**：
1. 读文件（Result 检查）。
2. 逐行：空行/`#` 注释/`TITLE` 跳过；`DOMAIN_MIN` → parse_rgb 全 0 才合法（否则 anyhow
   `"DOMAIN_MIN must be 0 0 0"`）；`DOMAIN_MAX` 必须 (1,1,1)；`LUT_1D_SIZE` →
   `"1D LUTs are not supported"`；`LUT_3D_SIZE` → **手写十进制 u64 解析**（±号、≤17 位、
   溢出检查）→ `size³` 数组扩容（×4B，0 填充）；数据行 → parse_rgb → **RGB9E5 打包**
   （0x1417c66b0）入数组；超过 size³ → `"Too many entries in the LUT. Bailing before an overrun"`；
   行数不足 size³ → 越界 panic（0x1428d9518，grading_lut.rs）。（O）
3. 建 3D 纹理 `{size,size,size}` → 尺寸准备（0x140bd5310）→ 上传 `size³×4` 字节
   （名 `"grading lut"`，0x142ae6c70）；失败 → unwrap panic。输出 GradingLut
   `{name String@+0, size u32@+0x18, 纹理句柄区@+0x28..+0x58}`。（O）
- `parse_rgb @0x140b01cc0`：&str → 空白切分（0x140b00920）→ 3×f32 解析（0x140d8ba40），
  失败 Err。（O）
- `load_unwrapped @0x140b00bc0`：展平 PNG → RGBA8 → 3D 纹理；路径前缀 `"assets/luts/"`。（O）
- `GradingLuts::load @0x140b01df0`：read_dir `"assets/luts"`（失败 panic `"read_dir"`）→
  逐 DirEntry 闭包（0x140ae5bb0，按扩展名分派 load_cube/load_unwrapped，I）→ 双 Vec：
  A stride 0x28、B stride 0x38（cap-first）。（O）
- `ColorGradingState::flattened @0x140b00b20`：{name, 4×f32, byte} 扁平化 → final_post/
  prepare_frame_constants 消费。（O）

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::grading_lut::GradingLut::asset_name`（行 78）— 待推断
- `country_core::resources::grading_lut::GradingLut::lut_name_to_asset_name`（行 82）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`closure`、`default`、`deserialize`、`eq`、`expecting`、`visit_map`、`visit_seq` 等 12 项

## render-gap 核对（2026-10-14）

- 7 处 `unimplemented!`（`flattened`/`load_unwrapped`/`load_cube`/`lut_name_to_asset_name`/`asset_name`/`parse_rgb`/`load`）
  的落码依据 = audio-misc §2.9 反编译 + `reconstruction/country-core-render/wp19/analysis/decomp/load_cube_140b01050.c`
  （另有 `wp15/contracts/startup.md`、`wp15/src/startup/load_glade_grading.rs` 可对照）。
  `load_cube` PDB 名 `asset_name` 错位已记（audio-misc §2.9）；`GradingLut::asset_name` 实为 load_cube。

