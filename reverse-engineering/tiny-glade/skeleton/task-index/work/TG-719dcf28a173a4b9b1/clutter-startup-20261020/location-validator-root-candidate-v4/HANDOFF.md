# location validator V4：独立装饰 ID 直接调用迁移

保留 V3 冻结及其完整原驱动业务。只在两个 REMOVE operation literal 写入各实际循环 `DecoratorId(id)`，适配 core lead/tree 唯一 canonical `DecoratorOp.decorator_id` 新字段；不读 Info.word2c，不扫描桶找 ID。V3 已独审报告 LOCATION_VALIDATOR_SOURCE_V3_REVIEW_V1.json 保持其原审查边界，新增两行另审。

必须与 canonical Op/Locator 配对应用；V4 不单独令尚未迁移的 production 可编译。不挂载旧 cull 系统，不宣称完整归档/feedback 已完成。十个源码测试保持不变，未执行 Cargo/rustc/GPU。原 before 与 V3 相同，V3_TO_V4.patch 适用于已整合 V3，SOURCE.patch 适用于原 before，禁止重复应用。
