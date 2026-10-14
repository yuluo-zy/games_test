# utils / input / config 域 HIGHLIGHTS（2026-10-14，dbid tg_ui2）

1. **DirtyTiles 即 tabloid 的 State/StateDelta**：64×64 位图（512 B）同时作状态与 delta，
   `impl$2::apply_to`(0x140960ef0) = OR 累积；订阅者含 wall lazy-update、world_raster copy_to_staging、
   autoclutter、stairs 传播。`DirtyTiles::rasterize_world_space_stroke`(0x140ade620) 用常量
   `{32/65, 1/2600, 32.0}`，单元=32 世界单位（64 格×32=2048×2048 地图）。
2. **`load_json_as_mesh` 为 country-core 特有自定义 JSON 网格解析器**（非 serde_json，断言
   `values.array_length()`）；键 `indicesbuffer/buffer/attributes`，属性 `type` int/float（tag 0..4）；
   共享 utils 无此函数，68 调用方依赖它。4 泛型实例 + `read_attribute_from_json`(0x14083e210) 全 O。
3. **input_stream 与 action_set 同构**：`InputFrame`(0xd0 B) 内 `Vec<[u8;12]>` = 
   `MainActionSet::deconstruct` 打包记录（tag `slot<<8|kind`）；`InputRecordReplayMode` 5 变体
   （AutoRecordPanicRepro/Records/Replay/DisableInput/None）；`drop_oldest_sync_point` 的 panic 串
   `assertion failed: self.frames.pop_front().is_some()` 证实 frames 为 VecDeque。
4. **action_set 的 46 个动作名**（serde_json 键）：`input_space_cursor_move`、`hardware_window_space_cursor_move`、
   `camera_pan/height/zoom/rotate`、`tool_{building,path,decorator,customize,terrain,eraser,garden,tree,water}`、
   `ui_{cancel,confirm,left,right,down,up,tab_prev,tab_next}`、`undo/redo`、`screenshot*`、`toggle_snapping` 等。
5. **game_key_bindings 40 槽 12B 步距 + 36 默认键**（`default`@0x14089b110 原字节，多前 10 键
   `0x2a,0x26,0x14,0x17,0x18,0x24,0xc3,0xc3,0xc3,0xc3`）+ `[0x1b0]=0xc3` + u16 `{1,2,1}`；ron serialize
   给出精确字段表（move_forward…mouse_focus_edit）。
6. **GameProfile 0x208 B，ron 字段** `{player_id, game_installation_id, tutorial, tutorial_cooldowns,
   key_bindings, new_glade_count}` —— 修正重建的 `tutorial_completion/tutorial_backoff`。
7. **AudioLoop 是游标结构非 Entity/Resource**：48 B entries Vec + Instant 计时 + kira
   `set_parameter("loop")`，挂在 AudioCore resource 下；stop/start_if_not_playing/current/
   instantiate_loop 全 O。
8. **meta_commander**：`commander_update`(0x14091f180) niche `^0x8000000000000000` 判别 case 0..0x13
   （会话加载/profiling/GPU+内存报告/despawn/ShutdownGame/exit）；`MetaCommand` 0x38 B；
   `MetaCommandFence`=Arc<Mutex<(bool,bool)>>；全局动作/设置名表 @0x14293d960+。
9. **utils/mod 事件为自定义 EventRegistry**（TypeId→Events HashMap），非 Bevy Events；
   `get_last_event`/`drain_events` 未注册 panic `Event not registered`。
10. **纠错**：DigitalActionState 机器码 `pressed=idx<3`（变体序 U，或 5 态）；DebugSphere 布局
    radius@0x10/center@0x14（重建字段序不符）；ProportionalScreenSpace=16B{size,aspect,1.0}。
