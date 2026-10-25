# 原始墙发布、逐拱段山墙与 Bevy 建筑链

本轮以 B 原始 RNE 指令为执行基线，A/C 只在明确登记的范围做三版比较。Ghidra 复用 B 持久数据库，只读导出；数据库的 patched EXE 不作为 instruction oracle。底层 std/Vec/Arc/hashbrown、Bevy Component/Bundle/Query、glam 与 CRT 数学调用保持依赖边界。

## 发布输入与字段纠正

`wall_collection.rs` 从原始 Curve2、shape、terrain_floor 和实际512×512 u16地形输入完成分段 resample、TerrainHeight::sample、2D→3D映射、Curve3累计U/长度以及相对地形指标。没有让调用者预先填写轮廓和采样结果，也没有凭空补闭合边。spacing=.5，矩形 tag1 为奇数分段点数；源计数和源长度保持原 Curve2。

先前字段解释正式更正：PublicWall+0x10 是原始点数，+0x30 与 +0x28c 是原 Curve2.length；不是重采样点数或墙高。原 `validate_read` 中正值 guard 相应检查曲线长度。shape tag≥2 只更新4字节tag，保留未写payload。发布后的基础缓存和山墙缓存均接真实 World 后续消费者。

B 入口与证据：发布采集 `0x140a8cb8e` 开始；resample/remap `0x14146bb70`；terrain `0x1409e9640`；两个 dirty 业务闭包 `0x140adeea0/0x140adf4b0`。Primary 的相对高度条件直接排队，Secondary 再做逐点地形差异判断，不能合并成同一过滤器。实际锁/订阅容器内部不重写。详见 ../building-collection/findings.md 和其三版比较/执行报告。

## 原始脏格候选生产

`wall_spatial_index.rs` 恢复64×64、world130、严格(-65,65)端点范围、逐原始段半径.28的格覆盖，使用段局部 Rectangle2d/AABB 原测试，不使用全局包围盒代替。三版坐标/完整段覆盖各512例，共1536例有序cell一致；只证明此membership producer。原 `WallSpatialHash` 入口 `0x140aa5150`，逐段 producer `0x140a8fd90/0x140aa5dd0`，覆盖 `0x1409ef870`。共享标准容器存储保持 std API。

runtime `TerrainDirtyTiles` 输入64个u64词，通过当前 raw Curve2 memberships解析候选ID，再执行对应subscriber规则。移动替换旧 memberships，RemoveWall清理 buckets/source。真实 World 测试确认同一脏格仅更新相交墙，移动和删除后无旧候选。这里不宣称恢复原 bucket 中用于其他消费者的完整 segment metadata/terrain映射行，或原整个scheduler。

## 完整 B regular-wall gable consumer

`wall_gable.rs` 对照 `0x14078bda3..0x14078c42d`，恢复矩形周长4个端点U、Width/Length对应两侧、檐口/profile、.28几何余量、基于原始长度的 .5*.95 U窗口、published XZ Curve2重建、每拱段 pow 曲线、半木结构基线和闭合端点额外写回。helper `Rectangle2d::get_point_u` 为 `0x140c996d0`，Curve2重建 `0x1411f9920`。每次从基础 arch heights重新计算，避免GableCache/FinalCache重复叠加；屋顶消失后恢复基础值。

320例执行完整已解析消费者并比较所有 arch 高度与 maxY；仅桥接标准分配/释放、公共墙查询解析与CRT powf，原private几何指令仍执行。包含方向0/1、profile、eave、half-timbered、闭合重复端点。三版init_wall_height_and_arch_segment_height/public_wall_state私有源路径和校验均登记于three-version-gable-source-comparison.json，校验差异确定这是待迁移游戏业务，不把共享几何误删为依赖。A/C完整逐拱段消费者尚未执行，不能拿B范围当C验收。

## Nani 与实际注册

原B创建 tuple 全部10组件已明确：7个Roof业务组件 + Desc/Transform/StoneFloor typed ZST marker。required-component图全部为空，派生self注册不是require-self边。`DrawableRank`及单tag drawable解释已撤销：AtlasPart0/tag1属于mesh enum，Desc+16的tag9属于subset kind。通过真实bevy_ecs0.16 Component/Bundle显式注册和插入。详见 ../building-nani/findings.md。

`NaniMeshAtlas` 接入Named/AtlasPart选择，缺失metadata报告错误；后端确认资源只有外部实际renderer可填写，CPU adapter不伪造ActiveInstance或InstanceHandle。Roof StoneFloor64字节CPU record、bounds、动画与更改检测接入Create/Update/Remove请求。A/B kind9、Ckind7；A record48、C updater新增PublicWalls和颜色/边距输入，禁止照搬B。

## 实际链与边界

原始输入 → 脏格候选/采集 → 发布 → 基础缓存 → Roof创建/关联/空间传播 → 逐拱段山墙 → CPU瓦片 → 动画 → Nani CPU请求 → 标记/删除 → 关联和Nani清理，运行在真实 Bevy App、Events、Commands、ApplyDeferred 上。raw-building-cycle.json 六帧连续执行包含采集、创建、编辑、重算、淡出、删除。

当前验收是B headless业务链；排程为显式恢复依赖顺序，不是已证明的原Bevy fork调度图。freeform cache、墙砖/支撑/开口、Floor Nani分支、GPU activation/upload与C迁移仍未关闭。RoofSpire/Support为原ZST marker注册，不能把marker注册说成已经生成支撑几何。既有ARM矩形CPU瓦片case14的一处Y差1ULP仍保留失败记录。
