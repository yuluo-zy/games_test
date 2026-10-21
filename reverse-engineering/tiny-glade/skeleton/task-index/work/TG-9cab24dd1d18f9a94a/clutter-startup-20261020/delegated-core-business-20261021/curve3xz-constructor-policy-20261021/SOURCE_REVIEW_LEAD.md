# Curve3XZ 必要构造域源码独审

结论：通过本源独审，可与已冻结 interpolate 消费者配对由 root 实际编译测试。原合法旧 claim 保持不变。

依据已批准完整782B合同及原独立702B输出/694B坐标包核源码：生产唯一变动是 S2D 的已证 Vec3XZ 编译期策略；去掉该常量及两行说明后，整个 cfg(test) 前生产前缀与 af00 逐字相同。原纯 f32 XZ 累计、length==0、逐 u 有限、完整 XYZ 有序重复检查保持既有 canonical 实现；不会额外扫描 Y，也不引入私有曲线。Vec2/XYZ/Thresholded/custom 的其它实现与 API body 未改。

三个新增测试覆盖非有限 Y 原位保留/XYZ失败、完整XYZ重复门与NaN/有符号零、TooFew→零长/规范化→重复失败顺序。旧工程 tiny 断言只把现已证的 XZ 构造改为原平方下溢错误，保工程测距与 Thresholded 断言。真实历史 opaque Vec/dirty 语义由 Duck 完整 World 测试包覆盖，utils 不反向依赖核心。

本次独立读取当前生产 before、候选、补丁 SHA，隔离临时树 patch --fuzz=0 精确重放，退出0，无offset/fuzz，最终字节与3275955f一致；生产未改，未执行Cargo/rustc/GPU。这里只接受 B 证据与源码，不将 target C/语法解析/哈希当运行验收。
