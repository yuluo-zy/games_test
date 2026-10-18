// 0x140a72810 music_audio_system
// signature: void __cdecl music_audio_system(void)
// callees: [{"name": "VAL", "address": "140c18130"}, {"name": "are_scopes_on", "address": "140c16a00"}, {"name": "begin_scope", "address": "140c16d40"}, {"name": "end_scope", "address": "140c170f0"}, {"name": "get_paused", "address": "141436fe0"}, {"name": "initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>", "address": "1428f87d0"}, {"name": "panic_access_error", "address": "1428d80d0"}, {"name": "panic_already_borrowed", "address": "1428d9250"}, {"name": "pause", "address": "1408b7490"}, {"name": "rfind<ref$<str$>_>", "address": "140a719d0"}, {"name": "set_parameter", "address": "140a53db0"}, {"name": "set_paused", "address": "141437160"}, {"name": "slice_error_fail", "address": "1428d9940"}, {"name": "start_if_not_playing", "address": "140a57300"}, {"name": "stop", "address": "141437440"}, {"name": "stop_photomode_snapshots", "address": "1408b6250"}, {"name": "time_continuous_lerp", "address": "140c904e0"}, {"name": "unwrap_failed", "address": "1428d9760"}, {"name": "update", "address": "1408b5500"}]


void __cdecl country_core::systems::audio::music::music_system::music_audio_system(void)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  char cVar5;
  undefined4 uVar6;
  undefined4 *puVar7;
  longlong lVar8;
  longlong lVar9;
  char extraout_AL;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined *puVar10;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  uint uVar11;
  longlong *plVar12;
  undefined8 *in_RDX;
  ulonglong uVar13;
  longlong *in_R8;
  undefined8 *in_R9;
  float fVar14;
  float extraout_XMM0_Da;
  float fVar15;
  undefined4 uVar16;
  longlong *in_stack_00000030;
  longlong *in_stack_00000038;
  longlong *in_stack_00000048;
  longlong *in_stack_00000050;
  longlong local_80;
  char cStack_78;
  
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    uVar13 = 0x45;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar13 != 0) {
        if (uVar13 < 0x45) {
          if ((char)(&DAT_142ad5fa8)[uVar13] < -0x40) {
LAB_140a7288f:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar13 != 0x45) goto LAB_140a7288f;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar13 != 0xfffffffffffffffe)) {
        if (uVar13 + 2 < 0x45) {
          if ((char)(&UNK_142ad5faa)[uVar13] < -0x40) {
LAB_140a728ed:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar13 + 2 != 0x45) goto LAB_140a728ed;
      }
    }
    puVar10 = &DAT_142ad62fb;
    do {
      bVar2 = puVar10[-1];
      uVar11 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar10[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar10[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar10 + -4;
            puVar10 = puVar10 + -4;
            uVar11 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar10 = puVar10 + -3;
            uVar11 = bVar4 & 0xf;
          }
          uVar11 = bVar3 & 0x3f | uVar11 << 6;
        }
        else {
          puVar10 = puVar10 + -2;
          uVar11 = bVar3 & 0x1f;
        }
        uVar11 = (uint)(bVar2 & 0x3f) | uVar11 << 6;
        if (uVar11 != 0x5c) goto LAB_140a729b9;
LAB_140a729d4:
        if (puVar10 == (undefined *)0x142ad62bf) break;
        if (puVar10 + -0x142ad62bf < (undefined *)0x3b) {
          if (-0x41 < (char)puVar10[1]) break;
        }
        else if (puVar10 + -0x142ad62bf == (undefined *)0x3b) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar10 = puVar10 + -1;
      if (uVar11 == 0x5c) goto LAB_140a729d4;
LAB_140a729b9:
      if (uVar11 == 0x2f) goto LAB_140a729d4;
    } while (puVar10 != &DAT_142ad62c0);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar12 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar12 = extraout_RAX_03, extraout_RAX_03 == (longlong *)0x0))
    goto LAB_140a73079;
    if (*plVar12 != 0) goto LAB_140a73085;
    *plVar12 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar12 = *plVar12 + 1;
  }
  uVar6 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  puVar7 = (undefined4 *)in_R8[2];
  *puVar7 = uVar6;
  lVar8 = *in_R8;
  lVar9 = *in_stack_00000038;
  fVar14 = *(float *)(lVar9 + 0x28) / 120.0 + *(float *)(lVar8 + 0x1c4);
  fVar15 = 0.0;
  if (0.0 <= fVar14) {
    fVar15 = fVar14;
  }
  fVar14 = 1.0;
  if (fVar15 <= 1.0) {
    fVar14 = fVar15;
  }
  *(float *)(lVar8 + 0x1c4) = fVar14;
  if ((fVar14 < 0.24) || (0.25 <= fVar14)) {
    if ((0.49 <= fVar14) && (fVar14 <= 0.5)) {
      if (fVar14 <= 0.501) {
        fVar14 = 0.501;
      }
      goto LAB_140a72b54;
    }
    if ((0.74 <= fVar14) && (fVar14 <= 0.75)) {
      if (fVar14 <= 0.751) {
        fVar14 = 0.751;
      }
      goto LAB_140a72b54;
    }
  }
  else {
    if (fVar14 <= 0.251) {
      fVar14 = 0.251;
    }
LAB_140a72b54:
    *(float *)(lVar8 + 0x1c4) = fVar14;
  }
  ::utils::time_continuous_lerp();
  fVar15 = (float)(~-(uint)(0.99 < extraout_XMM0_Da) & (uint)extraout_XMM0_Da |
                  -(uint)(0.99 < extraout_XMM0_Da) & 0x3f800000);
  *(float *)(lVar8 + 0x1c0) = fVar15;
  if ((fVar15 < 0.24) || (0.25 <= fVar15)) {
    if ((0.49 <= fVar15) && (fVar15 <= 0.5)) {
      uVar16 = 0x3f004189;
      goto LAB_140a72bff;
    }
    if ((0.74 <= fVar15) && (fVar15 <= 0.75)) {
      uVar16 = 0x3f404189;
      goto LAB_140a72bff;
    }
  }
  else {
    uVar16 = 0x3e808312;
LAB_140a72bff:
    *(undefined4 *)(lVar8 + 0x1c0) = uVar16;
  }
  plVar12 = (longlong *)*in_RDX;
  if (*(char *)(*in_stack_00000030 + 0x100) == '\0') {
    if (*plVar12 != 0) {
      resources::audio_resources::AudioCoreInner::set_parameter();
    }
  }
  else if (*plVar12 != 0) {
    resources::audio_resources::AudioCoreInner::set_parameter();
  }
  fVar15 = *(float *)(lVar9 + 0x28);
  *puVar7 = uVar6;
  fVar15 = fVar15 + *(float *)(lVar8 + 0x1c8);
  fVar14 = 0.0;
  if (0.0 <= fVar15) {
    fVar14 = fVar15;
  }
  fVar15 = 0.8;
  if (fVar14 <= 0.8) {
    fVar15 = fVar14;
  }
  *(float *)(lVar8 + 0x1c8) = fVar15;
  if (*plVar12 != 0) {
    resources::audio_resources::AudioCoreInner::set_parameter();
  }
  if ((*(uint *)*in_R9 | 8) == 10) {
    *puVar7 = uVar6;
    music_player::MusicPlayer::update();
    if (*(char *)(*in_stack_00000048 + 1) == '\0') {
      *puVar7 = uVar6;
      music_player::MusicPlayer::stop_photomode_snapshots();
    }
    else {
      cVar5 = *(char *)(*in_stack_00000050 + 9);
      *puVar7 = uVar6;
      if (cVar5 == '\x02') {
        resources::audio_resources::AudioInstance::start_if_not_playing();
        if ((*(int *)(lVar8 + 0xa0) == 1) &&
           (libfmod::EventInstance::stop(), local_80 != -0x7ffffffffffffffc)) {
                    // WARNING: Subroutine does not return
          core::result::unwrap_failed();
        }
      }
      else {
        resources::audio_resources::AudioInstance::start_if_not_playing();
        if ((*(int *)(lVar8 + 0x70) == 1) &&
           (libfmod::EventInstance::stop(), local_80 != -0x7ffffffffffffffc)) {
                    // WARNING: Subroutine does not return
          core::result::unwrap_failed();
        }
      }
    }
    if ((*(int *)(lVar8 + 0x160) == 1) &&
       (libfmod::EventInstance::set_paused(), local_80 != -0x7ffffffffffffffc)) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    if ((*(int *)(lVar8 + 400) == 1) &&
       (libfmod::EventInstance::stop(), local_80 != -0x7ffffffffffffffc)) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
  }
  else {
    *puVar7 = uVar6;
    music_player::MusicPlayer::pause();
    *puVar7 = uVar6;
    music_player::MusicPlayer::stop_photomode_snapshots();
    if (*(char *)(lVar8 + 0x1cc) == '\x05') {
      if ((*(int *)(lVar8 + 400) == 1) &&
         (libfmod::EventInstance::stop(), local_80 != -0x7ffffffffffffffc)) {
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      lVar9 = *plVar12;
    }
    else {
      resources::audio_resources::AudioInstance::start_if_not_playing();
      lVar9 = *plVar12;
    }
    if (lVar9 != 0) {
      resources::audio_resources::AudioCoreInner::set_parameter();
    }
    resources::audio_resources::AudioInstance::start_if_not_playing();
    uVar13 = *(ulonglong *)(lVar8 + 0x160);
    if (uVar13 == 1) {
      libfmod::EventInstance::get_paused();
      if (local_80 != -0x7ffffffffffffffc) {
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      if (cStack_78 != '\x01') goto LAB_140a72ff7;
      uVar13 = *(ulonglong *)(lVar8 + 0x160);
    }
    if (((uVar13 & 1) != 0) &&
       (libfmod::EventInstance::set_paused(), local_80 != -0x7ffffffffffffffc)) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
  }
LAB_140a72ff7:
  if (extraout_AL != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_02 == 1) {
      plVar12 = extraout_RAX_02 + 1;
    }
    else if ((*extraout_RAX_02 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar12 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0)) {
LAB_140a73079:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar12 != 0) {
LAB_140a73085:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar12 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar12 = *plVar12 + 1;
  }
  return;
}

