
void __cdecl
country_core::systems::render_loop::compute_passes::mouse_depth_value::mouse_depth_compute(void)

{
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong in_RDX;
  ulonglong uVar1;
  undefined8 local_3e8;
  undefined8 uStack_3e0;
  undefined8 local_3d8;
  undefined8 local_3d0;
  undefined8 uStack_3c8;
  undefined8 local_3c0;
  undefined8 local_3b8;
  undefined8 uStack_3b0;
  undefined8 local_3a8;
  undefined8 local_3a0;
  undefined8 uStack_398;
  undefined8 local_390;
  undefined8 local_388;
  undefined8 uStack_380;
  undefined8 local_370;
  undefined8 uStack_368;
  undefined8 local_360;
  undefined8 uStack_358;
  undefined8 local_350;
  undefined8 uStack_348;
  undefined8 local_340;
  undefined8 uStack_338;
  undefined8 local_330;
  undefined8 uStack_328;
  undefined8 local_320;
  undefined8 uStack_318;
  undefined8 local_310;
  undefined8 local_308;
  char local_2b2;
  undefined8 local_258;
  undefined8 uStack_250;
  undefined8 local_248;
  undefined8 local_240;
  undefined8 uStack_238;
  undefined8 local_230;
  undefined8 local_228;
  undefined8 uStack_220;
  undefined8 local_218;
  undefined8 local_210;
  undefined8 uStack_208;
  undefined8 local_200;
  undefined8 local_1f8;
  undefined8 uStack_1f0;
  undefined8 local_1e8;
  undefined8 uStack_1e0;
  undefined8 local_1d8;
  undefined8 uStack_1d0;
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
  char local_122;
  undefined8 local_120;
  undefined8 local_d8;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined1 local_71;
  undefined8 local_70;
  
  local_70 = 0xfffffffffffffffe;
  uVar1 = 0x4cab10763b3fdb80;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar1)) ||
      (uVar1 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar1 * 8), uVar1 == 0)) ||
     (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  uVar1 = 0x3d7b56e787d47c40;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_00 != 0) && (uVar1 < *(ulonglong *)(in_RDX + 0x218))) &&
     ((uVar1 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar1 * 8), uVar1 != 0 &&
      (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar1 * 0x60) != 0)))) {
    country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref_<str_>_>();
    country_core::render::pass_builder::compute_pass_builder::ComputePass::
    write_to_cpu_mapped_storage_buffer<country_core::resources::compute_mouse_depth_value::MouseDepthValueSsbo>
              ();
    local_3e8 = CONCAT44(local_3e8._4_4_,2);
    country_core::render::pass_builder::compute_pass_builder::impl$4::
    bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
              ();
    memcpy(&local_3e8,&local_258,400);
    if (local_2b2 != '\b') {
      country_core::render::pass_builder::uniform_binder::UniformBinder::
      write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<glam::f32::vec2::Vec2>,glam::f32::vec2::Vec2>
                ();
    }
    memcpy(&local_258,&local_3e8,400);
    local_71 = 1;
    uVar1 = 0x864ddca98c2312a6;
    bevy_ecs::component::Components::get_resource_id();
    if (((extraout_RAX_01 != 0) && (uVar1 < *(ulonglong *)(in_RDX + 0x218))) &&
       ((uVar1 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar1 * 8), uVar1 != 0 &&
        (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar1 * 0x60) != 0)))) {
      if (local_122 != '\b') {
        country_core::render::pass_builder::uniform_binder::UniformBinder::
        write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<glam::f32::vec2::Vec2>,glam::f32::vec2::Vec2>
                  ();
      }
      memcpy(&local_3e8,&local_258,400);
      if (local_2b2 != '\b') {
        country_core::render::pass_builder::uniform_binder::UniformBinder::
        write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<f32>,f32>
                  ();
      }
      memcpy(&local_258,&local_3e8,400);
      local_88 = local_1b8;
      uStack_80 = uStack_1b0;
      local_98 = local_1c8;
      uStack_90 = uStack_1c0;
      local_a8 = local_1d8;
      uStack_a0 = uStack_1d0;
      local_b8 = local_1e8;
      uStack_b0 = uStack_1e0;
      local_c8 = local_1f8;
      uStack_c0 = uStack_1f0;
      local_3d8 = local_248;
      local_3e8 = local_258;
      uStack_3e0 = uStack_250;
      local_3b8 = local_240;
      uStack_3b0 = uStack_238;
      local_3a8 = local_230;
      local_3d0 = local_228;
      uStack_3c8 = uStack_220;
      local_3c0 = local_218;
      local_370 = local_1a8;
      uStack_368 = uStack_1a0;
      local_360 = local_198;
      uStack_358 = uStack_190;
      local_350 = local_188;
      uStack_348 = uStack_180;
      local_340 = local_178;
      uStack_338 = uStack_170;
      local_330 = local_168;
      uStack_328 = uStack_160;
      local_320 = local_158;
      uStack_318 = uStack_150;
      local_390 = local_200;
      local_3a0 = local_210;
      uStack_398 = uStack_208;
      local_308 = local_d8;
      local_310 = local_120;
      local_388 = 0x100000001;
      uStack_380 = 0x100000001;
      rhapsody::rg::pass_builder::PassBuilder::
      render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
                ();
      if (local_122 != '\b') {
        rhapsody::raw::managed::handle::impl$6::drop();
      }
      return;
    }
    local_71 = 1;
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}


