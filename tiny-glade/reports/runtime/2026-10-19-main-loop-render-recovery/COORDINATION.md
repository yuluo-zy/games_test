# 主循环与渲染循环续接 — 2026-10-19

> 最新授权：2026-10-20 用户要求自主判断，主链路全通过后继续逐核心业务模块。当前执行边界与真实验收门槛见 [AUTONOMOUS-EXECUTION.md](AUTONOMOUS-EXECUTION.md)，不是等待下一次用户逐项指派。

> 2026-10-20 当前断点：以下为按时间保留的历史协调记录。最新授权为“跳过这些继续推进核心链路”，已停止等待缺源及任何占位桩工作。当前 root + 原有 7 位负责人编制不变，没有新增子 agent。生产已集成内容、真实 CPU/GPU 结果、未运行的上层变更和剩余消费者统一见 [CORE-CHAIN-CONTINUATION.md](CORE-CHAIN-CONTINUATION.md) 与各 `validation/*integration*.json`；历史“未整合/等待同步”不是当前状态。

用户授权同步其新实现后继续，要求 56 个 agent 并行，中央控制正确性与性能，禁止简化掩盖。旧停止请求已被此授权覆盖。当前分为8组，每组负责人创建6名成员；不额外继续分叉。

生产源码、Cargo/lib/注册由 root 单点整合；构建/GPU执行由 root 串行安排。所有候选在现有合法 claim 的 work 中，独立审查原始事实，不能把哈希或编译当语义验收。用户未提交的3个critters文件受保护，见 SYNC-BASELINE.json。

8组：main_loop_team（OS/App/worker生命周期）、main_graph_team（compile_main真实尾链）、world_graph_team（W1–54）、scene_startup_team（资源/场景/VAO/Nani）、surface_frame_team（水/冰/折射）、rhapsody_correctness_team（当前重构底层）、performance_team（测量和热点）、validation_integration_team（独立验收与流程经验）。各组6人具体角色向root回报，root核实际agent树后登记。

当前源已有compile_world_graph与warmup；其中注释明确尾链仍为子集。必须独立检查实际分支条件和生产者，完整原版UI/菜单/截图不能被默认关闭条件跳过后称完成。实际首个构建在缺rfd缓存处停止，root仅下载锁定依赖再编译。


## 用户修正：总数8，完善每个任务提示词

此前56人结构已撤销。总数包含中央root，保留7执行岗位，所有子agent停止且禁止再spawn。完整提示词已落agent-prompts/COMMON.md及7个任务文件，职责/独占文件/接口/交付/验收具体见AGENT-PLAN.md和roster.json。多个先前agent已报告账户usage limit；不能把已配置角色数说成当前成功运行数。


## 八人编制实际确认与首编译

重新配置后agent树确认仅root及7个负责人running；所有旧子agent均interrupted或errored且不唤醒。7份启动提示已下发并收到接收确认。Mac锁定依赖缓存已补齐，不升级依赖。v2实际编译首失败是utils/src/lib.rs:26缺sampling模块；独审发现主链多处声明缺文件，正向用户核实另一机器是否尚未提交新文件。在答复前先推进已有源码中的正确性窄修，避免从零重建用户已写的新模块。底层rhapsody独立编译正在做，可不依赖此utils缺口。


## 用户确认新文件待补交

用户已回复“是，我会补交新文件”。因此45处缺源快照属于已确认的同步缺口，不据此断言业务没有实现；先等待新文件提交再同步，不从零重造用户已写模块。当前7个执行agent已全部收到相应约束。现有重构rhapsody的独立lib基线实跑375通过/0失败（BASELINE-VALIDATION.json），这不覆盖后续候选和完整游戏帧。


## 2026-10-20 核心链路本批整合断点

当前源码整合清单覆盖30个实际源/测试文件，最新指纹、语法解析和限定diff检查通过，protected3保持基线。除已记录的rhapsody 379 CPU与26项选定GPU、render_frame27等范围外，新增上层代码及Nani新增测试未运行。Nani BDA生产v2两源与实际fixture修正已整合，见validation/nani-bda-integration-v2.json、nani-bda-fixture-integration-v2.json和final-source-checks-v2.json。root未提交/推送本批；不添加缺源桩。详细链路与剩余缺口见CORE-CHAIN-CONTINUATION.md。

## 2026-10-19T17:54:44.774086+00:00 — 完整 visual 依赖闭包续推

当前以 CORE-CHAIN-CONTINUATION.md 为准。Roof-present 源子域及类型域注释已入生产，v6 源快照 58 文件/54 Rust 的指纹、edition2024 语法和限定 diff 通过；v5 检查器 edition2021 错误保留，业务源码未为此修改。无新增完整 core/App typecheck 或 world/present 测试。

原 set25 两碰撞阶段及 member14 fullvisual→中间视觉组→copy_prev 已定位；真正 tracking cleanup 在 member9 validate 入口并保留 authority。当前同名 helper 错清 authority，restore 另依赖 destructive clear；TG889 foreign claim terrain-curve-20261018 未触碰。01 继续 validate 真实callback/现接口设计，02 fullvisual/精确Eq，03 canonical Window，04 ClothInfo/唯一VisualState，05 双tracker，06 调度/cleanup原证，07 独审。8 人含root上限，无新子agent。

Window V2设计通过0678c155；V1 changed-only collector错误保留未接受。fullvisual BODY设计通过83fb4932但源仍受schema/精确Eq门控。Atlas调查独审5e2e2186证实复用现初始路径，不造第二Atlas；live-link原四段链与observed sibling资产发布绑定仍待闭合。
