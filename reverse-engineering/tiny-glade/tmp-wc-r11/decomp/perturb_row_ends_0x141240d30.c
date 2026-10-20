// name: perturb_row_ends
// addr: 0x141240d30
// size: 389
// signature: void __cdecl perturb_row_ends(void)


void __cdecl system_wall_constructor::utils::perturb_row_ends(void)

{
  float *in_RCX;
  longlong in_RDX;
  float *in_R8;
  ulonglong in_R9;
  float extraout_XMM0_Da;
  float fVar1;
  float fVar2;
  float fVar3;
  ulonglong in_stack_00000028;
  float in_stack_00000030;
  float in_stack_00000038;
  
  ::utils::geometry::aabb::Aabb2::center();
  if (in_RDX == 0) {
    if (in_R9 <= in_stack_00000028) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    if (in_R9 <= in_stack_00000028 - 1) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    fVar2 = (in_R8[in_stack_00000028] + in_R8[in_stack_00000028 - 1]) * -0.5 + 1.0;
    fVar1 = extraout_XMM0_Da;
    if (fVar2 <= extraout_XMM0_Da) {
      fVar1 = fVar2;
    }
    fVar2 = (float)(-(uint)NAN(fVar2) & (uint)extraout_XMM0_Da | ~-(uint)NAN(fVar2) & (uint)fVar1) *
            1.99;
    fVar1 = in_stack_00000030;
    if (fVar2 <= in_stack_00000030) {
      fVar1 = fVar2;
    }
    fVar1 = (float)(-(uint)NAN(fVar2) & (uint)in_stack_00000030 | ~-(uint)NAN(fVar2) & (uint)fVar1);
  }
  else {
    if (in_stack_00000028 - 1 != in_RDX) {
      return;
    }
    if (in_R9 == 1) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    if (in_R9 == 0) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    fVar2 = (*in_R8 + in_R8[1]) * 0.5;
    fVar3 = 1.0 - extraout_XMM0_Da;
    fVar1 = fVar2;
    if (fVar3 <= fVar2) {
      fVar1 = fVar3;
    }
    fVar2 = (float)(~-(uint)NAN(fVar3) & (uint)fVar1 | -(uint)NAN(fVar3) & (uint)fVar2) * 1.99;
    fVar1 = in_stack_00000030;
    if (fVar2 <= in_stack_00000030) {
      fVar1 = fVar2;
    }
    fVar1 = (float)(-(uint)NAN(fVar2) & (uint)in_stack_00000030 | ~-(uint)NAN(fVar2) & (uint)fVar1);
    in_RCX = in_RCX + 2;
  }
  *in_RCX = fVar1 * in_stack_00000038 + *in_RCX;
  return;
}

