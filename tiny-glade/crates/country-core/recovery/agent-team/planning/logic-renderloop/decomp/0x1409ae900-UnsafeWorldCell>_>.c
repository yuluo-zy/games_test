// 0x1409ae900 call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$6,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_> size=1718

void __cdecl
core::ops::function::FnOnce::
call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_6,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
          (void)

{
  longlong lVar1;
  uint uVar2;
  ulonglong uVar3;
  undefined4 uVar4;
  ulonglong uVar5;
  longlong extraout_RAX;
  undefined8 *extraout_RAX_00;
  longlong extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  longlong extraout_RAX_03;
  undefined8 *extraout_RAX_04;
  void *in_RCX;
  longlong in_RDX;
  undefined8 *puVar6;
  longlong in_R8;
  ulonglong uVar7;
  undefined8 *puVar8;
  int iVar9;
  undefined1 local_450 [264];
  undefined8 local_348 [31];
  undefined4 local_250 [62];
  undefined8 local_158;
  char local_7a;
  undefined8 *local_60;
  ulonglong local_58;
  undefined1 local_4a;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  memcpy(local_450,in_RCX,0xf8);
  local_58 = *(ulonglong *)(in_RDX + 8);
  uVar7 = local_58 & 0xffffffff;
  if (uVar7 < *(ulonglong *)(in_R8 + 0x10)) {
    iVar9 = (int)(local_58 >> 0x20);
    if ((*(int *)(*(longlong *)(in_R8 + 8) + uVar7 * 0x14) == iVar9) &&
       (lVar1 = *(longlong *)(in_R8 + 8) + uVar7 * 0x14, *(int *)(lVar1 + 4) != -1)) {
      uVar2 = *(uint *)(lVar1 + 0xc);
      local_49 = 1;
      bevy_ecs::component::Components::get_id();
      if ((extraout_RAX != 0) && ((ulonglong)uVar2 < *(ulonglong *)(in_R8 + 0x1b0))) {
        local_49 = 1;
        bevy_ecs::storage::table::Table::get_component();
        if (extraout_RAX_00 != (undefined8 *)0x0) {
          if (((*(ulonglong *)(in_R8 + 0x10) <= uVar7) ||
              (*(int *)(*(longlong *)(in_R8 + 8) + uVar7 * 0x14) != iVar9)) ||
             (lVar1 = *(longlong *)(in_R8 + 8) + uVar7 * 0x14, *(int *)(lVar1 + 4) == -1)) {
            local_158 = local_58;
            local_49 = 1;
                    // WARNING: Subroutine does not return
            result::unwrap_failed();
          }
          uVar2 = *(uint *)(lVar1 + 0xc);
          local_49 = 1;
          local_60 = extraout_RAX_00;
          bevy_ecs::component::Components::get_id();
          if ((extraout_RAX_01 == 0) || (*(ulonglong *)(in_R8 + 0x1b0) <= (ulonglong)uVar2)) {
            puVar8 = (undefined8 *)0x0;
            uVar3 = *(ulonglong *)(in_R8 + 0x10);
            uVar5 = local_58;
            uVar4 = local_158._4_4_;
          }
          else {
            local_49 = 1;
            bevy_ecs::storage::table::Table::get_component();
            uVar3 = *(ulonglong *)(in_R8 + 0x10);
            puVar8 = extraout_RAX_02;
            uVar5 = local_58;
            uVar4 = local_158._4_4_;
          }
          local_158 = uVar5;
          local_58 = local_158;
          if (((uVar7 < uVar3) && (*(int *)(*(longlong *)(in_R8 + 8) + uVar7 * 0x14) == iVar9)) &&
             (lVar1 = *(longlong *)(in_R8 + 8) + uVar7 * 0x14, *(int *)(lVar1 + 4) != -1)) {
            uVar2 = *(uint *)(lVar1 + 0xc);
            local_49 = 1;
            local_158._4_4_ = uVar4;
            bevy_ecs::component::Components::get_id();
            if ((extraout_RAX_03 == 0) || (*(ulonglong *)(in_R8 + 0x1b0) <= (ulonglong)uVar2)) {
              puVar6 = (undefined8 *)0x0;
            }
            else {
              local_49 = 1;
              bevy_ecs::storage::table::Table::get_component();
              puVar6 = extraout_RAX_04;
            }
            memcpy(&local_158,local_450,0xf8);
            local_348[0] = *local_60;
            local_250[0] = 2;
            country_core::render::pass_builder::draw_builder::impl$3::
            bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
                      ();
            memcpy(local_250,&local_158,0xf8);
            if (puVar8 == (undefined8 *)0x0) {
              local_348[0] = 0x4b2c611c0a80f895;
            }
            else {
              local_348[0] = *puVar8;
            }
            local_158 = CONCAT44(local_158._4_4_,2);
            country_core::render::pass_builder::draw_builder::impl$3::
            bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
                      ();
            memcpy(&local_158,local_250,0xf8);
            if (local_7a != '\b') {
              country_core::render::pass_builder::uniform_binder::UniformBinder::
              write_uniform<country_core::render::pass_builder::draw_builder::impl_2::uniform::closure_env_0<bool>,bool>
                        ();
            }
            memcpy(local_250,&local_158,0xf8);
            if (puVar6 == (undefined8 *)0x0) {
              local_348[0] = 0xe410c6c6901dbb51;
            }
            else {
              local_348[0] = *puVar6;
            }
            local_158._0_4_ = 2;
            country_core::render::pass_builder::draw_builder::impl$3::
            bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
                      ();
            memcpy(&local_158,local_250,0xf8);
            local_348[0] = 0xe6d1a603f18a02b7;
            local_250[0] = 2;
            country_core::render::pass_builder::draw_builder::impl$3::
            bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
                      ();
            memcpy(local_250,&local_158,0xf8);
            local_348[0]._0_1_ = 0xf;
            local_158._0_4_ = 2;
            country_core::render::pass_builder::draw_builder::impl$3::
            bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
            memcpy(&local_158,local_250,0xf8);
            local_348[0]._0_1_ = 5;
            local_250[0] = 2;
            country_core::render::pass_builder::draw_builder::impl$3::
            bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
            memcpy(local_250,&local_158,0xf8);
            local_348[0]._0_1_ = 0xe;
            local_158._0_4_ = 2;
            country_core::render::pass_builder::draw_builder::impl$3::
            bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
            memcpy(&local_158,local_250,0xf8);
            local_348[0]._0_1_ = 0x13;
            local_250[0] = 2;
            country_core::render::pass_builder::draw_builder::impl$3::
            bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
            memcpy(local_250,&local_158,0xf8);
            local_348[0] = CONCAT71(local_348[0]._1_7_,9);
            local_158 = CONCAT44(local_158._4_4_,2);
            country_core::render::pass_builder::draw_builder::impl$3::
            bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
            memcpy(&local_158,local_250,0xf8);
            local_4a = 10;
            local_348[0] = CONCAT44(local_348[0]._4_4_,2);
            country_core::render::pass_builder::draw_builder::impl$3::
            bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
            memcpy(local_348,&local_158,0xf8);
            local_49 = 0;
            country_core::render::pass_builder::draw_builder::DrawBuilder::draw();
            return;
          }
          local_49 = 1;
                    // WARNING: Subroutine does not return
          result::unwrap_failed();
        }
      }
      local_49 = 1;
                    // WARNING: Subroutine does not return
      option::unwrap_failed();
    }
  }
  local_49 = 1;
  local_158 = local_58;
                    // WARNING: Subroutine does not return
  result::unwrap_failed();
}

