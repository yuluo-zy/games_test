# 编辑 → 历史 → 回放：第 11 轮增量

## 原证和两个关键误归属纠正

目标为 B 发行 RNE（SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`）。`original-ranges.json` 保存原发行与工作输入的四段原字节/哈希，均一致；本轮没有修改 Ghidra 数据库。ReVa 请求返回 No programs available，因此复用身份绑定的保留反编译，并用 LLVM 对 RNE 直接反汇编补核。脚本成功不作为业务验收。

1. Pop @140b383cc..3842c 的两个 clear **不是外部楼梯结构连接表**。同一 StairsState 指针的 current GraphMap@28、backup@b8；clear(b8)、clear(100=b8+48) 是 backup 的两个 IndexMap，随后 backed_up@178=0，再 make_backup。14157cec0 逐旧边/孤立节点检查当前 membership，把不存在的边标 dirty。保留证据：`evidence/building-integrate/B-roof-switch/140b38250.{c,asm.txt}`、`country-core-deep/collision-eco/decomp/decomp-0x14157cec0.txt`、`decomp-0x1408fefe0.txt`。原 StructuralGraphConnection 泛型标签不能作为状态所有权证据。

2. 原 prefab 同帧删除去重为 **HashSet<Entity>**。PDB `skeleton/B/crates/country-core/src/systems/prefab.rs:5520` 起的 `EventIterator<DespawnPrefabCmd> → map → HashSet<Entity>::extend` 与原 spawn 比较 cmd+10 的 qword互证。旧 PrefabSpawnRequest.global_id 只是历史适配命名，不能再认为另有独立 GlobalId。原 spawn `140840f72..f8c` 连续复制三段16B：cmd+18..48 = 完整 PlacementDataSsbo。`create-gpu-original.asm` 与保留 decomp140841b00 证明 cull.position/cull.radius 是同一 placement 的前12B和 scale@c，随后完整48B传给 GPU create_instances。

3. `create_prefab_rt` 中所谓“RT 延迟请求”已查明是 **EntityCommands::insert<BirdHaven>**：`country-core-audit/data/procs.tsv:43004` 的 callback1412dbe50明确单态化该组件。0x48字节记录保存原local Aabb center、半径、state0、clock10、flagfalse；{2, handle}则是内容向量里的RtMeshHandle，非未实现事件。已恢复真实组件写入，未新增无人消费队列。

4. `recalc_tree_position_on_terrain_height_change` @142268dc0 的原查询元素stride30，并调用 PlacementDataSsbo::position。其 owner是可变48B PrefabInstance，原候选写40B utility Transform错误。原 reader先移到当前event count，再遍历placement；现已批次消费，不逐帧重复一个事件。

## 已落地

- 原生/free/CPU Pop统一使用 StairsState::restore_replay_diff：restore_backup → apply_diff → mark_missing(backup,current) → clear backup → reset backed_up → make_backup。native路径真实支持非空diff，缺owner仍在弹桶前报错。
- 所有回放域的命令移除同步压缩timing副表（remove_command/retain_commands_mut）。原先Color/Decorator/Wall等只删commands，混合Tree桶会读取另一条时序甚至重复派发。
- 宿主新增`history_runtime.rs`，接入真实system_color replay及各set_*、core decorator/stairs、tree replay/process/despawn、FlatRoof真实roof consumer、墙色事件分流和墙音频，以及墙发布后的pending RoofAssociations重建。未同时注册core颜色直接写表路径，plaster分类、UiSound与楼梯连通传播由已有颜色owner执行。
- system-color烟囱consumer补实际smoke写入；roof consumer同时更新权威Roof颜色字段与RoofColorIds。GPU颜色上传仍为该库既有缺口，不宣称闭合。
- prefab真正ECS spawn/update/despawn消费者复用现有构造算法，完整传播placement、season、GpuWorld/RT/audio内容与Entity去重。宿主在真实设备owner就绪时运行，并通过chain应用Commands后再更新；AudioTriggerLibrary使用实际CPU空库构造，不用缺资源门控隐藏其初始化。
- prefab音频记录纠错：原记录保留part local Aabb，添加触发器时才变换；旧实现存了world Aabb，update再次缩放平移。现保存local，并验证移动后的单次变换。
- RT删除委托RtWorld::remove_mesh，保留topology dirty副作用。BirdHaven迁回core tree_storage canonical类型，critters重导出同一TypeId，构造写真实local center/radius/10秒/初态。

## 验证

`history-host-tests.log`：4通过，0失败：原发行intro13entry→同3墙；混合FlatRoof instant+Tree timed正确压缩时序、真实写Roof、解析sentinel并保留rotation/variant、次帧不重发；native非空stairs aux redo/undo丢弃临时图、重建backup并弹桶；实际snapshot空prefab→InsertTreeCmd→宿主已注册resolver。

`history-prefab-tests.log`：13通过，0失败（core native-physics；含一条真实ECS非空GPU存储+RT+音频实例spawn/move/delete+同帧删除去重）。其中GPU存储使用项目TestDevice，**不是硬件GPU像素验收**。编译过程中共享树仍有后续BirdHaven解锁编辑，最终快照须由root统一复跑。

`history-prefab-check.log`保留并发startup重构中MainCamera别名12处尚未就绪的失败日志，不将其记为本轮已通过。

## 仍未闭合

- 本轮末续修已恢复entry-owned aux和定向cull producer，详见CULL-PRODUCER.md。新two-entry生产App回归由root统一验证；保存MultiEdit原aux wire decoder仍硬拒绝未知载荷。
- 完整roof create/delete/edit与terrain/raster/UI树输入等宿主全调度、树动画完成删除、prefab热重载原流程仍需全链审查；本轮新增注册是有界路径。
- BirdHaven现共享并保存已证构造字段；critters原有wrens集合/行为投影不是本轮完整原版算法验收。
- system-color typed GPU颜色上传、原音效真实播放链与楼梯其余音频参数仍有既有缺口。
- 真实加载场景全交互/硬件GPU回读与原进程行为oracle未由本报告证明；36快照完整端到端验证未扩大执行。
