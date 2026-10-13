// address: 0x140906f40
// pdb name: draw
// signature: void __cdecl draw(void)

void __cdecl country_core::render::pass_builder::draw_builder::DrawBuilder::draw(void)

{
  longlong *plVar1;
  longlong lVar2;
  longlong *in_RCX;
  longlong local_e0;
  undefined1 local_d8 [184];
  undefined8 local_20;
  
  local_20 = 0xfffffffffffffffe;
  plVar1 = (longlong *)in_RCX[0x1c];
  local_e0 = *in_RCX;
  *in_RCX = 2;
  if (local_e0 != 2) {
    memcpy(local_d8,in_RCX + 1,0xb0);
    lVar2 = plVar1[2];
    if (lVar2 == *plVar1) {
      alloc::raw_vec::RawVec::
      grow_one<system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::ArchSegment,alloc::alloc::Global>
                ();
    }
    memmove((void *)(lVar2 * 0xb8 + plVar1[1]),&local_e0,0xb8);
    plVar1[2] = lVar2 + 1;
    core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>();
    return;
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}


