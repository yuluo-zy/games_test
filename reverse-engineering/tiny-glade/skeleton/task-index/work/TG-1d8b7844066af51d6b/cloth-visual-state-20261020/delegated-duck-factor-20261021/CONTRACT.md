# 原时间连续插值因子40B必要共享叶

root 本轮明确窄委派，保 TG1d8 cloth-visual-state-20261020 旧claim。本次 before 是当前root已修改utils/lib.rs 823ed9c9372bcf1e7e1c5feb9df5a4f427749b6ce3e434ebec0aae5483b32dbe，不回退旧root方法。只加邻接函数及该叶测试，四参lerp/exports/mod保持逐字不变。

原 utils::time_continuous_lerp_factor VA140c90540、PDB759:4276、40B；完整原pack在 TG0ae3/core-duck-building-20261021/interpolate-required-readonly/native-pack/continuous-factor-full.{asm.txt,json}，manifest绑定 B RNE f507a6ce/PDB2d9686b8。XMM0先 XOR负号mask1429258f0（浮点逐位负），再MULSS rate(XMM1)，调用原UCRT expf142923ec0，读1.0常量142925904，以1.0-expResult输出；无clamp、dt正数/finite门、Result/log副作用。

必要真实consumer interpolate1407cf500：cf7dda/7dd读GameTime+28 delta；cf7de2读142a7ba84=9.0(bits41100000)，cf7dea CALL此叶。该值用于old→new位置/切向插值。不能用四参lerp(0,1,..)增加mul/add而省略此接口。

候选最窄pub fn(delta:f32,rate:f32)->f32 { 1.0-(-delta*rate).exp() }，保所有f32顺序与唯一负号。正负0 delta都exp(±0)=1，结果+0；负delta/负rate会为负，不能钳0；乘法overflow按f32到±Inf，exp得到0或Inf，输出1或-Inf；Inf*0、NaN照原无检查传播NaN。runtime的UCRT与本机exp backend NaN payload/exception flag及所有末位未独立证明，不把is_nan测试当原bit oracle；C尚未独立验证。

源码测试分正常/负步长/负零/overflow/NaN域；不跑Cargo/rustc，root唯一验证。函数无分配，最窄生产 hunk 依 root 实际before匹配后集成，不由duck写生产。
