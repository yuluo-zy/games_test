# systems/wall/shapes/circle/ui_edit_circle.rs

> 文件作用：圆形编辑 UI：update_circle_radius（拖触发体 WallTriggerVolume::is_valid/edit_shape_default）到 update_wall_state_curve/update_wall_state_enclosure；finish_circle_edit 历史。

函数总计 13：业务 2 / 内联无独立符号 3 / 派生胶水 7

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle` | 0x14165d460 | 73 | ui_edit_circle（update_circle_radius → curve/enclosure；finish 历史） | O | wall\decomp\decomp-0x14165d460.txt |
| `country_core::systems::wall::shapes::circle::ui_edit_circle::show_full_circle_signifier` | 0x141660880 | 435 | UI 交互 | I | ui-input\decomp\decomp-0x141660880.txt, wall\decomp\decomp-0x141660880.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::shapes::circle::ui_edit_circle::finish_circle_edit`（行 463）— 生命周期控制
- `country_core::systems::wall::shapes::circle::ui_edit_circle::update_circle_radius`（行 521）— 系统:周期性处理
- `country_core::systems::wall::shapes::circle::ui_edit_circle::get_trigger_vol`（行 607）— 查询/计算

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 7 项

## 实现方式（算法级）

**ui_edit_circle（O，0x14165d460）**：14 参系统。编辑已放置圆形墙半径。处理链：

1. **查询目标墙**：`FUN_1405b0d70`（hashbrown 探针，0x78 步长 InnerWalls）取 WallId → WallState（+0x38 ShapeParameters）。
2. **update_circle_radius（case 2，核心）**：拖拽触发体（WallTriggerVolume::is_valid 类）命中后：
   - 新半径计算同 create（射线地平面求交 + `clamp(r, 0.9, 8.0)`，常量同 ui_create_circle：0.9@0x142a7c880、8.0@0x142934604）；
   - **变更阈值**：`|拖拽量| > *(f32*)(lStack_378+0x28)·1.0` 才更新；
   - `FUN_140c97cf0(&uStack_198, &puStack_120)`（**Circle2d::as_curve2**，新半径）→ `FUN_140aaa090(lVar19, wall, &uStack_2b8)`（**update_wall_state_curve**）→ `FUN_140aa9610(..., &uStack_2b8)`（**update_wall_state_enclosure**，写 `{tag=0, center, r}`）→ push 0x20B 历史行 `{u64, WallId, curve, u32, u16=4}`；再 push 0x28B 行 `{u64, WallId, curve, u32, eased_f32, flag=0}`（eased = `FUN_140c904a0` 平滑值）。
3. **变更检测（case 3，释放）**：`|new_r − old_r| <= 0.01f`（0x3c23d70a@0x142960674）→ **视为无变化** → `FUN_140b2c4b0`（DecoratorArchivist::extract_before_after_all 恢复）并跳过历史；否则 `FUN_141696f50`（CompoundEditBuilder 步）→ `FUN_140a84ec0`（record，路径 finish_circle_edit）。
4. **辅助**：`func_0x000140c97560/c97540`（圆 pos/normal）、`func_0x000140c906d0`（圆切线）、`FUN_141660880`（show_full_circle_signifier）、`FUN_14165b390`（预览网格）。非法状态 → panic `FUN_1428d9310(&UNK_142c82558)`。

**show_full_circle_signifier（I，0x141660880）**：全圆 outline signifier（ui-input 域另存 decomp）。
**常量**：变更阈值 0.01 @0x142960674、min 0.9 @0x142a7c880、max 8.0 @0x142934604。
**边界**：enclosure tag 非圆（bit0=0 检查 `*pbStack_130 & 1`）→ panic；半径差 ≤0.01 不记历史（恢复装饰）。
