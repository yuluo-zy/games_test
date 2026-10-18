
void __cdecl country_core::resources::roofs::roof_ridge::inverse_ridge_01_param(void)

{
  char in_DL;
  float in_XMM0_Da;
  float fVar1;
  float in_XMM2_Da;
  float fVar2;
  float in_XMM3_Da;
  
  if (in_DL == '\0') {
    in_XMM3_Da = in_XMM2_Da;
  }
  if ((in_XMM3_Da == 0.1) && (!NAN(in_XMM3_Da))) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  fVar1 = (in_XMM0_Da + -0.1) / (in_XMM3_Da + -0.1);
  fVar2 = 0.0;
  if (0.0 <= fVar1) {
    fVar2 = fVar1;
  }
  if (((uint)ABS(fVar1) < 0x7f800000 || fVar1 < 0.0) || 1.0 < fVar2) {
    fVar1 = 1.0;
    if (fVar2 <= 1.0) {
      fVar1 = fVar2;
    }
    if (!NAN(fVar1)) {
      return;
    }
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic();
}

