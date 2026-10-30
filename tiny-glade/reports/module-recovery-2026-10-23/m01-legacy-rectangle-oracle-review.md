# 两个旧矩形 oracle 测试的独立复核与期望恢复

2026-10-27。范围仅 `geometry::completion::original_completion_rectangle_deformation` 和 `original_completion_rectangle_intersections`。不改 Rectangle2d/Rectangle3d 业务实现，不扩大审计全几何库，不删测试、不跳过case、不放宽任何数值比较。

## 可复现失败

中央在原相机差分通过后执行utils全lib：179 passed、2 failed。原始失败已保留 `m01-legacy-rectangle-oracle-before-tests.log`：

- deformation：得到0.68282175（3f2ecd68），测试却期望0.7628218（3f43484a）。
- intersections：同一矩形相交实际false，测试期望true。

这两测试实际由utils/src/geometry/mod.rs包含 `tests/oracle/foundation-completion-2026-10-13/geometry/native_tests.rs`。不能把未参与此include的同名fixture镜像或历史进度当当前测试内容。

## 调用合同与原版映射

实际读了原 `evidence/foundation-completion-2026-10-13/geometry/oracle.py` / generate.py，以及共享native_oracle/boundary_oracle的Win64 ABI、内存/数学桥。映射并没有错：

| 当前被测函数 | 旧oracle实际VA | 独立PDB768完整bytes | 原源码行 |
| --- | --- | ---: | ---: |
| Rectangle2d::expand_along_edge | 140c9c620 | 258 | 1054 |
| Rectangle3d::rotate_projective_xz | 140c984f0 | 41 | 48 |
| Rectangle2d::intersects | 140c9b990 | 1015 | 674 |
| Rectangle2d::intersects_circle | 140c9b6e0 | 677 | 659 |

LLVM独立符号化、PDB模块原过程记录以及本轮四组全范围native evidence都再次核准。发行RNE SHA为f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03，匹配PDB SHA/GUID/Age见相邻manifest/report。原fixture JSON也绑定同一RNE，不是不同B构建差异。

## 独立直接执行裁定

`probe_m01_legacy_rectangle_oracle_x86.c` 仅按原ImageBase映射发行PE全部section，通过ms_abi直接调用以上四个完整原函数和实际正常callee。没有几何或数学替代函数，只有原panic出口可观测桥。本轮180输入均未panic。驱动脚本先核原RNE SHA，读取原fixture的80 deformation和100 intersections输入位，不从Rust候选取输出。

Rosetta x86_64完整原执行的180条结果，与最初原证据 `reverse-engineering/tiny-glade/evidence/foundation-completion-2026-10-13/geometry/native-fixtures.json` **180/180完全相同**：expanded6word、rotated8word、rectangle/circle两个bool都一致。证明不是当前新业务错误、不是VA/ABI映射错误，也不是最初生成器的数值失真。

问题在当前Rust oracle测试的期望曾被改写，与保留的最初原JSON及原evidence/native_tests.rs不一致。此次只恢复被改写的26个期望字段：25个expand case、1个rectangle bool，其余155条case期望原本一致。全部180输入、调用次数和原测试比较函数保持不变；rotate输出与circle bool均不改。

明确反例：deformation第1行（零基index1）原输入lower=0x3e4ccccd（.2），原输出size.y=.2、center words1060031848/1070009354。当前被改写测试要求size.y=0、center1061374026/1069506038，造成差值.08；这不是末位舍入差，也不能靠增大epsilon解决。原258B中140c9c649/66a MAXSS保下限、64d/66e MINSS保上限，现业务与原一致。

intersections第2行（零基index82为全180编号）的两个矩形完全相同，原边界相交谓词返回false，当前被改写测试要求true。它不是完整体积overlap合同；现库另有overlaps_rectangle2d。此处不能凭直觉把原严格边界谓词改成SAT或包含判定。circle同case原true仍保留。

完整逐条输入、最初原期望、本机原执行输出、hash与差异见 `m01-legacy-rectangle-oracle-direct-replay.json`；其中 differs=false 比较的是原JSON，不是被修改后的Rust断言。当前错误Rust断言改前/改后26处对照见 `m01-legacy-rectangle-oracle-test-corrections.json`，原错误测试完整副本见 `m01-legacy-rectangle-oracle-test-before.rs`，不抹掉失败历史。

## 修改归属与验收

诊断先发中央，中央同意后，仅定点修改现 `crates/utils/tests/oracle/foundation-completion-2026-10-13/geometry/native_tests.rs` 两个测试函数的期望字面量，补中文来源说明。**没有改任何rectangle.rs业务代码，没有改原证据JSON/历史fixture镜像，没有删测试、case或放宽close容差。** 原有2e-5比较维持原状；本轮原执行产出的JSON则逐word比对原JSON。

原2612项expand与5111项intersects严格差分并不因此重算或被本轮替代；本轮单独复核旧180输入，正是为辨别旧测试与新业务不一致的原因。最终当前utils窄组与全部lib结果见 `m01-legacy-rectangle-oracle-validation.json` 及对应日志。

本轮为原指令有界执行和Rust CPU测试；没有运行完整游戏、GPU或OS输入。脚本首次路径构造错误把原版定位到Desktop/B版本内容，FileNotFound后改为game/B版本内容再执行；未因此修改任何输入/期望或候选业务。

## 经验

1. 失败oracle先核实际include，再核原raw证据。标为oracle的Rust文件可能已被手改；同名fixture镜像不等于参与编译的断言。
2. 输入、VA、原B身份全部相同仍可能是测试期望漂移。应独立运行原程序指令，比较三方：原JSON、当前测试期望、当前业务结果，而非只比较后两者。
3. 直觉中的几何“相交”不是原业务合同，包含与边界相交要沿原函数分别保留；不能为了旧手改断言改变已严格验证的原实现。
4. 恢复期望须保原失败和逐条更正记录，不能删/忽略旧测试或放宽浮点容差。此轮25条差.2尺寸和1条bool差异都精确绑定原指令，不依赖候选输出填答案。
