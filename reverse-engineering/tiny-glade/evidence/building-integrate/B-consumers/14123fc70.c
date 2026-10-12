
void __cdecl
country_core::resources::walls::wall_holes::FinalizedWallHoles::
iter_holes_with_padding<system_wall_constructor::wall_constructor::impl_0::normalize_holes::closure_env_0>
          (void)

{
  float *pfVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  undefined8 uVar5;
  float *extraout_RAX;
  float *extraout_RAX_00;
  longlong in_RCX;
  float *in_RDX;
  float *pfVar7;
  undefined8 *in_R8;
  longlong *in_R9;
  longlong lVar8;
  float *pfVar9;
  longlong lVar10;
  float fVar11;
  float in_XMM1_Da;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  undefined8 uVar6;
  
  if (*(char *)(in_RCX + 0x1c) == '\0') {
    holes_no_duplicates();
    if (extraout_RAX_00 != in_RDX) {
      uVar5 = *in_R8;
      uVar6 = *in_R8;
      lVar10 = in_R9[2];
      lVar8 = lVar10 << 4;
      pfVar7 = extraout_RAX_00;
      do {
        fVar2 = *pfVar7;
        fVar12 = pfVar7[1];
        fVar13 = pfVar7[2];
        fVar14 = pfVar7[3];
        if (lVar10 == *in_R9) {
          alloc::raw_vec::RawVec::
          grow_one<toolbar::mei::layout::ComputedLayout,alloc::alloc::Global>();
        }
        pfVar7 = pfVar7 + 5;
        pfVar9 = (float *)(in_R9[1] + lVar8);
        *pfVar9 = fVar2 * (float)uVar5;
        pfVar9[1] = fVar12 * (float)((ulonglong)uVar5 >> 0x20);
        pfVar9[2] = fVar13 * (float)uVar6;
        pfVar9[3] = fVar14 * (float)((ulonglong)uVar6 >> 0x20);
        lVar10 = lVar10 + 1;
        in_R9[2] = lVar10;
        lVar8 = lVar8 + 0x10;
      } while (pfVar7 != in_RDX);
    }
    return;
  }
  fVar2 = *(float *)(in_RCX + 0x18);
  holes_no_duplicates();
  fVar12 = (float)*in_R8;
  fVar13 = (float)((ulonglong)*in_R8 >> 0x20);
  fVar14 = (float)*in_R8;
  fVar15 = (float)((ulonglong)*in_R8 >> 0x20);
  pfVar7 = extraout_RAX;
LAB_14123fce9:
  do {
    pfVar9 = pfVar7;
    if (pfVar9 == in_RDX) {
      return;
    }
    pfVar7 = pfVar9 + 5;
  } while (*(char *)((longlong)pfVar9 + 0x11) != '\0');
  fVar16 = *pfVar9;
  fVar11 = pfVar9[1];
  fVar3 = pfVar9[2];
  fVar4 = pfVar9[3];
  lVar10 = in_R9[2];
  if (lVar10 == *in_R9) {
    alloc::raw_vec::RawVec::grow_one<toolbar::mei::layout::ComputedLayout,alloc::alloc::Global>();
  }
  pfVar1 = (float *)(in_R9[1] + lVar10 * 0x10);
  *pfVar1 = fVar16 * fVar12;
  pfVar1[1] = fVar11 * fVar13;
  pfVar1[2] = fVar3 * fVar14;
  pfVar1[3] = fVar4 * fVar15;
  lVar8 = lVar10 + 1;
  in_R9[2] = lVar8;
  fVar16 = *pfVar9;
  if (in_XMM1_Da <= fVar16) goto LAB_14123fd52;
  fVar16 = fVar16 + fVar2;
  fVar11 = pfVar9[2] + fVar2;
  goto LAB_14123fd6d;
LAB_14123fd52:
  if (fVar2 < pfVar9[2] + in_XMM1_Da) {
    fVar16 = fVar16 - fVar2;
    fVar11 = pfVar9[2] - fVar2;
LAB_14123fd6d:
    fVar3 = pfVar9[1];
    fVar4 = pfVar9[3];
    if (lVar8 == *in_R9) {
      alloc::raw_vec::RawVec::grow_one<toolbar::mei::layout::ComputedLayout,alloc::alloc::Global>();
    }
    pfVar9 = (float *)(in_R9[1] + lVar8 * 0x10);
    *pfVar9 = fVar16 * fVar12;
    pfVar9[1] = fVar3 * fVar13;
    pfVar9[2] = fVar11 * fVar14;
    pfVar9[3] = fVar4 * fVar15;
    in_R9[2] = lVar10 + 2;
  }
  goto LAB_14123fce9;
}

