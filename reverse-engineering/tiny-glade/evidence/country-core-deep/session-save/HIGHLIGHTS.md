# session-save 域 HIGHLIGHTS（tg_a3，2026-10-14）

1. **AsyncSaveCmd 变体表完整解出**：3=OpenFile、4=CloseFile、5=ChangeSection、6=ReplaceEdits、
   7=BackupSave、8=ManualBackup、9=CaptureSnapshot、10=Flush、11=Shutdown（save_thread 以
   `cmd==0xb` 退出并 flush_deferred_and_sync）。变体 0/1/2 未在本 switch 出现（U）。
   证据：execute_save_cmd 0x141721aa0 的 debug 串 `AsyncSaveCmd::*` + case 3/4/5/6/7/9/10。

2. **存档文件协议实锤**：history.json 为手工拼 JSON（`'{'`、`"History":{`、edit 间 `",\n"`），
   节名走 SaveFileSection::json_key；增量写经 ShadowFile：`<path>.tmp` 临时文件 + 写数据 +
   set_length + sync + `Save file successfully opened and written to disk`；mark_dirty(start,end)
   8 槽脏块区间跟踪；ensure_current_snapshot 每次追加 edit 后调 mark_dirty。

3. **备份节拍器数值解出**（save_game 内联，0x140a3f470）：{counter u32 @+0x140, 累计活动秒
   f32 @+0x144（clamp [0,1]）, 阈值 f32 @+0x148}；仅阈值 <60.0 时累计；达到 1.0 时 counter++
   并按档位表 **{300, 600, 3600, 7200}**（5min/10min/1h/2h，@0x142a9f348）查触发档；
   `active_game_delta_seconds >= 0.0` 断言。wp13 的“数值 U”已闭合大半。

4. **快照 schema 不是 rkyv/ron 二进制，是 JSON + deflate**：节序 Version→Walls→Roofs→Decorators
   →WallColors→GateColors→Terrain→Trees→Raster；Terrain 载荷 flate2-1.0.24 压缩
   （`decompressing terrain data`）；`Version mismatch; expected , got `；已知结构
   StairsSnapshot{edges,checksum}、TerrainEditingData{strokes,next_stroke_id,next_stroke_index}、
   DecoratorStorageSnapshot{states}。wp13 “rkyv/ron 编码” 假设作废。

5. **SessionMeta 内容解出**：meta.json serde 字段名 {**state, glade, mutators, image**}（串块
   0x142c3a870/0x142aef526）；季节字节编码 {0=halloween2024, 1=其他万圣节, 2=xmas2024}；
   **is_halloween_any 机器码为“存在字节 ≤1”，合同写 “>1” 方向相反（纠错）**。

6. **LogoAnimation 5 阶段状态机全解**：progress[5] @+0..+0x14 + durations[5] @+0x14..+0x28；
   阶段 0 起始播 `event:/UI/Logo/Scribble`、阶段 2 播 `event:/UI/Logo/Branches`；is_waiting =
   (+0x0c)>0、is_finished = (+0x10)>=1.0、finish_wait = +0x0c:=1.0。PDB 名整体错位一位
   （0x140b19d30 名 finish_wait 实为 is_finished；真 finish_wait 在 0x140b19d33）。

7. **load_game 状态机与升级路径**：LoadGameState::LoadGameData → 写 `loading.tmp`（会话目录内，
   Windows HANDLE 直写）→ NextSessionKind 解码（1=parse_saved_game_data 含 upgrade 备份、
   2=on_load 命令回放+内存快照，I）→ PostGameLoadCommand 回放（fence 0x8000000000000003，
   SwitchAppMode 被忽略）→ **旧版颜色 ID 升级映射 {1→2, 2→5, 4→8, 5→3, 8→4}**（window color +
   arrow slits 两处，日志已捕获）→ extract_max_wall_id_from_history 写墙 id 计数器 +0xc0。

8. **会话目录文件协议**：{history.json, screenshot.jpg, meta.json, loading_background.jpg} +
   assets/glade 子目录；delete_session = 写标记文件到 deleted_saves()/uuid 后 rename 移入
   （两段删除，`Error deleting glade: `）；0x140b46300 行为=构建三文件完整路径
   （session_file_list），PDB 名 snapshots_folder_path 错位；0x140b46a50 实为
   loading_background_path（非 screenshot_path）。

9. **prefab 实例链**：PrefabLibrary 哈希桶 0x28B（Prefab 值仅 16B → reconstruction 的
   `Prefab { parts: Vec<PrefabPart> }` 布局存疑）；hot_reload 全链 =
   despawn_prefab_contents → create_prefab_audio_trigger → create_prefab_rt（GPU 实例 +
   颜色打包 u32 + spawn）；create_prefab_rt 按变体查位置表 @0x142a94358。

10. **NextSession 布局细节**：on_load 条目 0x20B = {u32 0, u32 0x80000000, u64 payload}；
    NextSession::load 置 kind=0x8000000000000002 + cap-first 空 Vec(cap=8) + ignore_snapshot=0；
    append_on_load<MetaCommandFence> 为 builder（memcpy 0xa0 返回）。

（详见 DIGEST.md；批量 66 目标 57 成功 9 NotFound 全部手解。）
