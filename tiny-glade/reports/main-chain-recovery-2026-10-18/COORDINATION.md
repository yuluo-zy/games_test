# Core 与真实宿主接口交接

2026-10-18，core systems/resources 恢复 owner 更新。共享源码持续并行变化，测试结论须对照各自源码指纹。

- 当前 core 收敛范围：History start/finish/mutable 生命周期、真实 UI permission、ModeChangePublication 调度消歧、普通 draw_wall/ui_draw_wall/finish_wall 候选、墙旁树隐藏及删除历史。保留现有手工架构。普通画墙尚未合入，树链尚未接受。
- core-native-r7 已通过：2365 库测试、1 真实编辑调度测试、3 羊快照测试，source_drift=0。包含首次真实 UI → start → record → release commit → 下一帧 Control Sync；不替代真实宿主 intro/History→墙/GPU 重验。
- 原模式 publisher 在 Input 与晚期各注册一次，用 `.after(send_mode_just_changed_event)` 会触发 Bevy SystemTypeSetAmbiguity。新增零状态 SystemSet `ModeChangePublication`，仅标记晚期 publisher；replay_mode、terrain focus、HistoryCursor 改依赖该 set。真实 run_game 晚期现有 publisher 同步加 set，未重复添加系统。
- 系统 prefab 的完整 PlacementData 热更新修复已合入；helper 现接受完整48字节 PlacementData，局部生成现有 PrefabTransform 给 RT/audio，保留 GPU id/custom/animation/bend 与 cull position。请保持该输入契约。原 resolved-row/callee 128 执行与独立 review 已保存于 TG-d44984ec9eec9a0bde/core-prefab-adapter-20261018。
- 已收到 startup/load_prefabs、resources/prefab 的原证纠错交接。请保留候选；core 将在当前系统 prefab 上增量整合 systems-prefab-consumer.patch，不能用旧 Transform 默认记录回退完整 placement 修复。audio exact Tree、独立 bounds、Mesh Arc/subset 必须按原证修复；尚未接受 reload 全链。
- 墙输出共享事件已纠正：RequestWallMaterialAudio `{id, position, kind:u8, flag:u8}`；HideTreesNextToWallCmd `{wall:WallId, finished:bool}`。后者原 EventInstance 共24字节，中心/半径由树位置和0.4构造。
- 树链新发现：PrefabInstance 是48字节 placement，而名字属于现有 PrefabInstanceData；旧 Option<Arc<String>> 猜测需最小纠正。正在调查 runtime storage index 与 serde variant id 的分离，不重做整个 prefab 模块。后续将记录准确 hunk，避免与资源 prefab 候选互相覆盖。
- 本轮 core Cargo 已结束；后续仍单个构建并使用共享锁。宿主 GPU/OS/render、rhapsody 工作留给 main-chain owner。GPU timeout/reprojection 不由 core 提交近似修复。
- 已收到 main-chain 将仅修改 reprojection.rs / gpu_generate_draw_lists.rs 及必要 pass-builder 元数据的范围通知；core 当前没有编辑这两个文件，会保留其原 shader 证据驱动的内容依赖修正。
- 树候选的最终最小组件接口为 `PrefabInstance(pub PlacementDataSsbo)`，保留完整48字节，名字仍由 PrefabInstanceData 拥有。原外层 TreeEdit serializer 已补证保存 index；不能将其当作内层 TreeData.id 或忽略该保存字段。
- 树上述接口已合并并通过 tree-history-r1：core2370/tree10，源码漂移0。其间实际 Move/Delete 改读写同 placement；保持原 Move→Insert→Delete 与 reserved-query失败，不补假出生。
- 普通新墙/栅栏已合入现 core，并通过 ordinary-wall-r1：core2382/graph1、源码漂移0；完整首次 UI→真实录制/账本/下一帧Sync已验。新增共享入口为 wall::registration::{register_wall_drawing_resources, add_wall_drawing_cpu_systems::<S>}；initialize_core_resources/init wall 已保留式注册资源，register_update_systems 已接本地 CPU系统。真实原全局host若接原阶段，应复用 draw_wall/ui_draw_wall/finish_wall，避免再装一套history。必要顺序 UI→history_start、draw→adjust_height→finish→Record→history_finish；相机/窗口/边界从真实owner供应。复杂 continuation/reversal/merge 仍进行中。
- 72项启动producer前置已列入 core 的 PENDING_RESOURCE_PRODUCERS.md；目前独立worker合法claim water/exclude/garden三mask，修canonical资源，不碰主链正改的deferred/DoF/shadow/pebble文件。

core 当前事实与剩余边界见 ../core-systems-resources-recovery-2026-10-18/README.md；恢复经验见该目录 RECOVERY_LESSONS.md。

## 后续原证与资源接口

- 三mask已合入并独审：Water/Exclude/Garden canonical u32×8192、原短name，actual typed library/map/readback + mock GPU tests；masks-r1 core2386全部通过、source_drift=0。请直接复用，不恢复旧32768元素。
- **TerrainEditingData owner接口已合入，尚待统一Cargo**：`rasterization_parts(&mut self) -> (&std::collections::HashMap<StrokeId,TerrainStroke>, &mut bool)`。旗标是同一现tg_terrain::TerrainEditingData的`rasterized`，Defaultfalse；唯一snapshot恢复literal也false，不复制map。原startup/W3四范围与canonical TypeId已双独核。原consumer继续按实际ResMut调用以保留早于early-return的change tick，四pass声明完整成功后set true。证据及15输入：TG-e73d16b0f4be6e76bf/core-terrain-render-20261018；independent review在building-audit/v2/terrain-rasterized-review。渲染owner可接现API，勿新建ActiveStrokes镜像。
- **新墙原数字组正证已到**：building-audit/v2/host-wall-integration/HOST-WALL-INTEGRATION.md，frozen SHA88dbd8cfb30534a4977c723b2288b1ad31bdb53b2b71139a66c0925632c85ff2。set19不是仅eraser：18项wall edit chain含ui_draw4/suppress5/ui_continue6/draw14/finish15/merge16，再串16项adjust/publication。set14=start，set34=4项wall/deferred recording，set35=finish_recording→process→clear_previous→replay_mode→finish_replay→signifier。原finish早于adjust；core本地helper已纠正该先后。不要将core UI.before(start)适配整体塞入host19，与14→19约束冲突。原全局widget/input生命周期还需保持现actualowner顺序。
- Delete独立height、续画false族Pending/drain与失败时先remove后color顺序已按freeze合入core；尚待统一Cargo，endpoint/reversal/merge仍候选，不能称复杂续画已验。

### canonical terrain field 再核

2026-10-18T12:32:40.626020+00:00：收到v45编译时旧二字段类型的诊断。当前三个生产文件已同批合入，`crates/systems/terrain/src/stroke.rs:70`确有pub rasterized:bool，workspace Cargo.toml:49 tg-terrain仍指该本地目录；非reconstruction旧副本。可能构建与跨文件整合交错，当前输入应重核而非补平行字段。当前SHA：
- crates/country-core/src/resources/terrain_editing.rs `1ae33696f5f9576efdb1f1148b43ef94aaba11ecd8aa37b34c29e78465526606`
- crates/systems/terrain/src/stroke.rs `7186a6f81af44b5891cfc04f372ab98da5fac7646b07c06e8429f0411725e783`
- crates/country-core/src/systems/save/snapshot.rs `ff15b0db910f2ee0c473c60800fc2a5da290742ababa5ee1ac612fa43ed888e7`

### 续画共享字段候选（尚未合入，publisher仍归main-chain-boot）

root合法持有WidgetParams/PublicWallState两组，准备了最小API候选：`WidgetVolumeShape::sphere(center:Vec3,radius:f32)`，保留旧tag/layer/size，新增逻辑center；native sphere byte1是padding，不造业务layer。`PublicWallState::is_being_bulldozed()`与`pub(crate) set_being_bulldozed(bool)`为published row派生字段，现projection写+297。只有真实publisher从actual AnimateWallBulldozeBuffer在重建时采membership之后，endpoint UI才能启用；不可UI时现查queue代替。

候选与精确原证位于TG-27aa1146cf8ff64436/core-wall-ui-adapter-20261018，frozen-inputs-v2.json SHA cad8ef60501bb06766b2ea9dff22e666c3a60bd602c3a87d315c474030414f3f（36inputs）。其中`publisher-owner.patch`是对你claimed publisher的**未应用**必要hunk；仅增加actual Res参数并为last_delta.changed重建row采membership。root不覆盖该生产文件。请保留原publisher其它变更。后续shared API审查通过并合入将另记。

### 主链 W3 与实机证据交接

canonical TerrainEditingData字段迁移完成后，v46 SSGI六项CPU合同测试通过。主链现W3直接使用现有`rasterization_parts`，Mut.set_changed仍在事件/early-return前，原XZ、Gauss float4/float、零infos真实页地址及实际BDA owner/range已接回当前架构。v48核心3项CPU通过，v49 core依赖及tiny-glade库测试编译通过；PassFrame延后报告admission错误时，host会恢复原rasterized标志并由失败帧取消整图。W3原shader实际GPU还未执行，fullworld caller/W4-W5 rocky producer仍不算闭合。真实BDA生命周期独立诊断GPU v44一项通过，shader不读取BDA输入；详见bda-owner-validation-v44.json，不混作W3算法验收。

- continuation-r2已结束：2399核心库+1实际编辑图全部通过，仅tiny-glade/src/world_terrain.rs漂移，核心/terrain/clutter/manifest当前hash与run after一致，详见core报告assessment。Bush-clutter-r1实际8项通过、source_drift=0。原full draw执行、GPU全局与host W3不据此接受。UI/public36仍候选，publisher未改；空装饰桶的首份Some(empty)归档独审发现新缺口，现draw/endpoint owner正准备最小delta，不为通过改变fixture成假None。

## 20:50 共享接口与启动producer分工

- **UI/public API现已合入，待统一Cargo**：WidgetVolumeShape::sphere(center:Vec3,radius:f32)与center字段、PublicWallState::is_being_bulldozed/set_being_bulldozed(bool)及projection+297。原36输入/v2及building wall-ui-public-review独审通过，无native union复制。真实publisher仍由main-chain-boot拥有，root没有改。请该owner在当前publisher上应用 `TG-27aa1146cf8ff64436/core-wall-ui-adapter-20261018/publisher-owner.patch`（相对tiny-glade根），仅加actual Res<AnimateWallBulldozeBuffer>，在consume_with_paths后对last_delta.changed row写真实contains。建议3帧验收：发布含queue→只移除queue但不重建仍true→重建false。endpoint须等实际hook后启用。
- root新增合法claim：TG-6e3c60ba760d787a24 StairsCounter、TG-57a33750ac4a866a76 Dormers、TG-3aa474b479ac4e3b2b Grounded，owner core-producer-20261018；后者按既有用户systems/resources范围明确解deferred。Lantern在现唯一render_ssbo.rs修本地factory，绝不复制render::lantern新定义。BushCPU、三mask现已合入/独审/回归，旧缺口勿重复实现；主链UI/water/garden/Boat/Stairs窄改保留。
- Rocky资源组由building-rocky-20261018独占，完整真实asset pack已核，正产保留load→Self/CPU Vec的原DTO/packer/固定Typed容量+VertCount候选，startup由main-chain owner接现返回数据，W4/W5不重复造资源。未完成之前不声称全部PreStartup ready。
- PrefabPart与真实Mesh Arc/audio separate bounds接口目前**尚未整合旧交接候选**；保留生产48B PrefabInstance(PlacementDataSsbo)和现完整placement热重载API，不能用旧Transform默认数据覆盖。root将核定增量字段后在此提供ready哈希；startup/load_prefabs仍归原owner，仅据canonical最终接口改wrapper，不擅用旧snapshot整体覆盖resources/prefab。

### 输入/保存与Prefab接口范围确认

root当前未领取、未编辑ModKey系统或save_game系统，避免与os原证worker抢写。现root主线是上述canonical producers、Rocky、绘墙续画/merge与已合typed历史。input/save的新原审计保留在TG-40ff43c62ef8791c53/oschain-20261018/host-edit-save-audit，后续实现必须先由实际工作owner在task_queue独占；root不会在对方已claim后并发覆盖。现StairsRasterValue仍是独立未填充audio输入，也不以新StairsCounter创建完成为该audio消费者恢复信用。

Prefab接口决策：原候选Part的 `name:String / audio_trigger:Option<{kind_name:String,bounds:Aabb3}> / render_mesh:Arc<Mesh> / render_subset:GpuWorldSubsetKind / rt_kind / albedos / bounds(RT)` 符合原producer/consumer分离方向；资源文件仍归main-chain-boot claimed owner，root未改其Part字段。该owner可按以上最终字段整合startup wrapper与真实atlas consumer，但**必须从当前production增量应用，保留现PrefabInstance(PlacementDataSsbo)48B，而非覆盖旧candidate里Option<Arc<String>>/gap的错误组件**。audio仅exact "Tree"，+28是String.len4而不是kind_tag，音频bounds独立，不再用RT box/name/rank。现热重载完整48B输入及computed cull数据不回退。root会复核同批consumer必要hunk和bounded tests，不把原旧snapshot整体接受为当前源码。

- 三producer canonical现已按27冻结/独审合入（尚待同批Cargo）：`systems::audio::terrain_stairs_audio::StairsCounterSsbo(u32)`，mapped cap1 NAME Stairs Counter/None4字节零；pass_kit::StairsCounterBuf已alias它；现LanternSsbo原唯一类型Instance/None0无上传；GroundedBuildingsSsbo原唯一类型NAME现为Building Shapes/Instance count0无None上传。3factory/bound tests未预填通过。输入合同/patch在TG-6e3c60ba760d787a24/core-producer-20261018，frozen SHA368d61...acd8。render/ssbo.rs仍由render-prelude-ssbo-20261018持有，root没有改其generic，`ssbo-owner-followup.patch`单行原map应比returned buffer.byte_size供该owner后续；Stairs本地factory已按原1128B exact create/map，无需阻塞其启动。原音频StairsRasterValue消费者仍独立待接，不能由构造正确推定整音频已完成。
- 校正UI编译报告归属：旧runner漏跟踪roof/toolbar/decorator共4个API调用点；r1五包all-target确实exit0，但仅5/9 API文件在before/after内。当前9文件都与冻结candidate相同，不能据此追溯漏项run0漂移。runner已改为所有workspace crates实际源，正在与新producer同批重验，详见wall-ui-api-r1-assessment.json；不将工具跟踪修复计游戏业务。

## canonical资源就绪（完整workspace跟踪）

producers-r1已完成：2401 core lib +1真实编辑图全通过、源码漂移0；新Dormer4/counter-instance3全实际ok。wall-ui-api-r2五包all-targets check通过，源码漂移0。所有crates源均被新runner纳入，旧r1漏跟踪上层4文件不追溯改写。Dormer、StairsCounter、Lantern、Grounded上述唯一canonical类型现在可供72项按原序注册；完整Startup/GPU全链未据此接受。参考core报告相应result/assessment/log。

- 两成功run结束后resources/rt.rs另由现owner修改；完整最新core/RT不由旧run接受。source assessment已列出当前差异，canonical producer/UI本批源保持匹配。

## 21:18 Rocky canonical已合入，Rust专项进行中

`resources/rocky_terrain.rs`现已增量整合冻结147输入（SHA e5b66476ec87a38c434f3a4a429fa0516af4e64ac09a2f54c01bc75657bd7d22），独审manifest a8fda5a0b7004d961d50ba77640fb629b4bd6ba7316af51952846fe077858a66。实际source SHA c96d80d17e75dd66bea77693f41abce3860f13d6c2bb9618bff36b43296f7fbc。canonical `RockyTerrainMeshVertCount(pub usize)`已就绪，从现`RockyTerrainMesh::load(...).into_vertex_count()`获得；两个唯一Typed SSBO已修fixed cap148794×48/49598×32/None不上传。5项actual-asset/library Rust专项正运行，未预填结果。

原Rocky loader明确是Startup八元组zero-based slot4，**不是PreStartup**；PreStartup十元组相似参数函数为init_color_library。主链72 producer仍按其原PreStartup处理，Rocky real asset loader/count另在现Startup组织，避免将这两种初始化混为一套。完整正证/实际asset path contract/现API用法：`TG-7e388fa8b16d3b9fc2/building-rocky-20261018/STARTUP-OWNER-HANDOFF.md`。W4用count/3，W5用full count，二者借同一canonical资源，不能将triangle_groups再除三或layout cast CPUVec。完整startup_terrain其余网格/UI/shader与GPU帧尚未接受。

新save/thread证据已收到，root当前仅安排readonly合同与claim归属复核；尚未编辑/领取save生产，也未授权他人覆盖既有owner。现`Flush=10`无reply、SaveGameCmd payload及SavingComplete ZST被新原证指出有缺口，后续以实际PE/PDB完整范围核定。

### Rocky test-only E0283 已修（21:19）

主链v54遇到的两处`assert_eq!`目标类型歧义，root rocky-r1同样实际复现（3 E0283/源码0漂移）。生产测试源现已于r2开始前修为`bytemuck::cast_slice::<_,u8>(&mesh.vertices/triangles)`，源SHA 22dc994d38a50af2d7a241f636042b27e12f02c85bfd6b6062be5f04ff9c051d；仅两处test表达式，资源算法无变化。r1失败和147原freeze均保留，rocky-r2正在编译。收到主链 input/save 接管说明，root保持不领取/不改这两组；readonly审查将提供原证与当前源码差异供实际owner复用。publisher actualhook与3帧test已见落盘，等待其编译回归，不从代码存在预填通过。

### 21:20 Rocky实际资产专项通过

rocky-r2：5 passed /0 failed /0 ignored，87.955s，5058个workspace输入source_drift=0。canonical source SHA 22dc994d38a50af2d7a241f636042b27e12f02c85bfd6b6062be5f04ff9c051d。独立147原证/字段审查与实际31MB文件已连到现Rust AssetSsboLibrary/两buffer mock上传，原Startup/W4-W5/full GPU仍不据此接受。主链可直接用现VertCount及两canonical类型，不再等资源构造。r1失败保留；root尚未改input/save/Prefab。详细core报告rocky-r2-result/assessment/log。

## History快照权威模型正在定型（21:26）

主链指出`History<()>`真实缺口已接续，但**先不把payload直接猜作Arc<SnapshotSaveData>**。现原chapter正面有`HistorySnapshotId / MaybePendingSnapshotId / HistoryChainSnapshot`及snapshot_id内联225；Vec元素16B=Arc+chain_index，TryFrom140a84260读取Arc payload+0x10 state<2，可能是线程pending-ID解析owner。readonly worker正在把CaptureSnapshot→set_current_snapshot→thread completion/serde引用连起来。root不另造SnapshotCaptureState镜像、不并发改save/input。

资源原family TG-3846e898cf43c29d09 当前仍由main-chain-boot合法持有（03:34 claim）；root将交现history/snapshots模块的最小candidate和类型API合同供其实际应用，或由该owner按工具释放后root领取，不能以“History owner”的口头称谓抢写。主链host/run_game泛型adapter由其自己接。公开通用HistoryTransactions<M,S>保留；生产canonical类型和真实payload按原证定型，测试的显式unit需要与生产迁移区分。

## 续画与History API候选（21:36）

endpoint20+empty-archive9已双独审并合入现ui_continue/draw/ui_draw。UI默认registry首帧、WallContinueState与现local history生命周期已作最小注册；新增真实初次input自动start/finish四组合测试，不预先start_history。完整core+实际graph endpoints-r1正在编译，不预填通过。主链numeric set19等仍由其原owner实际接，不把本地helper整个塞host。

History原证**确认旧谓词反向**：完整TryFrom是37B（PDB源码行263，不能当263B），Arc原子值0pending/1invalid/>=2真实ID。当前trait的<2返回resolved把Result Err判别读反。`history-snapshot-owner/candidate.patch`及CONTRACT.md（core报告下）已备两文件最小增量，**尚未生产/最终独审**。API按原MaybePendingSnapshotId(Arc<AtomicU64>)，new/default0、clone同owner、assign(id)/assign_invalid1、snapshot_id()->Option只取>=2，serde裸ID不强制>=2；History默认S改同owner；ResolvedHistoryChainSnapshot {chain_index,snapshot_id}与filter helper供save直接用。原data是Box<SnapshotSaveData>，当前Arc capture API可保留但不宣称原Arc；真实snapshotId来自blob calculate_hash而非cursor cast。等待原证freeze/候选复审后主链现claim owner可apply，生产泛型/测试unit传播需同批处理。

TerrainEntities新接口请求已收到：root现已合法claim Rocky组为core-rocky-entities-20261018，正窄核原16B floor/rocky两Entity合同；会只在既有resource加唯一Resource，无Default假entity。boot继续持原startup/spawn/W5；Prefab两consumer hunk root尚未改，仍由main-chain从现源增量应用，root保留48B placement。

### 保存链只读合同与唯一 History 快照 ID owner（独立原证）

core readonly audit已落盘：`TG-57a33750ac4a866a76/core-producer-20261018/read-only-save-audit/{SNAPSHOT-PAYLOAD-CONTRACT.md,SAVE-MINIMUM-CLOSURE.md,frozen-inputs.json}`。原B TryFrom全37B（源码行263）直接证明0pending/1invalid、>=2才resolved；当前trait方向反了。原tag9把同一MaybePendingSnapshotId Arc交worker和History；真实payload是共享AtomicU64 ID owner，非SnapshotSaveData Arc。Deserialize116B→转换452B每项alloc24B并保留输入u64；required serde键为chain_index:i32/snapshot_id:u64。原assign只有aligned MOV+PDB AtomicStore，不能区分原Relaxed/Release，现Acquire/Release须标适配。不得用SnapshotCaptureState镜像或cursor当ID。

另已独核实际140a3f470/141721aa0/14171f7f0共12781条原指令；未发现fullsave.asm字面文件。SaveGameCmd应ZST；Complete应session-folder PathBuf；tag10应Result回信并flush_deferred_and_sync；两request reader都需消耗，常规autosave不得被显式request门吞掉。保存生产由oschain合法owner继续，core未claim/未改save/input；historymod/snapshot/load/mod仍main-chain-boot claimed，canonical资源候选需交原owner应用。详情、当前源码checkpoint及claim快照见合同；无Cargo/JIT/生产修改。

### Endpoint整合初轮实际结果与owner阻断（21:36）

endpoints-r1完整core lib实跑2416pass/2fail/source_drift0；新endpoint/空桶/首次续画生命周期都通过。旧fresh首操作test暴露新增registrar漏PublicWalls（ui_continue真实依赖），已在register_wall_drawing_resources补同owner init_resource/PublicWalls+MainCameraMoved Events，保留现值。另一失败为主链owned `render_loop/compute_passes/dof.rs:545 indirect_offsets_match_the_exe_immediates`，actual16/expected0，root没有改其生产或测试，请现owner修原证/测试矛盾。r2只排除这个独立已知DoF失败，其余core+actualgraph重跑；不会据该run称整个最新core全通过。

History正式候选现已冻结12输入：core报告/history-snapshot-owner/frozen-inputs.json SHA233e75e1ec79f2d2d8af693be8e94be8530824587362400a49de97628af9109c，2文件candidate.patch+独立replay-counter.patch给现main-chain-boot owner；building最终复审进行中。正确pending模型+裸ID serde/0、1读入再过滤已涵盖；ReplayBuffer pop_counter构造0/真实pop尝试先wrappinginc/accessor/Clone保留，无新Default speed。原pop完整395B/107instr已root从B直接核1409f190d inc+0x20，未改生产。

### 21:38 registration依赖路径已修

root新init_resource相机路径错误已修为现有`camera_rig::main_camera::MainCameraMoved`，未造重复event/reexport。registration当前SHA b6c55a7227d8947f49470d0c1fb3a0a42143a310851ba7e53c58c6b1515172f7。endpoints-r2因该编译错误失败，失败保留；修改期跨run1文件漂移，未声称通过。待TerrainEntities窄类型同批再跑r3。主链72CPU/GPU owner/publisher最新成功由其v58证据负责，root不据此推shader/initial bytes已验。

### 21:40 TerrainEntities canonical API已增量合入，正证复审/编译进行中

现resources::rocky_terrain::TerrainEntities `{pub floor:Entity,pub rocky:Entity}`已窄加在既有资源模块；无Default，实际两spawn Entity由boot填，root不制造假实体。源码SHA 8e8f102f72461be7001794cc26fffbf40acad1147a3db5cb529996063d3a972b。仍保持所有已通过Rocky loader/packer/VertCount原body，Entities原TypeId/16B写与consumer独审由building收口；endpoints-r3统一编译开始，未预填通过。Prefab两必要consumer hunk root未改，可由boot从现源增量整合，不覆盖48B placement。

## History shared pending-ID + ReplayCounter正式候选独审通过（21:40）

**现main-chain-boot owner可整合**：core报告`history-snapshot-owner/candidate.patch`（history/mod默认+snapshots同现容器/API）和`replay-counter.patch`（现ReplayBuffer字段/真实pop，不改其它消费者）。12输入freeze SHA233e75e1ec79f2d2d8af693be8e94be8530824587362400a49de97628af9109c；building formal-candidate12-verification已全核hash、PE9原范围+pop395B107instr、临时base apply逐字节一致，无blocking。当前生产仍未由root应用；候选内CONTRACT的“独审pending”是freeze时历史，现由此条/独立review覆盖，不能改旧freeze。新增测试源码未执行，S迁移与production/test同TypeId需owner同批做。

可用API：`history::{MaybePendingSnapshotId,HistorySnapshotId,HistoryChainSnapshot,ResolvedHistoryChainSnapshot}`；`MaybePendingSnapshotId::new/default`0，clone共享，assign(id)/assign_invalid1，snapshot_id取>=2。`snapshots::resolved_snapshot_references(history.ledger().resolved_snapshots.as_slice())`提供原serde两required字段引用列表；载荷新同owner写回History，删除SnapshotCaptureState双权威，原blob hash ID不可用cursor。`ReplayBuffer::pop_counter()`构造0、每次实际pop先INC/包括空尝试，push不增，Clone复制。root不造新的保存writer或ledger；save/input继续os现owner。

本轮endpoints-r3又遇主链新Prefab测试`startup/tests.rs:130 [PrefabName::new("new")].into_iter().collect()` E0283目标类型歧义，root不改Prefab owner源，仍需该owner显式collect目标后统一编译；完整错误保留core报告endpoints-r3.log。这不是新wall运行失败。

补精确owner编译hunk：core报告`startup-test-owner-followup.patch`给startup/tests.rs:130显式`collect::<std::collections::HashSet<_>>()`（actual changed是HashSet，不能直接用rustc通用提示Vec）；root未apply该owned文件。endpoints-r3 lib生产编译完成，lib tests因这两E0283失败/0漂移，日志保留；下一次不在未改源上重复全编译。

### Snapshot writer schema与唯一入口协同

root serializer worker history_audit现已核原capture完整20581B：原Version2、Walls/Roofs/Decorators、Raster仅name list、三色含HalftimberColors、Terrain、TerrainHeightRange、Trees、AutoClutter、Stairs、StairsRemovedSupports；随后Motomoto JSON chunk+每Raster rawu64长度/Deflate level6压缩chunk，还有独立Terrain tag7。现snapshot.rs把loader日志当9key schema且漏域，save_thread::capture_snapshot仍直接写JSON文本，因此不能称完整snapshot已保存。

history worker正在同**既有snapshot.rs单一Result encoder API**准备必要candidate，复用现motomoto Writer/Reader（core Cargo必要依赖边），请os save/thread owner只接该writer Result和同pendingID，不在thread另造chunk/JSON engine。当前无新生产更改；待候选/原证完成给明确API/hash。

DecoratorStorage family TG-889aa76ca030a2e3cd刚查确实free，terrain worker现已合法claim制作take_snapshot非空修复。原真实bucket(addr,Vec<(id,DecoratorHistoryState)>)，必须保留Some(empty)桶；不能按旧flat整DecoratorInfo拼schema。DecoratorOpEvent/unleash family TG-43142338addb95ca2d仍main-chain-boot持有，terrain会交最小alias/EventWriter必要候选供ownerapply，不造第二136B payload。

## 主链最新真实验证与接口（v50–v63）

v50 loading canonical producer CPU/GPU及三帧WSI均通过；v52 W3实际原shader三帧GPU通过，clean帧零pass、dirty帧四pass，BDA实际读与owner在同session；旧“W3未GPU”仅历史时点。完整72 actual producer和gameplay PreStartup hook已合入，v56 CPU1pass+publisher bulldoze三阶段1pass，v58实际GPU物理owner/cursor事件1pass；GPU_ONLY不能host map，初值GPU shader验证不混作owner测试。Prefab Part/load/atlas wrapper与系统audio两个hunk已从当前production增量合入，48B placement保留；v61相关35CPU通过。W4/W5/W9及OnEnter Grass与单身份StairsStep已接源码，尚未GPU。gameplay新的资产PreStartup调度原输入tuple→atlas→Nani已接，接下来实际发行资产验收。DoF v62实际GPU因明确native32B uniform又被当private upload而失败；现窄修write_lease角色，pending重验。没有关闭guard/变usage/伪造ready。完整GameLoop仍未世界图warmup成功。

### Snapshot Result writer 候选交接（未生产，独审进行中）

history_audit 已冻结 `skeleton/task-index/work/TG-57a33750ac4a866a76/core-producer-20261018/snapshot-domain-candidate/`，61 输入 manifest SHA `84b47e0273a84b1a987eee51af9cb6ebe5e99268b99a7ff00033ac491756c0e1`。`candidate.patch` 给原 snapshot.rs owner，`integration-owner.patch` 仅补现 StairsSnapshot 的只读 `cached_checksum()->Option<u64>` 和现 motomoto Cargo 依赖边。当前 TG-3adf041a3108edb59a 仍 main-chain-boot claimed，本 worker未改生产/该owner work/Cargo；building正独审。原19静态范围34113B8142instr（17full、2明确bounded）、40字段字符串、56Clutter字符串及2patch隔离逐字节核验/Rustfmt通过，无Cargo/native/JIT执行。另1处新增test私有路径已独立更正：同owner目录 `snapshot-test-api-v2/candidate.patch`，4输入 manifest SHA `3d1dcd44537c8955a08c79b43d6fb8abe35880550b12f2cf012f231f9d34e3ee`；原61冻结不动，断言和生产逻辑未改。

唯一实际文件接口为 `snapshot::encode_game_snapshot(&SnapshotDomainData)->anyhow::Result<Vec<u8>>`，`SnapshotSaveData::encode()` 直接委派同入口。`to_json` 仅元数据，**不能继续直接写为快照文件**。文件先 Snapshot JSON chunk（Version2和14原字段），每Raster chunk先u64LE原长再Deflate6，最后Terrain chunk仅Deflate6无该长度前缀；均复用既有motomoto Writer。原数据Box与现Arc接口区别已明确，不能用其回退现手工设计。

实际 save/thread owner 请传真实收集数据并调用 `.with_terrain_and_supports(actual_raw_height_bytes, actual_height_range, actual_removed_supports)`。旧collect的空Trees/decorators/colors/IDs/raster迭代器不能冒充真实采集完成；缺rawTerrain/Range/RemovedSupports会明确Err，`stairs:None`也不再伪造empty准确备份。合法Some(empty)仍保存，包括已矿分桶空地址。原height采集是实际owner count×4原始字节和+0x50范围；不要从当前u16模型猜填数据或从strokes猜range。成功写入/计算真实blob hash后才assign同pending-ID owner。请用该Result接口，不在thread复制另一个JSON/chunk engine。

依赖 canonical DecoratorStorage69（true ID+bucket/state）、Roof139和现WallState serde；Root拥有各必要增量hunk整合。Roof139已由history_audit独审静态通过（24full/13271B3726instr），未跑Rust，报告位于同TG-57 owner的`roof-schema-review/`。旧JSON-only loader/DefaultInfo假恢复与SnapshotPrefix原生内存投影并不代表真实chunk load成功，完整load/真实idgen/域写回仍明确未闭合，不能把writer候选称完整保存读回验收。

Snapshot61后置必要API delta：纯源码子审与building均确认现WallId没有Serialize，不能把Vec<(WallId,WallState)>直接to_value。`snapshot-wall-id-v3/candidate.patch` 只在Walls字段借用映射 `(id.0,&state)`，保留原fullu64和现WallState，不改全局ID类型/JSON断言。5输入manifest SHA `124b35acd4c922760c466abb6265bb4e74c16ee22a109f3afa34e035d05f8d14`，同TG-57 owner；按61→testv2→WallIdv3顺序，最终source SHA `8508fceddfca138c2621a95d19f08ee6e125d6b8b5b0622b69225a4074210835`。隔离applyexact/Rustfmt通过，未Cargo；原61冻结独立存在该已知编译缺口，不应单独宣称可编译。Building正以同批v3关闭finding收口独审。

Raster chunk边界：原tag_from_str_trunc只取8字节，现Cobblestone1/2/3Raster的tag会碰撞；Snapshot保存的是full-name列表，encoder按原Vec顺序逐个写全部chunks。后续reader必须保留该对应顺序，不以truncated tag建唯一Map去重或覆盖。当前候选没有去重该Vec；这不是另造Map/第二writer的理由。

Snapshot61+testv2+WallIdv3已由building正式独审通过：`TG-f7070bb3446b7a37eb/building-audit-20261018/v2/snapshot-domain-independent-review/frozen-inputs.json`，93输入review SHA `c55f1b44fae62f5ec690e507da30f03bef3c2d58c677fec365e44805bc700fb0`。70作者输入全hash、19原范围、40字段真实RIP/编码长度、56Clutter实际分支ptr/len、3dispatch与独立Side678B、候选隔离逐字节应用均核过，最终source SHA仍`8508fced...0835`。WallId编译缺口以v3关闭；无Cargo/native执行，真实producer/worker接线与完整load仍未验收，不能把独审当运行测试。


## 主链实际增量（v64–v70）

v64 景深九个原版 pass 已在 Apple M3 GPU 成功执行，真实 32B 初值表、7 个间接调度槽、两个坐标分区与最终 resolve 像素均通过。修复是显式 native UniformBytes 改以真实 lease 绑定，避免同范围误判 private uniform 二次上传；保留严格读写 guard。

v64 真实发行资产 PreStartup 失败后，完整原指令确认空 trunk 删除 id 后跳过 appear_pos/age 补齐，Mesh 非空断言本身正确。窄修后 v69 同 GPU 启动通过：25 个 prefab、39 个部件、27 个 Nani 子集，真实 mesh Arc→atlas→实际 GPU owner。完整 Startup 的 Terrain 扩展正在另验，不能据此声称所有启动阶段完成。

v69 Apple M3 的 GPU DrawIndirectCount 平台适配通过原始命令偏移/stride、计数 2→1→0→2→3(cap2)和两个独立 scratch 的像素测试。原 Nani 不清旧命令尾部，不能把 count 简化成固定 max；适配复用同 heap/compiler/图/实际 GPU counter，不回读 CPU。

v70 显式允许原版未定义历史读取的当前图元数据已合入，5 个集成和 1 个真实缓存边界 CPU 测试通过。仍保留 reads_contents/依赖/同步/owner，不把 history 清零或虚报初始化。原 TAA 两帧 GPU 通过；TGSR 首帧为 [255,0,22,255] 触发旧纯红断言，原 shader Frexp/Exp2 写出的蓝通道 2^-7 经 resolve 恰为22，最窄预期已按原证修正，第二帧仍待重验，不能预填通过。

History 默认已迁 canonical MaybePendingSnapshotId，v69 整库实跑 2403 通过、22 失败，失败均为旧 fixture 的 History<()> 与新默认资源 TypeId 混用；不插入两份历史来隐藏。旧 grid 测试传 u32::MAX 行给恢复后的原循环，停止了本任务独占测试进程并改为小尺寸验证原 wrapping 运算，初次日志保留。

完整世界 W47..54 已接入当前 typed 后处理入口，W54 原副作用是反馈完整36B纹理替换，不是 desc_tail_copied 布尔；新同批编译/GPU正在进行。此前 geometry/植被/墙/草/水面、compile_main 与真实 warmup、相机工具交互以及 Motomoto 存档恢复仍需逐个接通；production world guard 仍在。详见 main-chain-validation-v64-v70.json 与 logs。

## 2026-10-18T23:28:55+08:00 Snapshot依赖同批已实际整合（本core线程）

已依序apply Roof139/circle旋转shared、storage69-owned+history_edit必要hunk、上层queue alias、Snapshot61+getter/Cargo边+testv2+WallIdv3。所有候选freeze已复核，patch逐增量check后应用，无整文件覆盖。现snapshot.rs最终SHA **8508fceddfca138c2621a95d19f08ee6e125d6b8b5b0622b69225a4074210835**，与正式v3完全一致；os save4可依现`encode_game_snapshot`/`SnapshotSaveData::encode`与真实`with_terrain_and_supports`接口推进，必须传真实captured域。core报告snapshot-batch-integration.json含8当前源输出/9patch，所有before备份独立保留。

当前decorator.rs在主链已有canonical Event/realcounter基础上发生owner漂移，69该文件未wholeapply；terrain worker正在基于最新源产最小兼容delta（pub type别名改pub use以支持同tuple constructor，保已有drain/counter测试，补actual system），不回退主链现代码。暂未启动统一Cargo；Cargo.lock仅需core→既有motomoto边，下一步补并核依赖不升级。此条不声称整合编译/保存load全链已通过。主链History旧fixture修复已读，root不回退/重复修该22夹具。

### 2026-10-18T23:31:20+08:00 相机canonical必要接口

Butterfly原唯一24B定义已按请求下沉`core::resources::critters::Butterfly`（同字段/derive/注释，算法未变），原`system_critters::components::Butterfly`改pub use，同一组件身份，未复制实体或造第二类型；新core资源文件与上层components当前已落盘，随snapshot-batch-r1核心编译，后续system-critters检查另做。相机可直接读core canonical路径。

GUI原组TG-cfac1f498c427933d2实际owner仍core-gui-20261015，root没有覆盖其源；core报告`camera-component-handoff/gui-owner.patch`是从当前GUI增量给DebugGui一个`camera_snapshot_command: systems::main_camera::CameraSnapshotCommand`+Default::None的必要hunk。当前production main_camera尚无该enum，所以**未应用GUI hunk**，需os camera enum一起集成；不另建Resource、不用旧整GUI候选覆盖。

Decorator event-v2最小delta5已合，保留主链实际Kind/Eventderive、unleash_to_events与countertest；typealias改pubuse支持相同tuple constructor，补actual System与已审测试。现decorator.rs SHA8b3c427c0c6df2110e2c291dd580c1a224817cf83404002afc6ec6a86102cd1e，统一Cargo进行中。Cargo.lock仅增country-core→已锁motomoto一条边，无依赖升级。

## 2026-10-18T23:35:07+08:00 保存资源/编码库统一回归通过

snapshot-batch-r1：2454 core lib+1 actual editing graph全部PASS，0fail/ignored/filtered；唯一漂移为独立gpu_generate_draw_lists.rs，不接受其新GPU行为。Roof/Storage/Event/Snapshot/motomoto/lock当前输入均在run无漂移，原testv2/WallIdv3修正已包含。核心报告有result/assessment/log。后续仅把两snapshot helper增加真实DecoratorIdGen参数并下传with_id_gen（不临时造gen），load-idgen-v2 6inputs已合，writer API/元数据不改，snapshot新SHA f67ce7cb5ab787aed3f00138f59fb90e01c4a963edea74e6c0372726314ce1d6；其targeted test与上层critters/decorator check继续。原61/v3 freeze不回写，保存owner按当前writer API直接接。真实非空保存文件回读、原State/Roof deserialize与actual load仍未通过。

### 2026-10-18T23:36:52+08:00 上层编译与真实gen adapter专项

snapshot-consumers-r1：system-critters/system-decorator全部target check通过，0漂移；同Butterfly/core reexport和唯一Deferred消费者已编译。snapshot-idgen-r1：现真实caller gen传播专项1pass，snapshot当前SHA f67ce7cb5ab787aed3f00138f59fb90e01c4a963edea74e6c0372726314ce1d6匹配runafter；run中另4源漂移：crates/country-core/src/resources/stairs/mod.rs, crates/country-core/src/resources/stairs/stairs_top_down.rs, crates/country-core/src/systems/render_loop/draw_passes/foliage_exclusion_mask.rs, crates/tiny-glade/src/world_terrain.rs。不据此接受那些并行新源或完整实际snapshot file load。core报告相应result/assessment/log。


## 2026-10-18T23:43:25+08:00 装饰 canonical Event 的实际 set25 注册交主链 owner

本 core 线程已整合并实测 Event-v2（保留当前 Kind/Event derive、drain_events、unleash_to_events 与旧 realcounter 测试，只 pub use 兼容名和实际 System 增量），2454 core lib+1 actual graph 通过；上层 system-decorator/system-critters 全 target check 通过。真实 DecoratorIdGen 已经两 snapshot helper 同 reference 下传 `restore_snapshot_with_id_gen`，专项1通过。以上不等于完整快照文件解码/加载成功；load_game 对真实 snapshot open、旧 DecoratorInfo JSON decoder 仍有明确 Err 边界，State/Roof 原 deserialize 另由原 owner 接续。

**实际宿主注册仍由 main-chain-boot owner 应用；本 worker 没有改 run_game.rs。** 必要最小补丁：`games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-889aa76ca030a2e3cd/terrain-curve-20261018/event-registration-readonly/run-game-owner.patch`。同目录 `frozen-inputs.json` 共23输入，SHA **fb24e95e3151017f2b407ce4494d50c392e798bb7268b265f7ae592307dc9bde**，`CONTRACT.md` 给逐层原证；owner-base-identity.json 保存当时真实宿主 base hash，补丁仅必要 hunk，遇新漂移请在当前架构增量应用，不覆盖宿主整文件。

补丁复用当前唯一 `DeferredDecoratorOpEvents`：增加 `app.add_event::<country_core::resources::walls::decorator::DecoratorOpEvent>()`（dependency EventRegistry/Events 正常维护），并将已有 core `unleash_decorator_op_events` 放入实际 Update 的 `GameSystemSet(25)`。不要另建 wrapper queue/第二136B payload，也不要仅 init_resource Events 而遗漏维护。

**set25 精确原合同：** B `DecoratorPlugin::build` VA **0x140342c10**, PDB17521B；`0x140342cc8` 调 `EventRegistry::register_event<DecoratorOpEvent>` VA0x140584b50。`0x1403451af` 调 `IntoScheduleConfigs::chain` VA0x1400d52f0（4650B）；原 typed19项 tuple 的 zero-based member15 恰为 `ResMut<DeferredDecoratorOpEvents>, EventWriter<DecoratorOpEvent>`。链内 `0x1400d57e0` 调该 exact typed into_configs VA0x1402de9d0（437B），其 System vtable 再落到既审原 unleash VA0x1408337c0。回到 build，`0x1403451b4` 写 **0x19=25**，以 vtable0x14292af98（原 GameSystemSet typed methods）在0x1403451d0 intern；0x1403451e2 in_set_inner，0x140345253 add_systems。三原范围合计22608B逐指令 PE 字节校验，原证静态，无 JIT/Cargo/生产改动。

因此不要为了“当帧清队列”再强制 `.after(replay set40)`：原 replay40 产生的队列项应在下一帧25发出。此 hunk 只恢复19项原 group 中当前可接的 actual unleash，不宣称其它 decorator input/visual/collision consumer 全部运行；当前上层 cull_oob_decorators 构造局部 Deferred 后丢弃的 producer 缺口仍单列，不能靠注册一个 System 隐藏。


## 主链实际增量（v74–v77）

v74 真实发行资产 Startup 已含 Terrain：25 prefab/39 parts/27 Nani 子集和8个地形实体在同M3设备/owner链通过；TAA2帧与TGSR2帧真实像素通过，两次TGSR蓝22均来自原SPV偏移量，不是放宽容差。输入/profile/事件维护与typed roof/mouse/visibility接线编译通过，host29CPU通过。

v75 后台真实GPU frame0→主线程原App frame1通过，同named历史owner、actual Arc、奇偶帧和像素连续；无unsafe Send和第二套asset表。W54实际捕获参考→整36B纹理替换→差异像素GPU通过；temporal15CPU通过。九级SPD首次失败InvalidBounds，随后按原1623B+535B消费侧证实None视图表示base起剩余级数，原bind_individual_mips须逐级数组；v77修当前host后真实九级两帧、最后两级mip回读、24B实际零buffer与同counterArc通过。当前fullybound ABI未用尾槽复用最后真实view，原SPV gate证实无越界访问，差异有界记录。纹理历史仍首帧false且None未初始化，buffer实际原zero上传与可用history独立。

v74整core2432pass/3个植被新fixture误期望重复resource0；独审保留生产guard，期望依原当前builder去重。v77整core2435pass/0fail。v76 autofocus Vec2→array编译错保留日志，修转换后通过。

新连续world_graph入口已按原W1..54逐项串接实际typedconsumer，W21/27追加原墙8B push/highlight高位和Prev/Next第二轮交换，W7仅查询canonical optional StairsTopDownMesh，绝不插默认Some；Surface已原地提取prepare/raster/finish安全借用同World，并增量canonical flags0x108与fragment/RMW元数据。**这些新源在下一批编译/真实整帧验证中，不能据局部GPU通过称整图已成功。** 当前production compile_main/warmup完整世界guard仍保留，canonical draw callbacks/完整clearing/相机交互/存档读回闭环继续推进。

证据与失败日志：main-chain-validation-v74-v77.json / logs；冻结二进制SHA在JSON，未复用编译中可变target二进制。

## 2026-10-18T23:50:16+08:00 用户要求当前全部修改提交并推送

本core线程正对games_test仓库main当前工作树做一致性Git快照并push origin/main。root子agent已安全停写，未完Roof/State reader候选仍按未冻结/未独审/未执行标记保存。请其他owner若读到此条，在当前安全文件边界短暂保持源/报告稳定，避免stage期间新增半写文件；新的恢复任务不在此commit宣称完成。已验证核心2454+1graph和上层两包check证据保留，后续并行新源仍按各自run边界，不把提交动作当业务验收。


## 2026-10-19T00:02:08+08:00 Git提交窗口结束

用户要求的当前修改已保存为Git提交；`origin/main`已成功推送至`1f738fea2133ec392e583ca4b3f113e704f5648b`，包括核心恢复主快照以及提交窗口内新增的水面启动、世界工作线程与主循环存档接线候选。398个约3.8GiB的本地Mach-O验证程序仅留本地并精确忽略；源码、测试、报告及有界原版证据已纳入。提交不代表未完成候选已经通过业务验收。

本条作为本次窗口最后协调记录随最终提交推送。主链owner确认本条已在`origin/main`后可恢复仓库写入；其后新增改动属于下一工作批次。core子agent本批仍保持安全停写状态。


## 2026-10-19 v79 连续世界图源码与既有 GPU 回归

v79 连续 W1–W54 世界图入口已实际编译通过；同批 core CPU **2457 pass/0 fail**，原 shader GPU 回归 **10 pass/0 fail**。后者是既有有界场景段、九级 SPD、后处理与同 owner 预热交接等用例，**不是完整非空世界帧验收**。当前生产 `compile_world_graph` 与 `warm_up_gameplay` 的缺失消费者 guard 仍明确保留。

森林／水面完整启动、canonical 绘制回调、相机 set9/10、save4 与非空 world→worker 新候选均不在这次 v79 接受范围；后加 RtWorld/GpuWorld 清理也待下一批编译。原始通过日志和冻结可执行文件 SHA 已保存 `main-chain-validation-v79.json` 与 `logs/core-v79-cpu.log`、`logs/world-segments-v79-gpu.log`。不按测试数推算游戏完成百分比。


## 2026-10-19T00:14:08+08:00 core共享阻塞恢复

Git快照已完成后，本root按原授权继续。相机所需`RaycastWorld::spherecast_w_terrain(&self, &TerrainHeightsData<TerrainHeightSsbo>, origin, dir, radius, ShapeCastOptions, mask)`现已最窄整合，当前source SHA8161e74e7b74bc3d128187c872a626505e8e68e32212f026843597004bb86ffe；旧Raycasting入口委托，只有一份原地形合并算法，同World。10输入独审freeze4df77518bf1a1e55be3c149b642f7b89abe0266c26c8efe42c80d24074fe1a7a，Cargo待本批统一。

Prefab唯一事件正在原系统owner目录增量恢复：API保持`prefab_name: Option<PrefabName>, entity: Option<Entity>`兼容现owned构造，第三字段改真实`placement: PlacementDataSsbo`全48B；Despawn仅保留`entity: Option<Entity>`，删除旧_gap。不造第二事件身份；当前forest7候选的Some(...)构造正好兼容，主链请按该API整合其forest7；本root仅处理core事件与tree-tool必要迁移，不覆盖forest owner候选/源码。Options为当前Rust工程表示，原事件均带真实name/entity，不用Default事件证明真实出生。

State候选原TG-57目录是Dormer误归属，旧证据原地保留仅只读引用；新State候选按主链owner此次委派放实际snapshot合法work下`TG-3adf041a3108edb59a/main-chain-boot-20261018/core-state-reader-handoff-20261019`，不抢或改claim，生产snapshot仅root后续最窄集成并保留真实gen传播。地形Option与真实写入由原owner在现wrapper内层唯一状态恢复。


## 2026-10-19 用户停止开发并要求提交推送

主链与三个子agent已在安全边界停止仓库写入，不在push后自动恢复。当前全部源码/候选/有界原证/进度作为在途快照提交。v81宿主32/0、core2465/1，实际非空world→worker→同atomic快照ID→真实文件和flush ack测试通过；最新v83整体构建未通过，duck.rs:818边界Buffer误选同名inherent接口类型不匹配，按用户停止指令保留未修。详见 STOPPED-2026-10-19.md、main-chain-validation-v80-v83.json 和 logs。canonicaldraw/forest/相机仍候选，Surface整帧未执行；宿主候选HOST-CONTRACT.md尚未写明，不用manifest存在冒充完整交付。


## 2026-10-19T00:23:58+08:00 用户停止任务：core及全部子agent安全停写，供主链统一提交推送

按主链转达用户最新明确「停止任务 提交推送吧」，本core root与building_audit/history_audit/terrain_curve三个子agent现均已在安全文件边界完全停写，无在途工具/Cargo/JIT/Git任务；本线程不stage/commit/push，不在主链push后自动恢复。此前Git窗口结束可恢复的协调记录已被本条最新停止要求取代。此条为本线程最后仓库写入，主链可现在统一快照。

### 本续轮实际生产增量与未验收边界

1. 仅相机同World碰撞借用API，以及Roof/Shape存档读取两个有界批次已精确合入。碰撞10freeze4df77518bf1a1e55be3c149b642f7b89abe0266c26c8efe42c80d24074fe1a7a；原Raycasting入口完整委托唯一World算法。Roof197 freezed8b7b07a1b6b40d89158f5993cbf310515b7b52fb0752b4b982bc9cc8218357d；本root独审+只读重验34full PDB/32301B/8116指令/75静态check后应用，保留88B/28B/12B owner、无geometry重写。

2. 本续轮没有执行Cargo，新增Roof/Shape10tests未跑；不能把此前2454/2457库测试或主链v83在途结果自动视作覆盖这些最新源。root审查整合记录在core报告collision-borrow-20261019/integration.json和roof-reader-20261019/review-integration.json。当前实际源码SHA：

   - `tiny-glade/crates/country-core/src/systems/collision/world.rs` SHA `8161e74e7b74bc3d128187c872a626505e8e68e32212f026843597004bb86ffe`

   - `tiny-glade/crates/country-core/src/resources/roofs/roof_shape.rs` SHA `efd8788a93b0a203245c717c3e37a07b25c0f81d4bf13086318e3d5540826523`

   - `tiny-glade/crates/country-core/src/systems/wall/valid_enclosure/mod.rs` SHA `22eb05b20add68b1265eb52147c13b1b1deeec275af9d29cc08dfb9d2da41c78`


### 已完整落盘但未生产整合的候选

- cursor24：TG-e73d16b0f4be6e76bf/terrain-curve-20261018/cursor-plane-20261019，freeze909118c7ba26bd690b2dcd92b6c1ad8effd81365ce0a1a8432560a97e0917b17。唯一inner Option、state1 Some先于SSBO、state3缺末笔画仍None、snapshot唯一literalNone均尚未应用。building独审4输入freezeeb5a60e72bedcc2d8de51ef0dfdfd41dcad8a05d67aa09cee6cf9db423e34ec2，在building合法work/v2/terrain-cursor-plane-review-20261019。原typed端口是ToolWasJustUsed而非grow_one别名Finish；3新tests待跑。

- Prefab18：TG-d44984ec9eec9a0bde/core-prefab-adapter-20261018/events-20261019，freeze485499187e22b8392992a5feb983258b5a1da045108fb2468d0d2f26fb1135fa。Spawn原72B=现Optionname/entity+完整Placement48，Despawn8移除_gap；tree必要word映射候选在新合法claimTG-3add2880f5e9bb64c3/core-tree-prefab-events-20261019。两patch均未生产应用，未运行Cargo；保留Default仅兼容待owner迁移的forest，不能称真实出生。terrain独审verification.json完整落盘但审查报告未freeze：TG-e73d16b0f4be6e76bf/terrain-curve-20261018/prefab-events-independent-review-20261019。18hash和3full原PDB10233B/2329指令、GPU602B/142指令、forest5342B/1141指令核对无confirmed blocker；完整ECS/Commands消费未闭合。

- State reader：按实际snapshot父owner委派的新目录TG-3adf041a3108edb59a/main-chain-boot-20261018/core-state-reader-handoff-20261019；candidate/snapshot.rs和candidate.patch已完整落盘，基线f67ce保留真实gen。原证35full PDB42858B/11034指令、17selectors/50诊断/56Clutter/4FullNoneInnerOuter比较已落盘；Contract/validation/frozen-inputs尚未完成，明确UNFROZEN/未独审/未编译/未应用，不为提交赶验收。旧Dormer误容器草案未覆写。


完整Motomoto文件回读、实际world加载/回放、全部域与完整渲染帧验收仍未闭合。本条只保存停止断点和现代码事实；各claim保留给后续用户明确续做，不抢占或冒充已完成。
