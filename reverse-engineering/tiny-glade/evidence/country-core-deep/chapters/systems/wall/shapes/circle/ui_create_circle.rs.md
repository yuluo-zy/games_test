# systems/wall/shapes/circle/ui_create_circle.rs

> 文件作用：圆形绘制 UI：CameraCursorRay::intersect_with_xz_plane 拖拽半径到 PreviewCircle；circle_signifier（custom shader）。

函数总计 6：业务 2 / 内联无独立符号 1 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::circle::ui_create_circle::ui_create_circle` | 0x14165bed0 | 0 | ui_create_circle（射线拖拽半径 → PreviewCircle） | O | ui-input\decomp\decomp-0x14165bed0.txt, wall\decomp\decomp-0x14165bed0.txt |
| `country_core::systems::wall::shapes::circle::ui_create_circle::circle_signifier` | 0x14165cd40 | 288 | circle_signifier（构建 cursor_circle_ 预览 signifier：FUN_14165bd40 圆圈预览 + signifier 流） | O |  |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle::new`（行 50）— 构造

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 2 项

## 实现方式（算法级）

**ui_create_circle（O，0x14165bed0）**：12 参系统。绘制圆形预览并提交 CreateCircleCmd。处理链：

1. **门控**：形状选择资源 `{tag@+0, ty@+4}` 须为 `(6, 1)`（圆），否则清 PreviewCircle（pbVar3[0..3]=0）返回。
2. **输入状态机**：`FUN_1409236d0(*param_7, &UNK_142c821f9, 0x12, ...)`（输入 action，名 = "wall_circle_canvas"）→ 状态字节（0/1/2/3）。
3. **光标距离 ≥ 60**（`*(f32*)(lVar15+0x3c) >= 60.0f@0x1429d18cc`）→ push 0x28B 事件行 `{u64, u32=0, center qword, radius qword}`（悬停重置）。
4. **拖拽半径（核心）**：射线（pfVar6 = 光线 {o.x,o.y,o.z, d.x,d.y,d.z}）与地平面求交：`t = (cursor_y − o.y)/d.y`；交点 `P = o + t·d`；`r = SQRT((cx−Px)²+(cy−Py)²)`（cx,cy = 锁定圆心，读 PreviewCircle+0x10）；**`radius = clamp(r, 0.9f, 8.0f)`**（0.9=0x3f666667@0x142a7c880、8.0@0x142934604）→ 写 `PreviewCircle + 0x18`（半径）；变化量 `|new−old| & 0x7fffffff`。
5. **预览写回**：`PreviewCircle + 0x10/+0x14/+0x20`（center/radius/…）；圆心在首击锁定时从光标位写入（`lVar15+0x30/+0x34`）。
6. **signifier**：`FUN_14165cd40(&uStack_128, {center, r}, flag)`（circle_signifier）→ `FUN_14146c880`（signifier 流）；另经 `func_0x000140c9cde0 ×7 / c97580 ×4 / c97560 ×3`（圆几何采样）构预览网格点，写入 `PreviewCircle + 0x348 / +0x360`（min/max 半径标记点：center + (0.9+Δ)·dir / center + (Δ+8.0)·dir）。
7. **提交**：释放（input state '3'）→ 写 param_8 Mut `{tag=0, id, ...}` + push CreateCircleCmd 事件行。

**circle_signifier（O，0x14165cd40）**：`FUN_14165bd40`（圆圈预览生成器，闭包）→ `FUN_140d79f90`（spawn）→ 分配 0x28B 记录 `{1, 1, center.x, center.y, radius, flag}`，前缀 `cursor_circle_`（0x142aed868 验证 = "edit_circle_" 前缀族；本函数类型名 UNK_142c82220/240）。
**常量**：min r 0.9 @0x142a7c880、max 8.0 @0x142934604、距离钳 60 @0x1429d18cc、abs 掩码 0x7fffffff @0x14292e980。
**数据结构**：PreviewCircle 记录 ~0x3a0B：+0x10 center Vec2、+0x18 radius f32、+0x20 附加、+0x348/+0x360 预览标记点、+0x3a0 分配 Vec。
**边界**：非圆模式 → 预览清零；`.unwrap()` 类 panic ×6（FUN_1428d9430）。
