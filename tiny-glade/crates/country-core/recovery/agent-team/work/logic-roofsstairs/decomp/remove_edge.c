
void __cdecl country_core::resources::stairs::visual::StairsVisualState::remove_edge(void)

{
  longlong lVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  longlong in_RCX;
  longlong lVar4;
  longlong in_R9;
  longlong local_280;
  undefined8 local_278;
  undefined8 local_270;
  ulonglong local_268;
  longlong local_178;
  longlong local_170;
  longlong local_168;
  longlong local_160;
  longlong local_148;
  longlong local_e8;
  longlong local_c8;
  longlong local_a0;
  longlong local_90;
  longlong local_68;
  longlong local_60;
  longlong local_58;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::resources::stairs::stairs_state::StairGraphEdge>_>
            ();
  hashbrown::raw::RawTable::
  remove_entry<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::visual::StairsEdgeVisualState>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::visual::StairsEdgeVisualState>_>
            ();
  if (!SBORROW8(0,local_280)) {
    memcpy(&local_178,&local_280,0x108);
    if (!SBORROW8(0,local_178)) {
      local_58 = local_178;
      local_60 = local_170;
      local_68 = local_168;
      if (local_a0 != 0) {
        core::ptr::
        drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo>_>
                  ();
      }
      if (local_90 != 0) {
        core::ptr::
        drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo>_>
                  ();
      }
      if ((local_c8 != 0) && (local_c8 + (local_c8 * 2 + 0x11U & 0xfffffffffffffff0) != -0x11)) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (local_160 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (local_148 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      core::ptr::drop_in_place<country_core::resources::stairs::visual::MagicSupports>();
      if (local_e8 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      *(undefined1 *)(in_RCX + 0x30) = 1;
      if (local_68 != 0) {
        lVar1 = *(longlong *)(in_R9 + 8);
        uVar2 = *(undefined8 *)(in_R9 + 0x18);
        uVar3 = *(undefined8 *)(in_R9 + 0x10);
        local_68 = local_68 << 3;
        lVar4 = 0;
        do {
          if (*(ulonglong *)(local_60 + lVar4) >> 0x20 == 0) break;
          local_280 = lVar1;
          local_278 = uVar3;
          local_270 = uVar2;
          local_268 = *(ulonglong *)(local_60 + lVar4);
          bevy_ecs::system::commands::EntityCommands::despawn();
          lVar4 = lVar4 + 8;
        } while (local_68 != lVar4);
      }
      if (local_58 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
  }
  return;
}

