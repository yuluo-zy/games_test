# systems/wall/adjust_wall_height/adjust_wall_height_impl.rs

> 文件作用：墙高命令执行：AdjustWallHeightCmd 0x30B{ty u32@+0x10, height f32@+0x14, WallId@+0x18, follow-terrain}；clamp 回退命令高度；follow-terrain 采样 TerrainHeightsData；预置形状到 _try_update_state<update_wall_state_height>（×2）+ 0x28B 事件{WallId,pos+height,u32,t,3}，否则 0x20B RequestWallMaterialAudio。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::adjust_wall_height_impl` | 0x140ac1f20 | 46 | adjust_wall_height_impl（0x30B 命令 + clamp 回退 + 地形采样 + _try_update_state<height>×2 + 0x28B/0x20B 事件） | O | wall\decomp\decomp-0x140ac1f20.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项

## 实现方式（算法级）

### adjust_wall_height_impl 0x140ac1f20（O）
- **输入**：EventReader<AdjustWallHeightCmd>（param_1 双缓冲 Events，事件行 **0x30B**；cursor 更新 `{ptr@+8,len@+0x10,..,+0x28 ptr2,len@+0x30,total@+0x40}`）。行布局：`flags u8@0（bit0=follow-terrain）、pos Vec2@+4（u64）、u32@+0xc（采样 y）、ty u32@+0x10、height f32@+0x14、WallId u64@+0x18、flag u8@+0x20`。
- **分派**：do-while 跳过 `ty==0` 行；`ty!=1` → `panic!("not yet implemented")`（adjust_wall_height_impl.rs:19，UNK_142adf330/142adf348）。
- **主流程（每行）**：hash 查 InnerWalls（ResMut param_2，0x78 步长）→ 当前高 `value+0x54`；调 **clamp_wall_height** 0x140ac7f30 → 返回 {bool, XMM0 f32}：真→钳制高，假→命令高 fVar45；`fVar45==当前高` → continue。changed 后：
  1. follow-terrain（bit0）：`pos.y += (float)FUN_1409e9640(terrain, x, y@+0xc)`（TerrainHeightsData 采样）。
  2. `_try_update_state<update_wall_state_height>` 0x140aaaca0(map, wall_id, &height) → 写 `value+0x54`。
  3. 预置形状分支：`shape tag(value+0x38)<2 且 cmd flag@0x20==0` → **同一墙再调 0x140aaaca0**（机器码证实 0x140ac24b3/0x140ac25a3 两调用点均传 `{rcx=map, rdx=wallid, r8=&[rsp+0x28]=height}`，Ghidra 漏第 3 参）。
  4. 事件输出：预置路径 push **0x28B** 行 `{u64?, WallId, pos u64, u32, f32=clamp01(func_0x140c904a0(0)), byte=3}`（FUN_140562ab0 扩容）；否则 push **0x20B** 行 `{u64?, WallId, pos, u32@+0x18, byte=0@+0x1c, byte=cmd flag@0x20@+0x1d}`（FUN_140562c30 扩容）。容器 cap-first Vec。
- **曲线采样**：`FUN_141467ce0(value)`（Curve2 取段+分数；WallState 曲线区 = {points Vec2@+0..0x18, u f32@+0x18..0x30}）→ 段索引 vs `value+0x10` 点计数越界 → 两 Vec2 点按分数 lerp → 与地形抬升 pos 组合（func_0x000140c9cdc0）。
- **纠错**：`_try_update_state` 第二调用点非独立函数，是同一函数重入；0x28B/0x20B 事件为两个不同事件（非旧述单事件）。
