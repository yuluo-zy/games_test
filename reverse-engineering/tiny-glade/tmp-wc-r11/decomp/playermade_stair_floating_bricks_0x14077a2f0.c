// name: playermade_stair_floating_bricks
// addr: 0x14077a2f0
// size: 9431
// signature: void __cdecl playermade_stair_floating_bricks(void)


void __cdecl
system_wall_constructor::construct_playermade_stairs::magic_support::
playermade_stair_floating_bricks(void)

{
  byte *pbVar1;
  float *pfVar2;
  longlong lVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  undefined1 auVar7 [16];
  byte bVar8;
  byte bVar9;
  ulonglong *puVar10;
  ulonglong *puVar11;
  uint uVar12;
  float fVar13;
  void *_Dst;
  bool bVar14;
  char extraout_AL;
  uint uVar15;
  int iVar16;
  uint uVar17;
  uint extraout_EAX;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  ulonglong uVar18;
  undefined8 *extraout_RAX_03;
  longlong lVar19;
  longlong lVar20;
  undefined8 *extraout_RAX_04;
  longlong lVar21;
  undefined8 *extraout_RAX_05;
  longlong *plVar22;
  longlong extraout_RAX_06;
  undefined8 *extraout_RAX_07;
  longlong lVar23;
  undefined8 *extraout_RAX_08;
  longlong lVar24;
  ulonglong extraout_RAX_09;
  undefined8 *extraout_RAX_10;
  undefined8 *extraout_RAX_11;
  undefined8 *extraout_RAX_12;
  longlong lVar25;
  longlong *extraout_RAX_13;
  longlong *extraout_RAX_14;
  longlong *extraout_RAX_15;
  longlong lVar26;
  longlong lVar27;
  longlong *plVar28;
  undefined8 *in_RDX;
  undefined8 *puVar29;
  ulonglong uVar30;
  ulonglong uVar31;
  ulonglong uVar32;
  longlong lVar33;
  undefined1 (*pauVar34) [16];
  undefined1 (*pauVar35) [16];
  ushort uVar36;
  undefined **ppuVar37;
  longlong *in_R9;
  ulonglong uVar38;
  ulonglong uVar39;
  longlong lVar40;
  longlong lVar41;
  ulonglong uVar42;
  bool bVar43;
  bool bVar44;
  byte bVar45;
  char cVar46;
  char cVar52;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  undefined4 extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  float extraout_XMM0_Da_05;
  float fVar47;
  float extraout_XMM0_Da_06;
  float extraout_XMM0_Da_07;
  float extraout_XMM0_Da_08;
  float extraout_XMM0_Da_09;
  undefined4 extraout_XMM0_Da_10;
  undefined4 extraout_XMM0_Da_11;
  undefined4 extraout_XMM0_Da_12;
  float fVar48;
  char cVar50;
  char cVar51;
  undefined1 auVar49 [16];
  undefined1 auVar53 [16];
  undefined1 auVar54 [16];
  float fVar55;
  float fVar56;
  float fVar57;
  undefined8 *in_stack_00000028;
  undefined8 *in_stack_00000030;
  undefined8 *in_stack_00000038;
  undefined8 *in_stack_00000040;
  undefined8 *in_stack_00000050;
  longlong *in_stack_00000058;
  undefined8 *in_stack_00000068;
  undefined8 local_358;
  float local_350;
  longlong local_2a8;
  longlong local_2a0;
  longlong local_238;
  longlong local_230;
  longlong local_218;
  longlong local_210;
  undefined8 uStack_1f0;
  ulonglong *local_190;
  undefined1 (*local_188) [16];
  undefined1 (*pauStack_180) [16];
  undefined1 (*local_120) [16];
  longlong local_118;
  
  puVar29 = in_RDX;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar14 = false;
  }
  else {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (puVar29 != (undefined8 *)0x0) {
        if (&DAT_00000064 < puVar29) {
          if (puVar29 != (undefined8 *)&DAT_00000065) goto LAB_14077a3b4;
        }
        else if (*(char *)(puVar29 + 0x2854fef4) < -0x40) {
LAB_14077a3b4:
                    // WARNING: Subroutine does not return
          core::str::slice_error_fail();
        }
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (puVar29 != (undefined8 *)0xfffffffffffffffe)) {
        if ((undefined1 *)((longlong)puVar29 + 2) < (undefined1 *)0x65) {
          if (*(char *)((longlong)puVar29 + 0x142a7f7a2) < -0x40) {
LAB_14077a408:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if ((undefined1 *)((longlong)puVar29 + 2) != (undefined1 *)0x65) goto LAB_14077a408;
      }
    }
    ppuVar37 = &PTR_DAT_142a7f920;
    do {
      bVar45 = *(byte *)((longlong)ppuVar37 + -1);
      uVar15 = (uint)(char)bVar45;
      if ((char)bVar45 < '\0') {
        bVar8 = *(byte *)((longlong)ppuVar37 + -2);
        if ((char)bVar8 < -0x40) {
          bVar9 = *(byte *)((longlong)ppuVar37 + -3);
          if ((char)bVar9 < -0x40) {
            pbVar1 = (byte *)((longlong)ppuVar37 + -4);
            ppuVar37 = (undefined **)((longlong)ppuVar37 + -4);
            uVar15 = bVar9 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            ppuVar37 = (undefined **)((longlong)ppuVar37 + -3);
            uVar15 = bVar9 & 0xf;
          }
          uVar15 = bVar8 & 0x3f | uVar15 << 6;
        }
        else {
          ppuVar37 = (undefined **)((longlong)ppuVar37 + -2);
          uVar15 = bVar8 & 0x1f;
        }
        uVar15 = (uint)(bVar45 & 0x3f) | uVar15 << 6;
        if (uVar15 != 0x5c) goto LAB_14077a4c4;
LAB_14077a4d6:
        if (ppuVar37 == (undefined **)&UNK_142a7f8cf) break;
        if ((undefined *)((longlong)ppuVar37 + -0x142a7f8cf) < (undefined *)0x50) {
          if (-0x41 < *(char *)((longlong)ppuVar37 + 1)) break;
        }
        else if ((undefined *)((longlong)ppuVar37 + -0x142a7f8cf) == (undefined *)0x50) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      ppuVar37 = (undefined **)((longlong)ppuVar37 + -1);
      if (uVar15 == 0x5c) goto LAB_14077a4d6;
LAB_14077a4c4:
      if (uVar15 == 0x2f) goto LAB_14077a4d6;
    } while (ppuVar37 != (undefined **)&DAT_142a7f8d0);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar28 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar28 = extraout_RAX_14, extraout_RAX_14 == (longlong *)0x0)) {
LAB_14077c675:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar28 != 0) {
LAB_14077c701:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar28 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar28 = *plVar28 + 1;
    bVar14 = true;
  }
  puVar10 = (ulonglong *)*in_stack_00000068;
  lVar19 = in_stack_00000068[1];
  uVar18 = *puVar10;
  uVar30 = 0;
  uVar32 = uVar18 - *(ulonglong *)(lVar19 + 0x18);
  if (uVar18 < *(ulonglong *)(lVar19 + 0x18)) {
    uVar32 = uVar30;
  }
  uVar31 = uVar18 - *(ulonglong *)(lVar19 + 0x38);
  if (uVar18 < *(ulonglong *)(lVar19 + 0x38)) {
    uVar31 = uVar30;
  }
  uVar18 = *(ulonglong *)(lVar19 + 0x10);
  uVar42 = *(ulonglong *)(lVar19 + 0x30);
  uVar39 = uVar42 - uVar31;
  if (uVar42 < uVar31) {
    uVar39 = uVar30;
  }
  uVar38 = uVar18 - uVar32;
  if (uVar18 < uVar32) {
    uVar38 = uVar30;
  }
  uVar30 = *(ulonglong *)(lVar19 + 0x40);
  *puVar10 = uVar30 - (uVar38 + uVar39);
  if ((uVar18 < uVar32 || uVar18 - uVar32 == 0) && (uVar42 < uVar31 || uVar42 - uVar31 == 0))
  goto LAB_14077aa1d;
  *puVar10 = uVar30;
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX_02 == (undefined8 *)0x0) {
LAB_14077c888:
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  if ((extraout_RAX_02[3] != 0x3a) ||
     (iVar16 = memcmp(&anon_11a7c4262c3a602e5b20874ec0ef09d1_1_llvm_11959870551130474958,
                      (void *)extraout_RAX_02[2],0x3a), iVar16 != 0)) {
                    // WARNING: Subroutine does not return
    core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
  }
  uVar18 = (**(code **)(extraout_RAX_02[1] + 0x18))(*extraout_RAX_02);
  if (uVar18 == 0) goto LAB_14077c888;
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX_03 == (undefined8 *)0x0) {
LAB_14077c896:
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  if ((extraout_RAX_03[3] != 0x2d) ||
     (iVar16 = memcmp(&anon_11a7c4262c3a602e5b20874ec0ef09d1_2_llvm_11959870551130474958,
                      (void *)extraout_RAX_03[2],0x2d), iVar16 != 0)) {
                    // WARNING: Subroutine does not return
    core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
  }
  lVar19 = (**(code **)(extraout_RAX_03[1] + 0x18))(*extraout_RAX_03);
  if (lVar19 == 0) goto LAB_14077c896;
  local_188 = (undefined1 (*) [16])*in_stack_00000050;
  auVar49 = *local_188;
  uVar15 = ~(uint)(ushort)((ushort)(SUB161(auVar49 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar49 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar49 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar49 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar49 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar49 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar49 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar49 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar49 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar49 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar49 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar49 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar49 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar49 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar49 >> 0x77,0) & 1) << 0xe |
                          (ushort)(byte)(auVar49[0xf] >> 7) << 0xf);
  uVar30 = (ulonglong)uVar15;
  lVar19 = in_stack_00000050[3];
  _Dst = (void *)*in_stack_00000050;
  uVar32 = in_stack_00000050[1];
  *in_stack_00000050 = &anon_bf4683eee411261bb73b14df3c83a622_0_llvm_420659448491601831;
  in_stack_00000050[1] = 0;
  in_stack_00000050[2] = 0;
  in_stack_00000050[3] = 0;
  pauStack_180 = local_188 + 1;
  uVar36 = (ushort)uVar15;
  while (lVar19 != 0) {
    uVar15 = (uint)uVar36;
    if (uVar36 != 0) {
      uVar17 = uVar36 - 1 & (uint)uVar36;
      uVar36 = (ushort)uVar17;
      if (local_188 != (undefined1 (*) [16])0x0) goto LAB_14077a837;
      local_188 = (undefined1 (*) [16])0x0;
LAB_14077a8f2:
      lVar19 = lVar19 + -1;
      pauVar35 = pauStack_180;
      if (lVar19 != 0) goto LAB_14077a910;
      break;
    }
    do {
      auVar49 = *pauStack_180;
      uVar36 = (ushort)(SUB161(auVar49 >> 7,0) & 1) | (ushort)(SUB161(auVar49 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar49 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar49 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar49 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar49 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar49 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar49 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar49 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar49 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar49 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar49 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar49 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar49 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar49 >> 0x77,0) & 1) << 0xe |
               (ushort)(byte)(auVar49[0xf] >> 7) << 0xf;
      local_188 = local_188 + -0x50;
      pauStack_180 = pauStack_180 + 1;
    } while (uVar36 == 0xffff);
    uVar15 = ~(uint)uVar36;
    uVar17 = uVar15 & -(uint)uVar36 - 2;
    uVar36 = (ushort)uVar17;
LAB_14077a837:
    uVar12 = 0;
    for (; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
      uVar12 = uVar12 + 1;
    }
    uVar30 = -*(longlong *)(local_188[(ulonglong)uVar12 * -5 + -5] + 8);
    if (SBORROW8(0,*(longlong *)(local_188[(ulonglong)uVar12 * -5 + -5] + 8))) goto LAB_14077a8f2;
    uVar30 = uVar18;
    country_core::resources::stairs::visual::MagicSupports::clear();
    core::ptr::drop_in_place<country_core::resources::stairs::visual::MagicSupports>();
    lVar19 = lVar19 + -1;
  }
  goto LAB_14077a9ad;
  while( true ) {
    lVar19 = lVar19 + -1;
    for (; (uVar17 & 1) == 0; uVar17 = uVar17 >> 1 | 0x80000000) {
    }
    core::ptr::drop_in_place<country_core::resources::stairs::visual::MagicSupports>();
    uVar17 = uVar15;
    if (lVar19 == 0) break;
LAB_14077a910:
    if ((short)uVar17 == 0) {
      do {
        auVar49 = *pauVar35;
        uVar36 = (ushort)(SUB161(auVar49 >> 7,0) & 1) | (ushort)(SUB161(auVar49 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar49 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar49 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar49 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar49 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar49 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar49 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar49 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar49 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar49 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar49 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar49 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar49 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar49 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar49[0xf] >> 7) << 0xf;
        local_188 = local_188 + -0x50;
        pauVar35 = pauVar35 + 1;
      } while (uVar36 == 0xffff);
      uVar17 = ~(uint)uVar36;
      uVar15 = uVar17 & -(uint)uVar36 - 2;
      pauStack_180 = pauVar35;
    }
    else {
      uVar15 = uVar17 - 1 & uVar17;
      if (local_188 == (undefined1 (*) [16])0x0) break;
    }
  }
LAB_14077a9ad:
  if (uVar32 == 0) {
    uVar18 = 0;
  }
  else {
    memset(_Dst,(int)CONCAT71((int7)(uVar30 >> 8),0xff),uVar32 + 0x11);
    uVar18 = (uVar32 + 1 & 0xfffffffffffffff8) - (uVar32 + 1 >> 3);
    if (uVar32 < 8) {
      uVar18 = uVar32;
    }
  }
  auVar49._8_8_ = 0;
  auVar49._0_8_ = uVar18;
  *in_stack_00000050 = _Dst;
  in_stack_00000050[1] = uVar32;
  *(undefined1 (*) [16])(in_stack_00000050 + 2) = auVar49;
LAB_14077aa1d:
  puVar10 = (ulonglong *)*in_stack_00000030;
  lVar19 = in_stack_00000030[1];
  uVar18 = *puVar10;
  uVar30 = 0;
  uVar32 = uVar18 - *(ulonglong *)(lVar19 + 0x18);
  if (uVar18 < *(ulonglong *)(lVar19 + 0x18)) {
    uVar32 = uVar30;
  }
  uVar31 = uVar18 - *(ulonglong *)(lVar19 + 0x38);
  if (uVar18 < *(ulonglong *)(lVar19 + 0x38)) {
    uVar31 = uVar30;
  }
  lVar20 = uVar32 * 0x10 + *(longlong *)(lVar19 + 8);
  uVar18 = *(ulonglong *)(lVar19 + 0x10) - uVar32;
  if (*(ulonglong *)(lVar19 + 0x10) < uVar32) {
    lVar20 = 8;
    uVar18 = uVar30;
  }
  lVar26 = uVar31 * 0x10 + *(longlong *)(lVar19 + 0x28);
  uVar32 = *(ulonglong *)(lVar19 + 0x30) - uVar31;
  if (*(ulonglong *)(lVar19 + 0x30) < uVar31) {
    lVar26 = 8;
    uVar32 = uVar30;
  }
  *puVar10 = (*(longlong *)(lVar19 + 0x40) - uVar32) - uVar18;
  lVar41 = uVar18 * 0x10 + lVar20;
  lVar19 = uVar32 * 0x10 + lVar26;
  do {
    if ((lVar20 == 0) || (lVar20 == lVar41)) {
      if ((lVar26 == 0) || (lVar26 == lVar19)) {
        local_190 = (ulonglong *)*in_stack_00000028;
        lVar19 = in_stack_00000028[1];
        uVar18 = *local_190;
        uVar30 = 0;
        uVar32 = uVar18 - *(ulonglong *)(lVar19 + 0x18);
        if (uVar18 < *(ulonglong *)(lVar19 + 0x18)) {
          uVar32 = uVar30;
        }
        uVar31 = uVar18 - *(ulonglong *)(lVar19 + 0x38);
        if (uVar18 < *(ulonglong *)(lVar19 + 0x38)) {
          uVar31 = uVar30;
        }
        local_230 = uVar32 * 0x10 + *(longlong *)(lVar19 + 8);
        uVar18 = *(ulonglong *)(lVar19 + 0x10) - uVar32;
        if (*(ulonglong *)(lVar19 + 0x10) < uVar32) {
          local_230 = 8;
          uVar18 = uVar30;
        }
        lVar20 = uVar31 * 0x10 + *(longlong *)(lVar19 + 0x28);
        uVar32 = *(ulonglong *)(lVar19 + 0x30) - uVar31;
        if (*(ulonglong *)(lVar19 + 0x30) < uVar31) {
          lVar20 = 8;
          uVar32 = uVar30;
        }
        *local_190 = (*(longlong *)(lVar19 + 0x40) - uVar32) - uVar18;
        lVar26 = uVar18 * 0x10 + local_230;
        lVar41 = uVar32 * 0x10 + lVar20;
        puVar10 = (ulonglong *)*in_stack_00000038;
        lVar19 = in_stack_00000038[1];
        uVar18 = *puVar10;
        uVar32 = uVar18 - *(ulonglong *)(lVar19 + 0x18);
        if (uVar18 < *(ulonglong *)(lVar19 + 0x18)) {
          uVar32 = uVar30;
        }
        uVar31 = uVar18 - *(ulonglong *)(lVar19 + 0x38);
        if (uVar18 < *(ulonglong *)(lVar19 + 0x38)) {
          uVar31 = uVar30;
        }
        local_210 = uVar32 * 0x28 + *(longlong *)(lVar19 + 8);
        uVar18 = *(ulonglong *)(lVar19 + 0x10) - uVar32;
        if (*(ulonglong *)(lVar19 + 0x10) < uVar32) {
          local_210 = 8;
          uVar18 = uVar30;
        }
        local_2a0 = uVar31 * 0x28 + *(longlong *)(lVar19 + 0x28);
        uVar32 = *(ulonglong *)(lVar19 + 0x30) - uVar31;
        if (*(ulonglong *)(lVar19 + 0x30) < uVar31) {
          local_2a0 = 8;
          uVar32 = uVar30;
        }
        *puVar10 = (*(longlong *)(lVar19 + 0x40) - uVar32) - uVar18;
        puVar11 = (ulonglong *)*in_stack_00000040;
        lVar19 = in_stack_00000040[1];
        uVar31 = *puVar11;
        uVar42 = uVar31 - *(ulonglong *)(lVar19 + 0x18);
        if (uVar31 < *(ulonglong *)(lVar19 + 0x18)) {
          uVar42 = uVar30;
        }
        uVar39 = uVar31 - *(ulonglong *)(lVar19 + 0x38);
        if (uVar31 < *(ulonglong *)(lVar19 + 0x38)) {
          uVar39 = uVar30;
        }
        local_238 = uVar42 * 0x28 + *(longlong *)(lVar19 + 8);
        uVar31 = *(ulonglong *)(lVar19 + 0x10) - uVar42;
        if (*(ulonglong *)(lVar19 + 0x10) < uVar42) {
          local_238 = 8;
          uVar31 = uVar30;
        }
        local_2a8 = uVar39 * 0x28 + *(longlong *)(lVar19 + 0x28);
        uVar42 = *(ulonglong *)(lVar19 + 0x30) - uVar39;
        if (*(ulonglong *)(lVar19 + 0x30) < uVar39) {
          local_2a8 = 8;
          uVar42 = uVar30;
        }
        *puVar11 = (*(longlong *)(lVar19 + 0x40) - uVar42) - uVar31;
        lVar19 = local_210 + uVar18 * 0x28;
        lVar21 = local_2a0 + uVar32 * 0x28;
        lVar3 = local_238 + uVar31 * 0x28;
        lVar25 = local_2a8 + uVar42 * 0x28;
        plVar28 = (longlong *)*in_stack_00000058;
        bVar44 = true;
        break;
      }
      lVar26 = lVar26 + 0x10;
      lVar20 = 0;
    }
    else {
      lVar20 = lVar20 + 0x10;
    }
    *puVar10 = *puVar10 + 1;
    frozen::FrozenMap::
    get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
              ();
    if (extraout_RAX_04 == (undefined8 *)0x0) {
LAB_14077c7a9:
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    if ((extraout_RAX_04[3] != 0x3a) ||
       (iVar16 = memcmp(&anon_11a7c4262c3a602e5b20874ec0ef09d1_1_llvm_11959870551130474958,
                        (void *)extraout_RAX_04[2],0x3a), iVar16 != 0)) {
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    lVar21 = (**(code **)(extraout_RAX_04[1] + 0x18))(*extraout_RAX_04);
    if (lVar21 == 0) goto LAB_14077c7a9;
    frozen::FrozenMap::
    get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
              ();
    if (extraout_RAX_05 == (undefined8 *)0x0) {
LAB_14077c7ba:
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    if ((extraout_RAX_05[3] != 0x2d) ||
       (iVar16 = memcmp(&anon_11a7c4262c3a602e5b20874ec0ef09d1_2_llvm_11959870551130474958,
                        (void *)extraout_RAX_05[2],0x2d), iVar16 != 0)) {
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    lVar21 = (**(code **)(extraout_RAX_05[1] + 0x18))(*extraout_RAX_05);
    if (lVar21 == 0) goto LAB_14077c7ba;
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    hashbrown::raw::RawTable::
    remove_entry<tuple_<country_core::asset_libraries::Handle<country_core::render::vao::Vao>,country_core::render::vao::Vao>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::asset_libraries::Handle<country_core::render::vao::Vao>,country_core::asset_libraries::Handle<country_core::render::vao::Vao>,country_core::render::vao::Vao>_>
              ();
    if (!SBORROW8(0,(longlong)pauStack_180)) {
      country_core::resources::stairs::visual::MagicSupports::clear();
      core::ptr::drop_in_place<country_core::resources::stairs::visual::MagicSupports>();
    }
  } while( true );
LAB_14077aea0:
  do {
    if (bVar44) {
      if (local_190 == (ulonglong *)0x0) {
LAB_14077af50:
        if ((local_210 == 0) || (local_210 == lVar19)) {
          if ((local_2a0 == 0) || (local_2a0 == lVar21)) {
            local_190 = (ulonglong *)0x0;
            local_210 = 0;
            if (local_238 != 0) goto LAB_14077aef7;
            goto LAB_14077afe0;
          }
          lVar33 = 0;
          local_210 = local_2a0;
          local_2a0 = local_2a0 + 0x28;
        }
        else {
          lVar33 = local_210 + 0x28;
        }
        *puVar10 = *puVar10 + 1;
        plVar22 = (longlong *)(local_210 + 0x18);
        bVar44 = true;
        local_190 = (ulonglong *)0x0;
        local_210 = lVar33;
      }
      else {
        if ((local_230 == 0) || (local_230 == lVar26)) {
          if (lVar20 == 0) {
            local_230 = 0;
          }
          else {
            local_230 = 0;
            if (lVar20 != lVar41) {
              lVar23 = lVar20 + 0x10;
              lVar33 = 0;
              local_230 = lVar20;
              goto LAB_14077af23;
            }
          }
          goto LAB_14077af50;
        }
        lVar33 = local_230 + 0x10;
        lVar23 = lVar20;
LAB_14077af23:
        *local_190 = *local_190 + 1;
        plVar22 = (longlong *)(local_230 + 8);
        bVar44 = true;
        lVar20 = lVar23;
        local_230 = lVar33;
      }
    }
    else {
      if (local_238 == 0) {
LAB_14077afe0:
        if ((local_2a8 == 0) || (local_2a8 == lVar25)) goto LAB_14077bafb;
        lVar33 = 0;
        local_238 = local_2a8;
        local_2a8 = local_2a8 + 0x28;
      }
      else {
LAB_14077aef7:
        if (local_238 == lVar3) goto LAB_14077afe0;
        lVar33 = local_238 + 0x28;
      }
      *puVar11 = *puVar11 + 1;
      plVar22 = (longlong *)(local_238 + 0x18);
      bVar44 = false;
      local_238 = lVar33;
    }
    lVar33 = *plVar22;
    country_core::resources::walls::public_walls::PublicWalls::get();
  } while (extraout_RAX_06 == 0);
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX_07 != (undefined8 *)0x0) {
    if ((extraout_RAX_07[3] != 0x3a) ||
       (iVar16 = memcmp(&anon_11a7c4262c3a602e5b20874ec0ef09d1_1_llvm_11959870551130474958,
                        (void *)extraout_RAX_07[2],0x3a), iVar16 != 0)) {
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    lVar23 = (**(code **)(extraout_RAX_07[1] + 0x18))(*extraout_RAX_07);
    if (lVar23 != 0) {
      frozen::FrozenMap::
      get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
                ();
      if (extraout_RAX_08 != (undefined8 *)0x0) {
        if ((extraout_RAX_08[3] != 0x2d) ||
           (iVar16 = memcmp(&anon_11a7c4262c3a602e5b20874ec0ef09d1_2_llvm_11959870551130474958,
                            (void *)extraout_RAX_08[2],0x2d), iVar16 != 0)) {
                    // WARNING: Subroutine does not return
          core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
        }
        lVar24 = (**(code **)(extraout_RAX_08[1] + 0x18))(*extraout_RAX_08);
        if (lVar24 != 0) {
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
          hashbrown::raw::RawTable::
          remove_entry<tuple_<country_core::asset_libraries::Handle<country_core::render::vao::Vao>,country_core::render::vao::Vao>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::asset_libraries::Handle<country_core::render::vao::Vao>,country_core::asset_libraries::Handle<country_core::render::vao::Vao>,country_core::render::vao::Vao>_>
                    ();
          if (!SBORROW8(0,(longlong)pauStack_180)) {
            country_core::resources::stairs::visual::MagicSupports::clear();
            core::ptr::drop_in_place<country_core::resources::stairs::visual::MagicSupports>();
          }
          if (plVar28[3] != 0) {
            core::hash::BuildHasher::
            hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
            lVar27 = *plVar28;
            bVar45 = (byte)(extraout_RAX_09 >> 0x39);
            auVar49 = ZEXT216(CONCAT11(bVar45,bVar45));
            auVar49 = pshuflw(auVar49,auVar49,0);
            lVar40 = 0;
            uVar18 = extraout_RAX_09;
            while( true ) {
              uVar18 = plVar28[1] & uVar18;
              auVar7 = *(undefined1 (*) [16])(lVar27 + uVar18);
              cVar46 = auVar49[0];
              auVar54[0] = -(auVar7[0] == cVar46);
              cVar50 = auVar49[1];
              auVar54[1] = -(auVar7[1] == cVar50);
              cVar51 = auVar49[2];
              auVar54[2] = -(auVar7[2] == cVar51);
              cVar52 = auVar49[3];
              auVar54[3] = -(auVar7[3] == cVar52);
              auVar54[4] = -(auVar7[4] == cVar46);
              auVar54[5] = -(auVar7[5] == cVar50);
              auVar54[6] = -(auVar7[6] == cVar51);
              auVar54[7] = -(auVar7[7] == cVar52);
              auVar54[8] = -(auVar7[8] == cVar46);
              auVar54[9] = -(auVar7[9] == cVar50);
              auVar54[10] = -(auVar7[10] == cVar51);
              auVar54[0xb] = -(auVar7[0xb] == cVar52);
              auVar54[0xc] = -(auVar7[0xc] == cVar46);
              auVar54[0xd] = -(auVar7[0xd] == cVar50);
              auVar54[0xe] = -(auVar7[0xe] == cVar51);
              auVar54[0xf] = -(auVar7[0xf] == cVar52);
              uVar36 = (ushort)(SUB161(auVar54 >> 7,0) & 1) |
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
              uVar15 = (uint)uVar36;
              while (uVar36 != 0) {
                uVar17 = 0;
                for (uVar12 = uVar15; (uVar12 & 1) == 0; uVar12 = uVar12 >> 1 | 0x80000000) {
                  uVar17 = uVar17 + 1;
                }
                uVar32 = uVar17 + uVar18 & plVar28[1];
                if (lVar33 == *(longlong *)(lVar27 + -0x40 + uVar32 * -0x40)) {
                  lVar33 = uVar32 * -0x40;
                  bVar43 = true;
                  if ((*(char *)(lVar27 + -8 + lVar33) == '\0') &&
                     (*(longlong *)(lVar27 + -0x20 + lVar33) == 0)) {
                    bVar43 = *(longlong *)(lVar27 + lVar33 + -0x18) != 0;
                  }
                  goto LAB_14077b2f0;
                }
                uVar36 = (ushort)(uVar15 - 1) & (ushort)uVar15;
                uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar36);
              }
              auVar53[0] = -(auVar7[0] == -1);
              auVar53[1] = -(auVar7[1] == -1);
              auVar53[2] = -(auVar7[2] == -1);
              auVar53[3] = -(auVar7[3] == -1);
              auVar53[4] = -(auVar7[4] == -1);
              auVar53[5] = -(auVar7[5] == -1);
              auVar53[6] = -(auVar7[6] == -1);
              auVar53[7] = -(auVar7[7] == -1);
              auVar53[8] = -(auVar7[8] == -1);
              auVar53[9] = -(auVar7[9] == -1);
              auVar53[10] = -(auVar7[10] == -1);
              auVar53[0xb] = -(auVar7[0xb] == -1);
              auVar53[0xc] = -(auVar7[0xc] == -1);
              auVar53[0xd] = -(auVar7[0xd] == -1);
              auVar53[0xe] = -(auVar7[0xe] == -1);
              auVar53[0xf] = -(auVar7[0xf] == -1);
              if ((((((((((((((((SUB161(auVar53 >> 7,0) & 1) != 0 ||
                               (SUB161(auVar53 >> 0xf,0) & 1) != 0) ||
                              (SUB161(auVar53 >> 0x17,0) & 1) != 0) ||
                             (SUB161(auVar53 >> 0x1f,0) & 1) != 0) ||
                            (SUB161(auVar53 >> 0x27,0) & 1) != 0) ||
                           (SUB161(auVar53 >> 0x2f,0) & 1) != 0) ||
                          (SUB161(auVar53 >> 0x37,0) & 1) != 0) ||
                         (SUB161(auVar53 >> 0x3f,0) & 1) != 0) ||
                        (SUB161(auVar53 >> 0x47,0) & 1) != 0) ||
                       (SUB161(auVar53 >> 0x4f,0) & 1) != 0) || (SUB161(auVar53 >> 0x57,0) & 1) != 0
                      ) || (SUB161(auVar53 >> 0x5f,0) & 1) != 0) ||
                    (SUB161(auVar53 >> 0x67,0) & 1) != 0) || (SUB161(auVar53 >> 0x6f,0) & 1) != 0)
                  || (SUB161(auVar53 >> 0x77,0) & 1) != 0) || auVar53[0xf] < '\0') break;
              uVar18 = uVar18 + lVar40 + 0x10;
              lVar40 = lVar40 + 0x10;
            }
          }
          bVar43 = false;
LAB_14077b2f0:
          lVar33 = *(longlong *)(extraout_RAX_06 + 0x40) +
                   *(longlong *)(extraout_RAX_06 + 0x48) * 0xc;
          lVar27 = *(longlong *)(extraout_RAX_06 + 0x40);
          do {
            lVar40 = lVar27;
            if (lVar40 == lVar33) break;
            country_core::resources::terrain_height::TerrainHeightsData::
            sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
            lVar27 = lVar40 + 0xc;
          } while (*(float *)(lVar40 + 4) - extraout_XMM0_Da <= 0.05);
          minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction()
          ;
          if ((byte)(bVar43 | lVar40 == lVar33 | *(byte *)(extraout_RAX_06 + 0x298) ^ 1) != 1) {
            std::alloc::__default_lib_allocator::__rust_alloc();
            if (extraout_RAX_10 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
              alloc::alloc::handle_alloc_error();
            }
            *extraout_RAX_10 = 0;
            extraout_RAX_10[1] = 4;
            extraout_RAX_10[2] = 0;
            *(undefined1 *)(extraout_RAX_10 + 3) = 0;
            lVar33 = *(longlong *)(lVar24 + 0x60);
LAB_14077b3c2:
            lVar27 = *(longlong *)(lVar33 + 8);
            do {
              if (lVar27 == -1) goto LAB_14077b3c2;
              if (lVar27 < 0) {
                    // WARNING: Subroutine does not return
                alloc::sync::impl$21::downgrade::panic_cold_display<ref_<str_>_>();
              }
              LOCK();
              lVar40 = *(longlong *)(lVar33 + 8);
              bVar43 = lVar27 == lVar40;
              if (bVar43) {
                *(longlong *)(lVar33 + 8) = lVar27 + 1;
                lVar40 = lVar27;
              }
              UNLOCK();
              lVar27 = lVar40;
            } while (!bVar43);
            LOCK();
            bVar43 = *(char *)(extraout_RAX_10 + 3) == '\0';
            if (bVar43) {
              *(char *)(extraout_RAX_10 + 3) = '\x01';
            }
            UNLOCK();
            if (!bVar43) {
              parking_lot::raw_mutex::RawMutex::lock_slow();
            }
            parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
            if (extraout_RAX_10[2] * 4 + extraout_RAX_10[1] == 4 || extraout_RAX_10[2] == 0) {
              pauStack_180 = *(undefined1 (**) [16])(lVar24 + 0x28);
              if (pauStack_180 == (undefined1 (*) [16])0x0) {
                    // WARNING: Subroutine does not return
                core::option::unwrap_failed();
              }
            }
            else {
              pauStack_180 = *(undefined1 (**) [16])(lVar24 + 0x28);
            }
            local_218 = 0;
            lVar24 = *(longlong *)(extraout_RAX_06 + 0x40);
            uVar18 = *(ulonglong *)(extraout_RAX_06 + 0x48);
            if (*(ulonglong *)(extraout_RAX_06 + 0x60) < *(ulonglong *)(extraout_RAX_06 + 0x48)) {
              uVar18 = *(ulonglong *)(extraout_RAX_06 + 0x60);
            }
            uVar32 = 0;
            while (uVar32 < uVar18) {
              ::utils::curve::Curve::
              get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                        ();
              if (*(ulonglong *)(extraout_RAX_06 + 0x48) <= (ulonglong)extraout_EAX + 1) {
                    // WARNING: Subroutine does not return
                core::panicking::panic_bounds_check();
              }
              ::utils::swizzlers::impl$0::x0y();
              fastrand::Rng::f32();
              pfVar2 = (float *)(lVar24 + uVar32 * 0xc);
              fVar55 = extraout_XMM0_Da_00 * 0.2 + 0.58000004;
              fVar57 = (float)((ulonglong)local_358 >> 0x20);
              fVar48 = *pfVar2;
              fVar13 = pfVar2[1];
              fVar4 = pfVar2[2];
              fastrand::Rng::f32();
              fastrand::Rng::f32();
              fastrand::Rng::f32();
              fastrand::Rng::f32();
              fastrand::Rng::f32();
              uStack_1f0 = CONCAT44(extraout_XMM0_Da_03,
                                    fVar55 * local_350 + fVar4 + (extraout_XMM0_Da_02 + -0.5) * 0.2)
              ;
              country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
              push<country_core::resources::sparkle::SparkleSsbo>();
              fastrand::Rng::f32();
              uVar32 = uVar32 + 1;
              if (0.2 <= extraout_XMM0_Da_04) {
                fastrand::Rng::f32();
                fVar47 = extraout_XMM0_Da_05 * 0.2 + 0.58000004;
                fVar4 = *pfVar2;
                fVar5 = pfVar2[1];
                fVar6 = pfVar2[2];
                fastrand::Rng::f32();
                fastrand::Rng::f32();
                fastrand::Rng::f32();
                fastrand::Rng::f32();
                fVar56 = extraout_XMM0_Da_09 * 0.525 + (1.0 - extraout_XMM0_Da_09) * 0.01;
                if (fVar56 <= 0.15) {
                  fVar56 = 0.15;
                }
                ::utils::random_orthogonal_quat();
                fastrand::Rng::f32();
                fastrand::Rng::f32();
                fastrand::Rng::f32();
                if (local_218 == 0) {
                  alloc::raw_vec::RawVec::
                  grow_one<country_core::resources::stairs::visual::StairsFloatingBrickParams,alloc::alloc::Global>
                            ();
                }
                lVar27 = local_218 * 0x40;
                *(ulonglong *)(lVar27 + 0x10) =
                     CONCAT44(fVar55 * fVar57 + fVar13 + -0.1,
                              (extraout_XMM0_Da_01 + -0.5) * 0.2 +
                              fVar55 * (float)local_358 + fVar48);
                *(undefined8 *)(lVar27 + 0x18) = uStack_1f0;
                *(float *)(lVar27 + 0x20) =
                     (float)local_358 * fVar47 + fVar4 + (extraout_XMM0_Da_06 + -0.5) * 0.2;
                *(float *)(lVar27 + 0x24) =
                     (fVar57 * fVar47 + fVar5 + -0.1) -
                     (extraout_XMM0_Da_08 * 0.39 + (1.0 - extraout_XMM0_Da_08) * 0.0);
                *(float *)(lVar27 + 0x28) =
                     local_350 * fVar47 + fVar6 + (extraout_XMM0_Da_07 + -0.5) * 0.2;
                *(float *)(lVar27 + 0x2c) = fVar56;
                *(ulonglong *)(lVar27 + 0x30) = CONCAT44(fVar56 * 0.75,fVar56 * 0.65);
                *(undefined4 *)(lVar27 + 0x40) = 0x3f800000;
                *(undefined4 *)(lVar27 + 0x44) = extraout_XMM0_Da_10;
                *(undefined4 *)(lVar27 + 0x48) = extraout_XMM0_Da_11;
                *(undefined4 *)(lVar27 + 0x4c) = extraout_XMM0_Da_12;
                local_218 = local_218 + 1;
              }
            }
            std::alloc::__default_lib_allocator::__rust_alloc();
            if (extraout_RAX_11 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
              alloc::alloc::handle_alloc_error();
            }
            *extraout_RAX_11 = 0;
            extraout_RAX_11[1] = 4;
            extraout_RAX_11[2] = 0;
            *(undefined1 *)(extraout_RAX_11 + 3) = 0;
            lVar23 = *(longlong *)(lVar23 + 0x60);
            do {
              lVar24 = *(longlong *)(lVar23 + 8);
              while (lVar24 != -1) {
                if (lVar24 < 0) {
                    // WARNING: Subroutine does not return
                  alloc::sync::impl$21::downgrade::panic_cold_display<ref_<str_>_>();
                }
                plVar22 = (longlong *)(lVar23 + 8);
                LOCK();
                lVar27 = *plVar22;
                bVar43 = lVar24 == lVar27;
                if (bVar43) {
                  *plVar22 = lVar24 + 1;
                  lVar27 = lVar24;
                }
                UNLOCK();
                lVar24 = lVar27;
                if (bVar43) {
                  minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::
                  get_instruction();
                  hashbrown::map::HashMap::
                  insert<country_core::resources::walls::WallId,country_core::resources::stairs::visual::MagicSupports,std::hash::random::RandomState,alloc::alloc::Global>
                            ();
                  if (!SBORROW8(0,lVar33)) {
                    core::ptr::drop_in_place<country_core::resources::stairs::visual::MagicSupports>
                              ();
                  }
                  parking_lot_core::parking_lot::deadlock_impl::release_resource();
                  LOCK();
                  bVar43 = *(char *)(extraout_RAX_10 + 3) == '\x01';
                  if (bVar43) {
                    *(char *)(extraout_RAX_10 + 3) = '\0';
                  }
                  UNLOCK();
                  if (!bVar43) {
                    parking_lot::raw_mutex::RawMutex::unlock_slow();
                  }
                  goto LAB_14077aea0;
                }
              }
            } while( true );
          }
          goto LAB_14077aea0;
        }
      }
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
LAB_14077bafb:
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX_12 != (undefined8 *)0x0) {
    if ((extraout_RAX_12[3] != 0x3a) ||
       (iVar16 = memcmp(&anon_11a7c4262c3a602e5b20874ec0ef09d1_1_llvm_11959870551130474958,
                        (void *)extraout_RAX_12[2],0x3a), iVar16 != 0)) {
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    lVar19 = (**(code **)(extraout_RAX_12[1] + 0x18))(*extraout_RAX_12);
    if (lVar19 != 0) {
      *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
      pauVar34 = *(undefined1 (**) [16])*in_RDX;
      lVar20 = ((undefined8 *)*in_RDX)[3];
      auVar49 = *pauVar34;
      pauVar35 = pauVar34 + 1;
      lVar26 = *in_R9;
      uVar15 = ~(uint)(ushort)((ushort)(SUB161(auVar49 >> 7,0) & 1) |
                               (ushort)(SUB161(auVar49 >> 0xf,0) & 1) << 1 |
                               (ushort)(SUB161(auVar49 >> 0x17,0) & 1) << 2 |
                               (ushort)(SUB161(auVar49 >> 0x1f,0) & 1) << 3 |
                               (ushort)(SUB161(auVar49 >> 0x27,0) & 1) << 4 |
                               (ushort)(SUB161(auVar49 >> 0x2f,0) & 1) << 5 |
                               (ushort)(SUB161(auVar49 >> 0x37,0) & 1) << 6 |
                               (ushort)(SUB161(auVar49 >> 0x3f,0) & 1) << 7 |
                               (ushort)(SUB161(auVar49 >> 0x47,0) & 1) << 8 |
                               (ushort)(SUB161(auVar49 >> 0x4f,0) & 1) << 9 |
                               (ushort)(SUB161(auVar49 >> 0x57,0) & 1) << 10 |
                               (ushort)(SUB161(auVar49 >> 0x5f,0) & 1) << 0xb |
                               (ushort)(SUB161(auVar49 >> 0x67,0) & 1) << 0xc |
                               (ushort)(SUB161(auVar49 >> 0x6f,0) & 1) << 0xd |
                               (ushort)(SUB161(auVar49 >> 0x77,0) & 1) << 0xe |
                              (ushort)(byte)(auVar49[0xf] >> 7) << 0xf);
      while (lVar20 != 0) {
        lVar20 = lVar20 + -1;
        if ((short)uVar15 == 0) {
          do {
            auVar49 = *pauVar35;
            uVar36 = (ushort)(SUB161(auVar49 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar49 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar49 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar49 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar49 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar49 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar49 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar49 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar49 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar49 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar49 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar49 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar49 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar49 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar49 >> 0x77,0) & 1) << 0xe |
                     (ushort)(byte)(auVar49[0xf] >> 7) << 0xf;
            pauVar34 = pauVar34 + -0x110;
            pauVar35 = pauVar35 + 1;
          } while (uVar36 == 0xffff);
          uVar15 = ~(uint)uVar36;
          uVar17 = uVar15 & -(uint)uVar36 - 2;
        }
        else {
          uVar17 = uVar15 - 1 & uVar15;
        }
        uVar12 = 0;
        for (; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
          uVar12 = uVar12 + 1;
        }
        uVar18 = (ulonglong)uVar12;
        if (*(longlong *)pauVar34[uVar18 * -0x11 + -10] == 0) {
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        lVar41 = *(longlong *)(pauVar34[uVar18 * -0x11 + -10] + 8);
        if (lVar41 == 0) {
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        LOCK();
        bVar44 = *(char *)(lVar41 + 0x18) == '\0';
        if (bVar44) {
          *(char *)(lVar41 + 0x18) = '\x01';
        }
        UNLOCK();
        if (!bVar44) {
          parking_lot::raw_mutex::RawMutex::lock_slow();
        }
        parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
        if ((*(longlong *)(lVar41 + 0x10) * 4 + *(longlong *)(lVar41 + 8) == 4 ||
             *(longlong *)(lVar41 + 0x10) == 0) && (*(longlong *)(lVar19 + 0x28) == 0)) {
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
        clear<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
        lVar21 = *(longlong *)(pauVar34[uVar18 * -0x11 + -0xc] + 8);
        lVar25 = *(longlong *)pauVar34[uVar18 * -0x11 + -0xb] * 0x40 + lVar21;
        for (; lVar21 != lVar25; lVar21 = lVar21 + 0x40) {
          fVar48 = *(float *)(lVar26 + 0x28) / 0.3 + *(float *)(lVar21 + 0x30);
          fVar4 = 0.0;
          if (0.0 <= fVar48) {
            fVar4 = fVar48;
          }
          fVar48 = 1.0;
          if (fVar4 <= 1.0) {
            fVar48 = fVar4;
          }
          *(float *)(lVar21 + 0x30) = fVar48;
          ::utils::easing_functions::EasingFunction::apply();
          sinf((float)*(double *)(lVar26 + 0x10) * (*(float *)(lVar21 + 0x38) + 2.0) +
               *(float *)(lVar21 + 0x34));
          country_core::resources::render::impl$1::from();
          country_core::geometry::instanced_wall::BrickSourceId::from_wall_id();
          country_core::geometry::instanced_wall::impl$3::default();
          country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
          push<country_core::geometry::instanced_wall::BrickTransformSsbo>();
        }
        parking_lot_core::parking_lot::deadlock_impl::release_resource();
        LOCK();
        bVar44 = *(char *)(lVar41 + 0x18) == '\x01';
        if (bVar44) {
          *(char *)(lVar41 + 0x18) = '\0';
        }
        UNLOCK();
        uVar15 = uVar17;
        if (!bVar44) {
          parking_lot::raw_mutex::RawMutex::unlock_slow();
        }
      }
      local_120 = (undefined1 (*) [16])*in_stack_00000050;
      local_118 = in_stack_00000050[3];
      auVar49 = *local_120;
      pauVar35 = local_120 + 1;
      uVar15 = ~(uint)(ushort)((ushort)(SUB161(auVar49 >> 7,0) & 1) |
                               (ushort)(SUB161(auVar49 >> 0xf,0) & 1) << 1 |
                               (ushort)(SUB161(auVar49 >> 0x17,0) & 1) << 2 |
                               (ushort)(SUB161(auVar49 >> 0x1f,0) & 1) << 3 |
                               (ushort)(SUB161(auVar49 >> 0x27,0) & 1) << 4 |
                               (ushort)(SUB161(auVar49 >> 0x2f,0) & 1) << 5 |
                               (ushort)(SUB161(auVar49 >> 0x37,0) & 1) << 6 |
                               (ushort)(SUB161(auVar49 >> 0x3f,0) & 1) << 7 |
                               (ushort)(SUB161(auVar49 >> 0x47,0) & 1) << 8 |
                               (ushort)(SUB161(auVar49 >> 0x4f,0) & 1) << 9 |
                               (ushort)(SUB161(auVar49 >> 0x57,0) & 1) << 10 |
                               (ushort)(SUB161(auVar49 >> 0x5f,0) & 1) << 0xb |
                               (ushort)(SUB161(auVar49 >> 0x67,0) & 1) << 0xc |
                               (ushort)(SUB161(auVar49 >> 0x6f,0) & 1) << 0xd |
                               (ushort)(SUB161(auVar49 >> 0x77,0) & 1) << 0xe |
                              (ushort)(byte)(auVar49[0xf] >> 7) << 0xf);
      while (local_118 != 0) {
        local_118 = local_118 + -1;
        if ((short)uVar15 == 0) {
          do {
            auVar49 = *pauVar35;
            uVar36 = (ushort)(SUB161(auVar49 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar49 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar49 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar49 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar49 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar49 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar49 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar49 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar49 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar49 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar49 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar49 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar49 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar49 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar49 >> 0x77,0) & 1) << 0xe |
                     (ushort)(byte)(auVar49[0xf] >> 7) << 0xf;
            local_120 = local_120 + -0x50;
            pauVar35 = pauVar35 + 1;
          } while (uVar36 == 0xffff);
          uVar15 = ~(uint)uVar36;
          uVar17 = uVar15 & -(uint)uVar36 - 2;
        }
        else {
          uVar17 = uVar15 - 1 & uVar15;
        }
        uVar12 = 0;
        for (; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
          uVar12 = uVar12 + 1;
        }
        uVar18 = (ulonglong)uVar12;
        if (*(longlong *)(local_120[uVar18 * -5 + -3] + 8) == 0) {
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        lVar20 = *(longlong *)local_120[uVar18 * -5 + -2];
        if (lVar20 == 0) {
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        LOCK();
        bVar44 = *(char *)(lVar20 + 0x18) == '\0';
        if (bVar44) {
          *(char *)(lVar20 + 0x18) = '\x01';
        }
        UNLOCK();
        if (!bVar44) {
          parking_lot::raw_mutex::RawMutex::lock_slow();
        }
        parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
        if ((*(longlong *)(lVar20 + 0x10) * 4 + *(longlong *)(lVar20 + 8) == 4 ||
             *(longlong *)(lVar20 + 0x10) == 0) && (*(longlong *)(lVar19 + 0x28) == 0)) {
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
        clear<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
        lVar41 = *(longlong *)local_120[uVar18 * -5 + -4];
        lVar21 = *(longlong *)(local_120[uVar18 * -5 + -4] + 8) * 0x40 + lVar41;
        for (; lVar41 != lVar21; lVar41 = lVar41 + 0x40) {
          minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction()
          ;
          fVar48 = *(float *)(lVar26 + 0x28) / 0.3 + *(float *)(lVar41 + 0x30);
          fVar4 = 0.0;
          if (0.0 <= fVar48) {
            fVar4 = fVar48;
          }
          fVar48 = 1.0;
          if (fVar4 <= 1.0) {
            fVar48 = fVar4;
          }
          *(float *)(lVar41 + 0x30) = fVar48;
          ::utils::easing_functions::EasingFunction::apply();
          sinf((float)*(double *)(lVar26 + 0x10) * (*(float *)(lVar41 + 0x38) + 2.0) +
               *(float *)(lVar41 + 0x34));
          country_core::resources::render::impl$1::from();
          country_core::geometry::instanced_wall::BrickSourceId::from_wall_id();
          country_core::geometry::instanced_wall::impl$3::default();
          country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
          push<country_core::geometry::instanced_wall::BrickTransformSsbo>();
        }
        parking_lot_core::parking_lot::deadlock_impl::release_resource();
        LOCK();
        bVar44 = *(char *)(lVar20 + 0x18) == '\x01';
        if (bVar44) {
          *(char *)(lVar20 + 0x18) = '\0';
        }
        UNLOCK();
        uVar15 = uVar17;
        if (!bVar44) {
          parking_lot::raw_mutex::RawMutex::unlock_slow();
        }
      }
      if (bVar14) {
        puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
        if (*extraout_RAX_13 == 1) {
          plVar28 = extraout_RAX_13 + 1;
        }
        else if ((*extraout_RAX_13 != 0) ||
                (std::sys::thread_local::native::lazy::Storage::
                 initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                           (), plVar28 = extraout_RAX_15, extraout_RAX_15 == (longlong *)0x0))
        goto LAB_14077c675;
        if (*plVar28 != 0) goto LAB_14077c701;
        *plVar28 = -1;
        puffin::ThreadProfiler::end_scope();
        *plVar28 = *plVar28 + 1;
      }
      return;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

