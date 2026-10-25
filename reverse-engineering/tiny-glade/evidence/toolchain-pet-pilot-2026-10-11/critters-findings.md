# Critters审计与C sheep pet切片

已使用native-rust-reconstruction，以及本轮新native-tool-batch、native-evidence-pack（消费其它agent交付的C原包）、native-module-handoff。旧B静态导出来自patched分析库，原C规则来自原发行指令；不混同analysis/oracle。

## 现有覆盖与最高收益缺口

本轮前源码只有firefly goal/velocity/smooth，独立harness；B-firefly-verification.json记录780/786/536例，ok=false，保留1例smooth NaN符号差异。不能称所有critters已恢复或B firefly已全验。原羊/鸭/wren/boids等仅有三版模块/PDB和B导出，无相应行为实现；更高收益是原宠物输入→事件→动作状态→实际消费者，非再扩firefly leaf。

新增C sheep pet源码：sheep_pet.rs、sheep_pet_runtime.rs、sheep_pet_c.rs及独立c-harness；根critters/mod.rs仅导出纯core，未改building/rootCargo/lib。C APIregister(App)安装Message reader和实际Query源更新，再派生locomotion gate；delta必须由真实GameTime+28提供，不默认。104B源观察投影与target来自原+20；不造Sheep constructor/ID。

| 版本 | 原source/owner | pet / UI / being_petted |
| --- | --- | --- |
| A | system-critters sheep_pet_animation.rs / ui_pet_sheep.rs；country-core critters/sheep.rs | 141b37860 / 141b3e820 / 141107f40 |
| B | 相同原模块路径，sourcechecksum独立 | 14231ac20 / 142324f80 / 14089d4c0 |
| C目标 | 同原模块路径，sourcechecksum不同，不自动等价 | 14293a350 / 142943c20 / 140935910 |

原source校验详见critters-three-version.json。A/B/C的名字与路径支持定位，不接受跨版本行为；本次实现/原码比较只为C。C pet原PE.pdata区间14293a350..14293a83d（1261B），UI142943c20..142944b28（3848B）；unwind interval不等于已证明完整原函数CFG。C包的两点source/inline不能泛化整个owner。

## 原业务规则

真实PlaySheepPetAnimation列表按source+20值做membership，每行最多处理一次。五tag dispatch原C数据表1435ec9d8：idle0收到event才写phase1/progress0/controlbyte1（当前weight保留）；rise1按delta/0.3先加progress，严格>1才进hold2且progress0，weight保未clamp的计算值；hold2按delta/0.2推进，原SETBE包含unordered，<=1或当前event继续保持，否则fade3/progress0，weight总1；fade3按delta/0.3推进，>1转complete4且不写progress，写controlbyte0与weight1-progress；complete4下一调用才回idle0与weight0。不能提前合并complete/idle、clamp过量或事件立即重启fade。

原tick赋值在idle有event及各phase1..4明确存在；idle无event不写，即使dt0业务值相同也不能以Eq替代该策略。原Sheep::being_petted完整C entry读取+58 amplitude，ordered>0.01才真；NaN false。原C walk caller142934fb9调用该helper；B完整consumer表明为pet时跳过walk动作推进。runtime派生gate消费该predicate与实际amplitude；+60仍只保observed字段，不宣称其其它renderer/控制语义已解。

## 独立工具forward-test与证明边界

critters-batch-plan.json是实际新skillforward-test：1metadata+1get_functions内含5个B函数，一operator/2MCP步骤，2.922424s，summary ok=true。local patchedEXE SHA+返回path校验通过，DB import hash未核；B pet库body1013B与PDB1309B不同且decompile漏5tag switch，故工具成功不是业务恢复成功。原C指令表/分支解决了这个实际gap，而不是等新的Ghidra全库。

pack-C-pet的cold/warm由另agent运行，本agent独立消费其原身份/ASM/metadata，未重复headless或冒称自己执行该collector。native小proof仅执行完整选择行dispatch→原query-loop join14293a489以及being_pettedentry→RET；C120例全104B/tick/predicate一致，原System整个query、eventmembership与UI入口未执行。World四项检查（3World+1pure）为C公开Bevy adapter，不是原fork调度证明。

当前切片关闭：observed源/原messagepayload→真实Message reader→五阶段源写入→amplitude/predicate locomotion gate→finish/Source移除/despawn清理。最高收益下一边：恢复原UIwidget dragged/picking→PlaySheepPetAnimation sender、country-core Sheep startup/ID、动作/renderer具体weight/controlbyte消费；再接移动/terrain/inside-building反应和record/replay。不能用观测Source或signifier手填资源称完整宠物完成。详见critters-handoff.json。

最终review修复：adapterGate用amplitude bits比较消除相同NaN重复publication，保signedzero；这是adapter选择，非原consumer tick。Source原changed策略用实际Bevy ticks追加核查。C锁seed修正后113registry identity全匹配，4checks/120小proof重新执行。handoff首轮新版audit因verified link含未解完整System边而拒绝，已保first-rejected文件并将有界切片与原sender/backend gaps分独立links。
