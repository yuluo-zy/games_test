# 必要梯位叶合同（待独立复核；不代表请求 producer 完成）

## 范围与身份

仅恢复既有 `stairs_assemble.rs::calculate_adjusted_ladder_positions` 和内联闭包；canonical 类型只落既有 TG-280c 的 `stairs_assembly_params.rs`。原 B RNE SHA `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`，PDB SHA `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。`native-pack/ladder-full` 598B、`ladder-closure-full` 603B，完整函数/清理范围；input-leaves-native-pack 为 4 个完整短叶。工具未执行原程序。无 Cargo/生产写入/C 版本等价声明。

## canonical 输入纠错

1. `derive_stair_node_params` 完整 455B 在 TG-f768 的 native-pack。输出每端 32B：anchor 位于 0..24，邻接度字节位于 +24。0/1 标签分别由 terrain/roof 分支写入，二者 payload 为 +4 的 StairExtra 首个 f32。墙分支写标签 2，payload 的 +8 为 **WallId u64**，+16 为 angle，不能继续把首 qword 当作 Vec2 position。
2. 独立墙身份链：14080c788→140a1c240，后者完整 75B 在 +8 返回 storage WallId；14080c79e 将该 RAX 放 RCX；140959e80 完整 7B仅返回 RCX 与角度；14080c7b9 将 RAX 存 anchor+8。`try_as_wall` 140959ef0 完整14B只在标签2返回+8；`wall_normal` 旧数学仍为(-sin,-cos)。此纠错不创建墙组镜像。
3. canonical `StairNodeAnchor` 用 Terrain{height}/Roof{height}/Wall(StairWallAnchor) 表达真实已证 variants；每端的 `(anchor, neighbor_count: u8)` 为 Rust typed projection，非额外 owner。原 derive 邻接度截为 u8，无饱和门。
4. `StairGraphEdgeIdentity` 原 140959e70 完整10B：`(a as u64)<<32 | b as u64`。本叶保留传入两端有序身份，仅在排除当前边/查询旧表 key 时调用已有 `StairSegment::new` 无符号 min/max，再转换 identity；不要使用当前 StairGraphEdge 的二个 u32 hash 代替这个 u64 key。
5. 第二表是 canonical 请求 owner +0x198 的 `HashMap<StairGraphEdgeIdentity, SmallVec<[StairSubSegment;1]>, RandomState>`，并非 SampledStairsPoints。独立 producer 1407d1982 指向 +198，1407d19bb 调 PDB 1546:14372/141293420，后者从每个请求 +128 取两个端点、从 +130 clone SmallVec，通过原 1412670e0 的显式 typed insert 写表（PDB 1505:58576）。value 元素 stride4，kind@0，count u16@2。字节1由并行 compute 原证进一步确认为真实 flag（默认新段写1，端点padding写bool异或），必须保留，不得当padding；本梯位叶只读取kind。
6. core 尚无 smallvec 直依赖，root 需在既有 Cargo 插入 `smallvec = "=1.13.2"`；本候选不生产写 Cargo。HashMap 必须 RandomState，原 hash_one 的 Entity 同地址 ICF 别名不用于推导业务类型。

## 598B 主算法与失败/浮点顺序

输入：两端实际 anchor+degree；两端 Vec3；只读 StairsState；上述旧 segment-map；与 from/to 对应的有序两个 DecoratorId。返回 [Vec3;2]。不得把此几何顺序先转成排序的 StairSegment。

- 用 `from.y > to.y` 选择较高端；原 JBE 包含 unordered，NaN 走 else。较高端是 wall 且 degree==1 且较低端非 wall，才设墙端移动量 0.28（位模式 0x3e8f5c29）。相等高度按第二端作为所选端，不能提前 return。
- 无条件计算 xz 差，`1 / sqrt(dz*dz + dx*dx)` 再乘差，构造 Y=0 的方向。零水平长度保持原 NaN；禁止 normalize_or_zero/epsilon repair。
- 若 to.y>from.y 且 from.degree>1，先调用 closure(from,to,last=true)。false 即主移动量0.3；true 才继续。否则进入第二条件。
- 第二条件是 from.y>to.y 且 to.degree>1 时调用 closure(to,from,last=false)。false 主移动量0.3，否则0。原 ucomiss/JBE 使 NaN 比较不触发 closure。
- 若 from.y>to.y：from 减方向×墙移动量，to 减方向×主移动量；否则 from 加方向×主移动量，to 加方向×墙移动量。保 Vec3 的 Y 运算，不能只改 xz 丢 signed-zero 细节。
- 常量读取在 ladder-constants.json，另有1.0位模式0x3f800000。

## 603B closure：必须保持顺序与短路

按实际 StairsState.graph().neighbors(node) 的 pinned petgraph0.8.1 邻接顺序遍历，不复制图、不排序。

- 原先对每个邻居验证底层 edge key 必存在（缺失在1407d9482 panic），然后构建两条 normalized StairSegment；等于当前边则跳过。
- 对其他边转换 canonical packed identity；表缺 key 则继续邻居，不 panic。
- **第一个命中的 key 即决定结果**：last=true 检查非空列表最后 kind==3；last=false 检查第一 kind==3。空列表立即 false，kind!=3立即false，不能 `.any()` 扫后续命中。
- 节点不存在、无邻居、全部缺表项/当前边，false。无写入、无分配、无新资源默认值。

## 候选验收（未运行）

1. 上升/下降两方向的 .28 wall terminal 偏移；等高端选择；双 wall 无 .28。
2. junction 两次 closure 的端点方向/first-last；缺表项继续与首命中 false 短路；空 SmallVec 与 spilled SmallVec。
3. actual 同 StairsState 图和 identity map，确认输入均不变；反向 pair 归一化 identity。
4. zero xz 保 NaN，不修复；NaN 高度走 unordered 既定分支，不触发 junction closure。
5. 类型叶检验高位 WallId 不作 float roundtrip、identity packing、SubSegment size4。

## 尚未完成边界

完整 request<false,true> producer、compute/interpolate、两 directed graph 和完整 0x1c8 owner 的 contract/实现继续进行。本叶可先独立编译复核，但在这些真实 caller 未完成前不计 cache dst4 或楼梯主链完成。当前没有生产 caller 使用旧空 ladder；后续 cache 与 interpolate 必须用同 canonical map，而非在消费者本地造表。

## 独审后端点顺序修正

lead发现原598B两个ID是原序，closure内部才做无向规范化；候选已改入参 `[DecoratorId;2]`，ascending用第0、descending用第1，内部只对排除边与表key排序。新增反向29→7升/降两实际邻接测试，不能以存档无向identity去推几何端点顺序。
