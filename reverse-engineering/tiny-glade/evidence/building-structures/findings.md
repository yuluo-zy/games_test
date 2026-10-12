# G05 lintel / support CPU owner

## 当前真实业务入口与所有权

B construct_walls140757800并不直接CALL construct_arches14075f680或construct_lintels14076d9f0。后二者是独立墙事件消费者；不能把registration邻居当内部调用。construct_walls实际调用add_wall_corners141215540、add_skirts1412221f0、stairs_signifiers141223d10。此阶段选择construct_lintels→calculate_wall_path_segments_impl14122ff90→place_support_beams141242530→beam实例实际CPU输出，避免恢复SupportBeamGrower leaf后停下。

`three-version-source-provenance.json`记录A/B/C private source checksum；`B-structures-native-call-map.json`为绑定PE `.pdata`实际function body范围和CALL。lintel/profile、arch_walker、walls的country-core/privateutils均是作者业务模块；std/slice/glam/allocator复用，不能把`crates/utils/rdp`误筛为cargo依赖。A/C source identity只归属证明，不表示行为等价。C新增construct_arches/wooden_arch_frame.rs另保留模块缺口。

## 真实caller数据链

construct_lintels14076e28c..297遍历storage的136B (0x88)WallPathSegmentationMask记录，CALL14122ff90@14076e39f输入该记录、Public+D0preparedsegmentfloat、Public+E0/E8 archarray、preparedCurve3+90/+98、preparedCurveU+B0/+B8、shape bool/Public::u_space。该函数返回materialized段Vec **104B (0x68)**，不得混用136Bstorage或PreparedWallPath.segment_values。后续修改clone arch数组，按每段最大(arch+worldY)对应地面Y及Public+284/+288生成新height。准确常数/字段仍按原指令恢复，不默认命名。

每104B段前3 Vec是heightarray、pointsVec3、U数组；+48/+50是原Mask local sourceordinal range(usize pair)，+58..64为LoopRange12；末4bytes保留unprovenpadding，不误称sourceCurveSlice coords。caller逐段height→mapper141204400聚合points/coord变换；为每点分配byteflags、首末置8；生成8B coord数组最后项(index=n-2,t=1)，然后CALL141242530@14076ea14。必须继续证明mapper对pointflags/coords/segment correspondence的实际变化，不能把任意flags fixture当World输入来源。

## 完整consumer依赖

- place_support_beams141242530：4609B完整greedy grower，从flags低4bit anchors双向propose、按1.0/3.15长度和曲率score合并/中止；剩余prefix/suffix处理不能漏掉。
- SupportBeamGrower::propose141fe09c0：463B；只内部边，不能将该leaf视为G05完成。grower布局初判：Option previous direction(0..16)、point index(16)、Vec3point(24)、accumulatedlength(36)、terminaltag(40)；返回proposal为index(0)、XZlength(8)、normalizedXYZdirection(12)、curvaturecost(24)、point(28)、tag(40)。名称由实际算术支持，padding保留。
- add_diagonal_support141243740：1233B，实际CurveSlice.extend_left141206ed0 / extend_right141207c10、arch-height sampling、Mat4/48B packer14089d560、metadata16B→**64B Beam ssbo**，最后SparseInstanceBufferWriter::push14120c430。输出不同于主墙96B；不能借墙schema。
- SupportBeamBuilder::emit_support_beams141243c20：1446B，私有Rdp141fe0b90简化range points，ArchWalker141233380生成真实64B WoodenPlankSsbo instances；不接受只恢复proposal或range通知即通过。
- 架构/帧dirty：SparseWriter::push返回true仅free-page pool耗尽（需扩容/重试），正常追加并更新dirty-bitset返回false；diag/plank据返回值写caller容量byte。原closure source/上传dirty契约；父接Bevy时按输入修改重建，wall删除清理SSBO/mesh/segment stats，不能只追加累计。

## 当前验收边界

G05前置104B materializer已恢复整owner：98 cases/304段全meaningful bytes一致；其中privateLoopSlice sampler需Vec3差分/乘/加顺序；现有dependency Vec3::lerp采用weighted form，替换后52cases浮点差分已保留glam-lerp-substitution-failure.json。正确重用glam operators而保留作者sampler指令顺序后重跑。此前74差分已保存first-materialization-failure.json，并从原141208d10证明clip endpoint使用(next-prev)*t+prev与bf75c290精确cap常量，未降低精度。此前完整64B消费者尚未验收；本批完整恢复与原码证明见下文。目标闭合真实136B段输入→104B分段→pointflags/heights/coords producer→完整双向grower→64B diagonal+plank CPU输出，再运行完整数组原码差分。B-core/B-emitters由父串行Ghidra导出，完整Capstone原PE body另保存供纠正Ghidra漏CFG。当前父继续G04真实worldsource集成，本目录不修改已冻结wall_production或共享runtime。

## 本批完整 lintel CPU 链与接入契约

`wall_structures.rs::construct_lintel_supports` 输入一个 **processed** `WallPathSegmentationMask`、真实 prepared positions/U、Public+C8/D0 的 interaction floats、Public+E0/E8 arch heights、shape<2 / shape!=2 布尔值，以及 Public+284/+288、WallId 和 mutable Public+F8/+100 third array。所有派生 heights、flags、coords、proposal、RDP indices 和 emitter context 都在内部生成，不向 World 暴露捕获值。返回完整有序 `Vec<WallWoodenPlankInstance64B>`，记录是 affine12f32 + segment ordinal + source index + WallId低32bit + 最后4B零；没有恢复材质字段的臆造。

真实调用者给每段 flags 全零，首尾置8，coords 全零且末项(n-2,1)。因此双向 grower 的首尾之外 prefix/suffix 均为空；当前恢复的是这个 **实际 lintel caller 域**，不是任意 flags 的通用 place_support_beams。成本是 `total*(direction_delta_length+.5)`；左右成本相同选右。曾将加数误作1，导致第60组几何完全相同但两条记录的 source_index 差异；`full-chain-greedy-cost-failure.json`保留该反例，按141fe0b04原literal纠正，没有忽略metadata。

ArchWalker 原 options 为 step=bits40499999 (≈3.15)、tail ratio=.1；plank先在 height-.225 处输出，x厚度.225、z厚度bits3eaccccc。斜撑增长预算为两侧各自累计长度*.85，合并时whole长度*.5；水平扩展距离是 length+.112500004+.1875。不是借其他构造上下文的.3参数。真实业务使用自有 RDP 的 endpoint-distance=1、epsilon=.2 与 short-left shortcut；复用 glam向量运算，不替代成普通 RDP 包。

**third写回顺序已证且实现**：14076e800..807 原位替换104B段的 height Vec 为 `max(selected_ground_y-clipped_ground_y-.2,0)+max(Public284+.5,Public288)`；14076f1f4..2e7随后读这个已替换数组，加 bitsbde66667 (−.112500004)，按段LoopRange将 local ordinal映射 prepared point index，与旧third做原maxnum（oldNaN选new）。不能使用初始materializer heights。闭合末点的额外 first→last复制属于 construct_arches另一个owner，lintel此处不添加该规则。

每processed mask调用一次该API，segment_ordinal在每mask从1开始（原caller如此）；third用同一墙缓存累计，caller取得结果后替换该墙既有64B实例集合。真正的special/source+65/Public+296 guard在Bevy外层：原字节==0跳过整个lintel loop。新增、洞/路径/高度/平台mask改变后重新计算；删除清理旧实例和对应墙缓存；精确EventReader和Bevy调度由父agent接，不将原SparseWriter容量增长byte误作业务dirty标记。

Public284是publication constrained_extra，special finite源由raw+54夹到.3..2.5；Public288从raw+5c按bits复制、尚未命名。因此有效special发布输入base下限.8。报告包含该下限以及负Public288，但不把任意损坏/NaN publication或negative-effective-height环境纳入验收；raw字段的命名/创建缺省不在此模块补猜。

## 原码证明边界

`verify_lintel_structures.py`原机执行14122ff90 materializer → caller14076e65c..86e（包含141204400真实mapper和height Vec替换）→完整141242530/141fe09c0/141fe0b90/141233380/141243740→实际14120c430 SparseWriter存储；第三高度另执行原caller14076f1f4..2e7完整迭代。所有范围检查是否到达实际return/末边，不能以instruction budget停止作为通过。

native fixture仅桥接标准分配、CRT（fmodf由原thunk/IAT进口名称证实）、关闭profile，以及已核Cargo `parking_lot_core::parking_lot::deadlock_impl::{acquire_resource,release_resource}` 的单线程TLS观察记账。原writer的free-pool、锁字段、count、payload、dirty bitsets均执行真实指令，所有段输出共享同一个writer，多页输出会覆盖跨页行为。没有hook掉游戏选择/扩展/矩阵/实例writer。

A/B/C模块归属差分先证作者代码，当前行为差分运行B；不声称A/C行为等价、外层ECS原码已执行、GPU mesh/材质上传已恢复或construct_arches/木拱框已完成。C wooden_arch_frame、arch profile、arches第三缓存writer是下一独立owner。

## 冻结结果（待独立 review / World 接入）

本批 Rust/core/probe 与 native/verifier 在 2026-10-11 冻结，不继续边改边绑定验证。`B-segment-materialization-verification.json`：98 whole-owner例、304 ordered segments，全meaningful bytes一致。`B-lintel-chain-verification.json`：178例、3,104条完整64B记录及全部third数组bytes一致；其中80组由Rust/原机各自执行calc→platform，产生304 processed masks /160 steps，含16组65/129/257点自由墙（16个跨页整墙用例、最大156条记录）。该报告使用同一个原SparseWriter跨所有段/Mask批调用并绑定源hash；容量足够，不桥接掉游戏writer或对每段重置writer来绕开跨页。

构建与复验：

```sh
cd reconstruction
cargo build --release --bin wall-structures-probe
cd ..
/Users/yuluo/Desktop/game/tools/venv/bin/python scripts/multi-version/verify_wall_structures.py
/Users/yuluo/Desktop/game/tools/venv/bin/python scripts/multi-version/verify_lintel_structures.py
```

全consumer接受范围是finite published special-wall author输入、真实caller首尾flags域、上述typed raw/prepared arrays。保存的反例及research文件不代表通过范围；`B-support-research.json`是134个内部view/525record辅助排障，正式接受以上178例完整链，不能以该leaf研究替代主链。

后续缺边仍明确：construct_arches实际CPU与third writer、C木拱框、原外层OnWallChanged HashSet/registry/GetMut/EventReader/ECS事务、GPU/material upload或visual mesh，不由这批证明替代。父接World真实ProcessedMaskRegistry和Publish/Opening/Height/Delete调度后另做完整业务链验收。
