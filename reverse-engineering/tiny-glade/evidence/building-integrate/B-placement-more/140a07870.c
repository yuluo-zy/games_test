
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::render::mesh::Mesh::aabb_bounds(void)

{
  char *pcVar1;
  undefined8 uVar2;
  longlong *plVar3;
  undefined8 *puVar4;
  uint uVar5;
  uint uVar6;
  ulonglong extraout_RAX;
  undefined8 *in_RCX;
  longlong in_RDX;
  ushort uVar7;
  longlong lVar8;
  uint uVar9;
  ulonglong uVar10;
  longlong lVar11;
  longlong lVar12;
  byte bVar13;
  char cVar14;
  undefined8 uVar15;
  char cVar17;
  char cVar18;
  char cVar19;
  undefined1 auVar16 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined8 uStack_60;
  
  if (*(longlong *)(in_RDX + 0x50) != 0) {
    core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref_<str_>_>();
    lVar12 = *(longlong *)(in_RDX + 0x38);
    bVar13 = (byte)(extraout_RAX >> 0x39);
    auVar16 = ZEXT216(CONCAT11(bVar13,bVar13));
    auVar16 = pshuflw(auVar16,auVar16,0);
    lVar11 = 0;
    uVar10 = extraout_RAX;
    while( true ) {
      uVar10 = *(ulonglong *)(in_RDX + 0x40) & uVar10;
      pcVar1 = (char *)(lVar12 + uVar10);
      cVar14 = auVar16[0];
      auVar21[0] = -(*pcVar1 == cVar14);
      cVar17 = auVar16[1];
      auVar21[1] = -(pcVar1[1] == cVar17);
      cVar18 = auVar16[2];
      auVar21[2] = -(pcVar1[2] == cVar18);
      cVar19 = auVar16[3];
      auVar21[3] = -(pcVar1[3] == cVar19);
      auVar21[4] = -(pcVar1[4] == cVar14);
      auVar21[5] = -(pcVar1[5] == cVar17);
      auVar21[6] = -(pcVar1[6] == cVar18);
      auVar21[7] = -(pcVar1[7] == cVar19);
      auVar21[8] = -(pcVar1[8] == cVar14);
      auVar21[9] = -(pcVar1[9] == cVar17);
      auVar21[10] = -(pcVar1[10] == cVar18);
      auVar21[0xb] = -(pcVar1[0xb] == cVar19);
      auVar21[0xc] = -(pcVar1[0xc] == cVar14);
      auVar21[0xd] = -(pcVar1[0xd] == cVar17);
      auVar21[0xe] = -(pcVar1[0xe] == cVar18);
      auVar21[0xf] = -(pcVar1[0xf] == cVar19);
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
      uVar9 = (uint)uVar7;
      while (uVar7 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar9; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        lVar8 = (uVar5 + uVar10 & *(ulonglong *)(in_RDX + 0x40)) * -0x38;
        if ((*(longlong *)(lVar12 + -0x28 + lVar8) == 0xf) &&
           (plVar3 = *(longlong **)(lVar12 + lVar8 + -0x30),
           *(longlong *)((longlong)plVar3 + 7) == 0x6e6f697469736f50 &&
           *plVar3 == 0x505f786574726556)) {
          if (*(int *)(lVar12 + -0x20 + lVar8) == 3) {
            lVar11 = *(longlong *)(lVar12 + lVar8 + -8);
            if (lVar11 == 0) {
              in_RCX[2] = DAT_142c774c0;
              uVar15 = _anon_369a2eef05c89d4ad2f1229306672ae6_22_llvm_6346813648557899910;
              uStack_60 = _UNK_142c774b8;
            }
            else {
              puVar4 = *(undefined8 **)(lVar12 + lVar8 + -0x10);
              uVar2 = *(undefined8 *)((longlong)puVar4 + 4);
              uVar15 = *puVar4;
              uStack_60 = CONCAT44(*(undefined4 *)puVar4,(int)((ulonglong)uVar2 >> 0x20));
              if (lVar11 != 1) {
                lVar12 = 0;
                do {
                  ::utils::geometry::aabb::Aabb3::expand_to_contain_point();
                  lVar12 = lVar12 + 0xc;
                } while (lVar11 * 0xc + -0xc != lVar12);
              }
              in_RCX[2] = uVar2;
            }
            *in_RCX = uVar15;
            in_RCX[1] = uStack_60;
            return;
          }
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_fmt();
        }
        uVar7 = (ushort)(uVar9 - 1) & (ushort)uVar9;
        uVar9 = CONCAT22((short)(uVar9 - 1 >> 0x10),uVar7);
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
      uVar10 = uVar10 + lVar11 + 0x10;
      lVar11 = lVar11 + 0x10;
    }
  }
                    /* WARNING: Subroutine does not return */
  core::option::unwrap_failed();
}

