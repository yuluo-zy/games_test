# 五个基础包源码审查（2026-10-17）

> 本页保留修复前的审查快照。下列问题已在后续重构中修复，当前实现与验证见 [重构报告](../2026-10-17-foundation-refactor/README.md)。

范围：用户列出的 lang-lib、asset、asset-pipe、motomoto、paths 共五包。仅审查当前源码与运行现有测试/针对性探针，没有执行逆向。本轮只更新文档，没有修复实现。各包 README 记录业务、入口、处理流程与尚缺能力。

## 发现（按修复优先级）

以下均为当前源码缺陷或已知兼容行为，不表示本轮引入回归；P2 表示应计划修复的功能问题。

| 优先级 | 位置 | 触发与影响 | 建议 |
| --- | --- | --- | --- |
| P2 | `crates/motomoto/src/lib.rs:127`、`:135` | chunk 长度为 `u64::MAX` 时被转换为 -1，seek 回退后生成反向切片，`try_next()` panic，无法通过返回的 `io::Result` 处理损坏输入。17 字节即可触发；现有测试明确保留这一行为。 | 使用 checked 长度运算，切片前验证 start <= end <= data.len()，损坏数据返回错误。 |
| P2 | `crates/asset/src/texture/scan.rs:82`、`:84` | 同目录 `a.png` 和 `a.jpg` 都归一化为 `a`，HashMap 静默覆盖；配置由 remove 取走，第二个条目使用默认值。最终源文件取决于 read_dir 顺序，可能丢失指定的 sRGB 等配置。 | 插入前检测重复逻辑名并报错，避免随机选择与配置消耗。 |
| P2 | `crates/asset-pipe/src/texture.rs:352`–`:357` | scanner 接受 `array_*/0.dds`，但数组分支固定调用普通图片解码；image 的 DDS 路径不受支持，合法 DDS 层无法像单张 DDS 一样处理。 | 为数组层使用支持 DDS 的分派，或在扫描时明确拒绝该组合。 |
| P2 | `crates/asset-pipe/src/texture.rs:423`–`:429` | 已存在 metadata 且源不变，但缓存被截断到不足8字节时，read_exact 直接报错，编译入口不重建可再生缓存。 | 把无效或不完整缓存视为 miss，重建后再发布。 |
| P2 | `crates/motomoto/src/lib.rs:122` | Writer 可生成最后一个空 chunk，但 reader 的 start >= len 判断会静默忽略它；单空 chunk 往返丢失，末尾空标记不可用。现有测试也保留此兼容行为。 | 接受恰好包含16字节头的空 chunk，另行明确残缺头处理策略。 |

## 完善程度

| 包 | 当前结论 |
| --- | --- |
| lang-lib | 语言包、回退、快照切换、偏好读写形成独立闭环；未发现确证功能 bug。未提供插值/复数等高级本地化能力，实际 UI 接线与多平台替换行为仍需验证。 |
| asset | 配置结构、继承、扫描、纹理 archive 可用；重复逻辑名检测存在缺口。它是资源数据层，不负责 GPU 上传或整个资源运行时。 |
| asset-pipe | 扫描、图片处理、压缩、缓存及索引形成流水线；DDS 数组与损坏缓存恢复存在问题。缓存只验证 magic/version，不验证完整 payload；只保留正确8字节头也可能被复用，这是现有测试明确的合同限制。输出资源及两个索引文件不构成跨文件事务。 |
| motomoto | chunk 编解码、流式回填、延迟 I/O 错误传播可用；错误长度与空 chunk 边界不完善。当前未发现其他 crate 的源码 consumer 接线。 |
| paths | 目录策略、独立实例、全局一次初始化、失败重试及并发保护可用；未发现确证包内 bug。全局不支持换账户，便捷 getter 要求先成功初始化；不负责存档事务。 |

## 验证

macOS 执行：

```sh
cargo test --offline -p paths -p lang-lib -p asset -p asset-pipe -p motomoto --all-targets
```

退出码0，共63项测试通过：asset 15、asset-pipe 15、lang-lib 13、motomoto 12、paths 8。intel_tex_2 的 kernel_ispc.o 出现缺少 platform load command 的链接警告，没有阻断测试。测试通过包含对历史边界行为的断言，不能用其否认上述问题。

lang-lib 另实际运行缺失缓存和环境策略 example：10000次缓存查询新增分配0、1000次包加载释放净保留0字节，五种隔离进程模式通过。针对性临时探针确认 asset 同名覆盖/配置丢失及 asset-pipe 短缓存报错。其余复现细节与补充验证见各包 README。

本轮未验证 Windows/Linux 真机、游戏完整接线或 GPU 消费。历史来源链接仅保留为文档背景，不代表本轮重新进行原版等价验收。
