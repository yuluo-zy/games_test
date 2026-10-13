// address: 0x140907cc0
// pdb name: draw_indexed_indirect_raw
// signature: void __cdecl draw_indexed_indirect_raw(void)

void __cdecl
country_core::render::pass_builder::draw_builder::DrawBuilder::draw_indexed_indirect_raw(void)

{
  undefined4 uVar1;
  longlong lVar2;
  void *in_RCX;
  longlong in_RDX;
  undefined8 in_R8;
  undefined8 in_R9;
  undefined8 in_stack_00000028;
  longlong local_208;
  undefined1 local_200 [176];
  longlong local_150 [28];
  longlong *local_70;
  undefined8 local_58;
  undefined8 local_50;
  undefined4 local_44;
  void *local_40;
  undefined1 local_31;
  undefined8 local_30;
  
  local_30 = 0xfffffffffffffffe;
  local_31 = 1;
  local_40 = in_RCX;
  bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
  uVar1 = *(undefined4 *)(in_RDX + 0x10);
  local_31 = 1;
  rhapsody::rg::pass_builder::PassBuilder::
  add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>();
  local_58 = in_R9;
  local_50 = in_R8;
  local_44 = uVar1;
  if (*(int *)((longlong)local_40 + 0x40) == 4) {
    if (*(longlong *)((longlong)local_40 + 0x48) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  else if (*(int *)((longlong)local_40 + 0x40) == 1) {
    rhapsody::raw::managed::handle::impl$6::drop();
  }
  *(undefined4 *)((longlong)local_40 + 0x40) = 7;
  *(undefined4 *)((longlong)local_40 + 0x44) = local_44;
  *(undefined8 *)((longlong)local_40 + 0x48) = local_50;
  *(undefined8 *)((longlong)local_40 + 0x50) = in_stack_00000028;
  *(undefined8 *)((longlong)local_40 + 0x58) = local_58;
  *(undefined4 *)((longlong)local_40 + 0x60) = 0;
  memcpy(local_150,local_40,0xf8);
  local_208 = local_150[0];
  local_150[0] = 2;
  if (local_208 != 2) {
    memcpy(local_200,(void *)((longlong)local_40 + 8),0xb0);
    lVar2 = local_70[2];
    if (lVar2 == *local_70) {
      alloc::raw_vec::RawVec::
      grow_one<system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::ArchSegment,alloc::alloc::Global>
                ();
    }
    memmove((void *)(lVar2 * 0xb8 + local_70[1]),&local_208,0xb8);
    local_70[2] = lVar2 + 1;
    local_31 = 0;
    core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>();
    return;
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}


