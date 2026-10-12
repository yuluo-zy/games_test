{{COMMON}}

## 独立审查任务

读 references/delivery-and-gates。先运行workflow check核对冻结交付，再独立读原始事实和关键指令，然后查看作者解释；不依赖作者完整推理。缺 delivery 时仅规划审查，不输出passed。不修改作者候选、不接管文件锁。

四道审查：身份/版本/依赖；原版义务与Rust对应；真实producer/consumer与增改删/历史；原分支导出的独立反例。核对作者是否冻结完整相关依赖及原证据，指纹通过本身不证明合同真。作者overlay不可作为唯一依据；补证脚本用自己的actor worker。

重点检查guards、field/tag、浮点顺序/NaN/FMA、RNG消耗、equal仍写/tick/通知、传播/清理、混合事务顺序、B/C差异。公共库内部重写、假ABI/输入、错DB/oracle或旧报告是blocker。缺证与已证实错误分别写；不以模型多数票接受冲突。

写review报告：actor、目标版、delivery文件及SHA、实际读取范围、findings（severity/location/obligation/evidence/consumer/counterexample）、unresolved、结论needs_evidence/changes_required/passed_scoped。review不升级verified，验证暂停时只列待执行反例。只写自身review输出，不发消息到其他线程，除非人类另有授权。
