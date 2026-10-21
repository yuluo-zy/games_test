# 墙烟囱两分支真实动态碰撞合同

## 范围、原证和owner

TG-1cafd83cc5b4685781原owner core-visual-data-20261021。root经lead授权其原canonical直接consumer必要闭包；只改mapped chimney_wall.rs，不新文件/registry，不扩视觉装配族，不Cargo/GPU/生产写。before7e56477796f7250ac5095c618922c37b887f97a5e5c95c895699885c86ece8d1。完整B1407c21b0/12159B及真实callee符号复用十源native-pack，RNE f507a6ce…、PDB2d9686b8…；所有机器指令/常量按同身份原发行文件。新增relative-height-pack三原段证明Public+78是outline.y减真实Terrain.sample_at_world_xz，不凭字段名挑表。

本合同恢复完整collision producer与canonical slots/source消费，不声称10f32的烟囱装配盒、ventpipe视觉shader、烟雾参数、其它blueprint visual payload已完成。当前is_valid_body_placement(info)->true没有实际消费者必需输入，必须重构成真实多输入过滤，不留默认true假验证。

## 共同行为和输出

原 subtype(19/20)required来自唯一ID map，非这两种直接跳过。cached transform required后，2a19先insert_for(真实当前Wall桶,info.state())，2a6b as_chimney required，2a9a..2aba真实cached transform写入唯一entry。不得把as_chimney/geometry的失败提前到entry之前；不再legacy new/末尾insert。

原没有现最外层info.dst_tag>=2的默跳门。19分支2b2f真正dst_as_wall required；20从asset构造不读墙目标载荷。只保当前现有Wall桶循环的范围，原跨Terrain/Stair存储迭代仍未恢复，不扩大任务。

原416b new_indexed：完整世界空间Triangle顺序flatten为vertices，indices=[0,1,2],[3,4,5]…，normals/UV为None。不能weld/去重/改用new_unindexed另一hash算法；当前new_indexed借slice的canonical API内部复制，由外部建唯一所需vertices与indices。原新外Arc克隆一次，4270两个slot同Arc；pose **ZERO/IDENTITY**，因为全部点已经变换到世界空间。set_cached_transform仍保原未烘焙缓存用于其它visual输入，不再次乘进collision。

## subtype20 vent pipe：真实资产分支

2ffd进入20分支，32a2 required AssetMeshLibrary.get_by_name("decorators/vent_pipe_collision")；原142a86700 len30真实ASCII已直接读RNE。必须新增同权威AssetMeshLibrary的Res，原loader26已先注册此真实JSON资产。原collision几何不是loaderSphere/bbox、也不是单位cuboid。

原仿射输入为cached rotation、cached scale，translation=cached.translation+cached.rotation*(0,0,0.28)，0.28原142a80cd0=0x3e8f5c29。原140a08280 Mesh::as_triangles、spec_extend14125fe70映射每个实际三角的3点到该Affine3A，保重复点/输入顺序，后走common new_indexed。原其它ventpipe视觉offset(-.07,1.42,.58)不是collision变换，不混进物理点。

## subtype19 body、lower和fallback

1. 2b0a required真实PublicWallState.max_y；2b2f required Info.wall_space_coord。以原未路径处理outline（Public+38/+40/+48，现geometry.outline）通过共享CurveSlice remap XZ extend_right(max(dst.u,0))取segment,t；原2bef/2bf6是(1-t)*outline[i].y+t*outline[i+1].y。不使用height_outline()取错prepared curve。共享CurveSlice::from_coord_range_with_remap借原[[f32;3]]，不每装饰clone曲线或重算cached length。
2. 原full_height=extra.f+(max(max_y-dst.y,0)+3.0)。原cached transform的scale改成(.85,full_height,.85)；body.center=(cached.x,cached.y+full_height*.5,cached.z)，再分别加q*(0,0,1/3)和q*(0,0,.15)，保两次原f32求值，不合并.483333。q与其它变换字段保持真实值。
3. lower_candidate初值为false，lower_height初值0。若authority.elevation()>0，按main body XZ在原geometry.outline共享approx_curve_coord，插值geometry.relative_heights；值>.5直接不尝试lower。NaN比较不触发此跳过。Public+78的producer原a8ce11..38→d38、d809/810→scratch+578、dd06..3d publish已另取真实260B；不得换terrain_y或正值clamp。
4. diff=dst.y-第1步outline插值Y。原2dee为有序diff<5才尝试lower；>=5或NaN保lower_height=0。允许时lower_height=max(diff,1.5)。lower Transform复制body，scale=(1.75,lower_height,1.2)，translation.y=(body.y-full_height*.5)-lower_height*.5，x/z和rotation同body。原lower过滤的wall-coordinate center_y另外为dst.y-lower_height*.5，不可用含旋转offset的世界center.y取代。
5. 原2e7b WallSpace为shape.tag()!=2时闭环，否则开段；length=wall.wall_space_length()。lower_u=space.project_unbounded(dst.u)。使用真实prepared_path.positions（Public+88/+90/+98）共享CurveSlice remap XZ extend_right(lower_u)，借wall.arch_segment_heights()（Public+e0/+e8）按(b-a)*t+a插值。prepared/height缺失或segment+1越界均required失败，不造0/default。负distance保持共享原assert，不悄悄clamp。这里只取借用，未新建Curve/Vec。
6. 37c2..3962遍历当前墙桶全部真实decorators，不限modified；原self也先执行required取表，最后才排除self的overlap。tag0/1用其ID→subtype和Info.rank exact collision_record；height=bounds.top-bottom，width=bounds.footprint.size.x。若(height*.5+lower_height*.5)>abs(other.dst.y-lower_center_y)且space.distance(other.dst.u,lower_u)<width*.5+1.75*.5，则除自己之外的命中拒绝lower。两个比较都严格有序，边界接触不算overlap。tag2原Roof条目不重叠；tag3/4是原unreachable失败，不能忽略或factory默认值。
7. 无其它overlap后4284/4289按原顺序计算limit=(lower_center_y+lower_height*.5)+.3；只有arch_height>limit有序成立才继续。42bf按body XZ在真实prepared positions共享approx_curve_coord；+c8/+d0的prepared_path.segment_values按(b-a)*t+a插值，必须恰==0（NaN拒绝）。以上全通过，4343先追加lower的真实box_trigger_volume(1,1,1,&lower)，设lower_valid=true。
8. 所有路径3997追加body真实box_trigger_volume(1,1,1,&body)，保原24点/36indices/12Triangle顺序由唯一canonical box入口生成。若lower_valid为false，3a2c复制body另建fallback：scale.x=body.scale.x-.2，scale.y=.4，scale.z=body.scale.z-.2；center.y=(body.y-full_height*.5)-.2，随后3ad3追加此真实box。因此成功lower路径24Triangle，拒绝lower路径24Triangle；次序/真实几何不同，不能只按数量验收。

## Rust与测试

复用原现存PublicWallState、DerivedDecoratorInfoResources.as_view、Mesh::as_triangles、CurveSlice、approx_curve_coord、WallSpace、canonical box/new_indexed。现源内私有helper表达这些有界算法，不引入flat registry/host镜像；required失败和entry部分mutation保留。借用owner不deepclone collision record/triangles作为旧colliders。Vec按真实Triangle/索引数量reserve，不每次采样创建曲线。

测试使用实际AssetMeshLibrary不规则indexed mesh验证20分支经完整system生成世界trimesh，非box/球；确切worldpoints、indices、None normals/UV、同Arc/ZEROpose及source/flags。19用实际WallState publication/prepared路径/height缓存与真TriangleVec：有效lower、邻居exact-rank重叠、自身排除、strict边界、elevation相对高度门、path非零、arch边界、missing required表及sourceentry失败次序；按原固定输入的明确完整点/面与原box顺序核，不把实现重算一遍当oracle。完整宿主/真实GPU交root。

## 常量与无序比较附录

collision-constants.json逐项直接读同RNE section映射与SHA，含全部碰撞常量VA/bytes/bits/f32以及机器literal；特别1429258d8=bytes9a99993e/bits3e99999a=.3，已改正旧反编译引出的十进制猜测。只以真表值实现。

原2c09 MAXSS(max_y-dst.y, +0)在NaN/相等时取第二operand+0，Rust helper为if value>0 {value}else{0}，不反转参数、不要用会保NaN的其它max。初outline distance同2b81 MAXSS(dst.u,+0)。lower diff<5必须有序成立，否则3976先清lower_height；随后2df8 MAXSS(diff,1.5)在NaN/相等取1.5（本分支已排除NaN）。relative>.5、elevation>0、arch_height>limit、邻居两个严格比较均为ordered，NaN不触发拒绝/通过对应门；最终path ==0要求有序相等，NaN拒绝。原const+0/-0和两个body offset分开计算不合并。
