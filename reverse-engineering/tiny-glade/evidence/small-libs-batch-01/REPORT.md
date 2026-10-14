# 小型基础库并行恢复：第一批

三个独立 agent 分别负责 motomoto、tg-package-integrity、tg-text，使用独立持久数据库 worker 和源码目录。父 owner 完成接口/规则审阅、依赖锁整合、共享 workspace 检查和进度登记。基线为 B。

## 实际交付

| 库 | 可调用交付范围 | 验证与剩余边界 |
|---|---|---|
| [motomoto](motomoto/BUSINESS_RULES.md) | tag → 写 chunk → Drop 回填长度 → Reader 迭代；byteorder/std IO 复用 | 6 测试、173 原 Reader 指令案例零差异。原 Writer/Drop 执行、完整存档消费者和 schema 待验收 |
| [tg-package-integrity](tg-package-integrity/BUSINESS_RULES.md) | 原清单 bytes 门禁 → JSON/version → 文件校验 → 递归 extra → 聚合结果 | 4 测试、原清单样本门禁通过。实际目录 1792 条目报告 2 corrupt / 288 extra；原入口运行、报告消费者、特殊 OS 边界待验收 |
| [tg-text](tg-text/BUSINESS_RULES.md) | FontTable 配置/lookup/scaled 与 ShapeStream/RawShapeStream CPU 编码 | 6 测试、51 原流编码 + 21 原缩放案例零差异。完整 TextFragment/TextRenderer、glyph/atlas/GPU 未恢复 |

源码位于 [shared-foundation](../../reconstruction/shared-foundation/Cargo.toml)，三个库已加入同一个 workspace。其余共享基础库的源码 owner 边界保留。模块状态为 candidate_partial，完整原库/游戏验收未升级。

## 规则与依赖审阅

- motomoto 保留原 position+16 >= length 的迭代结束条件，以及先消费前次 Drop 错误再转换 tag 的顺序。压缩和 JSON 属于真实存档消费者，库返回 payload slice。
- 完整性门禁为 BLAKE3 → Rust 数组 Hash 的长度前缀 → 指定初值 FNV → 低16位为零；复用原库。只精确跳过清单自身路径，只忽略根 tmp；重复条目仍分别检查。
- 绘制流按原顺序输出 u32 words，保留 index 截断、scaling 编码和 named resolver 调用次序。RawShapeStream::encode 与 Image::with_scaling 的规则合并入 append；字体整形/atlas 工作留给原 cosmic-text/etagere 依赖上的后续游戏策略恢复。
- 三根依赖闭包共22个公共包，版本、来源/commit、checksum均与原锁匹配；见 [依赖审计](integration/dependency-validation.json)。Cargo首选的较新传递包已修正并记录，未手写哈希、JSON、IO容器、路径解析或线程调度器。

## 验证与明确差异

16 项测试、三个库 all-targets Clippy（warnings as errors）、限定 cargo fmt 检查通过。后续补查的 schema 失败测试已单独复验文本库；未扩张原机器码案例数量。原 manifest 与字体表 fixture 逐字节匹配发行文件。

完整性候选以64文件一批调用 easy-parallel，以限制线程峰值；稳定文件系统的规则与顺序保持，文件并发修改时的时序差异仍公开登记。原库使用 CWD，候选使用显式 root 参数；类型表示为清楚的 Rust 接口，未声称原 ABI 或作者源码。

工具失败原样保留：motomoto 的 Writer 批量反编译未完成；文本库某次查询的 checkpoint 失败，后续独立 schema/checkpoint 成功。原指令/静态/候选证据各自按范围有效，失败 job 没有被升级为通过。三个专属 worker 已正常停止，overlay 未同步原数据库。

原入口运行、真实游戏消费者接线、A/C适配与完整游戏验收继续登记为缺口。总进度中的两条既有 country-core 缺失路径记录来自其他任务，本批未新增此类问题。

[整合结果与源码输入哈希](integration/summary.json) · [独立审阅](integration/source-review.json) · [模块进度](../../MODULE_PROGRESS.md)
