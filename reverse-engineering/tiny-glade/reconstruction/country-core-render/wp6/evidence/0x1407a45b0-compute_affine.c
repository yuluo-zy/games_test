// address 0x1407a45b0
// name compute_affine
// signature void __cdecl compute_affine(void)


void __cdecl utils::components::transform::Transform::compute_affine(void)

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
  float *in_RCX;
  float *in_RDX;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  
  fVar2 = in_RDX[7];
  fVar3 = in_RDX[8];
  fVar4 = in_RDX[9];
  fVar5 = *in_RDX;
  fVar6 = in_RDX[1];
  fVar7 = in_RDX[3];
  fVar8 = in_RDX[4];
  fVar9 = in_RDX[5];
  fVar10 = in_RDX[6];
  fVar1 = in_RDX[2];
  fVar16 = fVar1 + fVar1;
  fVar12 = fVar6 + fVar6;
  fVar14 = fVar5 * (fVar5 + fVar5);
  fVar11 = (fVar5 + fVar5) * fVar7;
  fVar15 = fVar16 * fVar5 - fVar12 * fVar7;
  fVar13 = fVar11 + fVar16 * fVar6;
  fVar17 = 1.0 - (fVar14 + fVar6 * fVar12);
  *in_RCX = fVar2 * (1.0 - (fVar6 * fVar12 + fVar1 * fVar16));
  in_RCX[1] = fVar2 * (fVar12 * fVar5 + fVar7 * fVar16);
  in_RCX[2] = fVar2 * fVar15;
  in_RCX[3] = fVar2 * fVar15;
  in_RCX[4] = fVar3 * (fVar12 * fVar5 - fVar7 * fVar16);
  in_RCX[5] = fVar3 * (1.0 - (fVar1 * fVar16 + fVar14));
  in_RCX[6] = fVar3 * fVar13;
  in_RCX[7] = fVar3 * fVar13;
  in_RCX[8] = fVar4 * (fVar12 * fVar7 + fVar16 * fVar5);
  in_RCX[9] = fVar4 * (fVar16 * fVar6 - fVar11);
  in_RCX[10] = fVar4 * fVar17;
  in_RCX[0xb] = fVar4 * fVar17;
  in_RCX[0xc] = fVar8;
  in_RCX[0xd] = fVar9;
  in_RCX[0xe] = fVar10;
  in_RCX[0xf] = fVar10;
  return;
}

