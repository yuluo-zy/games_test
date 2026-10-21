# 607B必要方向叶合同独审

通过，允许在当前8011真实before之后实现此公共leaf+边界测试，不改已提交bad/factor/其它lib。原607B完整ASM门、初次输入normalize_or(+X)、right二次normalize、独立翻right/forward形成(-right,Y,-forward)，以及矩阵→Quat四分支与pinned glam对应可复用。输入零/NaN/Inf/平方上下溢均归原+X fallback，不能替bad(-raw)或先归一再重复bad。

依赖glam0.29.3 Vec2.try_normalize 与现bad复用同初始门，Mat3/Quat整构件复用依赖，不手写yaw。测试应有axis/diagonal几何、各fallback输入与原+X路径比特、单位Y及-Z方向；不把几何近似断言称原target运行位等价。原有效矩阵有限；glam断言配置及跨架构NaN payload限制保留。

审者独读607全部原ASM及真实source/glam路径、当前8011依赖；合同身份/字段/必要call点完整。未写生产、未Cargo/GPU，源码实现还须独立复核。
