# garden 实时复合交付（A 植被，2026-10-16）

实现者：A 组实时复合 agent。依据 `garden-DELTA.md`。
源码：`crates/country-core/src/systems/garden/*`、`resources/glade_border.rs`。

## 真实代码接线

garden 组侧**无源码改动**。DELTA 唯一 garden 内剩余缺口 P2（Replay 分支）经原侧差分定论为"无需改动"，
其余缺口均属跨组（B 组 audio、C 组 ui_clutter、渲染域）并已申请仲裁。

## P2 Replay 分支定论（原侧差分，2026-10-16）

- 证据：`evidence/core-recovery-20261015/roofs-stairs/garden-draw.asm.txt`（0x140ae4620）。
- 跳表分派（0x140ae4651..677）：`movl (%rcx),%r9d` 读 AppMode tag → `leal -0x6(%r9)`/`addq $-0x5,%r9` →
  `cmpl $0xf` 越界保护 → `cmpq $0xc,%rdx`（tag 17 = Garden → 0x140ae474c）→ `cmpq $0x4,%rdx`
  （tag 9 = Eraser → 擦除分支）→ `jne 0x140ae4b7a`（默认）。
- 0x140ae4b7a 是纯 epilogue（`movaps 0xe0(%rsp),%xmm6; addq $0xf8,%rsp; pop...; ret`），即**默认分支直接返回**。
- 全函数无 `cmp $0xd`（tag 13 Replay）判定 → **draw_garden 内没有 Replay 分支**，Replay 落入默认返回。
- 结论：现行 `draw_garden` 的 `_ => return` 与原生一致；`route_garden_replay_feedback`
  （`RasterReplayFeedback` → `GardenDrawEvent`，mod.rs:42-57）是独立的历史回放消费者，为等效旁路。
  SPEC P2 "tag 13→Replay 等分支已实现"的表述不成立，以本差分为准。**无改动。**

## candidate 链测试

- 未改动 garden 系统，无新增测试。`garden/tests.rs` 既有 draw/record 链测试保持原状。
- cargo check：garden 文件 0 错误（全 crate 剩余错误见 ivy-DELIVERY.md 编译状态，均属其它链在制域）。

## 原侧验证

- draw_garden 分派：tag 17 Garden / tag 9 Eraser / 其余返回，与 asm 0x140ae4651..677 + 0x140ae4b7a 一致。
- P1.1 name 纠错：Eraser 分支 `"eraser"`（0x142ae2e80）、Garden 分支 `"garden"`（0x142ae2ea0）——
  现行代码正确，SPEC 此点过时（DELTA 已登记）。
- route_garden_replay_feedback 消费 `RasterReplayFeedback` 的 Garden domain 样本，逐点转回
  `GardenDrawEvent`——与原生 replay_raster 回调消费者语义一致。

## 仍缺项（跨组，非本组实现）

- **garden_audio_system 130.0 距离门 + 音量调制**：属 B 组 `systems/audio/garden_audio.rs`
  （组外）。已向中央 agent（a8a2e9731c4957436）发仲裁申请；本组生产者侧满足
  （draw/erase 事件 begin/extend/commit 逐点发出）。
- **ShowPinnedDecoClutter 56B union 完整变体与 renderer**：C 组 ui_clutter 域。garden 只保持
  `GardenPinnedClutterCircle` 花园专用事件（SPEC 3.4 明令不冒充完整 pin 系统）。
- **garden scatter compute passes**（`garden_rasterize_from_cpu`/`garden_spawn_vegetation`）：
  渲染域后置。CPU 侧 `GardenRaster` 生产者与 upload 契约按 world_raster 既有实现，无需改动。
- **原全局 add_systems 相对序**：中央接线域，本组仅保证 draw 先于 record 局部序（chain 语义）。

## 组外申请记录

- 已向中央 agent（a8a2e9731c4957436）发送 garden audio 仲裁申请与 P2 定论。
- 未修改任何组外文件。
