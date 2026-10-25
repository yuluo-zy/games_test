# resources/roofs/roof_ridge.rs

> 文件作用：**屋顶脊线尺寸与几何**——`RoofRidgeDims`（脊线最小/指定轴尺寸）、局部尖顶中心、世界脊线点/线段分支、
> 反向 01 参数。**全部业务已由 reconstruction/roof/ridge.rs 恢复**（沿 evidence/roof-ridge/ 的 14,776 案例 0 差异验证），
> 本域不再重挖；逐函数标映射。唯一未归入重建的是 `to_world_space` 的闭包（胶水）。

函数总计 8：业务 6 / 内联无独立符号 1 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `RoofRidgeDims::to_world_space` | 0x1408e2750 | 0 | 世界脊线：max_dim≤0.1→单点（保留尖顶偏移）；否则要求矩形并按 ridge_dir 旋转出线段端点（`0.0*height` 保留符号零）。**= ridge.rs `ridge_to_world_space_observed`** | O | 已由 reconstruction/roof/ridge.rs 恢复；roofs-rt/decomp/decomp-0x1408e2750.txt + evidence/roof-ridge/tg_...RoofRidgeDims_to_world_space.c |
| `RoofRidgeDims::get_inner` | 0x1400885d0 | 24 | 内侧脊线尺寸（内缩后两轴；**= ridge.rs 内联辅助**） | O | 已由 reconstruction/roof/ridge.rs 恢复；evidence/roof-ridge/tg_...get_inner.c |
| `RoofRidgeDims::from` | 0x1408e2690 | 31 | 从 Roof 构造脊线尺寸：tag≠1→{0.1,0.1}；否则 `(1-ridge)*0.1 + ridge*shape_dims[轴]`，ridge_dir bit(+0x3c) 选轴。**= ridge.rs `ridge_dims_from_observed`** | O | 已由 reconstruction/roof/ridge.rs 恢复；evidence/roof-ridge/tg_...from.c |
| `RoofRidgeDims::calc` | 0x1408e2700 | 42 | 脊线尺寸计算（ridge_length_01 线性混合，最小轴 0.1）。**= ridge.rs `ridge_dims_calc`** | O | 已由 reconstruction/roof/ridge.rs 恢复；evidence/roof-ridge/tg_...calc.c |
| `calc_roof_tip_center_ls` | 0x1408e30b0 | 118 | 局部尖顶中心：circle→(r+r, r+r)，rect→(dim_x, dim_z)，乘 tip_offset_01 与 scale。**= ridge.rs `calc_roof_tip_center_ls_observed`** | O | 已由 reconstruction/roof/ridge.rs 恢复；evidence/roof-ridge/tg_...calc_roof_tip_center_ls.c |
| `inverse_ridge_01_param` | 0x1408e30f0 | 122 | 反向：`(ridge_len-0.1)/(dim-0.1)`，NaN→panic，夹紧 [0,1]。**= ridge.rs `inverse_ridge_01_param`** | O | 已由 reconstruction/roof/ridge.rs 恢复；evidence/roof-ridge/tg_...inverse_ridge_01_param.c |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `RoofRidgeDims::max_dim`（行 27）— 两轴取 max（NaN 优先 = SSE MAXSS 语义），ridge.rs `max_num`

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项（to_world_space::closure$0，0x1408e2eb0）

## 与 reconstruction/roof 的映射

见 [reconstruction/roof/ridge.rs](../../../reconstruction/roof/ridge.rs)：`RIDGE_MIN=0.1`、`ridge_dims_calc/from`、`calc_roof_tip_center_ls`、`roof_tip_offset_xz`、`inverse_ridge_01_param`、`ridge_to_world_space_observed` 全部为原机器码直接还原；验证报告 evidence/roof-ridge/verification.json（14,776 案例 0 差异）。
