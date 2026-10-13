// address: 0x140909310
// pdb name: new_from
// signature: void __cdecl new_from(void)

void __cdecl country_core::render::pass_builder::pass_resources::PassResources::new_from(void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  undefined8 *in_RCX;
  longlong lVar7;
  longlong in_RDX;
  ulonglong uVar8;
  undefined8 uVar9;
  
  uVar8 = 0x8597ee0727b461b3;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar8)) ||
     (uVar8 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar8 * 8), uVar8 == 0)) {
LAB_140909644:
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  lVar7 = ~uVar8 * 0x60;
  if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar7) == 0) goto LAB_140909644;
  uVar1 = *(undefined8 *)(*(longlong *)(in_RDX + 0x1e0) + lVar7 + 0x10);
  uVar8 = 0xf0b963d163fe760f;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_00 != 0) && (uVar8 < *(ulonglong *)(in_RDX + 0x218))) &&
     (uVar8 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar8 * 8), uVar8 != 0)) {
    lVar7 = ~uVar8 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar7) != 0) {
      uVar9 = *(undefined8 *)(*(longlong *)(in_RDX + 0x1e0) + lVar7 + 0x10);
      goto LAB_1409093f5;
    }
  }
  uVar9 = 0;
LAB_1409093f5:
  uVar8 = 0xa68561b396784eaa;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_01 != 0) && (uVar8 < *(ulonglong *)(in_RDX + 0x218))) &&
     (uVar8 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar8 * 8), uVar8 != 0)) {
    lVar7 = ~uVar8 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar7) != 0) {
      uVar2 = *(undefined8 *)(*(longlong *)(in_RDX + 0x1e0) + lVar7 + 0x10);
      uVar8 = 0x8226cc786e679355;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_02 != 0) && (uVar8 < *(ulonglong *)(in_RDX + 0x218))) &&
         (uVar8 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar8 * 8), uVar8 != 0)) {
        lVar7 = ~uVar8 * 0x60;
        if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar7) != 0) {
          uVar3 = *(undefined8 *)(*(longlong *)(in_RDX + 0x1e0) + lVar7 + 0x10);
          uVar8 = 0x874b1541877dacfd;
          bevy_ecs::component::Components::get_resource_id();
          if (((extraout_RAX_03 != 0) && (uVar8 < *(ulonglong *)(in_RDX + 0x218))) &&
             (uVar8 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar8 * 8), uVar8 != 0)) {
            lVar7 = ~uVar8 * 0x60;
            if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar7) != 0) {
              uVar4 = *(undefined8 *)(*(longlong *)(in_RDX + 0x1e0) + lVar7 + 0x10);
              uVar8 = 0x47c94cb78283a72e;
              bevy_ecs::component::Components::get_resource_id();
              if (((extraout_RAX_04 != 0) && (uVar8 < *(ulonglong *)(in_RDX + 0x218))) &&
                 (uVar8 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar8 * 8), uVar8 != 0)) {
                lVar7 = ~uVar8 * 0x60;
                if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar7) != 0) {
                  uVar5 = *(undefined8 *)(*(longlong *)(in_RDX + 0x1e0) + lVar7 + 0x10);
                  uVar8 = 0xb65bf3141f93e938;
                  bevy_ecs::component::Components::get_resource_id();
                  if (((extraout_RAX_05 != 0) && (uVar8 < *(ulonglong *)(in_RDX + 0x218))) &&
                     (uVar8 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar8 * 8), uVar8 != 0)
                     ) {
                    lVar7 = ~uVar8 * 0x60;
                    if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar7) != 0) {
                      uVar6 = *(undefined8 *)(*(longlong *)(in_RDX + 0x1e0) + lVar7 + 0x10);
                      in_RCX[6] = uVar9;
                      *in_RCX = uVar1;
                      in_RCX[1] = uVar2;
                      in_RCX[2] = uVar3;
                      in_RCX[3] = uVar4;
                      in_RCX[4] = uVar6;
                      in_RCX[5] = uVar5;
                      return;
                    }
                  }
                    // WARNING: Subroutine does not return
                  core::option::unwrap_failed();
                }
              }
                    // WARNING: Subroutine does not return
              core::option::unwrap_failed();
            }
          }
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
      }
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}


