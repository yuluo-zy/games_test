# stairs必要三叶源码独立复核

core_business_lead，2026-10-21。接受交付索引所列三源窄候选，完整原证合同已有独审；本轮已独读三patch/完整函数/测试，核production before与candidate/patch SHA，并在/private/tmp隔离副本用patch -F0真实复现，逐字节匹配。未Cargo/rustc/原target或GPU；root需先添加exact smallvec1.13.2，再统一CPU。

core：WallId首qword不作float，三真实anchor/degree类型、packed high-a/low-b samplekey与4B SubSegment业务flag分别明确，AlignKind20不含外score。数学完整73B纠错flag!=0||byte2==1、unsupportedZERO；outgoing仍原panic。观察facade仅交叉验数学，不成为第二owner。

best733B：完整有序全组合递归，原cross unordered mask4、(current+child)+penalty、strictgreater/tiefirst、空候选/-MAX传播，接管真实child Vec避免整表clone；无greedy或空体。

ladder598+603B：原from-to和dz²+dx²，.28 terminalwall/.3junction、unordered与零水平NaN、真实petgraph邻接与firsthit短路。首次候选把端点有序身份误作无向排序StairSegment，独审已拒绝：规范化ID用于mapkey不等于几何方向。现修为有序[DecoratorId;2]，只排除key内部规范化，并补29→7升降两图反例，最终after必须7eb0058346411ecfdbb1bd31851b83f2d376a841721f1ebc91f5d328820f2585，不用旧a9e740e3。

源码检查无其它真实caller因这三签名而断裂；完整requests/compute/interpolate/cache仍在恢复，不能把本叶交付宣称该主链完成。现3个core新测试+纠错原四支、5个alignment、7个ladder只完成source审查，root执行后方有测试结论。保原中文说明与实际副作用/失败域，不加默认资源。
