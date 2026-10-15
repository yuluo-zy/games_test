// 0x14032e210 template_with_id<ref$<str$>_>
// signature: void __cdecl template_with_id<ref$<str$>_>(void)

void __cdecl toolbar::mei::config::impl$18::template_with_id<ref_<str_>_>(void)

{
  char *pcVar1;
  longlong lVar2;
  code *pcVar3;
  uint uVar4;
  uint uVar5;
  int iVar6;
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
  ushort uVar26;
  int iVar27;
  ulonglong extraout_RAX;
  undefined8 *in_RCX;
  longlong lVar28;
  longlong *in_RDX;
  undefined1 (*pauVar29) [16];
  ulonglong uVar30;
  uint uVar31;
  void *in_R8;
  size_t in_R9;
  ulonglong uVar32;
  ulonglong uVar33;
  byte bVar34;
  undefined1 auVar35 [16];
  char cVar36;
  char cVar37;
  char cVar38;
  char cVar39;
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
  undefined1 auVar40 [16];
  undefined1 local_120 [16];
  undefined1 *local_110;
  code *local_108;
  longlong **local_100;
  code *local_f8;
  longlong local_f0;
  undefined1 (*local_e8) [16];
  longlong local_e0;
  longlong *local_d8 [3];
  void *local_c0;
  undefined8 *local_b8;
  undefined1 (*local_b0) [16];
  undefined1 (*local_a8) [16];
  undefined1 **local_a0;
  undefined8 local_98;
  longlong local_90;
  undefined8 local_80;
  
  local_80 = 0xfffffffffffffffe;
  lVar2 = *in_RDX;
  local_f0 = *(longlong *)(lVar2 + 0x248);
  if (local_f0 == 0) {
    local_b0 = *(undefined1 (**) [16])(lVar2 + 0x230);
    uVar33 = *(ulonglong *)(lVar2 + 0x238);
    lVar2 = local_e0;
  }
  else {
    local_c0 = in_R8;
    ::core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref_<str_>_>();
    local_b0 = *(undefined1 (**) [16])(lVar2 + 0x230);
    uVar33 = *(ulonglong *)(lVar2 + 0x238);
    bVar34 = (byte)(extraout_RAX >> 0x39);
    auVar35 = ZEXT216(CONCAT11(bVar34,bVar34));
    auVar35 = pshuflw(auVar35,auVar35,0);
    cVar36 = auVar35[0];
    cVar37 = auVar35[1];
    cVar38 = auVar35[2];
    cVar39 = auVar35[3];
    pauVar29 = local_b0 + -2;
    lVar28 = 0;
    iVar6 = -(uint)(CONCAT13(unaff_XMM7_Bd,
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
    uVar32 = extraout_RAX;
    while( true ) {
      uVar32 = uVar33 & uVar32;
      pcVar1 = *local_b0 + uVar32;
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
      auVar35[0] = -(cVar10 == cVar36);
      auVar35[1] = -(cVar11 == cVar37);
      auVar35[2] = -(cVar12 == cVar38);
      auVar35[3] = -(cVar13 == cVar39);
      auVar35[4] = -(cVar14 == cVar36);
      auVar35[5] = -(cVar15 == cVar37);
      auVar35[6] = -(cVar16 == cVar38);
      auVar35[7] = -(cVar17 == cVar39);
      auVar35[8] = -(cVar18 == cVar36);
      auVar35[9] = -(cVar19 == cVar37);
      auVar35[10] = -(cVar20 == cVar38);
      auVar35[0xb] = -(cVar21 == cVar39);
      auVar35[0xc] = -(cVar22 == cVar36);
      auVar35[0xd] = -(cVar23 == cVar37);
      auVar35[0xe] = -(cVar24 == cVar38);
      auVar35[0xf] = -(cVar25 == cVar39);
      uVar26 = (ushort)(SUB161(auVar35 >> 7,0) & 1) | (ushort)(SUB161(auVar35 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar35 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar35 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar35 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar35 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar35 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar35 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar35 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar35 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar35 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar35 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar35 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar35 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar35 >> 0x77,0) & 1) << 0xe | (ushort)(auVar35[0xf] >> 7) << 0xf;
      uVar31 = (uint)uVar26;
      lVar2 = local_e0;
      while (local_e0 = lVar28, uVar26 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar31; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        uVar30 = uVar4 + uVar32 & uVar33;
        local_e8 = pauVar29;
        local_b8 = in_RCX;
        if ((*(size_t *)pauVar29[uVar30 * -2 + 1] == in_R9) &&
           (iVar27 = memcmp(local_c0,*(void **)(pauVar29[uVar30 * -2] + 8),in_R9), iVar27 == 0)) {
          local_d8[0] = *(longlong **)(local_b0[uVar30 * -2 + -1] + 8);
          LOCK();
          lVar2 = *local_d8[0];
          *local_d8[0] = *local_d8[0] + 1;
          UNLOCK();
          if (*local_d8[0] != 0 && SCARRY8(lVar2,1) == *local_d8[0] < 0) {
            core::NodeContext::id();
            core::impl$0::as_id();
            local_a8 = (undefined1 (*) [16])0x41223169ff28813b;
            local_a0 = (undefined1 **)0xa79b7268a2a968d9;
            ::utils::calculate_hash<tuple_<u64,core::any::TypeId>_>();
            ::utils::calculate_hash<tuple_<u64,u64>_>();
            instantiate_ui_template_impl();
            local_b8[4] = in_RDX[2];
            lVar2 = in_RDX[1];
            local_b8[2] = *in_RDX;
            local_b8[3] = lVar2;
            *local_b8 = local_c0;
            local_b8[1] = in_R9;
            LOCK();
            *local_d8[0] = *local_d8[0] + -1;
            UNLOCK();
            if (*local_d8[0] == 0) {
              alloc::sync::Arc::drop_slow<toolbar::mei::config::NodeConfig,alloc::alloc::Global>();
            }
            return;
          }
                    // WARNING: Does not return
          pcVar3 = (code *)invalidInstructionException();
          (*pcVar3)();
        }
        uVar26 = (ushort)(uVar31 - 1) & (ushort)uVar31;
        uVar31 = CONCAT22((short)(uVar31 - 1 >> 0x10),uVar26);
        lVar28 = local_e0;
        pauVar29 = local_e8;
        in_RCX = local_b8;
        lVar2 = local_e0;
      }
      auVar40[0] = -(cVar10 == (char)iVar6);
      auVar40[1] = -(cVar11 == (char)((uint)iVar6 >> 8));
      auVar40[2] = -(cVar12 == (char)((uint)iVar6 >> 0x10));
      auVar40[3] = -(cVar13 == (char)((uint)iVar6 >> 0x18));
      auVar40[4] = -(cVar14 == (char)iVar7);
      auVar40[5] = -(cVar15 == (char)((uint)iVar7 >> 8));
      auVar40[6] = -(cVar16 == (char)((uint)iVar7 >> 0x10));
      auVar40[7] = -(cVar17 == (char)((uint)iVar7 >> 0x18));
      auVar40[8] = -(cVar18 == (char)iVar8);
      auVar40[9] = -(cVar19 == (char)((uint)iVar8 >> 8));
      auVar40[10] = -(cVar20 == (char)((uint)iVar8 >> 0x10));
      auVar40[0xb] = -(cVar21 == (char)((uint)iVar8 >> 0x18));
      auVar40[0xc] = -(cVar22 == (char)iVar9);
      auVar40[0xd] = -(cVar23 == (char)((uint)iVar9 >> 8));
      auVar40[0xe] = -(cVar24 == (char)((uint)iVar9 >> 0x10));
      auVar40[0xf] = -(cVar25 == (char)((uint)iVar9 >> 0x18));
      if ((((((((((((((((SUB161(auVar40 >> 7,0) & 1) != 0 || (SUB161(auVar40 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar40 >> 0x17,0) & 1) != 0) || (SUB161(auVar40 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar40 >> 0x27,0) & 1) != 0) || (SUB161(auVar40 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar40 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar40 >> 0x3f,0) & 1) != 0) || (SUB161(auVar40 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar40 >> 0x4f,0) & 1) != 0) || (SUB161(auVar40 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar40 >> 0x5f,0) & 1) != 0) || (SUB161(auVar40 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar40 >> 0x6f,0) & 1) != 0) || (SUB161(auVar40 >> 0x77,0) & 1) != 0) ||
          auVar40[0xf] < '\0') break;
      uVar32 = uVar32 + local_e0 + 0x10;
      lVar28 = local_e0 + 0x10;
      local_e0 = lVar2;
    }
  }
  local_e0 = lVar2;
  local_a0 = (undefined1 **)(*local_b0 + uVar33 + 1);
  auVar35 = *local_b0;
  local_98 = CONCAT62(local_98._2_6_,
                      ~((ushort)(SUB161(auVar35 >> 7,0) & 1) |
                        (ushort)(SUB161(auVar35 >> 0xf,0) & 1) << 1 |
                        (ushort)(SUB161(auVar35 >> 0x17,0) & 1) << 2 |
                        (ushort)(SUB161(auVar35 >> 0x1f,0) & 1) << 3 |
                        (ushort)(SUB161(auVar35 >> 0x27,0) & 1) << 4 |
                        (ushort)(SUB161(auVar35 >> 0x2f,0) & 1) << 5 |
                        (ushort)(SUB161(auVar35 >> 0x37,0) & 1) << 6 |
                        (ushort)(SUB161(auVar35 >> 0x3f,0) & 1) << 7 |
                        (ushort)(SUB161(auVar35 >> 0x47,0) & 1) << 8 |
                        (ushort)(SUB161(auVar35 >> 0x4f,0) & 1) << 9 |
                        (ushort)(SUB161(auVar35 >> 0x57,0) & 1) << 10 |
                        (ushort)(SUB161(auVar35 >> 0x5f,0) & 1) << 0xb |
                        (ushort)(SUB161(auVar35 >> 0x67,0) & 1) << 0xc |
                        (ushort)(SUB161(auVar35 >> 0x6f,0) & 1) << 0xd |
                        (ushort)(SUB161(auVar35 >> 0x77,0) & 1) << 0xe |
                       (ushort)(byte)(auVar35[0xf] >> 7) << 0xf));
  local_a8 = local_b0 + 1;
  local_90 = local_f0;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<ref_<alloc::string::String>,std::collections::hash::map::Keys<alloc::string::String,alloc::sync::Arc<toolbar::mei::config::NodeConfig,alloc::alloc::Global>_>_>
            ();
  local_110 = local_120;
  local_108 = ::core::fmt::impl$74::fmt<str_>;
  local_f8 = alloc::vec::impl$27::fmt<ref_<alloc::string::String>,alloc::alloc::Global>;
  local_b0 = (undefined1 (*) [16])&PTR_s_No_UI_template_named___Available_1429bf718;
  local_a8 = (undefined1 (*) [16])0x2;
  local_90 = 0;
  local_a0 = &local_110;
  local_98 = 2;
  local_100 = local_d8;
                    // WARNING: Subroutine does not return
  ::core::panicking::panic_fmt();
}

