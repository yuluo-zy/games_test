
void __cdecl
country_core::systems::render_loop::compute_passes::compute_water::water_spawn_vegetation(void)

{
  longlong extraout_RAX;
  longlong in_RDX;
  ulonglong uVar1;
  undefined8 local_4e8;
  undefined8 uStack_4e0;
  undefined8 local_4d8;
  undefined8 uStack_4d0;
  undefined8 uStack_4c8;
  undefined8 local_4c0;
  undefined8 local_4b8;
  undefined8 uStack_4b0;
  undefined8 local_4a8;
  undefined8 uStack_4a0;
  undefined8 uStack_498;
  undefined8 local_490;
  undefined8 local_488;
  undefined8 uStack_480;
  undefined8 local_478;
  undefined8 uStack_470;
  undefined8 local_468;
  undefined8 uStack_460;
  undefined8 local_458;
  undefined8 uStack_450;
  undefined8 local_448;
  undefined8 uStack_440;
  undefined8 local_438;
  undefined8 uStack_430;
  undefined8 local_428;
  undefined8 uStack_420;
  undefined8 local_418;
  undefined8 uStack_410;
  undefined8 local_408;
  undefined8 uStack_400;
  undefined8 local_3f8;
  undefined8 uStack_3f0;
  undefined8 local_3e8;
  undefined8 uStack_3e0;
  char local_3b2;
  undefined8 local_3b0;
  undefined8 local_368;
  undefined8 local_358;
  undefined8 uStack_350;
  undefined8 local_348;
  undefined8 uStack_340;
  undefined8 local_338;
  undefined8 uStack_330;
  undefined8 local_328;
  undefined8 uStack_320;
  undefined8 local_318;
  undefined8 uStack_310;
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
  undefined8 local_180;
  undefined8 uStack_178;
  undefined8 local_170;
  undefined8 local_168;
  undefined8 uStack_160;
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
  char local_92;
  undefined1 local_32;
  undefined1 local_31;
  undefined8 local_30;
  
  local_30 = 0xfffffffffffffffe;
  if (_DEV_OVERRIDE_IS_DEMO_MODE == '\0') {
    country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref_<str_>_>();
    country_core::render::pass_builder::compute_pass_builder::ComputePass::
    write_storage_buffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo>
              ();
    country_core::render::pass_builder::compute_pass_builder::ComputePass::
    write_storage_buffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo>
              ();
    local_32 = 1;
    uVar1 = 0x74430dfcc6d8719b;
    bevy_ecs::component::Components::get_resource_id();
    if ((((extraout_RAX == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar1)) ||
        (uVar1 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar1 * 8), uVar1 == 0)) ||
       (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
      local_32 = 1;
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    if (local_92 != '\b') {
      country_core::render::pass_builder::uniform_binder::UniformBinder::
      write_uniform<country_core::render::pass_builder::draw_builder::impl_2::uniform::closure_env_0<bool>,bool>
                ();
    }
    memcpy(&local_358,&local_1c8,400);
    if (local_222 != '\b') {
      country_core::render::pass_builder::uniform_binder::UniformBinder::
      write_uniform<country_core::render::pass_builder::draw_builder::impl_2::uniform::closure_env_0<array_<u32,2>_>,array_<u32,2>_>
                ();
    }
    memcpy(&local_4e8,&local_358,400);
    local_358 = CONCAT71(local_358._1_7_,9);
    local_1c8._0_4_ = 2;
    country_core::render::pass_builder::compute_pass_builder::impl$4::
    bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>();
    memcpy(&local_1c8,&local_4e8,400);
    local_31 = 10;
    local_358._0_4_ = 2;
    country_core::render::pass_builder::compute_pass_builder::impl$4::
    bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>();
    memcpy(&local_358,&local_1c8,400);
    local_4e8._0_1_ = 0x13;
    local_1c8._0_4_ = 2;
    country_core::render::pass_builder::compute_pass_builder::impl$4::
    bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>();
    memcpy(&local_1c8,&local_358,400);
    local_4e8._0_1_ = 5;
    local_358._0_4_ = 2;
    country_core::render::pass_builder::compute_pass_builder::impl$4::
    bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>();
    memcpy(&local_358,&local_1c8,400);
    local_4e8._0_1_ = 0x14;
    local_1c8 = CONCAT44(local_1c8._4_4_,2);
    country_core::render::pass_builder::compute_pass_builder::impl$4::
    bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>();
    memcpy(&local_1c8,&local_358,400);
    local_4e8 = CONCAT71(local_4e8._1_7_,0x17);
    local_358 = CONCAT44(local_358._4_4_,2);
    country_core::render::pass_builder::compute_pass_builder::impl$4::
    bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>();
    memcpy(&local_358,&local_1c8,400);
    country_core::render::pass_builder::compute_pass_builder::ComputePass::
    write_storage_buffer_concurrent<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>
              ();
    if (local_92 != '\b') {
      country_core::render::pass_builder::uniform_binder::UniformBinder::
      write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<i32>,i32>
                ();
    }
    memcpy(&local_4e8,&local_1c8,400);
    local_318 = local_448;
    uStack_310 = uStack_440;
    local_328 = local_458;
    uStack_320 = uStack_450;
    local_338 = local_468;
    uStack_330 = uStack_460;
    local_348 = local_478;
    uStack_340 = uStack_470;
    local_358 = local_488;
    uStack_350 = uStack_480;
    local_1b8 = local_4d8;
    local_1c8 = local_4e8;
    uStack_1c0 = uStack_4e0;
    local_198 = uStack_4d0;
    uStack_190 = uStack_4c8;
    local_188 = local_4c0;
    local_1b0 = local_4b8;
    uStack_1a8 = uStack_4b0;
    local_1a0 = local_4a8;
    local_150 = local_438;
    uStack_148 = uStack_430;
    local_140 = local_428;
    uStack_138 = uStack_420;
    local_130 = local_418;
    uStack_128 = uStack_410;
    local_120 = local_408;
    uStack_118 = uStack_400;
    local_110 = local_3f8;
    uStack_108 = uStack_3f0;
    local_100 = local_3e8;
    uStack_f8 = uStack_3e0;
    local_170 = local_490;
    local_180 = uStack_4a0;
    uStack_178 = uStack_498;
    local_e8 = local_368;
    local_f0 = local_3b0;
    local_168 = 0xe100000001;
    uStack_160 = 0x1000000e1;
    rhapsody::rg::pass_builder::PassBuilder::
    render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
              ();
    if (local_3b2 != '\b') {
      rhapsody::raw::managed::handle::impl$6::drop();
    }
    country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref_<str_>_>();
    country_core::render::pass_builder::compute_pass_builder::ComputePass::
    write_storage_buffer<country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo>
              ();
    if (local_222 != '\b') {
      country_core::render::pass_builder::uniform_binder::UniformBinder::
      write_uniform<country_core::render::pass_builder::draw_builder::impl_2::uniform::closure_env_0<array_<u32,2>_>,array_<u32,2>_>
                ();
    }
    memcpy(&local_4e8,&local_358,400);
    local_358 = CONCAT71(local_358._1_7_,9);
    local_1c8._0_4_ = 2;
    country_core::render::pass_builder::compute_pass_builder::impl$4::
    bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>();
    memcpy(&local_1c8,&local_4e8,400);
    local_31 = 10;
    local_358._0_4_ = 2;
    country_core::render::pass_builder::compute_pass_builder::impl$4::
    bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>();
    memcpy(&local_358,&local_1c8,400);
    local_4e8._0_1_ = 0x13;
    local_1c8._0_4_ = 2;
    country_core::render::pass_builder::compute_pass_builder::impl$4::
    bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>();
    memcpy(&local_1c8,&local_358,400);
    local_4e8 = CONCAT71(local_4e8._1_7_,5);
    local_358 = CONCAT44(local_358._4_4_,2);
    country_core::render::pass_builder::compute_pass_builder::impl$4::
    bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>();
    memcpy(&local_358,&local_1c8,400);
    local_31 = 0x16;
    local_1c8 = CONCAT44(local_1c8._4_4_,2);
    country_core::render::pass_builder::compute_pass_builder::impl$4::
    bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>();
    memcpy(&local_4e8,&local_358,400);
    country_core::render::pass_builder::compute_pass_builder::ComputePass::
    write_storage_buffer_concurrent<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>
              ();
    if (local_92 != '\b') {
      country_core::render::pass_builder::uniform_binder::UniformBinder::
      write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<i32>,i32>
                ();
    }
    memcpy(&local_358,&local_1c8,400);
    local_4a8 = local_2b8;
    uStack_4a0 = uStack_2b0;
    local_4b8 = local_2c8;
    uStack_4b0 = uStack_2c0;
    uStack_4c8 = local_2d8;
    local_4c0 = uStack_2d0;
    local_4d8 = local_2e8;
    uStack_4d0 = uStack_2e0;
    local_4e8 = local_2f8;
    uStack_4e0 = uStack_2f0;
    local_1b8 = local_348;
    local_1c8 = local_358;
    uStack_1c0 = uStack_350;
    local_198 = uStack_340;
    uStack_190 = local_338;
    local_188 = uStack_330;
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
    local_180 = uStack_310;
    uStack_178 = uStack_308;
    local_e8 = local_1d8;
    local_f0 = local_220;
    local_168 = 0x9600000001;
    uStack_160 = 0x100000096;
    rhapsody::rg::pass_builder::PassBuilder::
    render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
              ();
    if (local_222 != '\b') {
      rhapsody::raw::managed::handle::impl$6::drop();
    }
  }
  return;
}

