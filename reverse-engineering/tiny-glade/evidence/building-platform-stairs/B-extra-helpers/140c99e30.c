
void __cdecl utils::geometry::rectangle::Rectangle2d::_get_normal_at_u(void)

{
  float fVar1;
  longlong in_RCX;
  float fVar2;
  float fVar3;
  float local_7c;
  float *local_78;
  code *local_70;
  undefined **local_68;
  undefined8 local_60;
  float **local_58;
  undefined8 local_50;
  undefined8 local_48;
  
  fVar1 = *(float *)(in_RCX + 0x14);
  fVar2 = fVar1 + *(float *)(in_RCX + 0x10);
  fVar3 = fVar2 + fVar2;
  local_7c = fVar1 / fVar3;
  if ((0.0 <= local_7c) && (local_7c <= __real_3f800000)) {
    if (!NAN(local_7c)) {
      local_7c = fVar2 / fVar3;
      if ((local_7c < 0.0) || (__real_3f800000 < local_7c)) goto LAB_140c9a0c6;
      if (!NAN(local_7c)) {
        local_7c = (fVar1 + fVar1 + *(float *)(in_RCX + 0x10)) / fVar3;
        if ((local_7c < 0.0) || (__real_3f800000 < local_7c)) goto LAB_140c9a0c6;
        if (!NAN(local_7c)) {
          return;
        }
      }
    }
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
LAB_140c9a0c6:
  local_78 = &local_7c;
  local_70 = core::fmt::float::impl$5::fmt;
  local_68 = &anon_ca83a639f917387dae152b8c26146856_1_llvm_10654403658398494596;
  local_60 = 1;
  local_48 = 0;
  local_58 = &local_78;
  local_50 = 1;
                    /* WARNING: Subroutine does not return */
  core::panicking::panic_fmt();
}

