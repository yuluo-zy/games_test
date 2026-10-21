# 树放置 producer 原证合同（待独立复核）

- 任务：TG-347c9d16cc440d611d，owner core-tree-chain-20261021；目标 task family C，当前只恢复 B 已证行为，不宣称 C 对齐。
- 原证：发行 B RNE SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03；PDB SHA256 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911。完整 procedure 范围 native-manifest.json → native/report.json，4 请求全部 ok，7 工具进程（5 analysis + 2 version），0.642 秒，327487 B 证据。PDB procedure / inline 数据来自 B 模块 3663。
- 写入范围仅当前 work_dir；未写生产，未运行编译或 GPU。共享 process_tree_cmds 的历史 claim 尚未获委派。

## 实际入口与输入

`ui_place_tree` 的点击事件仅 Vec3 位置，`place_tree` 真实系统签名经 PDB typed caller 确认：TreeStorage + EventReader<PlaceTreeCmd> + EventWriter<InsertTreeCmd> + History + ActiveSession。`random_tree_prefab` 原生参数 RCX=index, RDX=&ActiveSession；0x142269c0b..c16 从 next_index 和 ActiveSession 的 Res 解引用装载，无 LoadingCompletionState、DragState、BackupSaveFileKind 输入。

## 已证行为与修复

1. `TreeStorage::next_index` @142269b50 返回唯一存储 index；将该 index 原样作为本地 fastrand1.8.0 Rng seed（@b55..b66）。顺序为同一 RNG 的 `f32()`、`i32(0..10)`、`f32()`；中间整数分支是 PDB inline fastrand1.8.0 `Rng::i32/gen_mod_u32/gen_u32/gen_u64`，完整保留拒绝重采样。生产现在使用两个全局 f32 与独立 index 混合，必须复用 workspace 已 pinned 的 fastrand1.8.0，删除手写公共库算法。
2. 缩放按 `r1*0.9 + (1-r1)*0.5`；旋转按 `r2*360 + (1-r2)*0`。@142269aa0 是 `MOVSS memory, XMM8`，仅加载 .5 且清零高 96 位；不是把相邻地址 .49230769 当旋转下限。@142269c00..c8c packed 计算锁定第二 lane 下限 0。当前 .49220002 错误。
3. prefab index 只取 storage index 低位。@14226a17d..a1a1 与 PDB inline `ActiveSession::{is_olden,is_winter,is_autumn}` 锁定 ICF aliases：140a9fa70 = `GladeType::is_olden`；140ae8320 = `GladeType::is_winter`；14092e6c0 = `GladeType::is_autumn`。`SessionMeta::is_halloween_any` 唯一非 ICF gate，直接传 &session.meta。不得引入 readiness/corruption/drag 资源。
4. 选择表完整 7 路径：winter 且偶数→naked1_no_mushrooms；(olden 或 Halloween) 且偶数→naked1；否则 autumn→autumn/branchy[2,1]，其它→branchy[2,1]。全部字符串已用 RNE VA 直读核对。
5. InsertTreeCmd 完整 id/position/scale/rotation/variant/prefab/is_preplaced 字段保持既有共享型，不另造 TreeState；原 @142269d62..ddb 提交 Insert 事件，随后 @142269e13..e94 构造 Arc<TreeEdit::Place> 并 History::add。当前 History→Insert 顺序颠倒，恢复 Insert→History。
6. is_preplaced 原字节 @142269cc7=1，但共享字段是否经过 enum/bool 适配尚需 serializer/consumer 再核；本轮不得无证改变。

## 真实调用闭包与集成限制

- caller 1 `place_tree`：固定 false 参数应改为 session.is_olden()/meta.is_halloween_any()/is_winter()/is_autumn()。纯 helper 保持 bool 个数与顺序，仅把误判的参数名改回主题语义，避免立即破坏外部编译。
- caller 2 `process_tree_cmds` 的 missing-prefab fallback：当前 readiness.deny_load()/false/false 错误。必须同样改为同一 ActiveSession 的季节；在其 owner 委派或 owner 单独提交修复前，producer 候选只能局部集成，不能宣称整树链闭合。需继续读原 process 全 body，厘清 scale/mode/sound flags 的主题 gate。
- tests/numeric_assertions.rs 目前同一 helper 的 bool fixtures 数值行为未改变，旧注释误名需 owner 许可更新；新增 placement fixture 测试写在本 task 候选中，不改共享测试文件。
- ui_place_tree 的 raw SignifierFactory 仍有每帧 Box::leak、未唤醒 closure；原闭包完整 584 B 已取证。修复需原 `seed_cursor_icon`、UiRectangle/Assets/Builder consumer ABI，绝不通过调用现 build 空实现掩盖。另一个候选闭包处理。
- DEV_OVERRIDE_IS_DEMO_MODE=false 未接运行时全局。需 shared core 配置 owner 统一处理，不发明替代资源。

## 验收

- 原证静态复核：以上地址/指令、PDB inline owner、三个 caller resource 实参均可复查。
- root 编译并执行：原生固定 seed 的 f32 bit/variant/rotation fixtures；Bevy PlaceTreeCmd→InsertTreeCmd/History 完整字段一致；改变线程全局 RNG seed 不影响同 storage index 的放置；各主题与 Halloween 的两种 index 奇偶路径。
- process 候选另核：已有 Move→Insert→Delete 顺序、storage 置换删除、正常/缺失 prefab 两路径、water sound、delete strict alpha 与 replay，不以本 producer 完成替代全链验收。

## 独立复核后补证

- lead 已批准本 work producer 候选；其后 PDB S_PUB32 原记录解析证实140d7bd90是alloc::String::clone，不转换布尔。@142269cc7=1 → @d1b读取 → @d2c复制Insert；History构造@e54..e6e复制同一字节。当前候选把Insert/TreeEditData的is_preplaced恢复true。
- History::add错误不得静默丢弃，沿其它树录制调用使用expect；新增无录制失败且Insert已提交的测试。
