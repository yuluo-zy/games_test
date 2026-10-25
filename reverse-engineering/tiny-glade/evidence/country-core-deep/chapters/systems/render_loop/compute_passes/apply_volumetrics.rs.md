# systems/render_loop/compute_passes/apply_volumetrics.rs

> 文件作用：体积光/雾应用：`apply_volumetrics`（W48，应用体积效果到帧）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::apply_volumetrics::apply_volumetrics` | 0x1408a44c0 | 17 | W48 应用体积效果：pass "Volumetrics" + `/apply_volumetrics.cs.hlsl`；World 查资源（hash 0xbba451ee6a08a3ff）+ 读资源+0x4d0 状态字节（==0x04/0x01 切不同串）；MainCamera::mode + bind_shadowmap + read_texture + write_texture + uniform + dispatch；series 绑定 FUN_140905260/1409056d0/1408edd00/1408ee240 + FUN_1409dd4d0 收尾 | O | render-loop\decomp\decomp-0x1408a44c0.txt（串：Volumetrics、/apply_volumetrics.cs.hlsl）；wp5 compute-passes §7 |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `apply_volumetrics`（0x1408a44c0，W48）
处理链（O，B_c3 §2）：
1. World 查资源（TypeId `0xbba451ee6a08a3ff` = MainCamera）+ 读资源 +0x4d0 状态字节；
2. **相机模式 ∈{4,1} → (density 0.075, size 10.0)，否则 (1,1)**（density/size uniform）；
3. `shadow_map` 预设 0x11、`blue_noise`（磁盘纹理键 xxh3 0x579f39ab91a01865）、`color_tex` 写 0x32、`depth_tex`；`current_view_index=0`；
4. `dispatch_2d(color extent)`。
- 绑定链（series，O）：FUN_140905260（bind_texture RG）+ 1409056d0（FromDisk blue_noise）+ 1408edd00/1408ee240（受管句柄）→ FUN_1409dd4d0 收尾。i_infra 后：blue_noise 走真实 `TextureFromDisk`（不登记 RG 访问，apply_volumetrics RG 写序 `[F,T,F]`）。
- 状态字节 +0x4d0 的 0x04/0x01 分支 = 质量/模式选择（U 具体语义）；帧序 W48。

## 关键事实

- 状态字节 +0x4d0 的 0x04/0x01 分支 = 质量/模式选择（U 具体语义）。
- 帧序 W48（frame-wiring，W42 阴影后）。
