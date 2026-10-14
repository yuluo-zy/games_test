# roofs-rt 域 HIGHLIGHTS（10 条最重要发现）

1. **roof_shape 28 业务全部澄清**：24 个 O（其中 roof_ridge 6/6、Roof 几何族由 reconstruction/roof 的 ridge.rs/surface.rs/roof_state.rs 验证性恢复，不重挖）；`Roof::get_roof_shape` 完全 = surface.rs（3,004 案例 0 差异）。
2. **Roof 布局 0x58 字节全部关键偏移已证实**：+8 tag、+0x24/0x28 tip、+0x2c profile、+0x30 height、+0x34 ridge、+0x38 eave、+0x3c ridge_dir、+0x3d rim、+0x4c floor、+0x54 tag 字节；RoofShapeParams 28B 块映射齐全。
3. **RoofTriggerVolume 三 Vec 布局**：triangles/normals/uvs 各 cap-first，stride 0x24/0x24/0x18，set_* 带 `triangles.len()==normals.len()/uvs.len()` 断言（panic 串已取）；get_position_from_mesh_uv 复用 utils position_from_mesh_uvs。
4. **屋顶噪声链证实**：gpunoise 是 CPU 复刻的 shader value-noise（Wang 哈希 0x19660d/0x8001/0xae135d3）；`apply_roof_noise` 把 `noise(x*0.1,z*0.1)*1.5*smoothstep(strength)` 加到顶点 y；`get_height_01_unclamped_from_ws` 反向采样也调同一噪声。
5. **rt/mod.rs CPU 侧全链**：`update_rt_world`（帧差→脏位→RtBvhBuilderGlueResource::new，"UpdateRtWorldEvent requested"）与 `upload_rt_world`（脏条目 0x8000… 哨兵占位→字典取三角→3 SSBO stride 0x50/0x18/4×u32 上传）为两个真实系统；RtWorld/DirtyCow 引用计数语义（refcount==1 原地）。
6. **build_gpu_bvh**：输入三角 stride 0x28、输出 BVH 节点 stride 0x30，空输入退化 3 节点根；bvh_builder_worker 是独立线程状态机（busy/state/processing + 全局停止标志 0x14362c420）。对应 render-passes 的软件 BVH（无硬件光追）。
7. **MeshAtlasLibrary**：子集图/名称图 atlas 哈希表，register_meshes 每子集重建并接 rt（RtBvhBuilderGlueResource::new），panic 串确认源码路径 mesh_atlas_library.rs。
8. **TreeStorage**：16 字节条目哈希表（iter 用 pmovmskb 占用位图），next_index@+0x30；get/insert/remove 全证。
9. **Bevy 注册胶水归类**：全部 `register_required_components`（roof_animation/tiles/tile_sim/trigger_volume/tree_storage/shape）均为标准 Bevy 0.16 追加 required-components 模式，不计独立业务；`clone_behavior` 0x140002370 是跨 6+ 域共享的 `xor eax,eax;ret` 空桩（Roof 真实现 0x1408e6af0）。
10. **PDB 名/地址去重纠错**：0x140833230/0x1408c5680 与 wall 域 is_follow_terrain/is_fence 同地址（5 字节谓词去重）；旧域 0x140a1b100/0x14067dd80 的 decomp 副本是 NotFound 错文件，本域已重解——机器码为准。
