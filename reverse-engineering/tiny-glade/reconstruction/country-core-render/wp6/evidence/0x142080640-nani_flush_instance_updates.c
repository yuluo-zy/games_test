// address 0x142080640
// name nani_flush_instance_updates
// signature void __cdecl nani_flush_instance_updates(void)


void __cdecl country_core::render::nani::systems::nani_flush_instance_updates(void)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  longlong lVar5;
  longlong lVar6;
  char extraout_AL;
  uint uVar7;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  undefined8 *in_RCX;
  longlong *plVar8;
  longlong *in_RDX;
  ulonglong uVar9;
  longlong lVar10;
  undefined *puVar11;
  longlong lVar12;
  
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    uVar9 = 0x40;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar9 != 0) {
        if (uVar9 < 0x40) {
          if ((char)(&DAT_142fd328a)[uVar9] < -0x40) {
LAB_1420806ac:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar9 != 0x40) goto LAB_1420806ac;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar9 != 0xfffffffffffffffe)) {
        if (uVar9 + 2 < 0x40) {
          if ((char)(&UNK_142fd328c)[uVar9] < -0x40) {
LAB_14208070a:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar9 + 2 != 0x40) goto LAB_14208070a;
      }
    }
    puVar11 = &DAT_142fd345e;
    do {
      bVar2 = puVar11[-1];
      uVar7 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar11[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar11[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar11 + -4;
            puVar11 = puVar11 + -4;
            uVar7 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar11 = puVar11 + -3;
            uVar7 = bVar4 & 0xf;
          }
          uVar7 = bVar3 & 0x3f | uVar7 << 6;
        }
        else {
          puVar11 = puVar11 + -2;
          uVar7 = bVar3 & 0x1f;
        }
        uVar7 = (uint)(bVar2 & 0x3f) | uVar7 << 6;
        if (uVar7 != 0x5c) goto LAB_1420807c4;
LAB_1420807d6:
        if (puVar11 == &UNK_142fd342f) break;
        if (puVar11 + -0x142fd342f < (undefined *)0x2e) {
          if (-0x41 < (char)puVar11[1]) break;
        }
        else if (puVar11 + -0x142fd342f == (undefined *)0x2e) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar11 = puVar11 + -1;
      if (uVar7 == 0x5c) goto LAB_1420807d6;
LAB_1420807c4:
      if (uVar7 == 0x2f) goto LAB_1420807d6;
    } while (puVar11 != &DAT_142fd3430);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar8 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar8 = extraout_RAX_03, extraout_RAX_03 == (longlong *)0x0))
    goto LAB_142080989;
    if (*plVar8 != 0) goto LAB_142080995;
    *plVar8 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar8 = *plVar8 + 1;
  }
  *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
  lVar5 = *(longlong *)*in_RCX;
  lVar6 = *in_RDX;
  lVar10 = lVar6 + 0x50;
  for (lVar12 = 0x1440; lVar12 != 0; lVar12 = lVar12 + -0xc0) {
    if (-0x7fffffffffffffff < *(longlong *)(lVar10 + -0x48)) {
      shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::buffers::GpuInstance>()
      ;
      (**(code **)(*(longlong *)(lVar10 + 0x50) + 0x30))
                (*(undefined8 *)(lVar10 + 0x48),lVar5 + 0x10);
    }
    lVar10 = lVar10 + 0xc0;
  }
  *(undefined8 *)(lVar6 + 0x1448) = *(undefined8 *)(lVar6 + 0x1440);
  *(undefined8 *)(lVar6 + 0x1440) = 0;
  if (extraout_AL != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_02 == 1) {
      plVar8 = extraout_RAX_02 + 1;
    }
    else if ((*extraout_RAX_02 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar8 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0)) {
LAB_142080989:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar8 != 0) {
LAB_142080995:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar8 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar8 = *plVar8 + 1;
  }
  return;
}

