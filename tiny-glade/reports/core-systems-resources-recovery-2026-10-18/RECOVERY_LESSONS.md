# Tiny Glade 核心业务恢复：本轮可复用经验

适用：B 原生业务恢复到当前手工重构的 Rust 工程。此文件是项目复核清单，不是新的通用 Codex 技能包，不将有界样例通过称作完整游戏验收。

1. **先找现有 owner，再确定恢复边界。** 原文件缺失可能已合并到当前模块（TerrainStroke 在 tg-terrain；replay_terrain 已移到 systems/terrain）。搜索真实生产者、消费者和注册入口，不能按原目录重新复制模块。
2. **算法可重构，手工架构不回退。** 对比真实输入输出/状态；原算法更复杂时在现 InProgressCurve/Rdp/Sheep/History 类型中补齐。库与容器复用；不添同义组件、镜像 bool 或第二个历史引擎。
3. **ICF 名称不证明类型。** grow_one 同码可带无关类型名。事件身份要核完整 PDB SystemParam、原 caller 的字段写入与真实 reader。地形 Stairs 误认被纠正为 PlayTerrainSound。
4. **EventInstance header 不固定套偏移。** 先定同类型 stride、payload 与 EventId，再配 producer/consumer。羊字段、RMB InstaDeco 和树命令均不可把队列序号当实体/业务 ID。实体代数、storage index、generated ID、variant id 必须区分。
5. **有界原执行必须写清依赖。** 保存原 RNE/PDB/hash、完整入口寄存器初始化、原字节、输入/输出 bits；注明 query/库/分配桥接。样例验证的是该 range，不能扩称真实 query traversal/GPU/宿主。测试未覆盖的叶子、cold path、NaN payload 不推定通过。
6. **顺序正确还不等于 Bevy 调度合法。** 静态无环会漏重复 function type 的 SystemTypeSetAmbiguity。真实 schedule.initialize 必须跑；有多实例时给具体阶段普通 SystemSet 标签。另用实际 World 测首帧 Commands、事件 reader、跨帧生命周期。
7. **从真实第一次输入测到下一次输入。** 不预先 start_input_recording 或预填 UI canvas 掩盖缺口。核首帧注册权限→start→typed record→finish→Input Sync→第二帧mutable；再测空记录、最后事件、短路 reader、删除/Undo。
8. **回归夹具也会过期。** TestDevice 可忽略初始上传、GJK 不保证解析TOI误差、原存档是命名 enum 而非tag dump。先核生产与原证，再只修错误夹具，保留业务断言和roundtrip，不放宽阈值掩盖失败。
9. **结果与源码绑定。** before/after SHA、Cargo日志、source_drift、候选freeze、独审报告独立保存。并行任务更新接口时最小适配，不覆盖其代码，不归功其修复。
10. **清理和工具连接不计业务恢复。** 共享DB只读，损坏不重建覆盖；用原字节/LLVM/已分析证据继续。只清持锁确认过期的生成缓存，报告逻辑大小与实际可用空间，不删原二进制、源码或证据。
11. **内层数据 schema 不能代表整个操作。** TreeData 六字段只保存 placement，其中 id 是变体；TreeEdit 外层另存 index。必须追外层 serializer 与 replay，不能因内层无 index 就编造重新分配规则或以 variant 删除树。不同 EventInstance 的 EventId 还可能在末尾，InsertTree 的末尾队列编号曾被误作 height。
12. **泛型查询不等于游戏 callback。** 同一空间索引的树隐藏单态直接逐格筛 WallId、严格比较距离并早停；通用圆查询的排序、去重、<= 与另一种插值会改变结果。复用现存索引和数据，补专用业务谓词，并核极短段、NaN 与边界。

本轮总账见 README.md；永久用户规则见 country-core/AGENTS.md 和逆向项目 PROJECT_MEMORY.md。后续批次新增反例应追加纠错，不改已冻结证据来伪造一致性。

## 13. Counted mapped buffer：头行、唯一owner和空hash归一必须一起恢复

BushCPU原8B×9601的第0行是count，位置从第1行开始；不可用另一个从无人写入的Resource(count,Vec)代替AssetSsboLibrary里的实际映射。None不上传是该canonical类型的明确策略，不可为了它修改全部mapped类型。实际CPU reader须保留library缺失短路、先bounds后变更、raw u32双字排序（与float total_cmp不同），以及u32低位hash XOR emptyhash归零；否则首个空帧会错误发布4096脏格。复用std Hash/DefaultHasher，不翻写依赖SipHash。构造与上层World测试通过不等于GPU completion/coherency和后续populate消费者已通，现重复BushGrid/unused参数要继续追。旧745B reader范围在CALL opcode中截断，只是定位证据；正式750B PDB/原字节范围另冻，不覆盖旧失败记录。

模块状态图入口为module-progress.png / module-progress.html，结构化快照module-progress.json。它区分有界回归、挖掘候选、完整模块待审；不将文件数、测试数或接口库存换算为完成率。

## 14. 真实资产通过仍须核字段类型与注册身份

Rocky资产所有cell_id/is_corner/is_top非负，单看JSON样本与全pack hash不能证明u32。必须把精确key visitor tag→jump table→typed i32 parser→Result payload字段→实际packer连起来；负数与越界回归才覆盖样本未含域。记录内GPU raw u32不决定JSON整数类型。原成功返回的单usize VertCount要沿Commands thunk与W4/W5同时追，不能把手工CPUVec结构首字当原count。相同SystemParam形状也不能证明Startup身份：查真实plain系统名引用和SubApp::add_systems泛型schedule，PreStartup color init与Startup terrain在这里曾被混淆。保留原失败/纠错和候选freeze，不为通过回写旧证据。

## 15. Result判别与异步ID状态不能反读

原History快照TryFrom完整37B，PDB源码行263曾被误当字节大小；`cmp id,2; jb unresolved`跳到Err=1，id>=2才写成功结果。旧trait/测试把Err分支bool误当resolved，以致0pending/1invalid被接受而真实ID被过滤。需同时核Result成功载荷写、reader返回门与producer初值0/worker失败1/成功hash ID的全部端口，不能仅看某个比较式。ledger持同Arc<AtomicU64> pendingID，不是Arc域数据或另一个SnapshotCaptureState；load原样接受裸u64，0/1由getter过滤而非反序列化额外拒绝。x86 alignedMOV无法区分Relaxed/Release，不能硬说已恢复Rust ordering；现Acquire/Release只作适配选择。ReplayBuffer pop_counter属于真实pop owner，原空断言前已INC；不能用push或旁路tracker替代。

## 16. 文件快照必须核完整encoder、实际bucket和有界decoder

从loader日志摘9个key、把14域简化成Trees、用JSON文本代替Motomoto都无法得到原存档。实际Snapshot chunk保存Version2与14域元数据；Raster每chunk有u64原长+Deflate6，Terrain仅Deflate6。复用现motomoto Writer/Reader，不另造文件引擎。truncated8-byte Raster tag会碰撞，必须保full-name列表与原Vec顺序，不能以tag建唯一Map覆盖。装饰快照是addr→真实ID/HistoryState分桶，Some(empty)也属于状态；XOR/rotate哈希寻址不是ID编码。恢复时沿同实际DecoratorIdGen reserve，不在compat helper临时造gen。写入接口必须显式收真实raw terrain/range/supports，缺失Err；解码未恢复时明确Err，不能给每ID造DefaultInfo来伪装成功。Serialize通过不证明Deserialize默认/required/别名规则，分别核原reader并复审。
