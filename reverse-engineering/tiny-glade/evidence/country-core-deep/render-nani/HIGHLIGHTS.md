# render-nani 域 HIGHLIGHTS（tg_rn1，2026-10-14）

1. **17 章全填充、占位清零**：pass_builder 10 章 + nani 7 章（547 业务 / 126 内联 / 71 胶水函数），全部复用 B 阶段 + wp 合同 + reconstruction 源码既有证据，**新反编译 0 个**；DIGEST 见 `render-nani/DIGEST.md`。
2. **GpuWorld 权威布局**（O，0x1409917d0）：27×0xc0 `RwLock<Option<GpuWorldSubset>>` + `total_instance_count@+0x1440` + `Box<NaniDrawListParams>@+0x1450` + `TypedStorageBuffer<NaniMeshAtlasPartInfo>@+0x1458` + `TypedStorageBuffer<NaniDrawListInfo>@+0x1480`；wp3 合同"7 锁数组/+0x14a0"作废。InstanceKey = 12B `{id,index,kind}`（16B 是 InstanceCullData）。
3. **27 个 GpuWorldSubsetKind 真名**（O，0x142aba868/0x142aba940）：Canopy…StairsFloor；旧 Main/Bush/Unknown9..26 全是编造；kind 数字与 nani_update_* 一一对应（SolidVertexColor=3 … StairsFloor=0x1a）。
4. **compute_pass_builder 400B 组装**（O）：bindings/access_ranges/external_bindings/PassBuilder/UniformBinder(+0x136==8 无块)/注册表(+0x138)/设备(+0x140)/PassResources(+0x148)；访问码 0x1a 读/0x37 写/0x32 cpu 写/0x02 间接读（B 阶段纠错）；SSBO 访问**不经注册表**。
5. **DrawBuilder 0xf8B**（O）：+0xe0 是 `&mut RasterPass`（不是 GameContext）；draw 种类 0/1/2/3/4/5/7/8，终结操作无返回值；`bind_common` bool uniform = `flag==0`（B_i_api §1.1 纠错）。
6. **shader_uri parse 语义纠错**：path/stem/entry 三段（entry 可选、无 '.' → unwrap），旧 wp1 合同 whole/name/entry 全错；18 个游戏实现 = 9 nani cull + DoF×5 + BinLights + GaussBlur + TgsrMain + GenerateStrokes。
7. **buffers 两函数独立**：`nani_calculate_draw_list_params` 与 `default_value` 不是同一代码体（Ghidra 跳表误读）；LISTS_PER_KIND=4、108 项、汇总 (3788800,71936,2048,8) 互证；掩码 0x3fe 非 0x3ff。
8. **nani_draw 分派**（O，0x140904ae0）：PASS_VARIANT_TABLE=[0,0,1,2,3]；count 路径 = FORCE_FINE(0x143634d9c)||(kind<=1&&!FORCE_NO_FINE(0x143634d9d))；`list_index_to_cmd_offset` 读的是 **max_instance_offsets**（B_i_nani §2.2 纠错）。
9. **最大 U**：PassResources +0x18/+0x20 资源真名；GpuWorldSubsetDesc 字段与 External draw_count；19 个实例数据字段布局（尺寸 O）；NaniMeshAtlasPartInfo 元素大小；dispatch 模式→枚举名；`impl$44::fmt`（PDB 归属 gpu_world 实为 InstanceKey 派生 Debug，疑 ICF 别名）。
10. **待集成裁决**：`DrawMode(u32)` 保留原判别值；`ResourceLookup for World` = typed gap（new_from 对 &World 会照原版 unwrap panic）；PassHost 需补 World 资源访问器（B_i_nani A4 缺口）后 nani_draw 才可去显式参数。
