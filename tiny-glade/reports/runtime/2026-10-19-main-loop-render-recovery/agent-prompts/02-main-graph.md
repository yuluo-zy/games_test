# 完整主图与最终呈现

先读取同目录 COMMON.md。本文件覆盖此前创建6名子agent的所有指令；你现在是单一执行者。

目标：原compile_main所有实际可达分支串接到当前生产renderer，最终输出真实present所需资源；不能把W54或尾链子集称完整main。
输入：当前production_renderer.rs/world_postprocess.rs/effects_host.rs以及TG-0ef1874c29a03d0dec/main-chain-frame-20261018/evidence的compile_main10961B、当前shader资产。
唯一候选写面：production_renderer.rs及主图尾链helper必要hunk；W1–43/47–54内部由03，Surface44–46由05，注册由04。允许延续root旧TG-0ef/main-chain-frame-20261018，不改claim。
必须完成：世界显示条件前后的主目标创建/初始化；show_world=false路径不能被MainCamera等全局gate挡住；真实ImageFormation/ColorGrading/LUT→blur→finalpost；menu/Painterly/Vermeer、session/feedback/gallery/wireframe、MEI/UI/Egui/cursor/loadingoverlay、截图与panic capture按原时序/条件接线；export→resolve→present拥有真实format/samplecount/所有权。缺producer要恢复，不能以默认关分支绕过。
性能：保留当前图计划缓存/管线缓存与先submit后acquire设计，不每帧wait_idle/prewarm全部资源；新增缓存须准确失效。
验收：同一真实非空世界的warmup与正常帧，菜单/no-world分支，有效截图与多帧WSI；失败不呈现未定义颜色，历史引用和最终MSAA格式匹配。
