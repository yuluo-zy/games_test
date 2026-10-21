# 农舍老虎窗真实碰撞几何闭包合同

状态：完整原函数/实际消费者只读已锁，等待独立审核后写候选。唯一source writer为 core-dormer-collision-20261021；原外层visual由tree_commands写。仅此mapped collision_geo.rs，不新建生产文件。

## 身份、原证与旧误判

原B RNE f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03 + matching PDB 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911。完整 get_collision_mesh14128a0b0/10280B，PDB1539:2044，native-pack/get-collision-mesh-full.{json,asm.txt}；report两请求statusok，实际bytes全验证。C/B该source checksum相同只支持此范围复用，不把B运行验证宣称C目标验证。

旧注释“递归+两子网格”错误：完整函数没有递归，141289c80/496B是core::str::rfind（PDB1539:656），位于puffin source路径处理。公开依赖分配器、std HashMap、Arc、round/fmod、glam inverse、原ShapeParameters几何直接复用，不按独立业务恢复。原decomp只导航，最终以完整机器码/PDB判定。

## API、实际owner与调用者

保当前名字DormerCollisionCache，但map真实值改Arc<RaycastMeshData>，没有第二cache或Vec影子输出。原type叫DormerCollisionGeoLib；现32条以上PDB泛型签名直接锁为原外层system Local，cache-owner-pdb.json保其导航。拟函数：

```rust
pub fn get_collision_mesh(
    transform: &CachedDecoratorTransform,
    width: f32, height: f32, rank: u8, padding: f32,
    roof: &Roof, cache: &mut DormerCollisionCache,
) -> Arc<RaycastMeshData>
```

真实consumer原1407c8266：cachedTransform，width按rank[.8,1.5,2.2]，height1.0，rank，padding.2，同真实Roof，Local cache。返回的同一Arc在1407c82af作为collision/interaction两实参克隆（不同于trap两新Arc）。不让caller造key，不读decorator库bbox/mesh代替动态形状。consumer包由tree提供：TGdfec/core-visual-data-20261021/before-frozen-20261021-canonical-visual-data/native-pack/tg-4a10377b42911028e3。

## 读取、量化与cache命中顺序

1. **先调用roof.height()，后校验rank1..=3**；rank对应因子[.1,.15,.2] @142c28f60。取实际transform.translation P。bottomShape=roof.get_roof_shape_from_pos_ws_clamped(P)，先读其perimeter L。
2. hp=height+padding；frontBaseY=hp+P.y；topY=(roofHeight*factor)+frontBaseY（不可重排）。topShape在(P.x,topY,P.z)取同真实roof截面。两shape分别get_closest_u_from_pos(P.xz)，再分别get_pos_at_u，得两最近表面XZ位置。
3. key native+0/+4/+8为**u32**，+c为ranku8；旧i32(x,z,count)/i8(roof_idx)是假语义。key0=round(curvature*5) asu32；key1=round((topY-frontBaseY)*5) asu32；key2=round(length(bottomClosestPos-topClosestPos)*5) asu32；key3=rank。round原外部CRT ties-away，Rustf32.round对应，cast原饱和到0/u32::MAX且NaN→0。
4. rectangle curvature=0；circle curvature=1-MINSS(1,MAXSS(0,inverse_lerp(f32bits3f666667,8.0,bottomCircle.radius)))。用既有utils::inverse_lerp；其调用顺序a,b,value。常量.9000000357627869的bit不能写成不同位型.9。MAX/MIN保原第二operand的unordered策略。
5. key不含width/世界位置/rotation/scale原值。命中只Arc::clone直接return，**不能提前inverse变换或构建点/凸体**。这是原量化复用策略，不能为消除近似擅改键或加每实体cache。

## miss真实几何、算序与闭合

- 用实际scale/rotation/translation形成同glam Affine3A，再.inverse()；不normalize quaternion，不用只逆rotation，不忽略scale。现工程glam接口复用（记录实际版本）。
- 原外层(0..2).map(..).tuple_windows只有一对height t=(0,1)，不是递归/多层细分；loop-inline-symbolization.jsonl有131行真实range/map/tuplewindows来源。rank1每环N=2点；rank2/3每环N=3点。原capacity=6*N-6个Vec3，append全列表后new_convex。
- 对t=0和1分别严格算roofY=(topY*t)+((1-t)*P.y)，取真实roofSection；expand amount为 t*0.0+(1-t)*padding（低环），高环原为(1-t)*padding+0.0。用现ShapeParameters.expand，不另写圆/矩形算法。
- placementY=(topY*t)+((1-t)*frontBaseY)，drop=placementY-roofY。保drop*0.0形成X/Z lane，不折叠成literal0，从而不吞Inf×0的NaN。
- 两expanded section分别求P.xz最近u。half interval=(width+padding)/L * +/-0.5，L是**原bottom未expand的perimeter**，两个高度共用L，不换上层perimeter。端点原fmod1后负值+1；复用canonical CurveUCircular/CurveU。按原increasing-u wrapping_lerp，end<start则end+1，算 `(1-t)*start+t*end`，再fmod1；保原NaN/range失败。每个N点t=i/(N-1)。
- 每个四边strip的点：A=lowerRing(i), B=lowerRing(i+1), C=upperRing(i), D=upperRing(i+1)。各ring先ShapeParameters.get_pos_at_u再world(x,placementY,z)经真实inverseAffine进入local。不得固定三角形/用矩形bbox替代曲线。
- **有序点列表**逐strip追加（new_convex会哈希重复顶点，所以不能排序/去重）：主面(A,B,C),(B,D,C)；起始面在i=0以及rank!=1且i=1均追加(A,A-lowDrop,C),(A-lowDrop,C-highDrop,C)；末面在i=N-2追加(B-lowDrop,B,D-highDrop),(B,D,D-highDrop)；底面(tlower==0)追加(A,B,A-totalDrop),(B,B-totalDrop,A-totalDrop)。low/highDrop=(drop*0,drop,drop*0)；totalDrop=(hp*0,hp,hp*0)。**这些减法在已逆变换的local点上执行**，不擅改成inverse世界offset。
- rank1总24点(8三角)，rank2/3总42点(14三角)，只是凸体构造输入有序记录，不据此声称输出mesh有这些拓扑。
- 必须调用真实RaycastMeshData::new_convex(&all_points).expect(...)，失败panic且不插入cache。成功Arc::new，同Arc clone写cache、把原Arc返回；替换oldArc drop。无empty/hullfallback。缓存无额外eviction/clear；system Local按现App生命周期持有。编辑/删除消费由外层modified/blueprint链处理，cache可保留已量化key。

## 当前共享依赖与边界

复用实际CachedDecoratorTransform、Roof/roofSection、ShapeParameters/Shape、utils Circle2d/Rectangle2d、CurveUCircular与glam Affine3A、native-physics new_convex。原shape helper已存在typed实现；RoofSection到ShapeParameters是字段投影（circle center/radius/rotation bits；rectangle axis/center/size），不另造geometry owner。当前native-physics配置由root统一处理，worker不碰Cargo。

矩形/圆几何的canonical算法若有证据冲突只报告具体共享scope，不复制第二份算法。编译/真实物理query/GPU由root执行；此合同仅原证及候选需求，未通过runtime验收。

## 拟source验证

真实圆/矩形Roof与非trivial transform输入产非empty convex；cache hit返回同Arc且不触发singular inverse的cache-miss失败；rank非法与原先后读取门；key的饱和/round边界与实际语义；miss失败不污染cache；24/42有序顶点面闭合包含底/两端/中间面，并保实际scale逆变换。原源码旧空Vec缓存测试删除或改实际typed测试，不把bufferlength作为mesh物理正确性的唯一依据。
