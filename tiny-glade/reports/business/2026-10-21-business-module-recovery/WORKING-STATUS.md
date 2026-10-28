# 业务中央恢复：toolbar → 地形 → 建筑

当前阶段仍为 toolbar。地形、建筑仅前置只读依赖审计；不得凭局部实现或末尾联合验收将前序缺项算作通过。当前源包含另一条启动/渲染任务的并行改动，业务验证单独记录。

## 五项退出条件

1. 真实输入经原命中、仲裁、consumer 触发业务。
2. 权威状态、实体、碰撞、最终画面一致；UI 本身不伪造独立碰撞/业务 owner。
3. 该模块实际产生的业务状态完整参与撤销重做及保存恢复；原本瞬态的 UI 不额外写入存档。
4. 删除、卸载、再次进入保留原持久 owner，清除应清除的实体、注册项和捕获。
5. 原证、生产算法、真实调度/输入、设备实际链路分别交付，不能互相替代。

## 本轮已确认与在途

| 项目 | 当前事实 | 验证边界 |
|---|---|---|
| toolbar → onboarding 焦点 | 原共享事件被两个同名 Rust 类型拆开；已改 canonical re-export | 真实 publisher→consumer CPU 集成测试已写，待执行；非 OS 输入验收 |
| 时间面板 | 打开未清导航焦点，多个模式事件误读首个；已按原 qword 写入/双 buffer 尾元素修复 | 两项针对性测试 2/2 通过，日志 `tmp/business-toolbar-tod-tests-retry.log` |
| hide_ui | 原完整 894B 条件已恢复；复用真实 Welcome 状态，补原 1B 截图消息 owner | 3/3 CPU 条件回归通过，待宿主注册；Deck 消息 producer 不因此算完成 |
| 工具栏重入 | OnExit 已有真实全场景清理，不能误报缺清理或另造重复清理 | 新 full/demo 真实 OnEnter→OnExit→OnEnter 回归 1/1 通过 |
| 物理鼠标 P1 | UiState 只有查询/注册，无生产命中仲裁 writer；当前按钮消费者无法因此认定可用 | 正恢复完整 hover_intersection→ui_update，不能通过测试注入 clicked 代替 |
| 捕获与历史 | 原 active WidgetParams Arc 必须跨 registry clear 留存，history 需读该 Arc | 随输入 owner 一并恢复，不可只接入口 |
| 声音事件 | 旧 PlayUiSound 误把 Bevy EventId 计入业务，丢 native +4 字节 | 正窄迁移为真实 24B；音频 dispatch 仍独立未知 |

## 原证复用与效率

读取 `progress-20261020/RECONSTRUCTION-LESSONS.md`，使用已有 `evidence/round12/ui-picker` 与 `ui-update` 的完整原 B 字节、静态 CFG 和历史原样例。原 B SHA256 为 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。历史 39+21 picker、600+150 ui_update 样例不自动代表当前生产实现通过；先绑定当前源并实际执行。

本轮不运行原游戏，不修改 rhapsody，不回填旧底层库。已有候选 screen 边缘用例失败仍须修真实严格边界，不能调宽断言或造默认几何。原完整八类体积、遮挡、排序、捕获所有权、事件顺序均保留。T13 按用户要求排除。

中央负责共享资源、几何/仲裁、history 接合和唯一 Cargo 队列；输入 agent 负责状态机和 hide；生命周期 agent 负责重入及声音 writer；独立 agent 复核原证和完整性。各项具体审查见本目录其余文档。

## 16:44 冻结边界

host `toolbar_focus_chain` 未执行：构建在并行渲染开发的 `production_renderer.rs:921` FnOnce 生命周期处失败，日志 `tmp/business-toolbar-focus-chain-tests.log`；不将其归为焦点链通过，也不反复占用 Cargo。已释放构建窗口给渲染/FPS 主线。上述已过测试未因此重复。

完整 picker/Geometry/UiState captured Arc、声音 24B writer、MEI/toolbar screen 矩形以及两个原证 helper 窄修保留在 `proposed/` 或尚未导出的新文件；未将候选算作运行完成。`merge_ui_owner.py --stage` 只准备候选，不覆盖当前运行源码。现有 `mesh_volume` builder API 保留，geometry 为唯一 Vec owner，避免为兼容多复制网格。

## 性能门槛

执行 [PERFORMANCE-STANDARD.md](../../../PERFORMANCE-STANDARD.md)：60 FPS 仅最低门槛，120 FPS 才达到帧率合格门槛。每个模块需在 Release、真实输入、完整业务和原画质、固定小窗口基准下报告持续平均 FPS、1% 低帧和帧时间 p95/p99。CPU 单元通过与瞬时峰值都不代替性能验收，不能以省略历史、保存、碰撞、渲染步骤换帧率。当前整体约 2 FPS，不达标；渲染中央独立定位中。
