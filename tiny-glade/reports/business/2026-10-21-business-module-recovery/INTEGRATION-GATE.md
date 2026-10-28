# 输入闭环集中合入门槛

本文件记录渲染性能构建冻结期间的候选。未落生产、未编译、未实际游戏验证的项目不得写为完成。

## 第一原子批：几何、输入 owner、完整事件

1. `merge_ui_owner.py --stage` 窄合并完整八类 Geometry、24B PlayUiSound、active Arc 与完整 intersection、history owner 和 picker，保留当前库架构。
2. `stage_geometry_consumers.py` 准备 MEI 完整矩形、两个原机器码支持的 utils 边界修复，以及 wall_splitter 仅测试字段迁移。mesh_volume builder 保留，Vec 只归 Geometry 所有；resolved_geometry 借用 Cow，不重复克隆网格。
3. toolbar geometry/Replay hunk 从候选应用，不覆盖整文件；Replay 工厂只用恢复的真实闭包。
4. 全部 24B sound writer 同批迁移。ModeJustChanged 以 canonical current+previous alias 统一，不能用补发 `_gap=0` 的桥破坏原事件。onboarding 三处消费原末事件。
5. 导出 `ui::hover_intersection`、`ui::ui_update`；导出原例与真实 CPU 链路测试，必要锁文件更新仅引用已锁版本。

合入前逐文件核对候选基线 SHA 或使用窄 hunk；禁止覆盖并行渲染、用户及其他专项修改。

## 宿主由渲染中央集中接线

- `run_game.rs`：在真实 `UiState::at_startup` app owner 插入后，注册 `hover_intersection → ui_update` 原 set6 顺序。不得将 ray producer 错移到本帧前面：原 picker 自行从相机矩阵生成射线，读取前序已存在 terrain/depth owner。
- `mei_host.rs`：`mei_pre_update` 同 set6 且明确 `.after(country_core::systems::ui::ui_update::ui_update)`。原 MEIPlugin 的 after system_type 由 PDB/机器码确认，不由猜测决定。
- `run_game.rs`：在 PhotomodeState app owner 旁 `init_resource::<SteamdeckScreenshotMsgScreen>()`；这是 app 生命周期，不随 glade 重载重置。
- `toolbar_plugin.rs`：原 set7 链先 `hide_ui` 后 `render_taffy_ui_to_widgets`。源码恢复并不代表 Steamdeck 消息 producer 已恢复。
- signifier 原 set6 九系统尾段需完整独审后单独集中接入，不能把当前错误 Raw 占位 consumer 注册进主循环就称恢复。
- host physical-chain 测试需已锁 `bevy_tasks` dev dependency，仅用于真实 CPU picker worker pool。

## 真实证据分层

- 已执行：TOD 2/2、hide 3/3、toolbar Game full/demo unload/reenter 1/1。
- 原夹具待实际执行：picker 39、arbitration 21、ui_update 600+150；不能引用历史执行结果代替当前源。
- 新真实 CPU 输入链待执行：物理输入资源 → 原输入转换 → picker → ui_update → toolbar → mode_manager → 原事件 publisher；历史命令必须到下一帧，不允许调度倒流。
- 捕获 owner 卸载待执行：真实终止事件与原 unload 后 Arc/实体代数释放，同名重入不能幽灵点击。
- OS 小窗口输入、MEI 遮挡、画面同步、完整业务保存/回放及持续帧率仍须实际验收。

## 仍未闭合的依赖

原 signifier 是 hash+widget 16B，不是现有 8B hash；active params 持有工厂、inactive 不排空 stream、创建去重查询是批开始快照、标删除与动画结束销毁各有精确条件。专项完整恢复中。

PlayUiSound 当前无真实 audio dispatch consumer；不得混同另一类型的 PlayMenuUiSound。toolbar 尚有原导航/抢鼠标等宿主缺项，最终完整性需逐项复查。

## utils 窄修影响面复查

`is_inside_bbx` 当前生产源码只有定义，新 picker 才是新增调用者；`Aabb2::contains` 公共包含边界约定保持不变。

`Plane::intersect_ray` 除新 picker 外的当前真实调用者是 `resources/terrain_height.rs` 的 y=0 外围平面回退。该 consumer 本身按原 `t_max >= t` 再过滤 NaN，并要求位置越过 WORLD_HALF_EXTENT，因此不能把本应留给 consumer 的有序比较移进通用 helper。其它 `.intersect_ray` 搜索结果多数是 terrain 自己的方法，不是 Plane。外层 terrain 原有排序/判域保持不变。验证必须同时跑 utils plane 边界与 core terrain 相关现有回归，不能仅跑 picker 后假定无影响。

toolbar 整体保持部分完成；不得开始地形实现或以末尾联合验收补当前遗漏。性能执行 `D:/game/PERFORMANCE-STANDARD.md`，完整画质/业务下 60 FPS 最低，120 FPS 合格，持续区间与低帧/帧时间分别记录。
