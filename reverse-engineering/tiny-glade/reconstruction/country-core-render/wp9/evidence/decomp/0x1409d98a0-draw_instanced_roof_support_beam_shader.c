// signature: void __cdecl draw_instanced_roof_support_beam_shader(void)

void __cdecl
country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::
draw_instanced_roof_support_beam_shader(void)

{
  int iVar1;
  longlong extraout_RAX;
  undefined8 *extraout_RAX_00;
  void *in_RCX;
  ulonglong uVar2;
  longlong in_R8;
  undefined4 local_320 [62];
  undefined *local_228;
  undefined8 local_220;
  undefined8 local_130 [31];
  void *local_38;
  undefined1 local_2a;
  undefined1 local_29;
  undefined8 local_28;
  
  local_28 = 0xfffffffffffffffe;
  local_29 = 1;
  uVar2 = 0x8226cc786e679355;
  local_38 = in_RCX;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX != 0) && (uVar2 < *(ulonglong *)(in_R8 + 0x218))) &&
      (uVar2 = *(ulonglong *)(*(longlong *)(in_R8 + 0x210) + uVar2 * 8), uVar2 != 0)) &&
     (*(longlong *)(*(longlong *)(in_R8 + 0x1e0) + 0x20 + ~uVar2 * 0x60) != 0)) {
    local_29 = 1;
    frozen::FrozenMap::
    get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
              ();
    if (extraout_RAX_00 != (undefined8 *)0x0) {
      local_228 = &anon_107183261458dea85466985370debbb6_43_llvm_3641509820080252961;
      local_220 = 0x41;
      if ((extraout_RAX_00[3] != 0x41) ||
         (iVar1 = memcmp(&anon_107183261458dea85466985370debbb6_43_llvm_3641509820080252961,
                         (void *)extraout_RAX_00[2],0x41), iVar1 != 0)) {
        local_130[0] = 0;
        local_29 = 1;
                    // WARNING: Subroutine does not return
        core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
      }
      local_29 = 1;
      (**(code **)(extraout_RAX_00[1] + 0x18))(*extraout_RAX_00);
    }
    memcpy(&local_228,local_38,0xf8);
    local_320[0] = CONCAT31(local_320[0]._1_3_,9);
    local_130[0] = CONCAT44(local_130[0]._4_4_,2);
    country_core::render::pass_builder::draw_builder::impl$3::
    bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
    memcpy(local_130,&local_228,0xf8);
    local_2a = 10;
    local_320[0] = 2;
    country_core::render::pass_builder::draw_builder::impl$3::
    bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
    memcpy(local_320,local_130,0xf8);
    local_29 = 0;
    country_core::render::pass_builder::draw_builder::impl$2::
    read_storage_buffer<country_core::resources::roofs::roof_buffers::RoofSupportBeamSsbo>();
    local_29 = 0;
    country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced_w_gl_draw_flag();
    return;
  }
  local_29 = 1;
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

