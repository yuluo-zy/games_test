# 鸭子水声密度采样桥源码复核

复核者：core_business_lead。直接读取当前 utils 的动态 FloatGrid/FloatGridSampler 及完整候选补丁，结合原799B双线性与151B popcount读取合同，批准 root 整合。未编译或执行原指令；测试待 root 运行。

桥只为当前唯一 WaterAudioGrid 实现尺寸10×10的共享读取，复用现 FloatGridSampler 及进程既有居中策略。两个同名调用均显式限定 trait/inherent，未递归；没有重新实现双线性、分配镜像或改写 policy。读取的是 bits 密度，位置与 jitter sums 不参与；不提前夹值，保留 legacy 正边界外推。

三项测试覆盖非对称密度轴序、两采样策略与 NaN、共享只读、缺 cells 原失败，未把512格 sampler 包装误用于10格。before=`5f1d07fc59431f7c459cc1c3a95e1ce5956ad8fd70690c7679af351b11de4f5c`，after=`7a26c15b1076680f59acaac3a1335e6070c95c3cc3966933da2827f994a353ba`。

Duck consumer 必须同步借用本 grid，按原样本 MAXSS0→sqrt 设置起飞/落地音量；本桥本身不代表该 consumer 或真实 FMOD 验收完成。
