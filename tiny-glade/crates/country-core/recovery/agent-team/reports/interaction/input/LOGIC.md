# LOGIC — systems/backend/input + input/(action_set, input_stream) 逐函数逻辑判等

审计员: logic-verifier-input | 日期: 2026-10-17
方法: ghidra `--worker shared` fresh 反编译(2026-10-17 现场),逐函数重建控制流后与 src Rust body 逐语句对照。
证据: `evidence/logic-input-2026-10-17/`(23 份 fresh 反编译 + 2 份机器码 read_memory)。

## 函数表

| 原函数 | VA | Rust 位置 | verdict |
|---|---|---|---|
| `calculate_input_delta` | 0x1416feeb0 | `calculate_input_delta.rs::calculate_input_delta` | 判等 |
| `digital_delta`(inline) | — | `calculate_input_delta.rs::digital_delta` | 判等 |
| `translate_input` | 0x1417cd0a0 | `translate_input.rs::translate_input` | **未实现-需重做** |
| `translate_input_to_cursor_moved` | 0x140b17a80 | `translate_input_to_cursor_moved.rs` | 判等 |
| `input_record_replay` | 0x1409423d0 | `input_record_replay.rs::input_record_replay` | **未实现-需重做** |
| `control_input_replay` | 0x140941a30 | `input_record_replay.rs::control_input_replay` | **未实现-需重做** |
| `advance_input_replay_frame` | 0x140942280 | `input_record_replay.rs::advance_input_replay_frame` | 判等 |
| `normalize_cursor_pos`(inline) | — | `input_record_replay.rs::normalize_cursor_pos` | 判等(u32→f32 精确转换,魔数路径与 `as f32` 数值一致) |
| `unnormalize_cursor_pos`(inline) | — | `input_record_replay.rs::unnormalize_cursor_pos` | 判等 |
| `reset_input_state` | 0x140942b00 | `reset_input_state.rs::reset_input_state` | **未实现-需重做** |
| `record_and_replay_input_for_trailers` | 0x14083c620 | `input_replay_for_trailers.rs` | **未实现-需重做** |
| `do_input_replay_ui` | 0x14083cdd0 | `input_replay_for_trailers.rs::do_input_replay_ui` | **未实现-需重做** |
| `update_cursor_position`(inline,PDB @0x1413ee760) | — | `update_cursor_position.rs` | **未实现-需重做** |
| `MainActionSet::deconstruct` | 0x140946b80 | `action_set.rs::deconstruct` | 判等(见 B1) |
| `MainActionSet::reconstruct` | 0x140947c80 | `action_set.rs::reconstruct` | 判等(见 B2) |
| `DigitalActionState::pressed` | 0x14093fa70 | `action_set.rs::pressed` | 判等(机器码 `setb disc<3`) |
| `DigitalActionState::released` | 0x14093fa80 | `action_set.rs::released` | 判等(机器码 `add -3; setb <2`) |
| `DigitalActionState::just_pressed` | 0x14093fa90 | `action_set.rs::just_pressed` | 判等(机器码跳表 0→true/2→`0.0>=val` setae) |
| `DigitalActionState::just_released` | 0x14093fac0 | `action_set.rs::just_released` | 判等(机器码 `sete disc==3`) |
| `ActionSetAction::just_pressed` | 0x1409487b0 | `action_set.rs::just_pressed` | 判等(机器码 kind/slot 过滤+跳表) |
| `ActionSetAction::pressed` | 0x1409487e0 | `action_set.rs::pressed` | 判等(机器码 kind==1 && slot==id && disc<3) |
| `ActionSetAction::as_analog` | 0x140948800 | `action_set.rs::as_analog` | 判等(机器码 kind==0 && slot==id) |
| `InputStream::sync_point_count` | 0x140949170 | `input_stream.rs::sync_point_count` | 判等(机器码 `mov rax,[rcx+0x18]`) |
| `InputStream::frame_count` | 0x140949180 | `input_stream.rs::frame_count` | 判等(机器码 `mov rax,[rcx+0x38]`) |
| `impl$2::index` | 0x140949970 | `input_stream.rs::frame` | 判等(越界 `expect_failed` panic) |
| `InputStream::add_frame` | 0x140949190 | `input_stream.rs::add_frame` | 判等(缺 log callee+继承 D2,见 C1) |
| `InputStream::on_history_sync` | 0x1409494c0 | `input_stream.rs::on_history_sync` | 判等 |
| `InputStream::drop_oldest_sync_point` | 0x140949660 | `input_stream.rs::drop_oldest_sync_point` | **未实现-需重做** |

判等 20 / 未实现-需重做 8。

## 严重分歧点

### A. translate_input @0x1417cd0a0(未实现-需重做)

- **A1 Steam 门方向错(最严重)**:原生**无提前 return**——Steam 存在时 `SteamAPI_ISteamUtils_IsSteamRunningOnSteamDeck` 只参与 mode-tracker 更新门控;点击扫描/门/绑定翻译/滚轮/手势/历史按钮的**主体现恒运行**(反编译 0x1417cd112..c1fe 与主体现之间无 return)。Rust `if steam.0 { return; }`(translate_input.rs:270-272)在 Steam/Deck 上**完全跳过键盘/鼠标翻译**。REVIEW/RESTORE"有且活跃才走 Steam 路径"描述与 fresh 反编译不符。
- **A2 mode-tracker 更新**:原生在函数开头按 `(mouse_count!=0 || keyboard_count!=0)`(Deck 时仅 keyboard_count)条件更新 `ActiveInputModeTracker`(0x1417cd126 写 +0x10/+0x18);Rust 在 `gate_open` 块**末尾无条件** `*mode_tracker = new_default_mouse_keyboard(now)`(translate_input.rs:363)——条件、时机、位置三处不符。
- **A3 右键手势**:原生 `moved_len` = **最末** `InputCursorDelta` 事件长度(0x1417cd882 段,读最后事件 `SQRT(x²+y²)`);Rust `fold(0, |acc,d| acc + d.0.length())` 求**和**。且原生 right-just-pressed 时 `MouseMoved` **复位 0**(0x1417cd9ac 写 fVar35=0);Rust `mouse_moved += moved_len`。
- **A4 screenshot 修饰**:原生 slot25 修饰检查在 `puVar5==0`(无 Steam)才执行,Steam 存在时跳过(0x1417cdb9e 区);Rust 无条件。

已确认判等部分(供修复保留):10 模拟槽目标全部一致(analog[2].y/x=pan、[3].x=height、[4].x=zoom、[5].x=rotate,±/标量 30/1800/1.5、rotate 用原始 delta);数字槽 10..39 动作↔槽映射**机器码验证**(readmap:slot20→0x96 time_of_day、slot21→0x98 undo、slot22→0x99 redo、slot23→0x89 right_click、slot24→0x97 snapping_disable、slot28..35→ui_cancel..ui_tab_next 0x9a..0xa1、slot25→0xa5 screenshot+0x3d 修饰、slot37/38 鼠标 1/2→camera_rotate/pan_mode、slot39→0xa5);click 扫描;EscMenuState 门 `tag==2||(tag==10&&flag!=1)`(flag@+8);滚轮 tag==0→8.0 否则 1.0 累加 zoom.x;ui_cancel 硬编码 0x73→0x9a;历史按钮 0→undo/非0→redo(0x98/0x99);demo 键目标槽(0xa2/0xa4/0xa6/0xaa/0xab/0xac/0xad)+恒启 0xa7→0xae。绑定表条目原生为 {u32 哨兵@0, keycode@4}(get_inner 读 @4);Rust `ActionBinding{key@0}` 读 @0——**默认表键码正确,但字段偏移与原生不一致**(真实 GameProfile 绑定加载时会错位,当前只走 default 表故行为对)。

### B. input_record_replay @0x1409423d0(未实现-需重做)

- **B1 case2 回放 buffer 缩放缺失**:原生复制 11 qword 后**额外** `translation.buffer[1] = state[1] * (cursor.y / frame.divisor)`(0x1409423d0 case2 `CONCAT44(fVar34*(qword1.y), fVar34*(qword1.x))`,fVar34=cursor.y/frame@+0x44);Rust `translation.buffer = frame.state` 直接覆盖无缩放。
- **B2 normalize_cursor_delta 边界**:原生 `divps` 直接除(cursor.y==0→inf/NaN);Rust `if cursor.y == 0 { return ZERO }` 守卫——y==0 语义不同。
- **B3 未决**:record 分支 `add_frame` 的 record 标志(仅 AutoRecordPanicRepro 裁剪)实参为寄存器不可见;default 分支覆盖 modes 0/1,原生对 Records 也可能重建流(Rust 仅 AutoRecordPanicRepro 重建)——需机器码复核。

已确认:重建流条件(输入事件||external_record||canvas_busy@+0xa3)、帧构造(output→+0x30/0x38/0x3c、游标+divisor、状态块+0x68..0xb8、归一增量、time_delta@0xc8)、case2 越界 panic("Out of bounds access")、游标 clamp(NaN→0 由 CVTTSS2SI 饱和)、unnormalize、case3 清零 22 qword、case4 no-op。

### C. control_input_replay @0x140941a30(未实现-需重做)

- **C1 Load 失败应 panic**:原生 `try_load` 返回 Err 时 `panic_fmt`(anyhow `Result::expect`,消息 "Failed to load an input recording from : {}");Rust `log::error!` 后继续(control_input_replay.rs:488-494)。
- **C2 Load 成功应回退末同步点**:原生成功路径做 `index = 末同步点.frame_index - 1` + clone toolbar + `MainCamera::load_parameters` + `immediately_finish_interpolating`(0x140941a30 行 239..302,含 "No sync points found in replay state"/"Can only seek..." panic);Rust Load 只置 `mode=Replay, index=u64::MAX`。**REVIEW6"index=-1 一致"仅对单同步点(frame_index==0)成立;多同步点场景原生 seek 到末同步点**。

已确认:命令分派(Sync→on_history_sync)、mode niche 0x8000000000000002=Replay 判别、同步点字段偏移(+0x18/+0x20/+0x58)。

### D. reset_input_state @0x140942b00(未实现-需重做)

- **D1 事件排空短接**:原生 reader1(InputCursorMoved)有事件时**不排空** reader2(InputReplayCommand)(0x140942b00 空检查短路);Rust 无条件 `.read()` 排空两个 EventReader。11 qword 置换+buffer 清零+`had||had→reset_all` 均正确。

### E. record_and_replay_input_for_trailers @0x14083c620(未实现-需重做)

- **E1 E 键缺工具栏/相机恢复**:原生 index=u64::MAX 后 seek 末同步点并 `clone(Toolbar)` + `load_parameters` + `immediately_finish_interpolating`(0x14083c620 尾部);Rust `let _ = &toolbar; let _ = &camera;` 跳过(Rust 注释自认状态块子字段未解码)。
- **E2 E 键无同步点**:原生 `expect_failed` panic;Rust `if let Some` 静默。
- **E3 O 键 Records**:原生只置 mode=None,**不清流**(0x14083c620 default 分支 `*plVar1=-0x7ffffffffffffffc`=None niche);Rust `replay.stream = InputStream::new()` 清空流——录制数据被丢,与原生不符。

### F. do_input_replay_ui @0x14083cdd0(未实现-需重做)

- **F1 条件渲染**:原生 checkbox 未启用(`config@+0xfc != 1`)即 `return`,只画 checkbox;Rust 恒画全部元素。
- **F2 按钮不符**:原生有 "Replay sheep" 按钮;反编译未见 "Save" 按钮。Rust 有 "Save"(log "tmp/input_replay.json")+"Replay sheep"(占位 log)——按钮集合与动作未对齐原生。

### G. drop_oldest_sync_point @0x140949660(未实现-需重做)

- **G1 目标帧数取错**:原生 target = **弹后新头同步点 frame_index**(仍有≥1 同步点时;仅剩 0 个时 = frames.len 弹空全部);Rust target = **被弹同步点 frame_index**。仅剩 1 同步点时原生弹空全部帧、Rust 只弹到该同步点帧号。二者在 ≥1 同步点时结果不同,且原生保持"首帧对齐首同步点"不变量(剩余同步点 frame_index-=target 后新头=0),Rust 破坏之。

### C1. add_frame @0x140949190(判等,登记 2 项)

主逻辑判等:record && frames.len>=9999 时 `sync.len<2` →(>=49999 弹首帧)否则 `drop_oldest_sync_point`,无条件 push_back;VecDeque grow/truncate 一致。登记:①原生每若干帧打帧数 log(`hash(count)<0x14f8b588e3690` 间隔,Rust 无);②调用 drop_oldest_sync_point 时继承 G1 分歧。

## 备注

- `update_cursor_position`:PDB 签名 fresh 确认(`as_dyn_eq` @0x1413ee760 签名 = (Res<ActionSetStates>,Res<CursorCapturedPosition>,ResMut<CursorPositionSS>,EventWriter<CursorMovedScreenSpace>)),与 Rust 签名一致;但**正文全内联于宿主**,无独立 VA,无法 fresh 判等 → 标未实现-需重做(与 reviewer I 级结论一致)。
- `button_input.rs`/`mod.rs`:crate 内 bevy_input 等价抽象与注册/调度胶水,按 AGENTS.md 不计独立业务;`reset_all` 语义机器码核对(0x140942b00 尾段 `ButtonInput::reset_all<KeyCode>` @0x14089acb0)一致。
- action_set/input_stream 的 `original_functions` inventory 为空(共享基础模块);本审计按已实现业务函数逐函数过。
- deconstruct 记录字节布局**机器码确认** `mov word [E],0x100`(kind@0,slot@1)+`movlps [E+4]`(value)= Rust `to_bytes` 一致;reconstruct 反编译 kind 读取偏移显示 @E+3,与 deconstruct @E+0 不一致(疑 Ghidra 指针偏移伪影),slot/value 偏移一致,语义(默认 idle+kind/slot 分派)判等——wire 兼容如需逐字节核对建议另核 reconstruct 机器码。

## 结论

- **判等 20 函数**:calculate_input_delta(含 digital_delta)、translate_input_to_cursor_moved、advance_input_replay_frame、normalize/unnormalize_cursor_pos、9 个 action_set 谓词+deconstruct/reconstruct、input_stream 的 sync_point_count/frame_count/index/on_history_sync(判等)、add_frame(判等,2 项登记)。
- **未实现-需重做 8 函数**:translate_input、input_record_replay、control_input_replay、reset_input_state、record_and_replay_input_for_trailers、do_input_replay_ui、update_cursor_position、drop_oldest_sync_point。其中 translate_input 的 Steam 门方向、control_input_replay 的 Load 失败 panic/末同步点 seek、drop_oldest_sync_point 目标选择为**最严重分歧**,直接改变运行时行为。
- 本模块现状不可按"review_passed(限定已实现范围)"维持——已实现范围内的 8 个业务函数逻辑未判等。
