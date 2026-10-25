# W13—W17 原版业务与现typed host接线候选

本交付只在task-index合法工作目录写候选；没有修改production，没有运行Cargo/GPU。B原函数的业务声明复用现country-core计算pass，current Rhapsody架构/严格guard保持。不存在ResourceView或布局cast，也未建立任何GPU资源替身。SPIRV-Cross输出仅为辅助审计的派生GLSL，不是恢复出的原HLSL；发行SPV的OpSource为空白。

## 集成文件与边界

1. `world_vegetation-new-module.patch` 新增tiny-glade `world_vegetation.rs` typed入口；root在tiny-glade lib导出模块，并在自己世界图实现的W13/W14/W15/W16/W17五方法分别调用同名入口。已有render_world原顺序保持：W13 → W14 → W15 → W16 → W17，原无返回结果的声明现在把真实图admission错误沿当前Result传播，不伪造完成。
2. W13只改已获render_present协作授权的TG-60ea7c6fe1579df293/render-water-live-20261018/boot-vegetation/compute_water.rs.patch，不改同文件W2/ctor。W13原3246B完整函数@140a13970：demo全局先返回；非demo从真实ActiveSession.meta.glade.native_tag()==3生成is_autumn，225²睡莲+150²芦苇。当前host将既有GameConfig.dev.demo映射该原启动配置，不改global或虚构Steam。
3. W14属新合法claim TG-c15aa9d784e5b9fae6，文件`spawn_vegetation_around_buildings.rs.patch`。原@1408e4cc0/3394B TypeId3dce36237fb50dff/93a27cf4512d885c的+20已确认为canonical GladeSettings.vegetation_around_buildings。typed bool false在查GPU库前返回；true声明90²灌木、150²花。
4. W15属新合法claim TG-6b43449ef07afe08ea，增加read_bush_positions_for_cpu_with_building_prefix入口，typed host用canonical BushSsbo/BushCpuSsbo/DrawIndexedIndirectBuffer。false gate按原shader不读取Bush记录，使用root待合通用inactive_storage_buffer_contents方法；旧调用保留true默认。原@140866540/696B无gate、dispatch9601。另提取发行read_bush shader证实输出真实布局。GPU提交/CPU可读时点不由此声明函数捏造。
5. W16只改已获render_present协作授权的TG-f66f0656cb72f29658/render-garden-live-20261018/boot-vegetation/compute_garden.rs.patch，不改同文件W12/ctors。原@140a22b80/5234B四pass为90²灌木、150²花、180²marching stones、400²小花。
6. W17属foreign TG-74d7679193a92a0e8c，未抢claim，未写production/覆盖候选。`W17-FOREIGN-REQUEST.md`提供原SPV支撑的精确一行保留标记；该项未合前不宣称W17已闭合。

## 内容依赖（不能仅按concurrent group猜）

| Stage/pass | 真实输入 | 输出/保留规则 |
|---|---|---|
| W13 lilypads | presets9/A/13/5/14；ActiveSession季节；FrameConstants真实delta；preset17旧动画 | Lilypad、LilypadFlower完整记录首次生成；draw slots8/15 instanceCount atomic RMW；preset17 imageLoad→mix(旧值,mask,1.5*delta)→imageStore，必须preserve |
| W13 reeds | presets9/A/13/5/16 | Reed完整记录首次生成；draw slot9 RMW |
| W14 bushes | 地形9/A、exclusion E/13/5/10、garden14、building15/16 | Bush由slot10追加、Flower由slot11追加；这是本链Bush/Flower首次位置producer；draw packet保留 |
| W14 flowers | 同一实际mask集合；原代码重复绑定地形保持 | Flower继续同slot11 prefix，必须保留前一pass花；draw packet保留 |
| W15 read bushes | canonical Bush+draw slot10 | BushCpu row0两个count原始bits，row1..count是Bush变换平移XZ，tail未读；count0不fetch Bush，但row0仍写；不能伪造CPU完成 |
| W16 bushes | 地形9/A、exclusion、garden14、building16 | W14=true时Bush/Flower继续相同slot10/11并保留prefix；W14=false时不保留旧位置，作为首次producer；不是固定分区 |
| W16 flowers | 地形9/A、exclusion、garden14、building16 | Flower继续slot11 prefix，保留前序内容 |
| W16 stones | exclusion、garden14、地形9/A | GardenStone完整记录首次生成；draw slot14 RMW |
| W16 small flowers | 地形9/A、exclusion、garden14、building15 | Flower03完整记录首次生成；draw slot16 RMW |
| W17 pebbles | rockyE、地形9/A、path6、water13 | PebblePlacement完整记录首次生成；draw slot3 RMW |

所有draw packet保留指原SPV只原子增加instanceCount，不覆盖indexCount/firstIndex/vertexOffset/firstInstance与其他slot。本交付没有自行清零计数、用CPUcount代替GPU计数、把整个buffer标记为有效像素/实例。真正reset/prelude必须在同图以前生产原初始化packet；若缺失，必须保留真实失败，不补fill来蒙混。

W14 gate=false时，W15按真实slot10=0不会访问Bush记录，W16从slot10/11=0首次追加。候选此分支W15将Bush绑定内容标记inactive（保留原binding/access/sync），W16首pass不preserve Bush/Flower；W16第二pass始终preserve前一pass的Flower。仅真实counter reset证明slot10=0后才合法采用该inactive标记；若reset缺失，不得调用false入口掩盖错误。current图不能因import或元数据而把None buffer当成已初始化。本交付不扩大任何allow_undefined合同。whole-buffer读声明不等于shader读取tail，实际仅counter限定prefix；缺少真实reset仍是硬错误。

## 动画首次生产者已核

Preset17不是无初值纹理：已存完整原RNE证据`TG-c459c7f73345e24709/main-chain-boot-20261018/world-terrain-20261018/PRESET10-ZERO-PRODUCER.md`及6个完整asm说明全部25项preset均走tag1→init_texture_data→真实color/depth clear0。动画17同属该fold，不另加测试/生产clear；后帧保留真实旧值。当前install_asset_libraries→try_create_preset_textures→TextureHost→现Rhapsody create_texture_zeroed已有同语义的真实owner链。

## 验证边界

四份core patch在交付时`git apply --check` exit0，beforeSHA等于当时current productionSHA。现有声明测试内增加真实RMW/共享prefix与动画view metadata断言；没有运行测试。root后续统一编译，并用真实设备/原SPV/完整前置资源验证这10个compute dispatch（demo/gate会减少），包括W14false/true与同一App跨帧动画。GPU测试不得用empty world或假相机填资源；这些shader会读FrameConstants，必须已有真实startup/prepare producer。只有GPU返回且内容有限，不足以声称植物视觉正确或整个54段世界图已恢复。

原证文件路径与SHA、patch SHA、生产beforeSHA见delivery-manifest.json/core-candidates.json；W15额外发行流模块offset23/size6784见其原shader manifest。
