// despawn_fish 0x140f90c10 sig=void __cdecl despawn_fish(void)

void __cdecl system_critters::fish::despawn_fish(void)

{
  longlong *plVar1;
  undefined8 *in_RCX;
  longlong *in_RDX;
  longlong local_28;
  longlong lStack_20;
  longlong local_18;
  
  *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
  plVar1 = (longlong *)*in_RCX;
  if (*plVar1 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (plVar1[3] != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  *plVar1 = 0;
  plVar1[1] = 4;
  plVar1[2] = 0;
  plVar1[3] = 0;
  plVar1[4] = 8;
  plVar1[5] = 0;
  plVar1[6] = 0;
  FishSpawnLocations::new();
  *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
  plVar1 = (longlong *)*in_RDX;
  if (*plVar1 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  plVar1[2] = local_18;
  *plVar1 = local_28;
  plVar1[1] = lStack_20;
  return;
}

