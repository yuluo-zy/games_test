# 完整 compute 源码候选

只恢复原140807090/10808B与两个真实game callbacks，不把库容器内部算恢复；以冻结正文加NaN/normal/dirty独立addendum为唯一合同。source包含此前已审node/best叶，root可用相对node patch避免覆盖旧整合。

真实输入按StairAssemblyInput/Overrides分组；端点ID保有序到最后canonical无向key。VisualTerrain实际现代owner为`TerrainHeightsData<VisualTerrainHeightSsbo>`，源码terrain_height.rs的泛型marker、View与sample_at_world_xz已读；没有默认physics资源替换。测试同World故意同时插不同physics与visual grid，沿实际storage/cache/graph→collect/derive→compute，只验证本闭包，不伪造未实现request成功。

原None/Some(empty)、先高度采样再dir覆盖、实际curve.points_u宽度、raw tangent NaN、相邻原输入分类、三类压缩/count wrapping、每次Curve重建、两端padding与位域、真正dirtykey都完整实现。BitVec迭代借用by_vals，不通过owned into_iter触发into_boxed_bitslice额外缩容。

已核pinned bitvec1.0.1的repeat/insert/push/by_vals与glam0.29.3 normalize/normalize_or_zero/is_nan源码。仅需ROOT_CARGO_HUNK添加系统crate bitvec exact1.0.1；shared Cargo由root唯一修改。

七新增测试涵盖height双侧width/4和-.025/.1、endpoint覆盖先后、Someempty/覆盖Vec分配直接接管、native原CurveU与零段NaN、steep整体行压缩及真实coords重建、全部16个双端bool组合及rawDir/callerXZ、NaNkind1与65536段u16回绕、短tangent真实bounds失败。未运行Cargo/rustc/目标；需parent独立source review再root CPU。

保留边界：shared Curve当前极端f32/错误顺序差异等待canonical owner修复；不私建算法/吞错/声称全输入native等价。当前完整request、walk、interpolate、cache及两个62KBmesh仍未因此完成。
