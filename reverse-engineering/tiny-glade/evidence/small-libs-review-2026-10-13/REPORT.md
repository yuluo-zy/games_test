# 三个基础库独立并行 review（2026-10-13）

三个新 reviewer 分别审查 motomoto、tg-package-integrity、tg-text；父 owner 交叉审阅原证据、实际消费者搜索、源码输入哈希并合成结论。对照基线是 B Windows x86_64。

## 结论

本轮已恢复的核心范围没有发现已确认的业务算法缺陷或公共依赖重复实现。确认了 1 处证据/注释错误，已修正。完整功能仍不完善，不能因库内测试通过、函数名字都有映射就升级整库/游戏验收。

| 模块 | 实现审查 | 业务完整性 | 性能审查 |
|---|---|---|---|
| [motomoto](motomoto/REVIEW.md) | 分块/tag/回填/错误顺序与原证据匹配；新增限定 Writer/Drop 与错误后 Reader 原对照通过 | 真实存档 producer/consumer、schema/压缩/迁移未接；泛型 flush/IO失败原合同仍有未知 | Reader 借用 payload，无额外复制；File直写的多次Seek需真实consumer测量，当前无确认回退 |
| [tg-package-integrity](tg-package-integrity/REVIEW.md) | 当前x64 checksum、self排除、失败分类、重复条目、tmp/Unicode/junction规则未发现错误 | 完整verify原入口、极端schema、真实report/crash消费者未验收或接线 | 64批仍逐文件新建线程并整块读取；已量化成本，未证明原侧性能回归 |
| [tg-text](tg-text/REVIEW.md) | 已恢复字体表/shape CPU规则无确认代码缺陷；原 String::deserialize 的误述已修正 | 完整TextFragment/TextRenderer、字体整形/cache/atlas/GPU与真正纹理resolver尚缺 | append线性工作；clear后已有容量复用，probe零新增分配；无原完整渲染性能结论 |

## 已修正：证据误述 T-DOC-1

原 PDB module1186 的记录2768→2792→2816明确为 serde String::deserialize → RON deserialize_string。旧说明因共用地址的反编译器标签误写 deserialize_str。现有实现本来正确，修复只改两行源码注释并为当前 README 添加勘误引用；没有引入自定义 Visitor。历史报告/源码哈希保留原样，其误述由 [ERRATA](integration/ERRATA.md) 取代。独立 SHA 反向还原证明可执行代码未变。

## 性能成本的实际含义

完整性库复用 easy-parallel 3.2.0：每批最后一项在调用线程执行，其余各创建新线程。1792项时，64批共1764次线程创建，原一次性run为1791次；峰值降到最多63个新线程，但仍有O(N)创建成本。fs::read把同时读到的文件全部驻留，内存上界受这些文件大小影响。

本机单次合成probe使用dev库、优化harness和重复缓存文件：1792空文件条目约109.6ms；64×2MiB读取约37.9ms，采样峰值working set约22.3MB。它们用于确认成本来源，不代表发布版真实发行包速度，不证明性能退化。

目前没有据此替换并发/IO实现。真实consumer接线后若高频调用或大文件压力成为瓶颈，再用原锁定且审核过的现成线程池，以及BLAKE3的streaming API；保持错误分类、重复条目和顺序，避免手写池或哈希。稳定文件系统之外的并发读取时序差异继续公开。

## 函数列表如何使用

三个 reviewer 都以业务族映射查漏：
- 私有encode/with_scaling、chunk回填helper和文件检查closure可折入一个可调用业务链。
- str/u64或str/PathBuf的单态实例对应同一泛型规则，不要求重复写实现。
- clone/From/serde派生、Vec/HashMap/IO/哈希内部实例直接复用依赖；schema字段及失败策略仍属于应核实的游戏合同。
- TextFragment/TextRenderer族提示明确的未恢复功能；真实消费者缺失由源码搜索确认。原函数列表不是完整作者API分母，也不是按数量计算完成度的依据。

## 原依赖与新验证

独立依赖审查没有发现手写BLAKE3/FNV/JSON/RON/容器/字节序/线程调度/字体整形/atlas。自有tag framing、word packing、文件筛选和失效规则是业务策略。全部原依赖身份仍匹配原锁。

新增原侧验证针对review歧义：motomoto的限定Cursor Writer/Drop20例与pending error、Reader错误后恢复3组；完整性原FNV/array Hash叶4096例与128个依赖BLAKE3组合gate均匹配。完整verify和原serde/glyph/GPU仍未执行。文本probe验证请求方式、解析接受集合、resolver panic与分配复用；它只是候选观察，未重跑未变72组原数值案例。

失败的review-integrity工具会话从未ready，新live MCP查询未获成功。父owner后来用完整命令行与创建时间识别两个初始化进程，结束实际子进程后wrapper自动退出，剩余0；没有停止其他worker。见 [清理证据](integration/worker-cleanup.json)。独立原RNE叶与已保存原证据各自按范围有效。

## 后续优先项

1. 补真实存档与本地完整性报告消费者，把库结果接入已恢复的业务入口。
2. 沿真实toolbar/TextFragment调用恢复文本状态、失效和布局策略，复用原cosmic-text/etagere。
3. 对罕见schema/失败路径只做有具体歧义的原侧对照，并在真实release消费者上确定性能优化需求。

[结构化整合结论](integration/summary.json) · [motomoto findings](motomoto/findings.json) · [integrity findings](tg-package-integrity/findings.json) · [tg-text findings](tg-text/findings.json)
