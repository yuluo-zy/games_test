# REVIEW8 — roofs（组 state 卡A3 终验：roofs 主链 + roof_noise 新增）

> 审查员：reviewer-state，2026-10-16。复核 restorer 卡A3 落地（roofs 现状核对 + roof_noise 新增
> + 缺口登记 + recovered_building 边界）。
> 编译基线实测：`cargo check -p country-core --offline` **零错误**（4 警告）；全 crate 测试
> **1634/1**（唯一败 geometry::sphere::tests::uv_formula_matches_the_closure，components-geometry 组，
> 既知）；roof_noise 2/2、roof 模块测试通过。

## 逐项复核

### 1. roofs 现状
| 项 | verdict | 说明 |
|---|---|---|
| 创建/删除/类型切换/回放链（systems/roof editing/lifecycle/registration/replay） | **OK** | add_roof_cpu_systems 16 系统 chain（create/delete/edit/switch_direction/set_flat_* /apply_roof_type/mark/delete…）入 WallCpuStage::RoofDependencies + HistoryCpuStage::Apply；register_roof_events 注册屋顶命令/反馈/历史事件。 |
| register_roof_events 事件数 | PARTIAL(低) | 实测 17 个 `init_resource::<Events<..>>`（CreateRoofCmd/DeleteRoofCmd/SwitchRoofDirectionCmd/…/ReplayRoofEvent）；RESTORE 声称 18——1 项计数差，非业务缺口。 |
| Roof 0x58 投影 + 谓词（is_pitched/flat/gable/…） | **OK** | `Roof([u8;88])` + from_observed/observed；is_pitched/is_flat/is_flat_w_no_rim/is_gable（height_01 + +0x34）/gable_sides；高度/几何谓词委托 system_roof。 |
| roof_shape 几何委托共享 system_roof | **OK** | surface::height/section + parameters::RoofParameterBlock 委托；**不重复实现依赖数学**（遵守约束）。 |
| RoofTriggerVolume 三 Vec + set_* + get_position_from_mesh_uv | **OK** | triangles/normals/uvs 三 Vec；set_*；get_position_from_mesh_uv 委托 utils::geometry（len 断言）。 |

### 2. roof_noise.rs（本批新增）
| 项 | verdict | 说明 |
|---|---|---|
| 常量（尺度 0.5 / 振幅 0.3 / smoothstep (3−2s)s²） | **OK** | apply_roof_noise 0x1412348f0：`y += noise(x·0.5,z·0.5)·0.3·(3−2s)s²`；revert 逆操作。 |
| hash 常量 0x19660d / 0x8001 / 0x401 / 0xae135d3 | **OK** | hash1=(v·0x401>>6 ^ v·0x401)·9；hash_combine2=(h>>0xb ^ h)·0x8001 → (y·0x19660d+h+0x19660d)·0x19660d+0xae135d3；0x401 在 roof-tiles module498 反汇编互证。 |
| 角种子表（@0x142ac7e10：e10/e14=0x0ae135d3、e18/e1c=0） | **OK** | RNE 实测 `d3 35 e1 0a d3 35 e1 0a 00 00 00 00 00 00 00 00`，与 CORNER_SEED_0..3 一致。 |
| uint_to_u01_float mantissa 掩码修正（&0x7fffff 防 NaN） | **OK** | `(bits & 0x7fffff) | 0x3f800000) - 1.0`；注释说明"先掩掉指数位否则 OR 后成 NaN"。 |
| 测试 | **OK** | noise [0,1] 确定性 + apply/revert 强度尺度（2/2 实测通过）。 |

### 3. 缺口登记（诚实）
| 项 | verdict | 说明 |
|---|---|---|
| roof_ridge(7) / roof_visual_shape(6) / roof_tiles / dormer / buffers | GAP（登记） | RESTORE 逐文件列出缺项；roof_visual_shape 消费 roof_noise（get_height_01_unclamped_from_ws）待建；buffers 标"胶水不计业务"（HIGHLIGHTS #9）。 |
| roof_shape 剩余几何谓词（is_stepped/is_overhang/ridge_length_01/…） | PARTIAL（待续） | 多数委托 system_roof，逐函数核对待续，诚实登记。 |
| startup_roof.rs（0 函数） | GAP（登记） | 注册胶水随卡G 域注册并入。 |

### 4. recovered_building 复核边界
| 项 | verdict | 说明 |
|---|---|---|
| roofs-rt HIGHLIGHTS #1/#2/#3 重锚 | **OK** | #1 Roof 几何族经 surface.rs（3,004 案例 0 差异）验证性恢复；#2 Roof 0x58 布局偏移（+8 tag/+0x24 tip/+0x2c profile/+0x30 height/+0x34 ridge/+0x38 eave/+0x3c dir/+0x3d rim/+0x4c floor/+0x54）与 src new_observed 逐偏移谓词一致；#3 RoofTriggerVolume 三 Vec stride（0x24/0x24/0x18）与 src 一致。 |

## 结论与 DISPATCH 建议
- **卡A3 → 闭合（roofs 主链 + roof_noise 新增）**。创建/删除/类型切换/回放链、Roof 谓词、
  RoofTriggerVolume、roof_noise 哈希/噪声/常量全部核对；几何委托 system_roof 不重复实现。
- 微差/登记：register_roof_events 计数 17 vs 声称 18（非业务）；缺口（roof_ridge/roof_visual_shape/
  roof_tiles/dormer/buffers/startup_roof）与 roof_shape 剩余几何谓词逐函数核对待续。
- 模块状态建议：roofs → **review_passed（限定已实现范围）**；完整 core 验收仍 blocked（全局注册序/
  视觉 bundle/碰撞构造 + 各域系统批量并入）。
