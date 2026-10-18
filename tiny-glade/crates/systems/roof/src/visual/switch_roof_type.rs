//! 屋顶类型切换（visual/switch_roof_type.rs）。
//!
//! `switch_roof_type`（0x1421ac4f0，2392B/567）。参数面（fresh 反编译 +
//! `disassemble_function` 0x1421ac4f0，本批 `reports/consume/nani-roof-toolbar/`）：
//! `(EventReader<SwitchRoofTypeCmd>, Query<&Roof>, ResMut<InnerWalls>,
//!  Commands, Res<AssetMeshLibrary>, EventWriter<WallGableChanged>,
//!  EventWriter<RoofSound>, EventWriter<RecalculateWallBricks>, …)`。
//!
//! 机器码语义（逐条对照见 evidence/fresh-0x1421ac4f0.c + disasm-0.txt）：
//! - 事件 stride 0x20：entity@+0x10、source_flag@+0x18、target_type@+0x19；
//! - `Roof::ty()` 断言与 target_type 相等（否则 `assert_failed` panic，0x1421ac7dd）；
//! - target==flat（ty==0）：
//!   `Commands::entity(roof).insert(NaniBundle<StoneFloorInstanceData>)`
//!   （insert 命令错误处理器 = `bevy_ecs::error::handler::panic`）；
//!   bundle 载荷（disasm 0x1421ac95f..0x1421aca1e）：
//!   `Transform` = 单位变换（rot (0,0,0,1)、translation 0、scale 1）；
//!   `NaniInstanceDesc` = `NaniMeshSelector::AtlasPart(0)`（`XOR ECX,ECX` →
//!   `mov eax,ecx;ret` 的单位函数（PDB 误名 DecoratorRank::new，字节 `89 C8 C3`）→
//!   `From`（0x14082bd60）写 tag=1/value=0）+ `GpuWorldSubsetKind::RoofFloor`（9，
//!   `MOV byte [RBP+0xa0],0x9`）；`InstanceDataMarker<T>` ZST。
//!   RoofSound kind 6（flat_surface=roof[0x54]）；roof[0x34]==1.0（ridge==1）→
//!   WallGableChanged；source_flag==1 且墙高 !=1.6 → `InnerWalls::update_wall_state_height`
//!   （MAXSS(1.6,h), MINSS(14,·)）；
//! - target!=flat：`Commands::entity(roof).remove(DrawableMeshBundle)`（remove 的
//!   错误处理器 = `bevy_ecs::error::handler::warn`，与 bevy 0.16
//!   `EntityCommands::remove` 一致）→ `format!("roof_{:?}", entity)`（pieces
//!   "roof_" + Entity 的 **Debug** 格式化函数 0x140ceac00，输出 `{index}v{gen}#{bits}`）
//!   → `AssetMeshLibrary::try_get_handle_by_name(&name)`；
//!   `Some(handle)` → `TransientMeshRegistry::deferred_delete_mesh(handle)`
//!   （0x140aa8070：Mutex lock（中毒 unwrap）→ `Vec::push(handle)` → unlock）；
//!   insert NaniBundle；RoofSound kind 0；ridge==1 → WallGableChanged；
//! - 恒有（每事件末尾）：`Commands::entity(roof).remove(Handle<Vao>)`（warn）；
//!   `RecalculateWallBricks(roof.wall_id())`（EventInstance stride 0x10）。
use bevy_ecs::prelude::*;

use country_core::asset_libraries::mesh_library::AssetMeshLibrary;
use country_core::asset_libraries::Handle;
use country_core::components::drawable::{DrawableMeshBundle, NaniBundle};
use country_core::render::nani::gpu_world::{GpuWorldSubsetKind, InstanceDataMarker};
use country_core::render::nani::instance_data::StoneFloorInstanceData;
use country_core::render::nani::systems::{NaniInstanceDesc, NaniMeshSelector};
use country_core::render::vao::Vao;
use country_core::resources::{
    roofs::{Roof, commands::*},
    walls::InnerWalls,
};
use country_core::utils::components::transform::Transform;

/// B 0x1421ac4f0。
pub fn switch_roof_type(
    mut requests: EventReader<SwitchRoofTypeCmd>,
    roofs: Query<&Roof>,
    mut walls: ResMut<InnerWalls>,
    meshes: Res<AssetMeshLibrary>,
    mut commands: Commands,
    mut gable: EventWriter<WallGableChanged>,
    mut sounds: EventWriter<RoofSound>,
    mut bricks: EventWriter<RecalculateWallBricks>,
) {
    use country_core::resources::roofs::geometry::edit_rules::HeightCrossing;
    for request in requests.read() {
        let roof = roofs
            .get(request.entity)
            .expect("roof type change requires its entity");
        let pitched = roof.is_pitched();
        // decomp-0x1421ac4f0：assert Roof::ty() == target_type（事件 target 字节
        // @+0x19 未在 core SwitchRoofTypeCmd 承载，以 crossing==Up 表达目标坡顶）。
        assert_eq!(
            pitched,
            request.crossing == HeightCrossing::Up,
            "roof type event disagrees with Roof::ty"
        );
        // O：两分支都 insert `NaniBundle<StoneFloorInstanceData>`（单位 Transform +
        // desc AtalsPart(0)/RoofFloor；字段/常量见模块注释 disasm 偏移）。
        let bundle = NaniBundle::<StoneFloorInstanceData> {
            desc: NaniInstanceDesc {
                mesh: NaniMeshSelector::AtlasPart(0),
                subset_kind: GpuWorldSubsetKind::RoofFloor,
            },
            transform: Transform::identity(),
            marker: InstanceDataMarker::default(),
        };
        if !pitched {
            // O 0x1421ac9c8..0x1421aca2a：Commands::entity(entity).insert(NaniBundle)。
            // 错误处理器 = panic（bevy 0.16 `EntityCommands::insert` 默认）。
            commands.entity(request.entity).insert(bundle);
        } else {
            // O 0x1421ac812..0x1421ac832：remove DrawableMeshBundle（warn）。
            commands.entity(request.entity).remove::<DrawableMeshBundle>();
            // O 0x1421ac850..0x1421ac8e2：format!("roof_{:?}", entity) →
            // try_get_handle_by_name；Some 时 deferred_delete_mesh（lock/push）。
            let name = format!("roof_{:?}", request.entity);
            if let Some(handle) = meshes.try_get_handle_by_name(&name) {
                // 0x140aa8070：`registry.lock().unwrap().push(handle)`。
                meshes.transient_meshes().lock().unwrap().push(handle);
            }
            commands.entity(request.entity).insert(bundle);
        }
        sounds.write(RoofSound {
            entity: request.entity,
            wall: request.wall,
            pivot: roof.pivot_3d().to_array(),
            kind: if pitched { 0 } else { 6 },
            flat_surface: (!pitched).then_some(roof.observed()[0x54]),
        });
        // Flat transitions also notify when ridge==1; is_gable() would suppress
        // that notification because the height has already become flat.
        if f32::from_le_bytes(roof.observed()[0x34..0x38].try_into().unwrap()) == 1.0 {
            gable.write(WallGableChanged(roof.wall_id()));
        }
        if !pitched && request.source_flag == 1 {
            let height = walls.get_unwrapped(request.wall).height();
            if height != 1.6 {
                // 1421acc7e..8a: MAXSS(1.6,height), MINSS(14,result).
                let minimum = if 1.6 > height { 1.6 } else { height };
                let bounded = if 14.0 < minimum { 14.0 } else { minimum };
                walls.update_wall_state_height(request.wall, bounded);
            }
        }
        // O 0x1421accc7..0x1421accf0：remove Handle<Vao>（warn）。
        commands.entity(request.entity).remove::<Handle<Vao>>();
        bricks.write(RecalculateWallBricks(request.wall));
    }
}
