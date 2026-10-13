// address: 0x140907490
// pdb name: draw_instanced_w_gl_draw_flag
// signature: void __cdecl draw_instanced_w_gl_draw_flag(void)

void __cdecl
country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced_w_gl_draw_flag(void)

{
  longlong lVar1;
  void *in_RCX;
  undefined4 in_EDX;
  longlong local_1e8;
  undefined1 local_1e0 [176];
  longlong local_130 [28];
  longlong *local_50;
  undefined4 local_38;
  void *local_30;
  undefined1 local_21;
  undefined8 local_20;
  
  local_20 = 0xfffffffffffffffe;
  local_30 = in_RCX;
  local_38 = in_EDX;
  if (*(int *)((longlong)in_RCX + 0x40) == 4) {
    if (*(longlong *)((longlong)in_RCX + 0x48) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  else if (*(int *)((longlong)in_RCX + 0x40) == 1) {
    rhapsody::raw::managed::handle::impl$6::drop();
  }
  *(undefined4 *)((longlong)local_30 + 0x40) = 3;
  *(undefined4 *)((longlong)local_30 + 0x44) = local_38;
  memcpy(local_130,local_30,0xf8);
  local_1e8 = local_130[0];
  local_130[0] = 2;
  if (local_1e8 != 2) {
    memcpy(local_1e0,(void *)((longlong)local_30 + 8),0xb0);
    lVar1 = local_50[2];
    if (lVar1 == *local_50) {
      alloc::raw_vec::RawVec::
      grow_one<system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::ArchSegment,alloc::alloc::Global>
                ();
    }
    memmove((void *)(lVar1 * 0xb8 + local_50[1]),&local_1e8,0xb8);
    local_50[2] = lVar1 + 1;
    local_21 = 0;
    core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>();
    return;
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}


