# rhapsody 批量逆向与自动化计划

2026-10-12流程重审：后续任务分派和新增验收方式由[WORKFLOW_RESET.md](WORKFLOW_RESET.md)取代。本文件保留历史证据与已验收范围，不再按旧批次创建孤立candidate包。

日期：2026-10-12。目标构建为 `D:/game/ljxsj_92385/Tiny Glade`，二进制、PDB 与依赖边界沿用项目已保存证据。

## 目标

恢复单位从“单个函数”改成“类型族 + 状态机 + 端到端数据流”。函数只作为证据节点；只有纯规则、关键歧义或边界失败才做单函数机器码差分。

进度不再按函数数量计算，而按以下条件计算：

- 核心类型被生产者和消费者共同解释；
- 一条输入到输出或创建到回收的状态路径闭合；
- 工作包的模块边界快照或事件轨迹通过；
- 未知项被明确登记，并判断是否阻塞下一层；
- `country-core` 能通过恢复后的公开接口使用该子系统。

## 自动化优先级

执行顺序固定为：

1. **确定性抽取**：PDB 路径、函数范围、别名、直接调用、尾跳、公开符号、SPIR-V 反射。
2. **机械归一化**：按地址去重、泛型实例归族、共享地址冲突标记、依赖与编译器胶水分类。
3. **工作包生成**：按共享类型和状态流生成候选，不按文件逐个分派。
4. **缺口驱动取证**：只有工作包中的具体未知会触发反编译、xref、字段偏移或原指令查询。
5. **一次实现整条路径**：先类型和不变量，再实现工作包内相关函数。
6. **边界验证**：比较 compiled graph、生命周期事件、Vulkan 参数或 shader 接口，不机械镜像每个函数。

前 3 步应由脚本完成，不调用 AI。AI 只处理类型语义、跨函数不变量、歧义裁决和 Rust 设计。

## 七个工作包

| ID | 工作包 | 主要模块 | 边界产物 |
|---|---|---|---|
| A | 描述与同步模型 | `raw::{format,buffer,texture,binding,barrier,bytes}` | 资源描述、usage/access、barrier 描述 |
| B | 设备与资源创建 | `raw::{instance,init,physical_device,device,allocator}` 及创建路径 | Vulkan create info、错误清理轨迹 |
| C | GPU 生命周期 | `raw::managed`、`command_buffer`、`semaphore`、`submit`、`disposal` | `create→record→submit→complete→reclaim` 事件轨迹 |
| D | 图编译器 | `rg::{resource,resource_info,resource_registry,resource_pool,graph,pass_builder,pass_api,compiled,state}` | compiled graph 快照 |
| E | 图执行器 | `rg::{recorder,executor,import_export,temporal,util}` | batch、提交、跨帧资源轨迹 |
| F | Pipeline 与绑定 | shader、descriptor、render pass、framebuffer、`rg::binding` | 反射后的 pipeline/descriptor 接口 |
| G | 交换链与诊断 | swapchain、profiler、stats、debug、crash、ray tracing、leek/log | present/resize/诊断状态路径 |

文件归属只是初始路由。共享一个状态机的函数可跨文件合并；共享地址但语义名不相关的记录必须标记为 ICF/别名冲突，不能自动视为同一源码函数。

## 每个工作包的执行协议

每包维护一个机器可读清单，至少包含：

- 入口、终点和上层消费者；
- 原生实例、语义族、直接调用边和未解析间接调用；
- 核心类型、字段约束和所有权关系；
- 状态不变量、失败路径和依赖边界；
- 已观察、推断和未知；
- Rust 实现范围与统一 oracle；
- 会使旧验证失效的 source/contract/dependency/harness hash。

取证批次只围绕一个明确未知，默认不超过 20 个函数。已有证据足够时停止扩张，不扫描整个程序来回答局部问题。

## 验证策略

### 纯规则

短构造器、位掩码、尺寸和 mip 规则允许使用原机器码批量差分。它们是便宜的叶节点验证，不代表工作包完成。

### 图编译器

输入小型 graph DSL，统一比较：pass 顺序、资源生命周期、访问链、physical allocation、pre/post barrier、import/export 和 temporal swap。一份快照覆盖工作包内多函数协作。

### 生命周期

比较结构化事件：创建、录制使用、提交序号、CPU handle drop、GPU complete、reclaim。重点是资源何时允许复用和销毁。

### Vulkan 包装

在公开 API 边界记录 create info、descriptor write、barrier、submit/wait/signal 顺序。匹配公开依赖直接复用，不恢复其内部实现。

### Shader/Pipeline

先从发行 SPIR-V 批量反射 descriptor、push constant、输入输出和入口，再用 CPU 调用者确认语义。shader 算法与 CPU 接口分开验收。

## 当前执行批次：R0 全局工作图

输入：

- `evidence/render-skeleton/native-procedures.json`
- `evidence/render-skeleton/files.json`
- `evidence/render-architecture/public-symbols.json`
- 原 `tiny-glade.rne`

工具：`scripts/render-recovery/build_rhapsody_workgraph.py`。

输出：`evidence/render-recovery/workgraph/`，包含：

- `functions.json`：按 VA 归一化的函数、别名、语义族和分类；
- `calls.json`：直接调用、尾跳、间接调用计数和符号解析；
- `work-packages.json`：七个工作包及包间调用边；
- `summary.json`：身份、覆盖、解码缺口和自动化统计；
- `README.md`：本次生成结果的人类入口。

R0 的完成门槛：全部 432 条 `rhapsody` PDB procedure 记录和带地址的 public symbol 进入联合索引；仅 public symbol 的入口使用 PE 异常表 `RUNTIME_FUNCTION` 定界；相同 VA 保留全部别名；每个唯一地址有且仅有一个初始工作包；反汇编缺口、间接调用和未知目标显式计数。

## 后续批次

1. **R1 图编译器合同**：从 D 包选取 `ResourceInfo::calculate → registry/pool → compiled/state` 闭环，生成类型字段约束和小图 DSL。
2. **R2 图编译器实现**：一次实现完整 CPU 编译路径，使用 compiled graph 快照验证。
3. **R3 生命周期合同与实现**：联合恢复 C 包，使用资源事件轨迹验证。
4. **R4 执行链**：把 D 接到 E，验证 batch、barrier 和跨帧资源状态。
5. **R5 Pipeline**：结合发行 SPIR-V 完成 F 包接口。
6. **R6 游戏侧接线**：连接 `country-core` 的最小 cull/indirect、shadow/main pass。

## 并行批次 01 执行结果

首批三个互斥工作包已完成，统一入口为 `evidence/render-recovery/parallel/README.md`，机器可读交叉检查为 `evidence/render-recovery/parallel/integration.json`。

- `resource-info`：恢复 `ResourceInfo::calculate` 的完整 CPU 语义候选；512 个原入口快照零差异。
- `registry-pool`：恢复精确描述符 LIFO 池、origin 所有权、registry 映射与 consuming exports；候选测试 6/6。
- `compiled-state`：恢复 compiled 顶层布局和 BatchCompiler；1600 个原入口案例零失败。
- 跨包检查确认 `ResourceRegistry`、嵌入的 `ResourceInfo`、access record 和 lifetime event 消费极性一致。
- 当前证据不支持宣称 graph-local alias reuse 已恢复；完整 GPU population、barrier/record、queue/retire 和 temporal schema 仍待后续批次。

运行 `python scripts/render-recovery/validate_parallel_graph_contract.py` 可重新生成并验证整合合同。该检查器只做确定性账本对齐，不把候选测试升级为原作完整等价证明。

## 并行批次 02

第二批已启动，合同入口为 `evidence/render-recovery/parallel-batch-02/contract.json`：

- R2：整合第一批三个候选，建立小图 DSL 到 compiled snapshot 的统一 CPU oracle。
- R3：恢复 managed handle、command-buffer sequence、submit completion 与 reclaim/disposal 生命周期。
- R4：恢复 executor、提交、retire、exports 与 temporal parity/history 状态链。

本批次的核心闭环是 `compile -> record -> submit -> complete -> retire -> next frame`。GPU/Vulkan 调用通过结构化 backend 事件验证；只有证据足够的纯 CPU 状态机才进行原入口差分。

第二批现已形成一致的部分闭环：

- R2：小图 DSL 到 compiled snapshot 的 CPU 候选完成，17 项组合测试和 4 份快照通过。
- R3：`track -> last CPU drop -> waste -> command bucket -> completion -> dispose` 状态机闭合；29,692 个原入口叶案例零差异。
- R4：batch/CB/submit/end-frame、立即 CPU retire、per-key temporal parity 与 export 状态已形成候选；4 项测试通过。
- 关键修正：command sequence 与 queue timeline 是不同编号域；CPU retire 不等待 GPU；`export_temporal` 不翻转 parity；普通 initial transition 与显式 reused pre-transition 分工不同。

跨包门禁为 `scripts/render-recovery/validate_second_batch_contract.py`，输出 `evidence/render-recovery/parallel-batch-02/integration.json`。完整 Vulkan/GPU、同资源跨序列域映射、temporal payload/reset、pipeline/shader 和游戏 pass 接线仍未验收。

## 并行批次 03

第三批关闭第二批留下的三个接口缺口，合同与结果位于 `evidence/render-recovery/parallel-batch-03/`：

- R5A：`add_pre_pass_barriers`、`record_batch` 分段 oracle 与 `populate_resources` 原入口。1,344 个 barrier 原指令/Rust 比较、97 个 population fixture、6 个 R2 snapshot 检查和 4 个 Rust 测试通过。完整 pass recording 与 Vulkan 执行未覆盖。
- R5B：确认 command-buffer sequence 与 queue timeline 是独立编号域；timeline 达标后 CB 可立即回池，sequence gap 只延后 resource bucket drain。候选 2 项测试与严格 Clippy 通过；实际 counter constructor `0x140bf2560` 尚未展开，因此不能声称跨队列全局唯一。
- R5C：恢复 raw texture=0、raw buffer=1、exported=2 的跨帧状态；同帧重复 lookup 报错，下一帧由 exported import，texture extent 变化重建，其余 texture descriptor 不一致 panic，buffer 不比较 descriptor；pair 在两次 single lookup 后翻转 parity。7 项多帧测试、fmt 和严格 Clippy 通过。

跨包门禁 `scripts/render-recovery/validate_third_batch_contract.py` 为 13/13 通过，输出状态 `consistent_partial`。temporal reset/drop、完整 compile take/restore unwind、完整 record/submit 原入口、真实 GPU、pipeline/shader 与 country-core pass 仍未验收。

## 并行批次 04（已交付，待统一整合验收）

- R6A：展开 `GraphicsDevice::create` `0x140bf2560`，关闭 counter seed、queue identity 与全局 bucket key 唯一性。
- R6B：从 R5A 分段 oracle 继续覆盖 pass-recording tail、`submit_pending_cbs` 与 `queue_submit` 的 backend-event 边界。
- R6C：对发行树 404 个 compiled shader 做确定性接口反射、签名归族与 rhapsody/country-core 消费者交叉索引，不逐 shader 手工翻译算法。

合同由 `scripts/render-recovery/build_fourth_batch_contract.py` 生成到 `evidence/render-recovery/parallel-batch-04/contract.json`。三个 agent 已完成交付；R6A 已关闭 Ghidra（save:false），R6B/R6C 使用既有证据与批量工具。

各包验证报告：R6A 原 CPU 片段 2,840 例零差异与 6 项 Rust 测试通过；R6B 原码/候选 108 例零差异与 3 项 Rust 测试通过；R6C 全量 404 文件/638 SPIR-V 模块反射成功、0 失败，240 布局族/402 接口族与 4 项 Rust 测试通过。统一跨包门禁尚未执行。

第四批现已统一整合50/50通过。第五批3个agent和主流程进一步完成：Primary caller180、非空record774（330原bytes→submit）、descriptor创建5184原指令案例，全部零差异；新Rust候选5+2测试与统一门禁27/27通过。工具审计及只读runner改进见[TOOLING_AUDIT_2026-10-12.md](TOOLING_AUDIT_2026-10-12.md)，本地fake MCP27测试不算原作源码恢复。

当前进度与模块缺口详见 [RHAPSODY_PROGRESS.md](RHAPSODY_PROGRESS.md)。下一步为真实sequence生产接口适配 → 辅助队列内联/间接caller及并行录制/完整queue前段 → sparse layout producer/descriptor writes/完整pipeline → `country-core` 的最小 `cull/indirect → shadow/main` 接线。R5B fixture初值0不能直接当原counter初值1；CPU/API合同门禁不等于production GPU接线。

## Skill 沉淀规则

不为单次发现创建新 skill。满足以下任一条件才沉淀：

- 同一种分析步骤已重复两次；
- 它可由确定性脚本稳定完成；
- 遗漏该步骤会造成版本、依赖或证据边界错误；
- 它能显著缩短后续工作包的模型上下文。

当前复用并扩展项目现有 `tiny-glade-recovery` skill，新增 `rhapsody` 工作包模式。项目事实保留在本计划和生成证据中，通用工具逻辑保留在脚本中，避免把大量易过期清单塞进 skill 指令。

## 已知限制

- PDB 的 `void()` 不是真实 Rust 签名。
- 静态直接调用图不代表运行时路径；函数指针、vtable 和 Vulkan loader 间接调用需另行解析。
- 链接器 ICF 会让不相关名字共享地址；自动化必须保留冲突，不能选一个名字覆盖其他名字。
- 当前Codex会话没有直接暴露REA MCP入口；已通过官方SDK和现有RE-MCP stdio配置进行真实限定取证。新runner不修改全局配置，缺少原生入口不再阻塞SDK路径。doctor/配置状态不等于实际连接。
- 无auto-analysis的持久数据库xref覆盖不完整，必须与原码direct call索引交叉核对；反编译跳表可越PE函数边界。缓存没有provider manifest/DB revision输入时默认禁用。
- 本计划恢复的是原机器码约束下的 Rust 行为，不声称取得作者原始源码。
