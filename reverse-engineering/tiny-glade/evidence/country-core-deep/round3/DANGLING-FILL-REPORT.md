# round3 悬空引用补齐交叉核对报告（DANGLING-FILL-REPORT）

日期：2026-10-15　范围：`round3/dangling-refs.json` 25 条真悬空引用（章节 O 行引用了不存在的 decomp 文件）
方法：RE-MCP 定点反编译（dbid `tg_dang1`，`dang1-job.json` / `dang1-out.json`，`open/wait/decompile(allow_error)/close`）→ NotFound 用 `rne-read --hex` 原字节手解 → 逐条与章节行 O 结论交叉核对 → 就地更新章节证据列。

## 汇总

- 已补（真实反编译落盘，evidence 改真路径）：**12**
- 手解（Ghidra NotFound，rne-read --hex，evidence 改 "hex 手解"，无需文件）：**13**
- 其中**不一致就地修正**章节行：**2**（`get_name` locale 列表补全；`MainCamera::lens` 基址 +0x108→+0x100）

一致性总评：25 条章节 O 结论的**机制/常量/处理链全部被反编译或机器码证实**；仅 2 处文本级细节不精确（已修），另有 1 处命名证据（`next_stroke_id` 序列化 key）已在章节 实现方式 注明。

## 逐条明细

| # | VA | 章节文件 / 行 | 判定 | 证据路径 | 核对结论 |
|---|---|---|---|---|---|
| 1 | 0x1409f1c90 | chapters/resources/raster_stroke.rs.md L11 | 已补 | terrain-water\decomp\decomp-0x1409f1c90.txt | fmt::Debug 胶水一致；反编译=写结构名 "RasterStroke"(12c) + 字段 "is_additive"(11c)（Debug fmt 结构写出） |
| 2 | 0x1408e6b70 | chapters/resources/terrain_editing.rs.md L14 | 已补 | terrain-water\decomp\decomp-0x1408e6b70.txt | serde Value 序列化一致；字段 key 实证 "strokes"@+8(HashMap) / "next_stroke_id"@+0x38(u32)。章节 "last_stroke_id" 命名 → 原字段应为 `next_stroke_id`（已注 实现方式） |
| 3 | 0x140924900 | chapters/resources/terrain_editing.rs.md L15 | 已补 | terrain-water\decomp\decomp-0x140924900.txt | 一致；counter@+0x38==0 → 函数内 panic（串区 0x142aae908，trap int3）；否则查 HashMap 键 counter−1，缺失→None（lVar7==0→0），命中→&mut 值（元素+8） |
| 4 | 0x140a97880 | chapters/resources/terrain_stroke.rs.md L13 | 已补 | terrain-water\decomp\decomp-0x140a97880.txt | 一致（全覆盖章节判定序列）；len==0→push；len==1 且 dist>0.001（常量 0x3a83126f 实证）→push；len≥2 且 spacing≤dist→push（<0x400=1024，cap-first Vec @0/8/0x10）；否则覆盖 last=morph。grow 走 FUN_140b9f180 |
| 5 | 0x140afbe10 | chapters/systems/mode_manager.rs.md L11 | 已补 | ui-input\decomp\decomp-0x140afbe10.txt | 一致；AppMode serde deserialize visit_str 变体名匹配：All→0、Wall→1、Path→2、Bulldoze→3、Water→4、Garden→5、Tree→6（4 字节/短串 u32+后缀字符比较） |
| 6 | 0x140afa8a0 | chapters/systems/mode_manager.rs.md L16 | 已补 | ui-input\decomp\decomp-0x140afa8a0.txt | 一致（round2 capstone 全解复验）；非 force：*env0=0xd(Replay)、push 0x10B {计数,请求mode}、byte@1>=2 → panic_str(&UNK_142ae5650,0x28,&UNK_142ae5758)="internal error: entered unreachable code"+toolbar.rs:40（0x142ae5758 fat ptr→toolbar.rs）；force：push 0x20B {u32 4@0, 计数@+0x18} |
| 7 | 0x140a66970 | chapters/systems/render_loop/compute_passes/grass.rs.md L14 | 已补 | render-loop\decomp\decomp-0x140a66970.txt | 一致；Vec::push 8B 元素注册（len==cap→grow FUN_140b81df0），bevy register_required_components 派生胶水 |
| 8 | 0x14092df30 | chapters/systems/ui/resources/ui_state.rs.md L11 | 已补 | ui-input\decomp\decomp-0x14092df30.txt | 一致；16B 载荷@+8..0x18 拷贝到栈 → FUN_1405bdda0（=calculate_hash，见 mode_manager 章）→ u64 name-hash 身份 |
| 9 | 0x14092e340 | chapters/systems/ui/resources/ui_state.rs.md L13 | 已补 | ui-input\decomp\decomp-0x14092e340.txt | 一致；5B 尾调转发器 → FUN_14092df50（clearing_canvas_w_layer_and_sound），layer 透传 |
| 10 | 0x1409a8e80 | chapters/systems/ui/resources/widget_params.rs.md L12 | 已补 | ui-input\decomp\decomp-0x1409a8e80.txt | 一致；20 处 u32 写入 @0x18..0x74 + 8B 对 @0x58/0x60/0x68/0x70 → 96B sounds 区 @0x18..0x78，尾调 func_0x142923c00 拷贝 0xb0B |
| 11 | 0x1409a8ee0 | chapters/systems/ui/resources/widget_params.rs.md L17 | 已补 | ui-input\decomp\decomp-0x1409a8ee0.txt | 一致；`mov [rdx+0xa8],param_3` 整字节 bool@0xa8（非 bit0），尾调拷贝 0xb0B |
| 12 | 0x1409a92c0 | chapters/systems/ui_clutter/ui_focus_decoclutter.rs.md L11 | 已补 | ui-input\decomp\decomp-0x1409a92c0.txt | 一致；58KB 反编译，10 参数 ECS 系统（多 Res/ResMut/Query/EventReader），消费 RequestMoveClutter/RequestRotateDecorator/LastClonedClutter（wp20 合同 O 保留） |
| 13 | 0x14083c240 | chapters/resources/daily.rs.md L14 | 手解 + **不一致已修（locale 列表补全）** | hex 手解（无需文件） | `cmp r8,5; jne` kind==5 门实证；5 字节前缀匹配（首 4 字节 u32 + [rdx+4] 第 5 字符）共 **14 个 locale**：en-US/de-DE/es-ES/fr-FR/it-IT/ja-JP/ko-KR/pl-PL/pt-BR/ru-RU/sv-SE/tr-TR/uk-UA/zh-CN → index 1..14，经 index→名字表 {ptr,len} 解析（24B 元素，缺失回落 index 0 默认）；章节原只列 6 个 → 已就地补全（L6/L14/L23） |
| 14 | 0x1408ef180 | chapters/resources/mouse_terrain_raycast.rs.md L11 | 手解 | hex 手解（无需文件） | 56B SIMD：`t=(plane_y−pos.y)/dir.y`（xmm2 高/平面 y，MOVSHDUP 取 pos.y/dir.y），`result=pos+t*dir` 全 3D 交点（mulps/addps/addss，movlps+movss 存 12B）；pos@0/dir@0xc = 资源相对 +4/+0x10（与 mouse_terrain_raycast 0x1408ef320 读法一致） |
| 15 | 0x140a97860 | chapters/resources/raster_stroke.rs.md L12 | 手解 | hex 手解（无需文件） | 16B：`rax=[+8]`=points 切片 ptr、`rcx=[+0x10]`=len、`rdx=ptr+3·len·4`=end → 返回 {ptr,end} 迭代范围（(x,z) 映射在 50 处内联宿主） |
| 16 | 0x1409149d0 | chapters/systems/camera_rig.rs.md L14 | 手解 | hex 手解（无需文件） | 模式@+0x3d0 → lea r9 jumptable → movsxd/add/jmp；6 个 case 各设 {body,表,fov 对} + 模式 6（body=rdx+0x0 矩阵块区）；写结果 {body@0, 表@8, fov_target@0x10, fov_current@0x18} |
| 17 | 0x140911230 | chapters/systems/camera_rig.rs.md L18 | 手解 | hex 手解（无需文件） | 8B getter：`movzx eax,[rcx+0x4d0]; ret` 实证 |
| 18 | 0x140911240 | chapters/systems/camera_rig.rs.md L19 | 手解 | hex 手解（无需文件） | `movzx eax,[rcx+0x4d0]` → `lea rdx,[rip+0x219a91a]`(→0x142aabb68 表) → `mov rax,[rdx+rax*8]` → `add rcx,rax; add rax,0x100` → 表 13 项，条=fov 偏移（0x70/0x128/0x1a8/0x248/0x300/0x388…） |
| 19 | 0x140911260 | chapters/systems/camera_rig.rs.md L20 | 手解 + **不一致已修（基址 +0x108→+0x100）** | hex 手解（无需文件） | 同 focus 形制：`lea rdx,[rip+0x219a92a]`(→0x142aabb98 表) → 表[mode] → `add rax,0x100`；**机器码基址为 +0x100**（章节原文 +0x108 是模式 6 的结果值 self+0x108，非基址）；表[0]=0x8c → 默认 body lens@+0x18c；表条 13 项，槽 6..9=0x8/0x20/0x14/0x2c → 落 Normal body（章节此句正确） |
| 20 | 0x14092e5f0 | chapters/systems/ui/resources/ui_state.rs.md L25 | 手解 | hex 手解（无需文件） | `cmp byte[+0x18],2; jb ret0`（hover>1）、`movzx edx,byte[+0x19]; cmp dl,1; ja ret0`（click<2）、`test dl,1; jz ret0`（click&1），尾 `mov rcx,[self+8]; mov rcx,[ctx]; cmp [ctx+0x38],rcx; sete al`（id 匹配尾，章节未列但机制一致） |
| 21 | 0x14092e620 | chapters/systems/ui/resources/ui_state.rs.md L26 | 手解 | hex 手解（无需文件） | `cmp byte[+0x18],1; jne ret0`（hover==1）、`mov rdx,[+0x38]; cmp rdx,[self+8]; jne ret0`（id 匹配）、`movzx eax,byte[+0x19]` 返 click 字节 |
| 22 | 0x14092e640 | chapters/systems/ui/resources/ui_state.rs.md L29 | 手解 | hex 手解（无需文件） | `cmp byte[+0x18],0; jz` 或 id 不匹配 → `mov dword[rax],2`（tag-2 None）；否则 7 qword 拷贝 @+0x28..0x58 → +0..0x30（3×movups+1×mov） |
| 23 | 0x14092e680 | chapters/systems/ui/resources/ui_state.rs.md L30 | 手解 | hex 手解（无需文件） | `cmp byte[+0x18],0; jz`（hover!=0）、`cmp [ctx+0x38],[self+8]; jne`（id 匹配）、`cmp dword[+0x28],2; setne al`（dword@+0x28 != 2） |
| 24 | 0x1409a8e10 | chapters/systems/ui/resources/widget_params.rs.md L11 | 手解 | hex 手解（无需文件） | 默认序列实证：u32@0=0、@8=8、@0x10=0、@0x18=1、@0x20=0、@0x30=2、@0x48=3、@0x60=3、u16@0xa8=0x101；默认数据拷贝 @0x78..0xa0（movups 3 组） |
| 25 | 0x1409a9110 | chapters/systems/ui/resources/widget_params.rs.md L26 | 手解 | hex 手解（无需文件） | `mov [rax+1],dl`（layer@+1）、`mov rcx,0x4302000002430000`→[+4]（双 f32 130.0×130.0）、`mov byte[rax],4`（tag=4）→ {tag 4, layer, 130×130} |

## 说明与边界

- 13 个 NotFound 函数均为 Ghidra 未建函数的小函数（8B–410B）；PLAYBOOK §1.3 规则适用（rne-read --hex 手解，不落盘文件）。
- 章节行就地更新均为 evidence 列改写（+必要的 逆向结论/实现方式 修正），未改 reconstruction 源码。
- 命名证据：`terrain_editing.rs` 的 Value 序列化 key 为 "strokes"/"next_stroke_id"，与章节 "last_stroke_id" 不一致 → 已注 实现方式（后续合同/重建应改用 `next_stroke_id`）。
- `ui_focus_decoclutter` 反编译 58KB，仅做结构与规模核对（10 参数 ECS、事件消费族），语义细节沿用既有 wp20 合同 O。
