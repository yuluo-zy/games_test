// ADDRESS: 0x14093c970
// SIZE: 282
// SIGNATURE: void __cdecl get_handle_and_texture_by_name(void)

void __cdecl
country_core::asset_libraries::texture_library::AssetTextureLibrary::get_handle_and_texture_by_name
          (void)

{
  char *pcVar1;
  longlong lVar2;
  ulonglong uVar3;
  uint uVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  char cVar9;
  char cVar10;
  char cVar11;
  char cVar12;
  char cVar13;
  char cVar14;
  char cVar15;
  char cVar16;
  char cVar17;
  char cVar18;
  char cVar19;
  char cVar20;
  char cVar21;
  char cVar22;
  char cVar23;
  char cVar24;
  char extraout_AL;
  ushort uVar25;
  longlong in_RCX;
  longlong lVar26;
  ulonglong in_RDX;
  uint uVar27;
  ulonglong uVar28;
  byte bVar29;
  undefined1 auVar30 [16];
  char cVar31;
  char cVar32;
  char cVar33;
  char cVar34;
  undefined1 unaff_XMM7_Ba;
  undefined1 unaff_XMM7_Bb;
  undefined1 unaff_XMM7_Bc;
  undefined1 unaff_XMM7_Bd;
  undefined1 unaff_XMM7_Be;
  undefined1 unaff_XMM7_Bf;
  undefined1 unaff_XMM7_Bg;
  undefined1 unaff_XMM7_Bh;
  undefined1 unaff_XMM7_Bi;
  undefined1 unaff_XMM7_Bj;
  undefined1 unaff_XMM7_Bk;
  undefined1 unaff_XMM7_Bl;
  undefined1 unaff_XMM7_Bm;
  undefined1 unaff_XMM7_Bn;
  undefined1 unaff_XMM7_Bo;
  undefined1 unaff_XMM7_Bp;
  undefined1 auVar35 [16];
  
  if (*(longlong *)(in_RCX + 0x48) != 0) {
    lVar2 = *(longlong *)(in_RCX + 0x30);
    uVar3 = *(ulonglong *)(in_RCX + 0x38);
    bVar29 = (byte)(in_RDX >> 0x39);
    auVar30 = ZEXT216(CONCAT11(bVar29,bVar29));
    auVar30 = pshuflw(auVar30,auVar30,0);
    cVar31 = auVar30[0];
    cVar32 = auVar30[1];
    cVar33 = auVar30[2];
    cVar34 = auVar30[3];
    iVar5 = -(uint)(CONCAT13(unaff_XMM7_Bd,
                             CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))) ==
                   CONCAT13(unaff_XMM7_Bd,
                            CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))));
    iVar6 = -(uint)(CONCAT13(unaff_XMM7_Bh,
                             CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))) ==
                   CONCAT13(unaff_XMM7_Bh,
                            CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))));
    iVar7 = -(uint)(CONCAT13(unaff_XMM7_Bl,
                             CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))) ==
                   CONCAT13(unaff_XMM7_Bl,
                            CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))));
    iVar8 = -(uint)(CONCAT13(unaff_XMM7_Bp,
                             CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))) ==
                   CONCAT13(unaff_XMM7_Bp,
                            CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))));
    lVar26 = 0;
    while( true ) {
      uVar28 = uVar3 & in_RDX;
      pcVar1 = (char *)(lVar2 + uVar28);
      cVar9 = *pcVar1;
      cVar10 = pcVar1[1];
      cVar11 = pcVar1[2];
      cVar12 = pcVar1[3];
      cVar13 = pcVar1[4];
      cVar14 = pcVar1[5];
      cVar15 = pcVar1[6];
      cVar16 = pcVar1[7];
      cVar17 = pcVar1[8];
      cVar18 = pcVar1[9];
      cVar19 = pcVar1[10];
      cVar20 = pcVar1[0xb];
      cVar21 = pcVar1[0xc];
      cVar22 = pcVar1[0xd];
      cVar23 = pcVar1[0xe];
      cVar24 = pcVar1[0xf];
      auVar30[0] = -(cVar9 == cVar31);
      auVar30[1] = -(cVar10 == cVar32);
      auVar30[2] = -(cVar11 == cVar33);
      auVar30[3] = -(cVar12 == cVar34);
      auVar30[4] = -(cVar13 == cVar31);
      auVar30[5] = -(cVar14 == cVar32);
      auVar30[6] = -(cVar15 == cVar33);
      auVar30[7] = -(cVar16 == cVar34);
      auVar30[8] = -(cVar17 == cVar31);
      auVar30[9] = -(cVar18 == cVar32);
      auVar30[10] = -(cVar19 == cVar33);
      auVar30[0xb] = -(cVar20 == cVar34);
      auVar30[0xc] = -(cVar21 == cVar31);
      auVar30[0xd] = -(cVar22 == cVar32);
      auVar30[0xe] = -(cVar23 == cVar33);
      auVar30[0xf] = -(cVar24 == cVar34);
      uVar25 = (ushort)(SUB161(auVar30 >> 7,0) & 1) | (ushort)(SUB161(auVar30 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar30 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar30 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar30 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar30 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar30 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar30 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar30 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar30 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar30 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar30 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar30 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar30 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar30 >> 0x77,0) & 1) << 0xe | (ushort)(auVar30[0xf] >> 7) << 0xf;
      uVar27 = (uint)uVar25;
      while (uVar4 = uVar27, uVar25 != 0) {
        for (; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
        }
        ::utils::name_hash::impl$2::eq();
        if (extraout_AL != '\0') {
          return;
        }
        uVar25 = (ushort)(uVar27 - 1) & (ushort)uVar27;
        uVar27 = CONCAT22((short)(uVar27 - 1 >> 0x10),uVar25);
      }
      auVar35[0] = -(cVar9 == (char)iVar5);
      auVar35[1] = -(cVar10 == (char)((uint)iVar5 >> 8));
      auVar35[2] = -(cVar11 == (char)((uint)iVar5 >> 0x10));
      auVar35[3] = -(cVar12 == (char)((uint)iVar5 >> 0x18));
      auVar35[4] = -(cVar13 == (char)iVar6);
      auVar35[5] = -(cVar14 == (char)((uint)iVar6 >> 8));
      auVar35[6] = -(cVar15 == (char)((uint)iVar6 >> 0x10));
      auVar35[7] = -(cVar16 == (char)((uint)iVar6 >> 0x18));
      auVar35[8] = -(cVar17 == (char)iVar7);
      auVar35[9] = -(cVar18 == (char)((uint)iVar7 >> 8));
      auVar35[10] = -(cVar19 == (char)((uint)iVar7 >> 0x10));
      auVar35[0xb] = -(cVar20 == (char)((uint)iVar7 >> 0x18));
      auVar35[0xc] = -(cVar21 == (char)iVar8);
      auVar35[0xd] = -(cVar22 == (char)((uint)iVar8 >> 8));
      auVar35[0xe] = -(cVar23 == (char)((uint)iVar8 >> 0x10));
      auVar35[0xf] = -(cVar24 == (char)((uint)iVar8 >> 0x18));
      if ((((((((((((((((SUB161(auVar35 >> 7,0) & 1) != 0 || (SUB161(auVar35 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar35 >> 0x17,0) & 1) != 0) || (SUB161(auVar35 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar35 >> 0x27,0) & 1) != 0) || (SUB161(auVar35 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar35 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar35 >> 0x3f,0) & 1) != 0) || (SUB161(auVar35 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar35 >> 0x4f,0) & 1) != 0) || (SUB161(auVar35 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar35 >> 0x5f,0) & 1) != 0) || (SUB161(auVar35 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar35 >> 0x6f,0) & 1) != 0) || (SUB161(auVar35 >> 0x77,0) & 1) != 0) ||
          auVar35[0xf] < '\0') break;
      in_RDX = uVar28 + lVar26 + 0x10;
      lVar26 = lVar26 + 0x10;
    }
  }
  return;
}

