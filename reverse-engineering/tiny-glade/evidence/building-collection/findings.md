# 原始墙发布采集链（2026-10-11）

`reconstruction/building/wall_collection.rs` 接受真实的原始 Curve2、原始墙业务字段及 512×512 u16 地形网格，沿原逐段采样策略生产世界轮廓、Curve3 的 U/长度、每点地形值与 relative-height metrics。没有将已解析世界数组包装成“原始采集”。标准容器/Arc、Bevy/HashSet/Vec/allocator/tabloid/parking_lot 的实现仍复用，未当作业务重写。

## 三版归属与差异

PDB 私有源 `crates/utils/src/curve/mod.rs`、`country-core/src/resources/terrain_height.rs`、`country-core/src/systems/wall/wall_state/publish_public_wall_state.rs` 都属于游戏业务，三版 checksum 均不同，不能仅因同源算法共有而排除。`three-version-comparison.json` 保存原二进制 SHA、聚焦指令和常量；`three-version-symbols.json` 为 helper 候选，包含的标准库泛型别名只是边界证据，不计实现。

- A resample `140e8c2a0`，terrain `140c8d2e0`；terrain 采用旧采样策略与固定范围 -2.5/20。
- B resample `14146bb70`，terrain `1409e9640`；terrain 从 record+50/+54 读界限，DLL 导入的 sampling-fix bool 选择两种采样。
- C resample `141972c90`，terrain `140c71040`；同样具有动态范围和 fix 分支，但代码/私有源不同。A/C 完整 publisher 没有以 B 结果代验。
- 原 terrain axis 为 `u = world / 130 + .5`；旧路径坐标乘 512，base index 至多 510，边界 fraction 可以到 2。fix 路径乘 512 后减 .5，负坐标截零，两个 index 均至多 511。不能用常见的乘 511 或通用双线性边界替换游戏规则。

## B 原发布采集

原 publisher `140a8b530` 只分段读取，排除 HashMap/Arc/Vec/事件队列展开。

1. `140a8c700..71d`：raw 点数至少 2 且原 Curve2.length ordered >0，NaN 不通过。
2. `140a8cb8e..cd28`：完整克隆 Curve2 的 points/u/length，并读源 shape tag；`140a8cd33..6a` 把 `tag == 1`、spacing `.5` 及 terrain/source 捕获传给私有 resample。
3. `14146bb70` 对每段使用 `max(segment_length,1e-8)`，间隔 `.5`，roundf 后加 1、最少 2 点；tag 1 将偶数 count 增为奇数。先保留首点，再采每段 j=1..count-1，保留原重复闭合末点，不补闭合边、不去重。
4. closure：源 wall+58（publisher row-end-18）大于零时用作地形 floor，否则 floor=`c02ccccd` (-2.7)。世界点 Y 是 maxnum(地形,floor)；输出实际调用私有 `Curve::try_new_from_points<Vec3>` 形成 mapped Curve3。
5. `140a8ce11..ce43` 再次按世界 XZ 采地形，relative = 世界 Y − terrain Y；接既有 metrics/projection。

关键字段纠正：`140a8cce4` 从 raw Curve2.length 取 XMM7 写 public+30，`140a8d662/140a8d987` 同源 length 写 public+28c；因此 public+30 不应该叫“权威墙高度”。`140a8d50c..d86e` 克隆原 raw 点并写 public+10，那里 count 是原始点数，不是 mapped 点数。公开世界 Curve3/真正 max-y cache 是不同的数据。源 tag 应与 raw shape tag 相同，不能随意组合。

## 原 dirty 消费者

`141458b20/141458a50` 只做 tabloid subscriber/locks，业务筛选在 `140adeea0/140adf4b0`。原 DirtyTiles 使用 64 个 u64 位集；每个 tile 通过已有空间 bucket 的 wall-ID Vec（stride0x30）枚举候选，而不是将每个墙全重采。游戏空间索引的生产者与两个 subscriber 的原字段名仍待恢复。

- primary (`140adf3e6..3f8`) 对存在且有效的 public wall，仅 minimum-relative-height public+27c ordered <= .5 加入原 InnerWalls+90 更新集合。
- secondary (`140adf9f6..fbf2`) 对较低 relative 或不在既有 InnerWalls+60 lazy 集合的墙，放进临时 terrain 检查集合；较高 relative 且已排入集合者跳过。NaN 走高 relative/membership 分支；Rust 比较策略保留该结果。
- 临时候选随后运行 publisher `140a8c0e5..13e`：世界 Y 大于源 floor 时，terrain!=Y 即更新；否则只有 terrain>Y 才更新。NaN、Inf 和 floor 比较由原指令验证。

Rust 提供 `dirty_tile_candidate` 与 `terrain_requires_republication` 供真实 Bevy 输入层消费。下文增量恢复 bucket 的ID membership；原订阅资源名、完整 metadata/query 与重叠传播尚未确认。

## 真实空间候选生产者增量

`reconstruction/building/wall_spatial_index.rs` 已恢复原 dirty 所用索引的 **wall-ID membership**，使 raw Curve2 → 原格覆盖 → DirtyTiles mask → 墙候选不再要求调用者直接提供候选 wall IDs。bucket 的完整 segment metadata、曲线 U/terrain Y 行及后续 query 消费者仍不在恢复范围，不能把本模块称为完整 WallSpatialHash。

原 `WallSpatialHash::new` (`B140aa5150`) 分配 `0x30000` 字节，即4096×48 byte 的 bucket；count=4096、dimensions=[64,64]、world-size=[130,130]（原 constant `1429a3c70`）。dirty closures 的 InnerWalls spatial 数据每 bucket stride0x30，wall-ID Vec位于+18/+20/+28，64个u64 dirty mask正好4096位。这不是其它空间 hash 参数的类比：原构造器与实际消费者共同确认它们为同一规格。cell size = 130/64 = 2.03125，world bounds ±65。

原 `spatial_hash_update` (`B140a8fd90`) 对已发布原 Curve2 每一对连续 raw points 调用 `add_segment`，替换前先移除旧 wall ID。`140a90817/81a` 传 raw segment，`140a907f8` 传原固定 radius bits `3e8f5c29` (.28)。原 mapped Curve3 nearest-segment/U/Y插值用于添加 bucket segment metadata；它不改变上述 raw segment 的 cell membership。因此本模块只建立需要的 ID 关系，未补造原 metadata。

真正的 add-segment-specialized enumerator由实际 CALL确认：A `1405d320d→141045ff0`，B `140aa5f0c→1409ef870`，C `140ac1e1c→140b3e930`。同名另一个 query specialization 不可拿来代验；`three-version-spatial-comparison.json` 保存该归属、原二进制SHA、构造器/坐标/调用指令与私有源校验和。std HashMap/HashSet/Vec/allocator/Bevy的泛型代码依旧排除。

`segment_cells` 实现完整原枚举策略：

1. 两端均须严格位于 (-65,65)；边界或外部端点整段返回空，不进行线裁切。
2. 退化零长度段只返回所在格。
3. 非退化段用原 .28 radius 扩 candidate box，floor并限制到0..63。这只是 broad phase，不能直接返回整个AABB。
4. 每一格用原行优先顺序计算中心，减原 segment 中点，投影至 segment 局部基；以私有 Rectangle2d::new/aabb策略构建 cell局部边界，与中心0、dims=[length,.56] 的 segment Aabb2 包含等号相交。使用原 float 操作顺序和再次单位化过程，不以 capsule距离、DDA或全局AABB近似替换。
5. std bucket Vec 每墙只保留一个 ID，原曲线替换/移除清掉旧 membership；dirty mask按升序bit枚举 bucket，再用std集合去重。

新增 `verify_wall_spatial_index.py` 支持默认B、指定A/C或`--all`。每版256 unclamped coordinate +256整个原 segment cells，共1536三版案例通过，包括NaN/Inf坐标、strict world edges、零长、水平/垂直、正反对角和长段。完整原 geometry helper都执行，只有CRT floorf桥与已分配Vec输出缓冲；验收逐个有序cell ID，未用caller-resolved AABB替代原计算。报告为A/B/C-spatial-index-verification.json。这里只比较 membership producer，不是原 bucket segment rows或整个建筑系统。

真实runtime已由root接入 TerrainDirtyTiles mask resource/event与 raw曲线发布，墙移动/删除维护membership；World测试覆盖旧格不再关联、正确格继续发布、删除后候选清理。原两个tabloid订阅资源的字段名和完整私有重叠/query消费者仍需后续恢复。

## 验证与连接边界

- `B-verification.json`：1280 例，1024 terrain + 256 整段 raw Curve2→resample/mapping→Curve3 构造→terrain 采集；所有位置、mapped length、U 与 terrain bits 一致。allocator/Vec capacity 和 CRT memcpy/roundf 是依赖 bridge，地形/映射/Curve 游戏核心都执行原 RNE。源 Curve2 构造已有单独验证，本报告原 raw length 的差异不参与数值比较。
- `three-version-terrain-verification.json`：A/B/C 各 512 例共1536，包含边界、NaN/Inf、两种 fix 策略（B/C）；逐位一致。A 固定 bounds 用原常量配置，不冒称 A/C 整发布链。
- `B-dirty-verification.json`：256 primary branch + 1024 完整逐点 terrain predicate，原 sampler 直接执行，零差异。secondary membership 从原指令审阅，没有宣称原 HashSet 全流程验收。
- A/B/C-spatial-index-verification.json：各512例，整个原segment格覆盖与坐标转换，逐有序cell一致。三版完整publisher及bucket metadata仍不是此验收的结论。
- Probe 与脚本是独立入口；由 root 接入 runtime 的 RawWallPublicationSource、地形 resource、原始 dirty 输入、发布缓存与屋顶链。原 Bevy scheduler/fork 图、空间 index 的完整 metadata/query 消费者、其他公开字段和 freeform 建筑消费者不是这一模块的验收范围。

所有报告须在最终 workspace 稳定后重跑以绑定当前源哈希。库身份和差分例数不能换算成完整建筑/游戏完成率。
