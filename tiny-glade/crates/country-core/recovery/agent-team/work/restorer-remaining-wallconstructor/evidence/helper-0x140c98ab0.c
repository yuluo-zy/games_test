
void __cdecl utils::geometry::rectangle::Rectangle2d::as_points2(void)

{
  undefined8 uVar1;
  float *in_RCX;
  undefined8 *in_RDX;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  undefined8 uVar2;
  
  fVar3 = *(float *)(in_RDX + 2);
  fVar4 = *(float *)((longlong)in_RDX + 0x14);
  if (0.0 < fVar3 * fVar4) {
    fVar12 = (float)((ulonglong)*in_RDX >> 0x20);
    fVar9 = -fVar12;
    fVar11 = (float)*in_RDX;
    fVar10 = fVar3 * 0.5;
    fVar13 = fVar4 * 0.5;
    fVar3 = fVar3 * -0.5;
    fVar4 = fVar4 * -0.5;
    uVar1 = in_RDX[1];
    uVar2 = in_RDX[1];
    fVar5 = (float)uVar1;
    fVar6 = (float)((ulonglong)uVar1 >> 0x20);
    fVar7 = (float)uVar2;
    fVar8 = (float)((ulonglong)uVar2 >> 0x20);
    *in_RCX = fVar3 * fVar11 + fVar4 * fVar9 + fVar5;
    in_RCX[1] = fVar3 * fVar12 + fVar4 * fVar11 + fVar6;
    in_RCX[2] = fVar3 * fVar11 + fVar13 * fVar9 + fVar7;
    in_RCX[3] = fVar3 * fVar12 + fVar13 * fVar11 + fVar8;
    in_RCX[4] = fVar10 * fVar11 + fVar13 * fVar9 + fVar5;
    in_RCX[5] = fVar10 * fVar12 + fVar13 * fVar11 + fVar6;
    in_RCX[6] = fVar10 * fVar11 + fVar4 * fVar9 + fVar7;
    in_RCX[7] = fVar10 * fVar12 + fVar4 * fVar11 + fVar8;
    return;
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic();
}

