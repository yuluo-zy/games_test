
void __cdecl
country_core::resources::roofs::roof_visual_shape::get_roof_expanded_top_and_bottom(void)

{
  float fVar1;
  undefined4 *in_RCX;
  ulonglong uVar2;
  longlong in_RDX;
  ulonglong uVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  
  if ((*(byte *)(in_RDX + 8) & 1) != 0) {
    fVar4 = (float)*(undefined8 *)(in_RDX + 0x1c);
    fVar5 = (float)((ulonglong)*(undefined8 *)(in_RDX + 0x1c) >> 0x20);
    fVar6 = (SQRT(*(float *)(in_RDX + 0x2c)) + (1.0 - SQRT(*(float *)(in_RDX + 0x2c))) * 0.5) *
            (*(float *)(in_RDX + 0x38) * 1.5 + (1.0 - *(float *)(in_RDX + 0x38)) * 0.0);
    fVar1 = *(float *)(in_RDX + 0x34);
    fVar7 = (1.0 - fVar1) * 0.1;
    if (*(char *)(in_RDX + 0x3c) == '\0') {
      uVar3 = (ulonglong)(uint)(fVar4 * fVar1 + fVar7);
      uVar2 = 0x3dcccccd00000000;
    }
    else {
      uVar2 = (ulonglong)(uint)(fVar5 * fVar1 + fVar7) << 0x20;
      uVar3 = 0x3dcccccd;
    }
    *(ulonglong *)(in_RCX + 3) = uVar2 | uVar3;
    *(ulonglong *)(in_RCX + 1) = CONCAT44(fVar5 + 0.28 + fVar6,fVar4 + 0.28 + fVar6);
    *in_RCX = 1;
    return;
  }
  *(ulonglong *)(in_RCX + 1) =
       CONCAT44(0x3e4ccccd,
                ((SQRT(*(float *)(in_RDX + 0x2c)) + (1.0 - SQRT(*(float *)(in_RDX + 0x2c))) * 0.5) *
                 (*(float *)(in_RDX + 0x38) * 1.5 + (1.0 - *(float *)(in_RDX + 0x38)) * 0.0) * 1.2 +
                0.28) * 0.5 + *(float *)(in_RDX + 0x14));
  *in_RCX = 0;
  return;
}

