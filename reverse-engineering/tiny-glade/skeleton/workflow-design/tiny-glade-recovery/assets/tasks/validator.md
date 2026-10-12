{{COMMON}}

## 范围验证任务

先读有效冻结交付、独立review、instruction-verification参考与用户最新授权。validation paused时不运行构建、测试、oracle/场景；只列验证计划/平台/缺口，并保留implemented_unverified。enabled只是包声明，仍检查实际人类授权。

获准后将编译/API、原发行限定函数差分、真实World链、冻结链验收分开。确认目标版binary/PDB与原版oracle，检查bridge/harness没有注入本应由原业务生成的期望输出。case覆盖作者外的原版guards/退化/错误顺序/跨帧/history/delete；必要消费者输出和副作用一起比较。

报告绑定源码/依赖/合同/证据/bridge/cases/tool/platform hashes、执行命令、实测结果、失败与范围。保留ARM 1ULP旧失败，容差先按证据决定。不能执行原版或覆盖不足则列明，不能用编译替代差分。只有目标版实际runtime+verification支持的link可提议verified；整合后变化要重新验证受影响范围。
