# garden 剩余业务契约 SPEC（A 植被历史分析，2026-10-15）

模块文件：`systems/garden/*`、`resources/glade_border.rs`。
已实现基线见 `evidence/core-recovery-20261015/garden/REPORT.md`（draw/record 消费者、stroke slot 5、
GardenRaster/GardenRasterStagingSsbo、History::Garden、glade_border、ui_state 清画布）。
本 SPEC 只定义剩余缺口契约。证据源：draw_garden 原生反汇编（0x140ae4620，本次逐条复核）、
garden_audio_system 反汇编（0x140a89480）、decoclutter_show_pins 反汇编（0x1412357c0）、
compute_garden.rs（GPU 边界已文档化）、v2/v3 过程表、public-symbols。

区分：**可直接落地** / **需先取证**。

---

## P1 完整 UI 输入仲裁 / 其它 WidgetParams 变体 —— 部分取证

### 1.1 原生 draw_garden 的 UI 调用（0x140ae46f5..b1，逐条已核实）
两个分支各调一次 `clearing_canvas_w_layer_and_sound`（0x14092df50）：
- Garden 分支（0x140ae47dc..f1）：name="garden"（len 6），**argument_79=5，layer=3（[rsp+0x20]）**。
- Eraser 分支（0x140ae4718..2d）：name="garden"，**argument_79=9，layer=3**。
- 每次传 `[rsp+0x28] = &[rsp+0x80]`（4 个 dword 全 0 的 16B 参数结构）。现行
  `UiState::clearing_canvas_w_layer_and_sound` 把该参数硬编码为 `extent=(130,130), occlude=false,
  no_history_recording=true`（garden REPORT 已证：原生构造存 volume tag 1、arg@+0x79、shape tag 4、
  layer@+0x89、extent(130,130)、occlusion=false、flag=1）→ **4×0 参数 = 该默认构造的零输入，现行
  硬编码等价**，行为正确。
- 返回：`(UiWidgetContext*)` 在 rax/rdx；`widget+0x18`=相位字节、`+0x19`=相位副字节、`+0x38`=widget id
  （0x140ae4899..d1 现行 `dragged()`/state 机复用一致）。

### 1.2 其它 WidgetParams 变体（共享 UI，跨组）
- 原生 `WidgetParams` 位于 `systems/ui/resources/widget_params.rs`（共享，组外）：
  - `WidgetParams::new` 0x1409a8e10、`sounds` 0x1409a8e80、`occlude_by_world` 0x1409a8ee0、
    `no_history_recording` 0x1409a8f10、`allow_negative_distance` 0x1409a8f40。
  - `signifier<...>` 实例：0x1422bcec0/0x1422bc980/0x1422bcc20（ui_color_*）、0x1425df760
    （ui_move_tree）——garden 的两个调用点不建 signifier（现行 ClearingCanvasWidget 无 signifier）。
- **结论**：garden 消费的 UI 面 = 清画布 widget（默认 WidgetParams 全零变体）；"其它变体/完整仲裁"
  属共享 UI 系统的生产侧（SignifierStream、WidgetFlags），**不在 garden 组实现**，由中央/UI 组
  接入 `systems/ui/signifiers/*`（SignifierEntityBuilder 0x14089d9b0..dd50 已列入 v2）。
- **可直接落地**：把 `ClearingCanvasWidget` 的构造参数暴露为显式字段（extent/occlude/no_history）
  以便中央把 `WidgetParams` 变体喂给清画布——行为不变，仅结构对齐。

---

## P2 原全局调度（garden 相关 add_systems 顺序）—— 待取证
- garden 无独立 `mod.rs` 插件调度函数（与 ivy 的 `add_ivy_systems` 0x140944070 不同，v3 无
  `garden::impl$0::*` 过程）。draw_garden（0x140ae4620）与 record_garden_history（0x141695e10）
  挂在主插件/全局 schedule。
- 已证局部序：**draw 先于 record**（REPORT），且 `route_garden_replay_feedback` 消费
  `RasterReplayFeedback`（history 回放回调）→ 现行 `GardenCpuStage::{Draw,Record}.chain()` 成立。
- **待取证**：与 terrain/session/history 的全局相对序、run_if 门（现行 draw 由 `Res<AppMode>` 分派
  已覆盖原生 +0x118 分支？garden 用 AppMode 枚举，原生 draw_garden 入口 `[rcx]` dword = AppMode tag，
  tag 13→Replay 等分支（0x140ae4651..677）已实现）。

---

## P3 56B ShowPinnedDecoClutter union 与 renderer —— 部分取证

### 3.1 类型归属
- `ShowPinnedDecoClutter` 定义于 **`systems/ui_clutter/decoclutter_show_pins`** 模块（组外，C 组
  ui_clutter 拥有源文件 `systems/ui_clutter/*`），非 garden。事件槽注册见
  `upper-modules-plan/systems/clutter/ANALYSIS.md`（注册事件表含 ShowPinnedDecoClutter）。
- garden 是**生产者之一**：draw_garden Garden 分支在 begin/提交时写一个 pin 请求
  （现行 `GardenPinnedClutterCircle{circle, height}` 是观测到的花园变体投影）。

### 3.2 Events 行布局（decoclutter_show_pins 0x1412357c0，逐条已核实）
- `Events<ShowPinnedDecoClutter>` 行步长 **0x40（64B）= 8B EventId 前缀 + 56B payload**（0x141235ab7
  `shl rbx,6`）。
- renderer `decoclutter_show_pins`（0x1412357c0）：读双缓冲 → 对每事件查
  `decorator`/`DecoratorStorage`（0x141235c40 起，`[res+0x1a8]` / `[res+0x100]` 两条查找链，按
  `[res+0x108]` 字节选）→ 生成 pin 网格写入渲染缓冲。事件 payload 的字段分派（Garden vs 通用
  decorator）由 payload 判别驱动——**56B union 的变体布局待取证**（需对 show_pins 前半 payload
  解复用差分，或对 ui_autoclutter/ui_focus_decoclutter 写端差分）。
- 已观测花园变体（现行代码依据）：`{circle.center(Vec2), radius*0.5, height = position.y + 1.5}`；
  `0.5`=0x140ae4815..21 的 `[rip+0x1e41153]*brush*[rip+0x1f3248f]`、`1.5`=pin 请求 Y 偏移
  （garden REPORT）。**这不是完整 union**。

### 3.3 生产者清单（public-symbols 观测 EventWriter）
- draw_garden（garden）、`ui_autoclutter`/`ui_focus_decoclutter`/`rmb_autoclutter`、`defered_
  autoclutter_history` 等（C 组 ui_clutter 域）。消费者（EventReader FunctionSystemState）：
  0x1405d2600、0x1405d7190（均 ui_clutter 域系统）。

### 3.4 落地建议（供实时复合 agent + 中央仲裁）
- **保持 `GardenPinnedClutterCircle` 为花园专用事件，不冒充完整 pin 系统**（REPORT 已声明）。
- 若中央要求类型对齐：把 ShowPinnedDecoClutter 类型落入 ui_clutter（C 组文件），garden 侧
  draw 写该事件的花园变体；本组不拥有该类型，改前先仲裁。
- renderer（decoclutter_show_pins）与 pin 网格 shader 属 ui_clutter/渲染域，**不在本组**。

---

## P4 下游 garden scatter / audio 消费者 —— 边界已定义（后置 GPU，audio 已取证）

### 4.1 GPU scatter（后置，只定义真实 CPU producer/consumer 边界）
- 原生 compute passes（`systems/render_loop/compute_passes/compute_garden.rs` 已文档化）：
  - `garden_rasterize_from_cpu`（0x140a22140）：读 `GardenRaster`（遮罩标志 `res+0x29b==0` 直接返回，
    非 0 则清零并 memset +0x98 0x200B），两次高斯模糊（5.0），512×512。
  - `garden_spawn_vegetation`（0x140a22b80）：Spawn bushes/flowers/marching_squares/small_flowers
    四 pass → `BushSsbo`(48B, "Bushes")、`FlowerSsbo`(64B)、`GardenStoneSsbo`、`Flower03Ssbo`。
- **真实 CPU 边界**：draw_garden 写 `GardenRaster` → upload_world_raster 置位 `+0x29b` 并上传
  `GardenRasterStagingSsbo` → GPU pass 消费并清 `+0x29b`。CPU 侧生产者=本组 draw_garden；
  上传/GPU 执行/SSBO 布局=渲染域，**本组只保证 `GardenRaster` 写入与 upload 调用契约**（现行
  world_raster.rs 已接，无需改动）。

### 4.2 garden audio 消费者 —— 可直接落地（接线到 audio 组文件前先仲裁）
- `garden_audio_init_system`（0x140a88fa0）：字符串（0x140a88fd8 `[rip+0x204ff03]`, len 0x1c）
  经 `0x140a59220` 初始化音频（FMOD/GladeAudio 域）→ 属 audio 组（`resources/audio*`），本组只登记
  事实。
- `garden_audio_system`（0x140a89480）：
  - `cmp dword ptr [AppMode], 0xd (13=Replay)`：Replay → 固定音量 `[rip+0x1e9c1dd]`；否则
    `volume = [vol_res+0x1c] / [rip+0x1f48193]`（**待取证** vol_res 与除数浮点）。
  - 消费 **两个事件缓冲**（GardenDrawEvent 与 GardenEraseEvent，r14/rbx 两条 EventReader 等价），
    每事件取光标 `Option<Vec2>`，经 `0x140a14df0` 算距离，阈值 **130.0（0x43020000）** 内触发
    `0x14166d9f0`（音量 xmm2、bool rdx 是否播放、emitter rcx）→ 播放 garden 绘制/擦除音。
- 接线：现行 `systems/garden` 已产出 `GardenDrawEvent`/`GardenEraseEvent`；audio 组把
  `garden_audio_system` 接为消费者（跨组，需仲裁）。本组可提供的辅助：确保 draw/erase 事件在
  begin/extend/commit 逐点发出（现行 draw_raster 回调已发）。

---

## 待取证清单（无证据不落地）
1. ShowPinnedDecoClutter 56B union 的变体判别与完整字段布局（show_pins 写端/读端差分）。
2. garden 全局 add_systems 顺序（与 ivy/terrain/history 相对序）。
3. `garden_audio_system` 的 `[vol_res+0x1c]` 资源与除数浮点。
4. draw_garden 门控中 `[res+0x120]`（spawner 同款）在 garden 侧是否适用——garden 用 AppMode tag 分派，
   无额外 +0x120 判别（本次反汇编未见）。

## 明确排除（本组不实现）
- ShowPinnedDecoClutter 类型与 renderer（ui_clutter 域，跨组仲裁）。
- GPU compute passes 与 SSBO 布局（渲染域后置）。
- `resources/glade_settings.rs`、`resources/history/*`、`systems/history.rs`、audio 文件修改一律经中央仲裁。
