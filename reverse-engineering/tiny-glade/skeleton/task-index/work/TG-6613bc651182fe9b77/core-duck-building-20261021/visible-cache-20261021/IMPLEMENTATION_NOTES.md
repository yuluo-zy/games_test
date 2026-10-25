# 完整cache源码交付

完整实现批准的9331B业务域，复用当前唯一owner、Curve/旋转/屋顶/地形/ladder内核。生产循环无每帧entries收集或第二缓存，terrain/stair逐项插入成功才修改原bucket.modified_ids；壁/屋顶不新增比较。原无请求负零/负100/scaleZERO仍参与缓存比较。

新增测试覆盖真实Bevy World、真实Local双帧、实际资源/Query/物理terrain采样、roof63懒读、stair全部kind、原有序匹配/首条请求、子段耗尽start、数组/Option必需门、负signed step及SSE数值边界。测试夹具中未消费的ClutterMeshes槽使用原demo omission构造，不以该省略资源代替被测几何。

仅rustfmt语法解析与正反F0重放成功；未Cargo、Rust编译或GPU运行。Info63真实producer/宿主注册/可见mesh由其它闭包继续，不能从本源码候选推断全链完成。Rust共享panic诊断保有含义，未宣称原字符串/NaN payload/跨ISA逐位全等。
