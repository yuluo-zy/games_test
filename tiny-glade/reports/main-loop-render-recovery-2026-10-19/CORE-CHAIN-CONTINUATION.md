# 核心链路续推：实际接线、验证与剩余消费者

本轮开始于 2026-10-19，续推至 2026-10-20。基线提交为 `bd8f74a5cfb3c9cc279efb1abd27695bb1bfb8bf`。用户已确认另一机器的新模块待补交，并要求跳过这些继续核心链路。本轮没有应用缺源占位桩、删除模块声明或重造另一套 rhapsody；所有修改沿现有 App、country-core、rhapsody 接口推进。

此文依据实际源码、原版有界证据和本轮执行结果。文件数与测试数量不用于推算游戏完成百分比。原版取证主要覆盖 B；任务索引存在 C 不等于本轮完成 C 验收。

## 当前断点（2026-10-20 04:35，本机）

主链仍未完成，当前工作继续集中在真实非空世界的生产与消费。主体墙砖已挂到实际 App set28；set19的真实墙owner建立、set25中7/4参数砖宿主和set28成功后Trigger发布已接入源码。真实资产几何、Dormer两条生成分支、窗灯与玻璃消费及完整装饰调度仍在恢复。以下模块是主链必经业务，并非提前转去旁支。完整主链验收后再逐个深挖其余核心业务模块。

| 主链环节 | 当前可确认状态 | 完成前仍需验证的事实 |
|---|---|---|
| 启动与资源 | 已保留真实 App/PreStartup Atlas/Startup clutter owner，修正事件与线程生命周期 | 缺失模块补齐后完整当前源码构建，真实非空资产注册和加载完成顺序 |
| 墙与装饰生成 | 主体砖的 set28 section 已实际注册；fullvisual、窗口消费者、Dormer 上传与分配释放均已入源 | set19/25/28新宿主的当前源码类型检查与实际运行；真实mesh库与窗口/衣物/Dormer producer，原嵌套组及copy安装 |
| 逐帧渲染 | 生产入口、Nani set44、BDA owner和后处理合同已有修复；底层原 shader 专项 GPU 已跑 | 完整当前 country-core/host 编译，同一个真实 World 首帧/连续帧/编辑/重载进入所有 pass |
| submit / present | 现宿主错误传播与收尾已修复，底层图同步/寿命测试有实跑证据 | 实际窗口交换链呈现、世界画面、错误后的下一帧、第二 session 资源释放 |
| 交互与存档 | 现历史/保存链及 shutdown guard 保留；正在修真实位置和自动装饰交互值域 | 实际 UI 编辑→场景变化→保存→重启恢复，同一完整游戏入口端到端执行 |

实际可检查的最新整合检查点为 `validation/final-source-checks-v15.json`（SHA774ffb25828f0452e2ad20118855bf7954c0af4d4b033bee8fb073370c2de707）：89个整合文件的指纹、85份Rust限定语法通过，3份用户critters文件保持原指纹；这不是完整crate类型检查。该检查点包含真实墙宿主六源、共享CoreCurve2接口、借用点投影与nearest内核、typed clutter值和canonical Balcony、destination编码叶函数及名称解析。

曲线selected-source套件V7已实际通过63/63（48库测试+7properties+8review，0失败/忽略），见 `validation/curve-nearest-source-suite-v7.json`。它执行31份完全匹配的当前源/包含文件/测试，保全部旧测试；独立运行指纹复核待补。此前V4真实E0432编译失败及其test-import-only修复均保留。该套件不含CoreCurve2宿主，不替代完整utils/core/App或原程序执行。

`SYNC-RECHECK-20261020-v5.json` 记录最新实际fetch exit0，HEAD/origin/main仍为bd8f74a5。sampling、world_frame、startup/clearing、preplaced_autoclutter、history_runtime两种源形式均未出现。缺失文件中的间接注册尚未知；当前继续推进可见代码，未用旧rlib、占位桩或删声明绕过缺源。

本轮原窗灯核验确认旧实现存在真实差异：原六参读取TimeOfDay/Query/EventReader；逐现有map条目用frame_count+ordinal的fastrand1.8本地Rng；retain只按真实窗口key；亮度使用expf、当前brightness、GameTime.delta与rate3。旧cycle_t/global RNG/猜测retain/sin均有原证纠正。完整每帧→canonical组件→Nani玻璃链合同已冻结送独审，尚未写入生产，不能据合同宣称闭合。原Window候选40/48字段的新反例也经独立连续机器码审核推翻：新证据错跨栈槽覆写，已撤回，未错误改动正确源码。

## 已接入生产源码

| 真实链路 | 本轮修改 | 证据与实际验证边界 |
|---|---|---|
| OS 事件 → loading/gameplay 交接 → App.update | 保留交接前事件批次；先快照当前批次，再处理输出与捕获确认，避免清队列丢事件；零尺寸窗口保留原节流，实际 CloseRequested 可以结束循环 | `validation/core-chain-integration-v1.json`、`save-shutdown-integration-v1.json`；语法与独审通过，完整 OS/App 未运行 |
| App 所有权 → preparation/save worker → 设备清理 | 保留并 join 实际 preparation 线程；同一个 SubApp 在退出、失败及展开路径中停止保存调度、断开实际 sender、join 实际 save worker，保留首个错误并继续尝试清理 | `validation/save-shutdown-integration-v1.json`；新增 10 项相关测试尚未运行；原 disconnected 末次 flush 仍是 warn-only，不能把 join 当作磁盘持久化确认 |
| 主体墙发布 → 原set28基础砖section → 既有PostUpdate上传 | 实际App初始化四个canonical资源与五个事件生命周期，注册finalize_wall_holes→construct_wall_bricks，保原ignore_deferred；沿用既有一次SSBO上传 | `validation/base-producer-a-integration-v1.json`；仅原9成员组内已恢复section，非完整set25三lane/全App。原全hole表finalize有洞变化触发，旧“仅请求墙”建议已撤回 |
| set19真墙owner → set25两真实砖consumer → set28发布 | 沿同InstancedWall/Sparse/SSBO/tracker建立Awaiting→PendingUpload→Published状态；7/4参数宿主替代旧协议入口；失败不插默认Trigger，真几何成功才发布；同App注册19生产者与真实音效事件 | `validation/production-wall-hosts-integration-b-v2.json`；六源独审+语法通过，9新测试未运行；外crate cfg(test)设备导入问题已作test-only V2修复；未安装残缺member14 |
| 真实CoreCurve2 → 共享WallSpace/nearest | 对现有Curve2增加唯一借用trait实现，使用现有缓存和同一个已实测nearest内核，无第二曲线/Vec镜像，保原raw重复端点 | `validation/core-curve-sampling-integration-v1.json`；3项Core宿主测试未跑，旧location/calculate调用方仍需迁移 |
| typed杂物值/原名解析 → 实际交互入口 | 杂物creation destination与storage destination分开；真实Balcony六值与builder同TypeId；补全54原名映射及destination defined-byte codec | `validation/clutter-values-balcony-integration-v2.json`、`destination-codec-leaf-integration-v1.json`、`clutter-name-lookup-integration-v1.json`；测试未跑，populate/process/UI/location整条消费者仍在恢复 |
| Dormer屋顶删除 → canonical分配owner → 原waste回收 | 唯一full64位WallId到真实SparseAllocation的map，实际EventReader内remove/drop，空事件不触发ResMut更改；移除模拟游标/unsafe Send | `validation/dormer-owner-delete-integration-v1.json`；5项真实World/Schedule/Sparse测试未跑，几何生成与实际阶段注册仍缺，Drop不等于GPU完成 |
| Derived实际资源 → ID查表 → 同owner借用 | 用真实两个Res的SystemParam和借用View替代登记空壳，missing返回None/required查找panic，无Arc复制；旧bbox记录显式标为Legacy | `validation/derived-resource-view-integration-v1.json`；3新+4旧测试未跑，真实loader/4表payload与旧调用者迁移仍缺 |
| render_frame → production renderer → submit/present | 保留类型化错误；仅真实 OOM 进入原缩放回退；错误帧不提交成功状态，abort 无条件执行；观察值来自实际 receipt、呈现和资源拥有者 | `validation/core-chain-integration-v1.json`；render_frame 同源独立编译 27/27 通过，未覆盖完整生产宿主 |
| no-world → blur → final_post | 原 SPV 在 flag 选择之前已经读取输入。只对实际 no-world 条件下的最初输入标记允许未定义内容读取，保留实际读取、同步、生命周期和缓存约束；其余输入仍严格 | `validation/no-world-integration-v1.json`；既有底层接口 5/5 通过；新增 7 项上层桥接测试及实际 no-world GPU 画面未运行 |
| World 权威资源 → 非 cloth 绘制/水面 → Forest 动画 | 非 cloth 消费者读取实际 Weather/MainCamera/ActiveSession；共享 canopy 的 clamp/smoothstep；Forest 按原 set36 接入；不使用同名镜像资源 | `validation/noncloth-integration-v1.json`；独立原证/源码审查通过，完整场景、编辑与重载未运行 |
| decorator → canonical AlbedoMap → drawable | 移除 producer 的私有原始字节替身，实际构造插入 canonical AlbedoMap，保留公开导入路径 | `validation/albedo-producer-integration.json`；B 原指令/TypeId 静态核对，完整游戏未运行 |
| HZB → 原子计数器 → 多工作组完成 | 仅实际 push.mips > 6 的原子 read-modify-write 分支声明保留内容；沿用原访问码、dispatch、shader 和真实初始 owner | `validation/hzb-integration-v1.json`；原 SPV CFG 独审通过；新增边界测试与 HZB GPU/readback 未运行 |
| Nani W22/W28 → 真实 cull positions → 同帧图/command owner | 真实 generation-checked owner 与 active_count × 16 范围，经既有 device_address_reads 接入同 session 的读取、同步与拥有者链；只处理原 shader 实际读取的 Main/Plaster/Smoke/StairsFloor | `validation/nani-bda-integration-v2.json`；两生产源静态独审通过，新增限定测试未运行；CurvedHalfTimber 不添虚构读取 |
| VAO → Update44 → Nani 九类 canonical ECS producer | 恢复 build_missing → rebuild → maintain → 唯一 ApplyDeferred → 九个实际 update → dirty flush；补 SolidAlbedoMap/Emissive 与实际 builder 的 checked 单字节 selector；不改变其余资产阶段 | `validation/vao-prefix-integration-v1.json`、`nani-nine-integration-v2.json`；源已合入，新增上层测试未运行；其余 11 种实际 Nani 类型仍明确报未支持 |
| 蓝图窗口输出 → canonical Window 值/共享 Vec 合同 | 新增六分支值型、完整 WallId/hash 与保位 shutter 字段，按 ordered float 比较旋转/位置/缩放；不建立 OnWindowInput 镜像 | `validation/window-core-integration-v1.json`；schema与真实drain/manager/animation及必要App event companion已入源；原窗口生成、完整App阶段安装和11项新消费测试仍未完成，不以空Vec冒称场景 |
| Nani Roof-bearing 输入 → 原 marker 查询 → payload 更新 | 复用实际 Roof/Transform/动画通道与共享 Circle2d tangent，移除零返回算法；保留 With<StoneFloorMarker> 和 Option<Roof> 原查询域，缺 Roof 的不支持输入明确失败 | `validation/nani-roof-branch-integration-v1.json`；仅 Roof 存在子域已合入，6 项新测试未运行；不是完整第十类恢复，Floor-only 合同及真实业务注册仍缺 |
| 真实 clutter 资产 → 同 World MeshWatch/mesh library → Startup | 原 55+6 加载、metadata selector、demo 分支与 watch-before-add 统一接到 gameplay App 的单次 Startup，移除每帧 loader；实际 catalog 出现之前不读完成事件、不 warmup/render；同步更新 panic 经错误清理路径返回 | `validation/clutter-startup-integration-v1.json`；源已合入，上层新测试和真实 61 资产测试均未运行。`clutter-startup-lock-integration-v1.json` 实际 offline metadata 通过，锁文件仅增加现有本地依赖边，外部版本未变；这不是 typecheck |
| clutter 几何 → canonical 碰撞 owner → 源身份消费者 | 恢复 primitive 本地位姿及两个原适配器不同乘序；复用唯一实际 SlotMap，严格校验 packed generation，区分 Rapier 局部 handle；同位置但几何/来源变化仍更新；相交路由的两个消费者均解析同一实际 receipt | `validation/clutter-physics-integration-v1.json`；两源共同合入，11 项新测试未运行，未验证真实 Edit/Reload；未增加相交系统的生产注册或另写碰撞算法 |
| clutter 生成 → 真实 visual 列表/双 collider → 当前与上一帧蓝图 | 真实 metadata selector、尺寸与局部位姿进入 canonical builder/几何，保留原值构造与 Arc 分配顺序；恢复三态 mesh 槽、共享 Arc 快照及独立双 map 的 collider-only 消费、替换/删除；不再创建后丢弃 builder | `validation/clutter-producer-integration-v2.json`；三源与真实 native 物理依赖同批接入，新测试未运行；完整 14 参数视觉实例化及真实调度仍未闭合，不等同生成了游戏实体或画面 |
| 当前蓝图 → 完整 fullvisual 本体 → persistent/tracker/逐帧副输出 | 恢复原 14 个真实 SystemParam、同 canonical owner、先 collider/逐帧 holes-window-cloth 再 Arc/Eq、实际 build/seal 与 outline 失败顺序、旧新 tracker 和删除；仅 Eq 的 payload 比较按原短路顺序执行 | `validation/fullvisual-body-integration-v1.json`；本体与 6 新测试已入源，测试未运行；原 middle 消费者与 copy 的真实安装仍未完成，不能直接把旧协议/空实现挂上冒称闭合 |
| Params → 按墙差分日志 → 明确消费 | 原 40/32B 参数 ordered Eq、严格增删改状态机、dirty-wall drain；移除全世界 clone/假 tick，Removed 必须显式消费，Changed 不误发删除 | `validation/tracker-integration-v1.json`；实际三完整候选与 canonical 定义源切片的 Rust/HRTB 编译和 8/8 测试通过，0 ignored；不是完整 crate/App 或真实砖几何/音效测试 |
| 装饰位置 → 实际碰撞 query → 真实阻挡 source | 原固定 slot2（17 是切片长度）、同墙/当前 DecoratorId 过滤、严格 registry generation，返回同 owner 借用；另恢复原 max_toi 有界遮挡，保旧无限入口 | `validation/collision-query-integration-v1.json`、`bounded-ray-integration-v1.json`、`simple-ray-integration-v1.json`；三批实际查询已合入，共14个新增真实Rapier测试未运行，实际validator/cloth/Snap调度未安装 |
| 窗口 mesh metadata → 唯一 AutoClutter 组件合同 | 在当前 drawable 组织内加入单一真实实例动画值，完整保留 key/timer/progress/target_scale/mesh group/未命名 flag；不增加 Default 或根组件镜像 | `validation/auto-clutter-component-integration-v1.json`；共享类型与四个实际消费系统均已独审入源，见 `window-consumers-integration-v2.json`；11项消费测试及真实可见实例仍未验证 |
| Cloth builder 实际载荷 → canonical Nani query → 原数值 updater | 唯一drawable ClothlineCloth三字载荷、已有NormalMap、显式builder参数、实际texture owner及同phase44更新；原build_cloth数值与此前34个函数体保持不变 | `validation/nani-cloth-canonical-integration-v1.json`；三源独审合入，7项新测试未运行；实际衣物生成/CacheClothes、完整App与画面仍未完成 |
| reverse blur 顶层真实读取 → 同 mip 写入 → 图编译 → descriptor | 在原 native 0x37 升级前保留每个读取的内容合同；受影响资源按 pass 入口检查所有重叠别名，避免本 pass 的写入假装已提供旧内容；上层只给原首个 reverse input 许可，每个实际 image view 使用当前 pass 编译出的布局 | 当前 lib 387/387、真实图 4/4+边界 7/7；原始 shader 本机 GPU 两例通过全 mip 像素与显式 Vulkan 错误断言。见 `validation/reverse-blur-original-v2-gpu.json`；此入口运行现 rhapsody，host/core 新代码仍未完整编译执行 |
| temporal reset → pending command → retirement | 清除 named 强拥有者并释放可退休资源；在途实际物理资源仍由 pending submission 保留到完成 | `validation/temporal-reset-integration.json`、`temporal-reset-v4.json`；实际 GPU 2/2 通过，含 timeline gate、prepared 拒绝与多轮旧 owner 回收；未测峰值显存 |
| 并发诊断记录 → 整条日志可见性 | 修复回卷 writer 同槽交错写入；每槽 CAS 独占，繁忙/过期记录立即丢弃并计数，不自旋、不等待 | `validation/log-ring-integration.json`；保留此前真实失败日志，8/8 专项及最新完整 rhapsody lib 379/379 通过；属于当前工程修复，无原同名模块等价声明 |

每批生产整合均绑定候选、独立审核与前后 SHA。各阶段检查保留于 `validation/final-source-checks-v2.json` 至 v9；v9 覆盖完整 fullvisual 源本体、严格 tracker、Window/Cloth/AutoClutter 共享数据、两真实碰撞查询及此前整合，63 个集成文件的指纹、59 个 Rust 文件的语法和限定 diff 检查通过，用户已有 3 个 critters 文件保持同步基线。v5 检查器误用 edition2021，造成 6 个现有 let-chain 文件解析失败；失败报告保留，v6 读取实际 workspace edition2024 重验通过，未为检查器改动业务源码。后续变更需要新快照。语法解析不检查类型、借用或运行行为。

## 已执行的真实验证

- 新增严格 tracker 协议：实际包含三份完整候选源码与五个当前 canonical 定义原文切片，Rust/HRTB 编译及 8/8 测试通过、0 ignored；实际 Bevy World/Schedule 覆盖两个 ResMut adapter、跨墙迁移、Changed/Removed 区分与 strict failure。完整候选已与生产逐字一致。见 `validation/tracker-source-slice-v1.json`、`tracker-source-slice-v1-artifacts.json`；独立保留二进制 SHA38c9e477。依赖先从当前 lock 原物解析裁剪，公共身份100/100匹配；冻结模板恢复保留，实际运行在独立目录。此执行先于独立源码审完结，随后 reviewer96d17838核同一未变输入通过；不把此结果称完整 core/App、原砖几何、音效或 GPU 验收。

- 当前 reverse blur 库与布局查询修复后：rhapsody lib 387/387、真实图合同 4/4、边界 7/7；实际 Cargo、源码全量指纹和独立复制的执行二进制见 `validation/reverse-blur-consumer-v1-cpu.json`。正常生产 lib 也已实际编译通过，features 仅 default/suballocation，没有依赖测试自引用开启的 test-support，见 `reverse-blur-production-build-v1.json`。此前两个真实失败基线分别保留于 `reverse-blur-baseline-v2.json`、`reverse-blur-baseline-guards-v1.json`，没有用替换断言让测试变绿。
- 图内容合同修复后的相关兼容测试 65/65，覆盖资源合同、未定义/非活跃内容、子图和独立检查器，见 `validation/reverse-blur-compat-v1-cpu.json`。后续新增只读布局查询不由这批旧二进制覆盖，当前 lib 中的三项精确范围查询测试另已通过。
- 图内容合同修复后已重新实跑本机 Vulkan：rg_fixes_vulkan 11、graph_cache_eviction 3、frame_imports 3、transient_aliasing 7、temporal reset 2，共 26 项通过，REQUIRE_VULKAN=1 且未跳过。见 `validation/reverse-blur-gpu-v1.json`。这次执行早于后续新增只读布局查询；它验证图同步/缓存/寿命回归，不验证新 descriptor 查询的实际 GPU 消费，也不是世界渲染或 WSI 游戏入口验收。
- 当前精确布局 API 的 GPU 消费另已通过：使用原始 B 5657 字节容器和其中未经重编的 SPV，执行 N=1 与 N=3 reverse 链，full 是新建资源，没有清零或上传；全部 mip 逐像素回读符合原 false=tail、true=0.7×上一层+0.3×tail 的灰常量预期，量化容差 0.035。真实 descriptor 使用查询布局并由 bind_pass 校验；设备创建前安装 ValidationMessenger，释放真实 owner/设备后显式断言无 Vulkan 错误。2/2、未跳过，见 `validation/reverse-blur-original-v2-gpu.json`。这是现 rhapsody 的原 shader 路径，不是上层 ProductionRenderHost 或完整游戏的测试。v1 测试 adapter 的真实 E0308 首次失败保留于 `reverse-blur-original-build-v1.json`，v2 只修正 Arc owner 并加强验证收集。
- 当前 render_frame 同源模块：27/27，通过实际宿主协议的错误、回退和调用顺序检查，不替代 ProductionRenderer。
- 早先 preparation cancellation 四项只验证旧 run_game 的限定源码片段。后续加入存档 guard 后，旧四项不能证明新收尾代码已通过。
- 完整 App 构建已实际遇到 `utils::sampling` 缺源 E0583；本轮按用户要求跳过待补交模块，不继续重复相同失败，也不借用旧 country-core 二进制假装当前上层已通过。

## 本轮新增的 Nani BDA 消费者接线

W22/W28 的生成调用已经存在，使用真实 active instance count、GPU draw count 和现有间接提交，不能重复添加调用。

新定位的是实际 cull positions 缓冲的 BDA 读取：现代码只捕获数值地址；GpuWorld 的现有 import 只覆盖另一块实例数据，临时 BufferHost lookup 克隆也不进入 callback。需要把真实缓冲拥有者及 `active_count * 16` 范围经既有 `RecordedComputePass.device_address_reads` 接入同帧 session，才能让实际提交、后续 Edit 写入等待和旧 owner 回收看到该读取。

此处是已确认的声明缺口，尚无实测崩溃结论。原 SPV 核对还纠正了过宽修复范围：CurvedHalfTimber 虽携带数字 positions 指针，实际 shader 不解引用该指针；不得给它臆造读取。仅原 Main、Plaster、Smoke、StairsFloor 的实际 load 路径纳入修复。实现与独审交付在现有两个 render-nani-main claim 的 `nani-cull-bda-20261020/` 子目录，生产 v2 两源已整合；原 count/push/dispatch/上传算法均未改变。

已有实际 Main/None/单实例 GPU fixture 也已补入图断言，通过同一 session 的真实物理 owner 去重句柄，核对该 cull pass 的 0x1a/group1/0..16 读取。单独 import 不添加 pass access，不能伪造缺失读取使断言通过；原 visibility 等 descriptor 读取保持。第一版测试错误地把所有 0x1a/group1 读取合计为一条，独审发现其包含真实 visibility 读取，已保留失败候选并修为精确 physical resource 筛选。见 `validation/nani-bda-fixture-integration-v2.json`。新增 core 错误顺序测试、scene 真实 owner/同槽 generation 重用测试、上述 fixture 断言均尚未执行；custom/HZB、Edit/Reload 和实际在途命令寿命仍待验证。

## 保留的实现与验收缺口

1. **Nani set44 完整 ECS producer**：当前已有11类canonical ECS映射（含Cloth及Roof-bearing的RoofFloor子域）；当前源已有20类row-view/数值更新入口，其余9类缺的是canonical ECS绑定与真实producer闭包，RoofFloor的Floor-only分支另有缺口。不能把这些直接视为9个未知算法。已映射类型仍需当前完整App实际startup/update/GPU验证；不能用空注册、容量当实例数或镜像资源补齐。真实 RoofFloor producer 已存在，但当前可见宿主没有注册它；已恢复的 Roof-present 子域可进入实际更新，Floor-only 域仍明确失败。待补交上层模块的注册内容仍未知，不将当前无 caller 断言成用户未实现。
2. **Cloth 完整 producer**：原 Wall payload 为 builder 的连续 20 字节，旧“两段 u64”解读已纠正；现 setter 值流及 producer TypeId 尚未闭合。CPU ClothInfo 的四值合同和同一 VisualState.cloth_infos 已独审合入，4项测试未运行。canonical三字ClothlineCloth、现NormalMap、实际builder与Nani query也已合入，7项测试未运行；真实生成consumer两源也已独审入源（`validation/cloth-consumer-integration-v2.json`），12新+10旧测试未跑；窗口Box<ClothInfo> producer与真实App安装仍缺。20B WallFlag与普通flags另列，不将non-cloth的通过扩展到cloth。
3. **完整世界与呈现验证**：真实 startup/Update 后非空场景、首帧/连续帧、编辑、第二 session/reload、窗口交换链呈现及像素结果尚未执行。Surface 测试支持代码只是未运行候选，仍缺真实上游闭包。
4. **后处理整帧闭包**：reverse blur 的完整原 SPV 已证实先无条件取样、后按 sample_self 选择；顶层新 full mip 的读写别名是真实路径。库内容合同、精确 view 布局 API 与原 SPV 的 GPU/readback 已通过当前有界验证，上层精确条件和实际 descriptor 回填源已合入；完整上层编译与整帧场景仍待验证，不把所有读取放宽。
5. **真实性能**：本轮生产修复没有修改原 dispatch/count 算法，没有新增 CPU readback 或 wait-idle；数值回读仅用于测试。只读审查确认正常 reverse blur 的精确布局查询只扫描当前 pass，单 color/mip/layer 范围不会触发新增 SmallVec 堆分配；通用碎片范围仍可能分配，不能称全 API 零分配。审查记录在 Nani work 的 `performance-review/reverse-blur-layout-20261020/`。尚无完整游戏帧耗时、峰值显存或持续运行性能测量，不能据此声称性能达标。
6. **待同步模块**：源文件待用户补交的事实仍保留，但当前执行不以它们为理由停止已有消费者分析；也不把它们直接判成“未实现业务”。

SolidTexture 的两路实际是 set44 tuple8/9 的 SolidAlbedoMap(kind15) 与 SolidAlbedoMapEmissive(kind16)，不是 window。两者仅 With<InstanceDataMarker<T>>，无 Changed/Added 过滤；读取实际实例键、Transform 与 NaniSolidTexture。checked opaque canonical 类型、两条真实 builder 分支与九类 maintain/barrier/update/flush 已统一合入。原作者声明是 struct 还是 enum、变体名等仍未知；未给它添加臆造枚举名或 Default。原型宽度、单字节 clone 和原 9 项表的证据保留于 `reverse-engineering/tiny-glade/skeleton/task-index/work/TG-bdac7a16d3c26c954b/nani-gpu-counts-20261019/nani-maintain-20261020/`。

续查时原有 clutter 生成器丢弃 builder，blueprint 缺对应列表，假 loader 又按每帧运行。现已把原版 metadata+0x45 → builder+0x150 selector、真实 55+6 loader、canonical 资源与 Startup、真实 builder 列表/双几何生成及 collider-only 阶段接入源码，尚未通过完整上层编译或真实资产执行。不能把 catalog 或蓝图存在当作游戏实例、加载完成事件或呈现。碰撞依赖暴露的 cuboid 本地位姿丢失、同位置 mesh 更新漏检及 registry/Rapier 身份混用沿原 canonical owner 两源共同修复，11 新测试尚未运行；没有采用新注册表或单子 Compound。另两项原 Startup 订阅、完整生成/快照/消费调度及 14 参数 visual consumer 仍未闭合。初始 Atlas 入口经独审已确认存在：PreStartup 读取同安装根的 nani_meshes.ron，注册到同一 MeshAtlasLibrary/AssetMeshLibrary，经 startup_nani 实际上传，再进入 Update44；不能把 CPU clutter catalog 另造一条 Atlas 桥。旧三标量 cuboid helper 的另外五个调用仍返回 None，是明确业务缺口；启用真实依赖后只更正过时注释，没有假设未知位姿后冒称这些分支恢复。0x3e 输入域已用完整原构造/serde 证据核清：有效 typed 构造覆盖 0..61 与 63，62 不在已证明有效域内；现 raw-u8 拒绝仅是宿主边界，不能宣称原版非法输入也 panic。此处只纠正文档，未改算法。诊断日志 flag owner 仍未闭合。

## 当前完整视觉链的原版约束与实际缺口

完整 `instantiate_blueprints` 已按7991字节原函数独审合入（`validation/fullvisual-body-integration-v1.json`），实际注册仍未闭合。它先执行真实 collider helper，再对每个当前蓝图贡献 WallHoles、WindowAutoClutterCandidates 与 ClothInfo；同一个 Arc 或不同 Arc 但值相等时也贡献这些逐帧输出，之后才跳过实体重建。变更与删除必须经过现有 persistent owner 和两个真实 tracker，不能每帧全量 despawn/spawn。Window值型合同、实际collector/drain/manager/animation、ClothInfo和两tracker与混合Eq均已独审入源；只有tracker有界8测试已执行。真实窗口producer和完整阶段安装仍未完成，不以空列表或字节比较代替。

原总调度已定位两个不同碰撞时点：set25 外层 member7 的 collider-only，以及 member14 内 fullvisual 再次执行的 helper；中间有 flush、validate 和 archive 等业务。member14 第二支为 `fullvisual → (dormer config / chimney / doorstairs / Dormer upload) → copy_prev` 的真实 typed chain；不能凭概念顺序接成一条简化链。真正修改标记清理在 member9 的 validate 入口，保留 decorators authority。当前同名简化 helper 还被 restore 调用，后者需要另一个原 destructive clear 合同；仅重写 helper 会破坏恢复路径。上述是已定位原证，尚未声称所有对应源已实现或调度已接入。完整视觉所需的 canonical wall/dormer 列表和准确混合 Eq 投影已合入，见 `validation/fullvisual-eq-integration-v1.json`；builder 的三个原 None 判别门也已沿完整 build 原指令修复并合入，见 `validation/builder-none-tags-integration-v1.json`；2 个新增实际 build 测试未运行。完整本体和两个严格 tracker 已经独审入源，tracker 的有界同源编译与8tests已通过；完整本体6tests未跑，原 middle/copy真实阶段尚未安装。

Atlas 调查同时保留五个观察到的 helper/RON 输入反例：原 helper 的字符串、builder kind 和严格查找均有直接原指令支持，不为匹配相邻 RON 修改原算法。该 RON 只有 sibling 路径与哈希身份，尚未证明绑定 B 发布包。Live-link 原插件在 false 时不注册，true 时要求 `reload_prefabs → reload_mesh_atlases → reload_nani → hot_reload_prefabs` 四段实际 Update 链，三段接线不能称完整热重载。调查与独审分别见 atlas-investigation/REPORT.md 和 `independent-validation/clutter-startup-20261020/ATLAS_INVESTIGATION_REVIEW_V1.json`。

位置验证链另发现原 `0xb` 才是 Valid；当前 terrain/stair 分支反转、墙/屋顶恒 true，以及 collision 扫 mask 返回 bool 的简化均不能直接用于真实 validate。完整原 query 实际限定 slot2、同 Wall/current DecoratorId 过滤并返回真实 source 原因。query 与有限 max_toi 两批真实源码已合入；完整location的11种拒绝原因与真实mesh资产producer仍在恢复。RoofBounds和Snap五状态叶函数已按原1561B/3277B独审合入，分别4/8新测试未跑；当前calculate/merge/corner旧简化仍需同消费者重构，未注册新validator。

2026-10-19 18:06 UTC 再次实际 fetch（沙箱网络拒绝后按已授权网络重试 exit0），远端仍为 bd8f74a5；sampling.rs、world_frame.rs 等待补交源仍未出现。已有源码的独立恢复继续推进，完整 App 的编译和场景验收边界保持不变。

## 原证完整性纠错

续查 SolidTexture producer 时重新核对 procedure index：B 的 persistent/transient VisualEntityBuilder::build 均为 5949 字节。先前某份新 native pack 只提取 3281 字节，不能称完整函数；原提取物保留并补记截断。已合 Albedo 结论不依赖该截断包覆盖尾部：独审另用保留完整 asm 与 RNE 直接字节核对 u64 写值，因此局部结论保持；cloth 等完整 producer 合同仍不能据局部包通过。此修正是证据范围纠错，不计新增恢复业务。

## 本轮保留的恢复经验

- 从真实 producer 和 shader load 向图声明追踪。payload 中出现指针不证明实际读取，已有调用不证明资源寿命已登记。
- canonical Rust 类型身份比同名字节布局更重要。producer 与 consumer 的 TypeId 必须一致，镜像资源会绕过真实业务状态。
- 资源 retirement 用实际 command completion 和强/弱 owner 观察验证；线程 join 与文件持久化另行判断。
- 源码审查、同源限定编译、底层 GPU 回归、完整游戏验证分别记录。候选输入变动后旧验证的适用范围必须缩小或作废。
- 中央 agent 独占生产整合和构建；协作者在合法 claim 内冻结候选并附 before/after SHA，由独立 reviewer 查原证。纠错记录保留，工具连接与报告一致性不算恢复进度。
- Cargo 会覆盖同名 target 测试可执行文件。当前 CPU 和已运行 GPU/兼容测试额外复制到独立版本路径，再绑定哈希；第一库包三个 CPU 二进制未及时复制而已被后续重建覆盖，其历史日志与源码指纹保留，但不再称旧二进制可复核。见 `validation/reverse-blur-artifact-retention-v1.json`，当前 387 项执行另有完整独立副本。
- 原注册时机和真实调用者必须一起核对。名字为 loader 不证明它在启动期执行，简化 configure 中存在该项也不证明生产 App 调用了 configure；把真实 I/O 替换进每帧占位系统会引入新错误。现已沿原插件完整注册函数证明 clutter 属于 Startup，并核出同一 MeshWatch 的 watch-before-add。
- 第三方依赖可直接复用，但必须核它的输入合同。SlotMap 的 `KeyData::from_ffi` 会规范化 generation，不能直接用它宣称已实现原版精确 generation 检查；注册 key 与 Rapier 局部 collider handle 也不是同一身份。修复权威 owner 时要追完现有消费者，不能只改插入端。


### 共享算法与中间链纠错（2026-10-20 02:54）

共享 `random_splits`、`random_splits_into`、`xz_rotation_bad` 已按原机器码合入现有 utils。实际同源有界测试7/7通过、0忽略，fastrand/glam等4个公共依赖身份与原工作区锁对齐，真实测试二进制已留存；见 `validation/cloth-utils-source-slice-v1.json`。这不覆盖缺 sampling 的完整 utils 或 cloth 游戏链。位置所需 Rectangle 两个原版方法 V2 已通过独审并合入；4项新增测试尚未执行，V1测试属性位置错误的拒收记录保留。

原 member14 中间组第4项过去标为 cloth upload 的结论已纠正为 **Dormer upload**。07 独立核对 typed tuple、vtable 与完整原上传函数，冻结纠错 `PHASE_MIDDLE_DORMER_LABEL_CORRECTION_V1.json`（d1ce7827…）；旧证据保留。Cloth 不能冒用该调度槽。

Window四源已与资源和实际App sound事件companion V2同批独审合入。原注册manager→animation的ignore_deferred已修正到现configure；旧V1反序建议保留为拒收记录。完整configure未安装于实际App，原collector上游和全阶段接线仍未闭合。

最新 fetch 后 HEAD/origin/main 仍为 bd8f74a5，sampling.rs/world_frame.rs 仍未补交；见 `SYNC-RECHECK-20261020-v3.json`。不使用桩、删模块或旧 core 编译产物掩盖这些缺口。v9 后已有 utils/Rectangle 新源变更，需下一次源快照覆盖；v9不代表当前全部源。


最新限定源码快照 `validation/final-source-checks-v10.json` 覆盖65个已整合文件，61个Rust文件的edition2024语法与全部预期指纹通过，限定diff检查通过，3个用户critters文件保持原指纹。它增加共享数学、Rectangle V2与有限最近射线，不是完整crate类型检查。有限最近射线的5项真实Rapier测试仍未运行，见 `validation/simple-ray-integration-v1.json`；此前碰撞查询14项新增测试总计均不能作为已通过。

原跟墙移动函数新增定位出真实9个系统参数：PublicWalls、四事件reader、两tracker、DecoratorStorage和DerivedDecoratorInfoResources。当前单wall helper返回集合不能代替原对两个实际tracker的脏墙传播。该缺口已单独领取原文件组并恢复；不会向fullvisual猜补第15个参数或另建同步事件。


### Dormer 实际上传本体已入源

`validation/dormer-upload-integration-v1.json` 绑定两源54668a05候选、12ac7661独审和前后指纹：真正canonical48B蓝图列表进入15×15网格，每格50行、最多接受49条的原容量规则；依原optional buffer与错误顺序写900B计数及540000B记录，成功后分别发布真实InstanceBuffer计数。同App DeviceResource和原SSBO库直接转发，不另造BufferHost。13项新增CPU测试尚未执行；7个原record写入caller、嵌套屋顶geometry/DormerTiles/删除及原阶段安装仍在恢复，所以不以空列表或CPU fixture称实际游戏成功。

Window四源独审通过后，root companion V2在现configure中仅纠正manager→animation的ignore_deferred边界、reset和必需同owner资源；实际App另以add_event登记canonicalSound事件生命周期。此6源批次已通过独审并合入，见 `validation/window-consumers-integration-v2.json`；仍不能把未安装的完整原clutter调度称完成。原set36与fullvisual set25的全局先后已直接核原enum递增表；当前App同映射成立。


最终本阶段源检查见 `validation/final-source-checks-v11.json`：Window六源与Dormer两源纳入后，71个已整合文件的预期指纹、67个Rust文件的edition2024语法和限定diff均通过，受保护的3个critters源码保持原样。此检查不具备类型检查或真实游戏运行含义；源文件数不用于计算恢复进度。


Dormer后续原证进一步划清两支：48B洞记录由7个真实visual producer写入蓝图，经fullvisual逐帧副输出/实际上传消费；原嵌套屋顶生成系统并不读取该48B列表，只检查蓝图包含DecoratorId，再从实际Storage、CachedTransforms及Roof查询生成RoofTile分配。不能把前一支接通当后一支几何完成，也不能让当前8参stub写空BP覆盖本来的只读输入。原10参消费者和gothic真实数学依赖正在同合法文件组恢复。


`validation/final-source-checks-v12.json` 是当前阶段最新源快照，纳入canonical Cloth三源。实际重新读取71文件指纹并检查diff，只对变更的3份Rust执行语法解析，另外64份Rust复用v11相同SHA与edition2024的结果；均通过。这样避免为未变化的依赖反复检查，又保留可核验输入链。仍未执行完整core/App构建或新增7项Cloth测试。

当前新增业务审查还证实：populate旧实现每格自造kind、恒定mesh_type与shared=0，process又将placement key/mesh引用字段误当XZ并按错误字段过滤排序。两者原逻辑必须同闭包恢复；仅向旧近似算法补mesh名字不能成为可接受修复。源码writer、排序/filter和真实删除记录均已分配给同一负责人取证。


### 曲线数值与当前主体砖接线（2026-10-20 03:50）

原右裁剪921B的f32表达式顺序已恢复，保留端点先修改、后续失败不回滚等副作用。实际完整selected-source套件53/53通过、0忽略，覆盖31份逐字源码/include/test、原260fixture、既有1920/320输入及两integration文件；原fixture/指令lift不是本轮直接执行原进程。真实Cargo日志、16/16公共依赖身份和独立复制的3个二进制见 `validation/curve-right-source-suite-v3.json`，独审 `CURVE_V3_ACTUAL_RUN_PROVENANCE_REVIEW_V1.json`。V1缺实际Transform、V2测试driver漏原lerp导致的失败均保留；V3仅补齐真实依赖，没有修改业务以适应测试。

随后沿原793B恢复WallSpace负u外推符号、真实端段及两次f32 normalize，保留退化NaN；actual Curve2 raw tangent仅必要三行适配，同192输入/双侧/1e-5容差保留。三源已独审合入，V4以同完整selected-source套件复验，执行结果另记，不沿用V3旧53结论。Root也保留了当前原几何与prepared curve的字段区分：真实位置使用原始曲线，anchor profile使用已核publisher写入的outline缓存，不能按同名helper替代数据来源。

主体砖Batch A仅是交付批次名，原证版本是B。set28晚于装饰set25，装饰三lane分配须由此前set19真实发布路径建立。不能把主砖构造提前来绕开owner生命周期，也不能为尚未恢复的geometry安装空系统。原register中0x19代表十六进制25，外层19代表tuple arity；0x24为十进制36，两者已分开核验。原holes全map finalization必须保留，它会在只有洞变化时触发重建；第二次原finalize针对临时corner clone，并非冗余扫描。


V4实际重编结果已出现：exit101/E0432，新增WallSpace测试错误从curve根导入CurveUBoundless，而实际类型在curve::curve_u。没有生成测试binary、没有测试执行或通过；见 `validation/curve-wallspace-source-suite-v4-red.json`。原V4/源码候选V1失败记录保留，01另冻V2仅修实际use路径，由07独审后再建新运行版本。未改driver、全局reexport或删除输入来变绿。


V5只改实际测试导入后，完整同一selected-source套件实际56/56通过（lib41+properties7+review8），0失败/忽略；所有原输入、include与两integration文件均执行，3个新WallSpace回归也在内。16/16公共依赖与锁相符，所选公共resolve features与V3一致，仍与完整workspace的全局feature union有所不同。3个二进制独立复制并绑定真实hash，见 `validation/curve-wallspace-source-suite-v5.json`。此结果覆盖当前c144/044c/4905及同31文件，V4红记录保留；完整utils缺sampling、整个core/App/世界GPU仍未通过。


### Cloth真实消费与下一个上游（2026-10-20 03:55）

两源实际consumer已按完整11167B主系统、434B输入fold及greedy/thread/cloth helper原证，独审后合入当前工程：读取唯一VisualState.cloth_infos、显式Wall bucket lookup、原indexed排序/degree/有限Ray遮挡、真实Local缓存与sound事件、同global transient map和实际beam/garment/thread Commands。衣物通过已有builder传入canonical三字载荷与真实NormalMap owner；没有从modified flags造ClothInfo或丢弃builder。见 `validation/cloth-consumer-integration-v2.json`。V2仅补强测试直接读取实际cull row mesh_index，业务prefix与已审V1逐字相同；12新+10旧测试尚未运行，不能称衣物游戏闭环。

上游调查已找到两真实窗口应产生Box<ClothInfo>，现旧body仍是零位姿/丢builder/缺typedBox，不准以默认Box修补。接下来按原条件、实际Transform/dims/WallId/DecoratorId和原caller恢复它们。常规Some-style flags、卸载/实际资产路径与完整App调度仍分别有门；不把本批absent-style consumer直接套给这些输入。
