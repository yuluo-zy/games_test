# garden 核对 DELTA（A 植被历史核对，2026-10-16）

对照：`A-vegetation/garden-SPEC.md`（2026-10-15）vs 当前源码 `crates/country-core/src/{systems/garden,resources/glade_border.rs}`。
机器码依据：`evidence/core-recovery-20261015/garden/REPORT.md`、`roofs-stairs/garden-draw.asm.txt`、
`evidence/country-core-deep/chapters/systems/garden/draw_garden.rs.md`、`garden_audio.rs.md`。
状态判定同 ivy-DELTA。

| SPEC 项 | 当前状态 | 证据（源码位置 / 原地址） | 备注 |
|---|---|---|---|
| P1.1 draw_garden UI 调用（name/argument_79/layer、4×0 参数） | 真实实现 | `draw_garden.rs:57-67`；`ui_state.rs:72-90 clearing_canvas_w_layer_and_sound(name, arg79, layer)` | Garden 分支 arg79=5、Eraser=9、layer=3，与 garden-draw.asm（0x140ae47f1/0x140ae472d：r9b=5/9、[rsp+0x20]=3）一致。extent=(130,130)/occlude=false/no_history=true 硬编码=原生 4×0 参数零输入的等价默认构造（0x140ae46df..4700），REPORT 已证。**SPEC 纠错：Eraser 分支 name 是 `"eraser"`（rne-read 0x142ae2e80='eraser'），Garden 分支 `"garden"`（0x142ae2ea0），非 SPEC 所称双分支都叫 "garden"**——当前源码正确，SPEC 此点过时。 |
| P1.2 其它 WidgetParams 变体 | 不属本组 | `systems/ui/resources/widget_params.rs`（组外共享） | SPEC 已声明由中央/UI 组接入，garden 只消费清画布全零变体；`ClearingCanvasWidget` 字段已显式（argument_79/layer/extent/occlude/no_history），可供中央喂变体。无冒充。 |
| P2 原全局调度 / AppMode 分派 | 待取证 | `draw_garden.rs:57-65` `match *mode`（Garden/Eraser(All\|Garden)，其余 `_ => return`）；`mod.rs:81-98 add_garden_cpu_systems` draw.after(Pop).before(Record) | draw 先于 record 的局部序成立（chain 语义）。**Replay 分支（原生 tag 13，0x140ae4651..677）在 draw_garden 内未实现**（`_ => return`）；回放走 `route_garden_replay_feedback`（mod.rs:42-57）把 `RasterReplayFeedback` 转回 GardenDrawEvent，是等价旁路但非 draw_garden 自身分支——待取证。全局相对序未证。 |
| P3 ShowPinnedDecoClutter union 与 renderer | 真实实现（按 SPEC 指引）/ 仍缺（完整 union） | `GardenPinnedClutterCircle{circle,height}`（mod.rs:27-31），`draw_garden.rs:94-97` 写 `{center, radius*0.5, height=position.y+1.5}` | 0.5=pin 圆半径缩放（0x142925970×[rip+0x1f3248f]）、1.5=Y 偏移，与 REPORT/SPEC 观测一致。SPEC 3.4 明令"保持为花园专用事件，不冒充完整 pin 系统"——实现照做。完整 56B union 变体判别属 C 组 ui_clutter 域（仍缺，跨组仲裁）。 |
| P4.1 GPU scatter | 后置（边界合规） | `GardenRaster`/`GardenRasterStagingSsbo` 由 draw_garden → `draw_raster::<GardenRasterStagingSsbo>`（draw_garden.rs:122） | CPU 侧生产者与 upload 契约按 world_raster 既有实现，SPEC 说无需改动。 |
| P4.2 garden audio 消费者 | 生产者侧真实 / 消费者简化 | `draw_raster` 逐点回调写 `GardenDrawEvent`/`GardenEraseEvent`（draw_garden.rs:134-147）；`systems/audio/garden_audio.rs:57-74 garden_audio_system` | **garden 组侧（生产者）满足**：draw/erase 事件在 begin/extend/commit 逐点发出。**消费端 `garden_audio_system` 属 B 组 audio 文件（组外）**，已存在并消费两事件缓冲，但**未实现 SPEC 的 130.0 距离门（0x43020000）与音量调制（[vol_res+0x1c]/除数）**，仅"事件存在→播放"折叠——偏离登记，交 B 组/中央裁决。 |

## 合成 agent 应关闭的真实剩余缺口

1. **garden 侧仅剩 P2**：draw_garden 内原生 tag 13（Replay）分支是否必须在 draw_garden 内复现，或由 `route_garden_replay_feedback` 旁路等效——需原侧差分定论；其余全局调度顺序属中央接线。
2. **跨组缺口（非本组实现，需仲裁登记）**：ShowPinnedDecoClutter 56B union 完整变体与 renderer（C 组 ui_clutter）；`garden_audio_system` 130.0 距离门+音量（B 组 audio）；garden scatter compute passes（渲染域后置）。
3. **SPEC 修正登记**：P1.1 Eraser 分支 name 应为 `"eraser"`（0x142ae2e80），SPEC 写"双分支均 garden"过时；现行代码正确，无需改动。
