#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""w8 worker results writer."""
import json, io

results = [
    {
        "addr": "0x140002370",
        "verdict": "一致",
        "detail": "识别为依赖成立：PDB 精确符号 bevy_ecs schedule_system::impl$1::is_exclusive<FunctionSystem<...>> 与行内 fn 吻合；机器码 31 c0 c3 = xor eax,eax; ret，is_exclusive 返回 false，与非独占 FunctionSystem 一致。ICF 折叠区：S_PUB32 该址大量 is_exclusive/SystemSet::is_anonymous/clone_behavior 符号折叠（pdb_pubsym 另见 UiNav::clone_behavior 同址），多符号贴同一地址以机器码+PDB 唯一符号为准。注意：行内引用的 decomp-0x140002370.txt（audio-misc/roofs-rt 等 8 份）全部是 {\"error\":\"No function at 0x140002370\"} NotFound 错误文件，evid 链有缺陷，但识别结论由 PDB+机器码独立成立（round2 标准：先查 S_PUB32 含行内 fn 符号且机器码吻合→一致，ICF 折叠注记）。",
        "evidence": "rne-read --hex 0x140002370 16 = 31c0c3cccc...；PDB pdb_exact/pdb_owner 精确吻合；8 份 decomp 文件均为 NotFound error 内容（size 81）",
    },
    {
        "addr": "0x1408aa720",
        "verdict": "一致",
        "detail": "startup_vermeer 身份与行为吻合：PDB 精确符号 country_core::systems::render_loop::vermeer::startup_vermeer（127B）吻合；反编译显示 FUN_140b3fe40 调用两次，串解析确认 /vermeer/raster_strokes.raster.hlsl + /vermeer/combine.raster.hlsl 及入口名 vermeer_raster_strokes/vermeer_combine，与行内'load_shader_into_library ×2'一致。",
        "evidence": "decomp-0x1408aa720.txt（有效 418B）；rne-read --unks：UNK_142a9bd74/UNK_142a9bd97 = vermeer shader 路径串、UNK_142a9bcd8/UNK_142a9bd38 = 入口名",
    },
    {
        "addr": "0x1408c9a30",
        "verdict": "一致",
        "detail": "render_menu_bg 身份与行为吻合：PDB 精确符号吻合（1927B）；反编译串解析确认 'Menu bg' + '/gauss_blur.cs.hlsl:cs_main' + menu_background.rs 源码路径（fat 串），与行内'串证据 Menu bg + /gauss_blur.cs.hlsl:cs_main（背景高斯模糊 pass）'一致。",
        "evidence": "decomp-0x1408c9a30.txt（有效 9887B）；--unks：UNK_142aa3210='Menu bg/gauss_blur.cs.hlsl:cs_main...'、UNK_142aa3198/31b0/31c8/31f8=menu_background.rs",
    },
    {
        "addr": "0x14090d850",
        "verdict": "一致",
        "detail": "resolve_to_swapchain 身份与行为吻合：PDB 精确符号吻合（335B）；三个断言串全部解析确认：'Resolve to swapchain'（UNK_142aab180，作 RasterPass 名）、'raster_resolve_to_swapchain' 与 'raster_resolve_msaa_to_swapchain'（UNK_142aab112 含两串连排、UNK_142aab132），且反编译中 cVar1 != '\\x01' 分支按 bVar7 选择两串 = 行内 MSAA 档切换逻辑；另有 final_post.rs / mod.rs 源码路径。",
        "evidence": "decomp-0x14090d850.txt（有效 1609B）；--unks 解析 4 个 UNK 全部命中断言串",
    },
    {
        "addr": "0x14098bf60",
        "verdict": "一致",
        "detail": "inclusive_prefix_scan_u32_1m 身份与行为吻合：PDB 精确符号吻合（1811B）；三个断言串全部解析确认：'_Prefix merge'、'/prefix_scan/inclusive_prefix_scan_merge.cs.hlsl'、'/prefix_scan/inclusive_prefix_scan_subtotals.cs.hlsl'（UNK_142ab90bc/90f0/90fd），另有 prefix_scan.rs / read_terrain_data.rs 源码路径，与行内串断言完全一致。",
        "evidence": "decomp-0x14098bf60.txt（有效 9791B）；--unks：UNK_142ab90bc/90f0/90fd 命中全部三串",
    },
    {
        "addr": "0x1409c40b0",
        "verdict": "一致",
        "detail": "post_taa_transparency_pass 身份与行为吻合：PDB 精确符号吻合（945B）；串 'Post-TAA transparency' 解析确认（UNK_142abfeda），另有 common_draw_params.rs 源码路径，与行内'串证据：Post-TAA transparency'一致；decomp 有效非 NotFound。",
        "evidence": "decomp-0x1409c40b0.txt（有效 4249B）；--unks：UNK_142abfeda='Post-TAA transparency...'、UNK_142aa5bd8=common_draw_params.rs",
    },
    {
        "addr": "0x1409ccb20",
        "verdict": "一致",
        "detail": "wireframe_pass 身份吻合：PDB 精确符号吻合（929B）；decomp 有效，含 common_draw_params.rs 源码路径（CommonDrawParams 构造）与 pass 构造骨架（func_0x000140bd5790 随机/ID 生成 + 大栈帧状态组装，与 resolve_to_swapchain 同构），未发现 wireframe 专属串（pass 名串应在 draw_lists 着色器表）。注意：行内'set_polygon_mode/set_depth_test_enable'细节出自 wp5 draw-passes §10，本次未在 callee 级独立重验，但无矛盾。",
        "evidence": "decomp-0x1409ccb20.txt（有效 4222B）；--unks：common_draw_params.rs；callees 含 FUN_140868370/140867970/140905be0/14090b460",
    },
    {
        "addr": "0x1409daaa0",
        "verdict": "一致",
        "detail": "object_grass_push_pass 身份与行为吻合：PDB 精确符号吻合（3955B）；断言串全部解析确认：'Grass object push pass'（UNK_142ac2bc8）、'grass_object_push'（UNK_142ac2b38），另有 puffin 帧名 '...object_grass_push_pass::f'（UNK_142ac1f20）与 object_grass_push.rs 多份源码路径，与行内串断言一致。",
        "evidence": "decomp-0x1409daaa0.txt（有效 24118B）；--unks 命中两断言串 + puffin 帧名 + 源码路径",
    },
    {
        "addr": "0x140a0d190",
        "verdict": "一致",
        "detail": "calculate_reprojection_map 身份与行为吻合：PDB 精确符号吻合（2410B）；四个断言串全部解析确认：'Reprojection map'、'/calculate_reprojection_map.cs.hlsl'、'Copy depth'、'/copy_depth_to_r.cs.hlsl'（UNK_142ac6f78 起四个连排串），与行内串断言一致。",
        "evidence": "decomp-0x140a0d190.txt（有效 9735B）；--unks：UNK_142ac6f78/6f88/6f98/6fbb/6fc5 命中全部四串",
    },
    {
        "addr": "0x140a32770",
        "verdict": "一致",
        "detail": "rasterize_buildings 身份与行为吻合：PDB 精确符号吻合（4791B）；五个断言串全部解析确认：'Animate buildings mask'、'/rasterize_buildings_animate.cs.hlsl'、'Blur buildings mask'、'/gauss_blur_building_mask.cs.hlsl'、'buildings blur temp'（UNK_142acc410 起连排），另见 'Rasterize buildings/rasterize_buildings.cs.hlsl' 与 ssbo_library.rs 路径，与行内串断言一致。",
        "evidence": "decomp-0x140a32770.txt（有效 23891B）；--unks 命中全部五串 + Rasterize buildings 串",
    },
    {
        "addr": "0x140a39e70",
        "verdict": "一致",
        "detail": "apply_screen_shadows 身份与行为吻合：PDB 精确符号吻合（2163B）；四个断言串全部解析确认：'Screen shadows temporal'（UNK_142acd208）、'/apply_screen_shadows.cs.hlsl'、'Shadow mask mean'、'/shadow_mask_mean.cs.hlsl'（UNK_142acd2a4），另有 apply_screen_shadows.rs / generate_mips.rs 路径，与行内串断言一致。",
        "evidence": "decomp-0x140a39e70.txt（有效 8752B）；--unks 命中全部四串",
    },
    {
        "addr": "0x140a3ba60",
        "verdict": "一致",
        "detail": "calculate_tgsr 身份吻合：PDB 精确符号吻合（6211B）；decomp 有效，串解析仅见 taa.rs 两份源码路径 + unwrap 串，未见 '/tgsr.cs.hlsl:cs_main' 等串——着色器名串应位于 shader 反射表（同 bin_lights 先例'串仅 \\x01/\\x08'），行内 /tgsr.cs.hlsl 断言出自 wp5 compute-passes §3，本次未在 decomp 串层独立复现，但身份与源码归属无矛盾。",
        "evidence": "decomp-0x140a3ba60.txt（有效 22348B）；--unks：UNK_142acd4a0/4c0=taa.rs",
    },
    {
        "addr": "0x140a5a570",
        "verdict": "一致",
        "detail": "calc_dof 身份与行为吻合：PDB 精确符号吻合（12099B）；六个断言串全部解析确认：'Depth of field'、'/dof/fused.cs.hlsl'、'CoC dilated'、'DoF tile offsets'、'DoF tile coords'、'DoF dispatch args'（UNK_142ad31f8 起连排），另有 'dof output' 与 dof.rs 路径，与行内串断言一致。",
        "evidence": "decomp-0x140a5a570.txt（有效 9668B）；--unks 命中全部六串 + 'dof output'",
    },
    {
        "addr": "0x140a66970",
        "verdict": "一致",
        "detail": "register_required_components（胶水，按规则不计业务）身份与行为吻合：PDB 精确符号 grass::impl$4::register_required_components 吻合（110B）；行内引用的 decomp-0x140a66970.txt 不存在（chunk 中 decomp_files 为空）——evid 链缺陷，改由 rne-read 反汇编直接证实：0x140a66970-0x140a669dd 为典型 Vec push 胶水（读 {ptr,len,cap}、call 0x140a5fc10 构造组件值、满时 call 0x140b81df0 扩容、[ptr+len*8]=值、len+1 写回 +0x10），与行内'Vec::push/pop'一致（仅 push 无 pop，行内描述略宽）。",
        "evidence": "rne-read --hex 0x140a66970 110 + capstone 反汇编（decomp 文件缺失，机器码直接证实）",
    },
    {
        "addr": "0x140a8a090",
        "verdict": "一致",
        "detail": "leaf_particle_draw（分派目标）身份与行为吻合：PDB 精确符号吻合（628B）；decomp 有效：World 资源查找（FUN_1400dd6f0 hash 0x3dce36237fb50dff，与 resolve_to_swapchain 同构）+ FUN_140908610 两次 draw 调用 + 资源缺失路径 panic（FUN_1428d9310 指向 leaf_particle_draw.rs），与行内'按名分派目标 + DrawBuilder draw_instanced'一致；未直接见 0x1409ba540 调用（分派目标 = 被调用方，符合'O（分派目标）'语义）。",
        "evidence": "decomp-0x140a8a090.txt（有效 1684B）；--unks：UNK_142ad9080=leaf_particle_draw.rs",
    },
    {
        "addr": "0x14089d500",
        "verdict": "一致",
        "detail": "Affine3Packed::from_cols 手解成立：PDB 精确符号吻合（72B）；行内引用的 decomp-0x14089d500.txt 为 NotFound 错误文件（tg_rt1 未建函数），实际证据为 rne-read 原字节手解——本次独立复核：capstone 反汇编 + 自写 SSE 模拟器确认 4 列指针参数（rdx=x、r8=y、r9=z、[rsp+0x28]=w，12B/列）经 movlhps/shufps(0xd4,0x28)/unpcklps/movhps 重排后按列主序写入 +0x00/0x10/0x20 共 48B，模拟输出 [1..12] 与 reconstruction 布局及单测完全一致。行内描述'Mat4 列重排，shufps/unpcklpd 重排 16 个 f32'措辞不准确：实际用 unpcklps（unpcklpd 在 0x14089d560 impl$1::from）且输入为 12 个 f32（3 个/列），建议行文修正但结论不变。",
        "evidence": "rne-read --hex 0x14089d500 96 + capstone + SSE 模拟器（shufps 语义经 clang 实测校准）；HAND-DECODED.md 记录",
    },
    {
        "addr": "0x14089d550",
        "verdict": "一致",
        "detail": "Affine3Packed::add_y 手解成立：PDB 精确符号吻合（11B）；行内引用的 decomp-0x14089d550.txt 为 NotFound 错误文件，实际证据为 rne-read 手解——本次独立复核：addss xmm1,[rcx+0x28]; movss [rcx+0x28],xmm1; ret = 偏移 0x28 处 += y 参数，与 reconstruction 的 translation.y += y（cols[3].y 恰在 0x28）及行内断言完全一致。注：y 参数落在 xmm1（第二浮点槽）而非 xmm0，属寄存器分配细节，不影响语义。",
        "evidence": "rne-read --hex 0x14089d550 32 + capstone（addss/movss/ret 11B 精确吻合 PDB size 11）",
    },
    {
        "addr": "0x1408e30f0",
        "verdict": "一致",
        "detail": "inverse_ridge_01_param 成立：PDB 精确符号吻合（176B）；证据文件 evidence/roof-ridge/tg_country_core_resources_roofs_roof_ridge_inverse_ridge_01_param.c 存在且与 decomp-0x1408e30f0.txt 内容一致：(ridge+(-0.1))/(dim+(-0.1))、夹紧 [0,1]、两个断言 panic（'assertion failed: ridge_param.is_finite()' / '!ridge_param.is_nan()'，UNK_142aa5848/5890）+ 退化分支（dim==0.1）panic，与 HAND-DECODED 记录及'已由 ridge.rs 恢复'一致。",
        "evidence": "evidence/roof-ridge/...inverse_ridge_01_param.c 存在；decomp-0x1408e30f0.txt（有效 1042B）；--unks 两断言串",
    },
    {
        "addr": "0x1408e4040",
        "verdict": "一致",
        "detail": "roof_uv_at_ws_pos 成立：PDB 精确符号吻合（74B）；decomp 有效：调用 FUN_1408e4090（辅助计算）后按 found 标志写回 {found,uv} 结构（found@+0，uv@+4 / flags@+1），与行内'包 roof_uv_and_point_at_ws_pos，返回 {found, uv}'一致。",
        "evidence": "decomp-0x1408e4040.txt（有效 359B）",
    },
    {
        "addr": "0x1408e45b0",
        "verdict": "一致",
        "detail": "RoofTriggerVolume::set_triangles 成立：PDB 精确符号吻合（64B）；decomp：旧数组按 0x24（36B=三角形三顶点）释放（func_0x000140613c20），再整块拷入 {len,ptr,cap}（*param_1=*param_2、[1]=[1]、[2]=[2]），与行内'drop 旧后整块拷入新 Vec'一致。",
        "evidence": "decomp-0x1408e45b0.txt（有效 286B）",
    },
    {
        "addr": "0x1408e45f0",
        "verdict": "一致",
        "detail": "RoofTriggerVolume::set_normals 成立：PDB 精确符号吻合（106B）；decomp：按 0x24 stride 释放旧 normals + 拷入新数组，随后断言 panic 串解析确认 'assertion failed: self.triangles.len() == self.normals.len()'（UNK_142aa59e0，0x3c 行号），与行内断言描述逐字吻合。",
        "evidence": "decomp-0x1408e45f0.txt（有效 817B）；--unks：UNK_142aa59e0 断言串逐字命中",
    },
    {
        "addr": "0x140940d80",
        "verdict": "一致",
        "detail": "insert_resources 成立：PDB 精确符号吻合（898B）；decomp 有效（22959B），callee 列表包含行内列举的全部真实构造函数（FUN_140b18550/140c97500/140cff670/140cf8250/140d3fa40/140d4d740/140d64e00 等 20+ 资源构造调用）+ bevy_ecs 组件库路径，与行内'逐资源调真实构造函数，再经 bevy_app insert_resource 入 App'一致。",
        "evidence": "decomp-0x140940d80.txt（有效 22959B）；callee 集含行内 7 个构造函数地址",
    },
    {
        "addr": "0x140a38460",
        "verdict": "一致",
        "detail": "TreeStorage::get 成立：PDB 精确符号吻合（255B）；decomp：空表守卫（param_1[3]==0 → 0）+ FUN_1405b0d70(param_1+4,&key) 取 hash + bucket 探测（16 字节 key 逐字节比较构建匹配位图 auVar18），与行内'哈希查找 + 16 字节 key 逐字节比较 + 命中返回条目值指针'逐项一致。",
        "evidence": "decomp-0x140a38460.txt（有效 4897B）",
    },
    {
        "addr": "0x140a385a0",
        "verdict": "一致",
        "detail": "TreeStorage::remove 成立：PDB 精确符号吻合（83B）；decomp：FUN_1405b0d70(param_1+0x20,&key) 查找 + FUN_140b76f90 移除 + 非空时 CONCAT44 返回被删条目，与行内'哈希删除 + 返回被删 Tree'一致。",
        "evidence": "decomp-0x140a385a0.txt（有效 435B）",
    },
    {
        "addr": "0x140ae6570",
        "verdict": "一致",
        "detail": "mesh_atlas_library impl$2::from 成立：PDB 精确符号吻合（226B），pubsym 证实 From<&asset::prefab::PrefabName> for AtlassedMeshName；decomp：'prefab:' 串（UNK_142ae3330）+ 字符串构建/分配（func_0x000141556130、FUN_140d79f90、FUN_140c9cd80 + 释放），与行内'同族 From 转换（另一 key 类型）'一致。",
        "evidence": "decomp-0x140ae6570.txt（有效 942B）；--unks：UNK_142ae3330='prefab:'",
    },
]

out_path = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/round3/oa2-results/w8_renderloop_roofs.json"
import os
os.makedirs(os.path.dirname(out_path), exist_ok=True)
with io.open(out_path, "w", encoding="utf-8") as f:
    json.dump(results, f, ensure_ascii=False, indent=1)
print("written", len(results))
