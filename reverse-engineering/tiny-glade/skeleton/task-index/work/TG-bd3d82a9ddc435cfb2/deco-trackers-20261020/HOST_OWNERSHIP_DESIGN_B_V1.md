# 实际 assembly host：现有单 owner 映射设计（只读）

目标B；不改冻结tracker候选、PublicWalls、geometry/runtime、共享SSBO或其他owner源码。root明确指示优先复用现有InstancedWall/AssetSsboLibrary链。本文件提出所有权适配，不宣称恢复原Rust字段名/结构。

## 新原证与更正

两完整游戏helper已采集在各companion合法work：Chimney `14200bcd0/2836B`，Door `14200c7f0/4668B`；原RNE/PDB身份与tracker包相同，各有 native-host-B/report.json 和 host-direct-calls-B.json。

两者均直接调用 `14128cdd0`，PDB明确 `SparseInstanceBufferWriter::push<BrickTransformSsbo>`。不能把共同clear `14128d3c0` 的单个PDB spelling `RoofTileSsbo` 当真实consumer元素类型；这是共享/ICF泛型过程身份歧义。真实构造端的写入类型将方向绑定到现 canonical BrickTransformSsbo。早期root消息中的RoofTile推断在此撤回；冻结tracker源码/设计正文没有该类型假设，未受影响。

两个withdiff host从PublicWalls stride0x2a8 bucket取不同allocation slot：Chimney在bucket起点+0x168，Door在+0x178（依据 `1412aea80` / `1412af6e0` 的反向bucket寻址计算）。二者都锁allocation chunk list，拼同型writer并clear，然后遍历所有current构造；这两个slot的原字段名和完整外层布局未恢复，不能直接按offset改现Rust对象。

构造helper逐砖push后检查AL非0并设置累计flag。共享writer原证/现API已明确原bool true=NoFreeChunks失败，现API是Result。host必须记录失败方向，不能把该flag当几何构造成功。原clear后继续逐砖push的容量边界不能无证换成保留旧全批的replace_all语义。

## 现有真实路径（source已读，未改）

- `country-core/src/resources/walls/public_walls.rs`：现有 `WallEntities(HashMap<WallId,Entity>)` 是wall→entity权威关联，不能再建一份。
- `country-core/src/systems/wall/construct_wall_bricks.rs`：原墙producer适配将已建砖 `InstancedWall::new(id,instances)` 挂现墙Entity；后续 `value.update(instances)`。`construction/bricks.rs:360` 的WallBrickInstance直接重导出canonical BrickTransformSsbo，没有第二砖载荷。
- `country-core/src/geometry/instanced_wall.rs`：同一component持有待提交base砖列表和一个私有SparseInstanceBufferAllocation。实例不Clone，destroy/drop返回实际缓冲waste queue。
- `country-core/src/geometry/runtime.rs`：同一AssetSsboLibrary注册/获取BrickTransformSsbo的SparseInstanceBuffer，统一take_out_waste并每帧try_gpu_flush。没有pending base砖也会走已注册缓冲的最终flush，可观察直接writer产生的dirty位。
- `tiny-glade/src/world_walls.rs`：绘制使用同库BrickTransformSsbo，highlight通过WallHighlight.wall_entity取得该InstancedWall.base allocation；暂时hide后获取normal，再show。把所有装饰砖无条件合并到base allocation会改变此highlight排除集合。
- 当前CPU PublicWallState明确不承载GPU handles，缺上述原两个allocation。不能为补字段建立第二BufferHost、第二SSBO库、第二flush或第二wall registry。

## 建议的单 owner、分lane适配

保留现 `WallEntities → Entity → InstancedWall<BrickTransformSsbo>` 作为唯一解析和销毁owner。若root分配该component源owner，可由同一InstancedWall持有base、chimney、door-stairs三个具名allocation lane；所有lane借同一个AssetSsboLibrary里的BrickTransformSsbo SparseInstanceBuffer。此设计增加的是原本就有的两独立分配所有权，不是第二资源host或第二实体索引。

- base现 `allocation()` 与 `update(instances)` 保持其原owner职责；两个装饰lane不写base待提交Vec。否则下一次base update/clear会覆盖装饰砖。
- 每个per-wall consumer只借自己的lane allocation，调用共享writer.clear/push；`owns_allocation`检查仍复用现库边界，不自写chunk分配/回收算法，不延长MutexGuard lifetime。
- helper只生成canonical96B BrickTransformSsbo并push。现存矩阵/BrickSourceId/StorageElement依赖由各自owner/API复用，不逆写。这里不新造raw96B替代结构。
- highlight当前排除base allocation的逻辑继续保持；装饰lane如何参与原highlight必须以原draw consumer证据决定，不能为了“单owner”合并物理allocation而隐式改变绘制分组。
- parent墙Entity销毁时三个owned handles一起Drop回同缓冲waste，统一flush前回收。tracker `on_wall_removed`只丢差异log/dirty；它不应成为第二GPU allocation owner或执行另一套回收。
- 资源借用必须落真实ResMut/Query，保持同AssetSsboLibrary tick、同CPU shadow与dirty bitset；不另建render-owned砖map、BufferHost或本地假device。更新只在dirty墙，不能每帧扫描全World来重建id→owner索引。

三个lane名字/容器组织属于新安全Rust适配提案，不声称原PublicWallState实际包含这个现代component。当前component源claim未分配，本文件不实施该变化。

## 需要在源接入前补实的边界

1. 正常初始化顺序：PublicWalls存在但WallEntities/InstancedWall缺失时，不能默认spawn空主体或把消费视为PublicWalls miss。原存在墙但allocation空走panic。需root统一真实wall producer、Commands apply、allocation初始化和assembly system顺序，使正常路径拿到真owner；异常应显式错误，不静默消费丢工作。
2. library未注册/真allocation未初始化：原取库/非空守卫不能绕过。复用真实启动/上传owner，不创建第二库来填洞。
3. `BrickSourceId::from_wall_id`当前参数为publication_fields::WallId，而当前authority/tracker用walls::WallId；现construct_wall_bricks已经用ObservedWallId(id.0)桥接。本包不再定义WallId或偷偷依赖布局转换，后续应复用现显式值转换合同/由core owner统一类型。
4. 原utils::random_splits `140c90350/332B`调用面含retVec、count及浮点/rng/stack参数；当前decorator::utils局部3参猜测实现不能因同名直接使用。utils crate当前没有找到该真实函数。此共享依赖须先有接口/原行为合同，不能在两个assembly里各复制猜测算法。
5. 真Chimney current/removal音效、Door signifier、具体construct helper参数语义尚未完全恢复。protocoladapter仍不能注册为原七参/四参完整geometry system。
6. capacity失败：原per-wall先clear再逐砖push与现base原子replace_all失败保旧是不同合同。lane adapter应明确复用哪条原路径，保留NoFreeChunks累计及真实signifier输出，不能统一成“成功/无变化”。

## 性能边界

同一GPU缓冲、同一flush、同一wall→entity映射；dirty墙才借writer并重建其lane，不复制全frame/current map，不额外GPU flush。不对未测吞吐作承诺。多lane并不会把两系统锁竞争凭空消除；原持allocation锁的作用域与library借用冲突须在真实schedule接入后衡量，不能把源码相似当作无成本。
