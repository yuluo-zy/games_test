# REBUILD — input 8（restorer 复核/补齐，2026-10-17）

agent: restorer-input-session-worldraster
方法: ghidra `--worker shared` **fresh 反编译**（0x1417cd0a0/0x140941a30/0x140949660/
0x140942b00/0x14083c620/0x14083cdd0/0x1409423d0/0x1413ee760）+ radare2 定点反汇编
（0x14083c620 / 0x140941a30 / 0x140b46d60 等）。证据:
`reports/interaction/world_raster/evidence-restorer-2026-10-17/`（fresh-batch1/2/3 +
disasm-wr/disasm-cases/r2-* + mem-wr）。

本轮在既有 rebuild-input 基础上以 fresh 反编译逐条复核，发现并修正 4 处残余分歧：

## 1. 逐函数 verdict

| 函数 | VA | verdict | 说明 |
|---|---|---|---|
| translate_input | 0x1417cd0a0 | 判等（复核） | 无 Steam 提前 return；mode-tracker 在函数开头按鼠标/键盘事件更新（Deck 仅键盘）；右键手势取最末 InputCursorDelta 长度、just-pressed 复位 0、阈值 3.0；screenshot 修饰仅 `puVar5==0`（无 Steam）分支执行；历史按钮只消费最末事件；EscMenu 门 `tag==2 || (tag==10 && flag!=1)`；滚轮 8.0/1.0。全部与 fresh 对齐（A1–A4 保持）。 |
| control_input_replay | 0x140941a30 | 判等（重写 2 处分歧） | 见 §2.1 |
| drop_oldest_sync_point | 0x140949660 | 判等（复核） | 目标 = 弹后新头同步点 frame_index；无剩余同步点/原本 0 个 → frames.len 弹空；剩余同步点 frame_index -= target（下溢 unwrap_failed）。与实现逐分支一致。 |
| reset_input_state | 0x140942b00 | 判等（修正 D1 游标语义） | 11 qword 置换 + buffer 清零；reader1 有事件 → 不检查/不排空 reader2；原生把被检查 reader 游标写成 events 总计数（完整消费）→ 改用 `read().last()`（旧 `next()` 只推进 1 个事件）。 |
| record_and_replay_input_for_trailers | 0x14083c620 | 判等（修正 E1 状态组装） | 配置字节 +0xfc==1 门；键 0x22/0x1c；开始录制 = 工具栏克隆 + store_parameters + 1 同步点；Records → 只置 mode=None 不清流；E 键 seek 末同步点（rewind=0），无同步点 panic、mode panic、index=frame_index-1；工具栏/相机/翻译前 16B 恢复。 |
| do_input_replay_ui | 0x14083cdd0 | 判等（UI 动作登记） | checkbox "Enable input recording" 写 +0xfc；未启用即 return；"Stream frame count: {count|'-'}"；"`O` - start/stop recording"；"`I` - replay"；"Replay sheep" 按钮（无 Save）。按钮点击后原生调 `Ui::horizontal_with_main_wrap_dyn`（写 +0xfb 点击标志），本实现保留 log 占位（登记）。 |
| input_record_replay | 0x1409423d0 | 判等（修正 B3 两处） | switch(mode^0x8000..)：0/1 都走重建流（命令流有事件 || external || translation+buffer[9] byte3 @0xa3）→ 工具栏克隆 + store_parameters + 1 同步点；case2 越界 panic；11 qword state→buffer；output@+0x30/+0x38/+0x3c；游标 clamp；`buffer[1] *= cursor.y_f32/divisor`；unnormalize；case3 清 22 qword；case4 no-op。 |
| update_cursor_position | PDB 0x1413ee760 | 登记（无独立 VA） | 正文全内联于宿主，无法 fresh 独立判等；PDB 签名 `(Res<ActionSetStates>,Res<CursorCapturedPosition>,ResMut<CursorPositionSS>,EventWriter<CursorMovedScreenSpace>)` 与实现一致。 |

## 2. 本轮修正明细

### 2.1 control_input_replay（0x140941a30，fresh + disasm）
- **Sync 门控**：原生 `cmp 0,[mode]; jno → continue` —— **只有 `mode == AutoRecordPanicRepro`
  (niche 0x8000000000000000)** 才执行 `clone toolbar + store_parameters +
  on_history_sync`；其它模式完全跳过。旧实现无条件打同步点。
- **同步点状态来源**：同步变体事件只携带 record 字节（`movzx r8d, byte [r15+8]`）；状态由
  资源现场组装：工具栏克隆写块首 32 B（Vec {cap,ptr,len} + current@0x18）、翻译资源前
  16 B 写 `+0x20/+0x28`、`store_parameters()` 0x24 B 写 `+0x30..+0x54`。旧实现使用事件内
  的 state（事件 0x30 B 装不下 0x58 B 状态）。
- **Load 载荷**：事件 payload = `PathBuf` + `rewind: u64 @payload+0x18`（反向同步点偏移，
  0 = 末同步点）。`InputReplayCommand::Load` 增加 `rewind` 字段。
- **Load 流程顺序**：try_load（Err → panic "Failed to load an input recording from : {path}"）
  → 整体替换回放状态（mode=Replay、index=-1）→ sync_points 空 panic
  ("No sync points found in replay state") → `rewind > len-1` panic
  ("Not enough player inputs to rewind  history commands. Max rewind {len-1}") →
  mode!=Replay panic ("Can only seek to sync point if replaying but mode is {}") →
  index = `sync[len-1-rewind].frame_index.wrapping_sub(1)` →
  翻译状态 current[0..2] = 同步点 +0x20/+0x28 → toolbar.current = +0x18 低字节 →
  相机参数 +0x30.. 解码为 `CameraRigParametersCache` → `MainCamera::load_parameters`。
- 系统参数顺序按 fresh 修正为 `(ResMut<翻译状态>, ResMut<Toolbar>, ResMut<MainCamera>,
  ResMut<InputRecordReplayState>, EventReader<InputReplayCommand>)`（去掉旧 `_output`）。

### 2.2 input_record_replay / trailer 录制状态
- 同步点状态统一用 `InputTranslationState::from_sync_resources`（工具栏克隆 + 翻译前
  16 B + 相机 0x24 B）；旧实现只覆盖 toolbar.current 字节。
- 命令流判空改为 `read().last()`（原生先写游标为 events 总计数）。

### 2.3 reset_input_state
- `read().last()` 完整消费（见上表）。

## 3. 登记（U，非阻断）

- **`RasterStrokeCmd.strength`（draw_path 侧）**：原生 0x140ae51a9/0x51c5 两条命令路径的
  +0x8c/+0x9c 从未写入（Ghidra 反编译 + radare2 反汇编 + 原始字节 `c7 84 24 8c` 扫描均无），
  读取方 `start_new` 却把它当 param_b。本实现置 0.0 并记录；不影响 begin/extend/commit
  门控与半径。
- **工具栏 modes Vec 克隆**：同步点里是原生 Vec 裸头 {cap,ptr,len}；安全 Rust 不能从数值
  重建所有权。seek 恢复 current（+0x18）与翻译前 16 B；modes 克隆登记宿主接线。
- **`immediately_finish_interpolating`**：MainCamera 尚未恢复该方法（camera 域），seek 后
  调用点登记。
- **do_input_replay_ui "Replay sheep"**：点击后原生 `Ui::horizontal_with_main_wrap_dyn`
  回调 + 写 +0xfb，未恢复（log 占位）。
- **update_cursor_position**：正文内联，无法独立 fresh 判等（宿主级复核项）。

## 4. 自检

- `cargo check -p country-core --lib`：通过（0 error；本域文件 0 warning）。
- `cargo test -p country-core --lib`：**1979 passed / 0 failed**（交付时全 crate；基线 1966 + 本域新增 9 + 并行组增量）（全 crate；含新增
  `sync_state_packs_*` / `sync_command_only_records_in_auto_record_mode` /
  `seek_panics_*` 等）。
