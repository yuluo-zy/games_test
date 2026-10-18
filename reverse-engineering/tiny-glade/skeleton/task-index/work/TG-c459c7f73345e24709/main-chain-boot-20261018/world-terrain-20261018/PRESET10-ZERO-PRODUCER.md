# W3 PRESET_VISUAL_TERRAIN_HEIGHT：原producer与首帧初值

本合同回答W3/W4在W5本帧绘制Visual terrain height之前读取preset10是否需要虚构清屏。结论：**不需要；原create_preset_textures对所有preset真实使用zero初值，包括10。** 该事实来自本轮发行RNE完整函数，不把早期资料里的误称TextureInitialData::None当证据。

1. `AssetTextureLibrary::create_preset_textures@14093bd80/342B` 在`14093bdd9`调用原enum_iterator fold `141474b90/842B`。
2. fold对每项先`TexturePreset::from@140a59890`，`141474c94`明确写传入initialdata的判别字=1，然后`141474cbb`调用`GraphicsDevice::create_texture<TextureInitialData>@1416a71f0/2794B`。
3. `TexturePreset::from@140a59890/1933B`的原跳表`142ad35b0`第10项目标`140a59e70`，实际`new_2d([512,512],0x7c)`→sampled→depth_stencil_attachment，hash=`e673b662ce97671c`，即D16_UNORM深度纹理。不是假造的storage颜色纹理。
4. create_texture将该tag1传给`init_texture_data@140bfe270/990B`，其原跳表`142b174fc`的第1项目标=`140bfe2c0`，调用真实setup闭包`140c08560/718B`。
5. setup闭包根据aspect选择depth或color。深度路径`140c08679`写clear值qword=0（depth f32=0、stencil u32=0），`140c086c3`调用device原depth clear函数指针+3b0，范围包含原desc全部mip/layers；前后有原0→34→22屏障。color路径`140c086cd/6d0`明确xorps零颜色→+3a8 clear。同一tag1不是“无initial data”。

本目录完整原证：14093bd80-full-rne.asm、141474b90-full-rne.asm、140a59890-full-rne.asm、1416a71f0-full-rne.asm、140bfe270-full-rne.asm、140c08560-full-rne.asm；preset-initial-data-audit.json含完整原extent/bodySHA与跳表targets；shader-fixture-asset-audit.json含preset10目标与原shaderSHA。

当前现架构同一链已经存在：CoreRenderBridge::install_asset_libraries→try_create_preset_textures→TextureHost::create_texture（sampled且data空）→现rhapsody create_texture_zeroed。其原输入语义与本次tag1证据一致；不修改/复制现rhapsody内部架构。W3 fixture应直接复用此producer，不能为通过另外写一个测试clear。

原W3三个shader族（rasterize、两个Gauss编译变体、derivative）的发行SPIR-V只含set0局部资源，没有set1 FrameConstants或camera读取；shader-fixture-asset-audit.json对四个发行.bin解析实际SPIR-V所有DescriptorSet decorations并记录结果。因此fixture沿用已有原FrameConstants startup default只为宿主共通frame setup，不新增MainCamera或猜identity camera producer，也不声称复现了依赖相机的完整gameplay frame。

W3测试候选只核实际四pass/clean零pass/事件重跑、canonical map与changed tick、真实submit/completion与可读回；原地形noise/视觉图像尚无逐像素oracle，不因checksum可打印就声称画面正确或W4/W5/全世界图已通。
