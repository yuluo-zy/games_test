
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::resources::roofs::roof_shape::Roof::height(void)

{
  longlong in_RCX;
  float fVar1;
  float fVar2;
  float fVar3;
  
  if (__real_3dcccccd < *(float *)(in_RCX + 0x30)) {
    if ((*(byte *)(in_RCX + 8) & 1) == 0) {
      fVar1 = (*(float *)(in_RCX + 0x14) + ___real_bf666667) / __real_40e33333;
      fVar3 = __real_41000000;
    }
    else {
      fVar3 = *(float *)(in_RCX + 0x1c);
      fVar1 = *(float *)(in_RCX + 0x20);
      fVar2 = fVar1;
      if (fVar1 <= fVar3) {
        fVar2 = fVar3;
      }
      fVar1 = ((float)(~-(uint)NAN(fVar3) & (uint)fVar2 | -(uint)NAN(fVar3) & (uint)fVar1) +
              ___real_c0100000) / __real_414c0000;
      fVar3 = __real_41100000;
    }
    fVar2 = 0.0;
    if (0.0 <= fVar1 / __real_3ecccccd) {
      fVar2 = fVar1 / __real_3ecccccd;
    }
    fVar1 = __real_3f800000;
    if (fVar2 <= __real_3f800000) {
      fVar1 = fVar2;
    }
    fVar1 = (__real_40400000 - (fVar1 + fVar1)) * fVar1 * fVar1;
    if (fVar1 * __real_41400000 + (__real_3f800000 - fVar1) * fVar3 < 0.0) {
                    /* WARNING: Subroutine does not return */
      core::f32::impl$0::clamp::do_panic::runtime();
    }
  }
  return;
}

