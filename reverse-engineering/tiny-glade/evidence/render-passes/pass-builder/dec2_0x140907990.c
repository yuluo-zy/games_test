// address: 0x140907990
// pdb name: multi_draw_indexed_indirect
// signature: void __cdecl multi_draw_indexed_indirect(void)

void __cdecl
country_core::render::pass_builder::draw_builder::DrawBuilder::multi_draw_indexed_indirect(void)

{
  longlong lVar1;
  void *in_RCX;
  uint in_EDX;
  undefined8 in_R8;
  undefined8 in_R9;
  longlong local_218;
  undefined1 local_210 [176];
  longlong local_160 [2];
  undefined4 local_150;
  longlong *local_80;
  undefined8 local_68;
  undefined8 local_60;
  longlong local_58;
  undefined4 local_50;
  undefined4 local_4c;
  void *local_48;
  undefined1 local_39;
  undefined8 local_38;
  
  local_38 = 0xfffffffffffffffe;
  local_39 = 1;
  local_48 = in_RCX;
  asset_libraries::ssbo_library::AssetSsboLibrary::
  get_rg_buffer<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>();
  local_39 = 1;
  bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
  local_39 = 1;
  rhapsody::rg::pass_builder::PassBuilder::
  add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>();
  local_50 = local_150;
  local_58 = (ulonglong)in_EDX * 0x14;
  local_4c = *(undefined4 *)(&DAT_142aaaa78 + local_58);
  local_68 = in_R9;
  local_60 = in_R8;
  if (*(int *)((longlong)local_48 + 0x40) == 4) {
    if (*(longlong *)((longlong)local_48 + 0x48) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  else if (*(int *)((longlong)local_48 + 0x40) == 1) {
    rhapsody::raw::managed::handle::impl$6::drop();
  }
  *(undefined4 *)((longlong)local_48 + 0x40) = 7;
  *(undefined4 *)((longlong)local_48 + 0x44) = local_50;
  *(longlong *)((longlong)local_48 + 0x48) = local_58;
  *(undefined8 *)((longlong)local_48 + 0x50) = local_60;
  *(undefined8 *)((longlong)local_48 + 0x58) = local_68;
  *(undefined4 *)((longlong)local_48 + 0x60) = 1;
  *(undefined4 *)((longlong)local_48 + 100) = local_4c;
  memcpy(local_160,local_48,0xf8);
  local_218 = local_160[0];
  local_160[0] = 2;
  if (local_218 != 2) {
    memcpy(local_210,(void *)((longlong)local_48 + 8),0xb0);
    lVar1 = local_80[2];
    if (lVar1 == *local_80) {
      alloc::raw_vec::RawVec::
      grow_one<system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::ArchSegment,alloc::alloc::Global>
                ();
    }
    memmove((void *)(lVar1 * 0xb8 + local_80[1]),&local_218,0xb8);
    local_80[2] = lVar1 + 1;
    local_39 = 0;
    core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>();
    return;
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}


