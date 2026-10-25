# systems/ui_clutter/rmb_autoclutter.rs

> 文件作用：右键移除 auto-clutter（handle_rmb_autoclutter；wp20 合同）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui_clutter::rmb_autoclutter::handle_rmb_autoclutter` | 0x140976ca0 | 23 | 系统:输入处理（handle_rmb_autoclutter） | O：1.4KB（wp20 + 本域复核） | ui-input\decomp\decomp-0x140976ca0.txt |

## 实现方式（算法级）

- `handle_rmb_autoclutter` @0x140976ca0（O，本域手解）：右键移除 auto-clutter。
  - 消费右键命中后 hashbrown SIMD 查找（控制字节 probe + 实体位集 bitmap 判定），定位 auto-clutter 实体。
  - 朝向/旋转计算（O）：三向量点积 `fVar30 = a·p`，二次/交叉项 → `func_0x000142923e20`（平方根）→ 角度；经 `FUN_1408b83d0` 换算。
  - 推 0x48B `EvRightClickedAutoClutter` 事件（pos/rot、@+0x29=3）；末尾 alloc 0x18B Arc → `FUN_140a84ec0(tag 0xd)` 入历史。
  - panic 分支（O）：`FUN_1428d9760(&UNK_142ab66f0, 0x2b, ..., &UNK_142ab6968)` bounds 检查（@+0x1a8 组件表越界）。
