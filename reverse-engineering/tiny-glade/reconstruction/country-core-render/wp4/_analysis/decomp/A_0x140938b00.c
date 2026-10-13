// ADDRESS: 0x140938b00
// SIZE: 1395
// SIGNATURE: void __cdecl get_or_generate(void)

void __cdecl country_core::render::shader::RasterShaderProgramVariant::get_or_generate(void)

{
  ulonglong *puVar1;
  char *pcVar2;
  undefined8 *puVar3;
  ulonglong *puVar4;
  ulonglong uVar5;
  longlong *plVar6;
  undefined1 (*pauVar7) [16];
  code *pcVar8;
  uint uVar9;
  int iVar10;
  int iVar11;
  int iVar12;
  int iVar13;
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
  char cVar27;
  char cVar28;
  char cVar29;
  char extraout_AL;
  ushort uVar30;
  ulonglong extraout_RAX;
  ulonglong *extraout_RAX_00;
  undefined8 *in_RCX;
  uint uVar31;
  longlong lVar32;
  byte bVar33;
  longlong in_R8;
  longlong lVar34;
  uint uVar35;
  ulonglong uVar36;
  byte bVar37;
  undefined1 auVar38 [16];
  char cVar39;
  char cVar40;
  char cVar41;
  char cVar42;
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
  undefined1 auVar43 [16];
  ulonglong local_d8;
  undefined8 *puStack_d0;
  ulonglong local_c8;
  undefined6 uStack_c0;
  undefined2 uStack_ba;
  undefined6 uStack_b8;
  char cStack_b2;
  undefined1 uStack_b1;
  undefined7 uStack_b0;
  undefined1 uStack_a9;
  ulonglong local_a8;
  
  puVar1 = in_RCX + 1;
  uVar5 = in_RCX[1];
  if (uVar5 < 0xfffffffffffffff0 && (char)(((uint)uVar5 & 8) >> 3) == '\0') {
    LOCK();
    uVar36 = *puVar1;
    if (uVar5 == uVar36) {
      *puVar1 = uVar5 + 0x10;
    }
    UNLOCK();
    if (uVar5 == uVar36) goto LAB_140938b8c;
  }
  parking_lot::raw_rwlock::RawRwLock::lock_shared_slow();
LAB_140938b8c:
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  if (in_RCX[5] != 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<rhapsody::raw::shader::RasterPipelineDesc>_>();
    lVar32 = in_RCX[2];
    uVar5 = in_RCX[3];
    bVar37 = (byte)(extraout_RAX >> 0x39);
    auVar38 = ZEXT216(CONCAT11(bVar37,bVar37));
    auVar38 = pshuflw(auVar38,auVar38,0);
    cVar39 = auVar38[0];
    cVar40 = auVar38[1];
    cVar41 = auVar38[2];
    cVar42 = auVar38[3];
    lVar34 = 0;
    iVar10 = -(uint)(CONCAT13(unaff_XMM7_Bd,
                              CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))) ==
                    CONCAT13(unaff_XMM7_Bd,
                             CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))));
    iVar11 = -(uint)(CONCAT13(unaff_XMM7_Bh,
                              CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))) ==
                    CONCAT13(unaff_XMM7_Bh,
                             CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))));
    iVar12 = -(uint)(CONCAT13(unaff_XMM7_Bl,
                              CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))) ==
                    CONCAT13(unaff_XMM7_Bl,
                             CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))));
    iVar13 = -(uint)(CONCAT13(unaff_XMM7_Bp,
                              CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))) ==
                    CONCAT13(unaff_XMM7_Bp,
                             CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))));
    uVar36 = extraout_RAX;
    while( true ) {
      uVar36 = uVar5 & uVar36;
      pcVar2 = (char *)(lVar32 + uVar36);
      cVar14 = *pcVar2;
      cVar15 = pcVar2[1];
      cVar16 = pcVar2[2];
      cVar17 = pcVar2[3];
      cVar18 = pcVar2[4];
      cVar19 = pcVar2[5];
      cVar20 = pcVar2[6];
      cVar21 = pcVar2[7];
      cVar22 = pcVar2[8];
      cVar23 = pcVar2[9];
      cVar24 = pcVar2[10];
      cVar25 = pcVar2[0xb];
      cVar26 = pcVar2[0xc];
      cVar27 = pcVar2[0xd];
      cVar28 = pcVar2[0xe];
      cVar29 = pcVar2[0xf];
      auVar38[0] = -(cVar14 == cVar39);
      auVar38[1] = -(cVar15 == cVar40);
      auVar38[2] = -(cVar16 == cVar41);
      auVar38[3] = -(cVar17 == cVar42);
      auVar38[4] = -(cVar18 == cVar39);
      auVar38[5] = -(cVar19 == cVar40);
      auVar38[6] = -(cVar20 == cVar41);
      auVar38[7] = -(cVar21 == cVar42);
      auVar38[8] = -(cVar22 == cVar39);
      auVar38[9] = -(cVar23 == cVar40);
      auVar38[10] = -(cVar24 == cVar41);
      auVar38[0xb] = -(cVar25 == cVar42);
      auVar38[0xc] = -(cVar26 == cVar39);
      auVar38[0xd] = -(cVar27 == cVar40);
      auVar38[0xe] = -(cVar28 == cVar41);
      auVar38[0xf] = -(cVar29 == cVar42);
      uVar30 = (ushort)(SUB161(auVar38 >> 7,0) & 1) | (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe | (ushort)(auVar38[0xf] >> 7) << 0xf;
      uVar31 = (uint)uVar30;
      while (uVar30 != 0) {
        uVar35 = 0;
        for (uVar9 = uVar31; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
          uVar35 = uVar35 + 1;
        }
        rhapsody::raw::managed::handle::impl$7::eq();
        if ((extraout_AL != '\0') &&
           (*(int *)(in_R8 + 0x18) == *(int *)(lVar32 + (uVar35 + uVar36 & uVar5) * -0x28 + -0x10)))
        {
          parking_lot_core::parking_lot::deadlock_impl::release_resource();
          parking_lot_core::parking_lot::deadlock_impl::release_resource();
          LOCK();
          uVar5 = *puVar1;
          *puVar1 = *puVar1 - 0x10;
          UNLOCK();
          if ((uVar5 & 0xfffffffffffffff2) == 0x12) {
            parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow();
          }
          rhapsody::raw::managed::handle::impl$6::drop();
          return;
        }
        uVar30 = (ushort)(uVar31 - 1) & (ushort)uVar31;
        uVar31 = CONCAT22((short)(uVar31 - 1 >> 0x10),uVar30);
      }
      auVar43[0] = -(cVar14 == (char)iVar10);
      auVar43[1] = -(cVar15 == (char)((uint)iVar10 >> 8));
      auVar43[2] = -(cVar16 == (char)((uint)iVar10 >> 0x10));
      auVar43[3] = -(cVar17 == (char)((uint)iVar10 >> 0x18));
      auVar43[4] = -(cVar18 == (char)iVar11);
      auVar43[5] = -(cVar19 == (char)((uint)iVar11 >> 8));
      auVar43[6] = -(cVar20 == (char)((uint)iVar11 >> 0x10));
      auVar43[7] = -(cVar21 == (char)((uint)iVar11 >> 0x18));
      auVar43[8] = -(cVar22 == (char)iVar12);
      auVar43[9] = -(cVar23 == (char)((uint)iVar12 >> 8));
      auVar43[10] = -(cVar24 == (char)((uint)iVar12 >> 0x10));
      auVar43[0xb] = -(cVar25 == (char)((uint)iVar12 >> 0x18));
      auVar43[0xc] = -(cVar26 == (char)iVar13);
      auVar43[0xd] = -(cVar27 == (char)((uint)iVar13 >> 8));
      auVar43[0xe] = -(cVar28 == (char)((uint)iVar13 >> 0x10));
      auVar43[0xf] = -(cVar29 == (char)((uint)iVar13 >> 0x18));
      if ((((((((((((((((SUB161(auVar43 >> 7,0) & 1) != 0 || (SUB161(auVar43 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar43 >> 0x17,0) & 1) != 0) || (SUB161(auVar43 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar43 >> 0x27,0) & 1) != 0) || (SUB161(auVar43 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar43 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar43 >> 0x3f,0) & 1) != 0) || (SUB161(auVar43 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar43 >> 0x4f,0) & 1) != 0) || (SUB161(auVar43 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar43 >> 0x5f,0) & 1) != 0) || (SUB161(auVar43 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar43 >> 0x6f,0) & 1) != 0) || (SUB161(auVar43 >> 0x77,0) & 1) != 0) ||
          auVar43[0xf] < '\0') break;
      uVar36 = uVar36 + lVar34 + 0x10;
      lVar34 = lVar34 + 0x10;
    }
  }
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  uVar5 = *puVar1;
  *puVar1 = *puVar1 - 0x10;
  UNLOCK();
  if ((uVar5 & 0xfffffffffffffff2) == 0x12) {
    parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow();
  }
  LOCK();
  uVar5 = *puVar1;
  if (uVar5 == 0) {
    *puVar1 = 8;
  }
  UNLOCK();
  if (uVar5 != 0) {
    parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow();
  }
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  hashbrown::map::HashMap::
  rustc_entry<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>
            ();
  if (cStack_b2 != '\b') {
    plVar6 = (longlong *)*in_RCX;
    LOCK();
    lVar32 = *plVar6;
    *plVar6 = *plVar6 + 1;
    UNLOCK();
    if (*plVar6 == 0 || SCARRY8(lVar32,1) != *plVar6 < 0) {
                    // WARNING: Does not return
      pcVar8 = (code *)invalidInstructionException();
      (*pcVar8)();
    }
    rhapsody::raw::device::GraphicsDevice::
    create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global>_>
              ();
    if (SBORROW8(0,local_d8)) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_00 == (ulonglong *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    extraout_RAX_00[6] = local_a8;
    extraout_RAX_00[4] = CONCAT17(uStack_b1,CONCAT16(cStack_b2,uStack_b8));
    extraout_RAX_00[5] = CONCAT17(uStack_a9,uStack_b0);
    extraout_RAX_00[2] = local_c8;
    extraout_RAX_00[3] = CONCAT26(uStack_ba,uStack_c0);
    *extraout_RAX_00 = local_d8;
    extraout_RAX_00[1] = (ulonglong)puStack_d0;
    pauVar7 = (undefined1 (*) [16])*puStack_d0;
    uVar5 = puStack_d0[1];
    uVar36 = uVar5 & local_d8;
    auVar38 = *(undefined1 (*) [16])(*pauVar7 + uVar36);
    uVar30 = (ushort)(SUB161(auVar38 >> 7,0) & 1) | (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
             (ushort)(byte)(auVar38[0xf] >> 7) << 0xf;
    uVar31 = (uint)uVar30;
    if (uVar30 == 0) {
      lVar32 = 0x10;
      do {
        uVar36 = uVar36 + lVar32 & uVar5;
        auVar38 = *(undefined1 (*) [16])(*pauVar7 + uVar36);
        lVar32 = lVar32 + 0x10;
        uVar31 = (uint)(ushort)((ushort)(SUB161(auVar38 >> 7,0) & 1) |
                                (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
                                (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                                (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                                (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                                (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                                (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                                (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                                (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                                (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                                (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                                (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                                (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                                (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                                (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
                               (ushort)(byte)(auVar38[0xf] >> 7) << 0xf);
      } while (uVar31 == 0);
    }
    uVar35 = 0;
    for (; (uVar31 & 1) == 0; uVar31 = uVar31 >> 1 | 0x80000000) {
      uVar35 = uVar35 + 1;
    }
    uVar36 = uVar35 + uVar36 & uVar5;
    bVar37 = (*pauVar7)[uVar36];
    if (-1 < (char)bVar37) {
      auVar38 = *pauVar7;
      uVar31 = 0;
      for (uVar35 = (uint)(ushort)((ushort)(SUB161(auVar38 >> 7,0) & 1) |
                                   (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
                                   (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                                   (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                                   (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                                   (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                                   (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                                   (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                                   (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                                   (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                                   (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                                   (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                                   (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                                   (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                                   (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
                                  (ushort)(byte)(auVar38[0xf] >> 7) << 0xf); (uVar35 & 1) == 0;
          uVar35 = uVar35 >> 1 | 0x80000000) {
        uVar31 = uVar31 + 1;
      }
      uVar36 = (ulonglong)uVar31;
      bVar37 = (*pauVar7)[uVar36];
    }
    bVar33 = (byte)(local_d8 >> 0x39);
    (*pauVar7)[uVar36] = bVar33;
    pauVar7[1][uVar36 - 0x10 & uVar5] = bVar33;
    puStack_d0[2] = puStack_d0[2] - (ulonglong)(bVar37 & 1);
    puVar3 = (undefined8 *)((longlong)pauVar7 + uVar36 * -0x28 + -0x18);
    *puVar3 = CONCAT17(uStack_b1,CONCAT16(cStack_b2,uStack_b8));
    puVar3[1] = CONCAT17(uStack_a9,uStack_b0);
    puVar4 = (ulonglong *)((longlong)pauVar7 + uVar36 * -0x28 + -0x28);
    *puVar4 = local_c8;
    puVar4[1] = CONCAT26(uStack_ba,uStack_c0);
    *(ulonglong **)((longlong)pauVar7 + uVar36 * -0x28 + -8) = extraout_RAX_00;
    puStack_d0[3] = puStack_d0[3] + 1;
  }
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  uVar5 = *puVar1;
  if (uVar5 == 8) {
    *puVar1 = 0;
  }
  UNLOCK();
  if (uVar5 != 8) {
    parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
  }
  return;
}

