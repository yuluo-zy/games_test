utils::geometry::rectangle::Rectangle3d::overlaps_rectangle3d

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong tg_math_140c98580(undefined8 *param_1,undefined8 *param_2)

{
  code *pcVar1;
  ulonglong uVar2;
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
  float fVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  float fVar19;
  float fVar20;
  float fVar21;
  
  if ((*(float *)((longlong)param_1 + 0x1c) < *(float *)(param_2 + 3)) ||
     (*(float *)((longlong)param_2 + 0x1c) < *(float *)(param_1 + 3))) {
    return 0;
  }
  fVar3 = (float)param_1[2];
  fVar4 = (float)((ulonglong)param_1[2] >> 0x20);
  if (0.0 <= SQRT(fVar4 * fVar4 + fVar3 * fVar3)) {
    fVar3 = fVar3 * _UNK_1429258e0;
    fVar4 = fVar4 * _UNK_1429258e4;
    fVar13 = (float)param_2[2];
    fVar14 = (float)((ulonglong)param_2[2] >> 0x20);
    fVar5 = (float)param_1[1];
    fVar6 = (float)((ulonglong)param_1[1] >> 0x20);
    fVar8 = (float)((ulonglong)*param_1 >> 0x20);
    fVar12 = (float)*param_1;
    fVar9 = (float)*param_2;
    fVar11 = (float)((ulonglong)*param_2 >> 0x20);
    fVar7 = (float)param_2[1];
    fVar10 = (float)((ulonglong)param_2[1] >> 0x20);
    fVar19 = fVar7 - fVar5;
    fVar21 = fVar10 - fVar6;
    fVar17 = fVar21 * fVar8 + fVar19 * fVar12;
    fVar18 = fVar21 * fVar12 + fVar19 * (float)((uint)fVar8 ^ _DAT_1429258f0);
    fVar19 = _UNK_1429258e0 * fVar13;
    fVar21 = _UNK_1429258e4 * fVar14;
    fVar15 = (float)((uint)(fVar11 * fVar8 + fVar9 * fVar12) & _DAT_14292e980);
    fVar16 = (float)((uint)(fVar11 * fVar12 + fVar9 * (float)((uint)fVar8 ^ _DAT_1429258f0)) &
                    _UNK_14292e984);
    fVar20 = fVar21 * fVar16 + fVar19 * fVar15;
    uVar2 = 0;
    if ((fVar17 - fVar20 <= fVar3) && (0.0 - fVar3 <= fVar20 + fVar17)) {
      fVar15 = fVar19 * fVar16 + fVar21 * fVar15;
      uVar2 = 0;
      if ((fVar18 - fVar15 <= fVar4) && (uVar2 = 0, 0.0 - fVar4 <= fVar18 + fVar15)) {
        if (SQRT(fVar14 * fVar14 + fVar13 * fVar13) < 0.0) goto code_r0x000140c9b667;
        fVar5 = fVar5 - fVar7;
        fVar6 = fVar6 - fVar10;
        fVar7 = fVar6 * fVar11 + fVar5 * fVar9;
        fVar6 = fVar6 * fVar9 + fVar5 * (float)((uint)fVar11 ^ _DAT_1429258f0);
        fVar10 = (float)((uint)(fVar8 * fVar11 + fVar12 * fVar9) & _DAT_14292e980);
        fVar12 = (float)((uint)(fVar8 * fVar9 + fVar12 * (float)((uint)fVar11 ^ _DAT_1429258f0)) &
                        _UNK_14292e984);
        fVar5 = fVar4 * fVar12 + fVar3 * fVar10;
        uVar2 = 0;
        if ((fVar7 - fVar5 <= fVar19) && (0.0 - fVar19 <= fVar5 + fVar7)) {
          fVar3 = fVar3 * fVar12 + fVar4 * fVar10;
          uVar2 = (ulonglong)-(uint)(0.0 - fVar21 <= fVar6 + fVar3 && fVar6 - fVar3 <= fVar21);
        }
      }
    }
    return uVar2 & 0xffffffffffffff01;
  }
code_r0x000140c9b667:
  FUN_1428d9430(&UNK_142c1f4f8,0x26,&UNK_142c1f540);
  pcVar1 = (code *)swi(3);
  uVar2 = (*pcVar1)();
  return uVar2;
}

