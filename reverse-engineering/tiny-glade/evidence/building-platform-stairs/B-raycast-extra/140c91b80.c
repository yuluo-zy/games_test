
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl utils::IsoTransform::nested_in(void)

{
  float fVar1;
  float fVar2;
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
  float *in_RCX;
  float *in_RDX;
  float *in_R8;
  float fVar16;
  float fVar17;
  float fVar18;
  float fVar19;
  float fVar20;
  float fVar21;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  float fVar30;
  float fVar31;
  float fVar32;
  
  fVar1 = in_R8[4];
  fVar2 = in_R8[5];
  fVar3 = in_R8[6];
  fVar4 = in_R8[7];
  fVar16 = fVar4 * fVar4;
  fVar23 = fVar3 * fVar3 + fVar2 * fVar2 + fVar1 * fVar1;
  fVar5 = *in_RDX;
  fVar6 = in_RDX[1];
  fVar7 = in_RDX[2];
  fVar8 = in_RDX[3];
  fVar18 = fVar3 * fVar7 + fVar2 * fVar6 + fVar1 * fVar5;
  fVar18 = fVar18 + fVar18;
  fVar28 = fVar1 * fVar6 - fVar5 * fVar2;
  fVar17 = fVar4 + fVar4;
  fVar9 = in_R8[1];
  fVar10 = in_R8[2];
  fVar11 = in_R8[3];
  fVar12 = in_RDX[4];
  fVar13 = in_RDX[5];
  fVar14 = in_RDX[6];
  fVar15 = in_RDX[7];
  fVar24 = fVar1 * fVar15 * ___xmm_bf8000003f800000bf8000003f800000;
  fVar25 = fVar1 * fVar14 * _UNK_1429cd174;
  fVar26 = fVar1 * fVar13 * _UNK_1429cd178;
  fVar27 = fVar1 * fVar12 * _UNK_1429cd17c;
  fVar29 = fVar14 * fVar2 * ___xmm_bf800000bf8000003f8000003f800000;
  fVar30 = fVar15 * fVar2 * _UNK_1429cd184;
  fVar31 = fVar12 * fVar2 * _UNK_1429cd188;
  fVar32 = fVar13 * fVar2 * _UNK_1429cd18c;
  fVar19 = fVar13 * fVar3 * ___xmm_bf8000003f8000003f800000bf800000;
  fVar20 = fVar12 * fVar3 * _UNK_1429cd194;
  fVar21 = fVar15 * fVar3 * _UNK_1429cd198;
  fVar22 = fVar14 * fVar3 * _UNK_1429cd19c;
  *in_RCX = fVar17 * (fVar2 * fVar7 - fVar6 * fVar3) + fVar18 * fVar1 + (fVar16 - fVar23) * fVar5 +
            *in_R8;
  in_RCX[1] = fVar17 * (fVar3 * fVar5 - fVar7 * fVar1) + fVar18 * fVar2 + (fVar16 - fVar23) * fVar6
              + fVar9;
  in_RCX[2] = fVar17 * fVar28 + fVar18 * fVar3 + (fVar16 - fVar23) * fVar7 + fVar10;
  in_RCX[3] = fVar17 * fVar28 + fVar18 * fVar4 + (fVar16 - fVar23) * fVar8 + fVar11;
  in_RCX[4] = fVar19 + fVar29 + fVar24 + fVar4 * fVar12;
  in_RCX[5] = fVar20 + fVar30 + fVar25 + fVar4 * fVar13;
  in_RCX[6] = fVar21 + fVar31 + fVar26 + fVar4 * fVar14;
  in_RCX[7] = fVar22 + fVar32 + fVar27 + fVar4 * fVar15;
  return;
}

