
void __cdecl
country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::lilypad_draw(void)

{
  undefined4 local_318 [62];
  undefined8 local_220 [31];
  undefined4 local_128 [63];
  undefined1 local_29;
  undefined8 local_28;
  
  local_28 = 0xfffffffffffffffe;
  country_core::render::pass_builder::draw_builder::impl$2::
  read_storage_buffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo>
            ();
  local_220[0] = CONCAT71(local_220[0]._1_7_,9);
  local_128[0] = 2;
  country_core::render::pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
  memcpy(local_128,local_318,0xf8);
  local_29 = 10;
  local_220[0] = CONCAT44(local_220[0]._4_4_,2);
  country_core::render::pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
  memcpy(local_220,local_128,0xf8);
  common_includes::impl$2::
  bind_water_object_push<country_core::render::pass_builder::draw_builder::DrawBuilder>();
  local_220[0] = 0x4d64746dbda554d2;
  local_318[0] = 2;
  country_core::render::pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
  memcpy(local_318,local_128,0xf8);
  country_core::render::pass_builder::draw_builder::DrawBuilder::multi_draw_indexed_indirect();
  return;
}

