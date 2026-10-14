# resources/roofs/roof_visual_shape.rs

> 文件作用：**屋顶可视形状查询族**——世界位置→height01/截面、扩展上下界（RoofBoundsDims）。
> 前三个函数（PDB 归属 roof_shape::Roof）复用 roof_shape 的截面/高度算法；`get_roof_shape` 完全 = reconstruction/roof/surface.rs。

函数总计 6：业务 6 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `Roof::get_roof_shape_from_pos_ws_clamped` | 0x1408e4750 | 21 | `h01 = get_height_01_unclamped_from_ws(pos)`，clamp[0,1] 后调 `get_roof_shape(h01)` 得截面 | O | roofs-rt/decomp/decomp-0x1408e4750.txt |
| `Roof::get_height_01_unclamped_from_ws` | 0x1408e4790 | 39 | 反向映射：`(pos.y + noise(x,z)*0.12? - floor@+0x4c) / (height_cap - floor)`；cap 同 Roof::height 的 smoothstep；噪声 = gpunoise::noise(x*0.1,z*0.1)（roof_noise 章） | O | roofs-rt/decomp/decomp-0x1408e4790.txt |
| `Roof::get_roof_shape` | 0x1408e4920 | 51 | 截面枚举产出：q=1.5p+(1-p)0.5，t^(1/q) 混合屋檐/屋脊；矩→{frame,dims}（28B），圆→{frame,radius}（20B+保尾）。**= surface.rs `get_roof_shape`**（3,004 案例） | O | 已由 reconstruction/roof/surface.rs 恢复；evidence/roof-surface/tg_...Roof_get_roof_shape.c（已拷 roofs-rt/decomp） |
| `get_roof_expanded_top_and_bottom` | 0x140a880c0 | 78 | 扩展屋顶上下界（eave 外扩后的底/顶矩形或圆参数），喂瓦片/脊线生成 | O | evidence/roof-tiles/0x140a880c0.c（已拷 roofs-rt/decomp）+ 0x140a880c0-disassembly.txt |
| `RoofBoundsDims::as_circle` | 0x140a88200 | 115 | `tag bit0==1 → panic`；否则返回 `{f32@+4, f32@+8}`（圆心/半径对） | O | evidence/roof-tiles/0x140a88200.c + 手解（rne-read --hex） |
| `RoofBoundsDims::as_rectangle` | 0x140a88240 | 125 | `tag bit0==0 → panic`；否则拷 arg2+4 的 16 字节矩形参数到输出 | O | 手解（rne-read --hex 0x140a88240） |

## 布局/结构要点

- `RoofBoundsDims`：`+0` tag 字节（bit0=is_rectangle）；圆字段 +4/+8，矩字段 +4..+0x14（as_circle/as_rectangle 各断言匹配形状，panic 行 0x28=40）。
- `get_height_01_unclamped_from_ws` 与 `roof_shape_at_intersection_y`（roof_shape.rs 章）共用高度 cap 计算块（同机器码模式）。
- 交叉域：roof_noise（噪声采样）、roof_shape（截面）、reconstruction/roof/surface.rs、roof-tiles（扩展界）。
