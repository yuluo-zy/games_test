# -*- coding: utf-8 -*-
# Worker w9 (audio-misc 5 + backend-misc 5) round2 O-audit results writer.
# ASCII-only stdout per worker instructions; Chinese only inside the JSON file.
import json

results = [
 {
  "addr": "0x1408efd40",
  "verdict": "一致",
  "detail": "PDB 精确符号 country_core::startup::debug::debug（size 7378，owner off 0）与行内 fn 完全一致；decomp 有效（35954B，非 NotFound）。rne-read 逐串核对行内清单：assets/meshes/decorator_delete_scribble.json（0x142aa7517）、decorator_test.json（0x142aa7579）、decorator_test_turret.json（0x142aa75c6）、decorator_test_door.json（0x142aa769e）、decorator_test_cupola.json（0x142aa76f3）、decorator_test_dormer.json（0x142aa774d）、decorator_outline.json（0x142aa77a5）、stair_step.json（0x142aa7895）、particle_plane.json（0x142aa7914）9 个路径全部存在；debug_sphere_white/red/blue/green/grey（0x142aa79b5 区段 5 个连续串）全部存在；另见 ladder/sparkle/cube_grey/solid_vertex_color/leaf_particle.raster.hlsl/texture_test.raster.hlsl 等，与\"等网格\"表述相容。串呈 {标签}\\0assets/meshes/… 成对布局（个别标签含非 ASCII，读取处 \ufffd 为打印编码伪影，路径本身逐字节核对无误）。",
  "evidence": "expansion-b2/decomp/decomp-0x1408efd40.txt（有效 35954B）；rne-read 0x142aa7517/7579/75c6/769e/76f3/774d/77a5/7895/7914/79b5/79c7/79d7/79e8/79fa"
 },
 {
  "addr": "0x140b024e0",
  "verdict": "一致",
  "detail": "PDB 精确符号 country_core::utils::debug_viz::DebugSphere::radius（size 31）与行内 fn 一致；pubsym DebugSphere::radius::h58f6a40c2e31c50e 吻合。chunk 的 decomp_files 是 {\"error\":\"No function at 0x140B024E0\"} NotFound 无效文件（第一轮系统性模式），但行内 evid 为\"原字节短函数\"（未用该坏文件）；改用 rne-read --hex 0x140b024e0 48 复核：机器码恰好 31B，与 PDB size 精确相等。手解：mov rax,rcx; movss [rdx+0x10],xmm2; mov rcx,[rdx+0x20]; mov [rax+0x20],rcx; movups xmm0,[rdx]; movups [rax],xmm0; movups xmm0,[rdx+0x10]; movups [rax+0x10],xmm0; ret —— 将 float 参数写入结构 +0x10（Vec3 center 之后即 radius 字段位置）并整结构拷贝 40B 至 self 槽，\"设置半径\"成立。注：形态为\"改临时+整结构拷贝\"的 by-value builder/赋值式（float 走 xmm2，非简单 movss [rcx+0x10],xmm1;ret），reconstruction 若按 &mut self 直写实现应复核签名。",
  "evidence": "rne-read --hex 0x140b024e0 48（31B 与 PDB size 吻合）；chunk 内 decomp 文件为 NotFound 无效（未采用）"
 },
 {
  "addr": "0x140b02ea0",
  "verdict": "一致",
  "detail": "PDB 精确符号 country_core::utils::debug_viz::DebugRectangle3d::dashed_line（size 38）与行内 fn 一致。chunk 的 decomp_files 同样是 NotFound 无效文件，行内 evid\"原字节短函数\"；rne-read --hex 0x140b02ea0 64 复核：机器码恰好 38B，与 PDB size 精确相等。手解：mov rax,rcx; mov byte [rdx+0x34],1; movups xmm0,[rdx]; movups xmm1,[rdx+0x10]; movups xmm2,[rdx+0x20]; movups [rcx+0x20],xmm2; movups [rcx+0x10],xmm1; movups [rcx],xmm0; mov rcx,[rdx+0x30]; mov [rax+0x30],rcx; ret —— bool 置 1（true）于结构 +0x34 并整结构拷贝 56B，\"虚线开关\"成立。",
  "evidence": "rne-read --hex 0x140b02ea0 64（38B 与 PDB size 吻合）；chunk 内 decomp 文件为 NotFound 无效（未采用）"
 },
 {
  "addr": "0x140b04640",
  "verdict": "一致",
  "detail": "PDB 精确符号 country_core::utils::debug_viz::DebugRectangle2d::show（size 945）与行内 fn 一致；decomp 有效（3476B）。逐项核对行内断言：4 个 f32 角点从 param_1[0..4] 装载（uStack_d8..cc，经 CONCAT44 打包进组件）；FUN_140c991f0(&uStack_b8,param_1+5,param_1[0xb]) 为\"box\"子构建（claim 引用的 callee 之一确实被调，结果 16B 进入组件字段）；fmt：FUN_140d8c890/FUN_140b00840/FUN_140b05c70 函数指针组 + FUN_140d79f90（claim 引用的第二个 callee 确实被调）格式化入缓冲（lStack_f0/lStack_e0，调用后释放）；排队：func_0x000140613c10(0x48,8) 分配 72B 组件（含 type id 常量 0x42ae7088）填充后 FUN_14146c880(param_2,&uStack_b8) 提交。四个断言元素（角点/box/fmt/排队）全部在位。",
  "evidence": "expansion-b1/decomp/decomp-0x140b04640.txt（有效 3476B）；callee FUN_140c991f0、FUN_140d79f90、FUN_14146c880 均在函数体内"
 },
 {
  "addr": "0x140c9cd70",
  "verdict": "一致",
  "detail": "PDB 精确符号 utils::name_hash::impl$2::eq（size 10），pubsym 展开为 _$LT$utils..name_hash..NameHash$u20$as$u20$core..cmp..PartialEq$GT$::eq，即 PartialEq<NameHash>::eq，与行内 fn NameHash::eq 为同一函数（impl$2=PartialEq 实现，非地址错位）。rne-read --hex 0x140c9cd70 32：机器码恰好 10B，与 PDB size 精确相等。手解：mov rax,[rcx]; cmp rax,[rdx]; sete al; ret —— 两 u64 值 == 比较返回 bool，行内\"u64 比较\"成立。",
  "evidence": "rne-read --hex 0x140c9cd70 32（10B 与 PDB size 吻合）"
 },
 {
  "addr": "0x140865050",
  "verdict": "一致",
  "detail": "PDB 精确符号 country_core::systems::brush_preview::brush_preview（size 5355）与行内 fn 一致；decomp 有效（27418B ≈ 26KB，与行内\"（26KB）\"吻合）。结构核对：7 参数 Bevy 系统——运行条件检查（func_0x000140c16a00）、UTF-8 路径分段解析（\\\\ 与 / 分隔遍历 + panic 路径串 UNK_142a965e8/142a95f2b 等，rne-read 0x142a95dd0 为 /rustc/05f9846f… panic 定位串，非业务串）、实体遍历（0xa0 步长组件表查表）+ 密集 float 运算（fVar10-fVar16、fStack_180 等，UNK 常量多为浮点表，如 0x142925870 起为 3f000000/3efc0fc1 等 float 数据）、FUN_140c16d40/func_0x000140c18130 等访问器。\"预览网格跟随光标/笔刷大小更新\"与代码结构相符，未发现矛盾；具体光标/尺寸联动未逐指令复推（行内未给可逐项核对的数字断言）。",
  "evidence": "terrain-water/decomp/decomp-0x140865050.txt（有效 27418B）；rne-read --hex 0x142925870/0x142a95dd0 等（float 表 + /rustc/… panic 串）"
 },
 {
  "addr": "0x1408c1310",
  "verdict": "一致",
  "detail": "PDB 精确符号 country_core::systems::backend::flush_screen_log::log_warning_to_screen（size 616）与行内 fn 一致；decomp 有效（2729B，非 NotFound）。核对\"同上，条目 kind=1（Warning）\"：clone 串（func_0x000140613c10 + func_0x000142923c00 拷贝模式，与上一行 log_error_to_screen 0x1408c10a0 同构）；0x20B 条目环队列（基址 0x14362bff0-0x14362c000，len 计数 +1，FUN_140562c30 扩容）推送 {ptr@0, len@8, ptr@0x10, kind@0x18}，kind 字节写 1（Warning），与 log_error 的 kind=0（Error）对应。锁/condvar 全局（0x14362bfe8 + WakeByAddressSingle）为队列同步，panic 路径（FUN_1428d8fe3/FUN_1428d9760）为 Rust 溢出处理。",
  "evidence": "backend-misc/decomp/decomp-0x1408c1310.txt（有效 2729B）；*(undefined1 *)(entry+0x18)=1 写入点"
 },
 {
  "addr": "0x140a90e00",
  "verdict": "一致",
  "detail": "PDB 精确符号 country_core::systems::is_gameplay_active（size 22）与行内 fn 一致。独立 rne-read --hex 0x140a90e00 32 与行内手解逐字节吻合（22B，与 PDB size 精确相等）：mov rax,[rcx]; cmp byte[rax+0x30],1; jne+0a; mov rax,[rdx]; cmp byte[rax],1; sete al; ret; xor eax,eax; ret —— 谓词为 [rcx]@+0x30==1 && [rdx]@0==1，先查 rcx 资源失败即返 0，与行内断言（含顺序描述）完全一致；行内诚实标注\"资源身份 U（无 xref 证据）\"维持不变。chunk 的 decomp 文件本身即该手解转录（749B，非 Ghidra 输出亦非 NotFound），作为行内 evid 有效。",
  "evidence": "rne-read --hex 0x140a90e00 32（22B 与 PDB size 吻合）+ backend-misc/decomp/decomp-0x140a90e00.txt 手解转录对照一致"
 },
 {
  "addr": "0x140a90e80",
  "verdict": "一致（胶水，按规则不计业务）",
  "detail": "PDB 精确符号 country_core::systems::_::impl$0::dyn_hash（size 56）与行内 fn 一致；pubsym 展开为 _$LT$impl$u20$bevy_ecs..schedule..set..SystemSet$u20$for$u20$country_core..systems..GameSystemSet$GT$::dyn_hash（impl$0=SystemSet 实现，非地址错位）。decomp 有效（393B），机器码复核（rne-read --hex 0x140a90e80 0x60，56B 与 PDB size 吻合）：mov rdx,0x3da5601bd502516a; mov rcx,hasher; call [hasher_vtable+0x40] —— write_u64(0x3da5601bd502516a)；movzx edx,byte[rbx]（判别字节）; mov rax,[rsi+0x80]; jmp rax —— write_u8(判别) 尾调用。与行内\"先 write_u64 常量（类型标签），再 write_u8（判别）\"完全吻合。SystemSet dyn_hash 胶水形态，按规则不计业务，O 身份成立。",
  "evidence": "backend-misc/decomp/decomp-0x140a90e80.txt（有效 393B）+ rne-read --hex 0x140a90e80 0x60（56B 与 PDB size 吻合）"
 },
 {
  "addr": "0x140aab370",
  "verdict": "一致",
  "detail": "PDB 精确符号 country_core::systems::backend::texture_watch::texture_watch（size 2640）与行内 fn 一致；decomp 有效（23774B）。逐项核对：锁模式 *param_2+0x30 → 互斥标志 @+0x10 ✓（decomp L116-121 加锁/解锁标志读写）；日志串逐字节核对 —— \"Texture watch: detected changes: \"（fat-ptr 头在 0x142add810 → {0x142add828, len 0x21=33}）与 \"Error processing textures: \"（0x142add878，其前紧接 country_core::systems::backend::texture_watch 模块路径串）均吻合 ✓；FUN_140b73a50(&lStack_190, auStack_170, 0x70, 0x10) 两个调用点参数全同（auStack_170[16] 即 u16 位图载体）✓；FUN_140917860（L298 校验/比较）✓；FUN_140be24b0（L389）✓；func_0x00014003bb80（L388，无参调用）✓。瑕疵注记：func_0x00014003bb80 机器码为单字节 c3（1B ret no-op，ICF 折叠区，与第一轮确认的 Plugin::finish<MainCameraPlugin> 同址），调用它不执行任何\"释放\"动作，行内\"（句柄释放对）\"注释过强——建议改为\"空 drop 占位调用（1B ret no-op）\"，其余断言全部成立，O 结论维持。",
  "evidence": "backend-misc/decomp/decomp-0x140aab370.txt（有效 23774B）+ rne-read --hex 0x142add810 0x80 / 0x142add878 0x60（两条日志串逐字节）+ rne-read --hex 0x14003bb80 16（c3 单字节 ret）"
 }
]

path = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/round3/oa2-results/w9_audio_backend.json"
with open(path, "w", encoding="utf-8") as f:
    json.dump(results, f, ensure_ascii=False, indent=1)
print("wrote", path, "entries:", len(results))
