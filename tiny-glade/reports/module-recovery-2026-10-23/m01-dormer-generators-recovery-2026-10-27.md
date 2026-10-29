# M01 两类老虎窗视觉生成器收尾，2026-10-27

本轮先实读原 B 骨架 `crates/systems/decorator/src/decorator_visual/gothic_dormer.rs` 与 `cottage_dormers/{cottage_dormer.rs,collision_geo.rs,assemble_dormer_roof_tiles.rs,util_assemble_gothic_dormer.rs,maintain_dormer_tiles_draw.rs}`。恢复仍在对应既有原路径：两个入口，及 collision_geo 内既有 roof_section 的同族可见性。没有新增业务文件、目录，没有开展 M09 装饰工具 UI，没有修改共享 Blueprint、renderer 或 host owner。

## 本次否证的旧完成面

旧 cottage 入口只写凸体、canonical transform 后创建 WindowVisualEntityBuilder 并丢弃；`get_dormer_panel_xforms` 恒 None。旧 gothic 只写静态碰撞与 legacy outline，builder 同样丢弃，也没有 canonical transform。legacy `(0,0,0)` 描边不满足实际 fullvisual 的 Option<Transform> 门，不产生真实画面。两旧入口参数也不等于原 FunctionSystem：误有 transient state，缺 PublicWalls/Roof/UiSignifierStream 等。

原 GothicProfile 数据表不是 generate_gothic_dormers 的 callee；其独立消费者是 generate_dormer_roofs 和 cull。不能用“剖面表测试通过”代表主生成器完成，更不能因为主函数没调用剖面表而强塞一个默认网格。

## 原身份、参数与证据

B RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；B PDB SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。

| 入口 | 原 VA / PDB code_size | 原 SystemParam 顺序 |
| --- | --- | --- |
| generate_cottage_dormers | 1407c7ac0 / 11257B | DerivedDecoratorInfoResources, Res<DecoratorStorage>, Res<CachedDecoratorTransforms>, Res<PublicWalls>, Query<&Roof>, ResMut<DecoratorBlueprints>, Local<DormerCollisionGeoLib>, Res<UiSignifierStream> |
| generate_gothic_dormers | 1407c5650 / 7856B | 同上，去掉 Local |

PDB code_size 含分离 EH 区，不把反编译文本 KB 当函数指令大小。原 C、完整对应 ASM、FunctionSystem 参数证据及 shared 只读查询结果见同目录 `m01-{cottage,gothic}-dormer-native.*`、`m01-*-dormer-system-signature.txt`、`m01-dormer-native-reads.json`。未修改共享 Ghidra 数据库。

## 恢复的业务输出

共同入口保留真实 per-wall bucket 的 modified 集合、required subtype ID 查找、存在且非 flat 的实际 Roof、真实 cached transform 和 flags；不扫描全部装饰物，不为缺资源造默认值。生成只写 Blueprint，由现有 fullvisual 统一创建/替换/卸载持久实体。

cottage 原子类型2、rank1..3宽度0.8/1.5/2.2、高度1；原动态 convex 与量化 Local 缓存复用原 collision_geo 实现，两个 positioned mesh 槽保持同一 Arc，位置/旋转来自原 cached pose。canonical transform 明确写入，删除无意义 legacy outline。

原面板内联区间 `1407c82ec..1407c8f04` 的完整三分支已恢复：

- rank1：先取得当前 roof section/最近 u，然后保留原整个 pose；不制造曲线或改 scale。
- rank2：半区间 `(width/perimeter)*0.25`；在 circular u 两侧分别取当前截面 Curve3XZ 的真实位置、切向，投影法线、再归一化，经完整 xz_projective_rotation_bad 求各自旋转；scale1。
- rank3：半区间 `(width/perimeter)/3`；左右同样由真实曲线求位置和旋转，中间由最近 u 的真实位置配 cached 原旋转，三面板有明确顺序；scale1。

这会在矩形角附近及圆屋顶产生不同的面板旋转，不能替换成中心±width平移、单一 box 或默认法线。复用原 Shape/Curve/rotation 内核；只有原分支需要时才构造曲线，不逐装饰复制 mesh 库。

每个 cottage panel 写原 `decorators/window_cottage_1x1_dormer_frame` 与 `_glass` 两个 builder。frame ColorId 来自 Info+28；glass ColorId 来自严格 window extra；window light key 为 `rank*1000 + 实际map id`，保留 state 的 SemiTransparent。每 panel 的 cottage hole 按完整 quat 把 `(0.05,0.05,-0.05)` 转世界，再加世界Y0.5；尺寸 `(0.8,1,0.2)`、真实正面XZ、固定尾 `(1,-50)` 保留。花台候选保留原 seed suppression 条件、pinned fastrand 的抽取顺序与6项索引、原 `(DecoratorId,"flower_bed",panel_index)` hash，位置Y-0.1和原 wall key。

gothic 原子类型8：先读取真实 collision library bounds，insert Blueprint，再严格 roof dst 与 PublicWalls 原未校验查表读 style（get_unchecked）；两个原 rank mesh 名分别是 `window_gothic_{1,2,3}x1_dormer` / `_glass`，没有通用盒替代视觉 mesh。frame ColorId999选择 nani_solid_vertex_color_bricks，其余选择原 window shader；BrickColor 保留 seed、wall低32位、roof coord起点、起点+原rank宽度0.72/1.2/1.54、宽度和halftimbered字节。这里不能套墙窗 `u±width/2` 公式。collision 与 interaction 来自两张真实库，canonical transform 同原 pose。glass 保留原 window Color/light/albedo `NameHash("window_glass")`。

原 gothic hole `1407c6af0..1407c6cf1` 位置为 cached translation +世界Y截面中点+正面XZ乘 `(-extent.z*0.5+0.05)`，保留原乘加次序；native 并不使用 bounds 的XZ中心。完整 extent 和正面XZ、1.04、rank尾 bits `{3f1ef9db,3f1ef9db,3f0f9db2}` 写入真实48B记录。cottage hole 区间 `1407c962c..1407c979f` 同样执行了原完整 quat/constructor 链核对。

## 原指令差分与真实业务验收

`m01-dormer-panels-native.py/.json` 执行原整个面板内联区间，12组覆盖三个rank、近角、旋转矩形和圆截面。只挂已选择的 roof section 边界、系统分配/重分配/释放和外部 CRT 数学函数；Shape dispatch、Curve 构造/参数化/求段、插值、法线及两次归一化、原旋转与完整 Transform 写面均实际运行原指令。输出与 Rust 的十个 Transform words 逐位匹配。

圆分支的外部 Windows CRT 主体未执行，桥明确绑定 host C **float** `sinf/cosf/atan2f/hypotf/ceilf/fmodf`。最初误用 Python f64 math 再转 f32，95点中5点相差1~2ULP，累计 length 差2ULP，导致某 panel Z差37ULP。没有放宽断言或改游戏算法。修正数学桥后，独立 Rust probe 与原指令构造的95点、95个 points_u、length 全部逐位相同，12组面板全通过。此结果证明相同数学边界下的游戏算法，不伪称原 Windows CRT 与 host libm 已逐位等价。

同脚本产出 `m01-dormer-holes-native.json`，两孔原完整写面18组（每族三个rank×单位/Yaw/Pitch四元数），全部12 words 原样逐位匹配，含墙ID、tag/padding真实写面和rank尾，不只测 constructor 或轴向孔。

`CARGO_INCREMENTAL=0 cargo test -p system-decorator --lib dormer --offline -- --nocapture`：最终实际 **38 passed / 0 failed**，日志 `/tmp/m01-dormer-all-tests-v4.log`。原生预期失败测试内部 caught panic 不计失败。此前 v3 的37/1失败保留为上述数学桥定位证据，没有以旧family测试替代。

新增 `both_dormer_families_reach_real_fullvisual_hole_grid_change_and_delete` 对两族×三个rank的6个真实 World：

1. 真实 storage/subtype/cached transform、已发布 PublicWalls、关联实际 Roof、canonical collision library → 原入口 Blueprint。
2. 实际 instantiate_blueprints → 真实 NaniInstanceDesc/Transform/ColorId/WindowLight 实体与持久视觉 owner；不直接调用 builder 冒充 fullvisual。
3. 同批真实48B记录进入生产 build_dormer_hole_grid，逐记录验证真实单元包含；不从 colliders 推造孔。
4. 同一原 Arc 稳定帧保留 Entity ID；modified/clear/cached pose更新后由 producer/fullvisual 替换、旧 Entity 卸载，孔位置同步变化。
5. 原 storage.apply_decorator_remove → clear_blueprints → 两入口 → fullvisual：蓝图、持久实体、两类 collider owner 全清，原孔 grid 无旧成员。

既有真实 Local Schedule 测试仍验证动态 convex 缓存复用原同一 Arc、非法rank及失败 hull 的部分写入边界；源码资源初始化夹具已按原 Derived SystemParam 两owner与 UiSignifierStream 修正。没有 OS 输入、没有 GPU 画面声明；生产挂原19组和实际 GPU 上传由 root统一验收。

## 明确保留的下游缺口

独立 `generate_dormer_roofs1407cb1a0` 及 `assemble_dormer_roof_tiles14128db20`、`assemble_gothic_dormer_tiles141290b30` 的原复杂瓦片算法在当前文件仍有旧空实现/None，**本轮没有以简单瓦片或默认网格替代，也未把它们算作完成**。两入口已有完整 frame/glass/hole，不能据此宣称完整 dormer 屋顶瓦片完成。maintain_dormer_tiles 既有真实删除资源分支不因入口修复被重写。

## 复用经验

- 原函数的 PDB SystemParam 先核顺序和 owner；生成器通常只写蓝图，不能额外 spawn 或持有独立 transient state。
- complex rank 分支要沿 inline host 的真实 call 到 Shape/Curve/rotation；局部 Vec<Transform> 的数量不证明它只是线性平移。
- 差分失败先拆采样点、参数缓存、数学桥，再改 Rust；`sinf(f32)` 与 `sin(f64) as f32` 不能互换为“同一数学函数”。两ULP的累计长度会放大为几十ULP插值位置。
- 两孔必须核完整 producer write-body，constructor 表本身不能解释位置、局部偏置或rank参数。
- 原 required 资源缺失、查表顺序和部分 mutation 都保留；get_unchecked 与 get 的验证合同不可混用。
- 真实 World→唯一 fullvisual→删除验收与原指令差分互相补充，旧类型分类测试/单独 builder 测试都不能代表画面和卸载完成。
