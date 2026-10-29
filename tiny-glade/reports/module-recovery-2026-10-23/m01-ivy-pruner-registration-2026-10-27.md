# M01 所需 ivy_pruner 原注册调查，2026-10-27

仅调查M01编辑事件的必要消费者注册，不修改业务源码、不扩展到M12 ivy算法修复。先实际核对B skeleton的systems/ivy目录及mod.rs、ivy_pruner.rs、相邻系统文件；它们与当前既有路径一致。

## 结论

原ivy_pruner并非独立挂入编号22的set；它属于Update中的完整ivy链。原22是spawner的 `after_ignore_deferred(GameSystemSet(22))` 依赖值，不是系统数，也不是 `in_set(22)`。原C中无参的chain调用隐藏了Rust零大小函数item tuple，不能据此认定只有spawner。

注册链及每一层的原证：

1. `run_game_thread` 在 **14059f79a** 调用 `add_ivy_systems` **140944070/72B**。
2. 140944088写DL=0x16，14094408a调用 **141664d80/535B**；原PDB完整泛型和静态name证实它只构造 **ivy_spawner.after_ignore_deferred(GameSystemSet22)**。其内部 `after_ignore_deferred_inner` 可见，没有in_set。
3. **14094409a** 调用 **141450fa0/1672B** 的 `chain_ignore_deferred`。原PDB的第一个类型参数是真正的**8节点tuple**，而非单ScheduleConfigs。构造器按下表顺序写入8节点、最终len8，并调用 **1416a00a0** `Chain::set_chained_with_config<IgnoreDeferred>`。
4. **1409440a5** 调用 **140b67280** `SubApp::add_systems<Update, ScheduleConfigs>`。72B完整原范围没有run_if或额外in_set包裹。

| 外层节点顺序 | 原业务 | 原函数VA | 构造器/证据 |
|---|---|---|---|
| 0 | ivy_spawner.after_ignore_deferred(GameSystemSet22) | 14094cf90 | 141664d80 |
| 1 | ivy_grower | 14094a910 | 141451073→140a9f410；FnSystem名142adc21a |
| 2 | ivy_leaf_spawner | 140996080 | 14145107f→141664930；名字142c8343a |
| 3 | **ivy_pruner** | **140997410** | **14145108b→1416643e0**；名字**142c83335** |
| 4 | ivy_pruner_cmd | 140999330 | 141451097→1416645b0；名字142c83478 |
| 5 | ivy_branch_thickness_animation | 140995a80 | 1414510a3→141664b00；名字142c83367 |
| 6 | ivy_leaf_animation | 140b0e380 | 1414510ac→140a263f0；名字142aca7e8 |
| 7，内部二元tuple | ivy_send_to_gpu_segments、ivy_send_to_gpu_leaf | 140a9b4e0、1408c1580 | 1414510b8→1417d0940；名字142ce44c1/142ce447b |

即外层8节点，实际9个业务系统。尾部两GPU系统只有普通tuple `into_configs`，其Group2初始chain为空；没有把segments→leaf另造内部顺序。前7个节点到GPU二元组保持outer chain_ignore_deferred。

## ivy_pruner身份和条件

从原函数140997410反向的共享只读xref：原run_without_applying_deferred **1414bf4e2** 和run_unsafe **141662b50** 都调用相同业务；它们都是执行包装，不能当成注册父函数。

真正注册追踪以原FnSystem.name开始：原RNE字符串 **142c83335** 为 `country_core::systems::ivy::ivy_pruner::ivy_pruner`；xref **1416644a7/1416644ae** 进入 **1416643e0/453B** 的into_configs；该构造器唯一代码调用点是 **14145108b**，落在上述8节点chain中。原机器码逐构造器静态name核实了表内每项，未按资源签名猜成员。

pruner原参数7项：ResMut<IvyStorage>、ResMut<IvyLeafStorage>、Res<PublicWalls>、Res<WallSpatialHash>、Res<PrevWallHoles>、EventReader<CmdCheckForSheepInsideBuilding>、EventReader<OnCircleRotated>。配置构造器没有附加条件向量或mode/AppMode参数；原编辑触发来自两个实际事件。这里仅确认**该注册链自身没有run_if**，不把此结论扩大成整个App的全局加载状态策略。

## 资源、事件与卸载边界

- **insert_ivy_resources1409440c0/437B** 的唯一已观察代码caller是run_game_thread **14059f004**，向App插入IvyStorage、IvySpawnNoise、IvyLeafStorage、IvyLeafRng；不是每帧创建第二套资源。
- **add_ivy_events140944280/299B** 在run_game_thread **14059f7e1**，依次注册IvySegmentDoneGrowingMsg、IvyLeafIsGrowing、RemoveIvyCmd。M01消费的Sheep/Circle事件是原跨域同一身份，不应另造OnWallChanged代理。
- 以上三个mod.rs函数完整B范围均未注册OnExit、Unload或自定义session-clear系统；pruner本身无自定义Local，只有Bevy管理的两个EventReader状态。不能为凑“卸载完成”添加推测的清空顺序。
- 本轮**没有证明原整个会话切换时如何清空ivy存储/GPU缓冲**，也不宣称这一项闭合。startup资源插入和Update注册不等于session unload协议。若游戏恢复验收要求跨会话ivy清空，需在M12依据原重播/清场及GPU owner继续取证；M01这里只恢复已证编辑事件消费者拓扑。

## 当前宿主建议与范围

root可以按已证关系补入真正pruner及既有pruner_cmd消费者，至少保持 **leaf_spawner→pruner→pruner_cmd→branch_animation→leaf_animation** 的原方向；完整链起点在GameSystemSet22之后。不得把pruner随意放到before leaf_spawner，不得把after关系写成in_set，不得以chain()默认插入deferred屏障替代原chain_ignore_deferred。

当前registration.rs的旧“22系统”注释错误；`add_ivy_systems`旧 grower→spawner→pruner→cmd→leaf 顺序也与原不符，CPU helper旧cmd→leaf亦不符。它们需由root统一改动，本调查没有抢写共享业务文件或自动扩大整个ivy模块范围。

## 证据和失败记录

原RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`，B PDB SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。原B module1641 pruner into_configs、module1604 outer tuple及9个业务的原PDB范围见 `m01-ivy-tuple-signature.json` / `m01-ivy-b-source-functions.json`；原字节LLVM反汇编与静态name映射见同前缀ASM/JSON，原C留对应VA文件。

所有MCP调用为 `--worker shared` **read-only**，没有改共享分析库。`m01-ivy-registration-native-reads.json`保留实际nested结果；一次get_functions误用Windows adapter字段body被当前Mac拒绝，后续按返回的body_start/body_end改正，不能把该失败报成成功。调用同名标准库单态化函数，必须读取完整泛型及原构造器的成员，不能只看C省略参数的函数名。
