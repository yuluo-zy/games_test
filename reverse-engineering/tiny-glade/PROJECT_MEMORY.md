# 读取与维护规则（2026-10-23）

本文件是带日期的决策/经验档案，不是每个任务的必读上下文。先按模块名、任务 ID、地址或日期用 `rg` 搜索，只读命中条目及必要相邻段落；当前状态以活跃任务报告和当前源码为准，旧状态仅作历史证据。新增内容只记录会影响后续实现/验收的长期决策、纠错和可复用流程；临时进度写入对应任务报告，避免重复复制报告全文。

## 2026-10-19 最新优先级：集中完成启动→装配→主循环

用户明确要求把并行力量集中到这一条主流程。T16/归档等编辑候选冻结留存，不丢弃既有交付；8专项转为会话初始请求/CLI、卸载与Reload、预置树、真实环境/森林/墙drawable startup、相机与步行音频、装配及独立二次会话验收。中央单一Cargo owner。任务记录见 evidence/round12/startup-focus/dispatch.json。T13继续排除。

已中央合入：v23墙候选除stale run_game（21文件）、preplaced15、完整GCC10、会话lifecycle14、LoadUI36/Parsed缺主history修复、真实Started/Finished及Vermeer持久locals、原check_load_completion sticky/unlink36、真实resource-only装配。候选SHA逐一审计，现正fresh all-targets check，不计验收通过。旧v20实际exe运行241s在临时Decorator完整UI装配处B0002失败；其结构stub不得仅改ParamSet后冒充恢复，改为保真实资源并只挂已恢复叶。v20后因Load test类型/私有字段失败已结束，120源变化使它不是新source的验收。

新发现真实启动gap：初始NextSession由实际SessionFlowPlugin.build创建，已撤销“缺writer”的早期误判；首次intro是v16，必须经过legacy SpawnPreplacedTrees；原墙7全局drawable未生成；init_clearing完整6913B/forest80B producer仍缺；Walk footsteps真实8参数/rock/raster/input算法当前简化，专项恢复中。原Reveal writers分别来自MainLoop交接和Vermeer完成，不在load_game，不能凭request_reveal新造等待。首次非空+同主题二次+Summer→Winter验收未完成。

## 2026-10-19 最新中央并行验收检查点

当前上限为9个并发槽（含中央），本轮以8个专项agent并行，旧文的4槽记录仅为较早阶段。中央唯一Cargo owner，生产冻结后再验证；释放的成员动态转交屋顶UI、Load菜单、startup卸载链与独立复核。T13明确排除；其它复杂算法/完整交互不可简化。

独立执行：音频真实生产源43/43生命周期测试通过；MaterialStats对原B实际执行530/530位模式与selector匹配。v14实际GPU Egui单样本及原sRGB4×MSAA两测试通过；capture及实际gameplay/历史测试此前失败，已给候选修复但未重跑，不算通过。新roof原RNE28组执行不等于Rust数值验证通过。

已落但待统一验收：历史9/16变体身份、plaster/support/AutoClutter/Deprecated Decorator保存及nullable NaN往返；完整旧版本迁移、identity权威流与snapshot cursor；timelapse生命周期；Escape真实GUI及OS命令接线；roof tile/spire/trigger；墙typed GPU分配与样式消费者。Load菜单和颜色show/main、roof UI仍在恢复。原支撑梁只有静态零count channel证据，不编造非空producer。

最新发现的完整性断点：第二次同主题会话placement库重复load，需恢复整个unload/despawn生命周期；<19预置树事件有原证但消费者组件归属颠倒（原PrefabInstanceData48B、PrefabInstance88B），不能只注册writer宣布完成。最大缺口仍输入→权威→历史→实体/上传→实际绘制。

升级flag裁决：原wrapper d→bit40/window（GateColors及非邻接冷块三窗Decorator色），e→bit48/ArrowSlit999。两次独立核实完整CFG，不能因旧helper命名或截断连续反汇编再次翻转。webbrowser实际原锁是0.8.10，已按原依赖复用。

# Tiny Glade 项目记忆

## 2026-10-19 内网 LLM 委派与长函数基准

用户要求尽量把重复、独立且边界清楚的逆向分析/核对交给内网模型。已接入 `scripts/llm-worker/run.mjs`，密钥只读进程环境；中央入口为 AI_REVERSE_TOOLS.md §10，后续提示词 `scripts/llm-worker/review-production.txt`。实测模型 `deepseek-ai/DeepSeek-V4.1-Flash`；标准 thinking disabled 在本网关未生效，有效参数为 `chat_template_kwargs`，语义任务默认 `thinking:true, reasoning_effort:25`，最多 2 独立请求并发，冻结输入/缓存与格式门禁。快速模式在长函数和窄任务均出现关键错误，不能直接接受；低强度也只作助手，主流程保留原指令/ABI/跨模块/写入与集成裁决。T13 仍排除。

原 B replay_stairs 703 非空行/2373 原字节长函数基准已完成工具评估，未自动接受完整合同。最终原汇编独立任务约 49/45 秒，事件公式经主流程核对正确；数值指令事实需纠正 worker 把“加时间等待”误叫“应用命令”的标签。原指令进一步确认：事件 tag 根据 payload bit（当前 schema 的 recreate）选择 `0x11+undo` / `0x12-undo`，不是 instant 标志；当前 Rust 音频 kind 分支存在静态不一致，已登记未改码。NaN 等比较以 MAXSS/UCOMISS/JBE 原指令为准，初始反编译 checklist 的强结论已纠正。完整失败/性能/证据/裁决见 `evidence/llm-worker-2026-10-19/{REPORT.md,owner-adjudication.json}`。本轮没有修改游戏代码、数据库或提升游戏还原进度。

## 2026-10-19 后续执行范围：排除 T13，其余多 agent 推进

用户明确要求 **T13 反馈捕获不推进、直接跳过**。保留历史证据，不恢复其截图捕获/页面状态生产者，不把 T13 计入当前主流程完成阻碍。其它未恢复、未知、未验证内容继续集中多个 agent 推进，目标是完整 producer → 权威状态/算法 → consumer → 交互副作用调用链；不能以单消费者接线或一次有界增量结束代替原任务完成。

用户随后要求总结逆向经验并持续优化效率。已将本轮确认有效的方法补入现有 `skeleton/workflow-design/tiny-glade-recovery/references/original-driven-integration.md`：按完整调用链分工、公共源冻结与单owner编译、独立target避免其它任务锁、Git sparse保留证据精确提取、MCP异步失败判别、TypeId/Time字段与实际caller-set纠错、真实存档变体→回放消费者、真实App共用装配验收、画廊首格与captured纹理同帧依赖/Arc生命周期。效率提升不得以简化算法、删除交互或弱化验收换取。round12在途，不将新增源码自动记为通过。

用户进一步要求增加一批agent、root作为中央协调。当前环境同时上限4（含root）；旧startup/history/render成员交接结束后已新建session_owner、roof_visual、menu_interaction三专项，分别负责完整会话拥有数据转移、屋顶实体/几何/GPU、Load/Escape完整交互。root集中共享文件/接口裁决与单一验证，空槽继续轮换颜色菜单、墙GPU和旧版本迁移。进度和真实验收状态见 [round12报告](evidence/round12/REPORT.md) 与 [工作边界](evidence/round12/WORKLIST.md)，不能把已准备补丁算已落或把编译算通过。

## 2026-10-19 用户最高优先还原约束：正确性与完整交互

用户明确要求：还原首先确保**游戏逻辑的正确性、算法的正确性和设计的正确性**；**原作复杂算法和业务逻辑都不允许简化，保证整个游戏的交互完整性**。允许多态等设计更新，但设计变化必须保留全部业务语义，不得借架构适配删减功能、分支、状态、边界处理、副作用、生命周期、事件顺序或跨模块联动。

执行时以原版 producer → 权威状态/算法 → consumer → 交互结果为恢复单位。现有实现若比原作简单，应按原证重构现实现并同步修复消费者；不能只包装接口，不能用默认值、空操作、假数据或绕过未恢复路径获得假成功。证据不足须继续取证并保留明确未闭合项；编译、局部测试、类型覆盖、可构图均不等于完整交互或原版算法验收。此要求适用于当前三个主流程及后续全部 Tiny Glade 还原，与保留用户已重构 rhapsody/工具库架构的要求同时生效。

## 2026-10-19 三主流程继续恢复：round11

先fetch并快进合并62提交到1253c0ab，三方移植本地round9/10业务，未修改拉取后的rhapsody库。root本地Git仅为历史Mac非法文件名设core.protectNTFS=false，防索引刷新截断；拉取前186文件与补丁/哈希均在evidence/sync-20261019-local-backup，不覆盖最新API。

启动侧纠正load_glade_grading职责为ImageFormationConfigs三槽，独立真实加载全LUT与tony_mc_mapface/menu-active-lut；meadow真实48B placement→GPU owner→draw；纠正旧13相机槽/Modifiers/动作参数误读，复用全长原证恢复六body、Normal/Free/Walk、set9/10、ray/MouseDepth与完整snapshot磁盘UI。当前宿主录制格式2→3携depth/mouse/center，拒绝缺值旧格式，不声称原208B ABI。terrain平面成功创建先写raw distance，释放无条件清；两App单次MEI flush，构图重试只复用输出。

历史侧纠正两张所谓结构表实为stairs backup GraphMap，支持非空aux Pop；mixed命令/timing同步压缩；真实color/FlatRoof/tree/prefab消费者保完整placement、Entity去重、GPU/RT/audio/BirdHaven副作用。新增原证纠正：RT所谓延迟callback实际insert<BirdHaven>，不得再造RT请求队列。进一步修复HistoryEntry未持自身aux而错误回放global最近diff；每entry两Arc与方向性cull表直接由选中entry生成，不能退回索引映射或None。

渲染侧接T12 canonical String LUT、T16全部query/draw分支/唯一组件身份及真实layer-view、T21同MSAA owner；T11/T13按真实EscMenuState门控，不用默认false绕过。T11完整host、T12 preview捕获、T13 capture/state、其余尾部控制/截图，Walk入口teleport、其它startup/full UI输入、保存aux wire解码及颜色GPU等仍未闭合。三流程保持部分，不宣称完整compile_main/warmup/游戏交互。最终源码/日志指纹及测试以[evidence/round11/REPORT.md](evidence/round11/REPORT.md)和verification.json为准；子报告旧在途数字不是最新验收。

## 2026-10-18 用户最新约束：保留已重构的 rhapsody 架构

用户明确指出 `games_test/tiny-glade/crates/rhapsody` 已经重构：恢复整条游戏链时，应当给当前 rhapsody **添加所缺功能和修复缺陷，绝不能按原版再重新复现一遍 rhapsody**。当前 `crates/rhapsody` 的 API、资源所有权、渲染图与执行架构是工程基线。原版逆向用于确定游戏业务、调用顺序、参数和生命周期要求；游戏侧及宿主适配层优先对接现 API。库内确有缺口时作有界增量修复并保留验证，不引入平行 legacy 后端、不复制原内部 ABI、不回滚用户重构。此约束优先于旧恢复流程中的 `reconstruction/render` 单一入口等历史约定。主链当前记录见 `tiny-glade/reports/main-chain-recovery-2026-10-18/ORIGINAL-FIRST-REVIEW.md`。

用户随后补充：保留架构不等于保留简化算法。如果当前手工实现过于简单，而原版证据确认还有复杂算法、必要步骤或状态处理，应直接重构现有手写部分并修正消费者，复用已有可用能力，不能只在简化实现外面包装接口。该解释同样适用于 rhapsody 与 core。

本轮可复用经验已补入现有专用 skill：`skeleton/workflow-design/tiny-glade-recovery/references/original-driven-integration.md`，并从 SKILL.md 路由，技能格式检查通过。内容包括构造/CPU/shader 交叉核对、None/Some 各型合同、canonical 类型身份、同步与内容依赖分离、真实非空 GPU 回读、Timeout 分阶段定位及共享树验证指纹；不把当前未证实的 Nani Timeout 原因固化成规则。当前进度以 `tiny-glade/reports/main-chain-recovery-2026-10-18/ORIGINAL-FIRST-REVIEW.md` 及绑定源码/二进制的验证日志为准，不从历史统计推完成比例。

## 2026-10-18 旧恢复集合按工作区归类

用户要求迁出并删除 country-core/src/recovered_building 与 recovered_systems，随后明确“应该归类到工作区”。本次按已有 crate 与资源/系统所有权归类，替代早期“新增逻辑全部放 country-core”的组织约定：树删除判定回 system-tree/process_tree_cmds，小生物 locomotion 回 system-critters/components，水音频位投影回 system-water/water_audio_grid；core 保留墙/屋顶/历史资源、现有 core 消费者和核心音频/WorldRaster 合同。不要再新增统一 recovered_* 或共享收纳目录。当前路径及迁移前源码指纹见 tiny-glade/crates/country-core/recovery/module-migration-2026-10-18.json。此为源码归属/调用迁移，不提高原码行为验收；矩形历史回放、水音频生产资源不匹配、全局注册/GPU 缺口保持。


## 2026-10-18 components 审查与现有占位恢复

用户要求结构/注释/review 和全部现有 `_gap`。B 原注册描述符+创建/消费指令确认：TreeAnimation 16B=duration/progress/canopy_visibility/hidden_this_frame（+0/+4/+8/+12），TreeMoveBend 8B=XZ 缓动位置，Ui2dImage 32B=String+u32 layer@24；TreeComponent 16B=id+preplaced，ConvertedPreplacedTree 16B=PrefabName。UiDrawOrder 改 u64；Default ECS 克隆组件移除意外 Clone/Copy，Nani 网格选择器共用且不再误作 Component。UI/树/渲染拆分，旧公共导出兼容。树删除必须读 progress@4；forest 移除查询序 Local<Vec> 进度，改由实体组件持有；toolbar 不再丢 label。

当前相关四库 --lib 编译及 5 针对性测试通过；26 段 analyzed/RNE 静态字节一致，不是原进程行为 oracle。all-targets 的 pass_kit/test_env 仍旧 import_unowned_texture trait 方法；workspace --lib 的 system-water 水格字段不匹配，本轮均未改。其余 registry 类型/放置 GPU 写面/全局 canopy/原调度/InsertTreeCmd 原 flags 仍未闭合；旧空标记/alpha+gap 报告作历史。报告与指纹入口 tiny-glade/crates/country-core/recovery/components-2026-10-18.{md,json}；工作目录 skeleton/task-index/work/TG-abf1305389c44cc264/components-review-20261018。

## 2026-10-18 本机 Vulkan 配置与 GPU 验证（长期环境事实）

用户要求修复加载配置并验证。**本机已有 Vulkan/MoltenVK，禁止再把 dlopen 查找失败说成未安装。** loader=`/opt/homebrew/lib/libvulkan.dylib`，MoltenVK=`/opt/homebrew/lib/libMoltenVK.dylib`，ICD=`/opt/homebrew/etc/vulkan/icd.d/MoltenVK_icd.json`。真实设备 Apple M3，MoltenVK 1.4.2、设备 Vulkan API 1.4.357。

当前 `tiny-glade/.cargo/config.toml` 默认设置 `RHAPSODY_VULKAN_LIBRARY=/opt/homebrew/lib/libvulkan.dylib`（force=false，用户覆盖优先）；`GraphicsInstanceLoader::new` 读取变量后调用已有候选加载路径，失败仍保留系统 fallback。只加 `DYLD_FALLBACK_LIBRARY_PATH` 的首次尝试失败，Cargo 重写测试进程的动态库搜索路径；不要重复依赖该不稳定配置。直接运行已编译二进制不读取 Cargo 配置，须显式传 RHAPSODY_VULKAN_LIBRARY。

**环境必须区分两层**：普通执行沙箱禁止 Metal，loader 修复后仍会报 ERROR_INCOMPATIBLE_DRIVER；这不是库缺失或硬件不支持。允许 GPU 的宿主执行已枚举真实 Apple M3，并用 `RHAPSODY_REQUIRE_VULKAN=1 cargo test -p tiny-glade --test core_render_bridge --locked --offline -- --nocapture` 实跑 **2 passed / 0 failed / 0 ignored / 0 SKIP**，覆盖实际上传回读和构图失败/异常取消回收。严格标志不可去掉以获得假成功；测试内主动 game builder panic 被捕获属于预期。

证据：`tiny-glade/reports/vulkan-loader-config-2026-10-18/{README.md,verification.json,strict-bridge-host.log,vulkaninfo-host.log}`；失败尝试和沙箱日志一并保留。仅确认该机器配置及这两条 GPU 链，不提高整游戏/完整帧/原作等价验收。没有重装组件、改锁或改游戏业务。


## 2026-10-16 toolbar 开源来源核查

仅 B 原发行：本轮重新导出原 PDB 文件表，31 个有记录的 Taffy 源文件 SHA-256 全部匹配 crates.io 0.3.13 发布包，包哈希匹配原 Cargo.lock。toolbar 直接使用 Taffy；8 个定点函数的完整 PDB 代码范围在原 RNE/patched EXE 一致，MCP 证实 TaffyLayout→Taffy compute_layout/layout、文本→tg_text::TextFragment::render。mei 还含自己的布局/节点/模板/输入，不把其 compute_min_size_with_children 说成 Taffy 算法。整体应描述为游戏自有 toolbar/mei 加开源依赖；未找到整个 mei 来自哪一个公开 GUI fork 的证据，不证明从零原创。cosmic-text 的 PounceLight fork 经 tg-text 接入；iced fork 用于辅助 GUI，不能据依赖出现就判定 toolbar 是 iced/egui 改版。报告：`evidence/toolbar-origin-2026-10-16/REPORT.md`。无业务代码修改，不增加还原进度。

## 2026-10-15 core GUI 拥有状态与实现优先

用户纠正：先推进原版业务与算法实现，不再花大量时间扩展测试；只保留必要编译检查，疑义定点查二进制，后续由用户重新验收。仍只做 B，全部代码落 country-core。gui.rs 已补 EscMenuState/LoadMenuState/Backup/Settings/Video、Next/Prev 默认与移动、原菜单推进顺序、缩略图 positions 和调试标签；LUT 消费同一菜单/metrics，SessionId 复用原锁 uuid 1.15.1。资源原 TypeId 证明 LUT 读的是 EscMenuState，不是旧代码的 AppMode。+0x30 原为共享 Arc metrics，非字符串；time_update 内联证明 allow_time_passage 与限帧策略不同。

当前新增拥有状态仅 all-targets 编译通过，未扩展测试。先前 GUI 前缀/缩略图原对照保留限定有效，旧 foundation verification 标为历史，不用实时 inventory 证明旧哈希。当前 28 直接依赖，锁仅增加 core -> 既有 uuid 边。下一文件仍 gui.rs 的 PhotomodeScreen、完整 DebugGui/log、PendingEguiContextResource；状态 tag10/报告 flag/警告 ID 的精确命名、全局调度和持久化 sink 尚未闭合。具体见 `skeleton/task-index/work/TG-cfac1f498c427933d2/core-gui-20261014/REPORT.md`，当前逐文件入口仍 core/recovery。

## 2026-10-14 country-core 改按原源码目录推进

用户最新要求先核 core 依赖，再按原目录/文件查源码、章节、纠错与原二进制，持续恢复完整 core；游戏实现仍全部放 country-core，目标仍 B。当前唯一进度入口 tiny-glade/crates/country-core/recovery/{README.md,DIRECTORIES.md,inventory.json,contracts.json}。B PDB 文件表 539 个；旧重建 595 路径与 SOURCE_MAP 420 条合并后的 74 个非 B 路径单列，不能混算原文件。原章节实际匹配 B 文件 529 章，SOURCE_MAP 420 是另一库存范围。rustdoc 核实际模块/接口；27 直接依赖中 4 个 workspace 恢复包，23 个公开依赖身份均匹配原锁（不等于 Bevy fork 已证）。

本轮落码：UiSettings 7 字段/四对齐变体/serde(default)（原 JSON 入口 0x14142eba0 的 128 字段存在组合），WindowSize u32、菜单 7 读取谓词、亮度表、EditingHighlight clear、统一 A/B draw Resource 身份、Bevy 双缓冲事件 drain/last、共享 Transform look_at/abs_diff_eq；DirtyTiles cell_from_world_space 原入口 881 组（公开 fmaf 桥，返回 tag+两坐标），真实调用者 system-clutter add_birdnests 0x1422ca1c9。进一步接通 core WallEntities -> EventReader<HighlightWallCmd> -> wall_highlight -> 高亮资源 -> draw；原单系统 96 组匹配（仅公共哈希桥），保留事件游标而非 drain。

纠错：UiSettings 默认字为0x01010000（auto_hide=false/Bottom）；0x140a46dd0 是 ToolbarAlignment 非 UiSettings；EscMenuState 标志在+8，is_open 在0x14093fbc0，0x14093fc00为cast_ui_shadows；0x14093fc20 word-2 掩码；不编造 allow_time_passage。wall_highlight 每次先清 ty，只取最后未读命令，目标缺失不回退，写 RGB 不动 enabled。Window 0/0 ARM NaN 符号不同，现显式保留 B 0xffc00000；首失败留在本轮报告。完整 GUI/Next/Prev/DebugGui、PublicWalls、实体创建删除同步、启动/调度及大部分非渲染业务未闭合，不宣布整目录/整 core 完成。

## 2026-10-14 country-core 墙权威状态首批落码

用户要求非渲染恢复全部放在 country-core 内。新增 resources/walls/{inner_wall_state,inner_walls} 和 systems/wall/wall_state 的 CPU 发布消费者，接 core 现有采样/规则墙高度算法；未改 tg-building-runtime、依赖锁或旧恢复源码。原 RNE 指令纠正旧表：+0xc0 是新墙 ID counter（new=0、参数化建墙先 inc、显式 ID 不推进、reset 只清 counter）；freehand_ty 实在 +0x66，+0x60 是石膏剥离量 f32。显式 ID 冲突先替换旧行再 panic，不发 created notice。80 setter tails + 6 构造 + 8 默认拱高 + 3 reset 原指令生成 fixture，被编译 Rust 精确字段测试消费；lookup/通知属于明确排除的库桥。7 新测试、1349 既有 core 库测试、整体 check/fmt 通过；rustc1.98.1 严格 Clippy 受既有样式告警阻塞，core --no-deps 且仅豁免旧 chunks 建议后通过。完整 PublicWalls/空间/屋顶/历史、地形脏路由、原调度和 GPU 尚未闭合，NR1 不完成。报告 skeleton/task-index/work/TG-9f411268a571c6bce1/core-walls-20261014/REPORT.md；当前 SHA 台账 tiny-glade/provenance/core-wall-authority-2026-10-14.json。

## 2026-10-14 country-core systems/resources 第三轮（章节化源码地图）收齐

用户指正挖掘方式后改为**结构化章节流**：先由 B 源码骨架建完整**源码地图**（`evidence/country-core-deep/SOURCE_MAP.json`：420 文件/4,027 函数 = 1,690 业务 + 1,658 派生胶水(drop/debug/clone/serde visitor) + 667 内联无独立符号 + 12 依赖转发；分类规则：仅硬胶水模式剔除，new/add/get/set 等自定义实现一律保留为业务），再逐文件生成章节骨架并填充。第三轮 8 域全部完成：**chapters/ 下 420 章全部就地填充、占位符清零**；每章 = 文件作用（名称+证据推断）→ 业务函数表（地址/源码行/作用猜测→证实/逆向结论 O-I-U/证据链接）→ 内联函数名称推断 → 派生胶水分组。各域结论分布（业务行）：wall 430（O296/I83/U51，U 中 48 为 serde 胶水）、collision-eco 213、terrain-water 123、history 127、ui-input（42 章）+ 15 手解、session-save 17 章（O100/U5）、camera-time 21 章、audio-misc 59 章（补建 6 零函数章）。本轮新反编译累计 ~60 个、字节手解 ~30 个。剩余真实业务空白仅 10 行（wall：is_default_arch_height 卡点 + 4 闭包实例 + UiWallHeight 等，均已在章内列明）。章节结构成为后续实现阶段与继续深挖的权威入口（`chapters/` + `SOURCE_MAP.json` + 各域 DIGEST/REVIEW/CLOSURE 三层证据）。

## 2026-10-14 country-core systems/resources 第二轮（审查+闭合）收齐

第二轮 8 域全部交付：每域 `<domain>-REVIEW.md`（逐文件实现逻辑审查表，共 277 条：OK 99 / FIX 98 / WRONG 15 / PARTIAL 31 / GAP 20 / STUB 7 / LATER 6，机器可读 `evidence/country-core-deep/round2/FIX-LIST.{json,md}`）+ `<domain>-CLOSURE.md`（第一轮 U 项闭合，多数 CLOSED）。关键新理解：① **S_PUB32 只含 pub 项**，私有函数不在其中不等于"全内联"（session.rs 假象已澄清）；索引升级 **v3** = S_PUB32 + 骨架 CU"编译实例 RVAs"（1,864 过程，runner 已切）。② **.rdata CU 函数边界表**（0x14369a000/0x143682f74/0x143697308）可枚举全部本地符号——解决 Ghidra NotFound 的通用手段。③ 跨域闭合：CameraMode=核心 6+拍照槽 6..12（PhotoModeCameraMode 4 变体 next/next_demo 转移表双域交叉验证；槽 6..9=拍照子模式复用 Normal body I）；0x14362c420=panic 进行中标志（多域确认）。④ 典型实现错误模式已归档（地址丢高位、v1 错名沿用、签名不符、无证据布局）——实现阶段按 FIX-LIST 逐条修。仍 OPEN 的卡点（SubworldIndex 12 槽名、PhotoMode 精确判别映射、EscMenuState 变体名、Events fork 双布局等）登记在各 CLOSURE 卡点表，属 engine 层/跨 crate 依赖边界。

## 2026-10-14 utils B 深挖

用户明确后续只分析 B，不索取 A/C。继续沿真实 2D 楼梯构造→`densely_sample_spline_segment`（0x140809ad0）恢复密集采样，143 组原指令样例含 7 越界；严格系数检查发现旧标量 RHS/c1 运算顺序偏差，已修复并用原 20 组 3D 构造/120 查询逐位回归。Curve owned 转换 XZ/输入12/输出8 stride 已由 0x1402ba9e0 原 map/collect 确认，std/glam 直接复用。纠正旧 as_vec2 说法：四字节入口只证明 RCX→RAX，RDX 不证明长度，原借用类型仍未知。全部 4894 模块/85322 C13 行号段继续扫描，墙三个方法及 Spline2D 全局查询仍无正面原码映射，不宣称死代码。当前 formal utils Debug/Release 各93测试、严格Clippy通过，country-core1142 CPU测试通过；报告 evidence/utils-deep-followup-2026-10-14/REPORT.md，审计 tiny-glade/provenance/utils-deep-audit.json。Windows独立worker保存重开确认USER_DEFINED原型，仅辅助反编译，非作者Rust ABI；未同步原库，专属worker已停止。库仍partial，完整原进程未验收。

## 2026-10-14 tg-text 字体/帧缓存内部收口

用户明确先补内部字体角色/共享表与帧缓存，不处理调用方/GPU，不以适配代替原逻辑。当前 `tiny-glade/crates/tg-text` 已改真实角色/Arc模型：new移入Arc<FontTable>、AttrsOwned与基准/实际metrics分离；tag Label0/MenuButton1/TextEditor2/Hand3。原set_family不写回构造角色，切回早退/后续metrics查原角色的行为经原入口确认后保留；标准unwrap、移走lines再new_empty和take_editor状态恢复。FontMetrics只是原二f32状态的恢复命名，作者typename缺失；仅证实From<f32>=(v,v)，不添加tuple转换。

frame/stamp和片段生成/验证帧均u32，恢复无符号<、wrapping+1、同帧不增加epoch、frame0/MAX边界；usage Vec按原etagere rawID slot/gen，顺序验证保留前缀副作用、越界panic、cache hit不更新usage、expiry零哨兵、本次removed集合retain。移除u64内部epoch/自增allocation counter和render隐式prepare/强制左对齐/生成后清redraw。原同rawID复用会接受，不保留候选更强拒绝。consume在CPU上传处理后推进帧，payload返回供以后GPU接线；gpu.rs/真实游戏调用方未改。本库测试/示例只更新API。

原代码受控829项（字体17、cache791、追加21）通过；Windows rustc1.97.1当前源码Debug/Release各33测试通过、0失败/忽略、严格all-targets Clippy/fmt与gpu特性仅编译通过，113默认CPU外部依赖身份匹配原B，锁未改。范围/源码SHA见 `evidence/text-internal-2026-10-14/{REPORT.md,verification.json,delivery.json}` 与 `tiny-glade/provenance/text-internal-audit.json`。本轮指定内部规则已闭合；历史条目里的Arc角色/u32规则“待恢复”由此取代。原完整进程、冷生成全输入域、真实GPU/调用方与A/C不升级验收，模块整体保持部分恢复，不按case数算完成比例。未覆盖历史源码/发行文件，无新增MCP后端。

## 2026-10-14 country-core systems/resources 并行深挖收齐（8/8 域）

8 个领域 agent 全部交付 DIGEST（`evidence/country-core-deep/<domain>/`）：camera-time/session-save/audio-misc/wall/history/ui-input/collision-eco/terrain-water，共 ~1,270 份反编译落盘。**关键基础设施结论**：① 旧 sysres-procedures.json（PDB 编译单元"首条位置"派生）存在**系统性名/址错位**（四域各自证实 10+ 处），已用 `evidence/render-architecture/public-symbols.json`（S_PUB32，101,405 条）重建权威索引 **sysres-procedures-v2.json**（1,599 个 country_core systems/resources 过程，rustc-demangle 名，0 未映射）；此后一律以 v2 + 机器码为准，PDB 骨架名只作线索。② 跨域全局族三域合证：0x14362c420=panic-in-progress 门控、0x14362c3a8=全局日志级别、0x14362c011=楼梯校验开关。③ 每域 10-15 条合同/代码纠错已入 DIGEST（例：快照 schema=JSON+flate2 非 rkyv/ron；WallState=0x70B；CameraMode=13 槽；SubworldColliderHandle=24B 枚举 Terrain 槽=5；AllWorldRasterWrite::write 置 +0x299 全脏悬案闭合）。实现落地为下一批次，按各域 DIGEST 的 O 条目写码、I/U 保持诚实标注；INDEX.md 为总入口（含跨域闭合与冲突登记）。

## 2026-10-14 当前 utils 缺口原码恢复

用户指定直接继续 `tiny-glade/crates/utils/README.md` 的缺口，未回写历史 shared-foundation。全原 B PDB 4894 模块 fresh 扫描定位 IndexCondenser::insert IPI0x74ee 的两个真实宿主；重复 key 失败且不改状态，新 key 取旧 Vec 长度后 append/map insert。52 插入+78 完整 remove 原指令快照通过；共享 insert/get/remove_index 已接 country-core 的 OwnedGpuWorldSubset ID 重试、hint 修复和载荷交换，移除重复业务实现。FloatGridCells/RasterBufferRef 消费改为共享 utils；原 512x512 byte-grid 两分支100原指令样例逐位通过。纠正采样开关：public USE_FLOAT_GRID_SAMPLING_FIX 的 pointer slot=0x14362c308，actual byte=0x143638fac，写入者仍未定位（原 .text 有6处读取，Ghidra缺xref不能当无caller）。Windows x86_64 utils Debug/Release 各90通过、country-core 1142通过、两库 all-targets严格Clippy通过。Release旧圆形literal样例4ULP差通过black_box输入避免编译期求值后闭合，原输入/预期/逐位断言不变，生产算法未改；失败保留。墙3方法、2D全局查询无PROC/inline host仍未恢复，Vec3SliceView2投影轴/stride未知；原泛型ABI/巨大索引、原全进程/GPU、A/C及全平台浮点不计完成。证据 `evidence/utils-gap-recovery-2026-10-14/REPORT.md`，机器台账 `tracking/utils-gap-recovery.json`，当前源码审计 `tiny-glade/provenance/utils-gap-audit.json`。

## 2026-10-14 主题接口一次真实请求

用户后续明确授权请求一次。实际Rust reqwest/ReqwestTransport对原events URL做1次GET（禁重定向，无重试/循环/下载blob），HTTP200、123字节、events1项、max-age3600。原响应已保存，theme-server-proto及fetcher离线同响应复验通过。首次汇总的u128不被原serde_json接受，已改u64且未重发请求。证据 `evidence/theme-live-once-2026-10-14/REPORT.md`/`result.json`；只验证events可达和解析，不升级完整原主题流程。此前静态轮“无真实请求”是当轮历史事实，不适用于此授权后续。


## 2026-10-14 当前开发工程 integrity / text 续轮

用户指定 `tiny-glade/crates/{tg-package-integrity,tg-text}`，直接修当前工程，不回写历史 shared-foundation 导入快照。integrity 原 verify loader 新增 11 个 Windows 精确路径/Unicode/surrogate 边界全部匹配，冻结回归；修复 inspect_package 在 verify 前 read/parse expect 的提前 panic，5 个阶段输出验证通过，库内生产算法未改。tg-text 恢复 render 的借用 atlas descriptor + 最近 consume 图元数（0x140b8f670/0x140fac810），回调失败前 count 已保存而流/frame 不变；metrics 按 0x140b90780 搬移 lines 至 Buffer::new_empty 并 Editor::new（保留文本/attrs/actions，重置编辑状态）；scale setter 保留 pending growth；按原 upload fold 0x14180585e 修复 RGBA/SubpixelMask 错误 RGB3 转换。文字 19 个 getter/align/width 原指令案例通过，其他改动静态原字节/fresh MCP 合同；独立 Windows worker 保存/重开验证后停止。

当前 workspace 联合门禁：31 测试通过（integrity 8 + text 23）、0 失败/忽略，all-targets 严格 Clippy、fmt、tg-text gpu 特性 all-targets check 通过；CPU 120 外部依赖身份匹配原 B 锁。验收/当前源码 SHA：`evidence/small-libs-followup-2026-10-14/{REPORT.md,verification.json}` 与 `tiny-glade/provenance/foundation-followup-2026-10-14.json`。历史报告的“仅 ShapeStream CPU”不是当前实现范围。新确认的未知：原 set_family 为角色字节+Arc<FontTable>，候选 String/set_font_role 仍适配；render descriptor 非原 Texture；原 u32 atlas stamp/frame 与候选 u64 的同帧/rollover 差异未闭合。真实启动/toolbar/GPU consumer、A/C、全部 OS/并发异常仍未验收，candidate_partial，不宣称整库/整游戏等价。已明确排除的四个报告/云平台包不列为后续恢复缺口。

## 2026-10-14 主题三库静态补全

直接完善实际 `tiny-glade/crates/theme-defs`、`theme-fetcher`、`theme-server-proto`，不修改历史 shared-foundation 镜像。B 已定位库内业务/合同静态覆盖收口：HTTP Date/ETag/body 顺序和副作用、Response释放、分阶段时钟、CAS mtime错误传播、ZIP清单缓冲写入、21处原日志已恢复；defs/proto完整已发射合同及serde分支已复核。当前交付锁三库可达138公共包identity全匹配原B，26测试（含3跨库离线链）与all-targets严格Clippy/fmt通过，锁SHA门禁前后不变。无真实主题服务请求、无原游戏执行。原plugin消费langs/meta/history/credits属于库外；精确未发射命名/Display、完整原HTTP/CAS/ZIP入口oracle与A/C未验收，不升级整模块原行为等价。新Windows独立consumer库保存/重开/停止，未同步原库/Mac overlay。权威报告 `evidence/theme-static-2026-10-14/REPORT.md`；源码SHA与进度 `tracking/theme-static-recovery.json`。

## 2026-10-14 用户排除四个报告与云平台包

用户明确要求删除 `tiny-glade/crates/tg-integrity-report`、`tiny-glade/crates/crash-reporter-gui`、`tiny-glade/crates/feedback-reporter` 和 `tiny-glade/crates/geforcenow`，四者不需要逆向还原。从当前开发 workspace、当前源码清单和后续恢复范围排除；旧恢复目录与原锁文件、历史证据仅作历史保留，不重新导入，也不列为待恢复缺口。根 README 说明删除原因和功能影响。


## 2026-10-14 数值测试合同后续修复

零切线直接构造固定负 quiet NaN；圆形/Elastic 的原样例含 Windows UCRT 舍入，跨平台测试分别采用有界误差，Windows 分支保留逐位。utils debug/release 均 93 项通过、无忽略；这不是 Windows 原版逐位验收，也不升级完整业务恢复。详见 `evidence/foundation-review-fixes-2026-10-14/numeric-followup/REPORT.md`。此前 87/3 为历史断点。


## 2026-10-14 三库产品修复与完成度纠偏

已修复数组纹理所有叶文件的内容缓存失效、旧 metadata 迁移、零 mip/零尺寸/对齐溢出错误，以及独立法线选轴和 directed local-edge 距离中错误的 abs。utils 当前 87 通过/3 失败（圆形 4 ULP、越界切线 NaN 符号、缓动 1 ULP）；asset 15、asset-pipe 15 通过，三库严格 Clippy 通过。仍为候选/限定验证，不能说所有 B 业务实现基本完成或三库全部测试绿色。libm 根因待同平台/中间值证实；三个墙符号改为 compiled_body_not_located，未证明未编译。新增缓存和参数处理属于产品策略变化，旧原版等价报告不覆盖。详见 `evidence/foundation-review-fixes-2026-10-14/REPORT.md`。以下历史记录中的更强完成度/未编译/全绿表述由本断点纠正。


## 2026-10-13 utils/asset/asset-pipe 三库收口

用户选定三库完成实现+测试，明确"不造轮子用依赖库函数"，并对无证据符号要求主动逆向。本轮：`FOUNDATION_BUSINESS_COMPARISON.md` 确认为补全前快照（CHECKPOINT 自述），utils 的 9 无实现/22 部分缺失绝大多数已实现；真正缺代码仅 3 处符号。逆向结论：`WallSpaceCoord::{bound,is_bounded,to_affine3}` 在 B 中**未编译**（仅 PDB inlinee 声明 to_affine3 IPI 44791/is_bounded 44797/bound 44798，无独立过程/内联点/消费方；MCP find_functions、6 个墙系统消费者反编译、inline-bindings module 761 均无记录；另一会话 wall-affine CONTRACT.md 交叉印证），保持不猜测实现。已实现：`Spline2D::{find_segment_and_dt,position_at_t,tangent_at_t}`（镜像证据化 Spline3D 段选择，2D 局部求值复用原入口 0x140c92610/0x140c92660）、`Vec3SliceView2` 投影访问（x/z 轴候选，原轴/stride 未恢复）、`asset-pipe::texture::block_bytes`（DDS/BC 常量）、asset render 深层默认测试。shipped 三测试加本机发行目录回退（Windows 路径优先）。utils 82 通过/5 既有差分失败（含 CHECKPOINT 记载矩形两差异，非本轮回归）；asset/asset-pipe 全绿；三库 Clippy -D warnings 全绿。台账更新：foundation-business-functions.json（12 条状态）、wall-heightmap report.json、PROJECT_MEMORY。证据：`evidence/foundation-completion-2026-10-13/utils-gaps-2026-10-13/`。
- 差分修复续轮：`rectangle_deformation` 与 `rectangle_distance_normals` 均过（`get_closest_edge_id` 0x140c9acb0=逐边线段距离、sqrt、首个最小胜出；`expand_along_edge` 0x140c9c620=clamp 下界 **0** 非 min 参数，汇编 DIVSS/MINSS/MAXSS 确认；`closest_edge_normal`=|local|/size 选轴+sign，并发会话按此实现）；utils 现 **87 通过/3 失败**。剩余 3：`circle_boundary`（0x140c976f0 乘法归一化+MSVC atan2f vs macOS libm 平台差；除法仅巧合匹配单例已还原）、`curve2_rules`（+NaN/-NaN 符号）、`easing`（1 ULP）。


## 2026-10-13 paths/lang-lib/tabloid 原生复核与补全

共享 Ghidra B 后端逐函数反编译复核三个基础库（证据 evidence/foundation-plt-2026-10-13/，29 函数反编译 + raw asm 交叉）。结论：三个库全部 B 发射业务函数均已实现，无库内未实现功能——paths 10 函数、lang-lib 6 根+2 方法、tabloid publish/subscribe/take 泛型全部单态化仅 DirtyTiles 与 ColliderDiff 两类。**TG_DISABLE_LANG_FALLBACK 判别式疑点消除**：跨 load_locale_impl/find_standard_app_root_path 交叉验证 0x8000000000000001=Ok，现有 is_err 语义正确（不改行为）。**tabloid 通道归属修正**：原合同把 0x141453350/0x141453c00/0x141454290/0x141454410/0x141454590 全标 ColliderDiff 族有误——fresh 反编译证实仅 0x141453c00（publish<ColliderDiff,ColliderDelta>）与 0x141454410/0x1412926e0（subscribe<ColliderDiff>）是 ColliderDiff，其余为 DirtyTiles。**load_locale 写失败偏差**：原版 0x140b9c300 静默丢弃 last_used_locale 写错误，候选保留 log 为登记偏差（用户确认保留），代码未改。交付改动：tabloid 新增 DirtyTiles 语义契约测试（tests/dirty_tiles.rs，4 测试，zero-empty+OR-apply 镜像 0x140960ac0/0x140960ef0，闭合 tabloid:06 库内契约）；paths 新增初始化限制边界测试；lang-lib 仅注释登记。验证：32 项测试（含新增 4+1）与严格 all-targets Clippy 通过，cargo fmt 干净（顺带规范化既有 verify_locale_policy.rs 示例，纯样式）；verify_locale_policy 4 个 macOS 兼容模式通过（non-unicode 模式 Windows-only）。台账已更新：FOUNDATION_BUSINESS_COMPARISON.md 三行 + tracking/shared-foundation-business-comparison.json（source_modified/tests_rerun=true，SHA 现算）。真实 producer→consumer（autoclutter/stairs/structural_graph）与 DirtyTiles/ColliderDiff State/StateDelta 实现在 country-core 库外，仍为 tabloid:06 库外接线缺口。

## 2026-10-13 B未开始基础库并行实现

用户明确要求C暂不处理，集中B库和依赖。本轮3 agent已把 tg-iced、tg-integrity-report、crash-reporter-gui、tg-gpu-selector、feedback-reporter、geforcenow、theme-fetcher、theme-server-proto 接进共享workspace并按原源目录恢复。协议包只记录字段契约，serde/Clone/Drop等派生不算手写算法；其余记录手写状态/HTTP/CAS/ZIP/SDK调用组合策略。真实iced、winit、cosmic-text使用原fork commit，公共依赖闭包414项version/source/checksum均匹配原B锁；已有tg-text可选GPU路径的gpu-allocator0.27候选不在本8库闭包内，不能据此宣称全workspace分叉等价。共享locked/offline测试32项、严格all-targets Clippy通过，本地GFN fixture额外1项/14组；UI原叶131案例与GFN/反馈66原指令案例各保持限定范围。真实窗口与视觉验证以[本轮交付](evidence/foundation-unstarted-2026-10-13/REPORT.md)的最终报告为准。原全GUI像素、完整主题/反馈oracle及真实游戏producer/consumer仍未验收，8库为candidate_partial，不升级完整模块。专属worker仅保存独立B副本；C后置库存不算实现。

## 2026-10-13 shader 原配置与最终打包复验

38 个非零 compute 原宏 tuple 已从 native producer/caller 恢复并核原 key；实际 DXC 生成 404 个新包／638 阶段，失败 0，完整 typed 反射 397/404 相等。原 19 个 uniform 冲突包的 38 阶段经独立 SpirQ 复验全等。21 相关测试、严格 Clippy、新包 compute GPU 读回通过。原 artifact 宏与 TGV 还原源码选择宏分离。剩余 7 个 typed 反射差异、132 个 zero-key 原 tuple、325 阶段公共完整接口差异、源码依赖组织及原 DXC／离线 CLI 策略仍待恢复；不声明全 shader 语义或整帧验收。当前证据：[shader gap summary](evidence/shader-gap-investigation-2026-10-13/SUMMARY.md)。此前 shader-build 数字是历史验收，未覆盖本轮修复。

2026-10-13 用户要求基础库按游戏侧手写功能整理，自动Clone/Drop/派生visitor和库泛型胶水不作为功能任务。已建立[基础库业务功能清单](FOUNDATION_BUSINESS_FUNCTIONS.md)与[机器清单](tracking/foundation-business-functions.json)，覆盖58库的286条功能或定位记录；来源待分类、仅数据契约、仅包身份和依赖复用单列，不能当作已证明自研算法。手写chunk封口/GPU完成回收等归入对应业务闭包。原逐接口库存和验收状态保留，不从功能条目数提高完成度。

2026-10-13 shader 构建续轮三 agent 共研互审已交付：现有 rhapsody-rebuild 接通原 shader-prepper include 展开/pragma once、宏与 #line 配置→实际 DXC 原参数→精确 SpirQ fork 独立反射→rspirv 后处理→Speedy 原 schema 打包。404 个新包/638 阶段全部生成（393 包官方 DXC 1.9.2609、11 包官方 preview 1.10.2605.37），1446 份构建输入/产物由两个 agent 独立核对 SHA/长度一致；原 DXC 版本或 fork 未确认。最终 10 项受影响测试、严格 all-targets Clippy、新 fill 在 Intel UHD 630 上 64 word 读回及新 egui 两变体布局通过。原 SPIR-V 的 404 包完整反射再生成零差异；新编译包仅 210/404 完整反射一致、302/638 阶段公共接口一致、213/404 依赖集合一致，全部构建成功不等于完整原 shader 算法等价。170 compute 原宏 tuple 均未独立证实，其中 38 非零 define key 明确未知、采用标注的空宏诊断基线；234 raster tuple 有原 depth-only=0/1 证据。shader-prepper 是作者原生公开 MIT 库，SpirQ 是明确公开 fork，其余 local 工具包上游仍未知；公共实现直接复用，12 项 shader 依赖身份与原锁匹配。详见 [当前构建与来源交付](evidence/shader-build-2026-10-13/SUMMARY.md)。完整原宏 manifest、精确 provider/临时文件/错误/cache 策略、原资产库/pass 联接、raster draw、整帧与 A/C 仍未验收。

前轮 [schema/请求交付](evidence/shader-toolchain-2026-10-13/SUMMARY.md) 保留历史范围与哈希，当前编译入口及 Cargo 输入已变化，不覆盖旧报告成功标记。原 pipeline 五文件单列模块主键，compiler 原源路径未在 B PDB 定位，不补造路径。分析副本未同步原数据库，原历史报告保留。

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

2026-10-13 三路并行只读分析完成原作渲染 pass 清单，合成见 `evidence/render-passes/INDEX.md` 与 `INDEX.json`。数据源仅 B：B patched EXE（cf048e27…）反编译 + 同发行 `compiled-shaders/` 404 个 .bin → 260 唯一 pass（117R+143C，35 族）；`render_world`（0x1404ec110）确认 54 个 pass 调用 + 帧外层 20 步 + 8 条件分支（第 2 轮 \*0x14362c0d8、折射 \*0x14362c098、is_winter 选水/冰、DoF param_6[0xa9]、TAA↔TGSR param_6[0xab]）；country-core pass 声明 API（ComputePass 400B/DrawBuilder 0xf8B/ShaderUri::parse，255 URI 254 匹配 .bin）。关键观察：全程无硬件光追（GI/水"deferred RT"共用软件 BVH SSBO rt_bl_bvh/rt_triangles）、HZB/mips 用 FidelityFX SPD、nani GPU-driven 实例化、G-buffer、descriptor 约定（frame_constants set1/b0、sampler set0/0..12、$Globals set0/13）。这是分析证据非验收；逐 pass 变体绑定/descriptor 句柄/GPU 时序未恢复；A/C 版本未做此分析。

2026-10-13 完成 country-core 渲染域第一批并行重写（业务逻辑→整理→骨架实现，差分仅作校验），5 个工作包全部交付：wp1 pass 声明层（pass_builder 全族+ShaderUri）、wp2 GPU buffer 原语（TypedStorageBuffer/ShadowedBuffer/SparseInstanceBuffer 等）、wp3 nani GPU-driven（GpuWorld/MeshAtlas/实例数据）、wp4 mesh/shader/资产库接线、wp5 render_loop 帧 passes（W1–W54 的 CPU 组合）。每包交付业务逻辑合同（Observed/Inferred/Unknown + typed gap）+ 骨架对齐 Rust，落盘 `reconstruction/country-core-render/wp{1..5}/`。整合组装为独立 crate `reconstruction/country-core/`（111 个 .rs，edition 2024，rust 1.86），offline `cargo check`（0 错误/67 骨架警告）与 all-targets clippy（0 错误/133 警告）亲自复验通过。rhapsody 边界统一为 `rhapsody_contract`（wp4 扩展，含 RenderPassBinding/PassBuilder/Handle 等游戏侧表面）；`rhapsody-rebuild` path 依赖已声明并锁入，但其已恢复表面未覆盖游戏侧 API，未臆造桥接。已知缺口：`render/nani/systems.rs`（20 个 `nani_update_*` + `nani_maintain_instances` ECS 系统）因依赖整个游戏 components/resources 图而暂缓（文件保留未声明）；wp1 `pass_resource_access`/`draw_stream` 的 rhapsody managed-handle/temporal 表面为 todo 边界；整合报告见 `reconstruction/country-core-render/_integration/INTEGRATION_REPORT.md`。全部标记 implemented_unverified，无 runtime/GPU 验收；合同是当前可审阅依据，不因编译通过声称原作等价。

2026-10-13 完成原作 HLSL 源码恢复第一波（并行 agent，行为等价重建非作者原稿）。骨架：404 .bin 嵌入调试信息重建 `reconstruction/shader-source/shaders/`（413 源路径、260 main、68 inc）。共享 `inc/` 层由 3 个并行 agent（INC-A/B/C）重建并通过编译自检；`frame_constants.hlsl` 偏移 0..240 与原版 `_apply_screen_shadows` 结构逐字节一致。Wave 1b 六族重建：260 main 绑定+入口+LocalSize 硬门 151 个 wave-1 main 中 108 硬过；全指纹收敛仅少数（dxc 1.10.2605.37 vs 原版 generator 16ceb560 的 RuntimeDescriptorArray/StorageImageExtendedFormats 能力与 OpLine/OpString/直方图残差均为工具版本差异，非行为差异）。整合 pass（P1c）按判定记录应用 13 项 inc 修复并逐个验证：RDA 触发声明（307 个 frame_constants bin 能力集对齐）、include 漂移修剪 13 条（wave-1 多余 include 文件 121→0）、samplers 补全 s0..s12（原版确实声明 13 个 sampler，dxc 会 DCE 未用者）、current_view_index→$Globals b13、color_library cbuffer 改名、history_valid→is_history_valid（与 $Globals 成员撞名）、StairsCounter→{int count}、terrain_height 定义等；全树 260 main 编译 125 PASS/135 FAIL（135 全为 wave-2 骨架占位无入口，0 真实 dxc 错误）。F-R 15 bin 补验 12 硬过 + 3 f0ad 平凡变体。证据 `evidence/render-passes/{INDEX, RECOVERY_TRACKER, integration, recovery, shaders, skeleton, validation, cpu-frame, pass-builder}`。大算法族（nani cull/GI/水冰/DoF/AA/grass/nani raster/final_post）进入 wave-2 双人重建+review；dxc 版本差异、f0ad 平凡变体与行为等价残差不计为失败。

2026-10-13 第二批完成 country-core 渲染域最后的 nani ECS 系统恢复（WP6）：`render/nani/systems.rs`（21 个系统，含新发现非 Query 的 `nani_flush_instance_updates` 0x142080640 GPU-flush 系统）与它消费的 components/resources 作用域（`components::drawable`、`utils::components::transform::Transform`（游戏自有 40B {t@0x0,r@0xC,s@0x1C}，数学复用 glam）、`resources::{color_id,window_lights,critters,world_raster(_defs),walls/*,roofs/*,stairs/visual}`）全部恢复并接线进 `reconstruction/country-core/`。复验：crate 现 133 个 .rs，offline cargo check 0 错误（140 骨架警告）、all-targets clippy 0 错误。渲染域（38 渲染文件 + nani 系统 + 消费类型）编译闭环。剩余缺口如实登记：wp1 `pass_resource_access`/`draw_stream` 的 rhapsody managed-handle/temporal todo 边界（属 rhapsody 恢复工作流）、多数 component→T 字段映射为 typed gap（3 条 O 路径除外）、flush 30 槽 vs 27 kind 布局。全部 implemented_unverified，无 runtime/GPU 验收。WP6 交付见 `reconstruction/country-core-render/wp6/`，整合补丁见其 evidence.json。

2026-10-13 第三批完成内容域恢复（WP7 水/WP8 地形/WP9 屋顶/WP10 植被生物）：`resources::{water+water_mask, terrain_height, rocky_terrain, world_raster_defs, roofs/*, tree_storage, critters}` 与对应内容系统全部按业务逻辑→骨架恢复并接线进 `reconstruction/country-core/`（crate 现 157 个 .rs）。关键恢复：WaterRaster（WorldRaster<512> u8 网格）、GladeType@+0x58==4 判冬、TerrainHeightsData<T> 双实例（高度域 −2.5..24/512²）、RockyTerrainMesh::load JSON→SSBO、Roof 88B 组件布局（0x1408e31c0）+ roof_tiles_gravity 完整 Query→PushConstants→dispatch_with 链（0x140a29b10）、TreeStorage hashbrown+next_index、critters GPU 上限（Firefly 96/Butterfly 24）。既有 `reconstruction/{terrain,roof,wall,tree-tool,critters}` 与 task-index 候选逐函数复用验证（VA/公式一致，未重推导）。复验：offline cargo check 0 错误/134 骨架警告、all-targets clippy 0 错误。全部 implemented_unverified，无 runtime/GPU 验收；合同是审阅依据。WP7–WP10 交付见 `reconstruction/country-core-render/wp{7..10}/`。

2026-10-13 完成原作 HLSL 源码恢复第二波（大算法族，行为等价重建）。wave2a 11 个重建 agent（4 双人组 + 3 单人）全部交付：nani cull/wall **44/44**（含 -D 变体）、gi-deferred **22/22**、water-ice **22/22**、dof-aa **22/26**（4 为 dxc 版本非失败）、nani raster **26/30**、grass/clearing **19/19**、final_post/painterly/vermeer **16/16** 硬门 PASS。4 组双人 review 裁决并合并：water-ice 采用 A、gi-deferred 采用 B（合并 A 的 trace/grid/bin_lights）、dof-aa 逐文件混合、nani-cull 采用 B；合并 agent 曾以 bin 证据纠正 reviewer 两处 deferred-PS 论断（gbuffer 法线是 RGB8 normalize 而非八面体、alpha 0.0 而非 2.0）。软件 BVH 解码钉死：PackedBlBvhNode 80B、bitmask+compact-index、uint2 地址无 Int64（10 个 PBA bin 用 `[[vk::ext_capability(5347)]]` 近似，本机 dxc 不支持 buffer_reference）。wave-2c 补 4 孤儿（mei_shadow、instanced_roof_beams/spire、water_object_push.inc）、修 Affine3Packed 幂等守卫，**404/404 bin 均有匹配重建主文件，全树 260 main 编译 258 PASS（2 个 Affine3Packed 冲突已修复验硬过）、硬门 235/260**。残差全部为 dxc 版本码差（原版 generator 16ceb560 vs 1.10.2605.37）/f0ad 平凡 PS 变体/PBA 物理读不可表达/大 PS 结构级。证据 `evidence/render-passes/{review, integration/wave2-*, completion, RECOVERY_TRACKER}`。函数名因 dxc 内联多为推断；行为等价重建非作者原稿。

2026-10-13 完成 country-core 第四/五批恢复与终局整合：WP11–16（墙状态资源/墙系统/会话·历史·存档/音频/UI·输入·启动/杂项）与 WP17–21（历史回放/draw_shaders/风格化·菜单/相机·UI·时间·天气/资源余量）全部按业务逻辑→整理→骨架实现方法交付（交付于 `reconstruction/country-core-render/wp{11..21}/`）并整合进 `reconstruction/country-core/`。终局 crate 570 个 .rs，offline `cargo check` 0 错误（184 骨架警告）、all-targets clippy 0 错误（383 警告），亲自复验。覆盖审计：skeleton 539 源路径仅剩 3 个名义未覆盖（camera_rig/collision/painterly 的 mod.rs vs .rs 路径名差异，实际已覆盖）→ country-core 实质全量覆盖。关键恢复：GladeType 四季节（机器码验证）、History 权威布局 + ReplayCmd 0x30B 回放枚举、Roof 88B 组件、TerrainHeightsData 双实例、WaterRaster 512²、软件 BVH RT 世界、FMOD 胶水（211 事件串）、vermeer/painterly 笔触链、GradingLut::load_cube 完整 .cube 解析、MainCamera 0x4e0B + camera_rig 全家。全部 implemented_unverified——合同可审阅、编译通过，但无 runtime/GPU/画面验收，不声称原作等价；typed gap（字段布局/资源名/判别值）如实登记在各 wp 合同。整合报告 `reconstruction/country-core-render/_integration/{INTEGRATION_REPORT,INTEGRATION_BATCH4,INTEGRATION_FINAL}.md`。注：终局整合 agent 及 batch5 部分 agent 曾因账号限流/额度中断后重启完成，成果均已落盘复验。

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

2026-10-13 按用户引用FOUNDATION_BUSINESS_FUNCTIONS.md只读对照当前七包paths/lang-lib/theme-defs/tabloid/utils/asset/asset-pipe，98行齐全：60已有库内限定实现、22部分逻辑缺失、9未实现、1真实游戏状态接入缺口、2数据契约、4 C-only排除B主线。权威功能报告FOUNDATION_BUSINESS_COMPARISON.md、tracking/shared-foundation-business-comparison.json；57相关源码SHA现场匹配，未改运行代码或重跑tests。9未实现均在utils：沿墙偏移/区域/affine，曲线侧向、间距/点数重采样、平滑/移动，Transform矩阵组装，圆转曲线，矩形边缘距离，屏幕坐标转换。新确认高度图只返回crossing事件，缺原首点/连续上方区段更新与SplitReceiver协议；RT/audio/trigger深层默认和最终化为一项类型任务。CurveSlice view/extend/remap、prefab递归读RON/依赖登记、wrap/反向迭代/周期样条求解/连续裁剪/nearest-u外推已有代码，不能因旧remaining或无同名helper称未实现；194原接口无已审查映射不等于194业务功能没写。B九项纹理producer策略都有代码，原图片资源不足仅限制全面差分；真实country-core消费/热更新单列库外闭包。theme-defs活动选择属于theme-fetcher，不新造本包算法。后续优先按此功能闭包表推进，原PDB接口表辅助定位，宏/getter不单列功能。

2026-10-13 用户指出宏生成Default/fmt/Eq误计为未恢复函数。已逐项按源声明行、派生族与原callee分类，原609库存不删：468自有业务接口、62自动实现/依赖转发、46类型/schema helper、33待分类。业务主表280部分分析/188未触及/274候选映射，194无映射实现（143未业务分析、44有伪码未解释、7已有分析未实现）。旧314无映射/311未触及是混合口径历史数字，不能继续用于业务进度。46 schema helper按22声明组记录，另保留2组派生serde类型义务；不逐个逆向生成visitor。Transform identity Default、Timer初始状态、CurveU新建/clamp Deserialize等保留自定义业务。源码、依赖、历史原指令及测试日志均未变化，无需重复跑游戏测试。权威分类INTERFACE_CLASSIFICATION.md、tracking/shared-foundation-interface-classification.json；业务主表INTERFACE_PROGRESS.md，缺口UNRECOVERED_INTERFACES.md。

2026-10-13 共享基础七包（paths/lang-lib/tabloid/theme-defs/utils/asset/asset-pipe）完成三名独立agent加父级并行code review。修复小尺度三角相交漏判、JSON selector重复key分配、语言缺失键缓存命中分配、线性RGBA8压缩两次整图复制；清理Spline手写向量长度与Curve手写std二分。三角原入口304新+257旧有序端点对照通过；统一七包85测试、严格Clippy通过，104可达外部依赖身份匹配原锁。LoopRange长度误报已撤回。原RDP负epsilon不返回、原缓存/并行/生命周期风险及原消费者缺口保留，未宣称完整功能验收。当前原接口台账609观察/298部分分析/311未触及/295映射候选；314无映射实现分为199未业务分析、44有伪码未解释、64归属待定、7已有分析未实现。全清单UNRECOVERED_INTERFACES.md与tracking/unrecovered-shared-foundation-interfaces.json，review总入口evidence/shared-foundation-review-2026-10-13/REVIEW.md；tracking/shared-foundation-code-review.json附当前源码hash。历史差分SHA不覆盖。后续按原函数接口及业务规则推进，源码/ABI一致不是目标，公共依赖直接复用。其他聊天的motomoto/tg-text/tg-package-integrity和rhapsody范围未修改。


2026-10-13 小型基础库第一批由三个独立agent并行完成规则提取→原依赖实现→差异审阅；交付在 reconstruction/shared-foundation/crates/{motomoto,tg-package-integrity,tg-text}，整合报告 evidence/small-libs-batch-01/REPORT.md。motomoto恢复tag/Writer/Drop回填/Reader业务链，6测试与173原Reader受控指令案例零差异；Writer/Drop仅静态+候选，完整save消费者/schema/压缩未验。完整性库恢复原raw BLAKE3→数组Hash长度前缀→seeded FNV low16==0 gate、version/files、self精确排除、文件失败分类和根tmp额外文件策略；4测试与原264376byte清单样本通过，候选只读当前目录1792 entries报告2 corrupt/288 extra，无原入口oracle/报告consumer。其64文件easy-parallel分批与原全量job的并发时序差异公开保留。tg-text仅FontTable+ShapeStream/RawShapeStream CPU子链，6测试、51原append+21原scaled逐word/bits零差异；Raw::encode与Image::with_scaling实际inline写word规则合入append，lib.rs仅glue，TextFragment/TextRenderer/glyph/atlas/GPU未恢复。16测试、严格all-targets Clippy、限定fmt通过；三根22公共依赖version/source/checksum与原锁匹配，未手写依赖算法。原manifest/font-table fixture逐字节一致。三个独占Windows worker正常保存关闭，overlay未同步原DB；工具失败报告保留，不把局部CPU闭环升格整库/游戏/A/C验收。全局模块状态candidate_partial，并保留其他聊天的源代码owner范围。


2026-10-13 三个新reviewer并行独立审查motomoto/tg-package-integrity/tg-text，报告 evidence/small-libs-review-2026-10-13/REPORT.md。已恢复范围未发现确认业务算法bug/公共依赖重造；功能仍不完善：真实save/report/toolbar consumer缺接，完整TextFragment/TextRenderer/glyph/atlas/GPU未做。motomoto新增20限定原Cursor Writer/Drop+pending error、3原Reader异常恢复组匹配；integrity新增原x64 FNV/数组Hash叶4096与128依赖BLAKE3组合gate匹配，NTFS surrogate/junction/跨批候选探针通过，但原整verify/极端schema未执行。64批仍O(N)线程创建（1792项1764线程），fs::read整文件live缓冲，合成dev/cached probe仅量化成本、不证明原性能回退。tg-text确认原PDB module1186 record2768→2792→2816为String::deserialize/RON deserialize_string；旧deserialize_str来自同址标签误读，当前代码本来正确，仅修正文档注释与README，历史batch docs/hash保留，本review ERRATA取代误述。source21输入仅两处说明变化，无可执行body改动；22公共依赖identity仍匹配原锁。review-integrity初始化失败无ready，父用CIM完整命令行/创建时间核实两个专属进程并清理，剩余0，其他worker未停。函数列表仅业务族参考，私有折叠/trait/单态化不算独立功能分母；模块继续candidate_partial。


## 2026-10-18 asset_libraries 工程化修复

用户明确允许偏离逆向设计，以功能性、正确性优先。`games_test/tiny-glade/crates/country-core/src/asset_libraries` 完成六项审查问题修复：NoUninit 安全影子转换、真实纹理上传和 BindlessHeap 描述符租约、完整纹理描述与 D16 清零、ECS 热重载/VAO 刷新消费者、真实宿主每帧瞬态资源回收。中文业务注释与命名整理，复用既有 bytemuck/serde/rhapsody，不新增依赖版本；不计原作恢复信用，旧差分不覆盖当前源码。

验证：成功构建快照 2103 项库测试 + 6 项资产链测试通过；真实 Vulkan GPU 1 项通过，已编译产物再次直接运行通过（sampled_capacity=1020，重载回收后 live_leases=1）。最终 17 文件格式检查通过；最新全目标编译、GPU 重新编译和 Clippy 被其他聊天中输入/工具栏接口变更阻断，不宣称最终全工作区通过。报告 `games_test/tiny-glade/reports/asset-libraries-refactor-2026-10-18/README.md`、`verification.json`，工程来源登记 `provenance/asset-libraries-refactor-2026-10-18.json`。没有全游戏/原进程验收或量化性能基准，摆放数据仍为 x/y schema。


## 2026-10-18 asset_libraries 第二轮复审修复

用户要求重新 review 并修复，继续以工程正确性优先，不计原作恢复信用。再次审查当前 15 个模块文件及生产调用链，修复 5 项：typed SSBO 初始上传缺少 NoUninit（改用 bytemuck，numeric records derive 校验）；普通 mesh 删除遗留 GPU VAO（独立删除通知，缺宿主保留、维护阶段释放）；延迟描述符连续热重载耗尽槽位（复用槽位并提交最新资产）；SSBO 缓存命中绕过完整类型检查（校验前置、写锁重查）；VAO Result 接口缺 mesh 仍 panic（改为 Err）。增加 1 库测试、3 链测试和 1 编译失败测试，更新 1 旧 VAO 测试。

最新源码验证：country-core 2098 项库测试 + 9 项资产链测试通过；NoUninit 编译失败测试 1 项通过；重新编译并在本机严格 Vulkan 运行 GPU 测试 1 项通过（容量 1020，重载后 live_leases=1）；tiny-glade 全部目标 check、限定 core Clippy 均 exit 0；16 文件格式检查通过。Clippy 保留既有及锁定派生宏辅助项告警，未使用 -D warnings。本轮实际成功结果取代上一轮“全工程重编受阻”的状态，失败尝试仍保留。

验证期间遇到其他任务的存档接口变动、临时探针移除、Cargo 锁和磁盘不足；只清理未锁定旧增量会话与旧核心编译归档，不删除源码/资产/证据。报告 `games_test/tiny-glade/reports/asset-libraries-rereview-2026-10-18/README.md`、verification.json；来源 `provenance/asset-libraries-rereview-2026-10-18.json`。下层通用缓冲的全部字节转换尚未全量收紧；没有整游戏/原进程或量化性能验收。


## 2026-10-18 country-core 手工重构保留与算法升级（用户永久约束）

用户明确：country-core 其他功能模块已经手工重构过，后续涉及这些模块时应以当前代码为起点添加功能、修复 bug，不能按照原版又重新复现一遍、覆盖已有重构或另造重复模块。用户随后补充：这不意味着保留简化算法；当手工算法过于简单，而原版挖出的业务复杂且现实现承载不足时，应直接重构当前手写算法、补齐业务并复用已有可用部分。原版证据裁定业务规则与边界，不要求复制原ABI/内存布局或回滚现工程架构。优先复用当前类型和接口，必要变更要一起修消费者，不留下平行的新旧实现。

持久目录约束：`games_test/tiny-glade/crates/country-core/AGENTS.md`。本约束适用于 systems/resources 当前恢复涉及的上层及依赖模块。现有共享 utils 的工程改进不整体回退；需要原业务专用语义时做必要的增量适配，并明确验证范围。

## 2026-10-18 systems/resources 恢复审查补充（仍进行中）

- 本轮按 B 原 RNE/PDB 做有界恢复；默认 task-index 的 C 标签不允许跨版套地址。ICF 的 grow_one 等名称不能证明载荷类型，必须核 PDB 完整 SystemParam、原 caller 写入和真实 consumer。地形旧 StairsPlacementBucket 类型误认已撤回，改接当前 PlayTerrainSound、教学、Arrow、CursorCapture 等实际 owner。
- 羊当前手工完整组件与 core 快照组件已统一 TypeId，保留当前 CritterGoal Rust 字段顺序；原字段偏移只用于证明归属，不复制原生布局。曲线在现 InProgressCurve/Rdp 上升级算法，改进版 Rdp 默认接口保留。
- 本轮公开 CPU 调度入口与并行真实 tiny-glade host 是两个入口：后者已新增 set10 mode /35-40-41 history/wall 接线，不能沿用“所有宿主未接”的旧结论；但未发现其调用 terrain CPU/tool 或 onboarding 新配置入口。勿重复挂 mode，也勿将局部测试称完整宿主验收。
- 原光标事件应传 analog0 位移（含捕获路径），非绝对位置。实际地形记录链仍需开启/结束事务；预先手动 start_input_recording 的 fixture 不能证明真实 UI 可录制。画布权限/参数 ABI、可写/不可写每帧生命周期也需一起核。
- 报告入口：tiny-glade/reports/core-systems-resources-recovery-2026-10-18/README.md；各有界证据、独审、before快照、Cargo 失败及源漂移分开保留。493 文件盘点/原执行样例数均不能当恢复完成率。

- 本轮端口追踪再次证实：不要跨事件套用EventInstance header偏移。不同载荷的EventId可在不同位置；原writer/reader/stride须同类型配对。rmb_autoclutter旧输出误命名EvRightClicked、尾EventId误作entity，真实输出为InstaCreateDeco；真实EvRightClicked是另一reader。新适配还须保留last点击和entity generation/d2s/bitset校验，不能用unwrap替代skip。相关候选正在独审，不能以type名已对上当完整消费者恢复。

## 2026-10-18 systems/resources 第二次续推进

仍按当前手工架构增量恢复，B发行身份不变。tree-history-r1 core2370/system-tree10通过，ordinary-wall-r1 core2382/真实graph1通过，两个run均source_drift=0；数字包含其他owner新增测试，不作为本轮独占或完成率。树使用既有TreeAnimation.hidden_this_frame、现WallSpatialHash与48B PrefabInstance(PlacementDataSsbo)，不造可见性数组；实际名在88B PrefabInstanceData。TreeData.id i32是placement variant，原TreeEdit外层另存u64 index；恢复index/tree与Move index/from/to保存、实际replay、完整已知Insert3字段。Insert末尾EventId不是height。Move→Insert→Delete及reserved实体query失败按原保留，不为合成测试补假出生。原nominal候选tuple仍有类型名歧义，数据面与当前唯一owner分别记。

普通新墙/栅栏UI/nursery/smoother/draw/finish/history已接现core CPU注册；真实首帧空registry→录制→墙authority→结束提交ledger→下一帧Control Sync通过。原UI320案例与smoother163次执行支撑各自范围；draw原动态entry曾fetch-unmapped，因账户usage限制无法自动审查新探针，未绕过/不称原entry执行通过。独审将两次ceil误认纠正为实际IAT roundf，非整比末点权重回归已过。FinishWall bool后续从实际writer确认是新墙/续画族，不可命名成cancel；续画/反向/合墙仍在恢复。

实际宿主独立intro→墙authority/spatial重验通过，但主机仍有未接Gameplay world graph等边界，不等于所有UI消费者已装入真实全局调度。与host协同以tiny-glade/reports/main-chain-recovery-2026-10-18/COORDINATION.md为入口，root新增helper不覆盖其render窄改。资源72项producer候选前置见core报告PENDING_RESOURCE_PRODUCERS.md；water/exclude/garden三mask已由独立worker合法claim恢复，颜色高32bits与每帧重建色表另处理中。

方法已增量沉淀到现tiny-glade-recovery/references/original-driven-integration.md：EventInstance不固定头、外层serializer不可漏、真实schedule.initialize、空registry首帧闭环、混合批次reader、reserved Commands失败与import校正数值thunk。项目反例清单RECOVERY_LESSONS.md继续维护；未另装重复技能。
