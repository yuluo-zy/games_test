
void __cdecl
core::ops::function::FnOnce::
call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure_0::closure_0::closure_8::closure_env_4,tuple_<ref_mut_<egui::ui::Ui>_>_>
          (void)

{
  char *pcVar1;
  longlong *plVar2;
  longlong lVar3;
  int *piVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  ulonglong extraout_RAX;
  longlong lVar8;
  ulonglong uVar9;
  longlong *extraout_RAX_00;
  longlong *in_RCX;
  undefined1 (*pauVar10) [16];
  byte bVar11;
  undefined1 (*pauVar12) [16];
  ulonglong uVar13;
  ulonglong uVar14;
  ushort uVar15;
  ulonglong uVar16;
  longlong lVar17;
  longlong lVar18;
  byte bVar19;
  char cVar20;
  char cVar22;
  char cVar23;
  char cVar24;
  undefined1 auVar21 [16];
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  int local_c8;
  undefined8 local_c4;
  int local_bc;
  longlong local_b8;
  longlong local_a8;
  longlong local_a0;
  longlong *local_90;
  
  plVar2 = (longlong *)*in_RCX;
  *(undefined4 *)plVar2[2] = *(undefined4 *)((longlong)plVar2 + 0x1c);
  lVar3 = *plVar2;
  plVar2 = (longlong *)in_RCX[1];
  lVar18 = *plVar2;
  local_a8 = lVar3 + 0xd8;
  local_a0 = lVar18;
  hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  pauVar10 = *(undefined1 (**) [16])(lVar3 + 0xb8);
  uVar16 = *(ulonglong *)(lVar3 + 0xc0);
  uVar14 = uVar16 & extraout_RAX;
  bVar11 = (byte)(extraout_RAX >> 0x38);
  bVar19 = bVar11 >> 1;
  auVar21 = ZEXT216(CONCAT11(bVar19,bVar19));
  auVar21 = pshuflw(auVar21,auVar21,0);
  lVar17 = 0;
  uVar9 = uVar14;
  while( true ) {
    pcVar1 = *pauVar10 + uVar9;
    cVar20 = auVar21[0];
    auVar26[0] = -(*pcVar1 == cVar20);
    cVar22 = auVar21[1];
    auVar26[1] = -(pcVar1[1] == cVar22);
    cVar23 = auVar21[2];
    auVar26[2] = -(pcVar1[2] == cVar23);
    cVar24 = auVar21[3];
    auVar26[3] = -(pcVar1[3] == cVar24);
    auVar26[4] = -(pcVar1[4] == cVar20);
    auVar26[5] = -(pcVar1[5] == cVar22);
    auVar26[6] = -(pcVar1[6] == cVar23);
    auVar26[7] = -(pcVar1[7] == cVar24);
    auVar26[8] = -(pcVar1[8] == cVar20);
    auVar26[9] = -(pcVar1[9] == cVar22);
    auVar26[10] = -(pcVar1[10] == cVar23);
    auVar26[0xb] = -(pcVar1[0xb] == cVar24);
    auVar26[0xc] = -(pcVar1[0xc] == cVar20);
    auVar26[0xd] = -(pcVar1[0xd] == cVar22);
    auVar26[0xe] = -(pcVar1[0xe] == cVar23);
    auVar26[0xf] = -(pcVar1[0xf] == cVar24);
    uVar15 = (ushort)(SUB161(auVar26 >> 7,0) & 1) | (ushort)(SUB161(auVar26 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar26 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar26 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar26 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar26 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar26 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar26 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar26 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar26 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar26 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar26 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar26 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar26 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar26 >> 0x77,0) & 1) << 0xe | (ushort)(auVar26[0xf] >> 7) << 0xf;
    uVar6 = (uint)uVar15;
    while (uVar15 != 0) {
      uVar7 = 0;
      for (uVar5 = uVar6; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
        uVar7 = uVar7 + 1;
      }
      uVar13 = uVar7 + uVar9 & uVar16;
      local_b8 = lVar17;
      if (*(longlong *)pauVar10[-1 - uVar13] == lVar18) {
        pauVar12 = pauVar10 + -uVar13;
        goto LAB_1400c00d2;
      }
      uVar15 = (ushort)(uVar6 - 1) & (ushort)uVar6;
      uVar6 = CONCAT22((short)(uVar6 - 1 >> 0x10),uVar15);
    }
    auVar25[0] = -(*pcVar1 == -1);
    auVar25[1] = -(pcVar1[1] == -1);
    auVar25[2] = -(pcVar1[2] == -1);
    auVar25[3] = -(pcVar1[3] == -1);
    auVar25[4] = -(pcVar1[4] == -1);
    auVar25[5] = -(pcVar1[5] == -1);
    auVar25[6] = -(pcVar1[6] == -1);
    auVar25[7] = -(pcVar1[7] == -1);
    auVar25[8] = -(pcVar1[8] == -1);
    auVar25[9] = -(pcVar1[9] == -1);
    auVar25[10] = -(pcVar1[10] == -1);
    auVar25[0xb] = -(pcVar1[0xb] == -1);
    auVar25[0xc] = -(pcVar1[0xc] == -1);
    auVar25[0xd] = -(pcVar1[0xd] == -1);
    auVar25[0xe] = -(pcVar1[0xe] == -1);
    auVar25[0xf] = -(pcVar1[0xf] == -1);
    if ((((((((((((((((SUB161(auVar25 >> 7,0) & 1) != 0 || (SUB161(auVar25 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar25 >> 0x17,0) & 1) != 0) || (SUB161(auVar25 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar25 >> 0x27,0) & 1) != 0) || (SUB161(auVar25 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar25 >> 0x37,0) & 1) != 0) || (SUB161(auVar25 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar25 >> 0x47,0) & 1) != 0) || (SUB161(auVar25 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar25 >> 0x57,0) & 1) != 0) || (SUB161(auVar25 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar25 >> 0x67,0) & 1) != 0) || (SUB161(auVar25 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar25 >> 0x77,0) & 1) != 0) || auVar25[0xf] < '\0') break;
    lVar8 = uVar9 + lVar17;
    lVar17 = lVar17 + 0x10;
    uVar9 = lVar8 + 0x10U & uVar16;
  }
  if (*(longlong *)(lVar3 + 200) == 0) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::time_of_day::TempUiId,country_core::resources::gui::TodSnapshotGuiState>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::time_of_day::TempUiId,country_core::resources::gui::TodSnapshotGuiState,std::hash::random::RandomState>_>
              ();
    pauVar10 = *(undefined1 (**) [16])(lVar3 + 0xb8);
    uVar16 = *(ulonglong *)(lVar3 + 0xc0);
    uVar14 = extraout_RAX & uVar16;
    auVar21 = *(undefined1 (*) [16])(*pauVar10 + uVar14);
    uVar6 = (uint)(ushort)((ushort)(SUB161(auVar21 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar21 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar21 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar21 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar21 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar21 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar21 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar21 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar21 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar21 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar21 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar21 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar21 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar21 >> 0x77,0) & 1) << 0xe |
                          (ushort)(byte)(auVar21[0xf] >> 7) << 0xf);
    lVar18 = local_a0;
    if (uVar6 != 0) goto LAB_1400c007f;
  }
  else {
    auVar21 = *(undefined1 (*) [16])(*pauVar10 + uVar14);
    uVar15 = (ushort)(SUB161(auVar21 >> 7,0) & 1) | (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar21 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar21 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar21 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar21 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar21 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar21 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar21 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar21 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar21 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar21 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar21 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar21 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar21 >> 0x77,0) & 1) << 0xe |
             (ushort)(byte)(auVar21[0xf] >> 7) << 0xf;
    uVar6 = (uint)uVar15;
    if (uVar15 != 0) goto LAB_1400c007f;
  }
  lVar17 = 0x10;
  do {
    uVar14 = uVar14 + lVar17 & uVar16;
    auVar21 = *(undefined1 (*) [16])(*pauVar10 + uVar14);
    lVar17 = lVar17 + 0x10;
    uVar6 = (uint)(ushort)((ushort)(SUB161(auVar21 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar21 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar21 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar21 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar21 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar21 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar21 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar21 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar21 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar21 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar21 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar21 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar21 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar21 >> 0x77,0) & 1) << 0xe |
                          (ushort)(byte)(auVar21[0xf] >> 7) << 0xf);
  } while (uVar6 == 0);
LAB_1400c007f:
  uVar7 = 0;
  for (; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
    uVar7 = uVar7 + 1;
  }
  uVar9 = uVar7 + uVar14 & uVar16;
  bVar19 = (*pauVar10)[uVar9];
  if (-1 < (char)bVar19) {
    auVar21 = *pauVar10;
    uVar6 = 0;
    for (uVar7 = (uint)(ushort)((ushort)(SUB161(auVar21 >> 7,0) & 1) |
                                (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1 |
                                (ushort)(SUB161(auVar21 >> 0x17,0) & 1) << 2 |
                                (ushort)(SUB161(auVar21 >> 0x1f,0) & 1) << 3 |
                                (ushort)(SUB161(auVar21 >> 0x27,0) & 1) << 4 |
                                (ushort)(SUB161(auVar21 >> 0x2f,0) & 1) << 5 |
                                (ushort)(SUB161(auVar21 >> 0x37,0) & 1) << 6 |
                                (ushort)(SUB161(auVar21 >> 0x3f,0) & 1) << 7 |
                                (ushort)(SUB161(auVar21 >> 0x47,0) & 1) << 8 |
                                (ushort)(SUB161(auVar21 >> 0x4f,0) & 1) << 9 |
                                (ushort)(SUB161(auVar21 >> 0x57,0) & 1) << 10 |
                                (ushort)(SUB161(auVar21 >> 0x5f,0) & 1) << 0xb |
                                (ushort)(SUB161(auVar21 >> 0x67,0) & 1) << 0xc |
                                (ushort)(SUB161(auVar21 >> 0x6f,0) & 1) << 0xd |
                                (ushort)(SUB161(auVar21 >> 0x77,0) & 1) << 0xe |
                               (ushort)(byte)(auVar21[0xf] >> 7) << 0xf); (uVar7 & 1) == 0;
        uVar7 = uVar7 >> 1 | 0x80000000) {
      uVar6 = uVar6 + 1;
    }
    uVar9 = (ulonglong)uVar6;
    bVar19 = (*pauVar10)[uVar9];
  }
  bVar11 = bVar11 >> 1;
  (*pauVar10)[uVar9] = bVar11;
  pauVar10[1][uVar9 - 0x10 & uVar16] = bVar11;
  *(longlong *)(lVar3 + 200) = *(longlong *)(lVar3 + 200) - (ulonglong)(bVar19 & 1);
  pauVar12 = pauVar10 + -uVar9;
  *(longlong *)pauVar10[-1 - uVar9] = lVar18;
  pauVar12[-1][8] = 0;
  *(longlong *)(lVar3 + 0xd0) = *(longlong *)(lVar3 + 0xd0) + 1;
LAB_1400c00d2:
  lVar3 = *(longlong *)in_RCX[2];
  local_c8 = 2;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != (longlong *)0x0) {
    *extraout_RAX_00 = (longlong)&local_c8;
    extraout_RAX_00[1] = (longlong)(plVar2 + 1);
    extraout_RAX_00[2] = (longlong)plVar2 + 0xc;
    extraout_RAX_00[3] = (longlong)plVar2;
    extraout_RAX_00[4] = (longlong)(pauVar12[-1] + 8);
    extraout_RAX_00[5] = lVar3;
    egui::ui::Ui::horizontal_with_main_wrap_dyn<tuple_<>_>();
    LOCK();
    *local_90 = *local_90 + -1;
    UNLOCK();
    if (*local_90 == 0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    if (local_c8 != 2) {
      piVar4 = (int *)in_RCX[3];
      *piVar4 = local_c8;
      *(undefined8 *)(piVar4 + 1) = local_c4;
      piVar4[3] = local_bc;
    }
    return;
  }
                    // WARNING: Subroutine does not return
  ::alloc::alloc::handle_alloc_error();
}


