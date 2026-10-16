# RE-ANALYSIS — systems/save（异步捕获 + GameSaveData 边界）

分析师: re-analyst-state
日期: 2026-10-15
范围: DISPATCH 组 state 的 systems/save 缺口（save_game/save_thread/load_game）+ 跨组依赖
GameSaveData。既有 session-save/DIGEST.md 与 ALGORITHMS.md 已把大部分函数算法级闭合；
本报告**只补新增证据 + 修正**，不重复已 O 结论。证据 = fresh 反编译（shared）+ skeleton PDB + RNE 原字节。

---

## 1. GameSaveData 消费边界（O，新增）

- **定义**：`save_game.rs` PDB 命名实体（skeleton 行 6162），Box<GameSaveData>。
  过程引用 4：`save_game.rs:6162`(定义)、`save_thread.rs:350`(drop @ execute_save_cmd 0x141721aa0)、
  `load_game.rs:187`(drop @ 0x14146d030)。
- **save_game 0x140a3f470 组装**：
  - 系统参数面（skeleton PDB tuple，O）：
    `ResMut<History>, ResMut<AutoSaveState>, Res<InnerWalls>, Query<&Roof, Without<RoofMarkedForDeletion>>,
    Res<MainCamera>, Res<TimeOfDay>, Res<TerrainEditingData>, Res<TerrainHeightsData<TerrainHeightSsbo>>,
    ResMut<ActiveSession>, Res<DecoratorStorage>, Res<WallColorIds>, Res<GateColorIds>,
    Res<HalftimberColorIds>, Query<(&PrefabInstanceData,&PrefabInstance,&TreeComponent)>,
    Res<GladeColorCustomization>, Res<DeletedAutoClutter>` + 局部面
    `AllWorldRasterRead, Res<GameTime>, Res<AnimateWallBulldozeBuffer>, Res<ReplayBuffer>,
    Res<StairsState>, Res<StairsRemovedSupports>`…（= GameSaveData 的数据源集合）。
  - 序列化缓冲 {ptr,len}@AutoSaveState+0x60/+0x68；字段写区 +0x18..+0x55（16B/8B/4B/1B 混合，
    decomp 行 1705-1717）。
  - 同步保存（decomp 行 1728-1843）：`SessionMeta::write_to_disk(0x140b463c0)` →
    `FUN_1408d9a20` → 发 **AsyncSaveCmd::Flush(10)**（FUN_1408da280）→ 同步响应分派
    （FUN_1416cc990/1416741f0/1416798b0）→ `save_folder_path(0x140b46000)` + push 0x28B 会话条目。
- **execute_save_cmd 0x141721aa0 消费**：OpenFile 全量重写 = `FUN_141750c90(uVar38+0x700,
  uVar38+0x15a0, uVar24, uVar45)`（**save_game_to_file<Cursor<Vec<u8>>>**，decomp 行 580）——
  GameSaveData 位于栈 `uVar38+0x15a0`，序列化后 Box drop。
- **load_game 0x14146d030 重建**：parse_saved_game_data（FUN_141470d00 内联）→ 解析数据
  应用；`drop_in_place<Box<GameSaveData>>` 于 load_game 函数域。
- **结构级结论（交 restorer）**：GameSaveData = **完整存档序列化数据集合**
  （History 链/相机/时间/地形/装饰/三色 ID/glade 颜色/ActiveSession/树）——消费边界与数据源
  已闭合；**逐字段 layout 为残留**（需 save_game 0x140a3f470 分段逐槽比对）。

---

## 2. save_thread AsyncSaveCmd 变体表（核对 + 修正）

| 变体 | 名称 | execute_save_cmd case | 行为（O，DIGEST 已有） | 本批结论 |
|---|---|---|---|---|
| 0/1/2 | （无 debug 串） | **无专属 case** | — | **确认死/历史变体**（DIGEST「I：死/历史」维持；本批未发现生产调用点） |
| 3 | OpenFile | case 3 | 旧态 flush → 全量 save_game_to_file<Cursor> 重写 → ShadowFile::create | O |
| 4 | CloseFile | case 4 | flush 旧态 → UnsavedFileList::add | O |
| 5 | ChangeSection | case 5 | state==3 panic 门控 → change_section + mark_dirty×2 | O |
| 6 | ReplaceEdits | case 6 | state==3 panic 门控 → 逐 edit 追加 | O |
| 7 | BackupSave | case 7 | `history-auto` 前缀 + 备份限额 | O |
| 8 | ManualBackup | default | Copying inactive save file to backup | O |
| 9 | CaptureSnapshot | case 9 | snapshots 目录 + 快照 JSON 落盘 | O |
| 10 | Flush | case 10 | 刷新当前档 | O |
| 11 | Shutdown | 主循环 0xb | flush_deferred_and_sync + 发回确认 + 退出 | O |

---

## 3. load_game 内联函数族（核对，均为 O 已闭合，供 restorer 落地）

| 内联函数 | 位置 | 行为（O，ALGORITHMS §2.7） |
|---|---|---|
| `parse_saved_game_data` | load_game 行 ~413 | 节序 Version→BackwardCompatibility→Camera→Time of day；upgrade 前备份；失败「Couldn't load save file from 」 |
| `apply_parsed_game_data` | 行 744 | 解析数据应用（init_game_state_from_snapshot 族） |
| `apply_backward_compatibility_overrides` | 行 870 | 版本 → 全局开关 0x14362c308=(v>0x19)、0x14362c011=(v>0x1a) |
| `upgrade_window_color` | 行 880 | 颜色 ID 映射 {1→2,2→5,4→8,5→3,8→4}（0/3/6/7 不变、999 透传、default→1） |
| `extract_max_wall_id_from_history` | 行 899 | 遍历 MultiEdit 取最大 wall id → 计数器资源 +0xc0 |

- 新佐证：颜色升级条件 `+0x52 ∈ {0x38..0x3a}` 与 0x58 步长记录（decomp-0x14146d030）；
  on_load 两遍回放见 session/RE-ANALYSIS §2.3（fence 0x8000000000000003）。

---

## 4. 修正登记

- **DIGEST「on_load 列表条目 4B tag」** → 新证据：0x20B 条目、tag 首 8B、fence 0x8000000000000003
  （详见 session/RE-ANALYSIS §2.3）。
- **DIGEST「GameSaveData +0x28..+0x100 密集字段」** → 新证据：字段区确认为 AutoSaveState
  `+0x18..+0x55` + `+0x60/+0x68` 序列化缓冲；数据源 = 16+ 项 SystemParam（见 §1）。
- **AsyncSaveCmd 0/1/2**：维持「无 case / 死变体」结论（本批无生产调用点）。

---

## 5. 未决疑问

1. GameSaveData 逐字段 layout（最大残留，建议下一轮对 save_game 0x140a3f470 分段逐槽比对）。
2. OpenFile 命令的 GameSaveData 载荷来源：run_game_thread 或 save_game 侧打包——已定位
   save_game 组装区，但 Box 化与 channel 发送指令未逐字节追到（可 RNE 原字节扫 save_game 的 FUN_1408da280 调用前）。
3. snapshot.rs `capture_game_snapshot_save_data 0x1417a0430` 的 ~0x400B 快照结构逐字段（已有串证据，字段级 U）。
