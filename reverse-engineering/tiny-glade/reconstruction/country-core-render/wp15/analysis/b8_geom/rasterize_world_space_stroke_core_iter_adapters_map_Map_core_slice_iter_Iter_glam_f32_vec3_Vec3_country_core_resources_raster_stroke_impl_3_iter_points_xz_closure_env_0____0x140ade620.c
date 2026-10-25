// addr=0x140ade620 name=rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>_>
// signature=void __cdecl rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>_>(void)


void __cdecl
country_core::utils::dirty_tiles::DirtyTiles::
rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl_3::iter_points_xz::closure_env_0>_>
          (void)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 *in_RDX;
  undefined4 *puVar5;
  undefined4 *in_R8;
  
  if ((in_RDX != in_R8) && (in_RDX + 3 != in_R8)) {
    uVar1 = in_RDX[2];
    uVar2 = in_RDX[3];
    uVar3 = in_RDX[5];
    fmaf(*in_RDX,0x3efc0fc1,0x42000000);
    fmaf(uVar1,0x3efc0fc1,0x42000000);
    conservative_line_raster::impl$0::from();
    fmaf(uVar2,0x3efc0fc1,0x42000000);
    fmaf(uVar3,0x3efc0fc1,0x42000000);
    conservative_line_raster::impl$0::from();
    conservative_line_raster::
    conservative_plot_line<country_core::utils::dirty_tiles::impl_0::iter_world_space_stroke_cells::closure_env_0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env_2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env_1<country_core::systems::history::replay_path::Cobblestone2PathFilter>_>_>,country_core::utils::dirty_tiles::impl_0::rasterize_world_space_stroke::closure_env_0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env_2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env_1<country_core::systems::history::replay_path::Cobblestone2PathFilter>_>_>_>_>_>
              ();
    for (puVar5 = in_RDX + 6; puVar5 != in_R8; puVar5 = puVar5 + 3) {
      uVar1 = *puVar5;
      uVar4 = puVar5[2];
      fmaf(uVar2,0x3efc0fc1,0x42000000);
      fmaf(uVar3,0x3efc0fc1,0x42000000);
      conservative_line_raster::impl$0::from();
      fmaf(uVar1,0x3efc0fc1,0x42000000);
      fmaf(uVar4,0x3efc0fc1,0x42000000);
      conservative_line_raster::impl$0::from();
      conservative_line_raster::
      conservative_plot_line<country_core::utils::dirty_tiles::impl_0::iter_world_space_stroke_cells::closure_env_0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env_2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env_1<country_core::systems::history::replay_path::Cobblestone2PathFilter>_>_>,country_core::utils::dirty_tiles::impl_0::rasterize_world_space_stroke::closure_env_0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env_2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env_1<country_core::systems::history::replay_path::Cobblestone2PathFilter>_>_>_>_>_>
                ();
      uVar2 = uVar1;
      uVar3 = uVar4;
    }
  }
  return;
}


