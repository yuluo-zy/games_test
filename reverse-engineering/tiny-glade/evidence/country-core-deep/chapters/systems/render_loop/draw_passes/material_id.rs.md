# systems/render_loop/draw_passes/material_id.rs

> 文件作用：材质 ID 类型定义（`MaterialId` 类型 + fmt 实现，无业务函数）。

函数总计 1：业务 0 / 内联无独立符号 0 / 派生胶水 1

## 派生胶水（derive/宏生成；不计独立业务）

`fmt` 等 1 项（MaterialId 的 Display/fmt 实现）

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

- `MaterialId` = `#[repr(u32)]` 枚举（O，`<MaterialId as Debug>::fmt` 0x14085da60 读自 EXE 字符串表）：**Solid / Grass / Plant / Canopy / Forward / COUNT**（d_main §2）。
- 消费链：`build_draw_lists` 分派叶子写 `variant dword` = MaterialId 数值 → 模板参考值 `Some(variant)`/`Some(variant|0x80)`（跳转表 0x142abf3f0，d_gap §1.1）→ `DrawBuilder` variant（+0xb0）→ nani 剔除/材质 ID uniform。`MaterialId` 供 nani 材质剔除 ID 编号（wp5 §10）。

## 关键事实

- 无业务过程；`MaterialId` 供 nani 材质剔除 ID 编号（wp5 draw-passes §10 记名）。
