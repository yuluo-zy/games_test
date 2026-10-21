
void __cdecl system_decorator::decorator_visual::stairs::walk_shape::closure_0(void)

{
  short *psVar1;
  byte bVar2;
  longlong lVar3;
  int *piVar4;
  bool bVar5;
  longlong lVar6;
  undefined8 *puVar7;
  undefined8 *puVar8;
  float *pfVar9;
  byte bVar10;
  longlong in_RCX;
  ulonglong uVar11;
  longlong *plVar12;
  longlong lVar13;
  ulonglong uVar14;
  ulonglong *puVar15;
  byte *pbVar16;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float in_XMM1_Da;
  float fVar17;
  float fVar18;
  float in_XMM2_Da;
  float in_XMM3_Da;
  float fVar19;
  float fVar20;
  float fVar21;
  float in_stack_00000028;
  char in_stack_00000038;
  
  ::utils::lerp();
  fVar20 = in_XMM1_Da + in_XMM3_Da * extraout_XMM0_Da * 0.5;
  fVar21 = in_XMM2_Da + in_stack_00000028 * extraout_XMM0_Da * 0.5;
  plVar12 = *(longlong **)(in_RCX + 0x10);
  lVar6 = plVar12[1];
  lVar3 = plVar12[2];
  lVar13 = lVar6 + lVar3 * 8;
  if (lVar13 != 8 && lVar3 != 0) {
    if ((*(float *)(lVar13 + -8) == fVar20) && (!NAN(*(float *)(lVar13 + -8)) && !NAN(fVar20))) {
      if ((*(float *)(lVar13 + -4) == fVar21) && (!NAN(*(float *)(lVar13 + -4)) && !NAN(fVar21))) {
        return;
      }
    }
  }
  fVar19 = -in_XMM3_Da;
  if (**(char **)(in_RCX + 0x18) == '\0') {
    if (lVar3 == *plVar12) {
      alloc::raw_vec::RawVec::grow_one<tuple_<u8,u32>,alloc::alloc::Global>();
      lVar6 = plVar12[1];
    }
    *(ulonglong *)(lVar6 + lVar3 * 8) = CONCAT44(fVar21,fVar20);
    plVar12[2] = lVar3 + 1;
    plVar12 = *(longlong **)(in_RCX + 0x48);
    lVar13 = plVar12[2];
    if (lVar13 == *plVar12) {
      alloc::raw_vec::RawVec::grow_one<tuple_<u8,u32>,alloc::alloc::Global>();
    }
    lVar6 = plVar12[1];
    *(float *)(lVar6 + lVar13 * 8) = in_stack_00000028;
    *(float *)(lVar6 + 4 + lVar13 * 8) = fVar19;
    plVar12[2] = lVar13 + 1;
    plVar12 = *(longlong **)(in_RCX + 0x50);
    lVar13 = plVar12[2];
    if (lVar13 == *plVar12) {
      alloc::raw_vec::RawVec::grow_one<f32,alloc::alloc::Global>();
    }
    *(float *)(plVar12[1] + lVar13 * 4) = extraout_XMM0_Da;
    goto LAB_1412b28de;
  }
  ::utils::lerp();
  piVar4 = *(int **)(in_RCX + 0x30);
  if (*piVar4 == 1) {
    bVar10 = 4;
    if (in_stack_00000038 == '\x05') {
      fVar17 = (float)*(undefined8 *)(piVar4 + 1) - fVar20;
      fVar18 = (float)((ulonglong)*(undefined8 *)(piVar4 + 1) >> 0x20) - fVar21;
      fVar17 = ABS(**(float **)(in_RCX + 0x38) - extraout_XMM0_Da_00) /
               SQRT(fVar18 * fVar18 + fVar17 * fVar17);
      bVar10 = 3;
      if (fVar17 <= 2.5) {
        bVar10 = (0.25 < fVar17) + 1;
      }
    }
    puVar8 = *(undefined8 **)(in_RCX + 0x40);
    uVar11 = puVar8[2];
    puVar7 = puVar8;
    uVar14 = uVar11;
    if (1 < uVar11) {
      puVar7 = (undefined8 *)*puVar8;
      uVar14 = puVar8[1];
    }
    pbVar16 = (byte *)((longlong)puVar7 + uVar14 * 4 + -4);
    if (pbVar16 != (byte *)0x0 && uVar14 != 0) {
      bVar2 = *pbVar16;
      bVar5 = bVar2 < 2;
      if (1 < bVar10) {
        bVar5 = bVar2 == bVar10 && bVar2 >= 2;
      }
      if (bVar5) {
        if ((bVar2 == bVar10) && ((bVar2 | bVar10) < 2)) {
          if (*(char *)((longlong)puVar7 + uVar14 * 4 + -3) != '\0') {
LAB_1412b29de:
            psVar1 = (short *)((longlong)puVar7 + uVar14 * 4 + -2);
            *psVar1 = *psVar1 + 1;
            goto LAB_1412b281c;
          }
        }
        else if (1 < (bVar2 | bVar10)) {
          if (bVar10 == 3) {
            if ((plVar12[2] == 0) ||
               (puVar8 = (undefined8 *)(plVar12[1] + plVar12[2] * 8 + -8),
               puVar8 == (undefined8 *)0x0)) {
                    // WARNING: Subroutine does not return
              core::option::unwrap_failed();
            }
            *puVar8 = CONCAT44(fVar21,fVar20);
            lVar13 = *(longlong *)(*(longlong *)(in_RCX + 0x48) + 0x10);
            if ((lVar13 == 0) ||
               (lVar6 = *(longlong *)(*(longlong *)(in_RCX + 0x48) + 8), lVar6 + lVar13 * 8 == 8)) {
                    // WARNING: Subroutine does not return
              core::option::unwrap_failed();
            }
            lVar6 = lVar6 + lVar13 * 8;
            *(float *)(lVar6 + -8) = in_stack_00000028;
            *(float *)(lVar6 + -4) = fVar19;
            lVar13 = *(longlong *)(*(longlong *)(in_RCX + 0x50) + 0x10);
            if ((lVar13 == 0) ||
               (pfVar9 = (float *)(*(longlong *)(*(longlong *)(in_RCX + 0x50) + 8) + lVar13 * 4 + -4
                                  ), pfVar9 == (float *)0x0)) {
                    // WARNING: Subroutine does not return
              core::option::unwrap_failed();
            }
            *pfVar9 = extraout_XMM0_Da;
            lVar13 = *(longlong *)(*(longlong *)(in_RCX + 0x58) + 0x10);
            if ((lVar13 != 0) &&
               (pfVar9 = (float *)(*(longlong *)(*(longlong *)(in_RCX + 0x58) + 8) + lVar13 * 4 + -4
                                  ), pfVar9 != (float *)0x0)) {
              *pfVar9 = extraout_XMM0_Da_00;
              *piVar4 = 1;
              *(ulonglong *)(piVar4 + 1) = CONCAT44(fVar21,fVar20);
              **(float **)(in_RCX + 0x38) = extraout_XMM0_Da_00;
              return;
            }
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
          goto LAB_1412b29de;
        }
      }
    }
    puVar15 = puVar8 + 2;
    if (1 < uVar11) {
      puVar15 = puVar8 + 1;
    }
    uVar14 = 1;
    if (1 < uVar11) {
      uVar14 = uVar11;
    }
    uVar11 = *puVar15;
    if (uVar11 == uVar14) {
      smallvec::SmallVec::
      reserve_one_unchecked<array_<country_core::resources::stairs::stairs_assembly_params::StairSubSegment,1>_>
                ();
      puVar7 = (undefined8 *)*puVar8;
      uVar11 = puVar8[1];
      puVar15 = puVar8 + 1;
    }
    *(uint *)((longlong)puVar7 + uVar11 * 4) = bVar10 | 0x10100;
    *puVar15 = *puVar15 + 1;
  }
LAB_1412b281c:
  *piVar4 = 1;
  *(ulonglong *)(piVar4 + 1) = CONCAT44(fVar21,fVar20);
  **(float **)(in_RCX + 0x38) = extraout_XMM0_Da_00;
  lVar13 = plVar12[2];
  if (lVar13 == *plVar12) {
    alloc::raw_vec::RawVec::grow_one<tuple_<u8,u32>,alloc::alloc::Global>();
  }
  *(ulonglong *)(plVar12[1] + lVar13 * 8) = CONCAT44(fVar21,fVar20);
  plVar12[2] = lVar13 + 1;
  plVar12 = *(longlong **)(in_RCX + 0x48);
  lVar13 = plVar12[2];
  if (lVar13 == *plVar12) {
    alloc::raw_vec::RawVec::grow_one<tuple_<u8,u32>,alloc::alloc::Global>();
  }
  lVar6 = plVar12[1];
  *(float *)(lVar6 + lVar13 * 8) = in_stack_00000028;
  *(float *)(lVar6 + 4 + lVar13 * 8) = fVar19;
  plVar12[2] = lVar13 + 1;
  plVar12 = *(longlong **)(in_RCX + 0x50);
  lVar13 = plVar12[2];
  if (lVar13 == *plVar12) {
    alloc::raw_vec::RawVec::grow_one<f32,alloc::alloc::Global>();
  }
  *(float *)(plVar12[1] + lVar13 * 4) = extraout_XMM0_Da;
  plVar12[2] = lVar13 + 1;
  plVar12 = *(longlong **)(in_RCX + 0x58);
  lVar13 = plVar12[2];
  if (lVar13 == *plVar12) {
    alloc::raw_vec::RawVec::grow_one<f32,alloc::alloc::Global>();
  }
  *(float *)(plVar12[1] + lVar13 * 4) = extraout_XMM0_Da_00;
LAB_1412b28de:
  plVar12[2] = lVar13 + 1;
  return;
}

