// 0x140b4af70 draw_grass_shader size=412

void __cdecl
country_core::systems::render_loop::draw_passes::draw_shaders::grass_shader::draw_grass_shader(void)

{
  void *in_RCX;
  undefined4 local_220 [62];
  undefined4 local_128 [55];
  char local_4a;
  void *local_30;
  undefined1 local_21;
  undefined8 local_20;
  
  local_20 = 0xfffffffffffffffe;
  local_220[0] = CONCAT31(local_220[0]._1_3_,9);
  local_128[0] = 2;
  local_30 = in_RCX;
  country_core::render::pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
  memcpy(local_128,local_30,0xf8);
  local_21 = 10;
  local_220[0] = 2;
  country_core::render::pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
  memcpy(local_220,local_128,0xf8);
  local_21 = 8;
  local_128[0] = 2;
  country_core::render::pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
  memcpy(local_128,local_220,0xf8);
  country_core::render::pass_builder::draw_builder::impl$2::
  read_storage_buffer<country_core::systems::render_loop::compute_passes::grass::GrassBladeDataSsbo>
            ();
  country_core::render::pass_builder::draw_builder::impl$2::
  read_storage_buffer<country_core::systems::render_loop::indirect_params::AtomicCounterBuffer>();
  if (local_4a != '\b') {
    country_core::render::pass_builder::uniform_binder::UniformBinder::
    write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<i32>,i32>
              ();
  }
  memcpy(local_220,local_128,0xf8);
  country_core::render::pass_builder::draw_builder::DrawBuilder::multi_draw_indexed_indirect();
  return;
}

