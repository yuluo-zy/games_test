# T1 CONTRACT — garden_audio_system 130.0 门：音量/光栅资源身份定案

> 逆向 agent ① 交付。目标：把 music-DELIVERY 已解码契约的**资源身份**补全并纠偏，
> 供 ② 实现按原生签名对齐。全部证据 = B patched EXE 机器码 / PDB 符号 / rne-read 直读。
> 状态：**可直接落地**（两处纠偏：param_6 是 WaterRaster 非 GardenRaster；param_5 是
> CursorTerrainRaycast 且 +0x1c 是其一等字段 speed，非独立 GardenAudioVolume 资源）。

---

## 0. 结论摘要（② 需对齐的三点）

1. **param_5 = `Res<cursor_terrain_raycast::CursorTerrainRaycast>`**（不是独立"音量资源"）。
   原生布局 32B：`{ tag:u32@0x00, pos:Vec3@0x04, delta:Vec3@0x10, speed:f32@0x1c }`。
   `+0x1c` = `speed`（= `len(delta)/dist`，由 `mouse_terrain_raycast` 0x1408ef320 每帧写入）。
   音量 = `speed / 60.0`（`AppMode != Replay(0xd)`），`== Replay` → `1.0`。
2. **param_6 = `Res<world_raster_defs::WaterRaster>`**（包装 `WorldRaster<512>`；不是 GardenRaster）。
   采样面 = `+0x298` selector：0 → composite `+8/+0x10`(ptr/len)，非 0 → scratch `+0x20/+0x28`。
3. **事件只提供"有无 + 最后事件位置/半径"**：`GardenDrawEvent`/`GardenEraseEvent` stride 0x20，
   `{position:Vec3@0x00, radius:f32@0x0c, observed_flag:bool@0x10}`。音频只取 counts（栅格门后
   传事件尾指针），位置面来自事件本身（`update_draw_audio` 内部 set_position），
   **栅格采样坐标来自 param_5（CursorTerrainRaycast.pos.x/z @+4/+0xc）**。

---

## 1. 原生签名（PDB 权威，`evidence/country-core-audit/data/procs.tsv`）

`garden_audio_system` 的 bevy `FunctionSystem` 类型（多行一致，含 into_configs/initialize/run_unsafe）：

```
void (*)(
  Res<enum2$<country_core::systems::mode_manager::AppMode>>,
  ResMut<country_core::systems::audio::garden_audio::GardenAudioState>,
  EventReader<country_core::systems::garden::GardenDrawEvent>,
  EventReader<country_core::systems::garden::GardenEraseEvent>,
  Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,
  Res<country_core::resources::world_raster_defs::WaterRaster>
)
```
- 地址锚点：`into_configs<...>` @0x14166cd50（行 49396）、initialize 族 @0x14166d7b0..0x14166cd50。
- `path_audio_system`（0x1409d0f80）同构 + 第 7 参 `EventReader<terrain_stairs_audio::SuppressPathSound>`。
- `water_audio_system`（0x140873630）第 5 参同为 `Res<CursorTerrainRaycast>`，第 6 参 = `Res<ActiveSession>`
  （`+0x58` mode 字节 == 4 门，非栅格）。`tree_audio_system`（0x1414638e0）第 4 参同为
  `Res<CursorTerrainRaycast>`。→ **全 draw-audio 族共享 cursor 资源**。

## 2. param_5 = CursorTerrainRaycast（布局 O）

写入者 `mouse_terrain_raycast`（0x1408ef320，terrain-water/decomp）逐槽写入：
```
+0x00  tag: u32                    (*piVar7 = 1，命中置位；未命中分支不写，保持 0)
+0x04  pos: Vec3                   (+4/+8/+0xc；CONCAT44 双 f32 + piVar7[3]=(int)fVar20，movss 语义)
+0x10  delta: Vec3                 (本帧位置差；+(0x10/0x14/0x18)，仅前帧有效才写)
+0x1c  speed: f32                  (= sqrt(delta·delta) / [param_7+0x58]；param_7 资源距离场，身份待取证)
```
- `position()` @0x140a12650 只读 +4/+0xc（现有源 16B 版只覆盖到 0x10 → **缺 delta/speed**）。
- `garden_audio` 读 `[cursor+0x1c]` 当音量；`wall_audio_fmod` 读 `[cursor+0x1c]`；
  `terrain_stairs/terrain/tree/water` 同源（procs.tsv 逐系统核对）。
- **实现要求**：扩展 `resources/cursor_terrain_raycast.rs` 为 32B `{tag:u32, pos:Vec3, delta:Vec3, speed:f32}`；
  该文件改动涉组外资源（B 组 input 域已用过 window.rs 仲裁先例），② 落地前应走仲裁登记。
  `GardenAudioVolume` 适配资源不再需要——`+0x1c` 就是 CursorTerrainRaycast 一等字段。

## 3. param_6 = WaterRaster（WorldRaster<512> 选择面 O）

`garden_audio` 采样面（decomp-0x140a89480 行 60-68）：
```
lVar9 = *param_6;                       // &WaterRaster = &WorldRaster<512>
puVar1 = lVar9 + 0x20; puVar15 = lVar9 + 0x28;   // scratch Vec {cap@0x18, ptr@0x20, len@0x28}
if (*(char*)(lVar9 + 0x298) == 0) { puVar15 = lVar9+0x10; puVar1 = lVar9+8; }  // composite {cap@0,ptr@8,len@0x10}
uStack_78 = *puVar15; uStack_80 = *puVar1;      // RasterBufferRef {data.ptr, data.len}
FUN_140a14df0(&uStack_80, x, z, 130.0, 130.0)
```
- `FUN_140a14df0` = `utils::float_grid::bilinear_sample_at_world_xz<RasterBufferRef<512>>`
  （interaction-group/decomp；`USE_FLOAT_GRID_SAMPLING_FIX` 全局 0x14362c308/0x143638fac 选路径）。
- `x = [cursor+0x04]`（`uVar31`）、`z = [cursor+0x0c]`（`uVar33`）；`extent_x = _UNK_142a7b658` =
  `0x43020000` = 130.0f32（rne-read `00000243`）；`extent_z` = 130.0f32。
- 常量复核（rne-read）：`0x1429d18cc` = `00007042` = 60.0；`0x142925904` = `0000803f` = 1.0；
  `0x142960674` = `0ad7233c` = 0.01。
- **纠偏**：② 已按 `GardenRaster` 接线 → **应改 `WaterRaster`**。语义自洽：光标处水栅格值
  ≥0.01 即"画到水上"→ 播 GardenDrawWater/EraseWater 变体。

## 4. 门控与四 loop 分流（decomp-0x140a89480 行 156-241，O）

```
fVar34 = 1.0;  if (AppMode.native_tag() != 0xd) fVar34 = [cursor+0x1c] / 60.0;   // 音量
grid    = WaterRaster.current_buffer()（+0x298 选择）→ RasterBufferRef
sample  = bilinear_sample_at_world_xz(grid, cursor.pos.x, cursor.pos.z, 130.0, 130.0)

// 4 次 update_draw_audio<garden_closure>（FUN_14166d9f0），GardenAudioState.loops stride 0x38
loop[1] GardenDrawWater   @ slots+0x38 : count = sample >= 0.01 ? draws : 0
loop[0] GardenDraw        @ slots+0x00 : count = sample <  0.01 ? draws : 0
loop[3] GardenEraseWater  @ slots+0xa8 : count = sample >= 0.01 ? erases : 0
loop[2] GardenErase       @ slots+0x70 : count = sample <  0.01 ? erases : 0
```
- `draws/erases` = EventReader 新事件数（bevy 0.16 `Events<T>` 双缓冲 + EventCursor 计数；
  无新事件传 0，即不播）。
- **GardenAudioState 布局**（O，读面）：`{ loops_ptr@0x00, ?@0x08, *mut u32@0x10, u32@0x1c }`；
  `loops_ptr` 指向 4×`AudioLoop`（stride 0x38，= 现源 `AudioLoop` 56B）。`@0x10/@0x1c` 写回模式
  （4 次 `*[@0x10] = [@0x1c]`）= 计数同步，语义待取证；② 若以 `[AudioLoop;4]` 内联落地不破坏行为。

## 5. update_draw_audio<garden_closure> 模板（0x14166d9f0，O）与 ② 问的二次参数

签名 `(slot: &AudioLoop, last_event: *const GardenDrawEvent|null, volume: f32)`：
- `last_event != 0`（有新事件）：
  - `entries[cursor].set_position(last_event.position)`（FUN_140a57da0，position=*ptr/+8）
  - `entries[cursor].set_parameter("editing_speed"(0x142c84546, len 0xd), volume)`（FUN_140a57670）
  - `volume > 0.0` 时 `start_if_not_playing`（FUN_140a596b0）
- **二次参数**：`uVar9 = inverse_lerp(_UNK_142ac56c0=0.32f, _UNK_1429cd128=1.2f, radius@+0xc)`，
  然后 `set_parameter(entries[cursor], "PathSize"(0x142c84553, len 8), 2*uVar9)`（FUN_141437ce0）。
  - 含义：笔刷半径 `radius ∈ [0.32, 1.2]` → `PathSize ∈ [0, 2]`（`2*(r-0.32)/0.88`）。
  - 常量 rne-read：0x142ac56c0 = `0bd7a33e`=0.32；0x1429cd128 = `9a99993f`=1.2。
  - FMOD 事件共用 "editing_speed"/"PathSize" 参数名（garden/path 模板同串）。
- ② 现有 `update_draw_audio(loops, should_draw, position)` 签名缺 volume 与 PathSize 面，
  需按本契约扩。

## 6. 事件布局（O，producer draw_garden + 模板读面）

`GardenDrawEvent`/`GardenEraseEvent`（`systems/garden`）stride 0x20：
```
0x00 position: Vec3   0x0c radius: f32   0x10 observed_flag: bool
```
（现源字段名一致；音频模板只读 position/radius。）

## 7. 可实现性

**可直接落地**。② 待办：
1. garden_audio_system 签名改 6 参（加 `Res<AppMode>`、`Res<CursorTerrainRaycast>`、
   `Res<WaterRaster>`，去掉 `_core: Res<AudioCore<S>>`——原生无 AudioCore 参）。
2. CursorTerrainRaycast 扩 32B（delta/speed）；走仲裁。
3. param_6 改 WaterRaster（纠偏 GardenRaster）。
4. update_draw_audio 模板补 volume→"editing_speed"、radius→"PathSize" 面。

**待取证（低优先）**：`mouse_terrain_raycast` 的 `[param_7+0x58]` 除数资源身份；
GardenAudioState `@0x10/@0x1c` 计数写回语义。

## 8. 证据索引
- procs.tsv：行 49396/49402（garden into_configs/initialize）、行 18950（path）、
  water 行 0x140982480 等、tree 行 0x1405cf3b0。
- decomp：audio-misc/terrain-water `decomp-0x140a89480.txt`（garden 主）、`decomp-0x1409d0f80.txt`
  （path 同构）、`decomp-0x140873630.txt`（water）、`decomp-0x14166d9f0.txt`（模板）、
  `decomp-0x1408ef320.txt`（CursorTerrainRaycast 写入者）、`interaction-group/decomp-0x140a14df0.txt`
  （bilinear sampler）、`terrain-water/decomp-0x140a88fa0.txt`（init）。
- rne-read 常量：0x1429d18cc=60.0、0x142925904=1.0、0x142960674=0.01、0x142a7b658=130.0、
  0x142ac56c0=0.32、0x1429cd128=1.2；串 0x142c84546="editing_speed"、0x142c84553="PathSize"。
