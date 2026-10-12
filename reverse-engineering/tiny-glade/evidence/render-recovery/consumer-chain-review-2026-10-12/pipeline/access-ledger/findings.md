# PassBuilder 访问账本收集闭包

新同 crate 模块为 `reconstruction/rhapsody-rebuild/src/rg/access_ledger.rs`；不新增独立 crate，不修改 root 正在编辑的 pass_builder/graph/mod。原侧证据复用 A2 两轮已保存伪代码与 R2 字段纠正，没有新原机器码 oracle 或库存反射。

## 原合同与跨函数约束

原 `PassBuilder::add_resource_access` 两个单态化 `0x1409dca90` / `0x140533b10` 行为相同。每资源追踪表保存当前 pass id 和最后访问索引；当前 pass 首次引用追加，重复引用读取 **最后条目**。本模块用当前 pass Vec 的最后匹配条目表达同一效果，不跨 pass 去重，也不排序。原直接索引表与候选 Vec 查找的实现复杂度不同；恢复的是声明序列和后续消费者行为。

12 字节原账本是 `{index:u32@0, sync_group:u32@4, access_code:u8@8}`，填充未命名。`AccessLedgerEntry` 显式保留该载体，候选 `AccessDeclaration` 保持既有类型。`ResourceInfo::calculate` (`0x140530430`) 将 group 复制到 20B 记录 +12、code 复制到 +16，再生成 previous/next 链；不能把前轮错误的 +4/+8 含义再交换。

同步组原 `0x140bece50` 从全局 AtomicU32 初值 **2** 获取 fetch_add 旧值；同 crate `ResourceSyncGroup::new` 如实使用该规则，不额外跳过 wrap 后的 0。`0x140bece60` 仅比较 u32，因此组 0 与组 0 也等价。它与 managed store id/counter 是不同字段，不能复用 managed 的 NEXT_GROUP。

## 完整收集分支

1. 当前 pass 尚未引用资源：追加，保留资源首序。
2. code 相同且 group 相同：返回，不追加。
3. group 相同，`0x17→0x1d` 或 `0x1d→0x17`：原条 code 改 `0x1e`，不追加。
4. group 相同，`0x1a→0x02`：原条 code 改 `0x02`，不追加。反向 `0x02→0x1a` 不支持。
5. 已有 code 为 `0x19/0x1a`，新 code 为 `0x37`：即使 group 不同，先把已有条 code 改 **0x37**、group 清 0，然后 **追加新条 `{同资源,0,0x37}`**，最后访问索引转向新条。不能压成一个条目。原 C 字符 `'7'` 是 ASCII 0x37，不是 0x07。
6. 其它冲突原 panic，候选返回 `IncompatibleAccess`。该错误发生在新条/旧条修改之前，已成功的访问仍保持原序；原 panic/unwind 与分配失败未恢复。

0x17/0x19/0x1d/0x1e 保留原码而不猜用途。框架 resource handle 合法性由 root 的 `graph.check_handle` 在调用前验证；本模块不假造 graph/原 builder 的已消费状态 ABI。

## 给 root 的入口 patch

由 root 添加 `pub mod access_ledger;`；可在 BuildError 中增加 `IncompatibleAccess(access_ledger::IncompatibleAccess)`。在 `PassBuilder::add_resource_access` 保留先执行 `self.graph.check_handle(resource)?`，将旧无条件 push 替换为：

```rust
super::access_ledger::collect_resource_access(
    &mut self.declaration.accesses,
    resource.raw(),
    access_code,
    synchronization_group,
).map_err(BuildError::IncompatibleAccess)?;
Ok(self)
```

无需新增 PassBuilder 字段或另造声明类型。现有 Ref::bind 等消费者自动进入同一账本规则，render/render_bound clone 到真实 callback 的 pass 声明也继承合并后的列表。公开图声明直接手填 Vec 仍是重建入口，不据此声称原 compile 闭包 ABI 已恢复。

## 验证边界

仅增加四项模块入口测试：混合资源的首序/去重/对称合并；升级后双记录进入现有 ResourceInfo::calculate 并检查 previous/next/group/code；不兼容错误保留旧条及方向性；原子初值/溢出与12B载体偏移。root 已加入 mod 与 PassBuilder 实际入口，升级测试现在从真实图资源声明出发，调用 PassBuilder::add_resource_access→render→graph.build→ResourceInfo::calculate；先执行 invalid handle 失败，证明未污染账本。四项测试与 strict clippy 通过。原行为来源为保留伪码及R2直接PE复核，**没有新原入口动态比较**。真实 GPU 链仍由 root 验收。受影响验证日志与源码/依赖证据 hashes 由本目录 validation.json 记录。
