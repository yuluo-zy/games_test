
void __cdecl
system_decorator::ui_systems::handle_splitting_wall_decorators::
handle_moving_out_wall_decorator_part::closure_3(void)

{
  char cVar1;
  undefined1 uVar2;
  uint uVar3;
  undefined8 *puVar4;
  longlong lVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined1 extraout_AL;
  undefined1 extraout_AL_00;
  undefined4 extraout_EAX;
  int extraout_EAX_00;
  ulonglong extraout_RAX;
  longlong extraout_RAX_00;
  undefined8 *in_RCX;
  longlong in_RDX;
  uint *in_R9;
  undefined8 uVar8;
  undefined4 extraout_XMM0_Da;
  undefined4 extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float fVar9;
  float fVar10;
  undefined4 local_b0;
  
  puVar4 = *(undefined8 **)(in_RDX + 8);
  country_core::resources::walls::decorator_storage::DecoratorStorageAddr::try_as_wall();
  if ((extraout_RAX & 1) == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  country_core::resources::walls::public_walls::PublicWalls::get();
  if (extraout_RAX_00 == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  if ((**(byte **)(in_RDX + 0x10) & 1) == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  uVar3 = *in_R9;
  fVar9 = *(float *)(*(byte **)(in_RDX + 0x10) + 8);
  fVar10 = (float)in_R9[2];
  country_core::resources::walls::public_wall_state::PublicWallState::wall_space();
  ::utils::wall_space::impl$1::project();
  if (extraout_EAX_00 == 1) {
    if ((uVar3 & 1) == 0) {
      fVar10 = 0.0;
    }
    fVar9 = fVar9 + fVar10;
    country_core::resources::walls::public_wall_state::PublicWallState::get_wall_attachment_height()
    ;
    _ZN100__LT_country_core__components__drawable__WireframePass_u20_as_u20_bevy_ecs__component__Component_GT_14clone_behavior17h98754d446e2f5bc0E
              ();
    lVar5 = *(longlong *)(in_RDX + 0x18);
    if (*(char *)(lVar5 + 0x22) != '\x02') {
      uVar6 = *(undefined8 *)(lVar5 + 8);
      ::utils::wall_space::WallSpaceCoord::to_world_space();
      cVar1 = *(char *)(lVar5 + 0x22);
      if (cVar1 != '\x02') {
        uVar2 = *(undefined1 *)(lVar5 + 0x21);
        ::utils::wall_space::WallSpaceCoord::to_world_space();
        ::utils::wall_space::WallSpaceCoord::get_tangent();
        ::utils::xz_rotation_bad();
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::
        insert();
        if (uVar3 == 0) {
          uVar8 = 3;
        }
        else {
          uVar8 = *puVar4;
          uVar7 = puVar4[1];
          *(undefined4 *)((longlong)in_RCX + 0x2c) = local_b0;
          *(ulonglong *)((longlong)in_RCX + 0x24) = CONCAT44(extraout_XMM0_Da,extraout_EAX);
          bevy_app::plugin::Plugin::ready<country_core::systems::main_camera::MainCameraPlugin>();
          in_RCX[1] = uVar7;
          *(undefined4 *)(in_RCX + 2) = extraout_XMM0_Da_00;
          *(float *)((longlong)in_RCX + 0x14) = fVar9;
          in_RCX[3] = uVar6;
          *(float *)(in_RCX + 4) = fVar9 - extraout_XMM0_Da_01;
          *(undefined1 *)(in_RCX + 6) = extraout_AL;
          *(undefined1 *)((longlong)in_RCX + 0x31) = uVar2;
          *(char *)((longlong)in_RCX + 0x32) = cVar1;
          *(undefined1 *)(in_RCX + 7) = extraout_AL_00;
        }
        *in_RCX = uVar8;
        return;
      }
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}


