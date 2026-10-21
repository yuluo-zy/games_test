# 原楼梯必要 Curve 构造入口合同（只读；root 协调 canonical owner）

live show 保留 `live-show.json`：TG-9cab24dd1d18f9a94a 历史 owner clutter-startup-20261020 不改claim。当前源码 tiny-glade/crates/utils/src/curve/mod.rs SHA c653ddb2e61f4aa1f0a8b4d78ac7e6a012581e26b0d02d1b590c9b7dc6db50bc。本worker没有此源写权，不写production/第二curve算法。

## 全函数身份与签名

- 原B `try_new_from_points<Vec2,Semantic2D>` 140153900/698B，PDB60:13520，新增完整native-pack/curve2-try-new-full；逻辑输入是 owned Vec<Vec2> 和 `reject_duplicates:bool`（R8低bit），返回Result<Curve2,CurveError>。**try的R8确实是bool**，153961保存后153abb test低bit控制重复点检查。
- Vec3/Semantic3D同入口141269ce0/702B，PDB1506:13576，完整native-pack/curve3-try-new-full；owned Vec<Vec3>和相同bool。
- 原 `crashy_new_from_points<Vec2,Semantic2D>`14126a1e0/613B，PDB1506:18416，复用 ../readonly-compute-water/value-lifecycle-native-pack/curve-crashy-new。逻辑输入仅 owned Vec<Vec2>，R8是 `#[track_caller]` Location（14126a1fb保存至RSI，失败时传入unwrap错误报告），**不是reject_duplicates**。这个实例内联try(false)并unwrap，不检查重复点；当前手工API额外bool可以保持兼容，但本原call必须传false，不能由R8地址低位猜bool。

所有pack RNE/PDB SHA由manifest绑定；没有执行原target，也没有C版本验证。

## 完整原构造顺序

1. len<2：返回TooFewPoints(len as u8)，释放owned input；不先分配coords。
2. 分配len个f32坐标；初始previous=first，length=+0。遍历包括first本身，逐项previous-point，计算真实f32段长后 `length += distance`，push累计length；previous=point。不是先做points finite扫描、不是f64合计。
3. Vec2段长纯 `sqrt(dy*dy+dx*dx)`。Vec3原141269d90..db7为纯f32平方，先`dy²+dx²`后加`dz²`再sqrt；无f64 fallback、无epsilon。原两种语义只为所选Point类型，本任务不泛化其他SemanticThresholded3D。
4. 只有 length==0（有序等于，含负零）立即返回NonFiniteLength；NaN/Inf不在此提前拒绝，而进入下一步。
5. 每个coords依次除length，写回后按abs bits<=0x7f7fffff验证finite；第一个非finite返回NonFiniteLength并释放coords/input。之后才进行重复点检查。故本链所有非finite输入最终失败，但不能把原时序改成输入预扫描以宣称原异常顺序相同。
6. reject_duplicates=false：成功移动points/coords/length。true：从首相邻对依序比较，遇两分量全部有序相等（Vec3三分量）返回DuplicateSegment(index)，否则成功。重复段并不总失败；false允许[0,0,1]，但所有点相同仍在length==0失败。
7. crashy实例用同完整顺序的try(false)结果，失败unwrap保持caller位置，所有成功/失败前缀由Rust ownership drop，不默认成功。

## 当前差异与最窄 canonical 方向

当前 `CurveMetric<Semantic2D/3D>::segment_length` 只由 parameters_for_points调用（rg结果150/153/159/164/242），但它委托 CurvePoint::metric_length，Vec2/Vec3的后者在极小/极大输入使用f64补偿，改变原失败域；parameters_for_points还在分配前扫描非finite，并在长度阶段提前 `!finite || <=0`。

建议 root 在本已存在 canonical 构造器必要范围修复：让**构造器使用的 CurveMetric 段长**保原f32，不改其他几何/归一化/metric_distance_f64方法；参数构造按上面原顺序，不另建 native curve wrapper 或复制 producer 私有构造器。现手工通用API保留reject_duplicates；原crashy链传false，并恢复track_caller。具体generic方法设计由唯一owner按本原证复核后执行，不能未经验证全域替换稳健metric。

明确反例：Curve2输入[(0,0),(1e-30,0)]，原f32 square下溢为0→NonFiniteLength，当前f64补偿得到非零length→成功；[(0,0),(1e30,0)]原平方Inf后coords出现Inf/Inf→失败，当前f64补偿可能成功；含Inf/NaN的点原分配coords后在规范化失败，现输入预扫描时序不同。[(0,0),(0,0),(1,0)]false应成功/true应DuplicateSegment(0)，避免把“不拒重复”误改全允许零长度。

## 验收边界

主链compute/path采样必须等待canonical正确入口；不能在producer catch Err继续/改空curve掩盖。建议独立CPU测试这四类极端/重复分支、正常3D带高差曲线、len0/1错误、crashy caller panic；所有执行由root统一。本合同未写任何候选/生产源。
