# 页式绑定合同补充 A：adapter 与组准入（待审）

这是对 CONTRACT.md 的不可变增补，不覆盖首版；第一阶段 new_page 只提供物理页原语，完整宿主分页尚未实现。

## 漏项修正：IndexedCountAdapter

独立 reviewer 发现并由 worker 逐句复核：ShaderHost::indexed_count_adapter（render_host.rs:1027–1040）把模板 heap 传给 rhapsody::rg::IndexedCountAdapter::new；后者在 indirect_count.rs:254–271 独立 new BindlessParameters，写 source/count/scratch 三个 buffer，再加一个参数 SSBO。它不经过 ShaderBindings::parameters。root M3 caps DrawIndirectCount=false，使此适配器成为真实必经链，不能声称自动被游戏 shader 路由覆盖。

最窄方案：通用 BindlessPagePool/完整组准备边界位于 rhapsody，ShaderHost 与 adapter 共享同一个 pool。adapter 仍用模板的真实 layout 编译同一 internal SPIRV；add_pass 只声明原 commands/counter/scratch 访问与原尺寸，录制时才从 managed 获取实际 owner/range，提交完整逻辑绑定组与原 push 给 pool。需求至多 [0,0,4,0]，若 source/count 相同 owner/range/role 可按现 builder key 去重。prepared 返回后 bind 真实页，原 `ceil(maximum_draws/64)` dispatch 不变；无 FrameGlobals 或 sampler 注入。

现有 new(device,heap,compiler) 可保留单heap兼容行为；新增显式paged构造或完整group-preparer注入，不让 rhapsody 反向依赖 tiny-glade。不能只注入“返回 Arc<heap>”的选择器，因为没有组容量保护。模板 heap 不能继续被 adapter 偷写。具体签名与文件变更待 root/reviewer 审；本补充不引入第二个 renderer。

## GroupReservation 的确切作用域

rhapsody PagePool 外置 PageRecord 保存 heap 与 per-page group_mutex；heap 自身仍只管理物理 set/slots。Pool manager 的 mutex 只用来选择并 clone 实际 PageRecord owner，立即释放。随后从该 owner 建立带生命周期的 GroupReservation（借用该页并持 group_mutex guard），事务结束前不让其他 paging writer 消费该页。

采用 closure/借用 guard 入口，例如 pool.with_group(...)；owner Arc 在外层局部变量中覆盖 guard 借用期，不返回“同时拥有 Arc 又借用其 mutex”的自引用 guard，不用 unsafe 延长生命周期。返回值是已经拥有实际 leases 的 PreparedBindlessParameters，不是 guard。

所有 production paging writer 必须走此入口：模板页首次参数写入、每页 sampler 初始化、FrameGlobals 页初始化、local resource/individual-mip writer、同页 with_push、跨页重准备、IndexedCountAdapter。paged pool 不向其他生产调用方发布可自行分配的裸页。pipeline 编译只读取模板 layout；single-heap便利API保持原行为，不能暗中作为paged路径的写入捷径。

组准入在 guard 内、开始写 descriptor 前完成。free 预检只有在该互斥 guard 阻止其他所有分配者时才构成 reservation；单独读取 stats.free 不构成 reservation。准入信用覆盖本次实际新增的四类 lease 数及参数块，直到 Prepared 创建成功/回滚才释放。已有 lease Drop 或 retiring collect 只会增加 free，不破坏已准入组。

准入失败用 pool 内部明确的 InsufficientSlots 结果驱动换页；不能 catch 任意 Error::Exhausted 就重试，因为其他上传或预算错误不等同于页空间不足。OOM/DeviceLost/InvalidUsage 等原错误立即传播。fresh页仍无法容纳完整组返回 shader/ABI/逐类需求明细，不跳draw、裁数组或放松ABI。

## 精确缺失需求与缓存发布

先冻结 callback 首次使用的 ABI、sampler实际 Arc 元数据、FrameGlobals Arc、local resources/image_arrays、buffer角色和范围。compute 迁页必须重放这份快照，不能重新读取 ShaderBindings.frame_globals 的“当前帧”指针。

在 guard 内 lookup 同页 sampler/FrameGlobals 缓存；取得 owned snapshot 后立即释放缓存 map 锁，再做实际 lease 分配。需求是缺失 cache leases + 当前 builder 实际新增的 local keys + 参数块1。同页 with_push 的资源 leases 已固定，因此新增需求只有参数块 StorageBuffer1；跨页要重新计整组。

不能每次用“完整组总数”直接比较 free：例如页有128个sampled slots，缓存64个global后 free64，而某组逻辑sampled66；若不扣已有cache命中，就会每draw另建页。也不能过度去重：目前 write_lease 没把资源放入 builder.cache，local resource 若与外部缓存 lease 相同，现代码可能仍新增一次；需求必须与实际 writer一致，或统一在新准备路径中显式去重。declared/private-upload 角色永不合并，FrameConstants b0/b1仍共用 declared lease。

新缓存先暂存在组事务。全部 index/immutable 参数块/Prepared 创建成功，才把完整 cache snapshot 发布；预先为 map publication 准入必要元数据，不能在已经发draw后才出现 fallible cache 插入。失败Drop新unused leases，参数ring允许留下未使用chunk空洞，不声称host写入可回滚。

缓存身份必须拥有实际页，例如持 Arc<BindlessHeap> 的 identity key 并按 Arc::ptr_eq 比较；不能只存裸地址。heap不能反向持有 cache 或强 Arc<BindlessLease>。FrameGlobals可以拥有该页key和leases，PagePool拥有PageRecord，PageRecord不得反向持FrameGlobals，否则形成环。

## 确定锁序与逐出

1. ShaderBindings.frame_globals 当前指针只在冻结快照时短锁，离开此锁后才选页；不在页事务中再次取它。
2. manager锁内只选/clone/摘取待逐出owner；不取group、Slots、cache、DynamicConstants锁，不调用 device.poll。
3. manager锁外取得 per-page group_mutex。组事务中缓存查找锁短取短放，不持cache map锁调用heap或dynamic。
4. 实际heap writer仅短取Slots锁；不跨 dynamic allocation、device.poll 或缓存提交持Slots锁。DynamicConstants只在该页事务中、Slots锁已释放时短取，writer结束即释放；持DynamicConstants锁的其他路径不能回头申请group/manager。
5. 元数据发布仍在group guard内；cache map提交时不drop旧lease或调用heap。若有替换/逐出，先取出旧owner，离开map锁再drop。
6. manager/cache逐出都在锁外drop实际page/cache owner（lease Drop会取Slots锁）。每callback cursor持PageRecord owner覆盖其with_push序列；不能逐出仍被正在准备/换push的cursor使用的record。

上述顺序避免 manager→Drop→Slots 与 group→cache→manager 反转，也避免 manager锁跨driver poll/常量写入。只在当前页确证不足时有界选择下一页，常见同页准备O(1)。

## 下一实现边界

root已委派 phase1 bindless.rs/tests/bindless.rs；phase2必要路径明确为 render_host.rs、frame_setup.rs、raster_host.rs、rhapsody/src/rg/indirect_count.rs，并允许必要通用PagePool位于rhapsody。所有源码仍在合法work候选，生产整合/Cargo/GPU由root独占。phase2须与上述所有writer一起交付，不能只交缩表或模板页新建即声称M3玩法恢复。
