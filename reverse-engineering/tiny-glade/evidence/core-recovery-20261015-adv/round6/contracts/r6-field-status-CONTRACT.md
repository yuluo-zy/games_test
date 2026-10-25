# Round6 CONTRACT — 字段级状态汇总（可闭合优先裁定）

> 逆向 agent ① 交付。逐项标注：**可直接落地 / 部分定案 / 待取证**。无证据不编造。

---

## 1. Walls 0x78B — **可直接落地**（r6-walls-0x78b-CONTRACT.md）
记录 = {WallId u64@0, WallState 0x70B@8}；WallState +0x54 height f32 / +0x60 freehand u8 /
+0x64 arch u8 / +0x67 removed_supports u8（O）。曲线区 + JSON 形状待取证（中优先）。

## 2. GameKeyBindings 40×12B — **可直接落地**（r6-gamekeybindings-modkey-CONTRACT.md）
40 槽 × 12B {key u32@0, value u64@4}，default 键码 capstone 全解，现源 BindingsTable 已对齐；
GameProfile 改 40×12B 直读。mod_key 表2 键码枚举待取证（中优先）。

## 3. sheep 布局 — **可直接落地**（r6-sheep-layout-CONTRACT.md）
stride 0x68，+0x30 move_state / +0x31 force_reach_goal / +0x38 timeout_s / +0x58 being_petted(>0.01)。
+0x00..0x2f 等区间待取证；movement 状态机 + X4 Query 待取证（中优先）。

## 4. Decorators 0x28B/0x16B — **部分定案，逐字段待取证**
0x28B 记录 = {状态 16B(ptr@0+值@8), 子计数@0x10} + 0x16B 子记录（+0x52 状态字节）（O，round5
c-apply-section-parsing §2）；JSON serde 架构与逐槽字段待取证。② 可先按 0x28B 组装。

## 5. Terrain strokes 格式 — **待取证**（外部 tg_terrain serde；flate2 解压入口 O）

## 6. GameSaveData 240B / AsyncSaveCmd 变体 / session create 96B+32B+24B — **待取证**
- save-DELIVERY：GameSaveData 字段名纠错已登记（语义字段+自定名标注 U）；240B 精确字段待证。
- AsyncSaveCmd 变体（OpenFile/CaptureSnapshot(9) 等）：字段名待证（save-DELIVERY 已标）。
- session create 0x140b46960 96B 模板+32B 选项+24B 串/路径：字段名 U（session-DELIVERY）。

## 7. ivy count_range RNG 界 / send_to_gpu seed·wall_normal — **待取证**
- RNG 界 = spawn_ivy [rbp+0x190] 的调用方栈参映射（round4 t3-ivy-constants §4 标低优先）。
- send_to_gpu seed/wall_normal：GPU 后置（round4 t3-ivy-constants §5）。

## 8. ui_clutter 0x58B/0x60B — **0x60B parent 部分定案**（round4 t3-ui_clutter）
0x60B = {tag u32@0(<3 门), +0x10 u32, +0x30/+0x38 ClutterMeshTy::from 参}；AutoClutterRecord
0x58B 字段名仍 U（round4 ui_clutter-DELIVERY）。逐字段待取证（中优先）。

## 9. 5 域（Roofs/WallColors/GateColors/Trees/Raster）— **待取证**
round5 c-snapshot-apply-targets §1 节序 O；各域 apply 函数/目标资源映射待取证（中优先）。

## 10. 28-系统逐名 / G1·G3·G4 asset 组 / Schedule.run — **部分定案**（round5 a 契约）
G1(10)/G2(8)/G5(6)/G6(6) 系统面 O；G3(0x1415f4750)/G4(0x1409c8600) 各 1 系统面 O；
28-tuple 槽位映射 + GardenLoop::run 内 Schedule::run 调用点待取证（宿主/D 组用，低优先）。

## 证据索引
- round4/round5 契约（t3-ivy-constants、t3-ui_clutter-parent-map-0x60B、t2-run_game_thread、
  c-apply-section-parsing、c-apply-targets、a-ecs-scheduler-integration）。
- round6：r6-walls-0x78b、r6-gamekeybindings-modkey、r6-sheep-layout。
