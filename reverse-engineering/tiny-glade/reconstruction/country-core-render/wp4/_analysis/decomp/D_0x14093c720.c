// ADDRESS: 0x14093c720
// SIZE: 579
// SIGNATURE: void __cdecl create_name_mapping(void)

void __cdecl
country_core::asset_libraries::texture_library::AssetTextureLibrary::create_name_mapping(void)

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
  longlong lVar25;
  char extraout_AL;
  byte extraout_AL_00;
  ushort uVar26;
  ulonglong extraout_RAX;
  longlong in_RCX;
  longlong lVar27;
  uint uVar28;
  ulonglong uVar29;
  byte bVar30;
  undefined1 auVar31 [16];
  char cVar32;
  char cVar33;
  char cVar34;
  char cVar35;
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
  undefined1 auVar36 [16];
  undefined1 local_f8 [16];
  longlong local_e8;
  undefined **local_d8;
  undefined8 local_d0;
  undefined1 **local_c8;
  undefined8 local_c0;
  undefined8 local_b8;
  undefined1 *local_a8;
  code *local_a0;
  undefined1 *local_98;
  code *local_90;
  undefined1 local_88 [72];
  
  ::utils::name_hash::NameHash::compute();
  lVar25 = local_e8;
  if (*(longlong *)(in_RCX + 0x48) != 0) {
    lVar2 = *(longlong *)(in_RCX + 0x30);
    uVar3 = *(ulonglong *)(in_RCX + 0x38);
    bVar30 = (byte)(extraout_RAX >> 0x39);
    auVar31 = ZEXT216(CONCAT11(bVar30,bVar30));
    auVar31 = pshuflw(auVar31,auVar31,0);
    cVar32 = auVar31[0];
    cVar33 = auVar31[1];
    cVar34 = auVar31[2];
    cVar35 = auVar31[3];
    lVar27 = 0;
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
    uVar29 = extraout_RAX;
    while( true ) {
      uVar29 = uVar3 & uVar29;
      pcVar1 = (char *)(lVar2 + uVar29);
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
      auVar31[0] = -(cVar9 == cVar32);
      auVar31[1] = -(cVar10 == cVar33);
      auVar31[2] = -(cVar11 == cVar34);
      auVar31[3] = -(cVar12 == cVar35);
      auVar31[4] = -(cVar13 == cVar32);
      auVar31[5] = -(cVar14 == cVar33);
      auVar31[6] = -(cVar15 == cVar34);
      auVar31[7] = -(cVar16 == cVar35);
      auVar31[8] = -(cVar17 == cVar32);
      auVar31[9] = -(cVar18 == cVar33);
      auVar31[10] = -(cVar19 == cVar34);
      auVar31[0xb] = -(cVar20 == cVar35);
      auVar31[0xc] = -(cVar21 == cVar32);
      auVar31[0xd] = -(cVar22 == cVar33);
      auVar31[0xe] = -(cVar23 == cVar34);
      auVar31[0xf] = -(cVar24 == cVar35);
      uVar26 = (ushort)(SUB161(auVar31 >> 7,0) & 1) | (ushort)(SUB161(auVar31 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar31 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar31 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar31 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar31 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar31 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar31 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar31 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar31 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar31 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar31 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar31 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar31 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar31 >> 0x77,0) & 1) << 0xe | (ushort)(auVar31[0xf] >> 7) << 0xf;
      uVar28 = (uint)uVar26;
      lVar25 = local_e8;
      while (local_e8 = lVar27, uVar4 = uVar28, uVar26 != 0) {
        for (; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
        }
        ::utils::name_hash::impl$2::eq();
        if (extraout_AL != '\0') {
          ::utils::name_hash::NameHash::compute();
          bVar30 = extraout_AL_00;
          hashbrown::map::HashMap::
          insert<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash>_>,alloc::alloc::Global>
                    ();
          if ((bVar30 != 2) && ((bVar30 & 1) == 0)) {
            local_a8 = local_f8;
            local_a0 = core::fmt::impl$74::fmt<str_>;
            local_d8 = &PTR_s_Onoz__already_exists__and_is_a_r_142ab1d68;
            local_d0 = 2;
            local_b8 = 0;
            local_c8 = &local_a8;
            local_c0 = 1;
                    // WARNING: Subroutine does not return
            core::panicking::panic_fmt();
          }
          return;
        }
        uVar26 = (ushort)(uVar28 - 1) & (ushort)uVar28;
        uVar28 = CONCAT22((short)(uVar28 - 1 >> 0x10),uVar26);
        lVar27 = local_e8;
        lVar25 = local_e8;
      }
      auVar36[0] = -(cVar9 == (char)iVar5);
      auVar36[1] = -(cVar10 == (char)((uint)iVar5 >> 8));
      auVar36[2] = -(cVar11 == (char)((uint)iVar5 >> 0x10));
      auVar36[3] = -(cVar12 == (char)((uint)iVar5 >> 0x18));
      auVar36[4] = -(cVar13 == (char)iVar6);
      auVar36[5] = -(cVar14 == (char)((uint)iVar6 >> 8));
      auVar36[6] = -(cVar15 == (char)((uint)iVar6 >> 0x10));
      auVar36[7] = -(cVar16 == (char)((uint)iVar6 >> 0x18));
      auVar36[8] = -(cVar17 == (char)iVar7);
      auVar36[9] = -(cVar18 == (char)((uint)iVar7 >> 8));
      auVar36[10] = -(cVar19 == (char)((uint)iVar7 >> 0x10));
      auVar36[0xb] = -(cVar20 == (char)((uint)iVar7 >> 0x18));
      auVar36[0xc] = -(cVar21 == (char)iVar8);
      auVar36[0xd] = -(cVar22 == (char)((uint)iVar8 >> 8));
      auVar36[0xe] = -(cVar23 == (char)((uint)iVar8 >> 0x10));
      auVar36[0xf] = -(cVar24 == (char)((uint)iVar8 >> 0x18));
      if ((((((((((((((((SUB161(auVar36 >> 7,0) & 1) != 0 || (SUB161(auVar36 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar36 >> 0x17,0) & 1) != 0) || (SUB161(auVar36 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar36 >> 0x27,0) & 1) != 0) || (SUB161(auVar36 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar36 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar36 >> 0x3f,0) & 1) != 0) || (SUB161(auVar36 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar36 >> 0x4f,0) & 1) != 0) || (SUB161(auVar36 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar36 >> 0x5f,0) & 1) != 0) || (SUB161(auVar36 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar36 >> 0x6f,0) & 1) != 0) || (SUB161(auVar36 >> 0x77,0) & 1) != 0) ||
          auVar36[0xf] < '\0') break;
      uVar29 = uVar29 + local_e8 + 0x10;
      lVar27 = local_e8 + 0x10;
      local_e8 = lVar25;
    }
  }
  local_e8 = lVar25;
  local_a8 = local_f8;
  local_a0 = core::fmt::impl$74::fmt<str_>;
  local_98 = local_88;
  local_90 = core::fmt::impl$74::fmt<str_>;
  local_d8 = &PTR_s_Was_asked_to_map_to___but_the_la_142ab1de8;
  local_d0 = 3;
  local_b8 = 0;
  local_c8 = &local_a8;
  local_c0 = 2;
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}

