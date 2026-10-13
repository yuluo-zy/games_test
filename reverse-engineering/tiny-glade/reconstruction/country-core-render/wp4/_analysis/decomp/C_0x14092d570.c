// ADDRESS: 0x14092d570
// SIZE: 479
// SIGNATURE: void __cdecl try_get_handle_by_name(void)

void __cdecl
country_core::asset_libraries::mesh_library::AssetMeshLibrary::try_get_handle_by_name(void)

{
  ulonglong *puVar1;
  char *pcVar2;
  ulonglong uVar3;
  longlong lVar4;
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
  longlong lVar29;
  longlong in_RCX;
  void *in_RDX;
  uint uVar30;
  ulonglong uVar31;
  size_t in_R8;
  longlong lVar32;
  byte bVar33;
  undefined1 auVar34 [16];
  char cVar35;
  char cVar36;
  char cVar37;
  char cVar38;
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
  undefined1 auVar39 [16];
  
  puVar1 = (ulonglong *)(in_RCX + 0x60);
  uVar3 = *(ulonglong *)(in_RCX + 0x60);
  if (uVar3 < 0xfffffffffffffff0 && (char)(((uint)uVar3 & 8) >> 3) == '\0') {
    LOCK();
    uVar31 = *puVar1;
    if (uVar3 == uVar31) {
      *puVar1 = uVar3 + 0x10;
    }
    UNLOCK();
    if (uVar3 == uVar31) goto LAB_14092d5d4;
  }
  parking_lot::raw_rwlock::RawRwLock::lock_shared_slow();
LAB_14092d5d4:
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  if (*(longlong *)(in_RCX + 0x80) != 0) {
    core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref_<str_>_>();
    lVar4 = *(longlong *)(in_RCX + 0x68);
    uVar3 = *(ulonglong *)(in_RCX + 0x70);
    bVar33 = (byte)(extraout_RAX >> 0x39);
    auVar34 = ZEXT216(CONCAT11(bVar33,bVar33));
    auVar34 = pshuflw(auVar34,auVar34,0);
    cVar35 = auVar34[0];
    cVar36 = auVar34[1];
    cVar37 = auVar34[2];
    cVar38 = auVar34[3];
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
    lVar32 = 0;
    uVar31 = extraout_RAX;
    while( true ) {
      uVar31 = uVar3 & uVar31;
      pcVar2 = (char *)(lVar4 + uVar31);
      cVar11 = *pcVar2;
      cVar12 = pcVar2[1];
      cVar13 = pcVar2[2];
      cVar14 = pcVar2[3];
      cVar15 = pcVar2[4];
      cVar16 = pcVar2[5];
      cVar17 = pcVar2[6];
      cVar18 = pcVar2[7];
      cVar19 = pcVar2[8];
      cVar20 = pcVar2[9];
      cVar21 = pcVar2[10];
      cVar22 = pcVar2[0xb];
      cVar23 = pcVar2[0xc];
      cVar24 = pcVar2[0xd];
      cVar25 = pcVar2[0xe];
      cVar26 = pcVar2[0xf];
      auVar34[0] = -(cVar11 == cVar35);
      auVar34[1] = -(cVar12 == cVar36);
      auVar34[2] = -(cVar13 == cVar37);
      auVar34[3] = -(cVar14 == cVar38);
      auVar34[4] = -(cVar15 == cVar35);
      auVar34[5] = -(cVar16 == cVar36);
      auVar34[6] = -(cVar17 == cVar37);
      auVar34[7] = -(cVar18 == cVar38);
      auVar34[8] = -(cVar19 == cVar35);
      auVar34[9] = -(cVar20 == cVar36);
      auVar34[10] = -(cVar21 == cVar37);
      auVar34[0xb] = -(cVar22 == cVar38);
      auVar34[0xc] = -(cVar23 == cVar35);
      auVar34[0xd] = -(cVar24 == cVar36);
      auVar34[0xe] = -(cVar25 == cVar37);
      auVar34[0xf] = -(cVar26 == cVar38);
      uVar27 = (ushort)(SUB161(auVar34 >> 7,0) & 1) | (ushort)(SUB161(auVar34 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar34 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar34 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar34 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar34 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar34 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar34 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar34 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar34 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar34 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar34 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar34 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar34 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar34 >> 0x77,0) & 1) << 0xe | (ushort)(auVar34[0xf] >> 7) << 0xf;
      uVar30 = (uint)uVar27;
      while (uVar27 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar30; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        lVar29 = lVar4 + -0x20 + (uVar5 + uVar31 & uVar3) * -0x20;
        if ((*(size_t *)(lVar29 + 0x10) == in_R8) &&
           (iVar28 = memcmp(in_RDX,*(void **)(lVar29 + 8),in_R8), iVar28 == 0)) goto LAB_14092d6f8;
        uVar27 = (ushort)(uVar30 - 1) & (ushort)uVar30;
        uVar30 = CONCAT22((short)(uVar30 - 1 >> 0x10),uVar27);
      }
      auVar39[0] = -(cVar11 == (char)iVar7);
      auVar39[1] = -(cVar12 == (char)((uint)iVar7 >> 8));
      auVar39[2] = -(cVar13 == (char)((uint)iVar7 >> 0x10));
      auVar39[3] = -(cVar14 == (char)((uint)iVar7 >> 0x18));
      auVar39[4] = -(cVar15 == (char)iVar8);
      auVar39[5] = -(cVar16 == (char)((uint)iVar8 >> 8));
      auVar39[6] = -(cVar17 == (char)((uint)iVar8 >> 0x10));
      auVar39[7] = -(cVar18 == (char)((uint)iVar8 >> 0x18));
      auVar39[8] = -(cVar19 == (char)iVar9);
      auVar39[9] = -(cVar20 == (char)((uint)iVar9 >> 8));
      auVar39[10] = -(cVar21 == (char)((uint)iVar9 >> 0x10));
      auVar39[0xb] = -(cVar22 == (char)((uint)iVar9 >> 0x18));
      auVar39[0xc] = -(cVar23 == (char)iVar10);
      auVar39[0xd] = -(cVar24 == (char)((uint)iVar10 >> 8));
      auVar39[0xe] = -(cVar25 == (char)((uint)iVar10 >> 0x10));
      auVar39[0xf] = -(cVar26 == (char)((uint)iVar10 >> 0x18));
      if ((((((((((((((((SUB161(auVar39 >> 7,0) & 1) != 0 || (SUB161(auVar39 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar39 >> 0x17,0) & 1) != 0) || (SUB161(auVar39 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar39 >> 0x27,0) & 1) != 0) || (SUB161(auVar39 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar39 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar39 >> 0x3f,0) & 1) != 0) || (SUB161(auVar39 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar39 >> 0x4f,0) & 1) != 0) || (SUB161(auVar39 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar39 >> 0x5f,0) & 1) != 0) || (SUB161(auVar39 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar39 >> 0x6f,0) & 1) != 0) || (SUB161(auVar39 >> 0x77,0) & 1) != 0) ||
          auVar39[0xf] < '\0') break;
      uVar31 = uVar31 + lVar32 + 0x10;
      lVar32 = lVar32 + 0x10;
    }
  }
LAB_14092d6f8:
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  uVar3 = *puVar1;
  *puVar1 = *puVar1 - 0x10;
  UNLOCK();
  if ((uVar3 & 0xfffffffffffffff2) == 0x12) {
    parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow();
  }
  return;
}

