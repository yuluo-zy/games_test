// name: closure$6<true>
// addr: 0x14048b320
// size: 2292
// signature: void __cdecl closure$6<true>(void)


// WARNING: Removing unreachable block (ram,0x00014048b570)
// WARNING: Removing unreachable block (ram,0x00014048b590)
// WARNING: Removing unreachable block (ram,0x00014048b579)
// WARNING: Removing unreachable block (ram,0x00014048b586)
// WARNING: Removing unreachable block (ram,0x00014048b596)
// WARNING: Removing unreachable block (ram,0x00014048bbc2)
// WARNING: Removing unreachable block (ram,0x00014048b5d4)
// WARNING: Removing unreachable block (ram,0x00014048bba3)
// WARNING: Removing unreachable block (ram,0x00014048b5e1)
// WARNING: Removing unreachable block (ram,0x00014048bb84)
// WARNING: Removing unreachable block (ram,0x00014048b5ef)
// WARNING: Removing unreachable block (ram,0x00014048bb73)
// WARNING: Removing unreachable block (ram,0x00014048b5f8)
// WARNING: Removing unreachable block (ram,0x00014048bbe1)
// WARNING: Removing unreachable block (ram,0x00014048b71f)
// WARNING: Removing unreachable block (ram,0x00014048bbd0)
// WARNING: Removing unreachable block (ram,0x00014048b72c)
// WARNING: Removing unreachable block (ram,0x00014048bbb4)
// WARNING: Removing unreachable block (ram,0x00014048b73a)
// WARNING: Removing unreachable block (ram,0x00014048bb92)
// WARNING: Removing unreachable block (ram,0x00014048b743)
// WARNING: Removing unreachable block (ram,0x00014048bafb)

void __cdecl
system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::
closure_6<true>(void)

{
  longlong lVar1;
  longlong lVar2;
  ulonglong uVar3;
  uint extraout_EAX;
  longlong in_RCX;
  ulonglong uVar4;
  float extraout_XMM0_Da;
  undefined4 extraout_XMM0_Db;
  undefined4 extraout_XMM0_Dc;
  undefined4 extraout_XMM0_Dd;
  undefined1 auVar5 [16];
  undefined1 auVar6 [16];
  float fVar7;
  
  lVar1 = *(longlong *)(in_RCX + 8);
  lVar2 = *(longlong *)(in_RCX + 0x10);
  ::utils::curve::Curve::
  get_approx_coord_from_pos<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
  uVar4 = (ulonglong)extraout_EAX;
  uVar3 = *(ulonglong *)(lVar1 + 0x10);
  if (uVar3 <= uVar4) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  if (uVar3 <= uVar4 + 1) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  uVar3 = *(ulonglong *)(lVar2 + 0x10);
  if (uVar3 <= uVar4) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  if (uVar4 + 1 < uVar3) {
    lVar1 = *(longlong *)(lVar1 + 8);
    fVar7 = *(float *)(lVar1 + uVar4 * 4);
    fVar7 = extraout_XMM0_Da * (*(float *)(lVar1 + 4 + uVar4 * 4) - fVar7) + fVar7;
    auVar5._4_4_ = extraout_XMM0_Db;
    auVar5._0_4_ = fVar7;
    auVar5._8_4_ = extraout_XMM0_Dc;
    auVar5._12_4_ = extraout_XMM0_Dd;
    auVar6._4_12_ = auVar5._4_12_;
    auVar6._0_4_ = fVar7 / 0.56;
    roundf(auVar6._0_8_);
    ::utils::random_splits_into();
    return;
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic_bounds_check();
}

