# groupA RE-ANALYSIS — system_water（水面系统）

dbid: tg_xa1 · 日期 2026-10-16 · 目标: 为 restorer 提供可落码接口契约
decomp 证据: `systems-recovery/groupA/water/decomp/decomp-<va>.txt`

## 1. 覆盖盘点

| VA | PDB 名 | 结论 | 说明 |
|---|---|---|---|
| 0x141ba9130 | draw_water::draw_water | O | 主绘制系统（水/橡皮双模式） |
| 0x141ba9700 | draw_water::closure$0 | O | 越界 signifier 生成闭包 |
| 0x1406501d0 | impl$0::build | O | WaterPlugin::build（系统注册 + 3 资源 init） |
| 0x141baefe0 | update_water_audio_grid | O | 相机周围音频采样格更新 |
| — | record_water_history (line 12) | U(内联) | PDB inlinee，无独立 VA；逻辑并入调用点（draw_water/replay 侧） |

zeno crate 的 7 个 "business" 项（0x14282b030/0x14282ae40/0x1422632f0/0x1422637c0/
0x142264a90/0x142264390/0x142263c90）= **dependency_reuse**（registry/git 包
PounceLight/zeno@0.2.2，b8e8646d），不恢复其实现。`core::str::pattern::TwoWaySearcher`
= std。不计入水面业务。

## 2. 布局

**WaterPlugin::build (0x1406501d0)**：注册 2 个系统（内部标签字符串
'crates/systems/water/src/lib.rs' 行 16/34 → draw_water、update_water_audio_grid；
均为 Update 集）+ `init_resource::<T>` ×3（TypeId 常量 0xcc3c0165e2715d9d /
0xb050d5756cb9fc8f / 0x494426e4a24afc26 —— 类型身份见 §6 U）。

**draw_water 系统**（16 参 bevy 系统；关键参数面）：
- 入口门禁：进程全局 `*0x14362c098 == 0`（全局输入/UI 抢占标志，多个 system 共用；非 UiState）否则 return。
- 模式分发：`*param_1` = u32 模式枚举。`mode-5` ∈ {4, 10} → 业务分支：
  - lVar14==4 ⇒ mode==9 → **橡皮擦**分支
  - lVar14==10 ⇒ mode==15 → **画水**分支
  （其余模式直接 return）
- **橡皮分支**：`(mode[1] & 3) != 0 → return`（按键态检查）；brush = `lerp(0.3f32(0x1429258d8), 2.0f32(0x14292e6f0), t)`（t 来自 BrushSizes 资源 *param_15），再 `x = 4.0*x + (1.0-x)`（0x142934694 首 f32=4.0，0x142925904=1.0）；`assert brush_size.is_finite()`（panic 串 §4）；`UiState::clearing_canvas(&state, "eraser", 6, 0)`（0x14092e390）；`CursorTerrainRaycast::position` → `CameraCursorRay::intersect_with_xz_plane`；format! → SignifierFactory::new → UiSignifierStream::write（**笔刷大小显示 signifier**，scaled by 0.4922(0x142925870)）。
- **画水分支**：brush_size = `*(f32*)(param_15[4]+4)`（BrushSizes 第 4 槽）；同 assert；`clearing_canvas(&state, "water", 5, 4)`；cursor 位置 ×2；`Circle2d::new(center, r = 0.15f32(0x142a16cb8) * brush)`；`GladeBorder::is_pos_inside`（0x140ad2970）→ 圈内：
  - `DrawState::update(param_12, clicked, mode)`（0x140ae4bd0，见 §3）返回/写状态枚举 → `switch(*param_12)` {0: Idle, 1: 记录中(存 circle 数据), 2: **完成 → EventWriter::write(WaterDrawEvent{center, r, 0x100})**, 3: 保持}；
  - 向 stroke Vec（param_11 指向的两组 cap-first Vec<u64>）push {值, 计数}（8B 元素, grow=0x140562870）；
  - 结束时向第二 Vec push；`ScheduledTerrainReadback::on_*(param_9)`（0x140aabdc0 —— 编辑后调度地形回读）；末尾 `EventWriter::write`（0x142260ae0 = 水事件 2，WaterEraseEvent 候选）。
- 字符串常量：'eraser'(0x142eebc08) / 'water'(0x142eebc28) / 断言串(0x142eebba0)。

**update_water_audio_grid (0x141baefe0)**（5 参）：
- puffin 作用域名 = 从全路径串 'system_water::water_audio_grid::update_water_audio_grid::f'(0x142eed9b8) 经 rfind('/'|'\\') 切末段（profile_function 模式）。
- param_1 = `&mut u64` 帧计数（`f = f+1`）；param_2 = ResMut<WaterAudioGrid>：
  - 布局（与 forest 的 0x14067dd80 访问器同构）：`Vec<24B 条目>{ptr@+0x20, end@+0x28, len@+0x30}`，flag@+0x298；flag==0 时用 +0x8/+0x10 的空 Vec。条目 24B = {u64 打包位置, u64 活动位集(64 帧周期), u32 累加器}。
  - 每帧周期位 = `1 << ((11*(f+1)) & 0x3f)`；格偏移 `sVar = ((11*(f+1))>>2 & 0xe) - 7` ∈ {-7..7}×2。
  - MainCamera::eye_position → 每格距离² vs 阈值 0x142eed900；超阈值 → 清位并 `acc -= sVar`；否则 `Raycasting::intersects_any(world, ray, 1.0, layer_mask=0x32a3)`（0x1408cc320）遮挡 → 同清位；无遮挡 → 置位并 `acc += sVar`。
  - 即：**相机 64 帧活动位集 × 遮挡射线**的音频采样格（水声触发依据），格中心 = 打包 u16×2 位置（unpack 0x140c9cdc0）。

## 3. 关键小函数（手解，<40B）

- **0x140c97540 Circle2d::new(rcx=out, xmm1=x, xmm2=y, xmm3=r)**：`movss [rcx],x1; [rcx+4]=y; [rcx+8]=r; dword[rcx+0xc]=0` → Circle2d = {Vec2 center@+0, f32 radius@+8, u32 kind@+0xc=0}（kind 0=圆；供形状判别）。
- **0x140a12650 CursorTerrainRaycast::position(rcx=out, rdx=raycast)**：`test byte[rdx],1; jz → out={0,0}`；Some: out={Vec2@[rdx+4], f32@[rdx+0xc]}。raycast = {u8 有效标志@0, f32 x@+4, y@+8, z@+0xc}。
- **0x140ae4bd0 DrawState::update(rcx=&mut u8 状态, edx=动作, r8=?)**：`eax = clamp(dl-2, 0, 3)`；若 `rdi-1 < 2`（层 1..2）：`state' = (state-1 >= 2) ? r8 : state-1` 写回 [rcx]。→ DrawState = u8；调用点 switch 语义 {0:Idle, 1:开始, 2:完成, 3:?}。
- **0x14092e390 clearing_canvas(&self, name:&str, layer)**：包装器，构造 {0,0,0,0,&buf,kind=3} 调共用实现 0x14092df30 区。
- **0x14092e5f0 UiWidgetContext::clicked(self)**：`[val+0x18]==2 && [val+0x19]&1` 判单击（edge）；**0x14092e680 is_active**：`[val+0x18]!=0 && [val+0x28]!=2`。
- **0x140a14df0 栅格采样（O，机器码级）**：`(grid, x, y, u32, scale=130.0f32(0x43020000))` → 坐标/130 → clamp[0,1] → ×512 → 双线性（表 0x1429258e0..8ec）→ 返回 `grid[+0x201]` 处字节（512 宽栅格）。这是 **WorldRaster<512> 采样**（water/forest/tree 共用），开关 0x14362c308 分软件/硬件采样路径。

## 4. panic / 串

| 地址 | 内容 |
|---|---|
| 0x142eebba0 | `assertion failed: brush_size.is_finite()`（fat 后随 'crates/systems/water/src/draw_water.rs' 0x142eebc10/0x142eebbf0 两个位置） |
| 0x142eebc08 | 'eraser'（笔刷名/作用域名） |
| 0x142eebc28 | 'water'（'watereraser_water_' 打包区） |
| 0x142eed9b8 | 'system_water::water_audio_grid::update_water_audio_grid::f'（profile 名） |
| 0x142eedac0 | 'crates/systems/water/src/water_audio_grid.rs' |

## 5. 消费者线索

- EventWriter 双写：0x142260a60（WaterDrawEvent，case2）+ 0x142260ae0（同文件末尾；`world_raster::draw_raster<WaterRasterStagingSsbo>` 同名地址族 → 水事件由 draw_raster/replay_raster 消费，见 terrain-water 章节）。
- ScheduledTerrainReadback::on_*(0x140aabdc0) —— 水编辑 → 地形回读调度。
- GladeBorder::is_pos_inside(0x140ad2970) —— 与 glade_border 域交叉。
- 进程全局 0x14362c098（UI 抢占）与 0x14362c308（栅格采样路径开关）为跨域共享。

## 6. U 项（不臆造）

- build 中 3 个 init_resource 的 TypeId→类型名映射（墙/地形/水栅格候选；与 terrain-water 域 WATER 资源章节核对）。
- DrawState 枚举变体名（0..3）；WaterEraseEvent 具体字段（case2 事件含 {center, r, 0x100 标志}，另一事件 {cursor}）。
- record_water_history 内联体（历史编辑记录，猜测与 set_* 同构 EditType，U）。
