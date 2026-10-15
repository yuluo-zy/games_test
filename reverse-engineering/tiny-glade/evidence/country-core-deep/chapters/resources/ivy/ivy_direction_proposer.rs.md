# resources/ivy/ivy_direction_proposer.rs

> 文件作用：**生长方向噪声提议器：双 FastNoise 混合→θ∈[-40°,+40°]**

函数总计 3：业务 1 / 内联无独立符号 1 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer::get_direction` | 0x141234590 | 39 | 查询/计算 | O：IvyDirectionProposer::get_direction——3×noise3d+3×uniform 混合→θ∈[-40°,+40°]→(cosθ,sinθ) | collision-eco\decomp\decomp-0x141234590.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise::new`（行 107）— 构造

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项

## 实现方式（算法级）

- `get_direction`（0x141234590）：3×noise3d（双 FastNoise 资源，采样坐标映射 U）+ 3×uniform（fastrand）混合 → 角度 θ ∈ [−40°, +40°] → 返回 (cosθ, sinθ)。grower 主闭包每生长步调用（作为"提议方向"），随后 closest_segment_index 选墙段校验/吸附。
