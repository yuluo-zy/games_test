// name: above_to_below
// addr: 0x14123fab0
// size: 183
// signature: void __cdecl above_to_below(void)


void __cdecl system_wall_constructor::utils::trim_rows::impl$0::above_to_below(void)

{
  longlong lVar1;
  longlong lVar2;
  longlong *in_RCX;
  ulonglong in_RDX;
  undefined4 extraout_XMM0_Da;
  float in_XMM2_Da;
  
  if ((in_XMM2_Da < 0.0) || (1.0 < in_XMM2_Da)) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  if (*(ulonglong *)(*in_RCX + 0x28) <= (in_RDX & 0xffffffff)) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  if ((in_RDX & 0xffffffff) + 1 < *(ulonglong *)(*in_RCX + 0x28)) {
    ::utils::lerp();
    lVar2 = *(longlong *)(in_RCX[1] + 0x10);
    if (lVar2 != 0) {
      lVar1 = *(longlong *)(in_RCX[1] + 8);
      lVar2 = lVar2 * 0x20;
      if (lVar1 + lVar2 != 0x20) {
        *(undefined4 *)(lVar1 + lVar2 + -4) = extraout_XMM0_Da;
        return;
      }
    }
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic_bounds_check();
}

