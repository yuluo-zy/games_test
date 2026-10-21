# Record CPU 分段探针 V1 合同（工程观测，未改业务）

root 在 push 08ecee0011d9449ebe79dffcd2f9388c490d2f53 后委派此闭包。作者不修改生产、不运行 Cargo/GPU/Git、不启动子 agent；只在原 rawbinding work 冻结候选。目标是同一真实世界输入中分清记录CPU热点，不将debug耗时当最终FPS，也不以pass并行耗时之和代表frame wall。

## 明确开关及输出

复用现 `SubmitTrace` 的进程 OnceLock 开关 RHAPSODY_SUBMIT_TRACE=1：从其 new 提取 crate-private `configured()`，原提交日志格式/样本/调用点均不改变。新增记录探针同开，输出使用独立前缀 RHAPSODY_RECORD_TRACE，不将record聚合塞入原submission的phase字段。开关首次var_os初始化可能分配沿旧机制；关状态稳态无新增Instant、格式化、heap分配或TLS借用。只允许固定枚举/整数的短分支。原已有PGO Instant不属于本探针新增。

每个真实serial callback或parallel worker callback创建一个栈scope，输出最多一行，携带启用时唯一递增record_id、GraphId.raw/legacy0、pass_index、实际command handle、pass名及callback返回/是否unwind。GraphId+pass+command避免不同App/帧/同名pass混淆；record_id补匿名CPU adapter身份。启用之外不新增全局原子计数。

线程局部仅固定大小、Copy的数值数组及scope_id。scope保存旧TLS值并替换新值，Drop先恢复旧值、再输出；嵌套scope、callback false、提前返回和panic均恢复父scope。guard带!Send marker，不允许跨worker析构。每span读取同一cached enabled，随后只在存在当前scope时取Instant；Drop只有scope_id仍匹配才写回固定数值。TLS用Cell，不持borrow跨任何原callee，也不持全局/allocator/manager锁。输出错误忽略，不覆盖业务Error或使cleanup panic。

新scope创建放原PGO timer之前；原callback及GPU timestamp顺序保持。记录原duration之后才销毁scope输出，避免stderr耗时直接进入现pass PGO。启用打印仍会影响整个record wall及调度，仅用于数量级归因；最终速度须关探针的release构建测量。

## 第一阶段范围（六个既有文件，无host API扩展）

- raw/command_buffer.rs：共享原开关；不改submit业务。owner transfer由executor使用已有SubmitTrace测一条/command。
- rg/profiling.rs：固定枚举/数值TLS、RAII callback scope与span、纯诊断单测；不修改GPU timing ring。
- rg/executor.rs：覆盖serial/parallel两个真实callback入口；保原GPU timestamp与PGO次序；两条owner transfer处用SubmitTrace，数量来自ResourceOwners常数时间len。
- rg/owner_storage.rs：仅增加crate-private常数时间len读取存储长度，保持admission/预算/取消。
- bindless.rs：现完整组、reserve/update/bind/coverage与pool counter的只读计时/计数。
- bindless_parameters.rs：resource_requirements与finish/with_push、局部cache命中/实际新增、native push计时/计数。

本阶段不改host、不新增公共profiling API；native cmd首先测本闭包的vkUpdateDescriptorSets、vkCmdBindDescriptorSets与vkCmdPushConstants，raster draw/compute dispatch及host资源解析暂包含callback剩余部分。若剩余显著，再另交最窄host标签，不能先凭剩余猜driver或自行做缓存。

## 可解释字段

嵌套inclusive字段必须明确保留，不能全加：

- requirements：实际resource_requirements次数/元素数/耗时；with_push是否仍重做可计数确认。
- complete_group：cursor.prepare包络，group attempts、page reuse/search/new、insufficient retry；group_lock_wait与require_slots为内部子项。
- descriptor_reserve：budget/SyncCell/slot准入包络；descriptor writes按sampled/storage image/storage buffer/sampler实际成功调用计数。
- update_lock_wait与update_driver：原slots锁等待、实际vkUpdateDescriptorSets分别测量，原锁持有范围不变。
- managed_validate：validate_binding及每lease validate_pass_lease包络，lease数；coverage_accesses及semantics比较按现循环实际访问累加（仅启用时），不额外遍历。
- retain：原heap/pipeline/lease/upload owner收集的耗时与数量；不去重、不跳校验。
- native_bind / native_push：实际vkCmdBindDescriptorSets/vkCmdPushConstants调用；参数finish与with_push含这些前后子步骤需按嵌套解释。
- pool_counter_lock_wait / pool_counter_update：同一原state Mutex的等待与更新段。只做TLS数字记录、无锁内stderr，原drop/锁顺序保持。
- owner_transfer：现SubmitTrace独立scope，每serial/parallel command一条；包括原retain_resources/visit_uses/add_use_cell，不把其时间混入pass callback。

实际字段名可以在实现中按固定枚举统一，但不得无声缺失某入口、把需求次数当descriptor实际写次数，或用counter替代资源owner事实。

## 行为与验证

原顺序、条件、first Error、fallible准入、锁生命周期、回滚/取消、GPU use mark、跨App owner均保持。探针只读并按固定存储饱和累加，不引入可失败分配/新Result，不修改GC预算、descriptor容量、绘制数量或原算法。所有RAII span包含失败路径，使失败前的实际工作可见。

作者先rustfmt解析/静读；root后续运行诊断纯tests及原真实8bridge/30/WSI门；同一输入要有off release和on单帧/有界数帧配对，输出按GraphId/pass/command关联。至少验证串行与真实8chunk路径、错误/取消不变、禁用没有新时钟/alloc逻辑。diagnostic tests不计游戏业务恢复进度。
