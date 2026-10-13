// signature: void __cdecl roof_audio_system(void)

// WARNING: Instruction at (ram,0x000140aa4e75) overlaps instruction at (ram,0x000140aa4e73)
// 
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl country_core::systems::audio::roof_audio::roof_audio_system(void)

{
  byte *pbVar1;
  longlong *plVar2;
  char *pcVar3;
  undefined8 *puVar4;
  float fVar5;
  byte bVar6;
  byte bVar7;
  byte bVar8;
  ulonglong *puVar9;
  char extraout_AL;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined *puVar10;
  longlong *extraout_RAX_01;
  ulonglong extraout_RAX_02;
  int *piVar11;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  uint uVar12;
  undefined8 *in_RCX;
  undefined *puVar13;
  longlong *plVar14;
  longlong *in_RDX;
  ulonglong uVar15;
  longlong *plVar16;
  longlong lVar17;
  longlong *in_R8;
  undefined *puVar18;
  longlong lVar19;
  longlong *in_R9;
  undefined *puVar20;
  longlong *plVar21;
  longlong *plVar22;
  longlong *plVar23;
  undefined1 auStack_208 [32];
  undefined **local_1e8;
  longlong *local_1e0;
  undefined8 local_1d8;
  undefined8 local_1d0;
  ulonglong local_1c8;
  ulonglong uStack_1c0;
  longlong *plStack_1b8;
  longlong local_1b0;
  undefined8 local_1a8;
  longlong local_1a0;
  undefined4 local_198;
  longlong local_190;
  undefined4 local_188;
  longlong lStack_180;
  uint uStack_178;
  undefined4 uStack_174;
  longlong local_170;
  longlong local_160;
  undefined4 local_158;
  undefined4 uStack_154;
  longlong local_150;
  undefined4 local_148;
  longlong local_140;
  undefined4 local_138;
  undefined8 local_130;
  longlong *local_128;
  int local_11c;
  longlong lStack_118;
  undefined8 uStack_110;
  longlong lStack_108;
  longlong *plStack_100;
  undefined8 uStack_f8;
  undefined1 uStack_f0;
  undefined4 uStack_ef;
  undefined2 uStack_eb;
  undefined1 uStack_e9;
  longlong *local_e8;
  longlong *local_e0;
  longlong lStack_d8;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  longlong lStack_c8;
  longlong *plStack_c0;
  undefined8 uStack_b8;
  undefined1 uStack_b0;
  undefined4 uStack_af;
  undefined2 uStack_ab;
  undefined1 uStack_a9;
  undefined4 local_a4;
  longlong *local_a0;
  char local_92;
  char local_91;
  undefined8 local_90;
  
  local_90 = 0xfffffffffffffffe;
  puffin::are_scopes_on();
  local_e8 = in_R8;
  local_128 = in_R9;
  local_92 = extraout_AL;
  if (extraout_AL == '\0') {
    local_1d0 = 0;
  }
  else {
    uVar15 = 0x3b;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar15 != 0) {
        if (uVar15 < 0x3b) {
          if ((char)(&DAT_142adc7d3)[uVar15] < -0x40) {
LAB_140aa3908:
            local_1e8 = &PTR_s__home_h3__cargo_registry_src_ind_142adc8d8;
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar15 != 0x3b) goto LAB_140aa3908;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar15 != 0xfffffffffffffffe)) {
        if (uVar15 + 2 < 0x3b) {
          if ((char)(&UNK_142adc7d5)[uVar15] < -0x40) {
LAB_140aa3966:
            local_1e8 = &PTR_s__home_h3__cargo_registry_src_ind_142adc8f0;
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar15 + 2 != 0x3b) goto LAB_140aa3966;
      }
    }
    puVar10 = &DAT_142adcb1f;
    do {
      bVar6 = puVar10[-1];
      uVar12 = (uint)(char)bVar6;
      if ((char)bVar6 < '\0') {
        bVar7 = puVar10[-2];
        if ((char)bVar7 < -0x40) {
          bVar8 = puVar10[-3];
          if ((char)bVar8 < -0x40) {
            pbVar1 = puVar10 + -4;
            puVar10 = puVar10 + -4;
            uVar12 = bVar8 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar10 = puVar10 + -3;
            uVar12 = bVar8 & 0xf;
          }
          uVar12 = bVar7 & 0x3f | uVar12 << 6;
        }
        else {
          puVar10 = puVar10 + -2;
          uVar12 = bVar7 & 0x1f;
        }
        uVar12 = (uint)(bVar6 & 0x3f) | uVar12 << 6;
        if (uVar12 != 0x5c) goto LAB_140aa3a29;
LAB_140aa3a3e:
        if (puVar10 != &UNK_142adcaeb) {
          if (puVar10 + -0x142adcaeb < (undefined *)0x33) {
            if ((char)puVar10[1] < -0x40) {
LAB_140aa3a5d:
              local_1e8 = &PTR_s__home_h3__cargo_registry_src_ind_142adc8b8;
                    // WARNING: Subroutine does not return
              core::str::slice_error_fail();
            }
          }
          else if (puVar10 + -0x142adcaeb != (undefined *)0x33) goto LAB_140aa3a5d;
        }
        lVar17 = 0x32 - (longlong)(puVar10 + -0x142adcaec);
        goto LAB_140aa3a8d;
      }
      puVar10 = puVar10 + -1;
      if (uVar12 == 0x5c) goto LAB_140aa3a3e;
LAB_140aa3a29:
      if (uVar12 == 0x2f) goto LAB_140aa3a3e;
    } while (puVar10 != &anon_4935de7db48d1856e7679eb80d7ea883_48_llvm_17144954438807827936);
    lVar17 = 0x33;
LAB_140aa3a8d:
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar23 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar23 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0)) {
LAB_140aa452a:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar23 != 0) {
LAB_140aa4536:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar23 = -1;
    local_1d8 = 0;
    local_1e0 = (longlong *)0x1;
    local_1e8 = (undefined **)lVar17;
    local_a0 = plVar23;
    puffin::ThreadProfiler::begin_scope();
    *local_a0 = *local_a0 + 1;
    local_1d0 = 1;
    local_1c8 = extraout_RAX_02;
  }
  puVar20 = (undefined *)0x0;
  local_170 = *local_128;
  fVar5 = *(float *)(local_170 + 0x58);
  local_a4 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  local_e0 = (longlong *)in_RDX[2];
  *(undefined4 *)local_e0 = local_a4;
  local_a0 = (longlong *)*in_RDX;
  *(float *)((longlong)local_a0 + 0xa8) = fVar5 + *(float *)((longlong)local_a0 + 0xa8);
  fVar5 = *(float *)(local_170 + 0x58);
  *(undefined4 *)local_e0 = local_a4;
  *(float *)((longlong)local_a0 + 0xac) = fVar5 + *(float *)((longlong)local_a0 + 0xac);
  puVar9 = (ulonglong *)*local_e8;
  lVar17 = local_e8[1];
  uVar15 = *puVar9;
  puVar10 = (undefined *)(uVar15 - *(ulonglong *)(lVar17 + 0x18));
  if (uVar15 < *(ulonglong *)(lVar17 + 0x18)) {
    puVar10 = puVar20;
  }
  puVar13 = (undefined *)(uVar15 - *(ulonglong *)(lVar17 + 0x38));
  if (uVar15 < *(ulonglong *)(lVar17 + 0x38)) {
    puVar13 = puVar20;
  }
  plVar23 = (longlong *)((longlong)puVar10 * 0x28 + *(longlong *)(lVar17 + 8));
  puVar18 = *(undefined **)(lVar17 + 0x10) + -(longlong)puVar10;
  if (*(undefined **)(lVar17 + 0x10) < puVar10) {
    plVar23 = (longlong *)&DAT_00000008;
    puVar18 = puVar20;
  }
  plVar21 = (longlong *)((longlong)puVar13 * 0x28 + *(longlong *)(lVar17 + 0x28));
  local_130 = 0;
  puVar10 = *(undefined **)(lVar17 + 0x30) + -(longlong)puVar13;
  if (*(undefined **)(lVar17 + 0x30) < puVar13) {
    plVar21 = (longlong *)&DAT_00000008;
    puVar10 = puVar20;
  }
  *puVar9 = (*(longlong *)(lVar17 + 0x40) - (longlong)puVar10) - (longlong)puVar18;
  local_e8 = plVar23 + (longlong)puVar18 * 5;
  local_128 = plVar21 + (longlong)puVar10 * 5;
  plVar16 = (longlong *)*in_RCX;
  local_91 = '\x02';
  local_11c = 0;
  plVar14 = (longlong *)0x0;
LAB_140aa3c40:
  do {
    plVar22 = plVar21;
    uVar12 = (uint)plVar14;
    if ((plVar23 == (longlong *)0x0) || (plVar23 == local_e8)) {
      if ((plVar22 != (longlong *)0x0) && (plVar22 != local_128)) break;
      if (local_91 == '\x02') {
        *(undefined4 *)local_e0 = local_a4;
        country_core::utils::audio_utils::AudioLoop::stop();
      }
      else {
        local_188 = local_138;
        local_190 = local_140;
        *(undefined4 *)local_e0 = local_a4;
        country_core::utils::audio_utils::AudioLoop::start_if_not_playing();
        if (*(ulonglong *)((longlong)local_a0 + 0x10) <= *(ulonglong *)((longlong)local_a0 + 0x28))
        {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        resources::audio_resources::impl$10::set_position();
        ::utils::inverse_lerp();
        if (*(ulonglong *)((longlong)local_a0 + 0x10) <= *(ulonglong *)((longlong)local_a0 + 0x28))
        {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        resources::audio_resources::impl$10::set_parameter();
        if (*(ulonglong *)((longlong)local_a0 + 0x10) <= *(ulonglong *)((longlong)local_a0 + 0x28))
        {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        resources::audio_resources::impl$10::set_parameter();
      }
      if (local_11c == 0) {
        *(undefined4 *)local_e0 = local_a4;
        country_core::utils::audio_utils::AudioLoop::stop();
      }
      else {
        local_198 = local_148;
        local_1a0 = local_150;
        *(undefined4 *)local_e0 = local_a4;
        country_core::utils::audio_utils::AudioLoop::start_if_not_playing();
        if (*(ulonglong *)((longlong)local_a0 + 0x48) <= *(ulonglong *)((longlong)local_a0 + 0x60))
        {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        resources::audio_resources::impl$10::set_position();
      }
      if ((int)local_130 == 0) {
        *(undefined4 *)local_e0 = local_a4;
        country_core::utils::audio_utils::AudioLoop::stop();
      }
      else {
        local_1a8 = CONCAT44(local_1a8._4_4_,local_158);
        local_1b0 = local_160;
        *(undefined4 *)local_e0 = local_a4;
        country_core::utils::audio_utils::AudioLoop::start_if_not_playing();
        if (*(ulonglong *)((longlong)local_a0 + 0x80) <= *(ulonglong *)((longlong)local_a0 + 0x98))
        {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        resources::audio_resources::impl$10::set_position();
      }
      if (local_92 != '\0') {
        puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
        if (*extraout_RAX_05 == 1) {
          plVar23 = extraout_RAX_05 + 1;
        }
        else if ((*extraout_RAX_05 != 0) ||
                (std::sys::thread_local::native::lazy::Storage::
                 initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                           (), plVar23 = extraout_RAX_07, extraout_RAX_07 == (longlong *)0x0))
        goto LAB_140aa452a;
        if (*plVar23 != 0) goto LAB_140aa4536;
        *plVar23 = -1;
        local_e8 = plVar23;
        puffin::ThreadProfiler::end_scope();
        *local_e8 = *local_e8 + 1;
      }
      return;
    }
    *puVar9 = *puVar9 + 1;
    plVar2 = plVar23 + 5;
    pbVar1 = (byte *)(plVar23 + 1);
    piVar11 = (int *)(&DAT_142adcef8 + *(int *)(&DAT_142adcef8 + (ulonglong)*pbVar1 * 4));
    plVar21 = plVar22;
    plVar22 = plVar23;
    plVar23 = plVar2;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
    switch(*pbVar1) {
    case 0:
      goto code_r0x000140aa3c9e;
    case 1:
      goto code_r0x000140aa3e95;
    case 2:
      goto code_r0x000140aa3d81;
    case 3:
      goto code_r0x000140aa3df1;
    case 4:
      goto code_r0x000140aa3d09;
    case 5:
      goto code_r0x000140aa3f04;
    case 6:
      goto code_r0x000140aa3f3a;
    case 7:
      goto code_r0x000140aa3e5f;
    case 9:
      goto code_r0x000140aa3c1c;
    case 10:
      goto code_r0x000140aa3d4b;
    case 0xb:
      goto code_r0x000140aa3f8e;
    case 0xc:
      goto code_r0x000140aa4e58;
    case 0xd:
      goto code_r0x000140aa4edf;
    case 0xe:
      goto code_r0x000140aa4e8e;
    case 0xf:
      goto code_r0x000140aa4ea9;
    case 0x10:
      goto code_r0x000140aa4dbb;
    case 0x11:
      goto code_r0x000140aa4efa;
    case 0x12:
      goto code_r0x000140aa4f15;
    case 0x13:
      goto code_r0x000140aa4ec4;
    case 0x14:
      goto code_r0x000140aa4f4b;
    case 0x15:
      goto code_r0x000140aa4e73;
    }
  } while( true );
  *puVar9 = *puVar9 + 1;
  plVar21 = plVar22 + 5;
  piVar11 = (int *)(&DAT_142adcef8 +
                   *(int *)(&DAT_142adcef8 + (ulonglong)*(byte *)(plVar22 + 1) * 4));
  plVar23 = (longlong *)0x0;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(*(byte *)(plVar22 + 1)) {
  case 0:
code_r0x000140aa3c9e:
    if (0.2 < *(float *)((longlong)local_a0 + 0xa8)) {
      uStack_d0 = (undefined4)plVar22[4];
      lStack_d8 = plVar22[3];
      puVar18 = (undefined *)0x15;
      instantiate_audio_w_position();
      *(undefined4 *)local_e0 = local_a4;
      *(undefined4 *)((longlong)local_a0 + 0xa8) = 0;
      plVar14 = local_e0;
    }
    goto LAB_140aa3c40;
  case 1:
code_r0x000140aa3e95:
    if (0.2 < *(float *)((longlong)local_a0 + 0xac)) {
      uStack_d0 = (undefined4)plVar22[4];
      lStack_d8 = plVar22[3];
      puVar18 = (undefined *)0x15;
      instantiate_audio_w_position();
      *(undefined4 *)local_e0 = local_a4;
      *(undefined4 *)((longlong)local_a0 + 0xac) = 0;
      plVar14 = local_e0;
    }
    goto LAB_140aa3c40;
  case 2:
code_r0x000140aa3d81:
    local_91 = *(char *)((longlong)plVar22 + 9);
    plVar14 = (longlong *)0x0;
    if (*plVar16 != 0) {
      puVar18 = (undefined *)0xa;
      resources::audio_resources::AudioCoreInner::set_parameter();
      plVar14 = (longlong *)0x0;
      if (*plVar16 != 0) {
        plVar14 = (longlong *)(*plVar16 + 0x10);
        puVar18 = (undefined *)0xb;
        resources::audio_resources::AudioCoreInner::set_parameter();
      }
    }
    local_138 = (undefined4)plVar22[4];
    local_140 = plVar22[3];
    goto LAB_140aa3c40;
  case 3:
code_r0x000140aa3df1:
    local_148 = (undefined4)plVar22[4];
    local_150 = plVar22[3];
    local_11c = 1;
    plVar14 = (longlong *)*plVar16;
    if ((longlong *)*plVar16 != (longlong *)0x0) {
      puVar18 = (undefined *)0x10;
      resources::audio_resources::AudioCoreInner::set_parameter();
      plVar14 = (longlong *)*plVar16;
      if (plVar14 != (longlong *)0x0) {
        plVar14 = plVar14 + 2;
        puVar18 = (undefined *)0xd;
        resources::audio_resources::AudioCoreInner::set_parameter();
      }
    }
    goto LAB_140aa3c40;
  case 4:
code_r0x000140aa3d09:
    if (*plVar16 == 0) {
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_03 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      *extraout_RAX_03 = 0x532f3a746e657665;
      extraout_RAX_03[1] = 0x54666f6f522f5846;
      extraout_RAX_03[2] = 0x646e614c73656c69;
      lStack_d8 = 0;
      lStack_c8 = 0x18;
      uStack_b8 = 0x18;
      uStack_b0 = 2;
      plStack_c0 = extraout_RAX_03;
    }
    else {
      resources::audio_resources::AudioCoreInner::instantiate_audio();
      if (lStack_d8 == 2) {
        lStack_118 = CONCAT44(uStack_cc,uStack_d0);
        local_1e8 = (undefined **)&UNK_142adcc80;
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
    }
    uStack_110 = CONCAT44(uStack_cc,uStack_d0);
    uStack_f8 = uStack_b8;
    uStack_f0 = uStack_b0;
    uStack_ef = uStack_af;
    uStack_eb = uStack_ab;
    uStack_e9 = uStack_a9;
    lStack_108 = lStack_c8;
    plStack_100 = plStack_c0;
    lStack_118 = lStack_d8;
    uStack_d0 = (undefined4)plVar22[4];
    lStack_d8 = plVar22[3];
    plVar14 = &lStack_118;
    resources::audio_resources::impl$10::set_position();
    puVar18 = (undefined *)0xe;
    resources::audio_resources::impl$10::set_parameter();
    resources::audio_resources::AudioInstance::start_if_not_playing();
    resources::audio_resources::impl$13::drop();
    break;
  case 5:
code_r0x000140aa3f04:
    uStack_d0 = (undefined4)plVar22[4];
    lStack_d8 = plVar22[3];
    puVar18 = (undefined *)0x1a;
    plVar14 = plVar16;
    instantiate_audio_w_position();
    goto LAB_140aa3c40;
  case 6:
code_r0x000140aa3f3a:
    puVar20 = (undefined *)(ulonglong)*(byte *)((longlong)plVar22 + 9);
    uStack_178 = *(uint *)(plVar22 + 4);
    lStack_180 = plVar22[3];
    if (*plVar16 == 0) {
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_04 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      *extraout_RAX_04 = 0x532f3a746e657665;
      extraout_RAX_04[1] = 0x6f6c72654d2f5846;
      extraout_RAX_04[2] = 0x726165707041736e;
      lStack_d8 = 0;
      lStack_c8 = 0x18;
      uStack_b8 = 0x18;
      uStack_b0 = 2;
      plStack_c0 = extraout_RAX_04;
    }
    else {
      resources::audio_resources::AudioCoreInner::instantiate_audio();
      if (lStack_d8 == 2) {
        lStack_118 = CONCAT44(uStack_cc,uStack_d0);
        local_1e8 = (undefined **)&UNK_142adccd8;
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
    }
    plVar14 = &lStack_118;
    uStack_110 = CONCAT44(uStack_cc,uStack_d0);
    uStack_f8 = uStack_b8;
    uStack_f0 = uStack_b0;
    uStack_ef = uStack_af;
    uStack_eb = uStack_ab;
    uStack_e9 = uStack_a9;
    lStack_108 = lStack_c8;
    plStack_100 = plStack_c0;
    lStack_118 = lStack_d8;
    puVar18 = (undefined *)0xe;
    resources::audio_resources::impl$10::set_parameter();
    resources::audio_resources::impl$10::set_position();
    resources::audio_resources::AudioInstance::start_if_not_playing();
    resources::audio_resources::impl$13::drop();
    break;
  case 7:
code_r0x000140aa3e5f:
    uStack_d0 = (undefined4)plVar22[4];
    lStack_d8 = plVar22[3];
    puVar18 = (undefined *)0x1e;
    plVar14 = plVar16;
    instantiate_audio_w_position();
  case 8:
    goto LAB_140aa3c40;
  case 9:
code_r0x000140aa3c1c:
    local_158 = (undefined4)plVar22[4];
    local_160 = plVar22[3];
    local_130 = 1;
    goto LAB_140aa3c40;
  case 10:
code_r0x000140aa3d4b:
    uStack_d0 = (undefined4)plVar22[4];
    lStack_d8 = plVar22[3];
    puVar18 = (undefined *)0x16;
    plVar14 = plVar16;
    instantiate_audio_w_position();
    goto LAB_140aa3c40;
  case 0xb:
code_r0x000140aa3f8e:
    uStack_d0 = (undefined4)plVar22[4];
    lStack_d8 = plVar22[3];
    puVar18 = (undefined *)0x19;
    plVar14 = plVar16;
    instantiate_audio_w_position();
    goto LAB_140aa3c40;
  case 0xc:
    goto code_r0x000140aa4e58;
  case 0xd:
code_r0x000140aa4edf:
    *piVar11 = *piVar11 + -1;
    if (*piVar11 != 0) goto LAB_140aa4e00;
    alloc::sync::Arc::
    drop_slow<enum2_<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global>
              ();
    goto LAB_140aa4e00;
  case 0xe:
code_r0x000140aa4e8e:
    *piVar11 = *piVar11 + -1;
    if (*piVar11 != 0) goto LAB_140aa4e00;
    alloc::sync::Arc::
    drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>();
    goto LAB_140aa4e00;
  case 0xf:
code_r0x000140aa4ea9:
    *piVar11 = *piVar11 + -1;
    if (*piVar11 != 0) goto LAB_140aa4e00;
    alloc::sync::Arc::
    drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo>,alloc::alloc::Global>
              ();
    goto LAB_140aa4e00;
  case 0x10:
code_r0x000140aa4dbb:
    uStack_178 = uVar12;
    if (piVar11 != (int *)0x0) {
      plVar21 = (longlong *)CONCAT44(uStack_154,local_158);
      plVar22 = plVar21 + (longlong)piVar11;
      puVar20 = &DAT_142adcf28;
      plVar23 = local_1e0;
      goto LAB_140aa4e0d;
    }
    goto LAB_140aa4f96;
  case 0x11:
code_r0x000140aa4efa:
    *piVar11 = *piVar11 + -1;
    if (*piVar11 != 0) goto LAB_140aa4e00;
    alloc::sync::Arc::
    drop_slow<enum2_<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>
              ();
    goto LAB_140aa4e00;
  case 0x12:
code_r0x000140aa4f15:
    *piVar11 = *piVar11 + -1;
    if (*piVar11 != 0) goto LAB_140aa4e00;
    alloc::sync::Arc::
    drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>();
    goto LAB_140aa4e00;
  case 0x13:
code_r0x000140aa4ec4:
    *piVar11 = *piVar11 + -1;
    if (*piVar11 != 0) goto LAB_140aa4e00;
    alloc::sync::Arc::
    drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>();
    goto LAB_140aa4e00;
  case 0x14:
code_r0x000140aa4f4b:
    *piVar11 = *piVar11 + -1;
    if (*piVar11 != 0) goto LAB_140aa4e00;
    alloc::sync::Arc::
    drop_slow<enum2_<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>
              ();
    goto LAB_140aa4e00;
  case 0x15:
code_r0x000140aa4e73:
    puVar18[-0x7d] = puVar18[-0x7d];
                    // WARNING (jumptable): Read-only address (ram,0x000142adcef8) is written
                    // WARNING: Read-only address (ram,0x000142adcef8) is written
    _DAT_142adcef8 = uVar12 | 0xfdfc6da6;
    *(char *)piVar11 = (char)*piVar11 + (char)piVar11;
    pcVar3 = (char *)(((ulonglong)auStack_208 & 0xffffffff) + 99);
    *pcVar3 = *pcVar3 + (char)((ulonglong)auStack_208 & 0xffffffff);
    lVar17 = CONCAT71((int7)((ulonglong)piVar11 >> 8),(char)piVar11 + -0x7c);
    goto code_r0x000140aa4e83;
  }
  if (lStack_108 != 0) {
    puVar18 = (undefined *)0x1;
    plVar14 = plStack_100;
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  goto LAB_140aa3c40;
code_r0x000140aa4e58:
  plVar14 = (longlong *)(ulonglong)(uVar12 + (int)plVar16);
  while( true ) {
    lVar17 = plVar23[1];
    lVar19 = (longlong)plVar14 * 0x30;
    local_1c8 = *(ulonglong *)(lVar17 + lVar19);
    uStack_1c0 = ((ulonglong *)(lVar17 + lVar19))[1];
    puVar4 = (undefined8 *)(lVar17 + 0x10 + lVar19);
    plStack_1b8 = (longlong *)*puVar4;
    local_1b0 = puVar4[1];
    puVar4 = (undefined8 *)(lVar17 + 0x20 + lVar19);
    local_1a8 = *puVar4;
    local_1a0 = puVar4[1];
    memmove((void *)(lVar17 + lVar19),(void *)(lVar17 + lVar19 + 0x30),
            (longlong)(~(ulonglong)plVar14 + (longlong)plVar16) * 0x30);
    plVar23[2] = (longlong)((longlong)plVar16 + -1);
    if ((local_1c8 & 0xff) < 10) break;
    LOCK();
    *plStack_1b8 = *plStack_1b8 + -1;
    UNLOCK();
    if (*plStack_1b8 == 0) {
      alloc::sync::Arc::
      drop_slow<std::sync::poison::mutex::Mutex<winit::dpi::PhysicalSize<u32>_>,alloc::alloc::Global>
                ();
    }
LAB_140aa4e00:
    plVar21 = plVar21 + 1;
    if (plVar21 == plVar22) {
LAB_140aa4f96:
      if (CONCAT44(uStack_174,uStack_178) != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      return;
    }
LAB_140aa4e0d:
    plVar14 = (longlong *)*plVar21;
    plVar16 = (longlong *)plVar23[2];
    if (plVar16 <= plVar14) {
                    // WARNING: Subroutine does not return
      alloc::vec::impl$1::remove::assert_failed();
    }
  }
  lVar17 = (longlong)*(int *)(puVar20 + (local_1c8 & 0xff) * 4);
code_r0x000140aa4e83:
                    // WARNING: Could not recover jumptable at 0x000140aa4e86. Too many branches
                    // WARNING: Treating indirect jump as call
  (*(code *)(puVar20 + lVar17))();
  return;
}

