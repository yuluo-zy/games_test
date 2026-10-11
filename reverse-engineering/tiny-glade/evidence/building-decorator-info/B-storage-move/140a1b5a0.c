
void __cdecl country_core::resources::walls::decorator_storage::impl$8::update(void)

{
  char *pcVar1;
  undefined8 *puVar2;
  longlong *plVar3;
  longlong lVar4;
  uint uVar5;
  uint uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  ulonglong extraout_RAX;
  undefined8 *in_RCX;
  int in_EDX;
  ulonglong uVar9;
  longlong lVar10;
  uint uVar11;
  ushort uVar12;
  ulonglong uVar13;
  byte bVar14;
  char cVar15;
  char cVar17;
  char cVar18;
  char cVar19;
  undefined1 auVar16 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined1 local_7c [4];
  undefined1 *local_78;
  code *local_70;
  code *local_60;
  undefined **local_58;
  undefined8 local_50;
  undefined1 **local_48;
  undefined8 local_40;
  undefined8 local_38;
  
  plVar3 = (longlong *)in_RCX[2];
  if (plVar3[3] != 0) {
    uVar7 = *in_RCX;
    uVar8 = in_RCX[1];
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
    lVar4 = *plVar3;
    bVar14 = (byte)(extraout_RAX >> 0x39);
    auVar16 = ZEXT216(CONCAT11(bVar14,bVar14));
    auVar16 = pshuflw(auVar16,auVar16,0);
    lVar10 = 0;
    uVar9 = extraout_RAX;
    while( true ) {
      uVar9 = plVar3[1] & uVar9;
      pcVar1 = (char *)(lVar4 + uVar9);
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
      uVar12 = (ushort)(SUB161(auVar21 >> 7,0) & 1) | (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1 |
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
      uVar11 = (uint)uVar12;
      while (uVar12 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar11; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        uVar13 = uVar5 + uVar9 & plVar3[1];
        if (*(int *)(lVar4 + -0x18 + uVar13 * -0x18) == in_EDX) {
          puVar2 = (undefined8 *)(lVar4 + -0x10 + uVar13 * -0x18);
          *puVar2 = uVar7;
          puVar2[1] = uVar8;
          return;
        }
        uVar12 = (ushort)(uVar11 - 1) & (ushort)uVar11;
        uVar11 = CONCAT22((short)(uVar11 - 1 >> 0x10),uVar12);
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
      uVar9 = uVar9 + lVar10 + 0x10;
      lVar10 = lVar10 + 0x10;
    }
  }
  local_78 = local_7c;
  local_70 = decorator::impl$27::fmt;
  local_60 = decorator_storage::impl$25::fmt;
  local_58 = &anon_087c9b6d159f545cda87747878cb277f_55_llvm_10190951805537787575;
  local_50 = 2;
  local_38 = 0;
  local_48 = &local_78;
  local_40 = 2;
                    /* WARNING: Subroutine does not return */
  core::panicking::panic_fmt();
}

