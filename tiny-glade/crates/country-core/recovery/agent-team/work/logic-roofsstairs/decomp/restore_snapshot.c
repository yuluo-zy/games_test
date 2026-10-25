
void __cdecl country_core::resources::stairs::stairs_state::StairsState::restore_snapshot(void)

{
  ulonglong uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined *puVar4;
  undefined8 uVar5;
  undefined **ppuVar6;
  void *_Dst;
  undefined8 extraout_RAX;
  longlong *extraout_RAX_00;
  byte *extraout_RAX_01;
  longlong extraout_RAX_02;
  ulonglong uVar7;
  longlong in_RCX;
  byte *in_RDX;
  undefined **ppuVar8;
  longlong lVar9;
  longlong lVar10;
  longlong lVar11;
  longlong lVar12;
  longlong local_220;
  longlong local_208;
  undefined *local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  longlong local_d8;
  undefined **ppuStack_d0;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_78;
  undefined8 local_70;
  void *local_68;
  longlong local_60;
  byte *local_58;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_58 = in_RDX;
  compute_checksum();
  if ((*local_58 & 1) == 0) {
    ::utils::calculate_hash<country_core::resources::stairs::stairs_state::StairsSnapshot>();
    local_58[0] = 1;
    local_58[1] = 0;
    local_58[2] = 0;
    local_58[3] = 0;
    local_58[4] = 0;
    local_58[5] = 0;
    local_58[6] = 0;
    local_58[7] = 0;
    *(undefined8 *)(local_58 + 8) = extraout_RAX;
  }
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_00 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  lVar9 = in_RCX + 0x28;
  local_d8 = *extraout_RAX_00;
  ppuStack_d0 = (undefined **)extraout_RAX_00[1];
  *extraout_RAX_00 = *extraout_RAX_00 + 1;
  local_f8 = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
  uStack_f0 = 0;
  local_e8 = 0;
  uStack_e0 = 0;
  ppuVar8 = &local_f8;
  iter_edges_and_stubs<country_core::resources::stairs::stairs_state::impl_5::collect_edges::closure_env_0>
            ();
  ppuVar6 = ppuStack_d0;
  lVar10 = local_d8;
  uVar5 = uStack_e0;
  uVar3 = local_e8;
  uVar2 = uStack_f0;
  puVar4 = local_f8;
  lVar11 = *(longlong *)(local_58 + 0x20);
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_01 & 1) == 0) {
    local_49 = 1;
    std::sys::random::hashmap_random_keys();
    extraout_RAX_01[0] = 1;
    extraout_RAX_01[1] = 0;
    extraout_RAX_01[2] = 0;
    extraout_RAX_01[3] = 0;
    extraout_RAX_01[4] = 0;
    extraout_RAX_01[5] = 0;
    extraout_RAX_01[6] = 0;
    extraout_RAX_01[7] = 0;
    *(longlong *)(extraout_RAX_01 + 8) = extraout_RAX_02;
    *(undefined ***)(extraout_RAX_01 + 0x10) = ppuVar8;
    local_d8 = extraout_RAX_02;
  }
  else {
    local_d8 = *(longlong *)(extraout_RAX_01 + 8);
    ppuVar8 = *(undefined ***)(extraout_RAX_01 + 0x10);
  }
  *(longlong *)(extraout_RAX_01 + 8) = local_d8 + 1;
  local_e8 = 0;
  uStack_e0 = 0;
  local_f8 = &anon_c925c76b833903a453982b6800d1a02b_0_llvm_8881321364459548795;
  uStack_f0 = 0;
  if (lVar11 != 0) {
    ppuStack_d0 = ppuVar8;
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple_<>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple_<>,std::hash::random::RandomState>_>
              ();
    lVar12 = 0;
    do {
      hashbrown::map::HashMap::
      insert<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                ();
      lVar12 = lVar12 + 1;
    } while (lVar12 != lVar11);
  }
  local_49 = 0;
  local_f8 = puVar4;
  uStack_f0 = uVar2;
  local_e8 = uVar3;
  uStack_e0 = uVar5;
  local_d8 = lVar10;
  ppuStack_d0 = ppuVar6;
  reduce_diff();
  apply_diff();
  iter_edges_and_stubs<country_core::resources::stairs::stairs_state::impl_5::restore_backup::closure_env_0>
            ();
  if (*(longlong *)(in_RCX + 0xe8) != 0) {
    uVar1 = *(ulonglong *)(in_RCX + 0xd8);
    if (uVar1 == 0) {
      uVar7 = 0;
    }
    else {
      lVar9 = CONCAT71((int7)((ulonglong)lVar9 >> 8),0xff);
      memset(*(void **)(in_RCX + 0xd0),(int)lVar9,uVar1 + 0x11);
      uVar7 = (uVar1 + 1 & 0xfffffffffffffff8) - (uVar1 + 1 >> 3);
      if (uVar1 < 8) {
        uVar7 = uVar1;
      }
    }
    *(undefined8 *)(in_RCX + 0xe8) = 0;
    *(ulonglong *)(in_RCX + 0xe0) = uVar7;
  }
  lVar11 = *(longlong *)(in_RCX + 200);
  *(undefined8 *)(in_RCX + 200) = 0;
  if (lVar11 != 0) {
    lVar10 = *(longlong *)(in_RCX + 0xc0) + 8;
    do {
      lVar9 = 0;
      if (*(longlong *)(lVar10 + -8) != 0) {
        lVar9 = *(longlong *)(lVar10 + -8) << 3;
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      lVar10 = lVar10 + 0x28;
      lVar11 = lVar11 + -1;
    } while (lVar11 != 0);
  }
  if (*(longlong *)(in_RCX + 0x130) != 0) {
    uVar1 = *(ulonglong *)(in_RCX + 0x120);
    if (uVar1 == 0) {
      uVar7 = 0;
    }
    else {
      memset(*(void **)(in_RCX + 0x118),(int)CONCAT71((int7)((ulonglong)lVar9 >> 8),0xff),
             uVar1 + 0x11);
      uVar7 = (uVar1 + 1 & 0xfffffffffffffff8) - (uVar1 + 1 >> 3);
      if (uVar1 < 8) {
        uVar7 = uVar1;
      }
    }
    *(undefined8 *)(in_RCX + 0x130) = 0;
    *(ulonglong *)(in_RCX + 0x128) = uVar7;
  }
  *(undefined8 *)(in_RCX + 0x110) = 0;
  *(undefined1 *)(in_RCX + 0x178) = 0;
  iter_edges_and_stubs<country_core::resources::stairs::stairs_state::impl_5::restore_backup::closure_env_0>
            ();
  indexmap::map::core::impl$0::
  clone<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>_>
            ();
  uVar2 = *(undefined8 *)(in_RCX + 0x60);
  uVar3 = *(undefined8 *)(in_RCX + 0x68);
  indexmap::map::core::impl$0::
  clone<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,tuple_<>_>
            ();
  uStack_78 = *(undefined8 *)(in_RCX + 0xa8);
  local_70 = *(undefined8 *)(in_RCX + 0xb0);
  uStack_c0 = uVar2;
  local_b8 = uVar3;
  local_68 = (void *)(in_RCX + 0xb8);
  local_60 = in_RCX;
  core::ptr::
  drop_in_place<indexmap::map::core::IndexMapCore<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>_>_>
            ();
  lVar9 = *(longlong *)(local_60 + 0x120);
  if ((lVar9 != 0) && (lVar9 + (lVar9 * 8 + 0x17U & 0xfffffffffffffff0) != -0x11)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  _Dst = local_68;
  if (*(longlong *)(local_60 + 0x100) != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  memcpy(_Dst,&local_f8,0x90);
  *(undefined1 *)(local_60 + 0x178) = 1;
  if (local_220 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_208 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (*(longlong *)(local_58 + 0x10) != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

