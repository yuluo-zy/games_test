// 0x140f9fdf0 action_hint<u64>
// signature: void __cdecl action_hint<u64>(void)

void __cdecl toolbar::mei::widgets::impl$26::action_hint<u64>(void)

{
  undefined1 auVar1 [16];
  longlong *plVar2;
  code *pcVar3;
  uint uVar4;
  uint uVar5;
  undefined8 extraout_RAX;
  ulonglong extraout_RAX_00;
  undefined8 extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  undefined **ppuVar6;
  longlong lVar7;
  longlong *in_RCX;
  undefined8 *puVar8;
  undefined4 uVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  undefined1 in_R9B;
  longlong lVar12;
  uint uVar13;
  ushort uVar14;
  longlong lVar15;
  byte bVar16;
  char cVar17;
  char cVar19;
  char cVar20;
  char cVar21;
  undefined1 auVar18 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  undefined8 uStack_d8;
  undefined8 uStack_c8;
  undefined8 local_c0;
  undefined8 uStack_b8;
  undefined8 local_b0;
  undefined8 uStack_a8;
  undefined8 local_a0;
  undefined8 uStack_98;
  undefined8 local_90;
  undefined4 uStack_7c;
  uint7 uStack_77;
  longlong local_70;
  longlong lStack_68;
  longlong local_60;
  
  ::utils::calculate_hash<tuple_<u64,core::any::TypeId>_>();
  ::utils::calculate_hash<tuple_<u64,u64>_>();
  core::NodeContext::child_with_global_id();
  if (*(longlong *)(local_70 + 0x1b8) != 0) {
    ::core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar7 = *(longlong *)(local_70 + 0x1a0);
    uVar10 = *(ulonglong *)(local_70 + 0x1a8);
    bVar16 = (byte)(extraout_RAX_00 >> 0x39);
    auVar18 = ZEXT216(CONCAT11(bVar16,bVar16));
    auVar18 = pshuflw(auVar18,auVar18,0);
    lVar12 = 0;
    uVar11 = extraout_RAX_00;
    while( true ) {
      uVar11 = uVar10 & uVar11;
      auVar1 = *(undefined1 (*) [16])(lVar7 + uVar11);
      cVar17 = auVar18[0];
      auVar23[0] = -(auVar1[0] == cVar17);
      cVar19 = auVar18[1];
      auVar23[1] = -(auVar1[1] == cVar19);
      cVar20 = auVar18[2];
      auVar23[2] = -(auVar1[2] == cVar20);
      cVar21 = auVar18[3];
      auVar23[3] = -(auVar1[3] == cVar21);
      auVar23[4] = -(auVar1[4] == cVar17);
      auVar23[5] = -(auVar1[5] == cVar19);
      auVar23[6] = -(auVar1[6] == cVar20);
      auVar23[7] = -(auVar1[7] == cVar21);
      auVar23[8] = -(auVar1[8] == cVar17);
      auVar23[9] = -(auVar1[9] == cVar19);
      auVar23[10] = -(auVar1[10] == cVar20);
      auVar23[0xb] = -(auVar1[0xb] == cVar21);
      auVar23[0xc] = -(auVar1[0xc] == cVar17);
      auVar23[0xd] = -(auVar1[0xd] == cVar19);
      auVar23[0xe] = -(auVar1[0xe] == cVar20);
      auVar23[0xf] = -(auVar1[0xf] == cVar21);
      uVar14 = (ushort)(SUB161(auVar23 >> 7,0) & 1) | (ushort)(SUB161(auVar23 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar23 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar23 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar23 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar23 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar23 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar23 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar23 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar23 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar23 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar23 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar23 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar23 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar23 >> 0x77,0) & 1) << 0xe | (ushort)(auVar23[0xf] >> 7) << 0xf;
      uVar13 = (uint)uVar14;
      while (uVar14 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar13; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        lVar15 = (uVar4 + uVar11 & uVar10) * -0xc0;
        if (lStack_68 == *(longlong *)(lVar7 + -0xc0 + lVar15)) {
          lVar12 = lVar7 + lVar15;
          puVar8 = *(undefined8 **)(lVar7 + -0x38 + lVar15);
          if (puVar8 == (undefined8 *)0x0) {
            plVar2 = *(longlong **)(local_70 + 0x260);
            LOCK();
            lVar7 = *plVar2;
            *plVar2 = *plVar2 + 1;
            UNLOCK();
            if (*plVar2 == 0 || SCARRY8(lVar7,1) != *plVar2 < 0) {
                    // WARNING: Does not return
              pcVar3 = (code *)invalidInstructionException();
              (*pcVar3)();
            }
            std::time::Instant::now();
            uVar9 = (undefined4)uVar10;
            LOCK();
            *plVar2 = *plVar2 + -1;
            UNLOCK();
            if (*plVar2 == 0) {
              alloc::sync::Arc::drop_slow<tg_text::font_table::FontTable,alloc::alloc::Global>();
            }
            uVar10 = 8;
            std::alloc::__default_lib_allocator::__rust_alloc();
            if (extraout_RAX_02 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
              alloc::alloc::handle_alloc_error();
            }
            extraout_RAX_02[0xe] = CONCAT44(uStack_7c,uVar9);
            extraout_RAX_02[0xf] = (ulonglong)uStack_77 << 8;
            extraout_RAX_02[0xc] = local_90;
            extraout_RAX_02[0xd] = extraout_RAX_01;
            extraout_RAX_02[10] = local_a0;
            extraout_RAX_02[0xb] = uStack_98;
            extraout_RAX_02[8] = local_b0;
            extraout_RAX_02[9] = uStack_a8;
            extraout_RAX_02[6] = local_c0;
            extraout_RAX_02[7] = uStack_b8;
            extraout_RAX_02[4] = 0x8000000000000000;
            extraout_RAX_02[5] = uStack_c8;
            extraout_RAX_02[2] = 0x3a97789c06031ab2;
            extraout_RAX_02[3] = uStack_d8;
            *extraout_RAX_02 = 0x8000000000000000;
            extraout_RAX_02[1] = extraout_RAX;
            *(undefined8 **)(lVar12 + -0x38) = extraout_RAX_02;
            ppuVar6 = &PTR_drop_in_place<toolbar::mei::widgets::ActionHintContent>_142baf770;
            *(undefined ***)(lVar12 + -0x30) =
                 &PTR_drop_in_place<toolbar::mei::widgets::ActionHintContent>_142baf770;
            puVar8 = extraout_RAX_02;
          }
          else {
            ppuVar6 = *(undefined ***)(lVar12 + -0x30);
          }
          lVar7 = (*(code *)ppuVar6[5])(puVar8);
          lVar12 = (**(code **)(uVar10 + 0x18))(lVar7);
          if (((lVar7 != 0) && (lVar12 == -0x4845213154776834)) && (uVar10 == 0x3a97789c06031ab2)) {
            *(undefined1 *)(lVar7 + 0x78) = in_R9B;
            in_RCX[2] = local_60;
            *in_RCX = local_70;
            in_RCX[1] = lStack_68;
            return;
          }
                    // WARNING: Subroutine does not return
          ::core::option::expect_failed();
        }
        uVar14 = (ushort)(uVar13 - 1) & (ushort)uVar13;
        uVar13 = CONCAT22((short)(uVar13 - 1 >> 0x10),uVar14);
      }
      auVar22[0] = -(auVar1[0] == -1);
      auVar22[1] = -(auVar1[1] == -1);
      auVar22[2] = -(auVar1[2] == -1);
      auVar22[3] = -(auVar1[3] == -1);
      auVar22[4] = -(auVar1[4] == -1);
      auVar22[5] = -(auVar1[5] == -1);
      auVar22[6] = -(auVar1[6] == -1);
      auVar22[7] = -(auVar1[7] == -1);
      auVar22[8] = -(auVar1[8] == -1);
      auVar22[9] = -(auVar1[9] == -1);
      auVar22[10] = -(auVar1[10] == -1);
      auVar22[0xb] = -(auVar1[0xb] == -1);
      auVar22[0xc] = -(auVar1[0xc] == -1);
      auVar22[0xd] = -(auVar1[0xd] == -1);
      auVar22[0xe] = -(auVar1[0xe] == -1);
      auVar22[0xf] = -(auVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar22 >> 7,0) & 1) != 0 || (SUB161(auVar22 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar22 >> 0x17,0) & 1) != 0) || (SUB161(auVar22 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar22 >> 0x27,0) & 1) != 0) || (SUB161(auVar22 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar22 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar22 >> 0x3f,0) & 1) != 0) || (SUB161(auVar22 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar22 >> 0x4f,0) & 1) != 0) || (SUB161(auVar22 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar22 >> 0x5f,0) & 1) != 0) || (SUB161(auVar22 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar22 >> 0x6f,0) & 1) != 0) || (SUB161(auVar22 >> 0x77,0) & 1) != 0) ||
          auVar22[0xf] < '\0') break;
      uVar11 = uVar11 + lVar12 + 0x10;
      lVar12 = lVar12 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  ::core::option::expect_failed();
}

