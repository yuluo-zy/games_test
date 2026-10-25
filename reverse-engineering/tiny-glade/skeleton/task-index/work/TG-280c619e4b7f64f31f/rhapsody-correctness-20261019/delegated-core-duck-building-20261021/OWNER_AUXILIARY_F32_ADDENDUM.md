# +0x80 辅助向量元素 f32 新原证（取代旧 4B word 类型未知边界）

完整retained原RNE反汇编：`reverse-engineering/tiny-glade/skeleton/task-index/work/TG-b4f30a32cb343cc349/boot-terrain-20261018/140008c90-full-rne.asm`（62374B真assemble实例，原身份同owner合同）。本补充不修改旧合同冻结内容，只取代其4B元素类型未知一项；向量业务专名仍未知，不声称完整mesh组恢复。

- 14000968d reset_edge调用前按Params+128/+12c读取两ID；140009692将rbp+7e0恢复到r13。
- 140009c33从r13+78读取tangent.len，140009c40从+60读取width.len；140009c6f从+70读取tangent.ptr，140009c82从+58读取width.ptr。同真实Params字段在一个完整循环中绑定。
- **140009c8d读r13+90为辅助len**；len为0跳9d20；非0以r11做真实bounds检查（9cad..cbb），失败→140010fe9。
- **140009cc1读r13+88为辅助ptr**；140009ce3按r15*4找到起始元素。140009ce7形成末偏移（含+4）。
- **140009d09 `movss xmm0,[rcx+r9]`，140009d0f每次+4，140009d13 `ucomiss xmm0,xmm1`（xmm1由9cb0清零），140009d16 `ja`继续**。原消费者将元素按f32解释且直接与0比较，没有u32→f32数值转换。因此canonical唯一owner可用`Vec<f32>`保原含NaN位载荷；不需要无损u32壳再制造位转换。
- 前缀空向量路径与非空正值全通过路径不同；未知业务语义不能用全0补齐。compute原真实empty初始化及interpolate移动这同一向量应保持。

拟现代字段名`auxiliary_samples: Vec<f32>`。SampledStairsPoints第四Vec**尚未证明与Params此Vec转接**：原interpolate fresh路径将其初始化empty（stack190=0/188=ptr4/180=0），reuse路径仅保留旧第四Vec；不能按同为4B机械改f32。候选继续以auxiliary_words:Vec<u32>明确观测位容器，等其真正producer/consumer再绑定具体数值类型。两者精确业务专名仍未知，后续完整mesh算法再根据真实业务读写命名。该补充待parent独立读上述完整原slice复核后方进owner candidate。
