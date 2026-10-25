
void __cdecl country_core::resources::stairs::stairs_state::StairsState::make_backup(void)

{
  longlong lVar1;
  void *_Dst;
  longlong in_RCX;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 local_110;
  undefined8 uStack_108;
  undefined8 local_100;
  undefined8 uStack_f8;
  undefined8 local_f0;
  undefined8 uStack_e8;
  undefined8 local_e0;
  undefined8 uStack_d8;
  undefined8 local_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 local_90;
  undefined8 uStack_88;
  undefined8 local_80;
  undefined8 uStack_78;
  undefined8 local_70;
  undefined8 uStack_68;
  undefined8 local_60;
  undefined8 uStack_58;
  undefined8 uStack_50;
  undefined8 local_48;
  undefined8 uStack_40;
  void *local_38;
  longlong local_30;
  undefined8 local_28;
  
  local_28 = 0xfffffffffffffffe;
  if (*(char *)(in_RCX + 0x178) == '\0') {
    iter_edges_and_stubs<country_core::resources::stairs::stairs_state::impl_5::restore_backup::closure_env_0>
              ();
    indexmap::map::core::impl$0::
    clone<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>_>
              ();
    local_48 = *(undefined8 *)(in_RCX + 0x60);
    uStack_40 = *(undefined8 *)(in_RCX + 0x68);
    indexmap::map::core::impl$0::
    clone<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,tuple_<>_>
              ();
    uStack_d8 = *(undefined8 *)(in_RCX + 0xa8);
    local_d0 = *(undefined8 *)(in_RCX + 0xb0);
    local_158 = local_80;
    uStack_150 = uStack_78;
    local_148 = local_70;
    uStack_140 = uStack_68;
    local_138 = local_60;
    uStack_130 = uStack_58;
    local_128 = uStack_50;
    uStack_120 = local_48;
    local_118 = uStack_40;
    local_110 = local_c8;
    uStack_108 = uStack_c0;
    local_100 = local_b8;
    uStack_f8 = uStack_b0;
    local_f0 = local_a8;
    uStack_e8 = uStack_a0;
    local_e0 = uStack_98;
    local_90 = uStack_d8;
    uStack_88 = local_d0;
    local_38 = (void *)(in_RCX + 0xb8);
    local_30 = in_RCX;
    core::ptr::
    drop_in_place<indexmap::map::core::IndexMapCore<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>_>_>
              ();
    lVar1 = *(longlong *)(local_30 + 0x120);
    if ((lVar1 != 0) && (lVar1 + (lVar1 * 8 + 0x17U & 0xfffffffffffffff0) != -0x11)) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    _Dst = local_38;
    if (*(longlong *)(local_30 + 0x100) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    memcpy(_Dst,&local_158,0x90);
    *(undefined1 *)(local_30 + 0x178) = 1;
  }
  return;
}

