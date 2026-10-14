# input/input_stream.rs

> 文件作用：**有界输入录制环（trailer 回放）**。`InputStream`（64 B）= 两个 VecDeque
> （cap-first）：sync_points（0x60 B：InputTranslationState 88 B + frame_index）与 frames
> （0xd0 B InputFrame，内含 `Vec<[u8;12]>` = `MainActionSet::deconstruct` 打包动作集）。
> `add_frame` 录帧（9999/50000 裁剪 + 同步点裁剪）；`on_history_sync` 推同步点（≥9/10000 删最老）；
> `drop_oldest_sync_point` pop 最老并重基帧索引（panic `assertion failed: self.frames.pop_front().is_some()`）；
> `InputRecordReplayMode` 5 变体（AutoRecordPanicRepro/Records/Replay/DisableInput/None），
> `try_load` 从 JSON 载入。
> 证据：add_frame/on_history_sync/drop_oldest_sync_point/index/fmt/序列化反编译 + 原字节。

函数总计 24：业务 10 / 内联无独立符号 5 / 派生胶水 9

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `deserialize::visit_str<serde_json>` | 0x1413fe800 | 20 | 输入流 JSON 变体名匹配 | **O** | serde_json 变体匹配 |
| `serialize<Compact>` | 0x1402e0060 | 20 | `{replay_initial_state, first_frame}` | **O** | 两键（0x1429b3d4e/0x1429b3d62） |
| `serialize<Pretty>` | 0x1413fe6f0 | 20 | `{translation_state, prev_normalized_cursor_position, time_delta_seconds, main_action_set_value}` | **O** | 四键（0x00/0xc0/0xc8/0x68） |
| `InputStream::sync_point_count` | 0x140949170 | 119 | `[+0x18]` | **O** | 原字节 `mov rax,[rcx+0x18]` |
| `InputStream::frame_count` | 0x140949180 | 127 | `[+0x38]` | **O** | 原字节 `mov rax,[rcx+0x38]` |
| `InputStream::add_frame` | 0x140949190 | 131 | 录帧入环 + 裁剪 | **O** | 0xd0 B push；recording≥9999 时 sync<2 且≥50000 删最老帧（释放内层 Vec），否则 drop_oldest_sync_point；hash 限频日志 |
| `InputStream::last_frame_mut` | 0x140949480 | 152 | 末尾帧 | **O** | 原字节 `((count+head-1)-(>=cap?cap:0))*0xd0 + ptr` |
| `InputStream::on_history_sync` | 0x1409494c0 | 158 | 推同步点（截断/裁剪） | **O** | 非 recording 截断两队列；sync≥9 或 frames≥10000 删最老；push 11 qwords 翻译态 + 帧数 |
| `InputStream::drop_oldest_sync_point` | 0x140949660 | 178 | pop 最老同步点 + 重基 | **O** | 删帧至其索引；重基剩余 +0x58 frame_index；panic pop_front 断言 |
| `InputRecordReplayState::try_load` | 0x1409499c0 | 231 | JSON 载入 80 B 状态 | **I** | 成功写 8 qwords + 哨兵 -1；失败 Err(-0x8000000000000000) |

## 内联/无独立符号函数

- `InputReplayInitialState::new`（行 62）— **O**（构造）
- `InputStream::new`（行 108）— **O**（空队列构造）
- `InputStream::sync_point`（行 123）— **O**（按索引取同步点）
- `InputRecordReplayState::new`（行 224）— **O**（80 B 默认态）
- `InputRecordReplayState::seek_to_sync_point`（行 242）— **I**（名称推断，跳到同步点）

## 派生胶水（9 项：default、deserialize、expecting、fmt、index、visit_map、visit_seq、visit_str 等）

- `impl$2::index`（0x140949970）— **O**：`((index+write_idx)%cap)*0xd0 + ptr`，越界 panic `Out of bounds access`(0x142ab3224)
- `impl$8::fmt`（0x14094f2d0）— **O**：InputRecordReplayMode Display，5 变体名（AutoRecordPanicRepro/…/None）
