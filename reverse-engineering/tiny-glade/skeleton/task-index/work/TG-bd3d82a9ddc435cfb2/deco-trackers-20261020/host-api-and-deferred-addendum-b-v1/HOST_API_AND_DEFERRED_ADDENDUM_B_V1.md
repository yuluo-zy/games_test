# Member14 B 前置 owner / 实际 host 最窄 API 与 deferred addendum V1

此文追加 base-owner-init-design-v1（e9dcac8e/3a93759d）与几何V2（76e4f703/2f2e211e/a446faf1）；不是 foreign source 候选或授权。A run_game87fe 已由root整合，receipt validation/base-producer-a-integration-v1.json；仅原set28基础砖section实际挂载。B仍另批；不再采用旧“只requested wall finalize”建议，A8c7235a7已明确撤回。纯几何V2 source准入f223e2b5不等于5 tests已跑或实际writer/host闭环。

## 一、19正常 publication 之后的最窄生产者

当前 PublicWalls.consume_with_paths 每次开头重置 last_delta；只成功collection并插入当前record的行加入现有 last_delta.changed HashSet（重复created的原panic不算正常成功）。它还保留真实ordered events。适配的 owner producer 可在现set19链的 publish_public_wall_state、delete_wall 后借这份 changed集合逐行工作，不扫描所有墙、不收另一份event去重HashSet，也不改变publication事件顺序/数量。这个位置是现架构适配，原B在publisher内部先建allocations；不声称还原原PublicWalls全部布局。

- 从同 AssetSsboLibrary.get::<BrickTransformSsbo>() 取得已注册 Sparse storage，缺资源/entry必须显式错误或panic；不从consumer新建另一个BufferHost/typedbuffer。
- 同 WallEntities 唯一WallId→Entity。新墙只 Commands.spawn 一个持同storage三个真实empty owned allocations的 InstancedWall；base状态显式 AwaitingProduction，尚无WallTriggerVolume，不发WallBricksChanged/RT成功，不以空Vec当已构造。
- 已有真实owner只验证/保留handles。A/旧public API生成的legacy InstancedWall尚无deco pair时，若允许跨批迁移，须由**这一真实producer**调用显式legacy-upgrade方法（只建立缺失的pair和必要empty base handle，保留base pending/current状态及已存在allocation）；不能在assembly缺lane时偷偷修复。该兼容分支需要root/07在候选前选择准入，不能假称是原B缺handle行为。没有此迁移时应把existing legacy rows作为清楚的B接入门，不能按PublicWalls miss消费日志。
- 两deco handle放同一个私有 pair，避免只初始化一半。storage identity通过现 owns_allocation检查；未知/foreign allocation保失败，不重挂新设备。只第三/第四原alloc已证明Chimney/Door，三lane是当前base+二deco有界映射，不命名原未知第一/第二allocation。

生产者只在实际publication发生时运行正常工作。已有PublicWalls被外部fixture直接灌入、未经过publication的行没有normal初始化证明，不能扫全表假装原启动成功；测试/调用方要明确使用真实publication入口。

## 二、InstancedWall最小状态/API变化（待合法owner）

保持 `new(id,Vec)` / `update(Vec)` / pending_instances / allocation 现外部调用；新增显式state `AwaitingProduction | PendingUpload | Published` 取代private upload_pending布尔，仍复用同brick_transforms Vec。`new`为PendingUpload，前置owner构造为AwaitingProduction，`update`变PendingUpload；is_upload_pending只匹配PendingUpload。AwaitingProduction upload返回false，不做空更新finish，不表示base成功。真实upload成功才Published，容量失败保PendingUpload/旧base。

建议新增 `new_awaiting_production(id,&SparseInstanceBuffer<T>)`、只读 `is_awaiting_production()`、`chimney_allocation()`、`door_stairs_allocation()`。新构造一次建same-storage base和deco pair；update只改base pending Vec/state，不替换三个handles。legacy-upgrade若采用必须具显式命名/调用和状态测试，不从assembly writer fallback。

28现构造query改为 (&mutInstancedWall,Option<&mutWallTriggerVolume>)。existing AwaitingProduction的None trigger只在build_wall_bricks和真实trigger.recompute均成功后插入真正Trigger、update真实base并发原RT/event；非AwaitingProduction却缺Trigger仍StaleWallEntity错误。失败不发成功、不造Trigger、不清deco。现无owner分支能否继续供旧CPU-only调用，须和新增19producer的normal域分开列测试，不用晚28初始化证明早25可用。

## 三、deferred 可见的实际源码依据与待执行边界

actual run_game304..308对0..44所有GameSystemSet建立普通 `.before(next)` 边，19→20→…→25是同一Update图。当前源码全查无auto_insert_apply_deferred关闭/自定义ScheduleBuildSettings。Pinned Bevy0.16 schedule.rs2041..2048默认true，config.rs130..137的before不带IgnoreDeferred，328..337文档亦明示后者可见Commands；chain_ignore_deferred只给内部链edge添加IgnoreDeferred（261..266）。

auto_insert_apply_deferred.rs141读取has_deferred；149..160只有标记no_sync的edge延迟同步；后续未忽略的edge增加sync distance并在193..215插入ApplyDeferred。collapse_set216..255处理空sets，仅前后**均**no_sync才继承忽略。因现19→25跨set普通edge未标记，方案的Commands应在进入25前经公共builder同步，不需要member14内部额外flush。

这是固定依赖和当前配置的源码推导，**没有**运行实际App/打印最终schedule graph。候选后中央仍需以真实set配置+实际owner/source限定harness验证新entity、同ID删除重建及deco getter在25可见。不能用只看raw19/25/28数字或手加ApplyDeferred断言原链通过。现28→PostUpdate uploader既有end-of-schedule flush与此跨set证明不同。

## 四、7/4业务输入到同owner writer

原PDB已实证Chimney七项：DerivedResources；PublicWalls；Library；Tracker<Chimney>；TerrainHeightsData<TerrainHeightSsbo>；UiSignifierStream；EventWriter<PlayDecoratorSoundCmd>。Door四项：PublicWalls；Library；Tracker<Door>；UiSignifierStream。当前架构将allocations放Entity而不是原Public value，所以需要额外borrow同WallEntities/Query；这两项是明确适配输入，不将参数表面arity当等价证据。

最窄 shared core geometry/runtime SystemParam可聚合 `ResMut<AssetSsboLibrary> + Res<WallEntities> + Query<&InstancedWall<BrickTransformSsbo>>`，给两owned assembly源返回借用的chimney/door writer。只读Query足够：原allocation的chunk mutex由现storage.writer借用取得，不需把InstancedWall可写或转共享Storage到可写。getter核owner.id、lane非空、storage identity；缺owner/lane不是PublicWalls miss。该wrapper仍是同库，同CPU shadow/dirty/free/active/waste和同PostUpdate flush。

两host先public.get(wall)：真None才明确into_removals+已证空分支；真Some则取严格owner writer，clear，再在借用log的map次序中逐个current调用几何V2实际write_*_instance。NoFreeChunks累计flag，继续原push和sideeffects，不replace_all，不据flag发虚构signifier/retry。原Chimney with_removed@1412af077早于mutex unlock@af087..af09f；writer guard须保留到该同wall removal/normalization结束，不能延至后续wall或跨帧。容量partial后日志仍normal化，失败不自行重入dirty。Door removal空callback仅作为已证Door分支明确写出。

Chimney每项真实音效按原diff与当前/旧tag：Added tag0→kind19；Changed old非0→current0→kind19；Changed old0→current非0→kind20。position来自当前+8/+10/+c。Removed借**已合7506ddc7** DerivedResources::as_view/From，对真实DecoratorId try_get_subtype；Some firstbyte0x14才kind21，position来自旧参数；None不发。Changed不是Removed。此功能只需真实subtype lookup，不依赖anchor尚未恢复collision表。

canonical `country_core::systems::audio::PlayDecoratorSoundCmd{kind:u8,position:Vec3}` 已存在。actualtiny-glade源尚未注册其Events，也未调用core add_audio_events；已有PlayClutterSoundCmd不是替代。B实际mount需App.add_event唯一canonical类型，保真实EventWriter。事件输出不意味着现partial decorator_audio系统/NativeStudio端到端音频全恢复。UiSignifierStream已由actual gameplay_startup189初始化；原capacity flag未被host读取，不新增UI消费。

## 五、owner路径与实施边界

外部候选须root正常委派，当前未写：geometry/instanced_wall.rs（TG-a738caa535351a7991旧claim）、geometry/runtime.rs（同host companion）、core/systems/wall/construct_wall_bricks.rs（TG-1a3550f643b9278be7伴随现glue）、actualrun_game.rs（TG-c459c7f73345e24709老claim）。19producer可放同geometry/runtime.rs，避免改foreign PublicWalls/Storage；Library/Sparse/world_walls无需新owner或新算法。此处列的是需要root授权的源，不从历史目录推live锁。

owned两assembly source可在这些接口定型后替换协议入口为真实系统；原protocol回调如需兼容应显式命名legacy seam，不能将其注册冒充原系统。几何V2完整候选保持冻结，到同批源审时再带before/dependency hash。不因当前A已挂而宣B可达。

完整B的wallgeometry dirty/reanchor仍依赖真实九参数anchor：当前on_wall_changed无production caller，fullvisual整值Eq可retain，不可只靠已有fullvisual声称wall变更后重建闭合。原tailOnWallDeleted清tracker并不代替changed失效。anchor的Storage/prefix/collision门未解除。OnExit真实despawn_glade_entities缺源与二session验证也仍门；清WallEntities map不等于drop3allocations。
