# 建筑运行时与基础包源码审查

日期：2026-10-17；最终验收：2026-10-18。范围：`tg-building-runtime`、`tabloid`、`tg-package-integrity` 的全部 Rust 源码、示例/二进制入口及相关测试。流程为业务分析、首轮修复、交换审查范围的独立复审、最终验证、正式模块文档。本次只读取源码与已有测试输入，未使用原作二进制、反编译器或逆向工具。

已确认错误按正确业务语义修复。历史输出或旧测试与修复冲突时更新测试，不将其作为保留错误的理由。工作区已有其他改动均保留；本次没有创建提交。

本轮专项方向见 [审查方向](REVIEW_DIRECTIONS.md)，功能支持与边界见 [完整性矩阵](FEATURE_MATRIX.md)。

## 业务与模块边界

| 包 | 业务职责 | 正式文档 |
| --- | --- | --- |
| `tg-building-runtime` | 将墙源、地形、屋顶和装饰编辑接到 Bevy World，按阶段更新历史、路径、结构高度、洞口、CPU 几何、平台 Rapier 和 Nani 后端意图 | [模块 README](../../crates/tg-building-runtime/README.md) |
| `tabloid` | 按每个订阅者的消费边界累积 delta，共享相同批次的不可变快照 | [模块 README](../../crates/tabloid/README.md) |
| `tg-package-integrity` | 检查清单格式、路径、文件摘要和完整目录扫描，输出缺失/损坏/额外文件或阶段错误 | [模块 README](../../crates/tg-package-integrity/README.md) |

建筑运行时的权威状态与派生状态分开：原始 Curve2/地形 → 发布轮廓 → 路径/分段/高度 → 结构/洞口 → CPU 实例及碰撞；屋顶与历史通过显式阶段穿插更新。缺少原始输入或未完成算法分支使用 `pending`，不会制造可用的 GPU 对象。两个基础包分别负责增量消费与包校验，不承担完整游戏状态存储和启动流程。

## 首轮审查与修复

首轮由建筑核心、建筑墙/装饰、两个基础包三个职责组并行完成；每组先梳理数据和调用链，再修改实现。

| 范围 | 确认的问题及修复 |
| --- | --- |
| 墙发布、原始采集 | 拒绝空轮廓、非有限数、错误网格尺寸/范围及损坏 Curve2 参数化，避免采样 panic 和派生缓存污染；失败保留旧的有效权威 |
| 屋顶创建 | 拒绝现有及同帧重复创建，避免孤儿实体和关联图错误 |
| 高度/方向/参数事件 | 无效的独立请求诊断后继续处理后续请求，避免整批阻塞 |
| 屋顶类型 | 同实体多次阈值 crossing 以批次最终状态驱动派生类型 |
| Nani | atlas 解析后的网格片段变化也标记 `mesh_changed` |
| 建筑命令入口 | 固定长度 ASCII hex 及有限 f32 校验，非法外部文本返回错误而不是索引/解析 panic |
| 自由墙历史 | 每条有效变更及时移交历史，保留同墙 A→B→C 的起点以及不同墙的历史 |
| 矩形编辑 | 按序处理全部事件，消除只取最后一条导致的丢操作 |
| 装饰归档 | 克隆输入的真实 culling 策略，停止无条件放开筛选 |
| 台阶装饰行 | 按实际内容做缓存指纹，公开 rows 直接插入/修改不再触发缺失 revision 索引 panic 或保留旧结果 |
| native 洞口绑定 | 入口拒绝多个装饰身份占用同一个 OpeningId |
| tabloid | 替换的旧快照延后到释放全部订阅锁后析构，避免 State::drop 重入 take 死锁 |
| 包校验扫描 | 不存在/不可读目录、枚举失败及非法名称不再被静默跳过；不完整扫描必须失败 |
| 包校验路径 | 校验相对路径，并逐层拒绝链接/特殊文件，避免路径越界和设备/FIFO 读取 |

## 独立复审与追加修复

建筑核心与墙/装饰组交换全部源码范围；主审独立复审两个基础包，基础包审查者又复核主审追加的校验规则。

| 范围 | 复审确认的问题及修复 |
| --- | --- |
| 调度诊断 | native binding 消费明确位于 Begin 之后，防止刚写入的 fault 被帧初始化清除 |
| 装饰投影 | storage placement 成功后标脏，并在放置消费者前重新发布 legacy 投影；同帧输入可生效 |
| 公开绑定映射 | 对直接资源修改造成的 native/legacy 重复身份撤销冲突贡献并记录 pending，消除 HashMap 顺序决定结果 |
| 放置缓存 | 命中缓存前检查 raw authority，丢失权威时撤销旧洞口 |
| 地形缓存身份 | 保留 Arc 快照使写时复制与分配地址复用不会逃过台阶缓存失效 |
| 自由墙事务 | 在提交元数据、暂存源和历史之前验证 next source，防止部分权威提交 |
| 拱门/过梁缓存 | 缓存命中时重新应用已计算的高度数组；冗余 WallSpatialDirty 不再把派生高度重置回基础值 |
| 创建 ID | ID 计数器落到已占用墙 ID 时，在任何权威状态提交前拒绝创建 |
| 门生命周期 | 删除帧优先于该墙门构造请求，防止同帧复活门实体 |
| 视觉历史 | 所有屋顶视觉记录逐条保留，撤销回到初始值且不丢其他屋顶操作 |
| 诊断快照 | 详细生产 trace 和 lintel trace 遇到输入/高度缓存尚未就绪时输出 pending_reason，停止 unwrap panic |
| 清单结构 | 拒绝重复和保守 Unicode 大小写/规范化别名；非 self 条目必须为 64 位小写 hex 摘要 |
| checksum 编码 | 固定为 LE64 长度 32 + BLAKE3 摘要后输入 FNV，不再依赖宿主 usize 字长与字节序；本机已有格式 fixture 保持通过 |
| 校验错误/CLI | 补充结构化错误、Display/Error；诊断工具不做第二次不安全读取，参数/校验结果使用正确退出码 |

`fill_base_cache` 中的 freeform helper unwrap 经独立追踪，其唯一错误为 RegularShape，而调用已有 shape tag guard，未发现可达错误分支，因此没有添加无用的错误包装。串行历史队列的失败保留机制也经过检查，不能当作独立事件批次任意跳过。

## 完整性与高性能专项补审

在上一阶段基础上，按 [专项方向](REVIEW_DIRECTIONS.md) 再次审查，并由核心与墙/装饰组交换源码范围独立复核。以下为本轮追加修复，均有源码触发链或回归证据。

| 优先级 | 问题与影响 | 修复与验证 |
| --- | --- | --- |
| P1 | 录制期间缺少删除快照仍移除墙，删除无法撤销 | 缺源元数据/颜色时拒绝该墙删除，其他有效事件继续；真实 App 回归 |
| P1 | Keep 编辑、相同方向或冗余空间通知删掉已有门，producer 缓存命中后不再发构建命令 | 门生命周期跟随实际 producer 重建/失效和命令批次；原实体、CPU 输出保留，删除优先仍受测 |
| P2 | 回放未完即可 Start/TryStart 新录制，旧桶可能覆盖新历史分支 | 拒绝新录制并继续推进现有回放；扩展连续 undo 回归 |
| P2 | 依赖新增 Wall 操作未穷举，未支持操作可流入底层 panic/错误分派 | 使用 wall_id 接口，并在录入/回放前检查明确支持集；未知操作保留队列、拒绝提交 |
| P2 | 非有限屋顶输入或部分编辑提交污染派生几何 | 校验创建输入/输出，编辑在候选副本成功后整体提交；多种坏输入及后续有效输入回归 |
| P2 | 瓦片重建失败仍保留与新屋顶不匹配的旧组件 | 失败撤回旧 RoofCpuTiles；外部坏组件及异常投影回归 |
| P2 | native 绑定同帧交接/交换身份被旧归属阻止 | 按最终映射检查唯一性，明确同 native 的移除优先；handoff/swap 回归 |
| P2 | 永久无效装饰请求堵住整个队列 | 永久坏请求隔离并继续，Readonly/待修资源保持重试；合法后续请求及恢复回归 |
| P2 | 深目录扫描同时占用每层目录句柄，合法目录被误报无法校验 | 迭代目录栈，每目录关闭枚举后再深入；64 FD 限制下 100 层目录由 EMFILE 变成功 |
| P2 | 清单每 64 条反复创建线程，大量小文件开销高 | 一次 CPU 自适应 worker 组动态分配任务，仅保存失败结果，按原索引聚合 |
| P2 | 拆除/历史回放深拷贝全世界墙源，空闲帧也复制 | 空闲拆除早退，两条链只复制目标墙；空闲 0、单目标 1 的工作量断言 |
| P2 | 砖块缓存命中前已分配完整构造数组 | 先比较完整输入指纹，失配后准备数组；无变化帧 prepared_arrays_this_frame 为 0 |
| P2 | 相同 native 洞口每帧 Upsert、Nani 每帧重复投影两次 | 相同 authority 不发布；Nani 只投影最终状态一次；稳定 generation 和短命实体回归 |
| P2 | 历史逐项 remove(0) 为二次方移动，native 归属查找为 O(M×N) | 历史一次移除已接受前缀；每系统构造 HashSet 复用，归属查询总量降为 O(M+N) |
| P3 | 相同高度仍整段回填，legacy 空闲投影分配高度 bits 数组 | 比较 bits 后按需回填，借用迭代器比较并在实际更新时复用缓冲 |

结构整理把 `lib.rs` 从约 1,485 行降至 514 行，拆出私有 `wall_publication.rs` 和 `roof_runtime.rs`。公开类型、事件、导入路径及 Plugin 阶段顺序保持一致；18 个执行体经独立逐函数比对。注释集中描述当前职责、输入不变量和后端边界，不保留 draft/未构建进度文字。

性能对照使用同一 release harness、相同 2,048 个 10 字节文件、相同依赖锁，直接调用 check_files，避免 checksum nonce 搜索干扰。在本机 8 CPU 条件下，线程创建由约 2,016 次降至 7 次；五次样本中位耗时从 77.439 ms 到 17.824 ms。最终测量首个 598.246 ms 异常耗时原样保留，不能把这个小文件场景的约 4.34 倍中位改善外推为全游戏或其他平台收益。方法、全部样本与低 FD 结果见 [性能证据](validation/current-pair-performance.json)。

## 验证

当前工作区存在其他任务并发修改依赖和锁文件。最终验收使用当前三个包源码及其生产依赖的隔离副本，避免把不同检查阶段的依赖版本混在一起。仅收窄 workspace 成员并移除未受测依赖包的 dev-dependencies；生产源码、三个目标包的 manifest/README/测试保持原样。副本配置与源码哈希见 [snapshot.json](validation/snapshot.json)。82 个目标包文件已逐字节比对一致，见 [源码一致性记录](validation/snapshot-source-check.json)。测试和 Clippy 通过后仅对七个文件应用 rustfmt，最终格式检查通过。汇总见 [summary.json](validation/summary.json)。这不是整个工作区的构建声明。

| 检查 | 结果 | 日志 |
| --- | --- | --- |
| 三包全部 targets | 99 项通过：building 77、tabloid 11、integrity 11 | [snapshot-test.log](validation/snapshot-test.log) |
| README 文档示例 | 4 项通过：building 1、tabloid 1、integrity 2 | [snapshot-doc.log](validation/snapshot-doc.log) |
| 三包严格 Clippy | 通过，all-targets / no-deps / -D warnings | [snapshot-clippy.log](validation/snapshot-clippy.log) |
| 格式与 diff 空白 | 通过 | [snapshot-fmt.log](validation/snapshot-fmt.log)、[diff-check.log](validation/diff-check.log) |
| 完整性性能及资源边界 | 小文件对照、64 FD 深目录通过 | [性能证据](validation/current-pair-performance.json) |
| integrity 诊断 CLI | 首轮 11 场景退出码及 JSON 检查通过；本轮未重复 | [pair-cli.json](validation/pair-cli.json) |

回归集中覆盖同帧多事件、失败隔离、撤销起点、身份交接、数值边界、缓存与删除、实际工作量。优先复用已有集成 fixture；多个坏输入使用表驱动断言，没有为每个分支复制整套场景。

早期日志保留作过程证据；以 snapshot 前缀日志及最终汇总为本轮验收结果。命令以各 README 为准。

### 编译依赖修复

首次建筑包构建被 `country-core/src/systems/collision/world.rs` 中旧版两参数 `ColliderSet::remove` 调用阻塞。已做单点修改，使用当前 Rapier 所需的 IslandManager、body set 和 wake 参数；该查询世界只注册独立 collider，无模拟父刚体，故不唤醒。这个文件中的其他既有工作区修改不属于本次审查变更。

最终复核还遇到 `country-core/src/resources/history/decorator_edit.rs` 将 `u32` 字段赋给 `u64` 历史字段的类型不匹配，已加显式无损转换。共享依赖在检查命令之间继续修改，新增碰撞查询调用又出现 Rapier 接口不匹配；相应修正借用、四元数字段、共享 shape 所有权、回调参数和查询参数个数。依赖修改限于使当前工作区通过编译，不代表对这些新增底层业务完成了本轮独立审查。

`country-core` 仍有已有的 unused import/variable 和旧序列化字段命名警告；三包自身严格 Clippy 通过。未为消除依赖警告扩大本次重构范围。

## 文档与验证边界

三个 README 均已改为正式模块文档：职责、数据模型、业务流程、API、错误、生命周期/并发或文件系统约定、性能边界、使用示例和开发命令。历史恢复来源只保留简短链接，审查过程集中在本报告。

非 UTF-8 名称回归只在 Linux 启用，本机未执行；Windows/Linux 文件系统真机及其他位宽平台未运行。校验需要稳定文件树，常规文件系统 API 不提供并发替换时的原子快照。建筑包的真实 GPU/backend、完整原作场景和所有算法分支不属于本轮源码验证。上述边界没有被当作已验证能力写入 README。
