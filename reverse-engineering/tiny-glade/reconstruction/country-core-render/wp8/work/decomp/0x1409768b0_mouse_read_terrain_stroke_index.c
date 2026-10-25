
void __cdecl
country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::
mouse_read_terrain_stroke_index(void)

{
  byte *pbVar1;
  longlong extraout_RAX;
  undefined4 *extraout_RAX_00;
  longlong lVar2;
  longlong in_RDX;
  ulonglong uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined8 local_3b8;
  undefined8 uStack_3b0;
  undefined8 local_3a8;
  undefined8 local_3a0;
  undefined8 uStack_398;
  undefined8 local_390;
  undefined8 local_388;
  undefined8 uStack_380;
  undefined8 local_378;
  longlong local_370;
  undefined8 uStack_368;
  undefined8 local_360;
  undefined8 local_358;
  undefined8 uStack_350;
  undefined8 local_340;
  undefined8 uStack_338;
  undefined8 local_330;
  undefined8 uStack_328;
  undefined8 local_320;
  undefined8 uStack_318;
  undefined8 local_310;
  undefined8 uStack_308;
  undefined8 local_300;
  undefined8 uStack_2f8;
  undefined8 local_2f0;
  undefined8 uStack_2e8;
  undefined8 local_2e0;
  undefined8 local_2d8;
  undefined8 local_228;
  undefined8 uStack_220;
  undefined8 local_218;
  undefined8 local_210;
  undefined8 uStack_208;
  undefined8 local_200;
  undefined8 local_1f8;
  undefined8 uStack_1f0;
  undefined8 local_1e8;
  undefined8 local_1e0;
  undefined8 uStack_1d8;
  undefined8 local_1d0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  undefined8 local_1b8;
  undefined8 uStack_1b0;
  undefined8 local_1a8;
  undefined8 uStack_1a0;
  undefined8 local_198;
  undefined8 uStack_190;
  undefined8 local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  char local_f2;
  undefined8 local_f0;
  undefined8 local_a8;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined8 local_58;
  undefined8 uStack_50;
  undefined8 local_40;
  
  local_40 = 0xfffffffffffffffe;
  uVar3 = 0x62dfe8f62b6fcc5;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX != 0) && (uVar3 < *(ulonglong *)(in_RDX + 0x218))) &&
     (uVar3 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar3 * 8), uVar3 != 0)) {
    lVar2 = ~uVar3 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar2) != 0) {
      pbVar1 = *(byte **)(*(longlong *)(in_RDX + 0x1e0) + lVar2 + 0x10);
      uVar4 = 0;
      uVar5 = 0;
      if ((*pbVar1 & 1) != 0) {
        uVar4 = *(undefined4 *)(pbVar1 + 4);
        uVar5 = *(undefined4 *)(pbVar1 + 0xc);
      }
      country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref_<str_>_>();
      country_core::render::pass_builder::compute_pass_builder::ComputePass::
      write_to_cpu_mapped_storage_buffer<country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo>
                ();
      local_98 = CONCAT71(local_98._1_7_,0xb);
      local_3b8 = CONCAT44(local_3b8._4_4_,2);
      country_core::render::pass_builder::compute_pass_builder::impl$4::
      bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>();
      memcpy(&local_3b8,&local_228,400);
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_00 != (undefined4 *)0x0) {
        *extraout_RAX_00 = uVar4;
        extraout_RAX_00[1] = uVar5;
        if (local_370 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        local_370 = 8;
        local_360 = 8;
        memcpy(&local_228,&local_3b8,400);
        local_58 = local_188;
        uStack_50 = uStack_180;
        local_68 = local_198;
        uStack_60 = uStack_190;
        local_78 = local_1a8;
        uStack_70 = uStack_1a0;
        local_88 = local_1b8;
        uStack_80 = uStack_1b0;
        local_98 = local_1c8;
        uStack_90 = uStack_1c0;
        local_3a8 = local_218;
        local_3b8 = local_228;
        uStack_3b0 = uStack_220;
        local_388 = local_210;
        uStack_380 = uStack_208;
        local_378 = local_200;
        local_3a0 = local_1f8;
        uStack_398 = uStack_1f0;
        local_390 = local_1e8;
        local_340 = local_178;
        uStack_338 = uStack_170;
        local_330 = local_168;
        uStack_328 = uStack_160;
        local_320 = local_158;
        uStack_318 = uStack_150;
        local_310 = local_148;
        uStack_308 = uStack_140;
        local_300 = local_138;
        uStack_2f8 = uStack_130;
        local_2f0 = local_128;
        uStack_2e8 = uStack_120;
        local_360 = local_1d0;
        local_370 = local_1e0;
        uStack_368 = uStack_1d8;
        local_2d8 = local_a8;
        local_2e0 = local_f0;
        local_358 = 0x100000001;
        uStack_350 = 0x100000001;
        rhapsody::rg::pass_builder::PassBuilder::
        render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
                  ();
        if (local_f2 != '\b') {
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


