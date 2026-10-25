// address: 0x140907f90
// pdb name: draw_indexed_indirect_count
// signature: void __cdecl draw_indexed_indirect_count(void)

void __cdecl
country_core::render::pass_builder::draw_builder::DrawBuilder::draw_indexed_indirect_count(void)

{
  undefined4 uVar1;
  undefined4 uVar2;
  longlong lVar3;
  void *in_RCX;
  longlong in_RDX;
  undefined8 in_R8;
  undefined8 in_R9;
  longlong in_stack_00000028;
  undefined8 in_stack_00000030;
  undefined8 in_stack_00000038;
  longlong local_210;
  undefined1 local_208 [176];
  longlong local_158 [28];
  longlong *local_78;
  undefined8 local_60;
  undefined8 local_58;
  undefined4 local_50;
  undefined4 local_4c;
  void *local_48;
  undefined1 local_39;
  undefined8 local_38;
  
  local_38 = 0xfffffffffffffffe;
  local_39 = 1;
  local_48 = in_RCX;
  bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
  uVar1 = *(undefined4 *)(in_RDX + 0x10);
  local_39 = 1;
  rhapsody::rg::pass_builder::PassBuilder::
  add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>();
  local_39 = 1;
  bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
  uVar2 = *(undefined4 *)(in_stack_00000028 + 0x10);
  local_39 = 1;
  rhapsody::rg::pass_builder::PassBuilder::
  add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>();
  local_60 = in_R9;
  local_58 = in_R8;
  local_50 = uVar1;
  local_4c = uVar2;
  if (*(int *)((longlong)local_48 + 0x40) == 4) {
    if (*(longlong *)((longlong)local_48 + 0x48) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  else if (*(int *)((longlong)local_48 + 0x40) == 1) {
    rhapsody::raw::managed::handle::impl$6::drop();
  }
  *(undefined4 *)((longlong)local_48 + 0x40) = 8;
  *(undefined4 *)((longlong)local_48 + 0x44) = local_50;
  *(undefined4 *)((longlong)local_48 + 0x48) = local_4c;
  *(undefined8 *)((longlong)local_48 + 0x50) = local_58;
  *(undefined8 *)((longlong)local_48 + 0x58) = local_60;
  *(undefined8 *)((longlong)local_48 + 0x60) = in_stack_00000030;
  *(undefined8 *)((longlong)local_48 + 0x68) = in_stack_00000038;
  memcpy(local_158,local_48,0xf8);
  local_210 = local_158[0];
  local_158[0] = 2;
  if (local_210 != 2) {
    memcpy(local_208,(void *)((longlong)local_48 + 8),0xb0);
    lVar3 = local_78[2];
    if (lVar3 == *local_78) {
      alloc::raw_vec::RawVec::
      grow_one<system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::ArchSegment,alloc::alloc::Global>
                ();
    }
    memmove((void *)(lVar3 * 0xb8 + local_78[1]),&local_210,0xb8);
    local_78[2] = lVar3 + 1;
    local_39 = 0;
    core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>();
    return;
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}


