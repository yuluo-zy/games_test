# REVIEW — systems/backend/input 真实逻辑重建(rebuild-input 复验)

审查员: reviewer-interaction | 日期: 2026-10-17
基线: rebuild-input REBUILD.md(fresh 反编译 8 函数 + read_memory 机器码);
`cargo check --lib` exit 0(独立复验;5 个改动文件无新增 warning)。

## 逐项复验

### ① 分类(8 全真业务)— PASS
translate_input/input_record_replay/control_input_replay/reset_input_state/
record_and_replay_input_for_trailers/do_input_replay_ui/drop_oldest_sync_point 均真实业务;
`update_cursor_position` 正文**全内联于宿主**(PDB @0x1413ee760 签名确认,无独立 VA),无法
fresh 判等,按 PDB 语义保留并登记宿主级复核——诚实 ✓。0 宏胶水。

### ② translate_input — PASS(含 LOGIC 两处误判纠正)
- 去 Steam 提前 return:主体恒运行,Steam 仅门控 mode-tracker 更新 + screenshot 修饰跳过;
  `SteamInputActive{present, on_deck}` ✓。
- mode-tracker 移到开头,按输入条件更新(无 Steam/非 Deck → mouse.just_pressed ||
  keyboard.just_pressed +0x38 计数;Steam+Deck → 仅 keyboard),只更新 mouse_keyboard_deadline ✓。
- 右键手势:moved_len = **最末** InputCursorDelta 事件长度(SQRT(x²+y²)),right-just-pressed
  复位 0、pressed 累加最末长度 ✓。
- **LOGIC 纠正**:click = `mouse.pressed(Left)`(pressed.len 扫描 value==0),非
  just_pressed_any ✓;`0x73` → **main_menu(digital[27]@+0xa3)**(机器码 `or [r14+0xa3]`),
  非 ui_cancel(DIGITAL_TARGETS (28,18) = 槽 28 绑定 → ui_cancel@+0x9a,不同目标)✓。
- screenshot 槽 25 修饰仅无 Steam(0x3d);历史按钮只消费最末事件 ✓。

### ③ input_record_replay — PASS
- B1 case2:复制 11 qword 后**额外** `buffer[1] *= cursor.y_f32 / frame.divisor`
  (`CONCAT44(fVar34*(qword1.y), fVar34*(qword1.x))`,fVar34=y/divisor)✓。
- B2 normalize_cursor_delta **去 y==0 守卫**(原生 divps 直接除,y==0 → ±inf/NaN)✓。
- B3 record 分支 modes 0/1 **都重建流**(clone toolbar + store_parameters + 1 同步点
  on_history_sync),非仅 AutoRecordPanicRepro ✓。

### ④ control_input_replay — PASS
- C1 Load 失败 **panic**(anyhow Result::expect,"Failed to load an input recording from :
  {path}"),不再 log::error 继续 ✓。
- C2 Load 成功 mode=Replay + index=-1 → 回退末同步点(index=frame_index−1 + clone toolbar +
  load_parameters + immediately_finish_interpolating);**原生 seek 不写 translation.buffer**
  (buffer 由回放逐帧恢复),已去掉非原写入;camera load/finish 依赖宿主分配(库外登记)✓。

### ⑤ reset/record_replay/do_ui/drop_oldest — PASS
- D1 reset_input_state:reader1(InputCursorMoved)有事件时**不排空** reader2
  (InputReplayCommand);两 reader 都空才不复位 ✓。
- E1 第二键 seek 末同步点(index=frame_index−1+toolbar.current);E2 无同步点 **panic**
  (expect_failed "No sync points found in replay state",与 fresh 反编译一致——任务稿"不
  panic"为误);E3 O 键 Records 只置 mode=None,**不清流** ✓。
- F1 checkbox 未启用即 return 只画框;F2 **无 "Save" 按钮**,`` `I` - replay `` 串
  (非 `E`),"Stream frame count" 按 mode==None 显示 count 否则 "-" ✓。
- G1 drop_oldest_sync_point:目标 = **弹后新头同步点 frame_index**(仅剩 0/原本 0 时 =
  frames.len 弹空全部);剩余同步点 frame_index −= target(checked_sub unwrap 下溢)✓。

## 结论与 DISPATCH 建议

- input 真实逻辑重建复验通过:**需重做项(7+update_cursor_position 登记)全部闭合**,编译
  exit 0(独立复验)。
- 建议:input 需重做项 → 闭合;update_cursor_position 正文内联为宿主级复核项;
  toolbar Vec clone / camera 40B 参数映射依赖宿主分配,登记待接。
