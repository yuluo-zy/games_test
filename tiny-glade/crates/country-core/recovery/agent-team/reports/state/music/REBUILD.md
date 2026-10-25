# REBUILD — systems/audio/music（真实控制流重建）

> 重建者: rebuild-session。日期: 2026-10-17。
> 依据: 2026-10-16-logic-verify-protocol.md + LOGIC.md 分歧点。
> 全部以 Ghidra shared **fresh 反编译/反汇编**为准。

## 分类结论

| LOGIC 需重做函数 | 分类 | 结论 |
|---|---|---|
| `music_audio_system` 0x140a72810 | 真业务（photomode 分支 start 实例） | **核对后判等，无需改码** |

## `music_audio_system`（0x140a72810）— fresh 反汇编核对

LOGIC 分歧点声称 "Rust mode==2 起 first_person、else 起 photomode（start 实例互换）"。
**fresh 反汇编逐分支核对（Ghidra disassemble_function 0x140a72810）否定了该分歧**：

```
140a72ced  CMP byte ptr [RAX + 0x9],0x2      ; photomode_state+9 == 2（camera_mode==2）
140a72cf1  MOV dword ptr [R13],R12D
140a72cf5  JNZ 0x140a72ecb                    ; !=2 → else
140a72cfb  LEA RCX,[RBX + 0x70]              ; ==2 → start 目标 = photomode(+0x70)
140a72cff  CALL 0x140a57300                   ; start_if_not_playing(photomode)
140a72ecb  LEA RCX,[RBX + 0xa0]              ; else → start 目标 = first_person(+0xa0)
140a72ed2  CALL 0x140a57300                   ; start_if_not_playing(first_person)
```

即原生：mode==2 → 起 photomode(+0x70)、停 first_person(+0xa0)；else → 起 first_person(+0xa0)、
停 photomode(+0x70)。

当前 `music_system.rs::update_music_audio`（mtime 2026-10-15，早于审计）逐分支一致：

```rust
if camera_mode as u8 == 2 {
    state.player.photomode.start_if_not_playing();      // 起 photomode（+0x70）✓
    state.player.first_person.stop(StopMode::Immediate); // 停 first_person（+0xa0）✓
} else {
    state.player.first_person.start_if_not_playing();   // 起 first_person（+0xa0）✓
    state.player.photomode.stop(StopMode::Immediate);   // 停 photomode（+0x70）✓
}
```

结论：**start 实例未互换**，LOGIC 的 "Rust 反之" 系 verifier 误读 Rust 源码。本函数无需改码，
维持判等。其余（target_richness snap、mode==0x13 门、time_continuous_lerp、MusicRichness/
NewGladeTime/PreviewGladeType、EscMenuState tag|8==10、bevy Time delta、preview 分支、
stop_photomode_snapshots）LOGIC 已判等，未触碰。

## 汇总

- music 模块需重做清单 1 项：核对后为 verifier 误判，无代码改动。
- music_player 8 判等函数与 music_debug_gui 判等函数全部未动。
- 编译状态：music_system.rs 无编译错误（其余模块并行 agent 的 in-flight 错误与本批无关）。
