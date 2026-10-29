
void __cdecl country_core::resources::terrain_editing::TerrainEditingData::get_last_mut_stroke(void)

{
  char *pcVar1;
  longlong lVar2;
  uint uVar3;
  uint uVar4;
  ulonglong extraout_RAX;
  longlong in_RCX;
  ushort uVar5;
  ulonglong uVar6;
  longlong lVar7;
  uint uVar8;
  byte bVar9;
  char cVar10;
  char cVar12;
  char cVar13;
  char cVar14;
  undefined1 auVar11 [16];
  undefined1 auVar15 [16];
  undefined1 auVar16 [16];
  
  lVar2 = *(longlong *)(in_RCX + 0x38);
  if (lVar2 == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  if (*(longlong *)(in_RCX + 0x20) != 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
    bVar9 = (byte)(extraout_RAX >> 0x39);
    auVar11 = ZEXT216(CONCAT11(bVar9,bVar9));
    auVar11 = pshuflw(auVar11,auVar11,0);
    lVar7 = 0;
    uVar6 = extraout_RAX;
    while( true ) {
      uVar6 = *(ulonglong *)(in_RCX + 0x10) & uVar6;
      pcVar1 = (char *)(*(longlong *)(in_RCX + 8) + uVar6);
      cVar10 = auVar11[0];
      auVar16[0] = -(*pcVar1 == cVar10);
      cVar12 = auVar11[1];
      auVar16[1] = -(pcVar1[1] == cVar12);
      cVar13 = auVar11[2];
      auVar16[2] = -(pcVar1[2] == cVar13);
      cVar14 = auVar11[3];
      auVar16[3] = -(pcVar1[3] == cVar14);
      auVar16[4] = -(pcVar1[4] == cVar10);
      auVar16[5] = -(pcVar1[5] == cVar12);
      auVar16[6] = -(pcVar1[6] == cVar13);
      auVar16[7] = -(pcVar1[7] == cVar14);
      auVar16[8] = -(pcVar1[8] == cVar10);
      auVar16[9] = -(pcVar1[9] == cVar12);
      auVar16[10] = -(pcVar1[10] == cVar13);
      auVar16[0xb] = -(pcVar1[0xb] == cVar14);
      auVar16[0xc] = -(pcVar1[0xc] == cVar10);
      auVar16[0xd] = -(pcVar1[0xd] == cVar12);
      auVar16[0xe] = -(pcVar1[0xe] == cVar13);
      auVar16[0xf] = -(pcVar1[0xf] == cVar14);
      uVar5 = (ushort)(SUB161(auVar16 >> 7,0) & 1) | (ushort)(SUB161(auVar16 >> 0xf,0) & 1) << 1 |
              (ushort)(SUB161(auVar16 >> 0x17,0) & 1) << 2 |
              (ushort)(SUB161(auVar16 >> 0x1f,0) & 1) << 3 |
              (ushort)(SUB161(auVar16 >> 0x27,0) & 1) << 4 |
              (ushort)(SUB161(auVar16 >> 0x2f,0) & 1) << 5 |
              (ushort)(SUB161(auVar16 >> 0x37,0) & 1) << 6 |
              (ushort)(SUB161(auVar16 >> 0x3f,0) & 1) << 7 |
              (ushort)(SUB161(auVar16 >> 0x47,0) & 1) << 8 |
              (ushort)(SUB161(auVar16 >> 0x4f,0) & 1) << 9 |
              (ushort)(SUB161(auVar16 >> 0x57,0) & 1) << 10 |
              (ushort)(SUB161(auVar16 >> 0x5f,0) & 1) << 0xb |
              (ushort)(SUB161(auVar16 >> 0x67,0) & 1) << 0xc |
              (ushort)(SUB161(auVar16 >> 0x6f,0) & 1) << 0xd |
              (ushort)(SUB161(auVar16 >> 0x77,0) & 1) << 0xe | (ushort)(auVar16[0xf] >> 7) << 0xf;
      uVar8 = (uint)uVar5;
      while (uVar5 != 0) {
        uVar3 = 0;
        for (uVar4 = uVar8; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
          uVar3 = uVar3 + 1;
        }
        if ((int)lVar2 + -1 ==
            *(int *)(*(longlong *)(in_RCX + 8) + -0x30 +
                    (uVar3 + uVar6 & *(ulonglong *)(in_RCX + 0x10)) * -0x30)) {
          return;
        }
        uVar5 = (ushort)(uVar8 - 1) & (ushort)uVar8;
        uVar8 = CONCAT22((short)(uVar8 - 1 >> 0x10),uVar5);
      }
      auVar15[0] = -(*pcVar1 == -1);
      auVar15[1] = -(pcVar1[1] == -1);
      auVar15[2] = -(pcVar1[2] == -1);
      auVar15[3] = -(pcVar1[3] == -1);
      auVar15[4] = -(pcVar1[4] == -1);
      auVar15[5] = -(pcVar1[5] == -1);
      auVar15[6] = -(pcVar1[6] == -1);
      auVar15[7] = -(pcVar1[7] == -1);
      auVar15[8] = -(pcVar1[8] == -1);
      auVar15[9] = -(pcVar1[9] == -1);
      auVar15[10] = -(pcVar1[10] == -1);
      auVar15[0xb] = -(pcVar1[0xb] == -1);
      auVar15[0xc] = -(pcVar1[0xc] == -1);
      auVar15[0xd] = -(pcVar1[0xd] == -1);
      auVar15[0xe] = -(pcVar1[0xe] == -1);
      auVar15[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar15 >> 7,0) & 1) != 0 || (SUB161(auVar15 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar15 >> 0x17,0) & 1) != 0) || (SUB161(auVar15 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar15 >> 0x27,0) & 1) != 0) || (SUB161(auVar15 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar15 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar15 >> 0x3f,0) & 1) != 0) || (SUB161(auVar15 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar15 >> 0x4f,0) & 1) != 0) || (SUB161(auVar15 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar15 >> 0x5f,0) & 1) != 0) || (SUB161(auVar15 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar15 >> 0x6f,0) & 1) != 0) || (SUB161(auVar15 >> 0x77,0) & 1) != 0) ||
          auVar15[0xf] < '\0') break;
      uVar6 = uVar6 + lVar7 + 0x10;
      lVar7 = lVar7 + 0x10;
    }
  }
  return;
}


