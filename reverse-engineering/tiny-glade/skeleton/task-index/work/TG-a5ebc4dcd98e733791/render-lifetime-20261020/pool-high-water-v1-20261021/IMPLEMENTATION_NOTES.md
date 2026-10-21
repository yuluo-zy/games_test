# 实现边界

1. 当前仅一个既有源bindless.rs候选，无production写入/Cargo/GPU。原单heap统计历史与所有槽/lease/SyncSet/manager/group算法保持；只增加整池counter的订阅和事件计数。
2. Counter当前值及高水位在原Slots锁内的单短Counter锁处线性化，多个页并行事件只取一个counter锁，无嵌套；非池heap为Option::None快路。stats一次锁复制，无扫描/分配。高水位从池登记seed开始，从不回退；perkind同义。
3. 强Arc counter无反向owner引用，Pool Drop逐页unwatch；新页注册后manager.add失败，Slots Drop撤当前；同页重复unwatch不双扣。TempList订阅分配与Counter Arc元数据预算已计费，纯事件不分配。未改原错误/生产行为。
4. 新7项纯Slots回归为实际不同时间/多页/多kind/无stats采样/多队列/多pool/注册OOM/Drop/双线程输入，不依赖修改高水位阈值。root执行现bindless::tests及GPU ABI/原world30。
5. 先前真实sharedconstants30帧CPU/GPU全部执行完成，live已通过，旧peak因sum语义失败。新真实peak仍可能暴露实际预热/在途需求，必须等实际world，不能预宣称长稳通过。
6. memory预查：CAPTURES=[1,2,3,4,9,19,29]，原clean>=12为12,13..18,21..28。实际log rounded first12=922.4/max922.4/last28=921.9；constants4pages/4creations固定。alloc05/29 propsf 256KiB各10、512KiB各2固定，518400B screenshot buffer 2→1；原remaining3生命周期导致capture+2/+3仍可有该读回owner。不得将rounding推为exact memory断言已通过或擅改clean/阈值/GC。
