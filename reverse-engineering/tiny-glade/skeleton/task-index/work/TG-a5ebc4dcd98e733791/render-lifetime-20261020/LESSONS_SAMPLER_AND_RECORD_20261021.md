# 本轮采样器与录制观测经验

- 先按 descriptor kind 与 owner 查生命周期。真实 sampler52→70、retiring0，而 buffer/image相对稳定；源码是(page,name,Sampler Arc)强缓存逐步冷填。不能从总live增长直接断言native Sampler泄漏，也不能用旧弱缓存叙述套当前实现。
- 原B机器码固定13个sampler目录只证明启动静态资源，不证明本工程分页或可选目录预填；恢复原行为与工程适配分别标记。实际目录只登记本App成功安装pipeline的反射owner，不能拿Device全目录或跨App合池。
- 可选缓存不得先抢实际组预算。先完成真实group/FG/参数块，再同guard尝试extras；失败只取消extras。先扩目录再尝试失败后重跑group会再次推进DynamicConstants cursor，Drop参数块不能自动回滚这个历史。
- epoch只在invalidate开始递增不足。逐项锁外Drop删除期间，新prepare可捕获新epoch却借用尚未删缓存并发布完整stamp；最终同锁确认name为空时必须再撤epoch/stamp。精确interleave测试比仅测旧epoch准备更有价值。
- 真实V3世界日志独立观察：frame2..29 sampler72/ret0，8页容量不变；live3042..3048，peak3048不再增长；原30门全部通过。30帧不等于长稳，debug耗时不能外推FPS。
- 计时total必须在任何格式化/输出锁前冻结。并行stderr等待会污染writeln实参中的elapsed。先冻结total、恢复父TLS，再输出；原PGO边界保持。
- inclusive phase、各worker经过时间、frame wall要分开。descriptor验证/完整组/finish包含子阶段不能相加；scope未覆盖的prelude/barriers/chunk调度应作为明确残差，不能硬归driver。
- 关闭探针时callback边界读既有OnceLock；每descriptor只读const TLS活动ID，不反复读全局原子/锁、无新时钟/分配/RefCell。启用后紧循环只累积局部整数、退出一次写TLS，不让探针自身变成新的二次扫描。
- root唯一Cargo/GPU/生产整合者，作者candidate按before/after SHA冻结，独审后执行原测试；失败草稿保留但明确禁止整合。先Rust语法解析不叫编译通过，真实descriptor helper不叫shader执行通过。
