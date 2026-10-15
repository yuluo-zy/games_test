// ADDR 0x1408260d0
// SIG void __cdecl clear_blueprints(void)
// ORIG system::clear_blueprints


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl system_decorator::blueprint::clear_blueprints(void)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  undefined4 uVar5;
  longlong lVar6;
  undefined4 *puVar7;
  ushort uVar8;
  int iVar9;
  bool bVar10;
  undefined1 auVar11 [16];
  undefined1 auVar12 [16];
  char extraout_AL;
  uint uVar13;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong in_RCX;
  ulonglong uVar14;
  longlong *in_RDX;
  ulonglong uVar15;
  ulonglong uVar16;
  longlong *plVar17;
  longlong lVar18;
  undefined1 (*pauVar19) [16];
  longlong *plVar20;
  undefined *puVar21;
  ulonglong uVar22;
  undefined1 (*pauVar23) [16];
  uint uVar24;
  uint uVar25;
  bool bVar26;
  undefined1 auVar27 [16];
  undefined1 auVar28 [16];
  undefined1 auVar29 [16];
  undefined1 auVar30 [16];
  undefined1 auVar31 [16];
  undefined1 auVar32 [16];
  longlong local_110;
  longlong local_108;
  longlong local_100;
  longlong *local_f8;
  undefined1 (*pauStack_f0) [16];
  undefined1 local_e8 [16];
  undefined1 local_d8 [24];
  ushort uStack_c0;
  longlong local_b8;
  ulonglong local_a8;
  longlong local_a0;
  uint local_8c;
  
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar10 = false;
  }
  else {
    uVar15 = 0x2d;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar15 != 0) {
        if (uVar15 < 0x2d) {
          if ((char)(&DAT_142a9145d)[uVar15] < -0x40) {
LAB_140826159:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar15 != 0x2d) goto LAB_140826159;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar15 != 0xfffffffffffffffe)) {
        if (uVar15 + 2 < 0x2d) {
          if ((char)(&UNK_142a9145f)[uVar15] < -0x40) {
LAB_1408261b7:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar15 + 2 != 0x2d) goto LAB_1408261b7;
      }
    }
    puVar21 = &DAT_142a91679;
    do {
      bVar2 = puVar21[-1];
      uVar13 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar21[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar21[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar21 + -4;
            puVar21 = puVar21 + -4;
            uVar13 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar21 = puVar21 + -3;
            uVar13 = bVar4 & 0xf;
          }
          uVar13 = bVar3 & 0x3f | uVar13 << 6;
        }
        else {
          puVar21 = puVar21 + -2;
          uVar13 = bVar3 & 0x1f;
        }
        uVar13 = (uint)(bVar2 & 0x3f) | uVar13 << 6;
        if (uVar13 != 0x5c) goto LAB_140826274;
LAB_140826286:
        if (puVar21 == &UNK_142a9164f) break;
        if (puVar21 + -0x142a9164f < (undefined *)0x29) {
          if (-0x41 < (char)puVar21[1]) break;
        }
        else if (puVar21 + -0x142a9164f == (undefined *)0x29) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar21 = puVar21 + -1;
      if (uVar13 == 0x5c) goto LAB_140826286;
LAB_140826274:
      if (uVar13 == 0x2f) goto LAB_140826286;
    } while (puVar21 != &anon_607a6278b2141009247d0776e3b766a7_27_llvm_14222842113206979732);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar20 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar20 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0))
    goto LAB_140826c0f;
    if (*plVar20 != 0) goto LAB_140826c1b;
    *plVar20 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar20 = *plVar20 + 1;
    bVar10 = true;
  }
  **(undefined4 **)(in_RCX + 0x10) = *(undefined4 *)(in_RCX + 0x1c);
  country_core::resources::walls::decorator_storage::DecoratorStorage::iter_mut();
  auVar12 = ___xmm_7465646279746573646f72616e646f6d;
  auVar11 = ___xmm_6c7967656e657261736f6d6570736575;
  bVar2 = (byte)local_f8;
  local_110 = local_b8;
  uVar5 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  lVar6 = *in_RDX;
  puVar7 = (undefined4 *)in_RDX[2];
  local_a8 = local_e8._0_8_;
  local_a0 = local_e8._8_8_;
  local_108 = local_d8._0_8_;
  pauVar23 = (undefined1 (*) [16])local_d8._8_8_;
  local_100 = (longlong)pauStack_f0;
  uVar13 = (uint)uStack_c0;
LAB_1408263c3:
  if (local_a8 == 2) {
LAB_1408264b6:
    if ((local_100 != 0 & bVar2) != 1) goto LAB_1408268fb;
    local_a8 = 2;
    local_100 = 0;
    local_8c = uVar13;
  }
  else {
    if (local_108 == 0) {
joined_r0x0001408264ae:
      if ((local_a8 & 1) == 0) {
                    // WARNING: Read-only address (ram,0x0001429cd080) is written
                    // WARNING: Read-only address (ram,0x0001429cd090) is written
        local_108 = 0;
      }
      else {
                    // WARNING: Read-only address (ram,0x0001429cd080) is written
                    // WARNING: Read-only address (ram,0x0001429cd090) is written
        local_108 = 0;
        bVar26 = local_a0 != 0;
        local_a0 = 0;
        if (bVar26) {
          local_108 = 0;
          local_a0 = 0;
          local_8c = uVar13;
          goto LAB_1408264e7;
        }
      }
      goto LAB_1408264b6;
    }
    if (local_110 == 0) {
      local_110 = 0;
      goto joined_r0x0001408264ae;
    }
    if ((short)uVar13 == 0) {
      do {
        auVar27 = *pauVar23;
        uVar8 = (ushort)(SUB161(auVar27 >> 7,0) & 1) | (ushort)(SUB161(auVar27 >> 0xf,0) & 1) << 1 |
                (ushort)(SUB161(auVar27 >> 0x17,0) & 1) << 2 |
                (ushort)(SUB161(auVar27 >> 0x1f,0) & 1) << 3 |
                (ushort)(SUB161(auVar27 >> 0x27,0) & 1) << 4 |
                (ushort)(SUB161(auVar27 >> 0x2f,0) & 1) << 5 |
                (ushort)(SUB161(auVar27 >> 0x37,0) & 1) << 6 |
                (ushort)(SUB161(auVar27 >> 0x3f,0) & 1) << 7 |
                (ushort)(SUB161(auVar27 >> 0x47,0) & 1) << 8 |
                (ushort)(SUB161(auVar27 >> 0x4f,0) & 1) << 9 |
                (ushort)(SUB161(auVar27 >> 0x57,0) & 1) << 10 |
                (ushort)(SUB161(auVar27 >> 0x5f,0) & 1) << 0xb |
                (ushort)(SUB161(auVar27 >> 0x67,0) & 1) << 0xc |
                (ushort)(SUB161(auVar27 >> 0x6f,0) & 1) << 0xd |
                (ushort)(SUB161(auVar27 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar27[0xf] >> 7) << 0xf;
        local_108 = local_108 + -0x980;
        pauVar23 = pauVar23 + 1;
      } while (uVar8 == 0xffff);
      uVar13 = ~(uint)uVar8;
      local_8c = uVar13 & -(uint)uVar8 - 2;
    }
    else {
      local_8c = uVar13 - 1 & uVar13;
    }
    for (; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
    }
    country_core::resources::walls::decorator_storage::impl$16::from();
    local_110 = local_110 + -1;
    uVar13 = local_8c;
    if (extraout_RAX_02 == 3) goto joined_r0x0001408264ae;
  }
LAB_1408264e7:
  country_core::resources::walls::decorator_storage::Decorators::iter_modified_ids();
  lVar18 = local_d8._0_8_;
  plVar20 = local_f8;
  pauVar19 = pauStack_f0;
  uVar13 = (uint)(ushort)local_e8._8_2_;
  while (bVar26 = lVar18 != 0, lVar18 = lVar18 + -1, bVar26) {
    if ((short)uVar13 == 0) {
      do {
        auVar27 = *pauVar19;
        uVar8 = (ushort)(SUB161(auVar27 >> 7,0) & 1) | (ushort)(SUB161(auVar27 >> 0xf,0) & 1) << 1 |
                (ushort)(SUB161(auVar27 >> 0x17,0) & 1) << 2 |
                (ushort)(SUB161(auVar27 >> 0x1f,0) & 1) << 3 |
                (ushort)(SUB161(auVar27 >> 0x27,0) & 1) << 4 |
                (ushort)(SUB161(auVar27 >> 0x2f,0) & 1) << 5 |
                (ushort)(SUB161(auVar27 >> 0x37,0) & 1) << 6 |
                (ushort)(SUB161(auVar27 >> 0x3f,0) & 1) << 7 |
                (ushort)(SUB161(auVar27 >> 0x47,0) & 1) << 8 |
                (ushort)(SUB161(auVar27 >> 0x4f,0) & 1) << 9 |
                (ushort)(SUB161(auVar27 >> 0x57,0) & 1) << 10 |
                (ushort)(SUB161(auVar27 >> 0x5f,0) & 1) << 0xb |
                (ushort)(SUB161(auVar27 >> 0x67,0) & 1) << 0xc |
                (ushort)(SUB161(auVar27 >> 0x6f,0) & 1) << 0xd |
                (ushort)(SUB161(auVar27 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar27[0xf] >> 7) << 0xf;
        plVar20 = (longlong *)((longlong)plVar20 + -0x40);
        pauVar19 = pauVar19 + 1;
      } while (uVar8 == 0xffff);
      uVar13 = ~(uint)uVar8;
      uVar24 = uVar13 & -(uint)uVar8 - 2;
    }
    else {
      if (plVar20 == (longlong *)0x0) break;
      uVar24 = uVar13 - 1 & uVar13;
    }
    for (; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
    }
    *puVar7 = uVar5;
    local_d8._0_16_ = *(undefined1 (*) [16])(lVar6 + 0x20);
    auVar27._0_8_ = local_d8._0_8_;
    auVar27._8_4_ = local_d8._0_4_;
    auVar27._12_4_ = local_d8._4_4_;
    auVar28._0_8_ = local_d8._8_8_;
    auVar28._8_4_ = local_d8._8_4_;
    auVar28._12_4_ = local_d8._12_4_;
    local_e8 = auVar28 ^ auVar12;
    core::hash::sip::impl$5::write<core::hash::sip::Sip13Rounds>();
    local_f8 = SUB168(auVar27 ^ auVar11,0);
    uVar16 = (longlong)local_f8 + local_e8._0_8_;
    pauStack_f0 = SUB168(auVar27 ^ auVar11,8);
    uVar14 = (local_e8._0_8_ << 0xd | (ulonglong)local_e8._0_8_ >> 0x33) ^ uVar16;
    uVar15 = (local_e8._8_8_ << 0x10 | (ulonglong)local_e8._8_8_ >> 0x30) ^
             (longlong)pauStack_f0 + local_e8._8_8_;
    uVar22 = (longlong)pauStack_f0 + local_e8._8_8_ + uVar14;
    uVar16 = (uVar16 << 0x20 | uVar16 >> 0x20) + uVar15;
    uVar14 = (uVar14 << 0x11 | uVar14 >> 0x2f) ^ uVar22;
    uVar15 = (uVar15 << 0x15 | uVar15 >> 0x2b) ^ uVar16;
    uVar16 = uVar16 + uVar14;
    uVar22 = ((uVar22 << 0x20 | uVar22 >> 0x20) ^ 0xff) + uVar15;
    uVar14 = (uVar14 << 0xd | uVar14 >> 0x33) ^ uVar16;
    uVar15 = (uVar15 << 0x10 | uVar15 >> 0x30) ^ uVar22;
    uVar22 = uVar22 + uVar14;
    uVar16 = (uVar16 << 0x20 | uVar16 >> 0x20) + uVar15;
    uVar14 = (uVar14 << 0x11 | uVar14 >> 0x2f) ^ uVar22;
    uVar15 = (uVar15 << 0x15 | uVar15 >> 0x2b) ^ uVar16;
    uVar16 = uVar16 + uVar14;
    uVar22 = (uVar22 << 0x20 | uVar22 >> 0x20) + uVar15;
    uVar14 = (uVar14 << 0xd | uVar14 >> 0x33) ^ uVar16;
    uVar15 = (uVar15 << 0x10 | uVar15 >> 0x30) ^ uVar22;
    uVar22 = uVar22 + uVar14;
    uVar16 = (uVar16 << 0x20 | uVar16 >> 0x20) + uVar15;
    uVar14 = (uVar14 << 0x11 | uVar14 >> 0x2f) ^ uVar22;
    uVar15 = (uVar15 << 0x15 | uVar15 >> 0x2b) ^ uVar16;
    uVar22 = (uVar22 << 0x20 | uVar22 >> 0x20) + uVar15;
    uVar14 = (uVar14 << 0xd | uVar14 >> 0x33) ^ uVar16 + uVar14;
    uVar15 = (uVar15 << 0x10 | uVar15 >> 0x30) ^ uVar22;
    uVar22 = uVar22 + uVar14;
    local_f8 = (longlong *)
               ((uVar22 << 0x20 | uVar22 >> 0x20) ^ (uVar14 << 0x11 | uVar14 >> 0x2f) ^
                (uVar15 << 0x15 | uVar15 >> 0x2b) ^ uVar22);
    hashbrown::raw::RawTable::
    remove_entry<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::sync::Arc<system_decorator::blueprint::DecoratorBlueprint,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,alloc::sync::Arc<system_decorator::blueprint::DecoratorBlueprint,alloc::alloc::Global>_>_>
              ();
    uVar13 = uVar24;
    if (local_f8 != (longlong *)0x0) {
      LOCK();
      *local_f8 = *local_f8 + -1;
      UNLOCK();
      if (*local_f8 == 0) {
        alloc::sync::Arc::
        drop_slow<system_decorator::blueprint::DecoratorBlueprint,alloc::alloc::Global>();
      }
    }
  }
  country_core::resources::walls::decorator_storage::Decorators::iter_removed_ids();
  pauVar19 = pauStack_f0;
  plVar20 = local_f8;
  lVar18 = local_d8._0_8_;
  uVar24 = (uint)(ushort)local_e8._8_2_;
  while (bVar26 = lVar18 != 0, lVar18 = lVar18 + -1, uVar13 = local_8c, bVar26) {
    if ((short)uVar24 == 0) {
      do {
        auVar27 = *pauVar19;
        uVar8 = (ushort)(SUB161(auVar27 >> 7,0) & 1) | (ushort)(SUB161(auVar27 >> 0xf,0) & 1) << 1 |
                (ushort)(SUB161(auVar27 >> 0x17,0) & 1) << 2 |
                (ushort)(SUB161(auVar27 >> 0x1f,0) & 1) << 3 |
                (ushort)(SUB161(auVar27 >> 0x27,0) & 1) << 4 |
                (ushort)(SUB161(auVar27 >> 0x2f,0) & 1) << 5 |
                (ushort)(SUB161(auVar27 >> 0x37,0) & 1) << 6 |
                (ushort)(SUB161(auVar27 >> 0x3f,0) & 1) << 7 |
                (ushort)(SUB161(auVar27 >> 0x47,0) & 1) << 8 |
                (ushort)(SUB161(auVar27 >> 0x4f,0) & 1) << 9 |
                (ushort)(SUB161(auVar27 >> 0x57,0) & 1) << 10 |
                (ushort)(SUB161(auVar27 >> 0x5f,0) & 1) << 0xb |
                (ushort)(SUB161(auVar27 >> 0x67,0) & 1) << 0xc |
                (ushort)(SUB161(auVar27 >> 0x6f,0) & 1) << 0xd |
                (ushort)(SUB161(auVar27 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar27[0xf] >> 7) << 0xf;
        plVar20 = (longlong *)((longlong)plVar20 + -0x40);
        pauVar19 = pauVar19 + 1;
      } while (uVar8 == 0xffff);
      uVar24 = ~(uint)uVar8;
      uVar25 = uVar24 & -(uint)uVar8 - 2;
    }
    else {
      uVar25 = uVar24 - 1 & uVar24;
      if (plVar20 == (longlong *)0x0) break;
    }
    for (; (uVar24 & 1) == 0; uVar24 = uVar24 >> 1 | 0x80000000) {
    }
    *puVar7 = uVar5;
    local_d8._0_16_ = *(undefined1 (*) [16])(lVar6 + 0x20);
    auVar29._0_8_ = local_d8._0_8_;
    auVar29._8_4_ = local_d8._0_4_;
    auVar29._12_4_ = local_d8._4_4_;
    auVar30._0_8_ = local_d8._8_8_;
    auVar30._8_4_ = local_d8._8_4_;
    auVar30._12_4_ = local_d8._12_4_;
    local_e8 = auVar30 ^ auVar12;
    core::hash::sip::impl$5::write<core::hash::sip::Sip13Rounds>();
    local_f8 = SUB168(auVar29 ^ auVar11,0);
    uVar16 = (longlong)local_f8 + local_e8._0_8_;
    pauStack_f0 = SUB168(auVar29 ^ auVar11,8);
    uVar14 = (local_e8._0_8_ << 0xd | (ulonglong)local_e8._0_8_ >> 0x33) ^ uVar16;
    uVar15 = (local_e8._8_8_ << 0x10 | (ulonglong)local_e8._8_8_ >> 0x30) ^
             (longlong)pauStack_f0 + local_e8._8_8_;
    uVar22 = (longlong)pauStack_f0 + local_e8._8_8_ + uVar14;
    uVar16 = (uVar16 << 0x20 | uVar16 >> 0x20) + uVar15;
    uVar14 = (uVar14 << 0x11 | uVar14 >> 0x2f) ^ uVar22;
    uVar15 = (uVar15 << 0x15 | uVar15 >> 0x2b) ^ uVar16;
    uVar16 = uVar16 + uVar14;
    uVar22 = ((uVar22 << 0x20 | uVar22 >> 0x20) ^ 0xff) + uVar15;
    uVar14 = (uVar14 << 0xd | uVar14 >> 0x33) ^ uVar16;
    uVar15 = (uVar15 << 0x10 | uVar15 >> 0x30) ^ uVar22;
    uVar22 = uVar22 + uVar14;
    uVar16 = (uVar16 << 0x20 | uVar16 >> 0x20) + uVar15;
    uVar14 = (uVar14 << 0x11 | uVar14 >> 0x2f) ^ uVar22;
    uVar15 = (uVar15 << 0x15 | uVar15 >> 0x2b) ^ uVar16;
    uVar16 = uVar16 + uVar14;
    uVar22 = (uVar22 << 0x20 | uVar22 >> 0x20) + uVar15;
    uVar14 = (uVar14 << 0xd | uVar14 >> 0x33) ^ uVar16;
    uVar15 = (uVar15 << 0x10 | uVar15 >> 0x30) ^ uVar22;
    uVar22 = uVar22 + uVar14;
    uVar16 = (uVar16 << 0x20 | uVar16 >> 0x20) + uVar15;
    uVar14 = (uVar14 << 0x11 | uVar14 >> 0x2f) ^ uVar22;
    uVar15 = (uVar15 << 0x15 | uVar15 >> 0x2b) ^ uVar16;
    uVar22 = (uVar22 << 0x20 | uVar22 >> 0x20) + uVar15;
    uVar14 = (uVar14 << 0xd | uVar14 >> 0x33) ^ uVar16 + uVar14;
    uVar15 = (uVar15 << 0x10 | uVar15 >> 0x30) ^ uVar22;
    uVar22 = uVar22 + uVar14;
    local_f8 = (longlong *)
               ((uVar22 << 0x20 | uVar22 >> 0x20) ^ (uVar14 << 0x11 | uVar14 >> 0x2f) ^
                (uVar15 << 0x15 | uVar15 >> 0x2b) ^ uVar22);
    hashbrown::raw::RawTable::
    remove_entry<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::sync::Arc<system_decorator::blueprint::DecoratorBlueprint,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,alloc::sync::Arc<system_decorator::blueprint::DecoratorBlueprint,alloc::alloc::Global>_>_>
              ();
    uVar24 = uVar25;
    if (local_f8 != (longlong *)0x0) {
      LOCK();
      *local_f8 = *local_f8 + -1;
      UNLOCK();
      if (*local_f8 == 0) {
        alloc::sync::Arc::
        drop_slow<system_decorator::blueprint::DecoratorBlueprint,alloc::alloc::Global>();
      }
    }
  }
  goto LAB_1408263c3;
LAB_1408268fb:
  country_core::resources::walls::decorator_storage::DecoratorStorage::iter_removed_storages();
  uVar13 = (uint)(ushort)local_e8._8_2_;
  plVar20 = local_f8;
  pauVar23 = pauStack_f0;
  while (bVar26 = local_d8._0_8_ != 0, local_d8._0_8_ = local_d8._0_8_ + -1, bVar26) {
    if ((short)uVar13 == 0) {
      do {
        auVar27 = *pauVar23;
        uVar8 = (ushort)(SUB161(auVar27 >> 7,0) & 1) | (ushort)(SUB161(auVar27 >> 0xf,0) & 1) << 1 |
                (ushort)(SUB161(auVar27 >> 0x17,0) & 1) << 2 |
                (ushort)(SUB161(auVar27 >> 0x1f,0) & 1) << 3 |
                (ushort)(SUB161(auVar27 >> 0x27,0) & 1) << 4 |
                (ushort)(SUB161(auVar27 >> 0x2f,0) & 1) << 5 |
                (ushort)(SUB161(auVar27 >> 0x37,0) & 1) << 6 |
                (ushort)(SUB161(auVar27 >> 0x3f,0) & 1) << 7 |
                (ushort)(SUB161(auVar27 >> 0x47,0) & 1) << 8 |
                (ushort)(SUB161(auVar27 >> 0x4f,0) & 1) << 9 |
                (ushort)(SUB161(auVar27 >> 0x57,0) & 1) << 10 |
                (ushort)(SUB161(auVar27 >> 0x5f,0) & 1) << 0xb |
                (ushort)(SUB161(auVar27 >> 0x67,0) & 1) << 0xc |
                (ushort)(SUB161(auVar27 >> 0x6f,0) & 1) << 0xd |
                (ushort)(SUB161(auVar27 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar27[0xf] >> 7) << 0xf;
        plVar20 = (longlong *)((longlong)plVar20 + -0x400);
        pauVar23 = pauVar23 + 1;
      } while (uVar8 == 0xffff);
      uVar24 = ~(uint)uVar8;
      uVar13 = uVar24 & -(uint)uVar8 - 2;
    }
    else {
      if (plVar20 == (longlong *)0x0) break;
      uVar24 = uVar13;
      uVar13 = uVar13 - 1 & uVar13;
    }
    iVar9 = 0;
    for (; (uVar24 & 1) == 0; uVar24 = uVar24 >> 1 | 0x80000000) {
      iVar9 = iVar9 + 1;
    }
    lVar18 = (longlong)plVar20 - (ulonglong)(uint)(iVar9 << 6);
    pauVar19 = *(undefined1 (**) [16])(lVar18 + -0x30);
    lVar18 = *(longlong *)(lVar18 + -0x18);
    auVar27 = *pauVar19;
    uVar24 = ~(uint)(ushort)((ushort)(SUB161(auVar27 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar27 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar27 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar27 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar27 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar27 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar27 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar27 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar27 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar27 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar27 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar27 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar27 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar27 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar27 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar27[0xf] >> 7) << 0xf);
    pauVar19 = pauVar19 + 1;
    while (bVar26 = lVar18 != 0, lVar18 = lVar18 + -1, bVar26) {
      uVar25 = uVar24;
      if ((short)uVar24 == 0) {
        do {
          auVar27 = *pauVar19;
          pauVar19 = pauVar19 + 1;
          uVar24 = (uint)(ushort)((ushort)(SUB161(auVar27 >> 7,0) & 1) |
                                  (ushort)(SUB161(auVar27 >> 0xf,0) & 1) << 1 |
                                  (ushort)(SUB161(auVar27 >> 0x17,0) & 1) << 2 |
                                  (ushort)(SUB161(auVar27 >> 0x1f,0) & 1) << 3 |
                                  (ushort)(SUB161(auVar27 >> 0x27,0) & 1) << 4 |
                                  (ushort)(SUB161(auVar27 >> 0x2f,0) & 1) << 5 |
                                  (ushort)(SUB161(auVar27 >> 0x37,0) & 1) << 6 |
                                  (ushort)(SUB161(auVar27 >> 0x3f,0) & 1) << 7 |
                                  (ushort)(SUB161(auVar27 >> 0x47,0) & 1) << 8 |
                                  (ushort)(SUB161(auVar27 >> 0x4f,0) & 1) << 9 |
                                  (ushort)(SUB161(auVar27 >> 0x57,0) & 1) << 10 |
                                  (ushort)(SUB161(auVar27 >> 0x5f,0) & 1) << 0xb |
                                  (ushort)(SUB161(auVar27 >> 0x67,0) & 1) << 0xc |
                                  (ushort)(SUB161(auVar27 >> 0x6f,0) & 1) << 0xd |
                                  (ushort)(SUB161(auVar27 >> 0x77,0) & 1) << 0xe |
                                 (ushort)(byte)(auVar27[0xf] >> 7) << 0xf);
        } while (uVar24 == 0xffff);
        uVar24 = ~uVar24;
        uVar25 = uVar24;
      }
      for (; (uVar24 & 1) == 0; uVar24 = uVar24 >> 1 | 0x80000000) {
      }
      *puVar7 = uVar5;
      auVar27 = *(undefined1 (*) [16])(lVar6 + 0x20);
      auVar31._0_8_ = auVar27._0_8_;
      auVar31._8_4_ = auVar27._0_4_;
      auVar31._12_4_ = auVar27._4_4_;
      auVar32._0_8_ = auVar27._8_8_;
      auVar32._8_4_ = auVar27._8_4_;
      auVar32._12_4_ = auVar27._12_4_;
      core::hash::sip::impl$5::write<core::hash::sip::Sip13Rounds>();
      local_e8._0_8_ = SUB168(auVar32 ^ auVar12,0);
      local_e8._8_8_ = SUB168(auVar32 ^ auVar12,8);
      local_f8 = SUB168(auVar31 ^ auVar11,0);
      uVar16 = (longlong)local_f8 + local_e8._0_8_;
      pauStack_f0 = SUB168(auVar31 ^ auVar11,8);
      uVar14 = (local_e8._0_8_ << 0xd | (ulonglong)local_e8._0_8_ >> 0x33) ^ uVar16;
      uVar15 = (local_e8._8_8_ << 0x10 | (ulonglong)local_e8._8_8_ >> 0x30) ^
               (longlong)pauStack_f0 + local_e8._8_8_;
      uVar22 = (longlong)pauStack_f0 + local_e8._8_8_ + uVar14;
      uVar16 = (uVar16 << 0x20 | uVar16 >> 0x20) + uVar15;
      uVar14 = (uVar14 << 0x11 | uVar14 >> 0x2f) ^ uVar22;
      uVar15 = (uVar15 << 0x15 | uVar15 >> 0x2b) ^ uVar16;
      uVar16 = uVar16 + uVar14;
      uVar22 = ((uVar22 << 0x20 | uVar22 >> 0x20) ^ 0xff) + uVar15;
      uVar14 = (uVar14 << 0xd | uVar14 >> 0x33) ^ uVar16;
      uVar15 = (uVar15 << 0x10 | uVar15 >> 0x30) ^ uVar22;
      uVar22 = uVar22 + uVar14;
      uVar16 = (uVar16 << 0x20 | uVar16 >> 0x20) + uVar15;
      uVar14 = (uVar14 << 0x11 | uVar14 >> 0x2f) ^ uVar22;
      uVar15 = (uVar15 << 0x15 | uVar15 >> 0x2b) ^ uVar16;
      uVar16 = uVar16 + uVar14;
      uVar22 = (uVar22 << 0x20 | uVar22 >> 0x20) + uVar15;
      uVar14 = (uVar14 << 0xd | uVar14 >> 0x33) ^ uVar16;
      uVar15 = (uVar15 << 0x10 | uVar15 >> 0x30) ^ uVar22;
      uVar22 = uVar22 + uVar14;
      uVar16 = (uVar16 << 0x20 | uVar16 >> 0x20) + uVar15;
      uVar14 = (uVar14 << 0x11 | uVar14 >> 0x2f) ^ uVar22;
      uVar15 = (uVar15 << 0x15 | uVar15 >> 0x2b) ^ uVar16;
      uVar22 = (uVar22 << 0x20 | uVar22 >> 0x20) + uVar15;
      uVar14 = (uVar14 << 0xd | uVar14 >> 0x33) ^ uVar16 + uVar14;
      uVar15 = (uVar15 << 0x10 | uVar15 >> 0x30) ^ uVar22;
      uVar22 = uVar22 + uVar14;
      plVar17 = (longlong *)
                ((uVar22 << 0x20 | uVar22 >> 0x20) ^ (uVar14 << 0x11 | uVar14 >> 0x2f) ^
                 (uVar15 << 0x15 | uVar15 >> 0x2b) ^ uVar22);
      hashbrown::raw::RawTable::
      remove_entry<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::sync::Arc<system_decorator::blueprint::DecoratorBlueprint,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,alloc::sync::Arc<system_decorator::blueprint::DecoratorBlueprint,alloc::alloc::Global>_>_>
                ();
      if (plVar17 != (longlong *)0x0) {
        LOCK();
        *plVar17 = *plVar17 + -1;
        UNLOCK();
        if (*plVar17 == 0) {
          alloc::sync::Arc::
          drop_slow<system_decorator::blueprint::DecoratorBlueprint,alloc::alloc::Global>();
        }
      }
      uVar24 = uVar25 - 1 & uVar25;
    }
  }
  if (bVar10) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_03 == 1) {
      plVar20 = extraout_RAX_03 + 1;
    }
    else if ((*extraout_RAX_03 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar20 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0)) {
LAB_140826c0f:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar20 != 0) {
LAB_140826c1b:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar20 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar20 = *plVar20 + 1;
  }
  return;
}


