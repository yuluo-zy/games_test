# 游戏自有源码逆向任务与收益排序

## 2026-10-14 utils B 深挖增量

原楼梯2D dense-segment消费策略、共享标量运算順序及owned Curve XZ投影/stride已恢复并通过限定原指令验收，详情 [REPORT.md](evidence/utils-deep-followup-2026-10-14/REPORT.md)。用户明确只分析B。剩余墙bound/is_bounded/to_affine3、Spline2D全局查询和原借用as_vec2类型未获得正面编译体；已交叉扫描全PDB过程/内联/行号，不能通过复制3D或依据同址标签补造。优先寻找可归属的原消费链与数据流；采样开关写入者与完整消费者仍在队列，完整游戏/GPU后置。

## 2026-10-14 tg-text 内部指定缺口已关闭

用户限定的字体角色/共享Arc表与原帧缓存规则已直接恢复到当前crate：原角色查表/基准与实际metrics、set_family不写回角色的原行为、u32<及回绕、usage slot/gen/前缀副作用/零哨兵/同rawID接受、consume上传后推进；移除候选epoch/nonce和render隐式重排。原受控829例与Debug/Release各33 tests、严格Clippy/fmt通过。实际来源与验收见 `evidence/text-internal-2026-10-14/REPORT.md`、`tracking/project.json.tg_text_internal_recovery`；作者metrics typename仅元数据未知，不添加未证tuple From。

此前列为待恢复的字体共享模型和u32缓存规则不再作为当前内部缺口。调用方/native GPU按本轮要求不处理，保留后续独立任务；不恢复已排除的四个报告/云平台包，不升级整库/整游戏或A/C验收。

## 2026-10-14 当前 utils 续轮

已关闭原 insert 重复拒绝/新 key 稠密追加、完整 remove 返回值/映射修复，country-core 索引/固定网格消费者委托共享入口；230 条新增原指令样例通过，Windows utils Debug/Release各90、country-core 1142、严格Clippy通过。真实全部游戏消费者不据此升级。剩余为墙 bound/is_bounded/to_affine3、Spline2D全局查询的编译体定位，Vec3SliceView2 投影轴/stride，采样开关写入者和其他grid单态化、原完整进程及A/C。应先获得真实 compiled consumer/类型证据，不恢复已物理移除的猜测API，也不将无xref等同无调用者。交付与具体范围见 `evidence/utils-gap-recovery-2026-10-14/REPORT.md`；实际源码 `tiny-glade/crates/utils`，旧研究目录为历史范围。

## 2026-10-14 当前 integrity / text 续轮

按用户指定直接完善 `tiny-glade/crates/tg-package-integrity` 与 `tg-text`。已关闭：integrity 11 个 Windows 原路径/Unicode/surrogate 样例和诊断 example 提前 panic；text render/consume 数量快照、metrics 重建 Editor、scale 清理保留 overflow、非 Mask 原 RGBA 上传规则。当前工程两库 31 测试通过、0 失败/忽略，严格 all-targets Clippy、fmt 与 text gpu 编译通过；详情 `evidence/small-libs-followup-2026-10-14/REPORT.md`。

仍需逆向：TextFragment 原角色字节/共享 Arc<FontTable> 状态及 setter 完整合同；原 u32 atlas stamp/frame 的同帧顺序和 rollover；真实 rhapsody Texture/toolbar 消费联接。integrity 稳定磁盘原入口容器范围通过，不升级为真实游戏启动、全部 OS/并发或 A/C 验收。当前生产实现位于 tiny-glade，历史 shared-foundation 与历史 hash 保留；模块保持部分恢复。四个已排除报告/云平台包不重新纳入待办。

## 2026-10-14 主题三库当前交付

`theme-defs` / `theme-server-proto` 的B已发射合同及 `theme-fetcher` 已定位库内业务已静态复核补全，实际源码在 `tiny-glade/crates/`。26项离线测试、严格all-targets Clippy与fmt通过；没有请求真实主题服务。当前没有已知缺失的已定位B库内算法，后续工作是原完整入口验收、真实主题plugin与langs/meta/history/credits消费接线、未发射文本和A/C范围。详见 [REPORT.md](evidence/theme-static-2026-10-14/REPORT.md) 与 [机器台账](tracking/theme-static-recovery.json)，不能继续把旧日志/HTTP处理时序缺口当作当前未实现。

2026-10-14 范围调整：用户明确删除 `tiny-glade/crates/tg-integrity-report`、`tiny-glade/crates/crash-reporter-gui`、`tiny-glade/crates/feedback-reporter` 与 `tiny-glade/crates/geforcenow`；四者不需要逆向还原，不计后续恢复缺口。旧恢复与证据仅作历史保留。



## 2026-10-14 数值测试合同后续修复

零切线直接构造固定负 quiet NaN；圆形/Elastic 的原样例含 Windows UCRT 舍入，跨平台测试分别采用有界误差，Windows 分支保留逐位。utils debug/release 均 93 项通过、无忽略；这不是 Windows 原版逐位验收，也不升级完整业务恢复。详见 `evidence/foundation-review-fixes-2026-10-14/numeric-followup/REPORT.md`。此前 87/3 为历史断点。


## 2026-10-14 三库修复当前断点

已修复数组纹理所有叶文件的内容缓存失效、旧 metadata 迁移、零 mip/零尺寸/对齐溢出错误，以及独立法线选轴和 directed local-edge 距离中错误的 abs。utils 当前 87 通过/3 失败（圆形 4 ULP、越界切线 NaN 符号、缓动 1 ULP）；asset 15、asset-pipe 15 通过，三库严格 Clippy 通过。仍为候选/限定验证，不能说所有 B 业务实现基本完成或三库全部测试绿色。libm 根因待同平台/中间值证实；三个墙符号改为 compiled_body_not_located，未证明未编译。新增缓存和参数处理属于产品策略变化，旧原版等价报告不覆盖。详见 `evidence/foundation-review-fixes-2026-10-14/REPORT.md`。以下历史记录中的更强完成度/未编译/全绿表述由本断点纠正。


## 2026-10-14 共享基础三库收口与差分修复

用户选定 utils / asset / asset-pipe 三库完成实现+测试，明确"不造轮子用依赖库函数"，无证据符号主动逆向。已交付：`Spline2D` 2D 查询、`Vec3SliceView2` 投影访问、asset render 深层默认测试、`asset-pipe::texture::block_bytes`、shipped 三测试本机路径回退；三库 `cargo test` + `clippy -D warnings` 全绿。逆向结论：`WallSpaceCoord::{bound,is_bounded,to_affine3}` B 中未编译（仅 PDB inlinee 声明），语义不可恢复。差分修复：`rectangle_deformation` 与 `rectangle_distance_normals` 均过（get_closest_edge_id 0x140c9acb0=逐边线段距离首个最小；expand_along_edge 0x140c9c620=clamp 下界 0 非 min 参数；closest_edge_normal=|local|/size 选轴）；utils 现 **87 通过/3 失败**（circle_boundary MSVC/macOS libm、curve2_rules NaN 符号、easing 1 ULP）。证据：`evidence/foundation-completion-2026-10-13/utils-gaps-2026-10-13/REPORT.md`；台账：`tracking/foundation-business-functions.json`（175 partial）、`tracking/unrecovered-shared-foundation-interfaces.json`（185 条，9 已实现移出、3 墙符号 not_compiled_in_b）、`tracking/project.json`、`PROJECT_PROGRESS.md`、`PROJECT_MEMORY.md`。


## 2026-10-14 共享基础三库（theme-fetcher / feedback-reporter / tg-gpu-selector）库内收口

用户选定共享基础模块三库做**库内**实现/校验（明确不做上层游戏模块接线）。对每库：枚举 B 发射面原函数 → fresh 反编译逐项对照 → 修真实差异 → 隔离验证 workspace（`/tmp/tg-closures-validation-Ckh1`，不动共享 workspace）test/clippy 验证。

- **theme-fetcher**（已完成）：`new@0x14054aa00`、`run@0x140618310`（内联 fetch_events/download_blob/validate_cas_cache_entry/trim_cas）、`checked_zip::extract_with_checksums@0x140d921c0`、`verify_folder_checksums@0x140d939a0` 逐项对照，关键常量与 1s 睡眠经机器码字节核验。修复 D1：下载循环触发条件——原对**任意 Err 变体**每 1s 循环重试（Result 判别字非 0），Rust 原只重试 Pending → 改 `is_err()`。11 测试 + clippy `-D warnings` 通过。
- **feedback-reporter**（已完成）：`build@0x1406901f0`、`report_player_feedback@0x1406903a0`、`zip_dir@0x141002650`（内联 send_report/prepare_report/zip_benchmark_data）逐项对照，601/等待分支、multipart 顺序/MIME、HTTP 失败规则、obfstr endpoint 字节核验。修复 zip_dir 缺失跳过：原 `stat` 后**仅 NotFound（Windows err 2）跳过**、其它 metadata 错误落 `is_file()` panic → Rust 改 `ErrorKind::NotFound`-only-skip。9 测试 + clippy 通过。
- **tg-gpu-selector**（验证进行中）：target 8 个编译函数 + 内联对照中。

证据：`evidence/closures-2026-10-13/{theme-fetcher,feedback-reporter,gpu-selector}/`（B-SURFACE/BusinessLogic/Compare/Verification）。库外接线（tiny-glade 主题 plugin 消费、country-core 反馈 producer、rhapsody 设备选择接线）与 A/C 仍留，不计库内完成。注：`scripts/update-module-progress.py` 重生成被并发会话悬空引用 `reconstruction/math/window_aspect.rs`（文件不存在、无 module glob 扫描）阻塞，MODULE_PROGRESS.md 两行已手工同步，待其修复后脚本重生成。


## 2026-10-13 小型基础库第二批（batch-02）

三个并行任务深化 motomoto、tg-package-integrity、tg-text（共享 workspace 中另 agent 占用的六个库之外）：反编译核对原函数并修正候选差异。motomoto 的 add_chunk/Drop/iter/next 与 B 反编译逐条一致（12 测试过）；tg-package-integrity 的 verify/find_extra_files/checksum 叶一致，修复合成测试 OS 平台相关失败（`build-info/manifest.json/../manifest.json` Unix=Missing/Windows=Corrupt）并补 serde/schema 边界测试，登记报告/崩溃消费者接线（7 测试过）；tg-text 修正 rect 签名（opacity/shapes，颜色固定经 scale_u32_packed_srgba_color）、set_internal_scaled_metrics 签名（NaN 守卫）、scaled 操作数序，cosmic-text/etagere 接线经反编译确认，登记 prepare Action 0x22 break/bounds/replace_editor/render getter 差异（behavior 6 + text_pipeline 9 过，2 Windows 专属字体测试加非 Windows ignore）。三者严格 clippy 通过，无新增依赖。证据：[整合报告](evidence/small-libs-batch-02-2026-10-13/REPORT.md)。剩余：原入口 shipped-process runtime 全链、存档/报告 schema 完整消费、GUI 像素与 A/C。

2026-10-14 batch-02 完成轮：按用户指示补齐除"真实消费者接线/A/C"外的可闭合缺口。tg-text 确认 fork 中 cosmic_text::Action 为 32 字节/扁平标签 0x0–0x21，0x22 break 分类为超范围无效标签（防御守卫，行为等价）；set_text 入队动作改 BufferEnd（原 tag 0x14，光标落末尾）、prepare bounds[1] 改 scaled line_height、replace_editor 改 void + take_editor 用 mem::replace。integrity 新增 symlink/权限探针测试（8 测试过）。motomoto 补 as_tag/tag_from_str_trunc 反编译一致，登记原仅 Cursor 单态化范围、ChunkWriter 三字段 ABI 观察、Reader::new 地址勘误（batch-01 误标）。35 测试过 + 2 Windows 专属忽略，严格 clippy 过。剩余边界（真实消费者接线、A/C、shipped-process runtime 全链、render getter 契约、TextRenderer::new 装配、NaN 工具链、32/64 位 Hash）见 [REPORT 验收边界](evidence/small-libs-batch-02-2026-10-13/REPORT.md)。

完成度判定（2026-10-14，详见 [REPORT](evidence/small-libs-batch-02-2026-10-13/REPORT.md)）：按功能纳入规则枚举各库 PDB 业务函数后逐项核对——**motomoto 与 tg-package-integrity 库内业务逻辑实现+反编译验证完成**；**tg-text 库内业务函数全部实现、核心算法反编译验证**，保留项为简单 setter 未逐一 fresh 反编译复核、`render` 返回类型重建扩展、`TextRenderer::new` 装配细节、依赖/工具链边界。无"PDB 有业务函数但候选完全未实现"的情况（唯一例外 `image_name_to_index` 原文即 panic stub）。未闭合项全部落在用户保留范围（消费者接线、A/C）加登记边界。

2026-10-13 paths/lang-lib/tabloid 原生复核完成：三个库全部 B 发射函数已实现并逐函数反编译复核（证据 evidence/foundation-plt-2026-10-13/）。结论无库内未实现功能；TG_DISABLE_LANG_FALLBACK 判别式已交叉验证正确；tabloid 通道归属修正（ColliderDiff 仅 0x141453c00/0x141454410/0x1412926e0，Delta 类型为 ColliderDelta）。交付：tabloid DirtyTiles 语义契约测试 + paths 初始化限制边界测试；lang-lib 仅登记 load_locale 写失败偏差（保留 log）。库外缺口不变：真实 producer→consumer 与 DirtyTiles/ColliderDiff State/StateDelta 实现在 country-core。详见 [PROJECT_MEMORY](PROJECT_MEMORY.md)。

2026-10-13 最新B基础库批次：用户要求C先不管，集中B。3 agent已交付8库到[共享workspace](reconstruction/shared-foundation/Cargo.toml)，32普通Rust tests与严格Clippy通过，414项公共依赖身份完全匹配原B锁。主题/cache、报告/GUI、反馈插件/GFN为候选实现，协议schema不计手写算法；详见[交付及剩余边界](evidence/foundation-unstarted-2026-10-13/REPORT.md)。下一闭包是原tiny-glade每日主题插件→fetcher、country-core/UI反馈producer→reporter、rhapsody真实设备/driver_support→选择结果消费及启动完整性/crash回调接线；先恢复真实字段/类型再接入，不重复派生visitor或公共依赖。完整原oracle与A/C适配保留，C本轮不推进。

基础库的详细功能拆分见[基础库业务功能清单](FOUNDATION_BUSINESS_FUNCTIONS.md)。推进单位为游戏侧手写规则与生产消费闭包；自动派生、宏展开、通用库实现不列功能任务。未解释的入口、归属待确认和纯数据契约保持各自标记，原系统验收状态不变。

当前共享七包的源码对照以[业务功能对照](FOUNDATION_BUSINESS_COMPARISON.md)为准：98行中9条utils功能未实现、22条有部分库内逻辑缺口，原验证与库外接线分开。先补高度图状态化区段输出、墙空间/重采样等实际闭包；不因缺同名helper、原图片或宏生成符号重复建立“无实现”任务。

当前阶段是源码恢复；场景回放、GPU与最终画面验收后置。原作依赖全部保留在[dependency-policy.json](tracking/dependency-policy.json)，不要把901个package当作901个逆向任务。依赖实现不计入游戏源码覆盖。

2026-10-12 最新用户指示继续渲染框架源码恢复和编译/原码测试：S25已完成确定性rhapsody工作图与三批并行恢复。批次01恢复ResourceInfo、Registry/Pool、CompiledGraph/Batch；批次02整合小图DSL、executor/temporal与tracked GPU生命周期；批次03关闭barrier/record分段、sequence/timeline分域和temporal完整payload主要路径。累计新增512个ResourceInfo快照、1600个BatchCompiler、29692个生命周期叶、1344个barrier和97个population原入口案例零差异；批次03候选13项Rust测试通过，跨包门禁13/13。CPU `compile -> barrier/record projection -> submit events -> immediate retire` 与 deferred GPU disposal合同进一步闭合；真实Vulkan/GPU、完整pass recording、counter构造、temporal reset/drop、pipeline/shader和游戏pass接线继续恢复。第三批门禁见[evidence/render-recovery/parallel-batch-03/integration.json](evidence/render-recovery/parallel-batch-03/integration.json)。

## 当前选择

2026-10-13 用户指定先完成真实 DXC 和预处理/打包及来源调查：已在现有 rhapsody-rebuild 交付可调用完整构建入口，404 新包/638 阶段生成，10 测试、严格 Clippy、新 fill GPU 读回和 egui 布局通过。精确复用 shader-prepper/SpirQ fork/rspirv/Speedy；shader-prepper 为作者公开库，五个 local 工具包公开上游仍未知。新编译反射 210/404 与原一致；170 compute 原宏 tuple 未独证（其中 38 非零 key 未知）、原 DXC/fork/provider/异常/cache 与 shader 算法残差保留。当前构建证据见 [SUMMARY](evidence/shader-build-2026-10-13/SUMMARY.md)。后续优先从原 pass/variant 生产者恢复完整宏 manifest，再逐项修复绑定、vertex inputs、uniform 和 local_size 残差；原资产库状态机与整帧属于后续范围，不因构建链通过计完成。

最新执行规则以[WORKFLOW_RESET.md](evidence/render-recovery/WORKFLOW_RESET.md)为准。旧批次成果保留，停止继续创建R8等孤立函数包。优先把graph/registry/record/counter恢复结果接进单一共享调用入口，消除手填sequence与record输入；按真实consumer、私有stub移除和链级验收记进度。以下旧路线和批次数字仅作历史范围，不覆盖新规则。

后续实施：共享 `ExecutionChain::execute_graph` 已接通graph声明→compiled→record→counter→实际queue数组→receipt/completion；正常、callback失败、queue失败3个候选链场景通过。下一任务为原shader/interface选择生产者→descriptor共享callback及真实wait/signal生产者；managed资源物理生产者、temporal跨帧和country-core consumer仍待接线。候选链不等同原完整compile ABI/原整链oracle，GPU未运行。详见 [共享入口验证](evidence/render-recovery/parallel-batch-06/shared-entry-validation.json)。

最新功能范围：用户授权的新目录 `reconstruction/rhapsody-rebuild` 于 2026-10-13 在 lavapipe 复验 132 测试通过（2026-10-12 的 64 测试/13 探针/25 验收命令快照已过时），其 [MODULE_STATUS](reconstruction/rhapsody-rebuild/MODULE_STATUS.md) 是当前功能进度依据。旧共享入口新增带当前Command和compiled资源的callback上下文，4链级场景通过。下一功能链为主图registry→真实recording Command→managed descriptor→首个shader callback→同一submit/回收；新目录仍为两阶段桥。并行录制（`record_batch` 分块）已恢复、命令池来源等未恢复；跨队列所有权转移已查明原作没有（见 evidence/render-recovery/multi-queue）；缓存淘汰未调查；资产pass关联和本地分叉等价性继续保留。详情见 [消费链复核](evidence/render-recovery/consumer-chain-review-2026-10-12/README.md)；前段“GPU未运行”仅是旧共享CPU范围。

当前用户指定先审计逆向工具、再创建多个agent继续 `rhapsody`。已完成审计与3个agent本轮交付：第四批统一门禁50/50、第五批27/27通过。第五批新增Primary调用者180、非空资源record/submit774、descriptor layout/pool/compute创建边界5184原指令案例零差异；5+2候选测试通过。工具runner27个本地协议测试通过，不计作游戏源码恢复。

接下来按 **真实sequence生产接口适配 → 多队列真实调用/并行录制/完整queue前段 → sparse layout/descriptor writes/完整pipeline → country-core最小pass接线** 推进。模块状态、报告范围和下一步验收目标详见 [RHAPSODY_PROGRESS.md](evidence/render-recovery/RHAPSODY_PROGRESS.md)。xref缺少记录不等于无caller，跳表伪码需限定反汇编核对；缓存缺provider/数据库revision输入默认禁用。墙体、屋顶与历史任务保留在下表，当前优先恢复渲染框架。

2026-10-13 已完成 country-core 渲染域第一批并行重写（业务逻辑→整理→骨架实现）：5 个工作包（pass 声明层/GPU buffer 原语/nani GPU-driven/mesh·资产接线/render_loop 帧 passes）交付合同 + 骨架对齐 Rust，组装为独立 crate `reconstruction/country-core/`（111 .rs），offline cargo check + all-targets clippy 复验通过（0 错误）。边界统一为 `rhapsody_contract`；`nani/systems.rs`（20 个 ECS 系统）依赖全游戏 components/resources 图而暂缓。整合报告 `reconstruction/country-core-render/_integration/INTEGRATION_REPORT.md`；全部 implemented_unverified，GPU/画面验收未开展。

2026-10-13 country-core 全量恢复完成（21 个工作包，WP1–21）：渲染域（WP1–6）+ 内容域（WP7–10 水/地形/屋顶/植被生物）+ 墙/存档/音频/UI 等（WP11–16）+ 回放/draw_shaders/风格化/相机·UI·时间/资源余量（WP17–21）。终局 crate `reconstruction/country-core/` 570 个 .rs，offline cargo check 0 错误、all-targets clippy 0 错误（亲自复验）；覆盖审计 skeleton 539 源路径实质全量覆盖（仅 3 个 mod.rs 路径名差异）。全部 implemented_unverified，typed gap 登记于各 `wp*/contracts/`；整合报告 `_integration/INTEGRATION_FINAL.md`。GPU 运行时/画面验收仍后置。

优先级为基于现有证据的工程判断，不是准确工时预测：P0立即进行；P1核心后续；P2在状态/几何基础后；P3当前后置。每项必须交付游戏自有Rust源码、输入输出/类型证据和必要的原码校验，不能以调用了反编译工具或写了提取脚本代替交付。

| ID | 游戏自有任务 | 优先级 / 收益 | 明确逆向内容 | 直接复用/排除 | 交付与现状 |
|---|---|---|---|---|---|
| D00 | 原依赖适配纠正 | P0；消除重复工作和行为偏差 | 库版本/特性/调用顺序与游戏接口 | fastrand1.8.0/glam0.29.3/half2.4.1 | 已改用原版API；圆形/矩形全记录、seed及RNG回归通过 |
| S01 | 权威对象与ID模型 | P0；解锁全部系统 | Wall/Roof/Shape/Decorator字段、枚举、ID和所有权 | Bevy/slotmap/Vec/Arc实现 | 保留已证实布局，未知字段不补造 |
| S02 | 墙体高度与屋顶标高规则 | P0；连接已恢复屋顶并服务墙/楼梯 | clamp_wall_height、max_y、flat_roof_y、形状高度分支 | glam数值API、terrain容器、ECS查询 | 数值约束后段和3个缓存叶函数已实现；6624案例通过；查询前段/flag生产者待恢复 |
| S03 | 屋顶编辑/生命周期 | P0；把算法接到权威状态 | create/edit/update/delete/switch参数、触发条件与输出事件 | Commands/Query/Event库实现 | edit已解析后段、变化量/反馈源码已实现并通过4514案例；完整ECS/lifecycle待恢复 |
| S04 | Roof参数/枚举消费者 | P0；消除语义猜测 | height/profile/eave/ridge/tip和Width/Length映射、apply/replay字段更新 | serde/serde_json/ron解析器 | 328参数快照已提取，edit 0/1/2及对应字段消费者已核实；序列化/方向映射仍待恢复 |
| S05 | 历史事务/撤销数据逻辑 | P1；解锁可靠编辑 | mutation token、链索引、前后diff、提交/撤销/重做消费者 | Arc/drop/线程局部库、通用容器 | 已定位收尾，尚未确认全部容器语义；不运行场景回放 |
| S06 | 墙轮廓与形状编辑 | P1；核心建造输入 | circle/rectangle/freehand、闭合、变形、伸缩与形状转化 | glam、Bezier/geometry已有库及原fork | 源码规则和曲线输入输出闭环 |
| S07 | 墙几何/砖块/支撑 | P1；实现建筑主体 | 砖列、支撑、墙裙、拱口、边缘和实例布局 | RNG/half/glam、已有mesh依赖 | construct_brick_columns已精确定位；完整算法待恢复 |
| S08 | 建筑交叠与连接策略 | P1；复现自动适应 | 墙-墙/墙-屋顶关系、相交后修剪、脏变化传播 | Rapier/空间索引库通用算法 | 恢复游戏关系策略，不重写碰撞引擎 |
| S09 | 门窗/装饰附着 | P1；核心可见编辑 | 墙/屋顶UV宿主、位置/尺度、变形/宿主变化 | ECS/数学/容器实现 | Decorator diff数据已梳理，规则源码待恢复 |
| S10 | 门阶与楼梯 | P1；连接地形和建筑 | 阶梯分段、门口/地面适配、前后边关系 | 曲线与数学依赖 | construct_door_stairs已有4668B边界；CPU源码待恢复 |
| S11 | 开口/连窗/装饰成组 | P2；提升建造完整性 | 自动合并/分离、挤压、重叠限制与样式切换 | 几何/容器库 | 按真实消费者识别分支 |
| S12 | 地形编辑 | P1；世界基础 | 网格schema、笔刷核、插值/平滑、标高和建筑基础传播 | 已有noise/math/栅格容器库 | 恢复游戏调用/参数/组合，不恢复公开noise库 |
| S13 | 道路/石板/石子 | P2；打通地形和建筑 | 曲线笔刷、宽度/端点、交叉、台阶/墙开口关系 | 原CurveU/几何公共依赖 | 恢复自有布局和关系规则 |
| S14 | 水域CPU状态 | P2；补完整世界 | 水面标高/范围、岸线、输入约束、相邻对象变化 | 依赖库；水shader当前后置 | 状态与CPU源码先做 |
| S15 | 树/植被/园景生成 | P2；重要内容系统 | 自有种类、形态参数、散布、种子、地形/建筑关系 | fastrand/noise/glam；局部path包先核实归属 | 依据private source路径区分自有与已有库 |
| S16 | 动物/小生物行为 | P2；补游戏活性 | 自有移动/目标/状态机/避让与参数 | 通用寻路/碰撞/数学依赖 | history已有Sheep状态样本，不据此猜整个状态机 |
| S17 | 季节/天气/昼夜CPU控制 | P2；补环境状态 | 自有参数、过渡曲线、季节物件和事件 | suncalc等已有库；光照GPU后置 | 恢复控制代码和数据 |
| S18 | 音频游戏事件 | P2；补交互反馈 | 事件触发、材质/状态参数、随机选取、淡入淡出 | FMOD/libfmod/音频DSP实现 | 恢复游戏胶水，不逆向FMOD |
| S19 | 输入/拾取/相机/拍照逻辑 | P1/P2；可交互工程 | 指令映射、拖动阶段、gizmo策略、选择/相机约束 | Bevy输入/dolly/winit/平台接口 | 恢复游戏层输入与约束，库实现排除 |
| S20 | UI/工具栏/引导/本地化 | P2；可用产品 | 工具状态机、自有布局/选项/引导/资源key | egui/iced/字体平台库 | private包装与fork变化先核实，不重做整套GUI库 |
| S21 | 存档/快照/迁移 | P1；持久世界 | 游戏schema、压缩/打包调用、版本迁移、缺失快照处理 | 原rkyv/ron fork、serde、flate2等 | 22缺失引用登记；恢复源码不等于已回放全部样本 |
| S22 | 资产/prefab/mesh metadata | P1/P2；供全部模块取数据 | 游戏格式schema、资源ID、atlas/prefab构建与绑定 | rkyv/ron/image/dds等公开依赖 | 解自有schema和资源关系，不重写通用解码器 |
| S23 | 更新调度/缓存/任务 | P1；模块一致性基础 | 自有脏标记、依赖顺序、重建/取消/缓存策略 | Bevy scheduler/tasks与线程库 | 只恢复游戏系统的注册与依赖 |
| S24 | 引擎分叉必要适配 | 按实际阻塞选择 | 可证实且影响游戏调用的fork差异 | 原Bevy/引擎整体 | fork仓库/commit待核实；不从0恢复Bevy |
| S25 | 渲染框架/GPU游戏专属passes | 当前P0 | 自有资源图/同步/执行框架与pass/参数/效果 | 原ash/vk-sync/allocator/驱动依赖 | 共享graph声明→compiled→record→counter→queue→completion候选入口已接通，3个链级场景通过；descriptor接口已交付但真实shader选择/shared callback、managed物理生产者、wait/signal、多队列/并行、temporal跨帧、完整pipeline、原整链oracle和GPU仍未闭合 |
| S26 | 场景回放/视觉/性能/完整游戏集成 | P3；最终验收 | 未来跨模块功能/画面/存档/性能对照 | 已复用依赖及完成模块 | 当前后置，不作为源码阶段的先决条件 |

## 具体入口与本轮状态

| 入口 | 作用/为何优先 | 当前证据 |
|---|---|---|
| clamp_wall_height `0x140ac7f30` | 权威墙高度约束，解锁高度编辑/屋顶/门阶共用规则 | 匹配PDB422B；MCP读取完成，数值后段已实现并原码验证 |
| history_record_adjust_wall_height `0x140ac79e0` | 高度操作到历史事务联接 | 匹配PDB1349B；先查消费者 |
| edit_roof `0x1421ab7c0` | 现有屋顶核心的主要编辑入口 | 匹配PDB3175B；实际MCP读取；已解析编辑后段4514案例通过，ECS前段排除 |
| PublicWallState::max_y `0x140b2a620` | create/update Roof均使用的标高来源 | 匹配PDB82B；缓存值/缺失panic条件已恢复并原码验证 |
| PublicWallState::flat_roof_y `0x140b2a6e0` | 平屋顶与墙状态联接，补已恢复算法的缺口 | 匹配PDB90B；max_y偏移和缺失panic条件已恢复并原码验证 |

具体证据增量保存在 `evidence/scope-priorities/` 与 `evidence/mcp/`。实际MCP工具catalog、创建/反编译/引用调用必须保存结果；失败或缺失范围保留，不把MCP注册正确当成工具已使用。

所有新实现都按[项目记忆](PROJECT_MEMORY.md)排除依赖代码，并更新[项目进度](PROJECT_PROGRESS.md)和机器可读任务台账。


## 2026-10-13 shader 缺口调查后续

已完成原非零 compute 38 tuple、原 runtime load_compiled、404/638 实际预处理／DXC／打包及19冲突包修复。接下来按 evidence/shader-gap-investigation-2026-10-13/SUMMARY.md 的具体残差恢复7个反射差异、132个zero-key宏、uint2物理地址fork适配及图像格式；确认原离线CLI/provider/cache/temp策略。397包typed反射一致不能计为完整着色语义恢复，整帧后置。

## 2026-10-13 小型基础库第一批

三个并行owner已交付motomoto、tg-package-integrity及tg-text的限定CPU子链，详见 [整合报告](evidence/small-libs-batch-01/REPORT.md)。下一缺口：motomoto原Writer/Drop与真实save schema消费者；完整性原入口oracle/报告consumer及特殊OS/schema；tg-text真实TextFragment/TextRenderer策略与原cosmic-text/etagere接线。原依赖继续复用，候选编译/测试与完整原业务验收分开。
