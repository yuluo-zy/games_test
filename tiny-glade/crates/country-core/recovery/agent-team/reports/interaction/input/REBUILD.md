# REBUILD — systems/backend/input + input/(input_stream) 真实逻辑重建

restorer: rebuild-input | 日期: 2026-10-17
方法: ghidra `--worker shared` **fresh 反编译**（8 函数，`fields=decompiled_code,callees`）+ `read_memory` 机器码核对
（translate_input 4 段 / input_record_replay / do_input_replay_ui 串 / drop_oldest_sync_point）。
证据: `evidence/logic-input-rebuild-2026-10-17/`（result-rebuild-batch1 + mem1..5 + fresh 反编译 .c.txt）。

## 1. 分类（8 个 `未实现-需重做`）

| 原函数 | VA | 分类 | 结论 |
|---|---|---|---|
| `translate_input` | 0x1417cd0a0 | **真实业务** | 重建 |
| `input_record_replay` | 0x1409423d0 | **真实业务** | 重建 |
| `control_input_replay` | 0x140941a30 | **真实业务** | 重建 |
| `reset_input_state` | 0x140942b00 | **真实业务** | 重建 |
| `record_and_replay_input_for_trailers` | 0x14083c620 | **真实业务** | 重建 |
| `do_input_replay_ui` | 0x14083cdd0 | **真实业务** | 重建 |
| `update_cursor_position` | PDB @0x1413ee760（正文全内联） | **真实业务，无独立 VA** | 保留（无法独立判等） |
| `drop_oldest_sync_point` | 0x140949660 | **真实业务** | 重建 |

8 个全部是真实业务（含真实算法/控制流/调用链），**无宏生成胶水**。`update_cursor_position` 例外：
PDB 签名（`as_dyn_eq<FunctionSystem<...>>` @0x1413ee760）确认、正文**全内联于宿主**，无独立 VA，
无法 fresh 判等；按 PDB 签名 + 描述语义保留现有实现，登记为宿主级复核项。

## 2. 重建明细（fresh 反编译为唯一依据）

### A. translate_input 0x1417cd0a0（最严重）
- **A1 去掉 Steam 提前 return**：原生无 return，Steam 只门控 mode-tracker 更新（0x1417cd112..c1fe
  主体与 Steam 分支间无 return）与 screenshot 修饰跳过。`SteamInputActive` 扩为
  `{present, on_deck}`。
- **A2 mode-tracker 移到函数开头**，按输入条件更新：无 Steam/非 Deck →
  `mouse.just_pressed || keyboard.just_pressed`（+0x38 计数）；Steam+Deck → 仅 keyboard。
  只更新 `mouse_keyboard_deadline = now`（+0x10/+0x18），controller_deadline 不动
  （此前无条件 `new_default_mouse_keyboard` 于 gate 末尾）。
- **A3 右键手势**：`moved_len` = **最末** `InputCursorDelta` 事件长度（`SQRT(x²+y²)`，非 fold 和）；
  right-just-pressed 时 MouseMoved **复位 0**，pressed 时累加最末长度。
- **A4 screenshot 修饰**：仅无 Steam（`test r12,r12; jnz` 跳过）执行槽 25 修饰。
- **额外修正（LOGIC 判等不精确）**：
  - click 扫描 = `mouse.pressed(Left)`（pressed.len()!=0 扫描 value==0），非 `just_pressed_any`。
  - 硬编码 `0x73` → **main_menu**（digital[27]@+0xa3，机器码 `or [r14+0xa3]`），非 ui_cancel
    （LOGIC A 记"0x73→0x9a"有误；槽 28 绑定才 → ui_cancel digital[18]@+0x9a）。
  - 历史按钮只消费**最末**事件（原生读 last 事件 payload@+8）。
- demo 键目标槽/极性经机器码核验**正确**（0xa6 = `pressed(0xab) && pressed(0x3d)`，setne）。

### B. input_record_replay 0x1409423d0
- **B1 case2 回放补 `buffer[1] *= cursor.y_f32 / frame.divisor`**（复制 11 qword 后
  `CONCAT44(fVar34*(qword1.y), fVar34*(qword1.x))`，fVar34 = cursor.y_f32 / divisor@+0x44）。
- **B2 normalize_cursor_delta 去 y==0 守卫**：原生 `divps` 直接除（y==0 → ±inf/NaN）。
- **B3 record 分支 modes 0/1 都重建流**（非仅 AutoRecordPanicRepro），重建 = clone toolbar +
  store_parameters + 1 同步点（on_history_sync）。

### C. control_input_replay 0x140941a30
- **C1 Load 失败 panic**（anyhow `Result::expect`，消息 "Failed to load an input recording
  from : {path}"），不再 log::error 继续。
- **C2 Load 成功 seek 末同步点**：mode=Replay、index=-1 → `index = 末同步点 frame_index - 1`
  + clone toolbar + load_parameters + immediately_finish_interpolating。`seek_to_sync_point`
  恢复 index + toolbar.current；**原生 seek 不写 translation.buffer**（buffer 由回放逐帧恢复），
  已去掉非原写入；camera load/finish 依赖宿主分配（库外登记）。

### D. reset_input_state 0x140942b00
- **D1 事件排空短接**：reader1（InputCursorMoved）有事件时**不排空** reader2
  （InputReplayCommand）；两 reader 都空才不复位。

### E. record_and_replay_input_for_trailers 0x14083c620
- **E1 E 键 seek 末同步点**（index = frame_index-1 + toolbar.current；camera load/finish 登记）。
- **E2 无同步点 panic**（expect_failed "No sync points found in replay state"）。
  （任务稿"无同步点不 panic"与 fresh 反编译矛盾；以 fresh 反编译为准——原生 `expect_failed` panic。）
- **E3 O 键 Records 只置 mode=None，不清流**（原生 default 分支 `*plVar1 = None niche`）。
- O 键开始录制补 1 同步点（与 input_record_replay B3 一致）。

### F. do_input_replay_ui 0x14083cdd0
- **F1 checkbox 未启用即 return，只画 checkbox**。
- **F2 无 "Save" 按钮**；"Replay sheep" 按钮保留；`"`E` - replay"` → `"`I` - replay"`
  （原生串 `0x6572202d20604960`="`I` - re..."）；"Stream frame count" 按 mode==None 显示
  count、否则 "-"。

### G. drop_oldest_sync_point 0x140949660
- **G1 目标 = 弹后新头同步点 frame_index**（`LAB_140949775` `uVar3 = *(new_head+0x58)`）；
  仅剩 0 个（或原本 0 个）同步点时 = `frames.len` 弹空全部（`puVar6 = in_RCX + 7`）。
  剩余同步点 `frame_index -= target`（`unwrap_failed`）。相关 2 个测试断言同步更新。

## 3. 自检

- **编译**：`cargo check --manifest-path crates/country-core/Cargo.toml` **通过（0 error）**；
  我的 5 个改动文件无新增 warning（input_replay_for_trailers 的 `mut translation` 已去）。
- **测试**：`cargo test --lib` 目标被 **mod_key.rs / critters.rs 等预存测试编译错误**阻断
  （`update_mod_key_state` 测试以 `&mut` 调用 ResMut 签名、GraphicsDevice 结构体构造），
  非本次改动引入（git status 无我改这些文件）。drop_oldest_sync_point 新断言用独立脚本
  数值验证通过（弹 500 → 9500 帧 / 弹 1050 → 0 帧）；normalize y==0 → ±inf、scale_u64_as_vec2
  验证通过。
- **不改判等函数**：calculate_input_delta / translate_input_to_cursor_moved /
  advance_input_replay_frame / normalize/unnormalize_cursor_pos / action_set 9 谓词 +
  deconstruct/reconstruct / input_stream 的 sync_point_count/frame_count/index/on_history_sync /
  add_frame **未动**。
- **不改他模块**：本次只改 5 个 input 文件。
- **登记**：toolbar modes Vec clone / camera load_parameters+immediately_finish_interpolating
  依赖宿主真实分配（Vec 指针 / camera 40B 参数映射），库内 `let _ = (...)` 保留待接。

## 4. 遗留

- `update_cursor_position` 正文内联，无法 fresh 独立判等（宿主级复核项）。
- `input_record_replay` B3 同步点状态子字段（toolbar vec ptr/cap、camera 40B）编码依赖宿主。
- LOGIC.md 中 click 扫描"判等"、`0x73→0x9a` 两处结论经 fresh 反编译修正（见 A 节）。
