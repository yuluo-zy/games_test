# systems/wall/shapes/rectangle/ui_edit_rectangle_dims.rs

> 文件作用：矩形尺寸编辑 UI：get_distance_to_edge 拖边到新 dims 到 EditRectangleCmd；decorator_cull_allow_any。

函数总计 5：业务 2 / 内联无独立符号 1 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims` | 0x140ad6e00 | 0 | ui_edit_rectangle_dims（变体） | O | ui-input\decomp\decomp-0x140ad6e00.txt, wall\decomp\decomp-0x140ad6e00.txt |
| `country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::ui_edit_rectangle_dims` | 0x140ad6250 | 43 | ui_edit_rectangle_dims（get_distance_to_edge 拖边） | O | wall\decomp\decomp-0x140ad6250.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::get_distance_to_edge`（行 301）— 查询/计算

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`closure` 等 2 项

## 实现方式（算法级）

**ui_edit_rectangle_dims（O，0x140ad6250）**：系统外壳。处理链：
1. 事件读取（`FUN_140c170f0`/`func_0x000140c18130` = bevy event next/refresh）；
2. `func_0x000140c9b680`（矩形边几何）+ `func_0x000141240b10`（光标-边命中判断）+ `func_0x0001408d98a0`；
3. 状态字符串 = `country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::ui_edit_rectangle_dims::f`（UNK_142ae128d，`FUN_140ad1bc0` 取长/哈希）。
4. 委托给 **do_ui_edit_rectangle_dims**（0x140ad6e00）。

**do_ui_edit_rectangle_dims（O，0x140ad6e00）**：**get_distance_to_edge 拖边算法（核心）**。处理链（对命中边索引 lVar25∈0..3）：
1. `FUN_140c9a5a0(&uStack_138, lVar25)`（get_distance_to_edge：光标到边距离）；
2. `FUN_140c99600(&rect, lVar25)` / `(&rect, lVar25+1)`（**矩形两个角** = 边端点）；`fVar47/fVar48` = 角位，`edge = P_{i+1} − P_i`；
3. `L² = edge.x² + edge.y²`，**防零钳制** `L² = max(L², ~2.9e-20)`（0x1e3ce508@0x142a7baa0，仅除零保护）；
4. **投影参数** `t = ((cursor − P_i)·edge) / L²`；`1 − t` 为另一侧权重；
5. **新 dims**：`new = 光标在边法向上的投影`（`fVar50 = w0·(c − (t0·P + t1·Q)) + w1·...`，逐分量）→ 写 PreviewRectangle/Mut 编辑状态 `{tag@+0=2, id, ..., 边索引 lVar25, 3×qword}`；
6. **发 EditRectangleCmd**（0x28B：`{u32=2, u32, u64, lVar25 u64, new_center, new_dims, ...}`）→ 墙曲线/enclosure 更新经 edit_rectangle 系统消费；`_UNK_142ae0d30` = 常量对（4.0 / 15.0：0x40800000/0x41700000 区）。
7. `FUN_140c9a5a0 ×3 / FUN_140c99600 ×2 / FUN_140562870/5629f0/5627b0`（Vec 扩容）、`func_0x000140c9cde0/c9cdc0`（曲线）。

**数据结构**：编辑状态记录 0x1e0 区（`*(u8*)(rec+0x1e0)=1` 命中标记）；EditRectangleCmd 0x28B（tag=2）。
**边界**：边长度² 除零钳制；`.unwrap()` panic（UNK_142c3aaa0/142c3ab70 路径）。
**跨模块**：`FUN_140c9a5a0` = utils/geometry get_distance_to_edge；`FUN_140c99600` = 矩形 get_pos_at_u（角）。
