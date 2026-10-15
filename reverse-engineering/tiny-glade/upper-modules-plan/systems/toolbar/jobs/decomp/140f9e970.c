// 0x140f9e970 template_with_id<u64>
// signature: void __cdecl template_with_id<u64>(void)

void __cdecl toolbar::mei::config::impl$18::template_with_id<u64>(void)

{
  char *pcVar1;
  longlong lVar2;
  longlong *plVar3;
  code *pcVar4;
  uint uVar5;
  uint uVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  int iVar10;
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
  char cVar26;
  ushort uVar27;
  int iVar28;
  ulonglong extraout_RAX;
  undefined8 *in_RCX;
  longlong lVar29;
  longlong *in_RDX;
  undefined1 (*pauVar30) [16];
  ulonglong uVar31;
  uint uVar32;
  void *in_R8;
  size_t in_R9;
  ulonglong uVar33;
  ulonglong uVar34;
  byte bVar35;
  undefined1 auVar36 [16];
  char cVar37;
  char cVar38;
  char cVar39;
  char cVar40;
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
  undefined1 auVar41 [16];
  undefined1 local_120 [24];
  longlong local_108 [2];
  longlong *local_f8;
  code *local_f0;
  undefined1 *local_e8;
  code *local_e0;
  undefined1 (*local_d8) [16];
  longlong local_d0;
  void *local_c8;
  undefined8 *local_c0;
  undefined1 (*local_b8) [16];
  undefined1 (*local_b0) [16];
  longlong **local_a8;
  undefined8 local_a0;
  longlong local_98;
  longlong *local_88;
  undefined8 local_80;
  
  local_80 = 0xfffffffffffffffe;
  lVar2 = *in_RDX;
  local_88 = *(longlong **)(lVar2 + 0x248);
  if (local_88 == (longlong *)0x0) {
    local_b8 = *(undefined1 (**) [16])(lVar2 + 0x230);
    uVar34 = *(ulonglong *)(lVar2 + 0x238);
    lVar2 = local_d0;
  }
  else {
    local_c8 = in_R8;
    ::core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref_<str_>_>();
    local_b8 = *(undefined1 (**) [16])(lVar2 + 0x230);
    uVar34 = *(ulonglong *)(lVar2 + 0x238);
    bVar35 = (byte)(extraout_RAX >> 0x39);
    auVar36 = ZEXT216(CONCAT11(bVar35,bVar35));
    auVar36 = pshuflw(auVar36,auVar36,0);
    cVar37 = auVar36[0];
    cVar38 = auVar36[1];
    cVar39 = auVar36[2];
    cVar40 = auVar36[3];
    pauVar30 = local_b8 + -2;
    lVar29 = 0;
    iVar7 = -(uint)(CONCAT13(unaff_XMM7_Bd,
                             CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))) ==
                   CONCAT13(unaff_XMM7_Bd,
                            CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))));
    iVar8 = -(uint)(CONCAT13(unaff_XMM7_Bh,
                             CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))) ==
                   CONCAT13(unaff_XMM7_Bh,
                            CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))));
    iVar9 = -(uint)(CONCAT13(unaff_XMM7_Bl,
                             CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))) ==
                   CONCAT13(unaff_XMM7_Bl,
                            CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))));
    iVar10 = -(uint)(CONCAT13(unaff_XMM7_Bp,
                              CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))) ==
                    CONCAT13(unaff_XMM7_Bp,
                             CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))));
    uVar33 = extraout_RAX;
    while( true ) {
      uVar33 = uVar34 & uVar33;
      pcVar1 = *local_b8 + uVar33;
      cVar11 = *pcVar1;
      cVar12 = pcVar1[1];
      cVar13 = pcVar1[2];
      cVar14 = pcVar1[3];
      cVar15 = pcVar1[4];
      cVar16 = pcVar1[5];
      cVar17 = pcVar1[6];
      cVar18 = pcVar1[7];
      cVar19 = pcVar1[8];
      cVar20 = pcVar1[9];
      cVar21 = pcVar1[10];
      cVar22 = pcVar1[0xb];
      cVar23 = pcVar1[0xc];
      cVar24 = pcVar1[0xd];
      cVar25 = pcVar1[0xe];
      cVar26 = pcVar1[0xf];
      auVar36[0] = -(cVar11 == cVar37);
      auVar36[1] = -(cVar12 == cVar38);
      auVar36[2] = -(cVar13 == cVar39);
      auVar36[3] = -(cVar14 == cVar40);
      auVar36[4] = -(cVar15 == cVar37);
      auVar36[5] = -(cVar16 == cVar38);
      auVar36[6] = -(cVar17 == cVar39);
      auVar36[7] = -(cVar18 == cVar40);
      auVar36[8] = -(cVar19 == cVar37);
      auVar36[9] = -(cVar20 == cVar38);
      auVar36[10] = -(cVar21 == cVar39);
      auVar36[0xb] = -(cVar22 == cVar40);
      auVar36[0xc] = -(cVar23 == cVar37);
      auVar36[0xd] = -(cVar24 == cVar38);
      auVar36[0xe] = -(cVar25 == cVar39);
      auVar36[0xf] = -(cVar26 == cVar40);
      uVar27 = (ushort)(SUB161(auVar36 >> 7,0) & 1) | (ushort)(SUB161(auVar36 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar36 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar36 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar36 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar36 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar36 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar36 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar36 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar36 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar36 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar36 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar36 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar36 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar36 >> 0x77,0) & 1) << 0xe | (ushort)(auVar36[0xf] >> 7) << 0xf;
      uVar32 = (uint)uVar27;
      lVar2 = local_d0;
      while (local_d0 = lVar29, uVar27 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar32; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        uVar31 = uVar5 + uVar33 & uVar34;
        local_d8 = pauVar30;
        local_c0 = in_RCX;
        if ((*(size_t *)pauVar30[uVar31 * -2 + 1] == in_R9) &&
           (iVar28 = memcmp(local_c8,*(void **)(pauVar30[uVar31 * -2] + 8),in_R9), iVar28 == 0)) {
          plVar3 = *(longlong **)(local_b8[uVar31 * -2 + -1] + 8);
          LOCK();
          lVar2 = *plVar3;
          *plVar3 = *plVar3 + 1;
          UNLOCK();
          if (*plVar3 != 0 && SCARRY8(lVar2,1) == *plVar3 < 0) {
            lVar2 = in_RDX[1];
            local_b0 = (undefined1 (*) [16])0x41223169ff28813b;
            local_a8 = (longlong **)0xa79b7268a2a968d9;
            local_f8 = plVar3;
            ::utils::calculate_hash<tuple_<u64,core::any::TypeId>_>();
            local_b8 = (undefined1 (*) [16])lVar2;
            ::utils::calculate_hash<tuple_<u64,u64>_>();
            local_88 = plVar3;
            instantiate_ui_template_impl();
            local_c0[4] = in_RDX[2];
            lVar2 = in_RDX[1];
            local_c0[2] = *in_RDX;
            local_c0[3] = lVar2;
            *local_c0 = local_c8;
            local_c0[1] = in_R9;
            LOCK();
            *local_88 = *local_88 + -1;
            UNLOCK();
            if (*local_88 == 0) {
              alloc::sync::Arc::drop_slow<toolbar::mei::config::NodeConfig,alloc::alloc::Global>();
            }
            return;
          }
                    // WARNING: Does not return
          pcVar4 = (code *)invalidInstructionException();
          (*pcVar4)();
        }
        uVar27 = (ushort)(uVar32 - 1) & (ushort)uVar32;
        uVar32 = CONCAT22((short)(uVar32 - 1 >> 0x10),uVar27);
        lVar29 = local_d0;
        pauVar30 = local_d8;
        in_RCX = local_c0;
        lVar2 = local_d0;
      }
      auVar41[0] = -(cVar11 == (char)iVar7);
      auVar41[1] = -(cVar12 == (char)((uint)iVar7 >> 8));
      auVar41[2] = -(cVar13 == (char)((uint)iVar7 >> 0x10));
      auVar41[3] = -(cVar14 == (char)((uint)iVar7 >> 0x18));
      auVar41[4] = -(cVar15 == (char)iVar8);
      auVar41[5] = -(cVar16 == (char)((uint)iVar8 >> 8));
      auVar41[6] = -(cVar17 == (char)((uint)iVar8 >> 0x10));
      auVar41[7] = -(cVar18 == (char)((uint)iVar8 >> 0x18));
      auVar41[8] = -(cVar19 == (char)iVar9);
      auVar41[9] = -(cVar20 == (char)((uint)iVar9 >> 8));
      auVar41[10] = -(cVar21 == (char)((uint)iVar9 >> 0x10));
      auVar41[0xb] = -(cVar22 == (char)((uint)iVar9 >> 0x18));
      auVar41[0xc] = -(cVar23 == (char)iVar10);
      auVar41[0xd] = -(cVar24 == (char)((uint)iVar10 >> 8));
      auVar41[0xe] = -(cVar25 == (char)((uint)iVar10 >> 0x10));
      auVar41[0xf] = -(cVar26 == (char)((uint)iVar10 >> 0x18));
      if ((((((((((((((((SUB161(auVar41 >> 7,0) & 1) != 0 || (SUB161(auVar41 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar41 >> 0x17,0) & 1) != 0) || (SUB161(auVar41 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar41 >> 0x27,0) & 1) != 0) || (SUB161(auVar41 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar41 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar41 >> 0x3f,0) & 1) != 0) || (SUB161(auVar41 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar41 >> 0x4f,0) & 1) != 0) || (SUB161(auVar41 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar41 >> 0x5f,0) & 1) != 0) || (SUB161(auVar41 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar41 >> 0x6f,0) & 1) != 0) || (SUB161(auVar41 >> 0x77,0) & 1) != 0) ||
          auVar41[0xf] < '\0') break;
      uVar33 = uVar33 + local_d0 + 0x10;
      lVar29 = local_d0 + 0x10;
      local_d0 = lVar2;
    }
  }
  local_d0 = lVar2;
  local_a8 = (longlong **)(*local_b8 + uVar34 + 1);
  auVar36 = *local_b8;
  local_a0 = CONCAT62(local_a0._2_6_,
                      ~((ushort)(SUB161(auVar36 >> 7,0) & 1) |
                        (ushort)(SUB161(auVar36 >> 0xf,0) & 1) << 1 |
                        (ushort)(SUB161(auVar36 >> 0x17,0) & 1) << 2 |
                        (ushort)(SUB161(auVar36 >> 0x1f,0) & 1) << 3 |
                        (ushort)(SUB161(auVar36 >> 0x27,0) & 1) << 4 |
                        (ushort)(SUB161(auVar36 >> 0x2f,0) & 1) << 5 |
                        (ushort)(SUB161(auVar36 >> 0x37,0) & 1) << 6 |
                        (ushort)(SUB161(auVar36 >> 0x3f,0) & 1) << 7 |
                        (ushort)(SUB161(auVar36 >> 0x47,0) & 1) << 8 |
                        (ushort)(SUB161(auVar36 >> 0x4f,0) & 1) << 9 |
                        (ushort)(SUB161(auVar36 >> 0x57,0) & 1) << 10 |
                        (ushort)(SUB161(auVar36 >> 0x5f,0) & 1) << 0xb |
                        (ushort)(SUB161(auVar36 >> 0x67,0) & 1) << 0xc |
                        (ushort)(SUB161(auVar36 >> 0x6f,0) & 1) << 0xd |
                        (ushort)(SUB161(auVar36 >> 0x77,0) & 1) << 0xe |
                       (ushort)(byte)(auVar36[0xf] >> 7) << 0xf));
  local_b0 = local_b8 + 1;
  local_98 = (longlong)local_88;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<ref_<alloc::string::String>,std::collections::hash::map::Keys<alloc::string::String,alloc::sync::Arc<toolbar::mei::config::NodeConfig,alloc::alloc::Global>_>_>
            ();
  local_f8 = local_108;
  local_f0 = ::core::fmt::impl$74::fmt<str_>;
  local_e0 = alloc::vec::impl$27::fmt<ref_<alloc::string::String>,alloc::alloc::Global>;
  local_b8 = (undefined1 (*) [16])&PTR_DAT_142bb07c0;
  local_b0 = (undefined1 (*) [16])0x2;
  local_98 = 0;
  local_a8 = &local_f8;
  local_a0 = 2;
  local_e8 = local_120;
                    // WARNING: Subroutine does not return
  ::core::panicking::panic_fmt();
}

