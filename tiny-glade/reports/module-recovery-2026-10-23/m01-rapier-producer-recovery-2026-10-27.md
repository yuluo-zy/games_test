# M01 原碰撞更新主系统恢复（2026-10-27）

本轮在既有 `country-core/src/systems/collision/world.rs` 内恢复 `update_rapier_colliders@1408d0490` 的完整十二参更新流程。五张实体表共用现有 `MeshInstanceSources`、`upsert_collider`、`remove_collider`、Rapier 与 dirty/flush 实现；不增加第二个 RaycastMesh ECS 组件或旁路注册表。

## 原件、目录与参数

先核对原 B `skeleton/B/crates/country-core/src/systems/collision/world.rs`，保留当前手工重构结构。[身份记录](m01-rapier-producer-identity.json)绑定 RNE SHA `f507a6…d03`、分析 EXE SHA `cf048e…323`。原 PDB procedure module496/record68128/source1875 的物理范围31293B，在两个二进制完全相同，范围 SHA `081c905027375a9b741ad9ed74648dd23529b9da08d9311cce3f0e5d8968b2c1`。

procedure 的 `void()` 是调试占位。[十二参 typed 原证](m01-update-rapier-colliders-typed-params.json)来自原 FunctionSystem/candidate binding，而非本轮手填候选 C ABI。实际参数依次为：World；Changed WallTriggerVolume + InstancedWall；Removed WallTriggerVolume；Roof query；Changed RoofTriggerVolume/Roof 且 Without 删除；Removed RoofTriggerVolume；Added 删除屋顶；ArchSegments；LintelSegments；PublicWalls；Changed Transform + AutoClutter；Removed AutoClutter。

[完整原 ASM](m01-rapier-producer-full.asm)覆盖物理范围，包括原 EH 独立体；[完整恢复 C](m01-rapier-producer-no-cleanup.c)仅是辅助解释，业务条件仍回到 ASM 和 typed 参数核定。

## 五表与真实业务

| 原 World 表 | 真实 key | 来源载荷 | 构造和生命周期 |
|---|---|---|---|
| +3dc8 主墙面 | Entity，保留 generation | tag=`raw_flag65<<3`，WallId@8 | 从真实 quad 调 `get_collision_geo`；独立移除/替换 |
| +3df8 屋顶 | Entity，保留 generation | tag2，flat/gable 字节，WallId@8 | 原三角点加 floor；原 normals/UV；identity 外部位姿 |
| +3e28 地板 | Entity，保留 generation | tag1，WallId@8 | 矩形四角/圆0.5步长完整扇面；原 None UV |
| +3e58 拱/过梁 | Entity，保留 generation | tag9，WallId@8 | 原104B段按 zip 相邻窗口追加三角与法线；None UV |
| +3e88 自动杂物 | Entity，保留 generation | tag12，Entity 完整bits@8 | 原尺寸/中心→cuboid；读取真实 Transform 平移/旋转 |

处理顺序为 Auto changed→removed；Changed 墙逐行拱梁→主面→地板；Removed 墙主面→地板→拱梁；Roof Added删除→changed→removed。保持原失败前已提交的变化，不用事务回滚篡改原顺序。

墙与屋顶的缺失合同不同。原墙 lookup 的183d/18ce→5c37是 required unwrap：墙发布缺失或无效，即使 trigger 为空也先失败；墙正在推倒时移除三表。屋顶则允许公共墙缺失/推倒后移除旧屋顶实例，并且在此之前不读取空触发体。两个条件都已有真实 World 测试。

墙参数是 `u_range=None`、`normal_offset=Some(shape_tag<2 ? bits(3ea147ae) : 0)`、minimum0、关联真实平顶 Roof 时顶部偏移 bits(beeb851f)、apply=true。平顶只从该墙关联的有效 Roof Entity 查询，不扫描世界中的其他屋顶。

地板门控是 `minimum_relative_height>0`，不是 elevation 或普通墙高；有序比较排除 NaN。实际顶点 Y 是 authority.elevation。只接受 circle/rectangle 原 tag，保留原 normals 的负零。圆采样数量来自原140c97bc0/140c97c50的饱和转换：`max(3, ceil(radius*TAU/0.5) as usize)`；复用当前逐点算法，避免其他用途的 try 采样接口提前拒绝非有限半径。256个原数量位型覆盖 ±0、负半径、NaN、±∞、subnormal 和饱和边界；不实际分配无穷规模网格。

Arch/Lintel 表身份通过 typed 参数与原栈核定：`flag65==1`读取参数9 Lintel（RBP+4b0→r10→stack290，1b08）；否则读取参数8 Arch（RBP+4a8→stack2a8，1c50）。选定表缺失/累计零三角就移除，不转读另一表。验收中两表均有不同的非空几何，能发现倒置或错误 fallback。

Auto 原公式只用 footprint 宽/深与 top-bottom，local center 是 `(height*0*.5,height*.5,height*0*.5)`，不把 bottom、footprint center/axis、Transform.scale 混入原公式。[原256输入](m01-auto-clutter-collider-native.json)执行实际原数值块，有限输出逐字相同；跨 x86/ARM NaN传播只单列分类边界，不冒称 payload 全同。

屋顶复用已独立恢复的 [原 floor 投影](m01-roof-trigger-floor-recovery-2026-10-27.md)。Wall quad 复用 [原 get_collision_geo](m01-wall-trigger-collision-recovery-2026-10-27.md)；拱梁复用 [315组完整原入口](m01-arch-collision-rosetta.json)及 [canonical Lintel owner/writer](m01-arch-lintel-collision-recovery-2026-10-27.md)，保持单一类型与真实生命周期。

## 原侧执行与真实验证

- 原地板完整两分支96组输出：索引、顶点、法线均逐 word 断言；UV严格为空；实际构造 Rapier shape。原 circle count 入口256位型另作精确验证。
- 地板探针桥接外部分配器与 CRT libm，不替换原几何指令。首次 x86 host libm 与 ARM64 返回有34/3272次 trig 差别，首失败出现1 ULP顶点差异。保留[完整 x86 几何](m01-floor-mesh-x86-external.json)、[全部 trig 实参/两端返回](m01-floor-mesh-external-trig.json)及失败日志。再将真实 ARM64 libm 返回重放到原外部调用点，严格验证原几何操作/采样流程与 ARM 候选。这里不证明 Windows CRT 的跨平台位等价，不使用 ULP 宽容断言掩盖差别。
- 同一真实 Bevy World 中两墙、两屋顶、一个杂物形成9个物理实例；真实12参系统维护五表；64个干净帧保持 receipt 和活跃实例数稳定。
- 覆盖两张段表均非空的分派、选定表 missing/短段移除、纯 resource 改动不代替 Changed trigger、关联 flat roof、缺失/无效发布、空 trigger、bulldoze、floor ±0/负/NaN/freehand 门控。
- 实际 Changed 替换与 Weak 释放；RemovedComponents/Added 删除；Entity generation重用隔离；Reload 清理五表/mesh owner；再通过真实组件 Changed 重建；最终 flush 后物理 arena 和注册计数归零。
- 失败次序测试证明：新的拱已提交，主面因全部 quad 退化失败时旧主面和未处理的地板继续保留。

最终命令为 `CARGO_INCREMENTAL=0 cargo test -p country-core --lib systems::collision::world --features native-physics --offline`。最终结果和源/原证/probe/fixture哈希见[验收收据](m01-rapier-producer-final-validation.json)。早期 v1～v5和失败保留，最终以收据指定快照为准。

## 工具修复与本轮纠正

该原函数的共享反编译错误是 `Unable to find unique hash for varnode`。本人唯一 worker `m01-rapier-20261027` 修 body 时剔除别的函数拥有的 EH 体，保留原字节；清局部注释、补候选十二个间接参数仍不能解决。仅对本地 DecompInterface 关闭 cleanup action 后完整返回324941B C；没有修改共享实例或原程序指令。[操作记录](m01-rapier-producer-actions-job.json)与[正常停止后重开验证](m01-rapier-producer-reopen-result.json)确认 body23221B、12候选分析参数和 C SHA 持久一致；最后正常关闭自己的 worker。

核验过程中纠正两项错误解释：墙 missing 不能照搬 Roof 可缺失门；同为104B段的两个表不能靠反编译临时名判 Arch/Lintel。失败/更正与最终测试都保留。下一轮先用 typed 参数→原栈/实参→真实 writer/consumer 交叉核对，再建验收 fixture；不得反过来让错误 fixture 自证。

## 完成边界与后续

本轮恢复的是这一原主系统及必要几何/五表 owner，非整个 collision 模块170函数或整游戏联合验收。原31KB Bevy 主体没有伪称已直接执行端到端 native oracle：主体合同由完整原 ASM/typed 参数核对、原数值 leaf 执行和真实 ECS/Rapier 多帧验证共同支撑。

生产 collision plugin 的完整父调度、真实墙/屋顶生成器到该系统的整游戏阶段、历史/编辑与窗口 GPU 联合验收由中央继续接线。本轮独立 Roof/Auto projection wrappers 复用同一内核，仅供有界测试；正式生产应只挂完整12参入口，避免重复消费不同 Removed reader。没有进行 OS 真实输入。
