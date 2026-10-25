
void __cdecl system_clutter::autoclutter::add_birdnests::add_birdnests(void)

{
  byte *pbVar1;
  longlong *plVar2;
  char *pcVar3;
  undefined8 *puVar4;
  ulonglong uVar5;
  byte bVar6;
  undefined1 (*pauVar7) [16];
  undefined8 uVar8;
  code *pcVar9;
  uint uVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  ulonglong *puVar14;
  char extraout_AL;
  char extraout_AL_00;
  undefined1 extraout_AL_01;
  uint uVar15;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong lVar16;
  ulonglong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong lVar17;
  longlong *plVar18;
  longlong *in_RDX;
  int iVar19;
  uint *_Src;
  uint uVar20;
  longlong in_R8;
  undefined *puVar21;
  ulonglong uVar22;
  longlong lVar23;
  ushort uVar24;
  ulonglong uVar25;
  byte bVar26;
  char cVar27;
  byte bVar28;
  char cVar30;
  char cVar31;
  char cVar32;
  undefined1 auVar29 [16];
  undefined1 auVar33 [16];
  undefined1 auVar34 [16];
  undefined1 auVar35 [16];
  undefined1 auVar36 [16];
  float fVar37;
  undefined1 unaff_XMM8_Ba;
  undefined1 unaff_XMM8_Bb;
  char cVar38;
  undefined1 unaff_XMM8_Bc;
  char cVar39;
  undefined1 unaff_XMM8_Bd;
  char cVar40;
  undefined1 unaff_XMM8_Be;
  undefined1 unaff_XMM8_Bf;
  char cVar41;
  undefined1 unaff_XMM8_Bg;
  char cVar42;
  undefined1 unaff_XMM8_Bh;
  char cVar43;
  undefined1 unaff_XMM8_Bi;
  undefined1 unaff_XMM8_Bj;
  char cVar44;
  undefined1 unaff_XMM8_Bk;
  char cVar45;
  undefined1 unaff_XMM8_Bl;
  char cVar46;
  undefined1 unaff_XMM8_Bm;
  undefined1 unaff_XMM8_Bn;
  char cVar47;
  undefined1 unaff_XMM8_Bo;
  char cVar48;
  undefined1 unaff_XMM8_Bp;
  char cVar49;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  longlong *in_stack_00000038;
  longlong *in_stack_00000040;
  ulonglong local_738 [64];
  undefined8 local_538;
  undefined4 local_530;
  undefined8 local_338;
  uint uStack_330;
  undefined4 uStack_32c;
  longlong local_328;
  longlong lStack_320;
  undefined8 local_318;
  longlong lStack_310;
  undefined4 local_308;
  undefined4 uStack_304;
  undefined8 uStack_300;
  longlong local_2f8;
  longlong lStack_2f0;
  float local_2e8;
  undefined4 uStack_2e4;
  undefined4 uStack_2e0;
  undefined4 uStack_2dc;
  undefined4 local_2d8;
  undefined4 uStack_2d4;
  longlong *local_120;
  ulonglong *local_108;
  void *local_100;
  longlong local_f8;
  longlong *local_f0;
  longlong *local_e8;
  undefined4 *local_e0;
  longlong local_d8;
  longlong local_d0;
  longlong *local_c0;
  longlong local_b8;
  longlong *local_b0;
  longlong local_a8;
  float local_a0;
  undefined4 local_9c;
  longlong *local_90;
  ulonglong *local_88;
  undefined8 local_80;
  
  local_80 = 0xfffffffffffffffe;
  plVar18 = in_RDX;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    local_d0 = 0;
  }
  else {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (plVar18 != (longlong *)0x0) {
        if (&DAT_00000038 < plVar18) {
          if (plVar18 != (longlong *)0x39) goto LAB_1422c9d02;
        }
        else if ((char)plVar18[0x286083d7] < -0x40) {
LAB_1422c9d02:
                    /* WARNING: Subroutine does not return */
          core::str::slice_error_fail();
        }
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (plVar18 != (longlong *)0xfffffffffffffffe)) {
        if ((undefined1 *)((longlong)plVar18 + 2) < (undefined1 *)0x39) {
          if (*(char *)((longlong)plVar18 + 0x143041eba) < -0x40) {
LAB_1422c9d56:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if ((undefined1 *)((longlong)plVar18 + 2) != (undefined1 *)0x39) goto LAB_1422c9d56;
      }
    }
    puVar21 = &DAT_143041ff7;
    do {
      bVar26 = puVar21[-1];
      uVar15 = (uint)(char)bVar26;
      if ((char)bVar26 < '\0') {
        bVar28 = puVar21[-2];
        if ((char)bVar28 < -0x40) {
          bVar6 = puVar21[-3];
          if ((char)bVar6 < -0x40) {
            pbVar1 = puVar21 + -4;
            puVar21 = puVar21 + -4;
            uVar15 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar21 = puVar21 + -3;
            uVar15 = bVar6 & 0xf;
          }
          uVar15 = bVar28 & 0x3f | uVar15 << 6;
        }
        else {
          puVar21 = puVar21 + -2;
          uVar15 = bVar28 & 0x1f;
        }
        uVar15 = (uint)(bVar26 & 0x3f) | uVar15 << 6;
        if (uVar15 != 0x5c) goto LAB_1422c9e14;
LAB_1422c9e25:
        if (puVar21 == &UNK_143041fbf) break;
        if (puVar21 + -0x143041fbf < (undefined *)0x37) {
          if (-0x41 < (char)puVar21[1]) break;
        }
        else if (puVar21 + -0x143041fbf == (undefined *)0x37) break;
                    /* WARNING: Subroutine does not return */
        core::str::slice_error_fail();
      }
      puVar21 = puVar21 + -1;
      if (uVar15 == 0x5c) goto LAB_1422c9e25;
LAB_1422c9e14:
      if (uVar15 == 0x2f) goto LAB_1422c9e25;
    } while (puVar21 != &anon_f5943f1eb55ef962691c302130175527_15_llvm_7117307553093096281);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar18 = extraout_RAX_01 + 1;
    }
    else {
      if (*extraout_RAX_01 != 0) goto LAB_1422ca63f;
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      plVar18 = extraout_RAX_06;
      if (extraout_RAX_06 == (longlong *)0x0) goto LAB_1422ca63f;
    }
    if (*plVar18 != 0) goto LAB_1422ca64b;
    *plVar18 = -1;
    local_88 = (ulonglong *)plVar18;
    puffin::ThreadProfiler::begin_scope();
    *local_88 = *local_88 + 1;
    local_d0 = 1;
  }
  **(undefined4 **)(in_R8 + 0x10) = *(undefined4 *)(in_R8 + 0x1c);
  tabloid::Subscriber::take<country_core::utils::dirty_tiles::DirtyTiles>();
  local_88 = local_108;
  if ((local_108 == (ulonglong *)0x0) || (local_100 == (void *)0x0)) {
    iVar19 = 0;
  }
  else {
    memcpy(&uStack_330,local_100,0x200);
    iVar19 = 1;
  }
  local_338._4_4_ = 0;
  local_338._0_4_ = iVar19;
  country_core::utils::dirty_tiles::DirtyTiles::new_clear();
  _Src = (uint *)&local_538;
  if (iVar19 != 0) {
    _Src = &uStack_330;
  }
  memcpy(local_738,_Src,0x200);
  puVar14 = local_88;
  if (local_88 != (ulonglong *)0x0) {
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    LOCK();
    uVar5 = *puVar14;
    *puVar14 = *puVar14 - 0x10;
    UNLOCK();
    if ((uVar5 & 0xfffffffffffffff2) == 0x12) {
      parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow();
    }
    lVar16 = local_f8;
    local_f8 = 0;
    local_338 = lVar16 - 0x10;
    LOCK();
    plVar18 = (longlong *)(lVar16 + -0x10);
    *plVar18 = *plVar18 + -1;
    UNLOCK();
    if (*plVar18 == 0) {
      alloc::sync::Arc::
      drop_slow<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles>_>,alloc::alloc::Global>
                ();
    }
  }
  local_88 = *(ulonglong **)(*in_stack_00000030 + 8);
  local_f0 = (longlong *)(local_88 + *(longlong *)(*in_stack_00000030 + 0x10) * 8);
  local_c0 = (longlong *)*in_stack_00000040;
  local_e8 = local_c0 + 4;
  local_9c = *(undefined4 *)((longlong)in_RDX + 0x1c);
  local_b8 = *in_RDX;
  local_e0 = (undefined4 *)in_RDX[2];
  local_d8 = local_b8 + 0x48;
  local_b0 = in_stack_00000038 + 4;
  fVar37 = (float)__xmm_00000000000000003e19999a00000000;
  uVar5 = (ulonglong)__xmm_00000000000000003e19999a00000000 >> 0x20;
  iVar19 = -(uint)(CONCAT13(unaff_XMM8_Bd,
                            CONCAT12(unaff_XMM8_Bc,CONCAT11(unaff_XMM8_Bb,unaff_XMM8_Ba))) ==
                  CONCAT13(unaff_XMM8_Bd,
                           CONCAT12(unaff_XMM8_Bc,CONCAT11(unaff_XMM8_Bb,unaff_XMM8_Ba))));
  cVar38 = (char)((uint)iVar19 >> 8);
  cVar39 = (char)((uint)iVar19 >> 0x10);
  cVar40 = (char)((uint)iVar19 >> 0x18);
  iVar11 = -(uint)(CONCAT13(unaff_XMM8_Bh,
                            CONCAT12(unaff_XMM8_Bg,CONCAT11(unaff_XMM8_Bf,unaff_XMM8_Be))) ==
                  CONCAT13(unaff_XMM8_Bh,
                           CONCAT12(unaff_XMM8_Bg,CONCAT11(unaff_XMM8_Bf,unaff_XMM8_Be))));
  cVar41 = (char)((uint)iVar11 >> 8);
  cVar42 = (char)((uint)iVar11 >> 0x10);
  cVar43 = (char)((uint)iVar11 >> 0x18);
  iVar12 = -(uint)(CONCAT13(unaff_XMM8_Bl,
                            CONCAT12(unaff_XMM8_Bk,CONCAT11(unaff_XMM8_Bj,unaff_XMM8_Bi))) ==
                  CONCAT13(unaff_XMM8_Bl,
                           CONCAT12(unaff_XMM8_Bk,CONCAT11(unaff_XMM8_Bj,unaff_XMM8_Bi))));
  cVar44 = (char)((uint)iVar12 >> 8);
  cVar45 = (char)((uint)iVar12 >> 0x10);
  cVar46 = (char)((uint)iVar12 >> 0x18);
  iVar13 = -(uint)(CONCAT13(unaff_XMM8_Bp,
                            CONCAT12(unaff_XMM8_Bo,CONCAT11(unaff_XMM8_Bn,unaff_XMM8_Bm))) ==
                  CONCAT13(unaff_XMM8_Bp,
                           CONCAT12(unaff_XMM8_Bo,CONCAT11(unaff_XMM8_Bn,unaff_XMM8_Bm))));
  cVar47 = (char)((uint)iVar13 >> 8);
  cVar48 = (char)((uint)iVar13 >> 0x10);
  cVar49 = (char)((uint)iVar13 >> 0x18);
LAB_1422ca100:
  puVar14 = local_88;
  if (local_88 != (ulonglong *)local_f0) {
    plVar18 = (longlong *)(local_88 + 8);
    plVar2 = (longlong *)(local_88 + 2);
    local_88 = (ulonglong *)plVar18;
    if (local_c0[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      bVar26 = (byte)(extraout_RAX_02 >> 0x39);
      auVar29 = ZEXT216(CONCAT11(bVar26,bVar26));
      auVar29 = pshuflw(auVar29,auVar29,0);
      lVar16 = 0;
      uVar22 = extraout_RAX_02;
      while( true ) {
        uVar22 = local_c0[1] & uVar22;
        pcVar3 = (char *)(*local_c0 + uVar22);
        cVar27 = auVar29[0];
        auVar35[0] = -(*pcVar3 == cVar27);
        cVar30 = auVar29[1];
        auVar35[1] = -(pcVar3[1] == cVar30);
        cVar31 = auVar29[2];
        auVar35[2] = -(pcVar3[2] == cVar31);
        cVar32 = auVar29[3];
        auVar35[3] = -(pcVar3[3] == cVar32);
        auVar35[4] = -(pcVar3[4] == cVar27);
        auVar35[5] = -(pcVar3[5] == cVar30);
        auVar35[6] = -(pcVar3[6] == cVar31);
        auVar35[7] = -(pcVar3[7] == cVar32);
        auVar35[8] = -(pcVar3[8] == cVar27);
        auVar35[9] = -(pcVar3[9] == cVar30);
        auVar35[10] = -(pcVar3[10] == cVar31);
        auVar35[0xb] = -(pcVar3[0xb] == cVar32);
        auVar35[0xc] = -(pcVar3[0xc] == cVar27);
        auVar35[0xd] = -(pcVar3[0xd] == cVar30);
        auVar35[0xe] = -(pcVar3[0xe] == cVar31);
        auVar35[0xf] = -(pcVar3[0xf] == cVar32);
        uVar24 = (ushort)(SUB161(auVar35 >> 7,0) & 1) | (ushort)(SUB161(auVar35 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar35 >> 0x17,0) & 1) << 2 |
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
                 (ushort)(SUB161(auVar35 >> 0x77,0) & 1) << 0xe | (ushort)(auVar35[0xf] >> 7) << 0xf
        ;
        uVar15 = (uint)uVar24;
        if (uVar24 != 0) {
          do {
            uVar20 = 0;
            for (uVar10 = uVar15; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
              uVar20 = uVar20 + 1;
            }
            if (*plVar2 == *(longlong *)(*local_c0 + (uVar20 + uVar22 & local_c0[1]) * -8 + -8))
            goto LAB_1422ca100;
            uVar24 = (ushort)(uVar15 - 1) & (ushort)uVar15;
            uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar24);
          } while (uVar24 != 0);
        }
        auVar33[0] = -(*pcVar3 == (char)iVar19);
        auVar33[1] = -(pcVar3[1] == cVar38);
        auVar33[2] = -(pcVar3[2] == cVar39);
        auVar33[3] = -(pcVar3[3] == cVar40);
        auVar33[4] = -(pcVar3[4] == (char)iVar11);
        auVar33[5] = -(pcVar3[5] == cVar41);
        auVar33[6] = -(pcVar3[6] == cVar42);
        auVar33[7] = -(pcVar3[7] == cVar43);
        auVar33[8] = -(pcVar3[8] == (char)iVar12);
        auVar33[9] = -(pcVar3[9] == cVar44);
        auVar33[10] = -(pcVar3[10] == cVar45);
        auVar33[0xb] = -(pcVar3[0xb] == cVar46);
        auVar33[0xc] = -(pcVar3[0xc] == (char)iVar13);
        auVar33[0xd] = -(pcVar3[0xd] == cVar47);
        auVar33[0xe] = -(pcVar3[0xe] == cVar48);
        auVar33[0xf] = -(pcVar3[0xf] == cVar49);
        if ((((((((((((((((SUB161(auVar33 >> 7,0) & 1) != 0 || (SUB161(auVar33 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar33 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar33 >> 0x1f,0) & 1) != 0) || (SUB161(auVar33 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar33 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar33 >> 0x37,0) & 1) != 0) || (SUB161(auVar33 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar33 >> 0x47,0) & 1) != 0) || (SUB161(auVar33 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar33 >> 0x57,0) & 1) != 0) || (SUB161(auVar33 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar33 >> 0x67,0) & 1) != 0) || (SUB161(auVar33 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar33 >> 0x77,0) & 1) != 0) || auVar33[0xf] < '\0') break;
        uVar22 = uVar22 + lVar16 + 0x10;
        lVar16 = lVar16 + 0x10;
      }
    }
    local_a0 = *(float *)(puVar14 + 6) + 0.0;
    local_a8 = CONCAT44((float)(puVar14[5] >> 0x20) + (float)uVar5,(float)puVar14[5] + fVar37);
    local_120 = &local_a8;
    country_core::utils::dirty_tiles::DirtyTiles::cell_from_world_space();
    if ((local_338 & 1) == 0) {
LAB_1422ca240:
      local_90 = (longlong *)*plVar2;
      local_338._0_4_ = (int)local_90;
      local_338._4_4_ = (undefined4)((ulonglong)local_90 >> 0x20);
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      lVar16 = *in_stack_00000038;
      bVar26 = (byte)(extraout_RAX_03 >> 0x38);
      bVar28 = bVar26 >> 1;
      auVar29 = ZEXT216(CONCAT11(bVar28,bVar28));
      auVar29 = pshuflw(auVar29,auVar29,0);
      lVar23 = 0;
      uVar22 = extraout_RAX_03;
      while( true ) {
        uVar22 = in_stack_00000038[1] & uVar22;
        pcVar3 = (char *)(lVar16 + uVar22);
        cVar27 = auVar29[0];
        auVar36[0] = -(*pcVar3 == cVar27);
        cVar30 = auVar29[1];
        auVar36[1] = -(pcVar3[1] == cVar30);
        cVar31 = auVar29[2];
        auVar36[2] = -(pcVar3[2] == cVar31);
        cVar32 = auVar29[3];
        auVar36[3] = -(pcVar3[3] == cVar32);
        auVar36[4] = -(pcVar3[4] == cVar27);
        auVar36[5] = -(pcVar3[5] == cVar30);
        auVar36[6] = -(pcVar3[6] == cVar31);
        auVar36[7] = -(pcVar3[7] == cVar32);
        auVar36[8] = -(pcVar3[8] == cVar27);
        auVar36[9] = -(pcVar3[9] == cVar30);
        auVar36[10] = -(pcVar3[10] == cVar31);
        auVar36[0xb] = -(pcVar3[0xb] == cVar32);
        auVar36[0xc] = -(pcVar3[0xc] == cVar27);
        auVar36[0xd] = -(pcVar3[0xd] == cVar30);
        auVar36[0xe] = -(pcVar3[0xe] == cVar31);
        auVar36[0xf] = -(pcVar3[0xf] == cVar32);
        uVar24 = (ushort)(SUB161(auVar36 >> 7,0) & 1) | (ushort)(SUB161(auVar36 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar36 >> 0x17,0) & 1) << 2 |
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
                 (ushort)(SUB161(auVar36 >> 0x77,0) & 1) << 0xe | (ushort)(auVar36[0xf] >> 7) << 0xf
        ;
        uVar15 = (uint)uVar24;
        while (uVar24 != 0) {
          uVar20 = 0;
          for (uVar10 = uVar15; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
            uVar20 = uVar20 + 1;
          }
          uVar25 = uVar20 + uVar22 & in_stack_00000038[1];
          if (*(longlong **)(lVar16 + -0x10 + uVar25 * -0x10) == local_90) {
            if (*(char *)(lVar16 + uVar25 * -0x10 + -8) != '\0') goto LAB_1422ca100;
            goto LAB_1422ca3dc;
          }
          uVar24 = (ushort)(uVar15 - 1) & (ushort)uVar15;
          uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar24);
        }
        auVar34[0] = -(*pcVar3 == (char)iVar19);
        auVar34[1] = -(pcVar3[1] == cVar38);
        auVar34[2] = -(pcVar3[2] == cVar39);
        auVar34[3] = -(pcVar3[3] == cVar40);
        auVar34[4] = -(pcVar3[4] == (char)iVar11);
        auVar34[5] = -(pcVar3[5] == cVar41);
        auVar34[6] = -(pcVar3[6] == cVar42);
        auVar34[7] = -(pcVar3[7] == cVar43);
        auVar34[8] = -(pcVar3[8] == (char)iVar12);
        auVar34[9] = -(pcVar3[9] == cVar44);
        auVar34[10] = -(pcVar3[10] == cVar45);
        auVar34[0xb] = -(pcVar3[0xb] == cVar46);
        auVar34[0xc] = -(pcVar3[0xc] == (char)iVar13);
        auVar34[0xd] = -(pcVar3[0xd] == cVar47);
        auVar34[0xe] = -(pcVar3[0xe] == cVar48);
        auVar34[0xf] = -(pcVar3[0xf] == cVar49);
        if ((((((((((((((((SUB161(auVar34 >> 7,0) & 1) != 0 || (SUB161(auVar34 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar34 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar34 >> 0x1f,0) & 1) != 0) || (SUB161(auVar34 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar34 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar34 >> 0x37,0) & 1) != 0) || (SUB161(auVar34 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar34 >> 0x47,0) & 1) != 0) || (SUB161(auVar34 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar34 >> 0x57,0) & 1) != 0) || (SUB161(auVar34 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar34 >> 0x67,0) & 1) != 0) || (SUB161(auVar34 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar34 >> 0x77,0) & 1) != 0) || auVar34[0xf] < '\0') break;
        uVar22 = uVar22 + lVar23 + 0x10;
        lVar23 = lVar23 + 0x10;
      }
      if (in_stack_00000038[2] == 0) {
        hashbrown::raw::RawTable::
        reserve_rehash<tuple_<country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState>_>
                  ();
      }
      add_birdnests::closure_0();
      pauVar7 = (undefined1 (*) [16])*in_stack_00000038;
      uVar22 = in_stack_00000038[1];
      uVar25 = extraout_RAX_03 & uVar22;
      auVar29 = *(undefined1 (*) [16])(*pauVar7 + uVar25);
      uVar24 = (ushort)(SUB161(auVar29 >> 7,0) & 1) | (ushort)(SUB161(auVar29 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar29 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar29 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar29 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar29 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar29 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar29 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar29 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar29 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar29 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar29 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar29 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar29 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar29 >> 0x77,0) & 1) << 0xe |
               (ushort)(byte)(auVar29[0xf] >> 7) << 0xf;
      uVar15 = (uint)uVar24;
      if (uVar24 == 0) {
        lVar16 = 0x10;
        do {
          uVar25 = uVar25 + lVar16 & uVar22;
          auVar29 = *(undefined1 (*) [16])(*pauVar7 + uVar25);
          lVar16 = lVar16 + 0x10;
          uVar15 = (uint)(ushort)((ushort)(SUB161(auVar29 >> 7,0) & 1) |
                                  (ushort)(SUB161(auVar29 >> 0xf,0) & 1) << 1 |
                                  (ushort)(SUB161(auVar29 >> 0x17,0) & 1) << 2 |
                                  (ushort)(SUB161(auVar29 >> 0x1f,0) & 1) << 3 |
                                  (ushort)(SUB161(auVar29 >> 0x27,0) & 1) << 4 |
                                  (ushort)(SUB161(auVar29 >> 0x2f,0) & 1) << 5 |
                                  (ushort)(SUB161(auVar29 >> 0x37,0) & 1) << 6 |
                                  (ushort)(SUB161(auVar29 >> 0x3f,0) & 1) << 7 |
                                  (ushort)(SUB161(auVar29 >> 0x47,0) & 1) << 8 |
                                  (ushort)(SUB161(auVar29 >> 0x4f,0) & 1) << 9 |
                                  (ushort)(SUB161(auVar29 >> 0x57,0) & 1) << 10 |
                                  (ushort)(SUB161(auVar29 >> 0x5f,0) & 1) << 0xb |
                                  (ushort)(SUB161(auVar29 >> 0x67,0) & 1) << 0xc |
                                  (ushort)(SUB161(auVar29 >> 0x6f,0) & 1) << 0xd |
                                  (ushort)(SUB161(auVar29 >> 0x77,0) & 1) << 0xe |
                                 (ushort)(byte)(auVar29[0xf] >> 7) << 0xf);
        } while (uVar15 == 0);
      }
      uVar20 = 0;
      for (; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
        uVar20 = uVar20 + 1;
      }
      uVar25 = uVar20 + uVar25 & uVar22;
      bVar28 = (*pauVar7)[uVar25];
      if (-1 < (char)bVar28) {
        auVar29 = *pauVar7;
        uVar15 = 0;
        for (uVar20 = (uint)(ushort)((ushort)(SUB161(auVar29 >> 7,0) & 1) |
                                     (ushort)(SUB161(auVar29 >> 0xf,0) & 1) << 1 |
                                     (ushort)(SUB161(auVar29 >> 0x17,0) & 1) << 2 |
                                     (ushort)(SUB161(auVar29 >> 0x1f,0) & 1) << 3 |
                                     (ushort)(SUB161(auVar29 >> 0x27,0) & 1) << 4 |
                                     (ushort)(SUB161(auVar29 >> 0x2f,0) & 1) << 5 |
                                     (ushort)(SUB161(auVar29 >> 0x37,0) & 1) << 6 |
                                     (ushort)(SUB161(auVar29 >> 0x3f,0) & 1) << 7 |
                                     (ushort)(SUB161(auVar29 >> 0x47,0) & 1) << 8 |
                                     (ushort)(SUB161(auVar29 >> 0x4f,0) & 1) << 9 |
                                     (ushort)(SUB161(auVar29 >> 0x57,0) & 1) << 10 |
                                     (ushort)(SUB161(auVar29 >> 0x5f,0) & 1) << 0xb |
                                     (ushort)(SUB161(auVar29 >> 0x67,0) & 1) << 0xc |
                                     (ushort)(SUB161(auVar29 >> 0x6f,0) & 1) << 0xd |
                                     (ushort)(SUB161(auVar29 >> 0x77,0) & 1) << 0xe |
                                    (ushort)(byte)(auVar29[0xf] >> 7) << 0xf); (uVar20 & 1) == 0;
            uVar20 = uVar20 >> 1 | 0x80000000) {
          uVar15 = uVar15 + 1;
        }
        uVar25 = (ulonglong)uVar15;
        bVar28 = (*pauVar7)[uVar25];
      }
      bVar26 = bVar26 >> 1;
      (*pauVar7)[uVar25] = bVar26;
      pauVar7[1][uVar25 - 0x10 & uVar22] = bVar26;
      in_stack_00000038[2] = in_stack_00000038[2] - (ulonglong)(bVar28 & 1);
      *(longlong **)pauVar7[-1 - uVar25] = local_90;
      pauVar7[-1 - uVar25][8] = extraout_AL_01;
      in_stack_00000038[3] = in_stack_00000038[3] + 1;
      cVar27 = pauVar7[-1 - uVar25][8];
    }
    else {
      if (uStack_330 >> 6 != 0) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_bounds_check();
      }
      if (0x3fffffffff < local_338) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_bounds_check();
      }
      if ((local_738[uStack_330] >> (local_338 >> 0x20 & 0x3f) & 1) == 0) goto LAB_1422ca240;
      add_birdnests::closure_0();
      hashbrown::map::HashMap::insert<u64,bool,std::hash::random::RandomState,alloc::alloc::Global>
                ();
      cVar27 = extraout_AL_00;
    }
    if (cVar27 == '\0') {
LAB_1422ca3dc:
      *local_e0 = local_9c;
      lVar23 = puVar14[2];
      uVar8 = *(undefined8 *)(*in_stack_00000028 + 0x60);
      local_530 = *(undefined4 *)((longlong)puVar14 + 0x3c);
      local_538 = *(undefined8 *)((longlong)puVar14 + 0x34);
      plVar18 = (longlong *)*puVar14;
      lVar17 = puVar14[1];
      LOCK();
      lVar16 = *plVar18;
      *plVar18 = *plVar18 + 1;
      UNLOCK();
      if (*plVar18 == 0 || SCARRY8(lVar16,1) != *plVar18 < 0) {
                    /* WARNING: Does not return */
        pcVar9 = (code *)invalidInstructionException();
        (*pcVar9)();
      }
      lStack_310 = puVar14[3];
      local_2e8 = local_a0;
      lStack_2f0 = local_a8;
      uStack_2e4 = (undefined4)local_538;
      uStack_2e0 = (undefined4)((ulonglong)local_538 >> 0x20);
      local_2d8 = 0x3f800000;
      local_308 = CONCAT31(local_308._1_3_,3);
      uStack_304 = 0;
      local_338._0_4_ = 4;
      local_338._4_4_ = 0;
      uStack_330 = (uint)plVar18;
      uStack_32c = (undefined4)((ulonglong)plVar18 >> 0x20);
      lStack_320 = 0;
      lVar16 = *(longlong *)(local_b8 + 0x58);
      local_328 = lVar17;
      uStack_300 = uVar8;
      local_2f8 = lVar23;
      uStack_2dc = local_530;
      local_90 = plVar18;
      if (lVar16 == *(longlong *)(local_b8 + 0x48)) {
        alloc::raw_vec::RawVec::
        grow_one<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>
                  ();
      }
      lVar23 = *(longlong *)(local_b8 + 0x50);
      lVar17 = lVar16 * 0x68;
      *(ulonglong *)(lVar23 + 0x60 + lVar17) = CONCAT44(uStack_2d4,local_2d8);
      puVar4 = (undefined8 *)(lVar23 + 0x50 + lVar17);
      *puVar4 = CONCAT44(uStack_2e4,local_2e8);
      puVar4[1] = CONCAT44(uStack_2dc,uStack_2e0);
      plVar18 = (longlong *)(lVar23 + 0x40 + lVar17);
      *plVar18 = local_2f8;
      plVar18[1] = lStack_2f0;
      puVar4 = (undefined8 *)(lVar23 + 0x30 + lVar17);
      *puVar4 = CONCAT44(uStack_304,local_308);
      puVar4[1] = uStack_300;
      puVar4 = (undefined8 *)(lVar23 + 0x20 + lVar17);
      *puVar4 = local_318;
      puVar4[1] = lStack_310;
      plVar18 = (longlong *)(lVar23 + 0x10 + lVar17);
      *plVar18 = local_328;
      plVar18[1] = lStack_320;
      *(ulonglong *)(lVar23 + lVar17) = local_338;
      ((ulonglong *)(lVar23 + lVar17))[1] = CONCAT44(uStack_32c,uStack_330);
      *(longlong *)(local_b8 + 0x58) = lVar16 + 1;
    }
    goto LAB_1422ca100;
  }
  if (local_d0 != 0) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_04 == 1) {
      plVar18 = extraout_RAX_04 + 1;
    }
    else {
      if (*extraout_RAX_04 != 0) {
LAB_1422ca63f:
                    /* WARNING: Subroutine does not return */
        std::thread::local::panic_access_error();
      }
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      plVar18 = extraout_RAX_05;
      if (extraout_RAX_05 == (longlong *)0x0) goto LAB_1422ca63f;
    }
    if (*plVar18 != 0) {
LAB_1422ca64b:
                    /* WARNING: Subroutine does not return */
      core::cell::panic_already_borrowed();
    }
    *plVar18 = -1;
    local_88 = (ulonglong *)plVar18;
    puffin::ThreadProfiler::end_scope();
    *local_88 = *local_88 + 1;
  }
  return;
}

