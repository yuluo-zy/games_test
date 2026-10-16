# Tiny Glade 原作还原：里程碑与进度

2026-10-16 toolbar 来源核查：B 原 PDB 记录的 31 个 Taffy 源文件与 crates.io 0.3.13 完全一致；8 个原码定点函数确认游戏 Taffy 适配、mei 自有布局和 tg-text 渲染边界。不能把整套 toolbar 归为 iced/egui 或 Taffy 的修改版，完整公开 GUI 上游未确证。此次仅来源研究，无业务完成度增长，见 [报告](evidence/toolbar-origin-2026-10-16/REPORT.md)。

2026-10-15 按用户要求改为实现优先：core gui.rs 已补菜单拥有状态、Next/Prev、状态推进、共享缩略图布局和调试标签；LUT 改读正确的 EscMenuState 并直接消费位置算法，菜单 ID 复用原 uuid。当前增量仅 all-targets 编译检查，不扩展验证矩阵。GUI 仍缺 PhotomodeScreen、完整 DebugGui/日志、Egui 资源及全局调度；全 core 未完成。逐文件状态以 [core 目录推进](../../tiny-glade/crates/country-core/recovery/README.md) 为准。

2026-10-14 用户改为按原源码目录逐文件恢复 core。当前入口 [core 目录推进](../../tiny-glade/crates/country-core/recovery/README.md)。B PDB 539 文件与历史重建/辅助路径分开；真实导出由 rustdoc 核对，27 直接依赖分类。基础资源/default/serde/事件/坐标已补，wall_highlight 真实 ECS 单系统接到统一 draw 资源；原 JSON 128 组合、窗口/菜单等叶、DirtyTiles 881 组和 highlighter 96 组只支持各自限定范围。全 core 仍未完成，下一文件为 GUI 载荷及 PublicWalls/实体生命周期。旧“章节已收齐”不等于当前正式代码已完成。

2026-10-14 country-core 非渲染首批：墙权威集合/编辑通知/删除与 core CPU 发布资源已接通；7 新测试、1349 core 库内测试和 check/fmt 通过，原指令只验证状态 stores/参数化构造/默认拱高/reset，排除库 lookup/通知与原完整 publisher。原表 +0xc0 ID counter、freehand_ty +0x66 已纠正。完整 PublicWalls/空间/屋顶/历史及原调度仍缺，不升级整模块/游戏验收。具体范围见 [报告](skeleton/task-index/work/TG-9f411268a571c6bce1/core-walls-20261014/REPORT.md)。

2026-10-14 utils B 深挖增量：原 2D 楼梯分段采样已恢复，143 组原指令样例；共享标量运算顺序修复，原20组3D构造/120查询逐位通过；Curve有所有权XZ/12→8投影获正面证据并接线。Debug/Release各93测试、严格Clippy及country-core1142 CPU回归通过。墙三方法/2D全局查询仍未定位；原借用as_vec2类型/投影未知，旧pointer/length说法已纠正。全4894模块/85322 C13行号段扫描不作死代码定论。用户后续仅B。当前报告：[B深挖](evidence/utils-deep-followup-2026-10-14/REPORT.md)，[台账](tracking/utils-deep-followup.json)；下段90项为历史源码快照。

2026-10-14 utils 当前工程续轮：共享 insert/remove 返回合同与 country-core 索引/512网格消费者合并，230 新原指令样例通过；Windows utils Debug/Release各90测试、country-core 1142测试及两库严格Clippy通过。墙三方法/2D全局查询编译体、投影轴/stride和完整游戏运行仍未恢复/验收，维持 partial。最新范围见 [utils 交付报告](evidence/utils-gap-recovery-2026-10-14/REPORT.md) 与 [台账](tracking/utils-gap-recovery.json)。后文旧“87/3”“B中未编译”或“主线齐全”表述为历史快照，不能覆盖本轮的未定位/未实现边界。

2026-10-14 主题三库更新：实际 `tiny-glade/crates/theme-defs`、`theme-fetcher`、`theme-server-proto` 的 B 已定位库内业务/合同完成静态复核和补全，26项离线测试、all-targets严格Clippy、fmt通过；交付锁138公共依赖身份匹配原B。没有请求真实主题服务。完整原入口oracle、游戏插件/内容消费及A/C尚未验收；维持限定候选口径，详见 [主题交付报告](evidence/theme-static-2026-10-14/REPORT.md) 和 [当前源码/机器台账](tracking/theme-static-recovery.json)。

更新：2026-10-12T10:08:01.154567+00:00。目标：完成一个 Rust 项目，完整复现当前原作构建的游戏实现与行为。

**当前不是完整游戏。整体完成百分比暂不报告：完整原作功能清单及验收分母仍在建立。**

进度必须区分：定位原作 → 结构/调用链确认 → Rust 实现 → 局部机器码差分 → 系统场景验收。仅反编译不算还原验收。

当前用户指定**源码恢复优先**；最新指示已开始渲染框架的Rust恢复与原码验证，场景和完整GPU画面验收尚未开展。源码关口与最终系统验收分列，局部差分通过不替代完整游戏集成。

## 共享基础库进度（2026-10-14）

三个库（utils / asset / asset-pipe）完成收口：实现层面 B 主线库内逻辑齐全（无"无业务实现"条目，见 `tracking/foundation-business-functions.json`），三库 `cargo test` + `clippy -D warnings` 全绿；但验收层面未完成——utils 现 **87 通过 / 3 失败**（差分测试）。已修复：`rectangle_deformation` 与 `rectangle_distance_normals`（`get_closest_edge_id` 0x140c9acb0=逐边线段距离首个最小、`expand_along_edge` 0x140c9c620=clamp 下界 0、`closest_edge_normal`=|local|/size 选轴）。剩余 3 失败：`circle_boundary`（MSVC vs macOS libm 平台差）、`curve2_rules`（+NaN/-NaN 符号）、`easing`（1 ULP）。`WallSpaceCoord::{bound,is_bounded,to_affine3}` 经逆向确认 B 中未编译、语义不可恢复。详见 `evidence/foundation-completion-2026-10-13/utils-gaps-2026-10-13/REPORT.md`。

| ID | 里程碑 | 最终状态 | 源码关口 | 前置验收 | 验收条件 | 证据 |
|---|---|---|---|---|---|---|
| M00 | 原作身份与研究输入 | 完成验收 | — | — | RNE/PDB 匹配身份；副本哈希一致；EXE 差异登记 | [baseline.json](evidence/binary/baseline.json)<br>[identity.json](workspace/input/identity.json) |
| M01 | 可构建 Rust 工程与证据追踪 | 完成验收 | — | M00 | Cargo 锁定构建成功；每个已验收算法对应原作地址/报告；追踪工具可复现 | [Cargo.toml](reconstruction/Cargo.toml)<br>[Cargo.lock](reconstruction/Cargo.lock)<br>[project-build-verification.json](evidence/project-build-verification.json)<br>[update-progress.py](scripts/update-progress.py) |
| M02 | 已定位 Roof 状态函数 | 完成验收 | — | M00 | Roof::new/ty/is_gable 的观察输出通过机器码差分；浮点/内存边界登记 | [roof_state.rs](reconstruction/roof/roof_state.rs)<br>[roof-state-verification.json](evidence/native/roof-state-verification.json) |
| M03 | 屋顶曲面核心 | 进行中 | 源码实现及原码校验通过：截面/profile/Curve2构造与采样完整CPU链；23810用例 | M00, M01 | 原作实际曲面类型及参数来源确定；曲面/导数/法线 Rust 可运行；各原作分支差分通过 | [surface.rs](reconstruction/roof/surface.rs)<br>[verification.json](evidence/roof-surface/verification.json) |
| M04 | 脊线与屋顶形状组装 | 进行中 | 源码实现及原码校验通过：脊长/脊向/尖顶偏移及世界点/线段；14776用例 | M00, M01 | 脊向/脊长/檐口/屋顶分支输入输出明确；完整几何子链 Rust 与机器码差分 | [ridge.rs](reconstruction/roof/ridge.rs)<br>[verification.json](evidence/roof-ridge/verification.json) |
| M05 | 铺瓦核心 | 进行中 | 源码实现及原码校验通过：矩形33输入19024条完整记录；圆形37输入11467条，其中12为完整原函数入口 | M00, M01 | 屋顶采样到瓦实例布局/索引/变换的完整原作链恢复；CPU/GPU职责明确；差分通过 | [pipeline.rs](reconstruction/roof/pipeline.rs)<br>[roof-pipeline-verification.json](evidence/roof-pipeline-verification.json)<br>[circular-verification.json](evidence/roof-tiles/circular-verification.json) |
| M06 | 屋顶编辑与完整场景验收 | 进行中 | 部分源码及原码校验通过：4514案例：已解析Roof/Event的编辑后段、变化量与数值通知规则；ECS查询/调度未完成；场景后置 | M02, M03, M04, M05 | 历史命令到状态/曲面/脊线/瓦实例贯通；矩形、圆形及原作其他已观察类型编辑与撤销验证 | [edit_rules.rs](reconstruction/roof/edit_rules.rs)<br>[verification.json](evidence/source-edits/verification.json)<br>[findings.md](evidence/source-edits/findings.md) |
| M07 | 世界数据/历史/保存恢复 | 进行中 | 恢复中：静态历史结构已解析；Rust消费者/字段映射待恢复 | M00, M01 | 33 历史样本消费者、版本迁移、ID与快照恢复实现；撤销/重做/重放与原作对照；缺失样本显式登记 | [findings.md](reconstruction/history/findings.md)<br>[sample-verification.json](reconstruction/history/sample-verification.json) |
| M08 | 墙体与砖/支撑/开口 | 进行中 | 部分源码及原码校验通过：6624案例：已解析状态的墙高约束后段及3个标高缓存叶函数；墙轮廓/砖/支撑/开口待恢复 | M01, M07 | 各实际原作形状与相交关系实现；墙编辑到实例输出差分；建筑场景验收 | [height_rules.rs](reconstruction/wall/height_rules.rs)<br>[wall-rules-verification.json](evidence/scope-priorities/wall-rules-verification.json) |
| M09 | 门窗/装饰/附着关系 | 未开始 | — | M06, M07, M08 | 放置/尺寸/变形/冲突/宿主变化与恢复实现；原作样本与局部规则验证 | 尚无验收证据 |
| M10 | 地形/道路/水/园景 | 未开始 | — | M01, M07 | 坐标/笔刷/曲线/插值及跨对象变化传播实现；对应原作场景操作对照 | 尚无验收证据 |
| M11 | 输入/拾取/任务调度 | 未开始 | — | M06, M08, M09, M10 | 原作时序与脏更新依赖明确；连续拖动、反向、撤销与异步任务一致性通过 | 尚无验收证据 |
| M12 | 资源/着色器/渲染 | 进行中 | 部分源码及原码校验通过：五批CPU/API合同统一门禁通过；第四批50/50、第五批27/27；新增Primary caller180、非空record774、descriptor创建5184原指令案例；生产sequence适配/并行queue/sparse layout/writes/完整pipeline/真实pass/GPU仍未验收 | M01, M06, M08, M09, M10 | 原作实际 pass、实例、材质与资源格式恢复；固定相机画面及性能对照 | [Cargo.toml](reconstruction/render/Cargo.toml)<br>[Cargo.toml](reconstruction/render/original-skeleton/crates/rhapsody/Cargo.toml)<br>[structure.json](reconstruction/render/original-skeleton/crates/rhapsody/structure.json)<br>[verification.json](reconstruction/render/original-skeleton/crates/rhapsody/verification.json)<br>[resource_info.rs](reconstruction/render/resource_info.rs)<br>[resource_usage.rs](reconstruction/render/resource_usage.rs)<br>[verification.json](evidence/render-recovery/verification.json)<br>[findings.md](evidence/render-recovery/findings.md)<br>[report.json](evidence/render-skeleton/report.json)<br>[rendering-framework-map.md](evidence/render-architecture/rendering-framework-map.md)<br>[rendering-framework-map.json](evidence/render-architecture/rendering-framework-map.json)<br>[BATCHED_RHAPSODY_RECOVERY_PLAN.md](evidence/render-recovery/BATCHED_RHAPSODY_RECOVERY_PLAN.md)<br>[summary.json](evidence/render-recovery/workgraph/summary.json)<br>[functions.json](evidence/render-recovery/workgraph/functions.json)<br>[calls.json](evidence/render-recovery/workgraph/calls.json)<br>[work-packages.json](evidence/render-recovery/workgraph/work-packages.json)<br>[contract.json](evidence/render-recovery/graph-compiler/contract.json)<br>[query-queue.json](evidence/render-recovery/graph-compiler/query-queue.json)<br>[integration.json](evidence/render-recovery/parallel/integration.json)<br>[contract.json](evidence/render-recovery/parallel-batch-02/contract.json)<br>[integration.json](evidence/render-recovery/parallel-batch-02/integration.json)<br>[contract.json](evidence/render-recovery/parallel-batch-03/contract.json)<br>[integration.json](evidence/render-recovery/parallel-batch-03/integration.json)<br>[RHAPSODY_PROGRESS.md](evidence/render-recovery/RHAPSODY_PROGRESS.md)<br>[contract.json](evidence/render-recovery/parallel-batch-04/contract.json)<br>[validation.json](evidence/render-recovery/parallel-batch-04/device-counters/validation.json)<br>[validation.json](evidence/render-recovery/parallel-batch-04/record-submit/validation.json)<br>[validation.json](evidence/render-recovery/parallel-batch-04/pipeline-shader-interfaces/validation.json)<br>[Cargo.toml](reconstruction/render/work-packages/graph-compiler-r2/Cargo.toml)<br>[Cargo.toml](reconstruction/render/work-packages/gpu-lifecycle-r3/Cargo.toml)<br>[Cargo.toml](reconstruction/render/work-packages/executor-temporal-r4/Cargo.toml)<br>[Cargo.toml](reconstruction/render/work-packages/barrier-record-r5/Cargo.toml)<br>[Cargo.toml](reconstruction/render/work-packages/sequence-domain-r5/Cargo.toml)<br>[Cargo.toml](reconstruction/render/work-packages/temporal-r5/Cargo.toml)<br>[TOOLING_AUDIT_2026-10-12.md](evidence/render-recovery/TOOLING_AUDIT_2026-10-12.md)<br>[README.md](evidence/render-recovery/tooling/README.md)<br>[runner-validation.json](evidence/render-recovery/tooling/runner-validation.json)<br>[integration.json](evidence/render-recovery/parallel-batch-04/integration.json)<br>[README.md](evidence/render-recovery/parallel-batch-05/README.md)<br>[integration.json](evidence/render-recovery/parallel-batch-05/integration.json)<br>[validation.json](evidence/render-recovery/parallel-batch-05/queue-callers/validation.json)<br>[validation.json](evidence/render-recovery/parallel-batch-05/record-resources/validation.json)<br>[validation.json](evidence/render-recovery/parallel-batch-05/pipeline-descriptors/validation.json)<br>[Cargo.toml](reconstruction/render/work-packages/queue-callers-r7/Cargo.toml)<br>[Cargo.toml](reconstruction/render/work-packages/record-resources-r7/Cargo.toml)<br>[Cargo.toml](reconstruction/render/work-packages/pipeline-descriptors-r7/Cargo.toml)<br>[MODULE_STATUS.md](reconstruction/rhapsody-rebuild/MODULE_STATUS.md)<br>[functional-project-validation.json](reconstruction/rhapsody-rebuild/evidence/functional-project-validation.json)<br>[README.md](evidence/render-recovery/consumer-chain-review-2026-10-12/README.md) |
| M13 | 生态/天气/音频/UI/相机 | 未开始 | — | M07, M11, M12 | 观察到的外围系统行为/参数/资源依赖逐项实现并验证 | 尚无验收证据 |
| M14 | 完整 Rust 游戏集成验收 | 未开始 | — | M06, M07, M08, M09, M10, M11, M12, M13 | 可运行交互游戏；全部确认的原作功能清单关闭；历史、场景、视觉、性能及异常恢复回归通过 | 尚无验收证据 |

## rhapsody 当前进度

新rhapsody-rebuild候选功能统一验收通过：64测试、严格Clippy、13真实GPU探针，25命令无失败；112输入/53外部依赖经只读复核。旧共享入口新增当前Command/compiled callback上下文，4链级场景通过。新目录自动首个shader callback仍为两阶段桥，原并行/跨族/资产关联/分叉与原整帧等价性未验收。

已完成统一门禁的批次：1、2、3、4。当前工作 agent 数：0。M12保持 `verified_partial`。

[模块进度与下一步](evidence/render-recovery/RHAPSODY_PROGRESS.md) · [工具审计](evidence/render-recovery/TOOLING_AUDIT_2026-10-12.md)

## 已验证局部算法

| 算法 | 状态/范围 | 实现 | 原作参照 |
|---|---|---|---|
| WindowSize::aspect_ratio | verified_local：仅局部 f32 结果，1280 输入 | [Rust](reconstruction/math/window_aspect.rs) | [差分报告](evidence/native/window-aspect-verification.json) |
| Roof::new/ty/is_gable | verified_local：仅字节写入/布尔返回；256 构造器 + 两函数各1348输入 | [Rust](reconstruction/roof/roof_state.rs) | [差分报告](evidence/native/roof-state-verification.json) |
| Roof surface/Curve2 | verified_local：23810用例，原指令/CRT/曲线链；异常范围见报告 | [Rust](reconstruction/roof/surface.rs) | [差分报告](evidence/roof-surface/verification.json) |
| RoofRidgeDims/tip | verified_local：14776用例，点/线段脊线核心 | [Rust](reconstruction/roof/ridge.rs) | [差分报告](evidence/roof-ridge/verification.json) |
| Rectangular CPU full pipeline | verified_local：33完整输入、19024条64字节记录；没有prepared geometry注入 | [Rust](reconstruction/roof/pipeline.rs) | [差分报告](evidence/roof-pipeline-verification.json) |
| Circular CPU tiles | verified_local：37输入、11467记录；25prepared/12完整入口 | [Rust](reconstruction/roof/tiles.rs) | [差分报告](evidence/roof-tiles/circular-verification.json) |
| Wall height numeric policy / cached heights | verified_local：6624案例；clamp仅数值后段，3个缓存叶函数含缺失panic条件；ECS/哈希实现排除 | [Rust](reconstruction/wall/height_rules.rs) | [差分报告](evidence/scope-priorities/wall-rules-verification.json) |
| Roof resolved edit core / feedback rules | verified_local：4514案例；完整88字节状态、changed tick、变化量、gable重建及反馈数值；已解析后段，不是完整ECS系统 | [Rust](reconstruction/roof/edit_rules.rs) | [差分报告](evidence/source-edits/verification.json) |
| Renderer resource graph rules | verified_local：访问链、生命周期事件、并发判断；依赖classifier外供；78867案例 | [Rust](reconstruction/render/resource_info.rs) | [差分报告](evidence/render-recovery/verification.json) |
| Renderer Vulkan usage conversions | verified_local：2个完整private用途转换叶函数；ash原版本复用；142149案例 | [Rust](reconstruction/render/resource_usage.rs) | [差分报告](evidence/render-recovery/verification.json) |

## 尚未关闭的边界

- 发行树/PDB没有取回作者原始Rust文本，当前为行为恢复源码
- Roof历史字段名字到ABI偏移和Width/Length bit映射待原消费者核实
- 屋顶CPU算法通过限定原码校验；Flat屋顶/完整ECS调用者及全部异常路径未完成
- 22个快照引用目标缺失
- 完整游戏、场景回放与GPU尚未验收；GPU源码按最新用户要求开始恢复

## 当前后置工作

- 完整场景回放与运行交互验收仍后置
- GPU/渲染按用户最新指示开始源码恢复；完整shader动态/画面验收尚未开展

## 下一批可执行任务

1. rhapsody：按新目录MODULE_STATUS/统一功能验收记进度；将主图registry与正在录制的Command注入首个shader callback，闭合managed descriptor→同一submit/completion；随后核对原并行/跨族、资产关联/缓存与分叉依赖，禁止按路径/函数数计完成
2. S01/S03/S04：恢复Roof/Wall权威类型、create/update/edit状态消费者和事件触发；复用Bevy查询/容器
3. S02/S05：定位特殊墙高flag生产者，恢复height history提交/撤销与游戏对象合法性规则
4. S06/S07：优先恢复墙轮廓、construct_brick_columns与支撑源码；复用glam/RNG/half
5. S21/S22：恢复原rkyv/ron fork之上的游戏schema与版本迁移；不运行场景回放

工程：[Cargo.toml](reconstruction/Cargo.toml)。完整研究计划：[MASTER_PLAN.md](MASTER_PLAN.md)。源码任务与优先级：[TASK_BACKLOG.md](TASK_BACKLOG.md)。依赖排除与持久记忆：[PROJECT_MEMORY.md](PROJECT_MEMORY.md)。机器可读台账：[project.json](tracking/project.json)。

依赖列描述最终验收依赖，源码阶段可以沿已确认的数据接口继续推进。排期以实际源码覆盖和验证吞吐滚动更新，不承诺未经测量的整游戏完工日期。更新台账后执行 `scripts/update-progress.py`，它会核验已完成项的依赖、证据文件与局部差分结果。
