# 五个基础包重构与验收（2026-10-17）

> 本页为首轮重构记录。后续重新审查又修复了遗漏问题，当前发现与验收见 [第二轮复审](../foundation-rereview-2026-10-17/README.md)。

范围：lang-lib、asset、asset-pipe、motomoto、paths。仅做源码重构、正确性修复及现有数据测试，没有开展逆向。已将“发现的错误不会因历史行为或兼容测试保留”同步给全部 agent，并对交付代码追加独立复核。

## 结构与行为

| 包 | 结果 |
| --- | --- |
| motomoto | 按 tag、reader、writer 分层。Reader 使用剩余切片零拷贝解析，错误后终止；Writer 显式完成 chunk，任何实际 I/O 失败后永久停止续写，Drop 只作兜底。 |
| asset | 配置继承新增 fallible 接口；扫描完整验证 prefab 必填字段，报告源文件/part/字段。纹理扫描检测重名且不消耗覆盖配置。归档结构验证支持对齐借用与错位回退。 |
| asset-pipe | 分为 image、dds、array、layout、cache、compile、output。有限工作队列、逐层数组处理、受检布局、完整缓存校验、单文件原子发布。 |
| lang-lib | 分离 service/快照、global、preference、missing_cache，保留 ArcSwap 查询与加载事务；完善命名和错误/所有权说明。 |
| paths | 分离目录模型、平台策略与全局发布，保持独立实例和一次性全局初始化；失败可重试，重复初始化无新增副作用。 |

## 已修复问题

- chunk 异常长度 panic、末尾空 chunk 丢失、残缺头被静默忽略、失败后仍续写，以及回填位置 wrapping。
- 纹理同名静默覆盖及配置丢失；逻辑名称虽不同但输出路径相同（如 a 与 a.detail）时，在发布任务前返回错误，避免并发覆盖。保留现有 runtime 扩展名替换约定。
- prefab 缺少必填 render/RT/audio 字段时 panic；现在在扫描阶段返回上下文完整的错误，删除指令不要求渲染字段。
- 数组 DDS 被错误交给普通图片解码；DDS 声明多层却仅复制第一层；非法维度、mip 数、payload 长度及尺寸溢出。
- 缓存不足8字节直接阻断编译、仅校验 magic/version 导致截断归档命中、结构正常但像素已损坏仍命中。现在使用源指纹、pipeline revision、输出内容哈希和 archive 结构校验。
- 新源目录缺少初始 list 时无法通过 compile_or_load 编译；输出直接覆盖导致中断留下半文件。
- sRGB mip 在 transfer 前量化到8位导致暗部丢失；straight-alpha 图像直接滤波导致透明边缘暗化。现在保留线性浮点精度，并在滤波时预乘/反预乘 alpha。
- 公共 padding 的空图、缩小目标、尺寸乘积溢出，以及数组层元数据相同但实际像素数据不完整。

以上包含旧测试刻意固定的错误行为；这些测试已改成正确性回归。未发现确证问题的 lang-lib/paths 保持业务行为，无额外功能扩张。

## 兼容与迁移

- `.texture` 的 magic、版本8及 rkyv 字段布局不变，388份现有 archive 可字节一致往返；mip、DDS 输出内容因正确性修复允许变化。
- chunk tag+length 小端格式不变；空段和损坏尾部的读取行为已修正。
- RON/YAML schema、继承优先级及语言全局入口保持。新增 `try_*` 为外部配置提供错误返回；原已解决值 convenience getter 仍有明确前置条件。
- `pad_edges` 改为 `Result<RgbaImage>`，仓库调用方已迁移；`cache_is_current` 要求完整文件字节。
- `TextureAssetMeta` 新增 `pipeline_revision` 与 `output_hash`，旧 metadata 缺省读取并重建一次；外部手工构造此公开结构需补字段。
- 不升级依赖，移除 asset-pipe 不再使用的 easy-parallel 直接依赖。工作区原有其他变更保留。

## 验证

在 macOS 运行五包合并测试，并对最后追加的图像/缓存修复做 asset-pipe 定向回归：共69项通过（asset 20、asset-pipe 18、lang-lib 13、motomoto 10、paths 8）。没有为每个内部 helper 新增镜像测试；错误组合尽量并入表驱动用例，保留并发、故障注入和实际文件往返。

```sh
cargo test --offline -p paths -p lang-lib -p asset -p asset-pipe -p motomoto --all-targets
cargo clippy --offline -p paths -p lang-lib -p asset -p asset-pipe -p motomoto --all-targets --no-deps -- -D warnings
cargo fmt -p paths -p lang-lib -p asset -p asset-pipe -p motomoto -- --check
cargo check --offline -p tg-iced --all-targets
```

验证使用独立临时 target 目录，避免占用现有共享构建目录。严格 Clippy、包级格式检查和直接调用方 tg-iced 默认特性编译通过。原生 GUI 可选特性及完整游戏启动不在此次验证结果内。intel_tex_2 预编译 kernel_ispc.o 仍有平台 load command 链接警告；链接与测试通过，不修改依赖二进制来隐藏警告。

## 性能证据与取舍

- Reader 直接借用 chunk payload，无复制或分配。
- 512×512 linear BC7 入口没有新增整张 RGBA 分配；无 mip 的未压缩 linear 输出转移原 Vec，验证指针一致。
- 对齐 archive 结构验证直接借用输入，只有错位输入才分配 AlignedVec。
- 数组逐层处理；编译最多8个并发工作任务，避免每个源文件启动一个线程。该上限不等于整个进程硬内存预算。
- 语言探针实际运行：10000次翻译命中与已缓存缺失查询均0新增分配；1000次包生命周期净保留0字节；五种隔离环境策略通过。

可重复诊断：`cargo run --offline -p asset-pipe --example benchmark_pipeline`。32张256×256、compression=None，fixture生成不计时；热缓存取5次中位数。开发构建的单组观测：

| 版本 | 冷编译 | 热缓存中位数 |
| --- | --- | --- |
| 重构前 | 358130 µs | 6237 µs |
| 重构后（完整输出校验） | 317103 µs | 41298 µs |

这些是在共享机器开发构建上的诊断样本，不是受控吞吐量基准。热缓存现在读取并哈希完整输出，承担明确的正确性成本，不能宣称所有路径更快。优化成果以可复验的零拷贝/分配与并发上限为证据。

Windows/Linux 真机、完整游戏消费、断电持久化没有在本轮验证。批量编译的多个文件不是一笔事务；单文件原子替换、任务阶段失败不发布索引、内容验证重试共同提供可恢复语义，不承诺快照隔离。
