# Toolbar 生命周期专项（2026-10-21）

本轮没有发现可直接裁定的 toolbar_startup / undo_redo_systems 生产差异，未为制造增量而添加额外清理。新增真实状态转换回归由业务中央实际执行，1/1 通过（同一测试包含 full/demo 两分支），日志 `D:/game/tmp/business-toolbar-reload-tests.log`。专项未另行运行 Cargo。

## 五项交付条件

| 条件 | 本轮证据与未闭合范围 |
|---|---|
| 真实输入触发原业务 | 静态链为 render_taffy_ui_to_widgets 的 HistoryButton → canonical core UiHistoryButtonPressed → translate_input 末事件 → digital 16/17 → calculate_input_delta → mode_manager → HistoryCommand。真实 UiState.interaction producer 被独审发现缺失，最高优先修复中；不能把注册和手填事件当实际输入通过。 |
| 权威状态、实体、碰撞、画面同步 | toolbar 只选择模式和发出历史请求，业务状态由各模块 consumer 负责。此次没有验证实体/碰撞/GPU同步，不称完成。 |
| 撤销重做、保存恢复完整状态 | undo_redo_systems 只负责两按钮布局与自动隐藏；历史命令与文件保存的权威 owner 在 core History / save_game，不能给 toolbar 另造并行存储。各业务完整 payload 的回放和文件恢复待各模块逐项验收。RequestHistoryCommand 当前没有生产 writer，UiHistoryPlaySelectAnimation 没有 consumer，原归属待核；它不是鼠标历史按钮实际事件。 |
| 删除、卸载、再次进入无残留 | host OnExit(Game) 已真实调用原 despawn_glade_entities，Query Without<KeepOnLevelUnload>；toolbar 实体不含 Keep 标记。OnEnter(Game) 原 toolbar_setup 重置 Taffy，并创建普通/摄影/undo 三 owner。新增回归使用实际 OnEnter/OnExit 调度、full/demo 两分支，核旧 entity generation 消失、保留 owner 不丢、所有 Taffy/UiNav/父引用有效、两个历史按钮唯一、隐藏进度重置。中央实际执行 1/1 通过；新输入 owner 的 captured Arc/registry 生命周期另有候选验证，不能由此替代。 |
| 原证与实际链路验证 | 既有原证为 phase2-toolbar-layout/native-toolbar-setup-trees.json、phase2-toolbar-layout/native-schedule-config-names.json、central-undo-layout/native-undo-setup.json、phase2-undo-systems/original.asm；本轮读取实际生产接线，未重新运行原机器码/游戏/GPU。状态回归不能替代最终实际输入/画面验收。 |

## 已通过的回归与执行命令

源：`crates/systems/toolbar/src/toolbar_startup_tests.rs`。

```
cargo test -p toolbar --lib actual_game_reload_rebuilds_all_toolbar_owners_without_stale_entities --locked --offline -- --nocapture
```

此测试仅重用原 unload consumer；未添加假 ready、假 UI interaction、stub input 或简化算法。中央拥有 Cargo/GPU，专项遵守禁止并发编译要求。普通/摄影/undo 构造器和两份生产文件保持原样。

性能另按 `D:/game/PERFORMANCE-STANDARD.md`：60 FPS 为最低门槛，至少 120 FPS 才达到帧率合格门槛。此 CPU 回归不提供性能合格证据；必须以小窗口 960×540 Release、完整业务/画质实测持续 FPS、1%低帧及 p95/p99 帧时间，保留保存/历史/卸载清理。

## 追加：真实输入 producer 与历史生命周期的依赖

- `event_loop.rs` 的实际 OS MouseButton 直接调用 canonical MouseButtonInput.press/release，raw 0 是左键；确认后的绝对移动和相对移动写 InputCursorMoved。
- host `install_gameplay_device_input` set5 的实际顺序为 translate_input_to_cursor_moved → translate_input → calculate_input_delta → input_record_replay → (mod_key, update_cursor_position)。ActionSetStates 是 MainActionSet 的同一身份；结果为 digital 0/1/18/19 等状态和 CursorPositionSS。set3 reset_input_state 每帧轮换 ActionSetValues。
- set9 更新 MainCamera；set10 的 camera_mouse_ray → mouse_terrain_raycast → (mouse_capture, read_mouse_depth_value_from_gpu) 已生产 CameraCursorRay / CursorTerrainRaycast / MouseDepthValue。新 UI 仲裁必须按原证安排，不能因为 toolbar 在 set7 就假定其射线依赖在同帧已更新。
- 现成几何边界有 utils::geometry::intersect 的 ray_triangle_intersection / ray_sphere_intersection、RaycastWorld.raycast/raycast_w_terrain、TerrainHeightsData.intersect_ray。应复用当前实现与真实 owner，不复制旧底层库。
- 当前 UiState.widgets 和 clearing_canvases 只有插入与查询，无输入仲裁、候选枚举、每帧交换/清空和完整 active payload。实体卸载本身不会清空这些 registry，因此完整无残留验收还必须覆盖新输入 owner 的帧生命周期。
- 当前 toolbar screen-volume(tag3) 只把 Ui2dExtent.size 写入 volume.size，没有写屏幕中心 extent.pos；需要原证判定完整 volume 合同，不能在缺输入 owner 时误称全部状态正确。
- history/recording.rs 的 start 直接要求真实 UiState hover >= 2、phase == 3、active widget RECORD_HISTORY_WHEN_USED。输入 owner 的缺失同时阻断常规工具历史事务起点；历史孤立测试不能覆盖这个入口。

## 追加：PlayUiSound 迁移合同

原 round12/ui-update 保留证据确认业务载荷是 24B：kind u32 + native +4..24 共 20B；原 EventInstance 额外 +24..32 为 Bevy EventId。旧 PlayUiSound 的 id 字段误把事件计数当业务载荷，且漏掉 native +4 嵌套判别。

中央修改 canonical widget_params.rs 定义；专项迁移现有 writer，保持所有 kind、分支、事件顺序，不把 UI 声音替换成另一套 PlayMenuUiSound。当前源码没有 EventReader<PlayUiSound>；audio/menu_audio.rs 消费的是独立 PlayMenuUiSound，不能据此宣称已播放。完整 UI audio dispatch 仍为独立未闭合项。

Release 冻结期间，迁移仅放入本报告 `proposed/tiny-glade/crates/...` 下，**未改 9 个生产 writer**。`sound-writer-migration.patch` 是逐处窄补丁，`sound-writer-migration.json` 绑定原路径/原 SHA256/候选 SHA256。共迁移 9 个原全零载荷 literal 和 2 个 variant4 调用；color 的 kind9 已用 Default，无需修改。当前所有源 writer 均无非零自定义 payload，不存在重排时丢失非零数据的问题；新 ui_update 应使用 from_native_slot 保留全部 24B。

合入必须在中央统一窗口、canonical 新类型与全部 writer 同时落盘，先检查每个原 SHA 匹配。若某个文件已被另项改动，重算窄 hunk，不得以候选全文件覆盖新的改动。此阶段仅完成静态逐处复核，未编译/未运行。

## 新增未导出：输入 owner 卸载与重新进入

候选为 `proposed/tiny-glade/crates/country-core/src/systems/ui/ui_update_owner_lifecycle_tests.rs`，预期作为 `ui_update.rs` 的测试子模块导出。**未挂载、未编译、未运行**。现有三条 chain 已覆盖普通 press/hold/release、指针移出取消、同目标 Escape/Shutdown/焦点丢失的结束规则；新测试不重复这些分支穷举，只补 owner 跨状态的严格生命周期。

新测试包含 Escape/Shutdown 两分支：真实 picker 选取屏幕范围 → ui_update 捕获 Arc → 指针移出并发送真实终止事件 → phase0 的末帧仍持有 Arc → 原 OnExit(Game) despawn_glade_entities → 下一 Update 无注册目标，ui_update 自动清掉 active/registry/picker owner → Weak 无法升级 → 原实体槽实际复用但 generation 改变 → 同名 WidgetId 再注册，持续 Held/首次 Release 都不能触发旧点击 → 新 press/release 只能向新实体发布点击。

边界：test-only RegisteredTarget 查询生产者创建一个真实 ECS 实体并逐帧登记真实 WidgetParams，交互查询由真实 UiWidgetContext 完成；没有直接 clear_widgets、replace_interaction、set_hover_intersections 或填 clicked/used。这里不声称注册 fixture 是完整 toolbar 生产者，也不声称 Shutdown 已执行整个宿主退出；它验证共享输入 owner 对真实事件的处理。完整 toolbar 构造/删除由上面的 1/1 GameReload 验证，物理输入→完整 toolbar consumer 与实际游戏性能由对应专项验收。

静态复核后，候选已纠正为原跨帧顺序：picker → ui_update → consumer → 本帧注册，下一帧消费；首次进入与重新进入各有真实注册预热帧。没有为了让测试同帧命中而将注册提前到 picker 之前。已做 rustfmt 仅解析，仍未编译/执行。host 的 Reload PostUpdate 原闭包在下一状态转换进入 Game；真实 UI owner 卸载测试不替代全 host 会话切换。当前原 ui_update 在持续拖动且临时缺注册时有意保留 captured Arc，不能为消除测试残留而擅自删除该原始行为；卸载是否总有真实终止事件需在宿主会话回归中继续核验。

继续核对到的会话依赖：`menu/load_ui::queue_load` 与 `MetaCommander::commander_update_system` 的 SessionLoad 均能请求 Reload；前者来自已打开的 Escape/load 菜单，后者由 timelapse start/interrupt 排入。`startup_session` 写 AppMode::LoadGame 和 NewSessionStarted/SessionLoaded，未重建 UiState。`reset_input_state` 收到会话事件仅 reset keyboard，MouseButtonInput 持续状态不因此复位；`translate_input` 在菜单/模式条件前读取 mouse.pressed(0)。因此会话事件本身不是拖动终止的证据，直接 MetaCommander reload 和捕获状态的共同生命周期仍为 Unknown，需要真实系统联合回归。不能把调用 OnExit 或键盘 reset 等同于所有 UI owner 已释放。

## 模式事件的新增闭包候选

`ModeJustChanged` 原恢复为独立 tool+gap 结构，真实 publisher 却写 AppModeChanged，导致 toolbar 选中/动画及 TOD/color/tutorial 通知断链。已按原140a875e0将两名字统一为同一完整current/previous类型；5个读取native tag的consumer迁移，onboarding三处原批末事件语义纠正。候选、SHA和真实publisher回归见 `mode-event-identity-audit.md` 与 `mode-event-migration.json`。生产仍冻结，不能算通过。两份onboarding候选同时包含sound迁移，合入时使用最终candidate避免互相覆盖。
