当前仍在分批集成和验收。历史主循环、错误传播、资源退休和若干真实消费者的窄修通过限定源审查并由中央合入；**尚未验收完整游戏可启动、非空世界多帧 WSI、最终像素或性能回归**。缺文件是用户确认的待同步源，不等于对应作者未实现。本轮遵照最新指令跳过缺源，不应用编译桩、不绕过必经业务。

中央是唯一生产写入、Cargo/rustc/GPU/Git 执行者。本审查只读当前源码、冻结候选、原B证据和实际日志，并写项目内审查文件，没有并行启动构建或设备任务。后续记录覆盖同一文件时，旧记录仍保留其冻结输入与原有边界，不能把早期测试冒认为最终源码的测试。

| 范围 | 本轮结论 | 证据与界限 |
| --- | --- | --- |
| 01/02/03/04 主链七源 | passed_scoped，中央已合入 | SCOPED_REVIEW_BATCH1.json。typed error、abort、成功后推进、真实提交观察、worker 生命周期；02 空 receipt 修正另由 root 记录 delta。 |
| render_frame | 整个冻结模块 CPU 27/27 | validation/render-frame-v1.json；实际 ash0.38 缓存依赖。只验证泛型驱动与错误顺序，未验证 ProductionRenderHost/完整 App/GPU。 |
| GameplayPreparation 原取消 kernel | 真实 Bevy App/World CPU 4/4 | validation/worker-shutdown-v2.json。记录源为5ce22c7f；后续保存处置扩展改变 run_game，不把旧4项当新 guard 已过。 |
| temporal reset 退休 | passed_scoped，已合；真实 GPU 2/2 | TEMPORAL_RESET_REVIEW.md；外部 timeline gate 证明实际在飞资源保留，signal/wait 后下一次 begin_frame 回收；未测 VRAM 字节或实际 OOM。 |
| rhapsody 兼容 GPU | 四组 11+3+3+7，各自通过 | validation/rhapsody-v3.json，严格 REQUIRE_VULKAN=1；冻结二进制与日志可核，不外推游戏渲染。 |
| diagnostic ring | 单 CAS 写者/迟到拒绝静态通过；直接8/8 | LOG_RING_REVIEW.json。保留本轮真实374/1撕裂记录失败，再合独立修复。最终 rhapsody CPU379/0，见 validation/rhapsody-v5-cpu.json。 |
| 保存退出与零尺寸实际 Close | 四源 passed_scoped，中央已合 | SAVE_SHUTDOWN_REVIEW.json。先断真实 sender、join、检查原错误与 unsaved，同 World guard 后 drop；新增10测试未运行。原断连末 flush 仍 warn-only，join 成功不等于可靠落盘。 |
| no-world 内容合同桥 | 六源 passed_scoped，中央已合 | NO_WORLD_IMPLEMENTATION_REVIEW.json。重绑定恢复 strict、同槽 strict 优先，只 flag0 的 finalpost src0 与首 blur0 input；新增7测试未运行。原 API5/5只证不变 rhapsody API，不能代替新桥。 |
| Albedo 真实 producer 身份 | passed_scoped，中央已合 | ALBEDO_REVIEW.json。原B冷分支 u64 直接字节与 canonical public reexport 已核；不包含 cloth。 |
| noncloth canonical 八源 | passed_scoped，中央已确认合入 | CANONICAL_NONCLOTH_REVIEW.json。真实资源、canopy 两字段、原 Forest set36 顺序；cloth新增类型/alias/query/session/test全撤，drawable排除。尚未完整 typecheck。 |
| HZB atomic preserving | 新单源 passed_scoped | HZB_PRESERVING_REVIEW.json。原 shader push.mips>6才 atomicAdd，实际零上传 owner 及原 preserving 接线；新边界测试/GPU未运行，中央决定合入。 |
| Nani cull BDA | v2两源和修正fixture passed_scoped_static，中央决定整合 | NANI_BDA_DESIGN_REVIEW_V2.json、NANI_BDA_IMPLEMENTATION_REVIEW.json、NANI_BDA_FIXTURE_REVIEW_V2.json。10个原包/SPV已核；仅Main/Plaster/Smoke/StairsFloor真实物理读，Curved排除；同BufferHost generation/owner接既有exact-range/command寿命。fixture v1误计visibility已退回修正，v2按真实physical身份匹配，全部新测试未运行。 |
| Nani 七类真实 ECS | 仅候选限定静态通过，未合/未运行 | NANI_ECS_REVIEW.json。13/20映射未完成、维护相对 VAO 原顺序差异保留；实际 W22/W28 caller 已存在，作者已撤回“未接 caller”的误报。不混用旧 rlib 称新源码通过。 |
| SolidTexture canonical9（2026-10-20） | 四源 passed_scoped_static，未typecheck/运行 | solid-texture-20261020/CANDIDATE_REVIEW.json、CANDIDATE_DOC_DELTA_REVIEW.json。最终builder80a8…只文档纠正0x160槽；9/20、真实上游Some writer/生命周期、5新测试未运行。PERFORMANCE_INFERENCE_CORRECTION.json撤回仅凭TypeId推断必串行，实际调度与耗时待验。 |
| VAO Update44前缀（2026-10-20） | 两源v1 passed_scoped_static_with_companion | vao-set44-20261020/CANDIDATE_REVIEW_V1.json。原missing→rebuild本体/每App单注册/同Nani显式barrier；依赖canonical9同源整合，5测试未运行；PostUpdate重载下一Update消费的宿主边界保留。 |
| reverse-blur顶mip（2026-10-20） | 真实Graph红例确认；typed设计通过，尚无修复验收 | reverse-blur-top-20261020/BASELINE_REVIEW.json及DESIGN_REVIEW.json。3controls+1预期UninitializedRead红、原完整SPV真采样+PE2423B已核；typed occurrence/affected入口frontier/cache/oracle与actual descriptor layout合同，待实现及原SPV GPU。 |
| clutter真实producer合同（2026-10-20） | bounded core direction通过；集成验收门保留 | clutter-producer-20261020/CORE_DIRECTION_REVIEW.json。原B六完整ASM逐指令PE核对通过；metadata+45真实selector/visual-list非spawn；physics、原clear-copy注册、完整差分/raycast和0x3e域未验，不称实现完成。 |
| cloth | changes_required，未纳入 noncloth | CANONICAL_SCENE_REVIEW.json。真实 decorator raw Component 与新 canonical TypeId 不同；原字段映射仍调查，不能称真实 cloth 消费者闭合。 |

FINAL_PROVENANCE_AUDIT.json 仅记录其历史核对时点：当时留存日志/冻结二进制与声明哈希匹配，当时 rhapsody CPU 源指纹及用户三处 critters 与基线匹配。后续源码、构建和用户修改不能由该历史快照推断当前一致。各组可能重叠，不加总为“恢复率”或“游戏通过数”。第一次编译失败、ring 中间失败、后续有效结果均保留，不删除失败来制造进展。

普通 Steam main 当前固定 try_new(None)，未闭合。explicit 宿主继续是 Mac 的真实运行入口，但缺源待同步导致本轮没有完整源重建/运行证据。loading-only fixture、任意 World 的历史 present 成功、TG_SMOKE_FRAMES 总帧数或退出码0都不能证明 gameplay。真实后续验收必须绑定同一个实际 Gameplay App/World、ActiveSession、非空 draw lists/instances、完整图 receipt 与实际 present ticket、多帧 GPU 完成、最终图像读回和真实退出。输入、深度/readback、save/load/reload 与时间性能也均不能由上述基础库测试代替；具体计划保留在 ACCEPTANCE_PLAN.md。

本轮流程经验已落实在仓库内：先绑定原证/源哈希与候选、再审消费者 TypeId 和实际 caller，测试记录准确冻结输入；有界原证中的冷分支不能被短入口范围假装覆盖；shader 真实 fetch 不能因为后续 OpSelect 丢弃颜色就标成 inactive；compiler pending==0不能推断 ticket 成功；submission queued不能当 GPU 或 present 完成。无需写外部技能目录，也未启动额外 agent。

2026-10-20 后续阶段追加：reverse-blur 库包静态审查见 LIBRARY_CANDIDATE_REVIEW_V1.json，中央记录 `validation/reverse-blur-library-v1-cpu.json` 为384库测试+4基线+7guard；兼容CPU六组65/0在 `validation/reverse-blur-compat-v1-cpu.json`。consumer包见 CONSUMER_CANDIDATE_REVIEW_V1.json，中央当前记录 `reverse-blur-consumer-v1-cpu.json` 为387+4+7，通过正常无test-support生产build；这些均不是完整core/host/App验收。第一库后的26项GPU兼容结果在 `reverse-blur-gpu-v1.json`，不外推原SPV新fixture。原SPV fixture候选已通过 bounded static，见 GPU_FIXTURE_REVIEW_V1.json，真实执行结果另记。各计数有重叠，禁止相加当恢复率。

二进制留存更正：`validation/reverse-blur-artifact-retention-v1.json` 明记首次384+4+7三条target原二进制已被后续Cargo覆盖，仅历史日志/声明SHA保留，不能称原可执行仍在。当前387包和5条GPU兼容可执行由中央分别copy到独立路径，验收引用各自实际保留路径和SHA，不用易覆盖target路径冒充原物。

2026-10-20 原SPV fixture v2阶段追加：v1真实编译E0308（Texture应以Arc导入）保留在 `validation/reverse-blur-original-build-v1.json`，07静态审查漏检明确记录。独立v2 delta与执行provenance审查见 `reverse-blur-top-20261020/GPU_FIXTURE_REVIEW_V2.json`（SHA2f9d8aa5…）：source6d185b0c、显式Khronos validation collector覆盖Device创建前至销毁后，中央实际GPU2/2通过、0失败/忽略/skip，独立保存binary79499abd与完整日志/资产/当时库源150项指纹均匹配（另核v1两失败日志，共152项）。原B容器与SPV、N1/N3同图描述符layout和逐mip像素oracle真实执行通过，仍不覆盖country-core/ProductionRenderHost/完整App或WSI。此后文件变化需新记录，不将本快照外推当前所有源。

2026-10-20 clutter后续限定审查追加：

- `clutter-startup-20261020/CANDIDATE_REVIEW_V1.json`（SHA85672f76…）通过Startup5源静态审查，要求与canonical catalog5314749b、真实loader v2 f3c82bdb及App Cargo本地依赖同批整合。移除旧每帧加载、同World实际Startup加载成功后发布catalog，异步complete/warmup前与同步GUI/render前的真实角色/catalog门均已核。新CPU及ignored原61资产测试未执行，不称完整启动通过。
- `clutter-physics-20261020/PAIRED_IMPLEMENTATION_REVIEW_V1.json`（SHAe0fd3117…）通过world e06d与consumer7603共同静态审查。实际localpose、唯一SlotMap收据、Rapier物理handle分离、selected source equality、两个实际receipt消费者与原slot/flag分流保留；11新增测试未运行，intersection无当前业务注册，旧projectionfreehead非完整allocator/free链。
- `clutter-producer-20261020/PRODUCER_BLUEPRINT_REVIEW_V2.json`（SHA410df495…）通过producer/helper v2和Arc blueprint接口静态审查，明确root显式native-physics依赖、03physics与leaf/startup伴包。三态/同Arc snapshot/metadata45 selector/原构造及Arc分配顺序已核；0x3e有效域、5类helper/atlas配对、完整14参visual副作用和snapshot调度仍未闭。
- `clutter-producer-20261020/COLLIDER_ONLY_DESIGN_REVIEW_V1.json`（SHA5c947774…）仅为collider-only业务设计门，保持原live-map pending删除、先first后second失败/清理顺序；原PHYSICS_DEBUG_LOG_ENABLE控制owner与日志仍未恢复，不造本地false flag，不称完整helper等价。
- `solid-texture-20261020/ROOF_DOMAIN_DESIGN_REVIEW_V1.json`（SHAc03a0ff1…）通过Roof-present限定分支设计，Floor-only仍须preflight拒于key/count前；这是9完整适配加kind9的一部分，不能报告10/20完整。真实Roof writer源存在与宿主实际安装不同，当前未证明默认首帧运行该链。

以上设计与静态源审不能代替实际编译/CPU/GPU或主链路验收；中央持续集成的源变化与执行结果须绑定各自新manifest。原证与候选冻结对象不随新进度改写。

Roof设计必读增补：`solid-texture-20261020/ROOF_DOMAIN_DESIGN_ADDENDUM_V1.json`记录中央指出的独立marker+Key无Desc域。07原v1未覆盖此域，必需&Roof Query不能获候选准入；必须保原With marker并用Option<&Roof>，None显式unsupported，防maintain仅Desc检查之外静默跳过。此要求先于新候选接受，原v1不改写。Builder PartialEq目前三源/phase不需要，中央暂不合，保80a8源。

2026-10-20 后续业务阶段追加（上表和早期报告仍只代表各自冻结时点）：

- `clutter-producer-20261020/COLLIDER_ONLY_IMPLEMENTATION_REVIEW_V1.json` 与 `PRODUCER_BLUEPRINT_REVIEW_V2.json` 分别绑定 combined blueprint c86444c7、generator c0e3b92f 和 helper 0ff5c792。中央已源整合，并对 helper 做独立记录的纯文档 delta（当前586d2433）；显式 native-physics 本地依赖已在真实 metadata 中核到。实际碰撞、清理、三态失败副作用已静态核对；新测试、完整 crate typecheck、实际场景与完整视觉消费者仍未验收。两个 integration identity observation 各自保留其时点，不外推后续源。
- `solid-texture-20261020/ROOF_CANDIDATE_REVIEW_V1.json` 通过 systems dec2fd83 与 instance_updates b93bffcc 的限定静态审查，中央已源整合（`validation/nani-roof-branch-integration-v1.json`）。Option<&Roof> 覆盖无 Desc 的既有 marker+Key 域，None 明确拒绝；6 个新增测试尚未运行。仍是9类完整适配加 kind9 的 Roof-present 分支，不能报告第10类完整恢复或实际 roof event 已接入。
- `clutter-producer-20261020/TYPE_DOMAIN_REVIEW_V1.json` 以11个完整原B函数确认 typed serde 成功域为0..61和63，Clutter 子枚举成功值0..55直接复制，原父构造器不加62。当前 raw-u8 宿主把输入域扩大；对62的显式拒绝属于适配边界，不能声称原 helper 自带此 panic，也不创建第57种 mesh。
- `clutter-producer-20261020/FIVE_ATLAS_BINDING_RECHECK_V1.json` 独立复核 tag28、30..33 的原 helper、真实 builder 与 maintain 消费：原字节确实返回 `nani_solid_vertex_color`，当前 kind3 映射一致。观察到的 sibling RON 在5个名字上不匹配，但没有已证 B 发布资产配对；同目录不构成发布身份。保持原常量、严格匹配和该输入边界，不为适配 RON 复制 Atlas 项或篡改 subset。

下一阶段是完整视觉消费者：当前 collider-only 阶段不能替代原7991B、14参数的 instantiate、Window 候选、两种 wall tracker、cloth、wall holes 和实体差分副作用。`fullvisual-20261020/WINDOW_NATIVE_PREFLIGHT_V1.json` 仅验证03新 retained 16个原函数的每条字节、连续完整范围和选定 PDB 边界；尚未给新类型或调度设计通过结论。特别禁止将当前会清 authority.decorators 的同名 clear_modification_tracking 直接接入每帧末尾；06正在分开真正 tracking cleanup 与 destroy clear。各包先冻结设计再源审，中央独占整合与运行。


2026-10-20 完整视觉消费者阶段继续追加（不是收口声明；前文表格和“下一阶段”均为历史时点）：

- Window 唯一值型候选与全量收集约束已分别源审，见 `fullvisual-20261020/WINDOW_CORE_CANDIDATE_REVIEW_V1.json`、`WINDOW_FOUR_CONSUMER_DESIGN_REVIEW_V1.json` 和 `WINDOW_FOUR_CONSUMER_CANDIDATE_REVIEW_V1.json`（最终四源 manifest e42b3182）。同 Arc/值相等仍收集输出；manager 先 last-wins 再只检查实际保留目标元数据，真实 Nani/Color/texture 与同 drawable AutoClutter owner 已核。11 项实际 World/Schedule 测试尚未执行。资源注册 companion V1 正等待原 manager→animation、chain_ignore_deferred 的新原证与 root V2，不把旧相反 `.chain()` 次序准入为原调度；collector→App 完整接线未验。
- `AUTO_CLUTTER_SCHEMA_OWNER_REVIEW_V2.json` 与 `AUTO_CLUTTER_COMPONENT_CANDIDATE_REVIEW_V1.json` 允许当前架构将唯一 24B 非 owning 动画组件放在实际 drawable owner，源54703b32已由中央合入。没有碰 foreign components/mod、数值 TypeId/ABI 或默认成功状态。+15 仅 writer 已证，动画真实读+14作为 sound kind、context恒0；旧 manager-before-animation 的“建议”当时并非顺序原证，新 registration 合同须独立裁决。
- `TRACKER_CANDIDATE_AND_HARNESS_REVIEW_V1.json`（96d17838）通过三源 strict tracker/40B与32B Params/显式 HRTB 消费协议限定审查；中央实际同源 harness 8/8、0 ignored，`validation/tracker-source-slice-v1.json` 与独立留存 binary 38c9e477 对应。没有旧 core rlib、第二 registry、自动 Drop 空回调；此结果只覆盖这三完整候选与直接 canonical 定义，不代表原七/四参几何音频宿主或完整 App 已通过。
- `CLOTH_SCHEMA_DESIGN_REVIEW_V1.json`、`CLOTH_STATE_CANDIDATE_REVIEW_V1.json` 和 `BUILDER_NONE_TAGS_REVIEW_V1.json` 已允许中央源合唯一 ClothInfo/cloth_infos 与真实三处 None2 gate。`FULLVISUAL_BODY_CANDIDATE_REVIEW_V1.json`（b6ab82fe）绑定 blueprint d1e08030：原14参、先累积三个输出再 Arc/Eq、真实 Commands 差分/封存/失败与 tracker 副作用已静态核对，中央已源整合；6 项本体和5项 Eq 测试未执行。完整原 middle、正确 copy 时点和 failure-stop 仍是接线门。
- 原 phase 标签纠错见 `PHASE_MIDDLE_DORMER_LABEL_CORRECTION_V1.json`（d1ce7827）。旧06 CONTRACT09e185及07 PHASEd0fe40把中间组第四项写成 cloth upload，实际 vtable/wrapper/完整14341B目标是 Dormer upload；历史冻结对象不改写。fullvisual→中间视觉组→copy 的结构保持，不能按错误名字注册 Cloth。`DORMER_UPLOAD_DESIGN_REVIEW_V1.json` 只批准225格、每格最多49有效项、225×50记录与真实两次上传/metadata顺序的设计；随后两源实现和13新增测试须另审另执行，不能由设计推断已上传。
- `COLLISION_QUERY_CANDIDATE_REVIEW_V1.json`、`BOUNDED_RAY_CANDIDATE_REVIEW_V1.json` 和 `SIMPLE_RAY_CANDIDATE_REVIEW_V1.json` 分别绑定实际 canonical overlap 收据、finite intersects 和 finite simple ray 的独立窄增量。旧兼容入口仍显式 MAX；新入口只在真实 native-physics 域。各自5/4/5真实 Rapier 新测试尚未执行，当前 Snap/validator 实际业务调用与整 crate typecheck 未由静态审查建立。
- `VALIDATION_OUTER_INTERFACE_DESIGN_REVIEW_V1.json` 仅批准真入口 cleanup→记录门→带实际 addr/id 的 locator→首拒绝/created恢复与 unit MutePreview 的外层合同。11 reason、8143B位置数值、outline真实资源、load_decorators与音频 cursor 消费、foreign Storage destroy/restore 仍按原接口分批恢复；不把 tag11 Valid 翻成旧 bool、不以 bbox/默认资源代替。
- `CLOTH_CONSUMER_INPUT_OUTPUT_REVIEW_V2.json` 仅审真实 ClothInfo 输入、有限 ray、endpoint/garment/thread 的原 build 输出合同；缓存/候选几何/12B纹理投影/正常卸载等仍待设计。`CANONICAL_NANI_CLOTH_DESIGN_REVIEW_V1.json`（1650b172）另外批准唯一 opaque 三 word 组件与已有 NormalMap(u64) 的 builder→required ECS Query→原16word row→真实 flush 设计，禁止零 setter/默认 descriptor/foreign components/mod 修改。没有真实 garment payload writer 与场景闭包，不能称第10完整 family。
- 共享三个 math helper 已源合 feaaee38；`CLOTH_SHARED_UTILS_HARNESS_ADMISSION_V1.json` 严格绑定同源 bytes[880..7147)。中央 runtime-v1 实际7/7、0 ignored、offline locked 通过，独立复核 log01e31f6c和保留 binary ff31a28f；只覆盖这三个 helper，不代表缺源 utils 全 crate、原 assembly 或 GPU。作者消息中1512偏移已由独立观察纠正，原冻结 SOURCE_SPAN 一直为880。
- Rectangle v1 rustfmt虽过，4个 `#[test]` 却拼进 impl，静态发现的真实编译阻碍保留在 `RECTANGLE_CANDIDATE_REVIEW_V1_CHANGES_REQUIRED.json`。v2 6353c714 已把测试移入现有 cfg(test) 模块，production 两方法文本不变；`RECTANGLE_CANDIDATE_REVIEW_V2.json`（47d219df）通过限定源审，4测试未运行。不以语法解析替代类型/测试属性检查。
- `MEMBER14_MIDDLE_OWNER_GEOMETRY_CONTRACT_REVIEW_V1.json`（35bb7a21）合读 middle826443f2和必读 geometry addendum b77d3ffc：Chimney tag1原delta为−0.4，旧+1.6明确撤回；Door原.56为 depth、word6为 extra_vertical，保原max3/种子/有向距离。原publisher在 assembly 前建立本链第三/四allocation、原高亮只处理 base allocation，当前同storage三lane owner适配方向成立。clear→逐砖partial push→tracker normalize不自动容量retry，不发明UI。尚须中央授权外部owner并冻结真实 Commands/lane 初始化、实际7/4参 terrain/derived/audio宿主及设备失败策略；W21/W27 compute无raster同status gate，不能声称整帧fail-stop。

以上仍是分批设计、静态源准入及明确范围内的中央实测。所有 fullvisual/Window/Cloth/assembly 新包均不能凭基础库或同源 slice 测试外推完整当前源码 App、非空世界多帧 GPU/WSI、最终像素、退出与性能通过；后续源变化继续使用新的不可变 manifest/报告，不覆盖这些通过或退回记录。


### 2026-10-20 continuation: immutable bounded source and provenance reviews

This is the current append-only stage. Earlier “final” or “still matching” observations describe their recorded snapshot only; they are not blanket statements about the changing production tree.

- Window registration/event V2 `f9693d56…` and canonical Nani Cloth source `bef9feca…` passed scoped source review and were centrally integrated. This does not mount or accept the whole fullvisual/set36/App chain.
- Curve right-only source `b0884dc8…` passed scoped source and was centrally integrated. Harness V1 had the real included Transform missing (`4b5c276c…` changes_required); V2 added the actual canonical Transform (`2bc885de…` admission), then root's actual Cargo failed because the driver omitted the actual lerp dependency. That failure produced no test binary and is preserved. V3 adds exactly canonical lerp=0.4.0, leaves all31 selected source/fixture files unchanged, and has bounded runtime admission `f6dd3212…`; execution remains root-only and is not recorded as passed here.
- Original set25 registration recheck `095819ce…` independently byte-verifies seven complete original bodies,73 config-transfer instructions and the192B GameSystemSet vtable. Raw0x19 means decimal25;19-member means tuple arity. Fullvisual/middle/copy enter outer member14 before the whole config is assigned set25/Update. The old “outside19” sentence is superseded, preserved unchanged. Separate base raw0x1c=28 does not conflict. Fourth middle label remains actual Dormer upload, correcting the older cloth label.
- Geometry V1 was rejected for the real Chimney last-layer denominator defect (`7578442d…`):len-1 missed the final flare and the delta0 tests hid it. V2 `f223e2b5…` passes only the narrow source delta len-2 and actual nonzero-delta emitter regression; Door remains byte-identical. Five tests remain unexecuted by07; actual Sparse writer/fullhost/publication3lane/init/Commands/terrain/audio/GPU remain separate obligations.
- Bounds source `909be0b2…` and Snap source `169214a1…` pass scoped source review against their pinned actual dependencies and native design. Bounds4/Snap8 new tests remain unexecuted by07. Genuine mesh-library production data and real consumer mounting are not supplied by fixtures or default-empty libraries. Fullcalculate/corner/location/feedback remain separate gates.

All cited full reports are in `fullvisual-20261020/`; immutable JSON contains complete SHA256s and explicit input identities.07 ran no Cargo/rustc/test/GPU and changed no production, claim, or protected critters file. No new aggregate recovery count, fullgame WSI, pixels, or performance acceptance follows from these source reviews.


### 2026-10-20 later bounded handoff stage

- Curve V3 actual central run provenance accepted by07 in `CURVE_V3_ACTUAL_RUN_PROVENANCE_REVIEW_V1.json` (`6b8db77b…`):38lib+7properties+8review=53 passed,0ignored;31 source/fixture pairs and16 original-lock public identities independently checked,3 copied binaries/log retained. Actualfeatures are explicit subsets, not a globally identical union; this does not cover later WallSpace/raw-tangent changes.
- Base ownership/preassembly design `060b9efe…` allows only same WallEntities/InstancedWall three-lane/pending architecture after root delegation. Actual19→25 Commands visibility, real pending-vs-success, marker/OnExit and7/4 host/flush remain gates. Its earlier only-requested holefinalize suggestion is explicitly superseded by fresh original full-map evidence in BatchA contract8c7235a7.
- Actual App base registration BatchA source `e0c7304a…` passes scoped source only:4canonical owners+5App event lifecycles and existing real finalize/construct section at28,27added/0deleted. Original holes-only ALtrue work is preserved by the current disclosed eventadapter. No full original9tuple, set25assembly/Bthree-lane, typecheck/runtime/2session success inferred.
- Cloth two-source consumer numeric design `7945d30e…` passes bounded implementation design, not source/runtime. Original signedzero/f32/RNG/cache/event/actualfiniteRay/Wallbucket and realCommands requirements are mandatory. Upstream BoxClothInfo/generalflags/Some-style and host remain gates.
- Dormer unique allocation owner/delete design `686065fb…` and two-source candidate `b170d601…` pass their scopes. Full64WallId→actualSparse allocation and true EventReader replace invented rawcursor state;5 meaningful sameWorld/allocation/retirement tests remain unexecuted here. Actual roof tessellation/registration are separately unsupported.
- Clutter process/source/context facts `b723fe2d…` pass bounded original evidence/constraints only:actual Deleted filter, unsigned stablepriority, selected mesh identity, owned Arc slice/tag5 no-payload, Convertable-to-creation destination transport. No carrier schema,44KBpopulate approximation patch, genuine region producer/UI mounting or interactivehistory proof admitted by that fact review.
- Derived same-resource borrowed View design `f4cd5086…` and WallSpace793B raw-tangent design `e0e7e076…` pass bounded implementation design only. Explicit Legacy subtype-map backing is not full Info/loader restoration. WallSpace retains actual endpoint double normalization/negative branch and NaN behavior, preserves192 old inputs, with a documented Stationary prose typo corrected append-only. Candidate/type/runtime checks remain future separate events.

All new reviews preserve historical failed and superseded artifacts.07 changed only independent reports/index and executed no Cargo/rustc/CPU/GPU or production/claim operations. Root alone integrates and executes. Full current-source App/nonempty multiframe WSI/final pixels/performance remain unaccepted.

## 2026-10-20 追加：实际 V5、借用构造器与 Carrier schema

- 曲线 V5 实际执行 provenance 独审通过：[CURVE_V5_ACTUAL_RUN_PROVENANCE_REVIEW_V1.json](fullvisual-20261020/CURVE_V5_ACTUAL_RUN_PROVENANCE_REVIEW_V1.json)，SHA `62a0a0af08c7c1507dc2b8291fee8816f643bb4f4fe3af4a4c8f2cc4220b55de`。31 live/template/runtime 文件、三份独立保留二进制、日志与锁指纹全符；41+7+8=56 通过，0 ignored。V4 源码 test import 的实际 E0432 保留；07 漏审 export 已在 `0939502b…` 明确纠正，不将旧静态准入当编译通过。
- Derived View 源码限定通过 `bebe5b51…`，中央实际 source 整合 `84bd91ed…`；仍是唯一现有 backing 的借用适配，不代表完整 Info/loader 恢复。WallSpace import fix `0939502b…` 中央已整合，V5 覆盖其真实新三条回归。
- Cloth 两源最终 V2 静态源码通过：[CLOTH_CONSUMER_CANDIDATE_SOURCE_REVIEW_V2.json](fullvisual-20261020/CLOTH_CONSUMER_CANDIDATE_SOURCE_REVIEW_V2.json)，SHA `0f8592c295fe4cc7f12434382eb635d160ad4b1aaa0343b60a466bd17eb8c90e`。V2 test 真实断言 same-world subset position.mesh_index；生产 prefix 与 V1 相同。12 新+10 保留测试未执行，实际 Box 上游/样式与 general flags/完整 App 仍有门。
- Safe borrowed point ctor/profile 设计通过 `ac62ace5…`，候选源码通过：[BORROWED_POINT_CTOR_CANDIDATE_REVIEW_V1.json](fullvisual-20261020/BORROWED_POINT_CTOR_CANDIDATE_REVIEW_V1.json)，SHA `37734559a28ac73ab9aa6bc99cbb9ea2aca07bb2673066f1051c88ca67806ed8`。原左右扩展/量测与旧 tests 字节不变；三个新 tests 尚未执行，之前 V5 56 条不覆盖此更新。
- Carrier V3 先仅 schema/owner 设计准入：[CLUTTER_CARRIER_SCHEMA_DESIGN_REVIEW_V3.json](fullvisual-20261020/CLUTTER_CARRIER_SCHEMA_DESIGN_REVIEW_V3.json)，SHA `5d89cc518f4894fb8d66d5638e8f068689362c0f4f53cb9d9a1f6b0afe651167`。中性 source ID/真实 Arc slice/creation destination0或2/单 canonical Convertable。54-name 逆解析、实际 UI/process 消费者及44KB非空 populate 未获本审查通过；不得由 schema 挂整组36。

本段是当前阶段追加。旧文中的“收口/最终匹配”仅表示其冻结时点，不能外推到后续新源。完整 utils 缺 sampling，完整 core/App、非空多帧 WSI、最终像素和性能仍未验收。

## 2026-10-20 追加：消费者源准入、B 宿主测试边界与 Window 新反例撤回

本节为当前阶段追加；此前收口、当前指纹或测试状态均只对应各冻结时点。源码/设计准入不等于完整 App 类型检查、真实场景或 GPU/WSI 验收。

- [fullvisual-20261020/MEMBER14_HOST_CANDIDATE_SOURCE_REVIEW_B_V2.json](fullvisual-20261020/MEMBER14_HOST_CANDIDATE_SOURCE_REVIEW_B_V2.json) — `8228761d7d610277be0cd234adc9720931dae46d751293cc0339ba90a0f691b4`。六源 scoped 源通过；V1 外部 cfg(test) 设备 blocker 修正；9 新测试未运行，补充 6091468 包未审。
- [fullvisual-20261020/WINDOW_KEY_COUNTEREXAMPLE_REJECTION_V1.json](fullvisual-20261020/WINDOW_KEY_COUNTEREXAMPLE_REJECTION_V1.json) — `8b78c55a659225a9b77e2a6a699d4e6cb8c041eaf569cb86c8563f105be4a267`。拒绝后来的字段冲突反例：跨栈槽覆写误连。四完整原体 5280 指令独核；保既有 40 hash/48 outer-key 和 manager40 路由，无中性字段更名。
- [fullvisual-20261020/BORROWED_NEAREST_KERNEL_CANDIDATE_REVIEW_V1.json](fullvisual-20261020/BORROWED_NEAREST_KERNEL_CANDIDATE_REVIEW_V1.json) — `cf8a93f53866a9083fd9dbb5c6785ae8c4a49f9a12c09d7bebb14e1a813a10b0`。共享 nearest 源通过；缓存索引顺序和原 U 加法操作数保留。实际 V7 provenance 待另审。
- [fullvisual-20261020/CORE_CURVE_SAMPLING_ADAPTER_CANDIDATE_REVIEW_V1.json](fullvisual-20261020/CORE_CURVE_SAMPLING_ADAPTER_CANDIDATE_REVIEW_V1.json) — `73cc99fde37c9247a92f9f12fcbccadcfdccabcd9bec111865b8d3bc5e96985f`。单一 actual CoreCurve2 trait 源通过；3 core 测试未跑，不外推旧 guarded callers 已迁移。
- [fullvisual-20261020/DESTINATION_CODEC_LEAF_CANDIDATE_REVIEW_V1.json](fullvisual-20261020/DESTINATION_CODEC_LEAF_CANDIDATE_REVIEW_V1.json) — `38918191e32be386eb3a02fbe2787cda7220e60d75a9a5d5b71e27f7b561601a`。五 payload/Info 定义位 codec 源通过；实际512转换/parent不包含。
- [fullvisual-20261020/CLUTTER_VALUES_BALCONY_PAIRED_SOURCE_REVIEW_V2.json](fullvisual-20261020/CLUTTER_VALUES_BALCONY_PAIRED_SOURCE_REVIEW_V2.json) — `dd62d7bcc225197d8d09715a4ad23a9184fcc3161136495c33de6a53508f70d9`。唯一 drawable 值载体+实际 builder TypeId 配套源通过，消费者迁移另门。
- [fullvisual-20261020/CLUTTER_NAME_LOOKUP_CANDIDATE_REVIEW_V1.json](fullvisual-20261020/CLUTTER_NAME_LOOKUP_CANDIDATE_REVIEW_V1.json) — `f61c8ba6f48f66d5eb9eba2ed7218e959a2550e424d65fbe328c31d468ff708d`。54 原 literal/tag 纯 helper 源通过；当前 UI 旧业务未迁移。
- [fullvisual-20261020/XZ_PROJECTIVE_ADAPTER_DESIGN_REVIEW_V1.json](fullvisual-20261020/XZ_PROJECTIVE_ADAPTER_DESIGN_REVIEW_V1.json) — `795bbbe84a5917a3df74ab56802ee0acfc2891ee52a25fda0b3ee0758fbd9c31`。薄 x/z adapter 设计通过；等待中央共享 lib 新基线，非窗口闭包。
- [fullvisual-20261020/SHAPE_CLOSEST_U_DELEGATION_DESIGN_REVIEW_V1.json](fullvisual-20261020/SHAPE_CLOSEST_U_DELEGATION_DESIGN_REVIEW_V1.json) — `4199c4817323f6389092f6deb83cf335fa0235330400a220d564b085f53d5c95`。原 wrapper 委托 canonical Rectangle 设计通过；无几何大包通过。
- [fullvisual-20261020/RECTANGLE_ORDERED_OVERLAP_EDGE_WRAPPER_DESIGN_REVIEW_V1.json](fullvisual-20261020/RECTANGLE_ORDERED_OVERLAP_EDGE_WRAPPER_DESIGN_REVIEW_V1.json) — `f17bf6fe10f4430fcc30c20d55b5f9f7c6bee94e58222725d451b269f98c4f1b`。ordered vertical/5B edge wrapper 两设计通过，其余 Rectangle 不重开。
