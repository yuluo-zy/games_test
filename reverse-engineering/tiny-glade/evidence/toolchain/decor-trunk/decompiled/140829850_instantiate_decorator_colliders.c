// ADDR 0x140829850
// SIG void __cdecl instantiate_decorator_colliders(void)
// ORIG system::instantiate_decorator_colliders


void __cdecl system_decorator::blueprint::instantiate_decorator_colliders(void)

{
  byte *pbVar1;
  char *pcVar2;
  byte bVar3;
  byte bVar4;
  int iVar5;
  longlong lVar6;
  longlong *plVar7;
  longlong lVar8;
  code *pcVar9;
  int iVar10;
  uint uVar11;
  uint uVar12;
  char extraout_AL;
  uint uVar13;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  byte *extraout_RAX_02;
  longlong extraout_RAX_03;
  byte *extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong lVar14;
  ulonglong extraout_RAX_06;
  longlong lVar15;
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  longlong *extraout_RAX_09;
  longlong *extraout_RAX_10;
  longlong *extraout_RAX_11;
  longlong *extraout_RAX_12;
  longlong *extraout_RAX_13;
  longlong *extraout_RAX_14;
  longlong *extraout_RAX_15;
  longlong *extraout_RAX_16;
  undefined8 *in_RCX;
  longlong *plVar16;
  undefined8 *in_RDX;
  ulonglong uVar17;
  undefined1 (*pauVar18) [16];
  undefined1 (*pauVar19) [16];
  longlong lVar20;
  undefined1 (*pauVar21) [16];
  longlong lVar22;
  longlong lVar23;
  ushort uVar24;
  longlong *in_R8;
  undefined *puVar25;
  ulonglong uVar26;
  uint uVar27;
  longlong lVar28;
  undefined8 *puVar29;
  longlong lVar30;
  byte bVar31;
  char cVar32;
  char cVar34;
  char cVar35;
  char cVar36;
  undefined1 auVar33 [16];
  undefined1 auVar37 [16];
  undefined1 auVar38 [16];
  undefined8 ******local_1c8;
  undefined8 local_1c0;
  longlong local_1b0;
  undefined8 *local_1a0;
  undefined1 (*local_198) [16];
  undefined1 (*pauStack_190) [16];
  undefined1 *local_188;
  longlong lStack_180;
  longlong local_178;
  longlong lStack_170;
  longlong *local_168;
  undefined4 local_160;
  int local_15c;
  undefined8 ******local_158;
  code *local_150;
  longlong *local_138;
  longlong local_130;
  longlong local_128;
  undefined8 *local_120;
  undefined8 local_118;
  longlong *plStack_110;
  undefined8 ******local_108;
  longlong lStack_100;
  longlong local_f8;
  undefined8 *puStack_f0;
  undefined8 local_e8;
  longlong *local_e0;
  undefined8 local_d8;
  undefined4 local_d0;
  undefined4 local_cc;
  undefined **local_c8;
  undefined8 local_c0;
  undefined8 ******local_b8;
  undefined8 local_b0;
  undefined8 local_a8;
  longlong *local_98;
  undefined1 local_89;
  int local_88;
  undefined1 local_83;
  undefined1 local_81;
  undefined8 local_80;
  
  local_80 = 0xfffffffffffffffe;
  puVar29 = in_RDX;
  local_120 = in_RCX;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    local_1b0 = 0;
  }
  else {
    puVar29 = (undefined8 *)&DAT_142a9148d;
    uVar17 = 0x3c;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar17 != 0) {
        if (uVar17 < 0x3c) {
          if ((char)(&DAT_142a9148d)[uVar17] < -0x40) {
LAB_1408298ea:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar17 != 0x3c) goto LAB_1408298ea;
      }
      puVar29 = (undefined8 *)&DAT_142a9148d;
      core::str::impl$0::rfind<ref_<str_>_>();
      if (extraout_RAX_00 == 1) {
        if (uVar17 != 0xfffffffffffffffe) {
          if (uVar17 + 2 < 0x3c) {
            if ((char)(&UNK_142a9148f)[uVar17] < -0x40) {
LAB_140829948:
                    // WARNING: Subroutine does not return
              core::str::slice_error_fail();
            }
          }
          else if (uVar17 + 2 != 0x3c) goto LAB_140829948;
        }
        puVar29 = (undefined8 *)(&UNK_142a9148f + uVar17);
      }
    }
    puVar25 = &DAT_142a91679;
    do {
      bVar31 = puVar25[-1];
      uVar13 = (uint)(char)bVar31;
      if ((char)bVar31 < '\0') {
        bVar3 = puVar25[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar25[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar25 + -4;
            puVar25 = puVar25 + -4;
            uVar13 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar25 = puVar25 + -3;
            uVar13 = bVar4 & 0xf;
          }
          uVar13 = bVar3 & 0x3f | uVar13 << 6;
        }
        else {
          puVar25 = puVar25 + -2;
          uVar13 = bVar3 & 0x1f;
        }
        uVar13 = (uint)(bVar31 & 0x3f) | uVar13 << 6;
        if (uVar13 != 0x5c) goto LAB_140829a04;
LAB_140829a15:
        if (puVar25 == &UNK_142a9164f) break;
        if (puVar25 + -0x142a9164f < (undefined *)0x29) {
          if (-0x41 < (char)puVar25[1]) break;
        }
        else if (puVar25 + -0x142a9164f == (undefined *)0x29) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar25 = puVar25 + -1;
      if (uVar13 == 0x5c) goto LAB_140829a15;
LAB_140829a04:
      if (uVar13 == 0x2f) goto LAB_140829a15;
    } while (puVar25 != &anon_607a6278b2141009247d0776e3b766a7_27_llvm_14222842113206979732);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar16 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar16 = extraout_RAX_16, extraout_RAX_16 == (longlong *)0x0)) {
LAB_14082abd7:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar16 != 0) {
LAB_14082abe3:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar16 = -1;
    local_98 = plVar16;
    puffin::ThreadProfiler::begin_scope();
    *local_98 = *local_98 + 1;
    local_1b0 = 1;
  }
  pauVar19 = (undefined1 (*) [16])*local_120;
  lVar20 = local_120[1];
  auVar33 = *pauVar19;
  lVar28 = local_120[3];
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_02 & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX_02[0] = 1;
    extraout_RAX_02[1] = 0;
    extraout_RAX_02[2] = 0;
    extraout_RAX_02[3] = 0;
    extraout_RAX_02[4] = 0;
    extraout_RAX_02[5] = 0;
    extraout_RAX_02[6] = 0;
    extraout_RAX_02[7] = 0;
    *(longlong *)(extraout_RAX_02 + 8) = extraout_RAX_03;
    *(undefined8 **)(extraout_RAX_02 + 0x10) = puVar29;
    local_f8 = extraout_RAX_03;
  }
  else {
    local_f8 = *(longlong *)(extraout_RAX_02 + 8);
    puVar29 = *(undefined8 **)(extraout_RAX_02 + 0x10);
  }
  *(longlong *)(extraout_RAX_02 + 8) = local_f8 + 1;
  local_108 = (undefined8 ******)0x0;
  lStack_100 = 0;
  local_118 = (undefined **)&DAT_142a912a0;
  plStack_110 = (longlong *)0x0;
  puStack_f0 = puVar29;
  if (lVar28 != 0) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::decorator::DecoratorId,tuple_<>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::decorator::DecoratorId,tuple_<>,std::hash::random::RandomState>_>
              ();
  }
  local_188 = *pauVar19 + lVar20 + 1;
  local_158 = (undefined8 ******)&local_118;
  pauStack_190 = pauVar19 + 1;
  lStack_180 = CONCAT62(lStack_180._2_6_,
                        ~((ushort)(SUB161(auVar33 >> 7,0) & 1) |
                          (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1 |
                          (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
                          (ushort)(SUB161(auVar33 >> 0x1f,0) & 1) << 3 |
                          (ushort)(SUB161(auVar33 >> 0x27,0) & 1) << 4 |
                          (ushort)(SUB161(auVar33 >> 0x2f,0) & 1) << 5 |
                          (ushort)(SUB161(auVar33 >> 0x37,0) & 1) << 6 |
                          (ushort)(SUB161(auVar33 >> 0x3f,0) & 1) << 7 |
                          (ushort)(SUB161(auVar33 >> 0x47,0) & 1) << 8 |
                          (ushort)(SUB161(auVar33 >> 0x4f,0) & 1) << 9 |
                          (ushort)(SUB161(auVar33 >> 0x57,0) & 1) << 10 |
                          (ushort)(SUB161(auVar33 >> 0x5f,0) & 1) << 0xb |
                          (ushort)(SUB161(auVar33 >> 0x67,0) & 1) << 0xc |
                          (ushort)(SUB161(auVar33 >> 0x6f,0) & 1) << 0xd |
                          (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe |
                         (ushort)(byte)(auVar33[0xf] >> 7) << 0xf));
  local_198 = pauVar19;
  _ZN9hashbrown3raw21RawIterRange_LT_T_GT_9fold_impl17h0f15cd22ac3f5e03E();
  lVar23 = lStack_100;
  plVar16 = plStack_110;
  pauVar21 = (undefined1 (*) [16])local_118;
  pauVar19 = (undefined1 (*) [16])local_120[6];
  lVar20 = local_120[7];
  auVar33 = *pauVar19;
  lVar14 = local_120[9];
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_04 & 1) == 0) {
    local_83 = 1;
    std::sys::random::hashmap_random_keys();
    extraout_RAX_04[0] = 1;
    extraout_RAX_04[1] = 0;
    extraout_RAX_04[2] = 0;
    extraout_RAX_04[3] = 0;
    extraout_RAX_04[4] = 0;
    extraout_RAX_04[5] = 0;
    extraout_RAX_04[6] = 0;
    extraout_RAX_04[7] = 0;
    *(longlong *)(extraout_RAX_04 + 8) = extraout_RAX_05;
    *(longlong *)(extraout_RAX_04 + 0x10) = lVar28;
    local_f8 = extraout_RAX_05;
  }
  else {
    local_f8 = *(longlong *)(extraout_RAX_04 + 8);
    lVar28 = *(longlong *)(extraout_RAX_04 + 0x10);
  }
  *(longlong *)(extraout_RAX_04 + 8) = local_f8 + 1;
  local_108 = (undefined8 ******)0x0;
  lStack_100 = 0;
  local_118 = (undefined **)&DAT_142a912a0;
  plStack_110 = (longlong *)0x0;
  puStack_f0 = (undefined8 *)lVar28;
  if (lVar14 != 0) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::decorator::DecoratorId,tuple_<>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::decorator::DecoratorId,tuple_<>,std::hash::random::RandomState>_>
              ();
  }
  local_188 = *pauVar19 + lVar20 + 1;
  local_1c8 = (undefined8 ******)&local_118;
  local_158 = &local_1c8;
  pauStack_190 = pauVar19 + 1;
  lStack_180 = CONCAT62(lStack_180._2_6_,
                        ~((ushort)(SUB161(auVar33 >> 7,0) & 1) |
                          (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1 |
                          (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
                          (ushort)(SUB161(auVar33 >> 0x1f,0) & 1) << 3 |
                          (ushort)(SUB161(auVar33 >> 0x27,0) & 1) << 4 |
                          (ushort)(SUB161(auVar33 >> 0x2f,0) & 1) << 5 |
                          (ushort)(SUB161(auVar33 >> 0x37,0) & 1) << 6 |
                          (ushort)(SUB161(auVar33 >> 0x3f,0) & 1) << 7 |
                          (ushort)(SUB161(auVar33 >> 0x47,0) & 1) << 8 |
                          (ushort)(SUB161(auVar33 >> 0x4f,0) & 1) << 9 |
                          (ushort)(SUB161(auVar33 >> 0x57,0) & 1) << 10 |
                          (ushort)(SUB161(auVar33 >> 0x5f,0) & 1) << 0xb |
                          (ushort)(SUB161(auVar33 >> 0x67,0) & 1) << 0xc |
                          (ushort)(SUB161(auVar33 >> 0x6f,0) & 1) << 0xd |
                          (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe |
                         (ushort)(byte)(auVar33[0xf] >> 7) << 0xf));
  local_198 = pauVar19;
  _ZN9hashbrown3raw21RawIterRange_LT_T_GT_9fold_impl17h0f15cd22ac3f5e03E();
  local_1a0 = local_120 + 6;
  local_198 = (undefined1 (*) [16])local_118;
  pauStack_190 = (undefined1 (*) [16])plStack_110;
  local_188 = (undefined1 *)local_108;
  lStack_180 = lStack_100;
  local_178 = local_f8;
  lStack_170 = (longlong)puStack_f0;
  pauVar18 = (undefined1 (*) [16])*in_RDX;
  lVar28 = in_RDX[3];
  auVar33 = *pauVar18;
  uVar13 = ~(uint)(ushort)((ushort)(SUB161(auVar33 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar33 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar33 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar33 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar33 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar33 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar33 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar33 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar33 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar33 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar33 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar33 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe |
                          (ushort)(byte)(auVar33[0xf] >> 7) << 0xf);
  pauVar19 = pauVar18 + 1;
  local_128 = in_R8[3];
  local_168 = in_R8 + 4;
  local_130 = *in_R8;
  local_98 = (longlong *)in_R8[1];
  lVar20 = local_130 + -0x10;
  lVar14 = local_130 + -8;
LAB_140829d39:
  do {
    do {
      local_89 = lVar28 != 0;
      if (lVar28 == 0) {
        local_98 = plVar16;
        local_89 = 0;
        if (plVar16 == (longlong *)0x0) {
          local_130 = 0;
        }
        else {
          uVar17 = (longlong)plVar16 * 4 + 0x13U & 0xfffffffffffffff0;
          local_128 = (longlong)plVar16 + uVar17 + 0x11;
          local_168 = (longlong *)((longlong)pauVar21 - uVar17);
          local_130 = 0x10;
        }
        if (lVar23 != 0) {
          auVar33 = *pauVar21;
          uVar13 = ~(uint)(ushort)((ushort)(SUB161(auVar33 >> 7,0) & 1) |
                                   (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1 |
                                   (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
                                   (ushort)(SUB161(auVar33 >> 0x1f,0) & 1) << 3 |
                                   (ushort)(SUB161(auVar33 >> 0x27,0) & 1) << 4 |
                                   (ushort)(SUB161(auVar33 >> 0x2f,0) & 1) << 5 |
                                   (ushort)(SUB161(auVar33 >> 0x37,0) & 1) << 6 |
                                   (ushort)(SUB161(auVar33 >> 0x3f,0) & 1) << 7 |
                                   (ushort)(SUB161(auVar33 >> 0x47,0) & 1) << 8 |
                                   (ushort)(SUB161(auVar33 >> 0x4f,0) & 1) << 9 |
                                   (ushort)(SUB161(auVar33 >> 0x57,0) & 1) << 10 |
                                   (ushort)(SUB161(auVar33 >> 0x5f,0) & 1) << 0xb |
                                   (ushort)(SUB161(auVar33 >> 0x67,0) & 1) << 0xc |
                                   (ushort)(SUB161(auVar33 >> 0x6f,0) & 1) << 0xd |
                                   (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe |
                                  (ushort)(byte)(auVar33[0xf] >> 7) << 0xf);
          pauVar19 = pauVar21 + 1;
          do {
            if ((short)uVar13 == 0) {
              do {
                auVar33 = *pauVar19;
                uVar24 = (ushort)(SUB161(auVar33 >> 7,0) & 1) |
                         (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1 |
                         (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
                         (ushort)(SUB161(auVar33 >> 0x1f,0) & 1) << 3 |
                         (ushort)(SUB161(auVar33 >> 0x27,0) & 1) << 4 |
                         (ushort)(SUB161(auVar33 >> 0x2f,0) & 1) << 5 |
                         (ushort)(SUB161(auVar33 >> 0x37,0) & 1) << 6 |
                         (ushort)(SUB161(auVar33 >> 0x3f,0) & 1) << 7 |
                         (ushort)(SUB161(auVar33 >> 0x47,0) & 1) << 8 |
                         (ushort)(SUB161(auVar33 >> 0x4f,0) & 1) << 9 |
                         (ushort)(SUB161(auVar33 >> 0x57,0) & 1) << 10 |
                         (ushort)(SUB161(auVar33 >> 0x5f,0) & 1) << 0xb |
                         (ushort)(SUB161(auVar33 >> 0x67,0) & 1) << 0xc |
                         (ushort)(SUB161(auVar33 >> 0x6f,0) & 1) << 0xd |
                         (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe |
                         (ushort)(byte)(auVar33[0xf] >> 7) << 0xf;
                pauVar21 = pauVar21 + -4;
                pauVar19 = pauVar19 + 1;
              } while (uVar24 == 0xffff);
              uVar27 = ~(uint)uVar24;
              uVar13 = uVar27 & -(uint)uVar24 - 2;
            }
            else {
              uVar27 = uVar13;
              uVar13 = uVar13 - 1 & uVar13;
            }
            iVar10 = 0;
            for (; (uVar27 & 1) == 0; uVar27 = uVar27 >> 1 | 0x80000000) {
              iVar10 = iVar10 + 1;
            }
            local_160 = *(undefined4 *)((longlong)pauVar21 + (-4 - (ulonglong)(uint)(iVar10 << 2)));
            if ((*
                 __imp__ZN12country_core7systems9collision5world24PHYSICS_DEBUG_LOG_ENABLE17h8c0b6c8a40689d94E
                 != '\0') &&
               (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E)) {
              local_158 = (undefined8 ******)&local_160;
              local_150 = country_core::resources::walls::decorator::impl$27::fmt;
              toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
              local_f8 = *extraout_RAX_13;
              puStack_f0 = (undefined8 *)extraout_RAX_13[1];
              local_cc = (undefined4)extraout_RAX_13[2];
              local_e8 = 3;
              local_e0 = (longlong *)s_system_decorator__blueprint_142a91890;
              local_d8 = 0x1b;
              local_c8 = &PTR_s_Decorator_collider_REMOVED_142a91990;
              local_c0 = 2;
              local_b8 = &local_158;
              local_b0 = 1;
              local_a8 = 0;
              local_118 = (undefined **)0x0;
              plStack_110 = (longlong *)s_system_decorator__blueprint_142a91890;
              local_108 = (undefined8 ******)0x1b;
              lStack_100 = 0;
              local_d0 = 1;
              ::log::__private_api::impl$2::log();
            }
            country_core::systems::collision::world::RaycastWorld::
            remove_collider<country_core::resources::walls::decorator::DecoratorId>();
            if ((char)local_f8 != '\x11') {
              LOCK();
              *local_e0 = *local_e0 + -1;
              UNLOCK();
              if (*local_e0 == 0) {
                alloc::sync::Arc::
                drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
                          ();
              }
            }
            lVar23 = lVar23 + -1;
          } while (lVar23 != 0);
        }
        if ((local_98 != (longlong *)0x0) && (local_128 != 0)) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        local_98 = (longlong *)pauStack_190;
        if (pauStack_190 == (undefined1 (*) [16])0x0) {
          local_120 = (undefined8 *)0x0;
        }
        else {
          uVar17 = (longlong)pauStack_190 * 4 + 0x13U & 0xfffffffffffffff0;
          local_128 = (longlong)pauStack_190 + uVar17 + 0x11;
          local_130 = (longlong)local_198 - uVar17;
          local_120 = (undefined8 *)&DAT_00000010;
        }
        if (lStack_180 != 0) {
          auVar33 = *local_198;
          pauVar19 = local_198 + 1;
          lVar20 = lStack_180;
          uVar13 = ~(uint)(ushort)((ushort)(SUB161(auVar33 >> 7,0) & 1) |
                                   (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1 |
                                   (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
                                   (ushort)(SUB161(auVar33 >> 0x1f,0) & 1) << 3 |
                                   (ushort)(SUB161(auVar33 >> 0x27,0) & 1) << 4 |
                                   (ushort)(SUB161(auVar33 >> 0x2f,0) & 1) << 5 |
                                   (ushort)(SUB161(auVar33 >> 0x37,0) & 1) << 6 |
                                   (ushort)(SUB161(auVar33 >> 0x3f,0) & 1) << 7 |
                                   (ushort)(SUB161(auVar33 >> 0x47,0) & 1) << 8 |
                                   (ushort)(SUB161(auVar33 >> 0x4f,0) & 1) << 9 |
                                   (ushort)(SUB161(auVar33 >> 0x57,0) & 1) << 10 |
                                   (ushort)(SUB161(auVar33 >> 0x5f,0) & 1) << 0xb |
                                   (ushort)(SUB161(auVar33 >> 0x67,0) & 1) << 0xc |
                                   (ushort)(SUB161(auVar33 >> 0x6f,0) & 1) << 0xd |
                                   (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe |
                                  (ushort)(byte)(auVar33[0xf] >> 7) << 0xf);
          do {
            if ((short)uVar13 == 0) {
              do {
                auVar33 = *pauVar19;
                uVar24 = (ushort)(SUB161(auVar33 >> 7,0) & 1) |
                         (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1 |
                         (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
                         (ushort)(SUB161(auVar33 >> 0x1f,0) & 1) << 3 |
                         (ushort)(SUB161(auVar33 >> 0x27,0) & 1) << 4 |
                         (ushort)(SUB161(auVar33 >> 0x2f,0) & 1) << 5 |
                         (ushort)(SUB161(auVar33 >> 0x37,0) & 1) << 6 |
                         (ushort)(SUB161(auVar33 >> 0x3f,0) & 1) << 7 |
                         (ushort)(SUB161(auVar33 >> 0x47,0) & 1) << 8 |
                         (ushort)(SUB161(auVar33 >> 0x4f,0) & 1) << 9 |
                         (ushort)(SUB161(auVar33 >> 0x57,0) & 1) << 10 |
                         (ushort)(SUB161(auVar33 >> 0x5f,0) & 1) << 0xb |
                         (ushort)(SUB161(auVar33 >> 0x67,0) & 1) << 0xc |
                         (ushort)(SUB161(auVar33 >> 0x6f,0) & 1) << 0xd |
                         (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe |
                         (ushort)(byte)(auVar33[0xf] >> 7) << 0xf;
                pauVar19 = pauVar19 + 1;
              } while (uVar24 == 0xffff);
              uVar13 = ~(uint)uVar24;
              uVar27 = uVar13 & -(uint)uVar24 - 2;
            }
            else {
              uVar27 = uVar13 - 1 & uVar13;
            }
            for (; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
            }
            country_core::systems::collision::world::RaycastWorld::
            remove_collider<country_core::resources::walls::decorator::DecoratorId>();
            if ((char)local_f8 != '\x11') {
              LOCK();
              *local_e0 = *local_e0 + -1;
              UNLOCK();
              if (*local_e0 == 0) {
                alloc::sync::Arc::
                drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
                          ();
              }
            }
            lVar20 = lVar20 + -1;
            uVar13 = uVar27;
          } while (lVar20 != 0);
        }
        if ((local_98 != (longlong *)0x0) && (local_128 != 0)) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        if (local_1b0 == 0) {
          return;
        }
        puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
        if (*extraout_RAX_14 == 1) {
          local_138 = extraout_RAX_14 + 1;
        }
        else if ((*extraout_RAX_14 != 0) ||
                (std::sys::thread_local::native::lazy::Storage::
                 initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                           (), local_138 = extraout_RAX_15, extraout_RAX_15 == (longlong *)0x0))
        goto LAB_14082abd7;
        if (*local_138 == 0) {
          *local_138 = -1;
          puffin::ThreadProfiler::end_scope();
          *local_138 = *local_138 + 1;
          return;
        }
        goto LAB_14082abe3;
      }
      if ((short)uVar13 == 0) {
        do {
          auVar33 = *pauVar19;
          uVar24 = (ushort)(SUB161(auVar33 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar33 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar33 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar33 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar33 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar33 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar33 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar33 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar33 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar33 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar33 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar33 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar33[0xf] >> 7) << 0xf;
          pauVar18 = pauVar18 + -0x10;
          pauVar19 = pauVar19 + 1;
        } while (uVar24 == 0xffff);
        uVar27 = ~(uint)uVar24;
        uVar13 = uVar27 & -(uint)uVar24 - 2;
      }
      else {
        uVar27 = uVar13;
        uVar13 = uVar13 - 1 & uVar13;
      }
      iVar10 = 0;
      for (; (uVar27 & 1) == 0; uVar27 = uVar27 >> 1 | 0x80000000) {
        iVar10 = iVar10 + 1;
      }
      lVar28 = lVar28 + -1;
      iVar5 = *(int *)((longlong)pauVar18 + (-0x10 - (ulonglong)(uint)(iVar10 << 4)));
      lVar6 = *(longlong *)((longlong)pauVar18 + (-8 - (ulonglong)(uint)(iVar10 << 4)));
      local_118 = (undefined **)CONCAT44(local_118._4_4_,iVar5);
      if (local_128 != 0) {
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
        bVar31 = (byte)(extraout_RAX_06 >> 0x39);
        auVar33 = ZEXT216(CONCAT11(bVar31,bVar31));
        auVar33 = pshuflw(auVar33,auVar33,0);
        lVar15 = 0;
        uVar17 = extraout_RAX_06;
        while( true ) {
          uVar17 = (ulonglong)local_98 & uVar17;
          pcVar2 = (char *)(local_130 + uVar17);
          cVar32 = auVar33[0];
          auVar38[0] = -(*pcVar2 == cVar32);
          cVar34 = auVar33[1];
          auVar38[1] = -(pcVar2[1] == cVar34);
          cVar35 = auVar33[2];
          auVar38[2] = -(pcVar2[2] == cVar35);
          cVar36 = auVar33[3];
          auVar38[3] = -(pcVar2[3] == cVar36);
          auVar38[4] = -(pcVar2[4] == cVar32);
          auVar38[5] = -(pcVar2[5] == cVar34);
          auVar38[6] = -(pcVar2[6] == cVar35);
          auVar38[7] = -(pcVar2[7] == cVar36);
          auVar38[8] = -(pcVar2[8] == cVar32);
          auVar38[9] = -(pcVar2[9] == cVar34);
          auVar38[10] = -(pcVar2[10] == cVar35);
          auVar38[0xb] = -(pcVar2[0xb] == cVar36);
          auVar38[0xc] = -(pcVar2[0xc] == cVar32);
          auVar38[0xd] = -(pcVar2[0xd] == cVar34);
          auVar38[0xe] = -(pcVar2[0xe] == cVar35);
          auVar38[0xf] = -(pcVar2[0xf] == cVar36);
          uVar24 = (ushort)(SUB161(auVar38 >> 7,0) & 1) |
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
                   (ushort)(auVar38[0xf] >> 7) << 0xf;
          uVar27 = (uint)uVar24;
          while (uVar24 != 0) {
            uVar11 = 0;
            for (uVar12 = uVar27; (uVar12 & 1) == 0; uVar12 = uVar12 >> 1 | 0x80000000) {
              uVar11 = uVar11 + 1;
            }
            uVar26 = uVar11 + uVar17 & (ulonglong)local_98;
            if (iVar5 == *(int *)(lVar20 + uVar26 * -0x10)) {
              if (*(longlong *)(lVar14 + uVar26 * -0x10) != lVar6) goto LAB_140829e75;
              local_88 = iVar5;
              core::hash::BuildHasher::
              hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>
                        ();
              local_81 = 1;
              hashbrown::raw::RawTable::
              remove_entry<tuple_<country_core::resources::walls::decorator::DecoratorId,tuple_<>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,tuple_<>_>_>
                        ();
              core::hash::BuildHasher::
              hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>
                        ();
              local_81 = 1;
              hashbrown::raw::RawTable::
              remove_entry<tuple_<country_core::resources::walls::decorator::DecoratorId,tuple_<>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,tuple_<>_>_>
                        ();
              goto LAB_140829d39;
            }
            uVar24 = (ushort)(uVar27 - 1) & (ushort)uVar27;
            uVar27 = CONCAT22((short)(uVar27 - 1 >> 0x10),uVar24);
          }
          auVar37[0] = -(*pcVar2 == -1);
          auVar37[1] = -(pcVar2[1] == -1);
          auVar37[2] = -(pcVar2[2] == -1);
          auVar37[3] = -(pcVar2[3] == -1);
          auVar37[4] = -(pcVar2[4] == -1);
          auVar37[5] = -(pcVar2[5] == -1);
          auVar37[6] = -(pcVar2[6] == -1);
          auVar37[7] = -(pcVar2[7] == -1);
          auVar37[8] = -(pcVar2[8] == -1);
          auVar37[9] = -(pcVar2[9] == -1);
          auVar37[10] = -(pcVar2[10] == -1);
          auVar37[0xb] = -(pcVar2[0xb] == -1);
          auVar37[0xc] = -(pcVar2[0xc] == -1);
          auVar37[0xd] = -(pcVar2[0xd] == -1);
          auVar37[0xe] = -(pcVar2[0xe] == -1);
          auVar37[0xf] = -(pcVar2[0xf] == -1);
          if ((((((((((((((((SUB161(auVar37 >> 7,0) & 1) != 0 || (SUB161(auVar37 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar37 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar37 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar37 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar37 >> 0x2f,0) & 1) != 0) || (SUB161(auVar37 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar37 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar37 >> 0x47,0) & 1) != 0) || (SUB161(auVar37 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar37 >> 0x57,0) & 1) != 0) || (SUB161(auVar37 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar37 >> 0x67,0) & 1) != 0) || (SUB161(auVar37 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar37 >> 0x77,0) & 1) != 0) || auVar37[0xf] < '\0') break;
          uVar17 = uVar17 + lVar15 + 0x10;
          lVar15 = lVar15 + 0x10;
        }
      }
LAB_140829e75:
      local_88 = iVar5;
    } while (*(char *)(lVar6 + 0x138) != '\0');
    if (*(int *)(lVar6 + 0x60) == 0) {
      plVar7 = *(longlong **)(lVar6 + 0x70);
      LOCK();
      lVar15 = *plVar7;
      *plVar7 = *plVar7 + 1;
      UNLOCK();
      if (*plVar7 == 0 || SCARRY8(lVar15,1) != *plVar7 < 0) goto LAB_14082acb9;
      local_118 = (undefined **)CONCAT44(iVar5,(undefined4)local_118);
      plStack_110 = *(longlong **)(lVar6 + 0x50);
      local_108 = *(undefined8 *******)(lVar6 + 0x58);
      local_118 = (undefined **)CONCAT71(local_118._1_7_,3);
      local_81 = 1;
      country_core::systems::collision::world::RaycastMesh::new();
      plStack_110 = (longlong *)CONCAT44(plStack_110._4_4_,*(undefined4 *)(lVar6 + 0x6c));
      local_118 = *(undefined ***)(lVar6 + 100);
      local_108 = *(undefined8 *******)(lVar6 + 0x80);
      lStack_100 = *(longlong *)(lVar6 + 0x88);
      local_81 = 1;
      country_core::systems::collision::world::RaycastWorld::
      upsert_collider<country_core::resources::walls::decorator::DecoratorId,tuple_<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat>_>
                ();
      local_15c = local_1c0._4_4_;
      if (*
          __imp__ZN12country_core7systems9collision5world24PHYSICS_DEBUG_LOG_ENABLE17h8c0b6c8a40689d94E
          == '\x01') {
        if (local_1c0._4_4_ == 0) {
          if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
            local_158 = (undefined8 ******)&local_88;
            local_150 = country_core::resources::walls::decorator::impl$27::fmt;
            local_81 = 1;
            toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
            local_f8 = *extraout_RAX_12;
            puStack_f0 = (undefined8 *)extraout_RAX_12[1];
            local_cc = (undefined4)extraout_RAX_12[2];
            local_e8 = 3;
            local_e0 = (longlong *)s_system_decorator__blueprint_142a91890;
            local_d8 = 0x1b;
            local_c8 = &PTR_s_Decorator_collider_REMOVED_142a919f8;
            local_c0 = 2;
            local_b8 = &local_158;
            local_b0 = 1;
            local_a8 = 0;
            local_118 = (undefined **)0x0;
            plStack_110 = (longlong *)s_system_decorator__blueprint_142a91890;
            local_108 = (undefined8 ******)0x1b;
            lStack_100 = 0;
            local_d0 = 1;
            local_81 = 1;
            ::log::__private_api::impl$2::log();
          }
          goto LAB_14082a21d;
        }
        if (local_1c0._4_4_ == 1) {
          if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
            local_158 = (undefined8 ******)&local_88;
            local_150 = country_core::resources::walls::decorator::impl$27::fmt;
            local_81 = 1;
            toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
            local_f8 = *extraout_RAX_07;
            puStack_f0 = (undefined8 *)extraout_RAX_07[1];
            local_cc = (undefined4)extraout_RAX_07[2];
            local_e8 = 3;
            local_e0 = (longlong *)s_system_decorator__blueprint_142a91890;
            local_d8 = 0x1b;
            local_c8 = &PTR_s_Decorator_collider_REMOVED_142a91a48;
            local_c0 = 2;
            local_b8 = &local_158;
            local_b0 = 1;
            local_a8 = 0;
            local_118 = (undefined **)0x0;
            plStack_110 = (longlong *)s_system_decorator__blueprint_142a91890;
            local_108 = (undefined8 ******)0x1b;
            lStack_100 = 0;
            local_d0 = 1;
            local_81 = 1;
            ::log::__private_api::impl$2::log();
          }
          lVar15 = *(longlong *)(lVar6 + 200);
          lVar8 = *(longlong *)(lVar6 + 0xd0);
          lVar30 = lVar15;
          while (lVar22 = lVar30, lVar22 != lVar8 * 0x160 + lVar15) {
            lVar30 = lVar22 + 0x160;
            if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
              local_158 = (undefined8 ******)(lVar22 + 0xa0);
              local_150 = anyhow::context::impl$3::
                          fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
              local_81 = 1;
              toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
              local_f8 = *extraout_RAX_08;
              puStack_f0 = (undefined8 *)extraout_RAX_08[1];
              local_cc = (undefined4)extraout_RAX_08[2];
              local_e8 = 3;
              local_e0 = (longlong *)s_system_decorator__blueprint_142a91890;
              local_d8 = 0x1b;
              local_c8 = &PTR_s_mesh__142a91a90;
              local_c0 = 1;
              local_b8 = &local_158;
              local_b0 = 1;
              local_a8 = 0;
              local_118 = (undefined **)0x0;
              plStack_110 = (longlong *)s_system_decorator__blueprint_142a91890;
              local_108 = (undefined8 ******)0x1b;
              lStack_100 = 0;
              local_d0 = 1;
              local_81 = 1;
              ::log::__private_api::impl$2::log();
            }
          }
          goto LAB_14082a1ee;
        }
      }
      else {
LAB_14082a1ee:
        if (local_15c == 0) goto LAB_14082a21d;
      }
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
      local_81 = 1;
      hashbrown::raw::RawTable::
      remove_entry<tuple_<country_core::resources::walls::decorator::DecoratorId,tuple_<>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,tuple_<>_>_>
                ();
    }
LAB_14082a21d:
    if (*(int *)(lVar6 + 0x90) == 0) {
      plVar7 = *(longlong **)(lVar6 + 0xa0);
      LOCK();
      lVar15 = *plVar7;
      *plVar7 = *plVar7 + 1;
      UNLOCK();
      if (*plVar7 == 0 || SCARRY8(lVar15,1) != *plVar7 < 0) {
LAB_14082acb9:
                    // WARNING: Does not return
        pcVar9 = (code *)invalidInstructionException();
        (*pcVar9)();
      }
      local_118 = (undefined **)CONCAT44(local_88,(undefined4)local_118);
      plStack_110 = *(longlong **)(lVar6 + 0x50);
      local_108 = *(undefined8 *******)(lVar6 + 0x58);
      local_118 = (undefined **)(CONCAT71(local_118._1_7_,*(undefined1 *)(lVar6 + 0x139)) | 4);
      local_81 = 1;
      country_core::systems::collision::world::RaycastMesh::new();
      plStack_110 = (longlong *)CONCAT44(plStack_110._4_4_,*(undefined4 *)(lVar6 + 0x9c));
      local_118 = *(undefined ***)(lVar6 + 0x94);
      local_108 = *(undefined8 *******)(lVar6 + 0xb0);
      lStack_100 = *(longlong *)(lVar6 + 0xb8);
      local_81 = 1;
      country_core::systems::collision::world::RaycastWorld::
      upsert_collider<country_core::resources::walls::decorator::DecoratorId,tuple_<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat>_>
                ();
      iVar10 = local_1c0._4_4_;
      if (*
          __imp__ZN12country_core7systems9collision5world24PHYSICS_DEBUG_LOG_ENABLE17h8c0b6c8a40689d94E
          == '\x01') {
        if (local_1c0._4_4_ == 0) {
          if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
            local_158 = (undefined8 ******)&local_88;
            local_150 = country_core::resources::walls::decorator::impl$27::fmt;
            local_81 = 1;
            toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
            local_f8 = *extraout_RAX_11;
            puStack_f0 = (undefined8 *)extraout_RAX_11[1];
            local_cc = (undefined4)extraout_RAX_11[2];
            local_e8 = 3;
            local_e0 = (longlong *)s_system_decorator__blueprint_142a91890;
            local_d8 = 0x1b;
            local_c8 = &PTR_s_Decorator_collider_REMOVED_142a91af0;
            local_c0 = 2;
            local_b8 = &local_158;
            local_b0 = 1;
            local_a8 = 0;
            local_118 = (undefined **)0x0;
            plStack_110 = (longlong *)s_system_decorator__blueprint_142a91890;
            local_108 = (undefined8 ******)0x1b;
            lStack_100 = 0;
            local_d0 = 1;
            local_81 = 1;
            ::log::__private_api::impl$2::log();
          }
          goto LAB_140829d39;
        }
        if (local_1c0._4_4_ == 1) {
          if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
            local_158 = (undefined8 ******)&local_88;
            local_150 = country_core::resources::walls::decorator::impl$27::fmt;
            local_81 = 1;
            toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
            local_f8 = *extraout_RAX_09;
            puStack_f0 = (undefined8 *)extraout_RAX_09[1];
            local_cc = (undefined4)extraout_RAX_09[2];
            local_e8 = 3;
            local_e0 = (longlong *)s_system_decorator__blueprint_142a91890;
            local_d8 = 0x1b;
            local_c8 = &PTR_s_Decorator_collider_REMOVED_142a91b48;
            local_c0 = 2;
            local_b8 = &local_158;
            local_b0 = 1;
            local_a8 = 0;
            local_118 = (undefined **)0x0;
            plStack_110 = (longlong *)s_system_decorator__blueprint_142a91890;
            local_108 = (undefined8 ******)0x1b;
            lStack_100 = 0;
            local_d0 = 1;
            local_81 = 1;
            ::log::__private_api::impl$2::log();
          }
          lVar15 = *(longlong *)(lVar6 + 200);
          lVar6 = *(longlong *)(lVar6 + 0xd0);
          lVar8 = lVar15;
          while (lVar30 = lVar8, lVar30 != lVar6 * 0x160 + lVar15) {
            lVar8 = lVar30 + 0x160;
            if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
              local_158 = (undefined8 ******)(lVar30 + 0xa0);
              local_150 = anyhow::context::impl$3::
                          fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
              local_81 = 1;
              toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
              local_f8 = *extraout_RAX_10;
              puStack_f0 = (undefined8 *)extraout_RAX_10[1];
              local_cc = (undefined4)extraout_RAX_10[2];
              local_e8 = 3;
              local_e0 = (longlong *)s_system_decorator__blueprint_142a91890;
              local_d8 = 0x1b;
              local_c8 = &PTR_s_mesh__142a91a90;
              local_c0 = 1;
              local_b8 = &local_158;
              local_b0 = 1;
              local_a8 = 0;
              local_118 = (undefined **)0x0;
              plStack_110 = (longlong *)s_system_decorator__blueprint_142a91890;
              local_108 = (undefined8 ******)0x1b;
              lStack_100 = 0;
              local_d0 = 1;
              local_81 = 1;
              ::log::__private_api::impl$2::log();
            }
          }
          goto LAB_14082a538;
        }
      }
      else {
LAB_14082a538:
        if (iVar10 == 0) goto LAB_140829d39;
      }
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
      local_81 = 1;
      hashbrown::raw::RawTable::
      remove_entry<tuple_<country_core::resources::walls::decorator::DecoratorId,tuple_<>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,tuple_<>_>_>
                ();
      goto LAB_140829d39;
    }
    if (*(int *)(lVar6 + 0x90) != 1) {
      local_81 = 1;
      alloc::vec::spec_from_iter::impl$0::
      from_iter<ref_<str_>,core::iter::adapters::map::Map<core::slice::iter::Iter<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder>,system_decorator::blueprint::instantiate_decorator_colliders::closure_env_0>_>
                ();
      local_1c0 = alloc::vec::impl$27::fmt<ref_<str_>,alloc::alloc::Global>;
      local_118 = &PTR_s_Decorator_blueprint_did_not_spec_142a91bd8;
      plStack_110 = (longlong *)0x1;
      local_f8 = 0;
      local_108 = &local_1c8;
      lStack_100 = 1;
      local_1c8 = &local_158;
                    // WARNING: Subroutine does not return
      core::panicking::panic_fmt();
    }
  } while( true );
}


