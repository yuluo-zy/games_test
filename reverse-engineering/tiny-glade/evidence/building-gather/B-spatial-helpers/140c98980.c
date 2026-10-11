
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl utils::geometry::rectangle::Rectangle2d::new(void)

{
  float *in_RCX;
  float in_XMM1_Da;
  float in_XMM2_Da;
  float in_XMM3_Da;
  float in_stack_00000028;
  float in_stack_00000030;
  float in_stack_00000038;
  
  if (0x7f7fffff < (uint)ABS(in_XMM1_Da)) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  if ((uint)ABS(in_XMM2_Da) < 0x7f800000) {
    if ((float)((uint)(in_stack_00000028 * in_stack_00000028 + in_XMM3_Da * in_XMM3_Da +
                      __real_bf800000) & ___xmm_7fffffff7fffffff7fffffff7fffffff) <= __real_3951b717
       ) {
      *in_RCX = in_XMM3_Da;
      in_RCX[1] = in_stack_00000028;
      in_RCX[4] = in_XMM1_Da;
      in_RCX[5] = in_XMM2_Da;
      in_RCX[2] = in_stack_00000030;
      in_RCX[3] = in_stack_00000038;
      return;
    }
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
                    /* WARNING: Subroutine does not return */
  core::panicking::panic();
}

