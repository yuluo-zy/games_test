# Locator独立ID五源最小闭包源码独审

通过，implemented_unverified。原15B new/1468B apply证据与合同已审。逐hunk核：Locator Eq/Hash自然包含addr+独立u32；现代Op显式DecoratorId，from_change取patch.id；place/change/typed三history集合producer均真实ID，None目的沿源/Some目标；storage和既有legacy事件key同字段，typed仍patch.id。全部真实构造 external命令ID、cull两处循环ID、chimney实际lookup localID与测试helper均迁移，无Info.word2c回推/no bucket scan。window_extra body不变。Info v2仅doc勘误已独立过滤文档后逐字比较不变，保raw2 accessor测试、不mask。

5真实owner新增测试有意义：同墙两ID、重复集合与records增长、remove无locator、目标/沿源、typed变word不变key、失败前history已写且store/events未提交。已查实际lookup_storage/unleash/patch API，尚未Cargo。保原复杂字段应用/旧codec与rawunion既有边界，不把所有历史/烟囱UI/place producer称为恢复。

4 owned源及当前cull两个literal独立F0/hash均过。最小包不必等待完整validator：cull只两行，不mount旧cull host；peer唯一完整cull writer保持V4冻结，后续由peer对新生产base重基。禁止先套minimal两行又套旧V4 fullpatch重复覆盖。

编译、5测试及实际业务由root统一执行，未C等价证明。
