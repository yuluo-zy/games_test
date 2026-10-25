// render_world
// sig: void __cdecl render_world(void)
// size: 4173

// WARNING: Removing unreachable block (ram,0x0001404ed126)

void __cdecl tiny_glade::render_loop::render_world(void)

{
  char cVar1;
  undefined4 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  char extraout_AL;
  undefined4 extraout_EAX;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  undefined8 *in_RCX;
  longlong lVar7;
  ulonglong uVar8;
  longlong in_R8;
  code *pcVar9;
  ulonglong in_stack_00000028;
  longlong in_stack_00000030;
  undefined8 *in_stack_00000038;
  undefined8 local_1f8;
  undefined8 uStack_1f0;
  undefined8 local_1e8;
  undefined8 uStack_1e0;
  undefined8 local_1d8;
  undefined8 uStack_1d0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  undefined4 local_188;
  undefined4 local_168;
  undefined4 uStack_164;
  undefined4 uStack_160;
  undefined4 uStack_15c;
  undefined4 local_158;
  char cStack_154;
  undefined2 uStack_153;
  undefined1 uStack_151;
  undefined4 local_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 local_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 local_88;
  
  if ((in_stack_00000028 >> 0x20 == 0) || ((in_stack_00000028 & 0xffffffff) == 0)) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  uVar8 = 0x74430dfcc6d8719b;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX != 0) && (uVar8 < *(ulonglong *)(in_R8 + 0x218))) &&
      (uVar8 = *(ulonglong *)(*(longlong *)(in_R8 + 0x210) + uVar8 * 8), uVar8 != 0)) &&
     (*(longlong *)(*(longlong *)(in_R8 + 0x1e0) + 0x20 + ~uVar8 * 0x60) != 0)) {
    country_core::systems::save::save_backup_list::BackupSaveFileKind::is_corrupted();
    local_168 = (undefined4)*(undefined8 *)((longlong)in_stack_00000038 + 0x14);
    uStack_164 = (undefined4)
                 ((ulonglong)*(undefined8 *)((longlong)in_stack_00000038 + 0x14) >> 0x20);
    uStack_160 = (undefined4)*(undefined8 *)((longlong)in_stack_00000038 + 0x1c);
    uStack_15c = (undefined4)
                 ((ulonglong)*(undefined8 *)((longlong)in_stack_00000038 + 0x1c) >> 0x20);
    rhapsody::raw::texture::TextureDesc::format();
    rhapsody::raw::texture::TextureDesc::usage();
    rhapsody::rg::temporal::impl$11::
    get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref_<str_>_>
              ();
    if (cStack_154 == '\x02') {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    country_core::systems::render_loop::compute_passes::upload_world_raster::
    upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PathRaster>();
    country_core::systems::render_loop::compute_passes::upload_world_raster::
    upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone1Raster>
              ();
    country_core::systems::render_loop::compute_passes::upload_world_raster::
    upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone2Raster>
              ();
    country_core::systems::render_loop::compute_passes::upload_world_raster::
    upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone3Raster>
              ();
    country_core::systems::render_loop::compute_passes::upload_world_raster::
    upload_raster_staging_to_texture<country_core::resources::world_raster_defs::WaterRaster>();
    country_core::systems::render_loop::compute_passes::upload_world_raster::
    upload_raster_staging_to_texture<country_core::resources::world_raster_defs::GardenRaster>();
    country_core::systems::render_loop::compute_passes::compute_water::water_rasterize_from_cpu();
    country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::
    rasterize_terrain_stroke();
    country_core::systems::render_loop::compute_passes::displace_rocky_terrain::
    displace_rocky_terrain();
    country_core::systems::render_loop::compute_passes::read_terrain_data::read_terrain_data();
    country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::
    mouse_read_terrain_stroke_index();
    country_core::systems::render_loop::draw_passes::foliage_exclusion_mask::
    rasterize_foliage_exclusion_mask();
    country_core::systems::render_loop::compute_passes::highlight_active_terrain_stroke::
    highlight_active_terrain_stroke();
    country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass();
    country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks();
    country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings();
    country_core::systems::render_loop::compute_passes::compute_garden::garden_rasterize_from_cpu();
    country_core::systems::render_loop::compute_passes::compute_water::water_spawn_vegetation();
    country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::
    spawn_vegetation_around_buildings();
    country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::
    read_bush_positions_for_cpu();
    country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation();
    country_core::systems::render_loop::compute_passes::path_pebbles::path_pebbles_compute();
    country_core::systems::render_loop::compute_passes::leaf_particles::leaf_particles_compute();
    country_core::systems::render_loop::compute_passes::roof_tiles_gravity::
    roof_tiles_gravity_compute();
    country_core::systems::render_loop::compute_passes::stairs::stairs();
    country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::
    generate_wall_draw_lists();
    country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::
    gpu_generate_draw_lists();
    country_core::systems::render_loop::draw_passes::shadow_pass::shadow_pass();
    country_core::systems::render_loop::compute_passes::terrain_lighting_texture::
    calc_terrain_lighting_texture();
    rhapsody::raw::texture::TextureDesc::new_2d();
    rhapsody::rg::resource::impl$4::from();
    rhapsody::rg::graph::RenderGraph::create_raw_resource();
    rhapsody::raw::texture::TextureDesc::new_2d();
    rhapsody::rg::resource::impl$4::from();
    rhapsody::rg::graph::RenderGraph::create_raw_resource();
    uVar3 = *in_stack_00000038;
    uVar4 = in_stack_00000038[1];
    uVar5 = in_stack_00000038[2];
    uVar6 = in_stack_00000038[3];
    uVar2 = *(undefined4 *)(in_stack_00000038 + 4);
    country_core::systems::render_loop::draw_passes::main_pass::main_pass();
    country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb();
    if (*
        __imp__ZN12country_core7systems11render_loop9resources24FREEZE_OCCLUSION_CULLING17h90f467abb0e1e73eE
        == '\0') {
      country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::
      generate_wall_draw_lists_pass2();
      country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::
      gpu_generate_draw_lists();
    }
    country_core::systems::render_loop::compute_passes::grass::grass_compute();
    country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2();
    country_core::systems::render_loop::compute_passes::visibility_queries::run_visibility_queries()
    ;
    country_core::systems::render_loop::compute_passes::dof_autofocus::calculate_dof_autofocus();
    country_core::systems::render_loop::compute_passes::mouse_depth_value::mouse_depth_compute();
    country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass();
    country_core::systems::render_loop::compute_passes::half_res::extract_half_res_depth_normals();
    country_core::systems::render_loop::compute_passes::reprojection::calculate_reprojection_map();
    country_core::systems::render_loop::compute_passes::ssgi::calculate_ssgi();
    cVar1 = *(char *)(in_stack_00000030 + 0xab);
    if (cVar1 == '\0') {
      rhapsody::raw::texture::TextureDesc::extent_2d();
      rhapsody::raw::texture::TextureDesc::new_2d();
      rhapsody::raw::texture::TextureDesc::sampled();
      rhapsody::raw::texture::TextureDesc::storage();
    }
    else {
      uVar8 = 0xa4c02661e67b048b;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_00 == 0) || (*(ulonglong *)(in_R8 + 0x218) <= uVar8)) ||
         ((uVar8 = *(ulonglong *)(*(longlong *)(in_R8 + 0x210) + uVar8 * 8), uVar8 == 0 ||
          (*(longlong *)(*(longlong *)(in_R8 + 0x1e0) + 0x20 + ~uVar8 * 0x60) == 0)))) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      rhapsody::raw::texture::TextureDesc::new_2d();
      rhapsody::raw::texture::TextureDesc::sampled();
      rhapsody::raw::texture::TextureDesc::storage();
    }
    rhapsody::rg::temporal::impl$20::
    get_or_create_temporal_pair<country_core::render::pass_builder::pass_resources::PassResources,ref_<str_>_>
              ();
    local_88 = CONCAT13(uStack_151,CONCAT21(uStack_153,cStack_154));
    local_98 = uStack_164;
    uStack_94 = uStack_160;
    uStack_90 = uStack_15c;
    uStack_8c = local_158;
    local_a8 = 1;
    uStack_a4 = 8;
    uStack_a0 = 0;
    uStack_9c = local_168;
    country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi();
    country_core::systems::render_loop::compute_passes::eval_gi_screen_radiance::
    eval_gi_screen_radiance();
    country_core::systems::render_loop::compute_passes::bin_lights::bin_lights();
    country_core::systems::render_loop::compute_passes::restir_di::restir_sample_direct_lights();
    country_core::systems::render_loop::compute_passes::apply_screen_shadows::apply_screen_shadows()
    ;
    country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster();
    if (*__imp__ZN12country_core26_DEV_OVERRIDE_IS_DEMO_MODE17h6f2a49c135369f5aE == '\0') {
      rhapsody::raw::texture::TextureDesc::extent_2d();
      rhapsody::raw::texture::TextureDesc::new_2d();
      rhapsody::rg::resource::impl$4::from();
      rhapsody::rg::graph::RenderGraph::create_raw_resource();
      uStack_a4 = (undefined4)local_1d8;
      uStack_a0 = (undefined4)((ulonglong)local_1d8 >> 0x20);
      uStack_9c = (undefined4)uStack_1d0;
      local_98 = (undefined4)((ulonglong)uStack_1d0 >> 0x20);
      uStack_94 = (undefined4)local_1c8;
      uStack_90 = (undefined4)((ulonglong)local_1c8 >> 0x20);
      uStack_8c = (undefined4)uStack_1c0;
      local_88 = (undefined4)((ulonglong)uStack_1c0 >> 0x20);
      country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass();
      pcVar9 = country_core::systems::render_loop::draw_passes::transparency_pass::
               water_surface_pass;
      if (extraout_AL != '\0') {
        pcVar9 = country_core::systems::render_loop::draw_passes::transparency_pass::
                 ice_surface_pass;
      }
      (*pcVar9)();
      local_a8 = extraout_EAX;
    }
    country_core::systems::render_loop::draw_passes::transparency_pass::pre_taa_transparency_pass();
    country_core::systems::render_loop::compute_passes::apply_screen_shadows::
    compute_shadow_mask_mean();
    country_core::systems::render_loop::compute_passes::apply_volumetrics::apply_volumetrics();
    country_core::systems::render_loop::compute_passes::dof::dof_prepare();
    uVar8 = 0xc2b8e941c91a6428;
    bevy_ecs::component::Components::get_resource_id();
    if (((extraout_RAX_01 != 0) && (uVar8 < *(ulonglong *)(in_R8 + 0x218))) &&
       (uVar8 = *(ulonglong *)(*(longlong *)(in_R8 + 0x210) + uVar8 * 8), uVar8 != 0)) {
      lVar7 = ~uVar8 * 0x60;
      if (*(longlong *)(*(longlong *)(in_R8 + 0x1e0) + 0x20 + lVar7) != 0) {
        if (*(char *)((ulonglong)(**(int **)(*(longlong *)(in_R8 + 0x1e0) + lVar7 + 0x10) == 0x13) +
                      0xa9 + in_stack_00000030) != '\0') {
          country_core::systems::render_loop::compute_passes::generate_mips::
          generate_mips<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked>
                    ();
          country_core::systems::render_loop::compute_passes::dof::calc_dof();
        }
        if (cVar1 == '\0') {
          rhapsody::raw::texture::TextureDesc::extent_2d();
          rhapsody::raw::texture::TextureDesc::new_2d();
          rhapsody::rg::resource::impl$4::from();
          rhapsody::rg::graph::RenderGraph::create_raw_resource();
          country_core::systems::render_loop::compute_passes::taa::calculate_taa();
        }
        else {
          uVar8 = 0xa4c02661e67b048b;
          bevy_ecs::component::Components::get_resource_id();
          if ((((extraout_RAX_02 == 0) || (*(ulonglong *)(in_R8 + 0x218) <= uVar8)) ||
              (uVar8 = *(ulonglong *)(*(longlong *)(in_R8 + 0x210) + uVar8 * 8), uVar8 == 0)) ||
             (*(longlong *)(*(longlong *)(in_R8 + 0x1e0) + 0x20 + ~uVar8 * 0x60) == 0)) {
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
          rhapsody::raw::texture::TextureDesc::new_2d();
          rhapsody::rg::resource::impl$4::from();
          rhapsody::rg::graph::RenderGraph::create_raw_resource();
          country_core::systems::render_loop::compute_passes::taa::calculate_tgsr();
        }
        country_core::systems::render_loop::draw_passes::transparency_pass::
        post_taa_transparency_pass();
        country_core::systems::render_loop::compute_passes::shader_refactoring::
        render_shader_refactoring();
        *(undefined4 *)((longlong)in_RCX + 0x44) = local_88;
        *(ulonglong *)((longlong)in_RCX + 0x34) = CONCAT44(uStack_94,local_98);
        *(ulonglong *)((longlong)in_RCX + 0x3c) = CONCAT44(uStack_8c,uStack_90);
        *(ulonglong *)((longlong)in_RCX + 0x24) = CONCAT44(uStack_a4,local_a8);
        *(ulonglong *)((longlong)in_RCX + 0x2c) = CONCAT44(uStack_9c,uStack_a0);
        in_RCX[9] = local_1f8;
        in_RCX[10] = uStack_1f0;
        in_RCX[0xb] = local_1e8;
        in_RCX[0xc] = uStack_1e0;
        *(undefined4 *)(in_RCX + 0xd) = local_188;
        *in_RCX = uVar3;
        in_RCX[1] = uVar4;
        in_RCX[2] = uVar5;
        in_RCX[3] = uVar6;
        *(undefined4 *)(in_RCX + 4) = uVar2;
        return;
      }
    }
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

