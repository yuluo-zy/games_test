# 邻接对齐全组合搜索合同（待独立复核）

原 B 14080c360，完整733B/PDB已绑定于 helpers-native-pack/best-neighbor-full。该叶不是简单“最高分 id”，旧 Option<u32> 空体必须换成真实 choices+score。

输入为有序邻居 slice `&[(DecoratorId, Vec<(AlignKind,f32)>)]` 及已有方向掩码 u8，返回 `(Vec<(DecoratorId,AlignKind)>,f32)`。原邻居 stride32，候选 stride24=20B AlignKind+4B score；输出每元素24=4B id+20B kind。不同分配布局以 Rust 容器表达，不以裸字节重建业务。

- 邻居 slice 为空：无分配返回空 choices、score+0。
- 非空：取首邻居所有候选依次递归处理剩余 slice；初始最优空 choices，score=-f32::MAX（原14292597c）。
- kind.tag0 mask=1；tag1 mask根据 `first.x*second.y` 与 `first.y*second.x`：前者有序大于后者时2，其他情况4。原14080c424 ucomiss /427 setbe将 unordered 也归4，不能直接用 `<=` 丢NaN支路；其他 tag 在原14080c5a7 panic。
- 递归入参 used_mask|mask。返回后先 current.score+child.score，再加 penalty（mask&used_mask !=0 时 -10，否则+0；原142a16d10）。顺序不可重结合为child+penalty再加current。
- 只在总分有序严格大于现最优时取代；NaN从不取代；tie 保留第一个。每次获胜接管真实 child Vec，在首部插入当前(id,kind)，释放之前赢家；失败释放该 child。
- 当前首邻居无候选：返回空/-MAX，不提前改变父层算法。父层仍做完整float加法/比较，不能人为把缺候选判“不可能”跳过。
- 不排序、不贪心、不截断度数、不把 all-combination 搜索替换单邻居最优；原指数复杂度依每节点有限方向候选，按原真实算法恢复。

验收（候选测试，未执行）：空输入、1/多邻居全局选择、同方向-10使局部高分让步、tie保首个、空候选传播、NaN cross mask4及NaN score不获胜、非法tag panic、输入未变。原申请独立复核后才写source candidate；与完整30KB×2请求producer联动继续。
