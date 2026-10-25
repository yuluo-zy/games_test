# REVIEW11 — E1 吸收（组 state：EditType 15 变体权威表 + Terrain 保留号 + 操作 tag 登记）

> 审查员：reviewer-state，2026-10-16。复核 restorer E1（RE-EDITTYPE.md）吸收。
> 编译基线实测：`cargo check -p country-core --offline` **零错误**（6 警告，含他组在途）；全 crate 测试
> **1671/1**（唯一败 geometry::sphere::tests::uv_formula_matches_the_closure，components-geometry 组，
> 既知）；history 测试 10 通过 0 败。

## 逐项复核

### 1. EditType 扩 15 变体（E1 权威表，visit_str 0x140b3b630）
| tag | 变体 | verdict |
|---|---|---|
| 0 | Tree | OK |
| 1 | Path | OK |
| 2-4 | Cobblestone1-3 | OK |
| 5-8 | Water/Garden/Wall/Roof | OK |
| 9 | Color | OK |
| 10 | DecoratorV5 | OK |
| 11 | Decorator | OK |
| 13 | AutoClutter | OK |
| 14 | Stairs | OK |
| 12 | （4 字符名未解） | 登记未赋变体（枚举跳过该 tag，注释注明）——诚实 |
| — | src 此前错位全修 | OK：Tree 18→0、Color 15→9、Decorator 16→11、Stairs 17→14、DecoratorV5=10 新增。 |

### 2. Terrain 协调
| 项 | verdict | 说明 |
|---|---|---|
| EditType::Terrain 原版无此名 | **OK（不伪造）** | "TerrainPath" 串非 visit_str 命中名（属 RasterEdit 族或他枚举）；Terrain 保留变体 tag=15，明确标注"待定位，不伪造"，避免破坏 interaction terrain 在途。地形历史记录机制待 E 补挖。 |

### 3. WallHistoryEdit 17 操作 tag
| 项 | verdict | 说明 |
|---|---|---|
| WallHistoryOperationTag 枚举（WallChanged0…ChangeSkirt16，tag15 名未解 anon_95 13 字符） | **OK（登记）** | 判别=首字段 niche 与顶层不共用；**待并入 WallHistoryEdit 结构**（并入会破坏 wall replay 非穷尽匹配，E 补挖载荷后合并）——登记不冒充。 |
| serde/MultiEdit deserialize 跳表 0x142aee31c 对齐 | GAP（登记） | 待 E1 载荷补挖后合并。 |

## 结论与 DISPATCH 建议
- **E1 → 闭合（EditType 15 变体权威表 + Terrain 保留号 + 操作 tag 登记）**。native_tag 全表按
  visit_str 0x140b3b630 修正；Terrain 保留 15 不伪造；WallHistoryOperationTag 17 操作名表登记。
- 残留登记（不阻塞）：tag 12（4 字符名）、Terrain tag 定位/地形历史机制、WallHistoryOperationTag
  并入 WallHistoryEdit 结构 + serde/tag 对齐（E 补挖载荷后）。
- 模块状态建议：history → **review_passed（限定已实现范围）**；完整 core 验收仍 blocked
  （E 补挖载荷/并入 + 全局调度 + 各域系统批量并入）。
