# 第四批统一门禁

执行：

```powershell
workspace/python/Scripts/python.exe scripts/render-recovery/validate_fourth_batch_contract.py
```

`integration.json` 是此门禁当前输出；详细命令与完整结果在 `../tooling/batch04-*.json`。检查原 PE 与分析副本、原 Cargo.lock、workgraph/calls/第三批输入及声明 hash、第四批候选 source hash、shader inventory/module span 与 17 项原依赖 source/version/checksum。实际重新编译四个候选 crate 并执行 6+3+4+2 项候选测试，另外执行 3 项跨 crate 合同测试。

对固定的 2,840 个 device counter 输入和 108 个 record/submit 输入，本次同时重新仿真原 CPU 指令、运行当前 release Rust oracle，并核对保留 oracle；不重复调用数据库，不改旧包的 validation。404 个原 shader 文件与 638 个 SPIR-V span 逐一验证 hash；重新调用原 spirq fork 反射，运行 Rust descriptor/push/stage 接口门禁，并从每个 entry 重算 240 个 layout family、402 个 interface family 的 signature/成员关系。计数区分候选测试、跨包测试、原码案例与静态反射，不累计为源码覆盖率。

整合保持 `consistent_partial`。R6A 已证实每队列 counter 初值为 1；R5B 旧 candidate 初值 0 是其明确 fixture，不是原构造证据。跨 crate 测试显示两者的 queue collision、timeline gate 与 record/submit effect 合同相容，但 R5B 尚无接收原 sequence/seed 的生产 API。本测试的有限 `fixture+1` 只用于展示此边界，不能当作完整生产映射或回绕语义。R6B 仍限空资源顺序录制、原 submit 入口和注入 queue 尾段；R6C 为完整发行接口反射，仍不证明算法、实际 pass 绑定或画面。

仍需继续恢复：私有多队列 caller 和资源排他、非空资源/并行完整录制、完整 queue 前段、pipeline/descriptor 创建、country-core pass 选择、temporal reset/drop、真实 Vulkan/GPU 与图像等价。auto-analysis=false 的数据库 xref 空返回或 DATA-only 不证明全程序无 caller。没有修改总进度或 M12 状态；本门禁只是第四批范围的统一验收。
