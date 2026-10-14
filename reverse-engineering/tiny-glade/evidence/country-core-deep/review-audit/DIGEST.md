# country-core O 结论独立抽查 DIGEST（review-audit）

> owner: `tg_au1`（结论抽查审核）。日期：2026-10-14。
> 依据：MODULE_GAP_ANALYSIS §4（collision-eco 80% / ui-input 79% / audio-misc 59% 问题率；新章节 1413 条 O 从未抽查）。
> 方法：三域各抽 12 条 O 结论（共 36），优先布局/偏移/常量与"作用"明显相关行；独立用
> `rne-read.py --hex` 手解机器码 + 复用既有 decomp（独立读结论列，不盲信章节）复核。
> 函数身份：`sysres-procedures-v3.json`（S_PUB32 + CU RVA 合并）核对地址→名字（36 条中
> 34 条地址与章节声称的函数名一致；`color_id_roof::impl$0::default_value` 0x14099ca50 与
> `NameHash::compute` 0x140c9cd80 不在 v3 索引，但机器码直接证实其行为）。

## 判定分布

| 域 | ✓ | ⚠ | ✗ |
|---|---|---|---|
| collision-eco | 8 | 1 | 3 |
| ui-input | 7 | 4 | 1 |
| audio-misc | 11 | 0 | 1 |
| **合计** | **26** | **5** | **5** |

说明：抽样偏向"可机器码直接核"的小函数（按任务指引），这类结论在撰写时通常已过
decomp 校验，故 ✓ 占比偏高属预期。误差集中在**阈值/常量/布局细节**（判定 ✗ 者均为
章节给出具体数值/偏移而机器码不符）。

## 36 条抽查表

### collision-eco（章节 `chapters/` 内 collision/stairs/ivy/clutter/ui_clutter 归属文件）

| 地址 | 函数（章节声称） | 章节 O 结论 | 实测（机器码/decomp） | 判定 | 证据 |
|---|---|---|---|---|---|
| 0x14089d4c0 | Sheep::being_petted | f32@+0x58>0.0 | `movss xmm0,[rcx+0x58]; ucomiss xmm0,[0x142960674]; seta al`；常量=**0.01** 非 0.0 | ⚠ | 偏移对，常量错 |
| 0x1408cea20 | RaycastGeoSource::subworld | 查 disc→槽表（0x142aa4418） | `movzx eax,[rcx]; lea rcx,[rip+0x021d59eb]; movzx eax,[rcx+rax]`；表基址=**0x142aa4415**，表值 {0,9,1,2,8,14,3,7,6,10,4,5,11,12,13,15,16} 与 DIGEST 全同 | ✓ | 表地址差 3（0x4415 vs 0x4418），语义/映射全对 |
| 0x1408cdc20 | RaycastWorld::subscribe_to_changes | 25B tabloid Publisher<DirtyTiles>::subscribe | `movzx eax,dl; imul rax,rax,0x390; add rcx,rax; add rcx,0x170; jmp` —— 按槽号(byte)计算 **slot+0x170** 后尾跳订阅例程 | ✓ | 尺寸 25B 对；是"逐槽 publisher 地址计算"包装，章节略去了 byte 参数与 +0x390/+0x170 |
| 0x1408cc620 | SubworldColliderHandle::new | 写 {u32@+0(index),u32@+4(gen),u8@+8(slot)}；24B 枚举 | `mov [rcx+0x8],dl; mov [rcx],r8d; mov [rcx+0x4],r9d; ret` —— 写 {u32@0,u32@4,u8@8} | ✓ | 完全一致 |
| 0x1408d98a0 | xz_flatten_aabb | 输出 {min.x,min.y,max.y} | `movss [rcx],in[0]; movsd [rcx+4],in[8]; movss [rcx+0xc],in[0x14]` —— 输出 **{min.x,min.z,max.x,max.z}**（Aabb3 min@0/max@0x10 → in[8]=min.z,in[0xc]=max.x,in[0x14]=max.z） | ✗ | 声称 min.y/max.y，实为 xz 四元组 |
| 0x1408fe8b0 | StairsState::graph | graph getter=&self+0x28 | `lea rax,[rcx+0x28]; ret` | ✓ | 完全一致 |
| 0x140959f00 | AlignKind::position_offset | 变体1={flag u8@+1,Vec2@+4,Vec2@+0xc} 组合+常量缩放 | `movzx eax,[rcx]; xorps; disc0/1 分支；变体1: movsd xmm2,[rcx+4]; cmp byte[rcx+1],0 条件 movsd xmm0,[rcx+0xc]; addps; mulss [rip+const]` | ✓ | 结构完全一致（常量缩放存在） |
| 0x140869460 | Ivy::add_point | push_back 40B 点（id=next+1，reserved=0） | `param_1[0xb]=next+1` 后 push 0x28B 条目 {id@0=old+1, 点 20B, u32@+0x20=0} | ✓ | 完全一致 |
| 0x14094c800 | closest_segment_index | 28B 步长墙段 {xy0,xy1,h0@+0xc,h1@+0x1c} | 循环 `add r9,0x38`（**56B 步长**）；读 `[r9+0x18]`(h0) 与 `[r9+0x1c]`(h1)，h0<=max 或 h1<=max 跳过；xy0@0/xy1@8 | ✗ | 步长 28→**56**，h0 偏移 +0xc→**+0x18**；h1@+0x1c 与"高度≤max 过滤"语义对 |
| 0x1408b8e90 | clutter_mesh_ty_data | 静态表 switch(0..0x37)→8B 条目（名+kind） | switch(param_2) case 0..0x37 每 case `puVar4=param_3+0x18+i*8`（8B 条目），case 0x38→halt | ✓ | 完全一致（56 变体） |
| 0x141234590 | IvyDirectionProposer::get_direction | 3×noise3d+3×uniform 混合→θ∈[-40°,+40°]→(cosθ,sinθ) | decomp：FUN_141741c60(noise3d)×3 + func_0x141625ae0(uniform)×3 混合→归一化 2D 方向 | ✓ | 核心(3+3 混合→2D 方向)证实；θ 界 ±40° 未在机器码直接验证（I） |
| 0x1408fe7b0 | StairsState::try_set_endpoint | 候选邻边≥8→Unset，否则 Single | Unset 条件 `(lVar5!=0) && (7 < lVar5*8-8)`，lVar5=邻边 vec len → **≥2 条边**即 Unset | ✗ | 阈值 8→**2** |

### ui-input（`systems/ui*/input/mode_manager/toolbar/photo_mode` 归属文件）

| 地址 | 函数 | 章节 O 结论 | 实测 | 判定 | 证据 |
|---|---|---|---|---|---|
| 0x14092e5f0 | UiWidgetContext::clicked | hover>1 && click<2 && click&1 | `cmp byte[rax+0x18],2; jb fail; movzx edx,[rax+0x19]; cmp dl,1; ja fail; test dl,1; jz fail; cmp [rax+0x38],ctx.id; sete` | ✓ | 三条件全对（另有 id 匹配，章节略） |
| 0x14092e620 | UiWidgetContext::hovered | hover 字节 != 0 且 id@+0x38 == ctx id | `cmp byte[rax+0x18],1; jne fail` —— hover **==1** 非 !=0；并返回 `byte[rax+0x19]`（非纯 bool） | ⚠ | hover 条件错（==1）；返回值细节缺 |
| 0x14092e680 | UiWidgetContext::is_active | hover!=0 && id 匹配（同 hovered） | `cmp byte[rax+0x18],0; jz fail; id 匹配; cmp dword[rax+0x28],2; setne` | ⚠ | 缺 **`[+0x28]!=2`** 门控 |
| 0x14092e6c0 | DragState::is_started | ∈ {Started, Dragging} | `cmp byte[rcx],3; sete al` —— **单一判别==3**（ICF 折叠：同址=GladeType::is_autumn） | ✗ | 两变体成员关系不被机器码支持 |
| 0x14092df30 | WidgetId::from_name | = utils::calculate_hash(name) | 拷 &str 到栈→call 0x1405bdda0（哈希例程，载多组 8B 常量） | ✓ | 语义=对名字哈希 |
| 0x140b596b0 | PhotoModeCameraMode::next_demo | u8 判别 0→1→2→0；3→panic（串 0x142af1188） | decomp：case0→1, case1→2, case2→0, case3→FUN_1428d9390(&UNK_142af1188) panic | ✓ | 完全一致 |
| 0x1409a8ee0 | WidgetParams::occlude_by_world | 字节@0xa8 bit0 | `mov byte[rdx+0xa8], r8b` —— **整字节写**（独立 bool 字段，非 bit0） | ⚠ | 偏移对，"bit0"错（整字节字段） |
| 0x1409a8f10 | WidgetParams::no_history_recording | 字节@0xa9 bit0 | `and byte[rdx+0xa9],0xfe` —— 无条件**清 bit0**（bit0 在 +0xa9 对） | ✓ | 位置对；操作为无条件清除（疑似正向 history_recording 位/名向微偏） |
| 0x1409a8f40 | WidgetParams::allow_negative_distance | 字节@0xa9 bit2 | `movzx al,[rdx+0xa9]; and al,0xfb; shl r8b,2; or r8b,al; mov [rdx+0xa9],r8b` —— RMW 置 bit2 | ✓ | 完全一致 |
| 0x14089dc00 | SignifierEntityBuilder::position | Vec2 @0x7c | `mov [rdx+0x7c],rax(8B); mov [rdx+0x84],eax(4B)` —— 写 **12 字节** @0x7c..0x87 | ⚠ | @0x7c 对，但足迹 12B（Vec3 或 Vec2+extra f32），"Vec2"描述不完整 |
| 0x140942280 | advance_input_replay_frame | Replaying 时 index@+0x48++，帧尽→Finished | `cmp [rsi+0x40],0x8000000000000002; jne; mov rax,[rsi+0x48]; inc; mov [rsi+0x48],rax; cmp rax,[rsi+0x38]; jb` | ✓ | index@+0x48++ 与 len@+0x38 比对证实 |
| 0x1409a9150 | UiFocus::is_toolbar | ==1（u32 判别） | `cmp dword[rcx],1; sete al; ret` | ✓ | 完全一致 |

### audio-misc（`audio/steam/feedback/gpu_lut/color_*/image_formation/shader_*` 归属文件）

| 地址 | 函数 | 章节 O 结论 | 实测 | 判定 | 证据 |
|---|---|---|---|---|---|
| 0x140b06770 | ColorMenuBrightnessValue::to_f32 | 判别式索引 f32 表 0x142ae785c={0.25,1.0,4.0} | `movzx eax,cl; lea rcx,[0x142ae785c]; movss xmm0,[rcx+rax*4]`；表值 **(0.25,1.0,4.0)** | ✓ | 完全一致 |
| 0x140b06780 | ColorMenuBrightnessValue::default_f32 | 常量 1.0（0x142925904）→缺省变体=1 | `movss xmm0,[rip+0x142925904=1.0]` | ✓ | 完全一致 |
| 0x140909860 | FeedbackReportBgTexture::is_available | [+0x36]<7 | `cmp byte[rcx+0x36],7; setb` | ✓ | 完全一致 |
| 0x140909910 | FeedbackReportResources::reset_choices | [+0x40..0x46]={0,1,1,0,0,0} | `mov byte[rcx+0x46],0; mov word[rcx+0x44],0x0101; mov dword[rcx+0x40],0` → **[0,0,0,0,1,1,0]** | ✗ | 字节序/数量错（"1"在 +0x44/0x45 非 +0x41/0x42） |
| 0x1408aea50 | ImageFormationConfigs::current_mut | self+(kind<<6) | `movzx eax,[rcx+0xc0]; shl eax,6; add rcx,rax` | ✓ | 完全一致（kind 来自 +0xc0） |
| 0x1408aea80 | ImageFormationConfigs::get | self+(i<<6)（无边界检查） | `movzx eax,dl; shl eax,6; add rcx,rax` | ✓ | 完全一致 |
| 0x140a57300 | AudioInstance::start_if_not_playing | 判别&1→get_playback_state→位掩码 0xb（{Playing,Sustaining,Starting} 跳过）→start | `test byte[rcx],1; jz; call get_playback_state; mov ecx,0xb; bt ecx,eax; jb skip; call start` | ✓ | 完全一致 |
| 0x140a57470 | AudioInstance::stop | 判别==1→EventInstance::stop(0=STOP_IMMEDIATE) | `cmp dword[rcx],1; jne; lea rdx,[rcx+8]; xor eax,eax; call stop(0)` | ✓ | 完全一致 |
| 0x140b1f210 | ShaderRefactoringState::disabled | fn disabled(&self)->bool（[+0]==0） | `cmp dword[rcx],0; sete` | ✓ | 完全一致 |
| 0x140a53640 | Bank::load | loaded&1 早退；load_bank_file(path@+0x10)；失败 panic | `test byte[rcx],1; jnz 早退; lea rax,[rsi+0x10]; call load_bank_file` | ✓ | 早退/路径@+0x10 证实 |
| 0x14099ca50 | color_id_roof::impl$0::default_value | 返回 1（屋顶默认 ColorId=1） | `mov eax,1; ret` | ✓ | 完全一致（不在 v3，机器码直接证实） |
| 0x140c9cd80 | NameHash::compute | xxh3_64_with_secret(默认 secret, 0xc0, seed=0) | `lea secret→[rsp+0x28]; mov [rsp+0x20],0xc0; xor r8d,r8d(seed=0); call hash` | ✓ | secret+len 0xc0+seed0 证实（不在 v3） |

## 高价值纠正清单（新的事实 / 错位 / 可闭合项）

1. **`closest_segment_index`（0x14094c800）墙段布局错**：ivy grower 的墙段遍历是 **56B 步长**
   （机器码循环 `add r9,0x38`），高度在 **+0x18/+0x1c**（非章节所写 28B / h0@+0xc）。
   直接纠正 `chapters/systems/ivy/ivy_grower.rs.md` 的段布局结论（影响 reconstruction 的
   wall segment 表示）。h1@+0x1c 与"高度≤max_ivy_grow_height 过滤"是对的。
2. **`StairsState::try_set_endpoint`（0x1408fe7b0）阈值错**：Unset 条件是邻边 **≥2**（
   `lea rax,[rax*8-8]; cmp rax,8; jae` 即 `7 < len*8-8`），非章节所写 ≥8。修正 endpoint
   判定逻辑。
3. **`xz_flatten_aabb`（0x1408d98a0）输出错**：实为 **{min.x, min.z, max.x, max.z}**（4×f32，
   从 Aabb3 取 in[0]/in[8]/in[0xc]/in[0x14]），非章节所写 {min.x,min.y,max.y}。
4. **`Sheep::being_petted`（0x14089d4c0）阈值错**：`f32@+0x58 > 0.01`（ucomiss 常量
   0x142960674=0.01），非 0.0。INDEX 与章节均写 ">0"。
5. **`FeedbackReportResources::reset_choices`（0x140909910）字节序错**：机器码写
   `[+0x40..0x46] = {0,0,0,0,1,1,0}`（`dword+0x40=0`、`word+0x44=0x0101`、`byte+0x46=0`），
   章节写 {0,1,1,0,0,0}。两个"1"在 +0x44/+0x45。
6. **`DragState::is_started`（0x14092e6c0）两变体成员关系不被支持**：机器码为单一判别
   `byte[rcx]==3`（sete），与章节"∈{Started,Dragging}"矛盾；该址与 `GladeType::is_autumn`
   ICF 折叠。建议以 machine code 为准重排 DragState 判别值，或标注该谓词实为单值比较。
7. **`UiWidgetContext::hovered`（0x14092e620）hover 条件错**：`hover==1`（非章节所写 !=0），
   且返回值是 `byte@+0x19`（click 字节）。重建代码若写 `hover != 0` 会误含 clicked 态。
8. **`WidgetParams::occlude_by_world`（0x1409a8ee0）整字节字段非 bit0**：`mov byte[+0xa8],r8b`
   整字节写。而 `+0xa9` 确为位标志字节（bit0=no_history_recording 无条件清除、bit2=
   allow_negative_distance RMW）。重建若把 occlude_by_world 建模为位标志需改独立 bool。
9. **`SignifierEntityBuilder::position`（0x14089dc00）足迹 12B**：`mov [rdx+0x7c],rax(8B)` +
   `mov [rdx+0x84],eax(4B)`，写 +0x7c..0x87（Vec3 或 Vec2+extra f32），非仅 Vec2。
10. **`UiWidgetContext::is_active`（0x14092e680）缺门控**：机器码另有 `dword[+0x28]!=2`
    条件，章节"hover!=0 && id 匹配"不完整。
11. **`RaycastGeoSource::subworld` 表基址微调**：`lea` 目标=0x142aa4415（章节/INDEX 写
    0x142aa4418，差 3）。映射值全对，建议统一为 0x142aa4415。
12. **`subscribe_to_changes`（0x1408cdc20）是逐槽包装**：按 byte 参数计算 `slot+0x170`
    （0x390 步长槽内 publisher）后尾跳订阅例程；"25B subscribe"成立，但章节未提参数化。

## 未在抽查内、但同域值得复核的高价值线索

- `Rapier::subworlds` / `flush_colliders` 的 0x390 槽布局与 `+0x188`/+0x388 脏缓冲偏移
  （DIGEST §2.1/§2.4）——本抽查未独立复核，建议下一批纳入。
- ui-input `mode_manager` 早退 hash('esc_menu_canvas') 与 `check_if_can_switch` 尾段分派
  （INDEX 已列未决）——大函数，超出本次小函数抽查范围。
- audio-misc `MusicPlayer::update` 双跳表逐 case 语义（INDEX 未决）——同理未抽查。

## 方法记录

- 地址→函数名核对：34/36 命中 v3 索引；2 条（0x14099ca50、0x140c9cd80）不在 v3，靠机器码
  直接证实行为。
- 机器码常量：全部经 `rne-read.py --hex` 或脚本解 rip-relative 目标读取；being_petted 的
  0.01、to_f32 表 {0.25,1.0,4.0}、default_f32 1.0 均直接读 f32 证实。
- 反编译：复用各域 `decomp/` 既有文件（`review-audit/decomp/` 已存所依赖的 6 份），结论
  独立读代码得出。
- 本章节不修改 reconstruction 源码、不回写章节；修正由主 agent 编排。
