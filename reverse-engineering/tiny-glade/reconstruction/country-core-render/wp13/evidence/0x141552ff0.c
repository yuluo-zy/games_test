// name: has<ref$<asset::prefab::PrefabNameRef>_> size: 292
// void __cdecl has<ref$<asset::prefab::PrefabNameRef>_>(void)

void __cdecl
country_core::resources::prefab::PrefabLibrary::has<ref_<asset::prefab::PrefabNameRef>_>(void)

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
  ushort uVar25;
  int iVar26;
  void *_Buf1;
  ulonglong extraout_RAX;
  void *_Buf2;
  longlong *in_RCX;
  void *pvVar27;
  uint uVar28;
  void *in_R8;
  longlong lVar29;
  ulonglong uVar30;
  byte bVar31;
  undefined1 auVar32 [16];
  char cVar33;
  char cVar34;
  char cVar35;
  char cVar36;
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
  undefined1 auVar37 [16];
  
  ::toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  if (in_RCX[3] != 0) {
    pvVar27 = _Buf1;
    core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref_<str_>_>();
    lVar2 = *in_RCX;
    uVar3 = in_RCX[1];
    bVar31 = (byte)(extraout_RAX >> 0x39);
    auVar32 = ZEXT216(CONCAT11(bVar31,bVar31));
    auVar32 = pshuflw(auVar32,auVar32,0);
    cVar33 = auVar32[0];
    cVar34 = auVar32[1];
    cVar35 = auVar32[2];
    cVar36 = auVar32[3];
    lVar29 = 0;
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
    uVar30 = extraout_RAX;
    while( true ) {
      pcVar1 = (char *)(lVar2 + (uVar3 & uVar30));
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
      auVar32[0] = -(cVar9 == cVar33);
      auVar32[1] = -(cVar10 == cVar34);
      auVar32[2] = -(cVar11 == cVar35);
      auVar32[3] = -(cVar12 == cVar36);
      auVar32[4] = -(cVar13 == cVar33);
      auVar32[5] = -(cVar14 == cVar34);
      auVar32[6] = -(cVar15 == cVar35);
      auVar32[7] = -(cVar16 == cVar36);
      auVar32[8] = -(cVar17 == cVar33);
      auVar32[9] = -(cVar18 == cVar34);
      auVar32[10] = -(cVar19 == cVar35);
      auVar32[0xb] = -(cVar20 == cVar36);
      auVar32[0xc] = -(cVar21 == cVar33);
      auVar32[0xd] = -(cVar22 == cVar34);
      auVar32[0xe] = -(cVar23 == cVar35);
      auVar32[0xf] = -(cVar24 == cVar36);
      uVar25 = (ushort)(SUB161(auVar32 >> 7,0) & 1) | (ushort)(SUB161(auVar32 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar32 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar32 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar32 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar32 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar32 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar32 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar32 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar32 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar32 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar32 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar32 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar32 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar32 >> 0x77,0) & 1) << 0xe | (ushort)(auVar32[0xf] >> 7) << 0xf;
      uVar28 = (uint)uVar25;
      while (uVar4 = uVar28, uVar25 != 0) {
        for (; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
        }
        asset::prefab::PrefabName::as_name_ref();
        if ((pvVar27 == in_R8) &&
           (pvVar27 = _Buf2, iVar26 = memcmp(_Buf1,_Buf2,(size_t)in_R8), iVar26 == 0)) {
          return;
        }
        uVar25 = (ushort)(uVar28 - 1) & (ushort)uVar28;
        uVar28 = CONCAT22((short)(uVar28 - 1 >> 0x10),uVar25);
      }
      auVar37[0] = -(cVar9 == (char)iVar5);
      auVar37[1] = -(cVar10 == (char)((uint)iVar5 >> 8));
      auVar37[2] = -(cVar11 == (char)((uint)iVar5 >> 0x10));
      auVar37[3] = -(cVar12 == (char)((uint)iVar5 >> 0x18));
      auVar37[4] = -(cVar13 == (char)iVar6);
      auVar37[5] = -(cVar14 == (char)((uint)iVar6 >> 8));
      auVar37[6] = -(cVar15 == (char)((uint)iVar6 >> 0x10));
      auVar37[7] = -(cVar16 == (char)((uint)iVar6 >> 0x18));
      auVar37[8] = -(cVar17 == (char)iVar7);
      auVar37[9] = -(cVar18 == (char)((uint)iVar7 >> 8));
      auVar37[10] = -(cVar19 == (char)((uint)iVar7 >> 0x10));
      auVar37[0xb] = -(cVar20 == (char)((uint)iVar7 >> 0x18));
      auVar37[0xc] = -(cVar21 == (char)iVar8);
      auVar37[0xd] = -(cVar22 == (char)((uint)iVar8 >> 8));
      auVar37[0xe] = -(cVar23 == (char)((uint)iVar8 >> 0x10));
      auVar37[0xf] = -(cVar24 == (char)((uint)iVar8 >> 0x18));
      if ((((((((((((((((SUB161(auVar37 >> 7,0) & 1) != 0 || (SUB161(auVar37 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar37 >> 0x17,0) & 1) != 0) || (SUB161(auVar37 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar37 >> 0x27,0) & 1) != 0) || (SUB161(auVar37 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar37 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar37 >> 0x3f,0) & 1) != 0) || (SUB161(auVar37 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar37 >> 0x4f,0) & 1) != 0) || (SUB161(auVar37 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar37 >> 0x5f,0) & 1) != 0) || (SUB161(auVar37 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar37 >> 0x6f,0) & 1) != 0) || (SUB161(auVar37 >> 0x77,0) & 1) != 0) ||
          auVar37[0xf] < '\0') break;
      uVar30 = (uVar3 & uVar30) + lVar29 + 0x10;
      lVar29 = lVar29 + 0x10;
    }
  }
  return;
}


