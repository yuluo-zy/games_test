// addr 0x140b2cfa0 sig void __cdecl signifier_create(void) size 2458

void __cdecl country_core::systems::ui::signifiers::signifier_create::signifier_create(void)

{
  byte *pbVar1;
  char *pcVar2;
  byte bVar3;
  byte bVar4;
  undefined1 uVar5;
  longlong lVar6;
  longlong lVar7;
  void *_Src;
  undefined8 *puVar8;
  code *pcVar9;
  uint uVar10;
  uint uVar11;
  int iVar12;
  int iVar13;
  int iVar14;
  int iVar15;
  char extraout_AL;
  uint extraout_EAX;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined *puVar16;
  longlong *extraout_RAX_01;
  byte *extraout_RAX_02;
  longlong extraout_RAX_03;
  undefined *extraout_RAX_04;
  ulonglong extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  uint uVar17;
  undefined8 *in_RCX;
  longlong *plVar18;
  undefined8 *in_RDX;
  ulonglong uVar19;
  undefined8 *puVar20;
  undefined8 *in_R8;
  longlong in_R9;
  ushort uVar21;
  longlong lVar22;
  longlong lVar23;
  bool bVar24;
  char cVar25;
  byte bVar26;
  char cVar28;
  char cVar29;
  char cVar30;
  undefined1 auVar27 [16];
  undefined1 auVar31 [16];
  undefined1 auVar32 [16];
  undefined1 unaff_XMM6_Ba;
  undefined1 unaff_XMM6_Bb;
  undefined1 unaff_XMM6_Bc;
  undefined1 unaff_XMM6_Bd;
  undefined1 unaff_XMM6_Be;
  undefined1 unaff_XMM6_Bf;
  undefined1 unaff_XMM6_Bg;
  undefined1 unaff_XMM6_Bh;
  undefined1 unaff_XMM6_Bi;
  undefined1 unaff_XMM6_Bj;
  undefined1 unaff_XMM6_Bk;
  undefined1 unaff_XMM6_Bl;
  undefined1 unaff_XMM6_Bm;
  undefined1 unaff_XMM6_Bn;
  undefined1 unaff_XMM6_Bo;
  undefined1 unaff_XMM6_Bp;
  longlong *in_stack_00000028;
  undefined8 *in_stack_00000030;
  undefined8 *local_2f8 [3];
  longlong lStack_2e0;
  longlong local_2d8;
  longlong lStack_2d0;
  undefined8 local_2c8;
  undefined8 uStack_2c0;
  undefined8 local_2b8;
  undefined8 uStack_2b0;
  undefined8 local_2a8;
  undefined8 uStack_2a0;
  undefined8 local_298;
  undefined8 uStack_290;
  ulonglong local_198;
  undefined *local_188;
  ulonglong uStack_180;
  undefined8 *local_178;
  undefined8 *local_160;
  undefined8 local_158;
  undefined8 local_150;
  undefined8 *local_148;
  undefined8 *puStack_140;
  undefined8 *local_138;
  longlong *plStack_130;
  longlong local_128;
  undefined8 *puStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined1 auStack_100 [16];
  undefined1 auStack_f0 [16];
  undefined1 local_e0;
  undefined7 uStack_df;
  longlong local_d8;
  undefined *local_c8;
  ulonglong uStack_c0;
  undefined8 *local_b8;
  longlong *plStack_b0;
  longlong local_a8;
  undefined8 *puStack_a0;
  undefined8 *local_90;
  longlong local_88;
  undefined8 *local_80;
  undefined8 *local_78;
  longlong *local_70;
  longlong *local_68;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  local_90 = in_R8;
  local_78 = in_RDX;
  local_70 = (longlong *)in_R9;
  puffin::are_scopes_on();
  local_80 = in_RCX;
  if (extraout_AL == '\0') {
    lVar22 = 0;
  }
  else {
    in_RDX = (undefined8 *)&DAT_142aeb288;
    uVar19 = 0x49;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar19 != 0) {
        if (uVar19 < 0x49) {
          if ((char)(&DAT_142aeb288)[uVar19] < -0x40) {
LAB_140b2d036:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar19 != 0x49) goto LAB_140b2d036;
      }
      in_RDX = (undefined8 *)&DAT_142aeb288;
      core::str::impl$0::rfind<ref_<str_>_>();
      if (extraout_RAX_00 == 1) {
        if (uVar19 != 0xfffffffffffffffe) {
          if (uVar19 + 2 < 0x49) {
            if ((char)(&UNK_142aeb28a)[uVar19] < -0x40) {
LAB_140b2d095:
                    // WARNING: Subroutine does not return
              core::str::slice_error_fail();
            }
          }
          else if (uVar19 + 2 != 0x49) goto LAB_140b2d095;
        }
        in_RDX = (undefined8 *)(&UNK_142aeb28a + uVar19);
      }
    }
    puVar16 = &DAT_142aeb859;
    do {
      bVar26 = puVar16[-1];
      uVar17 = (uint)(char)bVar26;
      if ((char)bVar26 < '\0') {
        bVar3 = puVar16[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar16[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar16 + -4;
            puVar16 = puVar16 + -4;
            uVar17 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar16 = puVar16 + -3;
            uVar17 = bVar4 & 0xf;
          }
          uVar17 = bVar3 & 0x3f | uVar17 << 6;
        }
        else {
          puVar16 = puVar16 + -2;
          uVar17 = bVar3 & 0x1f;
        }
        uVar17 = (uint)(bVar26 & 0x3f) | uVar17 << 6;
        if (uVar17 != 0x5c) goto LAB_140b2d159;
LAB_140b2d16e:
        if (puVar16 == &UNK_142aeb817) break;
        if (puVar16 + -0x142aeb817 < (undefined *)0x41) {
          if (-0x41 < (char)puVar16[1]) break;
        }
        else if (puVar16 + -0x142aeb817 == (undefined *)0x41) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar16 = puVar16 + -1;
      if (uVar17 == 0x5c) goto LAB_140b2d16e;
LAB_140b2d159:
      if (uVar17 == 0x2f) goto LAB_140b2d16e;
    } while (puVar16 != &DAT_142aeb818);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar18 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar18 = extraout_RAX_08, extraout_RAX_08 == (longlong *)0x0))
    goto LAB_140b2d922;
    if (*plVar18 != 0) goto LAB_140b2d92e;
    *plVar18 = -1;
    local_68 = plVar18;
    puffin::ThreadProfiler::begin_scope();
    *local_68 = *local_68 + 1;
    lVar22 = 1;
  }
  lVar23 = *in_stack_00000028;
  puVar20 = (undefined8 *)in_stack_00000028[1];
  lVar6 = *(longlong *)(lVar23 + 0xf8);
  lVar7 = *(longlong *)(lVar23 + 0x100);
  uVar5 = *(undefined1 *)(lVar23 + 0x108);
  local_d8 = lVar22;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  local_68 = (longlong *)lVar22;
  if ((*extraout_RAX_02 & 1) == 0) {
    local_88 = lVar22;
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
    *(undefined8 **)(extraout_RAX_02 + 0x10) = in_RDX;
    local_a8 = extraout_RAX_03;
  }
  else {
    local_a8 = *(longlong *)(extraout_RAX_02 + 8);
    in_RDX = *(undefined8 **)(extraout_RAX_02 + 0x10);
  }
  puStack_120 = (undefined8 *)(lVar6 + lVar7 * 4);
  local_138 = puVar20 + 0x1f;
  puStack_140 = puVar20 + 0x34;
  *(longlong *)(extraout_RAX_02 + 8) = local_a8 + 1;
  local_b8 = (undefined8 *)0x0;
  plStack_b0 = (longlong *)0x0;
  local_c8 = &DAT_142aeb0e0;
  uStack_c0 = 0;
  auStack_f0 = (undefined1  [16])0x0;
  auStack_100 = (undefined1  [16])0x0;
  local_118 = 8;
  uStack_110 = 0;
  local_108 = 8;
  local_148 = puVar20;
  plStack_130 = (longlong *)lVar23;
  local_128 = lVar6;
  local_e0 = uVar5;
  puStack_a0 = in_RDX;
  bevy_ecs::query::iter::impl$1::size_hint<ref_<country_core::components::TreeAmbience>,tuple_<>_>()
  ;
  if (local_b8 < local_2f8[0]) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::history::HistorySnapshotId,tuple_<>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::history::HistorySnapshotId,tuple_<>,std::hash::random::RandomState>_>
              ();
  }
  uStack_290 = CONCAT71(uStack_df,local_e0);
  local_298 = auStack_f0._8_8_;
  local_2a8 = auStack_100._8_8_;
  uStack_2a0 = auStack_f0._0_8_;
  local_2b8 = local_108;
  uStack_2b0 = auStack_100._0_8_;
  local_2c8 = local_118;
  uStack_2c0 = uStack_110;
  local_2d8 = local_128;
  lStack_2d0 = (longlong)puStack_120;
  local_2f8[2] = local_138;
  lStack_2e0 = (longlong)plStack_130;
  local_2f8[0] = local_148;
  local_2f8[1] = puStack_140;
  core::iter::adapters::map::impl$2::
  fold<tuple_<u64,tuple_<>_>,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<ref_<country_core::systems::ui::signifiers::SignifierId>,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion>_>,country_core::systems::ui::signifiers::signifier_create::signifier_create::closure_env_0>,hashbrown::set::impl_10::extend::closure_env_0<u64,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<ref_<country_core::systems::ui::signifiers::SignifierId>,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion>_>,country_core::systems::ui::signifiers::signifier_create::signifier_create::closure_env_0>_>,tuple_<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env_0<tuple_<u64,tuple_<>_>,hashbrown::map::impl_82::extend::closure_env_0<u64,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<ref_<country_core::systems::ui::signifiers::SignifierId>,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion>_>,country_core::systems::ui::signifiers::signifier_create::signifier_create::closure_env_0>,hashbrown::set::impl_10::extend::closure_env_0<u64,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<ref_<country_core::systems::ui::signifiers::SignifierId>,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion>_>,country_core::systems::ui::signifiers::signifier_create::signifier_create::closure_env_0>_>_>_>_>_>
            ();
  local_148 = (undefined8 *)local_c8;
  puStack_140 = (undefined8 *)uStack_c0;
  local_138 = local_b8;
  plStack_130 = plStack_b0;
  local_128 = local_a8;
  puStack_120 = puStack_a0;
  *(undefined4 *)local_80[2] = *(undefined4 *)((longlong)local_80 + 0x1c);
  local_68 = (longlong *)*local_80;
  if ((char)local_68[3] != '\0') {
    *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
    _Src = (void *)*in_stack_00000030;
    memcpy(local_2f8,_Src,0x170);
    lVar22 = 0;
    memset(_Src,0,0x170);
    for (uVar19 = 0; uVar19 < local_198; uVar19 = uVar19 + 1) {
      append_only_vec::indices();
      if (0x2b < extraout_EAX) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      plVar18 = (longlong *)local_68[4];
      puVar20 = local_2f8[extraout_EAX];
      LOCK();
      bVar24 = plVar18[1] == 1;
      if (bVar24) {
        plVar18[1] = -1;
      }
      UNLOCK();
      if ((!bVar24) || (plVar18[1] = 1, *plVar18 != 1)) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      lVar23 = local_68[4];
      alloc::string::impl$6::clone();
      plVar18 = (longlong *)puVar20[lVar22 * 5 + 3];
      lVar6 = puVar20[lVar22 * 5 + 4];
      LOCK();
      lVar22 = *plVar18;
      *plVar18 = *plVar18 + 1;
      UNLOCK();
      if (*plVar18 == 0 || SCARRY8(lVar22,1) != *plVar18 < 0) {
                    // WARNING: Does not return
        pcVar9 = (code *)invalidInstructionException();
        (*pcVar9)();
      }
      local_b8 = local_178;
      local_c8 = local_188;
      uStack_c0 = uStack_180;
      lVar22 = *(longlong *)(lVar23 + 0x20);
      plStack_b0 = plVar18;
      local_a8 = lVar6;
      if (lVar22 == *(longlong *)(lVar23 + 0x10)) {
        alloc::raw_vec::RawVec::
        grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                  ();
      }
      lVar6 = *(longlong *)(lVar23 + 0x18);
      *(longlong *)(lVar6 + 0x20 + lVar22 * 0x28) = local_a8;
      puVar20 = (undefined8 *)(lVar6 + 0x10 + lVar22 * 0x28);
      *puVar20 = local_b8;
      puVar20[1] = plStack_b0;
      puVar20 = (undefined8 *)(lVar6 + lVar22 * 0x28);
      *puVar20 = local_c8;
      puVar20[1] = uStack_c0;
      *(longlong *)(lVar23 + 0x20) = lVar22 + 1;
      lVar22 = local_a8;
    }
    core::ptr::
    drop_in_place<append_only_vec::AppendOnlyVec<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>_>
              ();
    puVar20 = *(undefined8 **)(local_68[4] + 0x18);
    local_160 = puVar20 + *(longlong *)(local_68[4] + 0x20) * 5;
    local_158 = *local_78;
    local_150 = *local_90;
    iVar12 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                              CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                    CONCAT13(unaff_XMM6_Bd,
                             CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
    iVar13 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                              CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                    CONCAT13(unaff_XMM6_Bh,
                             CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
    iVar14 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                              CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                    CONCAT13(unaff_XMM6_Bl,
                             CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
    iVar15 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                              CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                    CONCAT13(unaff_XMM6_Bp,
                             CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
    for (; puVar20 != local_160; puVar20 = puVar20 + 5) {
      puVar8 = (undefined8 *)local_68[7];
      local_2f8[1] = (undefined8 *)puVar20[1];
      local_2f8[2] = (undefined8 *)puVar20[2];
      local_2f8[0] = puVar8;
      ::utils::
      calculate_hash<tuple_<country_core::systems::ui::resources::ui_state::WidgetId,enum2_<core::option::Option<ref_<str_>_>_>_>_>
                ();
      local_c8 = extraout_RAX_04;
      if (plStack_130 != (longlong *)0x0) {
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        bVar26 = (byte)(extraout_RAX_05 >> 0x39);
        auVar27 = ZEXT216(CONCAT11(bVar26,bVar26));
        auVar27 = pshuflw(auVar27,auVar27,0);
        lVar22 = 0;
        uVar19 = extraout_RAX_05;
        while( true ) {
          uVar19 = (ulonglong)puStack_140 & uVar19;
          pcVar2 = (char *)((longlong)local_148 + uVar19);
          cVar25 = auVar27[0];
          auVar32[0] = -(*pcVar2 == cVar25);
          cVar28 = auVar27[1];
          auVar32[1] = -(pcVar2[1] == cVar28);
          cVar29 = auVar27[2];
          auVar32[2] = -(pcVar2[2] == cVar29);
          cVar30 = auVar27[3];
          auVar32[3] = -(pcVar2[3] == cVar30);
          auVar32[4] = -(pcVar2[4] == cVar25);
          auVar32[5] = -(pcVar2[5] == cVar28);
          auVar32[6] = -(pcVar2[6] == cVar29);
          auVar32[7] = -(pcVar2[7] == cVar30);
          auVar32[8] = -(pcVar2[8] == cVar25);
          auVar32[9] = -(pcVar2[9] == cVar28);
          auVar32[10] = -(pcVar2[10] == cVar29);
          auVar32[0xb] = -(pcVar2[0xb] == cVar30);
          auVar32[0xc] = -(pcVar2[0xc] == cVar25);
          auVar32[0xd] = -(pcVar2[0xd] == cVar28);
          auVar32[0xe] = -(pcVar2[0xe] == cVar29);
          auVar32[0xf] = -(pcVar2[0xf] == cVar30);
          uVar21 = (ushort)(SUB161(auVar32 >> 7,0) & 1) |
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
                   (ushort)(auVar32[0xf] >> 7) << 0xf;
          uVar17 = (uint)uVar21;
          while (uVar21 != 0) {
            uVar10 = 0;
            for (uVar11 = uVar17; (uVar11 & 1) == 0; uVar11 = uVar11 >> 1 | 0x80000000) {
              uVar10 = uVar10 + 1;
            }
            if (extraout_RAX_04 ==
                *(undefined **)
                 ((longlong)local_148 + (uVar10 + uVar19 & (ulonglong)puStack_140) * -8 + -8))
            goto LAB_140b2d84d;
            uVar21 = (ushort)(uVar17 - 1) & (ushort)uVar17;
            uVar17 = CONCAT22((short)(uVar17 - 1 >> 0x10),uVar21);
          }
          auVar31[0] = -(*pcVar2 == (char)iVar12);
          auVar31[1] = -(pcVar2[1] == (char)((uint)iVar12 >> 8));
          auVar31[2] = -(pcVar2[2] == (char)((uint)iVar12 >> 0x10));
          auVar31[3] = -(pcVar2[3] == (char)((uint)iVar12 >> 0x18));
          auVar31[4] = -(pcVar2[4] == (char)iVar13);
          auVar31[5] = -(pcVar2[5] == (char)((uint)iVar13 >> 8));
          auVar31[6] = -(pcVar2[6] == (char)((uint)iVar13 >> 0x10));
          auVar31[7] = -(pcVar2[7] == (char)((uint)iVar13 >> 0x18));
          auVar31[8] = -(pcVar2[8] == (char)iVar14);
          auVar31[9] = -(pcVar2[9] == (char)((uint)iVar14 >> 8));
          auVar31[10] = -(pcVar2[10] == (char)((uint)iVar14 >> 0x10));
          auVar31[0xb] = -(pcVar2[0xb] == (char)((uint)iVar14 >> 0x18));
          auVar31[0xc] = -(pcVar2[0xc] == (char)iVar15);
          auVar31[0xd] = -(pcVar2[0xd] == (char)((uint)iVar15 >> 8));
          auVar31[0xe] = -(pcVar2[0xe] == (char)((uint)iVar15 >> 0x10));
          auVar31[0xf] = -(pcVar2[0xf] == (char)((uint)iVar15 >> 0x18));
          if ((((((((((((((((SUB161(auVar31 >> 7,0) & 1) != 0 || (SUB161(auVar31 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar31 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar31 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar31 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar31 >> 0x2f,0) & 1) != 0) || (SUB161(auVar31 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar31 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar31 >> 0x47,0) & 1) != 0) || (SUB161(auVar31 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar31 >> 0x57,0) & 1) != 0) || (SUB161(auVar31 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar31 >> 0x67,0) & 1) != 0) || (SUB161(auVar31 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar31 >> 0x77,0) & 1) != 0) || auVar31[0xf] < '\0') break;
          uVar19 = uVar19 + lVar22 + 0x10;
          lVar22 = lVar22 + 0x10;
        }
      }
      (**(code **)(puVar20[4] + 0x28))
                (local_2f8,
                 (*(longlong *)(puVar20[4] + 0x10) - 1U & 0xfffffffffffffff0) + puVar20[3] + 0x10,
                 local_158,local_150,local_70);
      local_78 = local_2f8[0];
      local_80 = local_2f8[1];
      local_90 = puVar20;
      if (local_2f8[2] != (undefined8 *)0x0) {
        lVar23 = (longlong)local_2f8[2] << 3;
        lVar22 = 0;
        do {
          uVar19 = *(ulonglong *)((longlong)local_80 + lVar22);
          if (uVar19 >> 0x20 == 0) break;
          plVar18 = *(longlong **)((longlong)local_70 + 8);
          if (plVar18 == (longlong *)0x0) {
            plVar18 = *(longlong **)((longlong)local_70 + 0x10);
          }
          lVar6 = plVar18[2];
          if ((ulonglong)(*plVar18 - lVar6) < 0x30) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          lVar7 = plVar18[1];
          *(code **)(lVar7 + lVar6) =
               core::ops::function::FnOnce::
               call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::systems::ui::signifiers::SignifierId>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          *(code **)(lVar7 + 8 + lVar6) = bevy_ecs::error::handler::panic;
          *(ulonglong *)(lVar7 + 0x10 + lVar6) = uVar19;
          *(undefined **)(lVar7 + 0x18 + lVar6) = extraout_RAX_04;
          *(undefined8 **)(lVar7 + 0x20 + lVar6) = puVar8;
          *(undefined1 *)(lVar7 + 0x28 + lVar6) = 0;
          plVar18[2] = lVar6 + 0x30;
          lVar22 = lVar22 + 8;
        } while (lVar23 != lVar22);
      }
      puVar20 = local_90;
      if (local_78 != (undefined8 *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
LAB_140b2d84d:
    }
  }
  if ((puStack_140 != (undefined8 *)0x0) &&
     ((longlong)puStack_140 + ((longlong)puStack_140 * 8 + 0x17U & 0xfffffffffffffff0) != -0x11)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_d8 != 0) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_06 == 1) {
      local_70 = extraout_RAX_06 + 1;
    }
    else if ((*extraout_RAX_06 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), local_70 = extraout_RAX_07, extraout_RAX_07 == (longlong *)0x0)) {
LAB_140b2d922:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*local_70 != 0) {
LAB_140b2d92e:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *local_70 = -1;
    puffin::ThreadProfiler::end_scope();
    *local_70 = *local_70 + 1;
  }
  return;
}


// CALLEES: [{"address": "140c18130", "name": "VAL"}, {"address": "140d4d740", "name": "VAL"}, {"address": "140613c20", "name": "__rust_dealloc"}, {"address": "140c16a00", "name": "are_scopes_on"}, {"address": "140c16d40", "name": "begin_scope"}, {"address": "14145a710", "name": "calculate_hash<tuple$<country_core::systems::ui::resources::ui_state::WidgetId,enum2$<core::option::Option<ref$<str$>_>_>_>_>"}, {"address": "140d7bd90", "name": "clone"}, {"address": "1428a9a60", "name": "do_reserve_and_handle<alloc::alloc::Global>"}, {"address": "14087d740", "name": "drop_in_place<append_only_vec::AppendOnlyVec<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>_>"}, {"address": "140c170f0", "name": "end_scope"}, {"address": "141475530", "name": "fold<tuple$<u64,tuple$<>_>,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<ref$<country_core::systems::ui::signifiers::SignifierId>,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion>_>,country_core::systems::ui::signifiers::signifier_create::signifier_create::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<u64,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<ref$<country_core::systems::ui::signifiers::SignifierId>,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion>_>,country_core::systems::ui::signifiers::signifier_create::signifier_create::closure_env$0>_>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<u64,tuple$<>_>,hashbrown::map::impl$82::extend::closure_env$0<u64,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<ref$<country_core::systems::ui::signifiers::SignifierId>,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion>_>,country_core::systems::ui::signifiers::signifier_create::signifier_create::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<u64,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<ref$<country_core::systems::ui::signifiers::SignifierId>,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion>_>,country_core::systems::ui::signifiers::signifier_create::signifier_create::closure_env$0>_>_>_>_>_>"}, {"address": "140562ab0", "name": "grow_one<indexmap::Bucket<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2$<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>"}, {"address": "1405b0d70", "name": "hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId>_>"}, {"address": "140d64e00", "name": "hashmap_random_keys"}, {"address": "140c111b0", "name": "indices"}, {"address": "1428f87d0", "name": "initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>"}, {"address": "142923c00", "name": "memcpy"}, {"address": "142923c20", "name": "memset"}, {"address": "1428d80d0", "name": "panic_access_error"}, {"address": "1428d9250", "name": "panic_already_borrowed"}, {"address": "1428d9518", "name": "panic_bounds_check"}, {"address": "1428aa7a0", "name": "reserve_rehash<tuple$<country_core::resources::history::HistorySnapshotId,tuple$<>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::history::HistorySnapshotId,tuple$<>,std::hash::random::RandomState>_>"}, {"address": "140b2ae70", "name": "rfind<ref$<str$>_>"}, {"address": "141795a10", "name": "size_hint<ref$<country_core::components::TreeAmbience>,tuple$<>_>"}, {"address": "1428d9940", "name": "slice_error_fail"}, {"address": "1428d9310", "name": "unwrap_failed"}]