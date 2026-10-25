# systems/wall/draw_wall.rs

> 文件作用：徒手墙绘制系统（最大生命周期系统）：平滑输入空早退；SnappingMode 吸附光标位置；InProgressWall::new 起笔；CurveNursery 累积笔画；InProgressCurve::add_weight_to_the_last_point；Curve::crashy_new_from_points 到 InnerWalls::update_wall_state_curve + update_freehand_ty。4 ResMut（InnerWalls/InProgressWallResource/CurveNursery/InProgressCurve）。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::draw_wall::draw_wall` | 0x140a7cba0 | 45 | draw_wall（4 ResMut；CurveNursery + crashy_new_from_points + update_freehand_ty） | O | wall\decomp\decomp-0x140a7cba0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项

## 实现方式（算法级）（O，decomp-0x140a7cba0，83KB 结构级）

**处理链**：11 参；入口输入可用性 `func_0x000140c16a00`，**空 → 早退**；puffin scope
`country_core::systems::wall::draw_wall`（0x142ad7358）。param_4 = query 束，`param_4[4]` =
InProgressWallResource temp 曲线区（大记录）。ResMut/EventWriter（+0x1c，**5 槽**：param_1/2/7/8/9；
DIGEST 只列 4，param_1 亦回写 —— 补充）。

**InProgressWallResource 字段（O）**：wall id u64@+0x80、f32*@+0x94、u32@+0x98、
**freehand_ty u8@+0x9c**、u8@+0x9d。

1. **吸附解析**：`FUN_140ad3f50` ×3（SnappingMode，返回 `{u8 mode,…}`），`mode==3` 判别。
2. **曲线输入记录**：`FUN_14145c350` ×2（记录鼠标输入进曲线/平滑）；`func_0x000142923fa0(1.0/fVar6)`
   （角度/比例，fVar6=平滑配置 f32，下限 `_UNK_142950d70`）；`_UNK_14295a0dc/14295a0e0` 区间裁剪。
3. **曲线更新**：`FUN_140892720` ×6（InProgressCurve 构造/吸附路径；temp 曲线非空时调用）。
4. **InnerWalls 查表**（param_1 deref，map@+0x20，条目步长 **0x78** = `{key u64, WallState 0x70}`）：
   key = InProgressWall id@+0x80；命中读既有墙高度 f32@`value+0x54`；`freehand_ty@+0x9c==0` 选
   EventWriter param_8/param_9，push **0x18B 行** `{u64@0,u64@8,u32@0x10,u8@0x14=0}`（扩容 FUN_140562db0）；
   随后 **update_freehand_ty FUN_140aa99e0 ×3**。
5. panic：panic_str（draw_wall.rs）多处；capacity FUN_1428d8fe3；曲线 `Result::unwrap` Err
   （0x142c63340，utils curve/mod.rs "Curve…"）。
