
void __cdecl utils::loop_range::LoopRange::new_loop(void)

{
  int *in_RCX;
  int in_EDX;
  int in_R8D;
  int in_R9D;
  
  if (in_EDX <= -in_R9D) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  if (in_R9D < in_EDX) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  if (in_R8D < 0) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  if (in_R8D <= (int)(in_R9D + 1U)) {
    if ((uint)(in_R8D - in_EDX) <= in_R9D + 1U) {
      *in_RCX = in_EDX;
      in_RCX[1] = in_R8D;
      in_RCX[2] = in_R9D;
      return;
    }
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
                    /* WARNING: Subroutine does not return */
  core::panicking::panic();
}

