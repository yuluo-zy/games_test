# 汇编图数据库：安装、迁移与重建

本目录是可迁移管理入口，分析器仍在 [`reverse-engineering/tiny-glade/tools/code-map`](../../reverse-engineering/tiny-glade/tools/code-map/README.md) 维护。`manage.py` 自动找到工作区，不依赖原电脑用户名。所有命令在工作区根执行；路径含空格时加引号。

交付包括安装与环境检查、输入证据和汇编分片打包、校验后恢复、Neo4j 重建、离线 dump/load、真实链验证。**源码仓库不包含被忽略的原版 PE/PDB、汇编缓存或数据库**；另一台电脑还需要本工具生成的迁移包。包包含本地原版文件和已有证据，按项目已有授权范围转移。

## 1. 环境

| 依赖 | 要求与固定方式 |
|---|---|
| Python | 3.11.8+；核心分析/迁移只用标准库 |
| Neo4j | Community **5.26.32**；官方 Unix tar.gz，SHA-256 见 [`environment.lock.json`](environment.lock.json)；自动校验，不装全局服务 |
| Java | 独立 **Java 21**；用本地配置或环境变量指定，不修改全局 Java |
| LLVM | `llvm-objdump` 与支持 X86 的 `libLLVM`；参考机器为 22.1.2；保留实际版本、库哈希与提取器身份 |
| ripgrep | `rg`，用于现有证据清点和基准文件检索 |
| MCP（可选） | 1.30.0；全部间接依赖和发行文件哈希见 [`requirements-mcp.lock`](requirements-mcp.lock) |
| 磁盘 | 开始至少 50 GiB；运行留 5 GiB；迁移前另检查包/恢复所需容量 |

支持 macOS 和 Linux；Windows 使用 **WSL2 内的 Linux Python、Java、LLVM**，建议数据放 WSL 文件系统。当前脚本使用 POSIX 文件锁和 Neo4j Unix 分发，不提供原生 PowerShell 安装。Linux 的 LLVM 必须包含共享库，只有 `llvm-objdump` 不够。

将 [`config.example.json`](config.example.json) 复制为 `tools/code-map.local.json`，填入目标电脑路径。macOS Java 路径通常以 `Contents/Home` 结尾；Linux 指向含 `bin/java` 的 JDK 根目录。Linux 的 `llvm_library` 可以是 `/usr/lib/llvm-22/lib/libLLVM.so`，以实际安装位置为准。可删除 `llvm_library` 让脚本在指定 LLVM 的 `lib/` 下发现。

也可用环境变量：`CODEMAP_JAVA_HOME`（优先于 `JAVA_HOME`）、`CODEMAP_LLVM`、`CODEMAP_LLVM_LIBRARY`、`CODEMAP_RUNTIME`、`CODEMAP_CONFIG`、`CODEMAP_WORKSPACE`。环境变量优先于本地配置。打包要求 runtime 在工作区内；无需使用默认目录，但不要把它放进工具源码目录。

```sh
python3 tools/code-map/manage.py doctor
python3 tools/code-map/manage.py setup          # 已打包官方 archive 时不再下载
# Agent 需要 MCP 时才安装：
python3 tools/code-map/manage.py setup --mcp
```

`doctor` 实际读取 Java/LLVM 版本，运行 X86 解码并检查空间。`setup` 固定下载来源及哈希，创建仅本机监听、开启认证的实例。新电脑生成自己的密码，凭据存 `tools/code-map-runtime/connection.json`（0600），不复用原电脑凭据。`--mcp` 在 runtime 内创建 venv，使用哈希锁；不更改全局 MCP 配置。

首次安装 Java、LLVM、Python 和 `rg` 由目标电脑环境管理器完成；本工具不自动升级全局软件。依赖包可从官方 PyPI 获取，Neo4j 来源为 `dist.neo4j.org`，网络调用清除本次进程的代理变量，不永久修改网络设置。

## 2. 推荐：携带当前证据与检查点重建

原电脑执行：

```sh
python3 tools/code-map/manage.py package --output tools/code-map-transfer-2026-10-30
python3 tools/code-map/manage.py verify-bundle tools/code-map-transfer-2026-10-30
```

输出是可直接复制的目录；可自行压缩，解压后仍需执行 `verify-bundle`。**不要只复制 Git 仓库**。迁移包自带本入口、分析源码与全部清单文件；可以从包内启动恢复，不需要预装其他逆向工具。

包包含：

- A/B/C 绑定的原 PE/PDB 和 B patched EXE，保留双方独立身份。
- 已登记的 PDB procedure、inline、type、source 与调用证据，必要元数据。
- SQLite 一致性备份、已提取汇编分片、原松散 ASM 及其归档、proof CAS。
- 真实链人工基准、实际引用的报告和 Rust 源码片段、模块规则。
- 全部 code-map 源码、独立 PE/PDB 校验脚本及来源哈希、官方 Neo4j 安装包（本机存在时）。

包**不含**密码、`connection.json`、活数据库文件、事务日志、venv、共享 Ghidra 数据库和其他 worker。打包只按 SQLite/证据清单发现输入，不扫描整个工作区。SHA 与已登记输入不一致则失败；不会把变化静默包装成原证。当前 Rust 与历史行为 receipt 的失效绑定继续保留为失效。

在新电脑，假设包解压为 `/data/code-map-transfer`，目标为 `/data/game`：

```sh
cd /data/code-map-transfer
python3 payload/tools/code-map/manage.py verify-bundle .
python3 payload/tools/code-map/manage.py restore . --workspace /data/game
cd /data/game
# 编辑 tools/code-map.local.json，添加本机 java_home、llvm 和可选 llvm_library。
python3 tools/code-map/manage.py doctor
python3 tools/code-map/manage.py setup --mcp
```

恢复要求目标目录为空，先校验整个包再写入；拒绝覆盖已有工程。每个文件复制后再次核 SHA。缺文件、损坏、路径越界、符号链接和不完整包都不会恢复为成功。中断保留现场，使用新的空目录重试。

开一个终端持续运行：

```sh
cd /data/game
python3 tools/code-map/manage.py console
```

另一个终端：

```sh
cd /data/game
python3 tools/code-map/manage.py rebuild
python3 tools/code-map/manage.py check --full
python3 reverse-engineering/tiny-glade/tools/code-map/code_map.py benchmark --repetitions 30
```

`rebuild` 使用保留的 catalog 和分片，顺序导入图、建搜索索引、导出模块地图与未审查队列，**不重新提取汇编**。换了平台/LLVM 也能使用旧分片；若继续新提取，旧 extractor 身份不同会被拒绝，需显式独立提取目录与审查，不能删掉身份文件绕过。

迁移映射只改变文件打开位置，不修改历史证据文本、SHA、catalog 内记录或 chain.json。重建时当前工作区/runtime 的绝对路径被纳入快照身份，**新快照 ID 可能不同**；同一 PE、RVA、指令字节与关系合同应保持。要原样保留数据库和快照 ID，用下一节的离线 dump。

## 3. 保留相同快照：离线 Neo4j dump

先协调所有使用者，正常结束**这个 code-map 实例**的 console（Ctrl-C）。脚本不自动停服务，不控制其他 Ghidra/ReVa worker。数据库仍运行时 `package --dump` 会拒绝，Neo4j 自身的文件锁也会阻止不安全导出。

```sh
python3 tools/code-map/manage.py package --dump --output tools/code-map-offline-transfer
python3 tools/code-map/manage.py verify-bundle tools/code-map-offline-transfer
# 原电脑需要继续工作时，在原终端重新运行 console。
```

在新电脑按第 2 节 `restore` → 配置路径 → `setup`；**先不要启动 console，也不要 rebuild**，然后：

```sh
python3 tools/code-map/manage.py load-dump /data/code-map-offline-transfer
python3 tools/code-map/manage.py console
# 另一个终端：
python3 tools/code-map/manage.py check --full
```

只导出 `neo4j` 业务库，不导出 `system` 认证库；目标使用新凭据。目标业务库已有数据时拒绝加载，不提供覆盖参数。先以相同的 5.26.32 恢复，不能在迁移操作中隐式升级数据库。dump 包还包含同一份汇编和 catalog，单独一个 `.dump` 不足以支持 assembly/evidence_pack。

## 4. 从原始已登记材料重新运行全部分析

在独立工作区和**新 runtime** 使用 `rebuild --from-scratch`。必须已准备 A/B/C PE/PDB、B patched EXE、全部 procedure/enrichment/source/type 索引、已有 roof pack、原调用导出、原松散 ASM、模块清单、chain 报告/源码，以及 `ghidra-functions.jsonl.gz`。当前迁移包按清单携带这些材料；新项目仍需要先生成自己的索引，脚本不是任意二进制的一键语义分析器。

若已从包恢复，可先把 `tools/code-map-runtime` **改名保留**，在新建同名目录只复制原 `ghidra-functions.jsonl.gz`，不要复制旧 catalog/extractor。新的导出工作应使用自己的独立 worker：

```sh
CODEMAP="$PWD/reverse-engineering/tiny-glade/tools/code-map"
python3 "$CODEMAP/export_job.py" --worker codemap-YOUR-UNIQUE-NAME --prepare-only
# 把生成的 job 交给目标电脑既有 ReVa operator；或显式传 --operator 和 --python。
# 核 success/timedOut/stdoutTruncated、原 EXE SHA 与导出哈希；正常停止自己的 worker。
python3 tools/code-map/manage.py setup
python3 tools/code-map/manage.py console
# 另一个终端：
python3 tools/code-map/manage.py rebuild --from-scratch
python3 tools/code-map/manage.py check --full
```

完整顺序：`inventory → capacity → enrich → associated → archive-existing → dispatch-boundaries → extract → import → optimize-search → export-map → module-reviews → byte-coverage`。`extract` 按范围续跑，`import` 幂等；未完成快照不发布。上游失败立即停止。失败后使用原 CLI 从失败阶段继续，`--from-scratch` 不会覆盖已创建的 catalog。

Ghidra 可复用随包导出的静态函数清单，无需在目标机重装/重分析 Ghidra。若要进一步 P-code、跳表或反编译取证，另按[原工具指南](../../reverse-engineering/tiny-glade/docs/current/AI_REVERSE_TOOLS.md)配置独立 worker，不会由迁移脚本隐式代办。

## 5. 分层模型与分析入口

按 2026-10-30 的全局清理要求，**业务图不再保存机械函数、通用名称符号及其关联**，覆盖全部模块和 A/B/C。规则在 `graph_filter.py` 单点维护，源码关联扫描和图导入共同使用：Default/Clone/fmt、serde serialize/deserialize/visitor、From/Into、SystemParam 等机械名称，以及未经 owner 原证确认的 new/get/insert/remove。不再从这些名字猜当前类型实现。

同址若有真实业务别名（例如 ICF 合并），保留物理实体和业务别名，只清理无效别名。纯机械入口、其专属范围/汇编图节点、相关调用点和边均从 Neo4j 实际删除；原 PE/PDB、分片、catalog、松散 ASM 作为证据保留。故“原证提取覆盖”和“过滤后图节点数”是不同口径。类型/schema 和源码模块库存不因方法通名被整体抹掉。

已有库的清理与重查：

```sh
python3 reverse-engineering/tiny-glade/tools/code-map/code_map.py prune-mechanical
# 上一条仅输出方案；明确要求执行时：
python3 reverse-engineering/tiny-glade/tools/code-map/code_map.py prune-mechanical --apply
python3 reverse-engineering/tiny-glade/tools/code-map/code_map.py optimize-search
```

清理不覆盖原快照身份：原快照先失效，保留实体迁到新快照，完成后发布。所有旧物化快照中的同类实体也删除；中断可按检查点继续。重建从导入源头应用同一规则，不能把它们再次导回。清理报告为 `graph-prune-result.json`，全库名称残留核查为 `graph-filter-audit.json`。旧版迁移备份只用于回溯；对外交付应使用清理后重新生成的包。

```text
原证据：PE/PDB + 输入 SHA/来源（原件保留）
    ↓ 按入口 RVA / 区间 / 指令字节核验
汇编层：不可变 ZIP 分片 + 每条范围记录 SHA（按需读取）
    ↓ catalog 检查点 / 幂等导入 / 完成后发布
Neo4j：Entity + LINK + Snapshot/Publication（结构、候选、确认分开）
    ↓ 人工原证审查
业务层：状态读写 / 生产消费 / receipt 生命周期 / 实现与验证绑定
```

物理函数身份为原二进制 SHA + 入口 RVA。符号别名、内联、范围、直接调用、尾跳候选、未解间接调用独立表达；不能把异常展开入口或调用聚类直接当成业务模块。原始 B RNE 与 patched B 严格分离，仅原范围字节一致才关联分析结果。

```sh
CODEMAP="$PWD/reverse-engineering/tiny-glade/tools/code-map"
python3 "$CODEMAP/code_map.py" query search --args '{"text":"0x1408cdc60"}'
python3 "$CODEMAP/code_map.py" query assembly --args '{"key":"function:B:8cc870"}'
python3 "$CODEMAP/code_map.py" query evidence_pack --args '{"key":"function:B:8cdc60"}'
python3 "$CODEMAP/code_map.py" query state_users --args '{"key":"state:B:WorldRoofTable"}'
python3 "$CODEMAP/code_map.py" query coverage
python3 "$CODEMAP/code_map.py" query unresolved
# MCP stdio，使用 setup --mcp 创建的解释器：
tools/code-map-runtime/venv/bin/python "$CODEMAP/code_map.py" serve
```

Agent 入口为九个固定只读工具，默认深度 2，最多 6，最多 200 项，数据库查询超时 10 秒；返回身份、快照、证据与截断标记。管理写入口是 CLI，不开放任意 Cypher 给 MCP。Browser 默认 `http://127.0.0.1:17474/`，Bolt 17687；自定义端口在本机配置里指定，凭据只从本地文件读取。

`module-map.json/.graphml`、`module-reviews.json`、`byte-coverage.json` 在 runtime 下。真实链：屋顶碰撞生产 → World/receipt → 全局 raycast → 最终 winner UV/法线转换 → terrain 合并 → 已有消费者。`check` 实际查这条链并做独立字节验证；`--full` 加全范围字节检查。结果写 `transfer-validation.json`，目标机要重新测性能，不沿用原机延时。

检查点图重建可能耗时数分钟，期间保留 5 GiB 并查看 CLI 进度。不要把服务启动成功当成查询通过；受管环境可能回收 detached 进程，优先保持 console。原优化与可靠性边界见[优化报告](../../reverse-engineering/tiny-glade/evidence/code-map-2026-10-28/OPTIMIZATION.md)。图谱导航成功仍不能代替全软件业务语义审查、生产 Rust 行为或 GPU 验收。

## 6. 本机整理验证记录

迁移相关实测与限制见 [`VALIDATION.md`](VALIDATION.md)。完整冷停机 dump/load 和跨物理电脑测试需在实际目标电脑执行；本工具不会为制作说明擅自停掉共享实例。
