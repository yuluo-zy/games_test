# 沿真实墙轮廓生成楼梯：walk_shape与receiver完整只读合同

B walk_shape `140809cc0/8680B`（451:60924）及callback0 `1412b2540/1264B`（1566:3984）。两个完整原包复用父work/helpers-native-pack；RNE f507a6ce/PDB2d9686b8完整身份见同目录偏好合同与pack report。本次必要追加rectangle streaming2528B、scalar lerp13B、project_clamp106B三个有界原包，walk-leaves-native-pack/report所有请求ok，每条机器指令比对原文件；未调用shared数据库/目标程序。唯一源码writer仍duck。

## 完整输入，禁止Vec-only或直线替代

逻辑输入为借用真实PublicWallState、两个AlignKind、两个原始outgoing direction Vec2、两端width、两端WallSpaceCoord(u,y)。RCX=out128B，RDX=wall，R8=&align0，R9=&align1。按调用者call前SP：20/28=startDir.xy，30/38=endDir.xy，40/48=w0/w1，50/58=u0/h0，60/68=u1/h1；原caller14028015f另写70槽，本body没有读该槽，不擅自补参数业务。

wall含真实source Curve2和ShapeParameters；现代用同PublicWallState.simplified_curve_2d()借用现source curve（已实现utils::wall_space::CurveSampling），不可换terrain-mapped Curve3/新造World/只用首尾线段。shape tag1为rectangle，其余tag0/2/未知走通用curve分支；原 `min(tag,2)&1` 判法只让1进入rectangle。矩形直接读取原shape六f32作为Rectangle2d(axis,center,size)，不是壁bbox/碰撞bounds。

所有分支先调用wall.wall_space：is_loop=(shape.tag()!=2)，length=原已发布curve_length；随后算 `delta = space.subtract(u1,u0)`。subtract的已确认canonical半周期差异见ALIGNMENT-PREFERENCES-CONTRACT，不可静默换策略或用Euclidean距离代替。原数值core已13/21/66/106B独立字节锁定。

## 输出：128B五个owner，不是success flag

|原偏移|真实值/ownership|
|---|---|
|00..20|**Option<SmallVec<[StairSubSegment;1]>>**，0=None、1=Some；+08起完整24BSmallVec，非MIN niche|
|20..38|owned Vec<Vec2> path points，总是有值|
|38..50|Option<owned Vec<Vec2>> tangent，None用isizeMIN niche|
|50..68|Option<owned Vec<f32>> width，同上|
|68..80|Option<owned Vec<f32>> height，同上|

独立真实consumer证明：true request140280995/99c把walk out起32B搬到rbp500，1402809e0/e7把该32B地址送compute caller stack98；compute的param20先读首bit0，Some从+08/+10/+18搬完整SmallVec（旧named.c389、660..662，实际asm同源）。request清理1402802e1先按out00判断None，再对out18>=2的SmallVec堆区释放out08。后三Vec分别按MIN niche释放。因此不能用bool模拟首qword、不能统一把五项变空Vec，也不可把Some(empty)偷换None。

## A. Rectangle：按原周界stream采样（9da2..a77a）

该分支不要求AlignKind为tag1。只在align.tag==1且flag==1时使用对应端半宽缩进；不读取align.byte2和两个caller rawDir。

令q=1/length；a=u0*q，b=u1*q；off0=(tag0==1&&flag0==1)?(0.5*w0*q):0；off1对应w1。以下算序由原寄存器逐项绑定：

- 若delta<0：内部开始为u1/h1/w1/off1，内部结束为u0/h0/w0/off0；否则（含无序比较）不交换。
- 取内部normalized start S、end E；paddedS=S+startOff，paddedE=E-endOff。
- 若 `(E-S).signum() * (paddedE-paddedS).signum() == 1.0` 则同时使用paddedS/E，否则同时保留原S/E。不能只clamp某一端/按正长度判替换，也不能把±0 signum变0。
- 依次构造两CurveUCircular：原只断言!NaN（a0df/a0f1），再fmodf(x,1)，负余数+1。这不是排序/OrderedFloat total_cmp。start先失败再end；原infinity通过!NaN而在fmod变NaN，当前canonical CurveUCircular::new要求finite更严格，此边界须公开，不声称两者全输入等价。
- 复用现Rectangle2d::circular_slice_segment_points(start,end,&mut RectangleSliceState,receiver)。原专属实例1412b1b60/2528B完整已核，实际调用callback0在1412b20a0/20c9/2133等；state跨emit保存前一normal，角点会产生kind4/5两类record。不可仅采两个端点或以角点重复去重替代。

共享矩形算法保有真实周界/轴变换：start/end包一圈；跨度MAXSS(1e-10,stop-start)；有序角点1..9；内部拐角先旧normal再新normal，精确end-corner使用后继normal。receiver保相同pending state（原wrap flags起始1/0，对应pending_normal=None，首次emit后join_pending=true）；不要每个角点重新default。当前canonical已具该方法和RectangleSlicePoint(position,normal,relative_u,tag)，复用现实现。

### receiver0：全部分支与副作用

捕获12个借用指针顺序为：wStart,wEnd,points,needHeight,hStart,hEnd,previousPointOption,lastHeight,subsegments,tangents,widths,heights。主rectangle producer把needHeight置true、previousPoint=None、SmallVec/四Vec真实空；false分支仍原函数有实现，下面完整列出。

输入为原矩形record position P、normal N、relative_u t、recordTag。使用原13B scalar `lerp(a,b,t)=a+(b-a)*t`（同现utils::float_grid::lerp可复用，不建立另一套数学算法）。

1. W=lerp(wStart,wEnd,t)；Q=P+N*(0.5*W)。先算W与Q再查points.last。
2. points.last存在且两个分量有序==Q时立即return（NaN不相等，±0相等），不改任何Vec/previous/segment。禁止epsilon去重或OrderedFloat Eq代替。
3. tangent=(N.y,-N.x)，保持原符号翻转。
4. needHeight=false：依次push Q、tangent、W，return；不采H、不写previous/SmallVec/heights。该callback分支真实存在，即便当前parent初始化true。
5. true：H=lerp(hStart,hEnd,t)。若previousPoint=None，不建segment；直接previous=Some(Q)、lastH=H，push Q/tangent/W/H。
6. previous=Some(prev)：recordTag!=5先定kind4；recordTag==5则slope=abs(lastH-H)/sqrt((prev.y-Q.y)^2+(prev.x-Q.x)^2)，按**strict有序**`if slope>2.5 {3} else if slope>0.25 {2} else {1}`。NaN→1（2676 seta/inc、2691 cmovbe），禁止沿旧compute合同误写NaN3；独立NAN-SLOPE-CLASSIFICATION-ERRATA已交lead/duck。
7. SmallVec.last不存在→新push `{kind,flag:1,count:1}`。若存在：
   - kind<2族：仅oldKind==kind且oldFlag!=0可count.wrapping_add(1)后append；old0/new1或old1/new0、flag0都新建segment。
   - kind>=2：仅oldKind==kind且oldKind>=2兼容。兼容且kind!=3→count.wrapping_add(1)后append（包括kind4）；不兼容→新segment。
   - 兼容kind3：**不追加行/不增count**，依次required替换最后points、tangents、widths、heights，最后更新previous=Some(Q)/lastH=H后return。四个last缺任一分别原unwrap panic（29ff/2a0b/2a17/2a23），不可default/skip。
8. append路径先写previous/lastH，再依次push四Vec。容器增长失败保原panic，不默认成功。新segment四字节原0x00010100|kind，保flag/count；u16原incw回绕。

### rectangle返回与反向

stream完成后，若原delta<0：reverse points；reverse tangents并对每项两分量取负；reverse widths；reverse heights；reverse完整4B SubSegments元素顺序（不改count/flag）。标准容器反转直接复用，无需复制原SIMD/unrolled库代码。最终返回Some(subsegments)、points、Some(tangents)、Some(widths)、Some(heights)。即使某Vec空也保持Some，不新增几何有效性过滤。

## B. Circle/freehand/其他tag：真实source curve采样（9e62、a206..b8f0）

1. align0.tag!=1：返回None(subsegments)、empty points、后三None；不检查align1，不造默认路径。
2. 创建真实points/widths空Vec，sideSign=if align0.byte2!=0 {-1}else{+1}。abs(delta)须有序>=f32 bits3a83126f（约.001）；不满足（包括NaN）直接返回empty points和后三None，不做后续to校验/curve调用。
3. steps=(ceil(abs(delta)) as usize).max(1)，pointCount=steps.wrapping_add(1)。原releaseusize加法存在回绕：saturating cast到usizeMAX时count回0，不可一概改成checked_add panic。其余capacity/bytes超过isizeMAX时原reserve/capacity-error panic。原先reserve points(count)，再继续后续运算；不要人为设采样上限掩盖输入错误。
4. sign=delta.signum()。若align0.flag!=0，u0=space.project_clamp(u0+sign*w0*0.5)。之后才检查align1.tag==1，失败释放已分配临时Vec并返回empty points/后三None；不得将其误写panic或前移这个门。
5. align1.flag!=0时，u1=space.project_clamp(u1+sign*w1*(-0.5))。两个raw dirs分别normalize_or_zero后旋转left(-y,x)，再乘sign作为首尾offset normal。此时未碰source path新副本。
6. 对i in 0..pointCount，t=(i as f32)/(steps as f32)：
   - U=space.project_clamp(u0+space.subtract(u1,u0)*t)；width_i=lerp(w0,w1,t)，先push width_i。
   - 再按原WallSpaceCoord::new(U,y0)第二次project_clamp，使用同真实source Curve2的get_pos_xz（不是简线）。原helper的y=0且XZ不读端height。
   - i=0用首端offset normal；i=steps用末端normal；其余用真实WallSpaceCoord.get_tangent(curve)，取normal=(-tan.y,tan.x)，不额外乘sign。
   - position_i=curvePos + normal * (width_i*0.5) * sideSign，按原先*.5再向量乘再sideSign/加法算序。push points。
7. count<=1：返回points和后三None，释放未返回widths。count>1：分配len(points)个Vec2 tangent，首=(p1-p0).normalize_or_zero，尾=(plast-pprev).normalize_or_zero，内部i=1..n-2为(p[i+1]-p[i-1]).normalize_or_zero。**通用采样的切线是central difference**，不要误复用compute默认tangent的next-current算法。

### offset strip凹折裁剪：循环至稳定，不能省略

原遍历 zip(points,tangents,widths) 的相邻tuple_windows并enumerate；每对i/i+1用两端宽分别 `width*0.5*1.25`，offset normal=(-tangent.y,tangent.x)*sideSign。构造原有序quad：

- sideSign正（align0.byte2==0）：[p_i+o_i, p_i-o_i, p_j-o_j, p_j+o_j]。
- 另一侧：交换i/j两端之后同次序，即[p_j+o_j,p_j-o_j,p_i-o_i,p_i+o_i]。

仅i!=0且i+1<points.len才考虑删除，保两端。quad取cycle的4个连续三元组(prev,current,next)，若任一个 `cross(prev-current,next-current) > f32bits322bcc77`（约1e-8）则不是原ccw convex，删除index i的points/tangents/widths三项。原比较b584/588是有序strict，NaN不触发；不以面积abs/epsilon>=/总排序替换。

删除后用新的points重算受影响邻点的tangent：

- i==1时重算tangent0=(p1-p0).normalize_or_zero，否则tangent[i-1]=(p[i]-p[i-2]).normalize_or_zero。
- i若已为新的末点：tangent[i]=(p[i]-p[i-1]).normalize_or_zero；否则tangent[i]=(p[i+1]-p[i-1]).normalize_or_zero。
- 原required下标/Vec::remove越界逐个panic，不能用filter_map跳过。删除后**从头重建同一zip/tuple_windows扫描**，直到完整一轮无删除；不保留失效索引/引用，也不偷换RDP/凸包/简单线段。

相同逻辑下可以用现Rust Vec/iterator、pinnedglam实现，不翻译std SIMD容器细节；在可观测结果与失败次序未另证前，不将重扫简化成一遍。最终None(subsegments)、真实points、Some(tangents)、Some(widths)、None(height)。由compute补terrain-aware高度和分段，不在此造高度默认。

## 共享接口复用与明确未闭合差异

- Rectangle2d/RectangleSliceState/CurveSampling、现Vec/SmallVec、pinnedglamnormalize_or_zero与std符号翻转/容器操作直接复用；数学lerp13B与现float_grid::lerp相同。领域算法是参数/采样/状态/裁剪组合，不把公共依赖实现算恢复任务。
- **真实待修**：WallSpace::subtract当前canonical在半周期tie使用f64，合法length10/to5/from2^-26产生+5而原-5；已交lead/root协调最窄原业务policy，duck不可私建subtract替身。
- **已核相符**：project_clamp当前CircularComparativeSpace源码94..97就是f32 `% period`、负余数+period，符合原106B；不得把另一个lerp_clamp的“==period归0”策略套过来。本worker曾短消息误报project差异，已立即向lead撤回，合同不采纳该误报。
- CurveUCircular::new现finite断言比原!NaN严格；对正常有限路径可复用，对infinity失败时机不等价应公开；不要借此回退整个通用工程增强。源/shape非法值、零length以及CurveSampling内部失败也不得伪称全输入已验证。
- 字节/metadata证明语义合同，不是Rust候选/目标执行/完整楼梯验收。本文已闭合所有本body分支和捕获值来源；完整request graph/maps生命周期仍由duck恢复，不在此扩新家族。

## 验证目标（root唯一执行）

rectangle周界跨角/精确角/包一圈/反向；两端tag/flag缩进会反转跨度时同时回退；重复Q有序相等/NaN不去重；receiver needHeight真假、kind4/1/2/3合并与梯段replace；NaN slope→kind1、u16count wrap；通用curve首尾与central tangent不同；两侧1.25 strip凹折删点/重扫/端点保护；align1错tag在已reserve/首端project后才返回；.001门边界/NaN；same-period signedzero/半周期policy；Some(empty)与None通过真实caller移入compute而非额外producer镜像。未执行以上验证，不以合同冻结计恢复完成。
