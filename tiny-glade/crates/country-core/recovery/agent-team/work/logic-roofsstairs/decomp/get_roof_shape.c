
void __cdecl country_core::resources::roofs::roof_shape::Roof::get_roof_shape(void)

{
  float fVar1;
  float fVar2;
  int iVar3;
  undefined8 uVar4;
  uint *in_RCX;
  longlong in_RDX;
  bool bVar5;
  float fVar6;
  float in_XMM2_Da;
  uint uVar7;
  uint uVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  
  fVar12 = *(float *)(in_RDX + 0x2c);
  fVar6 = powf(in_XMM2_Da,1.0 / (fVar12 * 1.5 + (1.0 - fVar12) * 0.5));
  bVar5 = *(int *)(in_RDX + 8) != 1;
  if (bVar5) {
    fVar1 = *(float *)(in_RDX + 0x38);
    fVar2 = *(float *)(in_RDX + 0x14);
    uVar7 = *(uint *)(in_RDX + 0x18);
    *(undefined8 *)(in_RCX + 1) = *(undefined8 *)(in_RDX + 0xc);
    in_RCX[3] = (uint)(fVar6 * 0.2 +
                      (1.0 - fVar6) *
                      ((((1.0 - SQRT(fVar12)) * 0.5 + SQRT(fVar12)) *
                        (fVar1 * 1.5 + (1.0 - fVar1) * 0.0) * 1.2 + 0.28) * 0.5 + fVar2));
    in_RCX[4] = uVar7;
  }
  else {
    fVar1 = *(float *)(in_RDX + 0x38);
    fVar2 = *(float *)(in_RDX + 0x34);
    fVar9 = (1.0 - fVar2) * 0.1;
    uVar4 = *(undefined8 *)(in_RDX + 0x14);
    iVar3 = *(int *)(in_RDX + 0x3c);
    *(undefined8 *)(in_RCX + 1) = *(undefined8 *)(in_RDX + 0xc);
    *(undefined8 *)(in_RCX + 3) = uVar4;
    fVar10 = (float)*(undefined8 *)(in_RDX + 0x1c);
    fVar11 = (float)((ulonglong)*(undefined8 *)(in_RDX + 0x1c) >> 0x20);
    fVar12 = (fVar1 * 1.5 + (1.0 - fVar1) * 0.0) * (SQRT(fVar12) + (1.0 - SQRT(fVar12)) * 0.5);
    uVar7 = (iVar3 << 0x1f) >> 0x1f;
    uVar8 = (iVar3 << 0x1f) >> 0x1f;
    *(ulonglong *)(in_RCX + 5) =
         CONCAT44(fVar6 * (float)(~uVar8 & 0x3dcccccd | (uint)(fVar11 * fVar2 + fVar9) & uVar8) +
                  (1.0 - fVar6) * (fVar12 + fVar11 + 0.28),
                  fVar6 * (float)(~uVar7 & (uint)(fVar10 * fVar2 + fVar9) | uVar7 & 0x3dcccccd) +
                  (1.0 - fVar6) * (fVar12 + fVar10 + 0.28));
  }
  *in_RCX = (uint)!bVar5;
  return;
}

