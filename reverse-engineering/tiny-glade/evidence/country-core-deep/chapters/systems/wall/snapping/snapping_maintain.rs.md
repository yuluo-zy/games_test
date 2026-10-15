# systems/wall/snapping/snapping_maintain.rs

> 文件作用：吸附维护：snapping_maintain = mod key 切换 + NotifyHintSystemSnapping（跳表分发）；check_for_disable_snapping = mode 在 {2,3} 到写事件；snapping_signifiers 双查询画段/端预览线。

函数总计 3：业务 3 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::snapping::snapping_maintain::check_for_disable_snapping` | 0x140a6a560 | 19 | check_for_disable_snapping（mode∈{2,3} → 写事件） | O | wall\decomp\decomp-0x140a6a560.txt |
| `country_core::systems::wall::snapping::snapping_maintain::snapping_signifiers` | 0x140a6a650 | 40 | snapping_signifiers（段/端双查询预览线） | O | wall\decomp\decomp-0x140a6a650.txt |
| `country_core::systems::wall::snapping::snapping_maintain::snapping_maintain` | 0x140a6ad00 | 93 | snapping_maintain（跳表分发 + mod key 切换 + NotifyHintSystemSnapping） | O | wall\decomp\decomp-0x140a6ad00.txt |

## 实现方式（算法级）

### check_for_disable_snapping 0x140a6a560（O，全读）

签名（3 参）：param_1 = &mut SnappingMode，param_2 = 资源（首字节 flag，语义 I：mod key / 进行中状态），param_3 = &mut Mut\<EventWriter\<DisableSnapping>>（tick 回写 @+0x1c；事件行 **0x10B** {u64 seq@0, u8 flag@+8}，Vec {cap@+0x20, ptr@+0x28, len@+0x30, seq@+0x40}）。

分支：
- mode==2 且 flag==0 → 写事件{flag=0}，**return（不重置 mode）**。
- mode==3 且 flag==0 → 写事件{flag=0}，return。
- mode∈{2,3} 且 flag!=0 → 写事件{flag=1}，**重置 mode=0**，return。
- mode∉{2,3}：flag!=0 → 重置 mode=0；flag==0 → return。

**纠错**：DIGEST §2.3 称事件"0x20B 行"——实测行步长 `lVar4*0x10` = **16B**。事件语义 = DisableSnapping{snapped/flag}，flag=1 路径同时清 mode，flag=0 路径只发事件。

### snapping_maintain 0x140a6ad00（O，dis.py 117B 全解；Ghidra NotFound 已手解）

签名（1 参）：`&mut Mut<SnappingMode>`（[rcx]=&mode，[rcx+0x10]=tick 槽，[rcx+0x1c]=tick）。**非 ECS 大系统**，是"把吸附态发布到 +0x20 快照"的小函数。**纠错**：章节"mod key 切换 + NotifyHintSystemSnapping"无机器码依据，实为 mode 跳表分发的状态打包。

跳表 @0x142ad55cc（4 个 i32 相对项）按 mode 选加载：
- mode 0 → 0x140a6ad1b：不加载字段（r9/r8/xmm 归零）。
- mode 1 → 0x140a6ad4c：只加载 snapped@1。
- mode 2 → 0x140a6ad1d：加载 u64@+4→xmm1、f32@+0xc→xmm0、u64@+0x10→r8、byte@+0x18→r10、byte@+1→r9。
- mode 3 → 0x140a6ad37：加载 u64@+4→xmm1、f32@+0xc→xmm0、byte@+1→r9、u64@+0x10→r8（无 +0x18）。

公共尾：tick 回写 `[rcx+0x10]=tick`，打包到 SnappingMode+0x20..0x38：
`{mode@0x20, snapped@0x21, u64@0x24(xmm1), f32@0x2c(xmm0), u64@0x30(r8), byte@0x38(r10)}`。

+0x20 即 snapping_audio 去重读的"上一帧 mode"。

### snapping_signifiers 0x140a6a650

见 `snapping/snapping_signifiers.rs.md`（本模块内函数，PDB 归于 snapping_maintain.rs，已另立章节）。
