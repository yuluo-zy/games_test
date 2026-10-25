# Shared include / Kajiya candidate classification

`classification.json` 覆盖原作 source-dependency 记录中全部 95 个 `shaders/inc/` 路径，以及 35 个 Kajiya 同布局候选的并集，共 104 条。原始 alias 路径（如 material/eval/../canopy_material）按原记录保留，不把别名当独立源码体积。

分类结果：70 generic_unresolved、25 game_specific、3 game_modified、2 likely_derived、4 confirmed_upstream。数字统计的是带有 scope 的文件条目，不能换算代码原创/开源比例。

- confirmed_upstream：reservoir helper、AMD SPD 两个 include、rt.hlsl 的 BVH8 核心。每个条目都有边界，例如 rt.hlsl 720 LF 中只确认指定的 BVH8 解码/遍历部分。
- game_modified：FrameConstants/ViewConstants 字段族继承并加入游戏主题等参数；重投影的量化与validity打包设计保留且阈值改变；SSGI temporal的统计/混合框架保留且邻域从上游5×5改成游戏3×3。
- likely_derived：unjitter 的两套 Gaussian 颜色与统计权重设计；HK include 的独特路径及133 LF布局。HK最终未发射执行指令，不能说其算法正在运行。
- game_specific：游戏草叶、灯笼、屋顶/墙/窗颜色、地形、植被排除及材质类型等领域接口。只判断字段/参数/组合层，不宣称整个文件的算法是从零原创。
- generic_unresolved：标准数学、噪声、SRGB、BRDF、packing、扫描、采样等仍无足够来源指纹。Kajiya同名文件、标准公式及常见常数不作为拷贝证据；这些文件可能含外部源码，不能归为游戏原创。

`raw-source-instruction-stats.json` 汇总直接从原发行 SPIR-V 读取的 OpLine 对应指令和所用float32常数；没有源码行指令代表本次编译产物中没有对应计算，不能证明源文件为空。`selected-instruction-traces.json` 保留重投影、SSGI temporal、unjitter代表模块的原指令offset、operands和constant definitions。它们不是重新编译恢复HLSL的结果。

本次使用上游 EmbarkStudios/kajiya main `9fdec0f074f9f822d395f9cfd041b58749e32303` 作比较，原作声明版本0.2.0，但具体forkcommit仍未知。原字节身份验证及SPD/BVH8原始指纹沿用同一audit根报告。
