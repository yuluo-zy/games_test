
void __cdecl country_core::systems::wall::deferred_wall_history::history_record_merge_walls(void)

{
  char *pcVar1;
  size_t sVar2;
  longlong *plVar3;
  longlong lVar4;
  void *pvVar5;
  uint uVar6;
  uint uVar7;
  longlong lVar8;
  longlong lVar9;
  longlong lVar10;
  longlong lVar11;
  ulonglong extraout_RAX;
  undefined1 *extraout_RAX_00;
  undefined1 *extraout_RAX_01;
  longlong in_RCX;
  undefined8 *in_RDX;
  ulonglong uVar12;
  longlong lVar13;
  undefined1 *puVar14;
  longlong *in_R8;
  longlong lVar15;
  uint uVar16;
  ushort uVar17;
  longlong lVar18;
  byte bVar19;
  char cVar20;
  char cVar22;
  char cVar23;
  char cVar24;
  undefined1 auVar21 [16];
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  
  *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
  plVar3 = (longlong *)*in_RDX;
  lVar18 = *plVar3;
  lVar8 = plVar3[6];
  lVar9 = plVar3[3];
  lVar4 = plVar3[0x21];
  *plVar3 = 2;
  if (lVar18 == 2) {
    return;
  }
  lVar10 = plVar3[0x11];
  lVar11 = plVar3[0x14];
  if (lVar18 == 1) {
    **(undefined4 **)(in_RCX + 0x10) = *(undefined4 *)(in_RCX + 0x1c);
    resources::history::EditType::new_height_changed();
    resources::history::History::add();
  }
  plVar3 = (longlong *)*in_R8;
  if (plVar3[3] == 0) {
LAB_1408dd508:
                    /* WARNING: Subroutine does not return */
    core::option::unwrap_failed();
  }
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  lVar18 = *plVar3;
  bVar19 = (byte)(extraout_RAX >> 0x39);
  auVar21 = ZEXT216(CONCAT11(bVar19,bVar19));
  auVar21 = pshuflw(auVar21,auVar21,0);
  lVar15 = 0;
  uVar12 = extraout_RAX;
  do {
    uVar12 = plVar3[1] & uVar12;
    pcVar1 = (char *)(lVar18 + uVar12);
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
    uVar17 = (ushort)(SUB161(auVar26 >> 7,0) & 1) | (ushort)(SUB161(auVar26 >> 0xf,0) & 1) << 1 |
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
    uVar16 = (uint)uVar17;
    while (uVar17 != 0) {
      uVar6 = 0;
      for (uVar7 = uVar16; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
        uVar6 = uVar6 + 1;
      }
      lVar13 = (uVar6 + uVar12 & plVar3[1]) * -0x78;
      if (*(longlong *)(lVar18 + -0x78 + lVar13) == lVar4) {
        resources::history::wall_compound_edit::CompoundEditBuilder::wall_deleted();
        resources::history::wall_compound_edit::CompoundEditBuilder::wall_deleted();
        lVar18 = lVar18 + lVar13;
        sVar2 = *(ulonglong *)(lVar18 + -0x60) * 8;
        if ((*(ulonglong *)(lVar18 + -0x60) >> 0x3d != 0) || (0x7ffffffffffffffc < sVar2)) {
LAB_1408dd843:
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error();
        }
        pvVar5 = *(void **)(lVar18 + -0x68);
        if (sVar2 == 0) {
          puVar14 = &DAT_00000004;
        }
        else {
          std::alloc::__default_lib_allocator::__rust_alloc();
          puVar14 = extraout_RAX_00;
          if (extraout_RAX_00 == (undefined1 *)0x0) goto LAB_1408dd843;
        }
        memcpy(puVar14,pvVar5,sVar2);
        sVar2 = *(ulonglong *)(lVar18 + -0x48) * 4;
        if ((*(ulonglong *)(lVar18 + -0x48) >> 0x3e == 0) && (sVar2 < 0x7ffffffffffffffd)) {
          pvVar5 = *(void **)(lVar18 + -0x50);
          if (sVar2 == 0) {
            puVar14 = &DAT_00000004;
          }
          else {
            std::alloc::__default_lib_allocator::__rust_alloc();
            puVar14 = extraout_RAX_01;
            if (extraout_RAX_01 == (undefined1 *)0x0) goto LAB_1408dd85e;
          }
          memcpy(puVar14,pvVar5,sVar2);
          resources::history::wall_compound_edit::CompoundEditBuilder::next();
          **(undefined4 **)(in_RCX + 0x10) = *(undefined4 *)(in_RCX + 0x1c);
          alloc::boxed::iter::impl$13::
          from_iter<country_core::resources::history::wall_compound_edit::ConcurrentEdits,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2_<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl_0::new::closure_env_0>_>
                    ();
          resources::history::History::add();
          if (lVar10 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          if (lVar11 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          if (lVar9 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          if (lVar8 == 0) {
            return;
          }
          std::alloc::__default_lib_allocator::__rust_dealloc();
          return;
        }
LAB_1408dd85e:
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error();
      }
      uVar17 = (ushort)(uVar16 - 1) & (ushort)uVar16;
      uVar16 = CONCAT22((short)(uVar16 - 1 >> 0x10),uVar17);
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
        (SUB161(auVar25 >> 0x77,0) & 1) != 0) || auVar25[0xf] < '\0') goto LAB_1408dd508;
    uVar12 = uVar12 + lVar15 + 0x10;
    lVar15 = lVar15 + 0x10;
  } while( true );
}

