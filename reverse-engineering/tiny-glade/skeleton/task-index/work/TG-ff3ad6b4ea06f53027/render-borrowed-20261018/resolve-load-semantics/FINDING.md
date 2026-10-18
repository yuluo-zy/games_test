# 原 resolve 的 1:1 texel fetch 与测试假设纠正

实际发行 compiled container 两个变体的SHA与严格顺序stage解包记录见identity.json；没有搜索任意偏移magic。提取的PS/VS原SPIR-V经spirv-dis保存，不依赖重建HLSL猜语义。

非depth变体 b903e43ffb3da915 PS：OpMemberName Globals[0]=transform_image、[1]=framebuffer_dims，Offset分别0和4。主体把FragCoord.xy OpConvertFToS为int2；当transform_image&1非0，y=framebuffer_dims.y-1-y；最后OpImageFetch(input_tex, coord, Lod0)。没有OpImageSample或源/目标尺度换算。FNV binding key transform_image=7b01c6bb382d8849，与原native closure立即数相同，先前报告中把此键写成render_upside_down是命名误述。

因此原测试4x4源→16x8目标后断言像素(2,4)为红的假设错误，该坐标已超源范围。不能通过添加stretch修代码，也不能把该黑色像素判定生产resolve实现错误。

经root明确授权，测试改成32x16源→16x8目标，所有可见采样坐标合法；源顶8行以红/蓝与黄/青分区，源后8行纯绿。flag0检查直接整数fetch；flag1检查target.height=8的翻转。错误传source.height=16会取到纯绿，错误缩放也会改变采样分区。测试仍调用同一原core resolve consumer、原发行shader和真实GPUreadback，未改变渲染实现或添加新clear。修正结果待统一build/run，不预填PASS。
