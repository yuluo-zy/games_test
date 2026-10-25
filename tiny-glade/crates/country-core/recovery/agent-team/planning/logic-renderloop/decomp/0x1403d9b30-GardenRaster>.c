// 0x1403d9b30 upload_raster_staging_to_texture<country_core::resources::world_raster_defs::GardenRaster> size=904

void __cdecl
country_core::systems::render_loop::compute_passes::upload_world_raster::
upload_raster_staging_to_texture<country_core::resources::world_raster_defs::GardenRaster>(void)

{
  undefined4 uVar1;
  longlong lVar2;
  longlong extraout_RAX;
  longlong lVar3;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong in_RDX;
  ulonglong uVar4;
  undefined1 in_R8B;
  int *local_3c0;
  code *local_3b8;
  undefined **local_230;
  undefined8 local_228;
  undefined8 **local_220;
  undefined8 local_218;
  undefined8 local_210;
  char local_fa;
  longlong local_a0;
  undefined8 local_98;
  char *local_88;
  undefined8 local_80;
  undefined8 local_78;
  undefined8 *local_70;
  code *local_68;
  int local_5c;
  longlong local_58;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  uVar4 = 0x979ffa6288aa259a;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX != 0) && (uVar4 < *(ulonglong *)(in_RDX + 0x218))) &&
     (uVar4 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar4 * 8), uVar4 != 0)) {
    lVar2 = *(longlong *)(in_RDX + 0x1e0);
    lVar3 = ~uVar4 * 0x60;
    if (*(longlong *)(lVar2 + 0x20 + lVar3) != 0) {
      uVar1 = *(undefined4 *)(in_RDX + 0x5b0);
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      if (*(char *)(extraout_RAX_00 + 0x29a) != '\0') {
        country_core::resources::texture_presets::impl$1::from();
        local_5c = (int)local_228;
        if ((int)local_228 == 0x4c) {
          local_80 = 4;
          local_88 = "r16f/upload_cpu_raster.cs.hlsl:";
        }
        else {
          if ((int)local_228 != 0x61) {
            local_3c0 = &local_5c;
            local_3b8 = ash::vk::const_debugs::impl$101::fmt;
            local_230 = &PTR_s_not_yet_implemented__1429d3698;
            local_228 = 1;
            local_210 = 0;
            local_220 = (undefined8 **)&local_3c0;
            local_218 = 1;
                    // WARNING: Subroutine does not return
            core::panicking::panic_fmt();
          }
          local_80 = 7;
          local_88 = "rgba16fr16f/upload_cpu_raster.cs.hlsl:";
        }
        *(undefined4 *)(lVar2 + lVar3 + 0x5c) = uVar1;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        *(undefined2 *)(extraout_RAX_01 + 0x29a) = 0x100;
        local_68 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
        local_230 = (undefined **)&DAT_1429d3640;
        local_228 = 1;
        local_218 = 1;
        local_210 = 0;
        local_220 = &local_70;
        local_70 = &local_88;
        alloc::fmt::format::format_inner();
        country_core::render::pass_builder::compute_pass_builder::ComputePass::
        new<alloc::string::String>();
        country_core::render::pass_builder::compute_pass_builder::impl$3::
        read_storage_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo>();
        local_68 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
        local_230 = (undefined **)&DAT_1429d3670;
        local_228 = 1;
        local_218 = 1;
        local_210 = 0;
        local_220 = &local_70;
        local_70 = &local_88;
        alloc::fmt::format::format_inner();
        local_58 = local_a0;
        local_49 = 1;
        local_78 = local_98;
        shader_pipeline::reflection::BindingNameHash::new();
        local_70 = (undefined8 *)CONCAT71(local_70._1_7_,in_R8B);
        local_230 = (undefined **)CONCAT44(local_230._4_4_,2);
        country_core::render::pass_builder::compute_pass_builder::impl$4::
        bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>();
        memcpy(&local_230,&local_3c0,400);
        if (local_fa != '\b') {
          country_core::render::pass_builder::uniform_binder::UniformBinder::
          write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<u32>,u32>
                    ();
        }
        memcpy(&local_3c0,&local_230,400);
        country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d();
        if (local_58 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
      }
      return;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

