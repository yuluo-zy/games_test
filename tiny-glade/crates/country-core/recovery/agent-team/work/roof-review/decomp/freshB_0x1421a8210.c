// 0x1421a8210
// void __cdecl ui_edit_roof_ridge_length(void)
// ui_edit_roof_ridge_length


// WARNING: Removing unreachable block (ram,0x0001421a8824)
// WARNING: Removing unreachable block (ram,0x0001421a8849)
// WARNING: Removing unreachable block (ram,0x0001421a8859)
// WARNING: Removing unreachable block (ram,0x0001421a880a)

void __cdecl system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length(void)

{
  byte *pbVar1;
  undefined1 auVar2 [16];
  byte bVar3;
  byte bVar4;
  byte bVar5;
  int iVar6;
  int *piVar7;
  longlong lVar8;
  longlong lVar9;
  longlong lVar10;
  ushort uVar11;
  char extraout_AL;
  uint uVar12;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  ulonglong extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  longlong *extraout_RAX_09;
  undefined8 *in_RCX;
  longlong *plVar13;
  ulonglong uVar14;
  longlong *in_R8;
  undefined *puVar15;
  longlong lVar16;
  uint uVar17;
  undefined4 local_108;
  undefined4 uStack_104;
  undefined1 (*local_100) [16];
  ushort local_f0;
  longlong local_e8;
  longlong *local_58;
  
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    uVar14 = 0x41;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar14 != 0) {
        if (uVar14 < 0x41) {
          if ((char)(&DAT_142ff6f77)[uVar14] < -0x40) {
LAB_1421a8293:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar14 != 0x41) goto LAB_1421a8293;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar14 != 0xfffffffffffffffe)) {
        if (uVar14 + 2 < 0x41) {
          if ((char)(&UNK_142ff6f79)[uVar14] < -0x40) {
LAB_1421a82f1:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar14 + 2 != 0x41) goto LAB_1421a82f1;
      }
    }
    puVar15 = &DAT_142ff71ac;
    do {
      bVar3 = puVar15[-1];
      uVar12 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = puVar15[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar15[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar15 + -4;
            puVar15 = puVar15 + -4;
            uVar12 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar15 = puVar15 + -3;
            uVar12 = bVar5 & 0xf;
          }
          uVar12 = bVar4 & 0x3f | uVar12 << 6;
        }
        else {
          puVar15 = puVar15 + -2;
          uVar12 = bVar4 & 0x1f;
        }
        uVar12 = (uint)(bVar3 & 0x3f) | uVar12 << 6;
        if (uVar12 != 0x5c) goto LAB_1421a83b4;
LAB_1421a83c5:
        if (puVar15 == &UNK_142ff7177) break;
        if (puVar15 + -0x142ff7177 < (undefined *)0x34) {
          if (-0x41 < (char)puVar15[1]) break;
        }
        else if (puVar15 + -0x142ff7177 == (undefined *)0x34) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar15 = puVar15 + -1;
      if (uVar12 == 0x5c) goto LAB_1421a83c5;
LAB_1421a83b4:
      if (uVar12 == 0x2f) goto LAB_1421a83c5;
    } while (puVar15 != &DAT_142ff7178);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar13 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar13 = extraout_RAX_07, extraout_RAX_07 == (longlong *)0x0))
    goto LAB_1421a896e;
    if (*plVar13 != 0) goto LAB_1421a897a;
    *plVar13 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar13 = *plVar13 + 1;
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_02 == 0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  piVar7 = (int *)*in_RCX;
  iVar6 = *piVar7;
  if (iVar6 == 0) {
    country_core::resources::walls::decorator_storage::Decorators::iter_removed_ids();
    lVar16 = CONCAT44(uStack_104,local_108);
    lVar10 = *in_R8;
    lVar8 = in_R8[1];
    uVar12 = (uint)local_f0;
    while (local_e8 != 0) {
      if ((short)uVar12 == 0) {
        do {
          auVar2 = *local_100;
          uVar11 = (ushort)(SUB161(auVar2 >> 7,0) & 1) | (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1
                   | (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar2[0xf] >> 7) << 0xf;
          lVar16 = lVar16 + -0x2a80;
          local_100 = local_100 + 1;
        } while (uVar11 == 0xffff);
        uVar12 = ~(uint)uVar11;
        uVar17 = uVar12 & -(uint)uVar11 - 2;
      }
      else {
        if (lVar16 == 0) break;
        uVar17 = uVar12 - 1 & uVar12;
      }
      for (; (uVar12 & 1) == 0; uVar12 = uVar12 >> 1 | 0x80000000) {
      }
      country_core::resources::walls::public_walls::PublicWalls::roof_entity();
      local_e8 = local_e8 + -1;
      uVar12 = uVar17;
      if (((extraout_RAX_03 >> 0x20 != 0) &&
          (uVar14 = extraout_RAX_03 & 0xffffffff, uVar14 < *(ulonglong *)(lVar8 + 0x10))) &&
         (lVar9 = *(longlong *)(lVar8 + 8),
         *(int *)(lVar9 + uVar14 * 0x14) == (int)(extraout_RAX_03 >> 0x20))) {
        uVar17 = *(uint *)(lVar9 + uVar14 * 0x14 + 4);
        uVar14 = (ulonglong)uVar17;
        if (((uVar14 != 0xffffffff) && (uVar14 < *(ulonglong *)(lVar10 + 0x38))) &&
           ((*(ulonglong *)(*(longlong *)(lVar10 + 0x28) + (ulonglong)(uVar17 >> 6) * 8) >>
             (uVar14 & 0x3f) & 1) != 0)) {
          create_ui_widget();
        }
      }
    }
    iVar6 = *piVar7;
  }
  if (iVar6 - 1U < 2) {
    country_core::resources::walls::public_walls::PublicWalls::roof_entity();
    if (extraout_RAX_04 >> 0x20 == 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
      if (extraout_AL == '\0') {
        return;
      }
      puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
      if (*extraout_RAX_05 == 1) {
        local_58 = extraout_RAX_05 + 1;
      }
      else if ((*extraout_RAX_05 != 0) ||
              (std::sys::thread_local::native::lazy::Storage::
               initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                         (), local_58 = extraout_RAX_09, extraout_RAX_09 == (longlong *)0x0))
      goto LAB_1421a896e;
      if (*local_58 != 0) goto LAB_1421a897a;
      *local_58 = -1;
      puffin::ThreadProfiler::end_scope();
      goto LAB_1421a8908;
    }
    uVar14 = extraout_RAX_04 & 0xffffffff;
    if ((*(ulonglong *)(in_R8[1] + 0x10) <= uVar14) ||
       (lVar10 = *(longlong *)(in_R8[1] + 8),
       *(int *)(lVar10 + uVar14 * 0x14) != (int)(extraout_RAX_04 >> 0x20))) {
LAB_1421a89ae:
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    uVar12 = *(uint *)(lVar10 + uVar14 * 0x14 + 4);
    uVar14 = (ulonglong)uVar12;
    if ((uVar14 == 0xffffffff) ||
       ((*(ulonglong *)(*in_R8 + 0x38) <= uVar14 ||
        ((*(ulonglong *)(*(longlong *)(*in_R8 + 0x28) + (ulonglong)(uVar12 >> 6) * 8) >>
          (uVar14 & 0x3f) & 1) == 0)))) goto LAB_1421a89ae;
    create_ui_widget();
  }
  std::alloc::__default_lib_allocator::__rust_dealloc();
  if (extraout_AL == '\0') {
    return;
  }
  puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
  if (*extraout_RAX_06 == 1) {
    local_58 = extraout_RAX_06 + 1;
  }
  else if ((*extraout_RAX_06 != 0) ||
          (std::sys::thread_local::native::lazy::Storage::
           initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                     (), local_58 = extraout_RAX_08, extraout_RAX_08 == (longlong *)0x0)) {
LAB_1421a896e:
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  if (*local_58 != 0) {
LAB_1421a897a:
                    // WARNING: Subroutine does not return
    core::cell::panic_already_borrowed();
  }
  *local_58 = -1;
  puffin::ThreadProfiler::end_scope();
LAB_1421a8908:
  *local_58 = *local_58 + 1;
  return;
}


