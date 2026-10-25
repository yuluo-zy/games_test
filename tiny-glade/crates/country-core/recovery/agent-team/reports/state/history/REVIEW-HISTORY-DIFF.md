# REVIEW — impl-history-difftypes 复验（组 state）

> 审查员：reviewer-state，2026-10-17。复核 impl-history-difftypes（REBUILD-HISTORY-DIFF.md）。
> 编译基线：`cargo check -p country-core --offline --lib` **通过**（13 警告，含他组在途）；
> 测试计数（decorator_edit 5/5 + archivist 2/2 + storage 12/12）为 restorer 记录——全 lib test
> 目标仍被 workspace rhapsody 在途打断，待收口后独立复核。

## 逐项复核

| 项 | verdict | 说明 |
|---|---|---|
| DecoratorHistoryState 重载 ~0x4fB（fresh 0x140b3f930 逐写） | **OK** | 平面 5 字段 → 原生 ~0x4fB（dst_wall/dst_secondary/dst_payload/u18..u4e 逐字段）；from(&DecoratorInfo) 逐分支（preview panic + style 2..8 派生簇：4 Chimney/5 Flag/6 Window/7 Lantern/8+ 默认）——测试断言派生值。 |
| to_info 逆映射 + style 推断 | **OK（U 登记）** | 逆映射实现（restore 回填面）；style 推断/唯一非默认者 U，不编造。 |
| DecoratorStorageAddrHistoryState{states,removed} / DecoratorStorageHistoryDiff{before,after} | **OK** | extract_diff 0x140b2cb30 callee 型名权威；extract_before_after_impl 0x140b2be70。 |
| DecoratorInfo::extra_style（+0x41） | OK | walls 域 decorator.rs。 |
| archivist 链 5（walls 域 decorator_archivist.rs） | **OK** | extract_diff（0x140b2cb30 清桶→逐墙 extract_before_after_pair→HashMap<Addr,Diff>+后置断言）；extract_before_after（0x140b2be70）；archive_storage_decorators（0/1/2 已存在 return+克隆，&str/String 标签参为日志面）；archive_current_decorators_in_storage（0x14123b5c0 per_wall/terrain/stairs）；_in_storages（0x1412b4f10 批量）。 |
| restore_decorators（0x14155ba30）+ apply_storage_diff + restore_decorators_diff | **OK** | 原生闭包形保留 + 真 diff 应用（批量）。 |
| 剩余登记 | **OK（诚实）** | collision 域 collider/mesh 重建（RaycastMesh/OffsetColliderTransform）跨域；extract_before_after_all 配对语义 U（Rust 侧按 before/after 桶逐 id 覆盖当前桶重建，非判等注释）；state serde 分变体选字段；to_info state 字节。 |

## 结论与 DISPATCH 建议
- **history 差分类型收口 → 闭合（实现层面）**。DecoratorHistoryState 原生布局/from 派生簇/逆映射 +
  DecoratorStorageHistoryDiff/AddrHistoryState + archivist 链 5 + restore_decorators 真 diff 应用——
  与 fresh 证据一致；跨域/配对语义 U 诚实登记。
- 残留登记（不伪造）：collision collider/mesh 重建、extract_before_after_all 配对语义、state serde 分变体、
  to_info style 推断。
- history → **review_passed（限定已实现范围）**；可回派 walls 实现 eraser finish/create_rectangle 剩余。
  完整 core 验收仍 blocked（跨域 collision + 全局调度 + rhapsody 收口后重验全 crate 绿窗口）。
