# M01 共享 signifier 流的空命中边界，2026-10-27

本轮先核实际 B 骨架目录：`crates/country-core/src/systems/ui/{mod.rs,hover_intersection.rs,ui_update.rs,resources/ui_state.rs,resources/widget_params.rs,signifiers/signifier_create.rs}`，及 `crates/systems/toolbar/src/ui_interaction.rs`。不改业务算法、producer、consumer 或 host；仅在现有 `ui/signifiers/tests.rs` 承载有界合同测试，不新增业务目录。

## 结论与证据边界

原流的已恢复合同就是：无活动 widget 不取流；首次有活动 widget 时，整批 pending 工厂转移到那个 widget 的独占 WidgetParams。不能因为担忧积累而给 circle 加 hover guard，或无条件清空 consumer。当前证据**没有证明 focused 必然命中**，也未找到独立逐帧 reset。因而不能把这一依赖边界宣布为“已证明绝不积累”；也不能仅靠合成无 hover 状态宣布原游戏实际泄漏。

B RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；PDB SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。本轮 shared Ghidra 全为只读；原 C、原 xref 和完整匹配泛型记录保存在同前缀附件。

## 原创建、消费和释放

- `add_ui140945c60`：原链挂 Update/set6；在初始化尾 `memset(...,0,0x170)`，随后 `140945e78 → App.insert_resource<UiSignifierStream>1415468b0`。此 ctor 的 observed code caller 只有 add_ui；add_ui 的 observed code caller 为 `run_game_thread14059f98b`。这是 App 初始化，不能当每帧清空。
- `signifier_create140b2cfa0`：`UiState+0x18 != 0` 内才拷贝整个 `0x170`B AppendOnlyVec 并将资源归零；随后克隆工厂 String/Arc 到活动 WidgetParams，并在 `140b2d591` drop 旧流。空活动分支不取、不清。
- `drop_in_place<AppendOnlyVec<UiAppendSignifier>>14087d740` 的 code xrefs：正常 create `140b2d591`、其 EH `140b2db36`、resource drop_ptr 的 `1412edcb0`、insert-resource EH `141546e5c`。EH 不可误作常规清理系统。
- 全 PDB proc 表匹配到33条 `ResMut<UiSignifierStream>` 泛型记录，唯一系统 `name<FunctionSystem<...>>` 为 `14086fd70`，其参数正是 signifier_create 的六组参数。其余为同类型的运行/调度/析构实例及包含它的 UI tuple，未见第二个 mutating 系统。其他 producer 原本通过 `Res<UiSignifierStream>` 的内部 AppendOnlyVec::push 写入；不能将当前重构后的 ResMut 签名倒推到原版。

这是已枚举符号/直接 xref 范围的负证据，并非排除所有未知 exclusive World/间接调用的全程序证明。当前源码全生产调用中也仅 create 执行 take；测试、兼容 drain API 的存在不等于生产 reset。

## picker、canvas、输入与时钟不能提供普遍命中保证

- `hover_intersection1408383a0` 的原系统参数为 UiState、MainCamera、CursorTerrainRaycast、CursorPositionSS、MouseDepthValue、WindowSize、Local cache。它从当前 cursor/window 重算 ray，遍历真实 registry 并按最近距离/遮挡仲裁；参数无焦点门禁、无 AppMode，也不访问流。空候选/全部不相交可以产生空集合，不凭空补 widget。
- `clearing_canvas_w_layer_and_sound14092df50` 全部 observed code xrefs 为 ui_draw_wall、wall_eraser、ui_draw_terrain、replay_mode、draw_garden、draw_path 以及两个 wrapper。wrapper14092e390 的调用者是 draw_water，wrapper14092e340 的调用者是 ui_delete_tree。没有 EditingWall 通用兜底 canvas 的直接调用证据。原 canvas extent130×130，其拾取还要求 terrainXZ 落入范围；不能把某绘制工具的 canvas 推广为任何焦点状态全屏必命中。
- 原八类 volume / closure140ac3f50 和 `UiState` 同步保持此前 `evidence/round12/ui-picker` 的独立原指令判等证据。WorldCanvas 缺 terrain 时取ZERO，但首先仍需对应 canvas 已注册。没有 canvas 时这一默认值不会生成候选。
- `render_taffy_ui_to_widgets142338990` 只查询带 UiMouseInteractable/Ui2dExtent/TaffyNode 的实体，以各自尺寸（原 padding）注册 screen AABB；这不等价于整屏无限 canvas。当前恢复也是如此。
- `ui_update140b2de60` 的无命中 hover 分支归零活动状态。WindowFocusEvent、Escape、Shutdown 用于结束捕获，不产生兜底命中。当前也在尾部清空本帧 widget registry，但不清 signifier 流；捕获拥有独立 Arc。
- `is_gameplay_active140a90e00` 只测试 GameTime.ran 与 AppLoadingState::Game。`time_update140922c30` 的 ran 来自 refactoring/menu/paused 门控，无 cursor/hover 依赖。即使生产将 circle 挂此 gate，也不能以此证明 focused ⇒ hover。

circle 的 focused 无 hover 仍写两项请求已由 circle/root 核原 `14165e1c4..e24c /141660880`；本轮不重复改写其 owner 文件。

## 当前可重现的有限资源边界

在既有 signifier 测试 World 中，每帧注册真实 screen widget，鼠标在其外，运行真实 `hover_intersection → ui_update → signifier_create → mark/delete`。逐帧写一个真实 owned factory，32帧得到活动tag0、pending1..32、无wake。鼠标移回实际 AABB 后，首次活动 widget 一次取得32项、流归零，32个同名 factory 因帧开始 entity 快照而均 wake，SignifierId 全归属于当前 widget。

此测试说明 pending 的所有权、有限积累与随后消费；它**不模拟整个游戏所有生产 registry**，不证明完整原窗口中这32帧一定可达。没有 OS 输入，未改变用户暂不真实输入验收的约束。

`CARGO_INCREMENTAL=0 cargo test -p country-core --lib real_picker_no_hit_retains_finite_requests_until_a_later_widget_owns_them --offline -- --nocapture`：实际运行1项，1通过/0失败（测试0.01s），日志 `/tmp/m01-signifier-stream-test-v2.log`。首个短名 filter 错加 --exact 实际0项，已明确作废，不计通过。

## 原与现差别及后续建议

当前重构的 Vec 接口在此生命周期上与原 AppendOnlyVec 保持一致；原 push 可并发、现系统 mut借用调度串行，这是容器实现差别，不授权修改业务消费门禁。普通 hover 每帧替换 params，持续拖动保留 Arc，也影响旧工厂留存；测试不能忽略这一所有权层。

原 App 生命周期结束由 resource drop 释放；当前 host 的直接 Reload 复用 World，既有 `clear_toolbar_session` 在 toolbar_plugin.rs 的流分支用 `*stream = Default::default()` 释放整批 pending，随后清 MEI 会话。该会话边界已存在，不是本轮新增修复；不能把它挪到每帧 create 的空活动分支。前期仅阅读该函数前半段容易漏看此分支，本结论已读到完整函数并纠正。

若要证明原生产永不积累，下一条必要证据应是焦点 Editing 模式的完整 registry 覆盖或原真实窗口有限帧复现；本轮枚举到的 ctor、canvas、时钟与输入链均不能提供该保证。按正确性要求保留显式未证边界，不创造消费策略。
