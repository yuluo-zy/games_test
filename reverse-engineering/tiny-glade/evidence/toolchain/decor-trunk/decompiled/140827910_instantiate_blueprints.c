// ADDR 0x140827910
// SIG void __cdecl instantiate_blueprints(void)
// ORIG system::instantiate_blueprints


void __cdecl system_decorator::blueprint::instantiate_blueprints(void)

{
  byte *pbVar1;
  char *pcVar2;
  char *pcVar3;
  float *pfVar4;
  float *pfVar5;
  float fVar6;
  undefined8 uVar7;
  byte bVar8;
  byte bVar9;
  int iVar10;
  undefined4 uVar11;
  undefined8 uVar12;
  uint uVar13;
  uint uVar14;
  undefined8 uVar15;
  undefined1 auVar16 [16];
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  char extraout_AL_03;
  char extraout_AL_04;
  char extraout_AL_05;
  uint uVar17;
  uint uVar18;
  int iVar19;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  byte *extraout_RAX_02;
  ulonglong extraout_RAX_03;
  ulonglong extraout_RAX_04;
  undefined1 (*pauVar20) [16];
  ulonglong extraout_RAX_05;
  longlong lVar21;
  ulonglong extraout_RAX_06;
  longlong lVar22;
  ulonglong extraout_RAX_07;
  longlong *extraout_RAX_08;
  longlong *extraout_RAX_09;
  longlong *extraout_RAX_10;
  longlong *in_RCX;
  longlong *plVar23;
  undefined1 (*in_RDX) [16];
  ulonglong uVar24;
  undefined8 *puVar25;
  undefined1 (*pauVar26) [16];
  longlong lVar27;
  undefined1 (*pauVar28) [16];
  undefined1 (*pauVar29) [16];
  undefined8 *in_R8;
  undefined *puVar30;
  longlong lVar31;
  ulonglong uVar32;
  longlong in_R9;
  longlong lVar33;
  undefined1 uVar34;
  ushort uVar35;
  ulonglong uVar36;
  ulonglong uVar37;
  undefined4 uVar38;
  undefined8 *puVar39;
  undefined1 (*pauVar40) [16];
  byte bVar41;
  char cVar42;
  char cVar46;
  char cVar47;
  char cVar48;
  undefined1 auVar43 [16];
  undefined1 auVar44 [16];
  undefined1 auVar45 [16];
  undefined1 auVar49 [16];
  undefined1 auVar50 [16];
  undefined1 auVar51 [16];
  undefined1 auVar52 [16];
  undefined1 auVar53 [16];
  undefined1 auVar54 [16];
  longlong in_stack_00000028;
  undefined8 *in_stack_00000030;
  longlong *in_stack_00000038;
  longlong *in_stack_00000040;
  undefined8 *in_stack_00000048;
  undefined8 *in_stack_00000050;
  undefined8 *in_stack_00000058;
  undefined8 *in_stack_00000060;
  undefined8 *in_stack_00000068;
  undefined8 *in_stack_00000070;
  undefined1 local_360 [352];
  undefined4 *local_200;
  undefined8 local_1f8;
  undefined4 *local_1f0;
  undefined1 (*local_1e8) [16];
  undefined1 (*pauStack_1e0) [16];
  undefined1 local_1d8 [16];
  undefined1 local_1c8 [16];
  longlong local_1b8;
  ulonglong local_1b0;
  undefined1 (*local_1a8) [16];
  longlong *local_1a0;
  undefined4 local_198;
  undefined4 local_194;
  undefined1 (*local_190) [16];
  longlong local_188;
  undefined8 *local_180;
  undefined1 (**local_178) [16];
  undefined4 *local_170;
  undefined4 *local_168;
  undefined1 (*local_158) [16];
  undefined1 (*pauStack_150) [16];
  undefined1 (*local_148) [16];
  ulonglong uStack_140;
  undefined8 *local_138;
  undefined8 uStack_130;
  longlong *local_128;
  longlong local_120;
  longlong local_118;
  longlong local_110;
  undefined1 (*local_108) [16];
  undefined1 (*local_100) [16];
  undefined1 (*local_f8) [16];
  longlong *local_f0;
  undefined1 (*local_e8) [16];
  undefined1 (*pauStack_e0) [16];
  undefined1 (*local_d8) [16];
  undefined1 (*pauStack_d0) [16];
  ulonglong local_c8;
  undefined8 *puStack_c0;
  undefined8 uStack_b8;
  undefined4 local_ac;
  undefined4 local_a8;
  undefined4 local_a4;
  longlong *local_a0;
  longlong *local_98;
  undefined1 (*local_90) [16];
  undefined1 (*local_88) [16];
  int local_80;
  char local_79;
  undefined1 (*local_78) [16];
  undefined8 local_70;
  
  local_70 = 0xfffffffffffffffe;
  local_180 = in_R8;
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    uVar24 = 0x33;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar24 != 0) {
        if (uVar24 < 0x33) {
          if ((char)(&DAT_142a91427)[uVar24] < -0x40) {
LAB_1408279a4:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar24 != 0x33) goto LAB_1408279a4;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar24 != 0xfffffffffffffffe)) {
        if (uVar24 + 2 < 0x33) {
          if ((char)(&UNK_142a91429)[uVar24] < -0x40) {
LAB_1408279fb:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar24 + 2 != 0x33) goto LAB_1408279fb;
      }
    }
    puVar30 = &DAT_142a91679;
    do {
      bVar41 = puVar30[-1];
      uVar17 = (uint)(char)bVar41;
      if ((char)bVar41 < '\0') {
        bVar8 = puVar30[-2];
        if ((char)bVar8 < -0x40) {
          bVar9 = puVar30[-3];
          if ((char)bVar9 < -0x40) {
            pbVar1 = puVar30 + -4;
            puVar30 = puVar30 + -4;
            uVar17 = bVar9 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar30 = puVar30 + -3;
            uVar17 = bVar9 & 0xf;
          }
          uVar17 = bVar8 & 0x3f | uVar17 << 6;
        }
        else {
          puVar30 = puVar30 + -2;
          uVar17 = bVar8 & 0x1f;
        }
        uVar17 = (uint)(bVar41 & 0x3f) | uVar17 << 6;
        if (uVar17 != 0x5c) goto LAB_140827ac4;
LAB_140827ad5:
        if (puVar30 == &UNK_142a9164f) break;
        if (puVar30 + -0x142a9164f < (undefined *)0x29) {
          if (-0x41 < (char)puVar30[1]) break;
        }
        else if (puVar30 + -0x142a9164f == (undefined *)0x29) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar30 = puVar30 + -1;
      if (uVar17 == 0x5c) goto LAB_140827ad5;
LAB_140827ac4:
      if (uVar17 == 0x2f) goto LAB_140827ad5;
    } while (puVar30 != &anon_607a6278b2141009247d0776e3b766a7_27_llvm_14222842113206979732);
    local_78 = in_RDX;
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar23 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar23 = extraout_RAX_09, extraout_RAX_09 == (longlong *)0x0)) {
LAB_1408292c5:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar23 != 0) {
LAB_1408292ef:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar23 = -1;
    local_98 = plVar23;
    puffin::ThreadProfiler::begin_scope();
    *local_98 = *local_98 + 1;
    in_RDX = local_78;
  }
  uVar38 = *(undefined4 *)((longlong)in_RCX + 0x1c);
  local_98 = (longlong *)in_RCX[2];
  *(undefined4 *)local_98 = uVar38;
  local_f0 = (longlong *)*in_RCX;
  local_f0[2] = 0;
  **(undefined4 **)(in_stack_00000028 + 0x10) = *(undefined4 *)(in_stack_00000028 + 0x1c);
  puVar39 = (undefined8 *)*in_stack_00000048;
  local_90 = (undefined1 (*) [16])*in_stack_00000050;
  **(undefined4 **)(in_R9 + 0x10) = *(undefined4 *)(in_R9 + 0x1c);
  puVar25 = puVar39;
  local_79 = extraout_AL;
  instantiate_decorator_colliders();
  pauVar40 = *(undefined1 (**) [16])*local_90;
  lVar27 = *(longlong *)(*local_90 + 8);
  auVar43 = *pauVar40;
  pauVar29 = *(undefined1 (**) [16])(local_90[1] + 8);
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
    *(ulonglong *)(extraout_RAX_02 + 8) = extraout_RAX_03;
    *(undefined8 **)(extraout_RAX_02 + 0x10) = puVar25;
    local_c8 = extraout_RAX_03;
  }
  else {
    local_c8 = *(ulonglong *)(extraout_RAX_02 + 8);
    puVar25 = *(undefined8 **)(extraout_RAX_02 + 0x10);
  }
  *(ulonglong *)(extraout_RAX_02 + 8) = local_c8 + 1;
  local_d8 = (undefined1 (*) [16])0x0;
  pauStack_d0 = (undefined1 (*) [16])0x0;
  local_e8 = (undefined1 (*) [16])&DAT_142a912a0;
  pauStack_e0 = (undefined1 (*) [16])0x0;
  puStack_c0 = puVar25;
  if (pauVar29 != (undefined1 (*) [16])0x0) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::decorator::DecoratorId,tuple_<>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::decorator::DecoratorId,tuple_<>,std::hash::random::RandomState>_>
              ();
  }
  local_148 = (undefined1 (*) [16])(*pauVar40 + lVar27 + 1);
  local_178 = &local_e8;
  local_1e8 = (undefined1 (*) [16])&local_178;
  pauStack_150 = pauVar40 + 1;
  uStack_140 = CONCAT62(uStack_140._2_6_,
                        ~((ushort)(SUB161(auVar43 >> 7,0) & 1) |
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
                         (ushort)(byte)(auVar43[0xf] >> 7) << 0xf));
  local_158 = pauVar40;
  _ZN9hashbrown3raw21RawIterRange_LT_T_GT_9fold_impl17he118415d8849be87E();
  local_1d8._8_8_ = pauStack_d0;
  local_1d8._0_8_ = local_d8;
  local_1c8._8_8_ = puStack_c0;
  local_1c8._0_8_ = local_c8;
  local_1e8 = local_e8;
  pauStack_1e0 = pauStack_e0;
  pauVar26 = (undefined1 (*) [16])*puVar39;
  auVar43 = *pauVar26;
  local_1b0 = (ulonglong)
              ~(uint)(ushort)((ushort)(SUB161(auVar43 >> 7,0) & 1) |
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
                             (ushort)(byte)(auVar43[0xf] >> 7) << 0xf);
  local_194 = *(undefined4 *)(in_RDX[1] + 0xc);
  local_1f8 = *(undefined8 *)*in_RDX;
  local_1f0 = *(undefined4 **)in_RDX[1];
  local_198 = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
  local_a0 = (longlong *)*in_stack_00000030;
  local_200 = (undefined4 *)in_stack_00000030[2];
  local_110 = *in_stack_00000058;
  local_f8 = (undefined1 (*) [16])*in_stack_00000060;
  local_108 = (undefined1 (*) [16])*in_stack_00000068;
  local_a8 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
  local_120 = *in_stack_00000038;
  local_170 = (undefined4 *)in_stack_00000038[2];
  local_a4 = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
  local_118 = *in_stack_00000040;
  local_168 = (undefined4 *)in_stack_00000040[2];
  pauVar40 = pauVar26 + 1;
  local_190 = local_90 + 2;
  local_128 = local_f0 + 3;
  local_1a0 = local_f0 + 7;
  lVar33 = puVar39[3];
  puVar39 = local_180;
  lVar27 = local_188;
  local_ac = uVar38;
LAB_140827e50:
  do {
    local_188 = lVar27;
    if (lVar33 == 0) {
      auVar43 = *local_1e8;
      local_108 = pauStack_1e0;
      if (pauStack_1e0 == (undefined1 (*) [16])0x0) {
        local_a0 = (longlong *)0x0;
      }
      else {
        uVar24 = (longlong)pauStack_1e0 * 4 + 0x13U & 0xfffffffffffffff0;
        local_88 = (undefined1 (*) [16])(pauStack_1e0[1] + uVar24 + 1);
        local_f0 = (longlong *)((longlong)local_1e8 - uVar24);
        local_a0 = (longlong *)0x10;
      }
      local_110 = local_1d8._8_8_;
      pauVar40 = local_1e8 + 1;
      uVar17 = ~(uint)(ushort)((ushort)(SUB161(auVar43 >> 7,0) & 1) |
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
                              (ushort)(byte)(auVar43[0xf] >> 7) << 0xf);
      local_1b8 = 0;
      pauVar26 = local_1e8;
      pauVar29 = local_90;
LAB_140828c7b:
      if (local_110 == 0) {
        if ((local_108 != (undefined1 (*) [16])0x0) && (local_88 != (undefined1 (*) [16])0x0)) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        local_78 = (undefined1 (*) [16])*in_stack_00000070;
        lVar27 = in_stack_00000070[1];
        uVar24 = *(ulonglong *)*local_78;
        uVar37 = 0;
        uVar36 = uVar24 - *(ulonglong *)(lVar27 + 0x18);
        if (uVar24 < *(ulonglong *)(lVar27 + 0x18)) {
          uVar36 = uVar37;
        }
        uVar32 = uVar24 - *(ulonglong *)(lVar27 + 0x38);
        if (uVar24 < *(ulonglong *)(lVar27 + 0x38)) {
          uVar32 = uVar37;
        }
        pauVar40 = (undefined1 (*) [16])(uVar36 * 0x10 + *(longlong *)(lVar27 + 8));
        uVar24 = *(ulonglong *)(lVar27 + 0x10) - uVar36;
        if (*(ulonglong *)(lVar27 + 0x10) < uVar36) {
          pauVar40 = (undefined1 (*) [16])&DAT_00000008;
          uVar24 = uVar37;
        }
        pauVar29 = (undefined1 (*) [16])(uVar32 * 0x10 + *(longlong *)(lVar27 + 0x28));
        uVar36 = *(ulonglong *)(lVar27 + 0x30) - uVar32;
        if (*(ulonglong *)(lVar27 + 0x30) < uVar32) {
          pauVar29 = (undefined1 (*) [16])&DAT_00000008;
          uVar36 = uVar37;
        }
        *(ulonglong *)*local_78 = (*(longlong *)(lVar27 + 0x40) - uVar36) - uVar24;
        local_88 = pauVar40 + uVar24;
        local_90 = pauVar29 + uVar36;
        local_98 = (longlong *)(local_120 + 0x20);
        local_128 = (longlong *)(local_120 + 0x50);
        local_110 = local_120 + 0x30;
        local_f8 = (undefined1 (*) [16])(local_118 + 0x20);
        local_108 = (undefined1 (*) [16])(local_118 + 0x50);
        if (pauVar40 == (undefined1 (*) [16])0x0) goto LAB_140829070;
        do {
          if (pauVar40 == local_88) goto LAB_140829070;
          pauVar26 = pauVar40 + 1;
          pauVar20 = pauVar29;
          while( true ) {
            pauVar29 = pauVar20;
            *(longlong *)*local_78 = *(longlong *)*local_78 + 1;
            *local_170 = local_a8;
            local_178 = *(undefined1 (***) [16])(*pauVar40 + 8);
            core::hash::BuildHasher::
            hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
            hashbrown::raw::RawTable::
            remove_entry<tuple_<country_core::resources::walls::WallId,std::collections::hash::set::HashSet<usize,std::hash::random::RandomState>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,std::collections::hash::set::HashSet<usize,std::hash::random::RandomState>_>_>
                      ();
            if (pauStack_e0 != (undefined1 (*) [16])0x0) {
              local_158 = pauStack_e0;
              pauStack_150 = local_d8;
              local_138 = puStack_c0;
              uStack_130 = uStack_b8;
              local_148 = pauStack_d0;
              uStack_140 = local_c8;
              if (pauStack_e0 != (undefined1 (*) [16])0x0) {
                hashbrown::raw::RawTableInner::
                drop_inner_table<tuple_<country_core::resources::walls::decorator::DecoratorId,enum2_<system_decorator::decorator_visual::deco_differential_state::DecoDiff<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams>_>_>,alloc::alloc::Global>
                          ();
              }
            }
            core::hash::BuildHasher::
            hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
            hashbrown::raw::RawTable::
            remove_entry<tuple_<country_core::resources::walls::WallId,tuple_<>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,tuple_<>_>_>
                      ();
            *local_168 = local_a4;
            local_178 = *(undefined1 (***) [16])(*pauVar40 + 8);
            core::hash::BuildHasher::
            hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
            hashbrown::raw::RawTable::
            remove_entry<tuple_<country_core::resources::walls::WallId,std::collections::hash::set::HashSet<usize,std::hash::random::RandomState>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,std::collections::hash::set::HashSet<usize,std::hash::random::RandomState>_>_>
                      ();
            if (pauStack_e0 != (undefined1 (*) [16])0x0) {
              local_158 = pauStack_e0;
              pauStack_150 = local_d8;
              local_138 = puStack_c0;
              uStack_130 = uStack_b8;
              local_148 = pauStack_d0;
              uStack_140 = local_c8;
              if (pauStack_e0 != (undefined1 (*) [16])0x0) {
                hashbrown::raw::RawTableInner::
                drop_inner_table<tuple_<country_core::resources::walls::decorator::DecoratorId,enum2_<system_decorator::decorator_visual::deco_differential_state::DecoDiff<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams>_>_>,alloc::alloc::Global>
                          ();
              }
            }
            core::hash::BuildHasher::
            hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
            hashbrown::raw::RawTable::
            remove_entry<tuple_<country_core::resources::walls::WallId,tuple_<>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,tuple_<>_>_>
                      ();
            pauVar40 = pauVar26;
            if (pauVar26 != (undefined1 (*) [16])0x0) break;
LAB_140829070:
            if ((pauVar29 == (undefined1 (*) [16])0x0) || (pauVar29 == local_90)) {
              if (local_79 != '\0') {
                puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
                if (*extraout_RAX_08 == 1) {
                  plVar23 = extraout_RAX_08 + 1;
                }
                else if ((*extraout_RAX_08 != 0) ||
                        (std::sys::thread_local::native::lazy::Storage::
                         initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                                   (), plVar23 = extraout_RAX_10, extraout_RAX_10 == (longlong *)0x0
                        )) goto LAB_1408292c5;
                if (*plVar23 != 0) goto LAB_1408292ef;
                *plVar23 = -1;
                local_78 = (undefined1 (*) [16])plVar23;
                puffin::ThreadProfiler::end_scope();
                *(longlong *)local_78 = *(longlong *)local_78 + 1;
              }
              return;
            }
            pauVar26 = (undefined1 (*) [16])0x0;
            pauVar20 = pauVar29 + 1;
            pauVar40 = pauVar29;
          }
        } while( true );
      }
      if ((short)uVar17 == 0) {
        do {
          auVar43 = *pauVar40;
          uVar35 = (ushort)(SUB161(auVar43 >> 7,0) & 1) |
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
                   (ushort)(byte)(auVar43[0xf] >> 7) << 0xf;
          pauVar26 = pauVar26 + -4;
          pauVar40 = pauVar40 + 1;
        } while (uVar35 == 0xffff);
        uVar18 = ~(uint)uVar35;
        uVar17 = uVar18 & -(uint)uVar35 - 2;
      }
      else {
        uVar18 = uVar17;
        uVar17 = uVar17 - 1 & uVar17;
      }
      iVar19 = 0;
      for (; (uVar18 & 1) == 0; uVar18 = uVar18 >> 1 | 0x80000000) {
        iVar19 = iVar19 + 1;
      }
      iVar19 = *(int *)((longlong)pauVar26 + (-4 - (ulonglong)(uint)(iVar19 << 2)));
      *(undefined4 *)local_98 = local_ac;
      local_158 = (undefined1 (*) [16])CONCAT44(local_158._4_4_,iVar19);
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
      hashbrown::raw::RawTable::
      remove_entry<tuple_<country_core::resources::walls::decorator::DecoratorId,enum2_<system_decorator::decorator_visual_state::PersistentVisualEntities>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,enum2_<system_decorator::decorator_visual_state::PersistentVisualEntities>_>_>
                ();
      pauVar20 = pauVar29;
      if ((int)pauStack_e0 != 2) {
        local_f8 = local_d8;
        local_78 = pauStack_d0;
        if (local_c8 != 0) {
          lVar33 = local_c8 << 3;
          lVar27 = 0;
          do {
            if (*(ulonglong *)(*local_78 + lVar27) >> 0x20 == 0) break;
            local_e8 = (undefined1 (*) [16])*local_180;
            pauStack_e0 = (undefined1 (*) [16])local_180[1];
            local_d8 = (undefined1 (*) [16])local_180[2];
            pauStack_d0 = (undefined1 (*) [16])local_180[3];
            local_c8 = *(ulonglong *)(*local_78 + lVar27);
            bevy_ecs::system::commands::EntityCommands::despawn();
            lVar27 = lVar27 + 8;
          } while (lVar33 != lVar27);
        }
        pauVar20 = local_90;
        if (local_f8 != (undefined1 (*) [16])0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
      }
      local_e8 = (undefined1 (*) [16])CONCAT44(local_e8._4_4_,iVar19);
      if (*(longlong *)(pauVar20[1] + 8) != 0) {
        local_110 = local_110 + -1;
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
        pauVar29 = local_90;
        lVar27 = *(longlong *)*pauVar20;
        bVar41 = (byte)(extraout_RAX_07 >> 0x39);
        auVar43 = ZEXT216(CONCAT11(bVar41,bVar41));
        auVar43 = pshuflw(auVar43,auVar43,0);
        lVar33 = 0;
        uVar24 = extraout_RAX_07;
        while( true ) {
          uVar24 = *(ulonglong *)(*pauVar20 + 8) & uVar24;
          pcVar2 = (char *)(lVar27 + uVar24);
          cVar42 = auVar43[0];
          auVar54[0] = -(*pcVar2 == cVar42);
          cVar46 = auVar43[1];
          auVar54[1] = -(pcVar2[1] == cVar46);
          cVar47 = auVar43[2];
          auVar54[2] = -(pcVar2[2] == cVar47);
          cVar48 = auVar43[3];
          auVar54[3] = -(pcVar2[3] == cVar48);
          auVar54[4] = -(pcVar2[4] == cVar42);
          auVar54[5] = -(pcVar2[5] == cVar46);
          auVar54[6] = -(pcVar2[6] == cVar47);
          auVar54[7] = -(pcVar2[7] == cVar48);
          auVar54[8] = -(pcVar2[8] == cVar42);
          auVar54[9] = -(pcVar2[9] == cVar46);
          auVar54[10] = -(pcVar2[10] == cVar47);
          auVar54[0xb] = -(pcVar2[0xb] == cVar48);
          auVar54[0xc] = -(pcVar2[0xc] == cVar42);
          auVar54[0xd] = -(pcVar2[0xd] == cVar46);
          auVar54[0xe] = -(pcVar2[0xe] == cVar47);
          auVar54[0xf] = -(pcVar2[0xf] == cVar48);
          uVar35 = (ushort)(SUB161(auVar54 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar54 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar54 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar54 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar54 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar54 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar54 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar54 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar54 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar54 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar54 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar54 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar54 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar54 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar54 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar54[0xf] >> 7) << 0xf;
          uVar18 = (uint)uVar35;
          while (uVar35 != 0) {
            uVar13 = 0;
            for (uVar14 = uVar18; (uVar14 & 1) == 0; uVar14 = uVar14 >> 1 | 0x80000000) {
              uVar13 = uVar13 + 1;
            }
            uVar36 = uVar13 + uVar24 & *(ulonglong *)(*pauVar20 + 8);
            if (iVar19 == *(int *)(lVar27 + -0x10 + uVar36 * -0x10)) {
              lVar27 = *(longlong *)(lVar27 + -8 + uVar36 * -0x10);
              if (*(longlong *)(lVar27 + 0x128) != 0) {
                *local_170 = local_a8;
                decorator_visual::deco_differential_state::WallAttachedDecoTracker::
                remove<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams>
                          ();
              }
              if (*(longlong *)(lVar27 + 0x130) != 0) {
                *local_168 = local_a4;
                decorator_visual::deco_differential_state::WallAttachedDecoTracker::
                remove<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams>
                          ();
              }
              goto LAB_140828c7b;
            }
            uVar35 = (ushort)(uVar18 - 1) & (ushort)uVar18;
            uVar18 = CONCAT22((short)(uVar18 - 1 >> 0x10),uVar35);
          }
          auVar51[0] = -(*pcVar2 == -1);
          auVar51[1] = -(pcVar2[1] == -1);
          auVar51[2] = -(pcVar2[2] == -1);
          auVar51[3] = -(pcVar2[3] == -1);
          auVar51[4] = -(pcVar2[4] == -1);
          auVar51[5] = -(pcVar2[5] == -1);
          auVar51[6] = -(pcVar2[6] == -1);
          auVar51[7] = -(pcVar2[7] == -1);
          auVar51[8] = -(pcVar2[8] == -1);
          auVar51[9] = -(pcVar2[9] == -1);
          auVar51[10] = -(pcVar2[10] == -1);
          auVar51[0xb] = -(pcVar2[0xb] == -1);
          auVar51[0xc] = -(pcVar2[0xc] == -1);
          auVar51[0xd] = -(pcVar2[0xd] == -1);
          auVar51[0xe] = -(pcVar2[0xe] == -1);
          auVar51[0xf] = -(pcVar2[0xf] == -1);
          if ((((((((((((((((SUB161(auVar51 >> 7,0) & 1) != 0 || (SUB161(auVar51 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar51 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar51 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar51 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar51 >> 0x2f,0) & 1) != 0) || (SUB161(auVar51 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar51 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar51 >> 0x47,0) & 1) != 0) || (SUB161(auVar51 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar51 >> 0x57,0) & 1) != 0) || (SUB161(auVar51 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar51 >> 0x67,0) & 1) != 0) || (SUB161(auVar51 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar51 >> 0x77,0) & 1) != 0) || auVar51[0xf] < '\0') break;
          uVar24 = uVar24 + lVar33 + 0x10;
          lVar33 = lVar33 + 0x10;
        }
      }
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    uVar17 = (uint)local_1b0;
    if ((short)local_1b0 == 0) {
      do {
        auVar43 = *pauVar40;
        uVar35 = (ushort)(SUB161(auVar43 >> 7,0) & 1) | (ushort)(SUB161(auVar43 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar43 >> 0x17,0) & 1) << 2 |
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
                 (ushort)(byte)(auVar43[0xf] >> 7) << 0xf;
        pauVar26 = pauVar26 + -0x10;
        pauVar40 = pauVar40 + 1;
      } while (uVar35 == 0xffff);
      uVar17 = ~(uint)uVar35;
      uVar18 = uVar17 & -(uint)uVar35 - 2;
    }
    else {
      uVar18 = uVar17 - 1 & uVar17;
    }
    local_1b0 = (ulonglong)uVar18;
    iVar19 = 0;
    for (; (uVar17 & 1) == 0; uVar17 = uVar17 >> 1 | 0x80000000) {
      iVar19 = iVar19 + 1;
    }
    iVar10 = *(int *)((longlong)pauVar26 + (-0x10 - (ulonglong)(uint)(iVar19 << 4)));
    local_78 = *(undefined1 (**) [16])((longlong)pauVar26 + (-8 - (ulonglong)(uint)(iVar19 << 4)));
    local_e8 = (undefined1 (*) [16])CONCAT44(local_e8._4_4_,iVar10);
    pauVar20 = local_90;
    local_1b8 = lVar33;
    if (*(longlong *)(local_90[1] + 8) != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
      lVar27 = *(longlong *)*local_90;
      bVar41 = (byte)(extraout_RAX_04 >> 0x39);
      auVar43 = ZEXT216(CONCAT11(bVar41,bVar41));
      auVar43 = pshuflw(auVar43,auVar43,0);
      pauVar20 = (undefined1 (*) [16])(lVar27 + -0x10);
      lVar33 = 0;
      uVar24 = extraout_RAX_04;
      while( true ) {
        uVar24 = *(ulonglong *)(*local_90 + 8) & uVar24;
        auVar54 = *(undefined1 (*) [16])(lVar27 + uVar24);
        cVar42 = auVar43[0];
        auVar52[0] = -(auVar54[0] == cVar42);
        cVar46 = auVar43[1];
        auVar52[1] = -(auVar54[1] == cVar46);
        cVar47 = auVar43[2];
        auVar52[2] = -(auVar54[2] == cVar47);
        cVar48 = auVar43[3];
        auVar52[3] = -(auVar54[3] == cVar48);
        auVar52[4] = -(auVar54[4] == cVar42);
        auVar52[5] = -(auVar54[5] == cVar46);
        auVar52[6] = -(auVar54[6] == cVar47);
        auVar52[7] = -(auVar54[7] == cVar48);
        auVar52[8] = -(auVar54[8] == cVar42);
        auVar52[9] = -(auVar54[9] == cVar46);
        auVar52[10] = -(auVar54[10] == cVar47);
        auVar52[0xb] = -(auVar54[0xb] == cVar48);
        auVar52[0xc] = -(auVar54[0xc] == cVar42);
        auVar52[0xd] = -(auVar54[0xd] == cVar46);
        auVar52[0xe] = -(auVar54[0xe] == cVar47);
        auVar52[0xf] = -(auVar54[0xf] == cVar48);
        uVar35 = (ushort)(SUB161(auVar52 >> 7,0) & 1) | (ushort)(SUB161(auVar52 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar52 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar52 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar52 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar52 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar52 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar52 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar52 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar52 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar52 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar52 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar52 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar52 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar52 >> 0x77,0) & 1) << 0xe | (ushort)(auVar52[0xf] >> 7) << 0xf
        ;
        uVar17 = (uint)uVar35;
        while (uVar35 != 0) {
          uVar18 = 0;
          for (uVar13 = uVar17; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
            uVar18 = uVar18 + 1;
          }
          uVar36 = uVar18 + uVar24 & *(ulonglong *)(*local_90 + 8);
          pauVar29 = pauVar20 + -uVar36;
          if (iVar10 == *(int *)*pauVar29) {
            pauVar20 = local_78;
            if (*(undefined1 (**) [16])(lVar27 + -8 + uVar36 * -0x10) != local_78)
            goto LAB_140827fa9;
            local_88 = (undefined1 (*) [16])((ulonglong)local_88 & 0xffffffff00000000);
            goto LAB_140827fb1;
          }
          uVar35 = (ushort)(uVar17 - 1) & (ushort)uVar17;
          uVar17 = CONCAT22((short)(uVar17 - 1 >> 0x10),uVar35);
        }
        auVar49[0] = -(auVar54[0] == -1);
        auVar49[1] = -(auVar54[1] == -1);
        auVar49[2] = -(auVar54[2] == -1);
        auVar49[3] = -(auVar54[3] == -1);
        auVar49[4] = -(auVar54[4] == -1);
        auVar49[5] = -(auVar54[5] == -1);
        auVar49[6] = -(auVar54[6] == -1);
        auVar49[7] = -(auVar54[7] == -1);
        auVar49[8] = -(auVar54[8] == -1);
        auVar49[9] = -(auVar54[9] == -1);
        auVar49[10] = -(auVar54[10] == -1);
        auVar49[0xb] = -(auVar54[0xb] == -1);
        auVar49[0xc] = -(auVar54[0xc] == -1);
        auVar49[0xd] = -(auVar54[0xd] == -1);
        auVar49[0xe] = -(auVar54[0xe] == -1);
        auVar49[0xf] = -(auVar54[0xf] == -1);
        if ((((((((((((((((SUB161(auVar49 >> 7,0) & 1) != 0 || (SUB161(auVar49 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar49 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar49 >> 0x1f,0) & 1) != 0) || (SUB161(auVar49 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar49 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar49 >> 0x37,0) & 1) != 0) || (SUB161(auVar49 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar49 >> 0x47,0) & 1) != 0) || (SUB161(auVar49 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar49 >> 0x57,0) & 1) != 0) || (SUB161(auVar49 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar49 >> 0x67,0) & 1) != 0) || (SUB161(auVar49 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar49 >> 0x77,0) & 1) != 0) || auVar49[0xf] < '\0') break;
        uVar24 = uVar24 + lVar33 + 0x10;
        lVar33 = lVar33 + 0x10;
      }
    }
LAB_140827fa9:
    local_88 = (undefined1 (*) [16])
               CONCAT44(local_88._4_4_,(int)CONCAT71((int7)((ulonglong)pauVar20 >> 8),1));
LAB_140827fb1:
    local_80 = iVar10;
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
    hashbrown::raw::RawTable::
    remove_entry<tuple_<country_core::resources::walls::decorator::DecoratorId,tuple_<>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,tuple_<>_>_>
              ();
    if (*(longlong *)(local_78[0xe] + 8) != 0) {
      country_core::resources::walls::decorator_storage::DecoratorStorageAddr::try_as_wall();
      if ((extraout_RAX_05 & 1) == 0) {
        if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
          local_e8 = (undefined1 (*) [16])&PTR_DAT_142a91930;
          pauStack_e0 = (undefined1 (*) [16])0x1;
          local_d8 = (undefined1 (*) [16])&DAT_00000008;
          pauStack_d0 = (undefined1 (*) [16])0x0;
          local_c8 = 0;
          toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
          local_158 = (undefined1 (*) [16])s_system_decorator__blueprint_142a91890;
          pauStack_150 = (undefined1 (*) [16])&DAT_0000001b;
          local_148 = (undefined1 (*) [16])s_system_decorator__blueprint_142a91890;
          uStack_140 = 0x1b;
          ::log::__private_api::log<tuple_<>,log::__private_api::GlobalLogger>();
        }
                    // WARNING: Subroutine does not return
        instantiate_blueprints::closure$0::panic_cold_explicit();
      }
      *local_1f0 = local_194;
      country_core::resources::walls::wall_holes::WallHoles::
      extend<core::iter::adapters::copied::Copied<core::slice::iter::Iter<tuple_<utils::wall_space::WallSpaceRegion,country_core::resources::walls::wall_holes::HoleType>_>_>_>
                ();
    }
    *local_200 = local_198;
    lVar27 = *(longlong *)local_78[0x11];
    uVar24 = *(ulonglong *)(local_78[0x11] + 8);
    lVar33 = local_a0[2];
    if ((ulonglong)(*local_a0 - lVar33) < uVar24) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar33 = local_a0[2];
LAB_140828088:
      lVar21 = lVar33 * 0x50 + local_a0[1];
      lVar31 = 0;
      lVar22 = 0;
      do {
        bVar41 = *(byte *)(lVar27 + lVar31);
        uVar17 = (uint)bVar41;
        uVar34 = 0x18;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
        switch(bVar41) {
        case 1:
          pauVar29 = *(undefined1 (**) [16])(lVar27 + 8 + lVar31);
          break;
        case 2:
          uVar34 = *(undefined1 *)(lVar27 + 1 + lVar31);
          uVar17 = *(uint *)(lVar27 + 4 + lVar31);
        }
        uVar7 = *(undefined8 *)(lVar27 + 0x40 + lVar31);
        uVar12 = *(undefined8 *)(lVar27 + 0x48 + lVar31);
        *(byte *)(lVar21 + lVar31) = bVar41;
        *(undefined1 *)(lVar21 + 1 + lVar31) = uVar34;
        *(uint *)(lVar21 + 4 + lVar31) = uVar17;
        *(undefined1 (**) [16])(lVar21 + 8 + lVar31) = pauVar29;
        puVar39 = (undefined8 *)(lVar27 + 0x10 + lVar31);
        uVar15 = puVar39[1];
        auVar43 = *(undefined1 (*) [16])(lVar27 + 0x20 + lVar31);
        auVar54 = *(undefined1 (*) [16])(lVar27 + 0x30 + lVar31);
        puVar25 = (undefined8 *)(lVar21 + 0x10 + lVar31);
        *puVar25 = *puVar39;
        puVar25[1] = uVar15;
        *(undefined1 (*) [16])(lVar21 + 0x20 + lVar31) = auVar43;
        *(undefined1 (*) [16])(lVar21 + 0x30 + lVar31) = auVar54;
        *(undefined8 *)(lVar21 + 0x40 + lVar31) = uVar7;
        *(undefined8 *)(lVar21 + 0x48 + lVar31) = uVar12;
        lVar22 = lVar22 + -1;
        lVar31 = lVar31 + 0x50;
      } while (-lVar22 != uVar24);
      lVar33 = lVar33 - lVar22;
      puVar39 = local_180;
      uVar38 = local_ac;
    }
    else if (uVar24 != 0) goto LAB_140828088;
    local_a0[2] = lVar33;
    puVar25 = *(undefined8 **)local_78[0x12];
    if (puVar25 != (undefined8 *)0x0) {
      *(undefined4 *)local_98 = uVar38;
      uVar12 = puVar25[6];
      uVar11 = *(undefined4 *)(puVar25 + 8);
      uVar7 = puVar25[7];
      local_e8 = (undefined1 (*) [16])*puVar25;
      pauStack_e0 = (undefined1 (*) [16])puVar25[1];
      local_d8 = (undefined1 (*) [16])puVar25[2];
      pauStack_d0 = (undefined1 (*) [16])puVar25[3];
      local_c8 = puVar25[4];
      puStack_c0 = (undefined8 *)puVar25[5];
      lVar27 = local_f0[2];
      if (lVar27 == *local_f0) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<glam::f32::affine3a::Affine3A,u32,array_<f32,2>_>,alloc::alloc::Global>();
      }
      lVar33 = local_f0[1];
      lVar21 = lVar27 * 0x50;
      auVar43._8_8_ = pauStack_d0;
      auVar43._0_8_ = local_d8;
      auVar16._8_8_ = puStack_c0;
      auVar16._0_8_ = local_c8;
      *(undefined1 (*) [16])(lVar33 + 0x20 + lVar21) = auVar16;
      *(undefined1 (*) [16])(lVar33 + 0x10 + lVar21) = auVar43;
      *(undefined8 *)(lVar33 + lVar21) = local_e8;
      ((undefined8 *)(lVar33 + lVar21))[1] = pauStack_e0;
      *(undefined8 *)(lVar33 + 0x30 + lVar21) = uVar12;
      *(undefined8 *)(lVar33 + 0x38 + lVar21) = uVar7;
      *(undefined4 *)(lVar33 + 0x40 + lVar21) = uVar11;
      local_f0[2] = lVar27 + 1;
    }
    iVar19 = local_80;
    pauVar29 = local_90;
    local_e8 = (undefined1 (*) [16])CONCAT44(local_e8._4_4_,local_80);
    if (*(longlong *)(local_90[1] + 8) != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
      lVar27 = *(longlong *)*pauVar29;
      uVar24 = *(ulonglong *)(*pauVar29 + 8);
      bVar41 = (byte)(extraout_RAX_06 >> 0x39);
      auVar43 = ZEXT216(CONCAT11(bVar41,bVar41));
      auVar43 = pshuflw(auVar43,auVar43,0);
      lVar33 = 0;
      uVar36 = extraout_RAX_06;
      while( true ) {
        uVar36 = uVar24 & uVar36;
        auVar54 = *(undefined1 (*) [16])(lVar27 + uVar36);
        cVar42 = auVar43[0];
        auVar53[0] = -(auVar54[0] == cVar42);
        cVar46 = auVar43[1];
        auVar53[1] = -(auVar54[1] == cVar46);
        cVar47 = auVar43[2];
        auVar53[2] = -(auVar54[2] == cVar47);
        cVar48 = auVar43[3];
        auVar53[3] = -(auVar54[3] == cVar48);
        auVar53[4] = -(auVar54[4] == cVar42);
        auVar53[5] = -(auVar54[5] == cVar46);
        auVar53[6] = -(auVar54[6] == cVar47);
        auVar53[7] = -(auVar54[7] == cVar48);
        auVar53[8] = -(auVar54[8] == cVar42);
        auVar53[9] = -(auVar54[9] == cVar46);
        auVar53[10] = -(auVar54[10] == cVar47);
        auVar53[0xb] = -(auVar54[0xb] == cVar48);
        auVar53[0xc] = -(auVar54[0xc] == cVar42);
        auVar53[0xd] = -(auVar54[0xd] == cVar46);
        auVar53[0xe] = -(auVar54[0xe] == cVar47);
        auVar53[0xf] = -(auVar54[0xf] == cVar48);
        uVar35 = (ushort)(SUB161(auVar53 >> 7,0) & 1) | (ushort)(SUB161(auVar53 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar53 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar53 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar53 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar53 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar53 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar53 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar53 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar53 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar53 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar53 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar53 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar53 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar53 >> 0x77,0) & 1) << 0xe | (ushort)(auVar53[0xf] >> 7) << 0xf
        ;
        uVar17 = (uint)uVar35;
        while (uVar35 != 0) {
          uVar18 = 0;
          for (uVar13 = uVar17; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
            uVar18 = uVar18 + 1;
          }
          uVar37 = uVar18 + uVar36 & uVar24;
          uVar38 = local_ac;
          if (iVar19 == *(int *)(lVar27 + -0x10 + uVar37 * -0x10)) {
            pauVar29 = (undefined1 (*) [16])(*(longlong *)(lVar27 + -8 + uVar37 * -0x10) + 0x10);
            lVar27 = local_188;
            goto joined_r0x000140828329;
          }
          uVar35 = (ushort)(uVar17 - 1) & (ushort)uVar17;
          uVar17 = CONCAT22((short)(uVar17 - 1 >> 0x10),uVar35);
        }
        auVar50[0] = -(auVar54[0] == -1);
        auVar50[1] = -(auVar54[1] == -1);
        auVar50[2] = -(auVar54[2] == -1);
        auVar50[3] = -(auVar54[3] == -1);
        auVar50[4] = -(auVar54[4] == -1);
        auVar50[5] = -(auVar54[5] == -1);
        auVar50[6] = -(auVar54[6] == -1);
        auVar50[7] = -(auVar54[7] == -1);
        auVar50[8] = -(auVar54[8] == -1);
        auVar50[9] = -(auVar54[9] == -1);
        auVar50[10] = -(auVar54[10] == -1);
        auVar50[0xb] = -(auVar54[0xb] == -1);
        auVar50[0xc] = -(auVar54[0xc] == -1);
        auVar50[0xd] = -(auVar54[0xd] == -1);
        auVar50[0xe] = -(auVar54[0xe] == -1);
        auVar50[0xf] = -(auVar54[0xf] == -1);
        if ((((((((((((((((SUB161(auVar50 >> 7,0) & 1) != 0 || (SUB161(auVar50 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar50 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar50 >> 0x1f,0) & 1) != 0) || (SUB161(auVar50 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar50 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar50 >> 0x37,0) & 1) != 0) || (SUB161(auVar50 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar50 >> 0x47,0) & 1) != 0) || (SUB161(auVar50 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar50 >> 0x57,0) & 1) != 0) || (SUB161(auVar50 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar50 >> 0x67,0) & 1) != 0) || (SUB161(auVar50 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar50 >> 0x77,0) & 1) != 0) || auVar50[0xf] < '\0') break;
        uVar36 = uVar36 + lVar33 + 0x10;
        lVar33 = lVar33 + 0x10;
      }
    }
    pauVar29 = (undefined1 (*) [16])0x0;
    lVar27 = local_188;
joined_r0x000140828329:
    local_188 = local_1b8 + -1;
    lVar33 = local_188;
    if ((char)local_88 != '\0') {
      local_1a8 = local_78 + 1;
      if ((((pauVar29 != (undefined1 (*) [16])0x0) &&
           (*(longlong *)local_78[5] == *(longlong *)pauVar29[4])) &&
          ((*(longlong *)local_78[5] != 0 ||
           (*(longlong *)(local_78[5] + 8) == *(longlong *)(pauVar29[4] + 8))))) &&
         ((core::slice::cmp::impl$4::
           equal<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder>
                     (), extraout_AL_00 != '\0' &&
          (*(longlong *)(local_78[0xe] + 8) == *(longlong *)(pauVar29[0xd] + 8))))) {
        lVar27 = *(longlong *)local_78[0xe];
        lVar33 = *(longlong *)pauVar29[0xd];
        lVar22 = *(longlong *)(local_78[0xe] + 8) + 1;
        lVar21 = 0x10;
        while (lVar22 = lVar22 + -1, lVar22 != 0) {
          fVar6 = *(float *)(lVar27 + -0x10 + lVar21);
          pfVar5 = (float *)(lVar33 + -0x10 + lVar21);
          if ((((((fVar6 != *pfVar5) || (NAN(fVar6) || NAN(*pfVar5))) ||
                (fVar6 = *(float *)(lVar27 + -0xc + lVar21),
                pfVar5 = (float *)(lVar33 + -0xc + lVar21), fVar6 != *pfVar5)) ||
               ((NAN(fVar6) || NAN(*pfVar5) ||
                (fVar6 = *(float *)(lVar27 + -8 + lVar21), pfVar5 = (float *)(lVar33 + -8 + lVar21),
                fVar6 != *pfVar5)))) || (NAN(fVar6) || NAN(*pfVar5))) ||
             (((fVar6 = *(float *)(lVar27 + -4 + lVar21), pfVar5 = (float *)(lVar33 + -4 + lVar21),
               fVar6 != *pfVar5 || (NAN(fVar6) || NAN(*pfVar5))) ||
              (pcVar2 = (char *)(lVar27 + lVar21), pcVar3 = (char *)(lVar33 + lVar21),
              lVar21 = lVar21 + 0x14, *pcVar2 != *pcVar3)))) goto LAB_140828860;
        }
        if (*(longlong *)local_78[0x10] == *(longlong *)pauVar29[0xf]) {
          lVar33 = *(longlong *)(local_78[0xf] + 8);
          lVar21 = *(longlong *)(pauVar29[0xe] + 8);
          lVar22 = *(longlong *)local_78[0x10] + 1;
          lVar27 = 0x2c;
          break;
        }
      }
LAB_140828860:
      *(undefined4 *)local_98 = uVar38;
      local_158 = (undefined1 (*) [16])CONCAT44(local_158._4_4_,local_80);
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
      hashbrown::raw::RawTable::
      remove_entry<tuple_<country_core::resources::walls::decorator::DecoratorId,enum2_<system_decorator::decorator_visual_state::PersistentVisualEntities>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,enum2_<system_decorator::decorator_visual_state::PersistentVisualEntities>_>_>
                ();
      if ((int)pauStack_e0 != 2) {
        local_100 = local_d8;
        local_88 = pauStack_d0;
        if (local_c8 != 0) {
          lVar27 = local_c8 << 3;
          lVar33 = 0;
          do {
            if (*(ulonglong *)(*local_88 + lVar33) >> 0x20 == 0) break;
            local_e8 = (undefined1 (*) [16])*puVar39;
            pauStack_e0 = (undefined1 (*) [16])puVar39[1];
            local_d8 = SUB168(*(undefined1 (*) [16])(puVar39 + 2),0);
            pauStack_d0 = SUB168(*(undefined1 (*) [16])(puVar39 + 2),8);
            local_c8 = *(ulonglong *)(*local_88 + lVar33);
            bevy_ecs::system::commands::EntityCommands::despawn();
            lVar33 = lVar33 + 8;
          } while (lVar27 != lVar33);
        }
        if (local_100 != (undefined1 (*) [16])0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
      }
      alloc::vec::impl$11::
      clone<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>
                ();
      local_100 = local_e8;
      local_88 = pauStack_e0;
      if (local_d8 != (undefined1 (*) [16])0x0) {
        pauVar28 = pauStack_e0 + (longlong)local_d8 * 0x16;
        pauVar20 = pauStack_e0;
        do {
          if (*(longlong *)*pauVar20 == 2) break;
          memcpy(local_360,*pauVar20 + 8,0x158);
          *(undefined4 *)local_98 = uVar38;
          system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::
          build<system_decorator::decorator_visual_state::DecoratorPersistentVisualState>();
          pauVar20 = pauVar20 + 0x16;
        } while (pauVar20 != pauVar28);
      }
      if (local_100 != (undefined1 (*) [16])0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      *(undefined4 *)local_98 = uVar38;
      decorator_visual_state::DecoratorVisualState::seal_persistent_entities();
      if (*(longlong *)*local_1a8 == 0 && *(longlong *)(*local_1a8 + 8) == 0) {
        if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
          local_e8 = (undefined1 (*) [16])&PTR_s_Decorator_blueprint_did_not_spec_142a91868;
          pauStack_e0 = (undefined1 (*) [16])0x1;
          local_d8 = (undefined1 (*) [16])&DAT_00000008;
          pauStack_d0 = (undefined1 (*) [16])0x0;
          local_c8 = 0;
          toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
          local_158 = (undefined1 (*) [16])s_system_decorator__blueprint_142a91890;
          pauStack_150 = (undefined1 (*) [16])&DAT_0000001b;
          local_148 = (undefined1 (*) [16])s_system_decorator__blueprint_142a91890;
          uStack_140 = 0x1b;
          ::log::__private_api::log<tuple_<>,log::__private_api::GlobalLogger>();
        }
                    // WARNING: Subroutine does not return
        instantiate_blueprints::panic_cold_explicit();
      }
      if (pauVar29 != (undefined1 (*) [16])0x0) {
        if (*(longlong *)(pauVar29[0x11] + 8) != 0) {
          *local_170 = local_a8;
          decorator_visual::deco_differential_state::WallAttachedDecoTracker::
          remove<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams>
                    ();
        }
        if (*(longlong *)pauVar29[0x12] != 0) {
          *local_168 = local_a4;
          decorator_visual::deco_differential_state::WallAttachedDecoTracker::
          remove<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams>
                    ();
        }
      }
      puVar25 = *(undefined8 **)(local_78[0x12] + 8);
      if (puVar25 != (undefined8 *)0x0) {
        *local_170 = local_a8;
        local_c8 = puVar25[4];
        local_e8 = (undefined1 (*) [16])*puVar25;
        pauStack_e0 = (undefined1 (*) [16])puVar25[1];
        local_d8 = SUB168(*(undefined1 (*) [16])(puVar25 + 2),0);
        pauStack_d0 = SUB168(*(undefined1 (*) [16])(puVar25 + 2),8);
        decorator_visual::deco_differential_state::WallAttachedDecoTracker::
        add<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams>();
      }
      puVar25 = *(undefined8 **)local_78[0x13];
      lVar33 = local_188;
      lVar27 = local_188;
      if (puVar25 != (undefined8 *)0x0) {
        *local_168 = local_a4;
        local_e8 = (undefined1 (*) [16])*puVar25;
        pauStack_e0 = (undefined1 (*) [16])puVar25[1];
        local_d8 = SUB168(*(undefined1 (*) [16])(puVar25 + 2),0);
        pauStack_d0 = SUB168(*(undefined1 (*) [16])(puVar25 + 2),8);
        decorator_visual::deco_differential_state::WallAttachedDecoTracker::
        add<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams>
                  ();
        lVar33 = local_188;
        lVar27 = local_188;
      }
    }
  } while( true );
LAB_1408284bc:
  lVar22 = lVar22 + -1;
  if (lVar22 == 0) {
    if (*(int *)local_78[6] == 0 && *(int *)pauVar29[5] == 0) {
      if (((((*(longlong *)local_78[7] != *(longlong *)pauVar29[6]) ||
            (*(float *)(local_78[6] + 4) != *(float *)(pauVar29[5] + 4))) ||
           (NAN(*(float *)(local_78[6] + 4)) || NAN(*(float *)(pauVar29[5] + 4)))) ||
          ((*(float *)(local_78[6] + 8) != *(float *)(pauVar29[5] + 8) ||
           (NAN(*(float *)(local_78[6] + 8)) || NAN(*(float *)(pauVar29[5] + 8)))))) ||
         ((*(float *)(local_78[6] + 0xc) != *(float *)(pauVar29[5] + 0xc) ||
          ((NAN(*(float *)(local_78[6] + 0xc)) || NAN(*(float *)(pauVar29[5] + 0xc)) ||
           (auVar44._4_4_ = -(uint)(*(float *)(pauVar29[7] + 4) == *(float *)(local_78[8] + 4)),
           auVar44._0_4_ = -(uint)(*(float *)pauVar29[7] == *(float *)local_78[8]),
           auVar44._8_4_ = -(uint)(*(float *)(pauVar29[7] + 8) == *(float *)(local_78[8] + 8)),
           auVar44._12_4_ = -(uint)(*(float *)(pauVar29[7] + 0xc) == *(float *)(local_78[8] + 0xc)),
           iVar19 = movmskps((int)local_78,auVar44), iVar19 != 0xf)))))) goto LAB_140828860;
    }
    else if (*(int *)local_78[6] != *(int *)pauVar29[5]) goto LAB_140828860;
    if (*(int *)local_78[9] == 0 && *(int *)pauVar29[8] == 0) {
      if ((((*(longlong *)local_78[10] != *(longlong *)pauVar29[9]) ||
           (*(float *)(local_78[9] + 4) != *(float *)(pauVar29[8] + 4))) ||
          (NAN(*(float *)(local_78[9] + 4)) || NAN(*(float *)(pauVar29[8] + 4)))) ||
         (((*(float *)(local_78[9] + 8) != *(float *)(pauVar29[8] + 8) ||
           (NAN(*(float *)(local_78[9] + 8)) || NAN(*(float *)(pauVar29[8] + 8)))) ||
          ((*(float *)(local_78[9] + 0xc) != *(float *)(pauVar29[8] + 0xc) ||
           ((NAN(*(float *)(local_78[9] + 0xc)) || NAN(*(float *)(pauVar29[8] + 0xc)) ||
            (auVar45._4_4_ = -(uint)(*(float *)(pauVar29[10] + 4) == *(float *)(local_78[0xb] + 4)),
            auVar45._0_4_ = -(uint)(*(float *)pauVar29[10] == *(float *)local_78[0xb]),
            auVar45._8_4_ = -(uint)(*(float *)(pauVar29[10] + 8) == *(float *)(local_78[0xb] + 8)),
            auVar45._12_4_ =
                 -(uint)(*(float *)(pauVar29[10] + 0xc) == *(float *)(local_78[0xb] + 0xc)),
            iVar19 = movmskps((int)local_78,auVar45), iVar19 != 0xf)))))))) goto LAB_140828860;
    }
    else if (*(int *)local_78[9] != *(int *)pauVar29[8]) goto LAB_140828860;
    core::slice::cmp::impl$4::
    equal<country_core::systems::collision::WindowAutoClutterCandidate,country_core::systems::collision::WindowAutoClutterCandidate>
              ();
    lVar33 = local_188;
    if (((extraout_AL_01 != '\0') &&
        ((((core::option::impl$14::
            eq<alloc::boxed::Box<system_decorator::decorator_visual_state::ClothInfo,alloc::alloc::Global>_>
                      (), extraout_AL_02 != '\0' &&
           (core::option::impl$14::
            eq<alloc::boxed::Box<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams,alloc::alloc::Global>_>
                      (), extraout_AL_03 != '\0')) &&
          (core::option::impl$14::
           eq<alloc::boxed::Box<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams,alloc::alloc::Global>_>
                     (), extraout_AL_04 != '\0')) &&
         ((core::option::impl$14::eq<utils::components::transform::Transform>(),
          extraout_AL_05 != '\0' && (local_78[0x13][8] == pauVar29[0x12][8])))))) &&
       (lVar27 = local_188, local_78[0x13][9] == pauVar29[0x12][9])) goto LAB_140827e50;
    goto LAB_140828860;
  }
  fVar6 = *(float *)(lVar33 + -0x2c + lVar27);
  pfVar5 = (float *)(lVar21 + -0x2c + lVar27);
  if (((((fVar6 != *pfVar5) || (NAN(fVar6) || NAN(*pfVar5))) ||
       ((fVar6 = *(float *)(lVar33 + -0x28 + lVar27), pfVar5 = (float *)(lVar21 + -0x28 + lVar27),
        fVar6 != *pfVar5 ||
        (((NAN(fVar6) || NAN(*pfVar5) ||
          (fVar6 = *(float *)(lVar33 + -0x24 + lVar27), pfVar5 = (float *)(lVar21 + -0x24 + lVar27),
          fVar6 != *pfVar5)) || (NAN(fVar6) || NAN(*pfVar5))))))) ||
      ((*(int *)(lVar33 + -0x20 + lVar27) != *(int *)(lVar21 + -0x20 + lVar27) ||
       (fVar6 = *(float *)(lVar33 + -0x1c + lVar27), pfVar5 = (float *)(lVar21 + -0x1c + lVar27),
       fVar6 != *pfVar5)))) ||
     (((NAN(fVar6) || NAN(*pfVar5) ||
       (((fVar6 = *(float *)(lVar33 + -0x18 + lVar27), pfVar5 = (float *)(lVar21 + -0x18 + lVar27),
         fVar6 != *pfVar5 || (NAN(fVar6) || NAN(*pfVar5))) ||
        ((fVar6 = *(float *)(lVar33 + -0x14 + lVar27), pfVar5 = (float *)(lVar21 + -0x14 + lVar27),
         fVar6 != *pfVar5 ||
         (((NAN(fVar6) || NAN(*pfVar5) ||
           (*(int *)(lVar33 + -0x10 + lVar27) != *(int *)(lVar21 + -0x10 + lVar27))) ||
          (fVar6 = *(float *)(lVar33 + -0xc + lVar27), pfVar5 = (float *)(lVar21 + -0xc + lVar27),
          fVar6 != *pfVar5)))))))) ||
      ((((NAN(fVar6) || NAN(*pfVar5) ||
         (fVar6 = *(float *)(lVar33 + -8 + lVar27), pfVar5 = (float *)(lVar21 + -8 + lVar27),
         fVar6 != *pfVar5)) || (NAN(fVar6) || NAN(*pfVar5))) ||
       ((fVar6 = *(float *)(lVar33 + -4 + lVar27), pfVar5 = (float *)(lVar21 + -4 + lVar27),
        fVar6 != *pfVar5 || (NAN(fVar6) || NAN(*pfVar5))))))))) goto LAB_140828860;
  pfVar5 = (float *)(lVar33 + lVar27);
  pfVar4 = (float *)(lVar21 + lVar27);
  lVar27 = lVar27 + 0x30;
  if ((*pfVar5 != *pfVar4) || (NAN(*pfVar5) || NAN(*pfVar4))) goto LAB_140828860;
  goto LAB_1408284bc;
}


