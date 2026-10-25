# B base producer 注册及 pre-assembly 同 owner 初始化合同 V1

状态：原注册/当前调用边界只读恢复 + 现架构实施提案，待07/root审。没有 source candidate、production mutation、Cargo/rustc/tests/benchmark/GPU。不是完整原九成员 WallConstructor chain、set25、core/App 通过。两geometry V2另冻结，不先行声称可达。

## 一、原注册闭合：base确在28，assembly在25

复用原 `app_systems::add_wall_systems@140493120/1413B` 完整原PE包与 GameSystemSet vtable身份。14049338e调用 `chain_ignore_deferred@14033def0/2586B`；393写raw0x1c，3a9 interning，3b8 in_set，414 addUpdate。新完整2586B pack和 exact B PDB inventory row已保留；正确解析第一tuple有9项（不是泛型末尾marker项）。zero-based slot5通过 call14033e263→into_configs140002c90/421B 构造。后者140002d43引用原142925835长0x39 UTF8 **system_wall_constructor::construct_walls::construct_walls**；原字节与typed参数双重绑定，不凭同签名猜名字。

slot5真实参数为：(PublicWalls,AssetSsboLibrary,Query<&Roof>,TerrainHeightsData<TerrainHeightSsbo>)；ResMut<WallEntities>；ResMut<WallHoles>；ResMut<RtWorld>；Query<(&mut InstancedWall,&mut WallTriggerVolume)>；OnWallChanged reader；RecalculateWallBricks reader；UpdateRtWorldEvent writer；Commands；RecalculatePlasterCmd writer；RecalculateHalfTimberMeshCmd writer；UiSignifierStream；(DecoratorStorage,DerivedDecoratorInfoResources)。分组/原字段布局未知不妨碍身份绑定。当前core glue仅恢复其CPU brick+trigger/RT子链，不能冒称原全部参数业务。

02/06新phase证明 DecoratorPlugin344fae构造fullvisual→middle4→copy链，作为outer19成员的member14 pair第二支；3451af outer chain，3451b4 **raw0x19=25**，3451e2 in_set，345253 addUpdate。旧PRODUCER_BLUEPRINT_DESIGN中的“fullvisual/copy在19外”文字已过期，必须以02 set25-registration-confirmation-v1替代。middle4其余项标签另有02/07更正，本合同只绑定已证Chimney/Door第2/3项，不借旧标签注册空节点。

当前actual run_game已表达GameSystemSet0..44 pairwise before；所以现正确原阶段关系是 publication19 → assembly25 → base28 → upload44/现PostUpdate维护。不能把base28挪到19，也不能在28建deco handles就宣称它们先于25存在。

## 二、原publisher allocation与失败域

原publisher140a8b530/18515B已全PE核：成功原构建路径19取同BrickTransform存储并建立四个Brick allocations，只有第三/第四精确追到Chimney/Door的PublicWallState value+160/+170（bucket+168/+178）；第一/第二不作标签外推。原builder与两assembly没有第二BufferHost。

补充原旧row分支：140a8c7ce是HashMap::remove，返回0x2a0旧row到stack；c838..c848搬到840，ca0d..ca55逐7个 Sparse allocation drop，继续共享process/ce7d取同库及四allocation建立。symbolization保留明确remove层，不把该helper误当动画测试。此为已进入有效重建路径；不能外推invalid-source guard也会drop旧row。当前PublicWalls::consume_with_paths在curve<2或length<=0前置拒绝时保旧row、无Changed；collection失败也无新Changed，path准备失败保实际published row并记录path_failures。这个保旧CPU投影策略与native panic/alloc失败不可混为同一证明。

原19 record内alloc可以先于base28的ECS实体。现三lane同entity/InstancedWall是有界架构映射，**不是原内存布局复刻**；保持已有同owner的deco handles直到25真实dirty rebuild即可，不必在每次publication recreate三个Box。base.update更不能清deco handles。

## 三、当前可见真实调用及缺口

- actual run_game::install_history_and_wall_runtime的set19现只有调整/发布/删除/空间/segmentation/height子链，没有base producer；set25现只有unleash。`construct_wall_bricks`只在旧core add_wall_cpu_systems和外部 add_wall_constructor_systems入口注册，actualApp未调用这两个旧入口。不能用函数存在代替接线，也不整体安装它们（会重复现publisher等并激活未恢复业务）。
- currentcore::construct_wall_bricks先真实build_wall_bricks、trigger.recompute成功，才update实例/RT并发布WallBricksChanged。existing value.update只改base Vec/upload_pending，不换allocation；new branch成功后Commands.spawn(InstancedWall::new,realtrigger)，立即登记唯一WallEntities。失败写BuildState并continue，不发布成功event。
- currentcore::geometry::sync_wall_bricks在PostUpdate同AssetSsboLibrary取真实Brick存储，回收waste、处理basepending、try_gpu_flush。productionPreStartup先通过GraphicsDeviceResource、同Library的init_populate_ssbo_library首项注册canonicalBrick。CoreRenderBridge::install_asset_libraries同HostTables同时安装GraphicsDeviceResource/AssetLibraryRuntime，拒设备替换。本链无需新库/new BufferHost或改Library/Sparse算法。
- WallEntities的已见非test消费者仅baseconstructor、delete_wall、wall_highlight以及hostunload；world_walls按highlight内entity找InstancedWall，要求base allocation Some。独立deco lanes保持globalnormal；highlight只hide/show base。没有别的已见实体consumer允许假造Trigger。
- CPU registration/geometry runtime现有tests用legacy InstancedWall::new，或CPU-only wall fixture没有SSBO owner。直接增加requiredLibrary参数会使这些fixture失配，必须同批真实适配或保留有名字/边界的旧CPU接口；不能Option<Library>缺失时偷偷走成功空owner。

## 四、推荐最窄现映射（待授权后候选）

1. 在真实19 publication成功之后、delete_wall移除旧映射之后，由一个明确的 **publication render-owner producer** 读取同OnWallChanged、PublicWalls和同AssetSsboLibrary。只对PublicWalls实际存在且WallEntities无entry的ID，在该**同映射**Commands.spawn一个pending-base InstancedWall；创建三个真实owned empty allocation（base/chimney/door）都来自同Brick存储并保存既有canonical glade/session marker。不是assembly消费端spawn，也不是第二wall registry。已有entry不重置任何handle；同帧重复Changed因map已登记而不双分配。禁止全表每帧扫描，工作量只随实际events。
2. pending-base状态必须可区分 `base尚未构建` 与 `合法empty base构建成功`。可用明确bool/enum与`has_base_geometry`访问器，pending时upload_pending=false且没有WallTriggerVolume；不能用Default trigger或把Vec::new当成功几何。三个alloc只是空owned handles，不领取chunk。建议新factory `new_published_in(id,&actualStorage)`，保旧`new(id,Vec)`语义给未启用assembly的旧fixture。最终命名/字段由owner审，不在本合同写伪实现。
3. 实际19→25必须经过Commands应用；使用已锁Bevy的真实schedule可见性并显式验证。若添加ApplyDeferred，需放在完成19 producer/delete后、25前的明确边界，不能擅自在native member14 chain_ignore_deferred内部加flush。原全局stage边/auto-insertion效果需单独原/源码证明，不能把现提案显式barrier冒称已见原显式barrier。当前source只已见44的明确Nani barrier，故19→25是待真实运行验证门。
4. 原base阶段28注册**仅**当前有原业务的base glue；不整体装9tuple或add_wall_cpu_systems。pending owner的query须兼容尚无Trigger：先成功计算真实trigger和brickVec，再插入该trigger/update同component及RT/event。失败保pending、无Trigger、无base成功event；已成功旧owner失败保旧base/trigger。现代码Query强制两component将把pending视为stale，必须修正，不能靠造Default Trigger满足query。base new branch在孤立无assemblyfixture仍可于真实构建成功后通过sameStorage factory创建，同真实API，不让assembly偷做fallback。
5. 正常25两个host从WallEntities取同entity，再取同InstancedWall实际lane；PublicWalls存在但map/component/lane缺失必须显式失败，不等价于PublicWalls miss，不consume假空日志。原清lane一次→遍历全部current构造→Removed真实audio/明确Door no-op→normalize。共用library get_mut、原partial NoFreeChunks语义、同waste回收和统一PostUpdateflush不变。7/4资源/DerivedView/audio原业务仍由owned assembly后续source审。
6. base28请求inputs需合法初始化：WallEntities、WallHoles、WallBrickBuildState、WallBricksChanged、RecalculateWallBricks及当前WallBrickRequests实际读的gable/presence/geometry事件；RtWorld/UpdateRtWorldEvent当前actualApp已装。现glue把original内联finalize拆到全表finalize_wall_holes，恢复接线不能漏finalize或无证每帧扩展全表pass。优先让base按实际requested wall、原WallHoles.finalize合同完成后才build；这需要原owner对最窄ResMut/请求工作集delta审查。全系统corners/skirt/plaster/half-timber/signifier剩余项明确未恢复，不靠空callback注册。

## 五、失败/删除/可见域

| 条件 | 必须保留的结果/门 |
|---|---|
| startup缺sameLibrary/Brick | 原资源必须项失败；不得消费diff或创建第二库，也不调用新device代替 |
| invalid-source或collection未成功publish | 无新的Changed，birth producer不造owner，已有renderowner保留；不借此称全部native失败等价 |
| path_prepare/base生产或trigger失败 | BuildState保真实error；pending无Trigger/无base成功，新deco可能已在25更新而base28失败，混合版本是否允许是**当前故障域**，不能说全framefailstop；需root决定现错误传播门 |
| PublicWalls存在、renderowner/alloc缺失 | host显式错误/原strict失败，绝不按miss消耗日志 |
| actualPublicWalls缺失 | 原明示miss分支可只消费removals；不泛化为资源/owner缺失 |
| NoFreeChunks | 每wall先clear，后partial push保留，继续全部；日志仍正常归一，原flag未read，无自动重试发明 |
| device flush失败 | 同共享dirty保重试；raster gpu_ready gate不能外推W21/W27 compute整帧停下 |
| normal wall delete | 同WallEntities.remove→Commands.despawn→apply→component三handles Drop→同waste→下次take_out_waste；CPU tracker.on_wall_removed只清日志，不替代GPU释放 |
| same-frame deleted再created同ID | 删除旧map后birth新entity；重复Changed不双建；真实Commands生效顺序必须测，不能先由base查询保留reserved entity造成stale |
| session unload | actualwalls_on_glade_unload重置map本身不despawn。另registered clearing::despawn_glade_entities的真实marker/当前缺源必须由owner交接；newrenderowner须同canonical glade标记并在OnExit实际despawn，不能仅map.clear声称三alloc回收 |
| CPU legacyfixture/otherApp | 保旧new入口明确无assembly；任何启用真实host的fixture必须同canonicalstorage/真实resource，不能mock Brick/writer/terrain；第二World单独owner不串表 |

## 六、需要root合法委派的文件/任务

已live复核原claims（rawshow附本目录）：InstancedWall TG-a738caa535351a7991 owner frameinputs-20261018；原construct_walls TG-1a3550f643b9278be7 owner main-chain-frame-20261018；publisher TG-3594e8acede6203727 owner main-chain-boot-20261018；Library TG-3f1019bf28c1d470d9 owner render-ssbo-lease-20261018。不根据旧work目录推live锁。

最窄潜在source范围：country-core geometry/instanced_wall.rs（三lane/pendingbase factory）、geometry/runtime.rs（publication-owner桥；现flush只需确认directlane dirty仍flush）、systems/wall/construct_wall_bricks.rs（当前glue映射/真Trigger/请求finalize）、tiny-glade/run_game.rs（所需资源events、set19桥/可见边界、原28单producer注册）、既有canonical glade marker实际owner接口。geometry/runtime和coreglue为现新文件无独立native任务，需root明确companion委派。Library、Sparse、PublicWalls结构、world_walls/drawlist无需改（已有实际API足够）；本worker不申请泛化这些foreignowner。若publisher函数本体需嵌桥而非邻接真实producer，须root旧publisherclaim另授权。

## 七、必要验证提案，均未执行

在same-source/realcurrentApp范围核：19 birth之前/之后与25实际host query可见；reserved entity不能提前被query；原28成功发布realTrigger/RT/event；existing.update保持3handle身份；newbuild/trigger失败pending与oldowner保留；同帧Changed重复/Deleted+Created；normaldelete和OnExit三handles实际waste；真实lane clear+超过一chunk的逐砖payload与partial容量失败；PostUpdate真实dirtyrangeflush/设备失败保dirty；basehighlight与deconormal分开。不能只数函数次数或用unregistered纯helper当App成功。

当前core全编译受已知缺源约束；新的限定验证若不能精确含canonical定义+完整候选/actualSparse实现则保未验证，不造mock或用旧rlib冒充。root统一执行，worker不会跑Cargo/并行benchmark。
