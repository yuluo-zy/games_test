# Rhapsody parallel recovery batch 06

日期：2026-10-12。本轮按 [WORKFLOW_RESET](../WORKFLOW_RESET.md) 和 [AGENT_EXECUTION_GUIDE](../AGENT_EXECUTION_GUIDE.md) 收敛到共享 consumer；旧批次门禁保留其限定范围。

三个 worker 已完成接口交付，主流程已在 `reconstruction/render` 接通公开入口 `execution::ExecutionChain::execute_graph`。本轮早期 R8 独立包作为现有实现使用，后续不再新增孤立 crate；ResourceInfo/graph compiler 必要源码迁入共享模块以避免反向依赖循环。旧 oracle 包不修改，不把 candidate 链计作原作完整入口等价。

| 工作包 | 验收目标 | 独占写入 |
|---|---|---|
| sequence-runtime | 原counter sequence结果贯通bucket、submit、complete、CB回池/资源drain，保留原碰撞语义 | sequence-runtime、sequence-runtime-r8 |
| queue-submit | 原所选queue_submit完整CPU前段生产wait/signal/CB/Vulkan submit数组，而非注入已造好的数组 | queue-submit、queue-submit-r8 |
| descriptor-binding | 原sparse BindingIndex到layout，并恢复descriptor write/update/bind最小路径 | descriptor-binding、descriptor-binding-r8 |
| 主流程 | graph→compiled→record→counter→submit→completion共享入口；temporal消费者取证 | 共享render、链级场景、总进度 |

## 本轮真正接通的入口

`GraphDeclaration → ResourceInfo/Registry → CompiledGraph → recorder → DeviceCounters → queue_submit → receipt → completion/reclaim`

输入为图/资源/pass/export声明、外供依赖classifier、明确的设备/managed资源后端与callback。compiled字段自动提供initial/reused、first/last访问和pass范围；sequence由原counter规则从1产生。去除手填访问字段、固定sequence7、预造compiled snapshot和预造Vulkan submit数组。Registry物理token交给显式managed-resource provider；共享代码不猜generation。

正常、callback失败、queue失败三个共享入口场景通过。queue失败在原作用顺序中已注册timeline并持有pending commands，入口保留这些状态、不给成功receipt，并显式阻止假completion。locked/offline check、链测试与all-targets Clippy通过。证据见 [shared-entry-validation.json](shared-entry-validation.json)。渲染锁文件新增本地包；12个registry依赖版本/source/checksum仍与原锁一致，旧leaf源码/runner未变，旧报告hash不覆盖改写。

## 尚未闭合

- 原游戏的GraphDeclaration构建生产者、原完整compile ABI和完整graph-to-submit原侧oracle。
- raw device/command创建、物理ControlBlock/generation生产者、真实锁/end-error/unwind及fatal recovery。
- 共享入口真实executor waits/signals、prior/flush生产者，多队列与并行录制。
- descriptor worker已交付reflection→sparse layout→write/update/bind接口；共享callback与真实shader/interface选择仍未接通。
- temporal跨帧整体闭包、reset/unused-key/drop与country-core consumer。

本轮为 **可运行CPU候选共享链**；`whole_original_chain_validated=false`、`live_gpu=false`、`module_complete=false`。不按案例数给新批次“整合百分比”。

持久数据库按取证窗口串行，CPU实现与原指令oracle并行。使用已验证只读SDK runner，无auto-analysis、save:false；xref空结果不作无caller证据，跳表伪码以限定原码纠正。

公共依赖按原锁复用。GPU/API capture、CPU原入口、注入片段和候选测试分别登记；只有实际关闭的缺口才更新台账。完整GPU与游戏画面仍未验收，不报告整体百分比。
