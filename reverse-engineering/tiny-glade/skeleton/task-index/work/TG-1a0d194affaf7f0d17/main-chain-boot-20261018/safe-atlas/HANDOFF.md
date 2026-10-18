# B atlas 窄候选交接

本包只写独立 work 候选，未写生产、未运行 Cargo/GPU。旧cos候选已由完整PE import反证并改为sin，见ROOF-ORDER-CORRECTION.md；manifest标出旧SHA失效。当前用户重构后的 Mesh / MeshAtlasLibrary / AssetMeshLibrary 是基线；不复制原 Rhapsody 内部 ABI。

## 可以先集成的文件

`load_mesh_atlases.rs`、`add-module.patch` 新增 `startup/load_mesh_atlases.rs`，`export-module.patch` 仅导出模块。候选 SHA 与逐个当前依赖 SHA 见 `manifest.json`。该窄版本不读取 PrefabPart 的任何字段，也不改 systems/resources，当前普通依赖已有 ron/serde/glam。

实际函数：

- `load_nani_meshes`：加载 `assets/nani_meshes.ron` typed schema；RoofFloor 原 assert；demo 跳过；实际 JSON mesh；同 watcher；先删属性再补缺属性；调用当前 atlas.add。
- `load_nani_meshes_for_subsets`：同唯一业务体，先按 changed subsets 筛选配置，再 assert/I/O。没有另写简化重载器。
- `generate_procedural_meshes`：原 roof6、curved6、plaster1、stairs5、smoke1 共19个网格。后3族按 encoded vertex ID 只产生索引，不填假位置。复用现 grid_gen_cw/ccw。
- typed schema / `apply_mesh_attributes`：保留已有属性；补属性元素个数取删除前任一属性的长度；沿原 Vec setter 非空断言，不从索引猜顶点数。

这只是完整 producer 的可安全拆出业务体。导出模块本身不接 PreStartup、不注册假句柄、不声明 gameplay ready。

## 完整 producer 与依赖边界

`../candidate/load_mesh_atlases.rs` 另外包含完整 initial/reload 调用边界。它依赖尚未进入当前生产的 canonical PrefabPart `render_mesh: Arc<Mesh>` 和 `render_subset: GpuWorldSubsetKind`，**现在不能直接复制编译**。原来把 Part.mesh 当 RT mesh 的模型已被实际 producer 反证，不能借该假字段拼通图集。

`resources/prefab.rs` 当前 SHA 相较旧候选已漂移，`systems/prefab.rs` 仍有外部 owner `core-prefab-adapter-20261018` / TG-d44984ec9eec9a0bde；本包不覆盖。旧 resource/consumer 整文件候选仅作证据交接，必须由 owner 基于最新体增量合并。`startup/load_prefabs.rs` 当前 SHA 未漂移，但它依赖该字段变更，因此同样暂缓生产。

## 原证据与顺序

发行 B RNE SHA：`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。完整 PDB 边界 RNE 包在本工作目录上一层，函数体字节 SHA 在 `../native-manifest.json`；保留 decomp 只作辅助。

- initial `14129f400` / 5138B：`@14129f56d..59c` 取 Part+88 Arc、Part+90 subset，Arc+20 非零才 add；Arc header16 + Mesh.indices Vec0/len10，故是 **indices 非空**，不是属性表。`@14129f5d2` 调 RON，之后五个生成族；`@1412a00ae` 最后 register_meshes。
- RON initial `1415f7190` / 5679B；reload `1415f5b60` / 5677B：二者共享 schema 与真实 JSON/属性转换，后者在消费每个 config 前按 subset 集合筛选。enum visitor `1415f8c40` / 204B 五个精确名字 I32/F32/F32x2/F32x3/F32x4。
- reload `1412a0820` / 4667B：独立 drain atlas_changes；从 modified prefab 所有 part 收集 subset；先所有 subset clear (`@0cff`)，再所有 modified name clear (`@0d95`)；重加谓词是 subset 命中 **或** prefab name 命中，再要求 indices 非空 (`@1153..123e`)；changed subset 非空才调用 reload RON (`@14e0`)；非空输入批最后 register_meshes (`@1512`) 并发真实 MeshAtlasesModifiedCmd。**不重做程序网格**。
- `/root/render_present` 对完整 reload RNE 已独立复核 OR 谓词、clear 顺序、末尾 register 和无程序网格重做。其初次把 Arc+20 称 attribute vector 的术语已撤回，最终与当前 Mesh 原布局交叉确认是 indices.len。
- PreStartup helper `140a97110` 原 outer4 有链：preset textures → 内部10项普通 tuple → load_mesh_atlases → startup_nani。10项内部未证链，不能称10chain。原注册入口 `140941110`。

五族常量独立复核来源：`TG-a31ead12d06d2bb8a2/render-nani-main-20261018/procedural-mesh-constants.json`。roof level0..5 的 grid/sin/sqrt 与 curved level0..5 的7列原位点、plaster 192编码索引、stairs n4..64、smoke 12×50 都保留现容器。

## prefab reload 反证交接

原 `load_prefabs_impl@1412a1df0` CL=reload，`140a04cf0` 完整326B筛选：`!reload || !library.has(name) || changed.contains(name)`。只 drain prefab_changes，扫描/转换 materialize 完后 `@1412a4e80..4ebf` 逐条 add，不删除库中扫描不到的旧名字。新名字即使不在通知中也载入，未通知已存在名字保持原 Arc。

`reload_prefabs@1412a1a60` 只在 prefab queue 非空时调用，成功发 HashSet<PrefabName>（原 payload48B+EventId8B），当前 unit PrefabsModifiedCmd 身份错误，应由 canonical owner 修。旧测试“删除文件即清空库”“同时drain两队列”“失败回填两通知”都不符合此原分支；相应最小测试迁移在 load_prefabs claim 的 candidate/startup-tests.rs，尚未合入。

## 尚未闭合

完整 PrefabPart/音频消费者 owner 整合；完整 outer4/inner10 PreStartup 注册（含72 SSBO和全部真实纹理/音频 producer）；加载失败的原 panic 与 host Result 报告边界；mesh processor 全生产回归与实际资产装入；主世界图与正常 Steam backend。窄候选不掩盖这些缺口。
