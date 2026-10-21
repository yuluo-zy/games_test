# V2 候选交接

V1 预算设计被独审阻断，不能整合。V2 采用已审 fc72c52b... 补充，实际组 finish 后才尝试可选目录 extras。pipeline 仍按原时机 Arc::new，只在原 table.try_reserve 后增加无错误返回的可选登记；保留 root 新 create_buffer 的读回初始化六行，patch 不涉及该函数。

工程适配：新缓存主动保留已成功安装 pipeline 的静态 sampler 目录，可能增加初次页面的少量 descriptor/CPU 常驻缓存，不能称全生命周期内存或所有错误机会完全等价。这里保证当前实际组先完成其原预算准入，不被同次 extras 抢占预算；extra 非预算错误仍可新增失败机会且必须暴露。后续新管线/自定义版本变化导致合理增长仍允许，原 30 阈值/采样/真实业务均未改。

生产闭包仅原 render_host.rs：私有 SamplerPages 状态、Snapshot::prepare finish 后单调用、compute/raster 安装后登记；未新建平行 renderer。所有新增注释中文。目录 App 所有权不跨 App/Device；页 stamp 用目录 Arc 与 epoch；lookup/commit 同 (name,ArcSampler) key；invalidate 移除同名所有版本且锁外 Drop。

测试共7项：6项会真实创建 headless Device/descriptor heap/sampler lease（严格 RHAPSODY_REQUIRE_VULKAN=1 不能 SKIP）；1项纯错误分类。helpers 测试的是私有 descriptor cache 与取消，不执行 shader/完整 BindlessParameters.finish。生产 actual finish→extras 顺序由差异和 root 现有 ABI/bridge/raster/world 验证，不夸大 helper 为并行 guard 或世界全链覆盖。metadata 登记错误使用仅 cfg(test) 本 App 一次故障注入，生产没有故障入口。

作者仅 rustfmt 语法解析及只读源码审；未 Cargo/GPU/Git/生产写。root 建议执行：cargo test -p tiny-glade --lib sampler_catalog_tests --no-run，随后沙箱外 RHAPSODY_REQUIRE_VULKAN=1 + Khronos validation 的相应实际二进制 --test-threads=1；再既有 ABI/bridge 和原 world30 保全部断言。root 当前还合了 readback-policy，不把这轮运行冒称严格只有 sampler 一处变量。
