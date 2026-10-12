# 逆向工作约束

先读 [PROJECT_MEMORY.md](PROJECT_MEMORY.md) 与 [TASK_BACKLOG.md](TASK_BACKLOG.md)。以原作锁定依赖与游戏自有源码为边界；直接复用依赖，不恢复其实现。当前源码优先，场景/GPU后置。依赖适配的变更必须标记旧差分报告过期，重新校验后再更新进度。

涉及rhapsody恢复时，执行[流程重审](evidence/render-recovery/WORKFLOW_RESET.md)。恢复单位为真实consumer驱动的可调用源码闭包；停止新增孤立candidate crate和按函数派任务。优先将已有实现接入共享恢复入口；只有源码/依赖/输入变化或具体语义歧义才重验相关差分。报告必须区分真实代码接线、candidate链测试与原侧行为验证，不以case数/报告一致性替代集成进度。
