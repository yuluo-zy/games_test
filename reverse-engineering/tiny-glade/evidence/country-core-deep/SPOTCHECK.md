# 第二轮 O 结论地址抽查（2026-10-14，spot-check）

方法：`tools/country-core-audit/audit-claims.py`（只读 PDB 过程表 + S_PUB32），
核验 REVIEW/DIGEST 里带地址的断言是否落在真实过程上。机器可读：
`evidence/country-core-deep/spotcheck-{claims,result}.json`。

## 结果

| 地址 | 断言 | 裁定 |
|---|---|---|
| 0x140a8a5d0 | AppMode::is_tool_selected（ui-input: 伪声明） | **MISMATCH**，实为 `timelapse_replay::cleanup_timelapse`（PDB+S_PUB32 双确认）→ ui-input 纠错成立 |
| 0x140af9000 | update_mod_key_state 实为 Toolbar::new（ui-input） | 非过程起点；owner=`Toolbar::new +0x3b0` → 地址在 Toolbar::new 体内，纠错方向成立 |
| 0x1408ccc20 / 0x1408cd5c0 / 0x1408d98a0 / 0x1408d9590 / 0x1408ce460 | collision-eco 五个地址重标 | **全部 MATCH**（raycast_simple / flush_colliders / xz_flatten_aabb / dispose_collider_mesh_instance / on_collider_upserted）→ v2 纠正可信 |
| 0x140b01050 | GradingLut::load_cube | **MATCH** |
| 0x140a86970 / 0x140a86980 | history is_recording / finish_potential_mutation | **MATCH**（与 PLAYBOOK 一致） |
| 0x1409a9140 | WelcomeScreenState::is_closed（ui-input） | **MISMATCH**，实为 `widget_params::UiFocus::is_mei` → 该地址疑被 ICF 折叠或断言地址有误，**需二查** |

## 结论

- collision-eco 的地址级纠错抽查全部通过（5/5），ui-input 的 `0x140a8a5d0` 伪声明成立。
- `0x1409a9140` 与 ui-input REVIEW 不符：UiFocus::is_mei 与 WelcomeScreenState::is_closed
  可能是同一折叠地址（`is_closed = 判别==2` 曾称 0x1409a9140 折叠处证实）；按机器码为准，
  该处暂标 U 待复核。
- 方法可复用：后续对 collision-eco 80%、ui-input 79% 的 WRONG/PARTIAL 批量抽地址，
  无需开 Ghidra。
