
void __cdecl utils::geometry::rectangle::Rectangle2d::get_point_u(void)

{
  longlong in_RCX;
  undefined8 in_RDX;
  float fVar1;
  float fVar2;
  float fStack_44;
  float *pfStack_40;
  code *pcStack_38;
  undefined **ppuStack_30;
  undefined8 uStack_28;
  float **ppfStack_20;
  undefined8 uStack_18;
  undefined8 uStack_10;
  
  fStack_44 = 0.0;
  switch(in_RDX) {
  case 0:
    break;
  case 1:
    fVar1 = *(float *)(in_RCX + 0x10) + *(float *)(in_RCX + 0x14);
    fStack_44 = *(float *)(in_RCX + 0x14) / (fVar1 + fVar1);
    break;
  case 2:
    fStack_44 = *(float *)(in_RCX + 0x14) + *(float *)(in_RCX + 0x10);
    fStack_44 = fStack_44 / (fStack_44 + fStack_44);
    break;
  case 3:
    fVar1 = *(float *)(in_RCX + 0x14);
    fVar2 = fVar1 + *(float *)(in_RCX + 0x10);
    fStack_44 = (fVar1 + fVar1 + *(float *)(in_RCX + 0x10)) / (fVar2 + fVar2);
    break;
  case 4:
    fStack_44 = __real_3f800000;
    break;
  default:
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  if ((0.0 <= fStack_44) && (fStack_44 <= __real_3f800000)) {
    if (NAN(fStack_44)) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic();
    }
    return;
  }
  pfStack_40 = &fStack_44;
  pcStack_38 = core::fmt::float::impl$5::fmt;
  ppuStack_30 = &anon_ca83a639f917387dae152b8c26146856_1_llvm_10654403658398494596;
  uStack_28 = 1;
  uStack_10 = 0;
  ppfStack_20 = &pfStack_40;
  uStack_18 = 1;
                    /* WARNING: Subroutine does not return */
  core::panicking::panic_fmt();
}

