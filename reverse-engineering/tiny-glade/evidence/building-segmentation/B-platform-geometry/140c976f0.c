
void __cdecl utils::geometry::circle::Circle2d::get_closest_u_from_pos(void)

{
  float _Y;
  undefined8 *in_RCX;
  float in_XMM1_Da;
  float fVar1;
  float fVar2;
  float in_XMM2_Da;
  float fVar3;
  float local_8c;
  float *local_88;
  code *local_80;
  undefined **local_78;
  undefined8 local_70;
  undefined4 **local_68;
  undefined8 local_60;
  undefined8 local_58;
  
  _Y = __real_3f800000;
  fVar1 = in_XMM1_Da - (float)*in_RCX;
  fVar2 = in_XMM2_Da - (float)((ulonglong)*in_RCX >> 0x20);
  fVar3 = __real_3f800000 / SQRT(fVar2 * fVar2 + fVar1 * fVar1);
  if ((int)fVar3 - 1U < 0x7fffff || (int)ABS(fVar3) - 0x800000U < 0x7f000000 && -1 < (int)fVar3) {
    _hypotf(fVar1 * fVar3,fVar2 * fVar3);
    fVar2 = atan2f(fVar2 * fVar3,fVar1 * fVar3);
    fVar1 = __real_40c90fdb;
    fVar3 = fmodf(*(float *)((longlong)in_RCX + 0xc),__real_40c90fdb);
    fVar1 = _Y - (fVar2 - (float)(~-(uint)(fVar3 < 0.0) & (uint)fVar3 |
                                 (uint)(fVar3 + fVar1) & -(uint)(fVar3 < 0.0))) / fVar1;
    if (NAN(fVar1)) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic();
    }
    fVar1 = fmodf(fVar1,_Y);
    local_8c = (float)(~-(uint)(fVar1 < 0.0) & (uint)fVar1 |
                      (uint)(fVar1 + _Y) & -(uint)(fVar1 < 0.0));
    if ((local_8c < 0.0) || (_Y < local_8c)) {
      local_88 = &local_8c;
      local_80 = core::fmt::float::impl$5::fmt;
      local_78 = &anon_ca83a639f917387dae152b8c26146856_1_llvm_10654403658398494596;
      local_70 = 1;
      local_58 = 0;
      local_68 = &local_88;
      local_60 = 1;
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_fmt();
    }
    if (NAN(local_8c)) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic();
    }
  }
  return;
}

