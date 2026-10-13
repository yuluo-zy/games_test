
void __cdecl
country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::reed_draw(void)

{
  undefined1 local_310 [248];
  undefined4 local_218 [62];
  undefined4 local_120 [63];
  undefined1 local_21;
  undefined8 local_20;
  
  local_20 = 0xfffffffffffffffe;
  country_core::render::pass_builder::draw_builder::impl$2::
  read_storage_buffer<country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo>()
  ;
  local_218[0] = CONCAT31(local_218[0]._1_3_,9);
  local_120[0] = 2;
  country_core::render::pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
  memcpy(local_120,local_310,0xf8);
  local_21 = 10;
  local_218[0] = 2;
  country_core::render::pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
  memcpy(local_218,local_120,0xf8);
  common_includes::impl$2::
  bind_water_object_push<country_core::render::pass_builder::draw_builder::DrawBuilder>();
  country_core::render::pass_builder::draw_builder::DrawBuilder::multi_draw_indexed_indirect();
  return;
}

