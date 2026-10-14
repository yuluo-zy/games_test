# Shader pass 来源分类

本批当前覆盖124个原始source路径，分类见classification.json。判定对象是B发行包原SPIR-V的OpName、布局、OpLine对应运算以及公开上游。恢复HLSL仅用来寻找候选，不作为作者归属证明。game_specific表示游戏自身实现/流程有证据，不表示每个标准数学公式均是作者发明。generic_unresolved是来源未定，不计作原创。

## 新确认的Kajiya入口实现

`shaders/blur_downsample.cs.hlsl`来自Kajiya `assets/shaders/blur.hlsl`的同源源码实现：原工作组64x1x1；原`vblur_out`数组138；每组输入tile128x2；两遍11tap（uint上界10）；边缘5；高斯权重`exp(-d*d*0.16)`。公开Kajiya的group_width64、kernel_radius5、vblur_window_size138、gaussian_wt和vblur_into_shmem组合相同。此证据强于模糊算法名称或路径。游戏新增MSAA读取入口与资源绑定，不声称源码逐字一致。

`shaders/blur_pyramid/blur.cs.hlsl`是同家族的改写：相同64线程、128x2、vblur_out和两遍Exp0.16结构，共享数组改136、两遍9tap，显式input_mip_dims与非整尺寸mask。归类game_modified。

`shaders/calculate_reprojection_map.cs.hlsl`是Kajiya同源重投影map改写：原指令中同时有32767 SNORM位移量化、GatherRed后wzxy排序、四点权值1/2/4/8并除15、prev_clip_to_prev_view和深度增量-0.2截断。公开Kajiya同名文件也有这组不寻常的具体组合。游戏改了速度空间、历史有效性、阈值与相机抖动规则，归类game_modified。单独的投影矩阵数学不是源码来源证据。

三项完整原指令operands、SPIR-V module偏移、源行、raw hashes、LocalSize/array layouts保存在new-upstream-fingerprints.json；full原二进制解码在rawproof.json。上游定位为Embark commit9fdec0f074f9f822d395f9cfd041b58749e32303，不是游戏私有commit。

## FXAA、DOF、TAA/TGSR

FXAA include很可能是NVIDIA FXAA3.11 PC Quality源码派生。原fxaa.inc.hlsl 1530 LF，实际源行708、881..1236发射核心指令；绿通道gather、二阶边缘梯度、1/12亚像素均值、0.166与0.75参数、quality12的1/1.5/2/4/12双向搜索、跨度符号判定共同吻合。参照Intel GameTechDev/CMAA2维护的FXAA3_11.h，其首部给出NVIDIA原文件链接，下载原URL返回404；镜像增加lumaR8路径，不认为它就是游戏原文。分类likely_derived/high，保留编辑版本/精确copy未确定；fxaa.cs.hlsl游戏入口分类game_adapter。

DOF当前B原指令包含DofTileOffsets、tile_coords_buffer、indirect dispatch、多分辨率ring_samples；autofocus包含DofFocus.focus0/focus1/lock_pos_ws/use_tilt_shift。公开Kajiya的dof/gather是RAD_SCALE0.4黄金角螺旋采样，缺少此tile/indirect/multires设计。当前B的DOF主系统归game_specific/medium；这表示游戏系统自己组合实现，不证明所有CoC或采样数学的创造者。不能仅dof同名判Kajiya复制。

当前B的taa.cs.hlsl资源和主策略也不同于Kajiya公开taa/taa.hlsl：原B有tile_luminance、sharpen_amount、bypass_neighborhood_clamping、history_valid，公开Kajiya使用input_prob/smooth_var_history/velocity_history多pass置信度系统。共享unjitter_taa、FrameConstants或YCbCr helper单独分类，不能使整份TAA判为复制。

TGSR原B有独立luminance/chroma重投影资源、blurred_luminance_tex、aux历史和tile锐化；官方Steam公告称TGSR为游戏自己的方案。这是developer_authored_claim加B自身算法结构的依据。公告提到后续TGSR2.0神经网络，不用于证明本B已实现神经网络或与该公告版本等价。分类game_specific，高置信度针对来源声明，私有源码/版本逐字边界仍未证。

## 共享组件与游戏pass外壳

SPD核心与game min-depth/average-color回调不同；XeGTAO核心与gameSSGI资源/参数调度不同；GPU-Raytracer BVH8与水冰/terrain材质hit/颜色组合不同；Kajiya reservoir与game deferred lighting不同。分类把入口作为game_adapter，原路径emitted_shared_sources和actual_source_emission给出何处实际发射。依赖表存在但被优化掉时只说明被include，不证明已执行。

UI/text/egui颜色按钮、cloud/sky、final_post、灯光binning等保留游戏资源结构和控制参数，标game_adapter；标准纹理copy、prefix scan、模糊等来源不够明确的标generic_unresolved。不能把未知一律游戏原创，也不能把一个上游include的消费者全部算成整文件复制。

公开参照：[Kajiya blur](https://github.com/EmbarkStudios/kajiya/blob/9fdec0f074f9f822d395f9cfd041b58749e32303/assets/shaders/blur.hlsl)、[Kajiya reprojection](https://github.com/EmbarkStudios/kajiya/blob/9fdec0f074f9f822d395f9cfd041b58749e32303/assets/shaders/calculate_reprojection_map.hlsl)、[Intel维护的FXAA原来源镜像](https://github.com/GameTechDev/CMAA2/blob/master/Projects/CMAA2/FXAA/Fxaa3_11.h)、[开发者Steam公告](https://steamcommunity.com/app/2198150/announcements/)。

## 补齐材质、水冰路径与SSGI分类纠偏

追加15路径：ice_plane、solid_vertex_color、solid_vertex_color_albedo_map、vertex_color、water/ice_cracks、water/ice_shared、water/water_object_push、water/water_surface_frag、water/waterfall_vert、water_plane、water_plane_refraction_copy、water_snow_plane、water_x_paths、waterfall、waterfall_refraction_copy。保留以前109项，vertex_color仅基本颜色变换，归来源未定；其余14项按原资源/类型/绑定合同归游戏适配，不声称一般反射/折射/BRDF/噪声数学原创。waterfall入口自身没有OpLine发射代码，其代码实际发射到waterfall_vert与water_surface_frag等include；WaterfallMesh.position/tangent/normal0/normal1和water_rt/ripple资源可观察，避免将其算一整份独立原创正文。

SSGI temporal_filter先前粗分game_adapter，深入对照后修正game_modified，与common报告一致：原raw138条该source指令，有0.2231301665/0.4723665416/0.2644112706核，mean混合0.25，dev×0.5×5，钳制历史后0.125混合当前；公开Kajiya同文件box_size0.5/n_deviations5，box_size平方混合均值、方差钳制和1/8时域混合组合相同。游戏将k2的5x5改为k1的3x3，新增history_valid/frame_index门控和绑定。继承核心不能因接口是游戏定制而被覆写成game_specific。完整可复现更新脚本followup.py，分类主脚本也同步了这项规则。
