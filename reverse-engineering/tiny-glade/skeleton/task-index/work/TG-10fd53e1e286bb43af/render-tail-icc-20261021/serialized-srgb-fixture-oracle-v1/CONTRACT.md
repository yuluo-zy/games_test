# 序列化 sRGB 测试基准修正 V1

生产函数（cfg(test) 之前）逐字不变；只修复两处错误 fixture 预期。

原版 B update_window_icc_profile@0x14093d830 是 Profile::new_file → new_srgb → RGBA16/Perceptual/flags0，当前实现保持此链。失败测试把 Profile::new_srgb().icc() 写盘再重读，却假设与未序列化内存 profile 数值恒等。固定版本 vendor2.15 中 XYZ 和 parametric TRC 的 s15Fixed16 存储使该假设错误。

root 独立执行 C 公共 API 参考（不是原目标程序执行），精确结果：memory→memory 32768 像素恒等，FNV1a64 小端 7129997588855b45；memory→serialized FNV a4368867549a25f2，34819 RGB 通道改变，逐通道最大差 [31,11,13]，像素31=[65535,0,6,255]。测试对全网格哈希、逐通道精确最大值、改变计数、alpha 和该点作精确断言，未放宽任何 tolerance。

参考生成使用与 Rust 相同固定源 LCMS 2.15 原生 archive，但独立 C API 路径。原 B 机器码证明调用合同，不声称 Windows 原机的所有浮点边缘与本机参考逐位一致；该基准绑定本项目固定依赖和此网格输入。不同平台若不一致必须查真实依赖/浮点路径，不自动放宽。

root-only 实际验证待执行：country-core ICC 8 tests。作者未 Cargo/编译/GPU。实际先前失败与 C 输出均保留。
