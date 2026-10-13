# 原shader接口残差调查与最小源码修复

基线为 shader-build-2026-10-13/dxc/merged-audit.json：404新容器/638stage真编译成功，但194完整metadata不同。原发行每资产SHA经compile-commands目录独立核对；raw-interfaces.json记录原/新SPIR-V变量、OpName、Location、成员Offset的字偏移、类型和module SHA，未把原metadata输入新producer。

194差异按组合分为depth104、color77、compute13；字段计数为descriptor sets/name/hash86、vertex inputs148、uniform35、local_size2。148个vertex差异里122仅名不同，其他26包括稀疏location压缩与真实额外/缺失输入。69源族138资产可直接恢复原输入semantic与显式vk::location；五个源族10资产涉及真实额外/缺失输入，不能删除活跃变量制造反射相等。映射/例外见safe-vertex-source-mappings.json与classification.json。

真实DXC两独立样本已通过：billboard_plant与garden_lowpoly_flower，仅改VS输入声明后原OpName/Location/Type完全相同，opcode histogram与未改baseline完全相同。garden原0/2/3空洞保留。build-entry同伴也独立读原bin和新SPV复验通过，见../build-entry/runtime-peer-review.json。

69源族在根融合remote后的隔离worktree HEAD902bb6e应用限定VS结构补丁；不改顶点计算、PS、新增TGV blocks或反射producer。另deferred canopy/plant的三个Globals成员从brdf/terrain/visual修为原terrain/visual/brdf，原offset104/112/120；两个独立真实DXC新PS全部19成员Offset精确相同。原字节证据deferred-globals-raw.json，实编译uniform-compile-probes.json。71文件工具补丁后实际复读，源码快照source-snapshots/。该段记录isolated probe阶段；最终全量打包验收见文末final-readonly-review.json。两probe本身不代表138个包已验证。

深度分支原证据：117 depth资产114原无Color output，而旧fresh113错误输出；38原有OpKill，70原无output/kill/conditionalbranch（不自动推成完整空shader）。远端113个提升PS分支对应这113集合。ice_plane、nani_gothic_window_bricks、nani_solid_vertex_color_bricks三者depth原仍有输出，未用统一void替代。

compute同伴已从原宏producer/caller恢复38非零key的8个宏，不再沿用旧报告未知结论。独立复审证明TGSR四配置wave32=1严格8×4、0严格8×8；sharpen1含ControlBarrier、0无；caller是unsignedsize<=32 AND support、orderedstrength>0（NaNfalse）。DOF parent enum13比较产生bool，经callee准确栈偏移zeroextend后传5subpass；Nani上游可选pointerpresence产生0/1后原32bit透传；未猜UIenum或pointer概念含义。见compute-tgsr-peer-review.json与compute-caller-peer-review.json及同伴../compute/original-compute-tuples.json。

本轮未启动新的Ghidra后端：接口证据来自真实发行SPIR-V字节，caller复审复用同伴专属SDK的原RNE反汇编并登记哈希。源算法、原DXC身份以及whole shader语义仍独立保留残差，不以接口修复宣称完成。

后续互审纠正：初69源补丁按同名struct扫到了34个PS块，分布28源；已生成restore-ps-inputs.patch由根应用（子agent tools.apply_patch审批状态挂起25分钟，未再使用该接口），本agent按原PS输入block独立复读34块全部恢复，见ps-restore-independent-check.json。正确VS仅按vs_main实际位置关联最近同名input struct，corrected-vs-only-plan.json保存更正证据；source-snapshots旧快照含历史PS误改，不能作为最终源。

本轮新增Globals名字修复仅生成patch、未直接改HLSL：119source、796个按源文件分别计数的identifier、2400代码行，在实际$Globals声明逐项匹配原同source的OpMemberName、set/binding、packoffset，0例外。定义与表达式引用一起rename _Globals_<原名>→<原名>；comments/strings、其它cbuffer、bool_、type/stride/layout不改。globals-prefix-plan.json含证据，globals-prefix-name.patch供根应用，generate_globals_name_patch.mjs可以在layout修复后重新生成context但不写source。cloud与billboard_forest两官方DXC VS探针全部原member名字/offset相等、opcode histogram与native基线不变，见globals-prefix-compile-probes.json。19pack严格跨stage UBO冲突的真实layout部分由根/pipeline owner负责，不放宽producer，也不注入stored metadata。

最终只读审查已冻结（final-readonly-review.json）：逐读隔离publish corpus-final-native的404新容器/638stage，404全部打包成功，397完整schema与原相等，313公共反射相等，300capability集合相等，88非debugopcode histogram相等。34PS scope恢复、119Globals改名规则及19失败包38阶段UMember type/name/offset/bytes正式互审均通过；solid两个variant四stage84B布局原等。保留7真实schema残差：gothic_window_bricks、solid_albedo_map_emissive、wall_brick_lod0共6个asset input名称不等但location/type等；final_post depth额外binding/UBO。未把这些说成无影响，未声明原shader计算语义全部相等。38nonzero compute配置已恢复，132zero compute tuple仍未独立证实；TGV source adapter defines不是原游戏宏。最终审查未跑DXC/SDK、未改源码。
