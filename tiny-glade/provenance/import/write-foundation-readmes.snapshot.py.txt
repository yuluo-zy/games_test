"""Write package status documents from the audited source selection."""
from pathlib import Path
import json

ROOT = Path(__file__).resolve().parents[1]
RESEARCH = '../../../reverse-engineering/tiny-glade/'
INFO = {
    'utils': {
        'overview': '游戏自有的几何、曲线、坐标、数值与缓存辅助库，供建筑、相机和渲染数据生产使用。向量、随机数、半精度、容器和序列化直接复用原版本依赖。',
        'modules': 'geometry/（AABB、圆、矩形、射线、三角形、高度图）；curve/（Curve2/3、参数化、采样、切片）；natural_cubic.rs、wall_space.rs、loop_range.rs；easing_functions.rs、rng.rs、rgb9e5.rs、index_condenser.rs；其余命名/文件/计时工具。',
        'done': '已恢复几何与曲线数值规则、自然样条构造和局部求值、3D 全局样条查询、墙空间转换、范围切片、缓动、随机分割、颜色打包与索引缓存 get/remove/clear。原指令 fixture 测试已搬成本包内文件。',
        'gaps': 'WallSpaceCoord::bound/is_bounded/to_affine3 未找到编译函数体，未加入猜测实现。2D 样条全局查询、Vec3SliceView2 的投影轴/stride、IndexCondenser::insert 原生产接口仍不明确：这些猜测方法及仅验证假设的测试已从正式 package 物理移除，原调查实现仍保存在 reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/utils/。已证实的 2D 构造/局部求值、3D 真实查询、借用 pointer/length 传输和索引 get/remove/clear 保留。country-core 内 IndexCondenser/FloatGridCells 有其消费者证据与不同接口，当前保留在那里，后续逐合同合并。跨平台 CRT/浮点逐位等价与真实全部游戏消费者仍需验收。',
        'evidence': ['evidence/foundation-completion-2026-10-13/utils-gaps-2026-10-13/REPORT.md', 'evidence/foundation-review-fixes-2026-10-14/REPORT.md', 'evidence/foundation-review-fixes-2026-10-14/numeric-followup/REPORT.md'],
        'deviation': '源工作树数值修复按导入时哈希登记，不能将最后 Git 提交当作这些修复的验收。正式包仅留已恢复内容，已删除此前实验门控的猜测 API 与 feature。原 get/remove/clear 测试直接初始化已观察到的私有数据状态，不借用猜测 insert 生产实现；未改研究目录。geometry/native_tests.rs 选择与已验收报告输入 SHA497528a3 相同的 Git fixture，未选用工作树中反向覆盖已确认 expand 下界和新增未验收样例的改动；两份哈希与选择理由均保留在审计清单。',
    },
    'paths': {
        'overview': '管理安装目录、账户配置和 saves/beta-saves/backup-saves/tmp/deleted-saves 的创建与查询。',
        'modules': 'src/lib.rs；tests/global_initialization.rs。',
        'done': 'B 版目录创建顺序、Steam 账户子目录、Saved Games 获取与 getter 均已有恢复。当前持续开发版本增加初始化错误返回、重试/并发边界、绝对路径化、非 Unicode 路径、Windows/macOS/Linux 默认路径，以及 preference_root_path。',
        'gaps': '跨平台默认策略与安装目录来自产品修复，并非 B 原版证明；Windows/Linux/Rust 1.86 多平台运行尚须复验。启动与真实存档生命周期仍需主工程接线。',
        'evidence': ['evidence/foundation-plt-2026-10-13/SUMMARY.md', 'reconstruction/shared-foundation/FOUNDATION_LIBRARY_REPAIRS.md'],
        'deviation': '保留 c05fad96 的持续开发修复：重复初始化返回 Err，不再 panic；installed 使用可执行文件父目录；不会默默回滚为旧等价快照。',
    },
    'lang-lib': {
        'overview': '加载 YAML 语言包，发布本地化快照，并保存/恢复最近语言偏好。',
        'modules': 'src/lib.rs（Localization 与全局服务）；src/language_library.rs（语言包与缺失 key 缓存）；src/tests.rs、tests/global_configuration.rs、examples/。',
        'done': '已恢复 pack 加载、en-US fallback、TG_DISABLE_LANG_FALLBACK、缺失 key 格式、偏好文件策略。当前版本采用 Arc<str> 和 ArcSwap 的可回收快照、显式配置、完整包事务加载、1024-key 缓存上限、验证语言标识和临时文件替换偏好。',
        'gaps': '真实游戏 UI key/语言 producer 的全链接线、发行语言包全量消费，以及各 OS 偏好替换行为需持续验证。安全所有权、缓存上限和事务偏好写入的产品策略不受历史原版逐函数报告覆盖。',
        'evidence': ['evidence/foundation-plt-2026-10-13/SUMMARY.md', 'reconstruction/shared-foundation/FOUNDATION_LIBRARY_REPAIRS.md'],
        'deviation': '保留 c05fad96 产品修复的 owned API。写偏好失败会记录日志，原 B 静默忽略；独立 Localization::load_locale 与带保存入口分离。',
    },
    'tabloid': {
        'overview': '游戏增量状态发布/订阅通道，订阅者批量消费从上次 take 之后积累的变化。',
        'modules': 'src/lib.rs；tests/dirty_tiles.rs；examples/review_publication_cost.rs。',
        'done': '已恢复 publish/subscribe/take 及克隆订阅者共享消费边界、晚订阅只收未来变化、pending 合并规则。DirtyTiles 的空状态/OR 合并有原码证据。当前版本使用不可变 Arc 快照、弱订阅登记、每独立 pending batch 一次 clone/apply，并处理序列耗尽。',
        'gaps': '真实 country-core 的 DirtyTiles/ColliderDiff producer→consumer 联接以及其 State/StateDelta 实现是库外任务。State 的独立 Clone、不可重入/不 panic 合同仍需调用方遵守；高增长状态的拷贝成本需实际负载测试。',
        'evidence': ['evidence/foundation-plt-2026-10-13/SUMMARY.md', 'evidence/foundation-plt-2026-10-13/TABLOID_WIRING_GAP.md', 'reconstruction/shared-foundation/FOUNDATION_LIBRARY_REPAIRS.md'],
        'deviation': '保留 c05fad96 产品 API：publish 返回 Result，PublicationReader 不持 read lock，订阅者可即时回收。历史原版报告不验证这些所有权与耗尽策略改变。',
    },
    'motomoto': {
        'overview': '存档使用的带 8 字节 tag 和长度头的 chunk 容器；这里只处理容器业务，不重写 IO 或压缩器。',
        'modules': 'src/lib.rs（Writer/ChunkWriter/Reader、tag 编码）；src/tests.rs、src/tests/io_contract.rs。',
        'done': 'tag 长度检查/截断、add_chunk 错误槽顺序、Drop 长度回填、Reader iter/next 的截断尾部/UnexpectedEof/溢出边界均实现并有原入口复核；存档 capture/read 消费者已定位。',
        'gaps': '实际世界 schema 和压缩快照消费者接线、发行进程整链、A/C 仍未验收。B 仅观察到 Cursor<&mut Vec<u8>> 单态化；泛型非 Cursor 的失败/flush 合同属开发适配。ChunkWriter 的原三字段 ABI 尚未追到候选字节布局。',
        'evidence': ['evidence/small-libs-batch-02-2026-10-13/REPORT.md', 'evidence/small-libs-batch-02-2026-10-13/motomoto/BUSINESS_RULES.md'],
    },
    'tg-package-integrity': {
        'overview': '检验发行包 manifest 和文件内容，将结果分成 missing/corrupt/extra 供启动与崩溃报告消费。',
        'modules': 'src/lib.rs；fixtures/manifest-contract.json 和 B-manifest.json。',
        'done': 'BLAKE3→Hash 长度前缀→seeded FNV low16 门、version==1 schema、并行文件校验、extra 递归扫描、tmp/符号链接跳过及读取失败策略均恢复；报告/崩溃调用者已经映射。',
        'gaps': '正式启动/崩溃全链仍待接线；发行进程 oracle、A/C 及特殊 OS 权限边界未全验收。32/64 位 Hash 长度前缀存在原平台相关性，不声明跨平台 manifest 一律互通。',
        'evidence': ['evidence/small-libs-batch-02-2026-10-13/REPORT.md', 'evidence/small-libs-batch-02-2026-10-13/tg-package-integrity/BUSINESS_RULES.md'],
    },
    'theme-defs': {
        'overview': '主题获取器与游戏之间的活动事件传输合同，承载数据目录、开始/结束时间和错误状态。',
        'modules': 'src/lib.rs（ActiveEvent、EventDataError、is_active_interval）。',
        'done': '活动数据字段、错误家族和 start<=now<end 半开区间规则已恢复；字段/错误名称为语义重建名。Clone/Eq/thiserror 由依赖派生。',
        'gaps': '完整主题资源 schema、真实 tiny-glade plugin 消费及 A/C 待恢复/接线。这里不包含主题下载或游戏季节内容实现，也未声明原 Rust ABI/错误 wire 格式。',
        'evidence': ['evidence/shared-foundation-2026-10-13/SUMMARY.md', 'evidence/foundation-unstarted-2026-10-13/REPORT.md'],
    },
    'tg-text': {
        'overview': '字体角色、文本编辑/整形、字形 atlas/cache 与 GPU primitive 数据生成，复用原 cosmic-text fork 和 etagere 通用算法。',
        'modules': 'src/font_table.rs、shape_stream.rs、color.rs；src/fragment.rs（TextFragment）；src/renderer.rs（TextRenderer/RenderBatch）；src/gpu.rs 与 examples/text_gpu.rs（可选独立离屏适配）。',
        'done': '此前遗漏的完整 TextFragment/TextRenderer 已迁入：字体装载、编辑动作队列、prepare/render、glyph cache/atlas、48B primitive、consume 与上传数据协议。已包含 BufferEnd、line-height bounds、rect/opacity 与 replace/take 的后续原码修正和全部本地测试 fixture。',
        'gaps': 'TextRenderer::new 全部装配、render 原 getter 返回合同与少量 setter fresh 复核仍需完善；真实 toolbar/SSBO/GPU consumer 和 A/C 未验收。gpu 特性为独立离屏适配，依赖 rhapsody，不能当作原 toolbar 异步调度已恢复。NaN 位模式及 cosmic/etagere 逐字节 fork 等价保留边界。',
        'evidence': ['evidence/tg-text-full-2026-10-13/REPORT.md', 'evidence/small-libs-batch-02-2026-10-13/REPORT.md', 'evidence/small-libs-batch-02-2026-10-13/tg-text/BUSINESS_RULES.md'],
    },
    'asset': {
        'overview': '游戏 prefab 与 texture 配置、继承合并、扫描和归档数据合同，供资产导入与运行时选择使用。',
        'modules': 'src/inheritable.rs；src/prefab/{config,scan}.rs；src/texture/{config,desc,blob,scan}.rs。',
        'done': 'prefab 命名/继承/default 配置、texture 配置与描述、目录扫描、blob archive schema 均有手写实现；RT/audio 可选对象的深层默认和 required markers 后续恢复也已迁入，含原 native-default fixture。发行 RON/texture/prefab 回归测试已迁入。原 rkyv/RON fork、bytecheck、相对路径等直接复用。',
        'gaps': '原类型/消费者深层合同及 native 布局仍需复核；archive wire 与 Rust native ABI 是不同验收层。完整 country-core 资产 producer→pass、A/C 和真实游戏加载流程仍未接线。',
        'evidence': ['evidence/shared-foundation-2026-10-13/SUMMARY.md', 'evidence/foundation-review-fixes-2026-10-14/REPORT.md'],
        'deviation': '已保留源工作树 prefab/config.rs 当前修复，准确修改状态和哈希见 foundation-audit；不以旧 Git 快照的验证结论覆盖它。',
    },
    'asset-pipe': {
        'overview': '读取 texture 配置和图片，生成 mip/压缩块并维护磁盘内容缓存；编解码/压缩器复用原 image、ddsfile、intel_tex_2 fork。',
        'modules': 'src/texture.rs；tests/pipeline_contract.rs、allocation_review.rs；examples/gamma_runner.rs。',
        'done': '图片/数组纹理处理、gamma/mip/块尺寸、压缩调用组合、缓存 metadata 和内容指纹均已恢复或按后续开发修复实现。数组所有叶文件的缓存失效、旧 metadata 迁移、零 mip/零尺寸和对齐溢出错误处理已保留。',
        'gaps': '完整原 image/cache producer 闭包与全部异常/缓存策略等价仍待验收；新参数检查与内容缓存策略属于产品改变。真实资产目录编译→country-core runtime 消费、A/C 及完整离线工具入口未闭合。',
        'evidence': ['evidence/shared-foundation-2026-10-13/SUMMARY.md', 'evidence/foundation-review-fixes-2026-10-14/REPORT.md'],
        'deviation': '326cb236 的缓存和参数处理修复保持默认行为；历史原版等价报告只覆盖其限定原算法范围。',
    },
    'tg-iced': {
        'overview': '游戏手写的本地化对话框选项、结果和文本入口，native-gui 特性直接使用原 iced fork。',
        'modules': 'src/dialog.rs、gui_runner.rs、lib.rs；src/fixtures/dialog_cases.rs；examples/tg_iced_native_probe.rs。',
        'done': '对话框标题/选项/结果、文本本地化、真实 GUI runner 和探针均已有实现。原库控件/窗口/font shaping 直接复用，不包含自写 iced。',
        'gaps': '原 GUI 默认样式、调度和完整像素验收、游戏启动真实 consumer、A/C 未完成。窗口/字体 glue 是重建适配，不能认作原函数 ABI。',
        'evidence': ['evidence/foundation-unstarted-2026-10-13/REPORT.md', 'evidence/foundation-unstarted-2026-10-13/ui/REPORT.md'],
        'deviation': '本工程默认仅库内策略，native-gui 显式启用；迁移时仅调整 feature 默认和 workspace 配置。',
    },
    'tg-integrity-report': {
        'overview': '把包校验结果呈现为报告，并处理修复建议与单文件删除动作。',
        'modules': 'src/lib.rs、controls.rs；examples/tg_integrity_report_native_probe.rs。',
        'done': 'missing/corrupt/extra 三列表顺序、all-empty 归一、单文件删除、状态/建议生成、tg-iced 对话框联接已有恢复实现。',
        'gaps': '启动入口真实 manifest/report 生命周期、原完整 GUI 调度/样式与 shipped-process oracle、A/C 仍未验收。native-gui 为 opt-in 原库后端适配。',
        'evidence': ['evidence/foundation-unstarted-2026-10-13/REPORT.md', 'evidence/foundation-unstarted-2026-10-13/ui/REPORT.md'],
    },
    'crash-reporter-gui': {
        'overview': '游戏崩溃诊断分类、报告对话框和后台任务联接。',
        'modules': 'src/lib.rs；examples/crash_reporter_gui_native_probe.rs。',
        'done': '诊断优先级、前 8 栈帧匹配、复制提示、一次 callback/thread、Tick 完成和内嵌完整性报告均已有实现。',
        'gaps': '真实 panic 回调 payload、报告/剪贴板/浏览器动作全链、原 GUI 像素/调度与 A/C 仍需验收。native-gui 显式启用；不会因导入自动发送报告。',
        'evidence': ['evidence/foundation-unstarted-2026-10-13/REPORT.md', 'evidence/foundation-unstarted-2026-10-13/ui/REPORT.md'],
    },
    'tg-gpu-selector': {
        'overview': '游戏层 GPU 选择结果、driver 建议、graphics 错误文案及 Vulkan workaround 策略。',
        'modules': 'src/gpu_selection.rs、vulkan_config.rs、lib.rs；src/fixtures/message_cases.rs。',
        'done': '设备选择 outcomes、错误/driver 建议与 allowlist 六项 workaround 状态已恢复，包含原码消息 fixture 与 native GUI 探针。',
        'gaps': '真实 rhapsody 设备枚举/driver_support/device 创建 consumer 尚须接线；原错误标签 Rust ABI、完整 GUI 和 A/C 未恢复。通用驱动和 Vulkan 枚举算法由依赖提供。',
        'evidence': ['evidence/foundation-unstarted-2026-10-13/REPORT.md', 'evidence/foundation-closures-2026-10-13/gpu-selector/B-SURFACE.md'],
    },
    'feedback-reporter': {
        'overview': '游戏反馈事件的等待/替换、文件收集、ZIP 打包和 multipart HTTP 提交策略。',
        'modules': 'src/lib.rs（等待状态、benchmark、文件选择、HTTP、Bevy Update 插件）。',
        'done': '601 计数边界、等待中的报告替换、指定文件 ZIP、benchmark 数据、multipart/失败处理和 Bevy Update 事件联接均有恢复实现；使用原 reqwest/zip/Bevy 0.16 依赖。',
        'gaps': 'country-core/UI 的真实反馈 producer、完整原 report/ZIP/HTTP oracle、真实服务响应与 A/C 未验收。用户提供的 Bevy fork commit 未核实，不宣称上游 ECS 与原 fork 等价；未执行真实报告发送。',
        'evidence': ['evidence/foundation-unstarted-2026-10-13/REPORT.md', 'evidence/foundation-unstarted-2026-10-13/feedback-cloud/HANDOFF.md', 'evidence/foundation-closures-2026-10-13/feedback-reporter/Verification.md'],
    },
    'geforcenow': {
        'overview': '游戏调用 GeForce NOW SDK 的动态库加载和 initialize/query/shutdown/unload 生命周期。',
        'modules': 'src/lib.rs。',
        'done': '固定 DLL 路径、三个导出全部加载后 initialize(0)、成功 query 和释放顺序、错误传播均已恢复；本地 fixture 覆盖 SDK 分支，不重建 NVIDIA SDK 内部。',
        'gaps': '真实 GFN SDK/云会话运行、启动 consumer 与 A/C 未验收。测试注入的 DLL 边界及 SDK fixture 不是发行 SDK 运行结果。本包 tests/fixtures 保留 owned C 源码和 build.ps1，编译后的 DLL 为忽略的构建产物；默认套件保留 1 项 ignored，已额外在 Windows 编译本地 fixture 并执行该 native 测试的 14 个分支。',
        'evidence': ['evidence/foundation-unstarted-2026-10-13/REPORT.md', 'evidence/foundation-unstarted-2026-10-13/feedback-cloud/HANDOFF.md'],
    },
    'theme-fetcher': {
        'overview': '周期获取活动主题列表与内容包，维护 HTTP 缓存、CAS 目录和活动数据发布状态。',
        'modules': 'src/lib.rs（ThemeFetcher/ReqwestTransport）；src/checked_zip.rs（校验/解包/检查）。',
        'done': 'ETag/Date/Cache-Control、200/304、时钟调整、backoff、半开活动范围、最后匹配胜出、变化才发布、CAS/ZIP/BLAKE3 校验和清理已恢复；run 循环与注入 Transport 可独立验证。',
        'gaps': 'tiny-glade 真实主题 plugin/内容 consumer、完整原 HTTP/CAS/ZIP oracle、真实服务与 A/C 未验收；wire/schema/本地文件生命周期仍要随 consumer 接线复核。',
        'evidence': ['evidence/foundation-unstarted-2026-10-13/REPORT.md', 'evidence/foundation-closures-2026-10-13/theme-fetcher/BusinessLogic.md', 'evidence/foundation-closures-2026-10-13/theme-fetcher/Verification.md'],
    },
    'theme-server-proto': {
        'overview': '主题服务器活动列表的序列化数据合同。',
        'modules': 'src/lib.rs（ActiveEvent、EventsResponse）。',
        'done': 'ActiveEvent 的 start/end:i64、cid:String 和 EventsResponse.events 字段已按 B producer 恢复；serde 派生用于编码/解码，不算手写协议算法。',
        'gaps': '更多服务器 API、错误 response 契约、真实 consumer/服务与 A/C 未验收；这里仅声明已观察到的字段，不扩造未知接口。',
        'evidence': ['evidence/foundation-unstarted-2026-10-13/REPORT.md', 'evidence/foundation-closures-2026-10-13/theme-fetcher/B-SURFACE.md'],
    },
}

audit = json.loads((ROOT / 'provenance/foundation-audit.json').read_text(encoding='utf-8'))
for package, info in INFO.items():
    info['evidence'] = [p.replace('evidence/foundation-closures-', 'evidence/closures-').replace('evidence/shared-foundation-2026-10-13/SUMMARY.md', 'FOUNDATION_BUSINESS_COMPARISON.md') for p in info['evidence']]
    files = [f for f in audit['files'] if f['path'].startswith(f'crates/{package}/')]
    revisions = sorted({f.get('source_commit') for f in files if f.get('source_commit')})
    dirty = [f['source_path'].split(f'crates/{package}/')[-1] for f in files if f.get('source_worktree_modified')]
    source = f'reconstruction/shared-foundation/crates/{package}'
    text = f'# {package}\n\n{info["overview"]}\n\n'
    text += f'## 功能与目录\n\n{info["modules"]}\n\n'
    text += f'## 来源\n\n- 恢复源码：[shared-foundation/{package}]({RESEARCH}{source}/)，完整源路径为 `reverse-engineering/tiny-glade/{source}/`。\n'
    text += f'- 原 B 包/PDB 归属：`crates/{package}/src/`；这是行为恢复源码，不是取得作者原始 Rust 文本。\n'
    text += '- 本次选用提交：' + '、'.join(f'`{r[:8]}`' for r in revisions) + '；每文件完整提交与输入/输出 SHA256 见 [foundation-audit.json](../../provenance/foundation-audit.json)。\n'
    if dirty:
        text += '- 导入时源工作树还有修改：' + '、'.join(f'`{p}`' for p in dirty) + '；是否选用以审计记录的 selected_revision/source_sha256 为准。未选的改动单列 unselected_worktree_sha256，旧提交验证不自动覆盖新的源内容。\n'
    text += '- 逆向/验收报告：' + '；'.join(f'[{Path(p).name}]({RESEARCH}{p})' for p in info['evidence']) + '。\n\n'
    text += f'## 已完成\n\n{info["done"]}\n\n'
    text += f'## 尚差的功能与验收\n\n{info["gaps"]}\n\n'
    if info.get('deviation'):
        text += f'## 持续开发差异\n\n{info["deviation"]}\n\n'
    if package in {'tg-iced', 'tg-integrity-report', 'crash-reporter-gui', 'tg-gpu-selector'}:
        text += f'在 workspace 根执行 `cargo check -p {package} --features native-gui --all-targets` 可检查原库窗口适配。默认 feature 只编译库内业务，便于持续开发。\n'
    elif package == 'utils':
        text += '在 workspace 根执行 `cargo test -p utils --all-targets --all-features` 验证正式恢复范围；包内没有猜测游戏算法的实验 feature。\n'
    elif package == 'tg-text':
        text += '在 workspace 根执行 `cargo test -p tg-text --all-targets` 验证 CPU 路径；`cargo check -p tg-text --features gpu --all-targets` 检查离屏适配。\n'
    else:
        text += f'在 workspace 根执行 `cargo test -p {package} --all-targets`。本说明列的是实现与证据范围，当前集成验证结果以根工程报告为准。\n'
    (ROOT / 'crates' / package / 'README.md').write_text(text, encoding='utf-8')
print(f'Wrote {len(INFO)} package READMEs')
