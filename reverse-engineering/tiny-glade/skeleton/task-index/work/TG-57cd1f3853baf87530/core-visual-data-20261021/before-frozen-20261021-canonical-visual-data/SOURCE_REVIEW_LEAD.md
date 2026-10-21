# 七源canonical碰撞调用接口独立源码审查

core_business_lead，2026-10-21。结论：本七源最窄typed owner/Arc/pose调用迁移通过，implemented_unverified，供root与canonical四表同组后实际编译；不宣称各visual完整原业务或真实主App闭环已恢复。

已读取最终七份源码diff、实际ECS夹具、原body调用点合同、现canonical View/Blueprint setter接口；候选及patch SHA与seven-source-delivery.json逐项实算一致。6库消费者从同一ID表required subtype→真实Info.rank→碰撞和交互各自表借用→仅Arc clone→真实cache translation/rotation，不Vec clone、不rank0、不把两不同表合成一份。lantern先16..18 subtype门才读extra/cache，消除对非灯笼as_lantern及ZERO位姿的错误。gothic门stairs借用真实bounds。

roof烟囱确为已证动态cuboid例外：Y高度extra+.1、宽深.85、零localpose、世界cached.y+height*.5、同原Arc喂两槽，与库Sphere无关。既有几何断言按原Y轴修正，新增真实World空mesh库仍真cuboid测试验证该来源；cottage门实际World双不同shape/Arc+rank2+非零cache/rotation测试验证两槽身份，并保missing ID/interaction required失败。测试尚未Cargo。

当前generate其它offset/hole/boxedpayload/outline/visual builder仍有旧部分实现，本包不改其分支、不把本次API改型当作这些算法完成。特别legacyblueprint metadata/存储来源/视觉实体完整生成和真实宿主消费需要各自完整原合同；不能仅凭两typed槽已填就称全视觉业务可用。wall/trap/cottage_dormer动态几何仍明确组外必要阻点，不能用这七源绕过其编译或插默认shape。root保持单一生产/Cargo/GPU整合，全部before/after以七源manifest为准。

## 后续真实consumer复核：v1不具挂载条件

进一步核blueprint.rs实际update_collisions发现，两个Mesh槽非空后required storage_addr（421/445）；本v1 legacyinsert仍None，原entry mutation/flags未接，实际consumer会panic。故上述源码审查仅保留typed Arc数据局部结论，**本v1不能整合挂载为可运行闭包**。必须随后v2逐原1408258b0恢复insert_for真实StorageAddr/第五参flags与副作用时序，再独审真实consumer。旧freeze保留不覆，不能通过默认source掩盖此缺口。
