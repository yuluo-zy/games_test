// name: emit_fences<system_wall_constructor::construct_fences::construct_fences::closure_env$0>
// addr: 0x1412388f0
// size: 914
// signature: void __cdecl emit_fences<system_wall_constructor::construct_fences::construct_fences::closure_env$0>(void)


void __cdecl
system_wall_constructor::construct_fences::process_segment::
emit_fences<system_wall_constructor::construct_fences::construct_fences::closure_env_0>(void)

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
  undefined8 *in_RCX;
  longlong *plVar6;
  ulonglong in_RDX;
  longlong in_R8;
  undefined *puVar7;
  undefined4 local_10c [5];
  undefined8 *local_f8;
  undefined8 *local_f0;
  undefined1 *local_e8;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined4 *local_c8;
  undefined8 *local_b8;
  undefined8 local_b0;
  undefined1 local_90 [8];
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 local_70;
  undefined8 uStack_68;
  undefined8 local_60;
  longlong *local_58;
  longlong *local_50;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  puffin::are_scopes_on();
  local_50 = (longlong *)CONCAT71(local_50._1_7_,extraout_AL);
  if (extraout_AL == '\0') {
    local_b0 = 0;
  }
  else {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (in_RDX != 0) {
        if (in_RDX < 0x47) {
          if ((char)(&DAT_142c1ebf0)[in_RDX] < -0x40) {
LAB_14123896c:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX != 0x47) goto LAB_14123896c;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (in_RDX != 0xfffffffffffffffe)) {
        if (in_RDX + 2 < 0x47) {
          if ("stem_wall_constructor::construct_fences::process_segment::emit_fences::f"[in_RDX] <
              -0x40) {
LAB_1412389c0:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX + 2 != 0x47) goto LAB_1412389c0;
      }
    }
    puVar7 = &DAT_142c1ed4f;
    do {
      bVar2 = puVar7[-1];
      uVar5 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar7[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar7[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar7 + -4;
            puVar7 = puVar7 + -4;
            uVar5 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar7 = puVar7 + -3;
            uVar5 = bVar4 & 0xf;
          }
          uVar5 = bVar3 & 0x3f | uVar5 << 6;
        }
        else {
          puVar7 = puVar7 + -2;
          uVar5 = bVar3 & 0x1f;
        }
        uVar5 = (uint)(bVar2 & 0x3f) | uVar5 << 6;
        if (uVar5 != 0x5c) goto LAB_141238a84;
LAB_141238a96:
        if (puVar7 == &UNK_142c1ed07) break;
        if (puVar7 + -0x142c1ed07 < (undefined *)0x47) {
          if (-0x41 < (char)puVar7[1]) break;
        }
        else if (puVar7 + -0x142c1ed07 == (undefined *)0x47) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar7 = puVar7 + -1;
      if (uVar5 == 0x5c) goto LAB_141238a96;
LAB_141238a84:
      if (uVar5 == 0x2f) goto LAB_141238a96;
    } while (puVar7 != &anon_55ff71ed5194c636fa55bcf52a35003b_15_llvm_1213158418736262939);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar6 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar6 = extraout_RAX_03, extraout_RAX_03 == (longlong *)0x0))
    goto LAB_141238c77;
    if (*plVar6 != 0) goto LAB_141238c83;
    *plVar6 = -1;
    local_58 = plVar6;
    puffin::ThreadProfiler::begin_scope();
    *local_58 = *local_58 + 1;
    local_b0 = 1;
  }
  local_88 = 0;
  uStack_80 = 0;
  local_78 = 0;
  local_f0 = &local_78;
  local_70 = 0;
  uStack_68 = 8;
  local_60 = 0;
  local_d8 = *(undefined8 *)(in_R8 + 0x90);
  uStack_d0 = *(undefined8 *)(in_R8 + 0x98);
  local_10c[0] = 0;
  local_f8 = &local_88;
  local_e8 = local_90;
  local_c8 = local_10c;
  local_b8 = &local_70;
  ::utils::threshold_range::
  threshold_range<system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env_0<system_wall_constructor::construct_fences::construct_fences::closure_env_0>,system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env_1<system_wall_constructor::construct_fences::construct_fences::closure_env_0>_>
            ();
  in_RCX[2] = local_78;
  *in_RCX = local_88;
  in_RCX[1] = uStack_80;
  in_RCX[3] = local_70;
  in_RCX[4] = uStack_68;
  in_RCX[5] = local_60;
  if ((char)local_50 != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_02 == 1) {
      plVar6 = extraout_RAX_02 + 1;
    }
    else if ((*extraout_RAX_02 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar6 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0)) {
LAB_141238c77:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar6 != 0) {
LAB_141238c83:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar6 = -1;
    local_50 = plVar6;
    puffin::ThreadProfiler::end_scope();
    *local_50 = *local_50 + 1;
  }
  return;
}

