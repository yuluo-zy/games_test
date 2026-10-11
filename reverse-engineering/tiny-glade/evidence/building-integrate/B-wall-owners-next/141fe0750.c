
void __cdecl
utils::geometry::split_heightmap::
split_heightmap_by_threshold<system_wall_constructor::utils::trim_rows::RowTrimmerSink>(void)

{
  undefined4 uVar1;
  float fVar2;
  longlong lVar3;
  longlong lVar4;
  longlong *plVar5;
  longlong lVar6;
  longlong lVar7;
  float *in_RCX;
  longlong lVar8;
  longlong in_RDX;
  longlong lVar9;
  longlong *in_R9;
  ulonglong uVar10;
  float in_XMM2_Da;
  float fVar11;
  
  if (in_RDX == 0) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  }
  fVar11 = *in_RCX;
  lVar4 = *in_R9;
  plVar5 = (longlong *)in_R9[1];
  lVar6 = in_R9[2];
  lVar3 = in_R9[3];
  if (in_XMM2_Da < fVar11) {
    if (*(longlong *)(lVar4 + 0x28) == 0) {
LAB_141fe0988:
                    /* WARNING: Subroutine does not return */
      core::option::unwrap_failed();
    }
    uVar1 = **(undefined4 **)(lVar4 + 0x20);
    lVar9 = plVar5[2];
    if (lVar9 != lVar6) {
      if (lVar9 != 0) {
        if (plVar5[1] + lVar9 * 0x20 != 0x20) {
          *(undefined4 *)(plVar5[1] + lVar9 * 0x20 + -4) = uVar1;
          goto joined_r0x000141fe0875;
        }
      }
LAB_141fe0994:
                    /* WARNING: Subroutine does not return */
      core::option::unwrap_failed();
    }
    if (lVar6 == *plVar5) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    lVar9 = plVar5[1];
    lVar8 = lVar6 * 0x20;
    *(undefined8 *)(lVar9 + lVar8) = 0;
    ((undefined8 *)(lVar9 + lVar8))[1] = 0;
    *(longlong *)(lVar9 + 0x10 + lVar8) = lVar3;
    *(undefined4 *)(lVar9 + 0x18 + lVar8) = uVar1;
    *(undefined4 *)(lVar9 + 0x1c + lVar8) = 0x3f800000;
    plVar5[2] = lVar6 + 1;
  }
joined_r0x000141fe0875:
  if (in_RDX != 1) {
    lVar9 = in_RDX * 4 + -4;
    uVar10 = 1;
    lVar8 = lVar6 * 0x20;
    do {
      fVar2 = in_RCX[uVar10];
      if (fVar11 <= in_XMM2_Da) {
        if (in_XMM2_Da < fVar2) {
          system_wall_constructor::utils::trim_rows::impl$0::below_to_above();
          goto LAB_141fe0903;
        }
      }
      else if (fVar2 <= in_XMM2_Da) {
        system_wall_constructor::utils::trim_rows::impl$0::above_to_below();
      }
      else {
LAB_141fe0903:
        if (*(ulonglong *)(lVar4 + 0x28) <= uVar10) goto LAB_141fe0988;
        uVar1 = *(undefined4 *)(*(longlong *)(lVar4 + 0x20) + uVar10 * 4);
        lVar7 = plVar5[2];
        if (lVar7 == lVar6) {
          if (lVar6 == *plVar5) {
            alloc::raw_vec::RawVec::
            grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                      ();
          }
          lVar7 = plVar5[1];
          *(undefined8 *)(lVar7 + lVar8) = 0;
          ((undefined8 *)(lVar7 + lVar8))[1] = 0;
          *(longlong *)(lVar7 + 0x10 + lVar8) = lVar3;
          *(undefined4 *)(lVar7 + 0x18 + lVar8) = uVar1;
          *(undefined4 *)(lVar7 + 0x1c + lVar8) = 0x3f800000;
          plVar5[2] = lVar6 + 1;
        }
        else {
          if (lVar7 == 0) goto LAB_141fe0994;
          if (plVar5[1] + lVar7 * 0x20 == 0x20) goto LAB_141fe0994;
          *(undefined4 *)(plVar5[1] + lVar7 * 0x20 + -4) = uVar1;
        }
      }
      uVar10 = uVar10 + 1;
      lVar9 = lVar9 + -4;
      fVar11 = fVar2;
    } while (lVar9 != 0);
  }
  return;
}

