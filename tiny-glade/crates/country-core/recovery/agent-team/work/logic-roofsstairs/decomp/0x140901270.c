
void __cdecl country_core::resources::stairs_walls::StairsPillarProposals::add(void)

{
  char *pcVar1;
  undefined8 *puVar2;
  longlong lVar3;
  longlong lVar4;
  uint uVar5;
  uint uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  ulonglong extraout_RAX;
  undefined8 *extraout_RAX_00;
  longlong *in_RCX;
  longlong lVar12;
  longlong *in_RDX;
  undefined8 *in_R8;
  ulonglong uVar13;
  longlong lVar14;
  uint uVar15;
  ushort uVar16;
  ulonglong uVar17;
  longlong lVar18;
  byte bVar19;
  char cVar20;
  char cVar22;
  char cVar23;
  char cVar24;
  undefined1 auVar21 [16];
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  longlong local_40;
  
  if (in_RCX[3] != 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar3 = *in_RCX;
    bVar19 = (byte)(extraout_RAX >> 0x39);
    auVar21 = ZEXT216(CONCAT11(bVar19,bVar19));
    auVar21 = pshuflw(auVar21,auVar21,0);
    lVar14 = 0;
    uVar13 = extraout_RAX;
    while( true ) {
      uVar13 = in_RCX[1] & uVar13;
      pcVar1 = (char *)(lVar3 + uVar13);
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
      uVar16 = (ushort)(SUB161(auVar26 >> 7,0) & 1) | (ushort)(SUB161(auVar26 >> 0xf,0) & 1) << 1 |
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
      uVar15 = (uint)uVar16;
      while (uVar16 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar15; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        uVar17 = uVar5 + uVar13 & in_RCX[1];
        if (*in_RDX == *(longlong *)(lVar3 + -0x20 + uVar17 * -0x20)) {
          lVar18 = uVar17 * -0x20;
          lVar14 = *(longlong *)(lVar3 + -8 + lVar18);
          if (lVar14 == *(longlong *)(lVar3 + -0x18 + lVar18)) {
            alloc::raw_vec::RawVec::
            grow_one<country_core::resources::stairs_walls::StairsPillarProposal,alloc::alloc::Global>
                      ();
          }
          lVar4 = *(longlong *)(lVar3 + lVar18 + -0x10);
          lVar12 = lVar14 * 0x2c;
          uVar7 = *in_R8;
          uVar8 = in_R8[1];
          uVar9 = in_R8[2];
          uVar10 = in_R8[3];
          uVar11 = *(undefined8 *)((longlong)in_R8 + 0x24);
          puVar2 = (undefined8 *)(lVar4 + 0x1c + lVar12);
          *puVar2 = *(undefined8 *)((longlong)in_R8 + 0x1c);
          puVar2[1] = uVar11;
          puVar2 = (undefined8 *)(lVar4 + 0x10 + lVar12);
          *puVar2 = uVar9;
          puVar2[1] = uVar10;
          *(undefined8 *)(lVar4 + lVar12) = uVar7;
          ((undefined8 *)(lVar4 + lVar12))[1] = uVar8;
          *(longlong *)(lVar3 + lVar18 + -8) = lVar14 + 1;
          return;
        }
        uVar16 = (ushort)(uVar15 - 1) & (ushort)uVar15;
        uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar16);
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
                    || (SUB161(auVar25 >> 0x27,0) & 1) != 0) || (SUB161(auVar25 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar25 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar25 >> 0x3f,0) & 1) != 0) || (SUB161(auVar25 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar25 >> 0x4f,0) & 1) != 0) || (SUB161(auVar25 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar25 >> 0x5f,0) & 1) != 0) || (SUB161(auVar25 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar25 >> 0x6f,0) & 1) != 0) || (SUB161(auVar25 >> 0x77,0) & 1) != 0) ||
          auVar25[0xf] < '\0') break;
      uVar13 = uVar13 + lVar14 + 0x10;
      lVar14 = lVar14 + 0x10;
    }
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != (undefined8 *)0x0) {
    uVar7 = *in_R8;
    uVar8 = in_R8[1];
    uVar9 = in_R8[2];
    uVar10 = in_R8[3];
    uVar11 = *(undefined8 *)((longlong)in_R8 + 0x24);
    *(undefined8 *)((longlong)extraout_RAX_00 + 0x1c) = *(undefined8 *)((longlong)in_R8 + 0x1c);
    *(undefined8 *)((longlong)extraout_RAX_00 + 0x24) = uVar11;
    extraout_RAX_00[2] = uVar9;
    extraout_RAX_00[3] = uVar10;
    *extraout_RAX_00 = uVar7;
    extraout_RAX_00[1] = uVar8;
    hashbrown::map::HashMap::
    insert<country_core::resources::stairs::stairs_assembly_params::StairGraphEdgeIdentity,alloc::vec::Vec<country_core::resources::stairs_walls::StairsPillarProposal,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>
              ();
    if ((!SBORROW8(0,local_40)) && (local_40 != 0)) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}

