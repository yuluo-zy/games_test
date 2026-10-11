# PDB → 证据数据库 → 类型化反编译 → 原模块 Rust 骨架：自动化流水线方案

日期：2026-10-11。此文是深入方案，未实现提取器、数据库或常驻 worker；未修改业务 Rust、Cargo、共享台账、skill、AGENTS；未构建、运行原指令或执行业务验证。只读小样本与本地 API 检查见 [automation-feasibility-sample.json](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/reconstruction-strategy-2026-10-11/automation-feasibility-sample.json)。

## 结论与最低可行选择

最优先建设的是完整的 **procedure / source line / inline / dependency ownership 证据索引和单个常驻反编译 worker**。三版 PDB 足以显著改善原模块映射和代码归属；不足以自动取回游戏 Rust 参数、局部变量、内存布局或作者源码。类型回填必须结合已证的原指令读写和调用合同。

最低可行工具组合：本机已有 LLVM 22.1.2 `llvm-pdbutil`、Python 标准库 `sqlite3`、本机 Ghidra 12.1.4（现有导出脚本使用 JDK26，不能直接切换到不兼容的旧字节码运行时）、现有 Java 导出脚本。第一批无需安装新的框架或引入 Windows DIA。首批直接复用有限流 YAML/目标 module dump，把小函数包写 compact JSON，再逐包入 SQLite；不开发新的全库 PDB reader。完整规模的薄 LLVM Native PDB reader 只作为后续小试点，先证明一个函数的 inline/C13 → SQLite 和成本收益再决定扩大。本轮仅确认 API 存在，未编译 reader。

现成 CLI 的 `pdb2yaml` 有 TPI/IPI、DBI、module symbols、文件校验、行号和 inlinee 子节选项；没有 `--modi`，不能把整 PDB 的巨量 YAML 当作每次局部查询。`dump --modi=N --symbols --files --il -l` 可做少量目标核查，不能继续用宽泛正则把整库 pretty text 作为唯一结构化来源。B 的 `pdb2yaml --pdb-stream --no-file-headers` 本轮成功，182 字节 YAML 中 GUID/Age 与身份索引一致。

## 1. 已确认的能力与真正缺失

与 PDB 审计 agent 联合核对的完整结果见 [pdb-capability-audit.md](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/reconstruction-strategy-2026-10-11/pdb-capability-audit.md)。以下不是依据一个模块作的推断：

| 数据 | A | B | C | 流水线用途 |
|---|---:|---:|---:|---|
| 全 DBI module | 4709 | 4894 | 5142 | 编译单元与源文件关系 |
| Rust rcgu module | 4560 | 4739 | 4986 | 与 C/C++ 阳性对照分开 |
| TPI record | 6133 | 6125 | 6126 | 所查游戏类型命中为零；库类型仍有价值 |
| IPI record | 119594 | 132599 | 144737 | 作用域、函数身份、构建信息 |
| Rust S_INLINESITE | 2333877 | 3204897 | 3966178 | 物理函数内的逻辑源码与依赖归属 |
| Rust S_LOCAL / REGREL / BPREL / DEFRANGE | 全零 | 全零 | 全零 | 不存在可直接批量导入的 Rust 命名变量/活跃位置 |

真实建筑普通函数和 fence `process_segment` 样本的 TPI 为 `0x1001 void()`、空参数。帧大小、保存寄存器、RBP frame 属性确实存在，但它们不提供栈槽变量名称、参数类型或结构字段。三版都不能相互提供缺失的完整 Rust locals。C/C++ 模块有真实参数、局部变量、TPI 和 defrange，证明 LLVM 能读取这些记录，也帮助依赖边界解释。

可自动提取：定义符号、模块、过程范围、源路径/校验/行表、内联 ID/父子关系/范围、IPI 名字、已有真实类型图、公开依赖来源。不能自动补造：游戏参数、捕获环境成员、Rust enum niche、字段名、默认值、业务 ID、调度边、作者 Cargo features。发现某名字不在 TPI 不等于其原源码不存在。

## 2. 现有脚本复用与需替换的部分

| 本地已有资产 | 保留价值 | 需改进 |
|---|---|---|
| [index_baselines.py](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/scripts/multi-version/index_baselines.py) | PE/RSDS 身份、三版本源码校验、Cargo source/rev/checksum | 目前函数索引仅五个游戏前缀 publics：A2261/B2949/C3500；不覆盖完整 LPROC、inline、私有 closure 与泛型定义归属。不能用其分母统计完整游戏 |
| [pdb_focus.py](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/scripts/binary_baseline/pdb_focus.py) | 目标 S_GPROC/LPROC 和直接 call 样本 | 硬编码 `.text` 地址换算、Windows 路径、部分 aliases 截断只能保留为历史局部证据；正式索引保存所有别名并按 PE section 映射 |
| [ExportFocusedFunctions.java](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/scripts/multi-version/ExportFocusedFunctions.java) | 已在单脚本中只 `openProgram` 一次，复用 `DecompInterface` 导出多函数 | 当前每批仍启动 headless；缺 exact function 会整批抛出。改成队列常驻、每请求异常隔离、逐项 checkpoint、清晰 missing/timeout 状态 |
| [DescribeFocusedDataTypes.java](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/scripts/multi-version/DescribeFocusedDataTypes.java) | 只读 DTM 类型/字段清单 | 增加类型来源、证据等级、版本与导入计划；现有 DTM 类型名不能当作 PDB 原生布局证明 |
| [mcp-native.mjs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/scripts/multi-version/mcp-native.mjs) | 既有 stdio MCP 的 job/call/catalog 落盘流程 | 可作为可选控制面。当前会话没有直接暴露 ReVa scripting API；不能把技能文档中的 API 当作已连通服务 |
| `tracking/baselines.json`、`version-map.json`、`dependency-exclusions.json`、`module-coverage.json` | 既有身份、跨版候选、依赖筛除、原模块清单 | 原样导入且注明旧 scope，不为新索引覆盖旧 ledger 或把候选升级为等价 |

尚未发现独立、完整的作者目录 Rust 骨架生成器。现有 `reconstruction` 是恢复源码工程；新生成物应在单独 skeleton 路径与其映射，不能直接覆盖它。

## 3. 一次读取 PDB 的抽取设计

### 首批：三个业务 agent 可直接领取的最小函数包

先复用既有样本/现CLI，按需要追加限定module；不等待百万inline入库，不写昂贵Rust类型反序列化器。老Windows `pdb_structure.py/pdb_focus.py`是文本正则脚本，不是可直接复用的PDB原始parser。首批新增的compact parser仅处理已观察的procedure/scope/line/inline行，并保存完整raw片段及unparsed状态；无法解释的annotation不强行转PC范围。

一次函数包交付给业务agent的最小内容：artifact身份与original/analysis边界；原logical名字和全部已知aliases；真实procedure(module,recordoffset,section,offset,size)；定义source校验/行范围；当前函数的inline父子与原annotations；直接callee/caller引用（有原ASM才给）；Cargo/exclusions归属理由；已证ABI/字段facts与unknown；现有Rust映射；相关原C/ASM路径或一次queue导出请求。一个包最多一个owner+必要直接helpers，完整资料留文件，摘要不超过所需字段。

| 业务agent包 | 最先交付 | 工作不被缺类型阻塞的边界 | 必须保留的缺口 |
|---|---|---|---|
| Wall producer/geometry | `construct_walls` + private `process_segment`源/inline包，A371/B394/C403与A1246/B1454/C1505 | 区分游戏规则和内联容器，再读已有caller/input facts | source/default与schema不从void()推；没证字段继续required gap |
| Roof/History | RoofCreate A3196/B3467/C3778，加当前history producer/consumer的PDB module定位与已有原C/ASM | 查original owner、创建/记录与回放独立路径，避免自动假record | mixedchannel排序、cancel/restoreID、未知history owner不造 |
| Decorator/Stairs/Collision | 先从已保存 `decorator-stairs-module3088.txt` 与当前consumer evidence给B函数包，再按A/C真实module定位匹配 | 原业务query/filter与Rapier/Bevy库实例区分；捕获env读写facts驱动 | B3088不能套到A/C；缺placement资源/真实调度保持pending |

首批退出：三个包都有正确artifact/procedure/source/rawinline/unknown清单，业务agent能直接打开相关owner和callee；不要求全库导入或完整Rust类型。示范包见同目录 `automation-function-pack-sample.json`（只读复用既有B RoofCreate证据，尚不是新的decompile/业务验收）。worker随后按包接所需有限函数，父无需逐地址代转。

### 扩展试点：LLVM Native 原始记录 → JSONL

本机 LLVM include 已确认以下入口，不需要反复执行一个函数一个 `llvm-pdbutil`：

- `NativeSession::createFromPdbPath` → `getPDBFile()`。
- `PDBFile::getPDBDbiStream/getPDBTpiStream/getPDBIpiStream`。
- `DbiStream::modules()`、`DbiModuleList::getModuleCount/getModuleDescriptor/source_files`。
- `TpiStream::types()` 同时服务 TPI/IPI；通过 CodeView record kind 分流。
- `ModuleDebugStreamRef::symbols/getSymbolArray/getSymbolsSubstream/readSymbolAtOffset/subsections`。
- `CodeViewYAML::SymbolRecord::fromCodeViewSymbol` 提供现成序列化参考；正式 JSON 还需额外保存 module、record offset 和原始 payload。

具体本地头文件根：`/opt/homebrew/opt/llvm/include/llvm/DebugInfo/PDB/Native`；CodeView 反序列化/visitor 直接复用 LLVM，不手写压缩 inline binary annotations 解码器。读取实现须处理 `Expected/Error`，把未知 leaf、损坏 record、未支持 subsection 记为 raw+gap，而非丢弃或错位继续解释。

单 PDB reader 一次打开文件，按 stream/module 分片：

1. MSF/Info/DBI 身份、模块 descriptor、section headers/contributions/map；OMAP 如存在必须登记并参与地址映射，如不存在登记明确 absence。
2. TPI：type index、kind、引用边、forward definition、field-list continuation、已知大小/字段；循环图分两遍建索引再解析，不递归无限展开。
3. IPI：FUNC_ID/MFUNC_ID、STRING_ID 作用域、BUILDINFO、UDT source line；这些 ID 与 TPI type index 分属 namespace。
4. Globals/publics：完整符号列表及名字别名；public 地址不是唯一函数/原型。
5. 各 module symbols：GPROC/LPROC/THUNK、scope END/parent 链、FRAMEPROC、INLINESITE、LOCAL/REGISTER/BPREL/REGREL、各类 DEFRANGE 等实际记录。
6. C13：文件校验→源文件 ID、LINES、INLINEELINES、跨模块 import/export、frame/unknown subsection。inlinee 起始源码行与 INLINESITE binary annotations 共同生成 PC/line 区间；只有 inlinee lines 不足以证明完整机器码范围。
7. 源文本/embedded stream另行清点；PDB 中的路径、SHA-256、natvis、`/names` 不等于作者 Rust 文本已恢复。

每 module 原子完成一次分片和 summary，记录解析游标/数量/error；写入线程一个，批次提交可从 1–10k records 开始实测。三版约 950 万 Rust inline sites，不能全转成 Python dict 留内存或全文送给 agent。先产出建筑链的目标 module、再增量覆盖全库；完整库不是继续已证业务的前置阻碍。

### 不先开发 reader 时的低成本入口

按需要用 `pdb2yaml --pdb-stream/--dbi-stream/--tpi-stream/--ipi-stream` 保存有限流与元数据；已有类型/ID结果不重抽，模块清单/目标 module 用现有 CLI 限定采样。YAML 流单次解析并入库，不在对话中反复 `rg` 大型 TPI/IPI。正式 symbols/inline 大规模导出转 Native reader；不能假设 YAML 已保存完整 record offset 或未知记录，须用原始 stream 对照确认。

CLI pretty `dump` 定位用，不作为长期 parser contract。已知 LLVM 同时开启 `--type-stats` 和 `--id-stats` 时只给一种统计，能力审计分次执行。reader 改版需写 parser schema/version，不与旧文本 parser 的结果静默混合。

## 4. 结构化证据数据库

建议 SQLite 一个 evidence DB，raw JSONL 按 artifact/module 存文件。数据库通过 Python 标准库可用；不新增通用检索框架。单个 writer 用 WAL，agent 只读查询或投请求。record 的稳定主键为 `(pdb_sha256, stream_kind, module_index?, record_offset/type_index)`，禁止仅用 VA、名字或 type index 跨 PDB 合并。

| 表组 | 必须保留的字段/关系 |
|---|---|
| `artifacts / streams / modules` | release、binary/PDB hash、RSDS GUID/age、PE sections、source binary 与 analysis DB hash、补丁/overlay、tool/schema；module stream/obj/compile 属性 |
| `source_files / checksums / module_files` | 原路径、规范路径、校验算法/bytes、module file id；原路径与 normalization分开，不丢 drive/大小写信息 |
| `types / type_edges / fields` | TPI/IPI namespace、index、raw kind/payload、forward ref、精确字段offset/size、证据等级；unknown/pointer/address-only不能偷偷补字段 |
| `symbols / procedures / aliases` | 名称原文、demangled树、mangled hash、section/offset、size、record scope/type ref；physical body 与逻辑 symbol 分离 |
| `line_ranges / inline_sites / scope_edges` | procedure、inline parent、inlinee IPI、PC intervals、file checksum/line/column（有则存）、annotations原文；按 address range 查询 |
| `locals / location_ranges` | module/scope/local id、param flag、type、piece/storage/register/frame offset、valid PC/gaps；Rust明确零记录也入 capability |
| `call_edges / field_facts / owner_contracts` | 原 instruction/callsite/hash、read/write offset/width、caller ABI、业务输入来源、producer/consumer；静态 call 不假作实际顺序 |
| `package_apis / dependency_matches` | Cargo source/rev/checksum、定义源路径、公开 API、匹配理由、unknown/fork edge；实例所在 module不是定义 owner |
| `claims / gaps / cross_version_candidates` | assertion、范围、支持/冲突证据、级别、candidate score、未证等价；negative/unknown同样可检索 |
| `decompiler_jobs / runs / skeleton_items` | 请求、缓存 key、overlay版本、输出/失败、来源映射、生成状态；生成骨架与生产Rust分离 |

所有外键绑定 artifact；每条事实保留来源 URI+record offset 或文件行号。采用明确状态 `pdb_exact / native_observed / inferred_candidate / opaque / missing / conflicting`；禁止用一个 boolean `recovered=true` 混淆名字恢复、结构恢复和行为实现。

地址存 section+offset、RVA、image base 映射来源；内联范围/函数 body 是 range set，不只一个连续 `[entry, entry+size)`。PDB code_size、Ghidra body、unwind/cold块分别保留，冲突生成 gap。库跨层包装可能在游戏 module 内，不能按 module 全部标业务。

### 给 agent 的最小 API

`function(release,rva)`、`symbols(name,owner,limit)`、`owners_at_pc(release,rva)`、`source_functions(path,checksum)`、`type(artifact,namespace,index)`、`calls(function,direction)`、`field_facts(type_or_base,offset)`、`locations(scope,pc)`、`dependency_api(definition)`、`cross_version_candidates(symbol)`、`gaps(owner)`、`decomp_status(job_id)`。

API 返回带证据位置的小摘要/分页，不自动返回整个 C、TPI 图、百万内联记录。请求可指定需要的基本块、字段、caller 或 inline owner。缺类型必须返回 null+reason，不猜 zero/default。当前工程模块映射可作为另一张 `reconstruction_mapping`，显式区别 original owner 和 adapter owner。

## 5. 单个常驻 Ghidra worker 与版本化类型导入

父 agent 建议的 queue worker 是当前最高收益可实现项。先用 Java GhidraScript 保持一个 headless process/program/decompiler，后续有实际 ReVa/PyGhidra连接才换控制面，不把每个函数重新 `analyzeHeadless`。

建议队列：`requests/*.tmp` 写完原子 rename 为 `.json`，包含 request id、release/artifact/program id、functions、type-overlay id、输出根和预算；worker lease 后串行消费，结果 `.json.tmp→.json` 原子发布。Agent 轮询 ready manifest而非父 agent 逐地址转发。重试复用相同 request id/cache key；取消是请求状态，不删已导出证据。全请求必须隔离路径，禁止任意 shared DB 修改请求。

一个 program 一个 writer；一请求可包含 50–250 个关联函数的初始预算，实际大小按可观测用时与失败调整，不能把它当已有性能结论。每个函数都记录 missing/timeout/error/cancelled/completed，某个失败不终止其他项目。单项90秒当前脚本预算可保留为首试配置，长函数独立重试；timer/心跳、request checkpoint 与 worker restart 防止掉线整批丢失。request 自身输出摘要，完整 C/ASM/Pcode/HighFunction/field refs 落盘。

### 只写 analysis clone，不写工作共享数据库

现 B 的 Ghidra DB 来自 patched executable，原 execution image 是 RNE；不同 hash不能消失。先明确每个导出绑定 `original_binary_sha / analysis_binary_sha / analysis_db_revision / overlay_sha`。不能把 patched CFG当原始 CFG。可用原 RNE 导入独立 analysis program，或保留 patched DB 提供名字/类型 overlay，再对目标原始 byte ranges/boundaries 单独确认。首次原始分析按 artifact 完成一次，后续复用。**项目没有 Git；隔离不依赖 git worktree，用独占目录、DB clone、patch artifact及父统一应用共享接口。**

类型导入是后续主动修改 analysis clone 的步骤，本轮未执行。每次事务只作用于 clone：先生成 `type_import_plan.json`，包含原字段证据与冲突；known struct 有明确总大小时使用精确offset/packing，只有局部字段时保存部分 view 或注释，不伪造原完整 struct。匿名/opaque、typedef、pointer分开；同名不同版本使用版本namespace。privateRust enum/niche无证不得套用C enum。

已检查安装包源 API：`Function.updateFunction` 支持 `FunctionUpdateType.CUSTOM_STORAGE`；`HighFunctionDBUtil.updateDBVariable/commitLocalNamesToDatabase` 可写变量；`DecompInterface.openProgram/decompileFunction/flushCache/dispose`可复用。调用技术存在不证明待写数据正确。

- 有精确参数/存储证据才能用 custom storage，特别隐藏返回指针、拆分参数、捕获env不能按统一Windows CABI猜。
- 不把 `void()` 占位强制写入，不能宣称游戏自动参数恢复成功。已证游戏合同从 native fact table 导入，保持业务名称和原作者名称的区别。
- C/C++ locals 可按 scope+valid PC+defrange映射；寄存器被优化复用时，一个register有多个不重叠变量。不能全函数把R14命名成一个业务对象；piecewise/composite/optimized-out无法安全绑定则保留注释/sidecar。
- 对HighSymbol映射还要核对 storage 和 use-point；精确局部不匹配当前高层SSA时不强制update。缓存失效与 `flushCache` 在类型/参数变化后明确执行。

首次只测计划中样本的类型/原型精度，未经该步骤验收不把 type overlay 全库推广。补丁化名字/类型导入可回滚，原raw PDB事实保持只读。

## 6. 依赖 API 匹配与私有 closure 处理

依赖匹配使用 Cargo.lock source/git rev/checksum + PDB `.cargo/registry`、git checkout、rustc commit路径 + 定义符号/inline source。源码同checksum可以复用源身份，不构成native行为一致。无source的local包按已有 exclusions核对上游；未知fork差异保持候选。

`Vec<GameType>`、Bevy Query/FunctionSystem、Serde/rkyv实例名含游戏类型不等于游戏实现；生成的 trait shim/drop/alloc/iterator adapter保留调用图边，排除业务覆盖。相反在游戏 closure内的 private选择、调度/输入关系、阈值是业务；不能只因为跨三版共享就排除。

Closure自动化处理顺序：

1. 收齐 LPROC/GPROC/Public/IPI 的原名，拆分作用域与泛型参数、`closure_env$N`/trait shim，保留原串以便复核。
2. 用 module definition+line+inline parent确定closure逻辑 owner；某module引用一个源码路径不能证明它定义closure。
3. 通过caller/env构造与load/store建立capture offset/width事实。只有已证capture才命名；无法证明成员、捕获方式/lifetime时输出 opaque capture gap。
4. 在physical function内利用inline PC区间切分业务片段与依赖实例；inline代码可能交错，不能机械剪下连续字节等同独立函数。
5. 同地址多别名/ICF保留 `physical_function_id→many logical_symbols`；同VA不意味着同业务。调用点泛型实例和capture合同可能不同，不能自动选一个游戏名字覆盖所有别名。
6. trait shim保留边界并尽量调用原依赖；不从反编译展开的容器代码生成新的Rust容器。

IPI大量字符串适合作用域/系统参数类别解析；不适合从名字里推 enum大小或capture布局。C版本新增 label可帮助CFG命名，不能转成局部变量。

## 7. 原模块 Rust 骨架与现工程的接合

生成目录树依据 private source相对路径/模块关系：例如country-core、systems/wall-constructor、systems/roof、systems/decorator，保留A/B/C存在性与checksum版本。源路径不是源码文本，缺inline/body也不补造作者签名。

骨架先产出 `module_manifest.json` 和 `.rs.in` 注释/待恢复项；未知ABI不生成伪零参数 `fn`，未知layout不生成零大小/Default struct。已证的public合同才可转为独立诊断crate中的真实 Rust声明。诊断骨架不得自动导出到生产工程或更新ledger完成状态。

每项带 `original_symbol_ids / source_checksum / native_ranges / owner / dependency_class / field_facts / typed_gaps / reconstruction_path / adapter_boundary`。原作者局部名不可得时使用明确的恢复名，不能宣称原作者文本。原源路径一个函数可能对应多个monomorph/inline instance，骨架是一逻辑业务入口，不按实例数重复写实现。

Cargo.lock仅帮助锁依赖，不足以复原作者Cargo.toml features、patches、target cfg/私有workspace关系。生成候选依赖表而不覆盖现有Cargo；父agent统一review共享注册/接口的patch artifact。把原模块mapping作为推进完整输入→状态→几何→consumer→删除/历史链的导航，不把骨架数量当完成率。

## 8. 三版合并、缓存和增量

跨版匹配分层：源路径规范化+checksum→logical owner；Rust symbol scope/generic identity→候选；line/inline结构与邻接call/CFG/bytes→进一步候选；业务差异由实际合同与后续必要原码验收确认。全部保留版本独立的类型index/moduleindex/RVA/field facts。一个名字或相同sourcechecksum不能把B字段直接贴到C。

抽取cache key：PDB hash+LLVM/parser/schema versions+抽取scope。反编译cache key：original目标byte-range hash+analysis DB身份/CFG revision+Ghidra language/compiler spec/version+函数signature/typeoverlay+decompiler options。地址、文件名或原二进制hash单独都不足够。

名字/类型overlay变更：失效当前function及其实际依赖closure/caller范围；未变函数可复用，但必须引用其旧输入key。全工具版本或parser schema变更只重跑对应层；不能通过修改reporthash宣称旧输出新验收。保留失败结果，不循环重开相同超时function。

跨agent并行：PDB只读抽取、数据库查询、独占业务源码可并行；同一DB writer/类型overlay串行，共享Rust lib/stage/Cargo由父统一apply。每agent原子投请求；唯一请求输出目录；record parser与owner业务互不改对方文件。缺库类型数据不阻塞有明确 native合同的pure业务，但缺原权威输入/默认/删除传播仍是业务gap。

## 9. 分阶段交付与退出条件

| 阶段 | 交付 | 退出条件 | 未关闭时怎么办 |
|---|---|---|---|
| P0 能力/身份 | 三版capability+tool/API sample+patched DB差异清单 | GUID/age/hash绑定；Rust零locals与C阳性对照分开；占位signature明确 | 身份不一致停止该artifact；不是停止其他版本 |
| P1 最小数据库 | 现CLI目标函数compactJSON、SQLite schema+query、已有TPI/IPI有限索引；Native reader仅1函数试点 | record offsets/raw/error可追踪；目标procedure/line/inline/ref关系可查；计数与CLI样本吻合 | 未支持record保raw；不得静默忽略 |
| P2 常驻导出 | 单worker队列/cache/checkpoint、原binaryanalysis clone | 同process多请求；missing/timeout不丢后续；typeoverlay分离；artifact每项可追溯 | 先只读batch，不假类型；MCP未接通用Javaworker |
| P3 类型与owner | exactC库样本导入、已有游戏nativefield/ABI overlay、dependency API索引 | 精确字段无冲突；native观察与PDBexact区分；占位/缺locals不被补造 | 保opaque/sidecar并继续caller/dataflow研究 |
| P4 原模块骨架 | source/module/closure manifest、诊断`.rs.in`、现Rust映射 | 依赖/私有owner区分；每待恢复项有证据和gap；无伪Default | 原型缺失仅注释骨架，不转生产 |
| P5 业务恢复使用 | 具体owner读写/producer/consumer清单+必要适配patch | 父统一接输入/发布/consumer/删除/历史；之后用户允许时正式验收 | 索引/骨架不是业务通过；后续验收独立 |

本轮实际仅P0范围的检查与方案，没有执行P1–P5。下一实现批先交三个最小函数包与P2常驻导出；P1只收包内需要的数据。Native reader和完整全库索引不能成为任何业务agent开工前置，若试点成本高立即维持现CLI有限包路线。

## 10. 第一批采样与失败处理

先取每版6类样本，共18个逻辑目标：真实建筑普通owner（construct_walls）、RoofCreate、history业务owner、真实privateprocess_segment/closure、同地址泛型别名、C/C++有locals阳性对照。使用审计已经确定的module anchors，避免仅凭路径首次命中：

| 样本 | A module | B module | C module |
|---|---:|---:|---:|
| construct_walls 源范围 | 371 | 394 | 403 |
| RoofCreate | 3196 | 3467 | 3778 |
| 真正 fence process_segment owner | 1246 | 1454 | 1505 |
| locals阳性对照 | 675（std_type_info） | 1647（lcms cmsxform） | 1743（lcms cmsxform） |

history/ICF样本用完整procedure+aliases查询后选，不套用Bmodule数字到A/C。先复用同目录capability已保存样本，避免本轮再次全PDBdump。native overlay样本选择当前已证Wall发布/height/Roof consumer合同；缺类型样本必须证明工具不会猜填。大函数超时、无exactentry、截断body、inline范围冲突各保一个负样本。

采样验收针对工具数据完整性与明确范围，不执行游戏oracle；后续业务验证应在用户重新允许验证后另行安排。记录missing与firstfailure；不得删失败目标、跳过调用后宣称全coverage。未支持record可定位到原stream；未知PC范围禁止硬贴变量。

## 11. 建议目录与所有权

以下是未来目录提案，目前未创建数据库/worker或修改这些文件：

```text
metadata/
  artifacts/<pdb-sha>/identity.json
  artifacts/<pdb-sha>/streams/                  # 原stream/有限YAML或压缩输出
  artifacts/<pdb-sha>/records/module-N.jsonl    # offset-preserving分片
  evidence.sqlite
  manifests/schema.json
  api/                                         # bounded query service
analysis/
  A/<binary-sha>/original-program/             # 独立持久DB
  B/<binary-sha>/original-program/
  C/<binary-sha>/original-program/
  overlays/<release>/<overlay-sha>/            # 类型/名字/CFG可追溯修改
  queue/{requests,leases,ready,failed,cancel}/
  exports/<release>/<cache-key>/               # C/ASM/Pcode/structured sidecars
skeleton/
  manifests/original-modules.json
  A/crates/.../*.rs.in
  B/crates/.../*.rs.in
  C/crates/.../*.rs.in
  reconstruction-mapping.json
```

父负责schema/version/worker/shared注册；metadata worker独占索引写入，业务agents只读API+单独owned路径。现有evidence与tracking保持历史可引用，无全目录搬迁。token控制融在工具接口：一次抽取、带hash缓存、范围查询、少量摘要、失败不重复dump、完整材料在本地文件。节省token不能以舍弃别名、inline scope、未知record或必要权威输入证据为代价。

## 12. 能承诺的自动化边界

可以自动化 metadata抽取、原module/line/inline定位、依赖筛除候选、已证类型overlay、批量decompile、缓存与骨架映射。无法从这三版PDB一键生成完整可运行作者Rust工程。缺失布局/ABI要靠原调用和字段读写，缺失业务输入、ECS真实调度、删除、历史和资源owner要靠实际producer/consumer恢复。最终推进依据仍是完整游戏业务链及其已允许的验收，不能用“几百万符号已入库”替代完成建筑系统。
