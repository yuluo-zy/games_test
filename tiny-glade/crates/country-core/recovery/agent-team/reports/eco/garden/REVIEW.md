# REVIEW — systems/garden（reviewer-eco，2026-10-15）

独立核对现有实现（draw_garden / record_garden_history / mod）是否实现原二进制业务逻辑。
证据：terrain-water/decomp/decomp-0x140ae4620.txt、decomp-0x141695e10.txt、章节、AppMode
native_tag 表、EraserLayer 表。restorer 尚未提交 RESTORE.md，本报告为对当前 src 的独立审查基线。

## 逐函数三栏对照

### systems/garden/draw_garden.rs（原 draw_garden 0x140ae4620 + 3 闭包）

| 维度 | 原行为（decomp 逐条） | Rust 现状 | 判定 |
|---|---|---|---|
| 模式分发 | `mode-6<0xf` 且 `mode-5∈{4,0xc}` → mode9=橡皮/mode17=花丛，否则 return | AppMode::Eraser(All\|Garden)=9、AppMode::Garden=17；其余 return | **OK** |
| 橡皮层门控 | `[param_1+4]!=0 && !=5` → return | EraserLayer::All=0 / Garden=5 匹配 | **OK** |
| 橡皮半径 | fVar17=inverse_lerp(0.3,2.0,brush)；fVar17*4+(1-fVar17)；非有限→panic | t=(b-0.3)/(2.0-0.3)；t*4+(1-t)；assert is_finite | **OK** |
| 花丛半径 | fVar17=*(param_11[4]+8)=brush.garden；非有限→panic | radius=brush.1.garden；assert is_finite | **OK**（纠错：章节"半径=笔刷+1.5"是把 +1.5 引脚高度误当半径） |
| 工具 id / UI 串 | 花丛=FUN_14092df50(串@0x142ae2ea0 len6, tool=5)；橡皮=串@0x142ae2e80 len6, tool=9；layer=3 | name="garden"/"eraser"，argument_79=5/9，layer=3 | **OK** |
| 界内校验 | GladeBorder::is_shape_inside；矩形尺寸=0.4*radius*0.8 | Circle2d(center,(0.4*radius)*0.8) | **OK**（0.4/0.8 与 decomp `_UNK_142925970 * fVar17 * _UNK_142a16cb8` 一致） |
| 出界事件 | 出界写 DisplayOutOfBorder；phase!=2 写 FeedbackInputOutOfBorder | 一致 | **OK** |
| 绘制状态机 | r=clamp(phase-2,0,3)；state 转移（begin/extend/commit/cancel，wrapping 减法 + 不允许扩展态） | 逐条对应 | **OK** |
| 引脚圆 | 圆半径=radius*0.5、高=cursor.y+1.5（`fStack_a0=1.5 或 cursor+1.5`@0x140ae4741 段） | GardenPinnedClutterCircle{Circle2d(center,radius*0.5),height:pos.y+1.5} | **OK** |
| draw_raster 分派 | FUN_140ab59e0（<GardenRasterStagingSsbo>）slot=5、strength=0.5 | draw_raster::<GardenRasterStagingSsbo,C>(...,5,..strength 0.5) | **OK**（测试断言 stroke.strength=0.5 一致） |
| 事件 | draw/erase/record 三回调（含回放 marker bool） | GardenDrawEvent/GardenEraseEvent/RecordGardenEditToHistoryCmd | **OK** |

### systems/garden/record_garden_history.rs（原 record_garden_history 0x141695e10）

| 维度 | 原行为 | Rust 现状 | 判定 |
|---|---|---|---|
| 事件消费 | drain 全部待消费通知 | events.read().last() 判空 | **OK** |
| 记录 | 取槽位编辑单元（12B×n）→ Arc(0x30) 封装 → History::add（tag6 @0x140a84ec0） | record_raster_history(strokes,5,history,EditType::Garden) | **OK**（共享历史链处理 Arc/tag；slot 5 与 draw_garden 一致） |
| 失败 | History 失败 → HistoryRecordingFailed | 一致 | **OK** |

### systems/garden/mod.rs

- GardenDrawEvent/GardenEraseEvent/GardenPinnedClutterCircle：合理事件投影。
- route_garden_replay_feedback：回放反馈（RasterReplayFeedback::Garden → draw 事件，replay 标记当 observed）——
  非原 inventory 函数，属接线辅助；与"draw 回调带 replay marker"语义一致。**OK 注记**。
- register_garden_events / add_garden_cpu_systems：注册与局部调度齐；**原全局 plugin 调度未接入**（文档边界，不判缺失）。

## 结论

- **DISPATCH 验收门槛"garden 模式、画笔输入、绘制、历史记录四链闭合"**：四链均已实现且与本机反编译
  逐条对上（模式分发/笔刷半径/UI/界内/状态机/绘制/历史记录），仅全局调度接线未做（属文档边界）。
- 状态建议：**review_passed**（在本批限定范围内；全局注册与 pin 渲染/散布/音频消费仍为后续项）。

## 遗留

- 全局 schedule/startup 注册、GardenPinnedClutterCircle 的 renderer 消费、pin/散布/音频下游：未闭合，仍属本模块 remaining。
