# M01 set25 parent19 的 slot1 / slot3 实际注册树

2026-10-27，只读追原 plugin build140342c10、parent1400d52f0 的配置搬运，未修改共享宿主注册。恢复原则是保留配置数组和原 chain，而不是按构造器创建时刻推测执行顺序。

## slot3 是六成员 chain

caller `140344f21` 调 `140287410`，输出暂存 RBP+2b0。`140345126..181` 将它搬到父输入 RBP+400；parent 的输入第三配置对应+e0，经入口RBP+8d0，最终 `1400d591d..5978` 写输出数组+150，即 slot3。原完整 PDB 构造器1658B，旧 Ghidra body1249B/短LLVM片段1291B都不代表完整函数；本次新native pack保留1658B。

| 子序号 | 原 leaf / ctor | 原真实合同摘要 | 本轮 owner |
| --- | --- | --- | --- |
| 0 | maintain_stair_graph140806110 / ctor140285ee0 | History、PhysicsColliders、Storage、Archivist、StairsState、StructuralGraph、RaycastWorld、StairColliderRegistry、DeferredEvents 共9参 | hints |
| 1 | cull_stair_attached_decorators14080ec10 / ctor140105110 | History、Storage、Archivist、DeferredEvents、Blueprints、StairsState、ArchivalResources、CachedTransforms 共8组 | height |
| 2 | generate_playermade_stair_requests<false>140276e80 / 父内inline | Storage、CachedTransforms、PublicWalls、VisualTerrainHeights、StairsState、Requests、Signifiers；现代adapter同一Demo owner | move_shapes |
| 3 | playermade_stairs_interpolate_and_compare1407cf500 / ctor1400290b0 | Requests、GameTime、StairsVisualState、RemovedSupports、Storage、Signifiers共6参 | move_shapes |
| 4 | cull_stair_attached_decorators_on_invalid_stairs14080f040 / ctor140104d90 | History、Storage、Archivist、DeferredEvents、Blueprints、Requests、ArchivalResources、CachedTransforms共8组；与子1的第6项不同 | height |
| 5 | cache_decorator_transforms / ctor1400c6200 | 既有完整9参数cache producer | root |

顺序依据：14028745e构造子0写RBP+160→output0；746a子1写+f0→output70；父内子2写+80→outpute0；75fe子3写+10→output150；7607子4写-60→output1c0；7613子5写+1d0→output230。`set_chained`将该六元数组标为普通chain，不是chain_ignore_deferred。

两个不透明ctor的身份同时由发行字符串确认：140029174指向1429274dd，94B内容就是interpolate_and_compare；140104e57指向14294e022，87B内容就是on_invalid_stairs。未根据SystemParam相似性猜名字。

parent slot2还有一次cache，slot11还有一次maintain graph，slot12还有一次edge cull。必须保留原多次调用位置；不能因相同函数名而删掉。原第一次cache为请求生成提供输入，slot3尾部cache消费更新后的请求/插值。

## slot1 的旧第二叶命名错误

原slot1配置来自 `14026c790`，普通chain恰好两叶：

1. ui_convert_autoclutter_to_decoclutter14099ca60。原typed资源以现有clutter/RMB/UI系统为权威，由rectangle负责完整恢复。
2. **continue_stairs14079c3a0**，ctor**1400b1230**。发行 `1400b12f1` 指向 `1429344da`，长度0x3d，字符串为 `system_decorator::ui_systems::ui_stairs_misc::continue_stairs`。原B确有该路径。此前“apply_stairs_op_events”是审计命名错误，源码无该函数不能直接视作缺文件。

continue真实3参是 AppMode、ResMut<StairsState>、Res<DeferredDecoratorOpEvents>。核验时当前手工版本却借Dispatcher/MoveInteraction/Backup/StairsState并手造raw事件端点，已报中央由其在原既有ui_stairs_misc.rs定点恢复；本owner未与中央交叉修改。

## 注册审计快照发现的缺口

maintain_stair_graph在核验快照仍是3参空体；两个cull包装仍5参且各自新建本地DeferredEvents，原8组History/真实事件/ArchivalResources未闭合。以上已按原真实合同分给各owner，不将“配置树查到”算成“算法和接线完成”。中央后续实现是独立变更，读取本报告时应看最终源和对应测试，不拿此中间快照覆盖后续修复。

请求和插值本体已经具有大量完整算法及真实World测试。本owner随后按原30178B请求、8622B插值进行复核并增加原128向量，见 `m01-set25-stair-dependencies-recovery.md`。这些验证不覆盖独立楼梯装配中仍标U的支撑/砖/订阅函数，也不替代M08业务收口。

## 原始依据

现目录 `m01-decorator-set25-parent.asm`、`m01-decorator-plugin-registration.asm`、`m01-set25-slot3-chain-full.{json,asm.txt}`、`m01-set25-slot3-interpolate-ctor.asm`、`m01-set25-slot3-invalid-cull-ctor.asm`、`m01-set25-slot1-continue-ctor.asm`、两个只读native查询json。PDB完整typed原树另见既有 `reverse-engineering/tiny-glade/skeleton/task-index/work/TG-10dd22b74858bd4c2e/decorator-chain-root-20261021/visual-host-unload-20261021/SET25_NESTED_TUPLE_MEMBER_TYPES_V1.json`，本次依然独立核原指令搬运与名字。
