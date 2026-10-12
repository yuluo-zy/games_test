# crates/country-core/src/resources/history/mod.rs 工具恢复合同

本文件 1 个逻辑组共用一份 Rust 候选，不分别复制 A/B/C。
实际原始 binary/PDB SHA、RVA/VA、范围/字节 SHA、caller/callee 与源元数据见 mapping.json；
全批次原证据位于 /Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/abc-tools-52-2026-10-11。原始 PE/PDB CodeView 身份经只读取证确认。

## 已观察事实

逐个函数读取 A/B/C 的 PDB 独立过程范围，完整指令原字节检查，
各版函数观察字节 SHA 与原索引一致，均无指令越界。
每个实现的中文注释记录运算顺序、字段宽度、tag 条件或借用/写入范围。
原路径位于游戏私有项目，索引 ownership=private_game_scope_candidate；
本批不重写 std/Vec/Arc/Bevy/Serde 或 RNG 内部。glam 模块使用原依赖 API。

## 推断及未知

Rust API 是语义候选；完整作者结构、返回类型、trait ABI、padding 与枚举全集
不由相同短机器码证明。明确命名为 observed/resolved/view/bits 的输入是调用方
必须提供的已解析边界，不是已恢复的 producer。原始 caller ID 与物理调用地址
已记录，但尚未逐个接入它们；不可宣称此文件构成完整业务链。
输入须满足各访问器有效记录/长度/初始化条件。安全借用投影的检查只用于防止
无效宿主访问，不宣称复现原 UB。无调用的 leaf 仍保留零调用事实。

## 副作用与消费者

getter 借用/读取现有输入；builder 按值返回；LoopRange::next_back 原地推进 end；
其它具体副作用以候选中文注释为准。不得自动为 getter 增加 history、ECS 通知、
删除清理、分配或 GPU 行为；这些由真实业务 owner 负责，不在本批范围。

## 交付与待验收

状态 implemented_unverified。本轮没有编译、测试、原码执行或独立 reviewer。
后续需核对真实调用参数/ABI、绑定 producer/consumer、用原始 A/B/C 做差分，
特别检查 NaN/±0、32 位回绕、无效区间、tag 与空集合。
现有生产工程未改，旧已验收生产范围不因这些独立候选而扩大。
