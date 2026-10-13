// signature: void __cdecl apply_roof_noise(void)

void __cdecl country_core::resources::roofs::roof_noise::apply_roof_noise(void)

{
  longlong in_RCX;
  float fVar1;
  float extraout_XMM0_Da;
  float in_XMM1_Da;
  float fVar2;
  
  fVar1 = 0.0;
  if (0.0 <= in_XMM1_Da) {
    fVar1 = in_XMM1_Da;
  }
  fVar2 = 1.0;
  if (fVar1 <= 1.0) {
    fVar2 = fVar1;
  }
  gpunoise::noise();
  *(float *)(in_RCX + 4) =
       extraout_XMM0_Da * 0.3 * (3.0 - (fVar2 + fVar2)) * fVar2 * fVar2 + *(float *)(in_RCX + 4);
  return;
}

