# W17 foreign owner最小接线请求

任务TG-74d7679193a92a0e8c属于startup-producer-20261018，本任务未取得该family修改授权，未生成完整source覆盖候选，未改production。

`path_pebbles_compute@140958f70/1496B`的原调用仍保持access0x37/group1；当前canonical `path_pebbles.rs` 真实 `write_storage_buffer_concurrent::<DrawIndexedIndirectBuffer>(binding::DRAW_COMMANDS,1)` 后需增加：

```rust
.preserve_storage_buffer_contents(binding::DRAW_COMMANDS)
```

这是当前图内容合同，不改原access码/顺序/同步group。原发行SPV `path_pebbles.spvasm:5155..5156` AccessChain指定DrawIndexedIndirect.instanceCount后OpAtomicIAdd；uniform槽3。原子操作只更新1个word，其余4个命令word及其他slot来自真实main prelude/reset，不能被discard。PebblePlacementSsbo是仅写完整记录的首次producer，不应按group blanket增加preserve；它的None构造保持无初值。

待root/原owner合并此1行后，新的typed host只调用canonical path_pebbles_compute，不旁建业务算法、不事后篡改RecordedComputePass。此请求没有运行Cargo/GPU。
