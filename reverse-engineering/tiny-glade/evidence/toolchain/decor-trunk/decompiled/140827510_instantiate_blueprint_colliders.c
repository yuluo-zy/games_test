// ADDR 0x140827510
// SIG void __cdecl instantiate_blueprint_colliders(void)
// ORIG system::instantiate_blueprint_colliders


void __cdecl system_decorator::blueprint::instantiate_blueprint_colliders(void)

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
  longlong in_RCX;
  longlong *plVar6;
  longlong in_RDX;
  ulonglong uVar7;
  undefined *puVar8;
  
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    uVar7 = 0x3c;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar7 != 0) {
        if (uVar7 < 0x3c) {
          if ((char)(&DAT_142a914cc)[uVar7] < -0x40) {
LAB_140827585:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar7 != 0x3c) goto LAB_140827585;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar7 != 0xfffffffffffffffe)) {
        if (uVar7 + 2 < 0x3c) {
          if ((char)(&UNK_142a914ce)[uVar7] < -0x40) {
LAB_1408275e3:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar7 + 2 != 0x3c) goto LAB_1408275e3;
      }
    }
    puVar8 = &DAT_142a91679;
    do {
      bVar2 = puVar8[-1];
      uVar5 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar8[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar8[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar8 + -4;
            puVar8 = puVar8 + -4;
            uVar5 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar8 = puVar8 + -3;
            uVar5 = bVar4 & 0xf;
          }
          uVar5 = bVar3 & 0x3f | uVar5 << 6;
        }
        else {
          puVar8 = puVar8 + -2;
          uVar5 = bVar3 & 0x1f;
        }
        uVar5 = (uint)(bVar2 & 0x3f) | uVar5 << 6;
        if (uVar5 != 0x5c) goto LAB_1408276a4;
LAB_1408276b6:
        if (puVar8 == &UNK_142a9164f) break;
        if (puVar8 + -0x142a9164f < (undefined *)0x29) {
          if (-0x41 < (char)puVar8[1]) break;
        }
        else if (puVar8 + -0x142a9164f == (undefined *)0x29) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar8 = puVar8 + -1;
      if (uVar5 == 0x5c) goto LAB_1408276b6;
LAB_1408276a4:
      if (uVar5 == 0x2f) goto LAB_1408276b6;
    } while (puVar8 != &anon_607a6278b2141009247d0776e3b766a7_27_llvm_14222842113206979732);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar6 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar6 = extraout_RAX_03, extraout_RAX_03 == (longlong *)0x0))
    goto LAB_140827815;
    if (*plVar6 != 0) goto LAB_140827821;
    *plVar6 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar6 = *plVar6 + 1;
  }
  **(undefined4 **)(in_RDX + 0x10) = *(undefined4 *)(in_RDX + 0x1c);
  **(undefined4 **)(in_RCX + 0x10) = *(undefined4 *)(in_RCX + 0x1c);
  instantiate_decorator_colliders();
  if (extraout_AL != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_02 == 1) {
      plVar6 = extraout_RAX_02 + 1;
    }
    else if ((*extraout_RAX_02 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar6 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0)) {
LAB_140827815:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar6 != 0) {
LAB_140827821:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar6 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar6 = *plVar6 + 1;
  }
  return;
}


