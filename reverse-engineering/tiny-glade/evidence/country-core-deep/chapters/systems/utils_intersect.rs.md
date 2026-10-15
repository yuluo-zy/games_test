# systems/utils_intersect.rs

> 文件作用：**线 × 形状重叠判定。`line_x_shape_overlap` 对圆形（Circle2d 射线相交 + t∈[0,1]）或矩形（OBB 两轴 orientation_mat2 + Aabb2 射线相交）做射线求交，t 值 fmod 回绕；返回 `{hit, t1, t2}`。**

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::utils_intersect::line_x_shape_overlap` | 0x14175ada0 | 0 | 自由函数：`param_5` 形状标签 byte@0 的 bit0：0=圆（`FUN_140c97980` = Circle2d 射线相交，t 夹 `_UNK_142925904`=1.0 范围，`func_0x000142923f10` fmod 回绕，`func_0x000140c904d0` 取 t），1=矩形（`func_0x000140c99540` = orientation_mat2 两 OBB 轴，`FUN_14123fff0`/`FUN_1412401c0` = Aabb2 射线相交，`func_0x000141240b10` = is_hit）；命中写 `{1, t1, t2}` 至 param_1，未中写 0。t 超界 panic `'assertion failed: (0.0..=1.0).contains(&t)'`（0x142aa53ed，源 `utils/src/curve/curve_u.rs:42`，行号 0x2a）。 | O | decomp/decomp-0x14175ada0.txt；rne-read --unks |

## 实现方式（算法级）

**line_x_shape_overlap @0x14175ada0（自由函数，O）**：
1. `param_5` 形状标签 byte@0 的 bit0 分流：0=圆 / 1=矩形。（O）
2. 圆：`FUN_140c97980` = Circle2d 射线相交 → t 夹取 1.0 范围（`_UNK_142925904`）→ `func_0x000142923f10` fmod 回绕 → `func_0x000140c904d0` 取 t。（O）
3. 矩形：`func_0x000140c99540` = orientation_mat2 两 OBB 轴 → `FUN_14123fff0`/`FUN_1412401c0` = Aabb2 射线相交 → `func_0x000141240b10` = is_hit。（O）
4. 命中写 `{1, t1, t2}` 至 param_1，未中写 0 → 返回 `{hit, t1, t2}`（Option<RayBoxIntersection>-类，非 bool）。（O）
5. t 超界 panic `'assertion failed: (0.0..=1.0).contains(&t)'`（0x142aa53ed，源 `utils/src/curve/curve_u.rs:42`）—— 复用 CurveU 插值辅助。（O）

**签名（O）**：含额外 `param_3`（射线源偏移）与 `param_6`（f32），与重构签名不符（DIGEST 差异节）。（O）

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项（行 19 闭包）

## 备注

- 重构返回 `bool` 与证据 `{bool, t1, t2}`（Option<RayBoxIntersection>-类）不符（见 DIGEST 差异节）。
- t 断言源为 `utils::curve::curve_u`（跨域：utils crate），说明该函数复用了 CurveU 的插值辅助。
