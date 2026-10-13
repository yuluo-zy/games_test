# 空 descriptor set gap 适配复核

根报告二帧 host AV 发生于set allocate1，set1是反射空gap；本文件复核适配合同，未独立重现该崩溃，也不确认Intel driver根因。

保留pipeline layout sets[0]/sets[1]/sets[2]编号，跳过bindings为空的pool/set分配是合法优化。分别bind(firstSet0,[set0])与bind(firstSet2,[set2])，仍使用相同完整pipeline layout。只要shader不statically use该空set，就不需要绑定它。不能压缩实际set向量后firstSet0一次bind，否则set2落到槽1。command必须继续持有实际set与资源；pipeline持完整layout/空slot所有权。

确认前提：layout.plan.bindings为空，与已核反射相符；遇非空placeholder必须重新分类，不能跳过statically used set。保留固定源码下的empty->syntheticglyph->warm多帧实际验证，不能因绕过崩溃点而宣称通用空layout API错误或driverbug已确认。

依据[Khronos Descriptor Sets](https://docs.vulkan.org/spec/latest/chapters/descriptorsets.html)中statically unused set无需绑定合同，与[vkCmdBindDescriptorSets](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets.html)的firstSet槽位映射。
