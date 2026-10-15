// ADDR 0x140826d70
// SIG void __cdecl copy_blueprints_to_prev(void)
// ORIG system::copy_blueprints_to_prev


void __cdecl system_decorator::blueprint::copy_blueprints_to_prev(void)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  longlong lVar5;
  longlong lVar6;
  undefined8 uVar7;
  char extraout_AL;
  uint uVar8;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *in_RCX;
  longlong *plVar9;
  longlong *in_RDX;
  ulonglong uVar10;
  undefined *puVar11;
  
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    uVar10 = 0x34;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar10 != 0) {
        if (uVar10 < 0x34) {
          if ((char)(&DAT_142a913f0)[uVar10] < -0x40) {
LAB_140826ddc:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar10 != 0x34) goto LAB_140826ddc;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar10 != 0xfffffffffffffffe)) {
        if (uVar10 + 2 < 0x34) {
          if ((char)(&UNK_142a913f2)[uVar10] < -0x40) {
LAB_140826e3a:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar10 + 2 != 0x34) goto LAB_140826e3a;
      }
    }
    puVar11 = &DAT_142a91679;
    do {
      bVar2 = puVar11[-1];
      uVar8 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar11[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar11[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar11 + -4;
            puVar11 = puVar11 + -4;
            uVar8 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar11 = puVar11 + -3;
            uVar8 = bVar4 & 0xf;
          }
          uVar8 = bVar3 & 0x3f | uVar8 << 6;
        }
        else {
          puVar11 = puVar11 + -2;
          uVar8 = bVar3 & 0x1f;
        }
        uVar8 = (uint)(bVar2 & 0x3f) | uVar8 << 6;
        if (uVar8 != 0x5c) goto LAB_140826ef4;
LAB_140826f06:
        if (puVar11 == &UNK_142a9164f) break;
        if (puVar11 + -0x142a9164f < (undefined *)0x29) {
          if (-0x41 < (char)puVar11[1]) break;
        }
        else if (puVar11 + -0x142a9164f == (undefined *)0x29) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar11 = puVar11 + -1;
      if (uVar8 == 0x5c) goto LAB_140826f06;
LAB_140826ef4:
      if (uVar8 == 0x2f) goto LAB_140826f06;
    } while (puVar11 != &anon_607a6278b2141009247d0776e3b766a7_27_llvm_14222842113206979732);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar9 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar9 = extraout_RAX_03, extraout_RAX_03 == (longlong *)0x0))
    goto LAB_14082705f;
    if (*plVar9 != 0) goto LAB_14082706b;
    *plVar9 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar9 = *plVar9 + 1;
  }
  *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
  lVar5 = *in_RDX;
  lVar6 = *in_RCX;
  hashbrown::raw::impl$13::
  clone_from<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::sync::Arc<system_decorator::blueprint::DecoratorBlueprint,alloc::alloc::Global>_>,alloc::alloc::Global>
            ();
  uVar7 = *(undefined8 *)(lVar6 + 0x28);
  *(undefined8 *)(lVar5 + 0x20) = *(undefined8 *)(lVar6 + 0x20);
  *(undefined8 *)(lVar5 + 0x28) = uVar7;
  if (extraout_AL != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_02 == 1) {
      plVar9 = extraout_RAX_02 + 1;
    }
    else if ((*extraout_RAX_02 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar9 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0)) {
LAB_14082705f:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar9 != 0) {
LAB_14082706b:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar9 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar9 = *plVar9 + 1;
  }
  return;
}


