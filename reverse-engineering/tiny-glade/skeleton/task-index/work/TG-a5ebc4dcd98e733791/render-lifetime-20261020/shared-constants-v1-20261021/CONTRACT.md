# 同 App 计算/光栅常量 owner 复用合同 V1

## 边界与原证

原 B compile-main/compile-present 的 DynamicConstantsResource 全局 owner 与 FrameConstants 上传路径已记于现 frame_setup.rs 模块头（compile-present 0x140166736 后的资源取得/分配），scene_compute::with_env 已将 raster.dynamic_constants() 放入真实 FrameLibraries。现宿主 GraphPasses::new 却在每次 CoreGraphFrame::with_compute 另建短命 DynamicConstants。候选是修此工程接缝，不重造 rhapsody 分配算法。

当前 RasterShared.dynamic **已经是** Arc<Mutex<DynamicConstants>>，无需再改字段类型。保持既有 dynamic_constants()->&Mutex 借用 API 和全部 chunk 格式/声明。

## 最窄接线

1. RasterShared 增加 crate 内只读 dynamic_constants_owner(expected_device)；Arc::ptr_eq 验证其现有 self.device 与 session device 相同，然后 clone 已有 Arc。不得取得动态池锁、分配新池或借此跨 App 找池。
2. GraphPasses 增加 crate 内 new_with_app_constants(graph,device,&RasterShared)，先由上述 owner 方法核设备，直接用已有 Arc 构造现 GraphPasses。共用私有 from_parts 初始化 metadata，原公共 new 继续新建独立池服务独立消费者/测试。避免先调用new再替换造成无谓元数据分配。
3. CoreGraphFrame::with_compute 唯一生产入口改调用新构造器并传 self.bridge.raster；保同 graph、tables、session、scope、finish/失败标记和错误顺序。CoreRenderOwners 已保本 App 的真实 RasterShared identity；不加入全局registry，不按Device找别的App池，不合并loading/gameplay owner。

## 私有范围、GPU完成与取消

- 复用原 DynamicConstants PageRing：参数区段仍各自线性分配、范围互不重叠，提交后 use_mark seal，同一页只有CPU唯一所有权、无Weak可升级且所有真实队列完成后才能discard/reset；MAX_ALLOCATIONS_PER_PAGE=1000、32pages/8MiB、预算及失败原样。不得扩大容量、清页或 wait_idle。
- 声明期 FrameConstants、BDA、UniformBytes、staging 使用同一物理页时，仍按各 chunk 精确range导入，session复用同物理resource identity；新私有参数块不能重叠declared range。仍由真实 prepare_submission_states producer/revision/range检查拒绝，不把池复用当忽略冲突的理由。
- private upload / declared roles不互换，b0/b1全局declared lease仍共享；各chunk字节写入后按现owner留存，不能覆盖在飞或仍被CPU/cache引用的页。
- 取消不发布GPU点、不回滚已写入字节或重置共享池。取消只释放原declaration/callback/command owners；后续分配继续合法尾部或经原唯一性+完成条件reset。noncoherent dirty queue仍由真实device flush/forget路径维护，不手工flush/forget未提交的共享页。

## 锁序审查

- 构造器owner借用只检查Arc设备identity并clone，不获取dynamic/group/SyncCell锁。session.borrow取得device的临时Ref在原语句末尾结束。
- 录制参数：既有 page group guard → dynamic Mutex →（allocate/refresh时）device poll / resource submission gate / native_state。未增加持锁跨度。
- 声明上传：现session RefMut + graph RefMut → 短dynamic分配；分配锁在语句末释放，再写chunk/导入/增加callback。现路径未持dynamic锁进入bindless group准备。
- 提交持SyncCell gates期间不反取dynamic Mutex；device flush队列独立，不取DynamicConstants；因此没有新增gate→dynamic→gate环。共享池引入计算/光栅录制间互斥，限定在原每次线性分配与写参数段，禁止将锁跨draw/dispatch/callback持有。

## 验证与边界

- 已有 raw dynamic_constants tests 覆盖真实所有权/Weak/多队列完成/used-page seal/4096range cells保护/取消flush；不新增镜像算法测试。
- 既有 tests/core_render_bridge.rs 增量实际GPU regression：同一frame的多个 with_compute staging分配必须计入同RasterShared pool、共享同native buffer且range互不重叠；取消后同pool仍可用且旧chunk字节不变；不同App即使同Device仍不同owner。GPU执行由root。
- root 跑所有 bridge/render_host相关现测试与真实原世界 probe，保 privateUpload 与 declared range全验证；观察同pool pages/creations/reuses、物理cell数、回执和内容。
- 30帧内存/描述符断言不变；不先改GC预算或按本设计声称retirement backlog已修。真实pending/completed-drain另采样区分。
