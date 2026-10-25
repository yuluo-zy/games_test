# C Sheep Pet 阶段独立源码审查

日期：2026-10-11。仅审查恢复源码与有界原ASM/PE常量；没有编辑Rust、重复MCP/decompile、构建或运行原opcode/World。pet owner另行进行差分/World检查，本审查不能代替它们。

## 结论

在所审SHA下，五阶段数值与显式change-tick规则无需更改。原producer/UI/原ECS与scheduler未被冒称完成。已恢复的是resolved事件成员关系→104B原row状态机→`being_petted`门控；runtime为明确的C Bevy公共API adapter，完整locomotion动画consumer和真实UI/picking/调度仍待恢复。

## 输入与审查SHA

| 文件 | SHA256 |
|---|---|
| `reconstruction/critters/sheep_pet.rs` | `dabf5ed7aa69a3a15d338e10b0ef78e66414ebb64b95f346c04fe98cc5dc7a6b` |
| `reconstruction/critters/sheep_pet_runtime.rs` | `3a40dc46ac93af2ceb34c0a347778a7ede0e47f5105c305d5427ca357076a87d` |
| `reconstruction/critters/sheep_pet_c.rs` | `091fa41b39b61a61693682360ee0321cd380fff563b75ba47cb9f2355fa0788e` |
| `pack-C-pet/cold/pet_owner.json` | `c244dfc39f8de0b04619bb096fb89a1ed2e7370ec68c01c0c23d04e1a94dcc74` |

实际pack ID是`pet_owner`，不是`sheep-pet.json`。该pack绑定C原EXE `d8f9261b...3ad80a9` 与PDB `df472d8e...c78902`，range `0x14293a350..0x14293a83d`、1261B；`function_boundary_verified=false`仍保留，不据此新宣称整个System已验证。主证据：[critters-C-pet-original.asm.txt](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/toolchain-pet-pilot-2026-10-11/critters-C-pet-original.asm.txt)、[pet_owner.json](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/toolchain-pet-pilot-2026-10-11/pack-C-pet/cold/pet_owner.json)。

所审三个Rust hash与现 [critters-C-pet-verification.json](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/toolchain-pet-pilot-2026-10-11/critters-C-pet-verification.json) 的candidate_sources相同；该报告声明120cases、failure0，限定per-row完整dispatch和独立predicate，不是wholeSystem/原UI/scheduler。本审查仅核对报告scope/hash，没有重跑案例。

## 原指令合同核对

原jump table `0x1435ec9d8`五项从C原PE实际读取，分别指向phase0..4：`704 / 7c9 / 740 / 782 / 725`（省略共享高位`0x14293a`）。read-only PE seek同时确认常量：

| 用途 | 原地址 | bits |
|---|---|---|
| fade-in/out divisor | `0x142f26dc0` | `0x3e99999a`，f32约0.3 |
| comparison/hold amplitude | `0x142f04a24` | `0x3f800000`，1 |
| hold divisor | `0x142f0a3b8` | `0x3e4ccccd`，f32约0.2 |
| being_petted阈值 | `0x142f231e0` | `0x3c23d70a`，f32约0.01 |

| 阶段 | 原业务行为 | Rust判断 |
|---|---|---|
| 0 | requested才写+60=1、qword+18=1（phase1/progress0）与tick；否则不写 | 完全吻合，Ok(false)无修改 |
| 1 | progress=delta/0.3+old；SETA/JA仅ordered>1进入2并清progress；amplitude保存未clamp的progress；总写tick | 吻合；overshoot不钳制；NaN保持1且保NaN |
| 2 | progress=delta/0.2+old；SETBE或requested保持2，否则进3/progress0；amplitude1；总写tick | `!(progress>1)||requested`吻合；unordered保hold |
| 3 | progress=delta/0.3+old；ordered>1进4并保留旧+1c，否则写new progress并保持3；+60=0；amplitude=1-newprogress；总写tick | 吻合；跨1时不误清旧progress；NaN留3 |
| 4 | 只phase0/amplitude0/tick；不改progress或+60 | 吻合；不填额外reset |

原query-row stride `imul ...,0x68`与104B observed row一致，target从+20读u64。成员匹配是bool，所以重复事件不会在单帧反复推进；phase3/4事件不立即restart。原phase大于4不属于已证合法enum域，Rusttypederror是adapter防护，不能称原错误分支恢复。

getter `0x140935910..0x140935920`本轮原C局部反汇编：`movss source+58; ucomiss [0x142f231e0]; seta; ret`。所以strict amplitude>阈值正确：等于阈值false、NaNfalse、+Inf true。源码没有把+60解释成being_petted，保observed flag是正确边界。

active phases每次显式写tick，即使delta0或字节恰好同值；core用bool返回该决策，runtime `source.0=next`走BevyMut写入。phase0/noevent只读clone/target不应触change。不能以row Eq变化代替原tick。

## Runtime范围与保留疑点

1. `(advance,consume_locomotion).chain()`是adapter顺序，没有原注册/scheduler证据；文件首注释与verification runtime_scope已承认。GameTime+28 delta必须提供，没造默认Frame.delta；observed target不冒称BevyEntity ID。
2. `SheepLocomotionPetGate`当前是predicate projection，未调用原行走动画producer/clip更新。注释“caller skips walk animation”需要相邻原caller证据才能接受该消费者语义；本轮只独立核了getter。后续总结应说“恢复pet状态与行走门控”，不要说完整原locomotion系统已恢复。
3. `previous!=Some(&next)`使用f32 PartialEq。如果amplitude为NaN，即使bits不变也每帧重insert gate，可能使派生Changed重复触发。这是adapter tick/churn差异，不是原pet状态机错误；尚无该状态原World可达性证据。若扩大adapter NaN域或新增Changed消费者，建议按amplitude.to_bits()+bool/flag比较，或明确有限域；不可猜clamp/dropNaN。
4. Source remove/despawn后的gate清理是明确adapter生命周期政策。已有来源则投影、无来源则移除，未捏造原启动/默认/手工事件producer。原UI/picking/message writer、原Query acquisition、异步调度、完整动画更新仍是required gaps。

除上述边界/后续NaN projection观察外，本审查没有必须改动phase1/2/3/4、常量、严格比较或core changedtick的发现。源码变更后该审查仅作历史证据，须按新SHA重新限定检查。

## Runtime位比较补审（同日，源码修订后）

新 `sheep_pet_runtime.rs` SHA：`92c8d5ee10e7f1ed34acb08c8e87bef582f24133a37957fee288bd31116dffa6`。浮点核心 `sheep_pet.rs` SHA仍是 `dabf5ed7aa69a3a15d338e10b0ef78e66414ebb64b95f346c04fe98cc5dc7a6b`，未变；原常量/phase规则的此前静态结论不变。

新publication比较同时核对 `allow_walk_animation`、`observed_control_flag` 和 `amplitude.to_bits()`。相同NaN payload不再因PartialEq的NaN不等于自身反复insert；不同payload与正负零位差仍能更新；previous缺失仍首次insert。这关闭了本审查指出的adapter churn问题。源码注释明确“不是原consumer tick声明”，没有扩大原调度或完整locomotion恢复范围。

此次只读新comparison与记录hash，未重复原常量/ASM查询、120case proof、Worldtests或依赖验收。新runtime的执行验收和锁文件漂移修复由pet owner/父统一重跑，不能沿用旧runtime hash报告称新源码已经验证。
