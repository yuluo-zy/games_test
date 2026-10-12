# 独立 C sheep pet harness 执行记录

2026-10-11。没有构建或验证building/root workspace。

实际命令：

```sh
cargo build --manifest-path reconstruction/critters/c-harness/Cargo.toml --release
cargo test --manifest-path reconstruction/critters/c-harness/Cargo.toml --release
/Users/yuluo/Desktop/game/tools/venv/bin/python evidence/toolchain-pet-pilot-2026-10-11/critters-verify-pet.py
```

release构建成功（初次下载/编译29.36s）；测试构建1.65s。初次三个tests成功；review增强后，在原C锁seed/prune生成的最终锁上四项检查全部成功（3World+1pure；31.03s含重新构建）：

- message_to_native_state_to_locomotion_and_finish：duplicate event不推进两次、其它sheep不变、exact1不提前transition、amplitude保原overrun、原fivephases与flag清理、finish后idle不重写。
- source_removed_with_live_entity_withdraws_derived_gate：删除Source而Entity仍在撤派生gate；重新插入恢复；despawn后无派生残留；不存在target不产生伪source。
- event_during_fade_out_does_not_restart_and_unordered_hold_is_retained：fadeout不会因当前event重启；unordered hold按原SETBE保持。

原C选择行FSM与being_petted120例全部通过，详见critters-C-pet-verification.json。该报告比较所有104B（包括未写opaque字节）、原tick写入决定及完整callee RET结果；selected event membership由输入提供，未执行整个原ECS系统/原UI。

- unchanged_nan_payload_does_not_republish_adapter_gate：相同NaN位模式不重复insert；这是adapter publication策略，不是原consumer tick规则。
- 实际source change ticks：inactive/finish-idle保持tick；hold2的dt0仍按原显式写入推进tick。

C runtime使用实际公开Bevy0.17.3 App/World/Message/Query/Commands；chain是adapter顺序，Sheep104源为observed投影，无startup/default；源+60布尔保原存储及展示payload，未知renderer用途未赋名。locomotion gate来自原C Sheep::being_petted140935910（amplitude>0.01；NaN false），原C walk caller142934fb9有直接CALL。这里只关闭该抚摸切片，不接受完整Sheep/UI/renderer、C原fork及其它critters。

manifest固定Bevy app/ecs=0.17.3；初次harness解析的74项transitive漂移经父review发现，旧lock/旧opcode报告已归档。最终以原C lock为seed，prune到113registry包，version/source/checksum全部匹配；features/fork仍不宣称与原完整C完全相同。B firefly仍用独立原0.29.3/Perchance依赖，不将本批C检查冒称修复其旧NaN差分。
