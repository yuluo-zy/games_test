# 必读 geometry addendum：原middle V1纠正及两helper表达闭合

本文件与 MEMBER14_MIDDLE_CHAIN_CONTRACT_B_V1.md 一起读取，发生冲突以本文件为准；不改V1原物。不代表完整host/注册/source批准。

## 明确更正

V1 Chimney tag1额外构造的“+10+1.6”是错误常量读法。主host XMM11实际载自142a7fcb0，原RNE bytes **cdccccbe = −0.4f32**；1412aecc2..cc7把它加到Params+10。因此该次helper两端为 **Params+10 与 Params+10−0.4**，尺寸0.2/0.2，smooth delta0.1。未写任何几何候选/生产，该错误未传播到源码。原V1其余tracker/owner合同不变。

Door stack参数明确：canonical words[0..8]依次可命名为center.x/center.z/axis.x/axis.z/end2/end3/extra_vertical/width（均f32）；host stack+20=width、+28=extra_vertical、+30=constant depth0.56。初始垂直分割height是end2−end3+extra_vertical，**不能把constant0.56当extra_vertical**。这些是安全Rust业务命名，不声称有原字段decl。

## Door原完整表达（14200c7f0/4668B）

依据caller1412af878..8f2和helperentry栈frame：calleeRBP=entryRSP−2c8；caller+20/+28/+30对应RBP+2f0/+2f8/+300，rng+308，axis+310/+318，writer/failed环境+320，outerWallId值+328。UiSignifierStream原捕获+330未在该完整helper发现业务读或写，不编signifier输出。

1. h=end2−end3；h_extra=h+extra_vertical。n_y=max(ceil(h_extra/0.2) as usize,2)，j_y=f32::from_bits(0x3d23d70b)/h_extra（原scalar0.0400000028）；ys=random_splits(n_y,j_y,rng)，按 `t*(end3−extra_vertical)+(1−t)*end2` 映射，保乘加顺序与两端次序。
2. k=(ceil(h/0.2) as usize).clamp(2,5)。depth_step=0.56/k；scale_depth=depth_step+0.1。n_x=max(ceil(width/0.45) as usize,3)，**有原max3**；j_x=0.179999992/width。half_width=width*0.5。
3. ys相邻pairs按row=0..枚举，row_height=abs(y1−y0)，row_y=y0*0.5+y1*0.5；steps=min(row+1,k)。每step=0..steps−1分别调用random_splits(n_x,j_x,rng)，再collect为xs：`t*half_width−(1−t)*half_width`。不能跨row/step缓存不同rng分割。
4. offset=(step as f32)*depth_step+0.28；row_step_center=(center.x+offset*axis.x,center.z+offset*axis.z)。每个xs相邻pair(a,b)：s=a*0.5+b*0.5；brick_xz=(row_step_center.x−s*axis.z,row_step_center.z+s*axis.x)。translation=(brick_xz.x,row_y,brick_xz.z)。rotation=共享utils::xz_rotation_bad(axis)；scale=(abs(b−a),row_height,scale_depth)；用原compute_affine语义及canonical Packed columns承载48B affine。
5. BrickTransformSsbo::default后覆affine；seed=(outerWallId low32).wrapping_add((2*row) low32)，**同row/step所有横砖共此seed，不加横砖序号**；material为原BrickSourceId::from(outerWallId)低32位编码；wall_distance=[a,b]保有向横区间；flags仍default。对应14200d194..d263和d4d2..d5a1。源编码表已从原142a953f0读出，只transient100000..100003/100011转80000000|index，与现canonical编码一致。
6. 两push点不是两类砖算法：14200cfbb先取第一xs pair，14200d27d len==2结束，否则d2b0..d2d4遍历剩余pairs；两条同payload写面是首迭代/余迭代展开。可安全用一个相邻windows循环表达业务，但必须保相同顺序、rng抽取时点、seed和字段。
7. 每砖NoFreeChunks设置累计failed，继续全部循环。原n_x/n_y上只有上述业务min/clamp，无全局固定上限；Rustusize转换饱和，分配溢出/OOM仍错误，不能变静默截断。

## Chimney tag0 terrain与音频补齐

原1412aed4e..de5向量运算与MEMBER14_HOST_CONSTANTS_B_V1：`p_plus=center+(axis*1.3)*0.5+perp(axis)*0.5`，`p_minus=center−(axis*1.3)*0.5+perp(axis)*0.5`，perp=(-axis.z,axis.x)。按p_plus、p_minus顺序调用原FloatGridSampler::bilinear_sample_at_world_xz@140257ba0（相同TerrainHeightsData当前heights/min/max/global采样开关）。现canonical TerrainHeightsData::sample_at_world_xz正复用该sampler，不能用flat0/default bbox替代。

另一端初始化 `bottom=Params+10−Params+4`，依次取bottom.min(sample)，保单NaN返回非NaN的原min恢复分支；helper end2=Params+10、end3=bottom，尺寸1.3/1.0、delta−0.3。两个端点不是装饰bbox重建。

主hostcurrent音频条件：Added且new tag0，或Changed从非0到0→kind19；Changed从0到非0→kind20；Current/其他Added/其他Changed不发。位置均是new current Params(+8,+10,+c)。真实Removed通过canonicalDerivedDecoratorInfo view按DecoId查try_get_subtype，只有Some首byte0x14发kind21，位置old Params(+8,+10,+c)；不是按oldParams+0代替subtype读取。PublicWalls存在时该业务不可空化。

01 main_loop_team当前持合法decorator_subtype claim并恢复canonical DerivedDecoratorInfoResources/From8B/try_get_subtype231B；本agent已交entry117B/完整Removed860B/SystemParam7原物，不新增第二resource或改foreign。只待真接口复用。

## 共享helpers实际验证更新

04已交root实测：cloth-utils-harness-v1/runtime-v1/test-run-v1.json exit0，root保留binary SHA ff31a28ff9bdb9fd715c95595ab6fa1c0084006266d3895edfc4d2b01864b178。独立读回receipt，范围仅当前exact canonical三个mathhelpers/7 kerneltests；不称完整utils、Assembly、core/App/native/GPU通过。

## 本阶段接口提案与未过门

两constructor可用canonicalParams/f32/WallId及真实 `FnMut(&BrickTransformSsbo)->Result<(),NoFreeChunks>` 发砖；生产闭包必须实际调用同Sparse writer.push，helper返回/累积failure方向保持原。测试可观察真实payload，不可把无输出closure注册成原system。helper只生成砖，不拥有第二GPU缓冲或全墙registry。

本addendum闭合业务表达供07继续核。source/整合仍等root合法外部owner及前置lane初始化、Commands apply和同库调度；现2个protocoladapter未被冒称完整7/4参assembly。未跑Cargo、benchmark或GPU。
