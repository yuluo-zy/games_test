//! 木门构造（construct_gates/mod.rs + systems.rs）。
//!
//! `systems::construct_gates`（0x140756370）。**9 参数 SystemParam 顺序**
//! （building-gates findings G06）：`(EventReader<ConstructGateCmd>,
//!  ResMut<Gates>, Commands, ResMut<AssetMeshLibrary>, Res<AssetShaderLibrary>,
//!  Res<PublicWalls>, Res<TerrainHeightsData>, Res<AssetSsboLibrary>,
//!  Res<UiSignifierStream>)`；120B 事件 + 8B EventId = 128B 元素。
//! `update_gates_hashmap`（0x140755690）：6 组合（changed/create/switch
//! rebuild + wall/roof removal）despawn 顺序；重建保留空条目。
//! `construct_wooden_gate_doors`（0x141210110）：门宽**严格 >2.7** 双门扇
//! （间距 min(.05/全长,.1)），==2.7 单门；header inset bits3f547ae2；footer
//! 在 mesh offset 之后重载端点；右轴归一=除 length 非倒数乘；136B spawn
//! payload；seed=inner·ordinal+leaf ordinal u32；Shader 键
//! solid_vertex_color_wooden_gate；负 Y 比例选择 + ratio>.5 内柱。
pub mod systems;
