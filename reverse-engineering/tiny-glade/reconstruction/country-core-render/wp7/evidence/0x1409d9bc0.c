
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl
country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::water_planes_draw
          (void)

{
  char cVar1;
  longlong lVar2;
  longlong lVar3;
  uint uVar4;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  void *in_RCX;
  longlong lVar5;
  longlong lVar6;
  ulonglong uVar7;
  longlong in_R8;
  undefined4 local_348 [62];
  undefined8 local_250 [27];
  char local_172;
  ulonglong local_158;
  undefined4 local_150;
  void *local_60;
  undefined8 local_58;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  if (_DEV_OVERRIDE_IS_DEMO_MODE != '\0') {
    memcpy(&local_158,in_RCX,0xf8);
    local_49 = 0;
    country_core::render::pass_builder::draw_builder::DrawBuilder::discard();
                    // WARNING: Read-only address (ram,0x000142ac1950) is written
                    // WARNING: Read-only address (ram,0x000142ac1960) is written
    return;
  }
  local_49 = 1;
  uVar7 = 0x74430dfcc6d8719b;
  local_60 = in_RCX;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX != 0) && (uVar7 < *(ulonglong *)(in_R8 + 0x218))) &&
     (uVar7 = *(ulonglong *)(*(longlong *)(in_R8 + 0x210) + uVar7 * 8), uVar7 != 0)) {
    lVar5 = ~uVar7 * 0x60;
    if (*(longlong *)(*(longlong *)(in_R8 + 0x1e0) + 0x20 + lVar5) != 0) {
      cVar1 = *(char *)(*(longlong *)(*(longlong *)(in_R8 + 0x1e0) + lVar5 + 0x10) + 0x58);
      local_49 = 1;
      uVar7 = 0x74430dfcc6d8719b;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_00 != 0) && (uVar7 < *(ulonglong *)(in_R8 + 0x218))) &&
         (uVar7 = *(ulonglong *)(*(longlong *)(in_R8 + 0x210) + uVar7 * 8), uVar7 != 0)) {
        lVar5 = ~uVar7 * 0x60;
        if (*(longlong *)(*(longlong *)(in_R8 + 0x1e0) + 0x20 + lVar5) != 0) {
          lVar5 = *(longlong *)(*(longlong *)(in_R8 + 0x1e0) + lVar5 + 0x10);
          lVar2 = *(longlong *)(lVar5 + 0x10);
          lVar3 = 0;
          do {
            lVar6 = lVar3;
            if (lVar2 == lVar6) break;
            lVar3 = lVar6 + 1;
          } while (*(char *)(*(longlong *)(lVar5 + 8) + lVar6) != '\0');
          *(byte *)((longlong)local_60 + 0xab) = *(byte *)((longlong)local_60 + 0xab) & 0x9f;
          memcpy(local_348,local_60,0xf8);
          local_250[0] = CONCAT71(local_250[0]._1_7_,9);
          local_158._0_4_ = 2;
          country_core::render::pass_builder::draw_builder::impl$3::
          bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
          memcpy(&local_158,local_348,0xf8);
          local_58._0_1_ = 10;
          local_250[0] = CONCAT44(local_250[0]._4_4_,2);
          country_core::render::pass_builder::draw_builder::impl$3::
          bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
          memcpy(local_250,&local_158,0xf8);
          local_348[0] = CONCAT31(local_348[0]._1_3_,0x13);
          local_158._0_4_ = 2;
          country_core::render::pass_builder::draw_builder::impl$3::
          bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
          memcpy(&local_158,local_250,0xf8);
          local_49 = 0;
          country_core::render::pass_builder::draw_builder::impl$2::
          read_storage_buffer<country_core::resources::water::RippleSourceSsbo>();
          local_250[0] = CONCAT71(local_250[0]._1_7_,0x18);
          local_158._0_4_ = 2;
          country_core::render::pass_builder::draw_builder::impl$3::
          bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
          memcpy(&local_158,local_348,0xf8);
          local_250[0] = 0x579f39ab91a01865;
          local_348[0] = 2;
          country_core::render::pass_builder::draw_builder::impl$3::
          bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
          memcpy(local_348,&local_158,0xf8);
          local_49 = 0;
          country_core::render::pass_builder::draw_builder::impl$2::
          read_storage_buffer<country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo>
                    ();
          local_348[0] = CONCAT31(local_348[0]._1_3_,9);
          local_158._0_4_ = 2;
          country_core::render::pass_builder::draw_builder::impl$3::
          bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
          memcpy(&local_158,local_250,0xf8);
          local_58 = CONCAT71(local_58._1_7_,10);
          local_348[0] = 2;
          country_core::render::pass_builder::draw_builder::impl$3::
          bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
          memcpy(local_348,&local_158,0xf8);
          local_250[0] = CONCAT71(local_250[0]._1_7_,8);
          local_158 = CONCAT44(local_158._4_4_,2);
          country_core::render::pass_builder::draw_builder::impl$3::
          bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
          memcpy(&local_158,local_348,0xf8);
          local_58 = 0x579f39ab91a01865;
          local_250[0] = CONCAT44(local_250[0]._4_4_,2);
          country_core::render::pass_builder::draw_builder::impl$3::
          bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
          memcpy(local_250,&local_158,0xf8);
          if (local_172 != '\b') {
            uVar4 = (uint)(lVar2 == lVar6);
            uVar7 = CONCAT44((int)(uVar4 << 0x1f) >> 0x1f,(int)(uVar4 << 0x1f) >> 0x1f);
            local_150 = *(undefined4 *)(&__real_3e9cac093eb0a3d7 + (ulonglong)(lVar2 == lVar6) * 4);
            local_158 = uVar7 & (ulonglong)__xmm_00000000000000003ebf7cee3f1851ec |
                        ~uVar7 & (ulonglong)__xmm_00000000000000003ec000003f066666;
            country_core::render::pass_builder::uniform_binder::UniformBinder::
            write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<glam::f32::vec3::Vec3>,glam::f32::vec3::Vec3>
                      ();
          }
          memcpy(&local_158,local_250,0xf8);
          local_49 = 0;
          country_core::render::pass_builder::bind_resources::impl$0::
          bind_ray_tracing<country_core::render::pass_builder::draw_builder::DrawBuilder>();
          if (cVar1 == '\x04') {
            memcpy(&local_158,local_348,0xf8);
            local_58 = 0xc555853f6bd247ac;
            local_250[0] = CONCAT44(local_250[0]._4_4_,2);
            country_core::render::pass_builder::draw_builder::impl$3::
            bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
                      ();
            memcpy(local_348,&local_158,0xf8);
          }
          memcpy(&local_158,local_348,0xf8);
          local_49 = 0;
          country_core::render::pass_builder::draw_builder::DrawBuilder::draw();
          return;
        }
      }
      local_49 = 1;
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
  }
  local_49 = 1;
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

