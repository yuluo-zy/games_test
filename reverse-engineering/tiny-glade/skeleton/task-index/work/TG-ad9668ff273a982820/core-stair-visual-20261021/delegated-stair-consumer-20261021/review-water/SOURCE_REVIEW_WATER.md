# Tree visual262f 独立 SOURCE 审查

结论：本次限定源码审查通过，准许与真实调用者迁移成组交 root 编译和 CPU 验证；没有发现确认的本源逻辑或 Rust 借用/所有权阻断。不是 Cargo/GPU/整游戏验收。审查者 Water 未编写 visual 候选；此前消费接口对齐不代替本次对 body、完整原机器码与测试的复核。

## 冻结对象

- source `tiny-glade/crates/country-core/src/resources/stairs/visual.rs`
- before `aa5e84f63204e496991be005ae6a312a80f3ade509a3135322bdef6e96449bec`
- after `262f54401c0bf92e115e395633777c488fa3d09f5b41081872ff2086d08c8639`
- patch `92c7d198c859c959ab8215085242ec4e3ea0dd2c9522ace6e7188ef6fbf7eacf`
- V2合同 `32d634f9c11e9f57063d7853893420500e17652f9bec8db9d318f1c5da35ecf3`，精确 reset 顺序附录 `ef0df8ae52be5ff141a712fb5468ad36e73c579a009d8eb0ab3fc817a3a9e9b9`。完整记录见同目录 SOURCE_REVIEW_WATER.json。

## 原事实独立复核

独立 PE section 映射将完整主 reset2322B/remove1087B、Magic.clear1039B、两个真实Drop785/210B、dirtycell2774B、required-components110B、浮砖consumer12847B、pillarconsumer14591B共9包全部连续指令与原发行 RNE 逐字节比对；另核 floor Arc Drop15B、真实floor producer9864B和实际Nani floor consumer2430B。12段全通过，精确条数/hash见 NATIVE_BYTES_INDEPENDENT_WATER.json。本体范围字节一致只支撑下列所审字段/生命周期，不把两个大consumer的其它算法升格为本轮已恢复。

- Drop-edge 原1406056b7..6ea先两state allocations；6ef起entities Vec，70e起cell set，744起Vec4，760起wall Vec24，780起Magic两allocation与Vec64，7c1起pillar Vec44。候选显式take/drop两state allocations，再Rust字段声明析构，remove先take实体Vec避免其过早释放，符合可观察owner顺序。
- Magic Drop1405ffbf6..c12先Brick后Sparkle，再Vec64。候选Drop只take/drop两allocation，字段后续析构Vec；不存在复制allocation/额外全局Magic owner。
- reset140aa8ce5..8d01 drain先清entities.len，按原顺序排Commands；8daf/8dba清floor/wall Vec，8dc5..8e0c清cell，8e1e清pillar，8e26两个flag，8e2d最后qword。候选逐项同序。未知word_f8/flag101只有存储与清零，没有发明业务门。
- remove140aa87ae写globaldirty在整个state资源释放之后，87e0..881e才排EntityCommands；missing直返且不dirty。候选 `map.remove→take entities→drop state→dirty=true→queue` 符合，绝不把RT/physics移进visual。
- 原required-components110B只定位注册与无附加required策略，不能证明marker。真实floor producer1407dd873..8db和consumer1420803d5..3e8证明Arc@0、chunk32@8、Info.u28@0x28；候选强owner+TrimeshChunk+u32正确，旧marker推断未回流。

## 实际 Sparse 借用与四分配

current Sparse source确为依赖after `6a5c0f15be40de8b4b7f3f5e498bbec88c856aa0bd9324795a5beada9bffc419`。逐读 clear_and_lock_allocation：返回guard只借Allocation，buffer借用在返回前结束；shared clear_parts使用同原逐chunk byte_map/active/free逆序清理，保data/dirty；writer_for_locked消费同guard，不重新lock/clear/allocate。

因此 reset的state Brick guard可以跨 `magic.clear(bricks,sparkles)` 持有；Magic自身Brick与Sparkle两个guard跨缓存清理，按Sparkle→Brick逆序drop；随后state Wooden guard保持至返回。返回两writer是原held guard重组，其余四返回项只借同一state不重建容器。结构化字段借用没有制造第二个可变引用；两state槽与Magic两槽各自独立Box，复用四个真实weak-waste owner，不误把三buffer变成单泛型。Sparkle不进入返回生命周期，原锁释放点正确。

## 类型、组件与后端物权

StairsFloatingBrickParams使用当前canonical Transform48B+4f32，原Vec64、align16、progress@30/amplitude@3c均有实际测试断言。cell保原RandomState HashSet<[u8;2]>；wallholes为实际WallId+[f32;4]；Pillar复用现共享记录，没有本源新造几何/第二类型。

StairsMeshBuffer是当前Copy Buffer边界必需的RAII适配：同Arc持原Buffer与创建者device，最后Drop仅 `same device.release_buffer` 一次；不构造GPU资源/新backend registry。组件Clone只clone同Arc。该Rust包装名不是声称原版另有该结构。instance_words逐word返回地址、index/vertex各u16组合、attributes_offset、球中心/半径原bits、Info.u28，无数值转换/NaN清理；9word与当前build_stairs_floor12word消费者匹配。

现StairsPillarProposal旧wrapper的native align4共享迁移仍是显式外部依赖，未在本源擅改/duplicate；本结论不把其完整语义恢复或真实floor ECS调度接线算入已验收。

## 测试和可重现性

逐读3项测试均用真实生产Sparse/Trimesh/World/Commands/Arc实现。RecordDevice仅记录后端调用，明确不冒充GPU：一次真实3类型buffer组串行使用，写四allocation、记录实际指针/chunk/影子/dirty，再force/reset/remove检验身份、容量、waste与回收。真实World floor组件共享同Buffer，Commands apply前实体/Buffer都存活，apply后最后owner释放一次；buffers最终Drop各释放一次。返回writer继续push验证无relock死锁的真实API。另一测试直接验证64B字段布局、9word NaN/负零保留、同Arc克隆/最后release；missing路径覆盖globaldirty真假、不建state、真entity deferred despawn。

本worker未执行这3测试，实际编译/运行仍由root负责。隔离before副本正向和反向 `patch -p1 --fuzz=0 --batch` 均exact，Rustfmt2024/skip_children解析exit0。候选和生产均未改；详细命令返回码/hash见JSON。

必须原子配对：Water完整invalidate八owner/RT consuming leaf/native-physics export adapter、Duck真实reset/remove调用和fixture迁移、lead/root实际NaniFloor ChangedOR ECS consumer，及完整mesh/共享proposal必要类型。不得独推本源破foreign编译，不把本source审查当完整可见楼梯/长稳完成。
