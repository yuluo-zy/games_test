# RE-ANALYSIS：interaction 模块 decompile-hardpoint + re-analysis 卡点

> 生成：re-analyst-hardpoints-interaction · 2026-10-17
> 范围：总账 `planning/2026-10-17-unimplemented-ledger.md` §4 decompile-hardpoint 与 re-analysis 分类、interaction 模块条目
> 后端：radare2（发行 RNE，worker `hardpoints-interaction`）+ Ghidra（共享只读，`--worker shared`）。未改 src，未改库。
> 证据：`evidence/` 子目录（radare2 反汇编/pdg、Ghidra 定点反编译、跳表解码）。

## 0. 两条方法论突破（适用于全仓同类卡点）

### 0.1 radare2 pdg 需先 `analyze` 才认函数
`half_timbering`（0x141218e30）此前三路失败（Ghidra/ReVa 空 + radare2 pdg "No function at this offset"）的根因是 **radare2 未分析该区**。在 job 中先执行 `analyze`（level 1，~2 分钟）后，`decompile_function` 直接产出 13KB 可读 C 反编译。**结论：凡 radare2 报 "No function at this offset"，先 analyze 再 pdg，勿判死。**

### 0.2 Rust 跳表编码 = `table_base + signed32`，可离线解码
本二进制 Rust switch 跳表统一为 4 字节有符号偏移，相对表基址：
```
lea rdx, [0x14xxxxxx]           ; 表基址
movsxd rcx, dword [rdx + idx*4]
add rcx, rdx
jmp rcx
```
`target = table_base + i32(entry)`。用 `hexdump` 读 .rdata 表 + Python 解码即可得到全部 case 目标（本会话解出 7 张表，全部落在宿主函数内，证明编码正确）。

### 0.3 Ghidra 函数边界在跳表处截断 = "jumptable 破坏"根因
`toolbar_alignment`（body_end 0x142344647）、`place_childen_impl<XY>`（body_end 0x140fe18b0）的 Ghidra 函数体在 `jmp rax` 处被截断，跳表 case 全部落入"函数外"，导致空/碎片反编译。**这是"跳表破坏"的真实机理**：函数本体完整，只需按 0.2 解码跳表 + radare2 定点反汇编即可重建。

---

## 1. interaction/clutter

### 1.1 `populate_autoclutter_regions`（0x1422d6ed0）· decompile-hardpoint → 已攻克（跳表族）

| 项 | 内容 |
|---|---|
| 失败原因 | 44KB 主体 Ghidra 碎片化（16 段）；两跳表未解码 |
| 攻克方法 | 读 .rdata 表 0x14304382c / 0x143043818，按 0.2 解码；radare2 定点反汇编 handler |
| **mesh-type 跳表 0x14304382c** | 26+ 项，全部落回函数内（0x1422d7d8e..0x1422de825）。分派（0x1422d7da2）：`movzx eax, byte[ptr-8]` 取 mesh_type → 跳表分派 |
| **mesh handler 语义** | idx 5,6,8,11,12,13,14,15,18,19 → **0x1422d7d8e**：仅透传，落 kind 分派（0x1422db570）；idx 0,1 → **0x1422d7df6**：sum 循环（f32 累加，stride 0x10/0x40）+ Vec 处理；idx 2/3/4 → 0x1422d7e2c/0x1422d7e28/0x1422d7e22：读 `[rbp+0x28]`/`[rbp+0x30]` 直通 0x1422d8090；idx 7 → 0x1422d8381（内嵌另一层跳表分派，字节被 radare2 误反汇编为数据，真代码从 0x1422d8395 起）；idx 9,10,17,20,21 → 0x1422de825（错误/panic 路径区：`enter unreachable code`、`populate_autoclutter_regions.rs` 行号 0x28/0x46 panic）；idx 16/22/23/24/25 → 0x1422d8993/0x1422d88fd/0x1422d8e7a/0x1422d8e3a/0x1422d8e61 |
| **kind 跳表 0x143043818** | 5 项 → 0x1422db5b0/0x1422db824/0x1422dc370/0x1422dca9f/0x1422dd1d7（kind 分派，每项带矩形/计数处理） |
| 逻辑结论 | filter_clutter_object 内联确认；mesh_type 过滤为主（多数类型透传至 kind 分派），少数类型做密度累加/panic；`RaycastWorld::legacy_raycast` 掩码 0x30fb、水位阈值 0.0 沿用 RESTORE 记录 |
| 剩余未知 | mesh 类型枚举值→语义映射需按 rust 源 enum 对齐；idx 7 内嵌第二层跳表基址（位于 0x1422d8381 区，字节被数据污染，需重对齐） |

### 1.2 `add_autoclutter_on_windows`（0x1422cd7d0）· decompile-hardpoint → 已攻克（反汇编）

| 项 | 内容 |
|---|---|
| 失败原因 | Ghidra bad instruction data（边界 364）；pdg 现报 "Bad instruction count exceeded: r0x00014304239f Unable to resolve constructor" |
| 攻克方法 | radare2 `analyze` 后 `disassemble_function` 得全函数 16KB 反汇编（`evidence/r2-1422cd7d0-*.disasm.txt`） |
| 逻辑结论 | 系统遍历窗口实体查询（`[rbp+0x78]` 迭代器），主循环 0x1422cd8c0：向 `[r15+0x50]` 的 Vec（stride **0x68**）写 Spawn 记录：`qword=5@+0`、Vec3@+8（`var_60h_2`）、`dword=1@+0x18`、`qword@+0x20`、`byte@+0x30`、`u16@+0x31`、`byte@+0x33`、`dword@+0x34`（r14d）、`qword@+0x38`（r13）、`qword@+0x40`（rdi）；空间不足时 `fcn.14078c880`（Vec reserve）+ panic 路径 `crates/systems/clutter/src/autoclutter/add_autoclutter_on_windo*` |
| 剩余未知 | 各字段的 Rust 类型（5=kind？+0x18=1=标志？）；实体遍历的过滤条件细节 |

### 1.3 `process_autoclutter_candidates`（0x1422ce260）· re-analysis → 未新增（沿用既有重建）
RESTORE/REVIEW 已重建（enumerate+filter_map→稳定排序→贪心重叠剔除→switch 3/4/5）。未发现新卡点，不需要补证即达实现门槛；closure_env 谓词猜测由 restorer 按已重建语义落库。

### 1.4 `load_clutter_meshes`（0x1422c0760）· re-analysis → 未新增
~50 个内联 closure_0 逐个资产加载 + `from_iter<RangeInclusive<i32>>`。网格名/资产 id 清单需 strings 提取，本轮未取（成本高、价值低——现 56 路径表为 I 推断，若 restorer 用资产加载通用链可先落库后核）。

---

## 2. interaction/onboarding

### 2.1 `hint_focused_edit_hint`（0x14227dd60）· decompile-hardpoint → 已攻克

| 项 | 内容 |
|---|---|
| 失败原因 | 跳转表 DAT_1430386a0 未取证 |
| 攻克方法 | Ghidra fresh 反编译（`evidence/ghidra-0x14227dd60-*.c.txt`，完整）+ 跳表解码 |
| 逻辑结论 | 完整控制流：`puVar3[1]==0` 时计算剩余计数并写 `*puVar4 = n - (x+y)`；`*(bool*)(puVar3+9) = 4 < count+1`（**count>4 置标志**，对应 ledger）；随后查询 focused edit（`lVar5+0x100` stride 0xa0 / `lVar9+0x1a8` stride 0x48）得 `lVar12`（0x58-stride 目标偏移）；门控 `lVar12!=0 && flag!=0 && *in_RCX==0 && legacy_raycast()!=2` → `geo_src_to_edit_subject()` → 按结果 local_d8 走跳表 |
| **跳表 0x1430386a0** | 6 项 → subject 0,3,4,5 → **0x14227e113（epilogue，无 hint）**；subject 1 → **0x14227e09f**（`fcn.140a91ef0` 取 subject，若 `[rax+0x258]`(hit type)<2 则落 0x14227e0c1，否则返回）；subject 2 → **0x14227e0c1**（`make_focus_or_camera_binding_hint 0x141c3de00` + `fcn.14227ba70` 显示，word tag 0x100/0x101） |
| 剩余未知 | `geo_src_to_edit_subject` 返回判别值语义（core-api 依赖，已列总账）；subject 1 的 0x258 hit type 枚举 |

### 2.2 `hint_position_update`（0x141c50510）· decompile-hardpoint → 已攻克

| 项 | 内容 |
|---|---|
| 失败原因 | 跳转表未恢复（默认分支 `UNK_142f14844` "Too many branches"）+ 相机矩阵/几何依赖 |
| 攻克方法 | Ghidra fresh 反编译（完整）+ 跳表 0x142f14844 解码 |
| 逻辑结论 | 入口 `WindowSize::as_vec2`；边距 `fVar17=W/1000`，`fVar24=20·(W/1000)`，`margin_x=30·(W/1000)`；双分支遍历（`cVar2=byte@+0x108` 0/非0 两套 stride 0xa0/0x48 查询）。**Placement 分派**（判别值 uVar7）：case 6=光标（`pos=cursor+{2·fVar24,0}`）；case 7=世界点+`CameraMatrices::eye_right`+`world_space_to_screen_space`（`pos=screen+{fVar24,fVar24}`）；case 8=`Rectangle2d::as_points3`×4 → 屏幕空间四角 min/max 夹取 + 边距；case 10=屏幕中心（`{w/2,h/2,0}`）；**default → 跳表 0x142f14844（21 项）**。末尾 `time_continuous_lerp` 平滑 + 写 `local_160/local_178`（`local_160[2]` 首次置 1，否则 lerp 更新 +0xc） |
| **跳表 0x142f14844** | 21 项 → 0x141c50d19/0x141c50e8d/0x141c5101a/0x141c5133c/0x141c513b1/0x141c51493/0x141c517fd/0x141c51815/0x141c51859/0x141c5273a/0x141c52753/0x141c52acd/0x141c52b8c/0x141c52c4d/0x141c52d20/0x141c52df3/0x141c53058/0x141c5306d（真实函数跨 0x141c50510..~0x141c53080，Ghidra body_end 0x141c50B12 仅首段） |
| 剩余未知 | 其余 17 个 Placement 变体的 case 体（可从上述 target 定点反汇编）；Placement 枚举值→变体名对齐（restorer 需 rust enum） |

---

## 3. interaction/wall-constructor

### 3.1 `half_timbering`（0x141218e30）· decompile-hardpoint → 已攻克（pdg 可用）

| 项 | 内容 |
|---|---|
| 失败原因 | Ghidra/ReVa 空（void() 占位 + varnode hash 失败）；radare2 pdg "No function at this offset" |
| 攻克方法 | **radare2 `analyze` level 1 → pdg 成功**（13KB，`evidence/r2-141218e30-half_timbering.pdg.txt`）+ 16KB 全函数反汇编（`evidence/r2-141218e30-*.disasm.txt`） |
| 逻辑结论 | 首段：SSBO 库类型分派——`fcn.140b2a600` 取库，`memcmp(类型名,61字节)` 匹配 `WoodenPlankSsbo/BrickTransformSsbo` 系 → 取 `[param_1+0x180]` 结构；非匹配走 `fcn.1428cab41` 错误路径。主流程：取 10 qword（`puVar23[4..0xb]`）→ `fcn.14120c9d0(&0x60B结构)` → 按 `cStack_f4`（`1408e3380` 返回的标志）分 `fcn.140b2a620` / `fcn.140f86cd0+141206480+140b2a680` → **`fcn.140b34f80`（split_height_into_skirts）** → `Vec<f32>` 追加 + 扩容（0x15 阈值分流 `14122cd30`/`14122f250`）→ 遍历扫描（`[param_1+0x280]+1.0` 上界）。类型名串含 `country_core::resources::roofs::roof_buffers::WoodenPlankSsbo`、`BrickTransformSsbo` |
| 剩余未知 | 逐梁数学/closure 装配（函数 27KB，pdg 已分页截断；按行号续取或 radare2 定点反汇编 case 块）；`0x180` 结构字段语义 |

### 3.2 `process_playermade_stair_pillars_proposals`（宿主 0x140178a70）· decompile-hardpoint → 已攻克

| 项 | 内容 |
|---|---|
| 失败原因 | bevy `impl$3::call_mut` 系统包装，内层业务块未定点 |
| 攻克方法 | Ghidra fresh 反编译宿主（47KB，`evidence/ghidra-0x140178a70-*.c.txt`） |
| 逻辑结论 | 宿主名 = `call_mut<tuple$(ResMut<StairsPillarProposals>, Res<UiSignifierStream>, ResMut<RaycastWorld>, ResMut<StairColliderRegistry>, ResMut<RtWorld>, EventWriter<UpdateRtWorldEvent>)>`——6 参 SystemParam 坐实。**业务块已内联**：`from_iter` drain `HashMap<StairGraphEdgeIdentity, Vec<StairsPillarProposal>>` → 合并附近提案（**距离<0.01 合并，size 取 min/max 包围**）→ `HashMap::insert` 回写 → 每提案 `StairGraphEdge` hash_one → RawTable 查 `StairColliderRegistry` → `reserve_rehash` → `LAB_140179334` 后续（raycast/碰撞/rt 事件）。**StairsPillarProposal stride 0x2c** = {4×f32 pos@0, u64@0x10, kind byte@0x18, u16@0x19, byte@0x1b} |
| 剩余未知 | LAB_140179334 之后的 raycast/碰撞注册/rt 事件细节（已内联于宿主，继续读该文件即可）；`StairGraphEdgeIdentity` 哈希细节 |

### 3.3 `construct_playermade_stairs_arches_n_walls<true>`（0x140496800）· decompile-hardpoint → 已攻克

| 项 | 内容 |
|---|---|
| 失败原因 | 两泛型 \<true\>/\<false\> 翻转、closure 绑定、装配未逐语句 |
| 攻克方法 | Ghidra fresh 反编译（97KB，`evidence/ghidra-0x140496800-*.c.txt`） |
| 逻辑结论 | 全流程：`from_iter`×多段（closure_env_1<false> 映射曲线点 → closure_env_2<false> 遍历 `(usize, StairArchType)` → closure_env_3<true> zip → closure_env_4<true> enumerate）→ **`ArchWalker::new()` → `ArchWalker::walk<closure_env_6<false>>`** → `RaycastWorld::raycast_simple` → **`StairsPillarProposals::add`** → `WallConstructor::from_curve`。**\<true\>/\<false\> 语义**：closure_env_3/4/7 用 \<true\>（构造 arches），closure_env_1/2/5/6/8/9 用 \<false\>（walls）——与"构造 arches + n_walls"含义吻合（true=arches 分支） |
| 剩余未知 | ArchWalker::walk closure_env_6<false> 的 per-brick 装配（与 4 单态 walk 已重建逻辑对齐即可） |

### 3.4 `sine_wave`（BrickRowSine，module 401 inlinee 24627/24641/24629）· decompile-hardpoint → 未攻克（方法已定）
内联无独立编译实例；宿主 per ledger 为 from_curve（source_line 12/45/82）。**建议路径**：half_timbering 现已可 pdg（见 3.1），其 17 closure 装配中应含 BrickRowSine 内联体——用 LLVM PDB inlinee 按行号在 half_timbering 反编译中提取；或候选宿主 construct_walls/arches 砖行块。本轮未完成（需 LLVM 提取脚本，成本高）。

### 3.5 `resolve_hole_overlap` case0 split_x · re-analysis → 结论已定，仅需冻结用例
fresh 反编译已证横切分支以 `MyBrick@0x25`（split_x）为门，country-core `b.split_x==0` 匹配。**不再需要反编译**；剩余动作 = 补定向冻结用例（split_x==1 && split_y==0 && Yoverlap && !Xoverlap），属 reviewer/restorer 测试任务，不属本 re-analysis 范围。

---

## 4. interaction/toolbar

### 4.1 `add_toolbar_item`（0x141e4dc40）· decompile-hardpoint → 已攻克

| 项 | 内容 |
|---|---|
| 失败原因 | kind 跳转表 DAT_142f8f774 U |
| 攻克方法 | Ghidra fresh 反编译（7KB，完整）+ 跳表解码 + case 体定点反汇编 |
| 逻辑结论 | `bVar2=*in_R9`（ToolbarItemKind 判别值）。**kind 4/5 = 容器**：遍历子项（`in_R9+8/+0x10/+0x18`，stride **0x40**，`*(char*)(item+0x38)!='\x02'` 为遍历界）+ 递归 `add_toolbar_item()` + `drop_in_place<ToolbarItemKind>`。**其余 kind**：`Taffy::new_leaf`+`add_child`，`Commands::spawn<TaffyNode,UiScaleWithUserSetting,UiInteractionState,UiAnimationState,UiMouseInteractable,Ui2dExtent,Ui2dImage,UiDrawOrder>`，按 `[in_R9+0x38]` 插 `UiShowIfParentIsInFocus` / `ShowOnlyInFpvAndFree` → **跳表 0x142f8f774** |
| **跳表 0x142f8f774** | 8 项 → kind0→0x141e4e5f3（调用 `[rax+0x18]` vtable 后压 **0x60B** 记录 {fn@0x140fdf9a0,0,0,0,0,0,rbx,r13}）；kind7→0x141e4e7e0（压 **0x20B** 记录 {fn@0x140fdf500,r13,r12,0}）；kind2→0x141e4e817（读 `[rax+1]` 子判别，三张 f32 表 0x142f8f6d8/0x142f8f70c/0x142f8f740 取朝向，调 0x140c904a0）；kind1/3/6→0x141e4e9fe/0x141e4e95b/0x141e4e911 |
| 剩余未知 | ToolbarItemKind 枚举值→变体名（Button/Icon/Label/…）；kind0/1/3/6 记录字段 Rust 类型 |

### 4.2 `place_childen_impl`（0x140fe12a0/0x140fe19a0）· decompile-hardpoint → 已攻克

| 项 | 内容 |
|---|---|
| 失败原因 | justify 全枚举 + 跳转表 0x142bc223c/0x142bc2260 坏分支未展开 |
| 攻克方法 | Ghidra fresh 反编译两单态（XY 17KB / YX 10KB）+ 两跳表解码 |
| 逻辑结论 | **LayoutStyle 布局**：`+0x44` swizzle 字节（0→XY，非 0→YX，对应 0x14068ea20 分派）；`+0x46` justify 字节。**justify switch**：XY 单态 33 case（0x0..0x20，`switch(*(byte*)(style+0x46))`），YX 单态 24 case。两跳表：XY→0x142bc223c（33 项，target 0x140fe14ef..0x140fe29ce），YX→0x142bc2260（24 项，target 0x140fe1c05..0x140fe2a34）。Ghidra 两函数 body_end 均在跳表处截断（XY @0x140fe18b0、YX @0x140fe1f9f），case 体在"函数外"——0.3 机理 |
| 剩余未知 | justify 枚举值→JustifyContent 名对齐（taffy 0.3.13 落库后）；每 case 的尺寸/位置数学由 decomp 直接提供 |

### 4.3 `toolbar_alignment`（0x142344560）· decompile-hardpoint → 已攻克

| 项 | 内容 |
|---|---|
| 失败原因 | jumptable 破坏；对齐 case 偏移公式/资源偏移 U |
| 攻克方法 | 反汇编头（0x142344630）发现分派 `movzx eax, byte[r13+0xf0]; lea rcx,[0x14304e6d8]; movsxd rax,[rcx+rax*4]; add; jmp` → 跳表解码 |
| **跳表 0x14304e6d8** | 20 项 → alignment 0→0x142344648（调 0x142376050/0x142375ce0/0x142375b40 设宽高）、1→0x142344760、2→0x1423446a0、3→0x142344700、4/6→0x14234485d、5/7→0x1423447c0、8→0x1423448c6、9→0x142344983、10→0x142344905、11→0x142344944、12→0x142344ac3、13→0x142344aff、14→0x142344ad6、15→0x142344ae8、16→0x142344d33、17→0x142344d49、18→0x142344d42、19→0x142344d3a |
| 逻辑结论 | 前置：遍历工具栏项（stride 0x28，`byte@+0x24` 清对齐位：`0x1020300 >> (byte[r13+0xf0]<<3)`）→ 按 alignment 跳表分派，各 case 调 layout setter 设位置/尺寸。**"对齐 case 偏移公式"即此跳表**。注意 Ghidra body_end 0x142344647 在跳表处截断（0.3 机理） |
| 剩余未知 | alignment 枚举值（0..19）→ 屏幕锚点名（restorer 按 case 体推断） |

### 4.4 `ui_sounds`（0x14233a7f0）· decompile-hardpoint → 地址更正 + 部分攻克

| 项 | 内容 |
|---|---|
| 地址更正 | **总账地址 0x14233c450 有误**：该地址实为 `toolbar::ui_interaction::ui_select_animation`（body 0x14233c300..0x14233c8aa）。真 `ui_sounds` = **0x14233a7f0**（body 0x14233a7f0..0x14233b30a） |
| 攻克方法 | Ghidra fresh 反编译（33KB，`evidence/ghidra-0x14233a7f0-*.c.txt`）+ 反汇编头 |
| 逻辑结论 | `switch(bVar6)` ~18 case（0..0x14），多 case 截断（"Bad instruction data"/"Sanity check requires truncation"）。前段：两路遍历窗口/聚焦项（stride 0xa0/0x48），`ucomiss xmm6,[r13+8]` 比较、`seta byte[rbx+2]`。另有 0x14233fc6a / 0x14233fe2d 两处未恢复跳表 |
| 剩余未知 | case 体完整语义（截断处需 radare2 定点反汇编 case target） |

### 4.5 `ui_slider`（0x1423486d0）· decompile-hardpoint → 部分攻克

| 项 | 内容 |
|---|---|
| 攻克方法 | Ghidra fresh 反编译（39KB，`evidence/ghidra-0x1423486d0-*.c.txt`）+ 反汇编头 |
| 逻辑结论 | 系统签名多参（MainCamera/BrushSizes 等接线）。**`switch(*(byte*)(entry+0x10))` ~28 case（0..0x1b）**，多处 "Bad instruction - Truncating control flow"。前段：`divss xmm10, [0x1429cd068]`（值/总长 → 01）、`xorps xmm14, 0xffff...`（-x） |
| 剩余未知 | 截断 case 体；entry+0x10 的 widget kind 枚举 |

### 4.6 `toolbar_auto_hide`（0x142343a10）· 部分（无独立跳表）
反汇编头：常量 `110.0`(0x14304e2fc)、`-1.0`、`10.0`(0x1429345a8)、`0.3`、`-0.5`、`0.5`、`{80,40}`/`{40,80}`(0x14304e300/0x14304e310)、`200.0`(0x142a13848)——即自动隐藏阈值/边距。系统多 SystemParam（栈参密集）。"对齐 case 偏移公式"实际属于 toolbar_alignment（4.3），非本函数。剩余：读 `[0x142376050 族]` setter 目标语义 + 完整反汇编。

### 4.7 `undo_redo_alignment`（0x1423448a0）/`undo_redo_auto_hide`（0x1423449f0）/`toolbar_steal_mouse_input`（0x142344df0）· 未展开
Ghidra body：0x1423448a0..0x1423448e5（短）、0x1423449f0..0x142344dd2、0x142344df0..0x1423454c0。短函数可直接 fresh 反编译（本轮未取，成本低，建议 restorer 直接取）。

---

## 5. 证据索引（`reports/re-analysis/hardpoints-interaction/evidence/`）

| 证据 | 覆盖 |
|---|---|
| `r2-jumptables.result.json` + 解码表 | 0x14304382c / 0x143043818 / 0x1430386a0 / 0x142f8f774 / 0x142bc223c / 0x142bc2260 / 0x14304e6d8 / 0x142f14844 |
| `populate-handler-*.disasm.txt` ×16 | populate_autoclutter_regions mesh/kind handler 反汇编 |
| `r2-1422cd7d0-add_autoclutter_on_windows.disasm.txt` | 全函数反汇编 |
| `r2-141218e30-half_timbering.pdg.txt` + `.disasm.txt` | half_timbering pdg（13KB）+ 反汇编 |
| `ghidra-0x141c50510-hint_position_update.c.txt` | hint_position_update 完整反编译 |
| `ghidra-0x14227dd60-hint_focused_edit_hint.c.txt` + `14227e09f/0c1/113.disasm.txt` | hint_focused_edit_hint 完整 + handler |
| `ghidra-0x141e4dc40-add_toolbar_item.c.txt` + `add_toolbar_item-kind*.disasm.txt` ×6 | add_toolbar_item 完整 + case 体 |
| `ghidra-0x140fe12a0/0x140fe19a0-*.c.txt` | place_childen_impl 两单态反编译 |
| `ghidra-0x140178a70-*.c.txt` | stairs proposals 宿主（47KB） |
| `ghidra-0x140496800-*.c.txt` | arches_n_walls<true>（97KB） |
| `ghidra-0x14233a7f0-*.c.txt` / `ghidra-0x1423486d0-*.c.txt` | ui_sounds / ui_slider 反编译 |
| `142343a10-toolbar_auto_hide.disasm.txt` / `142344560-toolbar_alignment.disasm.txt` | toolbar 头反汇编 |

## 6. 移交建议

- **restorer 优先按 §3.1/§2.1/§2.2/§4.1/§4.2/§4.3** 落库（这些卡点已有完整反编译/跳表，可直接重建）。
- `ui_sounds` 的**总账地址需更新为 0x14233a7f0**（原 0x14233c450 是 ui_select_animation）。
- `populate_autoclutter_regions` 用已解码跳表替换现 `process_dirty_cell` F3 近似；mesh-type 枚举值语义按 rust 源对齐。
- 剩余未攻克：`sine_wave`（LLVM inlinee 提取，宿主已可 pdg）、`load_clutter_meshes`（strings 清单）、toolbar `do_photomode`/`lut_gallery`（未展开）、`resolve_hole_overlap` 冻结用例（测试任务）。
