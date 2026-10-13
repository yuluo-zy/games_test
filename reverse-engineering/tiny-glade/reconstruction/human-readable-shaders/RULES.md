# 可读化重写规范(RULES)

本目录是 `shader-source/shaders`(转译/重建 HLSL)的人类可读重写版。每个文件的
重写必须遵守本规范,并在完成后通过 `readable_check.py` 校验。

## 目标

把 SPIR-V 编号变量(`_67`、`_123`)的转译代码和已手写代码,统一重写成:
- 局部变量、函数、注释都符合规范,注释一律**中文**;
- 可读性高(语义化命名、分节注释、文件头说明);
- 性能高(编译产物与原版逐指令一致,行为不变)。

## 目录结构(镜像原树)

```
reconstruction/human-readable-shaders/shaders/
  <pass>.cs.hlsl / <pass>.raster.hlsl / <family>/...
  inc/... 等 include 与 shader-source/shaders 同名同相对路径
```

include 根 = `reconstruction/human-readable-shaders/shaders`,相对 `#include`
(如 `"inc/common.hlsl"`)保持与原文一致,使新树可独立编译。

## 文件重写规则

对每个文件:

1. **读原文件** `reconstruction/shader-source/shaders/<path>`,先理解业务逻辑
   (文件头证据注释、cbuffer 成员、入口、变体、算法)。
2. **写新文件** `reconstruction/human-readable-shaders/shaders/<path>`:
   - 文件头加中文说明:着色器用途、入口函数(vs_main/ps_main/cs_main)、
     变体(TGV_ 哈希)对应关系、关键绑定(cbuffer/纹理/UAV)。
   - 语义化重命名局部变量:转译的 `_NN` 编号改为描述性英文名(如 `_55`→
     `radius`、`_63`→`i`、`_91`→`out_of_bounds`);函数参数同理。
   - **临时变量约定**:大型 fidelity-gated 转译文件(water_plane、deferred×4、
     generate_strokes、trace_gi_candidates 等 8 个关键大文件)已完成全量语义化;
     其余大文件的深层机械表达式(哈希链/噪声晶格/SSA 中间量)允许保留
     `vNN` 统一临时变量约定(顶层/关键变量必须语义化),并在文件头注明该边界。
     改名永远不改逻辑,`readable_check.py` 仍须 PASS。
   - 函数/代码块/非平凡表达式加中文注释,说明"在做什么、为什么"。
   - **必须原样保留**:
     - 变体守卫结构:`#if defined(TGV_<hash>) && (__SHADER_TARGET_STAGE == ...)`
       与 `#endif` 的层次(构建管线靠它选变体);
     - include 相对路径、cbuffer/常量缓冲的 register/space 绑定、纹理/UAV/
       sampler 绑定、入口签名、`numthreads`、`groupshared`;
     - 全部算术表达式、常量、控制流(循环/分支/early-out)顺序。
   - **禁止**改变任何算术或控制流;**不要**为了"好看"提取函数/改写循环
     (dxc 会改变代码生成)。改名+注释+排版足够。
3. **混合文件**(既有 `==RECONSTRUCTED==` 手写段又有转译变体段):
   - 手写段:补中文注释、微调命名,不改逻辑;
   - 转译段(`#if defined(TGV_...)` 内):做编号变量语义化 + 中文注释。
4. **头文件**(inc/ 等):加中文头注释 + 函数中文注释;逻辑一字不改。

## 校验(必须全部通过)

命令(在 `games_test` 根目录执行):

```sh
python3 tools/render-passes/readable_check.py \
  --orig reconstruction/shader-source/shaders/<path> \
  --new  reconstruction/human-readable-shaders/shaders/<path> \
  --manifest evidence/render-passes/validation/readable-manifest.json \
  --hlsl <path> \
  --new-include-root reconstruction/human-readable-shaders/shaders \
  --out /tmp/rc_<something>.json
```

- `summary.verdict == PASS` 才算通过(`pass == checked`)。
- 判定原理:原版与重写版用同一 dxc 命令编译,剥离全部调试信息后哈希必须一致
  —— 只改名/加注释永远一致;改算术/控制流一定不一致。
- `full_ok` 覆盖头文件重写:重写版用**新 include 根**编译,仍须与原版哈希一致。
- 若 FAIL:把改动退回为"纯改名+注释",重新校验,直到 PASS。禁止用放宽校验
  来掩盖逻辑差异。
- 头文件自身无独立 bin:以同组着色器文件的 `full_ok` 为准,重写头文件时只做
  注释/改名。

## 完成报告

每处理完一个文件,在最终报告中列出:`路径 | PASS/FAIL | 变体数 | 备注`。
所有文件通过后再回报。
