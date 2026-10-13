// ADDRESS: 0x14093ca90
// SIZE: 547
// SIGNATURE: void __cdecl get_by_name(void)

void __cdecl country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name(void)

{
  char *pcVar1;
  ulonglong uVar2;
  uint uVar3;
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
  ushort uVar26;
  longlong in_RCX;
  ulonglong in_RDX;
  undefined1 (*pauVar27) [16];
  uint uVar28;
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
  undefined1 local_f8 [24];
  undefined1 *local_e0;
  code *local_d8;
  undefined1 *local_d0;
  code *local_c8;
  longlong local_c0;
  undefined1 (*local_b8) [16];
  undefined1 (*local_b0) [16];
  undefined1 **local_a8;
  undefined8 local_a0;
  undefined8 local_98;
  undefined1 local_88 [8];
  undefined8 local_80;
  
  local_80 = 0xfffffffffffffffe;
  pauVar27 = *(undefined1 (**) [16])(in_RCX + 0x30);
  if (*(longlong *)(in_RCX + 0x48) == 0) {
    local_98 = 0;
  }
  else {
    uVar2 = *(ulonglong *)(in_RCX + 0x38);
    bVar31 = (byte)(in_RDX >> 0x39);
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
    while( true ) {
      uVar30 = uVar2 & in_RDX;
      pcVar1 = *pauVar27 + uVar30;
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
      uVar28 = (uint)uVar26;
      lVar25 = local_c0;
      while (local_c0 = lVar29, uVar26 != 0) {
        uVar3 = 0;
        for (uVar4 = uVar28; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
          uVar3 = uVar3 + 1;
        }
        ::utils::name_hash::impl$2::eq();
        if (extraout_AL != '\0') {
          if (*(ulonglong *)((longlong)pauVar27 + (uVar3 + uVar30 & uVar2) * -0x18 + -0x10) <
              *(ulonglong *)(in_RCX + 0x10)) {
            return;
          }
          local_e0 = local_88;
          local_d8 = ::utils::name_hash::impl$7::fmt;
          local_b8 = (undefined1 (*) [16])&PTR_s_Texture_not_loaded__142ab1ea0;
          local_b0 = (undefined1 (*) [16])0x1;
          local_98 = 0;
          local_a8 = &local_e0;
          local_a0 = 1;
                    // WARNING: Subroutine does not return
          core::panicking::panic_fmt();
        }
        uVar26 = (ushort)(uVar28 - 1) & (ushort)uVar28;
        uVar28 = CONCAT22((short)(uVar28 - 1 >> 0x10),uVar26);
        lVar29 = local_c0;
        lVar25 = local_c0;
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
      in_RDX = uVar30 + local_c0 + 0x10;
      lVar29 = local_c0 + 0x10;
      local_c0 = lVar25;
    }
    pauVar27 = *(undefined1 (**) [16])(in_RCX + 0x30);
    local_98 = *(undefined8 *)(in_RCX + 0x48);
    local_c0 = lVar25;
  }
  local_a8 = (undefined1 **)(*pauVar27 + *(longlong *)(in_RCX + 0x38) + 1);
  auVar32 = *pauVar27;
  local_a0 = CONCAT62(local_a0._2_6_,
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
  local_b0 = pauVar27 + 1;
  local_b8 = pauVar27;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<ref_<utils::name_hash::NameHash>,std::collections::hash::map::Keys<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture>_>
            ();
  local_e0 = local_88;
  local_d8 = ::utils::name_hash::impl$7::fmt;
  local_c8 = alloc::vec::impl$27::fmt<ref_<utils::name_hash::NameHash>,alloc::alloc::Global>;
  local_b8 = (undefined1 (*) [16])&PTR_s_No_such_texture____Available__142ab1e50;
  local_b0 = (undefined1 (*) [16])0x2;
  local_98 = 0;
  local_a8 = &local_e0;
  local_a0 = 2;
  local_d0 = local_f8;
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}

