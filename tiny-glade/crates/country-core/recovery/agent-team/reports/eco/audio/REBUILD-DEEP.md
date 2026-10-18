# REBUILD-DEEP.md — systems/audio 残留待办收口（impl-audio-deep，2026-10-17）

> 协议：`planning/2026-10-16-logic-verify-protocol.md` + 2026-10-17 分类补充。
> 依据：`reports/eco/audio/{LOGIC,REBUILD,REVIEW-REBUILD-AUDIO}.md`。
> 方法：每项 fresh ghidra 反编译 / fresh radare2（RNE 机器码）→ 分类 → 按控制流实现 / 判等。
> 证据落盘：`reports/eco/audio/deep/{disasm,jobs,results}/*`。只以 fresh 反编译为准，不改他模块。
> 代码改动（4 文件）：`timberframe_audio.rs`、`decorator_audio.rs`、`water_ambience_audio.rs`。
> 编译：`cargo check` / `cargo test --lib --no-run` 于改动后即时通过；最终复检被并行 agent 的
> rhapsody `rg/attachments.rs:490`（`self.framebuffers` 字段）在途改动临时阻塞（非音频域）。

## 批 1：sheep 0x1b..0x51 大分派 —— 闭合（判等，无需改码）

**结论：native `sheep_audio_system` 自身只处理 kind 0..7；"0x1b..0x51 大分派"是
Ghidra 函数合并伪影，属于 `signifier_audio_system`。现有 Rust（kind 0..7）判等。**

fresh 证据（`deep/jobs/b1-sheep-*.json` + `disasm/b1-sheep-a.txt`）：

1. **函数边界**：`sheep_audio_system` @0x1409d5ea0，body 到 0x1409d6201 `ret`（含 0x1409d6202+
   unwrap panic 路径），其后 0x1409d6250/0x1409d6290/0x1409d62e0 是三个独立函数，
   0x1409d6320 = `signifier_audio_init_system`，0x1409d6b90 = `signifier_audio_system`。
   `get_functions` size=405（0x405 字节）与 disasm 一致。
2. **分派**：`movzx ecx, byte [rbx+0x18]`（kind@+0x18）、`lea rdx,[0x142ac2cc8]` +
   `movsxd rcx,dword[rdx+rcx*4]`；kind 0..7 → handler 在 sheep body 内
   （0x1409d5ff0/6014/6020/61b0/6002/6032/6044/6060）。pitch@+0x14 经
   `0x140a57670`（set_parameter "Pitch" @0x142ac2460）。帧尾 0x1409d61bb 驱动 loop4。
3. **跳表归属（xref）**：`get_xrefs_to 0x142ac2cc8` → 仅 sheep_audio_system（4 处，
   全在 0x1409d5ea0..0x1409d6249）；`get_xrefs_to 0x142ac2ce8` → 仅 signifier_audio_system
   （0x1409d7b6a/7b71，在 0x1409d6b90+ 内）。
4. **两表同构**：`read_memory 0x142ac2cc8` / `0x142ac2ce8` 原始字节相同（相对偏移全等），
   基址差 0x20 —— 同一分派的两个 generic 实例（sheep/signifier 均泛型于 `S: Studio`）。
   sheep 表 0..7 指向 sheep body；8..0x51 指向 signifier 区（0x1409d6fb8..0x1409dae87），
   其中 8..0x51 的目标在 signifier 代码里多为**非指令边界**（如 0x1409d6fb8 落在
   `cmovbe rsi,rbx` 字节内），sheep 不可达（kind≥8 无 producer 发出）。

**判定**：sheep 事件 kind 域 0..7（Aww/Think/Sob/Angry/Baah-loop/Idea/Eyes/Confused）
由 critters/sheep 行为域产生；当前 Rust 实现逐条判等，**无需改码**。REBUILD.md 残留 1 关闭。

## 批 2：decorator 跳表 —— 部分重建（one-shot 名精确化，loop 槽位待对账）

fresh 证据：`disasm/b2-dec.txt`、`b2-dec2.txt`（0x14145e9c0..0x14145f800 逐 case）+ 复用
`logic/decomp/14145dc20.txt`、`14145c6b0.txt`。

**事件布局**（0x28 stride）：kind@+8、selector@+9、value f32@+0xc、position@+0x18、
value2@+0x1c。当前 `PlayDecoratorSoundCmd{kind,position}` 缺 selector/value 字段（producer
待填，登记）。

**one-shot 名（disasm 串表逐条验证，地址+长度）**：
| kind | 事件（0x142c62…） | 长度 |
|---|---|---|
| 1 | WindowPlace (808) | 0x16 |
| 2 | WindowCombine (852) | 0x18 |
| 3 | WindowPlacePreview (8f8) | 0x1d |
| 4 | WindowDelete (915) | 0x17 |
| 5 | WindowRemove (92c) | 0x17 |
| 8 | WindowChangeSeed (95a) | 0x1b |
| 9 | WindowHover (975) | 0x15 |
| 10 | WindowSplit (98b) | 0x16 |
| 0x10 | Lantern/Delete (a38) | 0x19 |
| 0x14 | ChimneyBaseDisappear (a9f) | 0x1f |
| 0x17 | ChimneyDelete (ada) | 0x18 |
| 0x19 | Flags/ChangeSeed (af2) | 0x1b |
| 0x1e | Flags/Delete (b78) | 0x17 |
| 0x1f | Cosmetics/Clutter_Clone (b8f) | 0x22 |

**loop kinds**：6/7/0xc/0xd/0x12/0x16/0x18/0x1c/0x1d/0x22/0x24（native 状态槽位
+0x10/+0x38/+0x70/+0xb8/+0xf0/+0x128/+0x160/+0x198/+0x1d0/+0x208/+0x240/+0x278/+0x2b0/
+0x2d8/+0x310/+0x358 = 16 槽；槽↔事件名对应需完整 DecoratorAudioState 布局，**待对账**）。
kind 0xf = 距离收集（`(dist,pos)` 稳定排序 → 帧尾 3 最近驱动）；kind 0 = no-op。
kind 0x20 = one-shot + SettingsMenuState 记录（名字待取证）。

**代码改动**：`decorator_audio.rs` —— 增 `decorator_one_shot(kind)`（上表 verified），
`kind_to_loop` 只收 loop kinds（近似映射，文档标注待对账）；系统先 one-shot、再 loop。
原 `kind_to_loop` 把 kind 1/2/… 当 loop（0..1→0、2..5→2、9→10、10→13）与 fresh decomp
**分歧**（kind 1/2 实为 WindowPlace/WindowCombine one-shot），已改正。

## 批 3：timberframe one-shot 名 —— 闭合（精确名实现）

fresh 证据：`disasm/b3-tf.txt`（0x140a67f60..0x140a68090）+ `logic/decomp/140a67b90.txt`。

kind 2/4/5/6/7 one-shot 名（disasm 串表 0x142ad4fe7..0x142ad505b，地址+长度逐条验证）：
- kind 2 → `event:/SFX/TimberframeStartEdit`（0x142ad5006，len 0x1f）
- kind 4 → `event:/SFX/TimberframeAppear`（0x142ad5025，len 0x1c）
- kind 5 → `event:/SFX/TimberframeDisappear`（0x142ad4fe7，len 0x1f）
- kind 6 → `event:/SFX/TimberFrameSnap`（0x142ad5041，len 0x1a；注意 Frame 大写 F）
- kind 7 → `event:/SFX/Building/BuildingCreate`（0x142ad505b，len 0x22）

loop kinds：0→Length、1→Width、3→Move（各带 inverse_lerp 参数）与当前实现一致。
REBUILD.md 残留 "Timberframe* 占位名与长度不匹配" **关闭**。

**代码改动**：`timberframe_audio.rs` —— 删占位 `TIMBERFRAME_ONE_SHOTS` 数组，改
`timberframe_one_shot(kind)` 精确表；state 去 `one_shot_events` 字段。

## 批 3b：signifier 21 参 / wall_path DecoratorId / orchestrator wall 域 —— 登记（需他模块类型）

- **signifier**（0x1409d6b90）：fresh disasm（`b1-sig.txt`）证实其 body 就是 sheep 合并区
  （0x1409d6fb8..0x1409dae87）—— 0x1b..0x51 表实为 signifier 的 tag 分派（DAT_142ac2ce8，
  0x52 项）。系统消费 7 个事件流（OnWallChanged/ModeJustChanged/LoadingGameComplete/
  InitAdjWallHeightMode/FinishAdjWallHeightMode/PlayUiSound/PlayPhotomodeSound）+ UiSignifierStream，
  21 参数全表仍**待对账**（当前 Rust tag 0/1→FirstPersonOff|On、其余 "Tool" 为部分重建，维持）。
- **wall_audio_path**（0x140a89bb0）：fresh decomp **推翻**现 Rust 模型 —— native 不是
  loop 驱动，而是消费**三个** wall-path 事件流（stride 0x10）并 `push` `(DecoratorId,
  Vec<PreferredNodeAlignment>)`（0x20B/条，`grow_one<tuple_<DecoratorId,Vec<PreferredNodeAlignment>>>`）
  到一个 Vec。现 `WallPathDraw`+request_play 模型**错误**，需 wall-path producer 事件类型 +
  `system_decorator::…::PreferredNodeAlignment`（跨 crate）才能重建，登记。
- **wall_audio_orchestrator**（0x141234bf0）：fresh decomp **推翻**现 Rust 模型 —— native
  是复杂材质变化协调：`hash_one<GlobalId>` 查墙表 → `MaterialStats::which_material_changed_the_most()/
  max_pillar_movement()` 判定 → `EventWriter::write<PlayWallAudio>` 发射 → `indexmap::Bucket<
  (DecoratorId,DecoratorId),AlignKind>` 写入 stairs 组装。现 `set_parameter` on loop[0] 模型**错误**，
  需墙状态/MaterialStats/AlignKind（他模块）重建，登记。

## 批 4：roof 参数名 / water_ambience 衰减（闭合部分）/ firstperson SSBO —— 登记

- **roof**（0x140aa3860）：REBUILD.md 已按 fresh decomp 重建（acc+0.2 门、kind 跳表、
  帧尾 3 loop）。`ROOF_PARAMS` 名 rne-read 已验证；FMOD 参数赋值细节**待对账**维持。
- **water_ambience**（0x1409ecea0）：**衰减公式闭合**（fresh disasm `b5-wa.txt`/`b5-wa2.txt`/
  `b5-wa3.txt`）：
  - 常量：`1.0`@0x142925904、`0.015625`@0x142ac4d00、`42.5`@0x142ac4d20。
  - cell 布局（0x18B）：world xz@+0、64-bit exists bits@+8（**popcount** 作音量权重）、
    2×i16 栅格样本@+0x10、state 字节@+0x18（==1 才 set_volume）。
  - 公式：`volume = sqrt( pop / (1 + dist² / (pop·0.015625·42.5)) )`，dist=cell 与 eye 距离。
  现 Rust `set_volume(1.0)` 占位 → 已实现公式 + cell 布局字段（`WaterGridCell{position,bits,state}`）。
  cell 位置 sample-offset（0x142ac4d10）与 y 语义**待对账**。
- **firstperson_steps**（0x140a889b0）：SSBO 位栅格细化需 `SsboType` NAME（0x3f/0x45 名断言）
  与 ActionSetStates/MainCamera 主机接线 —— 现用 RasterBufferRef 承载，**待接线**登记（跨 render 域）。

## 批 5：AudioCoreInner update 断言 / trait 面 —— 判等 + 登记

- `AudioCoreInner::update`（0x140a53a00）fresh decomp 三断言：
  `|fwd|-1.0<0.001`、`|up|-1.0<0.001`、`|fwd·up|<0.001` → panic；`set_listener_attributes`
  失败 unwrap；`Studio::update` 失败 log "FMOD update error: …"（0x142ad29c0）后继续。
  **Rust 已实现**（`audio_resources.rs:439-458`：assert fwd_len-1<0.001 / up_len-1<0.001 /
  dot.abs()<0.001；set_listener_attributes 失败 panic；update 失败 log 后继续）—— 判等。
- **trait 面**（Studio：`set_listener_attributes`/`get_vca`/`type Vca:VcaHandle`/
  `load_sample_data()->Result`；EventInstance：`release()`）在 crate 内定义完整（FakeStudio/
  FakeVca 测试面全实现）。**外部接线登记**：宿主 libfmod 绑定（库外）需实现新方法；
  本机 `systems/critters/src/audio.rs` 经 `CritterAudioCore` 回调缝消费，不受影响。

## 编译与测试

- `cargo check`（改动后即时）：**0 error**（10 warnings，全为既有 save/snapshot
  non_snake_case 等）；`cargo test --lib --no-run` **通过**；音频域 53 测试全过
  （含新增 `timberframe_event_names_are_original`、`decorator_one_shots_match_…`、
  `water_ambience_volume_matches_the_native_formula`）。
- 最终复检被并行 agent 的 rhapsody `rg/attachments.rs:490`（`self.framebuffers`）在途改动
  临时阻塞 —— 非音频域、非本改动。

## 剩余（诚实登记）

1. decorator loop 槽↔事件名映射（需 DecoratorAudioState 完整布局，fresh decomp 已给 16 槽偏移）。
2. signifier 21 参数全表（tag→参数映射）。
3. wall_path / orchestrator 正确语义需墙域事件类型 + MaterialStats/AlignKind 类型。
4. roof FMOD 参数赋值细节；water_ambience cell 位置 sample-offset/y 语义；
   firstperson SSBO 位栅格（SsboType NAME 0x3f/0x45）。
5. `PlayDecoratorSoundCmd` 事件需补 selector/value 字段（producer 待填）。

## 证据清单（deep/ 新增）

- jobs/results：b1-sheep-xrefs、b1-sheep-bounds、b1-sheep-tables、b1-sheep-disasm-a、
  b1-sig-disasm、b2-dec-disasm(+2)、b3-tf-disasm、b5-wa(-2,-3)、b5-wa2。
- disasm/：b1-sheep-a.txt、b1-sig.txt、b2-dec.txt、b2-dec2.txt、b3-tf.txt、b5-wa.txt、
  b5-wa2.txt、b5-wa3.txt。
