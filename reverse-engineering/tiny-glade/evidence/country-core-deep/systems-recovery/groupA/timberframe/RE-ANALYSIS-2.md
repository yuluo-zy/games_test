# groupA RE-ANALYSIS-2 — timberframe circle 输出路径（task ③）

dbid: tg_xa1 · 2026-10-16 · 证据: `timberframe/decomp/decomp-0x1421b67a0.txt`（28KB 全读）

## 结论（O）

**construct_timberframe_circle（0x1421b67a0）的梁实例输出落点 = `country_core::resources::roofs::roof_buffers::WoodenPlankSsbo`**（屋顶域 plank GPU 缓冲）。

证据链：
1. 函数头 `FUN_1421b5d00(*param_3, &UNK_142ff8bb8)` —— param_3 = Res（SSBO 库）；UNK_142ff8bb8 = 16B TypeId 键 → 返回 Arc<String>；校验 `puVar8[3] == 0x3d`(61) + `memcmp(&UNK_142ff8cf0, puVar8[2], 0x3d)` —— 类型名字串 0x142ff8cf0 = **'country_core::resources::roofs::roof_buffers::WoodenPlankSsbo'**（rne-read 全串确认）。此查找-比对模式与 color 域 SSBO 链（0x1425f18b0 + 类型名）同构 —— roof_buffers 域同一注册面。
2. 逐梁写 `FUN_1421b6000(&puStack_338, &uStack_188)` —— 0x1421b6000 = **单梁实例写入器**（buffer 头 + 0x50B 实例结构）；bVar5 成败标志 `uVar13 |= bVar5`。
3. 末尾 `*(longlong *)(lStack_290 + 8) = lVar14` —— lVar14 = 本帧梁数；lStack_290 = *param_5 = ResMut<WoodenPlankSsbo CPU 侧>（+8 = 数量字段）+ tick 回写（`*puStack_288 = uStack_22c`）。
4. 实例不足日志：`if (uVar13 & 1 && *0x14362c3a8) log` —— 消息 **'Timberframe constructor ran out of wooden plank pieces'**（0x142ff8f40；fat 位置 0x142ff8f50 = crates/systems/timberframe/src/timberframe_circle.rs；格式串 0x142ff8f68 31B）。

## 梁算法契约（O）

- param_1 = Res（墙形状空间哈希 0x28B 条目：{u64 id@+8, …}；跳过 bit0 置位条目）；param_4 = Res（屋顶：`*(u32*)(+8)` = 尺寸码，`*(f32*)(+0xc)` = 屋顶高 fStack_f4）；param_2 = ResMut（0x18 处互斥锁标志：`LOCK; if [r15+0x18]==0 → 置 1` 防重入，否则 goto panic —— 与 color 域 0x1425f18b0 的 Arc 模式同族）。
- **梁数 = max(6, ⌊perimeter(0x140c97970) / 0.63⌋)**（0.63 = 0x1429258d0 = 梁间距）；下限 6（`count < 7 → 6`）。
- 每梁：`u = i / count`（∈[0,1] 断言，越界 → 格式化 panic 'assertion failed: !u.is_nan()' 0x1431025d8/648 = utils circle.rs，两处循环同）；`Circle2d::get_pos_at_u`（0x140c975a0）→ 圆上位置；`pack_2x16f_uint`（0x140c9cde0）+ `unpack`（0x140c9cdc0）；**旋转 = `rotation_from_normal`（0x140c90a40）+ 四元数/矩阵乘**（0x14089d560 单态 = 实例组装器）；厚度系数 **0.2**（0x142a8c6f0 = {0.2, 0.2} 及 `1.0*0.2` 常量）；高度混合 `roof_height_ws_from_wall_shape`（0x1408e3c70，第二循环分支 `pbStack_280[0x1c]==0` 时）。
- 第二遍循环（code_0x1421b6d57）：遍历 f32 高度表（lStack_2e0，0x140562630 grow）逐梁二写；其后 **`FUN_140b34f80(&lStack_2e0)`** —— 接收 Vec<f32 高度>，机器码上下文 = **split_height_into_skirts 行为**（sysres 名 HighlightColor::to_rgb 与此上下文不符；归属冲突登记，行为证据偏向 split_height_into_skirts）。
- 实例尾常量 {u64 0x142b14ea0=0, 0x142b14ea8=0x200000000, u32 1, u32 1} —— 实例元数据（kind/范围标记，I）。
- 返回 char：成功/失败（`LOCK; [lVar1+0x18]==1 → 清零返回 1` 模式）。

## 交叉登记

- WoodenPlankSsbo 属 roofs/roof_buffers 域（render 侧，GPU 后置）；rectangle 版（0x141a41ba0）同目标（0x142ff8cf0 同串，见 RE-ANALYSIS-1）。
- `0x1408e3c70 roof_height_ws_from_wall_shape` / `0x1408e2700 RoofRidgeDims::calc` / `0x1408e3b60 ridge_length_01_from_rectangle_ratio` = roofs 域公式（章节化）。
- 全局 0x14362c3a8 = 调试日志开关（"ran out of planks" 门禁）。

## U 项

- 0x1421b5d00 / 0x1421b6000 / 0x14089d560 精确函数体（timberframe crate 内部；契约 = 名查找/实例写/组装器）。
- 0x140b34f80 最终归属（集成 owner 裁决）。
