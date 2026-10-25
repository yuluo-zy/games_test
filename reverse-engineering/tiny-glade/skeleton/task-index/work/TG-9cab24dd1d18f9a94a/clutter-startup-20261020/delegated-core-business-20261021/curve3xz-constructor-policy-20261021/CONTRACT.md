# Curve3XZ 构造参数策略的必要增量合同

只恢复原 B old-history 所用 Vec3+CurveCoordSemantic2D constructor；不展开其它 Curve 方法/模块，不新增 interpolate 私有曲线，不改共享 lib/Cargo。root 明确授权，TG9cab live仍由clutter-startup-20261020持有；本 worker 是此增量唯一writer，保旧claim，在原合法delegated work追加本目录。

## 唯一基线与原身份

before是已独审 `af00b0d9cfdf157e44e265c2b15ce62e877a29ce223f8ee7df23a9ba7abfce1f`，从旧delegated-core-business-20261021/candidate/tiny-glade/crates/utils/src/curve/mod.rs冻结；不能从尚未整合af00的production生成替代整源。原af00冻结不改。本次candidate.patch只从af00生成，root必须先装af00再按SHA应用此增量。

复用TG0ae3/core-duck-building-20261021/interpolate-required-readonly/curve-native-pack：原old Curve3XZ try1412699d0/782B/PDB1506:10828；output XYZ try141269ce0/702B与coord141269710/694B只读复用身份，不修改它们的算法。原RNE f507与PDB2d9686身份保持；已独立PE映射逐指令字节比对并验证全范围无间隙：213/193/163条、782/702/694字节，NATIVE-BYTES.json含精确hash。只证明B，不将队列target C作为C行为验收。

## 原完整782B的输入、顺序和失败

原输入owned Vec<Vec3>及reject_duplicates bool；返回原Result<Curve3XZ,CurveError>。现代保持已有typed Vec所有权/RAII，不复刻Rust allocator/析构/EHABI。

1. 1412699f5判断len<2，先返回TooFewPoints(len as u8)，并释放传入points allocation；不得在此前扫任何坐标finite。
2. len>=2先分配len个f32坐标；原alloc失败走panic/error handler并在EH释放points。沿af00 Vec::with_capacity与RAII，不换default/Result吞错。
3. a67/a6b只读first.x和first.z；a80..b0c按原顺序遍历，包括first自身。每段以previous-current作差，直接f32计算 `(dz*dz + dx*dx).sqrt()` 并顺序累计，写points_u。不读Y有限性、不用f64/hypot增强，不修改points的任何分量。
4. b14/b19只有有序 length==0 才提前NonFiniteLength。NaN/Inf继续逐项除以length；b30..b5a逐点规范化u并按absbits<=7f7fffff要求有限，首次失败NonFiniteLength。规范化完成才走duplicates gate，不能提前用duplicate错误替换非finite/零长错误。
5. reject_duplicates=false跳过重复检查；true时以相邻点的**完整XYZ有序相等**判断。bae..bcd和c36..c5d每个维度均有JNE/JP：任意NaN使这一对“不相等”，不是强行拒点或只比XZ。±0相等；相同Inf的Y有序相等。发现首个重复返回DuplicateSegment(index)。没有重复则move原points/新points_u及length构成Curve。

复用pinned glam0.29.3 Vec3（repr(C)、derive PartialEq，其XYZ f32比较有序），不手写三维比较或私有native metric。现Vec3.projected为Vec2(x,z)，S2D.constructor_segment_length已经是原 dz²+dx²纯f32；这些method body无需再改。

## 最窄源码范围

af00已经以CurveMetric::ORIGINAL_PARAMETER_DOMAIN编译期策略区分constructor原域与工程稳健测距，并已证明(Vec2,S2D)/(Vec3,S3D)。本轮仅将S2D的原构造域从pointDimension==2扩到已证2或3，增加原782B/不读取Y的中文说明。Vec3XZ随后自然走现af00完整f32累计、length==0、u有限、fullPoint duplicates流程，移除其先point.is_finite扫描；不加任何运行时history专用判断。

保留S3D const与constructor body，Vec2原f32策略不变；Thresholded/custom默认未证域仍保持原工程有限性前门与稳健度量。metric_length、metric_distance_f64、normalized、segment_length、所有slice/查询/坐标方法body均逐字不动；该alias的其它既有重参数化方法仍复用同canonical constructor policy，不另立算法。无额外Y归一化/默认几何/第二cache。

现af00同模块engineering/unverified测试把Curve3XZ tiny1e-30当工程域成功，此断言必须迁移成原NonFiniteLength；同测试自定义Engineering、Thresholded与metric稳健断言不变。未触及另包ROOT_CURVE_PROPERTIES_TEST_HUNK或curve_slice的极端测距断言。

## 原历史 consumer 反例与配对

完整interpolate原1407cfd87..fd94以old.points.clone调用Curve3XZ.Try(true)，Ok才继续reuse比较，Err进入fresh/dirty。原old.points=[(0,NaN,0),(1,NaN,0)]的XZ累计为[0,1]、length1、normalized[0,1]，XYZ点不同，因此XZ constructor必须Ok并原样保Y。旧先Vec3.is_finite策略会Err，错误清空第四opaque Vec并设dirty。

与之相反，原最终输出Curve3.Try对同两点应Err，写request.interpolated_curve=None；这一输出失败不等于old history无效。reuse时仍可重插原SampledStairsPoints整个value，保第四opaque Vec和dirty旧值。不能把NaN Y修成0或把XYZ输出Err映射为全部history fresh。

真实业务World fixture由interpolate唯一writer duck在其已授权同源候选配对，不让utils反向依赖core/history，也不在utils造fakeHistory：Requests从完整graph→producer获得finite geometry，old canonical sampled_points换成上述同点数XZ1/YNaN，metadata按实际Info/RemovedSupport取得，真实visual.reset_edge创建force=false，USE原true。预期reuse保opaqueWords与dirty=false、最终Curve3=None。duck已明确接受该测试要求，当前等待其完整interpolate合同独审后落source；此增量不假称该World测试已实现/通过。

## 本源验收与限制

拟三个新增unit：忽略非finiteY但保原NaN bit且与XYZ输出错误分离；fullXYZ有序duplicates（Y不同、NaN、±0）而非XZ-only；len/零长/规范化/重复的原失败顺序及tiny/overflow原f32域。现工程测试只改XZ那项断言。重复或非finite分支不能返回默认成功；所有输入Vec/normalized cache由既有owned Rust路径释放或保留。

实现后产af00增量patch、before/after SHA、保护API body hash表，作者scratch -F0重现；独立source review通过后root执行utils及真实interpolateWorld测试。当前无Cargo/rustc/原target/GPU执行，不把字节匹配或syntax parse冒作行为验收。
