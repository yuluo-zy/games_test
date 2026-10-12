
void __cdecl
utils::curve::Curve::get_approx_u_from_pos<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>
          (void)

{
  undefined8 uVar1;
  uint extraout_EAX;
  longlong in_RCX;
  ulonglong uVar2;
  float extraout_XMM0_Da;
  float fVar3;
  
  if (*(ulonglong *)(in_RCX + 0x10) < 2) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  get_approx_coord_from_pos<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
  uVar2 = (ulonglong)extraout_EAX;
  if (*(ulonglong *)(in_RCX + 0x28) <= uVar2) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  }
  if (*(ulonglong *)(in_RCX + 0x28) <= uVar2 + 1) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  }
  uVar1 = *(undefined8 *)(*(longlong *)(in_RCX + 0x20) + uVar2 * 4);
  fVar3 = extraout_XMM0_Da * (float)((ulonglong)uVar1 >> 0x20) +
          (__real_3f800000 - extraout_XMM0_Da) * (float)uVar1;
  if (!NAN(fVar3)) {
    if (ABS(fVar3) != INFINITY) {
      return;
    }
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
                    /* WARNING: Subroutine does not return */
  core::panicking::panic();
}

