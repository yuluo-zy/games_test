# 时间连续插值因子40B：独立合同与源码复核

结论：**合同与最窄源码候选通过独立静态审查**。唯一有界增量为 `pub fn time_continuous_lerp_factor(delta:f32,rate:f32)->f32` 及其两个边界测试；未发现函数算序/原门/参数错误。作者 duck，复核者 water。尚未 Cargo/原目标/CPU 数值差分/GPU 验证，不能称运行通过。

候选绑定：当前 production/before `823ed9c9372bcf1e7e1c5feb9df5a4f427749b6ce3e434ebec0aae5483b32dbe` → candidate `8011a64cbc9e738e996dfd859bd82ab7a4cbf16f55d01aad5ee45b6bb643722c`；patch `2075752dd63b2eaebb3150da2dbb6ae1027d2e04decf822447bbf84602e93869`。原TG-1d8b7844066af51d6b / cloth-visual-state-20261020 claim保留，授权委派仅本叶；本复核没有改源码或生产。

## 原40B与参数、消费者

原 B RNE f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03、PDB2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911。独读 retained 完整40B `TG-0ae36c51eb110c07c2/core-duck-building-20261021/interpolate-required-readonly/native-pack/continuous-factor-full.{asm.txt,json}`，PDB759:4276 / VA140c90540 / utils::time_continuous_lerp_factor：

1. 140c90544先对XMM0做XOR sign-mask，140c9054b再MULSS XMM1；即 **(-delta)*rate**，不能改成先乘后整体负、取abs或钳0。
2. 140c9054f调用142923ec0；本次从同PE直接读该6B thunk `ff256af28600`，解析IAT143193130，确为 `api-ms-win-crt-math-l1-1-0.dll!expf`，不是f64 exp、exp2或另一个游戏算法。
3. 140c90554读1，140c9055c执行1-expResult，返回XMM0。完整40B没有finite/dt正数/clamp/错误Result/副作用分支。
4. 同PE直接读常数：1429258f0=80000000 sign-mask、142925904=3f800000 (+1)、142a7ba84=41100000 (+9)。记录 `DIRECT_CONSTANTS_AND_EXPF_WATER.json`。

独读 retained interpolate-full.asm.txt 的实际consumer：**1407cf7dd**从GameTime+28载入delta到XMM0，**1407cf7e2**载9.0到XMM1，**1407cf7ea**调用此原叶。合同内“1407cf7dda/…cf7dea”是地址文字多出一位的笔误，以这三处完整原指令为准；不影响候选函数或输入语义。本报告作为该合同hash `7db6bff7aacf3c947341124132caea41994cf31d26332a1d061fdff2e5c04802` 的独立地址勘误。没有因此扩实现interpolate或改其source owner。

## 源码与浮点边界

候选唯一表达式 `1.0 - (-delta * rate).exp()` 与原三阶段对应：一元负号保留sign翻转、参数和乘积均f32、调用标准f32 exp、最后f32减法。没有转f64、融合乘加、`exp_m1`重排或四参lerp(0,1,...)附带mul/add。标准primitive API无新依赖/分配；保持本仓Rust1.88与现有std实现，不另造数学runtime。

- delta=+0/-0，有限rate=9时exp(∓0)=1，最后输出+0；两个to_bits断言准确，没有把负零输入当异常。
- 负delta或负rate可使因子为负；源码无clamp，测试不允许默默变0。
- 正MAX与正9先乘到-Inf，exp为0→因子1；负MAX与正9先乘到+Inf→exp Inf→因子-Inf。不能因overflow提前返回默认因子。
- Inf*0、输入NaN继续NaN；无静默finite检查。两个is_nan测试只验证分类，不是NaN payload bit oracle。
- expf为Windows UCRT、本机 `.exp()` 为宿主数学后端，所有末位、NaN payload、FP exception flags未作原版运行差分，合同明确保留这一限制；当前未核C。正常数值容差测试也是待运行例子，不冒精确原oracle。

## 其它内容不变与F0复现

独立逐字节剔除candidate新增的连续块，得到的全文**完全等于当前before**。因此四参 `time_continuous_lerp` 的函数/注释/测试、所有mod、exports和root此前加入的其它方法均未变，未用旧lib覆盖root内容。

从当前production实际复制到 `/private/tmp/tg-factor40-water-f0-*`，运行 `/usr/bin/patch -p1 --fuzz=0 -i candidate.patch` returncode0；结果逐字节等于candidate after8011a64c，production仍before823ed9c9。精确临时目录/argv/返回码/stdout/stderr/哈希在 `F0_REPRO_WATER.json`。未使用Cargo/rustc、没有改生产或额外lib函数。

本静态通过允许root将该40B共享叶与真实interpolate消费者编译组配套；原调用边、原资源delta和rate=9必须继续使用同owner。它不代表interpolate、历史清理、cache或楼梯mesh完整业务已验收。
