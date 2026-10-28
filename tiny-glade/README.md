# Tiny Glade

持续开发的 Rust workspace，按原 B 功能结构整理已恢复游戏源码、渲染框架、自有工具和 HLSL。集成入口为 crates/tiny-glade；各 package README 说明模块功能、原逆向文件位置、Git 来源、已完成内容与缺口。

## 逆向证据入口

本工程是正式实现。原版行为、PDB、差分证据与可编译验证工程在 [`../reverse-engineering/tiny-glade/`](../reverse-engineering/tiny-glade/README.md)，按需查阅，不要整体加载：

| 需要 | 打开 |
|---|---|
| 当前模块状态 | [`reports/business/2026-10-23-module-recovery/STATUS.md`](reports/business/2026-10-23-module-recovery/STATUS.md) |
| 原版证据索引 | [`../reverse-engineering/tiny-glade/evidence/README.md`](../reverse-engineering/tiny-glade/evidence/README.md) |
| PDB / 符号 / 结构 | [`../reverse-engineering/tiny-glade/skeleton/README.md`](../reverse-engineering/tiny-glade/skeleton/README.md) |
| reconstruction 与正式工程差异、移植清单 | [`../reverse-engineering/tiny-glade/reconstruction/PORTING-CHECKLIST.md`](../reverse-engineering/tiny-glade/reconstruction/PORTING-CHECKLIST.md) |
| MCP / Ghidra / ReVa | [`../reverse-engineering/tiny-glade/docs/current/AI_REVERSE_TOOLS.md`](../reverse-engineering/tiny-glade/docs/current/AI_REVERSE_TOOLS.md) |

先读 [`../reverse-engineering/tiny-glade/README.md`](../reverse-engineering/tiny-glade/README.md) 的“按任务直达”和“渐进式披露”，再进入具体目录。

## 功能归宿

| 内容 | 工程目录 |
| --- | --- |
| 渲染资源、Nani、pass builder、帧与 compute/draw 调度 | [country-core](crates/country-core/README.md) |
| 建筑权威状态、生命周期、几何、洞口、附着与历史 CPU 业务 | [墙系统与资源](crates/country-core/recovery/README.md) |
| 树、水、小生物、音频和 clutter 的已完成独立 CPU 函数 | [各资源和系统模块](crates/country-core/recovery/module-migration-2026-10-18.json) |
| 屋顶表面、脊、瓦片、编辑和回放 | [system-roof](crates/systems/roof/README.md) |
| 真实 Bevy headless 建筑开发接线 | [tg-building-runtime](crates/tg-building-runtime/README.md) |
| 历史 schema 和地形编辑 | [tg-history](crates/tg-history/README.md)、[tg-terrain](crates/systems/terrain/README.md) |
| GPU 资源、render graph、录制/提交 | [rhapsody](crates/rhapsody/README.md) |
| shader 协议、预处理、反射与编译 | [shader-pipeline](crates/rhapsody/crates/shader-pipeline/README.md)、[shader-compiler](crates/rhapsody/crates/shader-compiler/README.md) |
| 415个 HLSL 文件与638个原 stage 配置 | [shaders](shaders/README.md) |
| 自有数学、曲线、几何和网格工具 | [utils](crates/utils/README.md) |
| 资产、路径、语言、文字、主题、完整性、GUI和平台支持 | 对应 crates/*/README.md，完整包表见 [审计](provenance/AUDIT.md) |

2026-10-17：主题三包合并为 [tg-theme](crates/tg-theme/README.md)，提供经过验证的活动列表、事务缓存与可取消订阅；本包最低 Rust 1.89，其余 workspace 包继续采用各自声明。 [tg-iced](crates/tg-iced/README.md) 已按对话框策略、原生运行及截图拆分，原生命令适配错误由调用方处理。历史导入快照保留，不再代表当前重构代码逐文件一致。

按函数体和证据边界收纳，不能因完整系统尚有缺口就丢掉它已恢复的子模块。旧 country-core 570文件覆盖快照含大量骨架；文件数量不代表恢复完成。未知方法不填猜测默认值；未恢复服务保留明确输入/后端接口。草稿原目录保留，并列出未纳入原因。

## 已删除的报告与云平台功能

2026-10-14 按用户要求删除以下四个 crate，并从 workspace、依赖锁和当前源码清单中排除；这些功能不需要继续逆向还原。

| 已删除的 crate | 功能影响 |
| --- | --- |
| `tg-integrity-report` | 不再提供安装完整性检查结果的报告窗口和多余文件删除入口。 |
| `crash-reporter-gui` | 不再提供崩溃诊断、报告同意及发送结果对话框。 |
| `feedback-reporter` | 不再提供反馈事件处理、文件收集、ZIP 打包和 HTTP 提交。 |
| `geforcenow` | 不再提供 NVIDIA GeForce NOW 云环境识别、SDK 加载和会话集成。 |

以上属于用户明确缩减的项目范围，不列为待恢复缺口，也不重新导入。历史恢复源码、原作依赖锁和旧证据保留供查阅。既有测试报告保留其原验证范围；删除后的结构、清单检查及检查限制见 [scope-removal-validation.json](provenance/scope-removal-validation.json)。

2026-10-24：按用户要求删除许可证服务和整个 Credits 制作人员页面，包括菜单入口、页面状态、滚动控件和专属测试，并移除其 HTTP 依赖。原作分析证据保留为历史，不列为当前待恢复业务。当前模块状态与验证见 [业务台账](reports/business/2026-10-23-module-recovery/STATUS.md)。

2026-10-24：用户明确将真实 Steam SDK 接入排除出本轮修复范围（TB-R8），保留当前注入接口及 host 启动方式；不作为 toolbar 收口阻断。相关决定与其余修复见上述业务台账。

## 开发与验证

```bash
cargo check
cargo check-all
cargo test-all
cargo lint
cargo fmt --all --check
python3 tools/verify_inventory.py
```

Windows 环境可将最后一行改为 `py -3 tools/verify_inventory.py`。

根 Cargo.toml 管理 workspace、原公共依赖/fork、版本、最低 Rust、lint 和构建配置；每个包声明自己的功能与 README。恢复源码使用 let-chains，根声明最低 Rust 1.88；实际测试工具链在验收记录单列，不以新版本机编译成功冒称1.88已验证。默认入口是 library，游戏启动、窗口、实际 GPU/backend 和全场景仍需继续开发。

公共依赖按 provenance/original-Cargo.lock 的 B 版本/来源复用。原 Bevy fork 仓库/commit 未核实，不编造。原 gpu-allocator 本地 fork 未找回；可选 rhapsody/candidate-suballocation 使用明确登记的0.27候选，默认关闭，不计原依赖身份恢复。其它实验/候选功能见各包 feature 说明。

## 历史 Git 审计

以下记录针对早期仓库提交历史；当前工作区于 2026-10-25 建立了新的根仓库，未导入旧 Git 历史；以下提交号与推送描述仅作历史留存。

重点核对 b5965593、664f6a65 的 country-core 恢复、52a14460及后续 rhapsody 修复、81aa0042 的 HLSL、早期建筑/历史和基础库提交。上一版整理把部分系统未完成错误地当作整包不纳入，并将 paths/lang-lib/tabloid 选回旧快照；本次补回完成实现，保留最新产品修复，并在包 README 记录其原行为差异。

415个 HLSL 包含155个 header和260个 main 文件。恢复的 ice_refl_resolve/final_post 源码保留，同时保留 EDGE、原空/死 depth 路径和数值限制，不改标为全域 SAME。源码完成、特定原码对照和实际 GPU 接线分别验收。

来源映射与快照在 [sources.json](provenance/sources.json)，专项复核与排除原因在 [AUDIT.md](provenance/AUDIT.md)。Cargo/mod/trait 开发接线不计新逆向成果。旧导入快照和报告保留为 sources-initial-import.json 与 validation-initial-import.json。

当前编译、测试和清单结果见 [validation.json](provenance/validation.json)。历史原指令差分、静态分析和GPU证据仍在同工作区的 [`../reverse-engineering/tiny-glade`](../reverse-engineering/tiny-glade)。编译与单测不能推断整游戏、全GPU帧、全输入域或 A/C 版本等价。
