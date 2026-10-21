# RecordTally Clone 义务修复 V2

真实 root 编译 /private/tmp/tg-cache-sparse-tree-cpu-20261021.log 的 E0277 被保留。V1 漏核 covers_iter 的 Iterator + Clone 义务；本补丁替代 V1 对应三源，其他三源保持已合原候选。作者不 Cargo/GPU/生产/Git。

covers_iter(rhapsody/rg/resource_contracts.rs:119)先 writes.clone() 测 contains，然后为区间并集/图像slab继续多次clone真实惰性遍历。V1 filter捕获 &mut RecordTally，不可Clone；不能给Tally派生Clone而复制计数，也不能提前收集range改验证成本与错误次序。

本修仅将同一栈上RecordTally的两个u64改Cell<u64>，add(&self)在启用时get/set，filter捕获共享 &RecordTally，这种引用本身可Clone。所有clone读取仍累加同两个局部Cell；covers_iter签名/算法/输入顺序完全不变，不新增unsafe/Sync/Send/RefCell/TLS紧循环写。Tally仍!Send；所有iterator clone在本函数消费后销毁，Drop只读取两个Cell一次汇总进同scope TLS。关闭时add仅检查局部id==0，不读写Cell或TLS、不新增原子/锁/时钟/分配。

bindless覆盖filter和resource_requirements两调用点只去掉不再需要的mut；profiling原parallel测试同改不可变binding，原四项验证逻辑保留。off测试新增Cell保持0断言；新增第五项直接调用真实covers_iter，两个相邻[0,4)/[4,8)覆盖[0,8)需原3轮clone共6个range/15个semantics比较，短[0,4)只读取首项1/2，验证clone共享计数与惰性早退，同时类型编译义务不再遗漏。

root建议实际cargo test -p rhapsody --lib record_trace_tests（现5项），保真实编译failure日志，再继续优化构建on/off同输入；纯测试不算游戏业务恢复。作者只有rustfmt语法解析与原Clone源码静读；不能称编译已通过。
