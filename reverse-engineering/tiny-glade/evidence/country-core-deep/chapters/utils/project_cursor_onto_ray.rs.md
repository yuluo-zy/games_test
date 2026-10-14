# utils/project_cursor_onto_ray.rs

> 文件作用：**屏幕比例空间 + 光标→世界射线投影**。`ProportionalScreenSpace` = 16 B
> `{size: Vec2 (w,h), aspect: f32 (w/h), pad 1.0}`（`from_window_size` 原字节）；`project_onto_ray`
> 把光标经两个 4×4 矩阵变换 + 透视除 → NDC → 屏幕线，`t=((cursor-near)·dir)/|dir|²` 投影，clamp 到
> PSS 范围后 lerp 回世界点。
> 证据：from_window_size 布局原字节、project_onto_ray 反编译、f32::clamp panic（`/rustc/…/num/f32.rs`）。

函数总计 4：业务 3 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `ProportionalScreenSpace::from_window_size` | 0x140ac8420 | 18 | 由窗口尺寸构造比例空间 | **O** | 原字节：w/h→f32、aspect=w/h、+0xc=1.0f |
| `ProportionalScreenSpace::transform_screen_space` | 0x140ac8450 | 27 | 屏幕坐标重映射 | **I** | PSS 3 f32 {min,max,scale} + helper 0x140c906d0 语义未吃透 |
| `ProportionalScreenSpaceCoord::project_onto_ray` | 0x140ac84a0 | 41 | 光标投影到世界射线 | **O** | 矩阵→NDC→t 投影→clamp→lerp；f32::clamp panic |

## 内联/无独立符号函数

- `ProportionalScreenSpaceCoord::from_screen_space`（行 36）— **O**（uv = space.transform(p)，名称推断）
