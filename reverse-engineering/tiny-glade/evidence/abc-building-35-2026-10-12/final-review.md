# 35组冻结候选独立最终审查

Actor：contract_review_35；目标 C，A/B 参照。结论：`changes_required`（一处依赖现状文档措辞）；该文档修正并重冻结后，候选源语义与下述观察范围可记 `passed_scoped`。完整作者ABI、依赖兼容及实际消费者仍为 `needs_evidence`。任何结论均不升级 verified。

## 审查输入与实际读取

读取35组mapping对应的21份候选源码全文、assembly/lib.rs全文及10份上批被引用模块全文，关键任务CONTRACT/obligations；读取contracts-observations全35组ABC指令，并逐个读取对应105份pack原JSON，对比指令与body SHA；核读support-pack-A/B/C闭合墙洞过滤和C key caller；参考已有roof_components的两float动画表示。读取21份delivery manifest并核对其129个列举文件SHA；读取dependency-closure列举的273个文件并核对SHA。此处哈希检查是只读冻结一致性审查，不是游戏测试或执行。

35个mapping源码SHA、marker行与原符号一致；105个body指纹/指令与aggregate一致；21个manifest SHA全部一致；129个交付文件及273个closure文件均未发现指纹变动。assembly静态path包含31模块（21本批、10上批），全部在closure中；未发现未列入闭包的其他本地模块引入。本批没有Cargo.toml，不声明构建/API或版本features兼容已检查。

当前输入文件SHA：
- `mapping.json` `682c663cf1eb7da3ba14a6cce933fd1e4bee9c6dfffad11306918fd2096285ce`
- `deliveries.json` `851e75cf2d0d6a106418c479fb3463b7c41754f7b2aa8a62dc2921160a732f4c`
- `dependency-closure.json` `34f703ac74cae3a056847d6c04184658e66b7d9b35e892e7de4867dd4bd30ef1`
- `contracts-observations.json` `23948038d70af15afc6a0388fe0373b7d2d36ee6787eb903f5928b808a89e8e1`
- `contract-review.md` `4353ed2c59f68cead46feda2f60115824f1111e610470f94e6c3b4b8cd381fe3`

21份实际manifest SHA：

| Task | Manifest SHA256 |
|---|---|
| TG-fe6e3b5ee72432c3d7 | `37261732543e1f834fbef00d5e99a5a09c53e8863ff81f18a7ecb891c974948e` |
| TG-43142338addb95ca2d | `010d9200211805e6236031fd70facd73bf5b825857d6cdb621529c3d89c7cc44` |
| TG-8516251c5a664dec0c | `3229c3dd727142ce6ae59a0b7a302ec54da96d74d755976e1d25b426d25ae1e0` |
| TG-56afc78f73b7ab7966 | `ef10e48cb51eb98421d774397ed209ffcfe9a69053fd11f89a22616c68578892` |
| TG-3b800c682fa41d3683 | `5242f38528fd51ace2d4081b04473c93c2eb3b89157767bce0879aead53f11a0` |
| TG-774ee58bc153366d1d | `ea5d51d55771891e8b04a15d2abd7e6c0faa1c5432bce9a069b011de001dab82` |
| TG-c4cdc0aa5e62db9266 | `f00e6c789e1430cb5839dc64bcd0f68db5849d03252ce725482563e96477c196` |
| TG-3846e898cf43c29d09 | `35794baf7c483dc4ff42f2f0f3cdf805ba7eb08414ca7d6b9fcf620236ef2050` |
| TG-d63bc50864713c7344 | `cbe5b2db54c32c52a17c7f0773b1456be9d5b0a8d6e2b249cb8eeb2b54f3ebf6` |
| TG-268a3bb2f466c1c2b3 | `2e2e5e828ea19b29f8f48d595e425467ad7b13807a0e57e19794f518cf295668` |
| TG-1d8b7844066af51d6b | `3f68df6c44e60054ee06d9107544aea9cf57d45929cccfdf86ba9d7f7d1be590` |
| TG-60901c6c8508b1af76 | `a0e799dac9e662908d1eecd5c089295ee95bcaaeaa326955061fcd78612e128e` |
| TG-a5b23ebeaacbd24221 | `764a4398235a14c02bafca67403dd8f42a141459b1f8bc6ad6624834d5faf73e` |
| TG-3c7e1129dc1230a273 | `45916b5c6e94582029e5540a8f29ba969a6a65df17e848f98376c51821770de4` |
| TG-01726d288b6ab83595 | `b5141957a6e5e0301509898b3598106a19f640382e42dcf45e59ba203d1e6137` |
| TG-6e76d82e8a953a9a45 | `21ea78d60b442cccec0ac6c72e4d2c8a21175a883b4f9665c5e4dc5af1e99b53` |
| TG-f2c3841f004fb1ad05 | `8d7a3c74f99b632e1798a551e4e4b9c347ac3cc9d4420513bf950caa613b2538` |
| TG-ee62d4bfcf31848c9b | `1c09a454ea6a359c984dc5e4024d31f2f9fdc6a4f6fb4aa06a958fc189cd86d2` |
| TG-4509462cb49323f18d | `a240c2accf7c9b15de47891e199e93a55250d00c2176218d3051e5dd9ed62a3f` |
| TG-95525ca2902fab432c | `a76049907fb559657a3429652ee367f1a34445983e9b014dffc6008bbbf9e0b5` |
| TG-71ec411b5d563a05bc | `0a3b78e9aac280b6b3cca0b3a265b1c934c00aa5193b5f572fa79cf9fef5bca7` |

## Findings

### F01 minor：assembly声明依赖已绑定，与真实冻结范围不一致

位置：`skeleton/task-index/work/TG-1d8b7844066af51d6b/abc-building-35/assembly/lib.rs:1`。

原文“glam依赖使用目标C原锁版本”，但没有Cargo manifest，dependency-closure.external_crates明确说明依赖版本/features必须未来由整合绑定；合同也记录未接生产Cargo。该句将待履行义务写成现状，会使后续消费者误以为锁兼容已验收。应改为“glam依赖须由整合绑定目标C原锁版本”，重冻结assembly所属delivery与closure、deliveries聚合清单。无运行反例；这是版本来源声明的文档一致性问题。

未发现需要修改候选函数算法的已证实错误。F01修复前不签发最终无条件passed_scoped。

## 各项实际对应审查

| 范围 | 读取事实与Rust对应 | 本轮结论 |
|---|---|---|
| lerp/perimeter | 分别保留SUB/MUL/ADD、先sum再sum+sum；未换权重和/FMA | 源语义对应 |
| Aabb width/height/contains | max-min；按x下/上、y下/上四项ordered >=，边界true、NaN false | 源语义对应 |
| Circle contains_point | y²+x²，radius² strict >；未加负半径guard，未委托sqrt版is_point_inside | 源语义对应 |
| RNG u32入口 | seed零扩展，委托冻结base_rng中的wrapping_mul/wrapping_add/xor一份算法，无with_seed二次变换 | 源语义对应 |
| 动态网格 | signed32尺寸转f32除法，零/负分母原样；index y*width+x均wrapping后as i64 | 源语义对应 |
| LoopRange len | i32 wrapping_sub后符号扩展，未unsigned/abs | 源语义对应 |
| Rectangle3d | 水平old+delta、垂直delta+old两端；extent width/max-min/depth；orientation直接写无normalize | 源语义对应，实际repr不是本轮ABI验收 |
| Triangle/xvy | Vec3而非Vec3A；三点顺序保存，上批泛型points_mut复用同一type | 语义对应，锁/API与12B宿主布局待绑定 |
| holes_no_duplicates / holes | 前者filter origin==0，来自ABC closed消费者record+17谓词；后者不过滤，全序不变 | 限定消费者观察对应；不能声称HashSet去重或完整padding恢复 |
| Key/Rank/ID | key返回槽[输入edx,输入ecx]保留u32；rank和history index u32，DecoratorId/WallId u64 | 源语义对应，实际完整key ABI未知 |
| new_edit_sequence | std Vec::new泛型复用，无私造指针或容量；不顺带提交历史 | 语义空序列对应，元素类型/align与实际owner待补 |
| Shape center | tag32取4+8*tag的两float，要求合法initialized record，无伪默认分支 | 合法输入投影对应；非法Rust bounds panic非原码panic证明 |
| 各借用/长度 | Archivist两字段独立借用、storage透明泛型借用、slice原样返回、Replay observed_len | 仅安全字段投影，非完整native结构/owner恢复 |
| 默认/零构造/interrupt | Attachment零tag、Animation两个正零、history32常量0x02000200、Transient保存两输入且counter零 | 原返回/初始化观察对应，完整类型及父owner义务未知 |

## unresolved与接受范围

1. 所有35组原证据function_boundary_verified仍false；同码与ret不支持自动升级完整原型/边界、跨版本caller兼容性。候选源码默认Rust repr允许重排，FinalizedWallHole的kind/origin字段是语义投影；未经显式decode/布局适配，不得cast原20B记录为该struct。Triangle、Rectangle3d、TransientGenerator同样不据字段注释宣称原生ABI。
2. 所有消费者、producer合法域、完整enum、字段命名、历史/编辑/删除关系只恢复被取证的部分。闭合洞过滤字节为0是观察，“origin/主记录”语义尚属推断；原短构造体不自身执行过滤，closure规则来自支持consumer。C lantern使用key低字节顺序实证[0x11,info_byte]，完整subtype/rank映射仍不扩展到全caller。
3. borrowed字段的输入视图须实际owner适配。泛型<T> Vec空序列允许较原未知元素更宽输入（如ZST），它是语义API，不能用该泛型实例当原实际24B Vec ABI验收。
4. closure包含原锁指纹但没有Cargo manifest，std/glam版本/features、编译配置及实际API兼容尚待整合；没有重新恢复glam/Vec内部。上一批10模块仅本批依赖用法获本轮静态审查，不将上一批全部52组升级为完整review通过。
5. 普通Rust浮点对应分离运算/ordered比较；NaN payload、sNaN quieting、符号零和跨架构编译器实际指令不在本轮执行范围。未运行独立反例、native differential或World链。
6. 冻结与assembly静态闭包完整仅针对声明31模块与列举273文件；不能称为完整生产依赖闭包、全游戏整合或恢复结束。

未运行构建、测试、oracle；没有修改候选、共享生产工程或数据库。仅写本final-review.md。F01修复后需重新核对受影响manifest及closure，并追加新指纹/范围结论，旧冻结审查不自动继承。

## F01修复后独立复核与最终结论

作者已将assembly首行改为“glam依赖须由整合绑定目标C原锁版本”，准确表达未来整合义务。独立读取更新后的delivery、closure与assembly，重新核对21份manifest及其列举文件、273个closure文件，全部SHA一致。读取保留的delivery-before-F01.json对比，仅assembly/lib.rs及assembly/dependency-closure.json的列举SHA改变；35组函数源码无变动。此复核没有运行编译、测试或oracle。

新冻结身份：

- assembly/lib.rs：`5dfa70d6a9b51d90a50bb3c402aa1818073055c20aa416ceff44965216fe7b98`
- TG-1d8b7844066af51d6b delivery manifest：`a615d1cf4868b067fc487f0c606b5c94119dc69199e2adfee3304a833f5fc80e`
- dependency-closure.json：`2ff3f7264015d344a409cffbbbefc50d14a828c1b31da2b5e116dee00ec4dcf5`
- deliveries.json：`b71d5c87f8998c21a3c2d5ac270adcc4f607c0709387c2aba3521238b33e8b90`

F01已关闭。最终结论为 `passed_scoped`，范围仅35组叶体观察语义、明确标注的安全字段/寄存器投影、限定闭合洞过滤消费者、声明的31模块静态装配与当前冻结引用一致性。前文 `changes_required` 保留为修复前审查记录，由本节最终结论取代。完整作者ABI、真实producer/consumer/history/removal链、依赖版本/features兼容及运行行为继续 `needs_evidence`；所有函数保持 `implemented_unverified`，不升级 verified，也不声称构建通过或完整游戏恢复。
