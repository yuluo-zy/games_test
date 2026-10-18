// 0x1409cb4e0 rocky_terrain_shader size=501

void __cdecl
country_core::systems::render_loop::draw_passes::draw_shaders::rocky_terrain_draw::
rocky_terrain_shader(void)

{
  longlong extraout_RAX;
  void *in_RCX;
  ulonglong uVar1;
  longlong in_R8;
  undefined4 local_230 [62];
  undefined4 local_138 [55];
  char local_5a;
  undefined8 local_40;
  undefined1 local_31;
  undefined8 local_30;
  
  local_30 = 0xfffffffffffffffe;
  *(byte *)((longlong)in_RCX + 0xa8) = *(byte *)((longlong)in_RCX + 0xa8) & 0xf3 | 4;
  memcpy(local_138,in_RCX,0xf8);
  country_core::render::pass_builder::draw_builder::impl$2::
  read_storage_buffer<country_core::resources::rocky_terrain::RockyTerrainVertexSSBO>();
  country_core::render::pass_builder::draw_builder::impl$2::
  read_storage_buffer<country_core::resources::rocky_terrain::RockyTerrainTriangleSSBO>();
  local_40 = 0x84a037e697de0324;
  local_230[0] = 2;
  country_core::render::pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
  memcpy(local_230,local_138,0xf8);
  local_40 = CONCAT71(local_40._1_7_,0xc);
  local_138[0] = 2;
  country_core::render::pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
  memcpy(local_138,local_230,0xf8);
  if (local_5a != '\b') {
    country_core::render::pass_builder::uniform_binder::UniformBinder::
    write_uniform<country_core::render::pass_builder::draw_builder::impl_2::uniform::closure_env_0<bool>,bool>
              ();
  }
  memcpy(local_230,local_138,0xf8);
  local_31 = 1;
  uVar1 = 0xfa44538b95ae76c6;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX != 0) && (uVar1 < *(ulonglong *)(in_R8 + 0x218))) &&
      (uVar1 = *(ulonglong *)(*(longlong *)(in_R8 + 0x210) + uVar1 * 8), uVar1 != 0)) &&
     (*(longlong *)(*(longlong *)(in_R8 + 0x1e0) + 0x20 + ~uVar1 * 0x60) != 0)) {
    local_31 = 0;
    country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_index_buffer();
    return;
  }
  local_31 = 1;
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

