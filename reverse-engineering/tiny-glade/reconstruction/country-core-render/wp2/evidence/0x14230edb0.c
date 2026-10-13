// address 0x14230edb0  name cpu_write<country_core::systems::render_loop::resources::critters::ButterflySsbo>
// signature: void __cdecl cpu_write<country_core::systems::render_loop::resources::critters::ButterflySsbo>(void)

void __cdecl
country_core::render::ssbo::impl$6::
cpu_write<country_core::systems::render_loop::resources::critters::ButterflySsbo>(void)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  ulonglong uVar5;
  char extraout_AL;
  uint uVar6;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  undefined8 *in_RCX;
  longlong *plVar7;
  undefined1 *in_RDX;
  undefined1 *puVar8;
  ulonglong in_R8;
  undefined *puVar9;
  ulonglong uVar10;
  
  puVar8 = in_RDX;
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (puVar8 != (undefined1 *)0x0) {
        if (&DAT_00000074 < puVar8) {
          if (puVar8 != &DAT_00000075) goto LAB_14230ee1f;
        }
        else if ((char)puVar8[0x14304a670] < -0x40) {
LAB_14230ee1f:
                    // WARNING: Subroutine does not return
          core::str::slice_error_fail();
        }
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (puVar8 != (undefined1 *)0xfffffffffffffffe)) {
        if (puVar8 + 2 < (undefined1 *)0x75) {
          if ((char)puVar8[0x14304a672] < -0x40) {
LAB_14230ee73:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (puVar8 + 2 != (undefined1 *)0x75) goto LAB_14230ee73;
      }
    }
    puVar9 = &DAT_14304a5a4;
    do {
      bVar2 = puVar9[-1];
      uVar6 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar9[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar9[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar9 + -4;
            puVar9 = puVar9 + -4;
            uVar6 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar9 = puVar9 + -3;
            uVar6 = bVar4 & 0xf;
          }
          uVar6 = bVar3 & 0x3f | uVar6 << 6;
        }
        else {
          puVar9 = puVar9 + -2;
          uVar6 = bVar3 & 0x1f;
        }
        uVar6 = (uint)(bVar2 & 0x3f) | uVar6 << 6;
        if (uVar6 != 0x5c) goto LAB_14230ef34;
LAB_14230ef4f:
        if (puVar9 == &UNK_14304a57d) break;
        if (puVar9 + -0x14304a57d < (undefined *)0x26) {
          if (-0x41 < (char)puVar9[1]) break;
        }
        else if (puVar9 + -0x14304a57d == (undefined *)0x26) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar9 = puVar9 + -1;
      if (uVar6 == 0x5c) goto LAB_14230ef4f;
LAB_14230ef34:
      if (uVar6 == 0x2f) goto LAB_14230ef4f;
    } while (puVar9 != &DAT_14304a57e);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar7 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar7 = extraout_RAX_03, extraout_RAX_03 == (longlong *)0x0))
    goto LAB_14230f0c1;
    if (*plVar7 != 0) goto LAB_14230f0e8;
    *plVar7 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar7 = *plVar7 + 1;
  }
  uVar5 = in_RCX[1];
  uVar10 = uVar5;
  if (in_R8 < uVar5) {
    uVar10 = in_R8;
  }
  if (uVar5 < in_R8) {
                    // WARNING: Subroutine does not return
    core::slice::impl$0::copy_from_slice::len_mismatch_fail();
  }
  memcpy((void *)*in_RCX,in_RDX,uVar10 << 5);
  if (extraout_AL != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_02 == 1) {
      plVar7 = extraout_RAX_02 + 1;
    }
    else if ((*extraout_RAX_02 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar7 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0)) {
LAB_14230f0c1:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar7 != 0) {
LAB_14230f0e8:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar7 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar7 = *plVar7 + 1;
  }
  return;
}


