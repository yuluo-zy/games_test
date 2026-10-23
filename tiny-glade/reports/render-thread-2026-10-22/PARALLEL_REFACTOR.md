# 并行、所有权与增量传输重构（2026-10-23）

本文件承接 ARCHITECTURE.md 第二阶段，记录用户确认的修订设计。实现状态以文末表为准；设计不是已完成能力。

## 不变量与目标

- 保留声明/编译/执行在渲染线程；游戏最多领先一帧。CPU 消息完成与 GPU 完成分开。
- 游戏拥有可变 CPU 业务数据，渲染拥有 GPU 分配、图会话、导入缓存和历史。
- 静态资产共享不可变 Arc 版本；小数据复制；大型动态数据移动所有权或仅复制脏区间。
- 所有消息、上传、重试、缓存及空闲池按实际容量计入有界预算。
- 不通过 unsafe Send/Sync 让并行声明共享可变图。

## 帧与资产协议

渲染器最终常驻线程。FramePacket 携带 App 代际、帧号、实体/资源增量、资产操作、上传数据、相机快照、egui 与截图请求；FrameResult 返回分阶段确认、类型化写回、错误和可复用 CPU 容器。

RenderAssetId 带代际；RenderAssetOp 使用单调操作序号。分别确认接收、CPU 应用、上传提交、历史写入提交、GPU 完成、呈现。重复操作不重复应用；上传成功后编译失败不得重放上传/创建/事件。旧 App 写回不得污染新 World；旧截图必须结算。

写回使用字段合并、源版本检查、计数增量和消费确认；存在性也需要版本。游戏可见变化必须触发 change detection，提取版本单独管理。

初始化、预热、App 激活迁为显式消息；线程内构造/销毁 Rc 图，最终删除 Exclusive 包装。窗口限定操作保留主线程。

## 提取与上传

- SSBO 影子留在游戏侧，只传声明需要的元数据和脏区间；Raster 保留 scratch，传脏瓦片。
- 网格、纹理、VAO、Placement 按资产发布新增/更新/删除；GpuWorld 分离逻辑实例与 GPU 表。
- CPU 只读大视图仅为实际消费者注册；写时复制粒度为资产/页，不整库复制。
- 内部可变写入推进显式版本。合并相邻/重叠区间，不填补大空洞。
- 高频组件建立变更和删除日志；未覆盖所有写者前保留扫描；删除日志到确认后回收。
- 业务 GPU flush/mapped 写入迁为 CPU 编辑及上传请求。
- 上传区分完整替换与有序增量；增量不能随 epoch 取消，也不能跨帧读边界合并。完整替换仅可取消未被已接受帧依赖的版本。
- OwnedUploadPayload 移动字节所有权，按 capacity 记账；失败必须返还未接收数据。
- 小型动态数据用帧槽；大稀疏修改用范围依赖；整体重写用有界多版本；安全的帧槽直接映射继续保留。
- 旧物理版本与上传租约按真实完成点回收；超大上传分块。队列满时避免排空线程阻塞在自己的生产请求。

## 并行与缓存

DrawBuilder 从 RasterPass 可变借用解耦：Prepare 确定附件/反射/逻辑资源与公共值；Collect 使用只读上下文和任务独占常量/输出；Commit 按原 pass 和稳定 draw 顺序分配槽、重映射索引、登记访问。先迁移串行等价路径，再并行单 pass 的连续 draw 区间，最后并行依赖层中的 Collect。

保持 draw/discard、透明/模板/间接绘制与资源内容语义。失败回收未提交批次，检查结果顺序确定。

模板缓存只存稳定布局/绑定/模板，动态值逐帧填充；依赖不完整的回调不开缓存。不缓存图句柄、帧槽和上传切片；命中仍声明资源访问。默认 32MiB LRU，App/设备/shader/资产/附件版本参与失效。

ECS 自动边与业务边分离，调度重建重新计算；覆盖新系统与 Commands 可见性。525 对仅是历史观测。诊断不得改变执行器策略。

ECS/声明/录制共用 tg-compute，默认 clamp(核数-2, 1, 8)，可覆盖。声明阈值独立测量，首帧/轻任务内联；限制任务数，禁止工作任务阻塞等待同池任务。

## 事件循环与相机

Update → AwaitRender → Finalize；每帧只更新一次。等待返回事件循环，不重复 redraw；结果发布后唤醒，等待前复查避免丢通知。resize 带代际、重建视觉数据但保留增量；暂停/关闭/线程异常都有独立退出路径。

晚更新从模拟相机基线预览新输入，不消费游戏输入、不再次推进 rig 平滑。精确拖拽/笔刷/放置当帧冻结改变操作投影的晚输入，操作绑定输入序号与相机版本。视觉悬停可重算但不能修改业务。回放只用已记录输入。

最终相机统一用于剔除/LOD/阴影/速度/TAA。拆开模拟常量与相机常量，历史按对应图像写入成功提交推进，不以 present 成败决定；失败/重建/跳变按历史有效性处理。

## 实施与验收

| 阶段 | 内容 | 状态 |
|---|---|---|
| 0 | 基线、计数、写回与调度边界修复 | 进行中 |
| 1 | 版本/确认/重试、拥有所有权的上传接口 | 部分实现：owned 上传/有序增量，SSBO/网格/纹理版本，CPU/提交写回分离；全资产操作序号与幂等协议未完成 |
| 2 | 渲染器常驻、逐库所有权迁移、启动/预热/App 切换 | 渲染器常驻及线程内构造/销毁已实现，删除 unsafe Send 包装；逐库 GPU 写入权限迁移未完成 |
| 3 | 增量实体提取、池复用、删除完整快照 | 部分实现：移除稀疏实例影子快照，Raster 写时复制；实体日志/池未完成 |
| 4 | DrawBuilder 拆分、串行等价、并行 Collect | 实现中：独立 DrawDeclarations、任务独占批次、稳定 Commit、拥有常量字节的移交和共享池 Collect 已接入；连续四帧真实 GPU 逐像素等价、强制并行完整游戏短流程通过；默认仍走原串行路径，长跑进行中 |
| 5 | ECS 重建、池粒度、声明缓存 | 部分实现：ECS 重建顺序修复、共享池粒度、32 MiB 反射布局 LRU 已接入；跨 pass 依赖层 Collect 未完成 |
| 6 | 非阻塞事件循环、晚相机与交互一致性 | 部分实现：AwaitRender/poll/唤醒、提交后确认时序历史；控制命令与上传仍可能阻塞，真正晚输入/交互投影未完成 |
| 7 | 全链路、长跑、性能对照 | 核心库全量、主程序 180 项单元测试、30 帧原版场景/截图及强制并行短流程通过；统一最终回归与 1000 帧长跑进行中，独立启动夹具复验中，真机性能对照未完成 |

每阶段测试后继续。验证不同/重叠上传区间、替换/增量混合、预算和失败原子性；资源删除重建、旧代际反馈；调度新增/重建/Commands；并行/串行和缓存开关一致性；事件唤醒/resize/暂停/关闭；相机交互/回放/历史；既有 rhapsody、桥接、Raster、VAO、加载、截图和真实 present 测试。

性能覆盖静态、大量 draw、局部/连续编辑、加载切换；预热后至少 1000 帧、三轮，测 CPU/GPU p50/p95/p99、Prepare/Collect/Commit、复制/上传字节、分配、扫描量、排队/依赖等待与内存高水位。静态帧不复制大库；局部传输随脏数据增长；稳定容量复用；长跑有界。真机轻场景 p95 回退超过 5% 的策略不默认开启；软件 Vulkan 只证明正确性。

## 已落实的修正（2026-10-23，工作区，尚未提交）

### 调度、写回与上传基础

- ECS 每次重建按当前显式依赖重新排序并补冲突边，覆盖新系统；显式依赖可以改变原顺序。修复初始化时移出 `Schedules` 导致的资源重建断言。Commands 的业务可见性仍要求显式同步边，不能以冲突对数量证明完整正确性。
- 写回校验来源 World、资源添加/修改 tick 和存在性，避免覆盖游戏侧后续编辑、删除后复活、旧 App 写回；正常触发 change detection。缺少目标资源时保留待移交队列。
- 上传队列接受移动所有权的 Vec，按 capacity 计账，拒收返还原分配。有序增量跨 epoch 不取消、不合并；生产 BufferHost 的脏区间重试复用退回数据。
- 提取记录资源复制数、组件扫描/复制/删除数；这些是工作量计数，未当作堆字节或帧时间。

### 稀疏 SSBO 与 Raster

- `SparseRenderBuffer` 只持 GPU 分配租约、每块实例数、不可变可见位图；不持 CPU 实例数组、空闲表、脏表或废弃分配队列。渲染快照不再能经 `get_mut<T>` 取得游戏侧存储对象。
- 墙体、屋顶、Sparkle、Nani 稀疏剔除改读 `SparseDrawView`；绘制范围算法与游戏侧共用。墙体高亮按排除集合过滤，不再临时 hide/show 共享位图，也不再误把本来隐藏的块恢复为可见。
- 以当前 Brick 容量计算，单次快照不再复制 `0x200000 × 96 = 192 MiB` 实例影子；保留的块计数与可见表约 72 KiB。屋顶瓦片影子另减少 16 MiB。此为数据结构尺寸计算，不是实测帧率收益；变化的库仍需复制元数据表。
- Raster 的 composite、scratch、tile hashes 改用独立 Arc 版本，静态快照共享分配；编辑时按缓冲写时复制。composite/scratch 提交改为版本共享。修复旧快照 selector 指向 scratch、但 scratch 为空的问题；小型瓦片暂存区保持有效，发布器与脏标志各侧独立。
- Raster 目前是整张缓冲的 COW（512×512 时 256 KiB），不是脏瓦片增量提取。仍需后续按瓦片移交和容量池优化。
- `AssetSsboLibrary::register(&self)` 推进独立注册版本，`resource_with_revision` 将其与 ECS tick 合并判断。所有 SSBO 声明清单已接入，覆盖共享引用注册后 ECS tick 不变的情况；静态库保持复用。

### 常驻线程、提交确认与事件循环（后续增量）

- `RenderThread::spawn` 移交 Send 工厂，在工作线程构造、使用、销毁可含 Rc 的 renderer。只有 RenderSide、控制闭包及结果过线程，已删除 `Exclusive<T>` 和 unsafe Send。窗口平台操作仍留主线程；启动失败、控制异常、关闭均回收线程。
- 帧请求、GUI 数据及 CPU 读取租约随 RenderSide 移交；GUI tessellation 和 DPI 在游戏侧按帧固定，避免渲染读取已被下一帧改变的 Context。
- BufferHost/TextureHost 加入 CPU 声明至提交的读取边界，发送前封闭，完成或异常时释放，防止队列中的帧尚未开始时游戏提前修改物理资源。这是保守互斥：游戏侧物理写入仍可能等待，并非 GPU 完成确认。裸 mapped 指针等绕过宿主的写者及帧槽/多版本迁移仍待完成。
- 网格共享引用注册和纹理描述符发布接入显式版本。静态重复查询不推进版本；提取仍结合 World 的资源添加/修改 tick。
- 写回分为 CPU 结果与实际提交后的消费确认；失败时清除相关渲染副本，使未变的源资源也能重新提取。地形回读请求按捕获数量确认并保留期间新请求，校验来源 World 和资源添加 tick。
- 时序历史先生成候选；异步路径的准备不提前修改游戏历史。图像写入实际提交后才确认候选和帧号，后续 present 失败不撤销已提交历史；旧 World、删除重建及游戏侧历史重置拒收旧结果。
- AwaitRender 每帧只执行一次 Update，未完成时返回事件循环，结果发布后唤醒并继续 Finalize。控制操作、上传和部分准备仍可能等待，尚未实现所有路径非阻塞。

### 下一段工作及边界

1. 审计其余内部可变资产与全部发布/删除路径，补齐操作序号、幂等确认及有界回收。现有三类库的显式版本不代表全资产协议已完成。
2. 将保守 CPU 读边界迁为按帧槽/范围/物理版本的 GPU 写入协议，覆盖 mapped 写者。CPU 快照隔离、GPU 租约保活、有序上传都不能单独保证 GPU 物理内容的帧版本隔离。
3. DrawBuilder 的 Prepare/Collect/Commit 和单 pass 并行已实现；继续覆盖跨 pass 依赖层，保持图所有线程上的稳定 Commit。默认开关需通过真机性能验收。
4. 补齐非阻塞控制流程、真正晚相机及交互一致性、缓存与真机性能长跑；现有提交确认仍需全游戏失败路径验证。

### 常驻线程增量验证

### Collect 实现与后续审查（本次继续，未完成验收）

- DrawBuilder 改为借用 DrawDeclarations、查询宿主和视口，不再保留整个 RasterPass 的可变借用。Collect 宿主不持图或 GPU 常量分配器；任务只返回拥有所有权的 DrawBatch。Commit 在图所有线程上按连续区间原顺序执行，分配常量并重映射绑定槽及间接缓冲。
- 逻辑 SSBO ID 位于真实图的 u16 资源域之外，Commit 逐项解析。保留 discard 的访问、未命中 shader binding 的 SSBO 访问和非缓存访问；只有原 pass 缓存本来会去重的访问被去除。纹理缓存访问模式或同名缓冲来源跨区间不一致时回到原串行声明。
- 常量 Vec 移入批次的租约，不在 Collect 克隆字节。默认不启用新路径；`TG_DRAW_COLLECT=serial|parallel` 用于等价／性能对照，`TG_DRAW_COLLECT_CHUNK` 默认为 64。最多 8 个共享池任务、4096 个 draw、32 MiB 常量字节；超出范围回退原路径。这个常量预算不是整个 FramePacket 的完整容量预算。
- 审查发现保守上传门禁与共享任务池组合可能形成循环等待。新增临时 ECS/Collect 准入门禁：Update 活跃时渲染 inline Collect；Collect 活跃时新 Update 保留 GUI 输入并返回事件循环，等结果后继续。不能因此宣称完整上传协议或所有路径非阻塞已经完成。
- 新增 Prepare/Collect/Commit 时间及任务计数，合并到现有帧诊断。依赖层之间的并行 Collect、上传帧版本、真正晚相机仍未完成；布局 LRU 已在随后增量中接入。
- 56 项 DrawBuilder 测试通过，包括新加的多线程独占批次合并、discard、常量数据、缓存槽及间接缓冲等价。随后对独立 DrawDeclarations 的变更继续复验。
- 核心库全量：2876 通过、1 ignored、0 失败（`/tmp/render-country-full.log`）。真实 GPU 新增逐像素等价测试发现第二帧的 native initial range 超出声明错误，正在定位，未绕过检查。历史夹具接入完整无窗口 GPU 资产所有者，不跳过生产墙体／屋顶 flush；仍在补齐共享启动资源。
- 额外修复：转发事件校验来源 World；组件提取查询切换 World 时重建；已销毁游戏实体的渲染镜像随删除回收，避免空实体长期积累。

以下是此前常驻线程的独立验证记录，不代替本次 Collect 的最终验收。

日志 `/tmp/render-resident-validation.log`：`render_snapshots` 5、`core_render_bridge` 10、`geometry_gpu` 1、`compile_before_submit` 1、`original_scene_compute_gpu` 10、`core_raster_draw` 11，共 38 项通过。新增常驻线程真实 GPU 上传/读回连续 32 帧；线程单元测试 11 项通过。

日志 `/tmp/render-resident-present.log`：Xvfb + lavapipe 下生产 GameFrameDriver 加载/呈现 3 帧、原生 WSI present 3 次及关闭通过。完整游戏模式先修复异步 Gameplay App 缺 `SteamdeckScreenshotMsgScreen`，再修复预热直接使用游戏 World 时错误要求 `Present<TextRenderer>` 标记，保留实际 producer 或提取标记两条合法路径。

随后 `/tmp/render-resident-gameplay-trace.log` 确认常驻渲染线程的羊材质辅助函数缺 `Sheep`（同路径也读取 `SheepWalkRecord`）。补齐两项清单，增加生产提取后调用实际材质读取函数的隔离/更新/删除回归；缺组件错误附带类型及实体。异常后关闭改为明确失败状态并 join，避免再次 reclaim 已损坏的线程反复 panic。最新复验日志 `/tmp/render-resident-gameplay-final.log`；其中线程、提取、读取边界和时序历史的 18 项针对性测试通过。

该次完整流程已执行 13 帧非空游戏图，但原生 present 验收因主线程读取旧交换链代而失败：`validate_swapchain_and_acquire_image(&mut Arc<Swapchain>)` 可替换 Arc，常驻包装不能保留启动时的副本。现改为在替换后发布当前 Arc；`/tmp/render-resident-gameplay-present.log` 复验通过：149 帧加载、13 帧非空游戏场景、13 次游戏阶段原生 SUCCESS/SUBOPTIMAL present，OS 关闭、保存线程与 GPU 退出完成。此为 Xvfb/lavapipe 下加载→游戏→关闭的短流程验收，不是全部交互、窗口往返或真机性能验收。

日志 `/tmp/render-core-history-check.log`：核心库 52 项通过，覆盖相机常量/延后提交历史、纹理发布版本、Raster COW 和笔画提交等。同步修复测试侧 Arc 写入、嵌套模块导入及索引类型的编译问题。最近完整 tiny-glade lib 为 164 通过、6 项 history/intro 失败；补齐 CPU 启动资源后，这 6 项仍缺生产墙体所需的 `WallConstructorOutput`/`BrickTransformSsbo` 等所有者，需要完整夹具，不以空壳资源或跳过生产系统冒充通过。

`cargo check --offline --locked -p tiny-glade -p country-core --all-targets` 通过，日志 `/tmp/render-final-alltargets-check.log`；现有警告仍保留。`git diff --check` 通过。

全部构建与临时依赖位于本地磁盘；不使用网络挂载目录。上述增量验证不替代完整游戏、长跑、真实 GPU 性能验收；下方保留此前快照阶段的独立验证记录。

## 验证记录（2026-10-23）

### 后续多角度审查与实际修复（继续工作中）

- 原生 token：高两位为外部宿主域，不能推动内部自动 token 进入该域。此前外部缓冲 token 会与资源池旧图像 token 冲突；修复后串行/Collect 交替四帧逐像素一致。
- 描述符生命周期：页缓存强持有 lease，并以资源引用数判存活；不同范围/视图会互相保活。改为有界弱引用索引（最多 4096 键/页、预算覆盖桶容量），实际参数组和 GPU 命令持有强引用。新增同一原生分配 8192 个范围的回收/复用和无自环回归通过。30 帧原版场景的描述符高水位约 2376，随后保持；旧路径第 30 帧约 11404 且继续增长。这是 lavapipe 正确性观测，不是帧率收益结论。
- UI 渲染：普通图标与颜色按钮 shader 选择反了。发行 SPIR-V 的 uniform 名及 FNV 键确认无 CustomizationIcon 应使用 ui_2d，有则使用 ui_2d_color_button；修复后实际回放叉号像素回归通过。
- 数据提取：稀疏实例块计数/可见表未变时复用同一不可变 Arc；仍比较公开元数据以覆盖绕过 writer 的既有写者。实例字节更新不会分配新 draw 表。旧快照与隐藏、回收、新版本隔离回归通过。
- 常量所有权：16 MiB / 1024 块有界 CPU 空闲池按 Vec 容量及索引容量计账；Commit、回退、取消和 discard 归还拥有的字节。复用时清零可选 uniform。GPU 常量页继续按原生完成点独立回收。
- 映射上传：MappedStorageBuffer 的 cpu_write 可持原生写入租约，蝴蝶/萤火虫生产写入现在经过 CPU 读取边界及 Rhapsody 的实际 GPU 范围依赖；租约保活分配，删除不必要的 unsafe Send/Sync。新增跨线程写入门禁及宿主销毁后的租约回归通过。直接访问公开 mapped 的 Raster 暂存/回读等路径仍待迁移，不能据此声称完整物理版本协议已实现。
- 取消析构：CPU 读取租约移到 RenderSide 的首字段，先于 World 释放。断连退回/未执行帧的 GPU 所有者析构不会等待自身仍持有的门禁；专项测试通过。
- 性能诊断：AwaitRender 返回事件循环后保留原帧起点和 Update 耗时，单独记录 yield_wait_ms，避免把恢复调用的几微秒误记为完整帧耗时；同时新增每帧 GPU/CPU 内存、描述符和常量页容量诊断。该项正随新构建复验。

本段增量日志：`/tmp/render-country-final-full.log` 核心库 2827 通过（含原资产 ignored 测试）；`/tmp/render-mapped-owner-regression.log` 主程序单元 180、Raster 12、宿主 18、快照 5 全通过；`/tmp/render-bindless-review.log` 后端单元 421、bindless 6/1 ignored、ABI 5、setup 4 通过，随后新增范围索引回归通过（`/tmp/render-bindless-index.log`）。`/tmp/render-gpu-full-fixed.log` 原版游戏链 6 项全部通过，含 33 个历史文件 / 2120 个条目与真实 30 帧场景；截图与回放叉号分别通过。独立启动测试先对齐生产显存预算，随后发现 MEI 要求安装目录作为 cwd，已修正并排队复验。以上日志对应各增量构建，尚未代替最后一次统一回归。

强制并行完整游戏日志 `/tmp/render-parallel-timing.log`：每个观察帧派发 16 个 Collect 任务，13 次非空世界原生呈现及关闭通过。该增量构建的 1000 帧窗口长跑已通过：`/tmp/render-parallel-1000.log` 记录 1001 次世界原生呈现及 OS 关闭/保存/GPU 退出；最新增量仍需复验。

全量后端初次执行的多队列分支因 lavapipe 缺少独立计算队列失败，必须保留为当前环境不能验收的硬件能力项。其余发现的宿主预算故障注入、完成状态轮询和命令回收测试已修正；尚需统一复验。全部文件和构建均位于本地磁盘。

本轮在 Linux / Mesa lavapipe 上执行，`RHAPSODY_REQUIRE_VULKAN=1` 禁止因设备不可用而跳过。`core_raster_draw` 开启 Vulkan validation；其旧夹具遗漏帧资源声明器，已补齐，未放宽图内容或生命周期检查。

| 本轮测试目标 | 结果 | 覆盖 |
|---|---|---|
| `render_snapshots` | 4 通过 | CPU 编辑/回收后的旧快照、GPU 租约保活、并发高亮过滤、Raster COW/scratch/发布隔离、共享引用注册及删除重建 |
| `geometry_gpu` | 1 通过 | 真实墙砖脏块上传/读回、静态复用、更新与回收；游戏库销毁后快照保活真实 GPU 分配 |
| `compile_before_submit` | 1 通过 | 编译未提交、取消、后续帧真实写入 |
| `core_render_bridge` | 9 通过 | 导入、惰性库资源、取消/异常、帧租约、旧帧回收、App 隔离 |
| `original_scene_compute_gpu` | 10 通过 | 六类 Raster、水/花园、地形、重投影、SPD、DOF、TAA/TGSR、预热历史、风格化 |
| `core_raster_draw` | 11 通过 | 真实像素、索引/深度/外部资源、双帧在途连续 30 帧、temporal resize/取消、设备错误 |

本轮共 36 项通过，`git diff --check` 通过。日志：`/tmp/render-snapshot-final-tests.log`、`/tmp/render-raster-final-tests.log`。不是全游戏验收，也没有真机 p50/p95/p99 性能结论。

此前基础修正的验证：rhapsody lib 420 通过/2 ignored，传输 GPU 8 通过，render_thread 8 通过，ECS 重建 3 通过；本轮未修改 rhapsody/ECS 实现、未重复这些全量检查。此前 tiny-glade lib 还有 6 项 history/intro 测试因缺 `TerrainHeightsData` 失败，本轮未修复、未计为通过。

复现本轮命令（工作目录 tiny-glade；缓存源是已有的 USTC 离线缓存，不发起代理请求）：

```bash
unset HTTP_PROXY HTTPS_PROXY ALL_PROXY http_proxy https_proxy all_proxy CARGO_HTTP_PROXY
export NO_PROXY='*' no_proxy='*'
export CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1
export PKG_CONFIG_PATH=/tmp/render-build-deps/sysroot/usr/lib/x86_64-linux-gnu/pkgconfig:/tmp/render-build-deps/sysroot/usr/share/pkgconfig
export PKG_CONFIG_SYSROOT_DIR=/tmp/render-build-deps/sysroot
export LIBRARY_PATH=/tmp/render-build-deps/sysroot/usr/lib/x86_64-linux-gnu
export LD_LIBRARY_PATH=/tmp/render-build-deps/sysroot/usr/lib/x86_64-linux-gnu
export VK_LAYER_PATH=/tmp/render-build-deps/sysroot/usr/share/vulkan/explicit_layer.d
export RHAPSODY_VULKAN_LIBRARY=/usr/lib/x86_64-linux-gnu/libvulkan.so.1
export RHAPSODY_REQUIRE_VULKAN=1 VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
cargo --config 'source.crates-io.replace-with="cached-ustc"' \
  --config 'source.cached-ustc.registry="sparse+https://mirrors.ustc.edu.cn/crates.io-index/"' \
  test --offline --locked -p tiny-glade --no-fail-fast \
  --test render_snapshots --test geometry_gpu --test core_render_bridge \
  --test compile_before_submit --test original_scene_compute_gpu --test core_raster_draw
```

系统开发库和验证层仅解包至 `/tmp/render-build-deps/sysroot`；另一台机器需准备对应依赖。为完成链接清理过 `tiny-glade` 包的可再生成构建产物，未清理源码、资产和用户改动。

### 最新增量：资源身份、诊断与组件删除（2026-10-23）

- 所有 BufferHost / TextureHost 分配使用全局唯一代数，编码到 ID。只接收 ManagedHandle / ID 的图和材质入口也验证代数，拒绝另一个 App 的同号槽位及释放后复用。旧物理 owner 仍由已有 GPU 租约保活；分配身份不等同于内容多版本。新增真实 GPU 跨宿主 / 槽位复用测试。
- AssetVAOLibrary 按不可变 Arc<VaoEntry> 发布绑定元数据；克隆不再逐项复制绑定 Vec。全局发布版本防止两个克隆分支各自重建一次得到相同 revision。Raster 快照可复用相同 publication 的原生视图，命中仍校验宿主代数，不复活已释放缓冲。
- ProductionRenderThread 的诊断读取改为固定容量的最新快照，不再通过同步 call 取值而等待后续在途帧。快照携带对应交换链代的原生成功计数，窗口验收按来源 World / receipt 去重，避免异步缓存导致重计或跨代比较。三帧 WSI 夹具验证读取诊断后仍保留 in_flight。
- 组件镜像消费 Bevy 删除事件；静态帧不再改写 seen 表并再次遍历删除差分。删除记录过期时显式回退核对，保留变化 tick 扫描，尚非所有生产者的修改日志。回收最后一个镜像组件留下的空壳；切换目标渲染 World 强制重新提取，包含已存在同类型资源的目标。变更组件批量插入并复用 pending 容量。
- 启动 GPU 夹具只执行实际 PreStartup/Startup/可选 PostStartup，并显式调用生产 MEI flush。完整 Update 由真实 GameLoop / original_gameplay_chain 验证。Nani 断言逐个核对实际注册 mesh，其他新增加的 brush/UI preview 不再误触发固定总数断言。

增量验证：`/tmp/render-nonblocking-vao-final.log` 主程序 180、启动 2、VAO 1 及真实 WSI 三帧通过。`/tmp/render-asset-generations-final.log` 后端桥 10、启动 2、宿主 19、VAO 1 通过；其两个旧测试仍断言“复用槽位后 ID 相等”，已改为低 32 位槽位相等、完整 ID 不同，并在 `/tmp/render-unified-final.log` 复验主程序 180、Raster 12、桥 10、几何 1、截图 1、启动 2、宿主 19、回放 1、VAO 1 全通过。该次历史测试因命令漏设 TG_INSTALL_ROOT 失败，补上原资产目录后的专项 `/tmp/render-history-assets-final.log` 已通过；不修改测试放宽验收。核心库新构建含原资产完整 2828 通过（`/tmp/render-country-final-2828.log`）。删除日志增量主程序 182、快照 5 和 WSI 三帧通过（`/tmp/render-journal-final.log`）；随后的批量插入 / 目标资源重提取与完整 tiny-glade 回归正在执行。

后端全量已结束：`/tmp/render-rhapsody-final-full.log` 共 88 个目标，803 通过、7 失败、10 ignored；七个失败均为要求独立计算队列而设备初始化报 Unsupported，不能列为本机已通过。最新构建窗口往返使用独立本地 Xvfb 与 xfwm4（DISPLAY=:2）；先前增量的 DISPLAY=:1 长跑已通过，不能代替最新构建回归。


### 完整循环审计增量

用户授权的只读 agent 已完成从主循环到回收的核对并复核修复；详见 [RENDER_LOOP_AUDIT.md](RENDER_LOOP_AUDIT.md)。新增工作纳入原阶段：阶段 5 补录制准入和任务窃取隔离；阶段 6 拆加载 readiness/交接等待、后续迁移控制消息；阶段 1/2 缩窄物理读边界与异步冷管线准备；阶段 6/7 补运行时尺寸发布、实际重试相机历史及完整窗口往返。上述新代码仍须以最新日志验收。

当前确定未完成项继续保留：完整资产序号/幂等确认与物理内容版本、普通影子/生产者变化日志、Raster 脏瓦片、跨 pass Collect、晚输入相机/交互、非阻塞窗口控制、异步冷管线、自动分辨率 VRAM/adaptive 控制器、最终全量与硬件性能验收。历史全量 tiny-glade 只剩 30 帧描述符稳态峰值断言失败，正在分析；不通过扩大固定容差将它记成成功。


### 后续执行节点（2026-10-23，用户要求明确进度点）

整体未完成。以下替代模糊的“继续优化”，每节点必须更新实现、验证、缺陷、下一步四项。

| 节点 | 内容 | 当前状态与出口 |
|---|---|---|
| 1 | 当前缺陷闭环：telegraph旧UI路径、SSBO导出、控制邮箱 | telegraph 3项通过；SSBO稳定导出已修，专项待跑；主程序194单元、桥10、宿主19、默认WSI三帧通过。完整窗口异步代际验收尚未通过 |
| 2 | 冷管线Prepare/Pending/Ready/Execute | 有界deferred API、帧状态机、Update准入握手、取消及resolve依赖已接入；后端421+async11+parallel graph3通过，主程序194及默认WSI通过。随后新增的后台预热取消/关闭收尾尚未重编译验证；队列饱和及真实窗口回归待做 |
| 3 | 窗口异步控制完整闭环 | 有界最新状态、代际ACK、错误发布、忙帧不reclaim通知已实现；关闭优先及prepared/截图取消已接入，最新收尾待验证。窗口往返夹具已修正等待新代际的时机，完整往返待复验 |
| 4 | 上传/资产协议与物理内容版本 | 先前owned Vec/有序增量/部分ACK已实现；完整操作序号/幂等/范围与有界物理版本待完成；随后移除过宽门禁 |
| 5 | 增量提取与减少复制 | 稀疏视图/Arc共享/删除日志已实现；普通影子、生产者修改日志、Raster脏瓦片、容器池待完成 |
| 6 | 跨pass并行与稳定缓存 | 单pass实际共享池Collect 480任务/30帧通过；跨pass依赖层、PGO稳态及默认策略仍待完成 |
| 7 | 晚相机、交互投影、自适应比例 | 实际重试历史/手动尺寸已修；晚输入一致性及VRAM adaptive controller待完成 |
| 8 | 统一全量回归和多角度review | 修改收敛后统一；四场景1000帧三轮、资源/错误/窗口/取消/串并行等价；软件正确性和真机p95对照分开 |

当前优先完成节点1–3，再推进4–7，最后执行8。旧增量1000帧长跑已在1800秒时限结束：加载151帧、游戏809帧，未完成验收。该次不是最新冷管线代码，且Collect实际任务数为0，不能作为所有并行路径验收。此前另一增量1001帧通过也不替代最新版本回归。


### 当前推进记录：关闭、自适应与跨 pass（2026-10-23）

本节更新上表的增量事实，整体仍未完成：

- 节点 1–3：`/tmp/render-window-admission-final.log` 完整窗口循环通过（13 次世界原生呈现、代际 7、4 次实际 OS resize）；`/tmp/render-loading-cancel-final.log` 与 `/tmp/render-suspended-cancel-final.log` 均通过。加载/挂起状态关闭无需恢复渲染，保存和 GPU 所有者已收尾。Update 准入现在在主图提交后、WSI 尾部前释放共享读边界。
- 冷队列：`/tmp/render-deferred-pressure-tests.log` 5 项管线单元通过，含确定性饱和/重复票据/回收；该命令过滤了 async integration（0 项），不能误记为其通过。新增 owned SPIR-V 容量检查仍需单独执行。
- 自适应：canonical `AdaptiveRenderState` 已接入 resident Prepare；预算恢复使用 250ms 等待、指数退避及恢复观察期；OOM 锁定有效倍率。GameConfig 保持用户请求，字段反馈保护 World/资源身份、较新请求、resize 和 flag_20。无 VK_EXT_memory_budget 时显式采取平台适配：不把未知预算当作 0，也不拿 heap capacity 冒充预算。`/tmp/render-adaptive-tests.log` 主程序 199 项通过。
- 同次核心库 2891 通过、1 失败、1 ignored；失败定位为 SSBO 导出排序同 resource 编号未按 generation 次级排序，已修待复验。测试数量包含当前工作区其他模块新增测试，不代表本重构新增数量。
- 跨 pass：三个阴影级联已接入有界共享池收集和稳定顺序 Commit，增加独立 cross_pass_tasks 计数。第一轮真实游戏测试发现提前构造 RasterPass 后回退违反必须 render 的协议；已改为先完成工厂/反射/收集，再创建图 pass。`/tmp/render-cross-pass-tests.log` 的游戏链 2 失败不能计为通过；Raster 12、快照 5 通过。后续修复正在 `/tmp/render-projection-cross-pass.log` 强制并行复验。
- 通用 shutdown 已改成停止并归还 queued/Pending 帧，避免只在 production cancel 路径安全；新增专门回归待跑。PreUpdate 不再每帧无谓标记 Adaptive changed。
- Raster 所有权进一步收窄：生产提取投影只保留 dirty bitmap/消费标志，像素与 hash 仍由游戏拥有；GPU 数据来自原 staging SSBO。这样不因渲染快照使下一笔编辑触发整块 COW，普通完整 Clone 继续保持原语义。新增实际提取测试验证 0 CPU 像素保留及游戏编辑地址稳定，当前待验证。

仍待完成：完整上传序号/幂等与物理内容多版本、生产者变更日志、晚输入相机/交互、跨 pass 默认性能策略、最终统一全量及长跑。上述投影属于减少复制的实现变更，尚不等于完整上传协议或真机性能验收。


上述修复的验证结果：

- `/tmp/render-projection-cross-pass.log`：主程序 200、Raster GPU 12、真实游戏链 6、快照 6 全通过；30 帧实际共享池任务 1200，其中跨 pass 90。GPU 完成后描述符 actual/expected 都是 `[10, 0, 8, 72]`。强制 `TG_DRAW_COLLECT=parallel`、chunk=1、`TG_REQUIRE_CROSS_PASS=1`，没有用 inline 回退冒充跨 pass 验收。
- `/tmp/render-core-projection-full.log`：核心库本次 feature 集合 2833、资产库 9 全通过，包含 ignored 资产用例；SSBO 稳定导出修复通过。
- `/tmp/render-deferred-async-12.log`：异步管线 12 项全部通过，包含新增 owned SPIR-V 容量预算/稳定完成票据。
- 只读 agent 复核了六种实际 Raster 消费者，无 CPU 像素读取遗漏；跨 pass 工厂/反射失败、取消、异常析构与图 owner 边界未发现新的明确缺陷。
- 在最后一次小调整中，shutdown 对已有 idle packet 直接取回，避免停止线程后再 poll 断开的完成通道；该调整随 `/tmp/render-adaptive-window-final.log` 复验。该日志还验证最新完整窗口循环、截图与场景计算。

这组通过结果不替代剩余物理版本设计、晚相机、所有场景长跑或真实硬件性能门槛。


### 最新窗口闭环与物理版本接入约束

- 最新窗口测试曾暴露新增自适应依赖可选 PhysicalDevice 包装的问题；实际 WSI Device 未挂接该包装。改为 Device 缓存真实驱动 MemoryBudgetSnapshot，已有 refresh_memory_budget 发布，读取不触发额外 Vulkan 查询、不使用 allocator 配额冒充预算。`/tmp/render-device-budget-window-fixed.log`：主程序 201 项，完整窗口循环通过（loading 163/world 13/native 13、代际 7、实际 resize 4、OS close/save/GPU shutdown）。
- 该 panic 同时暴露 Prepare 异常未打开 Update 门禁，主循环被准备等待状态遮住。两个 panic 返回分支现先释放 active 读边界，再打开门禁，发布异常并唤醒；新增专项通过。随后主循环的准备等待及 Update 前增加非阻塞完成/异常 poll，防止先跑一次 Update 才传播错误；最新单元复验日志 `/tmp/render-panic-gate-final.log`。
- 原始预算缓存后端测试 `/tmp/render-budget-cache-confirmed.log` 5 项通过；`/tmp/render-budget-cache-final.log` 的 async 12 和预算快照 2 通过，其 device_budget 夹具最初错误假定初始化不查询预算，修正为比较最近实际查询后复验通过。

物理内容多版本的后续实现必须遵守本轮只读审计确认的约束：

1. 在 queue_frame 的现有读边界封口点捕获不可变 FrameResources，明确持有 logical ID→physical owner。CoreGraphFrame/FrameBufferGraph/DeviceHost/compute external slots/RasterPassHost/CollectHost/VAO binding 都必须解析同一个 frame view，不能遗漏路径后落回下一帧 live slot。
2. frame 内新建资源使用显式 local overlay，旧 ID 的 miss 必须报错，不能静默读取 producer 的新版本。图 token 使用实际 physical owner 的既有 import_frame_buffer/texture 路径，不能把同一 logical token 反复绑定不同 owner。
3. 首批仅限生产者显式登记、CPU 权威写入且无长期 mapping/BDA/GPU 写入的缓冲。SparseInstanceBuffer 已在 flush 传入完整 data slice 与 dirty ranges：有旧 frame pin 时从同一既存 CPU 全源初始化新 physical owner，无 pin 时仍只传 dirty；不额外增加整库 CPU 影子，也不按 STORAGE_BUFFER usage 猜测可迁移性。
4. 显式 frame pin 计数不能以 Arc::strong_count 代替。上传成功后原子发布 current；release 只撤销 current logical entry，旧帧、已接受上传、GPU receipt 的旧物理 owner 保持到真实完成。
5. 上传队列的 same_key/epoch 已同时比较 physical Arc identity 和 logical UploadKey，新旧 physical owner 天然隔离；不能用全局 cancel_before 清掉旧帧已接受工作。
6. MappedHostWriter/长期 mapped slice、CustomCursor、GpuToCpu 回读、嵌入 BDA 的 Trimesh/chunks，以及 roof gravity/Nani/草叶花园等 GPU 写资源，首批保留稳定 owner 与现有同步。Egui 还有直接 get+record_texture_regions 写入，纹理版本迁移不能遗漏。

以上约束已在下面的多人并行增量中开始接入生产；验证范围以最新日志为准，节点 4 的全部上传协议仍未完成。


### 用户授权多 agent 后的实现与验证（2026-10-23）

三名 agent 分别负责物理资源版本、生产者变更日志、晚相机；主 agent 负责跨 pass 扩展、主循环集成、统一构建及交叉审查。构建产物、临时文件与日志全部位于本地磁盘，不使用网络挂载。

- 物理资源：queue_frame 封口后冻结 logical ID→physical owner，Bridge/Raster/compute/VAO 使用同帧解析器；egui 在冻结后创建的 IB/VB 已通过 frame-local host 登记。帧完成、取消、shutdown 释放冻结视图。显式 CPU 权威的 sparse buffers 可在旧帧 pin 下 fork；映射/BDA/GPU 写路径继续稳定 owner 和读边界，不宣称全部 Update 已与 Prepare 重叠。退休 owner 按逻辑字节和 owner 数量限额，不能称为精确 Vulkan 分配字节预算。
- 资产提取：SSBO/Mesh/VAO 已接真实有界生产者日志和复用目的库的增量更新。日志过期或源 World 改变显式重建，稀疏可见性共享引用修改会通知日志。普通 SSBO CPU 影子与 Raster 像素仍只由游戏侧拥有。
- 组件提取：静态 Handle/DrawTopology/pass 标记已接 immutable 组件类型级 insert/remove hook 和增量镜像；新增 replace/remove/despawn、溢出、换 World 测试，当前统一构建中。Transform 等可变组件仍保留 tick 扫描，不把此子链当作所有组件已完成日志化。
- 跨 pass：已从三个阴影级联扩展到主 G-buffer、OC/2 与 late pass，保持原顺序 Commit 和附件/依赖语义；当前正在运行真实游戏链。性能默认策略尚未经过真机 p95 对照。
- 晚相机：已实现渲染侧独立 MainCamera 投影、非消费累计输入 mailbox、水位/边界校验和一次 latch；只在无精确编辑的持续旋转状态启用，点击/绑定/模式变化冻结。actual camera history 仍以真实提交为准。已添加随成功 GPU 提交发布的 late_camera_input 诊断，真实窗口注入场景待验证。

验证事实：

- `/tmp/render-incremental-core-full.log`：核心库 2838 单元 + 9 资产库集成全通过（含原资产 ignored 用例）。
- `/tmp/render-parallel-integration.log`：主程序 208、桥 10、宿主 19、快照 8 全通过。新增 GPU 用例实际读取旧/新 physical owner 内容，验证取消/代际复用/严格缺项/local overlay，以及预算不足时等待旧边界；晚相机幂等、交互冻结、重试历史用例通过。
- `/tmp/render-main-cross-pass-integrated.log`：新组件 hook 与扩展跨 pass 的下一轮真实链回归进行中，不能提前记为通过。

整体仍未完成：新实现统一全量、完整上传序号/确认协议、尚未迁移的可变组件生产者、窗口晚输入实际命中、最终多场景长跑及真机性能验收仍需继续。



### 并行/非阻塞收尾任务（2026-10-23 晚起，Windows 本机 + 本机 Vulkan）

此前一轮把回读消费者改成 `try_cpu_read`（等 GPU 完成才读、回调内持提交锁），方向错误：游戏线程与渲染提交互相加锁，且 GPU 每帧改写时长期读不到数据。已整体回退，按下表重做。每完成一项更新本表。

| # | 任务 | 状态 |
|---|---|---|
| 1 | 回退错误的 `try_cpu_read` 回读迁移与 `mapped` 私有化 | 完成 |
| 2 | GPU 回读改为渲染侧复制、完成后发布，游戏侧只收消息 | 完成 |
| 3 | 拆除 `UploadBoundary` 写阻塞：游戏侧物理写入改 owned 上传/帧槽 | 完成 |
| 4 | 移除 `ECS_COLLECT_GATE` | 完成 |
| 5 | 资产操作序号/分阶段确认/幂等 | 完成（库级；见下） |
| 6 | 增量提取：可变组件变更日志、Raster 脏瓦片、容器池 | 未开始 |
| 7 | 非阻塞控制流程（resize/App 切换/截图） | 未开始 |
| 8 | 跨 pass 收尾 | 未开始 |
| 9 | 晚相机验证 | 未开始 |
| 10 | 统一编译/测试验收与文档更新 | 未开始 |

#### 任务 2：回读消息化（完成）

- 游戏侧 `country_core::render::gpu_readback::GpuReadbacks`：每类型保留最近一份完整副本（16 字节对齐 owned 存储）、产生它的渲染已提交帧号、发布次数；旧帧结果拒收，被替换的容器返还渲染侧复用。`submitted_frame` 是游戏已确认的最新提交帧。
- 渲染侧 `tiny_glade::gpu_readback::ReadbackPublisher`（`complete_render_feedback` 中，位于提交写回之后）：每个成功提交帧对游戏真正消费的 7 类回读（岩石遮罩、鼠标地形笔刷、鼠标深度、DoF 对焦、bush、两类地形高度）发出 `TransferManager::read_buffer`；后续帧收尾非阻塞 `try_read`，完成的数据连同来源 WorldId、帧号经 `FrameFeedback` 移交。地形高度只在 `copy_issued` 的帧（或首次、或上次因暂存槽满欠下时）复制。暂存 48 槽 × 512 KiB 上限、惰性分配；槽满跳过并计数，未完成复制超过 40 个时丢最旧。`BufferHost::convert` 对主机可见缓冲追加 `TRANSFER_SRC`。
- 消费者改读发布数据，不再触碰映射、不拿 GPU 锁、不等待：地形笔刷索引/悬停/绘制、鼠标深度、脚步声、Nani 预置花、动物移动/避障、蝴蝶、森林、bush 网格（同一 revision 不重复处理）、拍照对焦。时序敏感项按帧号判定：地形高度等到"发出复制那一帧"之后的数据；预置树转换等到生成请求之后提交的帧的岩石遮罩。首份数据到达前使用缓冲初值（全零 / 索引 -1），与原版首帧前读到的初始内容相同。地形绘制恢复原版"先建笔画再读回读"的顺序。
- 验证：`cargo check --workspace --all-targets --exclude toolbar` 与 `-p toolbar --lib` 通过；country-core 相关 369 项单元、clutter/critters/forest/tree 共 88 项通过；新增真实 Vulkan（Windows 本机设备）测试：复制→非阻塞完成→仅向来源 World 发布、发布后映射改变不影响副本，通过。
- 已知范围：只有生产 `ProductionRenderer` 路径发布；无窗口 `render_headless_gameplay_frame` 等夹具不发布，依赖回读结果的 GPU 集成测试需要接入同一发布器后再复验（任务 10）。`render_host::accepted_sparse_upload_survives_submit_oom...` 在本机失败于其测试辅助 `read_buffer` 的 usage/范围校验（GpuOnly 稀疏缓冲），与本改动无关，留待任务 10 定位。
#### 任务 3：生产者写入不再等待（完成）

- `UploadBoundary::write_or_defer`：其他线程持有帧读租约时，写入作为拥有字节的作业入队并立即返回；租约释放（主图提交后的准入、取消或 shutdown）在重新开放边界前按 FIFO 执行全部作业。之后的直接写入因此仍排在先前延后的写入之后；租约持有者（渲染线程）自身的 render-local 写入照常立即执行、排在下一帧作业之前。延后字节上限 64 MiB，超出才回退为等待并计数（`BoundaryStats.budget_waits`）。作业失败记入粘滞错误，由下一次写入返回，不静默丢失。
- 改走 `write_or_defer` 的路径：`BufferHost::update_buffer_bytes`（同步校验、紧凑 owned 负载）、`BufferHost::release`、`MappedHostWriter::write`、`TextureHost::release`、`TextureHost::update_texture`（owned 像素副本）。`update_persistent_descriptor_set` 只做校验，去掉边界。显式同步的 `update_buffer_bytes_wait` 与 `MappedHostWriter::with_write` 保留等待语义，生产 `MappedStorageBuffer` 已不再调用后者。
- 主机可见目标（含 TRANSFER_DST）不再由 CPU 写持久映射（那要等在途 GPU 读完），改为与设备本地缓冲相同的有序 GPU 复制，由下一帧 `flush_pending_uploads` 在图执行前提交。没有 TRANSFER_DST 的主机可见缓冲仍按旧路径写映射。
- `MappedStorageBuffer`（有宿主写入器时）持有游戏侧 CPU 影子：编辑在影子上运行，按 256 B 块比较出变化区间，只上传变化部分（Raster 暂存区的脏瓦片编辑因此只传脏瓦片）。影子首次编辑时从映射初值播种（CPU→GPU 内存不被 GPU 写）。`cpu_contents()` 返回影子；本地/测试映射仍原地编辑。
- 验证：`render_upload_boundary` 新增"生产者不等待、延后写按序在边界重开前执行、失败由下次写报告"单元；按新语义改写 4 个旧"必须阻塞"测试：萤火虫映射写入在租约期间立即返回、释放并 flush 后 GPU 内容正确；COW 预算耗尽时写入延后而旧帧仍读旧字节；主机可见缓冲经 GPU 复制更新；Raster 暂存编辑不等待真实 GPU 读、其复制排在该读之后（读到 11，之后映射为 22）。真实 Vulkan 下 tiny-glade lib 211 通过；country-core lib 2841 通过；critters/clutter/forest/tree 全部通过；集成 core_render_bridge 10、render_snapshots 10、compile_before_submit 1、geometry_gpu 1 通过。
- 环境限制：本机无 Vulkan 验证层，`tests/render_host.rs`（19）与 `core_raster_draw`（11）在设备初始化即失败，未能验收；`gui_tod` 缺原版资产；`render_host::accepted_sparse_upload…` 的测试辅助对无 TRANSFER_SRC 的 GpuOnly 缓冲调用 `read_buffer`，为既有夹具问题。三者均留待任务 10。
#### 任务 4：移除 ECS_COLLECT_GATE 与准备期 Update 门禁（完成）

- 门禁存在的原因是"Update 的池任务可能阻塞在渲染帧的读租约上，而渲染的 Collect/录制任务排在它们之后"的循环等待。任务 3 之后生产者不再等待租约，唯一剩余的等待点是延后预算（64 MiB）耗尽：现在只有非池线程会等；`tg-compute` 池线程超额入队并计入 `budget_overflows`，从而在任何情况下都不形成池内循环等待。
- 删除 `ECS_COLLECT_GATE` 及其全部使用：`render_bridge::with_recording_admission`（execute_built / execute_compiled 直接执行，录制保持配置的并行 runner）、`raster_host::try_parallel_collect` 的 try_lock、`GameFrameDriver::update_if_admitted` 的让出分支。Update 与共享池 Collect/录制可以同时进行。
- 同时去掉 Update 前的"准备期准入"让出（`render.update_allowed()` 为假时保留输入返回事件循环）：其注释写明唯一原因是生产者会在冷管线编译期间阻塞于该包的读租约，现已不成立。模拟在冷管线编译期间继续推进；`update_allowed` API 与渲染线程内部状态机保持不变（窗口循环仍用它决定是否唤醒）。
- 测试：新增"池线程超出预算也不等待"单元；改写 `collect_admission…` 为"无准入门禁、保留输入合并、只更新一次"；删除只验证门禁回退的 `recording_admission_falls_back…`。tiny-glade lib 211 通过（剩余 2 项为任务 3 已记录的环境/夹具问题）；core_render_bridge 10、render_snapshots 10、compile_before_submit 1、geometry_gpu 1 通过。强制并行完整游戏短流程需原资产与窗口环境，留待任务 10。
#### 任务 5：资产版本分阶段确认与幂等（完成，库级粒度）

- 操作序号：沿用各资产库已有的单调发布版本（SSBO/网格/VAO/纹理的 `snapshot_revision` 等，经 `copy_incremental` 提取）。同一来源 World 内只增不减，渲染侧对已应用的 (World, 版本) 不重复应用；已有副本就地增量更新而不重建。
- 分阶段确认 `country_core::render::asset_ack::RenderAssetAcks`（游戏 World 资源）：`applied`（渲染副本已应用）→ `submitted`（引用该版本的帧成功提交）→ `gpu_complete`（该帧 GPU 完成）。`copy_incremental` 每次应用记入渲染 World 的 `AppliedAssetRevisions`；`ReadbackPublisher::finish_frame` 取走它，随反馈确认 applied/submitted；GPU 完成以同队列上帧后发出的回读复制完成为证（之后提交的同队列 fence 完成意味着该帧已完成），无需额外等待或 fence。
- 失败路径：编译/提交失败的帧只确认 applied，不推进 submitted/gpu_complete；增量库的渲染副本不回退，重试不重放已接受的上传/创建。确认按来源 WorldId 过滤，旧 App 不污染新 World。
- 测试：`asset_ack` 阶段推进/失败/乱序重复完成单元；`render_thread` 新增"同一版本只应用一次、再版本就地更新且确认"；真实 Vulkan `gpu_readback` 测试扩展为验证 applied/submitted、未提交帧只推进 applied、回读完成后 gpu_complete 推进。
- 范围说明：粒度是"库版本"，不是单个资产条目；条目级 RenderAssetId 代际由既有宿主分配代数（任务前已实现）承担。没有回读源的 App（加载界面）不产生 gpu_complete 确认。