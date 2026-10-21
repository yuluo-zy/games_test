# Duck所需WaterAudioGrid借用采样桥合同

沿TG-5ac8da716b7e89155d旧claim boot-audio-lifecycle-20261019获root窄委派，live show保留。当前water_ambience_audio.rs.before及utils FloatGrid源SHA见source-before.json；本轮不改变canonical grid/cells/滚动算法、音量算法或卸载。

原B PDB4514:264将RVA0x261f0c0绑定到`utils::float_grid::bilinear_sample_at_world_xz<WaterAudioGrid>`，完整799B；1409ebe80完整151B是该grid.load_index_unchecked。两者均由native-evidence-pack从原发行RNE f507/PDB2d968直接校验全指令字节，report.json两项ok，无原指令执行。前者明确读取14362c308间接采样policy，分居中/legacy两路，行列10，索引范围9，四点调用1409ebe80并沿z、再沿x的a+(b−a)*t。后者读取canonical cell.bits.count_ones()/64，保留Vec边界panic，不取position/sums。

已有utils::float_grid提供动态尺寸FloatGrid/FloatGridSampler，固定512包装FloatGridCells不是此10×10网格接口。必须实现`impl ::utils::float_grid::FloatGrid for WaterAudioGrid`返回[10,10]，load直接委托现WaterAudioGrid::load_index_unchecked，然后inherent `bilinear_sample_at_world_xz(&self,x,z,extent_x,extent_z)->f32`调用现FloatGridSampler，读现CENTERED_PATH Relaxed；不更改/复制双线性实现，不改变进程policy，不创建buffer/cells镜像，不混用WaterRaster。

与duck worker已确认API四参数；其真实typedwrapper第三参数为Res<WaterAudioGrid>，原move音频两分支142313459/4d0调用此原函数，跨度130/130。采样后MAXSS(sample,0)把NaN取0，再sqrt，由duck worker保持消费者顺序；core桥不提前clamp，否则会改变legacy正边界外推（可大于1）和原NaN路径。

方法纯共享借用，无事件/资源替换/副作用/历史修改。canonical grid未初始化/长度不足仍由现load边界失败，不返回0或default。原浮点顺序/NaN策略复用现共享算法：legacy夹[0,1]后n尺度、floor限制n−2并保正边界fraction2；centered先上限1、n尺度−0.5及下限0、相邻索引夹n−1。B算法静态比对支持桥，不把既有512 oracle当10格执行证明，也不升级C/游戏实际音频验收。

最窄候选仅同一core源增加trait impl和API，中文注释。验证使用非对称popcount ramp，明确[10,10]和z/x顺序，origin legacy35/64与centered31.5/64，正边界legacy70/64与centered63/64；API只读现global，测试显式trait两路不写global；位置/sums污染不影响density；missing cells仍panic。root执行Cargo及真实App，子agent不执行Cargo/rustc/GPU或生产写入。
