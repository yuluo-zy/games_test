// 0x1409cacb0 draw_floor_shader size=1363

void __cdecl
country_core::systems::render_loop::draw_passes::draw_shaders::floor_shader::draw_floor_shader(void)

{
  void *in_RCX;
  undefined1 local_410 [248];
  undefined8 local_318 [31];
  undefined4 local_220 [62];
  undefined4 local_128 [55];
  char local_4a;
  undefined1 local_29;
  undefined8 local_28;
  
  local_28 = 0xfffffffffffffffe;
  *(byte *)((longlong)in_RCX + 0xa8) = *(byte *)((longlong)in_RCX + 0xa8) & 0xf3 | 8;
  memcpy(local_128,in_RCX,0xf8);
  local_318[0] = CONCAT71(local_318[0]._1_7_,9);
  local_220[0] = 2;
  country_core::render::pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
  memcpy(local_220,local_128,0xf8);
  local_29 = 10;
  local_318[0] = CONCAT44(local_318[0]._4_4_,2);
  country_core::render::pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
  memcpy(local_318,local_220,0xf8);
  local_128[0] = CONCAT31(local_128[0]._1_3_,6);
  local_220[0] = 2;
  country_core::render::pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
  memcpy(local_220,local_318,0xf8);
  local_318[0] = CONCAT71(local_318[0]._1_7_,0x13);
  local_128[0] = 2;
  country_core::render::pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
  memcpy(local_128,local_220,0xf8);
  local_318[0] = 0x4a2b687d0c1ecf58;
  local_220[0] = 2;
  country_core::render::pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
  memcpy(local_220,local_128,0xf8);
  country_core::render::pass_builder::draw_builder::DrawBuilder::bind_stone_floor_textures();
  country_core::render::pass_builder::draw_builder::DrawBuilder::bind_wall_color_customization();
  local_318[0] = 0x84b51135b59095ba;
  local_128[0] = 2;
  country_core::render::pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
  memcpy(local_128,local_220,0xf8);
  local_318[0] = 0xb484366d00600ae5;
  local_220[0] = 2;
  country_core::render::pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
  memcpy(local_220,local_128,0xf8);
  local_318[0] = 0x59b98d76c4e95bd1;
  local_128[0] = 2;
  country_core::render::pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
  memcpy(local_128,local_220,0xf8);
  local_318[0]._0_1_ = 0x15;
  local_220[0] = 2;
  country_core::render::pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
  memcpy(local_220,local_128,0xf8);
  local_318[0]._0_1_ = 7;
  local_128[0] = 2;
  country_core::render::pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
  memcpy(local_128,local_220,0xf8);
  local_318[0]._0_1_ = 0xc;
  local_220[0] = 2;
  country_core::render::pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
  memcpy(local_220,local_128,0xf8);
  local_318[0]._0_1_ = 0xe;
  local_128[0] = 2;
  country_core::render::pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
  memcpy(local_128,local_220,0xf8);
  local_318[0] = CONCAT71(local_318[0]._1_7_,0x14);
  local_220[0] = 2;
  country_core::render::pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
  memcpy(local_220,local_128,0xf8);
  local_318[0] = 0x954b7e4974bbcd6b;
  local_128[0] = 2;
  country_core::render::pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
  memcpy(local_128,local_220,0xf8);
  if (local_4a != '\b') {
    country_core::render::pass_builder::uniform_binder::UniformBinder::
    write_uniform<country_core::render::pass_builder::draw_builder::impl_2::uniform::closure_env_0<bool>,bool>
              ();
  }
  memcpy(local_410,local_128,0xf8);
  country_core::render::pass_builder::draw_builder::DrawBuilder::draw();
  return;
}

