
void __cdecl utils::wall_space::impl$1::project_clamp(void)

{
  byte *in_RCX;
  float in_XMM1_Da;
  
  if ((*in_RCX & 1) == 0) {
    if (*(float *)(in_RCX + 4) < 0.0) {
                    /* WARNING: Subroutine does not return */
      core::f32::impl$0::clamp::do_panic::runtime();
    }
  }
  else {
    fmodf(in_XMM1_Da,*(float *)(in_RCX + 4));
  }
  return;
}

