# 楼梯对齐偏好完整原合同（B 14080beb0/1196B）

只读交付；stairs唯一writer仍duck，不改源码/Cargo/claim。优先交本叶，不等待walk_shape大函数。复用原helpers-native-pack/alignment-preferences-full.{json,asm.txt}，status=ok、原RNE逐指令字节匹配、范围无overhang；PDB451:75344/full1196B。原身份RNE SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03，PDB SHA 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911，GUID09F1A02A-49F2-F8C8-4C4C-44205044422E age1。完整旧decomp只作导航，NaN/ABI以原asm为准。

## 输入与输出

逻辑输入：from/to两个真实StairPoint、caller给的deltaXZ、同owner DecoratorStorage、PublicWalls。RCX为返回Vec区，RDX=from，R8=to，XMM3=delta.x；按原入口前SP计，stack20=delta.y、28=storage、30=PublicWalls（函数push/sub后分别rsp120/128/130）。实际caller true14027ef28..75对Point.position的XZ先做to-from，再以该顺序传入；helper不自行重算position。

Point48B已由nodefold原证锁定：addr16、Option<wall normal> tag+10（wall0/nonwall1）、normal(+14,+18)、position+1c..24、id+28、width+2c。本函数只读addr、normal option、normal、id；不读width，也不从position猜delta。normal是先对墙tangent符号翻转再交换得到的 `(ty,-tx)` / `(-ty,tx)`，非未旋转tangent。

输出有序候选序列，每项是“AlignKind有效20B载荷 + f32 score”，原步长24B。**不在本函数排序/选max/做OrderedFloat比较**；若后续消费者按OrderedFloat排序，直接复用其锁定类型/规则，不在此提前滤NaN或钳score。现代可用独立typed(kind,score)值，不要求复制native布局。现core AlignKind声明24B且称+14为padding，不能把本函数record的score@14误当kindpadding、也不可用24B原记录memcpy进该值再额外读score；此旧ABI断言待唯一owner协调。

## 读取与失败的严格顺序

1. 入口 `14080bee5` 先读from.normal Option tag bit0；None立即返回空Vec，不读取任何storage/to/publicwall。不能把全输入校验移到前面。
2. Some时读normal和from.addr/id，`14080bf32` 在实际给定addr桶required查fromInfo（原decorator_info(addr,id)），缺失在c319 panic。
3. 读fromInfo.dst_tag@22，仅0/1允许；>=2在c2ae panic，发生在查询to之前。读取fromInfo.wall_coord.u（首f32）。
4. `bf69` 按to真实addr/id required查toInfo，缺失在c325 panic。
5. 此后才要求from.addr.as_wall：原addr disc!=0在c2c6格式化panic；不提前移动此门。目标addr不是Wall不panic，转三候选。
6. 目标addr为同WallId且toInfo.dst_tag<2走samewall；其余（不同wall/目标非wall/目标dst_tag>=2）走三候选。

当前shared storage.decorator_info(id)手写面忽略传入addr，不可代替原显式addr lookup；现代在原顺序中借用同owner `storage.get(point.addr).and_then(|b| b.get(id)).expect(...)`，不造第二map/d2s镜像。

## 不同wall/目标非wall：完整三候选（原bf93..c11e）

先实际allocate三项（72B/align4），失败走原alloc_error，不先算浮点。然后：

```text
d = delta.normalize_or_zero()             // pinned glam0.29.3原inline，非rawnormalize
s = (d.y*n.x - d.x*n.y).signum()
turn.x = (-n.y) * s
turn.y = n.x * s
straight_dot = d.y*n.y + d.x*n.x
turn_dot = d.y*turn.y + d.x*turn.x
factor = if straight_dot > 0 { f32::from_bits(0x3ed413cd) } else { 1.0 }
score0 = straight_dot * factor
```

按如下顺序返回，不能按分数改序：

|项|tag|flag|byte2|first|second|score|
|---|---|---|---|---|---|---|
|0|0|0|0|n|inactive|score0|
|1|1|0|from_dst_tag ^ 1|n|turn|turn_dot|
|2|1|0|from_dst_tag ^ 1|n|-turn|-turn_dot|

第三项的turn与score是原XOR符号位翻转（c0e0/c104），不是重新dot求值。straight_dot>0是有序strict：NaN不满足，取1。from_dst_tag只可能0/1，故byte2为真实bool反转。

原normalize_or_zero reciprocal为`1/sqrt(delta.y²+delta.x²)`，只在有限且>0时乘原delta，否则返回ZERO；原inline明确glam0.29.3，直接复用该依赖。s使用std f32::signum；+0→+1，-0→-1，NaN→canonical+NaN(7fc00000)，不可以cmp正负返回0替代。原常量/bit已从RNE读到preferences-constants.json。

## samewall：一候选（原c154..c2a9）

先按原未归一化delta算 `delta.y²+delta.x²`（前段先算x平方再y平方，执行加为y²+x²）。仅**有序strict `< f32::from_bits(0x358637bd)`**（约1e-6）进入短距离：返回一项tag0/flag0/byte20/first=n/second inactive/score1。

原比较c17d的jbe使**NaN进入非短距离分支**；不能按旧decomp写 `if length²>=1e-6 { large } else { small }`，这会把NaN分错。

非短距离（包括相等、NaN、infinity）：

1. 调from.addr.as_wall（前面已保证，但保required来源）。required PublicWalls.get(fromwall)，缺失c34f panic；短距离分支不读publicwall。
2. PublicWallState.wall_space原140b2a740/21B返回is_loop=(shape tag@258 !=2)、length=@28c。
3. 原 subtract140c91f60/66B：XMM1=**toInfo.coord.u**，XMM2=**fromInfo.coord.u**，即 `space.subtract(to_u,from_u)`。不读两coord.y。
4. 获取diff后再allocate一候选（24B），分配失败原alloc_error。
5. `s = -diff.signum()`。**不能换成 `(-diff).signum()`**：原NaN常量明确ffc00000（负canonicalNaN），前者保此结果，后者会正canonicalize。
6. `turn = Vec2((-n.y)*s,n.x*s)`；如果from_dst_tag==0，之后再把turn两分量XOR符号翻转（保原次序，不并入一个sign简写）。
7. 返回一项tag1/flag0/byte2=from_dst_tag^1/first=n/second=turn/score1。

## canonical subtract 必要差异，尚待root修复

本叶的真实共享依赖不可按“方法同名”视为完成。原66B subtract只做f32：

```text
d = a-b
if !is_loop: d
else if d < (-0.5)*length: d+length
else if d >= 0.5*length: d-length
else: d
```

现 `utils::WallSpace::subtract` 委托 CircularComparativeSpace 的工程增强策略；当`d.abs()==length*0.5`等边界会使用f64重算。**合法已投影坐标反例**：length10、to_u=5、from_u=2^-26。原f32减法舍入为5，半周期tie走减10得-5；现wide差值略小5后round为+5，符号反转，导致本samewall方向翻转。无需多圈/非法坐标即可触发。此为静态精确运算反例，未执行原目标或候选。

原21+66B完整必要包已单独落required-leaves-native-pack，report所有请求ok、bytes verified。已报lead/root，**duck不应内造私有subtract替身，也不回退通用工程策略/新造输出镜像**；待root提供可复用的原业务policy入口，prefs依赖该最窄修复。此差异不能用简单线段长度或normalize方向绕开。

## 副作用与缺口边界

本函数只分配/返回候选，不改Point/storage/PublicWalls，不生成世界/碰撞几何，不清旧偏好；caller收集/取消/请求生命周期仍由duck的同一owner闭包负责。源/目标读取失败、from dst错误、from addr错误、required publicwall缺失均panic，不continue/default空。空输出只来自from.normal None，不能借它掩盖失败。

inactive字段/结构padding未初始化，不声称原零填；现代构造必要占位只在变体不读的字段，不能把占位当业务默认。当前原typename完整return泛型没有从void PDB签名证明，本文仅锁有序kind载荷/score行为和输入真实owner，不凭名字补定义。

建议验证：normal None绕过所有required读取；first缺失/first坏dst/to缺失/from坏addr的失败顺序；同墙短距离不读PublicWalls；不同wall三候选顺序/0与1两种dst；ZERO/NaN/inf方向的glam门；strict正dot折扣与-0 signum；samewall的负/正/±0/NaN diff；半周期合法反例；不存在额外Y/width读取。未运行Cargo/native/GPU，本合同不宣称全楼梯或游戏链已验收。
