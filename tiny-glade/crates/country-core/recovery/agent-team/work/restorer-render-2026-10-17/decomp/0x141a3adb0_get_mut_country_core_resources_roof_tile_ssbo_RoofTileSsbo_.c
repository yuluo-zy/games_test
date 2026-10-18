// 0x141a3adb0 get_mut<country_core::resources::roof_tile_ssbo::RoofTileSsbo> size=455

void __cdecl
country_core::asset_libraries::ssbo_library::AssetSsboLibrary::
get_mut<country_core::resources::roof_tile_ssbo::RoofTileSsbo>(void)

{
  char *pcVar1;
  longlong lVar2;
  ulonglong uVar3;
  undefined8 *puVar4;
  uint uVar5;
  uint uVar6;
  int iVar7;
  int iVar8;
  int iVar9;
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
  char cVar25;
  char extraout_AL;
  ushort uVar26;
  int iVar27;
  longlong in_RCX;
  longlong lVar28;
  uint uVar29;
  ulonglong uVar30;
  longlong lVar31;
  undefined1 auVar32 [16];
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
  undefined1 auVar33 [16];
  
  if (*(char *)(in_RCX + 8) != '\0') {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  if (*(longlong *)(in_RCX + 0x28) != 0) {
    lVar2 = *(longlong *)(in_RCX + 0x10);
    uVar3 = *(ulonglong *)(in_RCX + 0x18);
    uVar30 = uVar3 & 0x13d939590d705099;
    iVar27 = -(uint)(CONCAT13(unaff_XMM7_Bd,
                              CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))) ==
                    CONCAT13(unaff_XMM7_Bd,
                             CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))));
    iVar7 = -(uint)(CONCAT13(unaff_XMM7_Bh,
                             CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))) ==
                   CONCAT13(unaff_XMM7_Bh,
                            CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))));
    iVar8 = -(uint)(CONCAT13(unaff_XMM7_Bl,
                             CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))) ==
                   CONCAT13(unaff_XMM7_Bl,
                            CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))));
    iVar9 = -(uint)(CONCAT13(unaff_XMM7_Bp,
                             CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))) ==
                   CONCAT13(unaff_XMM7_Bp,
                            CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))));
    lVar28 = 0;
    while( true ) {
      pcVar1 = (char *)(lVar2 + uVar30);
      cVar10 = *pcVar1;
      cVar11 = pcVar1[1];
      cVar12 = pcVar1[2];
      cVar13 = pcVar1[3];
      cVar14 = pcVar1[4];
      cVar15 = pcVar1[5];
      cVar16 = pcVar1[6];
      cVar17 = pcVar1[7];
      cVar18 = pcVar1[8];
      cVar19 = pcVar1[9];
      cVar20 = pcVar1[10];
      cVar21 = pcVar1[0xb];
      cVar22 = pcVar1[0xc];
      cVar23 = pcVar1[0xd];
      cVar24 = pcVar1[0xe];
      cVar25 = pcVar1[0xf];
      auVar32[0] = -(cVar10 == '\t');
      auVar32[1] = -(cVar11 == '\t');
      auVar32[2] = -(cVar12 == '\t');
      auVar32[3] = -(cVar13 == '\t');
      auVar32[4] = -(cVar14 == '\t');
      auVar32[5] = -(cVar15 == '\t');
      auVar32[6] = -(cVar16 == '\t');
      auVar32[7] = -(cVar17 == '\t');
      auVar32[8] = -(cVar18 == '\t');
      auVar32[9] = -(cVar19 == '\t');
      auVar32[10] = -(cVar20 == '\t');
      auVar32[0xb] = -(cVar21 == '\t');
      auVar32[0xc] = -(cVar22 == '\t');
      auVar32[0xd] = -(cVar23 == '\t');
      auVar32[0xe] = -(cVar24 == '\t');
      auVar32[0xf] = -(cVar25 == '\t');
      uVar26 = (ushort)(SUB161(auVar32 >> 7,0) & 1) | (ushort)(SUB161(auVar32 >> 0xf,0) & 1) << 1 |
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
      uVar29 = (uint)uVar26;
      while (uVar26 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar29; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        ::utils::name_hash::impl$2::eq();
        if (extraout_AL != '\0') {
          puVar4 = *(undefined8 **)(lVar2 + -8 + (uVar5 + uVar30 & uVar3) * -0x10);
          if ((puVar4[3] == 0x35) &&
             (iVar27 = memcmp(&anon_c4dc3b5cd97938b42123e838db8de09d_1_llvm_3856994597783109560,
                              (void *)puVar4[2],0x35), iVar27 == 0)) {
            (**(code **)(puVar4[1] + 0x20))(*puVar4);
            return;
          }
                    // WARNING: Subroutine does not return
          core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
        }
        uVar26 = (ushort)(uVar29 - 1) & (ushort)uVar29;
        uVar29 = CONCAT22((short)(uVar29 - 1 >> 0x10),uVar26);
      }
      auVar33[0] = -(cVar10 == (char)iVar27);
      auVar33[1] = -(cVar11 == (char)((uint)iVar27 >> 8));
      auVar33[2] = -(cVar12 == (char)((uint)iVar27 >> 0x10));
      auVar33[3] = -(cVar13 == (char)((uint)iVar27 >> 0x18));
      auVar33[4] = -(cVar14 == (char)iVar7);
      auVar33[5] = -(cVar15 == (char)((uint)iVar7 >> 8));
      auVar33[6] = -(cVar16 == (char)((uint)iVar7 >> 0x10));
      auVar33[7] = -(cVar17 == (char)((uint)iVar7 >> 0x18));
      auVar33[8] = -(cVar18 == (char)iVar8);
      auVar33[9] = -(cVar19 == (char)((uint)iVar8 >> 8));
      auVar33[10] = -(cVar20 == (char)((uint)iVar8 >> 0x10));
      auVar33[0xb] = -(cVar21 == (char)((uint)iVar8 >> 0x18));
      auVar33[0xc] = -(cVar22 == (char)iVar9);
      auVar33[0xd] = -(cVar23 == (char)((uint)iVar9 >> 8));
      auVar33[0xe] = -(cVar24 == (char)((uint)iVar9 >> 0x10));
      auVar33[0xf] = -(cVar25 == (char)((uint)iVar9 >> 0x18));
      if ((((((((((((((((SUB161(auVar33 >> 7,0) & 1) != 0 || (SUB161(auVar33 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar33 >> 0x17,0) & 1) != 0) || (SUB161(auVar33 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar33 >> 0x27,0) & 1) != 0) || (SUB161(auVar33 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar33 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar33 >> 0x3f,0) & 1) != 0) || (SUB161(auVar33 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar33 >> 0x4f,0) & 1) != 0) || (SUB161(auVar33 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar33 >> 0x5f,0) & 1) != 0) || (SUB161(auVar33 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar33 >> 0x6f,0) & 1) != 0) || (SUB161(auVar33 >> 0x77,0) & 1) != 0) ||
          auVar33[0xf] < '\0') break;
      lVar31 = uVar30 + lVar28;
      lVar28 = lVar28 + 0x10;
      uVar30 = lVar31 + 0x10U & uVar3;
    }
  }
  return;
}


// callees: [{"name": "assert_failed<ref$<str$>,ref$<str$>_>", "address": "1428cab41"}, {"name": "eq", "address": "140c9cd70"}, {"name": "memcmp", "address": "142923bf0"}, {"name": "unwrap_failed", "address": "1428d9760"}]
