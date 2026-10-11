
void __cdecl utils::wall_space::WallSpaceCoord::from_curve_internal_coord(void)

{
  uint in_ECX;
  longlong in_R9;
  float in_XMM1_Da;
  
  if (*(ulonglong *)(in_R9 + 0x28) <= (ulonglong)in_ECX) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  }
  if ((ulonglong)in_ECX + 1 < *(ulonglong *)(in_R9 + 0x28)) {
    if ((0.0 <= in_XMM1_Da) && (in_XMM1_Da <= __real_3f800000)) {
      return;
    }
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
                    /* WARNING: Subroutine does not return */
  core::panicking::panic_bounds_check();
}

