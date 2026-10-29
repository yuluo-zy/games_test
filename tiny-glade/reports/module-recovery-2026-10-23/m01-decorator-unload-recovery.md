# M01 Reload 必要依赖：装饰物卸载

本轮仅恢复既有 B 路径 `crates/tiny-glade/src/decorator_plugin.rs::decorators_on_glade_unload`，不扩展其它业务模块，不修改渲染 agent 的文件，不创建项目目录。中央 agent 负责生产注册和完整 Reload/GPU 联验。

## 原始身份与边界

已实际读取 B 骨架该文件 864 行的 compiled_procedure 和 1443 行之后的 typed candidate；真实接口是两组 tuple，分别含 16 个 ResMut、4 个 ResMut 加 Commands。现签名沿用这些 owner，Archivist 采用当前手工重构的 `LiveDecoratorArchivist`，没有造第二份资源。

PDB `skeleton/_pdb/B/modules/00141.jsonl.gz` module141 record113876：VA `140347090`、`code_size=9451`、跨度 `[140347090,14034957b)`，包括异常析构 funclet。旧记录的 5395 B 是 Ghidra body 计数，不能当 PDB 函数完整跨度。正常返回 `14034852c`；源码行357–423。

`native-evidence-pack` 真实执行验证了原 RNE SHA、匹配的 PDB GUID/Age、2123 条原指令全部 bytes，以及5个源码采样。初次错误 PDB 文件名（写成 tiny-glade.pdb，实际 tiny_glade.pdb）被工具拒绝，纠正显式路径后 PASS；没有换版本或猜测身份。保留 manifest、report、原字节 JSON 与 ASM。冷运行0.817秒、4个工具进程（2个版本、2个分析），无目标程序执行；这些是证据收集指标，不是业务性能。

## 完整业务与清理顺序

原正常路径按以下顺序构造默认 owner 并析构旧 owner。Rust 赋值先求 RHS，随后释放旧值，保留标准容器/Arc 自身的异常和释放语义；不模拟 std/hashbrown 内部表的裸内存 ABI。

| 顺序 | 真实 owner | 行为 |
|---|---|---|
| 1 | DecoratorStorage | 整体默认替换：墙/地形/楼梯桶、removed、全局ID索引、immutable 全部复位 |
| 2–3 | DecoratorBlueprints、PrevDecoratorBlueprints | 两表析构并替换，释放蓝图 Arc 与其嵌套载荷 |
| 4 | LiveDecoratorArchivist | 归档桶、两碰撞快照表、culling许可整体替换，释放网格 Arc |
| 5 | DeferredDecoratorEditHistory | 记录、潜在存续集合、pending 全部复位 |
| 6 | DecoratorVisualState | cloth、持久/瞬时实体索引整体替换 |
| 7 | DecoratorIdGen | 复位到0；原ICF helper140002370为xor eax,eax，不能按符号名误解成组件clone |
| 8 | PlaceDecoratorInteractionState | 默认替换 |
| 9 | DecoratorAudioEvents | 两命令表和帧状态整体替换 |
| 10 | GlobalWindowLightsControl | 灯表和lights_on复位 |
| 11 | DecoratorPhysicsColliders | 两活碰撞注册表整体替换 |
| 12 | StoneColumnCollisionStorage | 整体替换 |
| 13 | DormerTiles | 析构稀疏 RoofTile 分配并进入同一缓冲退役队列 |
| 14 | WallAttachedDecoTracker<ChimneyAssemblyParams> | 完整替换，释放当前/旧值和dirty墙 |
| 15 | StairsState | 两图、dirty、endpoint tag3、checksum、backup标志默认替换 |
| 16 | StairColliderRegistry | 三边表与结构子段表整体替换 |
| 17 | WallAttachedDecoTracker<DoorStairsAssemblyParams> | 完整替换 |
| 18 | StairsVisualState | 完整替换，释放已建模的稀疏/atlas分配 |
| 19 | StairsStructuralGraph | `clear`；原14034849e保留图和六张表容量 |
| 20 | DecoratorAffordanceDispatcher | `clear`；原1403484b9写真实判别{6,0}，不把Default(state0)当成clear |
| 21 | Commands | 原1403484e0延迟移除StairsTopDownMesh；资源缺失合法，不提前立即移除 |

18个整体默认替换、两个原clear、最后一条真实 Commands。没有 World.clear、没有只清蓝图缓存、没有把未声明依赖偷偷默认化。

## 分离职责和生产阻碍

原9451 B函数及其callee没有逐实体despawn、没有RaycastWorld形参，也没有dispose_collider入口。实体索引析构不等于实体销毁；后者由同一关卡退出阶段的 `despawn_glade_entities`（Without<KeepOnLevelUnload>）负责。ColliderMeshInstance 是Copy查询凭据，不拥有全局Rapier世界；清登记表不等于清17槽碰撞世界。

独立代码检索发现，本轮实现前 `RaycastWorld::on_level_reload`（原1408d8710）生产未调用，只有方法和测试。已向中央 agent 报告：必须依据 collision plugin 原注册在正确退出时机挂载该 owner，不能给本函数强加 RaycastWorld 参数掩盖缺口。窄测试明确验证：本函数后mesh Arc仍被活world持有，原全局重载后才释放最后一份Arc。

当前 StairsVisualState 的原opaque实体槽仍未完整建模，StairsTopDownMesh 当前ManagedHandle是数值兼容句柄，资源移除本身不能作为实际GPU buffer已退役的证明。这些共享owner现存边界没有用假Drop或新资源掩盖；生产GPU资源联验由中央agent负责。

OnExit原注册条件与相对顺序交中央 agent/hints owner核验。本恢复函数不擅自更改build/run_game调度。

## 验证

命令：`CARGO_INCREMENTAL=0 cargo test -p tiny-glade --lib --offline decorator_unload_tests -- --nocapture`。tiny-glade依赖显式启用country-core/native-physics，不是cfg跳过的fallback测试。

测试创建真实20 owner、真实Rapier collider与网格、共享蓝图Arc、归档Arc、两个原tracker、楼梯graph+backup、灯和交互状态、真实RoofTile sparse allocation。分别检查原Commands延迟、稀疏退役队列、两份蓝图释放、ID重置、结构图capacity保留、无保留实体跨关卡残留，以及重复卸载/可选topdown缺失合法。实际执行 PASS：1 passed、0 failed、0 ignored，277 filtered，0.02s；原始日志 `m01-decorator-unload-tests.log`。两次测试夹具编译失败日志也保留（生产函数未报错；修正Buffer trait、真实RaycastMesh载荷、Sparse writer实参和单处map借用）。本CPU边界没有GPU提交或真实鼠标键盘输入，不宣称整游戏Reload/GPU验收。

## 可复用经验

1. 先读实际B骨架的泛型宿主参数；多tuple是原SystemParam边界，不按反编译void()猜签名。
2. Ghidra size与PDB procedure code_size分开登记。大函数含EH尾部时，只导出正常body不能称完整原跨度。
3. ICF别名首先读原指令；本例WireframePass clone_behavior别名其实返回0，业务身份是DecoratorIdGen默认值。
4. 默认替换、clear、drain、延迟Commands分别恢复，不能用一套清缓存概括。观察Arc引用计数和sparse退役队列能揭示只清索引的假完成。
5. 不给原未借用的owner强加副作用。全局实体和全局碰撞卸载缺注册，要修其原宿主，而不是在装饰物函数内越权清World。
6. CPU资源生命周期、Rapier查询世界、GPU真实退役是不同验收层次；报告须写清实际验证到哪一层。
