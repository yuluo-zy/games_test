// remove<system_critters::wren_boids::sim_wren_boids::closure_env$6> 0x141dc2540 sig=void __cdecl remove<system_critters::wren_boids::sim_wren_boids::closure_env$6>(void)

void __cdecl
system_critters::boids::BoidFlock::
remove<system_critters::wren_boids::sim_wren_boids::closure_env_6>(void)

{
  undefined8 *puVar1;
  longlong lVar2;
  ulonglong uVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  char extraout_AL;
  longlong in_RCX;
  longlong lVar13;
  longlong lVar14;
  longlong *in_RDX;
  longlong lVar15;
  longlong lVar16;
  
  lVar2 = in_RDX[1];
  uVar3 = in_RDX[2];
  if (uVar3 * 8 != 0x10 && 1 < uVar3) {
    lVar13 = uVar3 * 8 + -0x10;
    do {
      lVar13 = lVar13 + -8;
    } while (lVar13 != 0);
  }
  lVar13 = lVar2 + uVar3 * 8;
  lVar4 = *in_RDX;
  lVar5 = *(longlong *)(in_RCX + 8);
  lVar6 = *(longlong *)(in_RCX + 0x20);
  while( true ) {
    if (lVar2 == lVar13) {
      if (lVar4 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      return;
    }
    uVar3 = *(ulonglong *)(lVar13 + -8);
    if (*(ulonglong *)(in_RCX + 0x10) <= uVar3) break;
    lVar15 = *(ulonglong *)(in_RCX + 0x10) - 1;
    puVar1 = (undefined8 *)(lVar5 + lVar15 * 0x18);
    uVar7 = *puVar1;
    uVar8 = puVar1[1];
    *(undefined8 *)(lVar5 + 0x10 + uVar3 * 0x18) = *(undefined8 *)(lVar5 + 0x10 + lVar15 * 0x18);
    puVar1 = (undefined8 *)(lVar5 + uVar3 * 0x18);
    *puVar1 = uVar7;
    puVar1[1] = uVar8;
    *(longlong *)(in_RCX + 0x10) = lVar15;
    if (*(ulonglong *)(in_RCX + 0x28) <= uVar3) {
                    // WARNING: Subroutine does not return
      alloc::vec::impl$1::swap_remove::assert_failed();
    }
    lVar15 = uVar3 * 0x38;
    lVar16 = *(ulonglong *)(in_RCX + 0x28) - 1;
    lVar14 = lVar16 * 0x38;
    uVar7 = *(undefined8 *)(lVar6 + 0x30 + lVar14);
    uVar8 = ((undefined8 *)(lVar6 + lVar14))[1];
    puVar1 = (undefined8 *)(lVar6 + 0x10 + lVar14);
    uVar9 = *puVar1;
    uVar10 = puVar1[1];
    puVar1 = (undefined8 *)(lVar6 + 0x20 + lVar14);
    uVar11 = *puVar1;
    uVar12 = puVar1[1];
    *(undefined8 *)(lVar6 + lVar15) = *(undefined8 *)(lVar6 + lVar14);
    ((undefined8 *)(lVar6 + lVar15))[1] = uVar8;
    *(undefined8 *)(lVar6 + 0x30 + lVar15) = uVar7;
    puVar1 = (undefined8 *)(lVar6 + 0x20 + lVar15);
    *puVar1 = uVar11;
    puVar1[1] = uVar12;
    puVar1 = (undefined8 *)(lVar6 + 0x10 + lVar15);
    *puVar1 = uVar9;
    puVar1[1] = uVar10;
    *(longlong *)(in_RCX + 0x28) = lVar16;
    bevy_ecs::entity::Entities::contains();
    lVar13 = lVar13 + -8;
    if (extraout_AL != '\0') {
      bevy_ecs::system::commands::EntityCommands::despawn();
    }
  }
                    // WARNING: Subroutine does not return
  alloc::vec::impl$1::swap_remove::assert_failed();
}

