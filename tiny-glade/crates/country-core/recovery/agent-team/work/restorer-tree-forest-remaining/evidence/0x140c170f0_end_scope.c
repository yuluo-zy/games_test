// end_scope 0x140c170f0 size=646
// signature: void __cdecl end_scope(void)


void __cdecl puffin::ThreadProfiler::end_scope(void)

{
  ulonglong *puVar1;
  ulonglong uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  longlong lVar5;
  longlong lVar6;
  undefined **extraout_RAX;
  void *extraout_RAX_00;
  void *extraout_RAX_01;
  undefined8 *in_RCX;
  ulonglong uVar7;
  ulonglong in_RDX;
  undefined **ppuVar8;
  undefined **ppuVar9;
  undefined **ppuVar10;
  void *_Src;
  undefined8 local_b8;
  undefined8 local_b0;
  undefined **local_a8;
  void *local_a0;
  undefined **local_98;
  undefined **local_90;
  undefined8 uStack_88;
  undefined8 local_80;
  undefined8 uStack_78;
  undefined8 local_70;
  undefined4 uStack_68;
  undefined4 uStack_64;
  void *local_60;
  undefined **local_58;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  lVar5 = (*(code *)in_RCX[9])();
  uVar2 = in_RCX[0xb];
  uVar7 = in_RCX[6];
  if ((ulonglong)in_RCX[6] < uVar2) {
    uVar7 = uVar2;
  }
  in_RCX[6] = uVar7;
  in_RCX[5] = in_RCX[5] + 1;
  lVar6 = in_RCX[8];
  if ((longlong)in_RCX[8] < lVar5) {
    lVar6 = lVar5;
  }
  in_RCX[8] = lVar6;
  if (uVar2 == 0) {
    local_90 = &PTR_s_puffin_ERROR__Mismatched_scope_b_142b1ba48;
    uStack_88 = 1;
    local_80 = 8;
    uStack_78 = 0;
    local_70 = 0;
    std::io::stdio::_eprint();
  }
  else {
    in_RCX[0xb] = uVar2 - 1;
  }
  ppuVar8 = (undefined **)(in_RDX + 8);
  if (0xfffffffffffffff7 < in_RDX) {
                    // WARNING: Subroutine does not return
    core::slice::index::slice_index_order_fail();
  }
  ppuVar9 = (undefined **)in_RCX[4];
  if (ppuVar9 < ppuVar8) {
                    // WARNING: Subroutine does not return
    core::slice::index::slice_end_index_len_fail();
  }
  puVar1 = in_RCX + 4;
  lVar6 = in_RCX[3];
  *(longlong *)(lVar6 + in_RDX) = (longlong)ppuVar9 - (longlong)ppuVar8;
  if (ppuVar9 == (undefined **)in_RCX[2]) {
    ppuVar8 = &PTR_s__home_h3__cargo_registry_src_ind_142b1b9e8;
    alloc::raw_vec::RawVec::grow_one<u8,alloc::alloc::Global>();
    lVar6 = in_RCX[3];
  }
  *(undefined1 *)(lVar6 + (longlong)ppuVar9) = 0x29;
  ppuVar9 = (undefined **)((longlong)ppuVar9 + 1);
  *puVar1 = (ulonglong)ppuVar9;
  ppuVar10 = ppuVar9;
  if ((ulonglong)(in_RCX[2] - (longlong)ppuVar9) < 8) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    ppuVar10 = (undefined **)*puVar1;
    ppuVar8 = ppuVar9;
  }
  *(longlong *)(in_RCX[3] + (longlong)ppuVar10) = lVar5;
  in_RCX[4] = ppuVar10 + 1;
  if (in_RCX[0xb] == 0) {
    uVar3 = *in_RCX;
    uVar4 = in_RCX[1];
    std::thread::current::current();
    local_90 = extraout_RAX;
    std::thread::Thread::name();
    ppuVar9 = ppuVar8;
    if (extraout_RAX_00 == (void *)0x0) {
      ppuVar9 = (undefined **)0x0;
    }
    local_60 = (void *)0x1;
    _Src = extraout_RAX_00;
    if (extraout_RAX_00 == (void *)0x0) {
      _Src = (void *)0x1;
    }
    if ((longlong)ppuVar9 < 0) {
LAB_140c17364:
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    if (ppuVar9 == (undefined **)0x0) {
      local_58 = (undefined **)0x0;
    }
    else {
      local_58 = ppuVar8;
      std::alloc::__default_lib_allocator::__rust_alloc();
      local_60 = extraout_RAX_01;
      if (extraout_RAX_01 == (void *)0x0) goto LAB_140c17364;
    }
    memcpy(local_60,_Src,(size_t)ppuVar9);
    LOCK();
    *local_90 = *local_90 + -1;
    UNLOCK();
    if (*local_90 == (undefined *)0x0) {
      local_49 = 1;
      alloc::sync::Arc::drop_slow<std::thread::Inner,alloc::alloc::Global>();
    }
    local_a8 = local_58;
    local_a0 = local_60;
    local_90 = (undefined **)in_RCX[3];
    uStack_88 = in_RCX[4];
    local_80 = in_RCX[5];
    uStack_78 = in_RCX[6];
    local_70 = in_RCX[7];
    uStack_68 = *(undefined4 *)(in_RCX + 8);
    uStack_64 = *(undefined4 *)((longlong)in_RCX + 0x44);
    local_49 = 0;
    local_b8 = uVar3;
    local_b0 = uVar4;
    local_98 = ppuVar9;
    (*(code *)in_RCX[10])(&local_b8,&local_90);
    *puVar1 = 0;
    in_RCX[5] = 0;
    in_RCX[6] = 0;
    in_RCX[7] = 0x7fffffffffffffff;
    in_RCX[8] = 0x8000000000000000;
  }
  return;
}

