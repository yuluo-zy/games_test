# terrain-SPEC — terrain UI/new-stroke 输入生产者 + history/replay 生命周期 + 订阅链（C 世界，2026-10-15 第二轮）

> 历史分析结果，供同组实时复合 agent 直接落地。已实现边界见 `evidence/core-recovery-20261015/terrain/REPORT.md`（draw/edit/delayed readback、RecalculateHeightMapCmd、PlayTerrainSound 事件均已就位）。
> 证据源：`evidence/terrain-editing/B-core/*.c|*.asm.txt`、`evidence/country-core-deep/terrain-water/decomp/`、`evidence/country-core-deep/ui-input/decomp/`、章节 `chapters/systems/terrain/*.md`、`chapters/resources/history/terrain_edit.rs.md`、sysres v2。
> **跨组文件**：`resources/history/*`、`systems/history.rs` 是共享文件——TerrainEdit/EditType 变体接入须先经中央仲裁。

## 状态判定

- **可直接落地**：`ui_draw_terrain`（0x140a9fdc0，decomp 已读）；`deferred_terrain_history`（0x140ac9770，decomp 已读）；`terrain_hover_idx`（0x140aff720，decomp 已读）；`ui_edit_terrain`（0x140aef1f0，decomp 已读，事件面部分）；`replay_terrain`（0x140995150 + forward/backward）。
- **需先取证**：`ui_edit_terrain` 的完整 UI 状态机（内部状态名 U，ui-input 域 owns）；`ui_terrain_focus_mode`（0x1409a4fb0，167KB 近全内联，状态名 U）；audio 播放（属 audio owner）；原全局调度（plugin 资源初始化）。

---

## 1. terrain UI/new-stroke 输入生产者

### 1.1 `ui_draw_terrain` 0x140a9fdc0（O，decomp 已读，可直接落地）

SystemParam（章节）：`Res<AppMode, CursorTerrainRaycast, AssetSsboLibrary, WaterRaster, UiState, GladeBorder>` + `ResMut<TerrainEditingData, PendingTerrainHistory, DecoratorArchivist>` + 事件组。

控制流（decomp 全文 168 行）：
- **门控**：`*(int *)*param_1 == 10`（AppMode 值 10）才运行，否则 return。当前源码无 AppMode owner——落地需真实 AppMode 资源（input/backend 域）。
- 读 `TerrainEditingData`（param_4，lVar12）：stroke 相关字段在 `lVar12+0x20/+0x28`（active 判别 @+0x298 字节）；`*(char *)(lVar12+0x298) == '\0'` → 用 +0x8/+0x10 分支。
- 读 `CursorTerrainRaycast`（pbVar5）：`(*pbVar5 & 1)` 判别；位置在 `pbVar5+4/+0xc`。
- **光标高度**：`FUN_140a14df0(&cursor, x, y, _UNK_142a7b658, 0x43020000)` → fVar16（0x43020000 = 192.0f）；`fVar16 <= 0.0` → 跳过（界外）；
- **界外反馈**：`FUN_14092df50(&事件, &UNK_142adc2df, 0x11, 0, 1, &uStack_128)` —— **0x11=17 字节串 @0x142adc2df**（"DisplayOutOfBorder" 或同类 UI 事件）；后续 `FUN_14092df50(…, &UNK_142adc2f0, 0xc, 3, 2, …)`——**0xc=12 字节串 @0x142adc2f0**（FeedbackInputOutOfBorder 或同类）。这两个串的确切文本**待取证**（decomp 未显示文本，仅长度）。
- `func_0x000140c97540(auStack_124, x, y, _UNK_142a16cb8)` —— 光标→地形点换算；
- `FUN_140ad2970(uVar2, &uStack_128)` → bVar8 = **是否点按命中**（bool）；命中 → push 事件（8B 载荷，tag 由事件队列 `+0x18/+0x38/+0x19` 判别区决定：`*(char*)(lVar12+0x19) == 3` 分支写 `*in_stack_00000070 = *(pbVar5+8)`——把 **stroke id 写入输出**）；
- `uVar11` 状态字节（`*in_stack_00000068 = (char)uVar11`）→ 结尾间接跳转（`UNK_142adc498` 表，多分支——**状态机表，分支数待取证**）。
- 事件面（decomp 内联 grow 显示）：`EventInstance<WindowFocusEvent>`、`EventInstance<…WriteSettingsFileCmd>` 等（grow_one 单态名揭示）。**new_stroke + EvTerrainStrokeChanged** 的精确发射点在状态机分支内（章节结论）。

### 1.2 `ui_edit_terrain` 0x140aef1f0（O，decomp 已读）

SystemParam 事件面丰富（decomp 内联 grow 单态揭示）：
- 门控 `*(int *)*in_RCX == 10`（同 AppMode 10）；
- 读 TerrainEditingData / 地形高度（`uVar58/uVar63` 区间的四角裁剪）、光标；
- **地形笔刷 UI 状态机**（switch bVar42 0..4 + default）：case 1 写 `EventInstance<WindowFocusEvent>`（8B）+ 写 EditTerrainStroke 载荷（`*puVar26/*puVar25` = 参数事件，含 `+0x10`/`+0x14` 载荷、`*pfVar22 = 0.0` 清零累计）；case 2 写 `EventInstance<WriteSettingsFileCmd>`（8B）+ **台阶放置数据 push**（`indexmap::Bucket<(DecoratorId,DecoratorId), AlignKind>`，0x28B 元素：tag@0、float@+4、对齐字节@+8、双 DecoratorId@+0xc..、`+0x1c` 颜色/角、`+0x20` 计数）——这是 **terrain→stairs/decorator 消费链**；case 3 写两个事件（WindowFocusEvent 双 push，第二个负载=0）+ `EditTerrainStrokeCmd`（`*local_118 = 0x8000000000000001` 哨兵判别）；case 4 间接跳转表；case 6/8/0xc/0xf panic/断言。
- **UI 文本格式化**：`alloc::fmt::format_inner` + `core::fmt::num::imp::impl$6::fmt`（0x42ae3a08 区格式化常量）；"Requested terrain stroke {id}" panic 串族。
- **`get_last_mut_stroke` 0x140924900 / `new_stroke` 0x140924a10**（resources/terrain_editing.rs，sysres v2）——**new_stroke 生产者的核心**。当前 `resources/terrain_editing.rs` 只包 tg_terrain owner，**缺这两个方法**。`TerrainStroke::insert_or_morph_last_point 0x140a97880`（resources/terrain_stroke.rs）也是输入生产者必需。
- **日志**：`resources::terrain_editing::impl$11::fmt`（@0x142ae3aa0 区，"…terrain stroke" 族）。

### 1.3 `terrain_hover_idx` 0x140aff720（O，decomp 已读，可直接落地）

- 门控：`*(uint *)*param_1 < 0xb` 且位测 `0x421U >> (mode & 0x1f) & 1`（模式集合 = {0,5,9} 附近，位掩码 0x421=0b10000100001）；
- `FUN_140969720(…, &UNK_142c2e408)` 取地形资源（串名 == `&UNK_142a9df5b`（0x6c 长度）才继续——**串内容待取证**）；
- `iVar11 = *(int *)(*plVar9 + 0x10)` = **hovered stroke id**（经 vtable `(**(code **)(puVar8[1]+0x18))` 取高度）；
- 光标有效性：`fVar12 = FUN_140a14df0(&cursor, x, y, _UNK_142a7b658, 0x43020000)`；`fVar12 <= 0.0` → return（未命中）；`*(float *)(pbVar4 + 8) <= _UNK_142925870` → return（界外/阈值）；
- 命中 → `*param_3 = 1; param_3[1] = iVar11`（写入 Option<i32> 输出）。
- **输出形状**：`Option<i32>`（哨兵 -1 表示无 hover）。

### 1.4 `mouse_read_terrain_stroke_index` 0x1409768b0（GPU 域，后置）

读 GPU 读回的地形 stroke 索引（`FUN_1400dd6f0(param_2+0x38, …)` hash 查找 + `lVar3 = ~uVar5*0x60` 条目定位），返回 stroke id。**GPU 执行后置**，但 CPU 侧消费接口（Option<stroke id>）可留真接口。

### 1.5 `ui_terrain_focus_mode` 0x1409a4fb0（O 入口，状态机 U）

167KB 近全内联（alloc/memcpy + HashMap helper 0x140c9cde0×9）；闭包 4 个 O 地址：0x9a3620（FUN_140c97e10 曲线采样，0.5 参数）/0x9a8640/0x9a88c0/0x9a8b40（状态转换，OutlineWallCmd/PublicWalls 相关）。内部状态名 U（ui-input 域 owns）。**落地**：先只实现入口骨架 + 注册，不实现内联状态机（待取证）。

---

## 2. terrain history/replay 生命周期

### 2.1 record 接线：`deferred_terrain_history::history_record_terrain` 0x140ac9770（O，decomp 已读，可直接落地）

```
1. 读当前 stroke（Option，哨兵 &DAT_8000000000000002 = None）
2. None → return；Some → 构造 TerrainEdit（判别 0=NewStroke，写入 stroke id）
3. History::add（第 3 参 = track-caller 文件）
4. push 事件 EventInstance<WriteSettingsFileCmd>（8B）
```

**TerrainEdit 契约**（`chapters/resources/history/terrain_edit.rs.md`，O）：
- 判别（Debug::fmt 0x140858c80）：首字段 niche `x^0x8000000000000000 < 2` → 变体 0/1/2（0=NewStroke（+4 处 stroke id）、1=+1 处 0x11B 载荷、2=+4 处 0x10B 载荷）。
- `revert 0x140858d30` / `reapply 0x1408592d0`（对称）：判别 0（NewStroke）且 `param_6 < 0.99f`（静态 0x142a948d8 = 动画进度阈值）→ panic "Requested terrain stroke {id}"（terrain_edit.rs，@0x142a94cf8）；否则 hashbrown 查 48B/元素 stroke 表（FUN_1405b0ed0），命中 → 清/置该 stroke active 标志（+0x28）。
- `duration`（内联于 TerrainReplayCmd::forward）：`clamp(Σ相邻点距*0.01, 0.4, 3.0)`（cm→m），无点默认 0.3；"assertion failed: stroke_length.is_finite()"（@0x142a94c34）。
- `update_terrain_curve 0x140859740`：hash 查 48B stroke 表 → 拷贝 12B 点集 → FUN_141469230 fold 求进度 → 与全局时间 FUN_140c95ef0 (&0x142a94d48) 做 `(1-t)*u + t` lerp → 0..1 范围断言（"Tried to init CurveU from an invalid u: "）→ 回写。
- serde：serialize 0x1404f41b0 / 0x1417c1590、deserialize 0x1417c1ac0、visit_str 0x1417c23e0。

**跨组注意**：`History::add`（0x140a84ec0）与 EditType 定义在共享 `resources/history/*`。Terrain 需**新增 EditType::Terrain 变体**（原 tag 待取证——现有 1..8、13；terrain 应在 9..12 之一），**必须经中央仲裁**。

### 2.2 replay 接线：`replay_terrain` 0x140995150（O，可直接落地）

- 门控：`active(+0x18)==0 → return`；speed = ReplayBuffer+0x28；dt = Time+0x28；
- 前桶 cmds（ptr@+0x08, len@+0x10）0x30 步长迭代，**仅 tag 3**；载荷 = TerrainEdit（niche @+0x10 == 0x8000000000000000 时变体字段在 +0x18）；
- **instant 路径**（`duration(+0x10)==0.0 且非 NaN 且 undo(+0x1c)==0`）：拷贝点集（12B/点，断言点数 < 0x3fffff @0x142aae990）→ 更新地形 stroke 计数（+0x38 = max(索引+1, 原值)）→ FUN_141526d10 直接插入 TerrainEditingData（+0x8 起点）→ 释放拷贝；构造默认命令 {ptr, 0.4f, 2, stroke_id}（0x3ecccccd = 0.4f）；
- **动画路径**：`t = clamp(elapsed/duration, 0, 1)`，`eased = 1-(1-t)^3`（ease_out_cubic；0.4 为 clamp 下限 @0x1429cd060、1.0 @0x142925904）；`undo==0 → reapply 0x1408592d0(payload+0x10, …, eased)`，`!=0 → revert 0x140858d30`；`eased >= 1.0` 时 +0x18 计数--，<0 → 完成入列表；
- `elapsed += dt*speed`（+0x10）；
- 收尾同通用骨架（完成索引逆序移除：边界断言 → memmove 压缩 → 按判别字节 drop 分派，tag 3 → FUN_140b5c2a0）。
- `TerrainReplayCmd::forward 0x140994f30 / backward 0x140995040`（同构 1533B）：duration = clamp(Σ点距*0.01, 0.4, 3.0)（无点 0.3）；"assertion failed: stroke_length.is_finite()"（@0x142a94c34）；输出 {data ptr, +0x8=0, duration f32@+0xc, +0x10=2}。

**共享文件注意**：ReplayBuffer/ReplayCmd 在 `resources/history/replay_cmd_buffer.rs`（已存在，供 replay_auto_clutter 使用）。terrain replay 需 ReplayCmd tag 3 的判别——**经中央仲裁**。

### 2.3 待取证（terrain）

- EditType::Terrain 的原 tag（9..12 之一，待证据）。
- `history_record_terrain` 推的 `WriteSettingsFileCmd` 事件的确切作用（可能是进度持久化触发，I）。

---

## 3. audio 播放消费者（属 audio owner，给出接线建议）

`PlayTerrainSound` tags 0..4（已实现事件）：
- **`terrain_audio_init_system 0x1414613e0` / `terrain_audio_system 0x1414618c0`**（systems/audio/terrain_audio.rs，sysres v2）——读 PlayTerrainSound 事件，播放地形音效。
- `terrain_stairs_audio_system 0x141462dc0`（systems/audio/terrain_stairs_audio.rs）——台阶音效。
- **接线建议**：terrain 域只负责发射 PlayTerrainSound（已实现）；audio owner（B 组 music 域邻近）注册消费系统。不实现音频本体。

---

## 4. 原全局调度 / plugin 资源初始化

- **`startup_terrain` 0x140926bf0**（startup 域，O）：载入 terrain/rocky_terrain/stairs_step/path_pebble/stairs_pebble/billboard_plants 网格（8× load_mesh_into_library）+ 6 shader + 调 grid_gen_cw（`grid cw`）+ spawn 地形实体。
- 原 plugin 资源初始化与全局调度（Update 系统注册顺序）：**待取证**（当前 `add_terrain_cpu_systems` 为本地顺序，非原版调度）。
- `OnTerrainChange` 下游订阅链：wall/collision/scatter 消费者（`mesh_follow_terrain 0x140a68430`、碰撞域等）——**分属其他域，给出事件/资源接线建议即可**：delayed_terrain_changed_msg（已实现）发射 OnTerrainChange，下游域注册消费。

## 5. 真实消费者接线清单

| 生产者 | 消费者 | 状态 |
|---|---|---|
| ui_draw_terrain 0x140a9fdc0 | TerrainEditingData::new_stroke + EvTerrainStrokeChanged（已实现）| 未接线 |
| ui_edit_terrain 0x140aef1f0 | EditTerrainStrokeCmd / SetTerrainStrokePtsCmd（已实现）+ stairs/decorator 放置事件 | 未接线 |
| deferred_terrain_history 0x140ac9770 | History::add（TerrainEdit，共享文件需仲裁）| 未接线 |
| replay_terrain 0x140995150 | TerrainEditingData + RecalculateHeightMapCmd | 未接线 |
| terrain_hover_idx 0x140aff720 | Option<i32> hover 输出 → UI 状态 | 未接线 |
| edit/draw 已实现系统 | PlayTerrainSound（0..4）| 事件已实现，audio owner 未注册 |

## 6. 明确排除项

- GPU 执行（rasterize_terrain_stroke、read_terrain_data 等 compute pass）后置。
- 不实现音频本体（audio owner）。
- 不改 `resources/history/*` / `systems/history.rs` 直到中央仲裁批准。
- `ui_terrain_focus_mode` 状态机、`ui_edit_terrain` 完整 UI 状态表未取证前不实现内联细节。
