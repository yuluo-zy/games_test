// 0x1406816c0 hotkey
// signature: void __cdecl hotkey(void)

void __cdecl toolbar::mei::widgets::PillButtonView::hotkey(void)

{
  char *pcVar1;
  undefined8 *puVar2;
  longlong *plVar3;
  longlong lVar4;
  uint uVar5;
  uint uVar6;
  longlong extraout_RAX;
  ulonglong extraout_RAX_00;
  longlong extraout_RAX_01;
  ulonglong extraout_RAX_02;
  longlong *in_RCX;
  longlong lVar7;
  longlong *in_RDX;
  char in_R8B;
  ulonglong uVar8;
  longlong lVar9;
  uint uVar10;
  ushort uVar11;
  longlong lVar12;
  ulonglong uVar13;
  byte bVar14;
  char cVar15;
  char cVar17;
  char cVar18;
  char cVar19;
  undefined1 auVar16 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  
  core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::PillButtonContent>();
  *(char *)(extraout_RAX + 0x44) = in_R8B;
  if (in_R8B == '\x12') {
LAB_140681888:
    in_RCX[2] = in_RDX[2];
    lVar4 = in_RDX[1];
    *in_RCX = *in_RDX;
    in_RCX[1] = lVar4;
    return;
  }
  plVar3 = (longlong *)*in_RDX;
  lVar4 = in_RDX[1];
  ::utils::
  calculate_hash<tuple_<country_core::systems::ui::GlobalId,country_core::input::action_set::DigitalActionSetActionType>_>
            ();
  uVar13 = extraout_RAX_00 ^ 0xb1695320004fb443;
  uVar8 = uVar13;
  core::MeiContext::reuse_or_create_node();
  *(undefined1 *)(extraout_RAX_01 + 0x5d) = 0;
  if ((uVar8 & 1) == 0) {
    lVar9 = plVar3[2];
    if (lVar9 == *plVar3) {
      alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>();
    }
    lVar12 = plVar3[1];
    lVar7 = lVar9 * 0x38;
    *(undefined8 *)(lVar12 + lVar7) = 3;
    *(longlong *)(lVar12 + 0x10 + lVar7) = lVar4;
    puVar2 = (undefined8 *)(lVar12 + 0x18 + lVar7);
    *puVar2 = 0xc180000000000000;
    puVar2[1] = 0x3f8000003f800000;
    *(undefined8 *)(lVar12 + 0x28 + lVar7) = 0xbf800000bf800000;
    *(ulonglong *)(lVar12 + 0x30 + lVar7) = uVar13;
    plVar3[2] = lVar9 + 1;
  }
  if (plVar3[0x37] != 0) {
    ::core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar4 = plVar3[0x34];
    bVar14 = (byte)(extraout_RAX_02 >> 0x39);
    auVar16 = ZEXT216(CONCAT11(bVar14,bVar14));
    auVar16 = pshuflw(auVar16,auVar16,0);
    lVar9 = 0;
    uVar8 = extraout_RAX_02;
    while( true ) {
      uVar8 = plVar3[0x35] & uVar8;
      pcVar1 = (char *)(lVar4 + uVar8);
      cVar15 = auVar16[0];
      auVar21[0] = -(*pcVar1 == cVar15);
      cVar17 = auVar16[1];
      auVar21[1] = -(pcVar1[1] == cVar17);
      cVar18 = auVar16[2];
      auVar21[2] = -(pcVar1[2] == cVar18);
      cVar19 = auVar16[3];
      auVar21[3] = -(pcVar1[3] == cVar19);
      auVar21[4] = -(pcVar1[4] == cVar15);
      auVar21[5] = -(pcVar1[5] == cVar17);
      auVar21[6] = -(pcVar1[6] == cVar18);
      auVar21[7] = -(pcVar1[7] == cVar19);
      auVar21[8] = -(pcVar1[8] == cVar15);
      auVar21[9] = -(pcVar1[9] == cVar17);
      auVar21[10] = -(pcVar1[10] == cVar18);
      auVar21[0xb] = -(pcVar1[0xb] == cVar19);
      auVar21[0xc] = -(pcVar1[0xc] == cVar15);
      auVar21[0xd] = -(pcVar1[0xd] == cVar17);
      auVar21[0xe] = -(pcVar1[0xe] == cVar18);
      auVar21[0xf] = -(pcVar1[0xf] == cVar19);
      uVar11 = (ushort)(SUB161(auVar21 >> 7,0) & 1) | (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1 |
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
               (ushort)(SUB161(auVar21 >> 0x77,0) & 1) << 0xe | (ushort)(auVar21[0xf] >> 7) << 0xf;
      uVar10 = (uint)uVar11;
      while (uVar11 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar10; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        lVar12 = (uVar5 + uVar8 & plVar3[0x35]) * -0xc0;
        if (uVar13 == *(ulonglong *)(lVar4 + -0xc0 + lVar12)) {
          *(undefined4 *)(lVar4 + -0x10 + lVar12) = 0x43480000;
          widgets::impl$26::action_hint<u64>();
          goto LAB_140681888;
        }
        uVar11 = (ushort)(uVar10 - 1) & (ushort)uVar10;
        uVar10 = CONCAT22((short)(uVar10 - 1 >> 0x10),uVar11);
      }
      auVar20[0] = -(*pcVar1 == -1);
      auVar20[1] = -(pcVar1[1] == -1);
      auVar20[2] = -(pcVar1[2] == -1);
      auVar20[3] = -(pcVar1[3] == -1);
      auVar20[4] = -(pcVar1[4] == -1);
      auVar20[5] = -(pcVar1[5] == -1);
      auVar20[6] = -(pcVar1[6] == -1);
      auVar20[7] = -(pcVar1[7] == -1);
      auVar20[8] = -(pcVar1[8] == -1);
      auVar20[9] = -(pcVar1[9] == -1);
      auVar20[10] = -(pcVar1[10] == -1);
      auVar20[0xb] = -(pcVar1[0xb] == -1);
      auVar20[0xc] = -(pcVar1[0xc] == -1);
      auVar20[0xd] = -(pcVar1[0xd] == -1);
      auVar20[0xe] = -(pcVar1[0xe] == -1);
      auVar20[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar20 >> 7,0) & 1) != 0 || (SUB161(auVar20 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar20 >> 0x17,0) & 1) != 0) || (SUB161(auVar20 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar20 >> 0x27,0) & 1) != 0) || (SUB161(auVar20 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar20 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar20 >> 0x3f,0) & 1) != 0) || (SUB161(auVar20 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar20 >> 0x4f,0) & 1) != 0) || (SUB161(auVar20 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar20 >> 0x5f,0) & 1) != 0) || (SUB161(auVar20 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar20 >> 0x6f,0) & 1) != 0) || (SUB161(auVar20 >> 0x77,0) & 1) != 0) ||
          auVar20[0xf] < '\0') break;
      uVar8 = uVar8 + lVar9 + 0x10;
      lVar9 = lVar9 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  ::core::option::unwrap_failed();
}

