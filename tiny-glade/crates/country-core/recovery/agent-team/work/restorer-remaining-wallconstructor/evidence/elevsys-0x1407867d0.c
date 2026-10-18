
void __cdecl
system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments(void)

{
  byte *pbVar1;
  longlong lVar2;
  char *pcVar3;
  float fVar4;
  byte bVar5;
  byte bVar6;
  ulonglong uVar7;
  undefined8 *puVar8;
  undefined8 uVar9;
  uint uVar10;
  uint uVar11;
  int iVar12;
  int iVar13;
  int iVar14;
  int iVar15;
  undefined8 uVar16;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  uint uVar17;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  byte *extraout_RAX_02;
  ulonglong *puVar18;
  longlong *extraout_RAX_03;
  longlong extraout_RAX_04;
  byte *extraout_RAX_05;
  longlong extraout_RAX_06;
  ulonglong extraout_RAX_07;
  ulonglong extraout_RAX_08;
  longlong *extraout_RAX_09;
  longlong *extraout_RAX_10;
  longlong *extraout_RAX_11;
  longlong *extraout_RAX_12;
  undefined8 *in_RCX;
  ulonglong uVar19;
  undefined8 *in_RDX;
  ulonglong uVar20;
  ulonglong uVar21;
  longlong *plVar22;
  longlong lVar23;
  longlong *in_R8;
  undefined *puVar24;
  ushort uVar25;
  ulonglong uVar26;
  ulonglong uVar27;
  ulonglong uVar28;
  longlong lVar29;
  undefined1 (*pauVar30) [16];
  uint uVar31;
  longlong lVar32;
  undefined1 (*pauVar33) [16];
  bool bVar34;
  byte bVar35;
  char cVar36;
  char cVar38;
  char cVar39;
  char cVar40;
  undefined1 auVar37 [16];
  undefined1 auVar41 [16];
  undefined1 auVar42 [16];
  undefined1 auVar43 [16];
  undefined1 auVar44 [16];
  undefined1 unaff_XMM8_Ba;
  undefined1 unaff_XMM8_Bb;
  char cVar45;
  undefined1 unaff_XMM8_Bc;
  char cVar46;
  undefined1 unaff_XMM8_Bd;
  char cVar47;
  undefined1 unaff_XMM8_Be;
  undefined1 unaff_XMM8_Bf;
  char cVar48;
  undefined1 unaff_XMM8_Bg;
  char cVar49;
  undefined1 unaff_XMM8_Bh;
  char cVar50;
  undefined1 unaff_XMM8_Bi;
  undefined1 unaff_XMM8_Bj;
  char cVar51;
  undefined1 unaff_XMM8_Bk;
  char cVar52;
  undefined1 unaff_XMM8_Bl;
  char cVar53;
  undefined1 unaff_XMM8_Bm;
  undefined1 unaff_XMM8_Bn;
  char cVar54;
  undefined1 unaff_XMM8_Bo;
  char cVar55;
  undefined1 unaff_XMM8_Bp;
  char cVar56;
  undefined8 *in_stack_00000028;
  undefined8 *in_stack_00000038;
  longlong *in_stack_00000040;
  longlong *in_stack_00000048;
  undefined8 *in_stack_00000050;
  undefined8 *in_stack_00000058;
  undefined8 *in_stack_00000060;
  undefined8 *in_stack_00000068;
  char *in_stack_00000070;
  undefined8 *in_stack_00000078;
  undefined *local_678;
  ulonglong uStack_670;
  undefined8 local_668;
  longlong lStack_660;
  longlong local_658;
  ulonglong uStack_650;
  ulonglong *local_478;
  longlong lStack_470;
  longlong local_468;
  longlong lStack_460;
  undefined1 uStack_458;
  undefined7 uStack_457;
  undefined1 uStack_450;
  undefined7 uStack_44f;
  undefined1 uStack_448;
  ulonglong *local_278;
  longlong lStack_270;
  longlong local_268;
  ulonglong *local_260;
  longlong lStack_258;
  longlong local_250;
  ulonglong *local_238;
  undefined8 uStack_230;
  undefined8 local_228;
  longlong local_220;
  undefined8 local_208;
  undefined4 *local_200;
  undefined8 local_1f8;
  undefined4 *local_1f0;
  undefined4 *local_1e8;
  undefined8 *local_1e0;
  undefined4 *local_1d8;
  undefined4 *local_1d0;
  longlong local_1c8;
  longlong local_1c0;
  ulonglong *local_1b8;
  undefined4 uStack_1b0;
  undefined4 uStack_1ac;
  undefined4 local_1a8;
  undefined4 uStack_1a4;
  undefined4 uStack_1a0;
  undefined *local_178;
  ulonglong uStack_170;
  undefined8 local_168;
  longlong lStack_160;
  longlong local_158;
  ulonglong uStack_150;
  undefined *local_148;
  ulonglong uStack_140;
  undefined8 local_138;
  longlong lStack_130;
  longlong local_128;
  ulonglong uStack_120;
  longlong local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 *local_100;
  undefined8 local_f8;
  longlong local_f0;
  longlong local_e8;
  undefined4 local_e0;
  undefined4 local_dc;
  undefined4 local_d8;
  undefined4 local_d4;
  longlong *local_d0;
  longlong *local_b8;
  longlong *local_b0;
  longlong *local_a8;
  longlong local_a0;
  undefined8 *local_98;
  uint local_8c;
  longlong *local_88;
  undefined8 local_80;
  
  local_80 = 0xfffffffffffffffe;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    local_d0 = (longlong *)0x0;
  }
  else {
    uVar20 = 0x57;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar20 != 0) {
        if (uVar20 < 0x57) {
          if ((char)(&DAT_142a80506)[uVar20] < -0x40) {
LAB_140786866:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar20 != 0x57) goto LAB_140786866;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar20 != 0xfffffffffffffffe)) {
        if (uVar20 + 2 < 0x57) {
          if ((char)(&UNK_142a80508)[uVar20] < -0x40) {
LAB_1407868c5:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar20 + 2 != 0x57) goto LAB_1407868c5;
      }
    }
    puVar24 = &DAT_142a806c7;
    do {
      bVar35 = puVar24[-1];
      uVar17 = (uint)(char)bVar35;
      if ((char)bVar35 < '\0') {
        bVar5 = puVar24[-2];
        if ((char)bVar5 < -0x40) {
          bVar6 = puVar24[-3];
          if ((char)bVar6 < -0x40) {
            pbVar1 = puVar24 + -4;
            puVar24 = puVar24 + -4;
            uVar17 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar24 = puVar24 + -3;
            uVar17 = bVar6 & 0xf;
          }
          uVar17 = bVar5 & 0x3f | uVar17 << 6;
        }
        else {
          puVar24 = puVar24 + -2;
          uVar17 = bVar5 & 0x1f;
        }
        uVar17 = (uint)(bVar35 & 0x3f) | uVar17 << 6;
        if (uVar17 != 0x5c) goto LAB_140786984;
LAB_140786995:
        if (puVar24 == &UNK_142a8067f) break;
        if (puVar24 + -0x142a8067f < (undefined *)0x47) {
          if (-0x41 < (char)puVar24[1]) break;
        }
        else if (puVar24 + -0x142a8067f == (undefined *)0x47) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar24 = puVar24 + -1;
      if (uVar17 == 0x5c) goto LAB_140786995;
LAB_140786984:
      if (uVar17 == 0x2f) goto LAB_140786995;
    } while (puVar24 != &DAT_142a80680);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar22 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar22 = extraout_RAX_11, extraout_RAX_11 == (longlong *)0x0))
    goto LAB_140787871;
    if (*plVar22 != 0) goto LAB_14078787d;
    *plVar22 = -1;
    local_88 = plVar22;
    puffin::ThreadProfiler::begin_scope();
    *local_88 = *local_88 + 1;
    local_d0 = (longlong *)0x1;
  }
  puVar18 = (ulonglong *)*in_stack_00000060;
  lVar32 = in_stack_00000060[1];
  uVar20 = *puVar18;
  uVar19 = 0;
  uVar27 = uVar20 - *(ulonglong *)(lVar32 + 0x18);
  if (uVar20 < *(ulonglong *)(lVar32 + 0x18)) {
    uVar27 = uVar19;
  }
  uVar21 = uVar20 - *(ulonglong *)(lVar32 + 0x38);
  if (uVar20 < *(ulonglong *)(lVar32 + 0x38)) {
    uVar21 = uVar19;
  }
  uVar20 = *(ulonglong *)(lVar32 + 0x30);
  uVar26 = uVar20 - uVar21;
  if (uVar21 > uVar20) {
    uVar26 = uVar19;
  }
  uVar7 = *(ulonglong *)(lVar32 + 0x10);
  uVar28 = uVar7 - uVar27;
  if (uVar27 > uVar7) {
    uVar28 = uVar19;
  }
  uVar19 = *(ulonglong *)(lVar32 + 0x40);
  *puVar18 = uVar19 - (uVar28 + uVar26);
  if (uVar27 <= uVar7 && uVar7 - uVar27 != 0 || uVar21 <= uVar20 && uVar20 - uVar21 != 0) {
LAB_140786bbb:
    *puVar18 = uVar19;
    *in_stack_00000070 = '\x01';
  }
  else {
    puVar18 = (ulonglong *)*in_stack_00000068;
    lVar32 = in_stack_00000068[1];
    uVar20 = *puVar18;
    uVar19 = 0;
    uVar27 = uVar20 - *(ulonglong *)(lVar32 + 0x18);
    if (uVar20 < *(ulonglong *)(lVar32 + 0x18)) {
      uVar27 = uVar19;
    }
    uVar21 = uVar20 - *(ulonglong *)(lVar32 + 0x38);
    if (uVar20 < *(ulonglong *)(lVar32 + 0x38)) {
      uVar21 = uVar19;
    }
    uVar20 = *(ulonglong *)(lVar32 + 0x30);
    uVar26 = uVar20 - uVar21;
    if (uVar21 > uVar20) {
      uVar26 = uVar19;
    }
    uVar7 = *(ulonglong *)(lVar32 + 0x10);
    uVar28 = uVar7 - uVar27;
    if (uVar27 > uVar7) {
      uVar28 = uVar19;
    }
    uVar19 = *(ulonglong *)(lVar32 + 0x40);
    *puVar18 = uVar19 - (uVar28 + uVar26);
    if (uVar27 <= uVar7 && uVar7 - uVar27 != 0 || uVar21 <= uVar20 && uVar20 - uVar21 != 0)
    goto LAB_140786bbb;
    puVar18 = (ulonglong *)*in_RCX;
    lVar32 = in_RCX[1];
    uVar20 = *puVar18;
    uVar19 = 0;
    uVar27 = uVar20 - *(ulonglong *)(lVar32 + 0x18);
    if (uVar20 < *(ulonglong *)(lVar32 + 0x18)) {
      uVar27 = uVar19;
    }
    uVar21 = uVar20 - *(ulonglong *)(lVar32 + 0x38);
    if (uVar20 < *(ulonglong *)(lVar32 + 0x38)) {
      uVar21 = uVar19;
    }
    lVar29 = uVar27 * 0x10 + *(longlong *)(lVar32 + 8);
    uVar20 = *(ulonglong *)(lVar32 + 0x10) - uVar27;
    if (*(ulonglong *)(lVar32 + 0x10) < uVar27) {
      lVar29 = 8;
      uVar20 = uVar19;
    }
    lVar23 = uVar21 * 0x10 + *(longlong *)(lVar32 + 0x28);
    uVar27 = *(ulonglong *)(lVar32 + 0x30) - uVar21;
    if (*(ulonglong *)(lVar32 + 0x30) < uVar21) {
      lVar23 = 8;
      uVar27 = uVar19;
    }
    lVar2 = uVar20 + uVar27;
    *puVar18 = *(longlong *)(lVar32 + 0x40) - lVar2;
    local_b8 = in_R8;
    local_98 = in_RDX;
    local_88 = local_d0;
    std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
    if ((*extraout_RAX_02 & 1) == 0) {
      local_a8 = local_88;
      std::sys::random::hashmap_random_keys();
      extraout_RAX_02[0] = 1;
      extraout_RAX_02[1] = 0;
      extraout_RAX_02[2] = 0;
      extraout_RAX_02[3] = 0;
      extraout_RAX_02[4] = 0;
      extraout_RAX_02[5] = 0;
      extraout_RAX_02[6] = 0;
      extraout_RAX_02[7] = 0;
      *(longlong *)(extraout_RAX_02 + 8) = extraout_RAX_04;
      *(ulonglong *)(extraout_RAX_02 + 0x10) = uVar21;
      local_658 = extraout_RAX_04;
    }
    else {
      local_658 = *(longlong *)(extraout_RAX_02 + 8);
      uVar21 = *(ulonglong *)(extraout_RAX_02 + 0x10);
    }
    local_468 = uVar20 * 0x10 + lVar29;
    lVar32 = uVar27 * 0x10 + lVar23;
    *(longlong *)(extraout_RAX_02 + 8) = local_658 + 1;
    local_668 = 0;
    lStack_660 = 0;
    local_678 = &DAT_142a80360;
    uStack_670 = 0;
    uStack_458 = (undefined1)lVar32;
    uStack_457 = (undefined7)((ulonglong)lVar32 >> 8);
    uStack_450 = (undefined1)lVar2;
    uStack_44f = (undefined7)((ulonglong)lVar2 >> 8);
    uStack_650 = uVar21;
    local_478 = puVar18;
    lStack_470 = lVar29;
    lStack_460 = lVar23;
    hashbrown::map::impl$82::
    extend<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<bevy_ecs::event::iterators::EventIterator<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments::closure_env_0>,hashbrown::set::impl_10::extend::closure_env_0<country_core::resources::walls::WallId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<bevy_ecs::event::iterators::EventIterator<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments::closure_env_0>_>_>_>
              ();
    local_148 = local_678;
    uStack_140 = uStack_670;
    local_138 = local_668;
    lStack_130 = lStack_660;
    local_128 = local_658;
    uStack_120 = uStack_650;
    puVar18 = (ulonglong *)*local_98;
    lVar32 = local_98[1];
    uVar20 = *puVar18;
    uVar19 = 0;
    uVar27 = uVar20 - *(ulonglong *)(lVar32 + 0x18);
    if (uVar20 < *(ulonglong *)(lVar32 + 0x18)) {
      uVar27 = uVar19;
    }
    uVar21 = uVar20 - *(ulonglong *)(lVar32 + 0x38);
    if (uVar20 < *(ulonglong *)(lVar32 + 0x38)) {
      uVar21 = uVar19;
    }
    lVar29 = uVar27 * 0x10 + *(longlong *)(lVar32 + 8);
    uVar20 = *(ulonglong *)(lVar32 + 0x10) - uVar27;
    if (*(ulonglong *)(lVar32 + 0x10) < uVar27) {
      lVar29 = 8;
      uVar20 = uVar19;
    }
    lVar23 = uVar21 * 0x10 + *(longlong *)(lVar32 + 0x28);
    uVar27 = *(ulonglong *)(lVar32 + 0x30) - uVar21;
    if (*(ulonglong *)(lVar32 + 0x30) < uVar21) {
      lVar23 = 8;
      uVar27 = uVar19;
    }
    lVar2 = uVar27 + uVar20;
    *puVar18 = *(longlong *)(lVar32 + 0x40) - lVar2;
    std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
    if ((*extraout_RAX_05 & 1) == 0) {
      local_b0 = local_88;
      std::sys::random::hashmap_random_keys();
      extraout_RAX_05[0] = 1;
      extraout_RAX_05[1] = 0;
      extraout_RAX_05[2] = 0;
      extraout_RAX_05[3] = 0;
      extraout_RAX_05[4] = 0;
      extraout_RAX_05[5] = 0;
      extraout_RAX_05[6] = 0;
      extraout_RAX_05[7] = 0;
      *(longlong *)(extraout_RAX_05 + 8) = extraout_RAX_06;
      *(ulonglong *)(extraout_RAX_05 + 0x10) = uVar21;
      local_658 = extraout_RAX_06;
    }
    else {
      local_658 = *(longlong *)(extraout_RAX_05 + 8);
      uVar21 = *(ulonglong *)(extraout_RAX_05 + 0x10);
    }
    local_468 = uVar20 * 0x10 + lVar29;
    lVar32 = uVar27 * 0x10 + lVar23;
    *(longlong *)(extraout_RAX_05 + 8) = local_658 + 1;
    local_668 = 0;
    lStack_660 = 0;
    local_678 = &DAT_142a80360;
    uStack_670 = 0;
    uStack_458 = (undefined1)lVar32;
    uStack_457 = (undefined7)((ulonglong)lVar32 >> 8);
    uStack_450 = (undefined1)lVar2;
    uStack_44f = (undefined7)((ulonglong)lVar2 >> 8);
    uStack_650 = uVar21;
    local_478 = puVar18;
    lStack_470 = lVar29;
    lStack_460 = lVar23;
    hashbrown::map::impl$82::
    extend<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<bevy_ecs::event::iterators::EventIterator<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments::closure_env_0>,hashbrown::set::impl_10::extend::closure_env_0<country_core::resources::walls::WallId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<bevy_ecs::event::iterators::EventIterator<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments::closure_env_0>_>_>_>
              ();
    plVar22 = local_b8;
    local_178 = local_678;
    uStack_170 = uStack_670;
    local_168 = local_668;
    lStack_160 = lStack_660;
    local_158 = local_658;
    uStack_150 = uStack_650;
    bVar34 = true;
    if ((lStack_130 == 0) && (*in_stack_00000070 == '\0')) {
      bVar34 = lStack_660 != 0;
    }
    country_core::utils::dirty_tiles::DirtyTiles::new_clear();
    *(undefined4 *)plVar22[2] = *(undefined4 *)((longlong)plVar22 + 0x1c);
    lVar29 = *(longlong *)(*plVar22 + 8);
    lVar32 = lVar29 + *(longlong *)(*plVar22 + 0x10) * 8;
    bVar35 = 0;
    while (lVar29 != lVar32) {
      tabloid::Subscriber::take<country_core::utils::dirty_tiles::DirtyTiles>();
      lVar29 = lVar29 + 8;
      if (local_238 != (ulonglong *)0x0) {
        local_1b8 = local_238;
        uStack_1b0 = (undefined4)uStack_230;
        uStack_1ac = (undefined4)((ulonglong)uStack_230 >> 0x20);
        local_1a8 = (undefined4)local_228;
        uStack_1a4 = (undefined4)((ulonglong)local_228 >> 0x20);
        country_core::utils::dirty_tiles::DirtyTiles::extend();
        puVar18 = local_1b8;
        parking_lot_core::parking_lot::deadlock_impl::release_resource();
        parking_lot_core::parking_lot::deadlock_impl::release_resource();
        LOCK();
        uVar20 = *puVar18;
        *puVar18 = *puVar18 - 0x10;
        UNLOCK();
        if ((uVar20 & 0xfffffffffffffff2) == 0x12) {
          parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow();
        }
        lVar23 = CONCAT44(uStack_1a4,local_1a8);
        local_1a8 = 0;
        uStack_1a4 = 0;
        local_118 = lVar23 + -0x10;
        LOCK();
        plVar22 = (longlong *)(lVar23 + -0x10);
        *plVar22 = *plVar22 + -1;
        UNLOCK();
        if (*plVar22 == 0) {
          alloc::sync::Arc::
          drop_slow<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles>_>,alloc::alloc::Global>
                    ();
        }
        bVar35 = 1;
      }
    }
    memcpy(&local_678,&local_478,0x200);
    *in_stack_00000070 = '\0';
    if ((bool)(bVar34 | bVar35)) {
      puVar8 = (undefined8 *)*in_stack_00000028;
      find_overlap_segments::OverhangSupportFinderCache::new();
      pauVar30 = (undefined1 (*) [16])*puVar8;
      auVar37 = *pauVar30;
      uVar17 = ~(uint)(ushort)((ushort)(SUB161(auVar37 >> 7,0) & 1) |
                               (ushort)(SUB161(auVar37 >> 0xf,0) & 1) << 1 |
                               (ushort)(SUB161(auVar37 >> 0x17,0) & 1) << 2 |
                               (ushort)(SUB161(auVar37 >> 0x1f,0) & 1) << 3 |
                               (ushort)(SUB161(auVar37 >> 0x27,0) & 1) << 4 |
                               (ushort)(SUB161(auVar37 >> 0x2f,0) & 1) << 5 |
                               (ushort)(SUB161(auVar37 >> 0x37,0) & 1) << 6 |
                               (ushort)(SUB161(auVar37 >> 0x3f,0) & 1) << 7 |
                               (ushort)(SUB161(auVar37 >> 0x47,0) & 1) << 8 |
                               (ushort)(SUB161(auVar37 >> 0x4f,0) & 1) << 9 |
                               (ushort)(SUB161(auVar37 >> 0x57,0) & 1) << 10 |
                               (ushort)(SUB161(auVar37 >> 0x5f,0) & 1) << 0xb |
                               (ushort)(SUB161(auVar37 >> 0x67,0) & 1) << 0xc |
                               (ushort)(SUB161(auVar37 >> 0x6f,0) & 1) << 0xd |
                               (ushort)(SUB161(auVar37 >> 0x77,0) & 1) << 0xe |
                              (ushort)(byte)(auVar37[0xf] >> 7) << 0xf);
      local_f8 = *in_stack_00000038;
      local_b8 = (longlong *)
                 CONCAT44(local_b8._4_4_,*(undefined4 *)((longlong)in_stack_00000058 + 0x1c));
      local_208 = *in_stack_00000058;
      local_200 = (undefined4 *)in_stack_00000058[2];
      local_e0 = *(undefined4 *)((longlong)in_stack_00000050 + 0x1c);
      local_1f8 = *in_stack_00000050;
      local_1f0 = (undefined4 *)in_stack_00000050[2];
      local_d8 = *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
      local_f0 = *in_stack_00000048;
      local_1d8 = (undefined4 *)in_stack_00000048[2];
      local_d4 = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
      local_e8 = *in_stack_00000040;
      local_1d0 = (undefined4 *)in_stack_00000040[2];
      local_dc = *(undefined4 *)((longlong)in_stack_00000078 + 0x1c);
      local_98 = (undefined8 *)*in_stack_00000078;
      local_1e8 = (undefined4 *)in_stack_00000078[2];
      iVar12 = -(uint)(CONCAT13(unaff_XMM8_Bd,
                                CONCAT12(unaff_XMM8_Bc,CONCAT11(unaff_XMM8_Bb,unaff_XMM8_Ba))) ==
                      CONCAT13(unaff_XMM8_Bd,
                               CONCAT12(unaff_XMM8_Bc,CONCAT11(unaff_XMM8_Bb,unaff_XMM8_Ba))));
      cVar45 = (char)((uint)iVar12 >> 8);
      cVar46 = (char)((uint)iVar12 >> 0x10);
      cVar47 = (char)((uint)iVar12 >> 0x18);
      iVar13 = -(uint)(CONCAT13(unaff_XMM8_Bh,
                                CONCAT12(unaff_XMM8_Bg,CONCAT11(unaff_XMM8_Bf,unaff_XMM8_Be))) ==
                      CONCAT13(unaff_XMM8_Bh,
                               CONCAT12(unaff_XMM8_Bg,CONCAT11(unaff_XMM8_Bf,unaff_XMM8_Be))));
      cVar48 = (char)((uint)iVar13 >> 8);
      cVar49 = (char)((uint)iVar13 >> 0x10);
      cVar50 = (char)((uint)iVar13 >> 0x18);
      iVar14 = -(uint)(CONCAT13(unaff_XMM8_Bl,
                                CONCAT12(unaff_XMM8_Bk,CONCAT11(unaff_XMM8_Bj,unaff_XMM8_Bi))) ==
                      CONCAT13(unaff_XMM8_Bl,
                               CONCAT12(unaff_XMM8_Bk,CONCAT11(unaff_XMM8_Bj,unaff_XMM8_Bi))));
      cVar51 = (char)((uint)iVar14 >> 8);
      cVar52 = (char)((uint)iVar14 >> 0x10);
      cVar53 = (char)((uint)iVar14 >> 0x18);
      iVar15 = -(uint)(CONCAT13(unaff_XMM8_Bp,
                                CONCAT12(unaff_XMM8_Bo,CONCAT11(unaff_XMM8_Bn,unaff_XMM8_Bm))) ==
                      CONCAT13(unaff_XMM8_Bp,
                               CONCAT12(unaff_XMM8_Bo,CONCAT11(unaff_XMM8_Bn,unaff_XMM8_Bm))));
      cVar54 = (char)((uint)iVar15 >> 8);
      cVar55 = (char)((uint)iVar15 >> 0x10);
      cVar56 = (char)((uint)iVar15 >> 0x18);
      local_88 = (longlong *)puVar8[3];
      pauVar33 = pauVar30 + 1;
      local_1c8 = local_f0 + 0x20;
      local_1c0 = local_e8 + 0x20;
      local_1e0 = local_98 + 4;
      local_100 = puVar8;
LAB_1407870fe:
      do {
        if (local_88 == (longlong *)0x0) goto LAB_14078772d;
        if ((short)uVar17 == 0) {
          do {
            auVar37 = *pauVar33;
            uVar25 = (ushort)(SUB161(auVar37 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar37 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar37 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar37 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar37 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar37 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar37 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar37 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar37 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar37 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar37 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar37 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar37 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar37 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar37 >> 0x77,0) & 1) << 0xe |
                     (ushort)(byte)(auVar37[0xf] >> 7) << 0xf;
            pauVar30 = pauVar30 + -0x2a8;
            pauVar33 = pauVar33 + 1;
          } while (uVar25 == 0xffff);
          uVar31 = ~(uint)uVar25;
          uVar17 = uVar31 & -(uint)uVar25 - 2;
        }
        else {
          uVar31 = uVar17;
          uVar17 = uVar17 - 1 & uVar17;
        }
        uVar10 = 0;
        for (; (uVar31 & 1) == 0; uVar31 = uVar31 >> 1 | 0x80000000) {
          uVar10 = uVar10 + 1;
        }
        local_88 = (longlong *)((longlong)local_88 + -1);
        lVar29 = (ulonglong)uVar10 * -0x2a8;
        lVar32 = *(longlong *)((longlong)pauVar30 + lVar29 + -0x2a8);
        fVar4 = *(float *)((longlong)pauVar30 + lVar29 + -0x20);
        local_a0 = lVar32;
        if ((fVar4 == 0.0) && (!NAN(fVar4))) {
          *local_200 = local_b8._0_4_;
          country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage::
          clear_and_get_mut();
          *local_1f0 = local_e0;
          local_478 = (ulonglong *)0x0;
          uStack_450 = 0;
          uStack_44f = 0;
          uStack_448 = 0;
          lStack_460 = 0;
          uStack_458 = 0;
          uStack_457 = 0;
          lStack_470 = 4;
          local_468 = 0;
          hashbrown::map::HashMap::
          insert<country_core::resources::walls::WallId,country_core::resources::walls::elevation_support_stats::ElevationSupports,std::hash::random::RandomState,alloc::alloc::Global>
                    ();
          if ((!SBORROW8(0,(longlong)local_1b8)) && (local_1b8 != (ulonglong *)0x0)) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          detect_overhang_support_segments::closure_2();
          *local_1e8 = local_dc;
          lVar29 = local_98[6];
          uVar9 = local_98[8];
          if (lVar29 == local_98[4]) {
            alloc::raw_vec::RawVec::
            grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                      ();
          }
          lVar23 = local_98[5];
          *(undefined8 *)(lVar23 + lVar29 * 0x28) = 0;
          *(undefined8 *)(lVar23 + 8 + lVar29 * 0x28) = 4;
          *(undefined8 *)(lVar23 + 0x10 + lVar29 * 0x28) = 0;
          *(longlong *)(lVar23 + 0x18 + lVar29 * 0x28) = lVar32;
          *(undefined8 *)(lVar23 + 0x20 + lVar29 * 0x28) = uVar9;
          local_98[6] = lVar29 + 1;
          local_98[8] = local_98[8] + 1;
          goto LAB_1407870fe;
        }
        if (lStack_130 != 0) {
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
          bVar35 = (byte)(extraout_RAX_07 >> 0x39);
          auVar37 = ZEXT216(CONCAT11(bVar35,bVar35));
          auVar37 = pshuflw(auVar37,auVar37,0);
          lVar23 = 0;
          uVar20 = extraout_RAX_07;
          while( true ) {
            uVar20 = uStack_140 & uVar20;
            pcVar3 = local_148 + uVar20;
            cVar36 = auVar37[0];
            auVar43[0] = -(*pcVar3 == cVar36);
            cVar38 = auVar37[1];
            auVar43[1] = -(pcVar3[1] == cVar38);
            cVar39 = auVar37[2];
            auVar43[2] = -(pcVar3[2] == cVar39);
            cVar40 = auVar37[3];
            auVar43[3] = -(pcVar3[3] == cVar40);
            auVar43[4] = -(pcVar3[4] == cVar36);
            auVar43[5] = -(pcVar3[5] == cVar38);
            auVar43[6] = -(pcVar3[6] == cVar39);
            auVar43[7] = -(pcVar3[7] == cVar40);
            auVar43[8] = -(pcVar3[8] == cVar36);
            auVar43[9] = -(pcVar3[9] == cVar38);
            auVar43[10] = -(pcVar3[10] == cVar39);
            auVar43[0xb] = -(pcVar3[0xb] == cVar40);
            auVar43[0xc] = -(pcVar3[0xc] == cVar36);
            auVar43[0xd] = -(pcVar3[0xd] == cVar38);
            auVar43[0xe] = -(pcVar3[0xe] == cVar39);
            auVar43[0xf] = -(pcVar3[0xf] == cVar40);
            uVar25 = (ushort)(SUB161(auVar43 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar43 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar43 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar43 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar43 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar43 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar43 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar43 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar43 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar43 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar43 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar43 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar43 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar43 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar43 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar43[0xf] >> 7) << 0xf;
            uVar31 = (uint)uVar25;
            while (uVar25 != 0) {
              uVar10 = 0;
              for (uVar11 = uVar31; (uVar11 & 1) == 0; uVar11 = uVar11 >> 1 | 0x80000000) {
                uVar10 = uVar10 + 1;
              }
              uVar27 = uVar10 + uVar20 & uStack_140;
              uVar19 = CONCAT71((int7)(uVar27 * 8 >> 8),1);
              if (lVar32 == *(longlong *)(local_148 + uVar27 * -8 + -8)) goto LAB_140787343;
              uVar25 = (ushort)(uVar31 - 1) & (ushort)uVar31;
              uVar31 = CONCAT22((short)(uVar31 - 1 >> 0x10),uVar25);
            }
            auVar41[0] = -(*pcVar3 == (char)iVar12);
            auVar41[1] = -(pcVar3[1] == cVar45);
            auVar41[2] = -(pcVar3[2] == cVar46);
            auVar41[3] = -(pcVar3[3] == cVar47);
            auVar41[4] = -(pcVar3[4] == (char)iVar13);
            auVar41[5] = -(pcVar3[5] == cVar48);
            auVar41[6] = -(pcVar3[6] == cVar49);
            auVar41[7] = -(pcVar3[7] == cVar50);
            auVar41[8] = -(pcVar3[8] == (char)iVar14);
            auVar41[9] = -(pcVar3[9] == cVar51);
            auVar41[10] = -(pcVar3[10] == cVar52);
            auVar41[0xb] = -(pcVar3[0xb] == cVar53);
            auVar41[0xc] = -(pcVar3[0xc] == (char)iVar15);
            auVar41[0xd] = -(pcVar3[0xd] == cVar54);
            auVar41[0xe] = -(pcVar3[0xe] == cVar55);
            auVar41[0xf] = -(pcVar3[0xf] == cVar56);
            if ((((((((((((((((SUB161(auVar41 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar41 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar41 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar41 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar41 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar41 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar41 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar41 >> 0x3f,0) & 1) != 0) || (SUB161(auVar41 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar41 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar41 >> 0x57,0) & 1) != 0) || (SUB161(auVar41 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar41 >> 0x67,0) & 1) != 0) || (SUB161(auVar41 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar41 >> 0x77,0) & 1) != 0) || auVar41[0xf] < '\0') break;
            uVar20 = uVar20 + lVar23 + 0x10;
            lVar23 = lVar23 + 0x10;
          }
        }
        uVar19 = 0;
LAB_140787343:
        if (lStack_160 == 0) {
          bVar34 = false;
LAB_140787405:
          cVar36 = (char)uVar19;
          uVar31 = *(uint *)((longlong)pauVar30 + lVar29 + -0x48);
        }
        else {
          local_8c = (uint)uVar19;
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
          bVar35 = (byte)(extraout_RAX_08 >> 0x39);
          auVar37 = ZEXT216(CONCAT11(bVar35,bVar35));
          auVar37 = pshuflw(auVar37,auVar37,0);
          lVar23 = 0;
          uVar20 = extraout_RAX_08;
          while( true ) {
            uVar20 = uStack_170 & uVar20;
            pcVar3 = local_178 + uVar20;
            cVar36 = auVar37[0];
            auVar44[0] = -(*pcVar3 == cVar36);
            cVar38 = auVar37[1];
            auVar44[1] = -(pcVar3[1] == cVar38);
            cVar39 = auVar37[2];
            auVar44[2] = -(pcVar3[2] == cVar39);
            cVar40 = auVar37[3];
            auVar44[3] = -(pcVar3[3] == cVar40);
            auVar44[4] = -(pcVar3[4] == cVar36);
            auVar44[5] = -(pcVar3[5] == cVar38);
            auVar44[6] = -(pcVar3[6] == cVar39);
            auVar44[7] = -(pcVar3[7] == cVar40);
            auVar44[8] = -(pcVar3[8] == cVar36);
            auVar44[9] = -(pcVar3[9] == cVar38);
            auVar44[10] = -(pcVar3[10] == cVar39);
            auVar44[0xb] = -(pcVar3[0xb] == cVar40);
            auVar44[0xc] = -(pcVar3[0xc] == cVar36);
            auVar44[0xd] = -(pcVar3[0xd] == cVar38);
            auVar44[0xe] = -(pcVar3[0xe] == cVar39);
            auVar44[0xf] = -(pcVar3[0xf] == cVar40);
            uVar25 = (ushort)(SUB161(auVar44 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar44 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar44 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar44 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar44 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar44 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar44 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar44 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar44 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar44 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar44 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar44 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar44 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar44 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar44 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar44[0xf] >> 7) << 0xf;
            uVar31 = (uint)uVar25;
            while (uVar25 != 0) {
              uVar10 = 0;
              for (uVar11 = uVar31; (uVar11 & 1) == 0; uVar11 = uVar11 >> 1 | 0x80000000) {
                uVar10 = uVar10 + 1;
              }
              bVar34 = true;
              uVar19 = (ulonglong)local_8c;
              if (lVar32 == *(longlong *)(local_178 + (uVar10 + uVar20 & uStack_170) * -8 + -8))
              goto LAB_140787405;
              uVar25 = (ushort)(uVar31 - 1) & (ushort)uVar31;
              uVar31 = CONCAT22((short)(uVar31 - 1 >> 0x10),uVar25);
            }
            auVar42[0] = -(*pcVar3 == (char)iVar12);
            auVar42[1] = -(pcVar3[1] == cVar45);
            auVar42[2] = -(pcVar3[2] == cVar46);
            auVar42[3] = -(pcVar3[3] == cVar47);
            auVar42[4] = -(pcVar3[4] == (char)iVar13);
            auVar42[5] = -(pcVar3[5] == cVar48);
            auVar42[6] = -(pcVar3[6] == cVar49);
            auVar42[7] = -(pcVar3[7] == cVar50);
            auVar42[8] = -(pcVar3[8] == (char)iVar14);
            auVar42[9] = -(pcVar3[9] == cVar51);
            auVar42[10] = -(pcVar3[10] == cVar52);
            auVar42[0xb] = -(pcVar3[0xb] == cVar53);
            auVar42[0xc] = -(pcVar3[0xc] == (char)iVar15);
            auVar42[0xd] = -(pcVar3[0xd] == cVar54);
            auVar42[0xe] = -(pcVar3[0xe] == cVar55);
            auVar42[0xf] = -(pcVar3[0xf] == cVar56);
            if ((((((((((((((((SUB161(auVar42 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar42 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar42 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar42 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar42 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar42 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar42 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar42 >> 0x3f,0) & 1) != 0) || (SUB161(auVar42 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar42 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar42 >> 0x57,0) & 1) != 0) || (SUB161(auVar42 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar42 >> 0x67,0) & 1) != 0) || (SUB161(auVar42 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar42 >> 0x77,0) & 1) != 0) || auVar42[0xf] < '\0') break;
            uVar20 = uVar20 + lVar23 + 0x10;
            lVar23 = lVar23 + 0x10;
          }
          bVar34 = false;
          cVar36 = (char)local_8c;
          uVar31 = *(uint *)((longlong)pauVar30 + lVar29 + -0x48);
        }
        if (uVar31 < 2) {
          local_1b8 = *(ulonglong **)((longlong)pauVar30 + lVar29 + -0x48);
          uVar9 = *(undefined8 *)((longlong)pauVar30 + lVar29 + -0x3c);
          uVar16 = *(undefined8 *)((longlong)pauVar30 + lVar29 + -0x34);
          uStack_1b0 = (undefined4)*(undefined8 *)((longlong)pauVar30 + lVar29 + -0x40);
          uStack_1ac = (undefined4)uVar9;
          local_1a8 = (undefined4)((ulonglong)uVar9 >> 0x20);
          uStack_1a4 = (undefined4)uVar16;
          uStack_1a0 = (undefined4)((ulonglong)uVar16 >> 0x20);
          plVar22 = (longlong *)((longlong)pauVar30 + lVar29 + -0x44);
          if (((ulonglong)local_1b8 & 1) == 0) {
            local_118 = *plVar22;
            uStack_110 = *(undefined8 *)((longlong)pauVar30 + lVar29 + -0x3c);
            if (cVar36 == '\0') {
              ::utils::geometry::circle::Circle2d::aabb();
              country_core::utils::dirty_tiles::DirtyTiles::intersects_world_space_rect();
              if (!bVar34 && extraout_AL_01 == '\0') goto LAB_1407870fe;
            }
            detect_overhang_support_segments::closure_2();
            find_overlap_segments::find_overhang_support_segments();
            *local_1d8 = local_d8;
            uStack_458 = (undefined1)*(undefined8 *)(local_f0 + 0x40);
            uStack_457 = (undefined7)((ulonglong)*(undefined8 *)(local_f0 + 0x40) >> 8);
            local_468 = local_268;
            local_478 = local_278;
            lStack_470 = lStack_270;
            lStack_460 = local_a0;
            lVar32 = *(longlong *)(local_f0 + 0x30);
            lVar29 = local_f0;
            if (lVar32 == *(longlong *)(local_f0 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                        ();
              lVar29 = local_f0;
            }
          }
          else {
            local_108 = *(undefined8 *)((longlong)pauVar30 + lVar29 + -0x34);
            local_118 = *plVar22;
            uStack_110 = *(undefined8 *)((longlong)pauVar30 + lVar29 + -0x3c);
            if (cVar36 == '\0') {
              ::utils::geometry::rectangle::Rectangle2d::aabb();
              country_core::utils::dirty_tiles::DirtyTiles::intersects_world_space_rect();
              if (extraout_AL_00 == '\0') goto LAB_1407870fe;
            }
            detect_overhang_support_segments::closure_2();
            find_overlap_segments::find_overhang_support_segments();
            *local_1d0 = local_d4;
            uStack_458 = (undefined1)*(undefined8 *)(local_e8 + 0x40);
            uStack_457 = (undefined7)((ulonglong)*(undefined8 *)(local_e8 + 0x40) >> 8);
            local_468 = local_250;
            local_478 = local_260;
            lStack_470 = lStack_258;
            lStack_460 = local_a0;
            lVar32 = *(longlong *)(local_e8 + 0x30);
            lVar29 = local_e8;
            if (lVar32 == *(longlong *)(local_e8 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                        ();
              lVar29 = local_e8;
            }
          }
          lVar23 = *(longlong *)(lVar29 + 0x28);
          *(ulonglong *)(lVar23 + 0x20 + lVar32 * 0x28) = CONCAT71(uStack_457,uStack_458);
          plVar22 = (longlong *)(lVar23 + 0x10 + lVar32 * 0x28);
          *plVar22 = local_468;
          plVar22[1] = lStack_460;
          puVar8 = (undefined8 *)(lVar23 + lVar32 * 0x28);
          *puVar8 = local_478;
          puVar8[1] = lStack_470;
          *(longlong *)(lVar29 + 0x30) = lVar32 + 1;
          *(longlong *)(lVar29 + 0x40) = *(longlong *)(lVar29 + 0x40) + 1;
        }
      } while( true );
    }
    if ((uStack_170 != 0) && (uStack_170 + (uStack_170 * 8 + 0x17 & 0xfffffffffffffff0) != -0x11)) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if ((uStack_140 != 0) && (uStack_140 + (uStack_140 * 8 + 0x17 & 0xfffffffffffffff0) != -0x11)) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  if (local_d0 == (longlong *)0x0) {
    return;
  }
  puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
  if (*extraout_RAX_03 == 1) {
    plVar22 = extraout_RAX_03 + 1;
  }
  else if ((*extraout_RAX_03 != 0) ||
          (std::sys::thread_local::native::lazy::Storage::
           initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                     (), plVar22 = extraout_RAX_10, extraout_RAX_10 == (longlong *)0x0)) {
LAB_140787871:
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  if (*plVar22 != 0) {
LAB_14078787d:
                    // WARNING: Subroutine does not return
    core::cell::panic_already_borrowed();
  }
  *plVar22 = -1;
  local_88 = plVar22;
  puffin::ThreadProfiler::end_scope();
LAB_140786c07:
  *local_88 = *local_88 + 1;
  return;
LAB_14078772d:
  if (local_238 != (ulonglong *)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_220 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if ((uStack_170 != 0) && (uStack_170 + (uStack_170 * 8 + 0x17 & 0xfffffffffffffff0) != -0x11)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if ((uStack_140 != 0) && (uStack_140 + (uStack_140 * 8 + 0x17 & 0xfffffffffffffff0) != -0x11)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_d0 == (longlong *)0x0) {
    return;
  }
  puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
  if (*extraout_RAX_09 == 1) {
    plVar22 = extraout_RAX_09 + 1;
  }
  else if ((*extraout_RAX_09 != 0) ||
          (std::sys::thread_local::native::lazy::Storage::
           initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                     (), plVar22 = extraout_RAX_12, extraout_RAX_12 == (longlong *)0x0))
  goto LAB_140787871;
  if (*plVar22 != 0) goto LAB_14078787d;
  *plVar22 = -1;
  local_88 = plVar22;
  puffin::ThreadProfiler::end_scope();
  goto LAB_140786c07;
}

