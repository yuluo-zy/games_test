// 0x14032e920 tweaker_impl<ref$<str$>_>
// signature: void __cdecl tweaker_impl<ref$<str$>_>(void)

void __cdecl toolbar::mei::widgets::tweaker::impl$13::tweaker_impl<ref_<str_>_>(void)

{
  char *pcVar1;
  longlong *plVar2;
  code *pcVar3;
  uint uVar4;
  uint uVar5;
  undefined4 extraout_EAX;
  longlong extraout_RAX;
  ulonglong extraout_RAX_00;
  undefined4 *extraout_RAX_01;
  undefined *puVar6;
  longlong lVar7;
  longlong lVar8;
  undefined4 *puVar9;
  longlong in_RDX;
  ulonglong uVar10;
  ushort uVar11;
  ulonglong uVar12;
  longlong lVar13;
  uint uVar14;
  longlong lVar15;
  byte bVar16;
  char cVar17;
  char cVar19;
  char cVar20;
  char cVar21;
  undefined1 auVar18 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  
  core::impl$0::as_id();
  ::utils::calculate_hash<tuple_<u64,core::any::TypeId>_>();
  lVar7 = *(longlong *)(in_RDX + 8);
  ::utils::calculate_hash<tuple_<u64,u64>_>();
  core::NodeContext::child_with_global_id();
  if (*(longlong *)(lVar7 + 0x1b8) != 0) {
    ::core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar8 = *(longlong *)(lVar7 + 0x1a0);
    uVar10 = *(ulonglong *)(lVar7 + 0x1a8);
    bVar16 = (byte)(extraout_RAX_00 >> 0x39);
    auVar18 = ZEXT216(CONCAT11(bVar16,bVar16));
    auVar18 = pshuflw(auVar18,auVar18,0);
    lVar13 = 0;
    uVar12 = extraout_RAX_00;
    while( true ) {
      uVar12 = uVar10 & uVar12;
      pcVar1 = (char *)(lVar8 + uVar12);
      cVar17 = auVar18[0];
      auVar23[0] = -(*pcVar1 == cVar17);
      cVar19 = auVar18[1];
      auVar23[1] = -(pcVar1[1] == cVar19);
      cVar20 = auVar18[2];
      auVar23[2] = -(pcVar1[2] == cVar20);
      cVar21 = auVar18[3];
      auVar23[3] = -(pcVar1[3] == cVar21);
      auVar23[4] = -(pcVar1[4] == cVar17);
      auVar23[5] = -(pcVar1[5] == cVar19);
      auVar23[6] = -(pcVar1[6] == cVar20);
      auVar23[7] = -(pcVar1[7] == cVar21);
      auVar23[8] = -(pcVar1[8] == cVar17);
      auVar23[9] = -(pcVar1[9] == cVar19);
      auVar23[10] = -(pcVar1[10] == cVar20);
      auVar23[0xb] = -(pcVar1[0xb] == cVar21);
      auVar23[0xc] = -(pcVar1[0xc] == cVar17);
      auVar23[0xd] = -(pcVar1[0xd] == cVar19);
      auVar23[0xe] = -(pcVar1[0xe] == cVar20);
      auVar23[0xf] = -(pcVar1[0xf] == cVar21);
      uVar11 = (ushort)(SUB161(auVar23 >> 7,0) & 1) | (ushort)(SUB161(auVar23 >> 0xf,0) & 1) << 1 |
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
      uVar14 = (uint)uVar11;
      while (uVar11 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar14; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        lVar15 = (uVar4 + uVar12 & uVar10) * -0xc0;
        if (extraout_RAX == *(longlong *)(lVar8 + -0xc0 + lVar15)) {
          lVar13 = lVar8 + lVar15;
          puVar9 = *(undefined4 **)(lVar8 + -0x38 + lVar15);
          if (puVar9 == (undefined4 *)0x0) {
            plVar2 = *(longlong **)(lVar7 + 0x260);
            LOCK();
            lVar7 = *plVar2;
            *plVar2 = *plVar2 + 1;
            UNLOCK();
            if (*plVar2 == 0 || SCARRY8(lVar7,1) != *plVar2 < 0) {
                    // WARNING: Does not return
              pcVar3 = (code *)invalidInstructionException();
              (*pcVar3)();
            }
            tweaker::impl$1::create_node_content();
            uVar10 = 4;
            std::alloc::__default_lib_allocator::__rust_alloc();
            if (extraout_RAX_01 == (undefined4 *)0x0) {
                    // WARNING: Subroutine does not return
              alloc::alloc::handle_alloc_error();
            }
            *extraout_RAX_01 = extraout_EAX;
            *(undefined4 **)(lVar13 + -0x38) = extraout_RAX_01;
            puVar6 = &DAT_1429bf4e8;
            *(undefined **)(lVar13 + -0x30) = &DAT_1429bf4e8;
            puVar9 = extraout_RAX_01;
          }
          else {
            puVar6 = *(undefined **)(lVar13 + -0x30);
          }
          lVar7 = (**(code **)(puVar6 + 0x28))(puVar9);
          lVar8 = (**(code **)(uVar10 + 0x18))(lVar7);
          if (((lVar7 != 0) && (lVar8 == 0x76ed58e6b3e3dc10)) && (uVar10 == 0x175517bc5c12b9f6)) {
            core::NodeContext::with_nav();
            layout::horizontal();
            layout::LayoutStyle::space_between();
            core::NodeContext::layout();
            return;
          }
                    // WARNING: Subroutine does not return
          ::core::option::expect_failed();
        }
        uVar11 = (ushort)(uVar14 - 1) & (ushort)uVar14;
        uVar14 = CONCAT22((short)(uVar14 - 1 >> 0x10),uVar11);
      }
      auVar22[0] = -(*pcVar1 == -1);
      auVar22[1] = -(pcVar1[1] == -1);
      auVar22[2] = -(pcVar1[2] == -1);
      auVar22[3] = -(pcVar1[3] == -1);
      auVar22[4] = -(pcVar1[4] == -1);
      auVar22[5] = -(pcVar1[5] == -1);
      auVar22[6] = -(pcVar1[6] == -1);
      auVar22[7] = -(pcVar1[7] == -1);
      auVar22[8] = -(pcVar1[8] == -1);
      auVar22[9] = -(pcVar1[9] == -1);
      auVar22[10] = -(pcVar1[10] == -1);
      auVar22[0xb] = -(pcVar1[0xb] == -1);
      auVar22[0xc] = -(pcVar1[0xc] == -1);
      auVar22[0xd] = -(pcVar1[0xd] == -1);
      auVar22[0xe] = -(pcVar1[0xe] == -1);
      auVar22[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar22 >> 7,0) & 1) != 0 || (SUB161(auVar22 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar22 >> 0x17,0) & 1) != 0) || (SUB161(auVar22 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar22 >> 0x27,0) & 1) != 0) || (SUB161(auVar22 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar22 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar22 >> 0x3f,0) & 1) != 0) || (SUB161(auVar22 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar22 >> 0x4f,0) & 1) != 0) || (SUB161(auVar22 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar22 >> 0x5f,0) & 1) != 0) || (SUB161(auVar22 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar22 >> 0x6f,0) & 1) != 0) || (SUB161(auVar22 >> 0x77,0) & 1) != 0) ||
          auVar22[0xf] < '\0') break;
      uVar12 = uVar12 + lVar13 + 0x10;
      lVar13 = lVar13 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  ::core::option::expect_failed();
}

