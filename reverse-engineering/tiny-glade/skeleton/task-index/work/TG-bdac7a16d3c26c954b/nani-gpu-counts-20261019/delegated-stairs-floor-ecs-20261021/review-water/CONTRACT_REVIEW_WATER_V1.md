# Nani楼梯 floor ECS 合同独立准入审查

结论：合同V1通过，可按限定same-source增量实现和随后独立SOURCE审查。审查者Water不是本合同/预期ECSleaf作者。这里没有源码候选/生产/Cargo/GPU操作，不把合同准入当实现或真实帧完成。

## 冻结与依据

合同 SHA b5979da0031587db9a4d9572c560966c9e1575bfdebcd5690e63c7f990eedfea；系统增量必须基已审Gothic Nani afteraf8faaba3d5d373136f2e33af1668d87360500f16a973485a6d5b25473efced0，组件复用Tree唯一262f54401c0bf92e115e395633777c488fa3d09f5b41081872ff2086d08c8639，不覆盖旧砖叶或复制floor组件。

复用本人此前visual SOURCE独审的完整14207fcc0/2430B连续原字节证明，本次重核asm文件SHA未漂移。原B RNE f507a6ce/PDB2d9686b8，PDB3098:240364；exactQuery记录绑定原B skeleton16429类型。native完整637条：ff21 MOV DL1a→ff23 owned_subset_mut，之后才query迭代；803d5/+8/+18三次copy构成9word，3eb/3f3末3word零，3fa/401 position零，xmm6=1传sort，41d update，完整循环后436/43d才解锁。无Transform读取。容器/query/变更tick内核复用pin依赖，不抄其实现。

## ECS与数字载荷

原 Query 是同canonical active key+StairsFloorMeshChunk，With<Marker<StairsFloorInstanceData>> AND Or<(Changed<Chunk>,Changed<Key>)>。现InstanceKey是唯一active组件身份；&mut仅为现dense-index缓存hint修复，不可据此增加Changed门，也不能让bypass之外的解引用制造Changed<Key>。计划的两个required数据项和原ChangedOR filter正确，不要求Transform/desc在update query出现，不建rows Vec或第二key表。

same Owned subset26在空query也先锁一次，保缺subset/external失败先后；每行复用build_stairs_floor(chunk.instance_words())→same write_mapped_update，payload12u32=原9word+3zeros，posZERO/sort1。Tree instance_words已本人源码独审，地址高低、两个u16合字、原floatbits和Info.u28均保持，不加finite/NaN处理。

## guard、调度与真实owner

现startup/startup_nani.rs168明确 StairsFloor=>owned!(StairsFloorInstanceData)，gpu_world enum值26、instance_data12word48B已存在，不增加registry。maintain guard仅增第15marker和真Has<FloorChunk>，保单marker和现descriptor+Transform创建/存续域；这属于同宿主前置策略，不能转嫁为update的额外Transform/descriptor门。数据项和required marker不全仍在publish前失败，不fallback到全零实例。

核本地精确bevy_ecs=0.16.0：QueryData tuple实现0..15，Or/filter tuple0..15，schedule tuple1..20。拟marker15、outerInputs12、updates15均在现pin支持域；同update tuple保无序并沿maintain→ApplyDeferred→updates→flush既有链，不新增run_if/错误串行依赖。hint必须使用bypass_change_detection明确写，保原只读active Query的Changed事实。

## 测试准入与剩余硬门

计划6项实际World/Owned subset/MeshAtlas/Tree floor组件测试域合理：真实NaniBundle建key；9word/zeros/pos/sort；clear_trackers后无变化不写data，Chunk/Key各Changed触发；dense repair不自造Changed；去Transform/desc后单独运行此update仍处理真实Changedchunk（不能同时跑maintain并假称其guard放行）；空queryOwned26先锁；missing/ambiguous前置失败；sameWorld despawn实例撤销。不要用rowVec模拟ECS，也不要用空floor marker替代真实Arc组件。

floor Arc最后Drop由Tree实际buffer测试覆盖；此CPUleaf不能证明GPU在途安全。root必须独立把同Buffer的代际句柄解析到ManagedBuffer并纳本帧资源lease，不能只BDA copy就说已可见。此合同不授权提前mounthost、替未完整genfloors/两62KBmain/Info63默认化，也不宣称真实GPU/完整楼梯画面通过。

准入无确认阻断；请源作者按该合同产唯一增量、实际6tests、F0/parse后交独立SOURCE审查，root唯一Cargo/GPU整合。
