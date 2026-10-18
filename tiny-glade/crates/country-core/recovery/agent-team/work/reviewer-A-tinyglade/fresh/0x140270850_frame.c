// frame @ 0x140270850 size=1169
// sig: void __cdecl frame(void)


void __cdecl tiny_glade::fps_limiter::FpsLimiter::frame(void)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  undefined1 auVar5 [16];
  bool bVar6;
  bool bVar7;
  undefined *puVar8;
  char extraout_AL;
  undefined8 extraout_RAX;
  char *pcVar9;
  longlong *extraout_RAX_00;
  ulonglong extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  uint uVar10;
  undefined8 *in_RCX;
  longlong *plVar11;
  undefined4 in_EDX;
  int iVar12;
  float fVar13;
  float in_XMM1_Da;
  float fVar14;
  float fVar15;
  float fVar16;
  
  std::time::Instant::now();
  if (0.0 < in_XMM1_Da) {
    puffin::are_scopes_on();
    if (extraout_AL == '\0') {
      bVar6 = false;
      bVar7 = false;
      iVar12 = *(int *)(in_RCX + 1);
    }
    else {
      pcVar9 = "fps limiter";
      do {
        bVar2 = pcVar9[-1];
        uVar10 = (uint)(char)bVar2;
        if ((char)bVar2 < '\0') {
          bVar3 = pcVar9[-2];
          if ((char)bVar3 < -0x40) {
            bVar4 = pcVar9[-3];
            if ((char)bVar4 < -0x40) {
              pbVar1 = (byte *)(pcVar9 + -4);
              pcVar9 = pcVar9 + -4;
              uVar10 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
            }
            else {
              pcVar9 = pcVar9 + -3;
              uVar10 = bVar4 & 0xf;
            }
            uVar10 = bVar3 & 0x3f | uVar10 << 6;
          }
          else {
            pcVar9 = pcVar9 + -2;
            uVar10 = bVar3 & 0x1f;
          }
          uVar10 = (uint)(bVar2 & 0x3f) | uVar10 << 6;
          if (uVar10 != 0x5c) goto LAB_140270929;
LAB_140270963:
          if (pcVar9 == "") break;
          if (pcVar9 + -0x1429a7d37 < (char *)0x24) {
            if (-0x41 < pcVar9[1]) break;
          }
          else if (pcVar9 + -0x1429a7d37 == (char *)0x24) break;
                    // WARNING: Subroutine does not return
          core::str::slice_error_fail();
        }
        pcVar9 = pcVar9 + -1;
        if (uVar10 == 0x5c) goto LAB_140270963;
LAB_140270929:
        if (uVar10 == 0x2f) goto LAB_140270963;
      } while (pcVar9 != "crates/tiny-glade/src/fps_limiter.rsfps limiter");
      puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
      if (*extraout_RAX_00 == 1) {
        plVar11 = extraout_RAX_00 + 1;
      }
      else if ((*extraout_RAX_00 != 0) ||
              (std::sys::thread_local::native::lazy::Storage::
               initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                         (), plVar11 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0))
      goto LAB_140270ca2;
      if (*plVar11 != 0) goto LAB_140270cae;
      *plVar11 = -1;
      puffin::ThreadProfiler::begin_scope();
      *plVar11 = *plVar11 + 1;
      bVar6 = true;
      bVar7 = true;
      iVar12 = *(int *)(in_RCX + 1);
    }
    if (iVar12 != 1000000000) {
      iVar12 = (int)*in_RCX;
      std::time::Instant::duration_since();
      puVar8 = __imp__ZN10spin_sleep15MIN_TIME_PERIOD17he1a7293e73dba3c8E;
      fVar16 = 1.0 / in_XMM1_Da;
      fVar14 = (float)iVar12 / 1e+09 + (float)extraout_RAX_01;
      fVar15 = fVar16 + fVar16;
      fVar13 = fVar15;
      if (fVar14 <= fVar15) {
        fVar13 = fVar14;
      }
      fVar14 = (*(float *)(in_RCX + 2) + fVar16) -
               (float)(~-(uint)NAN(fVar14) & (uint)fVar13 | -(uint)NAN(fVar14) & (uint)fVar15);
      fVar13 = -fVar16;
      if (-fVar16 <= fVar14) {
        fVar13 = fVar14;
      }
      if (fVar13 <= fVar16) {
        fVar16 = fVar13;
      }
      if (fVar16 <= 0.0) {
        *(float *)(in_RCX + 2) = fVar16;
      }
      else {
        if (0x2fffffff < (uint)fVar16) {
          if ((uint)fVar16 < 0x3f800000) {
            auVar5._8_8_ = 0;
            auVar5._0_8_ = (ulonglong)((uint)fVar16 & 0x7fffff | 0x800000) <<
                           ((char)((uint)fVar16 >> 0x17) + 0x2aU & 0x3f);
            auVar5 = auVar5 * ZEXT816(1000000000);
            if ((uint)(byte)((auVar5[8] | !SBORROW8(0,auVar5._0_8_)) & auVar5[7] >> 7) +
                auVar5._8_4_ == 1000000000) goto LAB_140270cd4;
          }
          else if ((0x4affffff < (uint)fVar16) && (0x5f7fffff < (uint)fVar16)) {
LAB_140270cd4:
                    // WARNING: Subroutine does not return
            core::time::impl$0::from_secs_f32::panic_cold_display<ref_<str_>_>();
          }
        }
        if (*(longlong *)__imp__ZN10spin_sleep15MIN_TIME_PERIOD17he1a7293e73dba3c8E != 2) {
          once_cell::imp::OnceCell::
          initialize<u32,once_cell::sync::impl_6::get_or_init::closure_env_0<u32,once_cell::sync::impl_11::force::closure_env_0<u32,u32_(*)()>_>,enum2_<once_cell::sync::impl_6::get_or_init::Void>_>
                    ();
        }
        timeBeginPeriod(*(UINT *)(puVar8 + 0xc));
        std::thread::sleep();
        if (*(longlong *)puVar8 != 2) {
          once_cell::imp::OnceCell::
          initialize<u32,once_cell::sync::impl_6::get_or_init::closure_env_0<u32,once_cell::sync::impl_11::force::closure_env_0<u32,u32_(*)()>_>,enum2_<once_cell::sync::impl_6::get_or_init::Void>_>
                    ();
        }
        timeEndPeriod(*(UINT *)(puVar8 + 0xc));
        *(float *)(in_RCX + 2) = fVar16;
        bVar6 = bVar7;
      }
    }
    if (bVar6) {
      puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
      if (*extraout_RAX_02 == 1) {
        plVar11 = extraout_RAX_02 + 1;
      }
      else if ((*extraout_RAX_02 != 0) ||
              (std::sys::thread_local::native::lazy::Storage::
               initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                         (), plVar11 = extraout_RAX_03, extraout_RAX_03 == (longlong *)0x0)) {
LAB_140270ca2:
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      if (*plVar11 != 0) {
LAB_140270cae:
                    // WARNING: Subroutine does not return
        core::cell::panic_already_borrowed();
      }
      *plVar11 = -1;
      puffin::ThreadProfiler::end_scope();
      *plVar11 = *plVar11 + 1;
    }
  }
  *in_RCX = extraout_RAX;
  *(undefined4 *)(in_RCX + 1) = in_EDX;
  return;
}


