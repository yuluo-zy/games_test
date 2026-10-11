# 多 agent 恢复组织与完整建筑落地计划

日期：2026-10-11。状态：**方案审计，未实施、未验证**。本任务只读现有源码、脚本、台账和历史报告，新增本文件；没有运行 Cargo、probe、oracle，也没有修改业务代码、skill 或 AGENTS。当前用户要求“并行实现先行、暂不验证”；该要求继续适用于候选实现，不取消后续原码与连续运行链验收。

建议保留父 agent + 三个业务 agent，以**原模块及完整状态链**交付，统一元数据请求和接口变更批次，把共享登记从逐函数人工协作改为可审阅的交付清单。不要再为每个新叶函数冻结全工程，也不要把增加 agent 数量本身当成吞吐提高。

## 1. 当前证据与具体瓶颈

审计依据：[AGENTS](../../AGENTS.md)、[当前保存记录](../../checkpoints/building-parallel-2026-10-11T100110Z/CURRENT_IMPLEMENTATION.md)、[建筑计划](../../BUILDING_SYSTEM_RECOVERY_PLAN.md)、[并行所有权](../../tracking/parallel-implementation-conflicts/README.md)、[父集成缺口](../../tracking/parallel-implementation-conflicts/parent-integration.md)，以及下表列出的实际文件。PROJECT_MEMORY/TASK_BACKLOG 中较早的“下一步”已经多次被后续恢复替代，不能把它们拼成当前未完成清单；以具体源码、最新包状态和对应证据范围为准。

| 观察 | 含义 | 建议 |
| --- | --- | --- |
| [verification_provenance.py](../../scripts/multi-version/verification_provenance.py) 对 src/crates/math/roof/wall/building/history/tests 下全部 Rust、各 Cargo.toml 和根 Cargo.lock 绑定 | 业务叶函数证明与整个工程快照耦合；一个新独立模块也改变全局候选集合 | 保留全局归档身份，另建逐证明的实际依赖闭包，二者分工 |
| 当前 revalidate_building.py 列出66个 verifier，再加2个其他任务；历史阶段4 summary 是67项、127个 candidate_sources | 当前脚本任务集合已经变化；“67全过”是历史范围，不能机械当成永远固定的当前套件 | 用版本化 suite manifest 明确任务集合，原报告保持不可变 |
| 对66个脚本做静态文本筛查：42个调用 candidate_provenance，24个写显式 candidate_sources | 已经存在限定依赖的先例，但显式列表不等于经过审查的完整依赖闭包 | 先试点3个包，不立即重写所有验证器 |
| 当前 runtime lib.rs 1294行、CLI building-run.rs 800行/约60KB、history.rs 423行；父独占两级lib/Nani/CLI/图 | 每条业务线的最后一步集中到父 agent，零散注册、alias、CLI字段往返频繁 | 模块提交 register/事件/顺序/删除合同；父按一批共享改动整合 |
| 先前反复逐地址请求导出，再追加相关 helper；新的 pending probe 同时使用库alias和本地path模块 | 元数据排队和类型重复可能阻塞业务；相同源码被以不同模块身份编译可能产生类型不一致 | 模块级入口+直接游戏helper一次请求；共享类型只选一个导出身份 |
| 新并行包涉及 native opening 与 legacy intent、cancel 与新placement、DirtyTiles发布与订阅、CPU记录与Nani | 主要冲突是状态所有权和事件顺序，文本合并成功也不保证业务正确 | 冲突记录必须包含双writer、读取时点、撤回及失败队列合同 |

上述规模是本次静态读取时的观察，不是性能测量。**并非全部局部证明都有全Rust绑定问题**，例如已存在显式限定Rust/Cargo依赖的Rapier报告；全阶段runner使用wholefreeze对里程碑有不同用途，不能因局部优化而删除。阶段4的[冻结报告](../building-integrate/stage-reviews/stage4-frozen/summary.json)仍是其自身输入的证据。当前未验证候选、后来的任务增加和新依赖不能继承该报告的全工程结论。

## 2. 工作包要覆盖链，禁止以函数数量交付

每包的最小合同为：`真实输入owner → authoritative source/ID → dirty/关系 → CPU消费者 → no-change/失败 → 删除/取消/undo/redo`。已有原码核心直接复用；包内若使用 resolved 上游输入，准确写出它来自哪个实际 caller，且把未恢复 sender 留作下一条边，不新增一组永久fixture默认值冒充生产者。

当前四个并发席位建议如下。父 agent 不同时承担第四条新业务恢复长链；有可独立读取的索引查询，业务 agent 自行使用，只有共享 Ghidra 数据库的受限操作进入服务队列。

| 席位/包 | 原模块与本批边界 | 当前可复用入口 | 本批应交付的最后消费者 |
| --- | --- | --- | --- |
| A：编辑、附着与事务 | country-core walls/decorator/storage + system-decorator；G01/G09/G10/G14相邻接口 | create_shape_source、真实Raw/Public、Rectangle finish、PendingWallChanged、Archivist、统一Wall/Roof buckets、已选asset/subtype/opening规则 | source88入库→Info72→普通anchor→实际asset/opening→holes/bricks；矩形finish/cancel恢复后相同链更新；新增owner可删除/撤销，而非仅发通知 |
| B：结构与建筑输出 | system-wall-constructor及实际elevation/gate/fence/roof输出owner；G04/G05/G06，随后G08 | prepared path/cache、全kind from_curve、masks/stats、arch/lintel、Gate120、真实Solid64/Nani descriptor | gate/fence等CPU输出及实体增改删；稳定后继续floor/裙边/屋顶trigger/梁等一个完整族，不把marker计为几何 |
| C：碰撞、空间与传播 | 原RaycastWorld游戏过滤、DirtyTiles、wall segmentation/platform/stairs；G03/G12/G16局部 | 原锁定Rapier/Parry/slotmap、真实stairs geometry、DirtyTiles64、现有terrain候选/订阅 | 实际碰撞owner→query/filter→dirty发布→两端建筑重建；替换/删除释放，修补依赖消费者，不复写物理引擎 |
| 父：集成与共享服务 | G00契约、共享lib/StageGraph/CLI/Nani桥、依赖Lock与台账 | 现有真实App/World、统一History、运行链框架 | 批次注册、共享依赖解析计划、冲突裁决与最后链闭合；以后验收时协调独立review |

这不是把所有G00–G19平均分给三人。先完成能够连接现有主体的三条链；一个包稳定后，空出的agent领取下一完整模块族。**目标C-first：已有B核心和证明是已验参考，新功能先选C原入口、caller/enum/依赖契约，在C实现和验收；不要求先补齐一个完整B版再重做C。** 当前正在联接的B候选保留明确B状态；复用至C前必须确认各边界，必要时提供版本adapter而非混合ABI。特别避免每人各恢复“创建”而删除/历史留给父 agent。需要同一个共享owner时，指定一个主包，其他包提交输入/输出扩展请求，不并行改同一类型。

## 3. 元数据采集作为共享服务，不让父成为人工地址转发器

服务是现有工具和证据索引的组织方式，不要求新增常驻第五个 agent，也不要求重建Ghidra工程。父拥有串行数据库队列；三个业务 agent 可以并行查已落盘的符号、source/checksum、锁文件、line/inline/callsite索引。将一个完整模块的关键入口、直接游戏helper、descriptor/registration及必要常量作为一批请求；std/hashbrown/Serde/ECS依赖只取身份和业务边界，停止导出其巨大实例body。

建议请求schema（仅方案示例）：

```json
{
  "request_id": "decorator-anchor-b-02",
  "package": "decorator-attachment",
  "baseline": "B",
  "binary_identity_ref": "tracking/baselines.json#B",
  "hypothesis": "modified iterator selects the actual tracking set",
  "entries": ["Decorators::iter_modified_mut", "Decorators::mark_all_modified"],
  "addresses": ["0x140a1b0b0", "0x140a1b140"],
  "needs": ["symbol_aliases", "direct_game_calls", "bounded_asm", "descriptor"],
  "exclude": ["dependency_body", "unrelated_call_tree"],
  "output_dir": "evidence/building-decorator-info/B-modified-selector",
  "priority": "blocks-real-consumer",
  "status": "queued"
}
```

队列由 `queued → identity_checked → cached/exporting → ready/partial/capability_gap` 推进。`ready`只表示证据已返回，不表示源码恢复或验收。响应必须带baseline、原binary/PDB身份、实际地址/aliases、是否完整函数或范围、工具/数据库身份、输出文件、失败原因及下一判别点。地址按基线解释；不能把B地址套到C。

缓存键应包含binary/PDB身份、查询条件、导出范围和工具/数据库版本。缓存已读函数和负结果；新增假设、不同范围或身份变化才重新取。若header/tpi精确过滤没有匹配，返回“该查询未找到”而不是“该游戏字段不存在”。元数据提取过程与业务验证严格分开；不用保存全工具JSON到对话，返回紧凑摘要和磁盘引用。

PDB fallback 与其他审计协调：三版本procedure、source/line、inline记录仍有用；Rust局部变量/成员layout不能按其存在来规划自动迁移。策略顺序为：

1. 已有source/checksum/package/function索引确定原模块与依赖身份；复用native PDB工具的精确查询，不整库反复dump。
2. 查函数/inline/line范围、泛型名内嵌的FunctionSystem参数类型候选名、Bundle/descriptor注册。类型名只说明候选接口，不是原参数变量名、局部位置或字段布局。
3. 对缺layout的业务对象，沿producer stores、serializer字段名、accessor、caller参数和consumer reads交叉恢复；给每个字段独立证据引用。
4. 缺PDB变量时继续由实际机器码推进已证CPU链；未知来源/default/ID/owner保typed gap。A/B/C布局分别登记；不因B成功而自动迁移C。
5. 已确认依赖只调用匹配公开API，必要fork差异单独记adapter边界；不把巨大容器展开当核心恢复量。

详细统计见同组的[PDB能力审计](pdb-capability-audit.md)；[A](A-pdb-capability.json)、[B](B-pdb-capability.json)、[C](C-pdb-capability.json)是当前落盘数据入口。该审计已发现三版本Rust procedure/DEBUG_S_LINES与inline有丰富记录，但Rust local/位置/defrange为0；C库阳性参数/局部位置记录能正常读出，不以“工具坏了”解释Rust零值。`void()/0x1001`占位签名和TPI/IPI流存在不能当实际Rust参数layout。

另一个agent负责元数据自动化方案：目前publics索引只覆盖5个私有crate前缀，不能当完整DBI或依赖闭包；已有ExportFocusedFunctions.java在单批复用DecompInterface，但批间仍重启headless。建议先延伸现有工具为单进程消费批清单，采用exact/observed/opaque/missing分路，不新增第二套全量解析工程。

## 4. 契约先行、写隔离与共享登记排队

“契约先行”不是等所有字段都恢复才写代码。业务 agent 先提交本批最小typed API及已证领域，继续实现其独占文件；消费者可复用该API，未知部分显式Result/Option/pending。契约必须写清：

- 输入来自真实owner还是明确resolved caller；baseline/valid-domain/default证据。
- source与派生数据分别由谁写；ID保持/分配、资产与资源必填项。
- register究竟只登记资源事件，还是同时安装系统；所属stage、前后依赖、ApplyDeferred要求，禁止父又重复schedule同一consumer。
- no-change、缺资源、非法原panic域、部分成功及队列保留；pending是否可重试、是否保完整原request、事务录制时点。
- 增改删、取消、撤销/重做、宿主消失、缓存撤回及collision/GPU资源释放；通知不能替代实际状态更新。
- 现有Rust类型与实现样例。用相邻模块的错误/owner/register形态，复用Curve2/nearest/cache/history/asset规则；不复制数学、RNG、容器或仅为了命名漂亮重排工程。

每包独占 `building/<module>.rs`、对应runtime模块、probe、verifier、evidence目录及一个conflict文件。共享 `src/lib.rs`、runtime lib/StageGraph、Nani、CLI、Cargo manifests/Lock只有父可写；若确需history.rs或wall_authoring.rs等已有owner文件，先在队列预留，父确认新的单写owner和范围，再交付。现有agent ownership具有优先级，不能按这份方案擅自覆盖。

包交付一次 `handoff`：新文件列表、API签名、root alias请求、register调用、stage依赖、CLI最小事件schema、共享变更diff建议、依赖增量与冲突IDs。父按同一批应用，不再每新增一个helper就立即修改lib/CLI。非阻塞CLI观察字段可以等批末；核心状态consumer不能等CLI才能编写。

隔离优先顺序：先文件独占，必要时使用已授权的worktree/分支；共享合并由父一次完成。不要在并行phase让agent各自运行cargo并争同一target/Lock，不发明独立toy ledger/entity table来绕开接口竞争。probe引用游戏types时选择统一root alias或统一独立模块组，避免同一enum同时由path module和库module定义；最小probe crate作为后续构建隔离试点，不立即重建workspace。

冲突schema建议统一成如下记录，旧Markdown可以逐步补齐，不要求本轮迁移：

```json
{
  "id": "native-opening-single-writer",
  "package": "decorator-attachment",
  "status": "implemented_unverified",
  "owners": ["decorator_info_binding", "legacy_placement"],
  "files_and_symbols": ["decorator_info_binding.rs::resolve", "decorator_placement.rs::resolve"],
  "state_or_event": "OpeningId / UpsertOpening / RemoveOpening",
  "trigger": "same ID switches to native owner; stale legacy removal arrives",
  "authority": "explicit NativeDecoratorOpeningIds mapping",
  "required_order": "identity before Authoring; one writer before Openings",
  "evidence_refs": ["evidence/building-decorator-info/findings.md"],
  "proposed_resolution": "native-owned ID opts out of legacy upsert/withdraw/removal",
  "pending_fields": [],
  "acceptance_cases": ["switch owner", "forget mapping", "cancel plus placement", "host delete"],
  "resolved_by": null,
  "review_report": null
}
```

冲突“已实现防护”与“已验收解决”分开。未知字段、依赖身份、共享类型不一致、队列/owner竞争也登记，不能只登记Git文本冲突。父批末先整理契约和图，再安排后续验证；保持每项失败证据，不把未知请求默默弹掉以获得干净日志。

## 5. 暂停、降频与仍不可猜的工作

本轮立即停止：Cargo/tests/oracle、全workspace冻结复验、改旧hash冒认新proof、新的业务实现（本次方案审计各agent仅写指定策略文件）。后续恢复实现phase时仍按用户当时要求决定是否启用验证。

应降频：逐函数地址来回；重复全PDB索引/同一无匹配查询；已确认依赖body decompile；独立leaf完成即全工程freeze；为每个helper追加共享lib/CLI；反复全文读PROJECT_MEMORY/巨大反编译输出。每次读“当前包短handoff + 具体证据 + 下一消费边”，需要时再展开。大文件不整体交给新agent重写；将新功能落独立模块，父只做必要桥接。CLI后续可按包提取小解析函数，但这应是单独机械整理批次，不夹在业务恢复中改变命令语义。

不能降级的风险：raw与prepared、curve长度与缓存高度来源；RNG实际消费顺序；枚举niche、padding与真实active字段；同ID双writer；原事务owner/bucket/group順序；删除/取消/undo资源释放；失败后局部变更/队列丢失；真实startup/global/default值；特殊墙/特殊destination分支；Nani descriptor与实际GPU handle；C版本layout与调度。此前 Roof删除旧砖、旧Remove清fresh metadata、录制Roof/Wall顺序、native数组重叠都是这类风险，不能以“先实现”改成猜测或凭编译接受。

## 6. 后续验收分层与证明输入改革

当前所有新增候选保持 `implemented_unverified`。后续经用户恢复验证后，按层推进，每层保独立状态，避免“编译成功=整建筑成功”：

| 层 | 何时做 | 接受什么 |
| --- | --- | --- |
| L0 证据/合同 | 实现phase可继续静态审查 | 模块归属、依赖排除、字段/入口来源和未解边，不接受行为 |
| L1 批次构建 | 冲突整理、统一Lock/alias/register后 | 当前candidate能编译、类型身份一致；不接受原业务 |
| L2 受影响原码核心 | relevant输入冻结后 | 实际原entry/range→return、完整active输出/RNG等声明范围；允许仅正常domain但明确失败域 |
| L3 连续真实World链 | 同一候选revision | 创建→idle→编辑→missing resource→取消/删除→undo/redo→清洁idle，具体cache/实体/资源消费者状态 |
| L4 独立review | 包作者不自验，另一agent/父读该包 | 原证据与actual runtime binding相符、scope无过度声明，修复后的相关报告重执行 |
| L5 里程碑/最终 | 多包闭合或版本/依赖迁移完成 | 全workspace回归、C自身迁移、存档与最终backend/场景；仍不由局部统计产生完成百分比 |

逐证明manifest应绑定：实际候选源码的传递依赖、probe/adapter、依赖版本/source/features、构建flags/target/toolchain、原binary/实际游戏入口、oracle及所有共享harness、fixtures和比较规则。serde/glam/宏展开/build.rs/cfg带来的闭包必须计入；未建立闭包的旧verifier仍按全局保守失效，不能先删hash就宣称可复用。

| 身份/证明层 | 必须绑定 | 无关变化能否复用 |
| --- | --- | --- |
| 局部算法报告 | 真实传递源码/库边界、原端harness及inputs | 仅已审闭包不变的原历史证明；不能当新binary已执行 |
| 编译产物 | 实际binary hash、构建inputs/flags/toolchain | binary身份不沿用；不必因此宣称每个原算法都失败 |
| World链/图 | 实际模块与stage、event/资源writer-reader、sharedtypes、flush与运行fixture | 相关图或共享owner变化要重新运行；pure算法报告仍有自身范围 |
| 全workspace里程碑 | 完整suite manifest及冻结Rust/Cargo/构建/报告集合 | 不继承到新工程；只在计划的里程碑或不明影响范围下重验 |

全workspace hash继续用于保存/阶段快照；`proof_inputs`用于局部有效性。旧report不改写，其状态为“在历史inputs上成立”；当前inputs无关变化可显示“该模块inputs未变，历史证明可引用”，但不得把旧candidate binary hash替换成新hash。未冻结时写 `implemented_unverified`，不能用文档重绑定代替实际执行。

后续试点3个已有显式边界的包：wall_pending_changed、wall_production、一个World整链verifier。先人工审查传递依赖和harness（特别native_fixture），再验证变更影响选择。共享harness变更影响全部依赖它的报告；Cargo lock增量按实际依赖图/feature变化传播，无法判断则保守重验。相邻Rust样例的可复用部分是所有权/接口/测试驱动，不是重新复制算法。

若原binary、oracle/harness和fixtures均不变，可把完整原RET输出缓存作为原端证据，后续只重新执行受影响Rust候选再比较；缓存必须包含原码实际返回/范围、active-output schema和工具边界，不能缓存host代算结果或掩盖未到RET。第一次原端失败/输入域变化仍须实际原码取证。该优化仅为后续试点方案，本轮不实施。

全workspace构建一次/批次；同candidate执行受影响证明一次，新增失败/修复再重跑相关闭包。全套仅在多包里程碑、共享依赖/调度大改、C迁移或最终验收时跑。受影响范围不能可靠判定时继续全套，不以速度为由缩小验收。

## 7. 完整建筑落地的先后里程碑

1. **组织与保存：** 采纳单写owner/短handoff/metadata队列，整理当前所有 `implemented_unverified` 及未注册草稿，父一次对齐Lock、aliases、register/tuple/stage和native-ID single-writer。当前阶段不运行验证。
2. **关闭最近三条CPU链：** 附着source/anchor与矩形cancel→开口/历史；Gate/Fence输出→实体/Nani CPU释放；碰撞/DirtyTiles→几何订阅重建。unknown底部门/特殊Dst、17子世界等保明确子域，不把普通分支当整包。启用后续验收时先L1，再每包L2/L3/L4。
3. **补源头与所有权：** 实际UI placement/edit/remove/取消、ID/颜色/metadata/global startup生产者、装饰及平台历史owner；推进G15 snapshot/save/load恢复真实Source而非仅导入观察DTO。资源/档案加载使用原fork依赖，不复制parser。把永久resolved输入缩成有来源的caller边界。
4. **C-first补建筑族与输出消费者：** 新G06 floor/墙面族，G08屋顶trigger/尖顶/梁，G11 roof/terrain/stair/corner/arrow等装饰，G12玩家楼梯，G13材质/半木/灯光优先沿C自己的producer/caller恢复；已有B成果直接作为算法和接口参考，只有必要兼容/比较分支再补B。按“完整族+增改删+历史”领取，不逐marker或render函数统计完成。
5. **原图与已有B链C适配穿插、按包验收：** G16实际注册/事件/任务/取消消费者在每个包中采集；现有B包对应C迁移用C自己的enum/依赖/原碼证明，不等所有B完工才开始，也不从B源码checksum推C等价。C Message/fork边界单独恢复，不能登记B runtime就称C工程已接。A只对声明兼容范围取证。
6. **最终C建筑闭合：** G18真实实例上传/active-handle/释放、G15持久事务、G19跨模块/场景对照及未解决数值失败。GPU后置但必做；原Bevy fork只恢复影响游戏业务的必要接口，不从零重建引擎。当前ARM屋顶1ULP差异继续保留，不放宽精度消除失败。全部包仍无完整system acceptance，最终不得由case数/函数数合成覆盖率。

## 8. 可测指标与小试点

不要预报提速倍数或“90%恢复”。选下一轮三个相邻完整工作包试点，记录起止时间、blocked原因和实际消费者交付；在同类包之间比较，任务难度与原码输入域必须同时列出。

| 指标 | 记录方法 | 用来做什么 |
| --- | --- | --- |
| metadata排队时间/返回次数/缓存命中 | request queued/ready时间、同key重复次数、返回partial/gap原因 | 判断批导出和负结果缓存是否减少等待，而非只增加文件数 |
| API往返次数与共享文件触碰 | 每包contract_revision、登记请求次数、父shared edit次数 | 检查契约批次是否减少lib/CLI零散往返 |
| agent可推进/阻塞时间 | 每包handoff日志按metadata/API/资源/验证分类 | 判断父瓶颈与其他可独立工作，不用token量当产出 |
| 证明选择与实际用时 | 受影响/执行/跳过任务集合、理由、每task wall秒；全套另记wall时间 | 试点局部闭包是否安全减少重执行；并发task秒之和不能当总耗时 |
| 冲突与生命周期缺陷 | 文本冲突、双writer/错顺序/资源残留/失败队列缺陷分别计数 | 比较隔离质量，要求每个“resolved”有review/ref，不把少报当改善 |
| 完整链交付 | 每包真实source及最后consumer、no-change/error/delete/cancel/undo/redo哪些已接/已验 | 防止大量leaf/probe/marker替代完整建筑进展 |

试点成功标准是可核查的：元数据和共享写入能追到唯一owner；同一consumer不会重复注册或双writer；proof manifest能解释每项执行/跳过；模块交付包含生命周期并保持原失败证据。若出现遗漏依赖导致错误复用，退回保守全套并修manifest；若批请求等待显著阻碍链，缩小batch而不是增加无上下文agent。试点后由父根据日志选择流程，不提前宣称效率收益。
