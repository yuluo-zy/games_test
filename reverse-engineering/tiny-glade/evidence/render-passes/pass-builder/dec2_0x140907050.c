// address: 0x140907050
// pdb name: draw_multi_instanced
// signature: void __cdecl draw_multi_instanced(void)

void __cdecl
country_core::render::pass_builder::draw_builder::DrawBuilder::draw_multi_instanced(void)

{
  longlong lVar1;
  void *in_RCX;
  undefined8 *in_RDX;
  longlong local_1e0;
  undefined1 local_1d8 [176];
  undefined4 local_128;
  undefined4 uStack_124;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined4 uStack_118;
  undefined8 uStack_114;
  longlong *local_48;
  void *local_30;
  undefined1 local_21;
  undefined8 local_20;
  
  local_20 = 0xfffffffffffffffe;
  uStack_114 = in_RDX[2];
  uStack_124 = (undefined4)*in_RDX;
  uStack_120 = (undefined4)((ulonglong)*in_RDX >> 0x20);
  uStack_11c = (undefined4)in_RDX[1];
  uStack_118 = (undefined4)((ulonglong)in_RDX[1] >> 0x20);
  local_30 = in_RCX;
  if (*(int *)((longlong)in_RCX + 0x40) == 4) {
    if (*(longlong *)((longlong)in_RCX + 0x48) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  else if (*(int *)((longlong)in_RCX + 0x40) == 1) {
    rhapsody::raw::managed::handle::impl$6::drop();
  }
  *(undefined4 *)((longlong)local_30 + 0x40) = 4;
  *(ulonglong *)((longlong)local_30 + 0x44) = CONCAT44(uStack_124,local_128);
  *(ulonglong *)((longlong)local_30 + 0x4c) = CONCAT44(uStack_11c,uStack_120);
  *(ulonglong *)((longlong)local_30 + 0x50) = CONCAT44(uStack_118,uStack_11c);
  *(undefined8 *)((longlong)local_30 + 0x58) = uStack_114;
  memcpy(&local_128,local_30,0xf8);
  local_1e0 = CONCAT44(uStack_124,local_128);
  local_128 = 2;
  uStack_124 = 0;
  if (local_1e0 != 2) {
    memcpy(local_1d8,(void *)((longlong)local_30 + 8),0xb0);
    lVar1 = local_48[2];
    if (lVar1 == *local_48) {
      alloc::raw_vec::RawVec::
      grow_one<system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::ArchSegment,alloc::alloc::Global>
                ();
    }
    memmove((void *)(lVar1 * 0xb8 + local_48[1]),&local_1e0,0xb8);
    local_48[2] = lVar1 + 1;
    local_21 = 0;
    core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>();
    return;
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}


