# 非空水面、冰面和折射

先读取同目录 COMMON.md。本文件覆盖此前创建6名子agent的所有指令；你现在是单一执行者。

目标：W44–46在真实非空water/ice/refraction实体和实际生产数据上执行，输出可读回且接主世界图。
输入：当前startup_water、duck ripple、world_surface与Surface pass/原SPV及RT owner；已有TG-c27水面/TG-fef波纹/TG58da Surface有界原证先复用。
写面：world_surface.rs/必要Surface helpers与专属GPU测试候选；startup注册hunk交04，不改user的critters3文件。先核新duck错误是否已被用户修，不重复旧修复。
必须完成：真实mesh/shader→同AssetVAO/drawlists14/15；528B ripple真实Local平滑/末尾count上传；RT BVH/triangle/meta与贴图输入；Water/Ice shader分支、flags0x108/view/aspect、fragment原RMW和历史有效/未定义语义；refraction执行顺序和结果实际被后继消费。
禁止：手造drawlist、MeshID当VAO、空场景或清历史通过、改readonly/RMW合同避guard、固定max绘制、用无鸭特例替代非空水面验证。
验收：真实Summer水面与Winter冰面至少各两帧，实际528B上传和图输出读回，原History/phase/count连续且命中正确shader；由root执行GPU，作者不能把fixture已写当已通过。
