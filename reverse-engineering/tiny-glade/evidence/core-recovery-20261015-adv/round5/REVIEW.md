# 第五轮 ③ REVIEW（2026-10-16 基线 + 复审台账）

审查者：③ review agent。本轮三集中块：**A 调度器整合 / B sheep 写盘链 / C 快照应用+会话消费端**。
本文档 = 基线盘点（② 实现前）+ 逐块复审结论（② 交付后追加）。基准源码 = 第四轮末（整 crate 已编译通过）。

## 0. 本轮审查准则（协调器指令）

1. **孤儿系统是否真进生产 Schedule**（不是只定义）：逐个域登记入口查生产调用链，只定义/只测试/只 doc 引用 = 未进生产。
2. **Schedule 是否真可执行**：全部域系统注册后，注入资源是否能跑通（不因缺资源 panic）；宿主提供 Schedule 的前提成立。
3. **Sheep 节是否真写盘**（不是只定义函数）：从捕获 → `sheep_section_serialized` → `assemble_game_save_sections` → `AsyncSaveCmd::OpenFile` → save_thread 写盘的完整链路必须有真实生产调用点。
4. **apply_loaded_snapshot 是否逐域应用**：快照解析数据 → 各域（颜色/时间/羊/地形…）真实应用，不是空函数。
5. 对照原机器码给 ① 深挖方向；② 若引入回归（整 crate 编译/既有测试）立即标出。

## 1. 基线盘点（② 实现前，2026-10-16）

### A. 生产注册现状：孤儿系统清单

| 域 | 注册入口 | 生产调用点 | 状态 |
|---|---|---|---|
| ivy | `add_ivy_systems`（0x140944070）/ `add_ivy_events` / `add_ivy_cpu_systems` / `add_ivy_animation_systems` | 无（仅测试/注释） | **孤儿** |
| garden | `garden::build`（0x140b6d290，含 `register_garden_events`+`add_garden_cpu_systems`） | 无 | **孤儿** |
| wall | `register_wall_cpu_resources` / `add_wall_cpu_systems` | 无（wall/registration.rs:213/236 在 `#[test]` 内） | **孤儿** |
| input | `register_input_events` / `add_input_cpu_consumers` | 前者已接（run_game.rs）；后者无 | **input 系统孤儿** |
| audio | `add_audio_events` / `add_audio_systems` | 无（仅 music_audio_init_system 接） | **audio 系统孤儿**（garden_audio_system/water 均未挂） |
| signifier | `register_signifier_resources` / `add_signifier_systems` | 无 | **孤儿** |
| roof | `register_roof_events` | 经 register_history_events 接 ✓ | 已接 |
| ui_clutter | `register_ui_clutter_resources` / `add_ui_clutter_cpu_systems` | 经 add_history_cpu_systems 接 ✓ | 已接 |
| save/terrain/session/history | 各自注册 | run_game.rs register_core_startup 接 ✓ | 已接 |

→ **block A 待办**：ivy/garden/wall/input/audio/signifier 6 域的 add_*_systems 进入生产 Schedule（register_core_startup 或 add_game_startup_systems 28-系统序），并按 ① 契约定序。

### B. sheep 写盘链现状：**未写盘**

- `sheep_section_json`（control.rs L70）/ `sheep_section_serialized`（save_game.rs L396）：纯函数 + 测试，**生产零调用**。
- `assemble_game_save_sections`（save_game.rs L408）：定义，**无 crate 内调用方**。
- `write_json_sections`：仅被 `save_game_to_memory`（L93-101）调用，而 `save_game_to_memory` 无生产调用。
- `save_game_system`（节拍器）：发 `CaptureSnapshotRequested` + `AsyncSaveCmd::BackupSave`（备份生产）；**同步保存 `AsyncSaveCmd::OpenFile` 由宿主事件响应完成**（注释明示，非 crate 内）。
- → 现 `Sheep` 节（SaveFileSection::Sheep=5）**不写盘**。block B 待办：真实写盘链路（捕获 → sheep 节 → assemble → OpenFile → save_thread 写盘）。

### C. 快照应用 + 会话消费端现状：**空函数/未接线**

- `apply_loaded_snapshot`（snapshot.rs L225）：空函数（"域应用留待各域恢复后接入"）。
- `SnapshotLoadResources` 只登记合同；`init_game_state_from_snapshot`（0x1417a9c50）未实现应用。
- 会话事件消费者：`reset_input_state` 未消费 NewSessionStartedCmd/SessionLoadedCmd（① t2 契约 §3 6 消费系统均未接线）；`time_of_day`/`input_record_replay` 同。
- → block C 待办：apply_loaded_snapshot 逐域应用 + 会话事件消费端接线。

## 2. 给 ① 的深挖方向（原机器码）

1. **A 调度器**：`add_game_startup_systems` 0x140941110 28-系统 tuple 逐名（解 FUN_140a97110 → 顶层 builder FUN_140ce4b10/FUN_140a2bf40/FUN_1415f4750/FUN_1409c8600 → FUN_140a2bf40 聚合 10 叶）；`add_stages` 0x140945ed0 bevy Schedule 分阶段；各域 build 的 0x140b6d290（garden）/0x140944070（ivy）插件注册序。
2. **B sheep 写盘**：`save_game` 0x140a3f470 的同步保存触发路径（X4 Query 采集 → GameSaveSections 拼装 → AsyncSaveCmd::OpenFile 的**原生生产者**）；save_thread 0x141720680 OpenFile 处理写盘；Sheep 节在 GameSaveSections 的槽位写序。
3. **C 快照应用**：`init_game_state_from_snapshot` 0x1417a9c50 逐域应用序（颜色/时间/羊/地形）；会话消费端 6 系统的注册点与运行条件（t2 契约 §3 表）。

## 3. 逐块复审（2026-10-16，② 交付后）

### A 块（调度器整合）：**部分通过（2 项须修）**

孤儿系统真进生产 Schedule 核实（`register_core_startup` run_game.rs L92-145）：
- ✅ **ivy**：`register_ivy_storage(world)` + `add_ivy_systems(schedule)`（7 系统链 grower/spawner/pruner/pruner_cmd/leaf_spawner/branch_thickness/leaf_animation，chain）——真注册。
- ✅ **garden**：`register_garden_events(world)` + `add_garden_cpu_systems::<(),()>(schedule)`——真注册。
- ✅ **input**：`add_input_cpu_consumers(schedule)`（translate_input→cursor_moved→delta→reset + mod_key/update_cursor_position + 回放族）——真注册。
- ✅ **sheep**：`init_resource::<SheepControl>` + `add_systems(sheep_controls)`——真注册。
- ✅ **audio**：`add_audio_events(world)` + `add_audio_systems::<S,B>(schedule)`（music_debug_gui + water 两系统）——真注册。
- ✅ **G2 mesh 族**：startup_garden_brush/startup_ivy 入 add_game_startup_systems（① a 契约 §1）。
- ✅ 双泛型 `<B,S>`（MusicBackend+Studio）；wall/signifier 孤儿登记待接入（wall 触 roof 重构域、signifier_audio 用户错误——不越权，诚实）。

**⚠️ F3 仍开（① 已裁定，见下）：`garden_audio_system`（T1 已复审通过）未注册进任何生产 Schedule**——add_audio_systems 只挂 music_debug_gui+water。

**【① 裁定（audio-G6-alignment-CONTRACT.md + 消息）】**：
1. **garden_audio 注册点** = native `add_audio_systems`（0x1409411a0，systems::audio::impl$1，tag 0x26=38 音频系统），非 garden build 0x140b6d290。② 应把 garden_audio_system 加入 crate add_audio_systems → **F3 关闭**。
2. **G6 集合**（0x140b6bd50）= timberframe/elevation_supports/clutter/forest/tree/firstperson_steps **6 系统，③ 核实全部在 crate 存在但未注册**（timberframe_audio/elevation_supports_audio/clutter_audio_system/forest_audio_system/tree_audio_system/firstperson_steps）。water×2 不在 G6（属 water 子组）。crate 现 add_audio_systems（music_debug_gui+water）为局部子集。② 应按 native 系统面把 G6 6 系统 + garden_audio + 其余音频系统补入 add_audio_systems。

**⚠️ add_audio_systems 集合与 ① G6 契约不一致**：① a 契约 §1 G6（0x140b6bd50，6 音频系统）= timberframe/elevation_supports/clutter/forest/tree/firstperson_steps（PathRastersResources SystemParam），与 crate 现 add_audio_systems（music_debug_gui+water）集合不同——需 ① 定 crate 3 系统归属组，② 按 G6 对齐补 audio 系统。

Schedule 可执行性：① a 契约 §6 GameLoop（宿主 crate）驱动 Schedule::run；crate 只保证注册 + register_core_startup 内资源 init——库内模型成立；运行期验证宿主侧。

### B 块（Sheep 写盘链）：**部分通过（生产链真、写盘宿主）**

- ✅ `SheepSaveSectionValue(pub SerializedJson)` + `sheep_save_section_system`（读 SheepControl→sheep_section_serialized→资源）——**真生产系统，注册于 register_core_startup L143-144**。
- ✅ 生产链真实调用点：`sheep_controls`（A 块注册）→ SheepControl → sheep_save_section_system → SheepSaveSectionValue（宿主 assemble 取 SaveFileSection::Sheep=5）。F1b 生产半段闭合。
- ⚠️ **写盘仍宿主**：assemble_game_save_sections→write_json_sections→AsyncSaveCmd::OpenFile 由宿主事件响应触发（save_game.rs L206 登记）。crate 内 Sheep 节 VALUE 真生产，DISK WRITE 宿主。"Sheep 真写盘"= 生产链真、写盘宿主——半闭合，诚实登记。
- 10 节序对齐（Version/Identity/BackwardCompatibility/Camera/TimeOfDay/Sheep/Colors/ChainIndex/HistoryStart/Snapshots）✓。

### C 块（快照应用 + 会话消费端）：**部分通过（结构面 O、应用待①）**

- ✅ `SnapshotDomainDescriptor{ptr,len,count}` + `SNAPSHOT_HEADER_SIZE=0xc0` + `SNAPSHOT_DOMAIN_COUNT=11`（decomp-r5-0x1417a9c50）——**结构面 O 核实**。
- ⚠️ `apply_loaded_snapshot` 由空函数 → **按域迭代 + 登记目标映射**（Version→版本门控/Colors→GladeColorCustomization/TimeOfDay→TimeOfDayState/Sheep→SheepControl/Terrain→TerrainEditingData），**尚未实际写入**（目标资源映射待①，不伪造）。"逐域应用"未达（结构+迭代，应用闭包待①）。
- **【① 裁定】**：C 逐域 apply 目标已按 `c-snapshot-apply-targets-CONTRACT.md`（Walls/Decorators/Terrain 可直接落地，其余域待取证）；`on_event<NewSessionStartedCmd>` run 条件 = bevy 通用 run_if（public-session-names entry6，`FunctionSystem<fn(EventReader<NewSessionStartedCmd>)->bool>`），② 落会话切换链消费端时用 on_event 条件门。
- ✅ `time_of_day` 3 参子集系统 + TimeOfDayState/TimeOfDayLightingSink 资源注册；input_record_replay/reset_input_state 经 A 块 add_input_cpu_consumers 注册；库外 3 消费系统（add_preplaced_autoclutter/detect_overhang_support_segments/sheep_unstuck）留宿主注册点——诚实。

### 回归检查

- ② 声称整 crate 剩 4 错误全在用户 wall/snapping 重构域。git status 确认 wall/walls/shapes 大量未提交修改（用户并发重构）。② 的 5 文件（run_game/startup mod/save_game/snapshot/time_of_day）未见编译风险项。**我未跑 cargo**（角色限制），4 错误位置请父/宿主最终确认非 ② 引入。

### 给① 深挖方向（追加）

1. garden_audio_system 的 native 注册归属组（garden build 0x140b6d290 16+34 系统？独立音频组？）+ add_audio_systems 0x1409411a0 与 G6（0x140b6bd50）集合差异裁决。
2. B：X4 活羊 Query 精确面（b 契约 §3）。
3. C：逐域应用闭包目标资源映射（c 契约 §1）；on_event<NewSessionStartedCmd> run 条件（public-session-names entry6）。

### ② 修复项复审（2026-10-16，F3/G6/C apply）

**通过（无水分；C apply 仍为 dispatch 骨架，诚实登记阻塞）**：

1. **F3 闭合 ✓**：`garden_audio_init_system` + `garden_audio_system`（T1）补入 `add_audio_systems`（audio/mod.rs 批 2）。③ 复核 garden_audio_system 为真系统。
2. **G6 对齐 ✓**：`add_audio_systems` 现含 G6 6 域（timberframe/elevation_supports/clutter/forest/tree/firstperson_steps，init+main）+ F3 garden_audio + crate 其余音频域（audio_update/building/decorator/game_loading/menu/path/playermade_stairs/replay/sheep/signifier/terrain/terrain_stairs/trigger/wall_audio_fmod）init+main + weather 单条（bevy tuple ≤20 分 3 批）。**③ 逐函数核验全部存在**（含 building_audio/menu_audio/wall_audio_fmod 命名变体），并抽查 building_audio/menu_audio 为真实现（非桩）。water×2 保留（属 water 子组，可另注册，非问题）。
3. **C apply dispatch**：`apply_loaded_snapshot` 现按 c 契约 §2 分派 Walls→InnerWalls::load_state(0x14085ad80)/Decorators→DecoratorStorage::restore_snapshot(0x140a20d70)/Terrain→TerrainEditingData——**但仍只 log::trace，未实际调用**。阻塞诚实登记：`DecoratorStorage::restore_snapshot`（decorator_storage.rs:760）存在但 `DecoratorStorageSnapshot` 非 serde 无法从 JSON 填充；`InnerWalls::load_state` crate 无此 API；Terrain 解压字节格式待①。"逐域应用"未达（dispatch 骨架 + 目标登记），非伪造。
4. **on_event run 条件**：bevy_ecs 0.16 可用，应用对象待①（登记）。

**回归**：② 声称整 crate 零错误；③ 逐函数核验新增系统全部存在（类型检查可通过的前提成立）。无 ② 引入回归迹象。

### C apply API 格式复审（2026-10-16，② 二轮，c-apply-api-formats 契约）

**通过（无水分；apply 分派仍骨架，字段解码阻塞诚实登记待①）**：

1. **InnerWalls::load_state**（0x14085ad80）：crate 实现签名 `load_state(&mut self, &[InnerWallStateRecord])` + `InnerWallStateRecord(pub [u8; 0x78])`（repr(C) 120B）+ 空桶前置（crate 以 walls 空表达原生 param_1[3]/[0xf]/[0x15] 三内层 Vec 空判——近似等价，诚实标注）。**0x78B 逐字段名（+0x34..+0x5c）待①**——本实现登记签名+前置+迭代骨架，**不伪造解码**（不构造 WallState）✓。
2. **DecoratorStorageSnapshot**：显式标注 16B≈0x28B 节格式不符（decorator_storage.rs L312-315；`restore_snapshot` 0x140a20d70 签名+前置可直接用）——诚实登记，节格式对齐待①。
3. **Terrain**：flate2 解压已有；strokes 字节布局待①。
4. **apply_loaded_snapshot 分派**：引用真实 API（InnerWalls::load_state 已实现、DecoratorStorage::restore_snapshot 存在）——**但仍只 log::trace 未执行真实应用**（节→记录解析 Walls 0x78B 字段/Decorators 0x28B/Terrain strokes 待①）。

**结论**：目标 API 面已 O 真实（签名对 0x14085ad80/0x140a20d70），apply 骨架引用真 API；**"逐域应用"仍待① 补字段级格式后落地**——非 ② 可自行完成项，诚实登记不伪造。

### C 节→记录解析层复审（2026-10-16，② 三轮，c-apply-section-parsing 契约）

**通过（无水分；apply 分派仍未调用解析器，数据源接线阻塞待①）**：

1. **parse_wall_section**（L281）：真解析——`{count@0, ptr@8}` + 0x78B×count → `Vec<InnerWallStateRecord>` ✓。
2. **record.wall_id()**（inner_walls.rs L36-38）：record[0] LE → WallId（FUN_1402ca2f0=HashMap::insert<WallId,()>）✓；`load_state` 逐记录解码 wall_id + log，其余 14 qword → WallState 构造待① ✓。
3. **parse_decorator_section**（L303）：0x28B 记录形状登记（{状态 16B, 子计数@0x10}）+ 返回输入字节视图——**passthrough**，逐字段解码待①（显式标注，不伪解析）。
4. **inflate_terrain_entry**（L313）：flate2 解压真实现（复用 inflate_terrain）✓。
5. **apply_loaded_snapshot**：**仍未调用上述解析器**（只 log::trace）——阻塞 = **数据源接线待①**：原生快照节为二进制（0x78B/0x28B 记录），crate `LoadedSnapshotData.sections` 为 JSON 节，二进制节字节来源/编码未定。③ 判此为 C 应用落地的**首要阻塞**。

**【① 数据源定案（c-apply-data-source 契约 §2b，③ 复核 crate snapshot.rs 头注 "JSON + deflate" 一致）】**：
1. **快照文档 = serde_json JSON**（非二进制整节、非 base64）。节载荷 = JSON 值。
2. **Walls/Decorators 节**：JSON 值 = serde 序列化状态；**0x78B/0x28B 记录 = JSON 反序列化后的内存形式**（load_state/restore_snapshot 的消费面），非节载荷本身。**Terrain 例外**：deflate 字节经 JSON 串/数组承载（native 精确编码待证）。
3. **{count@0, ptr@8} = 内存 Vec cap-first 头**（cap@0/ptr@8/len@0x10），非节 JSON 布局。

**→ ② 接线修正**：parse_wall_section 当前以原始字节 + {count@0,ptr@8} 解析 = **内存记录段**（反序列化输出），非 JSON 节输入。正确数据流：节 JSON 值 → serde 反序列化 → 0x78B/0x28B 记录/状态 → load_state/restore_snapshot。**② 需按此重接 parse_wall_section 输入模型**（JSON 值反序列化，而非原始字节）。Terrain = deflate 解压 → tg_terrain。

### ② 数据源纠偏实施复审（2026-10-16，② 四轮）

**通过（无水分；apply 真分派，字段级阻塞诚实待①）**：

1. **parse_wall_section**：输入改 JSON 值 → serde 反序列化 `Vec<[u64;15]>`（15 qword=0x78，O 形状候选）→ `Vec<InnerWallStateRecord>`（record[0]=WallId）。JSON 形状不符时 log + 空 Vec（不伪造）✓。**形状候选待①确认**。
2. **parse_decorator_section**（JSON 值）→ 恒 `None`（JSON↔0x28B serde 架构待①，不构造假快照）——诚实。
3. **parse_terrain_section**（JSON 值）→ 串/数组首串 → flate2 inflate 真实现 ✓。
4. **apply_loaded_snapshot 真分派**：**Walls→parse→load_state 真调用**（JSON 可解析时）+ Decorators→restore_snapshot（parse 恒 None 未达）+ Terrain→inflate（解压字节未应用至 TerrainEditingData，strokes 布局待①）。其余域登记待①。
5. **load_state**：逐记录解码 wall_id，**未构造 WallState**（14 qword→WallState 待①）——真实深度止于 record 解码，诚实。

**结论**：C 应用管线真（JSON→parse→dispatch→target），数据源阻塞已解；剩余全为 ① 字段级格式（Walls 14 qword / Decorators serde 架构 / Terrain strokes / Walls JSON 形状确认）。**"逐域应用"达结构性真应用（Walls 记录解析+分派执行），字段级应用待①**——非 ② 可自行完成项。零错误可信。
