# M01楼梯附着装饰清理恢复

已恢复既有`cull_oob_decorators.rs`的两个原八参入口与共享实现。仅改该楼梯段和追加自己的测试，保留gpu owner恢复的墙体cull与位置validator。无新业务目录，原B路径已先实际读取。

原B源路径`crates/systems/decorator/src/cull_oob_decorators.rs`，Source SHA256 `a337fe462509fa1be95847b4c00151320d2d51fa6749827b6930845d2ced4ba0`；PDB SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`；RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。原入口edge `14080ec10`、invalid `14080f040`；两个单态impl分别`1412b6c30`、`1412b6420`，末尾断言与UD2也在保留ASM范围内。

## 原合同与实际业务

八个SystemParams依次为`History`、可变`DecoratorStorage`、可变`LiveDecoratorArchivist`、真实`DeferredDecoratorOpEvents`、可变`DecoratorBlueprints`、只读`StairsState`（edge）或`PlayermadeStairsAssemblyRequests`（invalid）、`DecoratorArchivalResources`、`CachedDecoratorTransforms`。删去旧假泛型与默认本地事件队列，root已在共享注册面摘掉旧`::<C>`。不创建替代资源。

- 两系统只遍历真实Stair桶，原`iter_stair_decorators@140a1e2c0`52B返回self+120的特殊桶迭代器；不遍历Wall/Terrain。原cull初筛许可失败是跳过，不是旧实现的无条件断言。
- 许可通过后才做原`dst_as_stair`tag4断言，并读取目标前两个u32端点；不读取`info.extra`，不根据装饰ty缩减范围。
- edge保留图中真实边或stub；invalid保留当前装配请求+128中两个端点逐32位严格相等的记录。后者既不是全删，也不是图边存在性，更不能把反向Info端点再规范化以制造匹配。请求的StairSegment自身仍按共享原构造器规范化。
- 原先收集`HashMap<DecoratorLocator, position>`再处理，键保留Stair地址与map id，不能用Info内id替代。缺缓存时原位置为`(-0,-1000,-0)`，没有缺缓存panic或局部位置回退。
- 处理前再次断言许可，先移出蓝图Arc，再经真实归档owner保存首份数据与两张物理map的真实mesh/pose。调用现有apply移除共享权威存储，并写共享136B延迟事件；`!History.is_recording()`在位置载荷+12，原kind_byte保持0。归档资源桥复用现有apply兼容层：带物理资源的首快照先保存，apply的数据桶归档不能覆盖它。
- 保留ResMut包装直到实际写入，空候选/不许可/有效边与有效请求的帧不会误标Storage、Archivist或Blueprints变更。请求扫描保持原线性短路语义，无全量曲线复制或多余排序。

## 原指令与真实World验证

`1412b6420-stair-cull-predicate.py/.txt/.json`：1024个原invalid请求端点扫描，真实执行`140833470`，从`1412b6788`到原保留/删除分支终点，无业务stub。每例把同样的规范化真实请求和Info目标送入完整八参Rust系统，检查存储、蓝图、归档和真实共享事件。8个合法Some/None缓存位置原指令案例含负零、最小正次正规和最大有限值，均验证最终事件位型与recording标记。

本轮6项定向测试全过：原1024案例端到端；原8位置案例×recording两状态；真实边/stub/许可与三存储域；全256个ty与错误dst的收集失败不提前删除；真实physics首归档、下游collider consumer卸载、归档mesh仍独立存活、差分undo/redo与释放；空帧真实owner变更tick不动。

原cached owner拒绝非有限位置；初稿Some NaN夹具触发该正确前置断言，未放宽guard，最终8例改为全部合法原缓存域。不能宣称通过Some NaN的真实World创建路径。

## 工具与流程经验

Ghidra这两个单态的伪C把第二处理阶段标为unreachable并删除；实际原ASM在收集循环之后完整包含许可复查、蓝图移除、History门与apply。不能凭decompiler短方法体下结论，也不为获取漂亮伪C修改共享数据库。实测原指令块的终点须显式断言，避免执行计数耗尽被误当返回。

只有业务范围内的原predicate/位置段做了原侧执行，未执行含原HashMap与线程TLS的整个native主函数；边/stub复用已有真实StairsState叶，不能把1024个请求扫描称作完整二进制主函数等价。实际OS/GPU和原整模块调度由root联合验收。本轮楼梯段无剩余空业务叶；未接手slot14装配/支撑等其它owner。
