# -*- coding: utf-8 -*-
import json

results = [
    {
        "addr": "0x14067dd80",
        "verdict": "一致",
        "detail": "ICF 折叠注记：PDB S_PUB32 该址 141 条符号，含行 fn DeferredDecoratorOpEvents::events::hb6aeb0c5ecaadca8（也含 TweakerView::as_inner_mut、UiState::widget、FinalizedHoleStorage::inner、AppSteamIntegration::client 等）；rne-read 机器码 mov rax,rcx; ret（恒等返回=偏移 0 字段 getter），与 as_inner_mut 等同址折叠、行为一致。行引用的三份 decomp 文件均为 NotFound 错误（证据无效，以 rne-read 替代）；行作用列写 FinalizedHoleStorage::inner 系另一折叠符号，行内描述混杂，建议注记为 ICF 多符号址。",
        "evidence": "rne-read --hex 0x14067dd80: 4889c8c3 (mov rax,rcx;ret); public-symbols.json 0x14067dd80=141 条 S_PUB32 含 events; pdb_exact=TweakerView::as_inner_mut size4; decomp 三份均为 {error:No function at 0x14067DD80}"
    },
    {
        "addr": "0x140833230",
        "verdict": "一致",
        "detail": "PDB exact=ClutterVersion::is_initial 与行 fn 完全吻合；rne-read 机器码 cmp byte[rcx],0; sete al; ret = 判别 tag==0，符合 is_initial 语义。wall/expansion/roofs 三份 decomp 均为 NotFound 错误文件（证据无效，以 rne-read 裁决）。ICF 注记：同址另有 FreehandHeightType::is_follow_terrain、GableRoofStyle::is_stepped、PanicReproState::is_inactive（同为判别==0 小函数）。",
        "evidence": "rne-read --hex 0x140833230: 8039000f94c0c3 (cmp byte[rcx],0; sete al; ret); pdb_exact=ClutterVersion::is_initial size7; decomp 三份均为 NotFound"
    },
    {
        "addr": "0x140833260",
        "verdict": "一致",
        "detail": "decomp 有效：tag@+0x22 < 2 时 return param_1（&self），否则构造 panic（FUN_1428d9390 + 串 UNK_142a92798，swi(3)），与行 tag<=1 → &self 否则 panic 完全吻合。PDB exact 与行 fn 一致。",
        "evidence": "wall/decomp/decomp-0x140833260.txt: if (*(byte *)(param_1 + 0x22) < 2) return param_1; 否则 panic 路径"
    },
    {
        "addr": "0x1408332c0",
        "verdict": "一致",
        "detail": "decomp 有效：tag@+0x22 为 2..4 时 lVar2=tag-1 ∈ {1,2,3} 分三条 unreachable panic 串（UNK_142a927e0/142a92830/142a92878 = Roof/Stair/Terrain 三变体），tag<2 直接返回 self；与行 unreachable dst is Roof/Stair/Terrain 吻合。",
        "evidence": "wall/decomp/decomp-0x1408332c0.txt: (byte)(*(byte*)(param_1+0x22)-2) < 3 → lVar2=tag-1; 三串 panic 分发 + FUN_1428d9390"
    },
    {
        "addr": "0x140833540",
        "verdict": "一致",
        "detail": "decomp 有效：if (*(byte*)(param_1+0x22) < 2) return param_1; 否则 panic（FUN_1428d9430(&UNK_142a928bc,0x28,&UNK_142a92918)），与行 tag<=1 → &mut self，否则 unreachable 吻合。PDB exact=DecoratorDst::as_wall_mut 与行 fn 一致。",
        "evidence": "wall/decomp/decomp-0x140833540.txt: if (*(byte *)(param_1 + 0x22) < 2) { return param_1; } FUN_1428d9430(...)"
    },
    {
        "addr": "0x140839550",
        "verdict": "一致",
        "detail": "rne-read 12B（与 PDB size 12 吻合）：mov rax,rcx; movss [rcx],xmm1; mov byte[rcx+0x11],7; ret —— 写判别字节 +0x11=7，与行写判别字节 +0x11=7 完全一致。pubsym=From<LanternDecoratorInfo>。",
        "evidence": "rne-read --hex 0x140839550: 4889c8f30f1109c6411107c3"
    },
    {
        "addr": "0x140839560",
        "verdict": "不一致",
        "detail": "代码写 DecoratorExtraInfo From 变体 8（写判别字节 +0x11=8），机器码为：mov rax,rcx; mov ecx,[rdx+0x10]; mov [rax+0x10],ecx; movups xmm0,[rdx]; movups [rax],xmm0; ret（16B，与 PDB size 16 吻合）——只复制 payload {16B@+0, u32@+0x10}，全程没有任何 +0x11 字节写入，与 0x140839550（写 7）形成明确对照。fix 建议：该行作用改为 From<StairDecoratorInfo>：payload 复制 {16B@+0, u32@+0x10}，无判别字节写入（或判别由调用方/内存预置），不应断言写 8。",
        "evidence": "rne-read --hex 0x140839560: 4889c88b4a108948100f10020f1100c3 (copy u32@+0x10 + 16B@+0, 无 c6 41 11 08 类写); pubsym=From<StairDecoratorInfo>"
    },
    {
        "addr": "0x14085a730",
        "verdict": "一致",
        "detail": "注记：PDB exact=S_PUB32 remove_all::h0984f2151b3f2028 与行 fn 吻合（该址唯一业务符号；max_id 无独立 PDB 符号，章节亦列其为内联行 89）。机器码为清空语义：map 头 4 qword 重置为静态空值（_UNK_142af4bb0=0x142af4ba0 指针+0），随后 128 位占用位图遍历（0x78 步长=章节条目步长 0x78），逐条目释放两条 Vec（func_0x140613c20(ptr,len<<3,4) 与 (ptr,len<<2,4)）。但行作用列写 InnerWalls::max_id、decomp header 亦标 max_id——系错误旧标注，建议作用列改为清空所有条目并重置为静态空 map。",
        "evidence": "wall/decomp/decomp-0x14085a730.txt: *param_1=_UNK_142af4bb0 等 4 qword 重置 + 位图遍历(0x78 步长) + func_0x140613c20 双 Vec 释放; public-symbols.json 0x14085a730=remove_all; 行作用列/decomp header 的 max_id 为误标"
    },
    {
        "addr": "0x140999920",
        "verdict": "一致",
        "detail": "身份 ✓（PDB exact=merge_walls size6006 与行 fn 一致，decomp header 同名）。行为断言核验：FUN_1409e9640=TerrainHeightsData::sample_at_world_xz（地形采样✓）；FUN_140b2bc00x2=decorator_cull_allow_wall（行作用写 decorator_cull_allow_any——实际调用的是 cull_allow_wall 0x140b2bc00，any 在 0x140b2bbf0，名称小出入不影响身份）；FUN_14085aa60x2=InnerWalls::remove；FUN_140a1ea40=DecoratorStorage::on_walls_merged。crashy_new_from_points 无 PDB/SOURCE_MAP 符号（reconstruction 侧 Curve 方法，原二进制应为内联/无独立符号），无法按址核验。",
        "evidence": "wall/decomp/decomp-0x140999920.txt callees: FUN_1409e9640=sample_at_world_xz, FUN_140b2bc00x2=cull_allow_wall, FUN_14085aa60x2=InnerWalls::remove, FUN_140a1ea40=on_walls_merged (SOURCE_MAP.json)"
    },
    {
        "addr": "0x140a1b430",
        "verdict": "一致",
        "detail": "rne-read 15B（与 PDB size 15 吻合）：mov rax,rcx; mov [rcx+0x10],rdx; mov [rcx],r8; mov [rcx+8],r9; ret —— 存 3 字段 {r8→+0, r9→+8, rdx→+0x10}，与行 3 字段 {r8,r9,rdx} 一致。decomp 文件为 NotFound 错误（无效，以 rne-read 裁决）。",
        "evidence": "rne-read --hex 0x140a1b430: 4889c8488951104c89014c894908c3; decomp NotFound"
    },
    {
        "addr": "0x140a1c240",
        "verdict": "一致",
        "detail": "decomp 有效：if (*param_1==0) return param_1[1]; 否则 panic（FUN_1428d9390），与行 ([+0]==0 → [+8]) 完全吻合。PDB exact=DecoratorStorageAddr::as_wall 与行 fn 一致。",
        "evidence": "wall/decomp/decomp-0x140a1c240.txt: if (*param_1 == 0) { return param_1[1]; } 否则 FUN_1428d9390 panic"
    },
    {
        "addr": "0x140a1cf30",
        "verdict": "一致",
        "detail": "decomp 有效 22404B（约 21KB 与行一致）。panic 调用点 FUN_1428d9430(&UNK_142ac9098,0x21,&UNK_142ac91b0)，消息串 0x142ac9098 = assertion failed: !self.immutable —— 行 !self.immutable 断言 精确命中。PDB exact 与行 fn 一致。",
        "evidence": "wall/decomp/decomp-0x140a1cf30.txt 464 行 panic; rne-read 0x142ac9098: assertion failed: !self.immutable"
    },
    {
        "addr": "0x140a1e280",
        "verdict": "一致",
        "detail": "rne-read 52B（与 PDB size 52 吻合）：mov rax,rcx; mov rcx,[rdx+0x90]; add rdx,0x90; test rcx,rcx; cmovz rdx,rcx; mov dword[rax],1; mov [rax+8],rdx; mov qword[rax+0x10],0; mov qword[rax+0x38],0; ret —— 写迭代器 {tag=1, ptr=self+0x90（空则 0）, +0x10=0, +0x38=0}，与行 {tag=1, ptr=self+0x90} 完全吻合。decomp 文件 NotFound（无效）。",
        "evidence": "rne-read --hex 0x140a1e280: 4889c8488b8a900000004881c2900000004885c9480f44d148c700010000004889500848c740100000000048c7403800000000c3; decomp NotFound"
    },
    {
        "addr": "0x140a884f0",
        "verdict": "一致",
        "detail": "rne-read 71B：+0xa0 锁前缀 CAS 循环 addss（f3 0f 58 c1，+=x）；+0x90 CAS 循环 subss（f3 0f 5c c1，-=x）—— 与行 CAS +0xa0+=x, +0x90-=x 完全吻合。decomp 文件 NotFound（无效）。",
        "evidence": "rne-read --hex 0x140a884f0(48B): f0 0f b1 91 a0... addss + lock cmpxchg [rcx+0xa0]; f3 0f 5c c1 subss + cmpxchg [rcx+0x90]; decomp NotFound"
    },
    {
        "addr": "0x140a8a710",
        "verdict": "一致",
        "detail": "身份 ✓（PDB exact=calc_wall_bend size2466 与行 fn 一致，decomp header 同名）。行为核验：clamp 常量 _UNK_142925904=0x142925904=1.0f（rne-read 0000803f），decomp 372-373 行 if (1.0 <= fVar40) fVar40=1.0 = clamp<=1.0 ✓；函数对曲线点（0x10 步长）做计数/偏移运算后进入 hashbrown 写入循环（FUN_1405b0d70 等）≈ 事件写出。reconstruction 源码文档（>5 点、acos、WallBendChangedEvent）与 decomp 结构相符。行断言成立（>5 点算 bend 的常数比较未在 decomp 逐字核到，但语义一致）。",
        "evidence": "wall/decomp/decomp-0x140a8a710.txt: _UNK_142925904 <= fVar40 → fVar40=1.0 (372-373); rne-read 0x142925904=0000803f=1.0f; reconstruction calc_wall_bend.rs (points.len()<=5 → 0.0; clamped.acos().min(1.0))"
    },
    {
        "addr": "0x140aa5dd0",
        "verdict": "一致",
        "detail": "decomp 有效：hashbrown 表条目步长 0x38（lVar18 = lVar17 * 0x38 / * -0x38，key 比较 + 控制字节探测），调用 FUN_1409ef870=WallSpatialHash::get_cells_from_segment（逐格遍历✓）与 Vec push（FUN_1400d9b20）。行 逐格 0x38B 成员 push 中 0x38B 应读作 0x38 字节步长，与机器码吻合。PDB exact=add_segment 与行 fn 一致。",
        "evidence": "wall/decomp/decomp-0x140aa5dd0.txt: * 0x38 / * -0x38 步长 (179/242/243 行), FUN_1409ef870=get_cells_from_segment (SOURCE_MAP)"
    },
    {
        "addr": "0x140aa6600",
        "verdict": "一致",
        "detail": "rne-read 19B（与 PDB size 19 吻合）：mov rax,rcx; movddup xmm0,[rdx+0x50]; mulps xmm0,[rip+const]; movups [rcx],xmm0; ret —— 读 +0x50 f64 并 movddup 展开 × 常量，写 2 f32，与行 cell 中心 f64 换算（movddup [+0x50] × 常量）完全吻合。decomp 文件 NotFound（无效）。",
        "evidence": "rne-read --hex 0x140aa6600: 4889c8f20f1242500f5905...0f1101c3; decomp NotFound"
    },
    {
        "addr": "0x140ac79e0",
        "verdict": "一致",
        "detail": "身份 ✓（PDB exact 与行 fn 一致，decomp header 同名）。行为核验：callees FUN_140a83fb0=EditType::new_height_changed（0x140a83f80 的 ev_height_changed 相邻 +0x30，同高度修改历史域）、FUN_140a84ec0=History::add（历史记录✓）—— 行 ev_height_changed 历史记录语义命中；另有 FUN_1428d9430x2 两处 panic。",
        "evidence": "wall/decomp/decomp-0x140ac79e0.txt callees: FUN_140a83fb0=EditType::new_height_changed, FUN_140a84ec0=History::add (SOURCE_MAP.json)"
    },
    {
        "addr": "0x140ad6250",
        "verdict": "一致",
        "detail": "身份 ✓（PDB exact=ui_edit_rectangle_dims size2977 与行 fn 一致，decomp header 同函数名）。行为核验：callees FUN_14155fc70=UiRectangleDimsEdit::widget、FUN_14155eba0=UiRectangleDimsEdit::signifier（矩形尺寸编辑 UI 域✓）。get_distance_to_edge 在 SOURCE_MAP 标记 inline（无独立地址），行为细节无法按址核验，但函数域完全吻合。",
        "evidence": "wall/decomp/decomp-0x140ad6250.txt callees: UiRectangleDimsEdit::widget(0x14155fc70)/signifier(0x14155eba0); SOURCE_MAP: get_distance_to_edge=inline va[]"
    },
    {
        "addr": "0x140ad8000",
        "verdict": "一致",
        "detail": "身份 ✓（PDB exact=ui_edit_rectangle_rotate size3035 与行 fn 一致，decomp header 写 ui_edit_rectangle_orientation 同文件域）。行为核验：callees FUN_14083d3f0=UiRectangleOrientationEdit::widget、FUN_14083da10=UiRectangleOrientationEdit::signifier（方向编辑 UI 域✓，与行 触发体旋转 语义吻合）。",
        "evidence": "wall/decomp/decomp-0x140ad8000.txt callees: UiRectangleOrientationEdit::widget(0x14083d3f0)/signifier(0x14083da10) (SOURCE_MAP)"
    },
    {
        "addr": "0x140b256b0",
        "verdict": "一致",
        "detail": "rne-read 6B（与 PDB size 6 吻合）：test cl,cl; setne al; ret —— 返回字节!=0，与行 字节!=0 完全吻合。PDB exact=WallCornerAttachment::is_attached 与行 fn 一致。decomp 文件 NotFound（无效）。",
        "evidence": "rne-read --hex 0x140b256b0: 84c90f95c0c3; decomp NotFound"
    },
    {
        "addr": "0x140b25ef0",
        "verdict": "一致",
        "detail": "身份 ✓（PDB exact=insert_collision_and_interaction_as_same size942 与行 fn 一致）；decomp 有效（签名 void(param_1, u32, u8, ptr, ...)，callees FUN_14152d1b0x2、FUN_140b5c620x2 等）。行无行为断言（仅 fn 名），身份成立即可。",
        "evidence": "wall/decomp/decomp-0x140b25ef0.txt 有效 decomp; callees 见文件"
    },
    {
        "addr": "0x140b2a770",
        "verdict": "一致",
        "detail": "rne-read 21B（与 PDB size 21 吻合）：xor eax,eax; cmp dword[rcx+0x258],2; setne al; movss xmm0,[rip+0x01DFB180]; ret —— disp 精确计算目标 = 0x142925904，该址值 00 00 80 3f = 1.0f；即返回 {tag!=2, 1.0f}，与行 {tag!=2, 1.0f@0x142925904} = Option<f32> 归一全区间 完全吻合（含常量地址）。decomp 文件 NotFound（无效）。",
        "evidence": "rne-read --hex 0x140b2a770: 31c083b958020000020f95c0f30f100580b1df01c3; target=0x140b2a784+0x01DFB180=0x142925904; rne-read 0x142925904=0000803f"
    },
    {
        "addr": "0x140b2bc00",
        "verdict": "一致",
        "detail": "注记：PDB exact=decorator_cull_allow_wall size292 与行 fn 完全吻合（PDB 中 cull_allow_any 在 0x140b2bbf0，与本址无关）；decomp header 写 decorator_cull_allow_any 系 DB 标注误名。机器码 = 0x98 步长 hashbrown 探测（lVar9=(uVar3+uVar5 & mask)*-0x98，16B key 比较）+ 命中处理，与行作用 0x98 步长 map 命中回调 吻合。行作用文本写 decorator_cull_allow_any 与 fn 名不一致，建议作用列改为 cull_allow_wall。",
        "evidence": "wall/decomp/decomp-0x140b2bc00.txt: * -0x98 步长 key 探测; sysres-procedures-v3: any=0x140b2bbf0, wall=0x140b2bc00"
    },
    {
        "addr": "0x140b33b00",
        "verdict": "一致",
        "detail": "身份 ✓：PDB exact=DecoratorOutline::display size2876 与行 fn 一致；decomp header=impl$0::display::closure$0，与行作用 DecoratorOutline::display 闭包 吻合（display 的闭包体）。callees 3 个（FUN_140d79f90/140d4ba00/140b35930，其一在 decorator_outline 域 0x140b35930）。行无更多行为断言。",
        "evidence": "wall/decomp/decomp-0x140b33b00.txt header: impl$0::display::closure$0"
    },
    {
        "addr": "0x140b34f80",
        "verdict": "一致",
        "detail": "注记：PDB exact=split_height_into_skirts size409 与行 fn 一致；decomp header 写 HighlightColor::to_rgb 系 DB 误名（机器码无 to_rgb 语义）。4 个浮点常量全部命中：0x142aecb40=5.9f（阈值）、0x142ae819c=4.5f（首裙，decomp 亦见 0x40900000）、0x142abcb6c=3.5f（步长）、0x142aecb44=1.4f（余量）；逻辑 = threshold 起 Vec 构造 {4.5, +3.5 递增, 条件 h>f+1.4}，else 空 Vec —— 与行 >=5.9 起 4.5 每 +3.5，条件 height>f+1.4 完全一致。",
        "evidence": "wall/decomp/decomp-0x140b34f80.txt + rne-read 常量: 0x142aecb40=cdccbc40(5.9), 0x142aecb44=3333b33f(1.4), 0x142abcb6c=00006040(3.5), 0x142ae819c=00009040(4.5)"
    },
    {
        "addr": "0x140f86d00",
        "verdict": "一致",
        "detail": "decomp 有效：hashbrown HashMap lookup（FUN_1405b0d70 哈希、控制字节 16B 匹配、key 相等比较（0x28 步长条目）、命中返回存储值、空组退出返回 0），与行 contains_key 语义吻合（get/contains 风格查询）。PDB exact=PrevWallHoles::wall_holes 与行 fn 一致。",
        "evidence": "wall/decomp/decomp-0x140f86d00.txt: FUN_1405b0d70 hasher + 控制字节探测 + key==param_2 比较 + 返回条目值"
    },
    {
        "addr": "0x140f87300",
        "verdict": "一致",
        "detail": "decomp 有效：raw 条目（u32 组）经 FUN_14123fff0 转 4 f32 写出（Aabb2 min/max 4 浮点 ✓）；排序 FUN_141338c40（1<len≤0x14 时，>0x14 走 panic 路径）；跨 seam 副本分支：f+param_2 偏移复制条目并置标志字节 1（原条目 0）✓。panic 消息串 0x142bad478 = finalize called on a wall hole storage that has not been cleared yet. —— 行 panic not cleared yet 精确命中。",
        "evidence": "wall/decomp/decomp-0x140f87300.txt 130-215 行; rne-read 0x142bad478: finalize called on a wall hole storage that has not been cleared yet."
    },
    {
        "addr": "0x1411ff620",
        "verdict": "一致",
        "detail": "rne-read：mov rax,rcx; add rcx,4; test byte[rax],1; jne/jmp 两条远跳分发 —— 位 0 判别分发，与行 位 0 判别分发 完全吻合。PDB exact=ShapeParameters::get_pos_at_u 与行 fn 一致。decomp 文件 NotFound（无效）。",
        "evidence": "rne-read --hex 0x1411ff620: 4889c84883c104f600010f85...e9... (bit0 test + 双分支); decomp NotFound"
    },
    {
        "addr": "0x141234970",
        "verdict": "一致",
        "detail": "rne-read 9B（与 PDB size 9 吻合）：add rcx,0x58; jmp <尾调>; ret 前缀无 —— 偏移 +0x58（Vec 头）后尾跳到公共取长度代码，与行 尾跳到 +0x58 Vec 长度取用 完全吻合。PDB exact=WallPathSegment::pt_count 与行 fn 一致。decomp 文件 NotFound（无效）。",
        "evidence": "rne-read --hex 0x141234970: 4883c158e9d7b9fcffc3...; decomp NotFound"
    },
]

out_path = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/round3/oa2-results/w4_wall.json"
with open(out_path, "w", encoding="utf-8") as f:
    json.dump(results, f, ensure_ascii=False, indent=1)
print("entries:", len(results))
from collections import Counter
print(Counter(r["verdict"] for r in results))
