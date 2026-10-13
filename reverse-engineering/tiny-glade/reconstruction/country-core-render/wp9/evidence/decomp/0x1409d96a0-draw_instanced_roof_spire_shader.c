// signature: void __cdecl draw_instanced_roof_spire_shader(void)

void __cdecl
country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::
draw_instanced_roof_spire_shader(void)

{
  int iVar1;
  longlong extraout_RAX;
  undefined8 *extraout_RAX_00;
  void *in_RCX;
  ulonglong uVar2;
  longlong in_R8;
  undefined8 local_220 [31];
  undefined *local_128;
  undefined8 local_120;
  void *local_30;
  undefined1 local_21;
  undefined8 local_20;
  
  local_20 = 0xfffffffffffffffe;
  local_21 = 1;
  uVar2 = 0x8226cc786e679355;
  local_30 = in_RCX;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX != 0) && (uVar2 < *(ulonglong *)(in_R8 + 0x218))) &&
      (uVar2 = *(ulonglong *)(*(longlong *)(in_R8 + 0x210) + uVar2 * 8), uVar2 != 0)) &&
     (*(longlong *)(*(longlong *)(in_R8 + 0x1e0) + 0x20 + ~uVar2 * 0x60) != 0)) {
    local_21 = 1;
    frozen::FrozenMap::
    get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
              ();
    if (extraout_RAX_00 != (undefined8 *)0x0) {
      local_128 = &anon_107183261458dea85466985370debbb6_68_llvm_3641509820080252961;
      local_120 = 0x36;
      if ((extraout_RAX_00[3] != 0x36) ||
         (iVar1 = memcmp(&anon_107183261458dea85466985370debbb6_68_llvm_3641509820080252961,
                         (void *)extraout_RAX_00[2],0x36), iVar1 != 0)) {
        local_220[0] = 0;
        local_21 = 1;
                    // WARNING: Subroutine does not return
        core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
      }
      local_21 = 1;
      (**(code **)(extraout_RAX_00[1] + 0x18))(*extraout_RAX_00);
    }
    memcpy(local_220,local_30,0xf8);
    local_21 = 0;
    country_core::render::pass_builder::draw_builder::impl$2::
    read_storage_buffer<country_core::resources::roof_tile_ssbo::RoofSpireSsbo>();
    local_21 = 0;
    country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced_w_gl_draw_flag();
    return;
  }
  local_21 = 1;
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

