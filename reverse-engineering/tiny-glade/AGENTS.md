# 逆向工作约束

函数进度必须区分自定义业务、宏派生/依赖转发、类型/schema合同和归属待分类项。已确认的Default/Debug/fmt/Clone/Eq/Hash/serde visitor/Bevy注册胶水直接复用derive或原依赖，不计独立未恢复业务函数；字段、枚举、默认值、serde属性和required-components策略按类型合同恢复。不能仅凭方法名删除自定义实现；Timer/Transform默认状态、CurveU范围反序列化等自定义策略仍是业务。证据不足单列待分类。共享基础权威分类入口为tracking/shared-foundation-interface-classification.json与INTERFACE_CLASSIFICATION.md。

当前共享七包的实际源码/逻辑缺口按FOUNDATION_BUSINESS_FUNCTIONS.md和FOUNDATION_BUSINESS_COMPARISON.md逐功能闭包推进。没有原同名helper不证明缺代码，合并后的等价业务实现也必须登记；源码缺失、原验证不足、库外游戏接线以及C-only范围分别列出。原接口库存只作定位和映射，不能解释为同数量的未实现功能。

先读 [PROJECT_MEMORY.md](PROJECT_MEMORY.md) 与 [TASK_BACKLOG.md](TASK_BACKLOG.md)。以原作锁定依赖与游戏自有源码为边界；直接复用依赖，不恢复其实现。当前源码优先，场景/GPU后置。依赖适配的变更必须标记旧差分报告过期，重新校验后再更新进度。

涉及rhapsody恢复时，执行[流程重审](evidence/render-recovery/WORKFLOW_RESET.md)。恢复单位为真实consumer驱动的可调用源码闭包；停止新增孤立candidate crate和按函数派任务。优先将已有实现接入共享恢复入口；只有源码/依赖/输入变化或具体语义歧义才重验相关差分。报告必须区分真实代码接线、candidate链测试与原侧行为验证，不以case数/报告一致性替代集成进度。
