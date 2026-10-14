# 本次源码归宿审计

本次由 country-core、渲染/HLSL、基础库三个 agent 分别复查，集成 owner 另核对早期建筑、历史与地形来源。核查对象是 Git 保存的函数体、类型、原码/静态证据与消费者接口，编排脚本不计恢复成果。

## 整理纠正

上一版以系统整体未完成为由排除了该系统已有实现，因此 country-core 只剩8个局部模块。本次按已完成函数闭包补回渲染、资产注册、Nani、pass、帧编排与较早建筑业务；21个旧渲染工作包逐项确认归宿，又补回 descriptor-binding-r8。415个HLSL均保留，包括此前漏掉的2个main与20个header，原EDGE/死depth限制分别登记。

paths/lang-lib/tabloid保留最新产品修复，而不再回滚旧快照。未知方法与静默假GPU成功接口被移除或改为明确外部服务边界；测试recorder限cfg(test)。未确认引擎fork不编造；通用依赖直接使用原B身份。

## 详细来源和功能级审计

- [tg-text 内部规则收口](text-internal-audit.json)：原角色/Arc表、基准与scaled metrics、u32 frame/stamp与精确cache状态规则；829受控原代码案例、Debug/Release各33测试和严格Clippy通过。替换先前该范围的适配实现，保留原源码/历史审计；真实游戏调用方与GPU接线未改。
- [完整性/文字后续恢复](foundation-followup-2026-10-14.json)：当前两个开发 crate 的规则修复、Windows 原指令边界与 31 项联合测试。后续源码 hash 已按两包范围更新 sources.json，原导入 Git/源文件 hash 与初始快照保留；[本轮报告](../../reverse-engineering/tiny-glade/evidence/small-libs-followup-2026-10-14/REPORT.md)列出角色模型、frame 与真实 Texture 消费边界。
- [country-core 来源映射](../crates/country-core/render-source-map.json)、[模块 README](../crates/country-core/README.md)及country-source-coverage.json：源码范围、适配原因、未纳入骨架与候选体。
- [rhapsody 来源映射](../crates/rhapsody/migration-sources.json)、[21工作包归宿](../crates/rhapsody/WORK_PACKAGE_AUDIT.md)、[逐源工作包清单](../crates/rhapsody/work-package-migration.json)。
- [基础库审计](foundation-audit.json)：14个包、源工作树与Git接受状态、已验证fixture选择、产品差异和原报告。
- [建筑/历史/屋顶/地形与开发接线](building-audit.json)，[建筑 CPU README](../crates/country-core/src/recovered_building/README.md)。
- [树/水/小生物/音频/clutter独立函数](small-system-audit.json)、[对应模块 README](../crates/country-core/src/recovered_systems/README.md)。完整草稿含未知生产者，不能整体搬入；完成leaf仍单独保留。
- [相机审计](camera-audit.json)：45度常量已纳入，矩阵复用已验收utils；其余停止时候选尚无Rust原码闭合证据。
- [当前源树覆盖表](source-coverage.json)、[统一源码映射/哈希](sources.json)。覆盖分类不是文件数或同名函数等价判定；具体功能范围以专项审计及README为准。

## 各 package README

每个正式成员声明readme=README.md；README含概述、功能、原逆向文件位置、Git或证据来源、已完成和缺失部分。包名与原B同名只是归类线索，不代表整个原包已恢复。

| package | 功能与来源说明 |
| --- | --- |
| `tg-terrain` | [README](../crates/systems/terrain/README.md) |
| `tg-building-runtime` | [README](../crates/tg-building-runtime/README.md) |
| `tg-history` | [README](../crates/tg-history/README.md) |
| `tiny-glade` | [README](../crates/tiny-glade/README.md) |
| `country-core` | [README](../crates/country-core/README.md) |
| `system-roof` | [README](../crates/systems/roof/README.md) |
| `rhapsody` | [README](../crates/rhapsody/README.md) |
| `shader-pipeline` | [README](../crates/rhapsody/crates/shader-pipeline/README.md) |
| `utils` | [README](../crates/utils/README.md) |
| `paths` | [README](../crates/paths/README.md) |
| `lang-lib` | [README](../crates/lang-lib/README.md) |
| `tabloid` | [README](../crates/tabloid/README.md) |
| `motomoto` | [README](../crates/motomoto/README.md) |
| `tg-package-integrity` | [README](../crates/tg-package-integrity/README.md) |
| `theme-defs` | [README](../crates/theme-defs/README.md) |
| `tg-text` | [README](../crates/tg-text/README.md) |
| `asset` | [README](../crates/asset/README.md) |
| `asset-pipe` | [README](../crates/asset-pipe/README.md) |
| `tg-iced` | [README](../crates/tg-iced/README.md) |
| `tg-gpu-selector` | [README](../crates/tg-gpu-selector/README.md) |
| `theme-fetcher` | [README](../crates/theme-fetcher/README.md) |
| `theme-server-proto` | [README](../crates/theme-server-proto/README.md) |
| `shader-compiler` | [README](../crates/rhapsody/crates/shader-compiler/README.md) |

## 未纳入的内容

2026-10-14 用户明确排除 `tg-integrity-report`、`crash-reporter-gui`、`feedback-reporter` 和 `geforcenow`：从当前 workspace 与源码清单移除，不需要继续逆向还原。旧恢复目录、原锁文件及历史证据保留为历史记录，不计当前交付或待恢复范围；功能影响见根 [README](../README.md)。

原country-core骨架中的todo、typed Gap、空消费者、写死展示数据与未确认类型不算完成实现；marker-free文件也另读函数体，不能只凭没有todo推断恢复完成。原camera/critters停止快照、tree/critters-water/audio/clutter完整草稿仍含猜测输入和调度，但明确证据的独立函数已抽入recovered_systems。

旧terrain readback记录为proposed且未接线/原码验收，因此不搬入；原渲染oracle、JSON runners、示例固定handle状态机与被新证据纠正的旧模型留作调查证据，规则归宿另有逐工作包说明。用户tiny-garden demo未参与本次归类或导入。

## 验证与限制

当前结果见 [validation.json](validation.json)，旧导入报告另存validation-initial-import.json。默认/all-features编译和单测验证工程能实际编译恢复体；shader配置/反射测试不等于本轮重跑全部DXC数值oracle。真实GPU桥、完整原ECS调度、完整游戏场景/画面与全输入域仍未在本次验收。实验/候选feature和唯一候选依赖在清单中明确登记，默认关闭。

用户要求不再重跑：后续完整矩阵已取消。仅汇总缺口、来源与已有验证记录。最终工具接口裁剪后的87项utils测试已由模块owner完成，全workspace1480项是较早实际运行结果，未执行的最终全feature测试不填为通过。

## 收尾时发现的当前缺口

最终仅文档收尾期间，workspace成员从先前验证的27个变为25个，`crates/tg-integrity-report` 和 `crates/crash-reporter-gui` 的目录及成员被移除。原来源位于 `reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/tg-integrity-report` 和 `crash-reporter-gui`，旧审计映射仍作为历史记录保留。三个审计agent未报告执行这项删除，操作来源和原因尚未确认；当前不能将这两个包写成已存在或已完成集成。

按照用户要求，此时不恢复源码、不更新依赖锁或重跑编译测试，仅登记实际状态。上述完整workspace编译/测试记录对应删除前的27成员状态，不代表当前25成员及最终依赖闭包已经重新验证。后续开发需确认这两包移除意图及其GUI/完整性报告/崩溃上报功能归宿。
