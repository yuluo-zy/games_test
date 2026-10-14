# render-gap HIGHLIGHTS（owner: tg_rg1，2026-10-14）

1. **§2.1 全部 19 单元核实完毕**：真实逻辑仅 2 项——`render_config::impl$7::from` 0x140968c40（`From<&GraphicsDevice> for LastKnownGraphicsDevice{uuid,driver}`，未实现待补）、dof 两个 `source_hash`（0x140a5de00/0x140a5d960，已实现只需补地址标注）；其余 17 项均为 serde/fmt/Clone/derive(Component) 胶水（register_required_components=`Handle<Vao>` 的 `#[derive(Component)]`）。
2. **reset_indirect_args 0x1e0 B 暂存区 U 项闭合**：反编译证实 `memcpy(staging, &0x142aaaa78, 0x1e0)`——静态表 = 24 条 `DrawIndexedIndirectCommand`（仅 index_count 非零，`[0x1b,0x12c,0x60,…]`）。B_c3"无写入"判断错误，reconstruction 的 0xAB 断言需改。
3. **shader_refactoring disc 5..9 机器码精确化**：disc5 = `Result::unwrap()` panic（非 `unreachable!`，c3 误判）；disc6..9 跳进共享收尾代码中部（退化分支），且第二表更新只产出 disc ∈{0,2,3,4} → disc5..9 **实际不可达**，现有 no-op 模型安全。
4. **三个 TypeId 组件真名已反查（O）**：`0x66476a13bbcc5e41`=`BackgroundTerrain`、`0xa6c0fc01ec67ef4c`=`DontReceiveContactShadows`、`0x192600af2434483b`=`OverwriteRoughness`（f32 缺省 0.8）；经 `component_ids` bundle 泛型 + env27 读序证实，确认 components/mod.rs 候选名。
5. **`*0x14362c0d8`/`*0x14362c098` 语义还原**：是指针全局 → `0x143634da0`=freeze_occlusion_culling、`0x143634d90`=demo 开关；非 0 分别跳过墙剔除重生成 / 折射水面 pass。
6. **CENTERED_PATH 无运行期写入者确认**：qword `0x143638fac` 全 EXE 仅 .data 初始值一处，.text 零绝对引用 → 恒 0。
7. **`TextureDesc::from(preset)` 0x140a59890 已反编译**：`From<TexturePreset> for TextureDesc` switch 0..0x1f，48 B desc + `NameHash`@+0x20；reconstruction 的 `From<u8>` 归因错位、`usage()` 11/12 判 depth 有误。
8. **grading_lut 7 处 unimplemented 落码依据齐备**：audio-misc §2.9 + wp19 `load_cube_140b01050.c`（`.cube` 解析器全 O：TITLE/#/DOMAIN_MIN/MAX/LUT_3D_SIZE/RGB9E5）。
9. **render_ssbo 5 处 typed-gap 有真实元素形态**：DofFocus(64B)/DofFocusReadback(16B)/u32[256]/f32 均来自重建 HLSL，可闭合。
10. **纠错登记**：`nani::gpu_world::impl$44::fmt` 是 Debug 胶水（写 "InstanceKey"），非"PDB 归属不符"；atmosphere serialize 字段揭示 atmosphere 资源 = 4×Vec3（与 SunSkyColors 3×Vec4 不同型）。

新反编译 17 个函数落盘 `render-gap/decomp/`；完整分析见 `render-gap/DIGEST.md`。
