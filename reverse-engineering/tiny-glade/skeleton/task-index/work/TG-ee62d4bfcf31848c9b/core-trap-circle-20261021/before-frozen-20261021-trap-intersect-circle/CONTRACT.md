# 活板门直接依赖：Circle2d::intersect_circle 完整原义合同

范围仅现 `crates/utils/src/geometry/circle.rs` 的该函数与真实边界测试；lead明确必要闭包授权，TG-ee62d4bfcf31848c9b先live NULL后normal claim，owner core-trap-circle-20261021。保持同文件其它算法和API。生产编译/GPU/root统一。

## 身份和完整原证

B RNE SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03；PDB SHA 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911。PDB circle.rs module767 procedure记录2876，VA140c97a40，完整207B（至140c97b0f前），源码167行，非依赖同名猜测。原证为兄弟TG0e52/core-visual-data-20261021/before-frozen-20261021-canonical-visual-data/input-gates-pack/circle-intersect-full.{json,asm.txt}，report statusok。

## 输入、返回和失败序

输入两个Copy Circle2d {center:Vec2,radius:f32,rotation:f32}，rotation不参与本函数。输出 Option<(Vec2,Vec2)>；原None tag0，Some tag1。没有panic、allocation、资源副作用。调用者trap@bef21比较返回tag0，bef30 JNE跳过，因此None才继续创建blueprint，Some跳过。不要把该函数改为简单contains/圆形bbox/只比较点中心。

1. 140c97a43..a60：d=self.center-other.center；distance=sqrt(d.y*d.y+d.x*d.x)，单精度独立乘加，无FMA。
2. a64..a7b：仅 `distance > self.radius+other.radius` 返回None；指令UCOMISS/JA是严格有序大于。NaN不满足此条件。
3. a81..a92：仅 `abs(self.radius-other.radius)>distance` 返回None，也是严格有序大于。相切相等都继续。不能改为 `!(distance<=sum && diff<=distance)`，那会让unordered走None。
4. a94..ac4：s0=r0*r0，s1=r1*r1；a=(distance*distance+(s0-s1))/(distance+distance)；h=sqrt(s0-a*a)。原没有max(0)、clamp、d==0守卫、finite守卫、半径正数检查。
5. ac8..b03：delta=other.center-self.center；mid=delta*(a/distance)+self.center；offset=(-delta.y,delta.x)*(h/distance)。Some((mid+offset,mid-offset))，顺序不能交换。
6. b07..b0e：写tag1并返回；d0/NaN/Inf导致载荷NaN时仍保Some，调用者消费tag，因此NaN载荷不是可以随意“修复”的无关数值。

## 当前差异和最窄实现

当前额外 `distance==0 || !distance.is_finite()` 返回None及radicand.max(0)均非原；组合 `!(<=&&<=)` 又改unordered。只移除这些额外语义，按两次原严格大于失败分支；保持现公开签名和现正确点序。不会为trap写私有复制函数。现rectangle contains_point_padding只读复核：原130B先将p-center投影axis/perp，再(size+padding)*.5，四个ordered<=，现函数相同；无必要rectangle改动。

## 验收

(0,0)r5与(6,0)r5→Some((3,4),(3,-4))，真实原3-4-5几何核对点序；外切、内切→Some两相同点；分离与严格套内→None；同中心同半径→Some NaN载荷、同中心不等半径→None；center NaN及无限距离但半径和也无限的unordered情形保Some NaN。额外明确trap直接消费者None才允许：套内None允许、相交Some拒绝，同圆SomeNaN也拒绝。测试不宣称运行native原程序；原机器码是控制流oracle，root统一cargo运行Rust。
