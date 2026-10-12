# Tiny Glade 项目记忆

## 用户已确定的目标与边界

- 原作研究路径：`D:/game/ljxsj_92385/Tiny Glade`。分析目录：`D:/game/reverse-engineering/tiny-glade`。Rust工程：`reconstruction/Cargo.toml`。
- 最终交付完整 Rust 游戏实现；当前先恢复源码、类型、状态与 CPU 算法。场景回放和 GPU 研究后置。
- 2026-10-12 最新用户指示开始按原PDB骨架/反汇编、Vulkan公开API和作者思路恢复渲染Rust并编译测试；本轮落实CPU资源图规则，完整GPU/场景验收未开展。此项覆盖此前渲染源码一律后置的安排。
- 不分析用户自己的 demo。不得把已有通用 Rust 依赖也当作游戏算法重新逆向/手写。
- 引擎为 **Bevy 的分叉**：这是用户明确提供的项目事实。随包 Cargo.lock/PDB 独立确认 `bevy_app/bevy_ecs/bevy_tasks` 等为 0.16.0；分叉代码仓库、具体commit及补丁范围尚未核实。不能据此猜测完整渲染等同上游 Bevy。

## 原作依赖基线：直接复用

唯一基线是原作 `build-info/Cargo.lock`，SHA256 `9e0cb4a0fa173122fa7cd552aa639f5ee977abc22c74aec92398808c707184bd`，901 个 package（多版本分别记录）。不能用最新版本覆盖这份基线。

| 依赖 | 原作使用版本/来源 | 当前处理 |
|---|---|---|
| Bevy family | 0.16.0；引擎分叉按用户说明 | 复用；只研究游戏调用及必要分叉改动 |
| glam | 游戏core/roof/utils用0.29.3；锁文件另有0.20.5/0.25.0/0.27.0 | 复用0.29.3数学API，不还原glam实现 |
| fastrand | 游戏core/roof/utils用1.8.0；另有2.3.0 | 复用1.8.0，不能误选2.3.0 |
| half | 2.4.1 | 调用f16转换，不手写转换器 |
| serde / serde_json | 1.0.200 / 1.0.85 | 复用；逆向的是游戏结构/格式和消费者 |
| ron | PounceLight/ron 0.8.1，commit `2787d8c05250c4dd706dfb358a2e9f16969d2968` | 复用原fork，不重写RON解析 |
| rkyv | PounceLight/rkyv 0.7.39，commit `031d58108ccf8c0e125b06b017f72b4afd67e9a3` | 复用原fork，恢复游戏存档schema |
| winit | PounceLight/winit 0.29.15，commit `d03de0a0a881be9422c360083151b0e7a8a2582e` | 复用原fork，窗口/平台研究后置 |
| slotmap / smallvec | 1.0.6 / 1.13.2 | 复用容器；恢复游戏ID和字段语义 |
| ash | 0.38.0+1.3.281 | 复用Vulkan接口；GPU后置 |

全部 package及来源应保存到 `tracking/dependency-policy.json` 和 `tracking/original-Cargo.lock`；原锁文件用于比对，不直接替换重建工程自己的锁文件。

## 排除范围与待归属范围

1. crates.io、公开git依赖与Rust标准库：读取公开源码/文档并使用匹配API，不反编译还原其实现。
2. 无source字段的52个path/local条目不能一律当作游戏独有代码。FidelityFX、XeGTAO、驱动/分配器等先确认已有上游；不要因为路径是local就重新实现。
3. `country-core`、`system-roof`、`system-wall-constructor`、`system-decorator` 等游戏模块为主逆向对象。游戏自有 `utils::CurveU/random_splits` 可以恢复；其中调用的glam、RNG、f16仍复用依赖。
4. 反编译器展开的Vec/Arc/ECS/Serde泛型胶水仅用于理解游戏数据与调用边界，不能计作游戏源码恢复成果。

## 已完成与证据界限

渲染首批5个私有函数已实现：访问链、生命周期事件、并发访问判断、buffer/image访问用途转换，221016个原机器码案例通过。见 `reconstruction/render/{resource_info,resource_usage}.rs` 和 `evidence/render-recovery/verification.json`。独立渲染crate复用原ash0.38.0+1.3.281，12个registry依赖source/checksum与原锁匹配，主工程锁不变；locked release和clippy通过。is_write确认属于vk-sync-fork公开依赖，使用外供classifier边界，未重新实现该库。原local fork来源/补丁尚待核实。PDB鲜读389编译单元定位93个限定范围原文件；其4124条筛选native签名均为void()占位，注释骨架不能计作完整类型/可执行源码。M12仅verified_partial，完整图/资源池/barrier/提交/pass/shader/画面未验收。

2026-10-12 完成 rhapsody 并行批次01/02的 CPU 合同恢复。批次01恢复 `ResourceInfo::calculate`、ResourcePool/Registry 所有权与 CompiledGraph/Batch 布局；512个calculate快照、1600个BatchCompiler原入口案例和资源池候选测试通过。批次02整合小图DSL到compiled snapshot（17测试/4快照），恢复executor/temporal候选（4测试），并闭合managed handle到submit completion/disposal的生命周期；track/sequence/clone/non-last-drop共29692个原指令案例零差异。跨包门禁见 `evidence/render-recovery/parallel-batch-02/integration.json`。关键约束：CPU retire可早于GPU完成；GPU安全由tracked handle/command bucket保护；command sequence不同于queue timeline；temporal parity按key保存；export_temporal不翻转parity。完整Vulkan/GPU执行、完整compile/record/submit原入口差分、跨序列域映射、pipeline/shader和游戏pass仍未验收，M12继续为verified_partial。

2026-10-12 完成 rhapsody 并行批次03。R5A 的 barrier/record 分段 oracle 通过1344个原指令/Rust案例，`populate_resources`通过97个原入口fixture，并完成6个R2 snapshot检查；R5B确认command sequence与queue timeline分域，且timeline完成后CB立即回池、sequence gap仅延后resource bucket drain；R5C恢复raw texture=0/raw buffer=1/exported=2的跨帧import/export、texture resize替换、buffer无descriptor比较、同帧重复lookup错误和pair parity。候选Rust测试为4+2+7项，第三批跨包门禁13/13通过，见 `evidence/render-recovery/parallel-batch-03/integration.json`。实际counter构造函数、temporal reset/drop、完整pass recording、真实Vulkan/GPU与游戏pass仍未知，M12保持verified_partial。

2026-10-12 已把其中55个PDB确认的 `rhapsody` 路径组织成独立Cargo可检查项目：`reconstruction/render/original-skeleton/crates/rhapsody/`。结构为2个crate-root文件、29个raw直接文件、9个raw/managed文件、15个rg文件；新增3个 `mod.rs` 仅为合成Cargo glue，原PDB未列出。`Cargo.toml`记录原锁确认的包名/版本和36个直接依赖名，但不激活尚未恢复kind/features/target及local/fork来源的依赖。`cargo metadata`、offline `cargo check`、fmt及55路径一一核对通过；这只证明项目结构闭合，不计作函数/类型/实现恢复。

屋顶曲面/Curve2、脊线、圆形与矩形CPU铺瓦源码已存在。此前临时手写的随机数、f16转换、四元数/向量运算已改为原版本库调用，重建工程6个依赖包的版本和checksum与原锁文件一致。替换后重新验证：矩形33输入/19024条记录、seed和最终RNG一致；圆形37输入/11467条记录逐字节一致。具体见 `evidence/roof-tiles/dependency-boundary.json` 和 `dependency-verification.json`。

新增墙高约束数值后段与3个完整高度缓存叶函数，通过6624个机器码案例。约束前段的对象查询、特殊flag生产者仍待恢复，不能把局部数值规则记成完整墙系统。

新增 `roof/edit_rules.rs`：已解析Roof/Event的编辑后段、变化量、change tick、gable重建和数值反馈，通过4514个原指令案例；tip长度限制复用glam公开API。原ECS解析/调度/失败/队列扩容未恢复，不能记成完整屋顶编辑系统。记住height Absolute下限0、Delta下限0.1；eave/ridge/tip Absolute保留raw，Delta才限制；变化量不是实际最终差值。细节和边界见 `evidence/source-edits/findings.md`。

发行树/PDB未取回作者原Rust文本；当前是原机器码约束下的行为恢复源码。不能声称获得作者原始源代码、完整ABI或整个游戏完成。

## 当前渲染推进状态

2026-10-12 当前功能实现另有用户授权聊天独占 `reconstruction/rhapsody-rebuild`。当前功能状态以其 `MODULE_STATUS.md` 和 `evidence/functional-project-validation.json` 为准：64测试、严格all-targets Clippy、13真实GPU探针及局部原码对照均通过；25条验收命令无失败。主流程只读复核112项记录输入无变化、53外部依赖身份匹配原锁，证据为 `evidence/render-recovery/consumer-chain-review-2026-10-12/functional-evidence-review.json`，未重跑未变用例、未编辑新目录。已覆盖shader/descriptor生产、附件、WSI、temporal、session、同步/BDA、诊断、分配器私有包装/格式/兼容策略的明确候选功能范围。原完整行为仍未完成，尤其首个主图shader callback的自动绑定/真实CommandState注入（当前两阶段bridge）、并行调度/跨族ownership transfer、缓存淘汰/资产pass关联、winit/gpu-allocator/vk-sync本地分叉。不能用55路径已写或GPU可运行计原作全部模块完成。

同轮旧共享入口已新增 `ExecutionChain::execute_graph_with_context`，callback直接借用当前counter产生的Command、pass索引、compiled registry及resource handles；旧句柄入口兼容包装。两批次场景验证sequence2/4对应实际提交句柄，4链级测试与Clippy通过，证据 `consumer-chain-review-2026-10-12/context-entry-validation.json` 取代旧共享源码hash作为当前输入校验。此API尚未迁入新工程，不能记为其自动callback完成。已通过只读MCP批量导出原AssetShaderLibrary四个add/get生产者及raster创建，共5个目标，成功关闭；原名字索引与FrozenMap容器边界、compiled反射驱动layout的字段约束见同目录README。三个worker续跑因模型容量失败，主流程完成本次工作，无active worker。

更新总进度另发现第五批统一门禁仅工具runner输入hash变化（工具维护方新增batch支持）；旧报告原样保留，批次5移入historical_integrated_batches并标记historical_tool_input_changed，不再声称当前输入一致。第四批输入仍一致，新功能报告和本轮context报告单独有效；未用覆盖历史hash或重跑未变原码案例来绕过检查。

2026-10-12 后续实施已完成首个共享CPU候选入口 `reconstruction/render/execution.rs::ExecutionChain::execute_graph`：graph声明→ResourceInfo/Registry→compiled字段生成record输入→原counter从1产生sequence→queue实际组装submit参数→receipt/pending→completion/reclaim。公开输入不再手填first/last、compiled snapshot、sequence7或SubmitInfo；Registry token到ControlBlock/generation仍由明确的外供private provider提供，不能视为原资源生产者恢复。三个链级场景（正常、callback失败、queue失败）通过，locked/offline check和all-targets Clippy通过。queue错误保留已注册timeline与state3/pending，不伪造receipt/completion。报告 `parallel-batch-06/shared-entry-validation.json` 区分 candidate链与原整链：后者及live GPU均未验收。三个worker已交付、active_agents=0。shader/interface producer与共享descriptor callback、wait/signal/prior/flush producer、temporal跨帧/真实游戏consumer仍待闭合。本次只迁入必要graph源码，其余复用现有包；不新增独立candidate交付包。渲染锁文件增加本地包，12个registry版本/source/checksum仍匹配原锁；历史leaf报告保留原hash并以共享报告单独复核依赖投影与未变leaf输入。以下流程审计的“尚未迁移”描述是早期状态，不是当前状态。

2026-10-12 用户指出流程再次退化为逐函数分析/差分，授权推翻旧流程。已完成结构审计并制定 `evidence/render-recovery/WORKFLOW_RESET.md`，其任务分派/新增验收规则取代旧批次计划。共享render/lib.rs仅2模块、16个work-package目录和重复Command/QueueIdentity等表明真实整合未完成。停止新增独立candidate crate；第一任务是graph→compiled→record→counter产生sequence→submit参数的共享入口，私有替身移除和真实consumer接线为进度指标。差分仅用于歧义裁决/受影响回归，不按case数扩张。当前只是流程审查与规则变更，共享入口迁移/真实帧捕获尚未实现；旧五批原报告保持限定有效，M12不升级。此前2–4周/6–12周估算无实测吞吐支撑，不再作为排期承诺。

2026-10-12：第四批统一整合50/50通过，本轮复核480项输入hash，重跑2840+108原码案例、15候选测试、3跨包测试和404/638全量反射。第五批3个agent及主流程交付已统一验收27/27：Primary调用者180案例；非空资源顺序record/submit774新案例（726非空pass access、48无last-access export、330原bytes转移），5测试；descriptor创建5184原指令案例（4438layout完整入口、486注入layout的pool完整入口、256compute layout片段、4错误入口停止），2测试。21个pipeline依赖均与原锁source/checksum一致。Ghidra窗口已close(save:false)，本批agent均完成。

重要接口边界：9个workgraph限定direct begin/submit调用点tag=0，不证明整游戏无aux调用；原per-queue sequence初值1，R5B candidate seed0仍为fixture、生产API适配未完成；record sequence7仍为fixture。顺序helper没有独立post-pass barrier scope，下一pre或final export在callback成功后继续。persistent layout runtime count=min(saturating_sub(device_scalar,32),8192)，scalar精确Vulkan property名未知；fixed数组count!=1用PARTIALLY_BOUND(4)，runtime sampled image用flags15/layout UPDATE_AFTER_BIND_POOL。638声明到enumerated layout仅candidate adapter，不证明原sparse binding/真实pass关联。

本轮工具审计及改进见TOOLING_AUDIT_2026-10-12.md与tooling/README.md。当前Ghidra无auto-analysis时xref仅PDB DATA，漏掉原码直接call，不能据空结果断言无caller；跳表伪码可越PE边界读错.rdata，要用限定原反汇编/表数据核对。新只读SDK runner默认save:false、共享锁和失败隔离，缺provider manifest/DB snapshot默认不缓存；27个fake MCP协议测试通过，不计作原作恢复。未改全局工具配置或安装新引擎。

下一步：真实sequence生产接口适配 → 多队列内联/间接caller、并行录制/完整queue前段 → sparse layout producer、descriptor writes及完整pipeline → country-core最小cull/indirect、shadow/main接线。temporal reset/drop、完整GPU与画面仍未验收，M12保持verified_partial。第四/五批integration与模块进度保存于 `evidence/render-recovery/RHAPSODY_PROGRESS.md`；不按文件、函数或case数推算整体百分比。

## 工具与推进方式

- 2026-10-12 用户询问从最底层逐层恢复的可行性，分析见 `evidence/render-recovery/bottom-up-plan.md`。建议原依赖来源核实 → BufferDesc/TextureDesc/View与纯规则 → 设备/资源创建 → managed/提交/回收联合状态 → shader/pipeline →完整rg→游戏GPU场景/pass/算法。允许查上层调用证据辅助确定底层接口，不能将模块名当成严格无环层级；本次只作计划分析，没有新增实现验收。

- 2026-10-12 用户专项询问自建渲染管线；静态架构结论见 `evidence/render-architecture/findings.md`。原构建由 country_core 专用 pass → rhapsody 渲染图/执行器 → ash/Vulkan；原锁文件无 bevy_render/pbr/core_pipeline/wgpu。实际RE-MCP反编译10个函数及20个原shader SPIR-V声明，确认资源访问链/生命周期/barrier、HZB前后绘制列表阶段、GI/透明/DoF与TAA或TGSR、主体图执行后获取交换链并呈现。完整shader算法/动态资源依赖/画面未验收，M12不因此完成，源码优先的总体任务顺序保持。跨版本C/1.16后端证据不得迁移到本构建。
- 2026-10-12 完成渲染框架包层次地图：`evidence/render-architecture/rendering-framework-map.md` 与机器可读 `rendering-framework-map.json`。地图将直接相关包分为 L0 应用/窗口/ECS、L1 country-core 游戏渲染、L2 rhapsody::rg、L3 rhapsody::raw、L4 Vulkan/分配/同步外部依赖、L5 shader 工具链、L6 资产/纹理管线、L7 游戏效果/外部算法边界；确认发行树 `compiled-shaders` 为 404 个（234 raster、170 compute）。这只是依赖和逆向路径地图，不改变 M12 的 `verified_partial` 状态；`gpu-allocator`/`vk-sync-fork`/`shader-*`/`asset-*` 归属和完整图执行仍待核实。

- 使用 `reverse-engineer-anything` skill 的原作证据流程。
- REA、ReVa、RE-MCP已安装；会话没有直接暴露其工具时，通过官方MCP SDK连接已配置stdio服务。连接、工具catalog和实际调用结果须落证据，不把doctor成功等同实际连接。
- 复用持久Ghidra数据库；按原作函数/引用/类型检索，不全程序重复分析十万条符号。
- 从高收益游戏规则和权威状态链推进：Roof编辑消费者/字段映射 → Wall形状/高度/生成 → 历史提交/撤销消费者 → 门窗附着等。
- 里程碑与源码验收分开记；依赖复用排除在逆向覆盖统计之外。
