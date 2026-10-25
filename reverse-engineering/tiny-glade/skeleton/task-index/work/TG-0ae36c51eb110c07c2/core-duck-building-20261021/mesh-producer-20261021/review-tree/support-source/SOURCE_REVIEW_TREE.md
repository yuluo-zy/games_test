# 支撑/不支撑分段源码独立复核

结论：批准40109881→1b6c227d的窄候选进入root验收。作者非本人；未改作者源码，未Cargo/GPU。

独立对冻结before/candidate/patch哈希，F0正向和逆向均精确复现。完整3075B原合同已本人独立读660原指令并逐PE字节核。实际修改仅恢复单输出空壳为真实三项输出，并增加七测试。

源码逐转移与原一致：先width0，内部点wall_normal→tangent→point→width；有序方向符号、finite正reciprocal/零fallback、原乘加关联/常量位/TOI1.78/mask1；hit实际position计算侧距；hit↔miss共享边界、单点supported段、分支width runs、最后实际widths.last。没有私有world、fakehit、强加等长校验、clamp或事件默认成功。循环O(n)，仅真实输出分配。

七测试确实建立RaycastWorld，真实cuboid+upsert/flush，再调用业务入口；覆盖全hit/allmiss/hit-miss-hit、Slot1被mask排除、端点不测、单/双点/额外尾width、zero/NaN/Inf tangent和必要bounds失败序。距离断言容差只用于Rapier几何命中浮点，区间/长度与width endpoint精确断言。fixtures的UiSignifierStream只为原未读参数，没有默认资源进入生产。短TOI的现wallfront1.4确在1.78内，但七测试尚无1.78外的专门反例；现明确literal已审正确，不构成源码阻断，root可按实际失败追加。

静态核pinned接口：system-decorator启用country-core/native-physics，RaycastWorld查询/真实upsert/NewCuboid/flush、StairWallAnchor::wall_normal、UiSignifierStream::default当前都存在，未见静态类型断点。该方法目前生产两个62KB主body尚未调用，不把叶源码或七CPU测试称宿主已接通。root应执行并保真实失败，后续主body必须消费三输出。
