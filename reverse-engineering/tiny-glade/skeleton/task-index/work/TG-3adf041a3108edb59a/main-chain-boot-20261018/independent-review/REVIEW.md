# B intro 合同独立审查（2026-10-18）

Reviewer `/root/boot_chain/intro_contract_review`。仅静态读证据/当前源码；未改生产源码、未运行 Cargo/场景/GPU，也未运行原进程。目标仅 B RNE SHA `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；保留的 Ghidra decomp 来自 patched B，不扩大为原 RNE 行为 oracle。输入指纹见 `input-hashes.json`。

## 结论

- `wall-save-serde.rs`: **passed_scoped**，限原 full/demo intro 实际字段形状（No、Wall、FollowTerrain、has_skirts=false）及保留原证据支持的缺省/偏移。候选未集成、未编译、未运行；不是完整 WallState 兼容层验收。
- 最小启动链：**changes_required**。真实 owner/核心消费者可复用，JSON→typed History、真实原事件入口与 App 接线仍未完成。不得以空 snapshot、原生 Vec 指针伪记录、第二套持久墙表制造完成。
- `LoadingGameComplete` 两份当前 Rust 类型：**blocker**，原来是一种带 16B AppMode 载荷的 `save::load_game::LoadingGameComplete`；save 侧 unit schema 不正确，mode_manager 侧另起名义类型会断线。

## WallState 候选核对

1. `0x1415b23b0` sequence visitor 独立显示字段顺序 curve,height,is_enclosed,elevation,arch_height,arch_ty,style,structure_ty,clutter_version,freehand_ty,plaster_peel,removed_supports，与候选 derive 声明顺序一致。前三字段必需；后续默认 0 / arch_height `0x40333333` 与原 object gates `0x1415bbba9..0x1415bbc0c` 及 stores `0x1415bbc43..0x1415bbce3` 相符。
2. 原 field visitor `0x140861520`：is_valid_enclosure→字段2，has_skirts→字段6；候选 alias 放置正确。原 legacy field helper `0x14085ebc0` 的 bool 和三字符串输入与候选匹配；现公开 WallStyle 的四字符串 derive 入口不是同一策略。
3. 曲线 `0x141285770` 由 Curve<Vec3> 取 x,z 并保留 points_u/length，候选直接 owned Vec 映射正确，不应重算 curve。intro 23个 before/after wall states 的实际输入仅 No/Wall/FollowTerrain/legacy false，足够支持该有界输入域。
4. Canonical `WallState::from_snapshot` 把 source+65（现误名 clutter_version）作为单一原字节，并派生 special；meta+66/+67/+69 和 +60 f32 bits 均存回同一 snapshot。候选把 structure_ty 写+65、clutter_version写meta+69，未见业务字节丢失。当前命名/注释须协调纠正，不能按旧名字又新建 owner。
5. 未验收兼容边界：`IsEnclosed::PremadeShape` 当前依赖 `ShapeParameters {tag,payload}` 安全适配格式，原 `0x1416ebaf0` 明显有多格式兼容尝试；不得把当前格式说成原 JSON schema。ConstantMaxHeight/Fence 等非 intro 分支在本次未独立重查其全部字符串入口；不以候选测试扩充证明范围。

## HistoryCmd 与加载完成事件

权威原 PDB `evidence/render-architecture/public-symbols.json`：

- `va=0x1408845f0` 的 load_game FunctionSystem 尾 tuple 顺序：Reader<ModeJustChanged>, Writer<HistoryCmd>, Writer<SpawnPreplacedTrees>, Writer<save::load_game::LoadingGameComplete>, Writer<DisplayCorruptedSaveFileWarning>。
- `va=0x1405e0200` 的 mode_manager FunctionSystem 明确 Reader<save::load_game::LoadingGameComplete>。不存在另一个 mode_manager::LoadingGameComplete 原身份。
- load SystemParam initialize `0x141749330` 注册上述同型（本 work `0x141749330.c:122..135`）。

`TG-c459c7f73345e24709/main-chain-boot-20261018/0x14146d030-fresh.c:279..297` 由上述 tuple 可定：`lVar36=in_stack_68[5]` 是 HistoryCmd Events；`local_f0=in_stack_68[0xd]` 是 LoadingGameComplete Events。state1 `:379..426` 向 HistoryCmd 写16B EventInstance（4B payload@+8），正 `0x03010101`，负 `0x03010001`，随后 state=2并return。每次调用初始化 local_f0，state1局部改写不影响下一次state2。

state2 `:440..482` 的 SwitchAppMode payload 分支向 `local_f0+0x20` Events buffer写 stride0x18，payload qwords=postcmd[1]/[2]，event ID@+0x10。故真实生产的是 LoadingGameComplete(AppMode)；原 load_game 签名没有 Writer<RequestAppModeChange>。不能把“加载通知已发”解释为全部 History replay 已结束；finish_replay `0x1408b36a0` 与 load_completion_checker 才另有结束策略。

`HistoryCommand {bytes:[u8;4]}` 能无损输运原 load state1 payload，无须造16B header。原 history_process `0x140af5b30` 的 EventReader<HistoryCmd> 双 buffer 本来就是16B EventInstance；PendingHistoryCommands 是其后的4B持久队列，当前所谓第二流原始 record 不能与 HistoryCmd 当作互不相关业务事件。

当前 `enqueue_history_commands` 直入队绕过原“方向相反清队/即时化/processed”入口策略；`process_history_cursor_commands` 只消费 direction byte1、instant byte2，byte3不继续传入命令展开。原 `0x140af5f81..0x140af6134` 会把 byte2 bit0 与 byte3 一起传至 generate_replay_cmds_from_edit（decomp `CONCAT12(uVar2,uVar3)>>8 & 0xffff01`）。这不是完整 HistoryCmd 合同。另当前 `process_history_control_stream` 无条件写 processed，违反原 `0x140af5e5d` 的 byte3==3 不发结果事件分支；不得将 load flag3 喂该现实现后称语义保真。全正 instant intro 子集可复用 cursor/generator核心，但 flag3需保留到真实入口/后续消费者。

## 最小真实调用链与缺口

原 intro 无有效 snapshots：`validate_and_load_closest_snapshot_data@0x1417a6e50` 探测失败删记录，`load_game@0x14146d030` fallback cursor=-1。下一步依次需要：

1. 原 history.json 的 typed 反序列化/拥有值转换，复用 canonical `History`, `HistoryEntry`, `EditType::Wall`, `WallCompoundHistoryEdit`；四操作分别映射真实 StateChange/Create/Delete/AdjustHeight。不要走 `parse_wall_section`/`from_record_bytes` 的只留 id/height/空曲线面。
2. 原 state0 应用History并计算真实 signed delta；再复用候选 emit_load_history_commands 只负责state1。当前 source `try_load_game_system` parse成功仍明确报业务未恢复，尚不构成已集成链。
3. 保留原并发组。当前 `expand_wall_compound` 的 `g.len()!=1` 守卫拒绝发行空组/双操作组；原 `0x140abf660`、`0x1414769d0` 按组长构造且有空组分支。删守卫仅为有据候选，不可 flatten 或声称所有 timed/undo默认均已通过。
4. 可复用 `HistoryTransactions::cursor_step_expanding`→`ReplayBucket::expand_with_aux`→`replay_wall`→`advance_wall_groups_observed`→同World `InnerWalls`；后者逐命令立即提交通知，临时HashMap不是第二份持久 owner。Create/Delete/StateChange即时 authority 与 AdjustHeight request核心已存在。
5. App必须注册/提供真实 GameTime、WallPublicationTerrain、History/ReplayBuffer、InnerWalls、height/颜色/墙发布消费者及各自产生者。`WallHistoryRestoreColor` 只是效果事件；仅发事件不等于颜色/发布世界完成。真实 `finish_replay`/mode restore/load completion 接线与原调度仍需分别核实。

本审核不提高任意完整游戏、原进程或 GPU 验收状态。
