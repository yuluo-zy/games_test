
void __cdecl country_core::systems::save::save_game::save_game(void)

{
  byte *pbVar1;
  uint *puVar2;
  float ****ppppfVar3;
  byte bVar4;
  byte bVar5;
  byte bVar6;
  byte bVar7;
  undefined1 uVar8;
  undefined1 uVar9;
  ushort uVar10;
  undefined2 uVar11;
  int iVar12;
  int iVar13;
  int iVar14;
  undefined8 uVar15;
  int *piVar16;
  undefined4 *puVar17;
  undefined8 uVar18;
  undefined8 uVar19;
  longlong lVar20;
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  code *pcVar24;
  undefined8 uVar25;
  float ******ppppppfVar26;
  bool bVar27;
  char extraout_AL;
  char cVar28;
  char extraout_AL_00;
  uint uVar29;
  int iVar30;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  float *****pppppfVar31;
  float *******extraout_RAX_02;
  undefined8 *extraout_RAX_03;
  float *******extraout_RAX_04;
  float *******extraout_RAX_05;
  float *******extraout_RAX_06;
  float ******extraout_RAX_07;
  float *******extraout_RAX_08;
  float *******extraout_RAX_09;
  float *******extraout_RAX_10;
  float ******extraout_RAX_11;
  float *******extraout_RAX_12;
  float *******extraout_RAX_13;
  longlong *extraout_RAX_14;
  float *******extraout_RAX_15;
  float *******extraout_RAX_16;
  float ******extraout_RAX_17;
  float ******ppppppfVar32;
  float *******extraout_RAX_18;
  float *******extraout_RAX_19;
  float *******extraout_RAX_20;
  longlong lVar33;
  float *******extraout_RAX_21;
  ulonglong *extraout_RAX_22;
  ulonglong *extraout_RAX_23;
  longlong *extraout_RAX_24;
  longlong *extraout_RAX_25;
  undefined8 *in_RCX;
  float *******pppppppfVar34;
  longlong *plVar35;
  undefined4 uVar36;
  int iVar37;
  undefined1 *in_RDX;
  undefined1 *puVar38;
  longlong lVar39;
  float ******ppppppfVar40;
  float *******pppppppfVar41;
  float *****pppppfVar42;
  ulonglong uVar43;
  float *******pppppppfVar44;
  float ******ppppppfVar45;
  float *******pppppppfVar46;
  float *****pppppfVar47;
  undefined8 *puVar48;
  float ******ppppppfVar49;
  float *******in_R8;
  undefined *puVar50;
  undefined4 *puVar51;
  float *******pppppppfVar52;
  uint uVar53;
  undefined8 *in_R9;
  ulonglong uVar54;
  float *****pppppfVar55;
  undefined4 *puVar56;
  undefined8 *puVar57;
  longlong lVar58;
  float *******pppppppfVar59;
  float *******pppppppfVar60;
  float ******ppppppfVar61;
  bool bVar62;
  bool bVar63;
  bool bVar64;
  bool bVar65;
  bool bVar66;
  bool bVar67;
  bool bVar68;
  bool bVar69;
  bool bVar70;
  float fVar71;
  float fVar72;
  longlong *in_stack_00000028;
  undefined8 *in_stack_00000030;
  float *****local_420;
  float *****pppppfStack_418;
  float *****local_410;
  float *****local_3a0;
  float ******local_398;
  float ******ppppppfStack_390;
  undefined8 local_388;
  float local_380;
  float fStack_37c;
  float fStack_378;
  float fStack_374;
  float local_370;
  float fStack_36c;
  float fStack_368;
  float fStack_364;
  float fStack_360;
  longlong local_358;
  undefined8 local_350;
  undefined8 local_348;
  undefined8 local_340;
  undefined8 local_338;
  undefined8 local_330;
  undefined8 local_328;
  float *****local_320;
  undefined8 *local_318;
  undefined4 local_30c;
  int local_308;
  int local_304;
  int local_300;
  int local_2fc;
  int local_2f8;
  int local_2f4;
  int local_2f0;
  int local_2ec;
  int local_2e8;
  int local_2e4;
  int local_2e0;
  int local_2dc;
  int local_2d8;
  uint local_2d4;
  float ******local_2d0;
  longlong *local_2c8;
  float *****local_2c0;
  float ******local_2b8;
  float ******ppppppfStack_2b0;
  float ******ppppppfStack_2a8;
  float ******ppppppfStack_2a0;
  float *****pppppfStack_298;
  undefined8 local_290;
  undefined8 local_288;
  char *pcStack_280;
  undefined8 local_278;
  undefined8 local_270;
  undefined **ppuStack_268;
  undefined8 local_260;
  float ******ppppppfStack_258;
  undefined8 local_250;
  undefined8 uStack_248;
  undefined8 local_1d0;
  undefined8 local_1c8;
  undefined8 local_1c0;
  undefined8 local_1b8;
  undefined8 local_1b0;
  float ******local_1a8;
  float ******ppppppfStack_1a0;
  undefined8 local_198;
  undefined8 local_190;
  undefined8 local_188;
  undefined8 local_180;
  float ******local_178;
  longlong local_170;
  float ******local_168;
  float ******local_160;
  undefined8 local_158;
  undefined8 local_150;
  float ******local_148;
  undefined8 *local_140;
  float ******local_138;
  float ******local_130;
  float ******local_128;
  float ******ppppppfStack_120;
  float ******local_118;
  float ******ppppppfStack_110;
  float *****local_108;
  char cStack_100;
  undefined4 uStack_ff;
  undefined2 uStack_fb;
  undefined1 uStack_f9;
  undefined8 local_f8;
  undefined8 uStack_f0;
  float ******local_e0;
  byte local_d1;
  float ******local_d0;
  float ******local_c0;
  undefined1 local_b2;
  undefined1 local_b1;
  undefined8 local_b0;
  
  local_b0 = 0xfffffffffffffffe;
  local_d0 = (float ******)*in_RCX;
  local_148 = (float ******)in_RCX[2];
  local_130 = (float ******)CONCAT44(local_130._4_4_,*(undefined4 *)((longlong)in_RCX + 0x1c));
  pppppppfVar34 = (float *******)in_RCX[4];
  pppppppfVar41 = (float *******)in_RCX[6];
  iVar30 = *(int *)((longlong)in_RCX + 0x3c);
  pppppppfVar60 = (float *******)in_RCX[8];
  local_178 = (float ******)in_RCX[0xc];
  uVar15 = in_RCX[0x35];
  ppppppfVar61 = (float ******)in_RCX[0xd];
  pppppppfVar52 = (float *******)in_RCX[0x33];
  pppppppfVar44 = (float *******)in_RCX[0x34];
  local_168 = (float ******)in_RCX[0xf];
  piVar16 = (int *)in_RCX[0x13];
  local_350 = in_RCX[0x17];
  local_348 = in_RCX[0x1b];
  local_170 = in_RCX[0x1f];
  puVar17 = (undefined4 *)in_RCX[0x21];
  local_30c = *(undefined4 *)((longlong)in_RCX + 0x114);
  local_320 = (float *****)in_RCX[0x23];
  local_338 = in_RCX[0x27];
  local_330 = in_RCX[0x2b];
  local_328 = in_RCX[0x2f];
  pppppppfVar59 = (float *******)in_RCX[0x36];
  uVar18 = in_RCX[0x3a];
  puVar57 = *(undefined8 **)(in_RDX + 0xc0);
  local_150 = *(longlong *)(in_RDX + 0xe0);
  lVar58 = *(longlong *)(in_RDX + 0x100);
  uVar19 = *(undefined8 *)(in_RDX + 0x120);
  local_340 = *(undefined8 *)(in_RDX + 0x140);
  local_158 = *(longlong **)(in_RDX + 0x160);
  local_358 = *(longlong *)(in_RDX + 0x168);
  puVar38 = in_RDX;
  local_160 = (float ******)in_R8;
  local_140 = in_R9;
  puffin::are_scopes_on();
  local_c0 = (float ******)pppppppfVar34;
  if (extraout_AL == '\0') {
    bVar27 = false;
  }
  else {
    local_138 = (float ******)CONCAT44(local_138._4_4_,iVar30);
    local_e0 = (float ******)pppppppfVar41;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (puVar38 != (undefined1 *)0x0) {
        if (&DAT_00000030 < puVar38) {
          if (puVar38 != &DAT_00000031) goto LAB_140a3f67a;
        }
        else if ((char)puVar38[0x142acdd49] < -0x40) {
LAB_140a3f67a:
                    // WARNING: Subroutine does not return
          core::str::slice_error_fail();
        }
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (puVar38 != (undefined1 *)0xfffffffffffffffe)) {
        if (puVar38 + 2 < (undefined1 *)0x31) {
          if ((char)puVar38[0x142acdd4b] < -0x40) {
LAB_140a3f6ce:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (puVar38 + 2 != (undefined1 *)0x31) goto LAB_140a3f6ce;
      }
    }
    puVar50 = &DAT_142acdf2c;
    do {
      bVar4 = puVar50[-1];
      uVar29 = (uint)(char)bVar4;
      local_318 = puVar57;
      if ((char)bVar4 < '\0') {
        bVar5 = puVar50[-2];
        if ((char)bVar5 < -0x40) {
          bVar6 = puVar50[-3];
          if ((char)bVar6 < -0x40) {
            pbVar1 = puVar50 + -4;
            puVar50 = puVar50 + -4;
            uVar29 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar50 = puVar50 + -3;
            uVar29 = bVar6 & 0xf;
          }
          uVar29 = bVar5 & 0x3f | uVar29 << 6;
        }
        else {
          puVar50 = puVar50 + -2;
          uVar29 = bVar5 & 0x1f;
        }
        uVar29 = (uint)(bVar4 & 0x3f) | uVar29 << 6;
        if (uVar29 != 0x5c) goto LAB_140a3f794;
LAB_140a3f7a5:
        if (puVar50 == &UNK_142acdefa) break;
        if (puVar50 + -0x142acdefa < (undefined *)0x31) {
          if (-0x41 < (char)puVar50[1]) break;
        }
        else if (puVar50 + -0x142acdefa == (undefined *)0x31) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar50 = puVar50 + -1;
      if (uVar29 == 0x5c) goto LAB_140a3f7a5;
LAB_140a3f794:
      if (uVar29 == 0x2f) goto LAB_140a3f7a5;
    } while (puVar50 != &anon_d981ca8783ed9a6875435cd0b1c3191f_26_llvm_15534962235601403176);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar35 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar35 = extraout_RAX_14, extraout_RAX_14 == (longlong *)0x0))
    goto LAB_140a41dff;
    if (*plVar35 != 0) goto LAB_140a41e0b;
    *plVar35 = -1;
    local_2c8 = plVar35;
    puffin::ThreadProfiler::begin_scope();
    *local_2c8 = *local_2c8 + 1;
    bVar27 = true;
    pppppppfVar41 = (float *******)local_e0;
    puVar57 = local_318;
    iVar30 = (int)local_138;
  }
  pppppppfVar46 = (float *******)local_c0;
  *(int *)pppppppfVar41 = iVar30;
  *(float *)(local_c0 + 0x29) = *(float *)(puVar57 + 5) + *(float *)(local_c0 + 0x29);
  pppppppfVar34 = (float *******)(local_c0 + 0x25);
  LOCK();
  bVar66 = *(byte *)(local_c0 + 0x25) == 0;
  if (bVar66) {
    *(byte *)(local_c0 + 0x25) = 1;
  }
  UNLOCK();
  if (!bVar66) {
    parking_lot::raw_mutex::RawMutex::lock_slow();
  }
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  local_e0 = (float ******)pppppppfVar34;
  if (pppppppfVar46[0x26] == (float ******)0x2) {
    std::sync::mpmc::zero::Channel::
    try_recv<country_core::systems::save::save_thread::AutosaveErrorState>();
  }
  else if ((int)pppppppfVar46[0x26] == 1) {
    std::sync::mpmc::list::Channel::
    try_recv<country_core::systems::save::save_thread::AutosaveErrorState>();
  }
  else {
    std::sync::mpmc::array::Channel::
    try_recv<country_core::systems::save::save_thread::AutosaveErrorState>();
  }
  ppppppfVar45 = local_e0;
  if (SBORROW8(0,(longlong)local_420)) {
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    LOCK();
    bVar66 = *(byte *)ppppppfVar45 == 1;
    if (bVar66) {
      *(byte *)ppppppfVar45 = 0;
    }
    UNLOCK();
    if (!bVar66) {
      parking_lot::raw_mutex::RawMutex::unlock_slow();
    }
  }
  else {
    ppppppfVar45 = pppppppfVar46[4];
    local_138 = (float ******)pppppppfVar59;
    if (ppppppfVar45 != (float ******)0x0) {
      ppppppfVar32 = pppppppfVar46[3] + 6;
      do {
        if (ppppppfVar32[-6] != (float *****)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        if (ppppppfVar32[-1] != (float *****)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        ppppppfVar32 = ppppppfVar32 + 8;
        ppppppfVar45 = (float ******)((longlong)ppppppfVar45 - 1);
      } while (ppppppfVar45 != (float ******)0x0);
    }
    if (pppppppfVar46[2] != (float ******)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    ppppppfVar45 = local_e0;
    pppppppfVar46[4] = (float ******)local_410;
    pppppppfVar46[2] = (float ******)local_420;
    pppppppfVar46[3] = (float ******)pppppfStack_418;
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    pppppppfVar46 = (float *******)local_c0;
    pppppppfVar59 = (float *******)local_138;
    LOCK();
    bVar66 = *(byte *)ppppppfVar45 == 1;
    if (bVar66) {
      *(byte *)ppppppfVar45 = 0;
    }
    UNLOCK();
    if (!bVar66) {
      parking_lot::raw_mutex::RawMutex::unlock_slow();
    }
  }
  if ((local_d0[0xb] != local_d0[2]) || ((float ******)local_d0[0xb] != pppppppfVar46[0x24])) {
    std::time::Instant::now();
    local_2b8 = (float ******)local_d0[1];
    ppppppfStack_2b0 = local_2b8 + (longlong)local_d0[2] * 5;
    ppppppfStack_2a8 = (float ******)local_d0[0xb];
    alloc::vec::spec_from_iter::impl$0::
    from_iter<country_core::resources::history::MultiEdit,core::iter::adapters::cloned::Cloned<core::iter::adapters::skip::Skip<core::slice::iter::Iter<country_core::resources::history::MultiEdit>_>_>_>
              ();
    pppppfStack_298 = local_d0[0xb];
    local_2b8 = (float ******)&DAT_00000005;
    std::sync::mpmc::Sender::send<enum2_<country_core::systems::save::save_game::AsyncSaveCmd>_>();
    if ((int)local_128 != 0xb) {
      local_288 = local_f8;
      pcStack_280 = (char *)uStack_f0;
      pppppfStack_298 = local_108;
      ppppppfStack_2a8 = local_118;
      ppppppfStack_2a0 = ppppppfStack_110;
      local_2b8 = local_128;
      ppppppfStack_2b0 = ppppppfStack_120;
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    *(int *)(pppppppfVar46 + 0x29) = 0;
    ppppppfVar45 = (float ******)local_d0[2];
    *(int *)local_148 = (int)local_130;
    local_d0[0xb] = (float *****)ppppppfVar45;
    pppppppfVar46[0x24] = (float ******)local_d0[2];
  }
  camera_rig::MainCamera::store_parameters();
  pppppppfVar34 = (float *******)*in_stack_00000030;
  pppppppfVar41 = (float *******)in_stack_00000030[1];
  bevy_ecs::query::iter::QueryIterationCursor::
  init<tuple_<bevy_ecs::entity::Entity,ref_mut_<utils::components::transform::Transform>,ref_mut_<country_core::resources::critters::sheep::Sheep>,ref_mut_<country_core::resources::critters::CritterGoal>_>,tuple_<>_>
            ();
  ppppppfStack_2a8 = (float ******)(pppppppfVar41 + 0x1f);
  ppppppfStack_2b0 = (float ******)(pppppppfVar41 + 0x34);
  local_2b8 = (float ******)pppppppfVar41;
  ppppppfStack_2a0 = (float ******)pppppppfVar34;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<country_core::resources::critters::sheep::control::SheepSnapshot,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple_<bevy_ecs::entity::Entity,ref_mut_<utils::components::transform::Transform>,ref_mut_<country_core::resources::critters::sheep::Sheep>,ref_mut_<country_core::resources::critters::CritterGoal>_>,tuple_<>_>,country_core::resources::critters::sheep::control::capture_sheep_snapshot::closure_env_0>_>
            ();
  pppppppfVar34 = (float *******)local_c0;
  ppppppfVar45 = local_d0;
  local_138 = ppppppfStack_120;
  local_e0 = local_118;
  lVar39 = *(longlong *)(lVar58 + 0x20);
  if (*(longlong *)(lVar58 + 0x18) == 0) {
    bVar66 = false;
    lVar58 = *local_158;
    *local_158 = lVar39;
    if (lVar39 != lVar58) goto LAB_140a3fbb9;
    lVar39 = local_150;
    if (*(longlong *)(local_150 + 0x10) == 0) {
      lVar39 = 0;
      bVar66 = SBORROW8(0,(longlong)local_d0[6]);
    }
  }
  else {
    *local_158 = lVar39;
LAB_140a3fbb9:
    bVar66 = false;
  }
  local_158 = (longlong *)CONCAT44(local_158._4_4_,(int)CONCAT71((int7)((ulonglong)lVar39 >> 8),1));
  local_168 = local_128;
  if (*(float *)(pppppppfVar46 + 0x29) <= 60.0 && *(float *)(pppppppfVar46 + 0x29) != 60.0) {
    if (*(float *)(puVar57 + 5) < 0.0) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    fVar71 = *(float *)(puVar57 + 5) + *(float *)((longlong)pppppppfVar46 + 0x144);
    *(float *)((longlong)pppppppfVar46 + 0x144) = fVar71;
    if (bVar66) {
      local_150 = CONCAT44(local_150._4_4_,2);
      fVar72 = 0.0;
      if (0.0 <= fVar71) {
        fVar72 = fVar71;
      }
      fVar71 = 8.0;
      if (fVar72 <= 8.0) {
        fVar71 = fVar72;
      }
      *(float *)((longlong)pppppppfVar46 + 0x144) = fVar71;
      if (1.0 <= fVar71) {
        puVar57 = (undefined8 *)(ulonglong)*(uint *)(pppppppfVar46 + 0x28);
        puVar51 = &DAT_142a9f354;
        while( true ) {
          uVar29 = (uint)puVar57;
          fVar71 = fVar71 + -1.0;
          if ((int)((ulonglong)puVar57 / 0x1c20) * 0x1c20 == uVar29) break;
          if ((int)((ulonglong)puVar57 / 0xe10) * 0xe10 == uVar29) {
            uVar53 = 0xe10;
            puVar51 = (undefined4 *)&DAT_142a9f350;
            goto LAB_140a41039;
          }
          if ((int)((ulonglong)puVar57 / 600) * 600 == uVar29) {
            uVar53 = 600;
            puVar51 = (undefined4 *)&DAT_142a9f34c;
            goto LAB_140a41039;
          }
          if ((int)((ulonglong)puVar57 / 300) * 300 == uVar29) {
            uVar53 = 300;
            puVar51 = (undefined4 *)
                      &anon_107183261458dea85466985370debbb6_157_llvm_3641509820080252961;
            goto LAB_140a41039;
          }
          puVar57 = (undefined8 *)(ulonglong)(uVar29 + 1);
          if (fVar71 < 1.0) goto code_r0x000140a3fd27;
        }
        uVar53 = 0x1c20;
LAB_140a41039:
        *(float *)((longlong)local_c0 + 0x144) = fVar71;
        *(uint *)(local_c0 + 0x28) = uVar29 + 1;
        if (uVar29 < uVar53) goto LAB_140a41054;
        puVar48 = puVar57;
        uVar29 = uVar29 - uVar53;
        if (puVar51 + -0x50aa7cd2 < (undefined4 *)0x10) {
          do {
            uVar29 = (int)puVar48 - uVar53;
            puVar56 = puVar51 + -0x50aa7cd2;
            do {
              if (puVar56 == (undefined4 *)0xc) goto LAB_140a410c6;
              puVar2 = (uint *)(&DAT_142a9f34c + (longlong)puVar56);
              if (*puVar2 == 0) {
                    // WARNING: Subroutine does not return
                core::panicking::panic_const::panic_const_rem_by_zero();
              }
              puVar56 = puVar56 + 1;
            } while (uVar29 % *puVar2 != 0);
            puVar48 = (undefined8 *)(ulonglong)uVar29;
          } while (uVar53 <= uVar29);
LAB_140a41054:
          local_158 = (longlong *)((ulonglong)local_158._4_4_ << 0x20);
          local_150 = (ulonglong)local_150._4_4_ << 0x20;
        }
        else {
LAB_140a410c6:
          local_2d4 = uVar29;
          local_150 = CONCAT44(local_150._4_4_,1);
          local_158 = (longlong *)((ulonglong)local_158._4_4_ << 0x20);
        }
      }
      goto LAB_140a3fd42;
    }
  }
  local_150 = CONCAT44(local_150._4_4_,2);
  goto LAB_140a3fd42;
code_r0x000140a3fd27:
  *(float *)((longlong)local_c0 + 0x144) = fVar71;
  *(uint *)(local_c0 + 0x28) = uVar29 + 1;
LAB_140a3fd42:
  ppppppfVar32 = (float ******)local_160[1];
  pppppfVar47 = (float *****)**local_160;
  pppppfVar31 = (float *****)0x0;
  pppppfVar42 = (float *****)((longlong)pppppfVar47 - (longlong)ppppppfVar32[3]);
  if (pppppfVar47 < ppppppfVar32[3]) {
    pppppfVar42 = pppppfVar31;
  }
  pppppfVar55 = (float *****)((longlong)pppppfVar47 - (longlong)ppppppfVar32[7]);
  if (pppppfVar47 < ppppppfVar32[7]) {
    pppppfVar55 = pppppfVar31;
  }
  bVar62 = ppppppfVar32[6] < pppppfVar55;
  pppppfVar55 = (float *****)((longlong)ppppppfVar32[6] - (longlong)pppppfVar55);
  bVar67 = pppppfVar55 == (float *****)0x0;
  if (bVar62) {
    pppppfVar55 = pppppfVar31;
  }
  bVar63 = ppppppfVar32[2] < pppppfVar42;
  pppppfVar42 = (float *****)((longlong)ppppppfVar32[2] - (longlong)pppppfVar42);
  bVar68 = pppppfVar42 == (float *****)0x0;
  if (bVar63) {
    pppppfVar42 = pppppfVar31;
  }
  pppppfVar47 = (float *****)((longlong)pppppfVar42 + (longlong)pppppfVar55);
  if (!bVar63 && !bVar68 || !bVar62 && !bVar67) {
    pppppfVar47 = pppppfVar31;
  }
  **local_160 = (float ****)((longlong)ppppppfVar32[8] - (longlong)pppppfVar47);
  lVar58 = local_140[1];
  uVar43 = *(ulonglong *)*local_140;
  pppppfVar47 = (float *****)(uVar43 - *(ulonglong *)(lVar58 + 0x18));
  if (uVar43 < *(ulonglong *)(lVar58 + 0x18)) {
    pppppfVar47 = pppppfVar31;
  }
  pppppfVar42 = (float *****)(uVar43 - *(ulonglong *)(lVar58 + 0x38));
  if (uVar43 < *(ulonglong *)(lVar58 + 0x38)) {
    pppppfVar42 = pppppfVar31;
  }
  bVar64 = *(float ******)(lVar58 + 0x30) < pppppfVar42;
  pppppfVar42 = (float *****)((longlong)*(float ******)(lVar58 + 0x30) - (longlong)pppppfVar42);
  bVar69 = pppppfVar42 == (float *****)0x0;
  if (bVar64) {
    pppppfVar42 = pppppfVar31;
  }
  bVar65 = *(float ******)(lVar58 + 0x10) < pppppfVar47;
  pppppfVar47 = (float *****)((longlong)*(float ******)(lVar58 + 0x10) - (longlong)pppppfVar47);
  bVar70 = pppppfVar47 == (float *****)0x0;
  if (bVar65) {
    pppppfVar47 = pppppfVar31;
  }
  pppppfVar47 = (float *****)((longlong)pppppfVar47 + (longlong)pppppfVar42);
  if (!bVar65 && !bVar70 || !bVar64 && !bVar69) {
    pppppfVar47 = pppppfVar31;
  }
  *(ulonglong *)*local_140 = *(longlong *)(lVar58 + 0x40) - (longlong)pppppfVar47;
  local_140 = puVar57;
  if ((((byte)local_158 &
       (((bVar65 || bVar70) && (bVar64 || bVar69)) && ((bVar63 || bVar68) && (bVar62 || bVar67))))
       == 0) && (bVar66)) {
    std::time::Instant::now();
    *(int *)local_148 = (int)local_130;
    pppppfStack_298 = local_178[0x1f];
    local_290 = (longlong)pppppfStack_298 + (longlong)local_178[0x20] * 4;
    local_260 = 0;
    ppppppfStack_258 = (float ******)0x0;
    local_270 = 0;
    ppuStack_268 = (undefined **)0x0;
    ppppppfStack_2a8 = ppppppfVar61 + 0x1f;
    ppppppfStack_2b0 = ppppppfVar61 + 0x34;
    ppppppfStack_2a0 = local_178;
    local_288 = 8;
    pcStack_280 = (char *)0x0;
    local_278 = 8;
    local_250 = CONCAT71(local_250._1_7_,*(byte *)(local_178 + 0x21));
    uVar36 = SUB84(&local_2b8,0);
    local_2b8 = ppppppfVar61;
    alloc::vec::spec_from_iter::impl$0::
    from_iter<country_core::resources::roofs::roof_shape::Roof,core::iter::adapters::cloned::Cloned<bevy_ecs::query::iter::QueryIter<ref_<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion>_>_>_>
              ();
    ppppppfVar61 = local_118;
    local_148 = local_128;
    local_130 = ppppppfStack_120;
    local_b1 = 0;
    local_398 = (float ******)pppppppfVar52;
    ppppppfStack_390 = (float ******)pppppppfVar44;
    local_388 = uVar15;
    resources::history::History::current_snapshot();
    ppppppfStack_2b0 = (float ******)CONCAT44(ppppppfStack_2b0._4_4_,uVar36);
    if (extraout_RAX_02 == (float *******)0x0) {
      if (3 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
        local_b1 = 0;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        pppppfStack_298 = (float *****)*extraout_RAX_03;
        local_290 = extraout_RAX_03[1];
        local_288 = 4;
        pcStack_280 = "country_core::systems::save::save_game";
        local_278 = 0x26;
        ppuStack_268 = &PTR_s_Capturing_a_game_snapshot_142ace2b8;
        local_260 = 1;
        ppppppfStack_258 = (float ******)&DAT_00000008;
        local_2b8 = (float ******)0x0;
        local_250 = 0;
        uStack_248 = 0;
        ppppppfStack_2b0 = (float ******)0x142acdf48;
        ppppppfStack_2a8 = (float ******)0x26;
        ppppppfStack_2a0 = (float ******)0x0;
        local_270 = CONCAT44(*(undefined4 *)(extraout_RAX_03 + 2),1);
        local_b1 = 0;
        ::log::__private_api::impl$2::log();
      }
      local_2b8 = local_148;
      ppppppfStack_2b0 = local_130;
      ppppppfStack_2a8 = ppppppfVar61;
      pppppfStack_298 = local_320;
      ppppppfStack_2a0 = (float ******)pppppppfVar60;
      memcpy(&local_290,in_RDX,0xc0);
      local_1d0 = local_338;
      local_1c8 = local_330;
      local_1c0 = local_328;
      local_1b8 = local_350;
      local_1b0 = local_348;
      local_1a8 = local_398;
      ppppppfStack_1a0 = ppppppfStack_390;
      local_198 = local_388;
      local_180 = local_340;
      local_b1 = 1;
      local_190 = uVar18;
      local_188 = uVar19;
      snapshot::capture_game_snapshot_save_data();
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_05 == (float *******)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      *extraout_RAX_05 = (float ******)0x1;
      extraout_RAX_05[1] = (float ******)0x1;
      extraout_RAX_05[2] = (float ******)0x0;
      LOCK();
      ppppppfVar61 = *extraout_RAX_05;
      *extraout_RAX_05 = (float ******)((longlong)*extraout_RAX_05 + 1);
      UNLOCK();
      local_178 = (float ******)extraout_RAX_04;
      local_160 = (float ******)extraout_RAX_05;
      if (*extraout_RAX_05 == (float ******)0x0 ||
          SCARRY8((longlong)ppppppfVar61,1) != (longlong)*extraout_RAX_05 < 0) {
LAB_140a4248a:
                    // WARNING: Does not return
        pcVar24 = (code *)invalidInstructionException();
        (*pcVar24)();
      }
      local_2b8 = (float ******)&DAT_00000009;
      local_b2 = 1;
      local_2d0 = (float ******)extraout_RAX_05;
      ppppppfStack_2b0 = (float ******)extraout_RAX_04;
      ppppppfStack_2a8 = (float ******)extraout_RAX_05;
      std::sync::mpmc::Sender::send<enum2_<country_core::systems::save::save_game::AsyncSaveCmd>_>()
      ;
      if ((int)local_128 != 0xb) {
        local_288 = local_f8;
        pcStack_280 = (char *)uStack_f0;
        pppppfStack_298 = local_108;
        ppppppfStack_2a8 = local_118;
        ppppppfStack_2a0 = ppppppfStack_110;
        local_2b8 = local_128;
        ppppppfStack_2b0 = ppppppfStack_120;
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      local_b2 = 0;
      resources::history::History::set_current_snapshot();
    }
    else {
      LOCK();
      *extraout_RAX_02 = (float ******)((longlong)*extraout_RAX_02 - 1);
      UNLOCK();
      local_2b8 = (float ******)extraout_RAX_02;
      if (*extraout_RAX_02 == (float ******)0x0) {
        local_b1 = 0;
        alloc::sync::Arc::
        drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo>,alloc::alloc::Global>
                  ();
      }
      if ((float *******)local_148 != (float *******)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
  }
  pppppppfVar60 = (float *******)local_e0;
  pppppppfVar41 = (float *******)ppppppfVar45[0xc];
  local_148 = (float ******)(pppppppfVar34 + 5);
  if (SBORROW8(0,(longlong)pppppppfVar34[5])) {
    iVar37 = piVar16[10];
    local_160 = (float ******)CONCAT44(local_160._4_4_,*piVar16);
    ppppppfVar61 = *(float *******)(piVar16 + 1);
    ppppppfVar32 = *(float *******)(piVar16 + 3);
    ppppppfVar49 = *(float *******)(piVar16 + 5);
    ppppppfVar40 = *(float *******)(piVar16 + 7);
    iVar30 = piVar16[9];
    local_178 = (float ******)pppppppfVar41;
    if (SBORROW8(0,(longlong)local_168)) {
      pppppppfVar60 = (float *******)&DAT_8000000000000000;
    }
    else {
      auVar21._8_8_ = 0;
      auVar21._0_8_ = local_e0;
      uVar43 = SUB168(auVar21 * ZEXT816(0x28),0);
      if ((SUB168(auVar21 * ZEXT816(0x28),8) != 0) || (0x7ffffffffffffffc < uVar43)) {
LAB_140a4215a:
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      if (uVar43 == 0) {
        local_2c0 = (float *****)0x4;
      }
      else {
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_07 == (float ******)0x0) goto LAB_140a4215a;
        local_2c0 = (float *****)extraout_RAX_07;
        if (pppppppfVar60 != (float *******)0x0) {
          uVar54 = 0;
          pppppppfVar34 = pppppppfVar60;
          pppppppfVar41 = (float *******)local_138;
          do {
            if (uVar43 == uVar54) break;
            if (*(int *)pppppppfVar41 == 1) {
              ppppppfStack_2b0 =
                   (float ******)
                   CONCAT44(ppppppfStack_2b0._4_4_,
                            *(undefined4 *)((longlong)local_138 + uVar54 + 0xc));
              local_2b8 = *(float *******)((longlong)local_138 + uVar54 + 4);
            }
            pppppppfVar34 = (float *******)((longlong)pppppppfVar34 + -1);
            ppppppfStack_120 = (float ******)CONCAT44(ppppppfStack_120._4_4_,ppppppfStack_2b0._0_4_)
            ;
            local_128 = local_2b8;
            *(uint *)((longlong)extraout_RAX_07 + uVar54) = (uint)(*(int *)pppppppfVar41 == 1);
            *(undefined4 *)((longlong)extraout_RAX_07 + uVar54 + 0xc) = ppppppfStack_2b0._0_4_;
            *(float *******)((longlong)extraout_RAX_07 + uVar54 + 4) = local_2b8;
            *(float *******)((longlong)extraout_RAX_07 + uVar54 + 0x10) = pppppppfVar41[2];
            *(int *)((longlong)extraout_RAX_07 + uVar54 + 0x18) = *(int *)(pppppppfVar41 + 3);
            *(undefined8 *)((longlong)extraout_RAX_07 + uVar54 + 0x1c) =
                 *(undefined8 *)((longlong)pppppppfVar41 + 0x1c);
            pbVar1 = (byte *)((longlong)pppppppfVar41 + 0x24);
            pppppppfVar41 = pppppppfVar41 + 5;
            *(int *)((longlong)extraout_RAX_07 + uVar54 + 0x24) = *(int *)pbVar1;
            uVar54 = uVar54 + 0x28;
          } while (pppppppfVar34 != (float *******)0x0);
          goto LAB_140a4093c;
        }
      }
      pppppppfVar60 = (float *******)0x0;
    }
LAB_140a4093c:
    pppppppfVar34 = (float *******)ppppppfVar45[5];
    uVar43 = (longlong)pppppppfVar34 << 4;
    local_130 = (float ******)pppppppfVar60;
    if (((ulonglong)pppppppfVar34 >> 0x3c != 0) || (0x7ffffffffffffff8 < uVar43)) {
LAB_140a41e28:
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    local_2f0 = *(int *)(pppppppfVar59 + 3);
    local_2f8 = *(int *)((longlong)pppppppfVar59 + 0x1c);
    local_300 = *(int *)(pppppppfVar59 + 4);
    local_330 = CONCAT71(local_330._1_7_,*(byte *)((longlong)pppppppfVar59 + 0x42));
    local_328 = CONCAT71(local_328._1_7_,*(byte *)(pppppppfVar59 + 8));
    local_2f4 = *(int *)((longlong)pppppppfVar59 + 0x24);
    local_2fc = *(int *)(pppppppfVar59 + 5);
    local_304 = *(int *)((longlong)pppppppfVar59 + 0x2c);
    local_2c8 = (longlong *)CONCAT44(local_2c8._4_4_,*(int *)(pppppppfVar59 + 6));
    local_318 = (undefined8 *)CONCAT44(local_318._4_4_,*(int *)((longlong)pppppppfVar59 + 0x34));
    local_2e4 = *(int *)pppppppfVar59;
    local_2e8 = *(int *)((longlong)pppppppfVar59 + 4);
    local_320 = (float *****)CONCAT71(local_320._1_7_,*(byte *)((longlong)pppppppfVar59 + 0x41));
    local_308 = *(int *)((longlong)pppppppfVar59 + 0x3c);
    local_d1 = *(byte *)((longlong)pppppppfVar59 + 0x14);
    local_2ec = *(int *)(pppppppfVar59 + 7);
    local_2d8 = *(int *)(pppppppfVar59 + 1);
    local_2dc = *(int *)((longlong)pppppppfVar59 + 0xc);
    local_2e0 = *(int *)(pppppppfVar59 + 2);
    local_338 = CONCAT44(local_338._4_4_,*(int *)(ppppppfVar45 + 0xd));
    local_350 = CONCAT62(local_350._2_6_,(short)iVar37);
    local_348 = CONCAT44(local_348._4_4_,iVar30);
    local_340 = CONCAT44(local_340._4_4_,*(int *)((longlong)ppppppfVar45 + 0x6c));
    local_d0 = (float ******)pppppppfVar34;
    if (uVar43 == 0) {
      local_3a0 = (float *****)0x8;
      ppppppfVar45 = (float ******)*local_148;
      lVar58 = local_170;
    }
    else {
      ppppppfVar45 = (float ******)ppppppfVar45[4];
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_11 == (float ******)0x0) goto LAB_140a41e28;
      lVar58 = local_170;
      local_3a0 = (float *****)extraout_RAX_11;
      if ((float *******)local_d0 == (float *******)0x0) {
        ppppppfVar45 = (float ******)*local_148;
      }
      else {
        uVar54 = 0;
        pppppppfVar34 = (float *******)local_d0;
        do {
          if (uVar43 == uVar54) break;
          pppppfVar47 = *ppppppfVar45;
          uVar36 = *(undefined4 *)(ppppppfVar45 + 1);
          LOCK();
          ppppfVar3 = *pppppfVar47;
          *pppppfVar47 = (float ****)((longlong)*pppppfVar47 + 1);
          UNLOCK();
          if (*pppppfVar47 == (float ****)0x0 ||
              SCARRY8((longlong)ppppfVar3,1) != (longlong)*pppppfVar47 < 0) goto LAB_140a4248a;
          pppppppfVar34 = (float *******)((longlong)pppppppfVar34 + -1);
          ppppppfVar45 = ppppppfVar45 + 2;
          *(float ******)((longlong)extraout_RAX_11 + uVar54) = pppppfVar47;
          *(undefined4 *)((longlong)extraout_RAX_11 + uVar54 + 8) = uVar36;
          uVar54 = uVar54 + 0x10;
        } while (pppppppfVar34 != (float *******)0x0);
        ppppppfVar45 = (float ******)*local_148;
      }
    }
    local_170 = lVar58;
    if (!SBORROW8(0,(longlong)ppppppfVar45)) {
      if ((!SBORROW8(0,(longlong)local_c0[8])) && ((float ******)local_c0[8] != (float ******)0x0))
      {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      core::ptr::drop_in_place<country_core::resources::history::HistoryChainSnapshots>();
    }
    local_c0[5] = (float *****)local_d0;
    local_c0[6] = local_3a0;
    local_c0[7] = (float *****)local_d0;
    local_c0[8] = (float *****)local_130;
    local_c0[9] = local_2c0;
    local_c0[10] = (float *****)local_e0;
    local_c0[0xb] = (float *****)local_178;
    *(int *)(local_c0 + 0xc) = local_2e4;
    *(int *)((longlong)local_c0 + 100) = local_2e8;
    *(int *)(local_c0 + 0xd) = local_2d8;
    *(int *)((longlong)local_c0 + 0x6c) = local_2dc;
    *(int *)(local_c0 + 0xe) = local_2e0;
    *(byte *)((longlong)local_c0 + 0x74) = local_d1;
    *(int *)(local_c0 + 0xf) = local_2f0;
    *(int *)((longlong)local_c0 + 0x7c) = local_2f8;
    *(int *)(local_c0 + 0x10) = local_300;
    *(int *)((longlong)local_c0 + 0x84) = local_2f4;
    *(int *)(local_c0 + 0x11) = local_2fc;
    *(int *)((longlong)local_c0 + 0x8c) = local_304;
    *(int *)(local_c0 + 0x12) = (int)local_2c8;
    *(int *)((longlong)local_c0 + 0x94) = (int)local_318;
    *(int *)(local_c0 + 0x13) = local_2ec;
    *(int *)((longlong)local_c0 + 0x9c) = local_308;
    *(byte *)(local_c0 + 0x14) = (byte)local_328;
    *(byte *)((longlong)local_c0 + 0xa1) = (byte)local_320;
    *(byte *)((longlong)local_c0 + 0xa2) = (byte)local_330;
    *(int *)((longlong)local_c0 + 0xa4) = (int)local_160;
    local_c0[0x15] = (float *****)ppppppfVar61;
    local_c0[0x16] = (float *****)ppppppfVar32;
    local_c0[0x17] = (float *****)ppppppfVar49;
    local_c0[0x18] = (float *****)ppppppfVar40;
    *(int *)(local_c0 + 0x19) = (int)local_348;
    *(undefined2 *)((longlong)local_c0 + 0xcc) = (undefined2)local_350;
    local_c0[0x1a] = (float *****)CONCAT44(fStack_37c,local_380);
    *(float *)(local_c0 + 0x1c) = local_370;
    local_c0[0x1b] = (float *****)CONCAT44(fStack_374,fStack_378);
    *(ulonglong *)((longlong)local_c0 + 0xe4) = CONCAT44(fStack_368,fStack_36c);
    *(ulonglong *)((longlong)local_c0 + 0xec) = CONCAT44(fStack_360,fStack_364);
    *(int *)((longlong)local_c0 + 0xf4) = (int)local_338;
    *(int *)(local_c0 + 0x1f) = (int)local_340;
  }
  else {
    if ((float *******)pppppppfVar34[0xb] != pppppppfVar41) {
      pppppppfVar34[0xb] = (float ******)pppppppfVar41;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_06 == (float *******)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      *extraout_RAX_06 = (float ******)pppppppfVar41;
      ppppppfStack_2a0 = (float ******)CONCAT71(ppppppfStack_2a0._1_7_,1);
      ppppppfStack_2a8 = (float ******)&DAT_142acdf70;
      local_2b8 = (float ******)&DAT_00000006;
      ppppppfStack_2b0 = (float ******)extraout_RAX_06;
      std::sync::mpmc::Sender::send<enum2_<country_core::systems::save::save_game::AsyncSaveCmd>_>()
      ;
      if ((int)local_128 != 0xb) {
        local_288 = local_f8;
        pcStack_280 = (char *)uStack_f0;
        pppppfStack_298 = local_108;
        ppppppfStack_2a8 = local_118;
        ppppppfStack_2a0 = ppppppfStack_110;
        local_2b8 = local_128;
        ppppppfStack_2b0 = ppppppfStack_120;
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      *(int *)(pppppppfVar34 + 0x29) = 0;
    }
    if ((((((*(float *)(pppppppfVar34 + 0x1a) != local_380) ||
           (NAN(*(float *)(pppppppfVar34 + 0x1a)) || NAN(local_380))) ||
          (*(float *)((longlong)pppppppfVar34 + 0xd4) != fStack_37c)) ||
         ((NAN(*(float *)((longlong)pppppppfVar34 + 0xd4)) || NAN(fStack_37c) ||
          (*(float *)(pppppppfVar34 + 0x1b) != fStack_378)))) ||
        (((NAN(*(float *)(pppppppfVar34 + 0x1b)) || NAN(fStack_378) ||
          ((*(float *)((longlong)pppppppfVar34 + 0xdc) != fStack_374 ||
           (NAN(*(float *)((longlong)pppppppfVar34 + 0xdc)) || NAN(fStack_374))))) ||
         (*(float *)(pppppppfVar34 + 0x1c) != local_370)))) ||
       (((((NAN(*(float *)(pppppppfVar34 + 0x1c)) || NAN(local_370) ||
           (*(float *)((longlong)pppppppfVar34 + 0xe4) != fStack_36c)) ||
          (NAN(*(float *)((longlong)pppppppfVar34 + 0xe4)) || NAN(fStack_36c))) ||
         (((*(float *)(pppppppfVar34 + 0x1d) != fStack_368 ||
           (NAN(*(float *)(pppppppfVar34 + 0x1d)) || NAN(fStack_368))) ||
          ((*(float *)((longlong)pppppppfVar34 + 0xec) != fStack_364 ||
           ((NAN(*(float *)((longlong)pppppppfVar34 + 0xec)) || NAN(fStack_364) ||
            (*(float *)(pppppppfVar34 + 0x1e) != fStack_360)))))))) ||
        (NAN(*(float *)(pppppppfVar34 + 0x1e)) || NAN(fStack_360))))) {
      *(float *)(pppppppfVar34 + 0x1a) = local_380;
      *(float *)((longlong)pppppppfVar34 + 0xd4) = fStack_37c;
      pppppppfVar34[0x1b] = (float ******)CONCAT44(fStack_374,fStack_378);
      *(float *)(pppppppfVar34 + 0x1c) = local_370;
      *(ulonglong *)((longlong)pppppppfVar34 + 0xe4) = CONCAT44(fStack_368,fStack_36c);
      *(ulonglong *)((longlong)pppppppfVar34 + 0xec) = CONCAT44(fStack_360,fStack_364);
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_08 == (float *******)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      *(float *)extraout_RAX_08 = local_380;
      *(float *)((longlong)extraout_RAX_08 + 4) = fStack_37c;
      extraout_RAX_08[1] = (float ******)CONCAT44(fStack_374,fStack_378);
      *(float *)(extraout_RAX_08 + 2) = local_370;
      *(ulonglong *)((longlong)extraout_RAX_08 + 0x14) = CONCAT44(fStack_368,fStack_36c);
      *(ulonglong *)((longlong)extraout_RAX_08 + 0x1c) = CONCAT44(fStack_360,fStack_364);
      ppppppfStack_2a0 = (float ******)CONCAT71(ppppppfStack_2a0._1_7_,3);
      ppppppfStack_2a8 = (float ******)&DAT_142acdfa8;
      local_2b8 = (float ******)&DAT_00000006;
      ppppppfStack_2b0 = (float ******)extraout_RAX_08;
      std::sync::mpmc::Sender::send<enum2_<country_core::systems::save::save_game::AsyncSaveCmd>_>()
      ;
      if ((int)local_128 != 0xb) {
        local_288 = local_f8;
        pcStack_280 = (char *)uStack_f0;
        pppppfStack_298 = local_108;
        ppppppfStack_2a8 = local_118;
        ppppppfStack_2a0 = ppppppfStack_110;
        local_2b8 = local_128;
        ppppppfStack_2b0 = ppppppfStack_120;
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
    }
    fVar71 = (float)piVar16[2];
    if ((*(float *)((longlong)pppppppfVar34 + 0xac) != fVar71) ||
       (NAN(*(float *)((longlong)pppppppfVar34 + 0xac)) || NAN(fVar71))) {
      fVar72 = (float)piVar16[3];
LAB_140a406bd:
      iVar37 = piVar16[10];
      iVar30 = piVar16[1];
      uVar18 = *(undefined8 *)(piVar16 + 4);
      uVar19 = *(undefined8 *)(piVar16 + 6);
      uVar15 = *(undefined8 *)(piVar16 + 8);
      *(int *)((longlong)pppppppfVar34 + 0xa4) = *piVar16;
      *(int *)(pppppppfVar34 + 0x15) = iVar30;
      *(float *)((longlong)pppppppfVar34 + 0xac) = fVar71;
      *(float *)(pppppppfVar34 + 0x16) = fVar72;
      *(undefined8 *)((longlong)pppppppfVar34 + 0xb4) = uVar18;
      *(undefined8 *)((longlong)pppppppfVar34 + 0xbc) = uVar19;
      *(undefined8 *)((longlong)pppppppfVar34 + 0xc4) = uVar15;
      *(short *)((longlong)pppppppfVar34 + 0xcc) = (short)iVar37;
      uVar10 = *(ushort *)(piVar16 + 10);
      pppppppfVar41 = (float *******)(ulonglong)uVar10;
      iVar37 = *piVar16;
      uVar15 = *(undefined8 *)(piVar16 + 1);
      uVar18 = *(undefined8 *)(piVar16 + 3);
      uVar19 = *(undefined8 *)(piVar16 + 5);
      uVar25 = *(undefined8 *)(piVar16 + 7);
      iVar30 = piVar16[9];
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_09 == (float *******)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      *(int *)extraout_RAX_09 = iVar37;
      *(undefined8 *)((longlong)extraout_RAX_09 + 4) = uVar15;
      *(undefined8 *)((longlong)extraout_RAX_09 + 0xc) = uVar18;
      *(undefined8 *)((longlong)extraout_RAX_09 + 0x14) = uVar19;
      *(undefined8 *)((longlong)extraout_RAX_09 + 0x1c) = uVar25;
      *(int *)((longlong)extraout_RAX_09 + 0x24) = iVar30;
      *(ushort *)(extraout_RAX_09 + 5) = uVar10;
      ppppppfStack_2a0 = (float ******)CONCAT71(ppppppfStack_2a0._1_7_,4);
      ppppppfStack_2a8 = (float ******)&DAT_142acdfe0;
      local_2b8 = (float ******)&DAT_00000006;
      ppppppfStack_2b0 = (float ******)extraout_RAX_09;
      std::sync::mpmc::Sender::send<enum2_<country_core::systems::save::save_game::AsyncSaveCmd>_>()
      ;
      if ((int)local_128 != 0xb) {
        local_288 = local_f8;
        pcStack_280 = (char *)uStack_f0;
        pppppfStack_298 = local_108;
        ppppppfStack_2a8 = local_118;
        ppppppfStack_2a0 = ppppppfStack_110;
        local_2b8 = local_128;
        ppppppfStack_2b0 = ppppppfStack_120;
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
    }
    else {
      fVar72 = (float)piVar16[3];
      if ((((((*(float *)(pppppppfVar34 + 0x16) != fVar72) ||
             (NAN(*(float *)(pppppppfVar34 + 0x16)) || NAN(fVar72))) ||
            (*(float *)((longlong)pppppppfVar34 + 0xb4) != (float)piVar16[4])) ||
           (((((NAN(*(float *)((longlong)pppppppfVar34 + 0xb4)) || NAN((float)piVar16[4]) ||
               (*(short *)((longlong)pppppppfVar34 + 0xcc) != (short)piVar16[10])) ||
              (*(int *)((longlong)pppppppfVar34 + 0xa4) != *piVar16)) ||
             ((*(int *)((longlong)pppppppfVar34 + 0xa4) != 0 &&
              ((*(float *)(pppppppfVar34 + 0x15) != (float)piVar16[1] ||
               (NAN(*(float *)(pppppppfVar34 + 0x15)) || NAN((float)piVar16[1]))))))) ||
            (*(float *)(pppppppfVar34 + 0x17) != (float)piVar16[5])))) ||
          (((((NAN(*(float *)(pppppppfVar34 + 0x17)) || NAN((float)piVar16[5]) ||
              (*(float *)((longlong)pppppppfVar34 + 0xbc) != (float)piVar16[6])) ||
             (NAN(*(float *)((longlong)pppppppfVar34 + 0xbc)) || NAN((float)piVar16[6]))) ||
            ((*(float *)(pppppppfVar34 + 0x18) != (float)piVar16[7] ||
             (NAN(*(float *)(pppppppfVar34 + 0x18)) || NAN((float)piVar16[7]))))) ||
           (*(float *)((longlong)pppppppfVar34 + 0xc4) != (float)piVar16[8])))) ||
         (((NAN(*(float *)((longlong)pppppppfVar34 + 0xc4)) || NAN((float)piVar16[8]) ||
           (*(float *)(pppppppfVar34 + 0x19) != (float)piVar16[9])) ||
          (NAN(*(float *)(pppppppfVar34 + 0x19)) || NAN((float)piVar16[9]))))) goto LAB_140a406bd;
    }
    if (SBORROW8(0,(longlong)local_168) || SBORROW8(0,(longlong)pppppppfVar34[8])) {
      cVar28 = SBORROW8(0,(longlong)local_168) && SBORROW8(0,(longlong)pppppppfVar34[8]);
    }
    else {
      core::slice::cmp::impl$4::
      equal<country_core::resources::critters::sheep::control::SheepSnapshot,country_core::resources::critters::sheep::control::SheepSnapshot>
                ();
      cVar28 = extraout_AL_00;
    }
    pppppppfVar60 = (float *******)local_e0;
    if (cVar28 == '\0') {
      if (SBORROW8(0,(longlong)local_168)) {
        pppppppfVar60 = (float *******)&DAT_8000000000000000;
      }
      else {
        auVar22._8_8_ = 0;
        auVar22._0_8_ = local_e0;
        uVar43 = SUB168(auVar22 * ZEXT816(0x28),0);
        if ((SUB168(auVar22 * ZEXT816(0x28),8) != 0) || (0x7ffffffffffffffc < uVar43)) {
LAB_140a423eb:
                    // WARNING: Subroutine does not return
          alloc::raw_vec::handle_error();
        }
        if (uVar43 == 0) {
          pppppppfVar41 = (float *******)&DAT_00000004;
        }
        else {
          std::alloc::__default_lib_allocator::__rust_alloc();
          if (extraout_RAX_10 == (float *******)0x0) goto LAB_140a423eb;
          pppppppfVar41 = extraout_RAX_10;
          if (pppppppfVar60 != (float *******)0x0) {
            uVar54 = 0;
            pppppppfVar34 = pppppppfVar60;
            pppppppfVar52 = (float *******)local_138;
            do {
              if (uVar43 == uVar54) break;
              if (*(int *)pppppppfVar52 == 1) {
                ppppppfStack_2b0 =
                     (float ******)
                     CONCAT44(ppppppfStack_2b0._4_4_,
                              *(undefined4 *)((longlong)local_138 + uVar54 + 0xc));
                local_2b8 = *(float *******)((longlong)local_138 + uVar54 + 4);
              }
              pppppppfVar34 = (float *******)((longlong)pppppppfVar34 + -1);
              ppppppfStack_120 =
                   (float ******)CONCAT44(ppppppfStack_120._4_4_,ppppppfStack_2b0._0_4_);
              local_128 = local_2b8;
              *(uint *)((longlong)extraout_RAX_10 + uVar54) = (uint)(*(int *)pppppppfVar52 == 1);
              *(undefined4 *)((longlong)extraout_RAX_10 + uVar54 + 0xc) = ppppppfStack_2b0._0_4_;
              *(float *******)((longlong)extraout_RAX_10 + uVar54 + 4) = local_2b8;
              *(float *******)((longlong)extraout_RAX_10 + uVar54 + 0x10) = pppppppfVar52[2];
              *(int *)((longlong)extraout_RAX_10 + uVar54 + 0x18) = *(int *)(pppppppfVar52 + 3);
              *(undefined8 *)((longlong)extraout_RAX_10 + uVar54 + 0x1c) =
                   *(undefined8 *)((longlong)pppppppfVar52 + 0x1c);
              pbVar1 = (byte *)((longlong)pppppppfVar52 + 0x24);
              pppppppfVar52 = pppppppfVar52 + 5;
              *(int *)((longlong)extraout_RAX_10 + uVar54 + 0x24) = *(int *)pbVar1;
              uVar54 = uVar54 + 0x28;
            } while (pppppppfVar34 != (float *******)0x0);
            goto LAB_140a40d1a;
          }
        }
        pppppppfVar60 = (float *******)0x0;
      }
LAB_140a40d1a:
      pppppppfVar34 = (float *******)local_c0;
      if ((!SBORROW8(0,(longlong)local_c0[8])) && ((float ******)local_c0[8] != (float ******)0x0))
      {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      pppppppfVar34[8] = (float ******)pppppppfVar60;
      pppppppfVar34[9] = (float ******)pppppppfVar41;
      pppppppfVar34[10] = local_e0;
      if (SBORROW8(0,(longlong)local_168)) {
        pppppppfVar41 = pppppppfVar34;
        pppppppfVar44 = (float *******)&DAT_8000000000000000;
      }
      else {
        auVar23._8_8_ = 0;
        auVar23._0_8_ = local_e0;
        uVar43 = SUB168(auVar23 * ZEXT816(0x28),0);
        if ((SUB168(auVar23 * ZEXT816(0x28),8) != 0) || (0x7ffffffffffffffc < uVar43)) {
LAB_140a42402:
                    // WARNING: Subroutine does not return
          alloc::raw_vec::handle_error();
        }
        if (uVar43 == 0) {
          pppppppfVar41 = (float *******)&DAT_00000004;
        }
        else {
          std::alloc::__default_lib_allocator::__rust_alloc();
          if (extraout_RAX_12 == (float *******)0x0) goto LAB_140a42402;
          pppppppfVar41 = extraout_RAX_12;
          if ((float *******)local_e0 != (float *******)0x0) {
            uVar54 = 0;
            pppppppfVar60 = (float *******)local_e0;
            pppppppfVar52 = (float *******)local_138;
            do {
              pppppppfVar44 = (float *******)local_e0;
              if (uVar43 == uVar54) break;
              if (*(int *)pppppppfVar52 == 1) {
                ppppppfStack_2b0 =
                     (float ******)
                     CONCAT44(ppppppfStack_2b0._4_4_,
                              *(undefined4 *)((longlong)local_138 + uVar54 + 0xc));
                local_2b8 = *(float *******)((longlong)local_138 + uVar54 + 4);
              }
              pppppppfVar60 = (float *******)((longlong)pppppppfVar60 + -1);
              ppppppfStack_120 =
                   (float ******)CONCAT44(ppppppfStack_120._4_4_,ppppppfStack_2b0._0_4_);
              local_128 = local_2b8;
              *(uint *)((longlong)extraout_RAX_12 + uVar54) = (uint)(*(int *)pppppppfVar52 == 1);
              *(undefined4 *)((longlong)extraout_RAX_12 + uVar54 + 0xc) = ppppppfStack_2b0._0_4_;
              *(float *******)((longlong)extraout_RAX_12 + uVar54 + 4) = local_2b8;
              *(float *******)((longlong)extraout_RAX_12 + uVar54 + 0x10) = pppppppfVar52[2];
              *(int *)((longlong)extraout_RAX_12 + uVar54 + 0x18) = *(int *)(pppppppfVar52 + 3);
              *(undefined8 *)((longlong)extraout_RAX_12 + uVar54 + 0x1c) =
                   *(undefined8 *)((longlong)pppppppfVar52 + 0x1c);
              pbVar1 = (byte *)((longlong)pppppppfVar52 + 0x24);
              pppppppfVar52 = pppppppfVar52 + 5;
              *(int *)((longlong)extraout_RAX_12 + uVar54 + 0x24) = *(int *)pbVar1;
              uVar54 = uVar54 + 0x28;
            } while (pppppppfVar60 != (float *******)0x0);
            goto LAB_140a40e9c;
          }
        }
        pppppppfVar44 = (float *******)0x0;
      }
LAB_140a40e9c:
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_13 == (float *******)0x0) {
        local_d0 = (float ******)pppppppfVar41;
        local_c0 = (float ******)pppppppfVar44;
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      *extraout_RAX_13 = (float ******)pppppppfVar44;
      extraout_RAX_13[1] = (float ******)pppppppfVar41;
      extraout_RAX_13[2] = local_e0;
      ppppppfStack_2a0 = (float ******)CONCAT71(ppppppfStack_2a0._1_7_,5);
      ppppppfStack_2a8 =
           (float ******)
           &
           PTR_drop_in_place<enum2_<core::option::Option<country_core::resources::critters::sheep::control::SheepControlSnapshot>_>_>_142ace018
      ;
      local_2b8 = (float ******)&DAT_00000006;
      ppppppfStack_2b0 = (float ******)extraout_RAX_13;
      std::sync::mpmc::Sender::send<enum2_<country_core::systems::save::save_game::AsyncSaveCmd>_>()
      ;
      if ((int)local_128 != 0xb) {
        local_288 = local_f8;
        pcStack_280 = (char *)uStack_f0;
        pppppfStack_298 = local_108;
        ppppppfStack_2a8 = local_118;
        ppppppfStack_2a0 = ppppppfStack_110;
        local_2b8 = local_128;
        ppppppfStack_2b0 = ppppppfStack_120;
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
    }
    iVar30 = *(int *)(pppppppfVar59 + 3);
    pppppppfVar41 = pppppppfVar34;
    if (*(int *)(pppppppfVar34 + 0xf) == iVar30) {
      iVar12 = *(int *)((longlong)pppppppfVar34 + 0x7c);
      iVar37 = *(int *)((longlong)pppppppfVar59 + 0x1c);
      if (((((iVar12 != *(int *)((longlong)pppppppfVar59 + 0x1c)) ||
            (iVar37 = iVar12, *(int *)(pppppppfVar34 + 0x10) != *(int *)(pppppppfVar59 + 4))) ||
           (*(byte *)((longlong)pppppppfVar34 + 0xa2) != *(byte *)((longlong)pppppppfVar59 + 0x42)))
          || (((*(byte *)(pppppppfVar34 + 0x14) != *(byte *)(pppppppfVar59 + 8) ||
               (*(int *)((longlong)pppppppfVar34 + 0x84) != *(int *)((longlong)pppppppfVar59 + 0x24)
               )) || ((*(int *)(pppppppfVar34 + 0x11) != *(int *)(pppppppfVar59 + 5) ||
                      ((*(int *)((longlong)pppppppfVar34 + 0x8c) !=
                        *(int *)((longlong)pppppppfVar59 + 0x2c) ||
                       (*(int *)(pppppppfVar34 + 0x12) != *(int *)(pppppppfVar59 + 6))))))))) ||
         (*(int *)((longlong)pppppppfVar34 + 0x94) != *(int *)((longlong)pppppppfVar59 + 0x34)))
      goto LAB_140a411cc;
      if (((ulonglong)pppppppfVar34[0xc] & 1) == 0) {
        if ((*(byte *)pppppppfVar59 & 1) == 0) goto LAB_140a41127;
        goto LAB_140a411cc;
      }
      if ((*(byte *)pppppppfVar59 &
          *(int *)((longlong)pppppppfVar34 + 100) == *(int *)((longlong)pppppppfVar59 + 4)) != 1)
      goto LAB_140a411cc;
LAB_140a41127:
      if ((((*(int *)(pppppppfVar34 + 0x13) != *(int *)(pppppppfVar59 + 7)) ||
           (*(byte *)((longlong)pppppppfVar34 + 0xa1) != *(byte *)((longlong)pppppppfVar59 + 0x41)))
          || (pppppppfVar41 = (float *******)local_c0,
             *(float *)((longlong)pppppppfVar34 + 0x9c) !=
             *(float *)((longlong)pppppppfVar59 + 0x3c))) ||
         (((NAN(*(float *)((longlong)pppppppfVar34 + 0x9c)) ||
            NAN(*(float *)((longlong)pppppppfVar59 + 0x3c)) ||
           (*(int *)(local_c0 + 0xd) != *(int *)(pppppppfVar59 + 1))) ||
          ((*(int *)((longlong)local_c0 + 0x6c) != *(int *)((longlong)pppppppfVar59 + 0xc) ||
           ((*(byte *)((longlong)local_c0 + 0x74) != *(byte *)((longlong)pppppppfVar59 + 0x14) ||
            (*(int *)(local_c0 + 0xe) != *(int *)(pppppppfVar59 + 2))))))))) goto LAB_140a411cc;
    }
    else {
      iVar37 = *(int *)((longlong)pppppppfVar59 + 0x1c);
LAB_140a411cc:
      bVar4 = *(byte *)((longlong)pppppppfVar59 + 0x42);
      uVar11 = *(undefined2 *)(pppppppfVar59 + 8);
      iVar13 = *(int *)(pppppppfVar59 + 7);
      iVar12 = *(int *)((longlong)pppppppfVar59 + 0x3c);
      bVar5 = *(byte *)((longlong)pppppppfVar59 + 0x14);
      iVar14 = *(int *)(pppppppfVar59 + 2);
      ppppppfVar45 = pppppppfVar59[4];
      ppppppfVar32 = pppppppfVar59[5];
      ppppppfVar61 = pppppppfVar59[6];
      ppppppfVar49 = pppppppfVar59[1];
      pppppppfVar41[0xc] = *pppppppfVar59;
      pppppppfVar41[0xd] = ppppppfVar49;
      *(int *)(pppppppfVar41 + 0xe) = iVar14;
      *(byte *)((longlong)pppppppfVar41 + 0x74) = bVar5;
      *(int *)(pppppppfVar41 + 0xf) = iVar30;
      *(int *)((longlong)pppppppfVar41 + 0x7c) = iVar37;
      pppppppfVar41[0x10] = ppppppfVar45;
      pppppppfVar41[0x11] = ppppppfVar32;
      pppppppfVar41[0x12] = ppppppfVar61;
      *(int *)(pppppppfVar41 + 0x13) = iVar13;
      *(int *)((longlong)pppppppfVar41 + 0x9c) = iVar12;
      *(undefined2 *)(pppppppfVar41 + 0x14) = uVar11;
      *(byte *)((longlong)pppppppfVar41 + 0xa2) = bVar4;
      bVar4 = *(byte *)((longlong)pppppppfVar59 + 0x42);
      bVar5 = *(byte *)(pppppppfVar59 + 8);
      ppppppfVar61 = pppppppfVar59[3];
      ppppppfVar45 = pppppppfVar59[4];
      ppppppfVar32 = pppppppfVar59[5];
      ppppppfVar49 = pppppppfVar59[6];
      iVar37 = *(int *)(pppppppfVar59 + 7);
      bVar6 = *(byte *)((longlong)pppppppfVar59 + 0x41);
      iVar30 = *(int *)((longlong)pppppppfVar59 + 0x3c);
      ppppppfVar40 = *pppppppfVar59;
      ppppppfVar26 = pppppppfVar59[1];
      bVar7 = *(byte *)((longlong)pppppppfVar59 + 0x14);
      iVar12 = *(int *)(pppppppfVar59 + 2);
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_15 == (float *******)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      *extraout_RAX_15 = ppppppfVar40;
      extraout_RAX_15[1] = ppppppfVar26;
      *(int *)(extraout_RAX_15 + 2) = iVar12;
      *(byte *)((longlong)extraout_RAX_15 + 0x14) = bVar7;
      extraout_RAX_15[3] = ppppppfVar61;
      extraout_RAX_15[4] = ppppppfVar45;
      extraout_RAX_15[5] = ppppppfVar32;
      extraout_RAX_15[6] = ppppppfVar49;
      *(int *)(extraout_RAX_15 + 7) = iVar37;
      *(int *)((longlong)extraout_RAX_15 + 0x3c) = iVar30;
      *(byte *)(extraout_RAX_15 + 8) = bVar5;
      *(byte *)((longlong)extraout_RAX_15 + 0x41) = bVar6;
      *(byte *)((longlong)extraout_RAX_15 + 0x42) = bVar4;
      ppppppfStack_2a0 = (float ******)CONCAT71(ppppppfStack_2a0._1_7_,6);
      ppppppfStack_2a8 = (float ******)&DAT_142ace050;
      local_2b8 = (float ******)&DAT_00000006;
      ppppppfStack_2b0 = (float ******)extraout_RAX_15;
      std::sync::mpmc::Sender::send<enum2_<country_core::systems::save::save_game::AsyncSaveCmd>_>()
      ;
      if ((int)local_128 != 0xb) {
        local_288 = local_f8;
        pcStack_280 = (char *)uStack_f0;
        pppppfStack_298 = local_108;
        ppppppfStack_2a8 = local_118;
        ppppppfStack_2a0 = ppppppfStack_110;
        local_2b8 = local_128;
        ppppppfStack_2b0 = ppppppfStack_120;
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
    }
    ppppppfVar45 = local_d0;
    pppppppfVar34 = (float *******)pppppppfVar41[7];
    ppppppfVar61 = (float ******)local_d0[4];
    pppppppfVar60 = (float *******)local_d0[5];
    if (pppppppfVar34 == pppppppfVar60) {
      lVar58 = local_170;
      if (pppppppfVar34 != (float *******)0x0) {
        lVar39 = 8;
        do {
          if (*(int *)((longlong)pppppppfVar41[6] + lVar39) !=
              *(int *)((longlong)ppppppfVar61 + lVar39)) goto LAB_140a41375;
          lVar33 = *(longlong *)((longlong)pppppppfVar41[6] + lVar39 + -8);
          lVar20 = *(longlong *)((longlong)ppppppfVar61 + lVar39 + -8);
          uVar43 = *(ulonglong *)(lVar33 + 0x10);
          if (lVar33 == lVar20) {
            if (uVar43 < 2) goto LAB_140a41375;
          }
          else {
            uVar54 = *(ulonglong *)(lVar20 + 0x10);
            if (((uVar43 < 2) || (uVar54 < 2)) || (uVar43 != uVar54)) goto LAB_140a41375;
          }
          lVar39 = lVar39 + 0x10;
          pppppppfVar34 = (float *******)((longlong)pppppppfVar34 + -1);
        } while (pppppppfVar34 != (float *******)0x0);
      }
    }
    else {
LAB_140a41375:
      uVar43 = (longlong)pppppppfVar60 << 4;
      if (((ulonglong)pppppppfVar60 >> 0x3c != 0) || (0x7ffffffffffffff8 < uVar43)) {
LAB_140a41eb8:
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      local_e0 = (float ******)pppppppfVar60;
      if (uVar43 == 0) {
        local_d0 = (float ******)&DAT_00000008;
      }
      else {
        std::alloc::__default_lib_allocator::__rust_alloc();
        local_d0 = (float ******)extraout_RAX_16;
        if (extraout_RAX_16 == (float *******)0x0) goto LAB_140a41eb8;
        if (pppppppfVar60 != (float *******)0x0) {
          uVar54 = 0;
          do {
            if (uVar43 == uVar54) break;
            pppppfVar47 = *ppppppfVar61;
            uVar36 = *(undefined4 *)(ppppppfVar61 + 1);
            LOCK();
            ppppfVar3 = *pppppfVar47;
            *pppppfVar47 = (float ****)((longlong)*pppppfVar47 + 1);
            UNLOCK();
            if (*pppppfVar47 == (float ****)0x0 ||
                SCARRY8((longlong)ppppfVar3,1) != (longlong)*pppppfVar47 < 0) goto LAB_140a4248a;
            pppppppfVar60 = (float *******)((longlong)pppppppfVar60 + -1);
            ppppppfVar61 = ppppppfVar61 + 2;
            *(float ******)((longlong)extraout_RAX_16 + uVar54) = pppppfVar47;
            *(undefined4 *)((longlong)extraout_RAX_16 + uVar54 + 8) = uVar36;
            uVar54 = uVar54 + 0x10;
          } while (pppppppfVar60 != (float *******)0x0);
        }
      }
      lVar58 = local_170;
      core::ptr::drop_in_place<country_core::resources::history::HistoryChainSnapshots>();
      local_c0[5] = (float *****)local_e0;
      local_c0[6] = (float *****)local_d0;
      local_c0[7] = (float *****)local_e0;
      ppppppfVar61 = (float ******)ppppppfVar45[5];
      uVar43 = (longlong)ppppppfVar61 << 4;
      if (((ulonglong)ppppppfVar61 >> 0x3c != 0) || (0x7ffffffffffffff8 < uVar43)) {
LAB_140a41ecf:
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      if (uVar43 == 0) {
        ppppppfVar32 = (float ******)0x8;
      }
      else {
        ppppppfVar49 = (float ******)ppppppfVar45[4];
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_17 == (float ******)0x0) goto LAB_140a41ecf;
        ppppppfVar32 = extraout_RAX_17;
        if (ppppppfVar61 != (float ******)0x0) {
          uVar54 = 0;
          ppppppfVar40 = ppppppfVar61;
          do {
            if (uVar43 == uVar54) break;
            pppppfVar47 = *ppppppfVar49;
            uVar36 = *(undefined4 *)(ppppppfVar49 + 1);
            LOCK();
            ppppfVar3 = *pppppfVar47;
            *pppppfVar47 = (float ****)((longlong)*pppppfVar47 + 1);
            UNLOCK();
            if (*pppppfVar47 == (float ****)0x0 ||
                SCARRY8((longlong)ppppfVar3,1) != (longlong)*pppppfVar47 < 0) goto LAB_140a4248a;
            ppppppfVar40 = (float ******)((longlong)ppppppfVar40 - 1);
            ppppppfVar49 = ppppppfVar49 + 2;
            *(float ******)((longlong)extraout_RAX_17 + uVar54) = pppppfVar47;
            *(undefined4 *)((longlong)extraout_RAX_17 + uVar54 + 8) = uVar36;
            uVar54 = uVar54 + 0x10;
          } while (ppppppfVar40 != (float ******)0x0);
        }
      }
      local_2b8 = ppppppfVar61;
      ppppppfStack_2b0 = ppppppfVar32;
      ppppppfStack_2a8 = ppppppfVar61;
      std::alloc::__default_lib_allocator::__rust_alloc();
      pppppppfVar41 = (float *******)local_c0;
      if (extraout_RAX_18 == (float *******)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      extraout_RAX_18[2] = ppppppfStack_2a8;
      *extraout_RAX_18 = local_2b8;
      extraout_RAX_18[1] = ppppppfStack_2b0;
      ppppppfStack_2a0 = (float ******)CONCAT71(ppppppfStack_2a0._1_7_,9);
      ppppppfStack_2a8 =
           (float ******)
           &PTR_drop_in_place<country_core::resources::history::HistoryChainSnapshots>_142ace088;
      local_2b8 = (float ******)&DAT_00000006;
      ppppppfStack_2b0 = (float ******)extraout_RAX_18;
      std::sync::mpmc::Sender::send<enum2_<country_core::systems::save::save_game::AsyncSaveCmd>_>()
      ;
      if ((int)local_128 != 0xb) {
        local_288 = local_f8;
        pcStack_280 = (char *)uStack_f0;
        pppppfStack_298 = local_108;
        ppppppfStack_2a8 = local_118;
        ppppppfStack_2a0 = ppppppfStack_110;
        local_2b8 = local_128;
        ppppppfStack_2b0 = ppppppfStack_120;
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
    }
    if (*(int *)((longlong)pppppppfVar41 + 0xf4) != *(int *)(ppppppfVar45 + 0xd)) {
      *(int *)((longlong)pppppppfVar41 + 0xf4) = *(int *)(ppppppfVar45 + 0xd);
      iVar30 = *(int *)(ppppppfVar45 + 0xd);
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_19 == (float *******)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      *(int *)extraout_RAX_19 = iVar30;
      ppppppfStack_2a0 = (float ******)CONCAT71(ppppppfStack_2a0._1_7_,7);
      ppppppfStack_2a8 = (float ******)&DAT_142ace0c0;
      local_2b8 = (float ******)&DAT_00000006;
      ppppppfStack_2b0 = (float ******)extraout_RAX_19;
      std::sync::mpmc::Sender::send<enum2_<country_core::systems::save::save_game::AsyncSaveCmd>_>()
      ;
      if ((int)local_128 != 0xb) {
        local_288 = local_f8;
        pcStack_280 = (char *)uStack_f0;
        pppppfStack_298 = local_108;
        ppppppfStack_2a8 = local_118;
        ppppppfStack_2a0 = ppppppfStack_110;
        local_2b8 = local_128;
        ppppppfStack_2b0 = ppppppfStack_120;
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      *(int *)(pppppppfVar41 + 0x29) = 0;
    }
    if (*(int *)(pppppppfVar41 + 0x1f) != *(int *)((longlong)ppppppfVar45 + 0x6c)) {
      *(int *)(pppppppfVar41 + 0x1f) = *(int *)((longlong)ppppppfVar45 + 0x6c);
      iVar30 = *(int *)((longlong)ppppppfVar45 + 0x6c);
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_20 == (float *******)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      *(int *)extraout_RAX_20 = iVar30;
      ppppppfStack_2a0 = (float ******)CONCAT71(ppppppfStack_2a0._1_7_,8);
      ppppppfStack_2a8 = (float ******)&DAT_142ace0c0;
      local_2b8 = (float ******)&DAT_00000006;
      ppppppfStack_2b0 = (float ******)extraout_RAX_20;
      std::sync::mpmc::Sender::send<enum2_<country_core::systems::save::save_game::AsyncSaveCmd>_>()
      ;
      if ((int)local_128 != 0xb) {
        local_288 = local_f8;
        pcStack_280 = (char *)uStack_f0;
        pppppfStack_298 = local_108;
        ppppppfStack_2a8 = local_118;
        ppppppfStack_2a0 = ppppppfStack_110;
        local_2b8 = local_128;
        ppppppfStack_2b0 = ppppppfStack_120;
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      *(int *)(pppppppfVar41 + 0x29) = 0;
    }
  }
  if ((byte)local_158 == '\0') {
    ppppppfStack_2b0 = (float ******)CONCAT44(local_2d4,(int)local_150);
    ppppppfStack_2a8 = (float ******)CONCAT44(ppppppfStack_2a8._4_4_,(int)local_140);
    local_2b8 = (float ******)&DAT_00000007;
    std::sync::mpmc::Sender::send<enum2_<country_core::systems::save::save_game::AsyncSaveCmd>_>();
    if ((int)local_128 != 0xb) {
      local_288 = local_f8;
      pcStack_280 = (char *)uStack_f0;
      pppppfStack_298 = local_108;
      ppppppfStack_2a8 = local_118;
      ppppppfStack_2a0 = ppppppfStack_110;
      local_2b8 = local_128;
      ppppppfStack_2b0 = ppppppfStack_120;
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
  }
  if ((!bVar65 && !bVar70 || !bVar64 && !bVar69) || (!bVar63 && !bVar68 || !bVar62 && !bVar67)) {
    lVar33 = (ulonglong)*(byte *)(local_358 + 0xc0) * 0x40;
    lVar39 = local_358 + lVar33;
    cVar28 = *(char *)(local_358 + 0x28 + lVar33);
    if ((((((cVar28 != *(char *)(local_358 + 0x28)) ||
           (*(size_t *)(lVar39 + 0x10) != *(size_t *)(local_358 + 0x10))) ||
          (iVar30 = memcmp(*(void **)(lVar39 + 8),*(void **)(local_358 + 8),
                           *(size_t *)(lVar39 + 0x10)), iVar30 != 0)) ||
         ((*(float *)(lVar39 + 0x18) != *(float *)(local_358 + 0x18) ||
          (NAN(*(float *)(lVar39 + 0x18)) || NAN(*(float *)(local_358 + 0x18)))))) ||
        ((*(float *)(lVar39 + 0x1c) != *(float *)(local_358 + 0x1c) ||
         ((NAN(*(float *)(lVar39 + 0x1c)) || NAN(*(float *)(local_358 + 0x1c)) ||
          (*(float *)(lVar39 + 0x20) != *(float *)(local_358 + 0x20))))))) ||
       (((NAN(*(float *)(lVar39 + 0x20)) || NAN(*(float *)(local_358 + 0x20)) ||
         (((((*(float *)(lVar39 + 0x24) != *(float *)(local_358 + 0x24) ||
             (NAN(*(float *)(lVar39 + 0x24)) || NAN(*(float *)(local_358 + 0x24)))) ||
            (*(float *)(lVar39 + 0x30) != *(float *)(local_358 + 0x30))) ||
           (((NAN(*(float *)(lVar39 + 0x30)) || NAN(*(float *)(local_358 + 0x30)) ||
             (*(float *)(lVar39 + 0x34) != *(float *)(local_358 + 0x34))) ||
            ((NAN(*(float *)(lVar39 + 0x34)) || NAN(*(float *)(local_358 + 0x34)) ||
             ((*(float *)(lVar39 + 0x38) != *(float *)(local_358 + 0x38) ||
              (NAN(*(float *)(lVar39 + 0x38)) || NAN(*(float *)(local_358 + 0x38)))))))))) ||
          (*(char *)(lVar39 + 0x3c) != *(char *)(local_358 + 0x3c))))) ||
        (*(char *)(lVar39 + 0x3d) != *(char *)(local_358 + 0x3d))))) {
      alloc::string::impl$6::clone();
      uVar43 = local_290;
      local_290 = CONCAT71(local_290._1_7_,cVar28);
      uVar54 = local_290;
      ppppppfStack_2a0 = *(float *******)(lVar39 + 0x18);
      pppppfStack_298 = *(float ******)(lVar39 + 0x20);
      uVar15 = *(undefined8 *)(lVar39 + 0x30);
      uVar36 = *(undefined4 *)(lVar39 + 0x38);
      uVar8 = *(undefined1 *)(lVar39 + 0x3c);
      uVar9 = *(undefined1 *)(lVar39 + 0x3d);
      local_290._1_4_ = SUB84(uVar43,1);
      uStack_ff = local_290._1_4_;
      local_290._5_2_ = SUB82(uVar43,5);
      uStack_fb = local_290._5_2_;
      local_290._7_1_ = SUB81(uVar43,7);
      uStack_f9 = local_290._7_1_;
      local_118 = ppppppfStack_2a8;
      local_128 = local_2b8;
      ppppppfStack_120 = ppppppfStack_2b0;
      *puVar17 = local_30c;
      local_290 = uVar54;
      ppppppfStack_110 = ppppppfStack_2a0;
      local_108 = pppppfStack_298;
      cStack_100 = cVar28;
      if ((!SBORROW8(0,*(longlong *)(lVar58 + 0x18))) && (*(longlong *)(lVar58 + 0x18) != 0)) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      *(float ******)(lVar58 + 0x38) = local_108;
      *(char *)(lVar58 + 0x40) = cStack_100;
      *(undefined4 *)(lVar58 + 0x41) = uStack_ff;
      *(undefined2 *)(lVar58 + 0x45) = uStack_fb;
      *(undefined1 *)(lVar58 + 0x47) = uStack_f9;
      *(float *******)(lVar58 + 0x28) = local_118;
      *(float *******)(lVar58 + 0x30) = ppppppfStack_110;
      *(float *******)(lVar58 + 0x18) = local_128;
      *(float *******)(lVar58 + 0x20) = ppppppfStack_120;
      *(undefined8 *)(lVar58 + 0x48) = uVar15;
      *(undefined4 *)(lVar58 + 0x50) = uVar36;
      *(undefined1 *)(lVar58 + 0x54) = uVar8;
      *(undefined1 *)(lVar58 + 0x55) = uVar9;
    }
    else {
      *puVar17 = local_30c;
      if ((!SBORROW8(0,*(longlong *)(lVar58 + 0x18))) && (*(longlong *)(lVar58 + 0x18) != 0)) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      *(undefined8 *)(lVar58 + 0x18) = 0x8000000000000000;
    }
    local_2b8 = *(float *******)(lVar58 + 0x60);
    ppppppfStack_2b0 = *(float *******)(lVar58 + 0x68);
    resources::session::SessionMeta::write_to_disk();
    if (extraout_RAX_21 != (float *******)0x0) {
      local_398 = (float ******)extraout_RAX_21;
      if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
        local_128 = (float ******)&local_398;
        ppppppfStack_120 = (float ******)anyhow::error::impl$5::fmt;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        pppppfStack_298 = (float *****)*extraout_RAX_22;
        local_290 = extraout_RAX_22[1];
        local_288 = 1;
        pcStack_280 = "country_core::systems::save::save_game";
        local_278 = 0x26;
        ppuStack_268 = &PTR_s_Failed_writing_session_meta__142ace148;
        local_260 = 1;
        ppppppfStack_258 = (float ******)&local_128;
        local_250 = 1;
        uStack_248 = 0;
        local_2b8 = (float ******)0x0;
        ppppppfStack_2b0 = (float ******)0x142acdf48;
        ppppppfStack_2a8 = (float ******)&DAT_00000026;
        ppppppfStack_2a0 = (float ******)0x0;
        local_270 = CONCAT44((int)extraout_RAX_22[2],1);
        ::log::__private_api::impl$2::log();
      }
      anyhow::error::impl$6::drop();
    }
    std::sync::mpmc::sync_channel<enum2_<core::result::Result<tuple_<>,anyhow::Error>_>_>();
    ppppppfVar61 = ppppppfStack_2b0;
    local_398 = ppppppfStack_2a8;
    ppppppfStack_390 = ppppppfStack_2a0;
    ppppppfStack_2b0 = local_2b8;
    ppppppfStack_2a8 = ppppppfVar61;
    local_2b8 = (float ******)&DAT_0000000a;
    std::sync::mpmc::Sender::send<enum2_<country_core::systems::save::save_game::AsyncSaveCmd>_>();
    if ((int)local_128 != 0xb) {
      local_290 = CONCAT17(uStack_f9,CONCAT25(uStack_fb,CONCAT41(uStack_ff,cStack_100)));
      local_288 = local_f8;
      pcStack_280 = (char *)uStack_f0;
      pppppfStack_298 = local_108;
      ppppppfStack_2a8 = local_118;
      ppppppfStack_2a0 = ppppppfStack_110;
      local_2b8 = local_128;
      ppppppfStack_2b0 = ppppppfStack_120;
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    if ((float *******)local_398 == (float *******)0x2) {
      std::sync::mpmc::zero::Channel::recv<enum2_<core::result::Result<tuple_<>,anyhow::Error>_>_>()
      ;
    }
    else if ((int)local_398 == 1) {
      std::sync::mpmc::list::Channel::recv<enum2_<core::result::Result<tuple_<>,anyhow::Error>_>_>()
      ;
    }
    else {
      std::sync::mpmc::array::Channel::recv<enum2_<core::result::Result<tuple_<>,anyhow::Error>_>_>
                ();
    }
    if ((char)local_2b8 == '\x01') {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    if ((float *******)ppppppfStack_2b0 == (float *******)0x0) {
      resources::session::SessionId::save_folder_path();
      local_c0 = local_2b8;
      local_d0 = ppppppfStack_2b0;
      local_128 = ppppppfStack_2a8;
      ppppppfStack_120 = ppppppfStack_2a0;
      *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
      lVar58 = *in_stack_00000028;
      lVar39 = *(longlong *)(lVar58 + 0x30);
      uVar15 = *(undefined8 *)(lVar58 + 0x40);
      if (lVar39 == *(longlong *)(lVar58 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                  ();
      }
      lVar33 = *(longlong *)(lVar58 + 0x28);
      *(float *******)(lVar33 + lVar39 * 0x28) = local_c0;
      *(float *******)(lVar33 + 8 + lVar39 * 0x28) = local_d0;
      puVar57 = (undefined8 *)(lVar33 + 0x10 + lVar39 * 0x28);
      *puVar57 = local_128;
      puVar57[1] = ppppppfStack_120;
      *(undefined8 *)(lVar33 + 0x20 + lVar39 * 0x28) = uVar15;
      *(longlong *)(lVar58 + 0x30) = lVar39 + 1;
      *(longlong *)(lVar58 + 0x40) = *(longlong *)(lVar58 + 0x40) + 1;
    }
    else {
      local_2d0 = ppppppfStack_2b0;
      if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
        local_128 = (float ******)&local_2d0;
        ppppppfStack_120 = (float ******)anyhow::error::impl$5::fmt;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        pppppfStack_298 = (float *****)*extraout_RAX_23;
        local_290 = extraout_RAX_23[1];
        local_288 = 1;
        pcStack_280 = "country_core::systems::save::save_game";
        local_278 = 0x26;
        ppuStack_268 = &PTR_s_Error_while_performing_a_synchro_142ace1f0;
        local_260 = 1;
        local_250 = 1;
        uStack_248 = 0;
        local_2b8 = (float ******)0x0;
        ppppppfStack_2b0 = (float ******)0x142acdf48;
        ppppppfStack_2a8 = (float ******)&DAT_00000026;
        ppppppfStack_2a0 = (float ******)0x0;
        local_270 = CONCAT44((int)extraout_RAX_23[2],1);
        ppppppfStack_258 = (float ******)&local_128;
        ::log::__private_api::impl$2::log();
      }
      anyhow::error::impl$6::drop();
    }
    core::ptr::
    drop_in_place<std::sync::mpsc::Receiver<enum2_<core::result::Result<tuple_<>,anyhow::Error>_>_>_>
              ();
  }
  if (((ulonglong)local_168 & 0x7fffffffffffffff) != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (bVar27) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_24 == 1) {
      plVar35 = extraout_RAX_24 + 1;
    }
    else if ((*extraout_RAX_24 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar35 = extraout_RAX_25, extraout_RAX_25 == (longlong *)0x0)) {
LAB_140a41dff:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar35 != 0) {
LAB_140a41e0b:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar35 = -1;
    local_c0 = (float ******)plVar35;
    puffin::ThreadProfiler::end_scope();
    *local_c0 = (float *****)((longlong)*local_c0 + 1);
  }
  return;
}

