# 第 10 轮 G2③ — 「回放消费者 + 读档残留」增量评审

日期：2026-10-19（评审快照约 13:20，worktree 为 ② 并发编辑中的状态）
评审对象：
- 契约 `evidence/round10/contracts/g2-replay-consumers-CONTRACT.md`
- 交付 `evidence/round10/deliveries/g2-replay-consumers-DELIVERY.md`（13:18 落盘）
- 源码 `D:/game/tiny-glade/crates/country-core/{systems/history.rs,systems/history/**,resources/history/**,systems/save/snapshot.rs}`、`crates/systems/tree-tool/**`

并发说明：评审期间 ② 多次改写（replay_color 13:15、replay_decorator 13:15、decorator_edit 13:16、history/tests 13:17），
13:18:52 后 G1 新增的 `startup/clearing.rs` 等文件使 **test target 编译失败**（E0277/E0560，属于 G1 文件，不是 G2）。
因此本报告区分「G2 最后一次可编译实测」与「最终快照」两种证据，不把 G1 的编译中断算到 G2 头上。

---

## 1. 结论

G2 的语义方向已按契约 C1–C4、G1 修正，四类 consumer 都写真实资源，`progress`/easing 已删除，
Pop 只有一条生产路径且顺序正确。**但仍有 1 个破坏性 Pop 错误路径、1 个可 panic 的颜色写回分支、
2 个未完成的读档残留接线，以及交付数字/缺口登记偏差**，不能判定为完整闭合。

- C1 颜色节奏：**通过**（elapsed==0 一次性派发、离散 from/to、delta 不乘 speed、floor 0.1、严格 `<`）。
- C2 装饰职责：**通过**（只做 cull-allow take 注入 + tag-7 计时 floor 0.05；不再写 DecoratorStorage；Pop 恢复）。
- C3 墙颜色分流：**通过**（special→GateColorIds / normal→WallColorIds），但直接写表为 partial。
- C4 楼梯音频：**载荷组装通过，消费链不完整**（kind 双路径正确；u30/flag 写入事件但唯一 reader 不读）。
- G1 非 native Pop：**结构通过**（CPU Pop 已实现并替换 pop_empty），但 native Pop 的失败路径会先弹桶再报错。
- G2 读档残留：**RoofAssociations 代码已接但读档顺序下实际走 gap；树 sentinel 无解析路径；OnTerrainChange 仍无 prefab reader；Clutter to_info 已修**。

**水分计数：2**
- W1：楼梯 u30/+0x30 仅写入事件，`stairs_audio_system` 不读取；交付 §5 写成「已按机器码写」，未登记下游无效（属于“载荷到位、效果未消费”的过度表述）。
- W2：`ColorEdit::FlatRoof` 在 `apply_color_replay` 是静默 no-op，交付表格提了“不写 CPU 色表”，但 §8 仍缺项未把它列为丢失/未接线，属于缺口登记不完整。

（不计水分但必须修：native Pop 先弹后验、颜色 consumer 的 variant panic。二者是 bug，不是虚报。）

---

## 2. 逐项审查表

| # | 审查项 | 证据（评审时行号） | 结论 |
|---|---|---|---|
| 1 | consumer 真写回，无 TODO/log/默认值冒充 | 颜色 → `WallColorIds/GateColorIds/RoofColorIds/HalftimberColorIds`、`DecoratorStorage`（`replay_color.rs:100-277`）；墙 → 两张色表（`replay_wall.rs:50-76`）+ `PlayWallAudio`（`:79-92`）；楼梯 → `PlayStairsAudioCmd`（`replay_stairs.rs:84-101`） | **基本通过**；见 W1/W2 与 M2 |
| 1a | `undo` 真参与计算 | `pick(undo, from, to)`（`replay_color.rs:88-94`），所有 target 一致；墙 `WallRestoreColorRequest` 值由 interpreter 的 undo 选定 | 通过 |
| 1b | `progress` 已删除/不参与 | 三文件 grep 无 `progress`/`cubic_in_out`/`lerp` | 通过 |
| 2 | 回放链真实顺序、无重复 pop | `history.rs:668-755`：Cursor(`history_process_system` → native `history_clear_previous_state.after(process)`) → Replay(各 replay + 三 consumer) → Pop(`replay_pop_bucket_system` 或 `replay_pop_bucket_cpu_system`) → `finish_replay.after(pop)` → Record；`pop_empty_replay_bucket` 只出现在测试 | **顺序通过**；native Pop 失败语义见 M1 |
| 3a | 颜色 consumer 目标真实存在 | `resources/color_id.rs:31/59/87/115` 四表均为预分配 `MAX_NUMBER_OF_WALLS`；`DecoratorStorage::lookup_decorator_info_mut` 存在（`decorator_storage.rs:566`） | 通过；窗口/旗/烟囱分支见 M2 |
| 3b | 装饰 consumer 职责 | `replay_decorator.rs:35-76`：`aux.front_mut().cull_allow.take()` → `allowed_ids.insert`；只处理 `ReplayCommand::Decorator`；instant 移除；timed `max(duration/speed,0.05)`、`required < elapsed`、`elapsed += delta`；逆序 remove。无存储写入 | 通过 |
| 3c | 墙颜色分流 | `replay_wall.rs:233-237` 用 `inner.get_unwrapped(...).is_special()` 覆盖；`:56-60` special→GateColorIds，normal→WallColorIds；`is_special`=`source+65 != 0`（`inner_wall_state.rs:222-224`） | 通过（plaster/sound/SSBO 为 partial，已声明） |
| 3d | 楼梯音频 | `replay_stairs.rs:84-101`：instant `0x12-undo`，animated `0x11+undo`；position=`CachedDecoratorTransforms[edge1]`，未命中 0；`tail[0..2]=u30` 低/高 u32；`flag=1`。唯一 reader `playermade_stairs_audio.rs:100-125` 只用 `kind/position/values[0]` | **载荷通过，消费不完整（W1）** |
| 4a | `RoofAssociations::rebuild` | `snapshot.rs:1591-1626`：rows=`Roof.wall_id`+`PublicWalls.get().building_projection()`，order=`wall_ids()`，phase=0，marked=false；`:1640-1660` 消费 `DeleteRoofCmd`/`WallRoofPresenceChanged` | **调用与消费真实**；但 load 顺序 `Walls→Roofs` 时 `PublicWalls` 尚未从载入墙发布，样本会走缺行 gap（交付 §6.1 已承认）。无测试断言 `report.roof_associations` |
| 4b | 空 prefab 树回退 | `snapshot.rs:1810-1816` 写 sentinel 进 `SpawnPrefabCmd`；解析仅在 tree-tool `process_tree_cmds.rs:189` 的 InsertTreeCmd 分支；`tree_tool::lib.rs:81 build` 无任何生产调用者；snapshot 发送的是 `SpawnPrefabCmd`（`:1818-1838`）而非 InsertTreeCmd | **不通过**：sentinel 被写出但没有可达的解析者；交付 §6.2/§8.7 只写“host 未接”，低估了路径错配 |
| 4c | `OnTerrainChange` consumer | writer `delayed_terrain_changed_msg.rs:13-33`；无 `systems::prefab::update_prefabs` reader；tree-tool 有 `recalc_tree_position_on_terrain_height_change.rs:19-46` 但 `build` 未装配 | **不通过/诚实登记**：交付 §6.3 与 §8.8 已登记 prefab reader 缺口；tree-tool reader 未注册未提 |
| 4d | Clutter `to_info` | `decorator_edit.rs:124-145`：`ty<0x38`/未知保留 `u30/u38/u3c/u40`，style 回读 `self.style`；新增 round-trip 测试通过（`clutter_state_round_trips...`）；旧 0x3e 期望已于 13:16 改为 0 | **通过（typed gap 仍在）**；缺 36 样本字节级 round-trip（契约清单 12） |
| 5 | 编译/测试 | 见 §4 实测 | G2 最后可编译快照 check/历史/存档通过；当前 lib+test 均被 G1 startup 文件阻断 |
| 6 | 缺口继承诚实性 | 交付 §8 九项基本真实；偏差见 W1/W2、§6.1 的“已接”措辞、§7 测试数字 | **部分通过** |

---

## 3. 实际调用链（评审快照）

```
[Cursor] history_process_system
         └─(native) history_clear_previous_state.after(process)
            只清 DecoratorArchivist.culling.allowed_ids（不是 Pop）
[Replay] replay_stairs / replay_auto_clutter / replay_wall / replay_rasters /
         replay_color / replay_decorator            // 只操作 front bucket + aux
         apply_color_replay  .after(replay_color)   // 写四色表 + DecoratorStorage
         apply_wall_replay_color / apply_wall_replay_audio .after(replay_wall)
         replay_mode
[Pop]    native-physics: replay_pop_bucket_system
         非 native:     replay_pop_bucket_cpu_system
         两者都只在 front.commands.is_empty() 时弹一个桶；decorator aux →
         restore_decorators(reverse=undo)；CPU 额外恢复 stairs aux；
         extract_diff + flush_colliders；发 ReplayBucketFinished
[Pop]    finish_replay.after(pop)                  // 不弹桶
[Record] record_roof_* / record_wall_history chain
```

- 每个构建只注册一条 Pop，`pop_empty_replay_bucket` 不在生产 Schedule：**不重复 pop**。
- Replay consumer 在 Pop 之前：符合原版“front bucket 命令空后才弹”。
- 发现：`ReplayBuffer.aux.cull_allow` 生产 producer 恒为 `None`（`history.rs:283-288`），
  注入链路只有测试可达；交付 §8.2 已诚实登记。

---

## 4. 回归实测（原始输出摘要）

最后可编译的 G2 快照（约 13:16，decorator_edit 修复前）：

| 命令 | 结果 |
|---|---|
| `cargo check -p country-core` | ✅ `Finished dev profile ... in 1m 11s`（仅 warning） |
| `cargo check -p tiny-glade` | ✅ `Finished dev profile ... in 1m 23s`（仅 warning） |
| `cargo test -p country-core --lib systems::save` | ✅ **49 passed; 0 failed**（0.85s） |
| `cargo test -p country-core --lib systems::history` | ✅ **59 passed; 0 failed**（13:16 复跑） |
| `cargo test -p country-core --lib` | ❌ **2508 passed; 1 failed**（13:16，`decorator_edit::tests::to_info_type_selects_default_extras_and_live_state`，0x3e 期望 3） |

13:16:33 已把该测试 `0x3e` 期望改为 0（`decorator_edit.rs:1042-1055`），但评审未能在其后再取得一次全绿的全库输出：
13:18:52 后 G1 新文件导致编译失败，且范围从 test target 扩大到 lib：
- `startup/spawn_preplaced_trees.rs:114-116`、`startup/mod.rs:157-159`：`Transform` 无字段 `t/r/s`（E0560）；
- test target 另有 `startup/clearing.rs:91` E0277 `(&str, Option<&str>)`/`(&&str, Option<&str>)` 比较。
- 13:23 复跑 lib：`error: could not compile country-core (lib) due to 6 previous errors`；test target 7 errors。
- 错误位置全部在 G1 文件，**不归因 G2**；G2 最后可编译快照的 check/test 数字见上表。

交付 §7 写「2509 passed / 0 failed」：截至评审快照**无法独立复核**（2509 是总数或含一次修复前的失败），
必须在 G1 恢复 test 编译后复跑并留下输出；在此之前该数字视为待复核而非已证实。

中途状态（13:10）曾出现 7 个 history 失败（颜色/装饰/墙 consumer 测试未接 Pop），② 在 13:15 给 fixture 补
`pop_empty_replay_bucket` 后全部转绿；说明最终测试确实执行，不是配置未跑。

---

## 5. 必须修复清单

1. **native Pop 先弹后验（破坏性错误路径）** — `history.rs:444` 已 `pop_front_bucket()`，`
   :445-451` 才检查 decorator diff 需要的 `DecoratorStorage/DecoratorIdGen`；缺资源时返回 Err 但桶/diff 已丢。
   CPU 路径（`:540-560`）是先验证后弹，应统一；补测试“缺 owner 返回 Err 且 front bucket 仍在”。
2. **颜色 consumer 的 variant panic** — `replay_color.rs:161-165`（`as_window_mut`）、`:191-192`（`as_flag_mut`）、
   `:205-206`（`as_chimney_mut`）在 `DecoratorInfo.extra()` 变体不匹配时 panic（`decorator.rs:341-360`）。
   原版 `set_*` 是按固定偏移写字段，不依赖判别；应改为无 panic 的字段级写入或 `try_as_*`，并加“错误 disc”测试。
3. **全库测试必须复跑至 0 failed** — G1 恢复编译后执行
   `cargo test -p country-core --lib`，确认 `to_info 0x3e` 修复生效，并把真实数字写回交付 §7。
4. **树 sentinel 没有可达解析路径** — `snapshot.rs:1810-1816` 写入 `SpawnPrefabCmd`，而 sentinel 解析只在
   `process_tree_cmds.rs:189` 的 InsertTreeCmd 分支，且 `tree_tool::build`（`lib.rs:81`）无调用者。
   要么 snapshot 走 InsertTreeCmd/统一 load 编排，要么由宿主在发送 `SpawnPrefabCmd` 前解析；否则 prefab loader 会把
   `"MISSING_TREE_PREFAB_NAME"` 当真实 prefab 名。
5. **RoofAssociations 在真实读档顺序下不可达** — 需在 Roofs apply 前由载入墙发布 `PublicWalls`
   （或改用 `InnerWalls` 的 building projection），否则 36 样本恒走“缺行 gap”。补一条断言
   `report.roof_associations == report.roofs` 的样本测试。
6. **楼梯 u30/+0x30 的消费缺口** — 补 instant/animated × undo × decorator 命中/过滤/缺失的 `tail/flag` 测试
   （契约清单 7 要求 4 组合 + u30），并在交付 §8 明确 `stairs_audio_system` 目前不读 `tail/flag/bytes`，属下游 partial。
7. **FlatRoof 回放静默丢弃** — `replay_color.rs:140` 的 `ColorEdit::FlatRoof { .. } => {}` 让平屋顶样式 undo/redo
   无任何效果；契约允许交由屋顶 customization owner，但必须列入 §8 缺口并给出 `SetFlatRoofStyleCmd` 接线方向。
8. **36 样本装饰 round-trip** — 契约清单 12 要求真实 `.snapshot` 的
   `to_history_state → to_info → to_bytes` 逐字节校验；当前只有 1 条合成 Clutter 测试。
9. **交付措辞修正** — §6.1 “RoofAssociations 已接”应改为“代码路径已接，load 顺序下走 gap”；§5 不应暗示
   u30/flag 已生效；§7 数字以复跑为准。

---

## 6. 给 ①（契约/逆向）的下一步方向

1. **颜色 writer 的变体语义**：fresh 反编译 `set_window_light 0x142298260`、`set_flag_connection_n_chimney_smoke
   0x142298970`、`set_decorator_color_id 0x1422af0c0`，确认是否按固定 `DecoratorInfo+0x28/+0x34/+0x38` 无条件写；
   若如此，core consumer 不应因 extra disc 不同而 panic。
2. **0x140a20d70 与 0x140a20f44 的 index-6 分支**：把 `ty<0x38`、`0x3e`、`0x3f` 的逐槽映射和 style 回读定案，
   支撑刚改的 `to_info` 与 36 样本 round-trip。
3. **读档顺序**：核对 `init_game_state_from_snapshot 0x1417a9c50` 中 Walls 发布/`PublicWalls` 填充与
   `wall_roof_hashmap 0x140a94cd0` 的先后；若原版在 roofs 前先 publish，则 core apply 必须补该步。
4. **树载入路径**：确认原版 init 对 Trees 是发 InsertTreeCmd（经过 `process_tree_cmds` 的 sentinel 解析）
   还是直接 prefab spawn；据此决定 sentinel 的解析 owner，避免在 core 复制 `random_tree_prefab`。
5. **Pop 失败语义**：复核 `0x140b38250` 是否可能在弹桶后失败；Rust 模型必须验证前置资源，不得以 Err 换数据丢失。
6. **楼梯音频载荷消费**：取证 FMOD `PlayStairsAudioCmd` +0x28/+0x30 进入哪个参数；若当前音频模型无法承载，
   保留事件字段但登记 no-op，别把“写入事件”当成“产生音效差异”。
7. **墙色副作用 owner**：确认生产 `system_color` 的 `set_*` 读者是否已注册；若已注册，评估 core 直接写表与
   后续 `set_*` 的冲突/重复应用，决定事件命令化还是显式 partial。
