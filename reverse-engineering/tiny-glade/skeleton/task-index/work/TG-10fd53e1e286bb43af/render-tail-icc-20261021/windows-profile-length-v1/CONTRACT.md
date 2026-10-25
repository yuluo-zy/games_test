# Win32 ICC 查询返回长度 V1

root明确ICC完整producer必要helper授权，复用原ICC work，不变更原helper任务claim、不写生产。

## 原证

B get_monitor_icc_profile@1403ac400。新增native-pack-with-monitor-length中 `icc-monitor-buffer-length` 82B逐指令核原字节：1403ac625读取DWORD返回size，1403ac62b..635 min(size,0x800)，1403ac644写Vec.len；随后1403ac653调用140d79ce0。callee独立symbolizer确认 CString::from_vec_with_nul（alloc/ffi/c_str.rs669）。原二进制/PDB身份同主ICC合同。此次7请求5命中2新增，3分析进程，未执行目标。

## 当前真实反例

现代码vec![0;2048]传Win32，GetICMProfileA成功写长度较短的NUL结尾路径；随后直接把整2048个字节交CString::from_vec_with_nul。有效路径的结尾NUL变成interior NUL，因缓冲后面仍有零而报错。查询层因此无事件，纵使核心ICC恢复也不会到finalpost。不是扩大容量可解。

## 最窄恢复

从成功分支提取安全decode函数，先Vec::truncate(returned_len)，再原CString校验和UTF8校验。截短等价原min，不扩充/读越界。不变更CreateIC/GetICMProfile/DeleteDC调用、原query失败保旧profile合同、原CString错误上下文或DC异常路径。

函数cfg(any(windows,test))，Win32运行用同一实现，Mac可运行4个纯测试：正常短路径原失败/新成功；缺NUL和interiorNUL/零len保错误；超容量返回len钳制；无效UTF8保path错误。无新增unsafe、默认路径、平台假成功或新依赖。

未Cargo/Windows/GPU执行。纯测试不证明真实Win32API或OS事件已接；tracker/hook仍分阶段待整合。现已知原DC两条CString失败路径泄漏保持，首包不顺带扩大策略。
