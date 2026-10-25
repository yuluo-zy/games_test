
void __cdecl
country_core::resources::stairs::stairs_state::StairsState::extract_diff_and_make_backup(void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined *puVar3;
  longlong lVar4;
  longlong lVar5;
  void *_Dst;
  longlong *extraout_RAX;
  longlong *extraout_RAX_00;
  longlong lVar6;
  undefined8 *in_RCX;
  longlong in_RDX;
  undefined8 uStack_1a0;
  undefined8 local_198;
  longlong lStack_190;
  longlong local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined *local_118;
  undefined8 uStack_110;
  longlong local_108;
  undefined8 uStack_100;
  longlong local_f8;
  longlong lStack_f0;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 local_d0;
  undefined8 uStack_c8;
  undefined8 local_c0;
  undefined8 uStack_98;
  undefined8 local_90;
  void *local_88;
  longlong local_80;
  undefined1 local_72;
  undefined1 local_71;
  undefined8 local_70;
  
  local_70 = 0xfffffffffffffffe;
  local_72 = 1;
  iter_edges_and_stubs<country_core::resources::stairs::stairs_state::impl_5::take_snapshot_impl::closure_env_0>
            ();
  local_72 = 0;
  StairsSnapshot::new();
  if (((ulonglong)local_118 & 1) == 0) {
    ::utils::calculate_hash<country_core::resources::stairs::stairs_state::StairsSnapshot>();
    local_118 = (undefined *)0x1;
  }
  if (local_108 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  local_f8 = *extraout_RAX;
  lStack_f0 = extraout_RAX[1];
  *extraout_RAX = *extraout_RAX + 1;
  local_118 = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
  uStack_110 = 0;
  local_108 = 0;
  uStack_100 = 0;
  iter_edges_and_stubs<country_core::resources::stairs::stairs_state::impl_5::collect_edges::closure_env_0>
            ();
  lVar5 = lStack_f0;
  lVar4 = local_f8;
  uVar2 = uStack_100;
  lVar6 = local_108;
  uVar1 = uStack_110;
  puVar3 = local_118;
  local_71 = 1;
  compute_checksum();
  local_71 = 1;
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_00 == (longlong *)0x0) {
    local_71 = 1;
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  local_f8 = *extraout_RAX_00;
  lStack_f0 = extraout_RAX_00[1];
  *extraout_RAX_00 = *extraout_RAX_00 + 1;
  local_118 = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
  uStack_110 = 0;
  local_108 = 0;
  uStack_100 = 0;
  iter_edges_and_stubs<country_core::resources::stairs::stairs_state::impl_5::collect_edges::closure_env_0>
            ();
  local_71 = 0;
  local_118 = puVar3;
  uStack_110 = uVar1;
  local_108 = lVar6;
  uStack_100 = uVar2;
  local_f8 = lVar4;
  lStack_f0 = lVar5;
  reduce_diff();
  for (lVar6 = lStack_190; lVar6 != lStack_190 + local_188 * 8; lVar6 = lVar6 + 8) {
    hashbrown::map::HashMap::
    insert<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
              ();
  }
  indexmap::map::core::impl$0::
  clone<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>_>
            ();
  uVar1 = *(undefined8 *)(in_RDX + 0x60);
  uVar2 = *(undefined8 *)(in_RDX + 0x68);
  indexmap::map::core::impl$0::
  clone<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,tuple_<>_>
            ();
  uStack_98 = *(undefined8 *)(in_RDX + 0xa8);
  local_90 = *(undefined8 *)(in_RDX + 0xb0);
  local_d0 = 0;
  uStack_c8 = 4;
  local_c0 = 0;
  uStack_e0 = uVar1;
  local_d8 = uVar2;
  local_88 = (void *)(in_RDX + 0xb8);
  local_80 = in_RDX;
  core::ptr::
  drop_in_place<indexmap::map::core::IndexMapCore<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>_>_>
            ();
  lVar6 = *(longlong *)(local_80 + 0x120);
  if ((lVar6 != 0) && (lVar6 + (lVar6 * 8 + 0x17U & 0xfffffffffffffff0) != -0x11)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  _Dst = local_88;
  if (*(longlong *)(local_80 + 0x100) != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  memcpy(_Dst,&local_118,0x90);
  *(undefined1 *)(local_80 + 0x178) = 1;
  in_RCX[8] = local_178;
  in_RCX[9] = uStack_170;
  in_RCX[6] = local_188;
  in_RCX[7] = uStack_180;
  in_RCX[4] = local_198;
  in_RCX[5] = lStack_190;
  in_RCX[2] = 0;
  in_RCX[3] = uStack_1a0;
  *in_RCX = 0;
  in_RCX[1] = 4;
  return;
}

