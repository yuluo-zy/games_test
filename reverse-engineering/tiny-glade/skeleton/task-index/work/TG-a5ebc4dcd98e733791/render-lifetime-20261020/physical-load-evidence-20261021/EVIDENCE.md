# 物理地址载入失败：有界证据与等价规范化

原运行日志 `/private/tmp/tg-paging-world-debug-20261021.log` 52143–52148 显示 nani_plaster VS 生成的 Metal 对 `const device uint2*` 直接做 float2 bitcast，遗漏载入。两个实际 shader dump 的原始 SHA 与逐字修改见 BYTE_EXPERIMENTS.json。它们是 lowering 后的模块，不能称为原包裸 SPIR-V。

从本机 B 安装的两个 compiled-shaders 包按 speedy 字段顺序读取（stage count、stage enum、entry 字符串、SPIR-V 长度及字），提取的原始 VS 身份见 INSTALLED_SHADER_IDENTITY.json。原包与 lower 后都保留 %134 PhysicalStorageBuffer<uint2>、%170 Function<uint2>、%310 pointer bitcast、%311 Aligned16 load、%322 float2 bitcast、%962 phi 的同一实际链。旧 reports/refactor/shader-assets-b 目录保存的是既有 lowering 结果。

官方 MoltenVK1.4.2 的 SPIRV-Cross pin 与三个源码函数链接见 UPSTREAM_SOURCE_IDENTITY.json。指针转换沿 numeric 值追溯至 Function 变量；解引用判定把 parent type/depth 相同但存储类不同的指针当成副本。这解释了已实际观察到的错误。安装 dylib 的具体构建 commit 尚未证明，不能仅从 Homebrew 版本号断言它就是该源码；CPU CLI 也是独立身份。

输入规范化只处理：Function uint32x2 变量的 OpLoad → 同 uint32x2 pointee 的 PhysicalStorageBuffer OpBitcast。添加同类型 OpConstantNull 与 OpIAdd(value,zero)，并只替换 cast 的输入 ID。每个 uint32 分量与零相加保持全部位，故地址 64 位不变；原物理 OpLoad 的结果类型、六个字、Aligned16、后续浮点 bitcast 和完整业务计算不变。不加 shaderInt64 能力，不降采样、不替换算法，不改依赖或系统驱动。

两个 dump 的字节实验均通过 spirv-val --target-env vulkan1.2。CPU CLI 同参数下基线复现遗漏解引用，规范化后输出真正的 uint2 解引用，再做 float2 bitcast。此结果只支持转换机制与输入可检验性；installed MoltenVK pipeline、真实世界 GPU 输出、长稳与性能仍待 root 实测。IR 每命中点多一个整数向量加法，无新增 SPIR-V 载入或分支；是否常量折叠及 native 成本尚未测量。
