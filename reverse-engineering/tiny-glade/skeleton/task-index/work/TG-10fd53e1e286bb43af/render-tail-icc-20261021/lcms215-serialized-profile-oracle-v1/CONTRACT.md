# LCMS 2.15 原公开API数值参考 V1（尚未运行）

root实测core8为6pass/2fail，实际sample31=[65535,0,6,255]，并非错误重排。原代码严格复用new_file→new_srgb→Transform RGBA16/Perceptual/flags0且grid测试过；失败测试的fixture把内存new_srgb保存到ICC再重开，却错误断言与另一个内存new_srgb数学恒等。

原锁定sys4.0.1 vendor cmsvirt.c101..221/cmsCreate_sRGBProfileTHR643..672先从双精度白点/primary/曲线建立内存profile。cmsplugin.c337..367 `_cmsWrite15Fixed16Number`/`_cmsWriteXYZNumber`将颜色数值编码为固定点；cmstypes.c335..369的XYZ读写dispatch和1279附近parametric curve读也有原固定点量化。所以 serialized profile 与 in-memory profile 不应先假定1LSB以内。这里只是静态高置信原因，不能以其自行增大容差。

本C参考直接链接root已经构建的lcms2-sys vendor2.15静态archive，不链接本机2.19。对照相同输入memory-sRGB与memory/serialized-reopen两个目标。两个分支均按真实32³/u16/alpha255原地cmsDoTransform，flags0。输出完整rgba16le、精确全字节FNV1a64、max差及index、sample31；另打印三个XYZ tag与半值TRC差。root运行后须保存archive/header/compiler身份、原输出/sha。这里不执行游戏，也不是重新实现LCMS。

RUN_ARGS.json仅本地诊断argv，非production Cargo路径。child没有执行compiler/Cargo/GPU/oracle。不要据静态原因直接放宽旧<=1断言；root实际oracle确认后，改fixture断言为明确memory-vs-file量化合同和精确oracle值/全grid指纹，生产函数字节不变。不同架构/编译flag可能需要独立numeric binding，不能把M3参考冒成原Windows目标执行。
