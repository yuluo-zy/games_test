# REVIEW-DEEP-AUDIO — systems/audio 深收口复验（reviewer-eco，2026-10-17）

复核对象：impl-audio-deep（REBUILD-DEEP.md）。方法：与 fresh 反编译/反汇编逐项判等。实测：
cargo check 0 error（10 既有 warning）；cargo test --lib --no-run 过；音频 53 测试过（audio 过滤
97 全过）；最终复检被并行 rhapsody rg/attachments.rs 在途改动临时阻塞（非音频域）。

## 逐项核对（协调者 5 点）

### 1. sheep 0x1b..0x51 大分派 — ✅ 闭合（判等，无需改码）

fresh xref 证实：跳表 0x142ac2cc8 仅 sheep_audio_system 引用（4 处全在 0x1409d5ea0..
0x1409d6249）；0x142ac2ce8 仅 signifier_audio_system（0x1409d7b6a/7b71）；两表原始字节同构
（相对偏移全等，基址差 0x20 = 同 generic 双实例）；8..0x51 目标落在 signifier 非指令边界
（如 0x1409d6fb8 在 cmovbe 字节内），sheep kind≥8 无 producer 发出。函数边界 size=405 与
disasm 一致。**sheep 自身 kind 0..7（Aww/Think/Sob/Angry/Baah-loop/Idea/Eyes/Confused）
判等，无需改码** ✓。

### 2. decorator 14 one-shot 名 — ✅ 部分重建

`decorator_one_shot(kind)` 表 verified：kind1 WindowPlace(0x16)/2 WindowCombine(0x18)/
3 WindowPlacePreview(0x1d)/4 WindowDelete(0x17)/5 WindowRemove(0x17)/8 WindowChangeSeed(0x1b)/
9 WindowHover(0x15)/10 WindowSplit(0x16)/0x10 Lantern/Delete(0x19)/0x14 ChimneyBaseDisappear(0x1f)/
0x17 ChimneyDelete(0x18)/0x19 Flags/ChangeSeed(0x1b)/0x1e Flags/Delete(0x17)/0x1f
Cosmetics/Clutter_Clone(0x22)——disasm 串表地址+长度逐条 ✓。`kind_to_loop` 只收 loop kinds
（6/7/0xc/0xd/0x12/0x16/0x18/0x1c/0x1d/0x22/0x24），原把 kind1/2 当 loop 的分歧已改正 ✓。
loop 槽↔名待对账登记（需 DecoratorAudioState 完整布局）。

### 3. timberframe 5 one-shot 名 — ✅ 闭合

kind2 TimberframeStartEdit(0x1f)/4 TimberframeAppear(0x1c)/5 TimberframeDisappear(0x1f)/
6 TimberFrameSnap(0x1a，Frame 大写 F)/7 Building/BuildingCreate(0x22)——disasm 串表地址+长度
逐条 ✓；loop 0/1/3（Length/Width/Move）不变。占位数组已删。

### 4. ⚠️ wall_path / orchestrator 现模型被 fresh 推翻 — ✅ 诚实登记（不冒充闭合）

- `wall_audio_path`（0x140a89bb0）：native 非 loop 驱动，实为消费 3 个 wall-path 事件流
  （stride 0x10）push `(DecoratorId, Vec<PreferredNodeAlignment>)`（0x20B/条）；现
  `WallPathDraw`+request_play 模型**错误**，需 wall-path producer 事件类型 +
  system_decorator::PreferredNodeAlignment（跨 crate）重建——登记。
- `wall_audio_orchestrator`（0x141234bf0）：native 为 `hash_one<GlobalId>` 查墙表 →
  MaterialStats::which_material_changed_the_most()/max_pillar_movement() →
  `PlayWallAudio` 发射 + indexmap::Bucket<(DecoratorId,DecoratorId),AlignKind>；现
  set_parameter on loop[0] 模型**错误**，需墙状态/MaterialStats/AlignKind（他模块）重建——登记。
- 判定：两函数**不判等、需他模块类型**，诚实登记待接线，不冒充闭合 ✓。

### 5. water_ambience 公式 + AudioCoreInner 断言 — ✅ 判等

- `water_ambience_volume = sqrt(pop / (1 + dist² / (pop·0.015625·42.5)))`（常量 0.015625@
  0x142ac4d00、42.5@0x142ac4d20、1.0@0x142925904）——源码逐公式对上；`WaterGridCell{position,
  bits(64-bit exists popcount), state@+0x18}`，state==1 才 set_volume ✓；cell 位置
  sample-offset/y 语义待对账登记。
- `AudioCoreInner::update` 三断言判等：fwd_len-1<0.001、up_len-1<0.001、dot.abs()<0.001 →
  panic；set_listener_attributes 失败 unwrap；update 失败 log 后继续 ✓。

## 剩余（诚实登记）

decorator loop 槽↔名、signifier 21 参数全表、wall_path/orchestrator 他模块类型重建、
roof FMOD 参数赋值细节、water_ambience cell y 语义、firstperson SSBO 位栅格、
PlayDecoratorSoundCmd 补 selector/value 字段。

## 结论与 DISPATCH 建议

- **audio 深收口（实现层面）= 闭合**：sheep 0x1b..0x51 伪影定案（判等无改码）、decorator 14
  one-shot + loop 分类修正、timberframe 5 名闭合、water_ambience 公式 + update 断言判等。
- **登记项（不阻塞，需他模块/续批）**：wall_path/orchestrator 需墙域事件类型 + MaterialStats/
  AlignKind 重建（最高优先）、signifier 21 参、decorator loop 槽↔名、firstperson SSBO。
- 建议：REBUILD-DEEP=review_passed（限定）；wall_path/orchestrator 交 wall 域/集成 owner 续派。
