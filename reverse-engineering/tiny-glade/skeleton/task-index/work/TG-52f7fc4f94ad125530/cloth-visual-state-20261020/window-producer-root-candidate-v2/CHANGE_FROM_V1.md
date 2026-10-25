v1冻结后自审发现测试夹具函数pub(super)仅对墙窗模块可见，角窗测试不能调用；v2仅将两个cfg(test)夹具函数可见性改为pub(in crate::decorator_visual)。生产算法、18项测试断言均不改。system-decorator的country-core依赖始终启用native-physics，因此真实cuboid夹具不需要虚构替代或额外feature。
