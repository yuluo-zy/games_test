# 首批墙启动闭合合同（2026-10-19）

本候选仅新增 `candidates-startup/`，不修改生产；v23-22冻结快照未变。中央独占Cargo，本候选只经rustfmt语法解析，尚未类型检查/单测，更没有真实GPU验收。

## 原证及本轮纠错

B `glade_startup_walls` PDB 140a92670/2320B。原文件指令在`wall-startup-original.asm`，原RO常量在`wall-startup-constants.json`。原完整函数直接原生执行结果在`wall-startup-native-policy.json`，脚本`probe_wall_startup.py`；文件Mesh加载、MeshWatch/library、shader编译/library及Bevy spawn_empty是显式记录边界，不能把该fixture当作这些宿主实现或GPU执行验证。

原18索引 `[7,5,4,4,6,7,3,7,6,6,2,3,1,5,7,7,3,1]`，Mesh label=`brick half cube`，登记名=`half_cube`，source=None、attributes为空。上轮“half_cube缺顶点”是错误归因，现按原实际index-only设计修复；不得以generic cube补属性。

原调用顺序是先读/登记brick，再构造/登记half_cube，再注册LOD0/LOD1 shader，最后按顺序创建七实体。每实体identity Transform、DrawableMeshBundle和同一个canonical三字节WallBrickInstancedDrawProxy：

|顺序|网格|shader|proxy已定义字节|OcclusionCulled|
|---|---|---|---|---|
|0|brick|LOD0|0,0|是|
|1|half_cube|LOD1|0,0|是|
|2|brick|LOD0|1,0|是|
|3|half_cube|LOD1|1,0|是|
|4|half_cube|LOD1|0,1,0|否|
|5|half_cube|LOD1|0,1,1|否|
|6|half_cube|LOD1|0,1,2|否|

前四command executor1412d6640的原PDB泛型明确包含OcclusionCulled；后三1412dd190只含DrawableMeshBundle+proxy；无KeepOnLevelUnload、DesignatedRenderPass、NaniInstanceDesc。常规第三字节未定义且callback不读；安全Rust初始化0，并不猜原枚举语义。与Nani startup相同真实mesh/shader/transform bundle owner，选择墙特有indirect callback，不新增Nani实例分配。

## 最小装配（中央/flow owner）

须移除run_game旧OnEnter tuple中的`startup_walls_meshes`，以免重复brick资产、shader+ECS仍缺。在设备侧原OnEnter(Game) batch2位置挂新函数，使用同一个ApplicationShaderLibrary Arc：

```rust
let wall_backend = hosts.shaders.clone();
let wall_root = installed.clone();
app.add_systems(OnEnter(AppLoadingState::Game),
    (move |meshes: Res<AssetMeshLibrary>, shaders: Res<ApplicationShaderLibrary>,
        watch: Res<MeshWatch>, mut commands: Commands| {
        country_core::startup::startup_walls::glade_startup_walls(
            &meshes, &shaders.0, wall_backend.as_ref(), &watch, &wall_root, &mut commands);
    }).after(country_core::startup::startup_session::load_game_session));
```

此片段刻意不加原证未证的run_once。普通OnEnter/重载生命周期由flow owner据原调度裁决。通过当前AssetMaintenance::Rebuild真正建立index VAO，PostUpdate完成后DrawLists才查询Handle<Vao>，不能把手造VAO身份测试当成GPU验收。

第一批InnerWalls::load_state会对每id写created；19 publisher `take_changes`后产生OnWallChanged。v23必需同帧真实链：publisher已有四栅格prepare_paths → calc_wall_bend → clear_constructor_planks → raw segmentation（MaterialStats reset/count）→ init heights（先base/flag再rectangle真实association修正）→22 roof map/invalid通知→split MinusStairs/step ranges→arches/lintels→28gate lifecycle/gates/fences/mainwall真实producer→44shared sparse flush→PostUpdate main InstancedWall同步/真实VAO→DrawLists。

无roof `max_y_gable`在19init之后为实际base；prepare之后init之前仍必须panic，不能使用getter fallback。candidate v23的getter/projection/publisher+init必须整组合入，不能只改init wrapper。

19已有publisher tuple不能另挂一次以免重复take_changes；core construct_wall_bricks与wall-constructor reexport同一producer，只注册一次。原set22五项必须同一chain_ignore_deferred；原set28尚缺platform creator/pillars/floors的完整wrapper，不挂TODO来凑完整。主墙geometry sync现已在PostUpdate AssetMaintenance::Geometry，不重复注册。原空sample旧壳不可挂，publisher已执行真实path sampling。

必需资源/事件最小组已交flow；尤其canonical SharedAssetShaderLibrary、ArchSegments、BuildingHadArchesLastFrame、Gates、WallConstructorOutput、raw/MinusStairs/PlatformStairsStorage必须在第一次Update前存在；同TypeId init只保留现owner。Bevy add_event需保留真实Events更新，不能只World插Events掩盖保留期。显式RecalculateWallBricks两个旧TypeId问题由flow以pub use同identity收敛。

## 验收边界

新CPU单测覆盖真实七实体identity/type/proxy/OC标记→实际build_draw_lists，期望Shadow7/Gbuffer7/OC2 4，共18条；half_cube当前VAO构造实际生成72B index buffer、零vertex bindings；三阴影mode仅选对应proxy。core test_device仅CPU描述记录，未声称实际设备上传。

真实首批三墙场景仍须中央证明：InnerWalls loaded3→PublicWalls3、publication failures/path_failures空；19后prepared/cache/gable已初始化；raw/processed masks实际非空（按墙style适用）；28每个适用main InstancedWall records非空；44 constructor gpu.last_device_error=None、其真实capacity failure策略可观测；PostUpdate WallBrickBuildState.failures空、WallBrickUploadStatus.gpu_ready且last_error=None、每适用实体allocation非空；7墙drawable具有真实VAO，DrawLists含上表7entities并且W21/W27正常batch非空；最后间接命令/SSBO真实GPU回读与最终显示目标执行。

当前warmup只要求final texture物理execute/submit完成，合法空DrawLists可通过；这是设备暖机合同，不是上述墙业务闭合证明。不得加入“必须固定3墙”的通用游戏policy（合法空glade存在），3仅属于已加载本fixture预期。错误检查应挂宿主验收/诊断，不猜原正常游戏gate。
