// address: 0x141629a20
// pdb name: optional_uniform<f32>
// signature: void __cdecl optional_uniform<f32>(void)

void __cdecl country_core::render::pass_builder::draw_builder::impl$2::optional_uniform<f32>(void)

{
  char *pcVar1;
  longlong lVar2;
  longlong lVar3;
  uint uVar4;
  uint uVar5;
  void *in_RCX;
  void *in_RDX;
  ulonglong uVar6;
  ushort uVar7;
  ulonglong uVar8;
  ulonglong in_R8;
  longlong lVar9;
  uint uVar10;
  byte bVar11;
  char cVar12;
  char cVar14;
  char cVar15;
  char cVar16;
  undefined1 auVar13 [16];
  undefined1 auVar17 [16];
  undefined4 in_XMM3_Da;
  undefined1 auVar18 [16];
  
  if (((*(char *)((longlong)in_RDX + 0xde) != '\b') &&
      (lVar2 = *(longlong *)((longlong)in_RDX + 0xe8), *(longlong *)(lVar2 + 0xe8) != 0)) &&
     (*(longlong *)(lVar2 + 0x130) != 0)) {
    lVar3 = *(longlong *)(lVar2 + 0x118);
    bVar11 = (byte)(in_R8 >> 0x39);
    auVar13 = ZEXT216(CONCAT11(bVar11,bVar11));
    auVar13 = pshuflw(auVar13,auVar13,0);
    lVar9 = 0;
    uVar6 = in_R8;
    while( true ) {
      uVar6 = *(ulonglong *)(lVar2 + 0x120) & uVar6;
      pcVar1 = (char *)(lVar3 + uVar6);
      cVar12 = auVar13[0];
      auVar18[0] = -(*pcVar1 == cVar12);
      cVar14 = auVar13[1];
      auVar18[1] = -(pcVar1[1] == cVar14);
      cVar15 = auVar13[2];
      auVar18[2] = -(pcVar1[2] == cVar15);
      cVar16 = auVar13[3];
      auVar18[3] = -(pcVar1[3] == cVar16);
      auVar18[4] = -(pcVar1[4] == cVar12);
      auVar18[5] = -(pcVar1[5] == cVar14);
      auVar18[6] = -(pcVar1[6] == cVar15);
      auVar18[7] = -(pcVar1[7] == cVar16);
      auVar18[8] = -(pcVar1[8] == cVar12);
      auVar18[9] = -(pcVar1[9] == cVar14);
      auVar18[10] = -(pcVar1[10] == cVar15);
      auVar18[0xb] = -(pcVar1[0xb] == cVar16);
      auVar18[0xc] = -(pcVar1[0xc] == cVar12);
      auVar18[0xd] = -(pcVar1[0xd] == cVar14);
      auVar18[0xe] = -(pcVar1[0xe] == cVar15);
      auVar18[0xf] = -(pcVar1[0xf] == cVar16);
      uVar7 = (ushort)(SUB161(auVar18 >> 7,0) & 1) | (ushort)(SUB161(auVar18 >> 0xf,0) & 1) << 1 |
              (ushort)(SUB161(auVar18 >> 0x17,0) & 1) << 2 |
              (ushort)(SUB161(auVar18 >> 0x1f,0) & 1) << 3 |
              (ushort)(SUB161(auVar18 >> 0x27,0) & 1) << 4 |
              (ushort)(SUB161(auVar18 >> 0x2f,0) & 1) << 5 |
              (ushort)(SUB161(auVar18 >> 0x37,0) & 1) << 6 |
              (ushort)(SUB161(auVar18 >> 0x3f,0) & 1) << 7 |
              (ushort)(SUB161(auVar18 >> 0x47,0) & 1) << 8 |
              (ushort)(SUB161(auVar18 >> 0x4f,0) & 1) << 9 |
              (ushort)(SUB161(auVar18 >> 0x57,0) & 1) << 10 |
              (ushort)(SUB161(auVar18 >> 0x5f,0) & 1) << 0xb |
              (ushort)(SUB161(auVar18 >> 0x67,0) & 1) << 0xc |
              (ushort)(SUB161(auVar18 >> 0x6f,0) & 1) << 0xd |
              (ushort)(SUB161(auVar18 >> 0x77,0) & 1) << 0xe | (ushort)(auVar18[0xf] >> 7) << 0xf;
      uVar10 = (uint)uVar7;
      while (uVar7 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar10; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        uVar8 = uVar4 + uVar6 & *(ulonglong *)(lVar2 + 0x120);
        if (*(ulonglong *)(lVar3 + -0x10 + uVar8 * -0x10) == in_R8) {
          uVar6 = (ulonglong)*(uint *)(lVar3 + -8 + uVar8 * -0x10);
          if (*(ulonglong *)((longlong)in_RDX + 0xc0) < uVar6) {
                    // WARNING: Subroutine does not return
            core::slice::index::slice_start_index_len_fail();
          }
          *(undefined4 *)(*(longlong *)((longlong)in_RDX + 0xb8) + uVar6) = in_XMM3_Da;
          goto LAB_141629b2a;
        }
        uVar7 = (ushort)(uVar10 - 1) & (ushort)uVar10;
        uVar10 = CONCAT22((short)(uVar10 - 1 >> 0x10),uVar7);
      }
      auVar17[0] = -(*pcVar1 == -1);
      auVar17[1] = -(pcVar1[1] == -1);
      auVar17[2] = -(pcVar1[2] == -1);
      auVar17[3] = -(pcVar1[3] == -1);
      auVar17[4] = -(pcVar1[4] == -1);
      auVar17[5] = -(pcVar1[5] == -1);
      auVar17[6] = -(pcVar1[6] == -1);
      auVar17[7] = -(pcVar1[7] == -1);
      auVar17[8] = -(pcVar1[8] == -1);
      auVar17[9] = -(pcVar1[9] == -1);
      auVar17[10] = -(pcVar1[10] == -1);
      auVar17[0xb] = -(pcVar1[0xb] == -1);
      auVar17[0xc] = -(pcVar1[0xc] == -1);
      auVar17[0xd] = -(pcVar1[0xd] == -1);
      auVar17[0xe] = -(pcVar1[0xe] == -1);
      auVar17[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar17 >> 7,0) & 1) != 0 || (SUB161(auVar17 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar17 >> 0x17,0) & 1) != 0) || (SUB161(auVar17 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar17 >> 0x27,0) & 1) != 0) || (SUB161(auVar17 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar17 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar17 >> 0x3f,0) & 1) != 0) || (SUB161(auVar17 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar17 >> 0x4f,0) & 1) != 0) || (SUB161(auVar17 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar17 >> 0x5f,0) & 1) != 0) || (SUB161(auVar17 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar17 >> 0x6f,0) & 1) != 0) || (SUB161(auVar17 >> 0x77,0) & 1) != 0) ||
          auVar17[0xf] < '\0') break;
      uVar6 = uVar6 + lVar9 + 0x10;
      lVar9 = lVar9 + 0x10;
    }
  }
LAB_141629b2a:
  memcpy(in_RCX,in_RDX,0xf8);
  return;
}


