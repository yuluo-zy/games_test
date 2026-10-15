# utils/project_cursor_onto_ray.rs

> 文件作用：**屏幕比例空间 + 光标→世界射线投影（游戏自有）**。`ProportionalScreenSpace`
> = 16 B `{size: Vec2 (w,h)@0, aspect: f32 (w/h)@+0x08, 1.0f@+0x0c}`（`from_window_size`
> 原字节）。`project_onto_ray` 把光标经两个 4×4 矩阵变换 + 透视除 → NDC → 屏幕线，
> `t=((cursor-near)·dir)/|dir|²` 投影，clamp 到 PSS 范围后 lerp 回世界点。

## 核心算法

```
from_window_size(w, h):  16 B {size=Vec2(w,h), aspect=w/h, pad=1.0f}（原字节）
transform_screen_space(p) -> Vec2:  （I→部分 O，2026-10-15 补反编译）
  per-axis 仿射重映射：helper(p.x?, p.y?, size.w, size.h)（FUN_140c906d0，机器码已读：
  x 分量 = v/min*2+c2；y 分量 = (c1-v)/size*2+c1；c1/c2 常量未解析——helper 未建函数
  NotFound，常量地址未定位）；x 结果再乘 aspect（param_1[2]）→ 归一化比例空间
project_onto_ray(cursor, ray):
  近/远点经两个 4×4 矩阵变换 + 透视除 → NDC → 屏幕线
  t = ((cursor - near)·dir) / |dir|²   // 点积投影
  t = f32::clamp 到 PSS 范围（越界/NaN panic，`/rustc/…/core/src/num/f32.rs`）
  世界点 = near + dir * t（lerp 回世界）
```

函数总计 4：业务 3 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `ProportionalScreenSpace::from_window_size` | 0x140ac8420 | 18 | 由窗口尺寸构造比例空间 | **O** | 原字节：w/h→f32、aspect=w/h、+0xc=1.0f |
| `ProportionalScreenSpace::transform_screen_space` | 0x140ac8450 | 27 | 屏幕坐标重映射 | **I** | 结构已明（双分量仿射 + aspect 乘）；helper FUN_140c906d0 常量未解析 |
| `ProportionalScreenSpaceCoord::project_onto_ray` | 0x140ac84a0 | 41 | 光标投影到世界射线 | **O** | 矩阵→NDC→t 投影→clamp→lerp；f32::clamp panic |

## 内联/无独立符号函数

- `ProportionalScreenSpaceCoord::from_screen_space`（行 36）— **O**（uv = space.transform(p)，名称推断）
