# M01 RoofUV 上游：真实屋顶触发网格的世界floor映射，2026-10-27

先实读B `crates/systems/roof/src/{create_roof.rs,roof_trigger_volume.rs,visual/generate_roof_stone_floor_and_roof_bottom.rs,visual/switch_roof_type.rs}` 与当前对应实现，检查实际迁移路径与原typed参数。最初“缺RaycastMesh ECS组件producer”的假设已被正面原证否定：原update_rapier_colliders1408d0490的12参直接读Roof/真实RoofTriggerVolume Query，不查询RaycastMesh组件。root已撤回该假设；本轮没有给RaycastMesh添加Component或给Roof造近似盒体。

## 已有真实生产端及本轮分界

`system-roof/src/roof_trigger_volume.rs::calculate_roof_trigger_volumes1421a3220/12048B` 已迁移；原2参为EventReader<RecalculateRoofCmd>与Query<(&mut RoofTriggerVolume,&Roof)>。按实际事件entity取屋顶，完整circle/rectangle、flat/gable、原strip/cap重复顺序、UV/预noise normals与roof noise都在既有generate_roof_trigger_volume中，不是空壳或默认mesh。已有原16case `round12/roof-visual/native-trigger-fixtures` 只作迁移证据；其旧验收保留2e-5绝对差值范围，**不能挪作本轮新增完整原指令逐位证明**。

本轮只修改B对应既有 `country-core/src/resources/roofs/roof_trigger_volume.rs`，恢复原triangles映射；不改complex generator、Roof截面、World、Rectangle RoofUV两个函数或全局注册。GPU owner负责唯一World/map/12参主函数，root负责宿主。

## 原调用证据与修复

原update Roof支路 `1408d5212..1408d5262`：先assert triangles非空，取Roof+0x4c为floor，构建Map{begin,end,floor}并调用collector140dce1d0。独立原getter `RoofTriggerVolume::triangles1408e4560` 在XMM2接收floor，非空后返回同Map；PDB原文件line32与closure line34归属明确。

完整421B collector对每个三角形的每个点计算 `point + Vec3(floor*0,floor,floor*0)`。不能只沿Y平移同时删XZ零乘；非有限floor时原XZ也传播NaN。getter非空守卫在映射之前，保持该失败顺序。

原随后 `1408d5267..1408d54ee` 直接clone真实逐三角normals/uvs，调用RaycastMeshData::new_unindexed1408cf7d0；不重新计算normal、不对UV/floor做猜测。`1408d5780..1408d57cc` 真upsert传零translation/identity quat，因为floor已经进入点位，不能在World Transform再加一次。source tag2及flat/gable两byte由GPU owner独立核原write面。

新API `triangles_at_floor_y(floor_y)->impl ExactSizeIterator<Item=Triangle>` 借用唯一原三角数组，不做中间Vec或改源缓存。旧raw `triangles()`保留，避免破坏现手工重构消费者。调用侧只在实际mesh构造时collect一次，然后进入现canonical new_unindexed/World。此API已交GPU owner，不另写注册管线。

## 原指令与测试

身份：RNE SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03，PDB SHA256 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911。

`m01-roof-trigger-floor-native.py` 完整执行原getter1408e4560与collector140dce1d0，仅桥分配器，没有CRT、资源查询或业务结果替身。54组覆盖空/奇偶/1..5三角形、±0、subnormal/max finite、正负有限floor以及Inf/NaN；9个原空volume panic保持，30个有限floor场景810个输出coordinate words全部逐位一致。15个非有限floor场景保留同NaN分类，非NaN words逐位；跨x86/ARM不虚称NaN payload/sign位型等价。

最终命令 `CARGO_INCREMENTAL=0 cargo test --offline -p country-core --lib original_roof_floor_projection`：1 passed / 0 failed，见final-tests日志与validation源指纹。此测试确认真实core helper与required失败边界，没有OS输入或GPU完成声明。

## 联合验收边界与经验

接下来由GPU owner使用此helper完成真实 RoofTriggerVolume→new_unindexed→World entity collider表→单次shapecast→RoofUV，并覆盖实际编辑/删除/Reload。此处不把helper局部绿冒称该联合链已完成。

经验：先查真实SystemParam/原writer而非仅rg名字；RaycastMesh可为World内部receipt，并不必为ECS组件。原几何生产器可能早已迁移，真正断点是Map getter丢失的一次世界空间投影。必须跟踪偏移到底在点、mesh-local pose还是World Transform，避免漏加或重复floor。NaN传播与负零也要保留，报告明确位比较与分类比较各自范围。

当前复杂producer复用基线也已实际复跑：system-roof同名original test 1/1通过，BIN夹具实际28 shapes（JSON旧快照16组），最大绝对差5.9604645e-8、354个float bit不等；明确保留旧2e-5数值合同，不称全网格逐位一致。日志m01-roof-trigger-complex-producer-reuse-tests.log。
