# 羊群快照字段纠错合同

当前正式工程目标 B；A 未验。C 具有同源 control.rs checksum，另已提取 C apply/serializer 原字节，只支持此字段绑定，不支持完整 C 生态。

观察事实：B capture 0x1412f03f0 将 Transform(+0x10,12B) 写 snapshot+0x1c；将 CritterGoal(16B) 写 snapshot+0；将 Sheep(+0x30,12B) 写 snapshot+0x10。B serializer 0x1416f9eb0 的 r9 实参依次为 snapshot+0x1c(position)、snapshot+0(goal)、snapshot+0x10(move_state)。旧 LOGIC.md 错把 Rust 的声明顺序当作原内存顺序，2026-10-17 的绑定纠错实际上引入 bug。

因此 position 来自 Transform.translation；goal 是 CritterGoal 的完整 Option<Vec3>；move_state 是 Sheep+0x30 的状态。SheepMoveState serializer 0x1413ff520 明确 timeout_s@+4、goal_change_timeout@+8、force_reach_goal@+1。禁止再把 enum 编码进坐标。being_petted@+0x58 的比较规则继续复用。

原 capture 按 Query 顺序且不保存实体ID；apply 0x1416f24f0 zip较短一侧，仅覆盖上述字段并写三列change tick。当前纯接口与 ECS debug owner必须共用绑定。serde wire 保持 position/goal/move_state，不声称 Rust/native ABI一致。None的无效payload和Idle padding不进入语义合同。

原指令验证：native_snapshot_probe.py 对身份绑定发行B执行 capture热dense分支和完整apply的已解析合成dense表；80行capture、128组apply检查及三列tick通过。未验证真实Bevy表发现、调度、完整游戏、save文件往返或所有羊移动算法。

证据：evidence/pack-B/{capture,apply,serialize-snapshot,serialize-move}.asm.txt；evidence/pack-C/{apply,serialize-snapshot}.asm.txt；evidence/native-projection.json。原二进制sha由pack及probe独立检查。工具沙箱内Unicorn JIT mem_map SIGILL，已获自动审核允许在沙箱外运行同一只读probe并通过；不把失败当通过。

剩余：system-critters拥有另一套更完整Sheep/SheepGoal；其与core类型统一、完整生态调度/存档装载由后续链处理，本补丁不宣称整域完成。
