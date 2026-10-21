# v2 文档勘误范围

按 lead 本轮独审要求，只纠正 DecoratorInfo 顶部、两个字段及两个 raw accessor 的既有错误说明：+47 保留原 flags，insert 的2是返回 Option 的 None niche；+2c保留旧字段名和旧包装访问器但不是独立map key，完整业务语义未知。所有 raw accessor、raw state2 测试和其它函数体不变；逐行过滤 `///` 后 v1/v2 代码完全一致。v1冻结不改，FROM_V1.patch为纯文档增量，candidate.patch从原生产8c2473构造完整最终候选。
