# RT 原版合同独立复核

Reviewer: os_chain，2026-10-18。只读复核 root 的 `main-chain-frame-20261018/RT-CONTRACT.md` 及原始字节，不改 RT/bind_resources/color_id 或 rhapsody，不运行 Cargo。

结论：合同重点成立，可按此恢复 country-core 的 typed InstanceBuffer 与计数发布。未发现阻止该有界实现的原行为矛盾。两处实现边界需明确保留：constructor `Some` 的容量比较是 `<=`，运行时整批 upload 的比较才是严格 `<`；上传逐个发布，不能改成先检查全部或出错回滚的事务。

## 独立字节复核

直接读取原 `.rne` PE，不只接受报告中的结论。整文件 SHA `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；9 个函数按 manifest 的完整 PDB 长度分别重算字节 SHA，再独立 Capstone 反汇编，与交付 asm 文本逐字比较。结果详见 `rt-independent-byte-check.json`。Ghidra 记录存在截短 body，实际判断以完整原机器码为准。

## 逐项证据

- **48B/count/40B buffer**：`add_new<RtTrianglesSsbo>@1412c3570` 以 R8=0 调 `TypedStorageBuffer::new`；`3578` 写本地首 qword=0；`3587` 分配 0x30 B；`359f..35b3` 将这个 qword 与相邻 40B 全部复制到 storage。另外两种 add_new 同形。计数与 buffer 是不同位置。
- **实例计数不是 handle**：upload `1408bd46f..472` 将 storage+8 作为 GPU buffer；`4cb` 将 triangles len 写 storage+0。meta `5c7`、BVH `6d4` 同样写首 qword。Compute `1415fc900`/`a2d` 与 Draw `1416293af`/`4aa` 读取 typed storage 首 qword，最后分别 `1415fca11`/`b14`、`141629490`/`58d` 只写 EAX 到 uniform。不可读取 buffer.id 代替 count。
- **元素/容量/NAME**：triangles constructor `141616339` 分配 0x600000 B，`364..368` 乘 24；meta `141617fc9` 0x100000 B，`ff4` 乘 4；BVH `141618aa9` 0x500000 B，`ad4..ad8` 乘 80。三者 usage 0x22、memory1、alignment4。NAME 原字节分别在 `142c7f930`（11B RtTriangles）、`142c7fbc0`（14B RtTriangleMeta）、`142c7fd5b`（5B RtBvh）。三个 add_new 原 key 为 `bea32db3bf3177c9` / `0aa4d4a026938e06` / `282b72fc752587c0`。
- **constructor 容量边界**：tri/meta 比较 `len < 0x40001`，BVH 比较 `len < 0x10001`，仅 Some（ptr 非空）超界时 panic。即允许恰好固定容量。该函数接收 typed slice；bytes 对齐/整元素检查属于 byte adapter/InstanceBuffer 层，不能误写成这三个 TypedStorageBuffer::new 本体中存在额外显式检查。
- **None 与 Some(empty)**：`1408bd2a4..2d4` 取出 pending tuple；`2db..2e5` 用首 Vec cap=i64::MIN 写回 None；`319..320` 比较后 overflow 跳到尾部。这不会将 cap0 的 Some(empty) 视为 None。Some(empty) 仍取出三个 storage，分别跳过 GPU write 并落 `4cb`/`5c7`/`6d4` 的零计数；None 不碰原计数/字节。
- **上传与发布次序**：triangles lookup/check/upload/count，之后才 meta，再 BVH。nonempty 才检查 byte capacity 并调用 update；empty 不写 GPU，但写 count。后续阶段 panic 前，前面阶段计数已经写入；没有回滚。
- **strict len 的 OOB 是日志返回**：`1408bd37a..39c` 是 `(tri_len|meta_len)<0x40000 && bvh_len<0x10000`；失败到 `6dc` 日志，`963..a02` stderr 格式化长度，`a07..a70` 释放三个 Vec，`ac6..ad9` 正常返回。原错误文字为 `Out of pre-allocated buffer space for ray tracing-data`。真正 byte-capacity/backend 失败走 `dd75`/`de33`/`dedf` 或相邻 unwrap_failed 分支；两类错误不能合并为 panic，也不能将后者吞掉。
- **绑定次序与条件**：Compute 先获取 tri/BVH typed count owner，资源读取 tri/meta/BVH/wall；Draw 同序并使用缓存 slot 与 access0x1f（Compute access0x1a）。两个 uniform hash 为 `1c9f2eef0ac739ac` 与 `c2b7f2fdf5e21521`。实际写入受 shader 参数存在与目标 offset 边界分支约束；并非无条件写任意内存。
- **启动闭包边界**：原 populate 记录确实登记 RT 三项，但还包含许多其它真实 SSBO。此上传/绑定恢复不等于 BVH 异步 builder 或整个 startup/scene 编排完成。24B triangle/80B BVH 的具体字段仍应保持其独立数据证据，不能仅凭大小发明字段。

## 当前架构要求

用户已重构 rhapsody；本合同的原布局和调用证据用于恢复 country-core 业务语义、tiny-glade 对当前 API 的适配。不能据此重新复制旧 rhapsody 内部架构。此次没有修改生产文件、没有测试运行，也不将 CPU recorder 当成真实 GPU 执行结论。
