# systems/render_loop/compute_passes/apply_volumetrics.rs

> 文件作用：体积光/雾应用：`apply_volumetrics`（W48，应用体积效果到帧）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::apply_volumetrics::apply_volumetrics` | 0x1408a44c0 | 17 | W48 应用体积效果：pass "Volumetrics" + `/apply_volumetrics.cs.hlsl`；World 查资源（hash 0xbba451ee6a08a3ff）+ 读资源+0x4d0 状态字节（==0x04/0x01 切不同串）；MainCamera::mode + bind_shadowmap + read_texture + write_texture + uniform + dispatch；series 绑定 FUN_140905260/1409056d0/1408edd00/1408ee240 + FUN_1409dd4d0 收尾 | O | render-loop\decomp\decomp-0x1408a44c0.txt（串：Volumetrics、/apply_volumetrics.cs.hlsl）；wp5 compute-passes §7 |

## 关键事实

- 状态字节 +0x4d0 的 0x04/0x01 分支 = 质量/模式选择（U 具体语义）。
- 帧序 W48（frame-wiring，W42 阴影后）。
