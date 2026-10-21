# 农舍墙窗完整主函数 B 合同（数值与发布顺序）

这是原 B 14303B 主函数的恢复设计，尚无通过验证的场景 producer。复用完整 1407ebfd0 原 RNE 汇编及原六/七参数 wrapper、helpers 合同与勘误；不把旧 fresh 反编译中的坏指令/缺参数当依据。root 已明确延续委派，原 claim 不变。C 行为一致性未验证。

## 唯一参数及权威

原七参依次为 DerivedDecoratorInfoResources、Res<DecoratorStorage>、Res<CachedDecoratorTransforms>、Res<PublicWalls>、ResMut<DecoratorBlueprints>、Res<AssetMeshLibrary>、Res<UiSignifierStream>。只生成蓝图与视觉 builder，不生成 Commands、不额外持有 transient owner。实际 modified key 与 outer WallId 分别保留；stored Info.decorator_id 只用于真实随机及 Cloth 判据，不拿它替换 modified key。

唯一库接口依赖 core lead 的 TG609：View::subtype(DecoratorId)，View::mesh_library() 同一四表，required collision_record/interaction_record 借用真实 {triangles,Arc,bounds}；每窗只取 bounds 副本/克隆两个 Arc，不复制 triangles。Info::window_extra() 原+30 u32/tag+41==6 由 TG431 提供。禁止 bbox、默认几何、None成功、另一registry。

## 入场与原失败/部分写入顺序

1. storage每墙每modified项必查 subtype；只有0/1进入（ec58f..591），其余跳过。必查 actual outer PublicWallState；必查 key::new(subtype,rank) 的 collision bounds，保留 missing-key panic。
2. 调 blueprints.insert_for(actualModifiedId,Wall(actualOuterWall),info.state)，保留旧 allocation 唯一性与 visual Vec 复用。随后才校验 rank.wrapping_sub(1)<3；无效 rank 失败时已经发生原蓝图替换。
3. shape.tag()==1 用实际 cached.transform(modifiedId)。其他形状先要求 wall destination tag<2，再取原坐标u/y。左右u=coord.u+width*(-.5)/coord.u+width*.5，分别用同 WallSpace.project_clamp；借真实 simplified_curve_2d 读世界点，只取XZ，分别乘.5后求和为位置XZ、原coord.y为位置Y。方向取端点差，倒数为1/sqrt(dx²+dz²)，法线为(-dz*inv,dx*inv)，复用utils::xz_rotation_bad；scale=ONE。WallSpace来自tag!=2与原curve length。
4. shape.tag()<2 的 pose.translation 加 rotation*Vec3(0,0,f32bits3e0f5c29=.14)。其余保持已算pose。
5. 然后才分别查实际 collision/interaction，并克隆同owner Arc；碰撞位置在premade时加 rotation*Vec3(0*depth,0*depth,depth)*.5，depth为collision bounds.extent().z。非premade为原pose位置。调用原set_positioned_colliders；随后写一次真实outline Transform，再追加主框builder。先前blueprint变更不得用事务回滚掩盖原panic边界。

## 框/玻璃/墙洞

所有rank主框依次为 decorators/window_cottage_{1x1,2x1,3x1}，nani_solid_vertex_color_window，pose原scale，ColorId=Info.u28完整bits，semi=Info.state!=0，其余组件保持原构造器None/tag2状态。

仅shape.tag()<2追加对应 *_glass：pose.translation减 rotation*Vec3(0,0,f32bits3d8f5c29=.07)；ColorId=required window_extra.0；window_light=1000u64*rank+u64(actualModifiedId)；shader=nani_solid_vertex_color_window_glass。这个 key 经过已通过源码审/CPU测试的 canonical builder→WindowLight→control→NaniGlass，同World无新TypeId。

接着所有形状调用dst_as_wall后追加原RawWallHole{center:[coord.u,coord.y],dimensions:[width,height],kind:0}。height=collision.bounds.top-bottom。circle(tag0)宽度另按rank加bitsbd4ccccd(-.05)/bdcccccd(-.1)/be19999a(-.15)，其余宽度原值。不对负宽度夹取。

## 第一段随机与分支

先 with_seed(utils::rng::init_rng(Info.decorator_id))。rank2先且只先消费一个完整u32，再取其bit31为shutterFlag；rank1/3标志false。这个消费发生在subtype1判断之前，不能移动或换全局RNG。

subtype0：rank1/2用真实wall subdivision，pieces=rank，width用原collision width（不是circle修正后的hole width）；将实际段Transform收集后调完整add_lintels。y_parameter=0、flags=0；callee读取但在flags0时不影响结果的height槽不可作为几何输入宣称。rank3不调lintels。非premade后续均转broken-glass。

subtype1且premade：局部复制bounds，只将footprint.size.y改为bits3f2147ae(.63)，不写回库。forward=rotation*Vec3::Z。第一个原DormerHoleSsbo::cottage(first_u32=outerWallId低32位，position=pose.translation+forward*bits3f8a3d71(1.08)*.5，dimensions=[width,(height+0)+.2,.63+.5]，tail=[forward.x,forward.z])。wall低32投影有完整4B原叶与caller写edx证据，未把元数据别名认作函数归属。

shutterFlag为true时第二洞：position=pose.translation+forward*(.63+.5)*.5；dimensions=[width+.7,height-.2,.5]；其余同上。然后追加对应 *_halfdormer_frame 主框（同pose/u28/semi）。subtype1且nonpremade直接转broken-glass，不发布这两个洞或half frame。

## nonpremade broken-glass

复用完整flowerbed_xforms(rank,subtype,parentPose,实际AssetMeshLibrary)。其真实 mesh positions 与 normals zip到较短长度；不按parent.scale缩放位置；parent.rotation*rotation_from_normal(n)、parent.rotation*p+parent.translation、原parent.scale。

每个实际采样Transform先用原同一RNG的usize(0..3)选 decorators/window_cottage_1x1_glass_broken_{0,1,2}（与位置索引无关），shader=nani_solid_vertex_color。位置先把采样translation.y加.8，再加采样rotation*Vec3(-0,-.1,-0)；rotation/scale来自采样，semi仍Info.state!=0，ColorId=None且无window light。所有nonpremade到此结束，不落入后段Cloth/花床/shutters逻辑。

## premade 的Cloth、花床候选、shutters

独立重新 seed 同 Info.decorator_id。判断 storedId.wrapping_mul(0xaaaaaab6).wrapping_add(0x5556884b)<=0x55555555 且rank<3。真时新Box<ClothInfo>{transform:pose,wall:actualOuterWall,dimensions:[已修正holeWidth,height],decorator_id:actualModifiedId}，按原替换/释放旧Box；不消费后段随机值。此条件不能简化成未经证明的mod3。

条件假时，消费u32取高2位；仅值1生成花床候选。复用同真实flowerbed_xforms，对每个采样索引i：key=原Sip13 zero-key calculate_hash<(DecoratorId(actualModifiedId),"flower_bed",usize(i))>；用同RNG usize(0..6)选真实mesh_index；追加canonical WindowAutoClutterCandidate{kind:WindowFlowerbed,rotation/translation/scale:采样,key,wall:outer}。零样本仅在真实属性为空时合法，不构造默认花床。

最后若shutterFlag，始终追加right/left两个canonical候选，即使前面已经写Cloth。right的位置=pose.translation+rotation*Vec3(width*.45,0,.2)，left=pose.translation+rotation*Vec3(width*(-.45),0,.2)，scale/rotation原pose；kind WindowShutter{side:false/true,native_u32_04:Info.u28}；key分别hash<(actualModifiedId,"shutter_right"/"shutter_left")>；wall取actual outer，不取Info平台/modifiedId。完整旧勘误已证明复用栈槽不能跨覆盖传播。

## helper与验证界限

flowerbed_xforms与add_lintels必须按已存完整473B/2637B合同恢复，去掉旧LintelRecord/Visual镜像；真实RawWallHole与builder写到同蓝图。分段TG652候选v2已经独审，root待跑5个真实Curve2测试；跨目标任意NaN payload不承诺逐bit。hash新完整406B/434B原包证明初始Sip13零key、u32 ID/str+ff/usize顺序；依赖复用std DefaultHasher，不另写SipHash算法。

下一步源候选必须测试真实库、actual storage/PublicWalls、modified vs stored ID区别、全rank/subtype/shape输出和panic部分写顺序；原全链挂载与26case真实loader仍是独立gate。本文不把14KB注释/候选case数当作源完成，不声称C/游戏/GPU行为验证。
