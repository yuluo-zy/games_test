# 实现和验收边界

- 三既有生产源只接已有常量 owner；第四个候选是既有 core_render_bridge 测试增量。没改 rhapsody raw allocator、range准备、descriptor角色或GC预算。
- RasterShared.dynamic已为Arc，字段原样；原dynamic_constants借用原样。新owner只Arc设备身份检查+Arc clone，无锁/分配。
- GraphPasses独立new语义保留；生产new_with_app_constants先保DeviceLost早返回，校已有owner，from_parts仍为每声明创建独立空uniform_uploads Vec。
- 新回归通过真实CoreGraphFrame::with_compute与真实mapped buffer分配检验owner；测试本身不提交GPU shader，不能替代原世界private参数/declared range混合实测。现已有core staging copy和原world测试继续覆盖真实提交。
- 需要root唯一执行：core_render_bridge全test；现render_host上传/参数/BDA回归；rhapsody raw dynamic_constants原tests（CPU/Weak/多队列/used-page seal/flush）；实际1478private读取原世界probe和30帧原断言，不改阈值。
- 真实world观察共享池页/分配/reuse、物理cells、pending retirement及内存分类，区分修复不必要池与GC吞吐。禁止以pool接通宣称长稳通过。
