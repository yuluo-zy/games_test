# render/directional_light.rs

> 文件作用：**方向光**——`DirectionalLight`（~0x78 B）：look-at 视图基 + 位置/目标/距离，`look_at` 建基、
> `set_direction_towards_light` 沿方向移动。证据：`reconstruction/country-core-render/wp4/contracts/
> view_frame_light_atmos_texture.md`（O）。

函数总计 4：业务 2 / 内联无独立符号 2 / 派生胶水 0

## 布局表（O，wp4）

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x00 | view-basis Mat4（旋转列 + 平移行，w=1） | look_at 0x140b18550 |
| +0x40 | position Vec3（+w 复用为打包对） | look_at |
| +0x50 | target Vec3（/+0x60 为 1.0 + target） | look_at |
| +0x70 | distance f32 | look_at / set_direction_towards_light |

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `DirectionalLight::look_at` | 0x140b18550 | 21 | 猜测：看向目标；证实：look-at 基构建 | **O**（725 B）：right/up/forward 归一化基、translation = -dot(basis, eye-ish)；存 position + target + distance=|eye-target| | wp4 |
| `DirectionalLight::set_direction_towards_light` | 0x140b18830 | 58 | 猜测：沿光向移动；证实：position += normalize(direction)*distance | **O**（695 B）：position += normalize(direction)*distance；重建 look-at 基朝 target；存新 position | wp4 |

## 内联/无独立符号函数

- `view_to_clip`（行 35）— **U**：视图→裁剪矩阵转换（无独立过程，体 U）
- `update`（行 75）— **U**：更新（无独立过程；I：transform/基重算入口）
- PDB 名存在无独立过程：`translate`（I）

## 纠错/差异

- 现有 reconstruction `directional_light.rs` 已按 wp4 合同落地（102 B 实现含测试），无差异。
- `translate`/`update`/`view_to_clip` 体 U（内联）。
