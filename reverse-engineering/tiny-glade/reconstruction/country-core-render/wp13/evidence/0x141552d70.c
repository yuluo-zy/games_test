// name: get<ref$<asset::prefab::PrefabName>_> size: 511
// void __cdecl get<ref$<asset::prefab::PrefabName>_>(void)

void __cdecl
country_core::resources::prefab::PrefabLibrary::get<ref_<asset::prefab::PrefabName>_>(void)

{
  char *pcVar1;
  uint uVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  char cVar7;
  char cVar8;
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
  ushort uVar23;
  int iVar24;
  void *_Buf1;
  ulonglong extraout_RAX;
  void *_Buf2;
  undefined8 *in_RCX;
  undefined1 *puVar25;
  size_t in_RDX;
  size_t sVar26;
  ulonglong uVar27;
  ulonglong uVar28;
  uint uVar29;
  undefined1 (*pauVar30) [16];
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
  undefined1 local_110 [24];
  undefined1 *local_f8;
  code *local_f0;
  undefined1 *local_e8;
  code *local_e0;
  undefined1 local_d8 [16];
  undefined1 (*local_c8) [16];
  undefined1 (*local_c0) [16];
  undefined1 **local_b8;
  undefined8 local_b0;
  longlong local_a8;
  longlong local_98;
  undefined1 *local_90;
  size_t local_88;
  undefined8 local_80;
  
  local_80 = 0xfffffffffffffffe;
  asset::prefab::PrefabName::as_name_ref();
  local_a8 = in_RCX[3];
  if (local_a8 == 0) {
    pauVar30 = (undefined1 (*) [16])*in_RCX;
    uVar27 = in_RCX[1];
  }
  else {
    core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref_<str_>_>();
    pauVar30 = (undefined1 (*) [16])*in_RCX;
    uVar27 = in_RCX[1];
    bVar31 = (byte)(extraout_RAX >> 0x39);
    auVar32 = ZEXT216(CONCAT11(bVar31,bVar31));
    auVar32 = pshuflw(auVar32,auVar32,0);
    cVar33 = auVar32[0];
    cVar34 = auVar32[1];
    cVar35 = auVar32[2];
    cVar36 = auVar32[3];
    puVar25 = pauVar30[-3] + 8;
    sVar26 = 0;
    iVar3 = -(uint)(CONCAT13(unaff_XMM7_Bd,
                             CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))) ==
                   CONCAT13(unaff_XMM7_Bd,
                            CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))));
    iVar4 = -(uint)(CONCAT13(unaff_XMM7_Bh,
                             CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))) ==
                   CONCAT13(unaff_XMM7_Bh,
                            CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))));
    iVar5 = -(uint)(CONCAT13(unaff_XMM7_Bl,
                             CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))) ==
                   CONCAT13(unaff_XMM7_Bl,
                            CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))));
    iVar6 = -(uint)(CONCAT13(unaff_XMM7_Bp,
                             CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))) ==
                   CONCAT13(unaff_XMM7_Bp,
                            CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))));
    uVar28 = extraout_RAX;
    while( true ) {
      pcVar1 = *pauVar30 + (uVar27 & uVar28);
      cVar7 = *pcVar1;
      cVar8 = pcVar1[1];
      cVar9 = pcVar1[2];
      cVar10 = pcVar1[3];
      cVar11 = pcVar1[4];
      cVar12 = pcVar1[5];
      cVar13 = pcVar1[6];
      cVar14 = pcVar1[7];
      cVar15 = pcVar1[8];
      cVar16 = pcVar1[9];
      cVar17 = pcVar1[10];
      cVar18 = pcVar1[0xb];
      cVar19 = pcVar1[0xc];
      cVar20 = pcVar1[0xd];
      cVar21 = pcVar1[0xe];
      cVar22 = pcVar1[0xf];
      auVar32[0] = -(cVar7 == cVar33);
      auVar32[1] = -(cVar8 == cVar34);
      auVar32[2] = -(cVar9 == cVar35);
      auVar32[3] = -(cVar10 == cVar36);
      auVar32[4] = -(cVar11 == cVar33);
      auVar32[5] = -(cVar12 == cVar34);
      auVar32[6] = -(cVar13 == cVar35);
      auVar32[7] = -(cVar14 == cVar36);
      auVar32[8] = -(cVar15 == cVar33);
      auVar32[9] = -(cVar16 == cVar34);
      auVar32[10] = -(cVar17 == cVar35);
      auVar32[0xb] = -(cVar18 == cVar36);
      auVar32[0xc] = -(cVar19 == cVar33);
      auVar32[0xd] = -(cVar20 == cVar34);
      auVar32[0xe] = -(cVar21 == cVar35);
      auVar32[0xf] = -(cVar22 == cVar36);
      uVar23 = (ushort)(SUB161(auVar32 >> 7,0) & 1) | (ushort)(SUB161(auVar32 >> 0xf,0) & 1) << 1 |
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
      uVar29 = (uint)uVar23;
      while (uVar2 = uVar29, uVar23 != 0) {
        for (; (uVar2 & 1) == 0; uVar2 = uVar2 >> 1 | 0x80000000) {
        }
        local_98 = local_a8;
        local_90 = puVar25;
        local_88 = sVar26;
        asset::prefab::PrefabName::as_name_ref();
        if ((sVar26 == in_RDX) && (iVar24 = memcmp(_Buf1,_Buf2,in_RDX), iVar24 == 0)) {
          return;
        }
        uVar23 = (ushort)(uVar29 - 1) & (ushort)uVar29;
        uVar29 = CONCAT22((short)(uVar29 - 1 >> 0x10),uVar23);
        puVar25 = local_90;
        sVar26 = local_88;
        local_a8 = local_98;
      }
      auVar37[0] = -(cVar7 == (char)iVar3);
      auVar37[1] = -(cVar8 == (char)((uint)iVar3 >> 8));
      auVar37[2] = -(cVar9 == (char)((uint)iVar3 >> 0x10));
      auVar37[3] = -(cVar10 == (char)((uint)iVar3 >> 0x18));
      auVar37[4] = -(cVar11 == (char)iVar4);
      auVar37[5] = -(cVar12 == (char)((uint)iVar4 >> 8));
      auVar37[6] = -(cVar13 == (char)((uint)iVar4 >> 0x10));
      auVar37[7] = -(cVar14 == (char)((uint)iVar4 >> 0x18));
      auVar37[8] = -(cVar15 == (char)iVar5);
      auVar37[9] = -(cVar16 == (char)((uint)iVar5 >> 8));
      auVar37[10] = -(cVar17 == (char)((uint)iVar5 >> 0x10));
      auVar37[0xb] = -(cVar18 == (char)((uint)iVar5 >> 0x18));
      auVar37[0xc] = -(cVar19 == (char)iVar6);
      auVar37[0xd] = -(cVar20 == (char)((uint)iVar6 >> 8));
      auVar37[0xe] = -(cVar21 == (char)((uint)iVar6 >> 0x10));
      auVar37[0xf] = -(cVar22 == (char)((uint)iVar6 >> 0x18));
      if ((((((((((((((((SUB161(auVar37 >> 7,0) & 1) != 0 || (SUB161(auVar37 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar37 >> 0x17,0) & 1) != 0) || (SUB161(auVar37 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar37 >> 0x27,0) & 1) != 0) || (SUB161(auVar37 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar37 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar37 >> 0x3f,0) & 1) != 0) || (SUB161(auVar37 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar37 >> 0x4f,0) & 1) != 0) || (SUB161(auVar37 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar37 >> 0x5f,0) & 1) != 0) || (SUB161(auVar37 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar37 >> 0x6f,0) & 1) != 0) || (SUB161(auVar37 >> 0x77,0) & 1) != 0) ||
          auVar37[0xf] < '\0') break;
      uVar28 = (uVar27 & uVar28) + sVar26 + 0x10;
      sVar26 = sVar26 + 0x10;
    }
  }
  local_b8 = (undefined1 **)(*pauVar30 + uVar27 + 1);
  auVar32 = *pauVar30;
  local_b0 = CONCAT62(local_b0._2_6_,
                      ~((ushort)(SUB161(auVar32 >> 7,0) & 1) |
                        (ushort)(SUB161(auVar32 >> 0xf,0) & 1) << 1 |
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
                        (ushort)(SUB161(auVar32 >> 0x77,0) & 1) << 0xe |
                       (ushort)(byte)(auVar32[0xf] >> 7) << 0xf));
  local_c0 = pauVar30 + 1;
  local_c8 = pauVar30;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<ref_<asset::prefab::PrefabName>,std::collections::hash::map::Keys<asset::prefab::PrefabName,country_core::resources::prefab::Prefab>_>
            ();
  local_f8 = local_d8;
  local_f0 = core::fmt::impl$74::fmt<asset::prefab::PrefabNameRef>;
  local_e0 = alloc::vec::impl$27::fmt<ref_<asset::prefab::PrefabName>,alloc::alloc::Global>;
  local_c8 = (undefined1 (*) [16])&anon_07bca3d3681a8122a663d14186d4696c_44_llvm_6185930617003689610
  ;
  local_c0 = (undefined1 (*) [16])0x2;
  local_a8 = 0;
  local_b8 = &local_f8;
  local_b0 = 2;
  local_e8 = local_110;
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}


