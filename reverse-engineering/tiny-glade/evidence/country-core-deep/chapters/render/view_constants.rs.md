# render/view_constants.rs

> 文件作用：**视图常量 + 视口常量构建器**——`ViewConstants`（~0x320 B：4×4 矩阵链 + 像素偏移 + 视口尺寸 +
> 缩放因子）+ `ViewportConstantBuilder`（构建完整矩阵链：proj×view、逆矩阵、prev view-proj 时间重投影、
> 像素偏移应用）。写入 `frame_constants` UBO（set1/b0）。证据：`reconstruction/country-core-render/wp4/contracts/
> view_frame_light_atmos_texture.md`（O）。

函数总计 4：业务 2 / 内联无独立符号 1 / 派生胶水 1

## 布局表（O，wp4）

| 偏移 | 内容 | 证据 |
|---|---|---|
| +0x00..0x7c | 两个 4×4 矩阵（view + 某矩阵） | ViewportConstantBuilder::build 写 +0x31c |
| +0x80..0xfc | 矩阵（proj / inverse view） | build |
| +0x100..0x17c | view-proj 矩阵 | build |
| +0x180..0x1fc | 更多矩阵 | build |
| +0x200 | u32 x/y（像素偏移） | build |
| +0x210 | 视口尺寸 u32 对 | build |
| +0x304..0x31c | 缩放因子 | build |

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `ViewConstants::set_pixel_offset` | 0x140b18af0 | 48 | 猜测：像素偏移；证实：重建投影列 | **O**（1131 B）：`ndc_offset = offset*2/viewport`；重建投影列（x/y 平移上的剪切）、逆 proj 与 view-proj；存 offset + viewport | render-top/decomp/decomp-0x140b18af0.txt（原 wp4） |
| `ViewportConstantBuilder::build` | 0x140b18f60 | 122 | 猜测：构建常量；证实：完整矩阵链 | **O**（2339 B）：proj×view、逆矩阵、prev view-proj（时间重投影）、像素偏移应用；写 ~0x320 B 结构（0x40 步长交错存储；逐列精确映射重且留为 I） | wp4；decomp: backend-misc/decomp/decomp-0x140b18f60.txt |

## 内联/无独立符号函数

- `ViewConstants::builder`（行 32）— **I**：返回构建器（4 个内联宿主）
- 内联宿主：`eye_position()` / `prev_eye_position()` / `fov_y_radians()` / `near_plane()`（I：平凡字段读）

## 派生胶水（1 项：default 等）

- ViewConstants 的 derive（Default），不补。

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `ViewportConstantBuilder::build`（0x140b18f60）—— 矩阵链
算法（O，2339B）：
1. `proj × view` → view-proj；
2. 逆矩阵（inverse view / inverse view-proj）；
3. **prev view-proj 时间重投影**（历史帧矩阵对，TAA/TGSR 运动向量源）；
4. 像素偏移应用（见 set_pixel_offset）；
5. 写 ~0x320B 结构（0x40 步长交错存储；逐列精确映射重，留 I）。
- 字段：+0x200 u32 x/y 像素偏移、+0x210 视口尺寸 u32 对、+0x304..0x31c 缩放因子。

### `ViewConstants::set_pixel_offset`（0x140b18af0）
算法（O，1131B）：`ndc_offset = offset*2/viewport` → 重建投影列（x/y 平移上的剪切）→ 逆 proj 与 view-proj → 存 offset + viewport。

- 消费链：写入 `frame_constants` UBO（set1/b0）→ 全部 draw/compute pass 的 `view_proj`/`prev_view_proj`/像素偏移 uniform（shadow map 级联也经 `sun_cascade` 视图矩阵）。
- U：ViewConstants 逐矩阵列映射（0x40 步长交错）；FrameConstants 全 UBO 字段列表。

## 纠错/差异

- U：`ViewConstants` 逐矩阵列映射（0x40 步长交错）；`FrameConstants` 全 UBO 字段列表。
- 描述符合同：`ViewConstants`/`FrameConstants` 写入 `frame_constants` UBO（set1/b0，render-passes/INDEX.md）。
- reconstruction `view_constants.rs`（279 B）已按合同实现（矩阵链 I 级落地）。
