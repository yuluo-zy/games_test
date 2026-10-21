# walk_shape + directed alignment flags 完整源码候选

沿现stairs.rs继续改，保此前node/compute/best源码。after-compute.patch是相对已审20386293的最窄增量；原始生产仍是冻结初始源时另交whole candidate.patch。未写生产、Cargo或其他source。

walk完整8680B+receiver1264B：实际PublicWall源curve/shape与已发布length，rectangle原stream和共享pending state，原连续角点/duplicate规则、needHeight两分支、kind4/1/2/3与u16wrap、steep末行替换；双端tag/flag缩进与同时回退、forward/reverse全部owner；通用sourcecurve真实间距、两端normal与central tangent、1.25 strip四角凹折检查、三Vec同步删除和重扫直到稳定。三OptionVec与OptionSmallVec不抹掉None/Someempty区别。

flags真实request内联两阶段扫描DirectedGraphMap后变更；完整false/true原块已由water独审，cross.signum()>0保+0正、-0/NaN非正；无straight不变，有两侧turn时分别传播flag/byte2；tag1只设置flag并保其它payload，不将边无向化。

已读当前utils Rectangle2d::circular_slice_segment_points/RectangleSliceState、CurveUCircular、WallSpace/ComparativeSpace、scalar float_grid::lerp、PublicWallState读法及sourceCurveSampling，复用现canonical。保两个共享精度/失败域缺口：subtract半周期f64补偿与原f32差异、CurveUCircular finite比原!NaN更严；不能声称全输入native等价，root须在唯一owner最窄修复，不私造policy。

八新增测试沿真实InnerWalls→PublicWalls发布墙/矩形：全矩形周界跨角且完整反转、双端缩进交叉同时回退、sourcecurve拐角central tangent与三个空门、hairpin双侧宽strip重复剪裁且保护端点；receiver needHeight/duplicate/steep replace/NaNkind1/kind4与u16wrap；实际DirectedGraphMap ±0/NaN/双turn/单项缺门/反向边/保payload。未运行Cargo/目标；需parent source review再root CPU。

上游两30KB请求/历史/interpolate/cache尚在推进，当前不把helper候选算整条业务链已闭合。
