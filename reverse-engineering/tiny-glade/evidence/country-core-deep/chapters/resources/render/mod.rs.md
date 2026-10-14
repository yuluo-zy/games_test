# resources/render/mod.rs

> 文件作用：**Affine3Packed 打包仿射矩阵**——渲染用紧凑 4x3（48 字节）仿射表示，供 GPU 实例/图集使用。
> 全部函数是纯矩阵→打包转换（业务面即布局与打包顺序）；`add_y` 确认 translation.y 在 +0x28。

函数总计 5：业务 4 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::render::Affine3Packed::from_mat3_translation` | 0x14089d4d0 | 16 | Mat3+translation→打包（movups/shuffle 重排到输出 48 字节 3×vec4 布局）；Ghidra 未建函数（<40B），机器码手解 | O | 手解（rne-read --hex 0x14089d4d0） |
| `country_core::resources::render::Affine3Packed::from_cols` | 0x14089d500 | 24 | 由列构造打包（Mat4 列重排，shufps/unpcklpd 重排 16 个 f32） | O | 手解（rne-read --hex 0x14089d500） |
| `country_core::resources::render::Affine3Packed::add_y` | 0x14089d550 | 39 | `addss [rcx+0x28]` —— translation.y += y（+0x28 是打包平移的 y 分量） | O | 手解（rne-read --hex 0x14089d550） |
| `country_core::resources::render::impl$1::from` | 0x14089d560 | 44 | `From<Mat4>`：3 行×4 列打包进 48 字节输出（movups/movlhps/shufps 重排，逐行写 +0x0/0x10/0x20） | O | 手解（rne-read --hex 0x14089d560） |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::render::Affine3Packed::from_scale`（行 32）— 由缩放构造（对角矩阵打包，内联）

## 布局/结构要点

- 输出 48 字节 = 3 个 xmm 寄存器（每行 16 字节），行写在 +0x00/+0x10/+0x20；平移 y 在 +0x28。
- 均为 Ghidra NotFound 的小函数（≤0x50 字节），已用原字节手解并落盘证据记录。
- 交叉域：下游为 render loop 实例变换上传；与 country-core-render 的 transform 上传合同对照。
