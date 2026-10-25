# 源码范围与接口定名

GRAPH_CONTRACT_V2 已由 Water 独审准入。仅本源七个业务方法、已锁真实类型/六表及直接测试调用迁移。七引用借用面最终定名 `parts_mut()`，返回 `StairsStructuralGraphParts`，相较合同末段拟名 assembly_parts 只有当前 Rust API 定名差异，无新增算法/分配。

结构图内部与六表均采用 RandomState；原 IndexMap 边表通过 pinned petgraph 自身访问，不另复制。原 Option 的 None niche 在安全 Rust 由 Option 表达，不把句柄 slot17 擅定合法域或伪造默认句柄。StairColliderHandle 原现安全包装、hash 与排序完整保留。

外部卸载 fixture 仅把真实 span 身份显式写为 `(edge,0)`；该测试原来就只建单采样。decorator_chain 确认未并写，后续其 registry init 将以 root 实际 after 增量。没有生产接口的默认采样转换。

14项意义测试包含保留原四闭包及补齐多采样、两入口永久false、缺span早返、自环、断临时边、失败时部分状态、真实六表clear。rustfmt只确认parse，未Cargo/GPU。完整create/distortion caller、两62KB mesh、Info63、visual RT/Drawable与宿主仍未闭合，不能由本包推定。
