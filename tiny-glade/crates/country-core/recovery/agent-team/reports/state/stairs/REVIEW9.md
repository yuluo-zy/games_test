# REVIEW9 — stairs（组 state 卡A4 终验：stairs 权威图 + 差分链）

> 审查员：reviewer-state，2026-10-16。复核 restorer 卡A4（stairs_state 权威图全核 + 缺口登记
> + recovered_building 边界）。本批为纯核对登记（无新增大件）。
> 编译基线实测：`cargo check -p country-core --offline` **零错误**（3 警告）；全 crate 测试
> **1665/1**（唯一败 geometry::sphere::tests::uv_formula_matches_the_closure，components-geometry 组，
> 既知）；stairs 相关测试 12 通过 0 败。

## 逐项复核

### 1. stairs_state 权威图全核（src vs 章节/README 声明）
| 函数/类型 | verdict | 说明 |
|---|---|---|
| `StairGraphEdge`（min/max 归一、is_stub） | OK | 0x140901260；自对=孤立节点语义（README 注释：非额外持久边表）。 |
| `add_node`/`add_edge`/`disconnect_edge`/`remove_node` | **OK** | add_edge 自环断言先于去重；disconnect_edge a==b 走 remove_node；dirty_edges 在变更时扩展。 |
| `get_all_connected_nodes`（BFS + sort + dedup）/ `contains_edge_or_stub` / `collect_edges` / `drain_dirty_edges` | **OK** | petgraph UnGraphMap + Bfs；stub = 孤立节点。 |
| endpoint 设置（`try_set_endpoint` 度数守卫 >=2 / `unset_endpoint` / `is_unset` tag==3） | **OK** | 0x1408fe7b0：成功时拷贝整个 24B endpoint；度数守卫为 >=2（7 < count*8-8），修正旧 reconstruction 的 >=8。 |
| 备份/恢复（`make_backup`/`restore_backup`/`clear_backup`） | OK | make_backup 先 mark_missing 旧备份废件再替换；restore 同。 |
| 快照（`take_snapshot`/`StairsSnapshot::new` sort 不 dedup/`compute_checksum` hash_edges） | OK | 0x140901120 / 0x14145b330；checksum 惰性缓存。 |
| 差分（`extract_diff_and_make_backup`/`reduce_diff`/`apply_diff` undo 交换/`restore_snapshot`） | **OK** | 0x1409007b0 / 0x140900010（先 diff 后克隆即使相等）；apply_diff = 移除→剪孤立端→插入；undo 交换 removed/added。 |
| `TransientStairEdgeId`/`Allocator`（id=旧 len+1，无独立计数） | OK | 0x1408fe5e0；重复边保留首 ID。 |
| `StairsAssemblyParams`（stairs_assembly_params.rs 112L） | OK | 组装参数存在。 |

### 2. 缺口登记（诚实）
| 项 | verdict | 说明 |
|---|---|---|
| invalidate / stairs_collider（胶水）/ stairs_removed_supports（replay_stairs undo 消费）/ structural_graph / visual / stairs_walls / replay_stairs | GAP（登记） | RESTORE 逐文件列出缺项并标注依赖（replay_stairs undo 插边集 StairsRemovedSupports/!=0 remove_entry + PlayStairsAudioCmd；卡C 已登记阻塞）；**不冒充实现**。 |
| audio / render_loop stairs | GAP（登记） | 其他组在途。 |

### 3. recovered_building 边界
| 项 | verdict | 说明 |
|---|---|---|
| wall_platform_stairs.rs（平台楼梯生成/物化） | **OK（不重挖）** | 生成/物化算法在共享 recovered_building（StepRange/blockers），不重复实现；stairs_state.rs = country-core 权威运行时图（petgraph UnGraphMap<u32,()> + endpoint + 快照/差分/备份链），与章节一致。 |

## 结论与 DISPATCH 建议
- **卡A4 → 闭合（stairs 权威图 + 差分链）**。stairs_state 全函数核对通过（图操作/endpoint/备份/快照/差分/
  TransientStairEdgeId 均与地址注释/章节一致）；StairsAssemblyParams 存在。本批纯核对登记（无新增大件）。
- 缺口登记（不阻塞）：invalidate/stairs_collider/stairs_removed_supports/structural_graph/visual/
  stairs_walls/replay_stairs + audio/render 衔接。
- 模块状态建议：stairs → **review_passed（限定已实现范围）**；完整 core 验收仍 blocked（视觉/碰撞/音频/
  渲染衔接 + 各域系统批量并入 + 全局调度）。
