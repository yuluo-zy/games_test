# 完整分页组 writer 审计（V1）

此附件对应 MANIFEST.json SHA `9accc593aa5fead871865cf18a80ff6267c105a43ebd6db56bae4741536c7b3a`。仅做静态审计，Cargo/GPU 由 root 执行。

| 路径 | writer 与准入 | owner/发布 |
| --- | --- | --- |
| 光栅 callback | `raster_host::bindless_parameters` 收集最终覆盖资源，转 `ShaderBindingSnapshot::prepare`，完整组 `require_slots` 后统一写入 | callback 保持一个 cursor；原 pipeline 用共享 Arc layout；prepared/bind_pass 保持实际页与租约 |
| 计算 callback | 首次冻结 `snapshot()`；资源数组只收集一次。首次完整准备；同页 push 只增 1 参数；迁页从同 snapshot 与实际资源重放 | previous prepared 与录制命令各自保持必要租约；不读取第二个 FrameGlobals 指针 |
| FrameGlobals | `prepare_page` 只准备页身份、缓存命中和缺失数；`write_to` 仅从上述 `prepare` 的 per-page guard 内调用 | b0/b1 共用声明角色 storage lease；64 个纹理不逐 draw 重写；参数 finish 成功后 commit |
| sampler | `SamplerPages::prepare` 只预留元数据；`SamplerPreparation::write` 仅从 guard 内调用 | cache 缺失计数按 name+实际 Arc sampler；所有 fallible 步骤结束后 commit；旧租约在 cache 锁外释放 |
| indexedCount GPU 适配器 | `ShaderHost::indexed_count_adapter` 改走 `new_paged`；实际 callback 取得 source/count/scratch，按实际去重键算最多 3 资源+1 参数，统一准入 | 不加入额外 globals；原 ceil(maxDraws/64) 分派和业务计数不改；旧 `new` 的单 heap API 兼容路径保留 |

模板 `ShaderHost.heap` / `ShaderBindings.heap` 在新生产 shader 路径只供编译布局、ABI 和兼容诊断；不再承担裸 storage/sampler 写入。`TextureHost` 的持久资源目录是已有独立 heap，不是 ShaderHost 页 writer；本补丁不变更该目录或裁剪实际 draw 资源。

锁序为 page.group → metadata cache（短锁、无 native writer）→ Slots（实际 writer 内部），dynamic constants 只在 manager 释放且 page.group 持有时使用。pool manager 只选取/持有页记录；new_page、设备 poll、Slots、cache 和可能最后 owner 的销毁都在 manager 锁外。缓存准备持有 Arc 命中租约，即使 sampler 配置失效也不会让其数值下标失效。

`require_slots` 依赖合作 writer 都经 per-page guard；该对象不会独立预分配占位槽。现有生产闭包中的所有 shader writer 已按上表迁入。Prepared 返回后 guard 可释放：其租约已是 Live，其他 writer 只能占用空槽；GPU 使用开始前的所有权由 Prepared 维持，开始后还由实际命令完成点维持。

V1 不声称实现页级缩容：页记录和 sampler cache 保留实际高水位，descriptor 槽按实际完成点回收、复用。稳态是否形成平台与场景相关的有界高水位需要真实主链多帧验证；不能把 page_count 增长或 cache-held live 与已证实泄漏混为一谈，也不能直接放宽旧阈值。pool peak 求和是每页历史峰值之和，未表示同时发生的全局峰值。
