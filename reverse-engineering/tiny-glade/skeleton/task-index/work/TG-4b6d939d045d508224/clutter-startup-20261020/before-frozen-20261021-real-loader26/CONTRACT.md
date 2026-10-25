# 真实装饰物启动 producer 26 分支恢复合同

## 范围、身份和权威输入

只修改现 `crates/systems/decorator/src/load_decorators.rs`；保留 root 转发的 TG-4b6d939d045d508224 原 owner `clutter-startup-20261020`，不改 claim。本目录 before.rs 为当前生产冻结件，SHA256 `ba7acd2da87e49cb34cbb7ea6eb825fcff857357dbcffe4c5bb6980c1cd33cbd`。父目录 retained `real-collision-producer-20261020/native-pack/report.json` 的五完整函数均 status=ok，原 B RNE SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03/PDB 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911；完整 producer 1407b66e0/17386 B，closure0 1407baad0/1069 B、closure1 1407baf00/597 B、closure3 1407bb160/833 B、bbox 1407bb4b0/1284 B。无原程序执行；机器码和 PDB 身份不外推 C。

原消费者为真实 PreStartup `fn(ResMut<DecoratorMeshLibrary>,Res<AssetMeshLibrary>,Res<MeshWatch>)`（保留旧完整1132 B安装证据）。root 负责生产 App 注册。现代入口提供 `load_decorators(root:&Path, library:&mut DecoratorMeshLibrary, assets:&AssetMeshLibrary, watch:&MeshWatch, demo_mode:bool)`，root 的三资源 PreStartup closure 捕获已有启动 root/config.dev.demo；这样不新增状态资源，亦不把 demo 硬编码 false。根目录只是测试/当前资源根适配，逻辑目录仍原 `assets/meshes/decorators`。

依赖 water worker 的唯一 canonical schema：key `DecoratorLibKey::new(subtype,rank)`；owned collision `{triangles:Vec<Triangle>,mesh:Arc<RaycastMeshData>,bounds:Rectangle3d}`；四表 insert 与 same-insert 保留原 clone/same Arc/释放顺序。outline 导出 `DecoratorOutline::{Mesh(String),Lines(Vec<Vec3>),Sphere(f32),None}` 与 collision-outline `Lines/Sphere/None`。空 collision 由原 `DecoratorCollision::empty()` 构造真实零 cuboid，绝不默认 shape None。

## 加载阶段与失败顺序

先 `read_dir(root/assets/meshes/decorators)`；失败 panic 并保留原 context，逐 entry/UTF8 错误均 panic。只有名字长度>=5、严格小写 `.json` 后缀才加载；去掉5字节得到 stem。原先切 stem[..7] 再比较 outline，因此 len>=7 但字节7非UTF8边界的 stem 必须 panic，不能用 starts_with 静默跳过该失败。前7字节 `outline` 的任意后续名均加白色，不只文件名正好 outline。先 JSON 解析→可选颜色→原名字 `decorators/{stem}`→MeshWatch.watch(&mesh)→AssetMeshLibrary.add(MeshAsset)；先订阅后发布，订阅失败不可留下资产。未排序原 read_dir，无清库、成功标记、静默跳过错误或二次 registry。

所有目录项完成后才进入 subtype 0..25 的原枚举顺序；每分支完成才更新下一个。任一 required 资产/outline加载/convex hull失败必须 panic，不回退bbox/empty/None。每 subtype 的 rank 单调从1到对应 DecoratorType.max_rank。full模式共52个 key，demo模式34个；6..9 与22/23原 guard 跳过。guard原导入14362c098的8字节 `904d634301000000` 指向143634d90进程 demo byte，现代已有 config.dev.demo 为同一业务输入。

## 原26分支矩阵

完整52条具体路径、每个 collision/interaction构造算法、outline形态和原case地址逐条保存 `branch-matrix.json`；不得由统一命名猜测替代矩阵。以下重复复用只共享原同body，不改真实算法。

| subtype | rank | collision / interaction | outline / collision outline |
|---|---|---|---|
|0|1..3|cottage Nx1 collision bbox / interaction bbox|原 cottage outline Mesh / collision_outline Lines|
|1|1..3|cottage Nx1 halfdormer bbox /普通 interaction bbox|普通 cottage outline Mesh /普通 collision_outline Lines|
|2|1..3|cottage Nx1 full_dormer 本体bbox / same Arc|full_dormer outline Mesh / collision_outline Lines|
|3|1..3|cottage corner indexed / interaction bbox|corner outline Mesh /同路径 Lines|
|4|1..3|balcony_door_rankN bbox / interaction bbox|outline_balcony_door_rankN Lines /同路径 Lines，独立加载两次|
|5,10|1..3|trap_door_cottage bbox / same Arc|outline Mesh / collision_outline Lines|
|6,7|1..3|gothic Nx1 convex / interaction convex|outline Lines / collision_outline Lines；demo跳过|
|8|1..3|gothic Nx1_dormer convex / interaction convex|outline Lines / collision_outline Lines；demo跳过|
|9|1..3|gothic balcony rankN convex / interaction convex|outline Lines / collision_outline Lines；demo跳过|
|11|1|flag_wall bbox / interaction bbox|Sphere(.3) / collision_outline Lines|
|12|1|flag_corner indexed / **flag_wall** interaction bbox|Sphere(.3) / corner collision_outline Lines|
|13,14|1|flag_roof bbox / interaction bbox|Sphere(.3) / collision_outline Lines|
|15|1|flag_terrain bbox / interaction indexed|Sphere(.3) / **collision本体** Lines|
|16|1|lantern_wall indexed / interaction indexed|Sphere(.3) / collision_outline Lines|
|17|1|lantern_terrain indexed / interaction indexed|Sphere(.4) / **collision本体** Lines|
|18|1|lantern_flat_roof bbox / same Arc|Sphere(.4) / collision_outline Lines|
|19,20,21,25|1|真实 Sphere::new(0,.5)→Mesh::try_from→indexed / same Arc|Sphere(.5) / chimney(19)、vent_pipe(20)、roof(21与25) Lines|
|22,23|1..3|arrow_slit_1xN：rank1/2 bbox，rank3 convex；interaction同算法|outline Lines /同outline路径再加载 Lines；demo跳过|
|24|1|empty() /第二次独立 empty()|Sphere(.2) /显式 None|

原7张rank路径相对指针表与绝对目标字节保留 `rank-table-binding.json`，格式化字面量精确长度见 `format-pieces.json`：资产prefix decorators/，outline读取 `assets/meshes/{name}.json`。

## 三个 callback 与 bbox helper

closure0：原先 Mesh::arrayed_indices，再 clone positions，再 clone可选 normals/uvs，交给原owned new_indexed。当前canonical new_indexed改用切片API且内部复制为真正拥有的缓冲；producer直接借用真实Mesh片段，避免外层再clone造成重复分配；indices分组三元Vec仍保留，之后真实 Mesh::as_triangles→collision_bbx，Arc在record内创建。这是复制所在层的安全API适配，不得用bbox shape代替indexed。

closure1：直接从文件 `root/assets/meshes/{name}.json` 重新解析，无 watcher或registry写；clone **未索引** positions，drop临时Mesh，取第一个点（空则panic）append到尾，得到闭合折线。已经首尾闭合也继续append，单点变2点；不得按indices展开、dedup或预造None。

closure3：required asset→clone全部positions→new_convex(...).expect("Convex mesh")，原空/退化 hull失败panic；temporarypositions在Mesh::as_triangles前释放；之后真实triangles与collision_bbx，Arc。无default hull或回退。

bbox：required资产→collision_bbx，严格assert footprint.axis==Vec2::X（NaN亦失败）。取width=size.x、height=top-bottom、depth=size.y；中心是(x,(top+bottom)*.5,z)，rotation原 `xz_rotation_bad(Vec2::new(-axis.y,axis.x))`，scaleONE。先原 `box_trigger_volume(width,height,depth,&Transform)` 真实12三角，再 new_cuboid_at(full_dimensions,center)，然后再次 required asset lookup+collision_bbx写最终bounds。保留两次lookup和实际bounds，不重算unitbbox。**core原bbox入口当前错误签名；已申请 TG-b0d4797db2453f6794 旧owner最窄委派，必须恢复现canonical原文件入口，不把box表复制进loader。**原1302B+690Bmap/558Bcollect及24点/36索引完整表在该task旧work，能够复用。

## 集成与验证

本worker仅候选/patch+源复核，不运行Cargo/GPU。root先集成TG609四表、TGda90 outline、TG450 exports和TGb0d box helper，再loader；PreStartup只有一次注册并使用真实两库+watch。测试使用实际AssetMeshLibrary/真实Mesh/真实RaycastMeshData：完整52keys与demo34keys、真实资源文件绑定/特殊路径、same Arc与独立interaction、lines重复首点、indexed与convex和bbox真实形状、empty真实零cuboid、缺资产/convex/UTF8/outline空/目录错误失败。可用原52条矩阵 fixture，但不可用mock geometry或loaded标志证明成功。watch-before-add顺序按源码与原call独立复核；当前watch_lock私有，producer测试不伪造毒化入口或声称已执行OS订阅失败注入。当前项目 JSON loader额外输入校验及 native-physics库实现差异属于已存在共享依赖边界，不在本次改写；不把它们宣称已独立重验。
