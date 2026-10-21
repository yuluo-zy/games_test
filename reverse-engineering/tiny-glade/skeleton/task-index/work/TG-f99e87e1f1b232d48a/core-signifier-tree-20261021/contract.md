# SignifierEntityBuilder 完整前置接口合同（待独立复核）

任务TG-f99e87e1f1b232d48a，owner core-signifier-tree-20261021；B原source为country-core/systems/ui/signifiers/mod.rs；当前生产分拆在signifier_factory.rs。保留已恢复SignifierFactory Arc工厂与其他UI状态，原地修复当前builder，禁止新生产文件、Cargo/lib/App重建。

## 原证绑定

native/report.json：11个PDB486方法完整范围逐指令原字节校验全ok，包含new274B与build3840B。B RNE/PDB SHA同任务输入。build原3840B SHA ca9a0fb11205bfc8f76e5ce151a7e3e5446b3909aa9b25dab7507c04296c144b，与既有round12 native Unicorn boundary probe fixture逐字节相同（previous-fixture-binding.json）。复用其各分支真实deferred payload值及失败/边界记录，但该probe隔离库/几何/Bevy边界，绝非整机/GPU验收。

## 安全 Rust 表示及公共接口

- 当前公开raw字段把typed应用对象误当任意qword，并遗失native mesh所有权；new(&[u64;6])/build(commands)无法取得原AssetMeshLibrary/Shader实参。恢复为当前同名SignifierEntityBuilder持有typed mesh/rotation/position/shader/animation/scale/lifespan/parent/pass，去掉手写unsafe Send、raw Box<shader>和无效Clone。
- SignifierMesh是原native五分支的owned表示：Named(String)、Empty、Points(Vec<Vec3>)、PointsWithColors{points,colors}、Generated(Box<dyn Fn()->Vec<Vec3>+Send+Sync>)。不再解引用原游戏指针/原vtable，不新增平行业务状态。
- builder::new(SignifierMesh)、from_mesh_name、from_points；原position/rotation/scale/animation_duration/custom_shader/set_ui_pass/minimum_lifespan/animation_easing/parent_to_widget逐一保留typed版本；mark_as_debug/disable_default_animation为原同型布尔设置入口。
- build(self,&AssetMeshLibrary,&AssetShaderLibrary,&mut Commands)->Entity；原依赖参数顺序恢复，Factory闭包本已获取二库，可直接传入。
- 当前全生产仅2个raw调用者（water draw closure0 / glade-border closure1）须同批转typed接口。更改scope仅Builder调用与真实库参数传递，不改水/边界算法；各自task已live NULL，等待root/lead协调normalclaim。core mod保持现SignifierEntityBuilder reexport，SignifierMesh可从现signifier_factory模块路径访问，无需共享mod编辑。

## 构造、五分支与副作用

- new默认rotation=IDENTITY、position=ZERO、shader="vertex_color_shader"、animation{CubicInOut tag1,flag8=0,flag9=1,value1,duration.2,t0,eased0}、scale/lifespan None、parent=false、wireframe=false、UI=true；对应new原常量/写偏移。
- Named严格查现mesh库，只借library-owned句柄；missing名称保持panic，在reserveEntity前发生。
- Empty注册真实Mesh::new("signifier empty")，不以spawn占位或default句柄替代。
- Points使用现mesh_from_pts("signifier custom",&points)，保留原点数断言；颜色通道白与顺序索引由该权威helper负责。
- PointsWithColors使用同现geometry helper，保留实际colors作为ATTR_COLOR覆盖；不假色或裁短。
- Generated调用原Fn返回真实Vec，再mesh_from_pts("signifier custom fn")；callback及临时Vec保持正常Rust释放。
- 后四分支均注册现MeshAsset并最终插TransientMesh，同一个真实句柄用于Drawable与回收凭据。
- 网格解析成功后reserveEntity；scale默认1。flag8=0时把explicit scale写animation.value，initial Transform.scale=ZERO；禁默认动画时保持value1、Transform.scale=explicit/defaultscale。
- shader严格按现AssetShaderLibrary查找，missing保持原失败（发生reserve后）；不降级默认shader，不改原失败时副作用顺序。
- 插真实DrawableMeshBundle(Transform/mesh/shader)与SignifierAnimation，然后Wireframe优先于UI（两者互斥）；必插DontCastShadow；可选lifespan(t0,min)/parent；动态mesh最后插TransientMesh。
- builder不插伪造SignifierId(0)，ID属于signifier_create按name分配。

## 生命周期真实consumer

Factory::from_closure → stream::Factory → signifier_create去重/唤醒 → 返回Entity赋SignifierId；mark_for_delete/animation/despawn维护实体。
TransientMesh::Drop仅向同Arc<Mutex<Vec<Handle<Mesh>>>>入队 → country-core asset_libraries/runtime.rs PostUpdate AssetMaintenance::Reclaim → delete_dropped_transient_meshes（释放锁后mesh.remove，再vao.remove） → 后续生产维护回收GPU对象。当前真实consumer存在；本任务不重写它。

## 配对树光标与范围

接入同builder的Empty分支 + duration0 + Ui2dExtent/Ui2dImage("seed_cursor_icon")，按UI task ui-icon-contract.md原size/位置算法恢复owned factory，消除每帧32B leak与无闭包占位。第二阶段树patch必须以第一阶段producer candidate为before，单独SHA，不能覆盖第一阶段。

## 验收

root执行测试：5 mesh分支真实句柄/mesh内容、Named无receipt/后4支receipt、drawable/pass/flags/animation/position/scale、失败顺序、callback一次调用/释放、drop Entity→receipt→mesh与VAO回收；树光标owned factory wake与同名去重/移动回收。
child不运行Cargo/rustc/GPU，不把前置builder实现冒充整UI宿主/GPU完成；保留对原私有native库不透明容器的边界说明。
