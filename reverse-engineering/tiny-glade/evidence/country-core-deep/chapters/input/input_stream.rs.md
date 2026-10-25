# input/input_stream.rs

> 文件作用：**有界输入录制环（trailer 回放）**。`InputStream`（64 B）= 两个 VecDeque
> （cap-first）：sync_points（0x60 B：InputTranslationState 88 B + frame_index@+0x58）与 frames
> （0xd0 B InputFrame，内含 `Vec<[u8;12]>` = `MainActionSet::deconstruct` 打包动作集）。
> 算法深挖（2026-10-15，详见 `evidence/country-core-deep/input/ALGORITHMS.md` §2）：
> `add_frame` 录帧（record && len≥9999 时：sync<2 && len≥50000 → 弹最老帧并释放内层两堆指针，
> 否则 drop_oldest_sync_point；然后 0xd0B 拷贝推尾 + 乘散列限频日志 "Stream frame count: "）；
> `on_history_sync` 推同步点（!record → 截断双队列；sync≥9 || frames≥10000 → 丢最老同步点；
> 推 12 qword = 11 翻译态 + frames.len）；
> `drop_oldest_sync_point` pop 最老同步点 + 按其 frame_index 剪帧（帧耗尽下溢 → panic
> `assertion failed: self.frames.pop_front().is_some()`）+ 剩余同步点 frame_index 重基
> （小于被删数 → unwrap_failed panic）；
> `InputRecordReplayMode` 5 变体（0 AutoRecordPanicRepro/1 Records/2 Replay/3 DisableInput/4 None，
> niche `0x8000000000000000+i`；Display 串权威；转移表见 ALGORITHMS.md §2.6——input_record_replay
> 系统 case 0/2/3/4 即 AutoRecordPanicRepro/Replay/DisableInput/None，非 DIGEST 旧标注 Idle/Replaying/
> Recording/Finished）；`try_load` JSON 载入（两错误路径 "Failed to read the file"/"Failed to
> deserialize"，成功写 8 qword + mode=Replay + index=-1）。InputFrame serde **字段序**
> = `{translation_state(0)@0x00, prev_normalized_cursor_pos(1)@0xc0, time_delta_seconds(2)@0xc8,
> main_action_set_value(3)@0x68}`（visit_str 0x1413fe800 variant 序 = serialize<Pretty> 0x1413fe6f0
> 写序，本轮闭合；**prev 名实际 26 字符**，§6.3）；回放读取状态块 @+0x68..0xb8
> （偏移冲突裁决见 ALGORITHMS §6.2——不同子结构）。
> 证据：add_frame/on_history_sync/drop_oldest_sync_point/index/fmt/try_load/serialize×2
> 反编译（本轮新采 0x1409499c0/0x140949970/0x14094f2d0/0x1413fe6f0/0x1402e0060 + wp15 b1_input）
> + visit_str 机器码手解（`input/decomp/hand-0x1413fe800-visitstr.txt`）。

函数总计 24：业务 10 / 内联无独立符号 5 / 派生胶水 9

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `deserialize::visit_str<serde_json>` | 0x1413fe800 | 20 | InputFrame serde 键→字段序号匹配 | **O** | 机器码手解（tg_ci1 NotFound，`input/decomp/hand-0x1413fe800-visitstr.txt`）：长度跳表 @0x142c5baa0（len-0x11 ≤9 分派，有效 17/18/21/26）+ 后缀共享 SIMD 双窗（len17 `translation_stat`+byte `e`→0；len21 `main_action_set_`+[rdx+5]`action_set_value`→3；len18 `time_delta_secon`+word `ds`→2；len26 `prev_normalized_`+[rdx+0xa]`lized_cursor_pos`→1）；未命中/其他长度→out[1]=4（unknown）。**serde 字段序 = [translation_state(0), prev_normalized_cursor_pos(1), time_delta_seconds(2), main_action_set_value(3)]** |
| `serialize<Compact>` | 0x1402e0060 | 20 | `{replay_initial_state, first_frame}` | **O** | 两键（0x1429b3d4e/0x1429b3d62）；本轮 MCP 复采（1.1KB）确认键序 |
| `serialize<Pretty>` | 0x1413fe6f0 | 20 | InputFrame 4 键 | **O** | 本轮 MCP：`translation_state`(17@0x142c5b3d0, val@+0x00)、`prev_normalized_cursor_pos`(26@0x142c5b3e1, val@+0xc0)、`time_delta_seconds`(18@0x142c5b3fb, val@+0xc8=200)、`main_action_set_value`(21@0x142c5b40d, val@+0x68)；Pretty 深度缩进（LF+空格循环 func_0x142923c00） |
| `InputStream::sync_point_count` | 0x140949170 | 119 | `[+0x18]` | **O** | 原字节 `mov rax,[rcx+0x18]` |
| `InputStream::frame_count` | 0x140949180 | 127 | `[+0x38]` | **O** | 原字节 `mov rax,[rcx+0x38]` |
| `InputStream::add_frame` | 0x140949190 | 131 | 录帧入环 + 裁剪 | **O** | 0xd0 B push；record && len≥9999（回绕判）时 sync<2 && ≥50000 删最老帧（释放内层 Vec @+0/@+0x18 指针），否则 drop_oldest_sync_point；满则 grow；slot=((len+head)-(cap≤?cap:0))*0xd0+ptr；len++；乘散列限频日志（×0x5d4e8fb00bcbe61d 低区 <0x14f8b588e3690 才打，经 TweakerView，行 33） |
| `InputStream::last_frame_mut` | 0x140949480 | 152 | 末尾帧 | **O** | 原字节 `((count+head-1)-(>=cap?cap:0))*0xd0 + ptr` |
| `InputStream::on_history_sync` | 0x1409494c0 | 158 | 推同步点（截断/裁剪） | **O** | !record 截断两队列（len=0 释放内容）；sync≥9 或 frames≥10000 删最老；push 12 qword（11 翻译态 @0..0x50 + frames.len@0x58），满则 grow |
| `InputStream::drop_oldest_sync_point` | 0x140949660 | 178 | pop 最老同步点 + 重基 | **O** | pop 头（head 回绕 + 内容释放）；按被弹同步点 frame_index 循环弹帧（释放每帧内层 Vec 两指针；frames 下溢 → panic `assertion failed: self.frames.pop_front().is_some()` 路径 0x140949940 折叠）；剩余同步点 frame_index < 被删数 → unwrap_failed panic，否则 -= 被删数（重基） |
| `InputRecordReplayState::try_load` | 0x1409499c0 | 231 | JSON 载入 80 B 状态 | **O** | 本轮 MCP：读串失败 → Err(i64::MIN 哨兵) + "Failed to read the file"(0x142ab35f0 23)；from_str 失败 → Err + "Failed to deserialize"(0x142ab3607 21)；成功写 8 qword（两 VecDeque）+ mode=0x8000000000000002(Replay) + index=-1（写入序 {ptr,cap,len,head} 与运行时 cap-first 差异登记 §6.5） |

## 内联/无独立符号函数

- `InputReplayInitialState::new`（行 62）— **O**（构造；serde 10 字段名 window_size/cursor_depth/
  cursor_pos_ws/center_pos_ws/ui_settings/sheep_positions/sheep_rotations/app_mode/toolbar/camera，
  串表 0x142c5b3d0 区）
- `InputStream::new`（行 108）— **O**（空队列构造）
- `InputStream::sync_point`（行 123）— **O**（按索引取同步点）
- `InputRecordReplayState::new`（行 224）— **O**（80 B 默认态；trailer 重建写 mode=0x8000000000000000）
- `InputRecordReplayState::seek_to_sync_point`（行 242）— **O**（内联于 control_input_replay
  0x140941a30：sync_points 空 → panic "No sync points found in replay state"(0x142ab2740)；
  mode!=2 → panic "Can only seek to sync point if replaying but mode is "(0x142ab3658, input_stream.rs)
  + fmt(mode)(0x14094f2d0)；成功 index = sync.frame_index-1 并恢复游标/Toolbar/相机）

## 派生胶水（9 项：default、deserialize、expecting、fmt、index、visit_map、visit_seq、visit_str 等）

- `impl$2::index`（0x140949970）— **O**：`index < frames.len → ((index+head)%cap 等价式)*0xd0 + ptr`，
  越界 panic `Out of bounds access`(0x142ab3224, 20 字符)
- `impl$8::fmt`（0x14094f2d0）— **O**：InputRecordReplayMode Display，5 变体名权威
  （0=AutoRecordPanicRepro@0x142ab3ad0(20)、1=Records@0x142ab3b08(7)、2=Replay@0x142ab3b38(6)、
  3=DisableInput@0x142ab3b44(12)、4=None@0x142ab3b50(4)）
