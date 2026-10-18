// 0x1408dbc50 draw_instanced_ivy_branch_shader size=855

void __cdecl
country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::
draw_instanced_ivy_branch_shader(void)

{
  undefined1 auVar1 [16];
  undefined8 *puVar2;
  uint uVar3;
  longlong extraout_RAX;
  void *in_RCX;
  longlong lVar4;
  ulonglong uVar5;
  undefined1 (*pauVar6) [16];
  uint uVar7;
  longlong in_R8;
  undefined4 local_320 [62];
  undefined8 local_228 [31];
  undefined4 local_130 [62];
  void *local_38;
  undefined1 local_2a;
  undefined1 local_29;
  undefined8 local_28;
  
  local_28 = 0xfffffffffffffffe;
  local_29 = 1;
  uVar5 = 0xdf356893cc4e98a8;
  local_38 = in_RCX;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX != 0) && (uVar5 < *(ulonglong *)(in_R8 + 0x218))) &&
     (uVar5 = *(ulonglong *)(*(longlong *)(in_R8 + 0x210) + uVar5 * 8), uVar5 != 0)) {
    lVar4 = ~uVar5 * 0x60;
    if (*(longlong *)(*(longlong *)(in_R8 + 0x1e0) + 0x20 + lVar4) != 0) {
      puVar2 = *(undefined8 **)(*(longlong *)(in_R8 + 0x1e0) + lVar4 + 0x10);
      pauVar6 = (undefined1 (*) [16])*puVar2;
      lVar4 = puVar2[3];
      auVar1 = *pauVar6;
      uVar7 = ~(uint)(ushort)((ushort)(SUB161(auVar1 >> 7,0) & 1) |
                              (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                              (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                              (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                              (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                              (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                              (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                              (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                              (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                              (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                              (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                              (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                              (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                              (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                              (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                             (ushort)(byte)(auVar1[0xf] >> 7) << 0xf);
      pauVar6 = pauVar6 + 1;
      do {
        if ((short)uVar7 == 0) {
          if (lVar4 == 0) {
            memcpy(local_130,local_38,0xf8);
            local_29 = 0;
            country_core::render::pass_builder::draw_builder::impl$2::
            read_storage_buffer<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo>
                      ();
            local_228[0] = CONCAT71(local_228[0]._1_7_,9);
            local_130[0] = 2;
            country_core::render::pass_builder::draw_builder::impl$3::
            bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
            memcpy(local_130,local_320,0xf8);
            local_2a = 10;
            local_228[0] = CONCAT44(local_228[0]._4_4_,2);
            country_core::render::pass_builder::draw_builder::impl$3::
            bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
            memcpy(local_228,local_130,0xf8);
            local_320[0] = CONCAT31(local_320[0]._1_3_,8);
            local_130[0] = 2;
            country_core::render::pass_builder::draw_builder::impl$3::
            bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
            memcpy(local_130,local_228,0xf8);
            local_228[0] = 0x579f39ab91a01865;
            local_320[0] = 2;
            country_core::render::pass_builder::draw_builder::impl$3::
            bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
                      ();
            memcpy(local_320,local_130,0xf8);
            local_228[0] = CONCAT71(local_228[0]._1_7_,5);
            local_130[0] = 2;
            country_core::render::pass_builder::draw_builder::impl$3::
            bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
            memcpy(local_130,local_320,0xf8);
            local_228[0] = 0xedf425b62b653c03;
            local_320[0] = 2;
            country_core::render::pass_builder::draw_builder::impl$3::
            bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
                      ();
            memcpy(local_320,local_130,0xf8);
            local_29 = 0;
            country_core::render::pass_builder::draw_builder::DrawBuilder::
            draw_instanced_w_gl_draw_flag();
            return;
          }
          do {
            auVar1 = *pauVar6;
            pauVar6 = pauVar6 + 1;
            uVar7 = (uint)(ushort)((ushort)(SUB161(auVar1 >> 7,0) & 1) |
                                   (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                                   (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                                   (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                                   (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                                   (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                                   (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                                   (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                                   (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                                   (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                                   (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                                   (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                                   (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                                   (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                                   (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                                  (ushort)(byte)(auVar1[0xf] >> 7) << 0xf);
          } while (uVar7 == 0xffff);
          uVar7 = ~uVar7;
        }
        for (uVar3 = uVar7; (uVar3 & 1) == 0; uVar3 = uVar3 >> 1 | 0x80000000) {
        }
        uVar7 = uVar7 - 1 & uVar7;
        lVar4 = lVar4 + -1;
      } while( true );
    }
  }
  local_29 = 1;
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

