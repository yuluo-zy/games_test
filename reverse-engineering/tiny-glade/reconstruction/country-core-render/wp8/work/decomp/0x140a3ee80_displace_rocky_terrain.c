
void __cdecl
country_core::systems::render_loop::compute_passes::displace_rocky_terrain::displace_rocky_terrain
          (void)

{
  longlong extraout_RAX;
  undefined4 *extraout_RAX_00;
  longlong lVar1;
  longlong in_RDX;
  ulonglong uVar2;
  undefined4 uVar3;
  undefined8 local_4e8;
  undefined8 uStack_4e0;
  undefined8 local_4d8;
  undefined8 uStack_4d0;
  undefined8 local_4c8;
  undefined8 uStack_4c0;
  undefined8 local_4b8;
  undefined8 uStack_4b0;
  undefined8 local_4a8;
  undefined8 uStack_4a0;
  undefined4 local_358;
  undefined4 uStack_354;
  undefined8 uStack_350;
  undefined8 local_348;
  undefined8 local_340;
  undefined8 uStack_338;
  undefined8 local_330;
  undefined8 local_328;
  undefined8 uStack_320;
  undefined8 local_318;
  undefined8 local_310;
  undefined8 uStack_308;
  undefined8 local_300;
  undefined8 local_2f8;
  undefined8 uStack_2f0;
  undefined8 local_2e8;
  undefined8 uStack_2e0;
  undefined8 local_2d8;
  undefined8 uStack_2d0;
  undefined8 local_2c8;
  undefined8 uStack_2c0;
  undefined8 local_2b8;
  undefined8 uStack_2b0;
  undefined8 local_2a8;
  undefined8 uStack_2a0;
  undefined8 local_298;
  undefined8 uStack_290;
  undefined8 local_288;
  undefined8 uStack_280;
  undefined8 local_278;
  undefined8 uStack_270;
  undefined8 local_268;
  undefined8 uStack_260;
  undefined8 local_258;
  undefined8 uStack_250;
  char local_222;
  undefined8 local_220;
  undefined8 local_1d8;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  undefined8 local_1b8;
  undefined8 local_1b0;
  undefined8 uStack_1a8;
  undefined8 local_1a0;
  undefined8 local_198;
  undefined8 uStack_190;
  undefined8 local_188;
  longlong local_180;
  undefined8 uStack_178;
  undefined8 local_170;
  undefined4 local_168;
  undefined4 local_164;
  undefined8 local_160;
  undefined8 local_150;
  undefined8 uStack_148;
  undefined8 local_140;
  undefined8 uStack_138;
  undefined8 local_130;
  undefined8 uStack_128;
  undefined8 local_120;
  undefined8 uStack_118;
  undefined8 local_110;
  undefined8 uStack_108;
  undefined8 local_100;
  undefined8 uStack_f8;
  undefined8 local_f0;
  undefined8 local_e8;
  undefined1 local_31;
  undefined8 local_30;
  
  local_30 = 0xfffffffffffffffe;
  uVar2 = 0xfa44538b95ae76c6;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX != 0) && (uVar2 < *(ulonglong *)(in_RDX + 0x218))) &&
     (uVar2 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar2 * 8), uVar2 != 0)) {
    lVar1 = ~uVar2 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar1) != 0) {
      uVar2 = **(ulonglong **)(*(longlong *)(in_RDX + 0x1e0) + lVar1 + 0x10);
      country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref_<str_>_>();
      country_core::render::pass_builder::compute_pass_builder::ComputePass::
      write_storage_buffer<country_core::resources::rocky_terrain::RockyTerrainVertexSSBO>();
      country_core::render::pass_builder::compute_pass_builder::ComputePass::
      write_storage_buffer<country_core::resources::rocky_terrain::RockyTerrainTriangleSSBO>();
      local_358 = CONCAT31(local_358._1_3_,9);
      local_1c8._0_4_ = 2;
      country_core::render::pass_builder::compute_pass_builder::impl$4::
      bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>();
      memcpy(&local_1c8,&local_4e8,400);
      local_31 = 10;
      local_358 = 2;
      country_core::render::pass_builder::compute_pass_builder::impl$4::
      bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>();
      memcpy(&local_358,&local_1c8,400);
      local_4e8._0_1_ = 0xe;
      local_1c8._0_4_ = 2;
      country_core::render::pass_builder::compute_pass_builder::impl$4::
      bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>();
      memcpy(&local_1c8,&local_358,400);
      local_4e8._0_1_ = 5;
      local_358 = 2;
      country_core::render::pass_builder::compute_pass_builder::impl$4::
      bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>();
      memcpy(&local_358,&local_1c8,400);
      local_4e8 = CONCAT71(local_4e8._1_7_,0x13);
      local_1c8 = CONCAT44(local_1c8._4_4_,2);
      country_core::render::pass_builder::compute_pass_builder::impl$4::
      bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>();
      memcpy(&local_1c8,&local_358,400);
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_00 != (undefined4 *)0x0) {
        uVar3 = (undefined4)(uVar2 / 3);
        *extraout_RAX_00 = uVar3;
        if (local_180 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        local_180 = 4;
        local_170 = 4;
        memcpy(&local_358,&local_1c8,400);
        local_4a8 = local_2b8;
        uStack_4a0 = uStack_2b0;
        local_4b8 = local_2c8;
        uStack_4b0 = uStack_2c0;
        local_4c8 = local_2d8;
        uStack_4c0 = uStack_2d0;
        local_4d8 = local_2e8;
        uStack_4d0 = uStack_2e0;
        local_4e8 = local_2f8;
        uStack_4e0 = uStack_2f0;
        local_1b8 = local_348;
        local_1c8 = CONCAT44(uStack_354,local_358);
        uStack_1c0 = uStack_350;
        local_198 = local_340;
        uStack_190 = uStack_338;
        local_188 = local_330;
        local_1b0 = local_328;
        uStack_1a8 = uStack_320;
        local_1a0 = local_318;
        local_150 = local_2a8;
        uStack_148 = uStack_2a0;
        local_140 = local_298;
        uStack_138 = uStack_290;
        local_130 = local_288;
        uStack_128 = uStack_280;
        local_120 = local_278;
        uStack_118 = uStack_270;
        local_110 = local_268;
        uStack_108 = uStack_260;
        local_100 = local_258;
        uStack_f8 = uStack_250;
        local_170 = local_300;
        local_180 = local_310;
        uStack_178 = uStack_308;
        local_e8 = local_1d8;
        local_f0 = local_220;
        local_168 = 1;
        local_160 = 0x100000001;
        local_164 = uVar3;
        rhapsody::rg::pass_builder::PassBuilder::
        render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
                  ();
        if (local_222 != '\b') {
          rhapsody::raw::managed::handle::impl$6::drop();
        }
        return;
      }
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}


