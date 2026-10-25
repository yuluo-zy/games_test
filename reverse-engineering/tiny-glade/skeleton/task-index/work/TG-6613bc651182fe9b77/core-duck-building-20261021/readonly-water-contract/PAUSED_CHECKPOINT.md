# cache 合同断点（未完成）

2026-10-21 lead 改派排 root 三项 chimney CPU 失败。完整9331B两包与实际f507 RNE逐byte一致、PDB458:2156确认、常量/jump表见 INPUT_BINDING_AND_CONSTANTS.json；这里还没有完成、冻结或通过的完整合同。未写 cache candidate、生产或共享utils候选。

已读完整业务原ASM：owner/wall/roof/terrain/dst4及panic部分。已向duck/lead交的主要事实：dst4首个 exact有序Params.segment匹配，先匹配后u校验，Curve2定位+i/t，interpolated Curve3取点；mode2数值量化，mode3用Params.nodes/真实StairsState/previous.sub_segments/Params.segment原序，y有序量化后沿downhill平移.2；earlier tangent的perpendicular normal仍供最后width*lateral。未匹配原(-0,-100,-0)+identity+ZERO仍走compare/insert/dirty。

roof63=first outgoing adjusted edge→xz_rotation_bad，并仅flat加extraheight；terrain63先加extraheight再first outgoing→独立xz_rotation_from_forward140c913e0/607B，现无publicAPI。其完整旧缓存已找到utils-tabloid/140c913e0.asm.txt和utils-base/0x140c913e0.c；待中央确认最窄共享leaf范围，不默替bad。

后续需整理完整分支/原失败先后与当前API适配（尤其每entry cache.insert后即时modified_ids标记，不能batch延迟改变panic副作用）；必要9param typed wrapper绑定和607B复用证据也应一并冻结。当前只是原证/进度，不是业务恢复验收。
