# Rhapsody parallel recovery batch 05

日期：2026-10-12。继第四批整合检查，继续推进原作私有源码和CPU/API边界。

本批主流程门禁 **27/27通过**，所有输入/源码/依赖hash保持当前版本。三包合计6138个新增原指令/边界案例零差异：caller180、record774、pipeline5184；其中完整入口与注入片段分别登记，不据总case数声称整体实现完整。record与pipeline候选测试5+2项，fmt/严格Clippy通过。本批三个agent均完成交付。

| 工作包 | 范围 | 证据入口 |
|---|---|---|
| queue-callers | 主队列直接调用者、pool selector与queue tag区分、ensure_active_cb | [findings](queue-callers/findings.md)、[validation](queue-callers/validation.json) |
| record-resources | 非空资源、初始/pre/export scopes、callback与managed track、record到submit | [findings](record-resources/findings.md)、[validation](record-resources/validation.json) |
| pipeline-descriptors | persistent layout私有入口、真实Vulkan create info边界、pool与compute layout | [findings](pipeline-descriptors/findings.md)、[validation](pipeline-descriptors/validation.json) |

三个独立Rust工作包在 `reconstruction/render/work-packages/`。这些候选保持外部设备/同步/用户callback接口，尚未并入可运行GPU后端。

源码入口：[queue-callers](../../../reconstruction/render/work-packages/queue-callers-r7/src/lib.rs)、[record-resources](../../../reconstruction/render/work-packages/record-resources-r7/src/lib.rs)、[pipeline-descriptors](../../../reconstruction/render/work-packages/pipeline-descriptors-r7/src/lib.rs)。

工具优化和第四批整合由另一个agent执行，入口为 [工具审计](../TOOLING_AUDIT_2026-10-12.md) 和 [第四批统一门禁](../parallel-batch-04/integration.json)。共享持久数据库首个窗口属于pipeline agent；另两条源码/原码路径复用既有证据完成，避免数据库争用。

验收区分完整私有入口、注入边界、纯候选测试、shipped接口候选adapter。原码oracle使用原发行机器码的CPU模拟，并不执行live Vulkan。没有从文件数、case数或反射库存覆盖率推算rhapsody/整游戏百分比。

主流程统一门禁写入 [integration.json](integration.json)。完整queue前段/并行录制、多队列实际使用、temporal reset/drop、pipeline完整创建和country-core真实pass绑定仍按实际证据登记；M12保持 `verified_partial`。
