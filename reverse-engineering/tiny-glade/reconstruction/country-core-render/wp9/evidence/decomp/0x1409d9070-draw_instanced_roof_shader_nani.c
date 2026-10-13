// signature: void __cdecl draw_instanced_roof_shader_nani(void)

void __cdecl
country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::
draw_instanced_roof_shader_nani(void)

{
  longlong extraout_RAX;
  void *in_RCX;
  ulonglong uVar1;
  longlong in_R8;
  undefined8 local_328 [31];
  undefined8 local_230 [27];
  char local_152;
  undefined4 local_138 [55];
  char local_5a;
  void *local_40;
  undefined1 local_32;
  undefined1 local_31;
  undefined8 local_30;
  
  local_30 = 0xfffffffffffffffe;
  local_31 = 1;
  uVar1 = 0x9311cf2688353878;
  local_40 = in_RCX;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX != 0) && (uVar1 < *(ulonglong *)(in_R8 + 0x218))) &&
      (uVar1 = *(ulonglong *)(*(longlong *)(in_R8 + 0x210) + uVar1 * 8), uVar1 != 0)) &&
     (*(longlong *)(*(longlong *)(in_R8 + 0x1e0) + 0x20 + ~uVar1 * 0x60) != 0)) {
    *(byte *)((longlong)local_40 + 0xa8) = *(byte *)((longlong)local_40 + 0xa8) & 0xf3 | 4;
    memcpy(local_230,local_40,0xf8);
    local_328[0] = CONCAT71(local_328[0]._1_7_,9);
    local_138[0] = 2;
    country_core::render::pass_builder::draw_builder::impl$3::
    bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
    memcpy(local_138,local_230,0xf8);
    local_32 = 10;
    local_328[0] = CONCAT44(local_328[0]._4_4_,2);
    country_core::render::pass_builder::draw_builder::impl$3::
    bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
    memcpy(local_328,local_138,0xf8);
    local_31 = 0;
    country_core::render::pass_builder::draw_builder::impl$2::
    read_storage_buffer<country_core::resources::walls::dormers::DormerHoleSsbo>();
    local_31 = 0;
    country_core::render::pass_builder::draw_builder::impl$2::
    read_storage_buffer<country_core::resources::walls::dormers::DormerHoleCountPerCellSsbo>();
    local_328[0] = 0x7db9e7483fa24078;
    local_138[0] = 2;
    country_core::render::pass_builder::draw_builder::impl$3::
    bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
    memcpy(local_138,local_230,0xf8);
    local_31 = 0;
    country_core::render::pass_builder::draw_builder::impl$2::
    read_storage_buffer<country_core::resources::color_id_roof::RoofColorIdSsbo>();
    local_230[0] = 0xe09955c0b15b3e04;
    local_138[0] = 2;
    country_core::render::pass_builder::draw_builder::impl$3::
    bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
    memcpy(local_138,local_328,0xf8);
    local_328[0] = 0xb35c3f60a2b76288;
    local_230[0] = CONCAT44(local_230[0]._4_4_,2);
    country_core::render::pass_builder::draw_builder::impl$3::
    bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
    memcpy(local_230,local_138,0xf8);
    local_328[0] = 0xa169bb74b48855e1;
    local_138[0] = 2;
    country_core::render::pass_builder::draw_builder::impl$3::
    bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
    memcpy(local_138,local_230,0xf8);
    local_31 = 0;
    country_core::render::pass_builder::draw_builder::impl$2::
    read_storage_buffer<country_core::resources::roof_tile_ssbo::RoofTileSsbo>();
    if (local_152 != '\b') {
      country_core::render::pass_builder::uniform_binder::UniformBinder::
      write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<i32>,i32>
                ();
    }
    memcpy(local_138,local_230,0xf8);
    if (local_5a != '\b') {
      country_core::render::pass_builder::uniform_binder::UniformBinder::
      write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<i32>,i32>
                ();
    }
    memcpy(local_230,local_138,0xf8);
    local_31 = 0;
    country_core::render::nani::draw_ext::impl$0::nani_draw();
    return;
  }
  local_31 = 1;
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

