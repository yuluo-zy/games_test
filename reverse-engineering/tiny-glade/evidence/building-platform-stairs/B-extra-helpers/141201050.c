
void __cdecl
alloc::vec::Vec::
retain<glam::f32::vec2::Vec2,alloc::alloc::Global,utils::curve::impl_4::try_new_from_points_remove_duplicates::closure_env_0<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>_>
          (void)

{
  undefined8 uVar1;
  longlong lVar2;
  longlong in_RCX;
  uint *in_RDX;
  longlong lVar3;
  longlong lVar4;
  uint uVar5;
  undefined8 *puVar6;
  longlong lVar7;
  bool bVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  
  lVar2 = *(longlong *)(in_RCX + 0x10);
  if (lVar2 == 0) {
    return;
  }
  puVar6 = *(undefined8 **)(in_RCX + 8);
  uVar5 = *in_RDX;
  uVar1 = *puVar6;
  fVar12 = (float)((ulonglong)uVar1 >> 0x20);
  fVar11 = (float)uVar1;
  if ((uVar5 & 1) != 0) {
    fVar10 = (float)in_RDX[1];
    if ((fVar10 == fVar11) && (!NAN(fVar10) && !NAN(fVar11))) {
      fVar9 = (float)in_RDX[2];
      lVar3 = 1;
      if ((fVar9 == fVar12) && (!NAN(fVar9) && !NAN(fVar12))) {
LAB_1412010ff:
        lVar4 = 1;
        lVar7 = lVar2 - lVar3;
        if (lVar7 != 0) {
          puVar6 = puVar6 + lVar3;
          do {
            while( true ) {
              uVar1 = *puVar6;
              fVar12 = (float)((ulonglong)uVar1 >> 0x20);
              fVar11 = (float)uVar1;
              if (((((uVar5 & 1) == 0) || (fVar10 != fVar11)) || (NAN(fVar10) || NAN(fVar11))) ||
                 ((fVar9 != fVar12 || (NAN(fVar9) || NAN(fVar12))))) break;
              lVar4 = lVar4 + 1;
              puVar6 = puVar6 + 1;
              lVar7 = lVar7 + -1;
              if (lVar7 == 0) goto LAB_14120110d;
            }
            *in_RDX = 1;
            *(undefined8 *)(in_RDX + 1) = uVar1;
            puVar6[-lVar4] = *puVar6;
            uVar5 = 1;
            puVar6 = puVar6 + 1;
            lVar7 = lVar7 + -1;
            fVar9 = fVar12;
            fVar10 = fVar11;
          } while (lVar7 != 0);
        }
        goto LAB_14120110d;
      }
    }
  }
  *in_RDX = 1;
  *(undefined8 *)(in_RDX + 1) = uVar1;
  if (lVar2 == 1) {
    lVar4 = 0;
  }
  else {
    lVar4 = 0;
    lVar3 = 2;
    fVar9 = fVar12;
    fVar10 = fVar11;
    do {
      uVar1 = puVar6[lVar3 + -1];
      fVar12 = (float)((ulonglong)uVar1 >> 0x20);
      fVar11 = (float)uVar1;
      if ((fVar10 == fVar11) && (!NAN(fVar10) && !NAN(fVar11))) {
        if ((fVar9 == fVar12) && (!NAN(fVar9) && !NAN(fVar12))) {
          uVar5 = 1;
          goto LAB_1412010ff;
        }
      }
      *in_RDX = 1;
      *(undefined8 *)(in_RDX + 1) = uVar1;
      bVar8 = lVar3 != lVar2;
      lVar3 = lVar3 + 1;
      fVar9 = fVar12;
      fVar10 = fVar11;
    } while (bVar8);
  }
LAB_14120110d:
  *(longlong *)(in_RCX + 0x10) = lVar2 - lVar4;
  return;
}

