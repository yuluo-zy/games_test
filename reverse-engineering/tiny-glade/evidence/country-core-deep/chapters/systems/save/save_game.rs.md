# systems/save/save_game.rs

> 文件作用：主存档系统：save_game（6 段 SystemParam；节拍器内联；capture→Arc→set_current_snapshot(0x140a84760)→发送 AsyncSaveCmd::CaptureSnapshot(9)）；SaveFileSection 10 变体节协议（json_key/padding 表 @0x142ace328/0x142ace378/0x142ace3c8）；save_game_to_file 手工拼 JSON（'{'、"History":{、edit 间 ",
"）；ensure_current_snapshot 增量历史写（mark_dirty 协议）；to_json_string 族 = 各节 serde 载荷（Colors=GladeColorCustomization、Identity=HistoryIdentity、Camera=CameraRigParametersCache、TimeOfDay、Sheep=Option<SheepControlSnapshot>、chain_index=HistoryChainIndex、snapshot=HistoryChainSnapshots）。

函数总计 33：业务 19 / 内联无独立符号 3 / 派生胶水 11

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| ``country_core::systems::save::save_game::save_game`` | 0x140a3f470 | 0 | IO/解析 | O：主系统（节拍器+快照捕获+AsyncSaveCmd 发送；错误串 Error while performing a synchronous save / Sync response from save thread / Failed writing session meta） | session-save\decomp\decomp-0x140a3f470.txt  |
| ``country_core::systems::save::save_game::save_game_to_file<std::fs::File,core::slice::iter::Iter<country_core::resources::history::MultiEdit> >`` | 0x140241ca0 | 0 | 待推断 | O：save_game_to_file<File>（手工 JSON：'{' 0x7b、"History":{、",\n" 0xa2c 分隔） | session-save\decomp\decomp-0x140241ca0.txt  |
| ``country_core::systems::save::save_game::save_game_to_file<std::io::cursor::Cursor<alloc::vec::Vec<u8,alloc::alloc::Global> >,core::slice::iter::Iter<country_core::resources::history::MultiEdit> >`` | 0x141750c90 | 0 | 待推断 | O：save_game_to_file<Cursor<Vec<u8>>>（同 File 实例） | session-save\decomp\decomp-0x141750c90.txt  |
| ``country_core::systems::save::save_game::write_json_sections<std::fs::File>`` | 0x140242b40 | 0 | 待推断 | O：write_json_sections<File>（节级序列化+padding） | session-save\decomp\decomp-0x140242b40.txt  |
| ``country_core::systems::save::save_game::impl$2::to_json_string<country_core::resources::glade_color_customization::GladeColorCustomization>`` | 0x14085ee90 | 454 | 序列化/serde | O：to_json_string<GladeColorCustomization> = serde_json::to_string（Colors 节载荷） | session-save\decomp\decomp-0x14085ee90.txt  |
| ``country_core::systems::save::save_game::impl$2::to_json_string<country_core::resources::history::HistoryChainSnapshots>`` | 0x140b38810 | 454 | 待推断 | O：to_json_string<HistoryChainSnapshots>（snapshot 节载荷） | session-save\decomp\decomp-0x140b38810.txt  |
| ``country_core::systems::save::save_game::impl$2::to_json_string<country_core::resources::history::HistoryIdentity>`` | 0x140b385e0 | 454 | 待推断 | O：to_json_string<HistoryIdentity>（Identity 节载荷） | session-save\decomp\decomp-0x140b385e0.txt  |
| ``country_core::systems::save::save_game::impl$2::to_json_string<country_core::resources::history::history_chain::HistoryChainIndex>`` | 0x1408aa7a0 | 454 | 待推断 | O：to_json_string<HistoryChainIndex>（chain_index 节载荷） | session-save\decomp\decomp-0x1408aa7a0.txt  |
| ``country_core::systems::save::save_game::impl$2::to_json_string<country_core::resources::time_of_day::TimeOfDay>`` | 0x140b389b0 | 454 | 待推断 | O：to_json_string<TimeOfDay>（Time of day 节载荷） | session-save\decomp\decomp-0x140b389b0.txt  |
| ``country_core::systems::save::save_game::impl$2::to_json_string<country_core::systems::camera_rig::CameraRigParametersCache>`` | 0x1416f8470 | 454 | 待推断 | O：to_json_string<CameraRigParametersCache>（Camera 节载荷；相机 schema 见 camera-time 域） | session-save\decomp\decomp-0x1416f8470.txt  |
| ``country_core::systems::save::save_game::impl$2::to_json_string<enum2$<core::option::Option<country_core::resources::critters::sheep::control::SheepControlSnapshot> > >`` | 0x140a51d40 | 454 | 待推断 | O：to_json_string<Option<SheepControlSnapshot>>（Sheep 节载荷） | session-save\decomp\decomp-0x140a51d40.txt  |
| ``country_core::systems::save::save_game::SaveFileSection::json_key`` | 0x140a43810 | 494 | 查表/访问 | O：json_key = tag 字节→len 表(0x142ace328)+ptr 表(0x142ace378) 双查表（NotFound 手解：movzx/shl 3/双 mov） | session-save\DIGEST.md  |
| ``country_core::systems::save::save_game::SaveFileSection::padding_bytes`` | 0x140a43830 | 552 | 查表/访问 | O：padding_bytes = 表 0x142ace3c8[tag] = [0,16,0,128,32,128,64,8,8,2048] | session-save\DIGEST.md  |
| ``country_core::systems::save::save_game::AutoSaveState::close_file`` | 0x140a43840 | 583 | 查表/访问 | O：AutoSaveState::close_file = 清 +0x120 字段+清 cap-first 列表（v1 批名 json_key 错位） | session-save\DIGEST.md  |
| ``country_core::systems::save::save_game::AutoSaveState::create_backup_of<std::path::PathBuf>`` | 0x141e59860 | 609 | 待推断 | O：create_backup_of<PathBuf>（备份目录创建） | session-save\decomp\decomp-0x141e59860.txt  |
| ``country_core::systems::save::save_game::AutoSaveState::last_error_state`` | 0x140a439a0 | 632 | 待推断 | O：last_error_state（错误状态访问） | session-save\decomp\decomp-0x140a439a0.txt  |
| ``country_core::systems::save::save_game::write_json_sections<std::io::cursor::Cursor<alloc::vec::Vec<u8,alloc::alloc::Global> > >`` | 0x141751c90 | 733 | 待推断 | O：write_json_sections<Cursor> | session-save\decomp\decomp-0x141751c90.txt  |
| ``country_core::systems::save::save_game::add_history_edit_to_save_file`` | 0x140a439b0 | 781 | 构造/注册/生成 | O：add_history_edit_to_save_file / v1 双名 ensure_current_snapshot = 追加 edit 计数+序列化 0x80B+",\n"(0xa2c)+ShadowFile::mark_dirty(start,end) | session-save\decomp\decomp-0x140a439b0.txt  |
| ``country_core::systems::save::save_game::save_session_meta_to_file<std::path::PathBuf>`` | 0x1417522e0 | 798 | 待推断 | O：save_session_meta_to_file<PathBuf>（SessionMeta 落盘，被 write_to_disk 调用） | session-save\decomp\decomp-0x1417522e0.txt  |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::save::save_game::SaveFileSection::save_cooldown_seconds`（行 510）— IO/解析
- `country_core::systems::save::save_game::ensure_current_snapshot`（行 754）— 待推断
- `country_core::systems::save::save_game::save_json_through_tmp_file`（行 810）— IO/解析

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`drop`、`eq`、`fmt`、`hash`、`next` 等 11 项

## 实现方式（算法级，2026-10-15 深挖；全文见 session-save/ALGORITHMS.md）

- **save_game 0x140a3f470**（6 段 SystemParam，行 ~507-591 内联节拍器）：
  1. **备份节拍器**：{counter u32@+0x140, 累计 f32@+0x144 clamp[0,1], 阈值
     f32@+0x148}；仅当阈值 < 60.0（或 NaN）累计（断言 delta>=0.0
     @0x142a9f2c0）；累计 >= 1.0 → counter++ 并扫**除数桶 {300,600,3600,7200}**
     （@0x142a9f348，从大到小整除匹配），命中后 counter -= 除数，**可一次消耗多个
     间隔**（内层按升序重新校验，桶内容为 0 → panic 除零）；
  2. 触发备份：log "Capturing a game snapshot" → capture_game_snapshot_save_data
     0x1417a0430 → Arc 克隆 → History::set_current_snapshot 0x140a84760 →
     FUN_1408da280 枚举分发发送 AsyncSaveCmd::CaptureSnapshot(9)；
  3. 同步保存：GameSaveData 组装（~9 资源 +0x28..+0x100 密集字段）→
     save_game_to_file；错误串 "Error while performing a synchronous save: " /
     "Sync response from save thread" / "Failed writing session meta: "。
- **save_game_to_file<Cursor 0x141750c90 / File 0x140241ca0>**：**手工拼 JSON**——
  写 '{' 0x7b、`"History":{` 字面量、edit 间 `",
"` 0xa2c；节名走 json_key
  （0x140a43810 双查表 @0x142ace328 len + @0x142ace378 ptr）；padding @0x142ace3c8
  = [0,16,0,128,32,128,64,8,8,2048]（10 节协议：Version/Identity/BackwardCompatibility/
  Camera/TimeOfDay/Sheep/Colors/chain_index/history_start/snapshot）。
- **write_json_sections<Cursor 0x141751c90 / File 0x140242b40>**：遍历 0x28B 节条目：
  json_key（FUN_140d79f90）+ 载荷 to_json_string（FUN_140b883f0）。
- **ensure_current_snapshot 0x140a439b0**（= add_history_edit_to_save_file）：
  1) `offset = 影子文件+0x38`（追加偏移）→ push 进 edit_offsets 列表；2) 零填充扩容 +
   写 u16 0x0a2c (",
") + mark_dirty(offset, offset+2)；3) alloc 0x80B 缓冲 →
   **MultiEdit::serialize**（FUN_140b39b80，h63c9 单态）→ 扩容 + memcpy + 更新 len +
   mark_dirty(旧端, 新端)。**增量写协议实锤**（round2 CLOSURE 的"History+0x38 快照
   id"描述修正为影子缓冲追加偏移）。
- **save_session_meta_to_file 0x1417522e0**：SessionMeta serialize → File::create
  （FUN_140d59dc0）→ write_all → sync → 错误日志族。
