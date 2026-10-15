// ADDR 0x14080ec10
// SIG void __cdecl cull_stair_attached_decorators(void)
// ORIG system::cull_stair_attached_decorators


void __cdecl system_decorator::cull_oob_decorators::cull_stair_attached_decorators(void)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  char extraout_AL;
  uint uVar5;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *plVar6;
  ulonglong uVar7;
  char *pcVar8;
  
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    uVar7 = 0x45;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar7 != 0) {
        if (uVar7 < 0x45) {
          if ((char)(&DAT_142a8e63b)[uVar7] < -0x40) {
LAB_14080ec8b:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar7 != 0x45) goto LAB_14080ec8b;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar7 != 0xfffffffffffffffe)) {
        if (uVar7 + 2 < 0x45) {
          if ((char)(&UNK_142a8e63d)[uVar7] < -0x40) {
LAB_14080ece9:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar7 + 2 != 0x45) goto LAB_14080ece9;
      }
    }
    pcVar8 = "assertion failed: decorator_archivist.is_decorator_cull_allowed(locator.decorator)";
    do {
      bVar2 = pcVar8[-1];
      uVar5 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = pcVar8[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = pcVar8[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = (byte *)(pcVar8 + -4);
            pcVar8 = pcVar8 + -4;
            uVar5 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            pcVar8 = pcVar8 + -3;
            uVar5 = bVar4 & 0xf;
          }
          uVar5 = bVar3 & 0x3f | uVar5 << 6;
        }
        else {
          pcVar8 = pcVar8 + -2;
          uVar5 = bVar3 & 0x1f;
        }
        uVar5 = (uint)(bVar2 & 0x3f) | uVar5 << 6;
        if (uVar5 != 0x5c) goto LAB_14080edb4;
LAB_14080edc6:
        if (pcVar8 == "") break;
        if (pcVar8 + -0x142a8e8af < (char *)0x33) {
          if (-0x41 < pcVar8[1]) break;
        }
        else if (pcVar8 + -0x142a8e8af == (char *)0x33) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      pcVar8 = pcVar8 + -1;
      if (uVar5 == 0x5c) goto LAB_14080edc6;
LAB_14080edb4:
      if (uVar5 == 0x2f) goto LAB_14080edc6;
    } while (pcVar8 != 
             "crates/systems/decorator/src/cull_oob_decorators.rsassertion failed: decorator_archivist.is_decorator_cull_allowed(locator.decorator)"
            );
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar6 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar6 = extraout_RAX_03, extraout_RAX_03 == (longlong *)0x0))
    goto LAB_14080ef3b;
    if (*plVar6 != 0) goto LAB_14080ef47;
    *plVar6 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar6 = *plVar6 + 1;
  }
  cull_stair_attached_decorators_impl<system_decorator::cull_oob_decorators::cull_stair_attached_decorators::closure_env_0>
            ();
  if (extraout_AL != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_02 == 1) {
      plVar6 = extraout_RAX_02 + 1;
    }
    else if ((*extraout_RAX_02 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar6 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0)) {
LAB_14080ef3b:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar6 != 0) {
LAB_14080ef47:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar6 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar6 = *plVar6 + 1;
  }
  return;
}


