# systems/wall/shapes/circle/signifiers/edit_arrow.rs

> 文件作用：圆形编辑箭头 signifier：Query 驱动箭头位姿（edit_circle_ 前缀）。

函数总计 5：业务 3 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::circle::signifiers::edit_arrow::impl$0::clone_behavior` | 0x140002370 | 14 | edit_arrow clone_behavior（xor eax,eax 返回 0） | O | audio-misc\decomp\decomp-0x140002370.txt, camera-time\decomp\decomp-0x140002370.txt, collision-eco\decomp\decomp-0x140002370.txt |
| `country_core::systems::wall::shapes::circle::signifiers::edit_arrow::impl$0::register_required_components` | 0x140b35ba0 | 14 | 构造/注册/生成 | I | wall\decomp\decomp-0x140b35ba0.txt |
| `country_core::systems::wall::shapes::circle::signifiers::edit_arrow::signifier_edit_circle_arrow` | 0x140b35120 | 18 | circle edit_arrow signifier | O | wall\decomp\decomp-0x140b35120.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 2 项

## 实现方式（算法级）

**signifier_edit_circle_arrow（O，0x140b35120）**：3 参系统（Query 驱动）。处理链：
1. 取墙记录 `lVar17 = *param_2`；**判别** `*(u8*)(lVar17+0x38) == 2`（WallState enclosure tag==Freehand）→ 跳过（不画箭头）。
2. 读 enclosure 参数（+0x2c 指针/+0x34）→ `FUN_140c90a40(&uStack_b8, &lStack_88)`（**圆箭头/切线方向计算**，utils/geometry）。
3. 读 +0x20/+0x28（箭头位姿：位置/旋转）→ `FUN_140d79f90`（spawn，闭包 `FUN_140b35b50`=register_required_components）→ 分配 **0x30B 记录 `{1, 1, 方向 qword, 位姿 qword, ...}`**。
4. signifier 前缀 = `edit_circle_`（UNK_142aed868 胖指针验证）。

**clone_behavior（O，0x140002370）**：`xor eax,eax; ret`（恒 0，clone 标记）。

**register_required_components（I，0x140b35ba0）**：返回组件注册表切片（{+0x20,+0x28} 对，如 wall_trigger_volume 同型）。

**边界**：Freehand 墙无箭头；分配失败 → alloc panic（FUN_1428d9000(0x10,0x30)）。
**跨模块**：`FUN_140c90a40` = utils/geometry 圆方向（central 关联）。
