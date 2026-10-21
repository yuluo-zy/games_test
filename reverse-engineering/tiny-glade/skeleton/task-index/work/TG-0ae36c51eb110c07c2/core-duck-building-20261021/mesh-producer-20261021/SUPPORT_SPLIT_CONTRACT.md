# supported/unsupported 分段：原3075B完整合同

## 原范围与参数
- 1407d94a0/3075B/PDB437:119232，完整helpers-native-pack/fn-7d94a0；decomp导航不得覆盖以下ASM。
- RCX sret72B，RDX=&ResMut<RaycastWorld>，R8=原UiSignifierStream（完整body不读），R9=&当前ArchSegment，第5实参caller stack20=&StairWallAnchor。原callee通过entry+28（其rbp+190）读取该anchor，不能误认R8。main14000f8a4..f8bd完整调用与callee9514/95fd/9811闭合。原input只读当前segment Curve3.points@0..18、tangents Vec@38..50、widths Vec@50..68，不读取其余tail。现代API可借实际current positions/tangents/widths slices，不另造same-wallcurve。
- 三输出各原Vec：+0 unsupported inclusive `(usize,usize)`；+18 supported inclusive ranges；+30 `Vec<Vec<f32>>`，每一条supported范围对应一个宽度侧距样本Vec。不是旧仅Vec范围返回。

## 几何/查询（每内部点i）
1. 原先初始化outputs：首侧距Vec仅`widths[0]*.5`，positions end=points.len().wrapping_sub(1)。i从1开始，严格i<end，不检测首尾碰撞。widths空原bounds panic；len1合法会产生supported(0,0)与两次端宽（可不等长），不能人为skip。
2. 每i先调用canonical wall_anchor.wall_normal()得到n=(nx,nz)，再要求tangents[i]存在。sign=`if nz*t.x > nx*t.y {-1.0}else{1.0}`（ordered strict，NaN→+1）；raw=(-t.y,0,t.x)*sign。
3. inv=1/sqrt((raw.y²+raw.x²)+raw.z²)，inv finite且>0则raw*inv，否则三个分量+0（原mask），不使用raw normalize或fallbackX。然后按原要求points[i]、widths[i]存在。
4. origin.x=((dir.x*w)*(-.5))+p.x；origin.y=((dir.y*w)*(-.5))+(p.y+(-1.17))；origin.z=((dir.z*w)*(-.5))+p.z。保0*NaN/Inf与加法关联。
5. 调canonical world.raycast_simple_with_max_toi(origin,dir,1.78,1)，同一真实RaycastWorld，不造命中注入。原1407d997f mask1，9993 XMM3=1.78，9996 call1408ccc20。其他来源slot不当墙命中；不按wallid过滤（该原入口不传excludedwall）。
6. miss=`result.is_miss()`（首tag17）。hit时侧距=`widths[i]*(-.5)+sqrt((origin.z-hit.z)^2+(origin.x-hit.x)^2)`，无clamp/abs/finite验证。

## 完整状态机/端点
初始supported=true，supported_start=0，width_runs=[[widths0*.5]]。
- Hit且之前supported：把侧距追加最后widthrun。
- Hit且之前unsupported：追加unsupported(last_support_boundary,i)；新增widthrun=[当前侧距]；supported_start=i；supported=true。
- Miss且之前supported：追加supported(supported_start,i-1)；unsupported_start=i-1；supported=false。
- Miss且之前unsupported：只继续，不多加range或widthrun。
结束若unsupported：追加unsupported(unsupported_start,end)，不追加末端宽；否则追加supported(supported_start,end)，最后widthrun追加`widths.last().unwrap()*.5`（原width Vec实际最后元素，不是points[end]索引）。
保单点supported range，不清理共享boundary重叠；unsupported区间包住两侧boundary供原后续slice。

## 原常量
RNE142a877a4 bitsbf95c28f=-1.1699999570846558；142a877a8=3fe3d70a=1.7799999713897705；1429ddcf0数组[+1,-1]；142925870=.5，14295a0e8/142a171f0=-.5。原PE身份同manifest。

## 失败/验收与范围
- 不新增输入长度整体验证：先widths[0]；每i wall_normal→tangent→point→width；最后真实widths.last()。positions为空的release wrapping end极大，按原后续实际bounds失败，不用预先assertLen2替代。
- 实际World tests：真实slot0 cuboid wall与flush；全hit/allmiss/hit-miss-hit三路径及共享boundary、对应侧距runs；非slot0不命中；len1/2、extra widths最后元素、短tangents失败顺序。No假world/注入rayhit。
- 本helper不修改world，是纯借用查询；现代接口可保unused signifier实参供原实际caller显式借用，不能为此插默认Ui资源。output vectors原分配和Rust现代Vec可保正常路径成本，不借此声称原OOM分配序在所有后端bit-equivalent。
- 完整main、ArchSegment局部记录完整定义与其后续支撑mesh仍待同轮完整合同，不以该3075B叶替代两个62KBbody。
