# B 组 input DELIVERY（2026-10-16 第三轮实现）

> 实现对象：input-DELTA.md 的真实剩余缺口。组：B。文件：`systems/backend/input/*`、
> `resources/window.rs`（中央 agent 批准加 `CursorMovedScreenSpace`）、`src/lib.rs`（debug 接线）。

## 真实代码接线

1. **`update_cursor_position` 按 PDB 签名重做**（`update_cursor_position.rs`）：
   首签 `(Res<ActionSetStates>, Res<CursorCapturedPosition>, ResMut<CursorPositionSS>,
   EventWriter<CursorMovedScreenSpace>)`（into_configs @0x1413ee760）。读
   `ActionSetStates.analog[0]`（input_space_cursor_move，经 translate_input_to_cursor_moved
   折叠 + calculate_input_delta 拷贝），非负 clamp 累加进 `CursorPositionSS`，发
   `CursorMovedScreenSpace`。`CursorPosition`（u32）保留给 `input_record_replay`（param_6）。
2. **`CursorMovedScreenSpace` 事件**：`resources/window.rs`（原类型
   `country_core::resources::window::CursorMovedScreenSpace`，中央 agent 批准）。
3. **`UiHistoryButtonPressed` 消费**（`calculate_input_delta.rs` + `translate_input.rs`）：
   事件改带 `u8` 载荷（O decomp 0x1417cd0a0 事件槽 @+8：`0`=undo → digital[16]/+0x98，
   非零=redo → digital[17]/+0x99）；translate_input 死参数改为真实消费（与 0xa7 恒启键同段）。
4. **`mod_key` 模块**（`mod_key.rs`，新建）：`Modifiers` 资源（native `{u32@+0, 字节@+1}`，
   位标志字节组）+ 两个已证谓词 `disable_snapping`（0x1408c5680，读字节@0）与
   `is_disable_snapping_just_pressed`（0x140af9bf0，读字节@1）。
5. **GameProfile 键位 default 表逐字节复核**：`BindingsTable::default` 的 40 槽
   `{key, mouse}` 与 native default @0x14089b110（内联代码逐槽写）rne-read 全部对上
   （0x2a,0x26,0x14,0x17,0x18,0x24 + 10×0xc3 区 + 0x07..0x23 + 0x27..0x4e + 0xc3 +
   鼠标 {1,2,1} @0x1bc..0x1c4）——注释从"抽查结构"升级为"逐字节核对"。

## candidate 链测试

- `update_cursor_position.rs`：`fold_cursor_state`（增量累加 + 非负 clamp + NaN→0）、
  旧 `fold_cursor_moved` 保留测试。
- `mod_key.rs`：两个谓词按字节位读（byte0/byte1 判别，高位不影响）。
- `translate_input.rs`：既有 default 表 / analog / digital / camera_rotate 测试保持；
  新增 history 折叠走 `digital[16]/[17]` 分支。

## 原侧验证

- update_cursor_position 签名：`cursor-update-procedures.json` into_configs @0x1413ee760
  四参数（Res<ActionSetStates>/Res<CursorCapturedPosition>/ResMut<CursorPositionSS>/
  EventWriter<CursorMovedScreenSpace>）✓。
- UiHistoryButtonPressed 载荷：translate_input decomp 0x1417cd0a0 行 780..833，
  `*(char*)(event+8)==0 → +0x98=1 else +0x99=1`（= digital[16]/[17]）✓。
- mod_key 谓词：0x1408c5680 `movzx eax, byte[rcx]`；0x140af9bf0
  `movzx eax, byte[rcx+1]`（手解 + 0x140af9bf0.asm）✓。
- default 键位表：0x14089b110 内联代码逐槽 imm32，rne-read 全表核对 ✓。

## 仍缺项（待取证 / 组外登记）

- **`update_mod_key_state` @0x140af9c00 producer**：7 组按键 handler 跳表与 u32@+0 字段
  语义未解码 → 未造空壳，登记缺口；`mod_key` + `update_cursor_position` 的调度对因此
  未完全成对（mod.rs 注释说明）。
- **GameProfile 直读**：`Res<GameProfile>.key_bindings` 现为 `Vec<(u32,u32)>` 占位
  （game_profile.rs，组外），与 native 40 槽 × 12B 数组不一致 → 维持已验证的
  `BindingsTable::default()`，直读需 `GameKeyBindings` 重构（组外，登记）。
- UiHistoryButtonPressed **生产者**（UI 域历史按钮，组外）未接线。
- `update_cursor_position` 捕获（`CursorCapturedPosition`）交互分支语义 I（章节级），
  `CursorMovedScreenSpace` 载荷形状 I（待取证）。
- trailer `do_input_replay_ui` 的 **Save / Replay sheep 按钮**仍只 log（serde 文件形态
  待取证，未伪造写盘）——保持诚实标注，登记。
- Steam 全链 / `update_steam_inputs` / OS 设备层维持库外登记。
