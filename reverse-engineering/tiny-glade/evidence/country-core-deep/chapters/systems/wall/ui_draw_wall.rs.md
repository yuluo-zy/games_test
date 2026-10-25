# systems/wall/ui_draw_wall.rs

> 文件作用：墙绘制 UI 入口：glade 内拖动且非清画布到 RecordMouseInputIntoWall 转发进 CurveNursery；释放到 WallDrawState::finish + FinishWall。5 ResMut（含 WallDrawState/CurveNursery）。

函数总计 3：业务 1 / 内联无独立符号 1 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::ui_draw_wall::ui_draw_wall` | 0x140aafe40 | 0 | ui_draw_wall（5 ResMut；RecordMouseInputIntoWall 转发 + FinishWall） | O | ui-input\decomp\decomp-0x140aafe40.txt, wall\decomp\decomp-0x140aafe40.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::ui_draw_wall::WallDrawState::finish`（行 40）— 生命周期控制

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项

## 实现方式（算法级）（O，decomp-0x140aafe40）

**处理链**：16 参。**模式门**：param_1 = Res<AppMode>；`tag==0x12`（fence 画线）或
`(tag==6 && 字节@+4==0)`（wall 画线）才继续，否则 return。signifier：**`fences`**（0x142ade145，
len 6）与 **`wall`**（0x142ade14b，len 4）。
**WallDrawState**（param_2 deref，ResMut）：`state u8@0`、`u64@+0x10`（清零）、`bool is_fence@+0x18=
(tag==0x12)`；param_3 = 另一 ResMut 记录（+0x10/+0x18 同域写）。
游标结构（query `FUN_14092df50`，param_8 Query + param_16 组件）：`state u8@+0x18`、+0x19 字节、
`int@+0x28`、`signifier ptr@+0x38`、ptr@+0x90。param_6 = Res<射线命中> `{present u8@0, pos…}`。

1. **绘制激活**（state@+0x18 != 0）且 signifier 匹配且非擦除类（+0x28 != 2）：
   - `FUN_1409e9640`（TerrainHeightsData 采样，param_12 deref）；**高度 < 0.2f（0x1429cd068）→
     water hint**：`func_0x000140c9cde0` 投影位置，构 0x30B 束 `{1,1,{x,y}打包,z,距离 SQRT…}`，
     **Commands::spawn**（FUN_140d79f90，signifier `water_hint` 经胖指针 0x142ade160、display 回调
     FUN_140d4b7d0），`FUN_14146c880`（**RecordMouseInputIntoWall 转发路径**，param_13 deref）。
2. **状态机**（`cVar22 = min(游标state-2, 3)` → jump table 0x142ade224）：
   - case1 起笔：按下且 `FUN_140c9a630`（GladeBorder 内）→ WallDrawState=1；否则 push 8B 事件
     （param_10 EventWriter）+ 间接分发。
   - case2/3：状态迁移/设 3；case4/7：位旋转（模式标志）。
   - 共享 switch（0x142ade234）：
     - case1：`FUN_1413e7230`（param_16 事件写）+ param_4 push 8B + WallDrawState 记录写（param_3）。
     - case2：param_4 push 8B；temp 曲线（param_15 deref）非空 → param_14 push **0x18B 行**
       `{u64@0, wall_id@8, u8@0x10=0}`。
     - **case3 收尾**：param_5（EventWriter<FinishWall>）push **16B 行 `{u64@0, u8=1@8}`**；
       param_7 push 8B；WallDrawState=0；param_9 `byte@+8=1`；param_14 push 0x18B 行。
3. **观察到 9 个 +0x1c tick 回写槽**：param_2/3/4/5/7/9/10/14/16（param_4/16 为 EventWriter，
   param_2/3/9 等为 ResMut）。**纠错/补充**：DIGEST「5 ResMut（param_5/7/9/10/14）」之外，
   param_2（WallDrawState）、param_3、param_4、param_16 亦有回写。
