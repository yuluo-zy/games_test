# walk / directed flags 完整源码独立复核（lead）

结论：通过原walk8680B、矩形receiver1264B及true/false已配对inline flags最窄源码范围。逐项读完增量正文、八新测试与原合同，并核现PublicWallState/Rectangle/CurveSampling/WallSpace实际API；whole与compute前版两patch在隔离目录F0退出0且字节完全等于b42f8928。未Cargo/运行，不改生产，不把helper通过当请求/cache/游戏链验收。

矩形用真实源shape字段/已发布墙长；PublicWallState::simplified_curve_2d确实借authority.source.curve，不是terrain Curve3。双端半宽缩进仅tag1/flag1，两端一起判方向一致才替换；负delta交换所有端点输入，最终倒转points/tangents且翻tangent、widths/heights/完整SubSegments。原共享RectangleSliceState跨整个emit stream复用，保角点record，而非直线或重置每角pending。receiver先实际W/offset点去重、needHeight false不污染previous/段/height；true按原strict NaN kind1和kind4分类/flag/count回绕，kind3末四行替换无count增长。Some(empty)不改成None。

通用分支保两个tag门、.001有序门及原reserve/首端project先于末端tag检查；ceil cast、wrapping_add count不额外封顶。按实际sourcecurve等距u取点，首尾caller normal与内部墙tangent来源不同；width push先于position。count>1真实central差分，不复用compute前差分。1.25 strip四角有序convex检查保双侧quad顺序和strict1e-8，删中间三Vec同下标、重算左右受影响tangent、从头重扫直到稳定，未用凸包/RDP/单遍简写。保护首尾，下标失败未吞。

flags是实际有向GraphMap，完整scan收集后才统一修改；cross.signum()>0使+0正、-0/NaN另一侧，未改为cross>0。两个turn侧累计，required mut lookup失败保原；tag0覆两个位、tag1只设flag不改byte2及向量、None保持，反向边不自动相互修改。原source530..584业务不能当容器derive省略。

八新测试含实际InnerWalls→consume发布矩形/折线/hairpin、周界反向/缩进回退、central tangent/三空门、双侧strip剪裁端点保护、真实receiver NaN/needHeight/kind4/u16wrap，以及真DiGraphMap±0/NaN/双turn/方向/payload。源码API复核通过不等于实际类型检查，root仍须按锁定依赖执行。

仍有真实canonical共享差异必须修：WallSpace.subtract半周期f64 vs原f32、CurveUCircular finite vs原!NaN失败时序；加上上一compute的Curve构造极端fallback。没有在本源复制第二数学policy/吞错。完整双request、对齐偏好、历史/interpolate/cache/mesh仍未完成，不把helper产物直接挂宿主造成假闭环。

最终source `b42f892823a61e2426f0cbe38cc7a8b59c3f9367b320103b654aa0741133e9b4`，production before `87435b0f6c78d5695d7f1d89a2f8768585d7a62a34ab26a60d4e175ece0427c6`。历史node/compute/leaf审查仍用于对应已恢复片段，root应消费每源最后版本。
