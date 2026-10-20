# 第 10 轮 G2① — 回放消费者与读档残留契约

日期：2026-10-19
范围：Tiny Glade B 版 `country-core` 编辑历史回放消费者、History Pop、读档残留。
本文件只写证据与契约，不改源码。

证据根：
- 还原 worktree：`D:/game/tiny-glade/crates/`（line 号是 2026-10-19 取证时的 worktree 快照；② 正在并发修改，行号可能漂移，以函数名/地址为准）
- 骨架/PDB：`D:/game/reverse-engineering/tiny-glade/skeleton/B/crates/`
- Ghidra/反编译证据：`D:/game/reverse-engineering/tiny-glade/evidence/`
- 机器码 oracle：`python D:/game/reverse-engineering/tiny-glade/scripts/rne-read.py`

置信度：
- **O** = 机器码/反编译/PDB 签名直接证实
- **I** = 由多条 O 证据推出，仍是解释性结论
- **U** = 未决，不得编造

> 取证期间发现 ② 已把 `systems/history.rs`、`replay_color.rs`、`replay_decorator.rs`、`replay_wall.rs`、
> `replay_stairs.rs`、`snapshot.rs` 改到“有消费者”的版本。本契约以**原版机器码**为判据，
> 对其中 4 处语义偏差（C1–C4）与 2 处剩余缺口（G1–G2）给出明确纠偏，见 §0。

---

## 0. 结论与纠偏摘要

| ID | 主题 | 原版事实 | 当前 worktree 状态 | 处置 |
|---|---|---|---|---|
| C1 | 颜色回放节奏 | 只在 `elapsed == 0` 派发一次；`undo?from:to`；**无插值/无 easing**；完成判据 `required=max(duration/speed,0.1)`，`required<elapsed`；`elapsed += GameTime.delta`（不再乘 speed） | `replay_color` 每帧发事件，`cubic_in_out(elapsed/duration)` 进度，`dt=delta*speed`，`elapsed>duration` 才移除；亮度按 progress 插值 | 按 §2 重写节奏；颜色是离散 id 的一次性写回 |
| C2 | 装饰回放职责 | `system_decorator::replay_decorator` **不写 `DecoratorStorage`**；只做桶 cull-allow 集合注入 + tag-7 命令计时/移除；存储恢复在 Pop 的 `restore_decorators` | `apply_decorator_replay` 在回放中直接写存储，`progress<1` 跳过，`cubic_in_out` 进度 | 按 §3 改为 cull+计时；删除“回放中写存储”；floor=0.05，`elapsed += delta` |
| C3 | 墙颜色 | `WallRestoreColorRequest.observed_special_owner` 来自 source+65（StructureType≠0），原版用它选 observed writer 1/2 | 无条件写 `WallColorIds`，忽略 special | 按 §4 分流（normal→墙色表；special→门/结构色表），并把 plaster/sound 副作用显式登记 |
| C4 | 楼梯音频载荷 | kind 两条路径**不对称**：instant `0x12-undo`，animated `0x11+undo`；`DecoratorInfo.u30` 写事件 `+0x28`；`+0x30` 写 1 | 两条路径都用 `0x11+undo`；u30 写进 `values[0]`；`+0x30` 未承载 | 按 §5 修正 |
| G1 | 非 native-physics Pop | 原版 Pop 一定恢复 decorator/stairs aux 并 `extract_diff`/flush collider | `#[cfg(not(native-physics))]` 仍只注册 `pop_empty_replay_bucket` | 补 CPU 全量 Pop 或显式 gap（§1.2） |
| G2 | 读档残留 | RoofAssociations / 树 prefab 回退 / OnTerrainChange reader / Clutter to_info | RoofAssociations 已接；树仍记 gap；OnTerrainChange 无 reader；to_info 仍清零 Clutter 簇 | 按 §6 |

---

## 1. 历史回放的调用顺序与 owner

### 1.1 原版三流顺序（ALGORITHMS §1.1/§1.2，O）

```
工具/编辑
  → History::add 0x140a84ec0 / History::add_decorator_diff 0x140a854a0
  → pending bucket（edits + Arc decorator diff + Arc stairs diff）
  → History::finish_input_recording 0x140a86730（上 edit_chain）
  → [Undo/Redo 请求] history_process 0x140af5b30（Cursor/命令展开）
       generate_replay_cmds_from_edit（tag 跳表）→ ReplayCommand 0x30B×N
       + bucket 携带 aux（decorator Arc / stairs Arc / cull-allow Arc）
       → ReplayBuffer.push
  → 各域 replay 系统（同一 ReplayBuffer 前桶）
       replay_wall 0x140951b80、replay_terrain 0x140995150、
       replay_stairs 0x140af75b0、replay_auto_clutter 0x140aa4a60、
       replay_raster×6（0x1409f1cf0..）、
       system_color::replay_color 0x141c7f270、
       system_decorator::replay_decorator 0x14080c8d0
  → replay_pop_bucket 0x140b38250（前桶命令空时）
  → finish_replay 0x1408b36a0
```

owner：
- 队列/桶/aux：`country_core::resources::history::replay_cmd_buffer`（Bucket 0x40B、aux Arc）。
- 颜色回放：`system_color` crate，不是 `country_core::systems::history`。
- 装饰回放：`system_decorator` crate，不是 `country_core::systems::history`。
- Pop：`country_core::systems::history::replay_pop_bucket`。

本仓库的局部调度（`crates/country-core/src/systems/history.rs:562-648`，I：原版全局 Bevy 顺序未回收）：

```
Cursor  history_process_system                      (history.rs:585)
Cursor  history_clear_previous_state (native-physics)(history.rs:589-597)
Replay  replay_stairs / replay_auto_clutter / replay_wall /
        replay_rasters / replay_color / replay_decorator        (history.rs:598-603)
Replay  apply_color_replay / apply_decorator_replay /
        apply_wall_replay_color / apply_wall_replay_audio        (history.rs:604-613)
Replay  replay_mode
Pop     replay_pop_bucket_system (native-physics) 或 pop_empty  (history.rs:619-624)
Pop     finish_replay.after(pop)                                (history.rs:625-636)
Record  recording 四系统 chain                                  (history.rs:639-648)
```

结论：
- **Pop 必须在所有 Replay 消费者之后**，否则命令尚在桶中就被弹出（O：原版 Pop 判据是前桶 `commands.len()==0`，§1.2）。
- **Record 在 Pop 之后**是当前局部适配；原版跨系统全局顺序未取证（U），不得宣称已判等。
- `history_clear_previous_state` 必须在 `history_process_system` 之后、任意 replay_decorator/消费者之前（两者都读/写 `NewReplayCommandEvent` 与 archivist allowed 集合）。当前 `history.rs:589-597` 已满足（O 语义，I 调度）。

### 1.2 Pop 三个实现的差别（O）

| 函数 | 位置 | 行为 | 与原版 0x140b38250 的差距 |
|---|---|---|---|
| `pop_empty_replay_bucket` | `crates/country-core/src/systems/history.rs:304` | 仅前桶命令空 → `pop_front_bucket` → 发 `ReplayBucketFinished` | **缺**：前桶非空时 `extract_diff`；pop 后 `restore_decorators`；stairs aux；`RaycastWorld::flush_colliders`。**不得作为生产 Pop** |
| `replay_pop_bucket`（自由函数） | `history.rs:346` | 前桶非空 → `extract_replay_diff`；空 → pop + decorator aux `restore_decorators` + stairs aux（`restore_backup/apply_diff/make_backup`）+ `extract_diff` + collider flush | 语义最全；stairs 两张结构连接 map 未建模（已登记 U）；需要调用方提供 storage/archivist/raycast |
| `replay_pop_bucket_system` | `history.rs:409`（`#[cfg(feature="native-physics")]`） | ECS 版：前桶非空 → `archivist.extract_diff()` 立即 drop；空 → pop；decorator aux `restore_decorators`；**stairs diff 非空则返回 Err**（不吞载荷）；drop bucket → `extract_diff` → `flush_colliders` → 发 `ReplayBucketFinished` | 与原版一致的生产路径；stairs 结构连接为显式拒绝 |
| `history_clear_previous_state` | `history.rs:509` | 每个 `NewReplayCommandEvent` 清 `archivist.culling.allowed_ids`；不排空 decorators/collision | O：原版 inline @0x141439e7b..ecc 只清 permitted-ID set。**不是 Pop**，不要与 Pop 混用 |

原版 Pop 关键事实（O，decomp 0x140b38250；ALGORITHMS §2.7）：
1. 每帧只检查一次前桶；`commands.len()==0` 才 `pop_front`，最多弹一个桶（无循环）。
2. 先应用 decorator aux（0x14155ba30 `restore_decorators`），再应用 stairs aux 链。
3. 收尾无条件 `archivist.extract_diff()` + `RaycastWorld::flush_colliders`。
4. `ReplayBucketFinished` 语义由 history 侧适配发出（原版无此 Bevy 事件名）。

**生产选择（契约）**：
- `native-physics`：只注册 `replay_pop_bucket_system`；`finish_replay` 依赖它。
- 非 `native-physics`：必须提供等价的 CPU Pop（调用 `replay_pop_bucket` 自由函数，collider 用 CPU remover 或 no-op 并登记），否则 decorator/stairs aux 永远不会被恢复。**当前仍注册 `pop_empty_replay_bucket`，是 G1 缺口。**
- 任何构建只允许一条 Pop 路径，禁止重复弹桶。

---

## 2. 颜色回放的消费者契约

### 2.1 原版 owner（O）

- `system_color::replay_color` @ **0x141c7f270**（PDB：`evidence/country-core-deep/systems-recovery/groupA/BASELINE-signature-surface.md:170-171`；实现侧判等见 `crates/systems/color/src/replay_color.rs`）。
- 15 参：`ResMut<ReplayBuffer>` + 13×`EventWriter<...>` + `Res<GameTime>`。
- 它**不直接写颜色表**：把命令翻译为 13 类写回命令事件；真正的写表 owner 是同 crate 的
  `set_wall_color_id` 0x142290820、`set_gate_color_id` 0x142299840、
  `set_roof_customization` 0x142299dd0、`set_halftimber_color_id` 0x142299250、
  `set_decorator_color_id` 0x1422af0c0、`set_window_light` 0x142298260、
  `set_flag_connection_n_chimney_smoke` 0x142298970。

载荷：命令 tag 6、`Arc<ColorEdit>`；`ColorEdit` 14 个 `ColorTarget` 变体（3..=16）见
`crates/country-core/src/resources/history/color_edit.rs:1-60`（O，fresh serialize 0x14003c5d0）。

### 2.2 原版语义（O，decomp 0x141c7f270）

| 项 | 事实 | 证据 |
|---|---|---|
| 派发门 | 仅当命令 `elapsed@+0x18 == 0.0` 且非 NaN 才派发；同一命令只派发一次 | `evidence/country-core-deep/systems-recovery/groupA/color/decomp/decomp-0x141c7f270.txt:236,263,290` |
| undo | 所有目标一致：`undo==0 → to`，`undo!=0 → from` | 同上（各 case） |
| instant | `flags bit0==0` → 派发后立即记移除，不推进 elapsed | 同上 :204,594 |
| timed 完成 | `required = duration@+0xc / speed`；`required <= 0.1 → 0.1`；`required < elapsed → 记移除`；随后**无条件** `elapsed += GameTime.delta` | 同上 :608-621；floor 常量 `0x142925984 = 0x3dcccccd = 0.1f`（rne-read） |
| 无插值 | case 内直接写 `from`/`to`，没有 easing/lerp | 同上各 case |
| 删除 | 完成索引 `len>1` 时先反转再逐个 `Vec::remove` | 同上 :624-694 |
| 未派发 | `StairSupport`（target 16）在 switch 中无 case；原版 target>0xd 被夹到 FlatRoof（Rust 闭合枚举不可表示，登记 U） | `crates/systems/color/src/replay_color.rs` 文件头 |

目标/写者表（O）：见 `crates/systems/color/src/replay_color.rs` 的 14 行表：
Wall→`SetWallColorIdCmd`、Gate→`SetGateColorIdCmd`、Roof→`SetRoofColorIdCmd`、
FlatRoof→`SetFlatRoofStyleCmd`、Decorator→`SetDecoratorColorIdCmd`、
WindowLight→`SetWindowLightCmd`、Halftimber→`SetHalftimberColorIdCmd`、
FlagBunting→`SetFlagConnectionCmd`、Brightness→`SetDecoratorBrightnessCmd`、
ChimneySmoke→`SetChimneySmokeCmd`、StairLadder→`SetLadderColorIdCmd`、
StairRailing→`SetStairRailingCmd`、MagicColor→`SetMagicColorIdCmd`。

### 2.3 consumer 契约（core 侧边界事件，若保留即时事件）

若 G2 选择在 core 内保留 `ColorHistoryReplayApply`（而不是直接接 system_color）：

```
ColorHistoryReplayApply {
    edit: Arc<ColorEdit>,
    undo: bool,
    // 原版只派发一次；不得用 progress 表达动画。
    dispatch: bool,      // 仅 elapsed==0 帧为 true
}
```

- `dispatch` 帧：按 `undo?from:to` 选值，写目标资源；离散 id 与亮度 f32 都是**一次写入**。
- 不允许每帧重复写（会覆盖后续编辑/重放其它命令），也不允许 `lerp/brightness` 插值。
- 命令移除：`required = max(duration/speed, 0.1)`；`required < elapsed` 时移除；
  `elapsed += GameTime.delta`（**不乘 speed**；speed 只进入 required 的分母）。
- `progress` 字段应删除或恒为 1.0；保留会在契约上误导。
- 目标资源（core 可直接写）：
  - `resources::color_id::{WallColorIds,GateColorIds,HalftimberColorIds,RoofColorIds}`（CPU 表）
  - `resources::walls::decorator_storage::DecoratorStorage`：`DecoratorInfo` 的 color_id（+0x28）
    与窗灯/旗/烟囱/灯/楼梯附加簇
- **不要**在 core 直接写表后仍让 `system_color` 的同名 `set_*` 读者运行：两者会重复应用。
  要么走事件命令（system_color 消费），要么 core 独占（必须同时复刻 plaster 分类、声音、
  SSBO dirty，当前为 U）。推荐前者：core 定义的事件类型（`resources::color_id`、
  `resources::walls::decorator`、`resources::roofs::commands`）能覆盖 12 个目标；
  `SetRoofColorIdCmd` 定义在 `system_color`，属跨 crate 接缝，登记中央。

### 2.4 对当前 worktree 的纠偏（C1）

`crates/country-core/src/systems/history/replay_color.rs`（取证时 454 行）：
- `dt = time.delta * replay.speed` + `elapsed += dt`：原版是 `elapsed += delta`，speed 只在 `duration/speed`。
- 每帧 `apply.write(...)` + `cubic_in_out`：原版只在 `elapsed==0` 派发一次。
- 完成判据 `elapsed > duration`：原版 `required=max(duration/speed,0.1)` 且严格 `required < elapsed`。
- 亮度 `lerp_by_progress`：原版直接取 `from/to`。

---

## 3. 装饰回放消费者契约

### 3.1 原版 owner（O）

- `system_decorator::replay_decorator` @ **0x14080c8d0**（PDB：`evidence/country-core-audit/data/procs.tsv:16387`；
  fresh 反编译：`evidence/restorer-state-decorator/decomp/0x14080c8d0_replay_decorator.c`）。
- 参数（O，寄存器）：`ResMut<GameTime>`、`ResMut<ReplayBuffer>`、`ResMut<DecoratorArchivist>`。
- 现有模型实现：`crates/systems/decorator/src/replay_decorator.rs`（文件头诚实登记 core 缺口）。

### 3.2 原版行为（O，decomp）

1. 取 `ReplayBuffer::front_mut_bucket`。
2. 取桶 **`+0x18/+0x20` 的 `Arc<[DecoratorId]>` cull-allow 表**，把指针置 0（**take 语义**）；
   非空则逐 id `HashMap::insert` 进 `DecoratorArchivist.culling.allowed_ids`，随后 Arc 释放
   （decomp :46-60；当前 crate 的 `DecoratorCullPolicy.allowed_ids` 见
   `resources/walls/decorator_archivist.rs:73-85`）。
3. 扫描命令，步长 0x30，只处理 `tag == 7`（decomp :254）：
   - `flags bit0 == 0`（instant）→ 立即记完成；
   - 否则 `required = max(duration@+0xc / speed, 0.05)`；`required < elapsed@+0x18` → 完成；
     未完成/完成都**无条件** `elapsed += GameTime.delta`（decomp :270-283；floor 0.05 为 O）。
4. 完成索引反转后逐个 `Vec::remove`，按 tag 释放载荷（6/7 都是 `Arc<DecoratorHistoryEdit>`）。
5. **不读 `edit` 载荷、不写 `DecoratorStorage`**；载荷只在移除时 drop。

### 3.3 载荷与写回 API

- `DecoratorReplayCmd { edit: Arc<DecoratorHistoryEdit>, timed, duration=0.2, elapsed, undo }`
  （`resources/history/replay_cmd_buffer.rs:83-114`）。
- `DecoratorHistoryEdit { diffs: HashMap<DecoratorStorageAddr, DecoratorStorageHistoryDiff> }`
  （`resources/history/decorator_edit.rs:26-36`）。
- 真正的存储写回 API（Pop 阶段）：
  - `DecoratorStorage::restore_decorators(diff, reverse, id_gen, colliders, events)`
    @0x14155ba30（`resources/walls/decorator_storage.rs:1124`；`reverse == bucket.undo`
    → 选择 before/after 侧）。
  - `send_state_diff_events` @0x1417bdae0（同文件 :1152，写入 `DeferredDecoratorOpEvents`）。
  - 批量 `restore_decorators_diff`（同文件 :1305）供无 collider 的 CPU Pop 使用（I 等价）。

### 3.4 consumer 契约（C2）

在 G2 文件权内（`resources/history/**`、`systems/history/**`）可实现：

```
ReplayBucket {
    ...,
    cull_allow: Option<Arc<[DecoratorId]>>,   // 原 bucket +0x18/+0x20；Option::take()
}
```

`replay_decorator` 契约：
- 取 `bucket.cull_allow.take()`；Some → 全量 id 插入 `DecoratorArchivist.culling.allowed_ids`；
  None → 不插入（不是清空；清空由 `history_clear_previous_state` 在 NewReplayCommandEvent 时做）。
- 只处理 `ReplayCommand::Decorator`；instant → `finish`；timed `required=max(duration/speed,0.05)`，
  `required < elapsed` → finish；`elapsed += GameTime.delta`（不乘 speed）。
- 完成后逆序移除。
- **禁止**像当前 `apply_decorator_replay` 那样在回放中写 `DecoratorStorage`/`send_state_diff_events`；
  这会在 Pop 的 `restore_decorators` 之外重复应用，并与原版 0x14080c8d0 不符。
- `DecoratorHistoryReplayApply` 事件应删除或仅保留为“cull 完成”通知，不带 `progress`。

`history_clear_previous_state`（已注册）负责每新 replay 命令前清空 allowed 集合；两者顺序不能反。

---

## 4. 墙颜色 / 墙音频回放

### 4.1 载荷（O）

`resources/history/wall_edit.rs:373-384`：

```
WallRestoreColorRequest { wall: WallId, color_id: u32, observed_special_owner: bool }
WallReplayAudio { wall: WallId, anchor: [f32;3], kind: u8, flag: u8 }
```

生产者：`WallReplayEditCmd::process` 的 instant/动画路径（`wall_edit.rs:467-520`；
`systems/history/wall_replay_interpreter.rs`）。`observed_special_owner` 在
`replay_wall.rs` 里最终以 `inner.get_unwrapped(...).is_special()` 覆盖（`replay_wall.rs` 约 :226）。

`is_special` = source **+65**（StructureType）≠ 0（`resources/walls/inner_wall_state.rs:218-227`；
`resources/walls/saved_wall_state.rs:219-222`；O：source +65 → Public +296）。

### 4.2 墙颜色 consumer 契约（C3）

原版 instant process “preserves real restored colors and uses source65 to select observed writer 1 or 2”
（`evidence/building-wall-history/findings.md:15`，O 读面 / I 命名）。

契约（I；两个 writer 都是 O 存在的同构写入面）：
- `observed_special_owner == false` → 写 `SetWallColorIdCmd`（`resources/color_id.rs:147`），
  由 `system_color::set_wall_color_id` 0x142290820 消费：`WallColorIds[idx]=color`；
  idx 断言 `< MAX_NUMBER_OF_WALLS=0x5000`；custom 色集 `{4..10,12}` 触发 `CreatePlasterCmd`，
  旧色在集合而新色不在触发 `DeletePlasterCmd`；`PlayWallUiSound kind=6`。
- `observed_special_owner == true` → 写 `SetGateColorIdCmd`（`resources/color_id.rs:164`），
  由 `system_color::set_gate_color_id` 0x142299840 消费：`GateColorIds`，“墙/门共享索引空间”
  （`evidence/country-core-deep/systems-recovery/groupA/color/RE-ANALYSIS.md:48`，O）。
- 当前 worktree 的 `apply_wall_replay_color`（`replay_wall.rs:44-64`）忽略 special，直接写
  `WallColorIds`：**必须按上表分流**；否则结构墙（gate/fence）颜色落到墙色表。
- 若不想在 core 复刻 `set_*` 的 plaster/sound/SSBO 副作用，应发对应命令事件让
  `system_color` 消费，并登记“core 直接写表为 partial”这一 U；不得把 partial 写成完成。

### 4.3 墙音频 consumer 契约

原版事件面：`PlayWallAudio` 槽 stride 0x28：`kind@+8`、`value f32@+0xc`、
`position Vec3@+0x10`、`volume f32@+0x1c`、`flag@+0x20`
（`systems/audio/mod.rs:55-66`；`evidence/multi-baseline/audio-events-analysis.md:290`，O）。
读者：`wall_audio_fmod` 0x140950920（`systems/audio/wall_audio_fmod.rs:66-...`，O 签名；
kind 0..23 跳表 0x142ab4998）。

映射契约：
- `kind = WallReplayAudio.kind`
- `position = anchor`
- `flag = WallReplayAudio.flag`
- `value = 0.0`、`volume = 1.0`（I：`WallReplayAudio` 载荷没有对应字段；wall/bulldoze 证据只出现
  “constant anchor audio kind0/flag0”“kind0/flag1”，见 `evidence/building-wall-history/findings.md:15,17,43`。
  若后续取证到 anchor 音量/曲线进度，再补字段，不提前编造。）
- 当前 worktree 的 `apply_wall_replay_audio`（`replay_wall.rs:67-83`）与此一致，保留；
  建议补一条 kind/flag 表测试。

---

## 5. 楼梯回放音频

### 5.1 原版命令与事件（O）

- `replay_stairs` 0x140af75b0：命令 `{tag 10, flags@+8 bit0=instant, duration@+0xc,
  payload@+0x10 (edge1 u32@+4, edge2 u32@+8, kind bit@+0x10), elapsed@+0x18, undo@+0x1c}`。
- 附挂装饰：`DecoratorStorage::lookup_decorator_info(DecoratorId(edge1))`；
  过滤 `(info.extra_style() | 0xf8) > 0xf9` → 跳过 pair（decomp 0x140af75b0 :200-208）。
- 位置：`CachedDecoratorTransforms` 以 **edge1** 为键 SIMD 查找（0x1405b0ed0），
  命中记录 `+0x20..+0x2c` 的 12B 作为 position；未命中写 0（decomp :221-235、:332-345）。
- `PlayStairsAudioCmd` 槽 stride 0x38（`systems/audio/mod.rs:79-90`；
  `evidence/multi-baseline/audio-events-analysis.md:292`，O）。
- 消费端 `stairs_audio_system` 0x140f880a0：kind 0..18；kind 17/18 复用
  `event:/SFX/Stairs/StairSupportAppear`（`systems/audio/playermade_stairs_audio.rs:41-66`，O）。

### 5.2 载荷组装契约（C4）

| 槽 | 内容 | 置信度 | 证据 |
|---|---|---|---|
| `kind@+8` | **instant 路径**：`0x12 - undo`（undo=0→18，undo=1→17）；**animated 路径**：`0x11 + undo`（undo=0→17，undo=1→18） | O | decomp 0x140af75b0 :219（`cVar40 = '\x12' - undo`）与 :501（`cVar40 = undo + '\x11'`）；两条路径的边集 undo 语义也相反（:212-218 vs :371-379） |
| position@+0x1c (12B) | CachedDecoratorTransforms[edge1] 命中记录的 12B；未命中 0 | O/I | decomp :221-235 组装进入 auVar21 |
| `u64@+0x28` | `DecoratorInfo.u30`（lookup 命中项的 `*(u64*)(info+0x30)`）；过滤/未命中 → 0 | O（存储点）/I（语义名 u30） | decomp :317-333 |
| `byte@+0x30` | 常量 1 | O | decomp :520 |
| 其它 bytes/values | 原版载荷来源未解码（Ghidra 栈别名）；**不得声称等于 u30 或 0** | U | decomp :507-518 与 :595-603 存在别名冲突 |

当前 worktree `replay_stairs.rs:78-100` 的问题：
1. kind 对 instant 路径错误（写 `0x11+undo`）。
2. `values[0] = info.u30` 位置错误；u30 应写 `+0x28`（`tail` 槽，按两个 u32 拆分）。
3. `+0x30 = 1` 没有承载（`PlayStairsAudioCmd` 需补 `flag: u8` 或 `trailing: u8` 字段）。
4. 建议测试覆盖 instant/animated × undo 两向 × decorator 命中/过滤/缺失三种 position。

---

## 6. 读档残留

### 6.1 `RoofAssociations::rebuild`

- 类型/API：`resources/roofs/associations.rs:49-110`：
  `ResolvedRoofAssociation { wall_id: publication_fields::WallId, entity: u64,
  marked: bool, public_wall: Option<&PublicWallBuildingState> }`；
  `rebuild(rows, public_wall_order, phase) -> Result<AssociationChanges, AssociationError>`；
  `AssociationChanges { invalid_roof_requests: Vec<WallId>, wall_presence_changes: Vec<WallId> }`。
- 原版 owner：`wall_roof_hashmap` 0x140a94cd0（O 结构，47KB）：
  - 第一遍 `Query<(Entity, &Roof, Option<...>)>`，roof 记录 0x58B，`wall_id = *roof_rec`；
  - `PublicWalls` 0x2a8 查表；shape tag `>1`（Freehand）**跳过**；
  - tag<2（预置形状）→ 设 `roof_entity`；同一 wall 已有 roof → panic
    `Only one roof should be associated with a wall id`；
  - 删除模式（AppMode 0xd/0xe）单独分支。
  （`evidence/country-core-deep/chapters/systems/wall/wall_state/wall_roof_hashmap.rs.md:19-34`；
  `evidence/country-core-deep/wall/ALGORITHMS.md:196-212`）
- 调用点：`systems/save/snapshot.rs` Roofs 段（取证时 :1580-1646）已在 spawn 后调用
  `associations.rebuild(rows, &order, 0)`；当前 worktree 已接。
- 契约/复核项：
  1. rows 必须来自 `Roof` 实体的 wall_id + 对应 `PublicWalls.get(wall).building_projection()`
     （O：`PublicWallBuildingState` 行）；缺行不能伪造，保持 gap。
  2. `marked` 在 load 路径为 false（capture 用 `Without<RoofMarkedForDeletion>`）；若以后支持
     带删除标记的 load，改为 `RoofMarkedForDeletion` 判定。
  3. `public_wall_order` = `PublicWalls` 的实际 key 迭代顺序（当前 `wall_ids()`）；用于
     `wall_presence_changes`。
  4. `phase`：load 传 0（原版 13/14 只属于其它阶段；传 13/14 会把普通 shape 判为
     `InvalidShapeDuringPhase*`）。
  5. `AssociationChanges` 必须被消费：`invalid_roof_requests` → 屋顶删除请求（roof 域），
     `wall_presence_changes` → 映射更新/事件；当前只 push gap，属 partial（I）。
  6. 原版 “同一 wall 多 roof” panic、Freehand 跳过、tag<2 限制已由 `association_action`
     表达（O 语义，行 :23-47）；不要放宽。

### 6.2 树 prefab 缺失回退：core / tree-tool 边界

- core 已有 sentinel：`resources/history/tree_edit.rs:191-197`
  `missing_prefab() -> "MISSING_TREE_PREFAB_NAME"`（O，0x1409e6210）。
- tree-tool：`random_tree_prefab(selector: u8, deny_load: bool, is_halloween: bool,
  is_corrupted: bool, drag_started: bool) -> &'static str` @0x14226a170
  （`crates/systems/tree-tool/src/ui_place_tree.rs:108`；O 反编译）。
- 原版消费点：`process_tree_cmds` Insert 分支，当事件 prefab 名 == sentinel 时调用
  `random_tree_prefab(cmd.id & 1, deny_load, is_halloween, is_corrupted, false)`
  （`crates/systems/tree-tool/src/process_tree_cmds.rs:183-199`；O）。
- 依赖方向：`tree-tool` 依赖 `country-core`，core **不能**反向依赖 tree-tool
  （`crates/systems/tree-tool/Cargo.toml:10`）。因此 core 读档不得直接调用
  `random_tree_prefab`。
- 契约：
  1. `snapshot.rs` Trees 段对 `prefab_name.is_empty()` 的条目写 **sentinel**
     `missing_prefab()` 到 `SpawnPrefabCmd.prefab_name`（而不是不 spawn / 空串），
     同时保留“待解析”标记。
  2. 由 tree-tool 或上层 load 编排在 spawn 前解析 sentinel：
     `resolve_loaded_tree_prefab(selector, deny_load, is_halloween, is_corrupted, drag_started)`
     → `random_tree_prefab(...)`；`selector = entry.tree.id & 1`；`drag_started=false`（T4 未取证，
     见 `process_tree_cmds.rs:68`）。
  3. 当前 worktree 空 prefab 只 `empty_prefabs += 1` 并记 gap（snapshot.rs:1742-1763），
     未写 sentinel、未解析 → 真实树缺失（G2 缺口）。
  4. 不允许把 `random_tree_prefab` 复制进 core（业务归属 tree-tool）。

### 6.3 `OnTerrainChange` 的真实 reader

- writer（O）：`systems/terrain/delayed_terrain_changed_msg.rs:15-33`，0x140aabdd0：
  两帧软件延迟后，在 authority→CPU copy 与 visual→CPU copy **之间**发 `OnTerrainChange`。
- 真实 reader（O）：`country_core::systems::prefab::update_prefabs` @ **0x140841d60**
  （PDB：`evidence/country-core-audit/data/procs.tsv:16640-16641`）。
  PDB 签名（skeleton 泛型关系）：
  `(Query<&mut PrefabInstanceData>, Res<TerrainHeightsData<TerrainHeightSsbo>>,
  EventReader<OnTerrainChange>)`。
- owner：`crates/country-core/src/systems/prefab.rs`；当前仅有纯函数
  `update_prefabs_instance`（`systems/prefab.rs:557`），**没有**带 `EventReader<OnTerrainChange>`
  的系统，也未注册。
- 契约：G2 文件权不含 `systems/prefab.rs`，② 不要在地形/历史域伪造 reader。登记为跨模块缺口：
  “`OnTerrainChange` reader = `systems::prefab::update_prefabs` 0x140841d60（PDB 签名 O），
  待 prefab 域实现/注册”。地形 writer 本身正确，不需改。

### 6.4 Clutter 的 `DecoratorHistoryState::to_info` 扩展簇

- 现状：`resources/history/decorator_edit.rs:119-145`：
  ```
  let style = match ty { 0x38..=0x3a => 6, 0x3b => 4, 0x3c => 7, 0x3d => 5,
                        0x3f => self.style, _ => 3 };
  let (...) = match ty { ... 0x3f => (u30|u3c<<32, u40, u44, u4b),
                         _ => (0,0,0,0) };
  ```
  对 `ty < 0x38`（Clutter，`CLUTTER_MESH_NAMES` 索引 0..=55，见
  `systems/save/snapshot.rs:544` 与 `decorator_ty_tag` :2455-2472），fallback 把
  `u30/u38/u3c/u40` 清零，且 style 写 3。
- 反向路径 `DecoratorInfoJson::to_history_state` 已写全
  `h_offset/u30、window_light/u34、lantern/u38、stair_width/u3c、ladder/u40、magic/u44、
  railing/u4b、allow_connection/u4c、chimney_smoke/u4d、style/u4e`
  （`systems/save/snapshot.rs:2461-2505`；`style = u8::from(stairs_supports.is_some())`
  :2573，Clutter 为 0）。
- 影响：round9 实测 6/134 条样本装饰的 h_offset/ladder/magic/railing/style 回落默认
  （`evidence/round9/deliveries/g1-snapshot-apply-DELIVERY.md:61`；
  `evidence/round9/reviews/g1-review.md:253-255`）。
- 原版 restore：0x140a20d70 的 style 分派以 `ty - 0x38` 夹取 0..=6 后跳表
  （`evidence/round9/tools/dec-0x140a20d70.c:163-176`；O：clamp+switch 形状；
  各 case 的逐槽映射仍有 U）。Clutter（ty<0x38）与 ty=0x3e/未知类型共享夹取后的 index 6 分支。
- 契约：
  1. 不允许对 `ty < 0x38` 无条件清零 `u30/u38/u3c/u40`。至少保留原始槽到 `DecoratorInfo`
     （即使语义未定），避免真实状态丢失。
  2. 按 0x140a20d70 逐 case 复核：哪些槽在 switch **之前**无条件拷贝、哪些由 case 选择；
     对 Clutter 使用与 index 6 相同的规则。
  3. style 不得默认写 3；Clutter 的 `to_history_state` 写出 0，`to_info` 必须能回读 0
     （否则 `to_bytes` round-trip 不等，现有测试 `snapshot.rs:2933` 已暴露 style 语义）。
  4. 验收：36 个真实 `.snapshot` 语料做 `to_history_state → to_info → to_bytes` 逐字节 round-trip；
     Clutter 行不得出现扩展簇被清零。若映射仍未定，保持 typed gap + 原始槽保留，不得静默丢字段。

---

## 7. ② 实现清单（≤12 步，按调用顺序）

> 前 7 步是回放消费者，8–9 是 Pop，10–12 是读档残留。每步都要有测试；不得 no-op/日志冒充写入。

1. **颜色节奏（C1）**：改 `systems/history/replay_color.rs`：只在 `elapsed==0` 派发一次；
   `undo?from:to`；删 `cubic_in_out`/亮度 lerp；完成判据 `required=max(duration/speed,0.1)`、
   `required<elapsed`；`elapsed += GameTime.delta`。补 instant/timed/NaN/floor 测试。
2. **颜色写回**：`apply_color_replay` 不再每帧重写；按 §2.3 写目标表/DecoratorStorage，
   或改发 12 个 core 已有命令事件让 `system_color` 消费；`SetRoofColorIdCmd` 跨 crate 登记中央。
   删除 `progress` 字段或固定 1.0。
3. **装饰职责（C2）**：`ReplayBucket` 增加 `cull_allow: Option<Arc<[DecoratorId]>>`（take 语义）；
   `replay_decorator` 只做 cull 注入 + tag-7 计时（floor 0.05，`elapsed += delta`）；
   删除 `apply_decorator_replay` 的存储写回/`DecoratorHistoryReplayApply`。
4. **装饰恢复归位**：确认 decorator aux diff 只在 Pop 的 `restore_decorators(reverse=undo)` 应用；
   加测试“run replay 一帧后 `DecoratorStorage` 不变，Pop 后才变”。
5. **墙颜色分流（C3）**：`apply_wall_replay_color` 按 `observed_special_owner` 写
   `SetWallColorIdCmd` 或 `SetGateColorIdCmd`；补索引 `<0x5000` 与 special 两向测试；
   把 plaster/sound 副作用列为显式 follow-up（或走事件）。
6. **墙音频**：保留 `PlayWallAudio` 映射（kind/anchor/flag，value=0，volume=1）；
   补 kind 0/1/2 × flag 0/1 的表测试，注明 value/volume 常量来源（I）。
7. **楼梯音频（C4）**：kind 按路径 `instant?0x12-undo:0x11+undo`；position 取
   `CachedDecoratorTransforms[edge1]`，缺失 0；`u30` 写 `+0x28`（两个 u32）；
   `+0x30` 常量 1（补事件字段）；补 4 种路径×undo 组合测试。
8. **Pop 生产路径（G1）**：`native-physics` 只注册 `replay_pop_bucket_system`（已做）；
   非 native-physics 用 `replay_pop_bucket` 包 CPU system（decorator aux + extract_diff +
   collider flush/no-op + 发 `ReplayBucketFinished`），替换 `pop_empty_replay_bucket`；确保只弹一次。
9. **清 allowed 集合**：保持 `history_clear_previous_state` 在 Cursor 且
   `after(history_process_system)`（已注册）；补测试：NewReplayCommandEvent 后旧 allowed 清空，
   replay_decorator 再注入新 cull 集合。
10. **RoofAssociations**：复核 snapshot Roofs 段：rows 来自 `Roof.wall_id` + `PublicWalls`
    投影；`marked` 从删除标记取；phase=0；消费 `AssociationChanges`（或登记 typed gap）；
    用 36 样本验证 `report.roof_associations` 等于样本屋顶数。
11. **树 prefab 边界**：空 prefab 写 `missing_prefab()` sentinel；由 tree-tool/host 在 spawn 前
    调用 `random_tree_prefab(id&1, deny_load, is_halloween, is_corrupted, false)`；
    更新 gap 文本；不得在 core 复制该函数。
12. **Clutter 与 OnTerrainChange**：`to_info` 对 `ty<0x38` 保留 u30/u38/u3c/u40 并按
    0x140a20d70 复核、style 回读 0；补 36 样本 round-trip 测试。`OnTerrainChange` 真实 reader
    登记为 `systems::prefab::update_prefabs` 0x140841d60（PDB 签名 O，文件权外），勿伪造。

---

## 8. 证据索引（本契约引用）

| 主题 | 文件 |
|---|---|
| history 三流/算法 | `evidence/country-core-deep/history/ALGORITHMS.md:1-66,288-345` |
| Pop 全文 | `evidence/country-core-deep/chapters/systems/history/replay_pop_bucket.rs.md:19-30`；decomp 0x140b38250 |
| 颜色回放 | `evidence/country-core-deep/systems-recovery/groupA/color/decomp/decomp-0x141c7f270.txt:236-621`；`crates/systems/color/src/replay_color.rs` |
| 颜色写者 | `evidence/country-core-deep/systems-recovery/groupA/color/RE-ANALYSIS.md:15-65` |
| 装饰回放 | `evidence/restorer-state-decorator/decomp/0x14080c8d0_replay_decorator.c:41-283`；`crates/systems/decorator/src/replay_decorator.rs` |
| 装饰写回 | `crates/country-core/src/resources/walls/decorator_storage.rs:1124-1152`；decomp 0x14155ba30 / 0x1417bdae0 |
| 墙回放载荷 | `crates/country-core/src/resources/history/wall_edit.rs:373-520`；`evidence/building-wall-history/findings.md:15-17` |
| 墙音频 | `evidence/multi-baseline/audio-events-analysis.md:215-300`；`crates/country-core/src/systems/audio/wall_audio_fmod.rs:66` |
| 楼梯回放 | `evidence/country-core-deep/history/decomp/decomp-0x140af75b0.txt:180-520`；`crates/country-core/src/systems/history/replay_stairs.rs` |
| 楼梯音频 | `evidence/multi-baseline/audio-events-analysis.md:215-300`；`crates/country-core/src/systems/audio/playermade_stairs_audio.rs` |
| Roof 映射 | `evidence/country-core-deep/chapters/systems/wall/wall_state/wall_roof_hashmap.rs.md`；`crates/country-core/src/resources/roofs/associations.rs` |
| 树 prefab | `crates/country-core/src/resources/history/tree_edit.rs:191-197`；`crates/systems/tree-tool/src/ui_place_tree.rs:99-130`；`process_tree_cmds.rs:183-199` |
| OnTerrainChange | `evidence/country-core-audit/data/procs.tsv:16640-16641`；`crates/country-core/src/systems/terrain/delayed_terrain_changed_msg.rs:15-33` |
| Clutter to_info | `evidence/round9/deliveries/g1-snapshot-apply-DELIVERY.md:61`；`evidence/round9/tools/dec-0x140a20d70.c:163-176`；`crates/country-core/src/resources/history/decorator_edit.rs:119-145` |
