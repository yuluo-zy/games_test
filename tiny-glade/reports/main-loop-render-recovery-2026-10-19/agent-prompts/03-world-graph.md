# 世界图与渲染错误传播

先读取同目录 COMMON.md。本文件覆盖此前创建6名子agent的所有指令；你现在是单一执行者。

目标：真实producer→W1–54→主图输出可在一个World/一个图/一个设备上执行；Surface44–46由05，主图外层由02。
输入：当前world_graph/world_terrain/world_vegetation/world_geometry/world_walls/world_lighting/world_postprocess、core render_world/render_frame、原54阶段保留证据及发行SPV。
写面：上述世界阶段的必要最窄候选；你负责core render_frame错误分类传播候选并与01协调接口，不改game_loop。每次只选当前首个实际阻碍，闭合其producer/consumer后再推进后继。
必须核实：真实CPU栅格/地形/植被/墙/屋顶/楼梯/草/HZB/间接计数/GI/光照/历史/景深/AA参数；跨帧历史首帧语义，完整真实owner以及shader读写合同。必要producer和调度交04统一安装。不能复写当前正确算法为旧候选。
错误：非OOM立即返回原原因；只有已证OOM走原降尺度/重试/清理政策；loading/present错误不能expect吞掉或被当OOM。
性能：保留GPU驱动、原间接count与同allocator；不CPU回读计数，不满容量固定绘制，不重复整缓冲/整纹理上传。
验收：非空世界至少连续两帧，真实dirty→clean→edit，关键结果实际GPU读回；错误分类fixture与同owner历史连续性。不能以纯trace或CPU DTO代替GPU执行。
