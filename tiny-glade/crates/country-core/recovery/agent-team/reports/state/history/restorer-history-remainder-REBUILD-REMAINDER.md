# history 域剩余函数重建（restorer-history-remainder，2026-10-17）

> 依据 2026-10-16-logic-verify-protocol.md（只以 fresh 反编译为准）。
> fresh 证据落盘：`recovery/agent-team/impl-history-remainder/decomp/`（ghidra shared 批次
> A–P + r2 原始字节/线性反汇编 `decomp/r2/`）；结果 `results/batch*.json`、`results/r2-*.json`。
> 红线遵守：跨域类型对接已有类型，缺则登记、不编造；只改 history 域文件
> （含 History 容器与 replay 解释器，均属历史链）；宏胶水不重建。

## 0. 汇总结论

| # | 函数 | verdict |
|---|---|---|
| 1 | replay_wall 0x140951b80 | **判等**（组循环/done/is_disjoint/移除逐段；操作定时源 U） |
| 1b | WallReplayEditCmd::process 0x140abfb30 | **判等-限定**（done/timed/instant 跳表/cubic/niche 解码；op5..14/16 载荷 U） |
| 2 | replay_edit 0x140af6170 / generate 0x14143fb50·0x1414423e0 | **判等-限定**（tag→命令全表；调度分桶未接线，登记） |
| 3 | replay_mode 0x140a7c550 | **判等-限定**（进出/清画布/hold 0.2/速度公式/lerp/焦点事件；HistoryCmd→AppMode 段 U） |
| 4 | finish_replay 0x1408b36a0 | **判等-限定**（AppMode 写回门 + toolbar 阈值；原生内联清理循环 U） |
| 5 | closest_resolved_snapshot 0x140a84a50 + impl$9::try_from 0x140a84260 | **判等**（谓词 `[+0x10] < 2` 落地） |
| 6 | history_process 0x140af5b30 第二事件流 | **部分判等**（case 0-3 判等；4..0x14 谓词落地为 `can_merge_with`；0x15..0x17 登记） |
| 7 | ColorEdit 0x14003c5d0 / DecoratorHistoryEdit 0x1404f46e0 / RasterEdit 0x1404896f0·0x1416e8510 / TreeEditData 0x14003d4e0 / StairsHistoryEdit 0x140259af0·0x140a18ee0 | **判等-限定**（Color/Decorator 重写；Raster/Tree/Stairs 复核判等） |
| 8 | replay_pop_bucket 0x140b38250 | **判等-限定**（全流程落地；两 IndexMap/+0x178 U） |
| 9 | get_param 0x140ae5830 / history_finish_recording 0x1409dd790 / missing_prefab 0x1409e6210 / is_default_chimney_smoke 0x1408c5680 | get_param 判等-限定；finish_recording 判等-限定；missing_prefab 判等；is_default_chimney_smoke **VA 错位** |

全量 `cargo check -p country-core --lib` 通过；`cargo test -p country-core --lib` **2024 passed / 0 failed**
（resources::history 32、systems::history 27、recovered_building::history* 13）。

## 1. replay_wall 0x140951b80 + WallReplayEditCmd::process 0x140abfb30

### 1.1 process（fresh decomp + r2 跳表解码）

- 入口 `cmd[+0x15] != 0` → return（done 字节）。
- `cmd[+0x00] & 1`：timed 路径；`0` → instant 且**先写 `+0x15=1`** 再分派。
- 进度：`ratio = clamp(elapsed(+0x10) / duration(+0x4), 0, 1)`（MAXSS/MINSS 常数
  0.0/0x3f800000）→ `cubic_in_out`（0x140c961e0）。
- nil 解码：`disc = *(u64*)(*(u64*)(cmd+8) + 0x10)`；`disc < -0x7fffffffffffffff0`
  （= 0x8000_0000_0000_0010）时 `op = disc - 0x7fff_ffff_ffff_ffff`，否则 op=0。
- 跳表（self-relative 4B offset；`r2` 原始字节逐项解码）：
  - instant 表 0x142adf408（0..=17，18 项；op17 落在指令中间，实为 padding/无效）：
    op0 0x140ac0366 / op1 0x140ac02a2 / op2 0x140ac04b3 / op3 0x140ac042c /
    op4 0x140ac0253 / op5 0x140ac0612 / op6 0x140ac0667 / op7 0x140ac05b1 /
    op8 0x140ac0484 / op9 0x140ac0699 / op10 0x140ac0635 / op11 0x140ac06c8 /
    op12 0x140ac0224 / op13·14 0x140abfc80 / op15 0x140ac15b7 / op16 0x140ac05e3。
  - timed 表 0x142adf44c：仅 op0/3/5/6/7/13/14 有动画体；op1/2/4/8..12/16 →
    0x140ac1576 panic；op15 → 0x140ac15b7 deprecated panic。
- 操作体实施映射（helper 名来自 fresh 单态函数）：
  op6→update_elevation 0x140aaaac0、op7→update_arch_height 0x140aaa710、
  op8→update_arch_type 0x140aa9eb0、op9/16→update_style 0x140aaa350、
  op10→update_plaster_peel 0x140aaa530、op11→update_freehand_ty 0x140aa99e0、
  op12→update_supports 0x140aa9800、op13/14→_deprecated_morph_curve 0x140952570。
  **登记**：op8/op12 与 WALLEDIT 表中 ChangeSupports/ChangeArchType 的编号命名不一致，
  以 fresh helper 语义为准，待 walls 域对账。
- panic 文案（fresh read_memory）：0x142adf1e0 len 0x87 =
  `"CreateWall, DeleteWall, MarkEnclosure, ChangeArchType and ChangeSkirt do not
  implement animation. The code should not have reached here"`；
  op15 经 panic_deprecated_operation（0x140a82e20）消息 `"WallHistoryEdit::MarkEnclosure"`。

### 1.2 replay_wall（fresh）

前桶（bucket stride 0x40；命令 stride 0x30；`*cmd == 4` 为 Wall）：逐命令遍历组
VecDeque（组 stride 0x18），组内逐 `WallReplayEditCmd`（stride 0x18）调 `process`，
`done &= cmd[+0x15]`；全 done 后把本组 WallId 集 insert，再取下一组 WallId 集做
`HashSet::is_disjoint`，弹本组并**仅在 disjoint 时继续同帧处理下一组**；末尾把已空组
的命令索引逆序移除（原版二分割 swap + `Vec::remove`，drop 按命令 tag）。
src `advance_wall_groups_observed` 已同构（现有 3 个 replay_wall 测试通过）。

### 1.3 Rust 落地

- `WallReplayCommand::advance`（recovered_building/wall_history_replay.rs）新增：
  instant 分派（op15 panic、AdjustHeight 写 height 请求、其余走 apply_instant）、
  timed 分派（AdjustHeight 插值/完成；5/6/7/13/14 → UnrestoredOperation；其余 panic 原文）。
- 5 个新测试（instant/timed AdjustHeight、timed Create panic、op15 panic、unrecovered）。

## 2. replay_edit 0x140af6170 / generate_replay_cmds_from_edit

- 跳表 0x142c5f284（18 项 self-relative）逐项解码 → edit tag→命令生成体：
  0 Terrain 0x14143fb8c、1..6 Raster 0x14143fdef/0x14143fd17/0x14143fd62/
  0x14143fc75/0x14143fe3a/0x14143fe85、7 Wall 0x14143fdad、8 Roof 0x14143ff27、
  9 Color 0x14143fcc0、10 DecoratorV5 0x14143fed0、11 Decorator 0x141441b83
  （`unreachable!()`，文案在 0x142c5f031）、12 Tree 0x14143ff7e、13 AutoClutter
  0x14143fc1e、14 Stairs 0x14143fbc7、15..17 → 0x1414423e0 系列（EditType 无此 tag）。
- 命令记录 0x30B：`tag@0`、`flags bit0=timed @+8`、`duration f32@+0xc`、
  `Arc 载荷@+0x10`、`elapsed@+0x18`、`undo@+0x1c`。
  常数：Color/Decorator/Wall/Stairs 0.2（0x3e4ccccd）、Tree/AutoClutter 0.1
  （0x3dcccccd）。

**Rust 落地**：`ReplayBucket::expand_with_aux` 全 tag 分派。
- 新增 `ColorReplayCmd`（命令 tag 6，duration 0.2）、`DecoratorReplayCmd`（tag 7，0.2）。
- `EditType::Decorator(11)` → `unreachable!("internal error: entered unreachable code")`。
- 删除 `observed_auxiliary_counts != [0;3]` 拒绝；新增 `ReplayBucketAux`
  `{decorator_diff, stairs_diff}` 与 `ReplayBuffer.aux` 并行队列，`push/pop_front_bucket`
  同步；`process_history_cursor_commands` 从 History 两 Arc 克隆随桶携带。
- 新增 `replay_color` / `replay_decorator` 消费系统（推进 timed/instant 并写
  `ColorHistoryReplayApply` / `DecoratorHistoryReplayApply` 事件交给实际 owner；
  **消费者本体登记 U**——crate 内无颜色/装饰应用写回面）。
- 调度谓词 `ReplayCommand::can_merge_with`（fresh switch 0,4,5,6,7,8,10）落地 + 测试。
  **登记 U**：原生按谓词把命令切成多个 ReplayBucket + `to_arc_slice` 装饰 cull
  allow-list（filter closure 未反编译）；当前仍按每条 history entry 一桶。
- Terrain(0) 生成器需要 `TerrainEdit` 点集载荷（native `+0x20/+0x28`），
  crate `deferred_terrain_history::TerrainEdit` 无点集 → 保持 UnrestoredReplayOwner 登记。

## 3. replay_mode 0x140a7c550

fresh 逐段（decomp + r2 0x140a7c8f0 段）：
- AppMode != Replay：进入标志（DAT_143634d88）置位则 log + 清标志，return。
- AppMode == Replay：首次 log + 置标志；**每帧** `clearing_canvas_w_layer_and_sound`。
- 输入覆盖 `input byte0 bit0` → `speed = input f32@+4`，return。
- 未 fast：`accrued = (input[+0x80]<3 || input[+0x88]<3) ? hold + dt : 0`；
  `fast = 0.2 < accrued`（UCOMISS+seta，NaN false）；`hold = fast ? 1.0 : accrued`；
  fast → speed=1.0；否则 speed = NaN-aware max(speed, 0.5*(pending_len+1-(replay_empty?1:0))²)。
- 已 fast：`input[+0x80]<3` → 焦点事件 0x03000000 + lerp 到 6.0；否则
  `input[+0x88]>=3` → 事件 0x03000200 + speed=1.0 + hold 清零 + max 公式；
  否则事件 0x03000100 + lerp。
- `time_continuous_lerp(a,b,rate,dt)`（0x140c904e0 反汇编）=
  `a + (b-a)*(1-exp(-rate*dt))`；调用常数 6.0/0.2。

**Rust 落地**：`ReplayFastHoldInput{override_speed,speed_override,primary,secondary}`、
`time_continuous_lerp`、`nan_aware_max`、`ReplayFocusEvent`、每帧 clear、三事件分支、
NaN-aware max 公式；8 个测试。**登记 U**：最后一条 pending HistoryCmd（native 0x28B
记录）→ 恢复 AppMode 事件 + speed=1.0/hold 复位的整段（crate `HistoryCommand` 为 4B
模型，无法承载 +0x10 子 tag 与 payload）。

## 4. finish_replay 0x1408b36a0

- 顶层只看 `*AppMode == 0xd` → 写回 `RestoreAppTo`（16B）。
- toolbar：`tag-6 < 0xf`（tag∈6..=20）取 `tag-5`；`==0xf`（tag 20）或 `0`
  （tag∉6..=20）→ 直写 toolbar `+0x18=7`；否则 `Toolbar::switch_mode`。
- MetaCommander 队首 `!= -0x7fffffffffffffff` → return；否则 log
  `"History replay has naturally finished. Unblocking game commander."` + pop。
- 队列长度检查只门控内联清理循环（0x140ac1576 两路 VecDeque 迭代），**不门控
  AppMode 写回**——旧实现"双队列空"条件已纠正。
- **登记 U**：内联清理循环（stride 0x10、byte+9 终止标记）语义未解；`CmdUnfreezeSheep`
  属 sheep 域无 body。

## 5. closest_resolved_snapshot + impl$9::try_from

fresh 0x140a84260：`out[0] = (*(u64*)(*in_RDX + 0x10) < 2)`；resolved 时 out 带
payload/chain_index。0x140a84a50 先 filter_map 再二分最近者（平局取后者）。
Rust：新增 `HistoryChainSnapshotPayload` trait（`resolution_state()`/`is_resolved()`，
上限常量 `RESOLVED_SNAPSHOT_STATE_LIMIT = 2`），实现 `()` 与 `u64`；`resolve_snapshot`
+ `closest_resolved_snapshot` 改为谓词内建，删除调用方 `is_resolved` 参数；测试覆盖
0/1/<2、2/MAX、别名与最近者选择。

## 6. history_process 0x140af5b30 第二事件流

- 16B 记录 `switch(_Src[9])` 跳表 0x142ae5420（26 项 self-relative 解码）：
  case 0/1/2/3 判定队尾 HistoryCmd byte1 并清队/`make_animations_instant`（已判等，
  现有 2 测试）。
- case 4..0x14：与 `replay_edit` 内联调度同一谓词体（0x140af64b0 起内层命令 tag
  分派 0..10，r2 原始字节解码）：`replay_schedule` 规则已落为
  `ReplayCommand::can_merge_with`（0→0 域/undo 相等、非 4/8；4→0；5→4；6→6；
  7→{7,10}；8→tag&7==0；10→7）。
- case 0x15..0x17：跳表项落在命令移除+按 tag drop 的内联块（0x140af7d4c 起），
  Ghidra 跳表解码在 15+ 与另一 switch 别名，**未解** → 登记（解锁条件：ReVa
  High P-code/签名修复或 0x140af7c00–0x140af7e80 原始字节逐块重建）。
- 其余：每记录按 @+0xb 写事件（==3 跳过）与默认压入 @+8 u32 已判等。

## 7. edit 载荷 5 个

| 载荷 | fresh 结论 | Rust |
|---|---|---|
| ColorEdit 0x14003c5d0 | target 判别 `[edit+8]`，15 项跳表 0x14292c628，target 3..=16；键表 0x14292c11e；载荷表见下 | **重写**为 14 变体 enum + ColorTarget(3..=16) + 手写 serde |
| DecoratorHistoryEdit 0x1404f46e0 | 单 entry map `{"diffs": HashMap<DecoratorStorageAddr, DecoratorStorageHistoryDiff>}`（键 "diffs" @0x1429ec6c6 len5） | **重写**为 `{diffs}`；内层 addr wire U（占位编码 `wall:<id>/terrain/stair`，登记） |
| RasterEdit 0x1404896f0 / 0x1416e8510 | points(Vec<Vec3>)/is_additive(bool)/brush_size(f32)/blend_mode(StrokeBlendMode)；From 0x140a976b0 | 现有实现**判等** |
| TreeEditData 0x14003d4e0 | 6 entry：position/scale/rotation_degrees/id(i32)/prefab_name/is_preplaced | 键名修正 `"rot"→"rotation_degrees"`、`"prefab"→"prefab_name"` → **判等** |
| StairsHistoryEdit 0x140259af0 / 0x140a18ee0 | bit0 单边枚举 `{"SupportsDelete|SupportsRecreate":{"edge":StairGraphEdge}}` | 现有实现**判等**；生成器 duration 修正 0.1→**0.2** |

Color payload（跳表 index→类型，serialize_entry 单态逐项）：
- 3/4/5/9（Wall/Gate/Roof/Halftimber）：WallId + TerrainStrokeId×2
- 6 FlatRoof：WallId + FlatRoofStyle×2
- 7/8/13/15（Decorator/WindowLight/StairLadder/MagicColor）：DecoratorId + TerrainStrokeId×2
- 10/12（FlagBunting/ChimneySmoke）：DecoratorId + bool×2
- 11 Brightness：DecoratorId + f32×2
- 14 StairRailing：DecoratorId + StairRailingType×2
- 16 StairSupport：DecoratorId + StairSupportType×2
- 17（unreachable from visit_str）：空载荷 `{}`
- visit_str 还接受 len-16 `"DecoratorTerrain"` → Decorator。
- **登记**：`TerrainStrokeId` 以 `terrain_editing::StrokeId`（tg_terrain）对接（4B id 同概念）；
  `StairRailingType`/`StairSupportType` crate 缺失，暂以 u8 不透明持有；
  `FlatRoofStyle` 无 src serde，按其 12B 观察投影数组写 wire（U）。

## 8. replay_pop_bucket 0x140b38250

fresh 逐段 + Rust `replay_pop_bucket(...)`（纯函数，参数面全为真实协作者）：
1. 前桶命令非空 → 仅 `DecoratorArchivist::extract_diff`（结果 drop），不弹桶。
2. 命令空 → `pop_front` → aux decorator Arc → `DecoratorStorage::restore_decorators`
   （`reverse = bucket.undo`，登记方向源）→ aux stairs Arc → `StairsState::restore_backup`
   /`apply_diff(diff, undo)`/`iter_edges_and_stubs`/`make_backup`。
3. drop 桶 → `extract_diff` → `RaycastWorld::flush_colliders` → `ReplayBucketFinished`。
- **登记 U**：两张 IndexMap clear（`<DecoratorId,Vec<(DecoratorId,CompactDirection)>>`
  与 `<SubworldColliderHandle×2, StructuralGraphConnection>`，类型未建模）与 stairs
  `+0x178=0`（字段未锚定）未落地；`ReplayDiffExtractor` trait 适配泛型 archivist。
- 2 个新测试（非空路径不弹、空路径弹桶+aux 同步）。

## 9. 其余

- **get_param 0x140ae5830**：fresh 4 索引稀疏表取 4 组（类型名 `GameTimeResource`/
  `History`/`AssetSsboLibrary`/`ReplayBuffer`，缺失 panic "Resource requested by …"），
  输出 4×`{data, p58, p5c, flag, +0x1c}`；现有 `RasterReplayParams` 4 资源面
  **判等-限定**（载荷指针语义 U）。
- **history_finish_recording 0x1409dd790**：pending None 早退；三 owner 计数差 + entry
  标志为变更位；有变更 → extract_diff/extract_diff_and_make_backup →
  `add_decorator_diff` → `finish_input_recording`；末尾 `History+0x1b0=1`。
  Rust `history_finish_recording(...)` 实现（变更位以 diff 非空建模，计数 owner 与
  +0x1b0 登记 U）。测试：未录制/无变更/有变更三态。
  `add_decorator_diff` 扩为 `impl<S>`，并在 pending 上同步 aux counts（decorator-only
  diff 也能提交条目；建模登记）。
- **missing_prefab 0x1409e6210**：`"MISSING_TREE_PREFAB_NAME"`（len 0x18）——已判等。
- **is_default_chimney_smoke 0x1408c5680**：fresh 为
  `walls::inner_wall_state::StructureType::is_fence`（4B 空体，`return;`）——**VA 错位/
  地址复用**，本任务不建；登记待核。
- 未改：History::add tag-7 校验、send_state_diff_events/add_decorator_diff/convert/from、
  replay_stairs（roofs+stairs agent 已修 undo 源）。

## 10. 登记（解锁条件）

1. Color `StairRailingType`/`StairSupportType`（decorator 域导出两枚举后替换 u8）；
   `TerrainStrokeId` 正式类型；`FlatRoofStyle` serde。
2. `DecoratorStorageAddr` 的 wire serde（decorator/walls 域）；当前占位地址键。
3. wall process op5..op16 载荷（MorphPremadeShape/ChangeElevation/ChangeArchHeight/
   ChangeSupports/ChangeStairsType/ChangePlasterPeelAmount/ChangeFreehandHeightType/
   ChangeArchType/MorphFreehandWall/DeprecatedMorphOriginal/ChangeSkirt）——walls 域
   `WallHistoryEdit` 变体载荷；在此之前 instant 返回 UnrestoredOperation。
4. wall timed 命令的 duration 来源（生成器 tag-7 体未逐指令解，src 沿用 0.4）。
5. `replay_edit` 多桶调度 + `to_arc_slice` 装饰 cull allow-list（filter closure 未解）。
6. replay_mode 的 HistoryCmd 0x28B → AppMode 恢复段；原生清理循环（finish_replay）。
7. history_process 第二流 case 0x15..0x17（移除命令）与 case 4..0x14 的原生记录侧
   语义（谓词已解，记录驱动未接线）。
8. `TerrainEdit` 点集载荷（terrain 域）→ tag 0 桶展开。
9. replay_pop_bucket 的两 IndexMap clear 与 stairs `+0x178` 字段。
10. 颜色/装饰 replay 消费者本体（颜色 owner / DecoratorStorage 应用面）。
