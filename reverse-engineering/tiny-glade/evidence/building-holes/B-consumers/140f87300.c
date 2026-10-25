
void __cdecl country_core::resources::walls::wall_holes::WallHoleStorage::finalize(void)

{
  ulonglong uVar1;
  undefined8 *puVar2;
  undefined1 uVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  ulonglong uVar8;
  ulonglong uVar9;
  longlong lVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  longlong lVar14;
  longlong lVar15;
  int *in_RCX;
  longlong lVar16;
  char in_R8B;
  float in_XMM1_Da;
  float fVar17;
  float fVar18;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined4 local_118;
  int local_f8;
  int iStack_f4;
  int iStack_f0;
  int iStack_ec;
  ulonglong local_e8;
  undefined2 uStack_db;
  undefined1 uStack_d9;
  
  if (*in_RCX == 1) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_fmt();
  }
  uVar9 = *(ulonglong *)(in_RCX + 8);
  lVar10 = *(longlong *)(in_RCX + 10);
  uVar11 = *(undefined8 *)(in_RCX + 0xc);
  uVar12 = *(undefined8 *)(in_RCX + 0xe);
  in_RCX[8] = 0;
  in_RCX[9] = 0;
  in_RCX[10] = 4;
  in_RCX[0xb] = 0;
  in_RCX[0xc] = 0;
  in_RCX[0xd] = 0;
  in_RCX[0xe] = 0;
  *(undefined1 *)(in_RCX + 0xf) = 0;
  lVar4 = *(longlong *)(in_RCX + 0x10);
  lVar5 = *(longlong *)(in_RCX + 0x12);
  lVar6 = *(longlong *)(in_RCX + 0x14);
  lVar7 = *(longlong *)(in_RCX + 0x16);
  uVar8 = *(ulonglong *)(in_RCX + 6);
  uVar13 = *(undefined8 *)(in_RCX + 2);
  lVar14 = *(longlong *)(in_RCX + 4);
  in_RCX[2] = 0;
  in_RCX[3] = 0;
  in_RCX[4] = 4;
  in_RCX[5] = 0;
  in_RCX[6] = 0;
  in_RCX[7] = 0;
  local_f8 = (int)uVar9;
  iStack_f4 = (int)(uVar9 >> 0x20);
  iStack_f0 = (int)lVar10;
  iStack_ec = (int)((ulonglong)lVar10 >> 0x20);
  if (lVar5 == lVar7 && lVar4 == lVar6) {
    in_RCX[0] = 1;
    in_RCX[1] = 0;
    *(longlong *)(in_RCX + 0x16) = lVar7;
    *(longlong *)(in_RCX + 0x12) = lVar5;
    *(longlong *)(in_RCX + 0x14) = lVar6;
    *(ulonglong *)(in_RCX + 0xe) = uVar8;
    *(longlong *)(in_RCX + 0x10) = lVar4;
    *(undefined8 *)(in_RCX + 10) = uVar13;
    *(longlong *)(in_RCX + 0xc) = lVar14;
    *(undefined8 *)(in_RCX + 6) = uVar11;
    *(undefined8 *)(in_RCX + 8) = uVar12;
    goto LAB_140f876b7;
  }
  local_e8 = 0;
  if (in_R8B == '\0') {
    if (uVar9 < uVar8) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    else {
      local_e8 = 0;
      if (uVar8 == 0) goto LAB_140f87639;
    }
    lVar16 = 0;
    lVar15 = 0;
    do {
      ::utils::geometry::aabb::Aabb2::from_center_dims();
      local_118._0_2_ = (ushort)*(byte *)(lVar14 + 0x10 + lVar16);
      *(undefined8 *)(lVar10 + lVar16) = local_128;
      ((undefined8 *)(lVar10 + lVar16))[1] = uStack_120;
      *(undefined4 *)(lVar10 + 0x10 + lVar16) = local_118;
      lVar15 = lVar15 + -1;
      lVar16 = lVar16 + 0x14;
    } while (-lVar15 != uVar8);
    local_e8 = -lVar15;
LAB_140f87639:
    if (1 < local_e8) {
      if (local_e8 < 0x15) {
        core::slice::sort::shared::smallsort::
        insertion_sort_shift_left<tuple_<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl_0::sort_by::closure_env_0<tuple_<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl_1::finalize_from::closure_env_1>_>
                  ();
      }
      else {
        core::slice::sort::stable::
        driftsort_main<tuple_<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl_0::sort_by::closure_env_0<tuple_<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl_1::finalize_from::closure_env_1>,alloc::vec::Vec<tuple_<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::alloc::Global>_>
                  ();
      }
    }
  }
  else if (uVar8 != 0) {
    lVar15 = lVar14 + uVar8 * 0x14;
    lVar16 = lVar14;
    do {
      while( true ) {
        ::utils::geometry::aabb::Aabb2::from_center_dims();
        uVar3 = *(undefined1 *)(lVar16 + 0x10);
        if (local_e8 == uVar9) {
          alloc::raw_vec::RawVec::
          grow_one<rhapsody::rg::resource_info::ResourceAccessInfo,alloc::alloc::Global>();
        }
        puVar2 = (undefined8 *)(lVar10 + local_e8 * 0x14);
        *puVar2 = local_128;
        puVar2[1] = uStack_120;
        *(undefined1 *)(lVar10 + 0x10 + local_e8 * 0x14) = uVar3;
        *(undefined1 *)(lVar10 + 0x11 + local_e8 * 0x14) = 0;
        uVar1 = local_e8 + 1;
        if (0.0 <= (float)local_128) break;
        fVar17 = (float)local_128 + in_XMM1_Da;
        fVar18 = (float)uStack_120 + in_XMM1_Da;
joined_r0x000140f87529:
        if (uVar1 == uVar9) {
          alloc::raw_vec::RawVec::
          grow_one<rhapsody::rg::resource_info::ResourceAccessInfo,alloc::alloc::Global>();
        }
        *(float *)(lVar10 + uVar1 * 0x14) = fVar17;
        *(undefined4 *)(lVar10 + 4 + uVar1 * 0x14) = local_128._4_4_;
        *(float *)(lVar10 + 8 + uVar1 * 0x14) = fVar18;
        *(undefined4 *)(lVar10 + 0xc + uVar1 * 0x14) = uStack_120._4_4_;
        *(undefined1 *)(lVar10 + 0x10 + uVar1 * 0x14) = uVar3;
        *(undefined1 *)(lVar10 + 0x11 + uVar1 * 0x14) = 1;
        local_e8 = local_e8 + 2;
        lVar16 = lVar16 + 0x14;
        if (lVar16 == lVar15) goto LAB_140f87639;
      }
      if (in_XMM1_Da < (float)uStack_120) {
        fVar17 = (float)local_128 - in_XMM1_Da;
        fVar18 = (float)uStack_120 - in_XMM1_Da;
        goto joined_r0x000140f87529;
      }
      lVar16 = lVar16 + 0x14;
      local_e8 = uVar1;
    } while (lVar16 != lVar15);
    goto LAB_140f87639;
  }
  in_RCX[0] = 1;
  in_RCX[1] = 0;
  *(longlong *)(in_RCX + 0x16) = lVar7;
  *(longlong *)(in_RCX + 0x12) = lVar5;
  *(longlong *)(in_RCX + 0x14) = lVar6;
  uStack_db = (undefined2)((ulonglong)uVar12 >> 0x28);
  uStack_d9 = (undefined1)((ulonglong)uVar12 >> 0x38);
  *(ulonglong *)(in_RCX + 0xe) = uVar8;
  *(longlong *)(in_RCX + 0x10) = lVar4;
  *(undefined8 *)(in_RCX + 10) = uVar13;
  *(longlong *)(in_RCX + 0xc) = lVar14;
  *(ulonglong *)(in_RCX + 6) = local_e8;
  in_RCX[8] = (int)in_XMM1_Da;
  *(char *)(in_RCX + 9) = in_R8B;
  *(undefined2 *)((longlong)in_RCX + 0x25) = uStack_db;
  *(undefined1 *)((longlong)in_RCX + 0x27) = uStack_d9;
LAB_140f876b7:
  in_RCX[2] = local_f8;
  in_RCX[3] = iStack_f4;
  in_RCX[4] = iStack_f0;
  in_RCX[5] = iStack_ec;
  return;
}

