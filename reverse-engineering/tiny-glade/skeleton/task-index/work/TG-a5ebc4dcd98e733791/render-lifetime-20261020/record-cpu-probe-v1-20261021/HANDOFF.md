# 录制 CPU 探针交接

仅既有 rhapsody 六源，host/业务/ABI/容量/GC/阈值没有改动。原业务 callee 调用与错误/清理顺序保留，新增只读分段包络；coverage原惰性迭代不变，紧循环只做局部两个整数加法，退出一次写固定TLS。原 ResourceOwners 增 crate-private O(1) len，无存储或准入变化。

serial/parallel callback均在原首GPU timestamp后、PGO timer前建立RecordScope；原末GPU timestamp和PGO elapsed写入后complete/drop。false与panic均RAII恢复TLS，先冻结total→restore→I/O，默认每真实callback最多一行。身份graph/pass/command/启用时record_id/name可配对，legacy graph为0；同名不合并。两条owner-transfer独立旧SubmitTrace，graph/command/owners数可对应；retain_resources phase固定在I/O前，旧total字段不作严格归因。

root建议先 cargo test -p rhapsody --lib record_trace_tests，4纯测试需实际执行；再既有serial/parallel executor与ABI/bridge/原world输入，确认off没有RECORD输出、on在真实世界输入和真实chunk worker各得到独立归属。使用RHAPSODY_SUBMIT_TRACE=1后record与submit探针同开，日志前缀不同。作者未Cargo/GPU/生产/Git，只有rustfmt语法解析与源码审查。

后续测量按frame真实record_ms与scope行区分：CompleteGroup包含RequireSlots/DescriptorReserve/Finish/WithPush等子项；ManagedValidate覆盖binding+逐lease角色/覆盖检查；Retain包含现owner保留，native binds/push独立；UpdateDriver区分真正driver与Slots等待；counter等待/更新仅callback子集。多worker的CPU服务时间可汇总，但不可当帧墙钟或互斥phase百分比。probe开启结果用于定位，关闭探针的优化构建用于真实性能。
