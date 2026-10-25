
void __cdecl country_core::resources::roofs::roof_shape::Roof::get_height_01_unclamped_from_ws(void)

{
  longlong in_RCX;
  float fVar1;
  float fVar2;
  float fVar3;
  
  roof_noise::gpunoise::noise();
  if (0.1 < *(float *)(in_RCX + 0x30)) {
    if ((*(byte *)(in_RCX + 8) & 1) == 0) {
      fVar1 = (*(float *)(in_RCX + 0x14) + -0.90000004) / 7.1;
      fVar3 = 8.0;
    }
    else {
      fVar1 = *(float *)(in_RCX + 0x1c);
      fVar3 = *(float *)(in_RCX + 0x20);
      fVar2 = fVar3;
      if (fVar3 <= fVar1) {
        fVar2 = fVar1;
      }
      fVar1 = ((float)(~-(uint)NAN(fVar1) & (uint)fVar2 | -(uint)NAN(fVar1) & (uint)fVar3) + -2.25)
              / 12.75;
      fVar3 = 9.0;
    }
    fVar2 = 0.0;
    if (0.0 <= fVar1 / 0.4) {
      fVar2 = fVar1 / 0.4;
    }
    fVar1 = 1.0;
    if (fVar2 <= 1.0) {
      fVar1 = fVar2;
    }
    fVar1 = (3.0 - (fVar1 + fVar1)) * fVar1 * fVar1;
    if (fVar1 * 12.0 + (1.0 - fVar1) * fVar3 < 0.0) {
                    // WARNING: Subroutine does not return
      core::f32::impl$0::clamp::do_panic::runtime();
    }
  }
  ::utils::inverse_lerp();
  return;
}

