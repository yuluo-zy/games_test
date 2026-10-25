# Cull-allow producer：原证及 entry-owned 修复

原入口 `replay_edit140af6170`（保留 `evidence/building-holes/B-storage-frame/140af6170.c`）从**所选 MultiEdit** 的 `+18` 克隆 DecoratorHistoryDiff Arc，再经 `to_arc_slice<cull_allow_list>`140a96d20收集。它不从当前 LiveDecoratorArchivist 快照取allowed_ids。

原RNE定点反汇编已落 `cull-collect-original.asm`（1412feb50）与 `cull-filter-original.asm`（1415635a0）。collect中1412fefff以storage bucket stride70定位；`ptr-60` 是before map，`ptr-30`是after map；1412ff012..01b通过undo低位交换两者。随后迭代选定map.keys，filter对另一map做contains，命中继续扫描、不命中yield。因此：

- redo（undo=false）：每storage的 `before.keys().filter(|id| !after.contains_key(id))`。
- undo（undo=true）：每storage的 `after.keys().filter(|id| !before.contains_key(id))`。

跨storage按Values→FlatMap输出，未见全局去重；移到另一storage的同ID可能仍在表中。这是允许原storage进行cull的ID集合，不应按全局存在性合并。

修复前的真实owner错误：`HistoryEntry<M>`只有edits、auxiliary_counts、stairs_checksum，**没有保存每entry的Decorator/Stairs diff Arc**。`add_decorator_diff`只把最近diff写到HistoryTransactions全局字段；`advance_history_cursor`随后把全局最近diff复制进任意选中的entry回放。两个历史entry有不同diff时会选错payload，既影响Pop，也影响cull表。

现已按原证修正：country-core原有`HistoryEntry<M>`直接持`HistoryEntryAux`（Decorator/Stairs两条真实Arc），`add_decorator_diff`绑定pendingentry，正常commit转移entry；头插入/branch/truncate/clone都由原Vec/Arc移动语义保持owner，不添加易失效的索引map。游标选中entry后，`HistoryEntryAux::for_replay(undo)`产生上述定向allow表和同entry的diff。

新增生产App测试`two_committed_entries_restore_their_own_aux_and_directional_cull_ids`实际通过`history_finish_recording`提交两笔不同diff，再执行Undo两次/Redo两次，逐帧核decorator存储ID、楼梯边、allow单ID、cursor和Pop。全局latestdiff仍停留最后一笔，旧实现会在第二次Undo失败。最终执行结果由root统一验证记录。

已保存但原格式未恢复的MultiEdit aux decoder继续显式拒绝；本轮不把缺失wire内容替换成最近diff或猜测值。
