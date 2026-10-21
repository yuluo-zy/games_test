# 指示符五分支与调用链源码独立复核

复核者：core_business_lead。已直接审完整 typed builder、树光标第二阶段、水/边界配对补丁及新增测试，合读完整3840B builder/两个原闭包身份包和旧 probe 的函数字节匹配合同。允许 root 在按 paired-handoff.json 核对 before 后配对整合；未运行编译或 GPU，语义与版本最终验收另记。

新 builder 原地替换现 raw 对象解释，不新建平行模块。五种真实来源全部使用已有 Mesh/AssetMeshLibrary：Named 由库持有，Empty/Points/PointsWithColors/Generated 各持实际动态句柄与 TransientMesh 回收凭据。保留网格解析、实体预留、shader 查询的顺序，默认动画、pass 优先级、DontCastShadow、可选寿命与父组件均沿原合同落地；未伪造 SignifierId 或成功句柄。

两处真实 raw caller 同时传回已有工厂提供的 mesh/shader 库。边界闭包已根据复核反馈改为借用 &[Vec3]，保持原复制点序，同时消除 safe 函数接受任意裸指针的 UB 入口。水闭包只变 typed 调用衔接，没有顺带改水业务。

树光标第二阶段以第一阶段 producer 的 after 为 before，保持放置/历史修复。owned Arc 工厂替换每帧 Box::leak，Empty 网格和图像、尺寸/NaN/Inf/偏移按原语义接入现创建、同名去重、失活回收链。原 Empty 确实无属性和索引，GPU 宿主必须正确处理零索引绘制，不能添假几何。

core 五项与树两项及边界测试均未执行。测试 shader backend 只承载 CPU 工件且 GPU 创建不可达，不能用这些测试宣称实际 shader 或可见图像通过。精确最终 before/after、所有合法目录和原证范围以 paired-handoff.json 为准；其中 border 最终 after 为 `0a06f99c46b836c36df1791a9975bb84a9e8d11c8005b1bbae33f30e680b6dc2`。

第一阶段 processor 的新测试已同步 root 的 PrefabNameRef::as_str 修复，最终 after=`d894459741b8c1de1f46fb2d03fb72443679b9c86a12d52e09862310fd2d3db9`；该修订仅影响测试 API，不改变业务。没有用原近似 flags/主题注释替代实际调用证据。
