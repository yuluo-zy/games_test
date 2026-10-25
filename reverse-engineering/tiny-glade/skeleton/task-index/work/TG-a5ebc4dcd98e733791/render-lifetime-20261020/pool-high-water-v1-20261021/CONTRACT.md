# Bindless 整池真实高水位合同 V1

## 已观察失败

真实共享常量池版30帧全部渲染完成，live稳态约3040..3056且原live断言通过；失败仍是原测试 peak+16断言。当前 BindlessPagePool::stats 把各页各自历史peak相加。不同页在不同时间达到高水位，会让这个和持续变大，即便整池实际同时存活量不增加。该值原先已注明保守上界，不能继续作为整池peak语义，也不能靠改阈值绕过。

本改动是当前工程分页适配的统计修复，不声称原版B存在此分页算法。不得清历史峰、不将当前值伪装峰、不减少绘制/资源、不改阈值/GC/页容量/lease寿命。

## 统计域与所有权

- 每个 BindlessPagePool 持有一个仅计数的 PoolCounters Arc，无指向Heap/Page/Pool的反向引用。Counter含现 BindlessStats 的整池一致快照和预算ticket。
- Slots 使用现 TempList<Arc<PoolCounters>> 登记订阅，正常只有一个；原公开API允许同一template被多个Pool使用，也允许池建立前template已有leases，不能用唯一tracker覆盖或新增此类限制。
- Counter pool lifetime 从首次注册当下current seed开始。既有heap自己的历史peak完整保留；此前独立heap历史不伪造为新pool的同时峰。Pool新页注册时纳入其current容量/占用。
- 使用强Arc订阅而非Weak：避免Weak controlblock延长实际分配但ticket已归还，也免除每次事件Weak upgrade。Pool Drop在其独占&mut self下从pages.get_mut逐页取消订阅；Slots Drop处理未成功加入manager的新页注册回滚。无Heap反边，故无引用环。

## 原子事件与预算

- 只接当前Slots全部实际状态改写点：成功allocate、generation溢出转Exhausted、Live→Retiring或NeverUsed→Free、collect已达到真实SyncSet的Retiring→Free、注册/注销/Slots Drop。
- 每个事件在原Slots锁内一次获取该Counter短Mutex，更新整池及per-kind current/free/exhausted，再以max(old,current)更新真实peak。Live→Retiring的两个量在同一锁内改变，不能出现混合中间态；collect按本次真实各kind数量一次更新。
- stats仅复制Counter一致快照，不拿manager/Slots，不触发poll，不分配/扫描页/描述符表。事件不分配，不反取任何其他锁，不执行GPU或回收。
- 注册持Slots锁，先为订阅TempList做fallible reserve（旧/新allocation重叠按现TempList计费），再checked seed counter，最后add_reserved，防seed成功但订阅失败/丢事件。每heap-counter pair只登记一次。
- 新pool先完成所有page/list元数据准入，最后注册template；之后无新的fallible步骤。append新页先注册再发布到manager，pages.add失败时最后page owner锁外Drop→Slots取消current计数，历史peak不降低；无伪造成功页。
- Counter和订阅metadata均沿现AllocatorStats预算真实计费。全pool累计容量checked add，失败保持原counter不变，不改变Slots状态。数据事件只在已注册容量内变化，所有余额满足capacity=live+retiring+free+exhausted。

## 锁与失败顺序

既有writer Slots→Counter，逐个counter释放后才处理下一个；不同时持多个counter。Counter snapshot从不反取Slots/manager。注册元数据预算锁结束后才取Counter。Pool Drop不持manager guard取Slots；page add失败最后owner锁外drop。原描述符分配错误、写入回滚、lease drop、GPU SyncSet回收、perheap stats与所有manager/group时序不变。

## 回归与实际门槛

同一现bindless.rs内纯Slots回归：错峰两页旧sum16/真实peak8；不经stats读取的短暂高峰仍被记录；多kind同时峰与各kind峰分开；错峰retiring/多队列完成；已有template seed与多个Pool订阅独立；注册OOM原子；page注销/drop只撤current不清peak；并行页读写强制重叠。每步与实际Slots current之和及独立max模型比较，不镜像单个字段赋值。

root唯一Cargo/GPU执行。先现Slots全部tests和bindless GPU/ABI既有回归，再同原world30原live/peak/memory/像素断言。真高水位仍可能因真实预热/在途资源增加而提高，不能承诺新计数必过阈值。steady memory另据exact owner/capture证据，不改测试。
