# Tiny Glade 独立 PDB 源码骨架

本次处理范围已扩大并核对为**整个现有 A/B/C 项目**，包含主项目、后来恢复的 rhapsody2/shabriri、生成源码、引用元数据、未知来源文件与 manifest stub。A/B/C 当前分别 1,070 / 1,159 / 1,438 个 `.rs` 文件，共 3,667 个，均有结构检查区段；不再只按最初 3,057 个主源码路径统计。C 新增识别 373 个此前因为 package 不在顶层 `crates/` 而遗漏的 PDB 命名实体。

逐项目/模块/文件范围见 [WHOLE_PROJECT_COVERAGE.md](WHOLE_PROJECT_COVERAGE.md) 和 `pdb-whole-project-coverage.json`，全目录核对见 `pdb-whole-project-verification.json`。`WHOLE PROJECT EVIDENCE V1` 区段及同名 `.rs.project-evidence.json.gz` 补入非屋顶原指令局部布局、历史/资源 JSON schema 与已有证据关联；PDB 精确定义仍位于 `PDB STRUCTURES V1`。这两个区段区分原指令字段、序列化数据形态与仅供查阅的来源关联。没有证据的文件明确记为 unknown；没有把遍历文件数当作完整游戏恢复率。

上一批 PDB/TPI 扫描已覆盖三版主项目，但手选原指令字段偏重屋顶/建筑。本次补齐漏掉的附加包和所有现有文件，并补入 B 地形、装饰、相机及各历史命令/资源 schema，C 羊抚摸状态等非屋顶证据。已有 B 资源的 604 条 schema 观察只填入 B，对 A/C 保持版本未知；A/C 的字段只接受各自证据。

本批增强信息已合并到本目录原有的 `A/`、`B/`、`C/` 项目文件中，以这里为交付入口。现有基础 PDB 记录和依赖注释保留，参数类型候选、系统接口、类型关系、闭包与引用方位于各文件的 `PDB ENRICHMENT V1` 区段及同名附档。共享接口和地址别名索引在 `_pdb/<版本>/enrichment/`。合并记录见 `enrichment-merge-report.json`，核对结果见 `pdb-enrichment-verification.json`。

此目录与 `reconstruction/` 隔离。A/B/C 不相互合并。源文件路径来自既有 PDB 源路径和校验和清单；源码骨架排除已确认的公共依赖及生成文件，完整 PDB 元数据仍归档保留。

三版本批量导出和独立核对已完成：3,057 个源文件、35,323 个按文件归属的函数符号/类型条目、20,871,649 条模块符号记录。模块与类型的每种记录数量均与此前独立审计一致；原始 TPI/IPI 记录长度与种类及屋顶目标模块过程/内联记录抽查通过。结果见 `pdb-restoration-report.json` 与 `pdb-verification.json`。这些计数包含编译实例、内联与库调试信息，不代表同样数量的独立游戏函数或恢复完成率。

## 数据结构、枚举及原指令字段补充

结构已直接写入本目录原有 A/B/C 项目的源文件，位于 `PDB STRUCTURES V1` 区段。同名 `.rs.structures.json.gz` 保存逐项完整定义、字段、枚举值、类型引用图、名称引用及证据；文件内的注释逐条展示可确认字段和关系。没有可归属记录的源文件明确标注 `unknown`。

三版分别包含 523 / 523 / 524 条非前向 UDT 定义记录，386 / 386 / 387 条前向记录；共 6,606 条结构字段条目、1,835 条枚举值条目。数字按版本和类型记录计数，可能包含同名或共享字段表的 C/C++ 依赖类型，不是独立游戏结构数。全部 18,384 条 TPI 记录均展开到对应文件或明确的未归属文件；每个记录的原始字节仍保留。

有 `LF_UDT_SRC_LINE` 时保留真实源路径与行号；前向声明借名称关联到完整定义时标为候选，原声明文件保持未知。外部库/系统头文件的元数据放在各项目现有 `_references/sources/` 分类内，原路径逐项保留；未归属记录位于各项目 `_unmapped/`，不改成猜测的游戏文件。仅名称及泛型符号的游戏命名实体可能是类型、枚举变体或关联符号，不能仅凭名称确定完整定义。结构成员、继承、联合体、嵌套类型、方法、指针、修饰符、参数表及枚举底层类型均保留 PDB 原始关系。

已有逆向证据补入 Roof 三版构造字段、版本对应 Nani subset；B 另补 RoofShapeParams、RoofRidgeDirection、RoofEdit、PublicWallState、RawInnerWall 发布投影、WindowSize、TerrainEdit、动画/瓦片状态及屋顶创建组件关系。原指令观察范围与 PDB 类型定义分开标注，局部字节投影不宣称完整 Rust ABI。PublicWall+0x10 为原 Curve2 点数、+0x30/+0x28c 为原长度；缓存有效标记仅确认单字节 bit 0 测试。未知字段名、未读字节、缺失默认值、完整大小/对齐及 enum repr 保持 `unknown`；B 偏移不迁移到 A/C。派生 self 注册不构成 required-component 边。

共享索引：`_pdb/<版本>/structures/type-definitions.json.gz`、`game-named-symbols.json.gz` 和 `report.json`。总体结果及审计见 `pdb-structures-report.json`、`pdb-structures-verification.json`。批量工具 `_tools/fill_type_structures.py`，独立核对工具 `_tools/verify_type_structures.py`；原 PDB 刷新和接口刷新脚本会保留结构区段。此任务没有生成猜测的可编译 Rust 定义或函数实现，未修改 production `reconstruction/`，未另建交付目录。

## 渲染 Pass 的接口、依赖、调用位置与 Shader

本次信息直接补入原有 A/B/C 文件的 `RENDER PASSES V1` 区段，同名 `.rs.render-pass.json.gz` 保存完整记录。覆盖 A 80、B 83、C 76 个渲染相关文件，包括游戏计算/光栅 Pass、帧入口、辅助上下文和 C 的 render-std；文件数不是独立 Pass 数。原有函数、结构、依赖和其他证据区段保留。

已记录 768 条 PDB 资源接口动作（纹理读写、uniform、命名绑定、temporal 等），626 个唯一 PDB 目标名称对应的静态调用位置。位置按物理父函数内指令地址排列，保留前后位置及目标文件候选；它不能证明跨分支、循环或 GPU 执行先后。同地址折叠目标的所有指令和别名仍在附档及共享索引中，是否调用该 Pass 标为 unknown，未将其列为已确认顺序边。间接目标、未覆盖调用和运行时条件保持未知。

版本对应的 1,086 个 Shader 文件中展开 1,924 个 SPIR-V 模块，保留入口/stage、工作组模式、descriptor set/binding、类型、访问限定、字段偏移及 stage input/output 声明；模块边界均成功解析。这不是完整容器解码、Shader 算法恢复或 GPU 执行证明。读取/写入仅按显式限定及可确认的存储类别记录；未限定的 storage 访问保持 unknown。声明资源的实际使用、运行时句柄和渲染图输出不由声明推断。

共 639 条 Shader 关联记录：A/B 的 412 条仅为文件名/家族候选，实际绑定未知；C 的 227 条由 PDB 类型化入口或 permutation 类型关联到 precomp 头部的 Shader 身份，具体帧的绑定和排列仍未知。记录数含不同宿主/上下文的重复关联，不等于独立 Shader 数。descriptor 名称或 set/binding 相同不自动形成跨 Pass producer/consumer 边；真实资源版本、尺寸/格式、队列所有权、屏障和完整依赖图保持 unknown。

索引直接放在现有 `_pdb/<版本>/`：`render-pass-report.json`、`render-shader-catalog.json.gz`、`render-address-aliases.json.gz` 和 `render-callsite-sequences.json`。总报告 `render-pass-report.json`，核对 `render-pass-verification.json`。批量工具 `_tools/fill_render_passes.py`，核对工具 `_tools/verify_render_passes.py`；`restore_pdb.py` 已保留此区段，其余区段更新器按标记替换而保留外围内容。未新建目录，未修改 `reconstruction/`，未执行游戏或 GPU。

## 文件内容

- `A|B|C/crates/**/*.rs`：对应源文件的完整函数符号名称、泛型/impl/闭包作用域、源码行、TPI/IPI 引用、编译实例地址、签名可用性和类型引用图。内容以 Rust 注释保存，避免把未知 ABI 生成为假的可调用声明。
- 同名 `.rs.pdb.json`：文件级来源、校验和、编译单元及计数。
- 同名 `.rs.pdb.jsonl.gz`：该文件全部可归属的逐条信息，包括行表、内联实例及全部二进制注解操作、物理函数、公开别名、帧/编译上下文和对应的原始符号字节。
- `_pdb/A|B|C/types.jsonl.gz`：完整 TPI/IPI 原始记录、LLVM YAML 解释和结构化字段。
- `_pdb/A|B|C/global-symbols.jsonl.gz`：完整公共/全局共用符号记录，包含原始字节与 LLVM 解释。
- `_pdb/A|B|C/modules/*.jsonl.gz`：所有编译单元的完整符号及 C13 子节原始字节、文件校验和、行号/列号、内联起始行信息；包含依赖、无法归属的符号和未知记录。
- `_pdb/A|B|C/container.txt`：MSF stream/block、命名流、模块、PE sections、section contributions、section map 和 FPO。
- `_pdb/A|B|C/original.pdb`：SHA-256 校验的独立原始 PDB 快照，保留解析器未解释的数据及所有原始流。
- `pdb-restoration-report.json`：导出完成状态、覆盖计数和缺失信息；各版本的 `decode-gaps.json` 保留解析限制。

## 签名边界

三版本 Rust 模块的调试信息没有命名参数/局部变量记录，函数类型普遍为 `0x1001 void()` 占位。该记录不是原始 Rust 无参数签名。函数名、泛型类型字符串和作用域可保留，但缺失的真实参数列表、返回类型、ABI、字段布局、函数体和作者原始文本不能从 PDB 恢复。实际存在的非占位 TPI 类型及完整引用图原样保留。

源码行表重叠代表机器码的源码归属，不自动等于函数定义归属。内联函数按该编译单元的 inlinee file/checksum 关联；没有充分来源的符号保留在全局和模块归档，避免凭名字强行放进某个文件。相同地址的别名保留，不据此推断行为等价。

游戏骨架的 Cargo.toml 仍是空占位文件（C/rhapsody2 的依赖清单已单独重建）：PDB 不提供完整 Cargo features、patches 或 workspace 配置，此骨架不宣称可构建。

## 参数、接口、闭包与引用方的补充

每个 `.rs` 的 `PDB ENRICHMENT V1` 区段补充可读记录；同名 `.rs.enriched.json.gz` 保存完整函数、参数类型候选、系统接口、泛型关系、闭包、源码映射与引用位置，`.rs.enrichment-summary.json` 保存计数。

参数类型来自 PDB 泛型符号中的函数指针拼写，包含非 `void` 返回类型。`Res/ResMut/Query/EventReader/EventWriter/MessageReader/MessageWriter/With/Without/ParamSet` 等只按真实出现的类型记录。文件级同编译单元关联不能证明某个函数的参数；泛型宿主包含游戏内联函数、闭包环境类型引用等更具体的关联仍标为候选。参数名、捕获字段、偏移、真实 ABI 和函数体缺失时写 `unknown`。

引用方分成三类：原始程序解码出的直接 `call` 或跳到其他过程的 `jmp`、PDB 内联宿主、闭包环境类型引用。直接静态转移不代表运行时执行；跳转记录不自动等于已证明的尾调用语义。间接目标和不完整调度关系均保留未知。解码从 PDB 过程入口遍历可确认的分支，遇到未知间接跳转或解码缺口不会猜测后续代码，因此结果不是完整动态调用图。

同地址折叠的符号保留全部别名，不任意选一个函数。完整别名位于 `_pdb/<版本>/enrichment/address-aliases.json.gz`；按引用记录中的 `caller_alias_group_rva` / `target_alias_group_rva` 查询。直接引用的原始指令字节、地址和扫描范围位于同目录 `native-reference-records.jsonl.gz`，并逐条与原始 PE 核对。

缺失函数的来源优先使用身份相容的 PDB 符号化结果；命名空间/方法匹配和唯一源码路径推断保留候选标记。无法判定的记录放入各版本 `_unmapped/module-*.rs`，不宣称恢复作者定义文件。生成源码位于 `generated/`，外部源码参考位于 `_references/sources/`，均为元数据注释，不实现依赖库。

完整候选接口、类型关系和所有包装器记录位于 `_pdb/<版本>/enrichment/candidate-interfaces.json.gz`；文件映射说明位于 `missing-function-mappings.json`。此次补充的状态见根目录 `pdb-enrichment-report.json`，验证见 `pdb-enrichment-verification.json`。早期 `unfilled-functions-report.json` 是补充前审计快照，以本次报告中的未知映射状态为当前结果。

## 文件引入与函数依赖注解

已覆盖全部 3,057 个骨架源文件、35,323 个函数条目，其中 27,715 个条目存在可归属的依赖证据。对 18,313 个原 PDB 过程范围完成了原始二进制的有界指令读取；三版本屋顶创建函数各 26 条调用均与独立 objdump 的地址、字节及直接目标一致。没有证据的条目保持未知，不按文件数推算恢复进度。

每个 `.rs` 的顶部增加了注释形式的 `use` 命名空间候选；每个函数条目下增加了带 `{ ... }` 形式的函数体依赖注解。该注解记录依赖函数名、定义库/源码、可用的锁定版本或 git 来源以及证据位置，不生成缺失的函数体。

关系明确分为 `pdb_inline_direct`（PDB 内联树中的直接子节点）、`pdb_inline_transitive`（经中间节点的传递内联）、`native_direct_call_in_physical_body`（原始机器码物理函数内的直接调用）、`native_indirect_call_unresolved`（间接目标未知）和 `native_external_jump_candidate`（可能的尾部转移，不能直接等同调用）。物理函数可能包含库内联，直接机器码调用不自动代表原始 Rust 源码的直接调用。

同名 `.rs.dependencies.json` 保存按函数归并的依赖；`.rs.dependencies.jsonl.gz` 保存逐实例证据。`_dependencies/A|B|C/native-calls.jsonl.gz` 保存原始二进制的有界 LLVM 解码、调用地址、原始字节和直接目标；`native-input.json` 绑定二进制/PDB 哈希与读取器版本。共享地址的全部符号别名保存在结构化附档中，源码注解显示候选数量，不任意挑选一个名字。

`use` 候选不是原作者导入语句；依赖记录不能证明原始 Cargo features、公开可见性、运行时分派或没有其他调用。没有函数实例/调用证据的条目明确保留未知。结果和核对见 `dependency-annotation-report.json` 与 `dependency-annotation-verification.json`；核对采用 `native-evidence-pack` 的原始字节检查及独立 objdump 样本，未执行游戏。

## 读取与重新导出

用 Python 标准库 `gzip` 与 `json` 顺序读取 `.jsonl.gz`。所有记录的身份绑定版本、PDB SHA-256、模块号及 record offset/type index。子节的 `c13_offset` 相对于模块 C13 区域；函数地址通过对应版本的 PE section 表换算。

导出器使用已有 LLVM 22.1.2 Native PDB API，工具源码在 `_tools/export_pdb.cpp`，批量处理在 `_tools/restore_pdb.py`。执行解释器为项目现有的 `tools/mcp-servers/ghidra-mcp/.venv/bin/python`（包含 PyYAML）。重新导出只接受空文件或带生成标记的文件，手工修改的 `.rs` 不会被覆盖。导出产物及其数量不计作游戏行为恢复或验收。

源码映射已修正为保留全部重叠行表，包括同一起始地址的多个源码文件块。A/B/C 各 1/1/19 个此前遗漏的符号名已归入骨架；此前未写入符号的原因分析见 `C-structure-diagnosis.json`。Git/registry 依赖及 OUT_DIR 生成文件不列入游戏骨架，此类符号仍完整保留在 PDB 归档，不能将其统计为缺失的游戏业务函数。

C 的 rhapsody2 依赖已按用户请求单独补入 `C/rhapsody2/`：85 个仓库源文件、6 个生成文件、2,329 个函数符号/类型条目。保留原 `src/`、`crates/`、`render-std/` 结构；这是明确标注的依赖元数据扩展，不改变前述游戏骨架统计。详情见 `C/rhapsody2/README.md` 与其 `_metadata/verification.json`。

C PDB 记录的 8 个 shabriri.rs 生成文件已全部创建：6 个位于 C/rhapsody2/_generated/，余下 2 个位于 C/_generated/。新增生成文件写入 483 个函数符号条目，且相关文件展开完整可用 TPI/IPI 引用图。C/rhapsody2 的 13 份 Cargo 清单补齐 88 条原锁依赖边、54 个依赖身份；features/build/target 分类仍未恢复，详见其 _metadata/CARGO_RECONSTRUCTION.md。

PounceLight/shabriri 的 C 独立骨架位于 C/shabriri/：9 个 PDB 源文件、228 个函数符号/类型条目，3 个包的 24 条依赖边已按 C 原锁重建。shabriri 主包在 PDB 中没有源路径/编译单元，单独保留于 _unlocated-packages/shabriri，未恢复生成器实现。

## Cargo 模块清单

已补齐 128 个模块的 Cargo.toml，以及 A/B/C 三个版本根清单，共 131 份；C/rhapsody2 保留独立 workspace。B/C 的 92 个模块包版本来自各自原始锁文件，registry 依赖使用精确 =版本号，git 依赖写入完整 commit，已定位的私有模块使用独立骨架内的相对 path。rhapsody2 的 workspace 继承依赖保留原有组织，并在各模块中注释实际版本。

A 没有 Cargo.lock：公开依赖版本仅使用 A 的 PDB 路径证据，直接依赖关系属于注解候选；36 个本地包的原始版本保持未知，不套用 B/C 版本。Cargo 对省略 package.version 的默认值 0.0.0 不作为原作版本。缺失 git URL、完整 commit 或未定位 local/path 模块写入 package.metadata.reconstruction 的待补记录。

共写入 1,726 个可表达的依赖条目；116 个待补条目保留在 TOML 元数据和证据附档中，其中 B/C 未定位条目仍保留原锁文件的精确版本。依赖类别 normal/build/dev、target cfg、features/default-features/optional、原始 aliases 和 edition 无法从锁文件恢复；重建清单用 [dependencies] 表达锁文件依赖的合并清单，edition 2021 / resolver 2 是明确标注的检查设置。

B/C 根目录的 Cargo.lock 是各自原始锁文件的完整副本。四个 workspace 均通过 cargo metadata --no-deps --offline；TOML、全部 B/C 锁定依赖身份/版本/来源、git commit、相对路径和锁文件原样保留的核对通过。检查并未编译游戏。两个 A 模块缺少 PDB 入口文件，_manifest_stub.rs 仅用于 Cargo 元数据检查，不是恢复的作者入口。

每份模块清单附 Cargo.toml.evidence.json；总结果见 cargo-restoration-report.json，核对见 cargo-restoration-verification.json；复现入口为 _tools/restore_cargo_manifests.py 与 _tools/verify_cargo_manifests.py。

后续依赖分析和清单还原按用户要求仅处理 B/C，跳过旧 A 版；现有 A 版文件作为历史资料保留。范围见 `dependency-analysis-scope.json`。


## 函数与原始汇编对应（屋顶模块首批）

已将 A/B/C 屋顶模块的原始汇编注释写回 76 个现有 `.rs` 文件：190 个函数条目建立对应，141 个物理代码范围包含 70,661 条指令，全部逐字节与各版本原始 PE 核对。源行号、内联帧、同地址别名、参数候选接口 ID 和调用方索引一并保留；对应附档为 `.rs.assembly.json.gz`。

尚无编译/内联宿主证据的 15 个函数条目明确标注未知；同名函数在同一宿主多次内联时，各次内联的专属区间仍未知。参数到寄存器/栈的机器 ABI 未确认。此批仅覆盖屋顶模块及所需的内联宿主，不代表全项目或原始源码逻辑恢复。

详情与示例见 [_assembly/roof-v1/README.md](_assembly/roof-v1/README.md)，核对见 [_assembly/roof-v1/verification.json](_assembly/roof-v1/verification.json)。

## Git发布范围：A/B/C代码

A/B/C的3,667个`.rs`文件、Cargo清单与锁、说明文档和代码形式的Shader示例已纳入发布范围，保持三个版本各自原目录。代码中的PDB/字段/接口/原指令注释属于观察骨架，不代表完整可运行游戏实现。共享52+35组有界候选在`task-index/work`中保留单份代码和ABC对应。

同名`.json`/`.gz`旁路元数据、`_pdb`、全量索引和缓存继续保留本地；上述读取/重新导出说明依赖相应本地原作输入，不表示全部原始数据随Git发布。
