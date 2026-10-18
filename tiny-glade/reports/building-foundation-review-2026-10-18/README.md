# 三包独立复审与工程化改造

日期：2026-10-18。范围为 `tg-building-runtime`、`tabloid`、`tg-package-integrity` 的全部 Rust 源码、二进制/示例入口、相关测试及模块文档。本轮在[上轮审查](../building-foundation-review-2026-10-17/README.md)之后重新独立检查，未使用原作二进制或逆向工具，未创建提交。

三个职责组分别审查建筑权威/历史、建筑派生链和基础包；基础包组交叉检查建筑核心，派生组独立核对 CLI 拆分，主审复核基础包、门缓存和历史回放。已确认错误按正确业务语义修复，不因历史输出或兼容测试保留。

业务与工程化方向见[审查方向](REVIEW_DIRECTIONS.md)，能力及输入边界见[完整性矩阵](FEATURE_MATRIX.md)。正式业务文档：[建筑运行时](../../crates/tg-building-runtime/README.md)、[tabloid](../../crates/tabloid/README.md)、[包完整性](../../crates/tg-package-integrity/README.md)。

## 确认问题与修复

未发现有证据的 P0 问题。下表为本轮确认项，按严重程度排序，均已修复；行号指向修复后的相关入口。

| 级别 | 位置 | 原因与影响 | 修复及验证 |
| --- | --- | --- | --- |
| P1 | [roof_runtime.rs:351](../../crates/tg-building-runtime/src/roof_runtime.rs#L351)、[219](../../crates/tg-building-runtime/src/roof_runtime.rs#L219) | 外部非法 Roof 的方向/删除请求流入底层 shape assert，导致更新 panic | 消费前验证；删除仍可清理，不为非法几何生成声音；异常组件回归 |
| P1 | [wall_publication.rs:38](../../crates/tg-building-runtime/src/wall_publication.rs#L38) | 参数容差接受非 0/1 端点或短段逆序，曲线查询可能越界/assert | 精确端点和非递减检查；扩展原有坏输入表，保留上次有效权威 |
| P2 | [wall_authoring.rs:175](../../crates/tg-building-runtime/src/wall_authoring.rs#L175) | 非法创建源在下游才被拒绝，上游 ID、颜色、元数据和历史已提交 | 先验证完整候选，再提交；录制缺必要元数据同样拒绝且不耗 ID |
| P2 | [wall_authoring.rs:126](../../crates/tg-building-runtime/src/wall_authoring.rs#L126) | ID 计数器到 MAX 后回绕，破坏分配单调性和身份 | 分配前 checked_add，耗尽时明确诊断；扩展既有计数器回归 |
| P2 | [wall_authoring.rs:211](../../crates/tg-building-runtime/src/wall_authoring.rs#L211) | 未录制创建仍深拷贝快照入队，后续永不消费，持续占用内存 | 仅录制时入队；连续未录制创建队列保持为空，更新保留错误行为的旧断言 |
| P2 | [history_replay.rs:14](../../crates/tg-building-runtime/src/history_replay.rs#L14)、[history.rs:197](../../crates/tg-building-runtime/src/history.rs#L197) | 同桶后续命令失败时，前面命令已推进/移除，但局部输出被丢弃 | 屋顶用共享编辑的候选进度；墙保存紧凑进度检查点，成功后才 drain 完成组；修正后重试不丢效应 |
| P2 | [history.rs:121](../../crates/tg-building-runtime/src/history.rs#L121) | 活跃回放期间拒绝 cursor/reset 等操作时提前 return，原回放也停止推进 | 独立拒绝操作后继续推进当前桶；真实 App 回归 |
| P2 | [wall_bulldoze.rs:98](../../crates/tg-building-runtime/src/wall_bulldoze.rs#L98) | 同墙重复拆除写入重复历史；非法锚点/时间污染动画 | 拒绝重复活动命令和非有限输入；回归覆盖历史条数及后续恢复 |
| P2 | [lib.rs:445](../../crates/tg-building-runtime/src/lib.rs#L445)、[roof_runtime.rs:261](../../crates/tg-building-runtime/src/roof_runtime.rs#L261) | NaN/负时间污染屋顶动画；两个有限高度相减仍可能溢出 | 非法帧时间诊断并按零推进；校验回放速度及乘积；无效瓦片步进保留有效模拟，允许删除动画完成 |
| P2 | [gates.rs:214](../../crates/tg-building-runtime/src/gates.rs#L214) | 单帧门命令在缺输入时丢失，上游缓存不重发，补齐输入后永久无门 | 保留每墙最新批次；空生产批次替换旧值，删除优先；缺地形后恢复回归 |
| P2 | [gates.rs:153](../../crates/tg-building-runtime/src/gates.rs#L153) | 门缓存遗漏地形依赖，地形变化仍使用旧 CPU 几何 | 纳入范围、采样策略和保持存活的 Arc 身份；范围变化及 COW 回归 |
| P2 | [decorator_placement.rs:159](../../crates/tg-building-runtime/src/decorator_placement.rs#L159) | 失败撤回洞口实体后未标脏；输入恢复为旧成功值时误命中缓存 | 保持 dirty，恢复后重新创建；新增失败→恢复回归 |
| P2 | [integrity lib.rs:211](../../crates/tg-package-integrity/src/lib.rs#L211) | self 清单条目直接判 Valid，文件缺失或为空目录仍可能校验成功 | 仅豁免摘要，仍要求可打开的普通文件；通过公开入口复现与验证 |
| P2 资源边界 | [integrity lib.rs:260](../../crates/tg-package-integrity/src/lib.rs#L260) | easy-parallel 创建线程失败会 panic，整轮验证中断 | scoped Builder 创建失败后由调用线程和已有 worker 完成；混合文件串并行结果一致。未实际耗尽系统线程 |
| P3 健壮性 | [tabloid lib.rs:78](../../crates/tabloid/src/lib.rs#L78) | 准备阶段 delta panic 时，候选/已准备状态可能在订阅锁内析构，重入析构可死锁 | 准备对象先释放 guards，再析构状态；不捕获生产 panic。两批次回归证明锁外析构、原批次和序号不提交 |

五项建筑针对回归已在修改生产源码前实际失败，见 [before-core.json](validation/before-core.json) 及对应日志；基础包两项和门恢复的修复前输出见 `validation/before-gate-recovery.log`、`validation/pair-before-*.log`，后者为当时工具输出的转存摘录，未冒充重新执行。其余边界由源码触发链及最终回归验证，未声称每一个分支都单独执行过修复前版本。

## 工程化改造

以下属于可维护性和成本改进，与功能缺陷分开记录：

- 复用已有 `PublicWallBuildingState` 接口；私有 `RoofRecord` 和 `WallProjection` 只补已知字段访问，不假定 Rust ABI，也不猜测未知字段业务含义。固定记录切片的长度由数组类型与常量范围保证。
- 将墙回放事务推进独立为私有模块；候选计算、校验与提交分开。墙回放只复制目标源和进度标量，未深拷贝整份历史账本。
- `building-run` 分为输入解析、操作派发、快照和构造 trace；保留所有操作分支、JSON 字段与记录编码顺序。快照按类型化 ID 排序，移除 JSON 字符串反向解析；诊断缺输入使用 pending/可选字段。
- 记录使用共享 hex 编码器，按可用 size hint 预留空间，避免逐字节 format 临时分配；不承诺未知长度迭代器只分配一次。
- 门重试借用保留批次，移除第二次命令深拷贝及临时请求数组；砖块指纹复用工作缓冲，BuildingReport 清空时保留容量。空闲工作量断言继续受集成测试覆盖。
- 整理长路径导入、压缩控制流和业务变量名。注释说明锁/析构顺序、曲线参数、候选提交、缓存身份和后端边界；恢复过程与审查过程留在报告，不混入日常实现说明。
- tabloid 补全公开接口及 panic/所有权合同；完整性模块分离 worker 索引领取逻辑并移除 easy-parallel 依赖。公开 API 未作破坏性调整。

本轮没有重新测量上轮小文件测速，因此不把上轮耗时或倍数作为当前修改后的性能数据。性能结论限于已移除的复制/分配路径与现有工作量断言，不外推为全游戏帧率收益。

## 验证

| 检查 | 结果 | 证据 |
| --- | --- | --- |
| 三包全部 targets | 110 项通过，0 失败 | [final-test.log](validation/final-test.log) |
| README 示例 | 4 项通过 | [final-doc.log](validation/final-doc.log) |
| 严格 Clippy | all-targets / no-deps / -D warnings 通过 | [final-clippy.log](validation/final-clippy.log) |
| 格式与 diff | 通过 | [final-fmt.log](validation/final-fmt.log)、[diff-check.log](validation/diff-check.log) |
| CLI 生命周期对照 | 两墙创建、方向切换、删除及墙移除，共 4 帧；全部字段和值一致，仅按业务键归一无序集合 | [cli-equivalence.json](validation/cli-equivalence.json) |
| 源码一致性 | 88 个目标包文件、55 个建筑算法依赖文件与隔离副本逐字节一致 | [source-check.json](validation/source-check.json) |

命令、环境与退出码见 [final-commands.json](validation/final-commands.json)，汇总见 [summary.json](validation/summary.json)。先前失败和中间检查日志保留作过程记录，以 `final-*` 为最终结果。

当前工作区仍有共享锁文件变化，以及范围外 utils 几何代码的编译错误。验收沿用固定生产依赖副本，仅收窄 workspace 成员、移除未受测依赖包的 dev-dependencies；三个目标包的源码、manifest、README、测试原样复制，建筑算法依赖另作当前源码一致性核对。这不是整个工作区构建成功的声明。为控制磁盘占用，最终命令关闭增量编译，未修改项目构建配置。

## 验证边界

三个 README 已按正式模块文档更新，覆盖职责、业务流程、API、错误、生命周期、并发、资源和性能边界。真实 GPU 分配/绘制、完整游戏场景和未支持算法分支仍按完整性矩阵明确标识，未作为已完成功能。

本机未运行 Windows/Linux 真机验证、Linux 专属非 UTF-8 名称回归或真实线程耗尽实验。校验仍要求稳定文件树，不提供并发替换时的原子文件系统快照。三个包自身没有新增生产 unsafe/FFI；本轮不等于对全部第三方及范围外底层代码的安全认证。
