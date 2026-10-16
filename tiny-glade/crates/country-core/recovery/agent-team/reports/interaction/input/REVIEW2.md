# REVIEW2 — systems/backend/input(二轮复验与裁决)

审查员: reviewer-interaction | 日期: 2026-10-15
基线: restorer-interaction RESTORE.md + 新增 5 文件;编译 `--lib` 0 error,本组 12 测试过。
本轮:①裁决 translate_input_to_cursor_moved 方向冲突;②复验 Shape B、input_record_replay 帧偏移等。

## ① 裁决:translate_input_to_cursor_moved 方向 — **restorer 正确,一轮主张撤回**

新证据(本次取证):
1. **地址一致**:enriched PDB `translate_input_to_cursor_moved.rs.enriched.json.gz` 的
   `compiled_instances[0].va = 5380340352 = 0x140b17a80`(`rva 0xb17a80`)。协调器消息里
   "5380340352=0x140b16a80" 是**十六进制换算笔误**(0x140b16a80=5380336256);真实地址就是
   一轮反编译的 0x140b17a80,**无 0x1000 差**,不存在 ICF/别名问题。
2. **PDB 类型签名**:enriched `structures.json.gz` 的 FunctionSystem 类型 =
   `(Res<CursorPositionSS>, Res<WindowSize>, Res<CursorCapturedPosition>, ResMut<ActionSetValues>,
   EventReader<InputCursorMoved>, EventWriter<InputCursorDelta>)`。且两个调用者(本次
   get_functions 复核)签名完全一致:
   - 0x140986070 = `run_unsafe<FunctionSystem<...同 6 参...>>`
   - 0x1414c22d0 = `run_without_applying_deferred<InfallibleSystemWrapper<...同 6 参...>>`
   → **方向:consumes `EventReader<InputCursorMoved>`、produces `EventWriter<InputCursorDelta>`**。
   `InputCursorDelta` 是原事件(PDB named_symbol_entities 有实体),一轮"自造类型/方向反转"
   主张**不成立**,撤回应部分采纳。
3. **夹取上界 = WindowSize(机器码定案,证据 `evidence/b17a80-clamp-disasm.txt`)**:
   `movsd xmm3,[rdx]`(param_2=WindowSize 的 u32 对)→ `unpcklps` → `orpd xmm3,xmm1` →
   `subpd xmm3,xmm1` → `cvtpd2ps xmm3,xmm3`,其中 xmm1 = `___xmm_43300000000000004330000000000000`
   = `0x4330000000000000`×2 = **2^52 的 i32→f64 转换魔数**(非"原点 192")。decomp 里
   `(float)(auVar26|auVar13)-192` 是 Ghidra 对魔数路径的误译;真实语义 = `(f32)width,(f32)height`。
   一轮"上界 = cursor-192"主张**不成立**。当前 `window.as_vec2()` 夹取 **正确**。
4. **捕获/非捕获路径**:`cmp dword [r12],0; je not_captured`(param_3 = CursorCapturedPosition
   判别),与 `captured.0.is_some()` 一致。

### ⚠️ 二轮新 FIX(restorer 引入的回归):捕获路径 tag 2 处理**反了**

机器码(证据 `b17a80-clamp-disasm.txt` + fresh decomp):
- **捕获路径**(0xb8..0x129):`cmp dword [r15+8], 1; jne 0x115(add)` → **tag==1 跳过,tag 0 和
  tag 2 都累加原始值**。
- **非捕获路径**(0x12b..):`cmp ebp, 2; jne process` → tag 2 跳过;tag 0 夹取;tag 1 差分。

即语义:tag 2(RelativeWhileCaptured)在**捕获时累加、非捕获时跳过**。
restorer 的 `FIX-2 采纳子项`把捕获分支写成只累加 `Relative`(tag 0)、跳过
`Absolute|RelativeWhileCaptured`(tag 1/2)——**tag 2 在捕获路径被错误跳过**。一轮原码
`Relative | RelativeWhileCaptured => delta += value` 才是对的;restorer 把它改坏了。

**FIX(具体)**:`fold_cursor_events` 捕获分支改回:
```rust
if captured {
    match event {
        InputCursorMoved::Relative(value) | InputCursorMoved::RelativeWhileCaptured(value) => delta += value,
        InputCursorMoved::Absolute(_) => {}
    }
}
```
同步 `update_cursor_position::fold_cursor_moved` 的捕获语义(它目前相对事件只加 Relative;
update_cursor_position 为内联 I,可留,但与 translate 同帧消费同一流,建议对齐)。

## ② 其余复验

### digital_delta Shape B(槽 20..25)— **OK(采纳正确)**

`auto_repeat && state.disc == 1`:`timer = val+dt; if timer < 0.2 {disc 1, val=timer} else
{disc 2, val=0.1}`。与 fresh 0x1416feeb0 槽 20 段机器码 `fVar7+=dt; bVar5=fVar7<0.2;
if(!bVar5) fVar7=0.1; iVar6=2-bVar5` 逐一对应 ✓。`uses_ui_auto_repeat(20..=25)` 与字节
0x44..0x49 的 Shape B 段一致 ✓。测试覆盖连发时序 ✓。

### calculate_input_delta 其余 — OK(一轮已核,未变)

analog 6 拷贝、previous@0x30/current@0x88 映射、press→0/idle→4/release→3/repeat 0.1/0.05
均与 fresh decomp 一致。

### input_record_replay @0x1409423d0 — **PARTIAL**

- mode 命名:共享 `InputRecordReplayMode`(AutoRecordPanicRepro=0/Records=1/Replay=2/
  DisableInput=3/None=4)取自 Display@0x14094f2d0,章节 Idle/Recording/Finished 命名错误,
  **当前命名正确**。
- 状态机分派:0/1 录制建帧、2 回放(+越界 panic "Out of bounds access"、f32→u32 clamp
  0..u32::MAX)、3 清零翻译状态、4 no-op——与章节 case 映射一致 ✓。
- **帧偏移待对账**:RESTORE 声称状态块 @0x68..0xb8,章节 @0x58..0xa8(差 0x10);且 Rust
  `InputFrame` 结构体字段实际从 @0x00 排布(无原版 @0x00..0x30 的 0x30 B 头),不逐字节匹配
  原 0xd0 B 帧。内部录制/回放同型自洽,但录制文件 serde/ABI 不保真——**待取证**,不阻塞
  内部链。
- 归一化 `normalize_cursor_delta`/`unnormalize_cursor_pos` 为近似(矩阵 -0.5/0.5 来自
  RNE 常量,精确公式待取证,restorer 已标注)。
- **缺失**:control_input_replay 的同步点 seek(mode 检查 + "Can only seek to sync point"、
  "Not enough player inputs to rewind" panic、sync point 恢复 Toolbar/相机)、try_load 的
  录制文件 serde、case 0 的 `param_2` 打包动作集来源。RESTORE 已列待取证,维持 PARTIAL。

### reset_input_state @0x140942b00 — **PARTIAL**

- buffer→current 拷贝 + buffer 清零 ✓(章节"11 qword 置换";当前为恒等拷贝,是否真置换
  待取证)。
- **章节"按最后 sync point 修剪 InputStream"未实现**(当前只排空事件)。待取证/补。

### translate_input @0x1417cd0a0 — **GAP(结构壳,主体未恢复)**

- 逐动作键码绑定表缺失:`BindingsTable::default` 把证据里的 26 键序列"平铺进前 30 槽",
  **逐槽映射未取证**;模拟槽方向表(单键 `y -= step`)为推断。
- Steam 路径用 `SteamInputActive` 布尔代替(`Option<Res<SteamClientResource>>` 未恢复);
  MouseWheel 未消费;UiHistoryButtonPressed 语义 U。RESTORE 已如实列待取证。

### input_replay_for_trailers / update_cursor_position — 结构近似

`record_and_replay_input_for_trailers`/`do_input_replay_ui` 为结构实现(日志串/键门),
同步点 seek 与录制文件保存待取证;`update_cursor_position` 为内联 I 推断。均 PARTIAL/GAP。

## 结论与对 DISPATCH 建议

- 裁决已定:**方向与夹取 restorer 正确,一轮主张撤回**;新增 1 处确认 FIX(捕获路径 tag 2
  应累加而非跳过,restorer 改反了)。
- input 模块本批已闭合函数骨架、编译/测试过,但 translate_input 键绑定、Steam、录制文件
  serde、同步点 seek、逐槽方向表仍是真实缺口,**不能按现状宣布完成**。
- 建议:状态维持 `in_progress`(待 re-analyst 补键绑定表/方向表/serde),同时
  **fix_pending 项 = 捕获路径 tag 2**(交 restorer 修);修后复验一次。
