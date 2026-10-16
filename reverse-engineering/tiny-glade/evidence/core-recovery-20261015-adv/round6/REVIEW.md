# 第六轮 ③ REVIEW（2026-10-16 基线 + 复审台账）

审查者：③ review agent。本轮范围：**② 字段级实现**——字段名真对机器码（非编造）、
apply 逐字段真应用、sheep 布局/spawn·movement 真实性、常量核对。产出：
① 深挖方向 + ② 修正项 + round6/REVIEW.md。**wall/signifier 为用户重构域，不审代码、只登记**。
整 crate 零错误基线，② 引入回归立即标出。

## 1. 审查准则（协调器指令）

1. **字段名真对机器码**：② 新增字段/常量必须可追溯到原地址/decomp/rne 字节，不允许"语义合理但无机器码依据"的编造字段名。
2. **apply 逐字段真应用**：快照/存档应用闭包逐字段写入目标资源，不是 log/登记冒充。
3. **sheep 布局/spawn·movement 真实性**：+0x00..0x2f / +0x3c..0x57 / +0x5c..0x67 区间、spawn（startup_sheep=资产载入非实体 spawn）与 movement 状态机——有机器码证据才落，否则保持待取证。
4. **常量核对**：rne-read/decomp 复核 ② 引入的所有新常量。
5. wall/signifier：只登记（用户重构域，不审代码）。

## 2. 待复审基线（round5 遗留字段级阻塞，② 本轮实现对象）

| 项 | 当前状态 | 审查点 |
|---|---|---|
| Walls 0x78B 记录 14 qword → WallState 构造 | `load_state` 只解码 record[0]=WallId，其余待① | 字段名对 0x14085ad80/0x1402ca2f0 机器码；JSON 形状 `Vec<[u64;15]>` 确认 |
| Decorators JSON↔0x28B serde 架构 | `parse_decorator_section` 恒 None | 0x28B 记录 {状态 16B, 子计数@0x10} + 0x16B 子记录逐字段 |
| Terrain strokes 字节布局 | 解压字节未应用至 TerrainEditingData | strokes/next_stroke_id/next_stroke_index 布局 |
| sheep 完整布局 | +0x30 move_state / +0x58 being_petted_timer 已落；+0x00..0x2f/+0x3c..0x57/+0x5c..0x67 待取证 | 不凭名猜字段 |
| sheep spawn/movement | startup_sheep=资产载入（非实体 spawn）；movement 状态机待取证 | 有证据才落 |
| 快照节数据形态 | 已定案（serde_json JSON，0x78B/0x28B 为内存形式） | ② 按定案接线 |

## 3. 给① 深挖方向（字段级，round5 阻塞延续）

1. **Walls 0x78B 记录字段名**：0x14085ad80 load_state + 0x1402ca2f0（HashMap::insert<WallId,()>）的 15 qword 逐字段（+0x34..+0x5c 槽语义）；确认快照 Walls 节 JSON 形状是否 `Vec<[u64;15]>`。
2. **Decorators 0x28B/0x16B 逐字段**：0x140a20d70 restore_snapshot + FUN_140a1cf30 恢复的 0x28B {状态 16B, 子计数@0x10} + 0x16B 子记录字段；JSON 节 serde 架构。
3. **Terrain strokes 布局**：tg_terrain TerrainEditingData{strokes,next_stroke_id,next_stroke_index} 的 strokes 字节序列化格式（flate2 解压后）。
4. **sheep 完整布局 + spawn/movement**：+0x00..0x2f/+0x3c..0x57/+0x5c..0x67 字段（可能动画态/SSBO 后置）；startup_sheep 资产载入 vs 实体 spawn；movement 状态机（旧草稿谓词作废，需重新取证）。

## 4. 逐项复审（2026-10-16，② 第一批 r6-field-closure-1）

**通过（无水分；1 项布局注释不符需修 + 1 项重要常量纠偏确认）**。

### 1. Walls 0x78B 记录（✓ 字段名可溯源）
- `InnerWallStateRecord([u8;0x78])` = {WallId u64@0, WallState 0x70B@8}，stride 0x78（① r6-walls 契约 §1，map 条目权威）✓。
- 访问器全对 ① 契约：`wall_id`（record[0..8]）、`height`（WallState+0x54 f32，写闭包 0x14085ad60→0x140aaaca0）、`freehand_type`（+0x60 u8，0x140aa99e0）、`arch_type`（+0x64 u8，0x14085ad40→0x140aa9eb0）、`removed_supports`（+0x67 u8，0x140aa9800）✓。
- `load_state` 解码 WallId + 4 O 字段 + log；**曲线区 WallState+0x00..0x53 → WallState 构造待①**（不伪造）✓。

### 2. GameKeyBindings 40×12B（⚠️ 布局注释与结构不符）
- KeyBinding {key u32@0, value u64@4}，① 契约槽 stride 0xc=12B；40 槽键码与 BindingsTable（translate_input.rs KEYS）同源 0x14089b110 逐槽一致 ✓；serde 经 Vec 中转（N>32 限制）✓。
- **⚠️ 问题**：`KeyBinding` **无 repr 属性** → Rust 默认布局 u64 对齐到 8 → **实际 16B**，而注释声称"12B 键位槽/槽 stride 0xc"。原生 12B 槽 value@4 是**压缩布局**（misaligned u64）。**② 须加 `#[repr(packed)]`（或 `#[repr(C, packed)]`）使结构真为 12B**，否则注释冒充布局。serde 路径（GameProfile ron）不受影响，但若未来字节兼容则错。

### 3. sheep 布局（✓ + 重要常量纠偏确认）
- being_petted 阈值 **0x292f(1.48e-41) → 0.01**（常量 0x142960674=0x3c23d70a）：③ 独立解码 `0x14089d4c0` 代码字节 `0f2e05 a8310c02`（ucomiss [rip+0x020c31a8]）→ 目标 **0x142960674**（python 验算 0x14089d4cc+0x020c31a8）——**r6 纠偏正确**。round-3/4 sheep DELIVERY 声称 0x142930f74=0x292f 系**地址算术错**（其公式 0x14089d4c5+7+0x20c31a8 实得 0x142960674，文档却写 0x142930f74）。**这是 round-3 复核通过项的纠偏**——① r6 再解 + ② 应用正确。
- +0x30 move_state 判别/force_reach_goal@+0x31/timeout_s@+0x38、+0x58 being_petted 均对 ① r6-sheep 契约 ✓；+0x00..0x2f/+0x3c..0x57/+0x5c..0x67 保持 U 不填 ✓；movement 状态机 + X4 羊 Query 待①（诚实）。

### 4. audio G6 补全（✓）
- roof_audio（init+system）、wall_audio_ui、wall_audio_orchestrator、wall_audio_path、ivy_audio_init_system 全部加入 add_audio_systems，③ 逐函数核验存在 ✓。

### 回归
- 整 crate 零错误（② 声明）；新增系统/访问器逐函数核验存在，无 ② 引入回归迹象。

### 给① 追加
- Walls 曲线区 +0x00..0x53（WallState 构造）+ 快照 Walls 节 JSON 形状；GameKeyBindings value 动作 id；sheep movement 状态机 + X4 羊 Query——延续 round5/6 待证。
