
void __cdecl
std::collections::hash::map::HashMap::
get<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>
          (void)

{
  char *pcVar1;
  char cVar2;
  longlong lVar3;
  ulonglong uVar4;
  uint uVar5;
  uint uVar6;
  ulonglong extraout_RAX;
  longlong *in_RCX;
  char *in_RDX;
  ushort uVar7;
  uint uVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  longlong lVar11;
  longlong lVar12;
  byte bVar13;
  char cVar14;
  char cVar16;
  char cVar17;
  char cVar18;
  undefined1 auVar15 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  
  if (in_RCX[3] != 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>_>
              ();
    lVar3 = *in_RCX;
    uVar4 = in_RCX[1];
    uVar10 = uVar4 & extraout_RAX;
    bVar13 = (byte)(extraout_RAX >> 0x39);
    auVar15 = ZEXT216(CONCAT11(bVar13,bVar13));
    auVar15 = pshuflw(auVar15,auVar15,0);
    cVar14 = auVar15[0];
    cVar16 = auVar15[1];
    cVar17 = auVar15[2];
    cVar18 = auVar15[3];
    if (*in_RDX == '\x1a') {
      lVar12 = 0;
      while( true ) {
        pcVar1 = (char *)(lVar3 + uVar10);
        auVar21[0] = -(*pcVar1 == cVar14);
        auVar21[1] = -(pcVar1[1] == cVar16);
        auVar21[2] = -(pcVar1[2] == cVar17);
        auVar21[3] = -(pcVar1[3] == cVar18);
        auVar21[4] = -(pcVar1[4] == cVar14);
        auVar21[5] = -(pcVar1[5] == cVar16);
        auVar21[6] = -(pcVar1[6] == cVar17);
        auVar21[7] = -(pcVar1[7] == cVar18);
        auVar21[8] = -(pcVar1[8] == cVar14);
        auVar21[9] = -(pcVar1[9] == cVar16);
        auVar21[10] = -(pcVar1[10] == cVar17);
        auVar21[0xb] = -(pcVar1[0xb] == cVar18);
        auVar21[0xc] = -(pcVar1[0xc] == cVar14);
        auVar21[0xd] = -(pcVar1[0xd] == cVar16);
        auVar21[0xe] = -(pcVar1[0xe] == cVar17);
        auVar21[0xf] = -(pcVar1[0xf] == cVar18);
        uVar7 = (ushort)(SUB161(auVar21 >> 7,0) & 1) | (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1 |
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
        uVar8 = (uint)uVar7;
        while (uVar7 != 0) {
          uVar5 = 0;
          for (uVar6 = uVar8; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
            uVar5 = uVar5 + 1;
          }
          if (*(char *)(lVar3 + -0x48 + (uVar5 + uVar10 & uVar4) * -0x48) == '\x1a') {
            return;
          }
          uVar7 = (ushort)(uVar8 - 1) & (ushort)uVar8;
          uVar8 = CONCAT22((short)(uVar8 - 1 >> 0x10),uVar7);
        }
        auVar19[0] = -(*pcVar1 == -1);
        auVar19[1] = -(pcVar1[1] == -1);
        auVar19[2] = -(pcVar1[2] == -1);
        auVar19[3] = -(pcVar1[3] == -1);
        auVar19[4] = -(pcVar1[4] == -1);
        auVar19[5] = -(pcVar1[5] == -1);
        auVar19[6] = -(pcVar1[6] == -1);
        auVar19[7] = -(pcVar1[7] == -1);
        auVar19[8] = -(pcVar1[8] == -1);
        auVar19[9] = -(pcVar1[9] == -1);
        auVar19[10] = -(pcVar1[10] == -1);
        auVar19[0xb] = -(pcVar1[0xb] == -1);
        auVar19[0xc] = -(pcVar1[0xc] == -1);
        auVar19[0xd] = -(pcVar1[0xd] == -1);
        auVar19[0xe] = -(pcVar1[0xe] == -1);
        auVar19[0xf] = -(pcVar1[0xf] == -1);
        if ((((((((((((((((SUB161(auVar19 >> 7,0) & 1) != 0 || (SUB161(auVar19 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar19 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar19 >> 0x1f,0) & 1) != 0) || (SUB161(auVar19 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar19 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar19 >> 0x37,0) & 1) != 0) || (SUB161(auVar19 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar19 >> 0x47,0) & 1) != 0) || (SUB161(auVar19 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar19 >> 0x57,0) & 1) != 0) || (SUB161(auVar19 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar19 >> 0x67,0) & 1) != 0) || (SUB161(auVar19 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar19 >> 0x77,0) & 1) != 0) || auVar19[0xf] < '\0') break;
        lVar11 = uVar10 + lVar12;
        lVar12 = lVar12 + 0x10;
        uVar10 = lVar11 + 0x10U & uVar4;
      }
    }
    else {
      lVar12 = 0;
      while( true ) {
        pcVar1 = (char *)(lVar3 + uVar10);
        auVar15[0] = -(*pcVar1 == cVar14);
        auVar15[1] = -(pcVar1[1] == cVar16);
        auVar15[2] = -(pcVar1[2] == cVar17);
        auVar15[3] = -(pcVar1[3] == cVar18);
        auVar15[4] = -(pcVar1[4] == cVar14);
        auVar15[5] = -(pcVar1[5] == cVar16);
        auVar15[6] = -(pcVar1[6] == cVar17);
        auVar15[7] = -(pcVar1[7] == cVar18);
        auVar15[8] = -(pcVar1[8] == cVar14);
        auVar15[9] = -(pcVar1[9] == cVar16);
        auVar15[10] = -(pcVar1[10] == cVar17);
        auVar15[0xb] = -(pcVar1[0xb] == cVar18);
        auVar15[0xc] = -(pcVar1[0xc] == cVar14);
        auVar15[0xd] = -(pcVar1[0xd] == cVar16);
        auVar15[0xe] = -(pcVar1[0xe] == cVar17);
        auVar15[0xf] = -(pcVar1[0xf] == cVar18);
        uVar7 = (ushort)(SUB161(auVar15 >> 7,0) & 1) | (ushort)(SUB161(auVar15 >> 0xf,0) & 1) << 1 |
                (ushort)(SUB161(auVar15 >> 0x17,0) & 1) << 2 |
                (ushort)(SUB161(auVar15 >> 0x1f,0) & 1) << 3 |
                (ushort)(SUB161(auVar15 >> 0x27,0) & 1) << 4 |
                (ushort)(SUB161(auVar15 >> 0x2f,0) & 1) << 5 |
                (ushort)(SUB161(auVar15 >> 0x37,0) & 1) << 6 |
                (ushort)(SUB161(auVar15 >> 0x3f,0) & 1) << 7 |
                (ushort)(SUB161(auVar15 >> 0x47,0) & 1) << 8 |
                (ushort)(SUB161(auVar15 >> 0x4f,0) & 1) << 9 |
                (ushort)(SUB161(auVar15 >> 0x57,0) & 1) << 10 |
                (ushort)(SUB161(auVar15 >> 0x5f,0) & 1) << 0xb |
                (ushort)(SUB161(auVar15 >> 0x67,0) & 1) << 0xc |
                (ushort)(SUB161(auVar15 >> 0x6f,0) & 1) << 0xd |
                (ushort)(SUB161(auVar15 >> 0x77,0) & 1) << 0xe | (ushort)(auVar15[0xf] >> 7) << 0xf;
        uVar8 = (uint)uVar7;
        while (uVar7 != 0) {
          uVar5 = 0;
          for (uVar6 = uVar8; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
            uVar5 = uVar5 + 1;
          }
          uVar9 = uVar5 + uVar10 & uVar4;
          cVar2 = *(char *)(lVar3 + -0x48 + uVar9 * -0x48);
          if (((cVar2 != '\x1a') && (*in_RDX == cVar2)) &&
             (in_RDX[1] == *(char *)(lVar3 + uVar9 * -0x48 + -0x47))) {
            return;
          }
          uVar7 = (ushort)(uVar8 - 1) & (ushort)uVar8;
          uVar8 = CONCAT22((short)(uVar8 - 1 >> 0x10),uVar7);
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
        if ((((((((((((((((SUB161(auVar20 >> 7,0) & 1) != 0 || (SUB161(auVar20 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar20 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar20 >> 0x1f,0) & 1) != 0) || (SUB161(auVar20 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar20 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar20 >> 0x37,0) & 1) != 0) || (SUB161(auVar20 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar20 >> 0x47,0) & 1) != 0) || (SUB161(auVar20 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar20 >> 0x57,0) & 1) != 0) || (SUB161(auVar20 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar20 >> 0x67,0) & 1) != 0) || (SUB161(auVar20 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar20 >> 0x77,0) & 1) != 0) || auVar20[0xf] < '\0') break;
        lVar11 = uVar10 + lVar12;
        lVar12 = lVar12 + 0x10;
        uVar10 = lVar11 + 0x10U & uVar4;
      }
    }
  }
  return;
}


