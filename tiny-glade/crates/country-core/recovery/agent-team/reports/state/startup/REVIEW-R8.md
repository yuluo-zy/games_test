# REVIEW — R8（组 state：startup 批量分类登记）终验

> 审查员：reviewer-state，2026-10-16。复核 restorer R8（startup 缺文件批量 27 分类登记）。
> 本批为**纯核对登记**（无新增大件）。
> 编译基线实测：`cargo check -p country-core --offline` **零错误**（6 警告，含他组在途）；全 crate 测试
> **1709/1**（唯一败 geometry::sphere::tests::uv_formula_matches_the_closure，components-geometry 组，既知）。

## 逐项复核

### 1. 已实现核对
| 项 | verdict | 说明 |
|---|---|---|
| init_photomode_state / load_glade_audio / load_textures / startup_{garden_brush,ivy,session,terrain} + mod.rs | **OK** | 9 文件实存（ls 确认）。 |
| 卡G register_core_startup / add_game_startup_systems / load_shader_into_library | **OK** | 卡G 已核对（REVIEW7）；入口合同保留。 |

### 2. 缺 27 文件分类（诚实）
| 类别 | verdict | 说明 |
|---|---|---|
| 域注册胶水（startup_roof/sheep/time_of_day/walls/water/glade_themes 7 项，0 函数/组件注册） | OK（登记） | 依赖域类型在域侧，随墙/屋顶/羊/时间/水/主题恢复后并入。 |
| 资产库绑定（load_audio/glade_grading/…/sky_and_sun 17 项，mesh/shader/prefab 载入） | OK（登记） | 依赖 AssetMesh/Shader/Prefab 库 + render 管线（在途）；load_shader_into_library 入口已备。 |
| panic_reporter_startup（行 22，I 无机器码证据） | OK（登记） | 明确标注 I/无机器码证据，不冒充。 |
| init_color_library（assets/colors.json → color library 资源，0x14082c000 O） | OK（登记） | 资源在 render 侧，待接。 |

### 3. 诚实预算判定
| 项 | verdict | 说明 |
|---|---|---|
| 未伪造实现 | **OK** | 27 文件仅分类登记依赖状态，无假代码；register_core_startup 预留 `add_game_startup_systems` 域系统并入缝。 |

## 结论与 DISPATCH 建议
- **R8 → 闭合（startup 批量分类登记）**。已实现文件确认、27 缺文件四类依赖状态诚实分类、
  未伪造实现。
- 残留登记（不阻塞）：域系统并入随各域在途类型闭合；资产库绑定随 render/asset 组；panic_reporter/
  init_color_library 待对应域。
- startup → **review_passed（限定已实现范围）**；完整 core 验收仍 blocked（域系统批量并入 + 资产库/
  render 管线接线 + 全局调度）。
