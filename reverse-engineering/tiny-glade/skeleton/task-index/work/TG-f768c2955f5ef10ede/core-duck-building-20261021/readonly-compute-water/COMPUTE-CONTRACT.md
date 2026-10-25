# compute_stair_assembly_params 完整只读计算合同

唯一writer仍duck_building_reaction（TGf768/0ae3/TG280）；本目录只读辅助合同，不改claim/source/candidate/生产/Cargo。主源B140807090/10808B完整复用，RNE f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03，PDB2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911。父目录native-pack/compute-full.{json,asm.txt}；raw反编译仅导航。新补两个game closures嵌在std from_iter/fold内，复用std容器，不把std循环计独立游戏恢复。

## 输入与真实调用ABI

request true原140280a99/140281590两调用（父native-pack/request-true-full），logicaltyped分组为：

- out返回0x150完整params（RCX）；两32B endpoint/degree输入整块64B（RDX）；真实VisualTerrainHeightsData（R8）；两个StairPoint（R9、stack20）。
- start/end AlignKind（stack28/30）；调用者给的startXZ/endXZ（38/40、48/50），startDir/endDir（58/60、68/70），不可擅自改读curve.first/last或StairPoint.position.xz。
- edge实际endpoint载荷（stack78）；可选tangent Vec（80）、width Vec（88）、height Vec（90）、SubSegments SmallVec（98）；旧dirty HashSet（a0）；owned完整Curve2（a8）。这些是原Win64 ABI stackoffset，不强制现代Rust保22个伪参数。

StairPoint前置已由duck独立锁为48B：addr16，+10wall-tangent Option tag，+14/+18 wall tangent，+1c/+20/+24 required CachedTransform.position，+28 id，+2c StairExtra.width。故原p4/p5+20是端点height，+2c是width。所有optional Vec/curve按owned移动处理，不另外建镜像容器或用空库fallback。

## 输出0x150类型和未证项

完整逐偏移表见output-layout.json。Curve2@0..38是56B完整curve；heightVec@38、widthVec@50、tangentVec@68；@80为24B Vec、每元素4B/align4/无元素drop（1405d2a39..61），**具体f32/u32字段名未证**，compute总明确写空0/ptr4/0。不能为此猜业务类型或伪装已完成消费者；可保观测字，不造值。@98 OptionCurve3在本producer明确None；None的inactive bytes没有原有效值。@d0..110两32B端点完整复制。@110真实BitVec<usize,Lsb0>，@128无向排序identity，@130真实SmallVec<[4BSubSegment;1]>，@148 dirty。尾padding149..14f不能宣称原零填。

## 默认向量与失败读取顺序

先读H0/H1、W0/W1（两个StairPoint的position.y/width）。三个Option Vec的None niche是isize::MIN；Some(empty)是确切空，不是None：

1. tangent override有值直接move；None时对0..curve.points.len调用原closure0（141293590/406B，收集器1412618c0/327B）。i非尾取next-current，尾取last-previous；原CurveInternalCoord选择段，然后 **raw normalize** `delta*(1/sqrt(dy²+dx²))`。没有central tangent、平滑或normalize_or_zero。零段会NaN。
2. width override有值直接move；None时按**原curve.points_u**每个u计算 `u*W1 + (1-u)*W0`（原140807...向量化与scalar同式）。不能按index/N猜u，不做全有限/等长预校验。
3. 创建len=原curve.points.len、全false的BitVec；直接复用bitvec依赖正确head/len/cap与移位，不能当Vec<usize>。
4. height override有值直接move且保持BitVec false；None时按下节真实双侧采样。
5. 至此才检查tangents[0]存在，写startDir.normalize_or_zero；再以**原curve.points.len-1**检查并写endDir.normalize_or_zero。即使override也改首尾；过短tangent是原bounds panic，不能提前统一调整长度。原先双侧采样使用的是覆盖之前的tangent输入，不能把endpoint normalize前移。

## 真正height采样callback与mask

完整原callback141292e30/1094B，generic收集器141260990/429B。四重zip `points × points_u × tangents × widths`，长度是四者min，再enumerate：

```text
offset = Vec2(-tangent.y,tangent.x) * (width*0.5*0.5)
plus  = VisualTerrain.sample_at_world_xz(point + offset)
minus = VisualTerrain.sample_at_world_xz(point - offset)
floor = (plus + (-0.025)).max(minus + (-0.025))
line  = H0*(1-u) + H1*u
mask[i] = (floor+0.1) > line
height[i] = floor.max(line)
```

两次调用是原140257ba0 `FloatGridSampler::bilinear_sample_at_world_xz<TerrainHeightFloatGrid>`；父实际第四SystemParam是VisualTerrain权威owner，复用其相同canonical采样接口/共享policy；不新格、不改统一policy、不用物理Terrain owner代替。连续两乘.5的算序是原width/4，不能误写width/2。常量bits：-.025=0xbccc_cccd，+.1=0x3dcc_cccd；sample-constants.json保RNEraw。`max`这里native带NaN correction，正是Rust f32::max语义，与无修正SSE MAXSS不同。mask是严格有序比较，非>=、非默认grounded；NaN比较false。每次写真实BitVec.set(index)，失败保持原bounds策略。

## None SubSegments：完整分类/梯段压缩

Some覆盖值直接move并跳过此整步；不是空覆盖时重新推断。None时把原Curve.points和已得三向量以及mask逐行zip（min长度），建立新owned输出：

- 第一点只append完整行(points/tangent/width/height/mask)，还没有SubSegment。
- 随后每对**相邻原输入**计算 slope=`abs(previousHeight-currentHeight)/sqrt(dz²+dx²)`，<=2.5且>.25为kind2，<=.25为kind1，>2.5或NaN为kind3。previous输入每步更新，不能用上个保留输出作为previous。
- 若末SubSegment不同kind，追加 `{kind,flag=1,count=1u16}` 并append整行。同kind1/2时flag允许合并（原kind1要求已有flag!=0；原新建flag始终1），`count=count.wrapping_add(1)`，仍append整行。
- 同kind3时**不增长count、不追加行**：required覆盖最后points/tangent/width/height/mask为当前行。连续陡段压成一个梯段，保block起点与最终终点。原last_mut/bit last读写失败均panic，不能用default或continue掩盖。
- 全部后用真`Curve2::crashy_new_from_points`重建positions+归一化points_u+length；旧points_u释放。不能只换positions，不能在新参数u下重采/重插值height与width。正常完整输入得到平行等长向量；overrides长度不一致时按原zip截短，随后真实Curve构造可能失败，不自加全等长门。

SubSegment 4B是`{kind:u8,flag:u8,count:u16}`；SmallVec依赖保inline1、spill/replace/drop。kind0只可能由后续端点padding构造；不能偷换成单Vec或丢flag。

## 两端罕见padding分支

两端分别只有 `align.tag==0 && (align.byte1!=0 || align.byte2&1!=0)` 才进入；其他tag不执行这一padding。

**start**：前插SubSegment `{kind=byte1^1,flag=byte2^1,count=1}`；前插点=`callerStartXZ-W0*align.firstVec2`；真实Curve2再次重建、旧points_u释放。前插height=H0、width=W0、tangent=原rawStartDir（不重用先前normalized值）。BitVec原push(false)+copy/rotate等价**index0 insert false**，其他bit右移；绝不是push false留末。

**end**：追加SubSegment `{kind=byte2^1,flag=byte1^1,count=1}`（bool顺序反向）；追加点=`callerEndXZ-W1*align.firstVec2`；重建Curve2。appendheight=H1、width=W1、tangent=rawEndDir；mask **append true**（140807d21明确DL=1）。不要把它改成false或重算地形。

原处理start在end之前；每次Curve重建都保失败，而不是为了成功跳过重复/极小段。提供的tag/boolean是原有效enum域，inactive payload不读取。

## identity、dirty与生命周期

原1408fe5c0/21B读取输入两个24B endpoint各ID（+0x10/+0x28），unsigned min/max分别EAX/EDX返回。typed实现应从同真实endpoint IDs建canonical StairSegment/Identity；不把输入当8B pair transmute，不沿decomp第二临时变量误读。outputdirty严格等于旧dirty HashSet.contains(sortedIdentity)；本leaf不清/写set、不因重算一律true。

最后一次性move完整Curve/三向量/mask/SmallVec/复制两个endpoint元数据/identity/dirty进output；显式empty辅助Vec和None optionalCurve属于已证producer初始化，其他类型消费者不因此算完成。原异常分支容器drop和owned输入释放由Rust/原依赖承担；不默认成功。

## canonical shared精度边界（必须明确）

- 默认tangent callback本身是game选择段+原glam raw normalize；当前utils::CurvePoint::normalized()对零/极小段有稳健fallback，不能拿它代替本callback。直接同glam .normalize()可保原运算。
- 原crashy_new14126a1e0/613B：点数>=2；纯f32累积sqrt(dy²+dx²)；total==0失败；逐points_u/total必须finite；没有重复段拒绝。R8是track_caller Location，非bool。当前host的crashy_new(points,false)对应不拒绝duplicates，但其generic metric_length对极小/极大输入有f64补偿，且先做points finite检查，**严格原浮点极小段/失败时序存在差异**。这里不并写shared、不复制第二curve算法；root需在原canonical utils scope裁决修复或明确真实输入域验证，不能把“复用helper”自动宣称原bitwise等价。
- +80元素名/业务类型仍未知；drop只证明4B trivial元素。当前需要完整typedpayload时应保观测表示和未知标记，不擅造支持点/height语义。

## 建议实施验收（worker未执行）

实际VisualTerrain+三点曲线反例验证width/4与-.025/.1门；Some(empty)与None分支；default采样在端点dir覆盖之前；全gentle、同middle、连续steep替换、kind变化、NaN坡度kind3、u16count wrap；start/end四bool组合与不同rawdir；mask false前插/true后插；新curve.points_u/length随压缩/padding刷新；identity顺序反转同dirty；required短tangent/重建失败不得吞掉。与计算合同同一producer/consumer闭包验证，不能只测常量表。
