# xz_rotation_from_forward：完整607B合同

仅utils/lib.rs必要公共leaf。原B140c913e0/607B，PDB759:11376；完整原指令和PE bytes复用TG6613旧work/readonly-water-contract/xz-rotation-from-forward.asm.txt（SHA70272a9a）与TYPED_WRAPPER_AND_FORWARD_BASIS.json（SHA5a0843fe）；原body SHA81f92a06431131d44f5bf1bab8bbea98e13d219298818972363688225f555dfe。实际调用是cache terrain63 1408166a6；roof63仍bad 140815ee8。不能按名字将二者合并。

## 参数与全部数值路径

`pub fn xz_rotation_from_forward(direction: glam::Vec2) -> glam::Quat`。原RCX outQuat，XMM1/XMM2两个方向分量，无Result/Option/资源读写/分配。

1. 原140c913f6..1455算方向平方和、sqrt、1/length。原bits门接受正且finite的倒长度，包括正subnormal，排除±0、负、Inf、NaN；成立乘两个原分量，否则选择(+1,+0)。直接复用pinned glam0.29.3 `Vec2::try_normalize().unwrap_or(Vec2::X)`同判断。零、有NaN/Inf、有限平方溢出、极小平方下溢等均fallback，而不是earlyErr/panic或返回identity。
2. forward=(normalized.x,0,normalized.y)，up=Y，right=up.cross(forward).normalize()。右轴的第二次归一化不能省掉；不以atan2 yaw代替。
3. 原140c914cf/14d9翻right与forward signmask；实际PE1429258f0四lane80000000。最终旋转矩阵按**列(-right,up,-forward)**。1429cd0c0=(0,0,1,0)是SSE重排中间常量，不是最终up=Z。对应返回Quat把本地 -Z 指向调用方方向。
4. 用同版 `Quat::from_mat3(&Mat3::from_cols(...))`，复用整个依赖矩阵→quat四分支，不手工简化成只有yaw，也不normalize返回quat改变符号。当前依赖源码 sse2/quat.rs:198..270 直接对照：

| 原门 | 原分支地址 | pinned glam分支 |
|---|---|---|
|m22<=0, dif10=m11-m00<=0|140c915df..61f|four_xsq=(1-m22)-dif10，x dominant|
|m22<=0, dif10>0|140c91557..59a|four_ysq=(1-m22)+dif10，y dominant|
|m22>0, sum10=m11+m00<=0|140c9159f..5dd|four_zsq=(1+m22)-sum10，z dominant|
|m22>0, sum10>0|140c91509..53b|four_wsq=(1+m22)+sum10，w dominant|

每支均用.5/sqrt(four_sq)，原包含其余三lane加减乘与分支选定符号；直接复用pinned依赖全支。有效平面输入主要到Y/W支，但不能以此删X/Z依赖分支；几何断言开启时glam仍保其规范化检查。库在架构间的NaN payload/FP控制字差异未独立验证，不谎称native bitwise全证。

## 不能采用的捷径

`xz_rotation_bad(-raw_direction)`错误：raw0/NaN/Inf将先在bad里fallback+X，缺原fallback之后翻向；`bad(-normalized)`又会重复normalize造成微小变化。原bad的矩阵列为(+right,up,+forward)，新函数保原独立方向策略和一次输入归一化，不能改已通过的bad路径。

## 源与测试边界

before是当前utils/lib.rs真实8011a64cbc9e738e996dfd859bd82ab7a4cbf16f55d01aad5ee45b6bb643722c（含已提交factor）；只增该函数及其cfg(test)边界测试，保全部既有字节。新注释中文。建议几何契约样例覆盖四象限、对角、正负轴，本地-Z映射方向、上轴保持Y、Quat选定Y/W符号；zero/±0/NaN/Inf/MAX/MIN_POSITIVE fallback与原+X结果逐bit一致。测试不是原程序oracle，source/patch需独审，root执行。
