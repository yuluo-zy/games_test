# M01 slot14：脏标记传播 → 结构碰撞与初始图

2026-10-27。只恢复slot14第4/5叶及其必要真实订阅、瓦片身份和结构registry交接；不改宿主注册，不把后续assemble/calculate_distortion等未恢复阶段计作完成。

## 原版归属与身份

原B文件为 `crates/systems/decorator/src/decorator_visual/stairs_assemble.rs`，源校验值 `5547521ff43efe863fba348656e8c55203e556ccfd647d82ee794436eead86cd`。实际实现仍在同一现有文件；瓦片身份位于原B `country-core/src/lib.rs`，视觉owner位于原B `resources/stairs/visual.rs`，没有新增业务目录或重复框架。

RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；PDB SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。[逐函数代码摘要](m01-stair-dirty-structural-original-identities.json)。

| 原入口 | VA | PDB物理跨度 | Ghidra正常body |
| --- | --- | ---: | ---: |
| propagate_dirty_flags | 1407d16b0 | 2243B | 1884B |
| create_structural_colliders_and_initial_graph | 1407d20c0 | 10442B | 6910B |
| stairs_subscribe_lazy | 1407cf210 | 750B | 423B |
| LazyTile::from_index | 140833250 | 3B | 3B |
| From<UVec2> for LazyTile | 140aac630 | 13B | 13B |

物理跨度含EH；完整原ASM保存在 `m01-stair-4-1407d16b0-full.asm`、`m01-stair-5-1407d20c0-full.asm`、`m01-stair-lazy-init-1407cf210-full.asm`。共享Ghidra只读用于交叉解释，真实SystemParam从B骨架绑定的FunctionSystem取得，不能把void()调试签名当成无参函数。

## 真参数与初始化

`stairs_subscribe_lazy` 的原4参是 RaycastWorld、TerrainHeightsData<TerrainHeightSsbo>、TerrainHeightsData<VisualTerrainHeightSsbo>、Commands。原 `1407cf25f..263` 明确写字节数组 `[0,1,9]`，随后依次订阅两路高度Publisher；旧注释“3/0x90000”未表达实际参数，不能据此猜槽。新的StairsLazySubscriber持有5个真实tabloid Subscriber，退出由Vec/Arc owner正常释放；没有默认空listener。

propagate为原4组参数：Subscriber；(Visual,Requests)；UiSignifierStream；(DecoratorOpEvent reader, RecalculateWallHoles writer, PublicWalls)。structure为原7资源：RaycastWorld、StairColliderRegistry、StairsStructuralGraph、Requests、RemovedSupports、StairsState、UiSignifierStream。旧1参/3参空投影已移除。

中央接线前提：上述真实发布者和事件资源已存在，在首次propagate前执行完整lazy订阅。这里记录数据依赖要求，未伪称已核定并修改生产Startup时序；本轮没有修改宿主。

## 脏传播的完整业务

先将本轮请求的identity→SmallVec段记录clone进previous.sub_segments，保留其它旧identity，重复key按真实HashMap.extend覆盖。不能清整张历史表，也不能让历史借用本轮可变段。

随后消费同一个DecoratorOpEvent TypeId。只有type字节+0x81为63时处理墙洞。原 `1407d1af5..1b1a` 先按完整u64判别选择当前地址：disc3/4位于+8，其余位于+0；不能固定读+8。当前地址try_as_wall成功且PublicWalls存在、+284受约束高度有序 `<=3.0` 时写RecalculateWallHoles。另一个独立分支在低32位disc<3且旧地址+16为Wall时，用+24墙ID做同样判断。两分支不去重；同墙会收到两条事件，NaN高度不通过。保留原EventReader游标，已读批次不能下一帧再发。

最后take五路真实Publisher批次，按DirtyTiles位图OR合并。原按每个脏tile遍历已有visual，依赖键命中才置force_rebuild_requested；不新增缺失visual，不清其分配，不置全局dirty，不改请求dirty。空发布不扫描视觉map或触发其ChangedTick。

瓦片身份纠错：LazyTile是**u32线性index**。原from_index只返回ECX；From<UVec2>按32位环绕计算 `x+(y<<6)`。原视觉consumer在 `1412946a2..6ae` 再拆x=index&63、y=index>>6，各截u8组成**tuple(u8,u8)**集合键。已按此恢复，未把LazyTile误定义成两字节或把视觉集合误建为HashSet<LazyTile>。`affected_tiles`是真owner字段，后续assemble仍需真正生产它。

## 结构碰撞与图初始化

只处理dirty且interpolated_curve存在的请求。先断言widths长度等于terrain_contacts位数；原+60是widths长度，不能误读成heights。每个subsegment按累计count形成闭区间，按positions→tangents→widths→BitSlice的原先后检查，不能zip截短吞掉非法请求。

kind3梯段严格要求两个点，调用已恢复的完整梯端墙锚/邻接调整。其余kind保留所有相邻采样窗口。只有边记录为移除支撑且窗口两个端点contact均false时才生成source tag16和结构高度1.0；其它情况生成tag15与正零。source有效字段是sample索引+4、完整边+8，不是装饰物/墙单ID。

生产复用已有 `upsert_structural_stair_segment`，补回原外变换 `(0,0.3,0)`；保留原8顶点凸包算法及凸包失败的tiny-sphere路径。真实upsert返回Inserted/Replaced/Kept句柄均交给同一StairsStructuralGraph.upsert_span；原替换节点/邻居通知和双向映射由现有原helper执行，没有第二份碰撞或图镜像。

曲线采样数-1的计数非零才写segment_counts，并按原全索引范围连接相邻span；不存在的末尾span由原add_edge_by_span跳过。独立第二阶段遍历整个当前StairsState图，各节点邻边两两连接：规范化边低端为当前节点时用sample0，高端时要求segment_count存在并checked_sub(1)，缺计数跳过、零计数失败。两类连接都为永久false，不能只连接本轮dirty请求或默认空support图。

静止帧保留原第二阶段的图遍历，未虚构“空请求就完全return”的优化。未出现dirty有效请求时不重建碰撞；存在span与句柄正常复用。删除/失效职责仍遵守上游invalidate的A/B分离规则，不通过清整个World/结构图掩盖后续阶段缺口。

## 原指令差分与真实World

| 原侧范围 | 输入 | 桥边界与真实执行 |
| --- | ---: | --- |
| 1407d20c0完整入口至自然ret | 139，含18原失败、12墙锚升降等高 | 执行原主分支、梯端内联、gen_stairs_structural_collider与swizzler/SIMD；只桥Parry凸包构造、存储/物理写入、显式空输入图getter。248次几何参数/顶点记录 |
| 1407d32d1..3755完整junction业务区间 | 16，含2原失败 | 显式非空原图/邻接/计数，桥IndexMap/RawVec依赖与图存储；原端点选择、lookup、缺失/零计数和永久连接顺序实际执行 |
| 1407d16b0完整入口至自然ret | 420 | 显式空请求/订阅输入；桥空fold、零位图memset、PublicWalls存储读取；原事件变体选择、比较、reader消费与writer内存写出实际执行 |
| LazyTile原两入口+原视觉截断指令 | 144 | 无候选业务作oracle，覆盖正常/溢出/越界数值和u8截断 |

所有脚本先核RNE SHA并按PE sections展开，输出用exclusive-create防覆盖。v1结构探针桥gen，v2改为执行原gen，v3再补12个梯端墙锚案例；旧输出保留，不把不同验证边界混为一份完成声明。最初探针错误把128B结构返回面写144B污染栈，已纠正为128B；这是探针错误，不当作业务缺陷。默认沙箱中本地venv Python退出132，精确有界命令获准后执行，未改原二进制。

最终 `CARGO_INCREMENTAL=0 cargo test -p system-decorator --offline --lib dirty_structural_tests -- --test-threads=1` **10/10通过**，日志 `m01-stair-dirty-structural-tests-final-ladder.log`，0.10s。139个结构样本中基础127和新增12分别对照实际World：真实mesh.hash对原24顶点字位、source有效字段、真实Rapier句柄、span映射和高度；16原junction与144原tile也经过现有owner。420原事件通过真实PublicWalls和同一Bevy EventReader/Writer，另验同墙双通知和下一帧不重发。

其它真实链测试覆盖5个原Publisher各自实际发布→同一视觉consumer（碰撞通过upsert+flush，两高度通过copy_from_gpu）、未命中不误标、稀疏实例owner未被误清、空发布ChangedTick不变、退出重新订阅不接收历史批次、旧段clone隔离、碰撞句柄跨帧复用、原0.3位移和永久junction。使用既有真实请求生成器→插值→invalidate→propagate→structure的联合World已生成非空collider/graph，没有用空图或手填装配输出冒充上游完成。

既有 `cargo test -p system-decorator --offline --test stairs_structural_collider -- --test-threads=1` **4/4通过**，包括原8顶点/fallback逐字比较、实际凸包/球和distinct registry disposal，日志 `m01-stair-dirty-structural-existing-geometry-tests.log`。对整个stairs_assemble相邻回归另记最终验证清单，不将并行owner的编译中间态等同本业务失败。

## 未跨越的边界与可复用经验

本轮不表示18叶组或整游戏渲染完成。后续assemble必须从真实几何产出affected_tiles与完整visual payload，现默认空集合不能算生产懒重建生效。calculate_distortion/订阅及后续支撑、视觉装配/卸载仍按原顺序继续，不在本轮凭空清表补副作用。没有OS/GPU验收，也没有更新STATUS或提交Git。

经验：按完整SystemParam替换空投影；先查事件变体真实地址再解释字段；不要从常量打包字面值猜发布槽；同址ICF方法仍须原调用处验证真实宽度；瓦片index与集合坐标是不同表达；图建立必须验非空旧状态和失败先后；几何helper的正确性不包含caller外变换，需沿实际Rapier位置再验一层。
