# systems/wall/eraser/animate_wall_bulldoze.rs

> 文件作用：墙推土动画：AnimateWallBulldozeBuffer 152B 命令行{Source112, WallId+0x70, low/high+0x78/+0x7c, anchor+0x80, phase+0x8c, finished+0x90}；每帧 phase=clamp01(phase+dt/0.2)（先推进后应用 CubicInOut）；增长写 Source 切片+enclosure tag2+style；完成移除 authority（128 场景/768 全 RET 先验验证）。

函数总计 7：业务 1 / 内联无独立符号 4 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::eraser::animate_wall_bulldoze::animate_wall_bulldoze` | 0x140ac27e0 | 74 | animate_wall_bulldoze（phase 推进 CubicInOut + 曲线切片 + authority 移除） | O | wall\decomp\decomp-0x140ac27e0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeCmd::new`（行 30）— 构造
- `country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer::new`（行 59）— 构造
- `country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer::is_empty`（行 63）— 谓词/校验
- `country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer::contains`（行 67）— 谓词/校验

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 2 项

## 实现方式（算法级）

### AnimateWallBulldozeBuffer 布局（O）
- **AnimateWallBulldozeCmd = 0x98B（152B）**：`+0x54/+0x5c Source（2×u64，曲线数据）、+0x64 u32、+0x68 u8（==1 判定）、+0x69 u8、+0x70 WallId、+0x78 u_lo f32、+0x7c u_hi f32、+0x80 anchor、+0x88 u32、+0x8c phase f32、+0x90 finished u8`。
- **AnimateWallBulldozeBuffer**：`Vec<Cmd>@+0（cap-first）` + 内嵌 hashbrown 集合（供 publish 判定动画中墙）。
- **new 0x140a82b20（O，机器码）**：`mov [rcx],0; mov [rcx+8],8; mov [rcx+0x10],0; ret` = `{cap=0, ptr=8, len=0}` 空 Vec。
- **0x140b772b0（O，MCP 新反编译）——纠错：非 contains**：实为 hashbrown **remove-and-return-entry**（0x2a8 步长 map，即 PublicWalls 级条目）：命中 → 按集群探针位置把 ctrl 字节写 0x80/0xff（`growth_left(+0x10)+=1, items(+0x18)-=1`），`func_0x000142923c00(out, entry, 0x2a8)` 拷贝出条目；未命中 → `out+8 = 0x8000000000000000`（None）。publish 用它把正在推土的墙条目取走而非"跳过"。
- **FUN_1412f6a20（O，MCP 新反编译）= retain_finished**：按 0x98 步长扫描 buffer，`元素+0x90==1`（finished）→ free 其 Vec3/f32 数组并移除，否则 memmove 0x98 压缩；`len@+0x10 -= 移除数`。

### animate_wall_bulldoze 0x140ac27e0（O）
- ResMut<AnimateWallBulldozeBuffer>（+0x1c tick）、Res<InnerWalls>（0x78 查墙）、ResMut 事件（0x20B 行）、param_4+0x58（Time/帧 dt）。
- 对每个 Cmd：InnerWalls 查 `cmd+0x70`；**phase 推进**：`cmd+0x8c = clamp01(cmd+0x8c + (time/duration))`，duration=**0.2**（_UNK_1429cd068）。`ease = func_0x000140c961e0()`（缓动，I：CubicInOut）。
- `phase==1.0` → 完成：`FUN_14085aa60(inner, wall_id)`（InnerWalls 移除 authority）+ free Source 数组 + `cmd+0x90=1`。
- 否则 **曲线切片 u**：`u_lo = low×ease`（`low×t+(1-t)×0`），`u_hi = lerp(1.0, high, ease)`；u 越界/NaN → CurveU panic（`!u.is_nan()` line 0x1d / `Tried to init CurveU...`）；`FUN_141466bb0(out, cmd, u_lo, u_hi)`（曲线切片）→ `_try_update_state<update_wall_state> 0x140aa9bc0(map, wall_id, &sliced_state)` 写回 WallState；写 **0x20B 事件** `{u64, WallId, anchor, u32, 0 u16}`（tag 含 `cmd+0x68==1`、`cmd+0x69`、enclosure tag=2）。
- 帧末 `FUN_1412f6a20(buffer)` 清理 finished。
