// 0x1409b8680 call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$21,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_> size=1497

void __cdecl
core::ops::function::FnOnce::
call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_21,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
          (void)

{
  byte *pbVar1;
  longlong lVar2;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  void *in_RCX;
  ulonglong uVar3;
  longlong lVar4;
  longlong in_RDX;
  ulonglong uVar5;
  longlong in_R8;
  undefined1 local_348 [264];
  undefined4 local_240 [55];
  char local_162;
  ulonglong local_148 [27];
  char local_6a;
  undefined8 local_50;
  undefined1 local_42;
  undefined1 local_41;
  undefined8 local_40;
  
  local_40 = 0xfffffffffffffffe;
  memcpy(local_348,in_RCX,0xf8);
  uVar5 = *(ulonglong *)(in_RDX + 8);
  uVar3 = uVar5 & 0xffffffff;
  if (((*(ulonglong *)(in_R8 + 0x10) <= uVar3) ||
      (*(int *)(*(longlong *)(in_R8 + 8) + uVar3 * 0x14) != (int)(uVar5 >> 0x20))) ||
     (lVar4 = *(longlong *)(in_R8 + 8) + uVar3 * 0x14, *(int *)(lVar4 + 4) == -1)) {
    local_41 = 1;
    local_148[0] = uVar5;
                    // WARNING: Subroutine does not return
    result::unwrap_failed();
  }
  uVar5 = (ulonglong)*(uint *)(lVar4 + 0xc);
  local_41 = 1;
  bevy_ecs::component::Components::get_id();
  if ((extraout_RAX != 0) && (uVar5 < *(ulonglong *)(in_R8 + 0x1b0))) {
    local_41 = 1;
    bevy_ecs::storage::table::Table::get_component();
    if (extraout_RAX_00 != 0) {
      local_41 = 1;
      bevy_ecs::component::Components::get_id();
      if ((extraout_RAX_01 != 0) && (uVar5 < *(ulonglong *)(in_R8 + 0x1b0))) {
        local_41 = 1;
        bevy_ecs::storage::table::Table::get_component();
        if (extraout_RAX_02 != 0) {
          memcpy(local_148,local_348,0xf8);
          if (local_6a != '\b') {
            country_core::render::pass_builder::uniform_binder::UniformBinder::
            write_uniform<country_core::render::pass_builder::draw_builder::impl_2::uniform::closure_env_0<bool>,bool>
                      ();
          }
          memcpy(local_240,local_148,0xf8);
          if (local_162 != '\b') {
            country_core::render::pass_builder::uniform_binder::UniformBinder::
            write_uniform<country_core::render::pass_builder::draw_builder::impl_2::uniform::closure_env_0<bool>,bool>
                      ();
          }
          memcpy(local_148,local_240,0xf8);
          local_50 = 0x61c03c1f48298c31;
          local_240[0] = 2;
          country_core::render::pass_builder::draw_builder::impl$3::
          bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
          memcpy(local_240,local_148,0xf8);
          local_50 = 0x1c9b640db8cc8190;
          local_148[0]._0_4_ = 2;
          country_core::render::pass_builder::draw_builder::impl$3::
          bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
          memcpy(local_148,local_240,0xf8);
          local_50 = 0x93b4b6c9587d6b45;
          local_240[0] = 2;
          country_core::render::pass_builder::draw_builder::impl$3::
          bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
          memcpy(local_240,local_148,0xf8);
          local_50 = 0x92f0e1606bfe7473;
          local_148[0] = CONCAT44(local_148[0]._4_4_,2);
          country_core::render::pass_builder::draw_builder::impl$3::
          bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
          memcpy(local_148,local_240,0xf8);
          if (local_6a != '\b') {
            country_core::render::pass_builder::uniform_binder::UniformBinder::
            write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<i32>,i32>
                      ();
          }
          memcpy(local_240,local_148,0xf8);
          if (local_162 != '\b') {
            country_core::render::pass_builder::uniform_binder::UniformBinder::
            write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<i32>,i32>
                      ();
          }
          memcpy(local_148,local_240,0xf8);
          local_50 = 0xb72fba719985a58c;
          local_240[0] = 2;
          country_core::render::pass_builder::draw_builder::impl$3::
          bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
          memcpy(local_240,local_148,0xf8);
          local_42 = 1;
          uVar5 = 0x74430dfcc6d8719b;
          bevy_ecs::component::Components::get_resource_id();
          if (((extraout_RAX_03 != 0) && (uVar5 < *(ulonglong *)(in_R8 + 0x218))) &&
             (uVar5 = *(ulonglong *)(*(longlong *)(in_R8 + 0x210) + uVar5 * 8), uVar5 != 0)) {
            lVar4 = ~uVar5 * 0x60;
            if (*(longlong *)(*(longlong *)(in_R8 + 0x1e0) + 0x20 + lVar4) != 0) {
              lVar2 = *(longlong *)(*(longlong *)(in_R8 + 0x1e0) + lVar4 + 0x10);
              lVar4 = 0;
              local_50 = 0xe52375bc32a964a4;
              do {
                if (*(longlong *)(lVar2 + 0x10) == lVar4) goto LAB_1409b8b73;
                pbVar1 = (byte *)(*(longlong *)(lVar2 + 8) + lVar4);
                lVar4 = lVar4 + 1;
              } while (1 < *pbVar1);
              local_50 = 0x4c58d4d61ec450d2;
LAB_1409b8b73:
              local_148[0] = CONCAT44(local_148[0]._4_4_,2);
              country_core::render::pass_builder::draw_builder::impl$3::
              bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
                        ();
              memcpy(local_148,local_240,0xf8);
              local_41 = 0;
              country_core::render::pass_builder::draw_builder::DrawBuilder::draw();
              return;
            }
          }
          local_42 = 1;
                    // WARNING: Subroutine does not return
          option::unwrap_failed();
        }
      }
      local_41 = 1;
                    // WARNING: Subroutine does not return
      option::unwrap_failed();
    }
  }
  local_41 = 1;
                    // WARNING: Subroutine does not return
  option::unwrap_failed();
}

