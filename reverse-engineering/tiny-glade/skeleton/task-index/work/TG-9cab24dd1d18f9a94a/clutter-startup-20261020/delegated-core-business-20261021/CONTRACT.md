# Canonical Curve 构造器：两个已证单态原f32域

原owner clutter-startup-20261020保留，root明确同构造器/必要测试caller授权。本source before c653ddb2e61f4aa1f0a8b4d78ac7e6a012581e26b0d02d1b590c9b7dc6db50bc；不改生产/Cargo。

## 完整原函数与身份

同B RNE f507a6ce/PDB2d9686b8，TGf768/core-duck-building-20261021/curve-constructor-readonly/native-pack的完整try2@140153900/698B、try3@141269ce0/702B均statusok；本worker已独读入口/float loop/normalize/dup和drop分支。crashy@14126a1e0/613B复用readonly-compute-water/value-lifecycle-native-pack/curve-crashy-new，已独读完整实指令至失败分配/返回分支。三个原证仅覆盖(Vec2,Semantic2D)、(Vec3,Semantic3D)、Vec2 crashy(false)；不顺带宣称Thresholded3D/XZ/自定义Point全原证。

## 原完整failure/ownership序

len<2先TooFewPoints(len as u8)，由owned Vec自然释放；否则先allocate len个f32坐标，再读first到previous、初始length=+0。遍历包含first，difference=previous-point，Vec2纯sqrt(dy²+dx²)，Vec3纯sqrt((dy²+dx²)+dz²)，每一步f32 length+=distance并存coords，绝不f64/epsilon/FMA。原try2循环展开2项不改变累加依赖，现代同一线性循环保该顺序。

总length只有有序==0先NonFiniteLength；NaN/Inf继续逐项coords/=length，先写回再absbits有限检查，第一个非finite失败。之后reject_duplicates=true才按有序相等逐相邻对，首重复DuplicateSegment(i)；false允许重复段但全零总长仍失败。错误按coords先drop、ownedpoints后drop，由现parameters_for_points借用内部局部释放再外层?传播保持；不默认成功。

crashy原R8是track_caller Location，非bool，原内联try(false)。保现公开bool参数兼容其它已有caller，只给该wrapper补#[track_caller]并仍unwrap；原stairs caller已将bool传false，不基于Location地址低位发明true。

## 最窄现代实现设计

不能直接把CurvePoint::metric_length/metric_distance_f64/normalized或CurveMetric::segment_length替换为f32；它们是既有通用稳健API，保持逐字不动。也不在stairs producer造私有nativecurve替身。采用同canonical Curve构造器的编译期policy：

- CurvePoint增加默认0的原构造单态维度const，只有内建Vec2=2/Vec3=3；自定义point默认不启用，不破坏其已有实现。
- CurveMetric增加默认false的原构造域const与默认转调现segment_length的constructor_segment_length；Semantic2D仅当Point维度2启用，Semantic3D仅维度3启用；Thresholded3D、Curve3XZ、自定义Point/Semantic保持既有稳健路径。
- 已证2D分支constructor段长用投影Vec2显式f32 dy²+dx²；已证3D用现Vec3纯f32 metric_squared().sqrt()（该method没有f64补偿；pinned glam正平方相加与原先xy后z相同）。原segment_length本体不改。
- parameters_for_points仅原构造域按上述原序：去输入finite预扫描、先分配、只length==0早退。其余域保既有finite预扫与!finite/<=0总长门，段长仍现segment_length。使用编译期const单态优化，无运行类型比较/注册表、无多份生产curve。

这样原值域恢复限制在完整原证的两个真实类型组合，通用稳健构造/测距/切片方法不被这次native恢复误改。已证组合的reparam/extend等同canonical parameters caller自然读取同原参数业务，不增私有策略owner。

## 实际direct caller与旧测试必要迁移

当前parameters_for_points唯一段长调用；现该source reparam、去重更新、try_extend、serialize projection/try_new wrappers复用它。通用slice measure_length通过CurvePoint robust metric不动。utils/tests/curve_properties.rs第506..507旧2行仍对length=[1e-30,MIN_POSITIVE,1e20,MAX]强制try_new成功，和完整原try2严格冲突。root已授权同work附ROOT_CURVE_PROPERTIES_TEST_HUNK.patch+current beforeSHA，将这2行改为原NonFiniteLength Err断言；其它右/左slice和measure_length稳健断言原样保留，不删测试、不伪cachedcurve。当前测试before已冻结curve_properties-before.rs/baseline.json，不改生产测试。

## 验收

正常Vec2 3-4-5累计长度/坐标、Vec3带高差两段长度；len0/1；重复false成功/true首重复、全重复true仍NonFiniteLength先于dup；两类型极小平方下溢/极大平方上溢均失败；NaN/Inf输入失败且不走Default；crashy(false)重复成功、失败panic真实caller位置由track_caller传递。补通用metric_length极值仍非零与custom semantic/Thresholded/XZ旧稳健路径不变的回归，防误扩大值域。测试由root Cargo统一，worker不执行native/Rustc。
