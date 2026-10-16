# 下一批分派建议(2026-10-15,coordinator)— 验收层面开放项

> 背景:本批 9 模块已全部 `review_passed(限定已实现范围)`(DISPATCH.md 确认)。本文件把每模块的
> **验收层面开放项**按依赖顺序编排为下一批任务卡,供主会话派给 workers。域类型证据已派
> re-analyst-state(2026-10-15 续派)。

## 依赖顺序总览

```
[证据前置]                                     [组装/接线]
域类型 schema(re-analyst-state,已派) ──► 卡1 save GameSaveData 域组装 ──► 卡2 会话切换链
(独立可并行)卡3 Steam 全链 / 卡4 录制往返 / 卡5 DecoratorStorage 实体
(库内已备)卡6 render 域 GPU 接线(ivy send_to_gpu 已在库内,等 GraphicsDevice)
```

## 任务卡

### 卡0 域类型证据(re-analyst-state,2026-10-15 已派;本卡登记追踪)
- **目标**:save_game `to_json_string` 各节载荷类型 schema/layout 定点挖证:
  `CameraRigParametersCache`(Camera 节,camera-time 域)、`HistoryIdentity`+`HistoryChainSnapshots`(Identity/snapshot 节,history 域)、
  `GladeColorCustomization`(Colors 节)、`GameProfile`(profile 节/backward-compatibility)。
- **产出**:RE-ANALYSIS.md + decomp,字段序/默认值/serde 键,供卡1 域组装与 load apply 复用。
- **依赖**:无(独立)。**验收**:每类型字段序与原生逐字节对上,登记章节回填。

### 卡1 save GameSaveData 域组装(state/restorer,依赖卡0)
- **目标**:save_game 0x140a3f470 同步路径 GameSaveData 组装(16+ SystemParam 源 → 栈上大结构
  +0x28..+0x100)+ 各节 `to_json_string` 载荷真实接入;补 `load_completion_checker`
  (SessionLoadReadiness)与 `save_file_identity` 往返。
- **依赖**:卡0 域类型 schema。**验收**:捕获→Arc→set_current_snapshot→AsyncSaveCmd 全链、
  节载荷类型闭合、文件 identity 往返。
- **开放(不阻塞卡1)**:`AutoSaveState` 阈值门(0x140a3f470 行 507)、计数器口径。

### 卡2 会话切换链(state + startup,依赖卡1;全局调度收口)
- **目标**:`load_game_session` 0x14166e7d0 完整切换 + `commander_update`(meta_commander.rs
  新建:80 B MetaCommander/MetaCommand 0x38B tag 0..0x13/fence)+ load_game on_load 回放 +
  `SessionFlowPlugin::build` 全局注册。这就是 README 的"完整全局调度/存档往返"收口链。
- **依赖**:卡1(域组装)、meta_commander 证据(re-analyst-state 已出 RE-ANALYSIS)。
- **验收**:新会话复制/继续会话/meta.json 写读/历史快照载入全链、NextSession 变体 1/2 两遍回放、
  不伪造加载成功。**红线**:load_game_session 未闭合前 session/save 不得标 100%。

### 卡3 Steam 全链(interaction/restorer,独立可并行)
- **目标**:`translate_input` 的 Steam 门(`IsSteamRunningOnSteamDeck`)、设备列表、MainActionSet
  动作↔槽映射、绑定表(0xc3 哨兵)、analog 翻译仅 mode==2 或(10 且 sub!=1)。
- **依赖**:input_record_replay/update_cursor_position 库内已闭(review_passed)。**验收**:键盘/
  鼠标/Steam 三源→翻译→光标/动作增量→录制回放全链闭合。**开放**:动作↔槽映射真值表待取证。

### 卡4 录制往返(interaction/restorer,独立可并行)
- **目标**:`input_record_replay` 帧 serde / `try_load` / `SyncPoint`、`input_replay_for_trailers`
  `record_and_replay_input_for_trailers`/`do_input_replay_ui` 可见消费。
- **依赖**:无(独立)。**验收**:录制帧往返(写→读→回放)与帧步进与原字节/章节对上。

### 卡5 DecoratorStorage 实体(interaction/restorer,跨 crate 接线)
- **目标**:ui_clutter 可见消费——`DecoratorStorage` 实体机制、`ui_convert_autoclutter_to_decoclutter`
  quat 接线、`decoclutter_show_pins` 注册、`ui_focus_decoclutter` 主体、`debug_spawn_clutter`
  消费端;与 `system_clutter::impl$0::build` 库外消费对账。
- **依赖**:system_clutter 库外接线(跨 crate,宿主决定范围)。**验收**:拾取→转换→生成→延迟历史
  可见链闭合。**开放**:DecoratorStorage 实体布局待宿主/库外补证。

### 卡6 render 域 GPU 接线(eco/ivy + render 域)
- **目标**:ivy `ivy_send_to_gpu_leaf/segments` 提交需 `GraphicsDevice`(render 域)、`SsboId::of`
  建模对账(与 GroundedBuildingsSsbo 同法);garden pin 渲染、terrain 延迟回读 GPU 执行、
  `read_terrain_data` 消费。
- **依赖**:ivy 库内 send_to_gpu 已闭(review_passed);render 域 GraphicsDevice 属宿主接线。
- **验收**:GPU 提交链(ivy leaf/segment/flower SSBO)、garden pin 渲染、terrain 回读真实执行。
  **开放**:48B↔104B 叶映射、SsboId NAME、Flower03Ssbo NAME 待取证。

## 卡点(回主会话裁决)

- 卡2 会话切换链依赖卡1 与 meta_commander;若 re-analyst-state 的 GameSaveData 逐字段 layout
  仍卡(run_game_thread 3 命令载荷 / fence 双 tag / 变体 2 unblock 偏移),回主会话裁决是否
  拆分卡2 或改派 interaction/reanalyst。
- 卡5/卡6 属跨 crate/宿主接线(DecoratorStorage/system_clutter、GraphicsDevice render 域),
  需主会话确权范围后再派。
