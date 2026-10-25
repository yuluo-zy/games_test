# asset / asset-pipe 独立 review

本轮审查了两个包的 12 个实现文件（含合成模块入口）和 6 个测试文件，并复核 B 原 PDB 接口、限定反编译、继承差分与原依赖身份。发现并修复 **1 项实测性能缺陷**；未确认新的原业务逻辑偏差。两个模块的功能和原调用链仍未完整验收，不能把通过测试理解为没有其他缺陷。

| 编号 | 严重度 / 状态 | 发现与证据 | 处理与验证 |
|---|---|---|---|
| AR-01 | P2，已修复 | 线性 RGBA8 压缩前，`decoded.to_rgba8()` 与 `with_rgba8().to_rgba8()` 各复制整幅图。原 `with_rgba8` 线性压缩分支直接传原缓冲到 Intel compressor。 | 消费已拥有 RGBA8，使用 `Cow` 借用线性视图；mip 输出转换使用 `into_rgba8`。512×512 BC7 探针修复前 2 次 1 MiB 像素分配，修复后 0 次；输出与原版本 Intel 依赖直接压缩逐字节相同。 |
| AR-02 | P1，已知功能缺口 | 61 个原接口中仍有 19 个未触及，包括 prefab 深层 `concrete_default` / 继承边界。局部 RT/audio 配置可能保留未解析字段；当前扫描解析成功并不证明 consumer 能获得完整具体值。 | 不猜测默认 scale/albedo 或 opaque impl owner。保留逐接口缺口，需结合原默认值与 consumer 恢复。此项是完整性缺口，尚未证明一个新增原逻辑偏差。 |
| AR-03 | P1，验证缺口 | 原图片输入缺失，未完成完整像素、mip、BC 压缩 producer 到发行输出的对照；country-core consumer 未接线。 | 388 个发行 archive 的解档/逐字节回写与 synthetic PNG/DDS 测试继续通过，范围不扩张成原整链一致。 |
| AR-04 | P2，适配范围限制 | `deserialize_inheritable_option` 虽为泛型 serde 签名，兼容实现依赖 RON `RawValue` + 新 RON decoder，要求 `DeserializeOwned`。其他格式、借用值和外层 extension 继承未验证。 | 原当前已观察 caller 是 RON，发行输入、裸值/Some/None/unit 均通过。不能推断原游戏一定使用其他格式，也不能声明此 adapter 是通用 serde 完整替代。 |

性能探针测的是 Rust 分配次数及累计缓冲字节：此路径减少了 2 MiB 的分配与复制；没有声称峰值 RSS 减少 2 MiB或取得某个耗时倍数。原 ISPC 内部分配不在该计数器内。mip 的消费转换减少复制基于源码与原 image API，未另报耗时数字。

另保留三项性能/缓存风险，均未当作新的候选实现偏差修复：原 `easy-parallel::each(...).run()` 给每个 descriptor 一个任务，大图片批次可能产生并发内存和线程压力；原数组缓存观察目录 mtime，编辑既有 layer 内容可能不改变目录 mtime；原合法八字节 cache header 允许缺失 payload。加入线程池限额、递归 mtime 或强制 payload 检查会改变已观察原规则。安全 archive loader 还保留输入、对齐副本及 owned payload 三部分内存，实际运行时预算未测。

未发现这些实现重新编写通用解析、解码、滤波、压缩、容器或线程池算法。`image` / Intel compressor / `ddsfile` / RON / serde / RKYV / `easy-parallel` / std / `relative-path` 均直接复用；当前两个包可达的 **49 个外部包** version/source/checksum 与原 lock 精确匹配。游戏私有继承、transfer 常量、RGB/alpha 选择、压缩资格、mip/边缘填充、扫描/缓存及任务结果策略应保留。该结论仅覆盖本轮两个包，不覆盖其他历史还原目录；原完整 feature set 仍未确认。

复验：两个包 **20 项测试通过**（asset 9，asset-pipe 11），含发行物、继承、名字借用、数组/DDS/cache、错误/panic 全任务 join 和新分配回归；all-targets strict Clippy 通过。旧差分报告原 hash 原样保留；当前源码 hash、依赖身份与验证范围见 [review.json](review.json) / [validation.json](validation.json)。本轮没有增加已分析接口数，也没有新增原游戏运行时验收。
