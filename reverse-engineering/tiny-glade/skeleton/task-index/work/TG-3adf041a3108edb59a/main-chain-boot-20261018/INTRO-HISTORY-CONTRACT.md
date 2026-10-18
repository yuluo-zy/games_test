# B intro history 最小链审查与边界

目标原B RNE SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03。生产源码冻结；本目录候选不等于已接线、已编译或GPU成功。

## 原数据实查

发行 assets/starting-builds/{full,demo}/intro 仅有 history.json，无 snapshots 子目录。full intro Version16，History.chain_index=12，13个 edit_chain；snapshot记录指向3929725480290049075，但文件不存在，原 validate_and_load_closest_snapshot_data@1417a6e50会真实探测并移除无效记录后回退，不能以JSON里有snapshot记录称文件存在。

13个条目内有26个WallNewSystem并发组：14单操作组、8空组、4双操作组。实际操作CreateWall5、WallChanged8、DeleteWall2、AdjustHeight7。它们是业务fixture内容，不是逆向完成数量。当前 expand_wall_compound 的 len!=1 守卫会拒绝真实发行资产。

原 load_game@14146d030 LAB14146dfa6在ignore_snapshot或无有效snapshot时写History cursor=-1；随后扫描全history max wall id、保存加载前数据、生成后续应用/回放命令。不能跳到cursor12或直接写空snapshot后发LoadingGameComplete。

## 原WallState字段纠正

fresh visit_array@1415b23b0以及原RNE object visitor@1415bb1c0逐store一致：

| 原偏移 | 原字段 | 已证默认/兼容 |
|---|---|---|
| 00..38 | curve | 必需；curve2_serialized_as_curve3@141285770映射实际(x,z)，保留原points_u/length |
| 38..54 | is_enclosed | 必需；is_valid_enclosure别名同字段2 |
| 54 | height | 必需 |
| 58 | elevation | 默认0 |
| 5c | arch_height | 默认0x40333333=2.8 |
| 60 | plaster_peel | 默认0 |
| 64 | arch_ty | 默认tag0 |
| 65 | structure_ty | 默认tag0=Wall，非零=Fence |
| 66 | freehand_ty | 默认tag0 |
| 67 | removed_supports | 默认false |
| 68 | style | 默认tag0；has_skirts别名同字段6 |
| 69 | clutter_version | 默认0 |

当前WallState在原字节上保留数据，但将+65写作clutter_version、+69写作unknown metadata，语义名称错。source.special据+65判断实际表示Fence，不能把它解释成杂物版本。迁移时保留单一canonical owner，不能新建影子资源。

legacy deserialize_wall_style@14085ebc0原bool分支@14085ebec直接拷贝bool成tag0/1；字符串只允许Stone、StoneWithSkirts、HalfTimbered。另派生WallStyle enum visitor@140860d50允许WattleAndDaub→2，这两个入口策略不同，现Deserialize<WallStyle>仅deserialize_str不能代替原字段custom deserialize_any。不得凭现有test3strings称旧资产schema完整。

object visitor原默认门@1415bbba9..bbc0c，store@1415bbc43..bbce3；field visitor@140861520完整600B。Ghidra body尺寸错且object fresh decompile超时，合同以保留原RNE全范围为准。原PDB extent含EH清理段，不能只取Ghidra1608B当完整5253B。

## 原history enum输入合同

wall-edit-deserializer-dispatch.json来自原跳表142c5bb20，不依赖Ghidra不收敛的跳转：

| tag | 原visitor | 字段 |
|---|---|---|
| 0 WallChanged | 1415e6c50 | wall_index,operation_ty,before,after |
| 1 CreateWall | 1415ef890 | wall_index,wall_height,elevation,after,redo_instant_state,color_id |
| 2 DeleteWall | 1415dad80 | wall_index,wall_height,before,undo_instant_state,color_id |
| 3 AdjustHeight | 1415d7430 | wall_index,before,after |

旧intro省略redo_instant_state/undo_instant_state，after/before是真实WallState值。完整enum还包括其余13操作，不能用serde unknown_variant转成默认墙操作。

## 已有真实可复用消费者与仍缺环节

- canonical resources/history::History、ReplayBuffer、HistoryEntry和EditType::Wall均存在真实状态逻辑；不是都要重写。
- systems/history/replay_wall::replay_wall将同World InnerWalls逐命令投影并立即写回，真实发送AdjustWallHeightCmd/颜色/声音事件，不能绕开到第二份持续HashMap。
- 原WallReplayCmd组构造closure140abf660按组长分配24B命令，1414769d0逐Arc遍历；空组分配分支也明确，不存在len==1守卫。原replay_wall140951b80对整组执行process、以finished做AND、保留相邻组WallId交集门。删除当前len!=1守卫可作为有原证据的候选，但 timed/undo策略必须另核；不能flatten组。
- 当前WallCompoundHistoryEdit/WallAuthoritySnapshot没有原JSON Deserialize；从值转换的适配尚缺。
- 当前StateChange/Create/Delete/AdjustHeight域消费者可用，但旧序列化source字段必须完整进入owned WallState，不能通过120B heap指针record导入。
- 当前load_game仍在JSON成功后显式失败，缺原HistoryControl请求、loading signed-counter事件producer、回放结束→postcommands→ready链。错误比假ready可审计，但不是最终业务完成。
- game App需要原HistoryPlugin/墙发布/height consumers真实注册及其资源producer。仅注册replay_wall而无terrain高度、颜色/墙发布消费者不能称渲染世界完成。

下一步应先恢复canonical WallState原serde和4个intro操作输入，然后证明/接回原loadGame→History请求→ReplayBuffer→同World InnerWalls→墙发布。这条链通过后，再处理有实际snapshot的续玩域，不以空snapshot绕过。


## 后续已取证的state1 producer

同目录load-counter-producer.json及原RNE锚点证明：load_game正delta逐条写HistoryCmd payload0x03010101@14146d8a2，负delta写0x03010001@14146d952。两者bytes分别[1,1,1,3]/[1,0,1,3]；Events stride16中id在+0、payload在+8。Original SystemParam initialize141749330具名确认Events<history_process::HistoryCmd>，不能相信grow_one ICF误标WindowFocusEvent。

对应load-counter-producer.rs为真实有界候选：只接已经实际应用World/History之后的ApplyCounter，不发ready。当前生产HistoryCommand四字节可作为语义输运，但process_history_control_stream的16B记录建模和flag3消费者仍需整合；它注释说flag3不发processed，现实现却无条件发，已交独立审核。普通Undo/Redo事件与加载控制tag不能在未经原签名核对时混身份。

## 独立复核：LoadingGameComplete不是unit事件

intro_contract_review独立读取public-symbols完整系统参数：mode_manager@1405e0200的reader是save::load_game::LoadingGameComplete；loadGame FunctionSystem@1408845f0的尾tuple为ModeJustChanged reader、HistoryCmd writer、SpawnPreplacedTrees writer、LoadingGameComplete writer、DisplayCorruptedSaveFileWarning writer。不存在RequestAppModeChange writer。

fresh loadGame L279..297对应local_f0=in_stack68[0xd]=LoadingGameComplete；state1临时改local_f0=lVar36+0x20不改变state2初始owner。state2 L440..482的SwitchAppMode分支写stride24，两个qword AppMode载荷16B、event_id@+16。它是同一LoadingGameComplete{restore_to:AppMode}。当前save owner定义unit、mode_manager另定义struct，是实质ECS事件身份断线。restore-loading-complete-identity.patch统一save canonical owner并mode_manager reexport；两个调用者保持同一事件身份。候选未应用，source hashes保留。

通知时机也需纠正：state1排入真实history控制命令，下一state2处理postcommands时发送此事件，mode_manager随后切Replay。async worker只检查非空并开始原prewarm，不意味着13条墙回放已耗尽。不能按常识加“等全部回放结束才ready”条件；后续finish_replay/load_completion_checker控制真实游戏阶段。

独立审核进一步限制候选接受面：当前ShapeParameters的tag/payload JSON不是原Premade存档schema的完整证明，wall-save-serde因此对PremadeShape显式报缺口，未复用该类型后伪称兼容。实际23个intro WallState全为No/Wall/FollowTerrain，候选只覆盖其有原依据的输入面。

HistoryCommand现四字节能保留全部原payload，producer无需伪造16B EventInstance头。现直接enqueue_history_commands会绕过第二stream的反方向清队/processed策略；process_cursor只看bytes1/2，第四byte3尚未完整消费。原history_process@140af5e5d在flag3时跳过processed事件，当前process_history_control_stream无条件发送是偏差。不能仅因为intro全正向instant命令能被cursor核心读取就称完整HistoryCmd协议恢复。

后续RNE反证（2026-10-18，新证据优先）：旧“第四字节必须传播桶”推断过强。原history_process将byte3放R8d bit16传replay_edit，后者原封交generator；但完整generator10371B仅读DIL低字节，无额外EDI/DI消费，高位不用持久化到ReplayBucket。byte3已确认行为是入口140af5e5d/68/6c的flag3跳过processed。完整asm在TG-c4cdc0aa5e62db9266/main-chain-boot-20261018。故独立review相应传播断言标记被此原始指令审查收窄，继续保留四字节输运但不创造额外桶字段。
