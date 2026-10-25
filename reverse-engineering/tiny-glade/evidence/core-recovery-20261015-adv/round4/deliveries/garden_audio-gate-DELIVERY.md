# T1 garden_audio 130.0 门 + 音量调制 DELIVERY（2026-10-16 第四轮 ② 实现，已按①契约对齐）

> 实现对象：① T1 契约 round4/contracts/t1-garden_audio-130-gate-CONTRACT.md（PDB
> procs.tsv 行 49396 权威签名）+ 已解码契约（music-DELIVERY 新增段）+ 原生反编译直读
> （decomp-0x140a89480、decomp-0x14166d9f0）。文件（② 文件权 + 中央仲裁批准的两处）：
> `systems/audio/garden_audio.rs`、`resources/cursor_terrain_raycast.rs`（组外扩字段，
> 仲裁批准，见 PROGRESS.md 仲裁表）、render 测试构造点 8 处（`..Default::default()`）。

## 真实代码接线

1. **`garden_audio_system`** 按原生 6 参签名（PDB FunctionSystem 权威）对齐：
   `(Res<AppMode>, ResMut<GardenAudioState<S::Event>>, EventReader<GardenDrawEvent>,
   EventReader<GardenEraseEvent>, Res<CursorTerrainRaycast>, Res<world_raster_defs::WaterRaster>)`。
   无 `AudioCore` 参（原生无）。
2. **param_6 = `WaterRaster`**（纠偏 GardenRaster，① §3）：`+0x298` selector →
   `current_buffer()`（0=composite ptr@+8/len@+0x10，非 0=scratch ptr@+0x20/len@+0x28），
   采样 `bilinear_sample_at_world_xz<RasterBufferRef<512>>` extent 130.0×2。
3. **param_5 = `CursorTerrainRaycast`**（① §2 布局 32B，中央仲裁已批准扩字段）：
   - `resources/cursor_terrain_raycast.rs` 由 16B `{tag:u8, pos}` 扩为 32B
     `{tag:u32@0, pos:Vec3@0x04, delta:Vec3@0x10, speed:f32@0x1c}`；`tag` u8→u32 兼容
     既有 `tag&1`/`tag==0`，`position()`/`distance()`/`update()` 保持；写入者
     `mouse_terrain_raycast` 0x1408ef320 doc 注释登记。
   - 采样坐标 = `cursor.pos.x@+4 / pos.z@+0xc`（**非事件位置**）。
   - 音量 = `cursor.speed`（+0x1c）当 `mode.tag != 0xd`，`/60.0`；Replay → `1.0`。
   - **`GardenAudioVolume` 适配资源已删除**（①定案 +0x1c 是 CursorTerrainRaycast 一等字段，
     非独立资源）。
4. **门逻辑** `garden_loop_flags(sample, draw, erase)`（decomp-0x140a89480 四次调用）：
   sample>=0.01 → GardenDrawWater(1)/EraseWater(3)；<0.01 → GardenDraw(0)/Erase(2)；
   计数非零且 gate 通过才 play，否则 stop。采样按③建议门控（双通道无事件 → 不采样，
   防空缓冲越界；与原生 per-loop count==0 门控输出等价）。
5. **模板 `update_draw_audio_volume`**（O：FUN_14166d9f0）：
   - should_play → set_position(last_event.position) +
     `set_parameter("editing_speed", volume)` +
     `set_parameter("PathSize", garden_path_size(radius))` + start_if_not_playing。
   - **PathSize 二次参数定案**（① §5）：`garden_path_size(radius) = 2 * inverse_lerp_unclamped(
     0.32, 1.2, radius)`（常量 0x142ac56c0=0.32/0x1429cd128=1.2，rne-read 复核；native
     inverse_lerp 0x140c904a0 无 clamp）——笔刷半径 [0.32,1.2] → FMOD PathSize [0,2]。
   - 位置/半径取自最后一个匹配事件（native 传事件尾指针）；GardenDrawEvent/EraseEvent
     stride 0x20 `{position@0, radius@0xc, observed_flag@0x10}`（O，① §6，现源字段名一致）。
6. **常量**（全部 rne 字节复核）：GARDEN_WATER_GATE=0x3c23d70a(0.01)、
   VOLUME_NORMALIZER=0x42700000(60.0)、REPLAY_VOLUME=1.0、GARDEN_RASTER_EXTENT=
   0x43020000(130.0)、PATH_SIZE_MIN=0x3ea3d70b(0.32)、PATH_SIZE_MAX=0x3f99999a(1.2)、
   PATH_SIZE_SCALE=2.0、PATH_SIZE_PARAMETER="PathSize"、EDITING_SPEED_PARAMETER。

## candidate 链测试

- 常量/字节、loop index↔原生偏移、water gate 分流、零计数停播、PathSize 映射
  （0.32→0、1.2→2、0.76→1、无 clamp 外推）——garden_audio.rs tests。
- 8 处 `CursorTerrainRaycast { tag, pos }` 测试构造点（render_loop/draw_passes，
  `#[cfg(test)]`）补 `..Default::default()`，tag 字面量 1/0/2 推断 u32。
- 全 ECS 系统级测试需宿主注入 Studio/AudioCore/WaterRaster，未跑；整 crate `cargo check`
  剩余 3 错误全在用户 roof 重构域（`resources/roofs/roof_shape.rs` E0599 raycast ×2、
  `systems/paths.rs` E0308——两文件均为工作树并发修改，非本组），本组文件零错误。

## 原侧验证

- PDB procs.tsv 行 49396/49402：`garden_audio_system` bevy FunctionSystem 6 参类型逐参对齐。
- decomp-0x140a89480：音量分支（`*(int*)*param_1 != 0xd` → `[cursor+0x1c]/60.0`）、
  param_6 光栅 +0x298 选择器、四次模板调用 gate（>=0.01/<0.01、draw/erase 计数、
  loop@+0/+0x38/+0x70/+0xa8）逐行核对。
- decomp-0x1408ef320：CursorTerrainRaycast 32B 写入面（tag/pos/delta/speed 偏移）。
- decomp-0x14166d9f0：模板（set_position + "editing_speed" + PathSize=2*inverse_lerp
  (0.32,1.2,radius) + start；==0 → stop）。
- rne-read 常量：0x142960674=0.01、0x1429d18cc=60.0、0x142925904=1.0、0x142a7b658=130.0、
  0x142ac56c0=0.32、0x1429cd128=1.2；串 0x142c84546="editing_speed"、0x142c84553="PathSize"。

## 仍缺项（待取证，登记不补造）

1. `mouse_terrain_raycast` 0x1408ef320 的 `[param_7+0x58]` 除数资源身份（speed 分母）。
2. **运行时依赖（③ REVIEW.md §4b，非水分）**：`mouse_terrain_raycast` 未实现为 crate 内
   生产者，`CursorTerrainRaycast.speed/delta` 在 T2 宿主接线（run_game_thread 装配写入者）
   前保持 0 —— standalone 非 Replay 音量 = 0。已登记为 T2 宿主接线依赖。
3. `GardenAudioState` 原生 `@0x10/@0x1c` 计数写回语义（contract §4；② 以内联
   `[AudioLoop;4]` 落地，行为等价，未改）。
4. `GardenAudioState` 原生 `{loops_ptr@0, ...}` 非内联数组布局（stride 0x38 同）；
   crate 内联数组行为等价，登记。
5. 28-系统注册顺序 / garden_audio_system 进宿主调度（T2 ①契约落地后接线）。
6. 原生模板 `if volume>0` 条件 start 的双调用以单次幂等实现（注释说明）。

## 组外申请记录

- `resources/cursor_terrain_raycast.rs` 扩 32B + 8 处 render 测试构造点补 `..Default::default()`：
  **中央 agent a8a2e9731c4957436 已批准**（PROGRESS.md 仲裁表登记；共享只读资源、增量字段、
  无并行写）。
