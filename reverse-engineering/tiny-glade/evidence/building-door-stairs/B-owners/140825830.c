
void __cdecl system_decorator::blueprint::DecoratorBlueprint::add_door_stairs(void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 *extraout_RAX;
  longlong in_RCX;
  undefined8 *in_RDX;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX != (undefined8 *)0x0) {
    uVar1 = *in_RDX;
    uVar2 = in_RDX[1];
    uVar3 = in_RDX[3];
    extraout_RAX[2] = in_RDX[2];
    extraout_RAX[3] = uVar3;
    *extraout_RAX = uVar1;
    extraout_RAX[1] = uVar2;
    if (*(longlong *)(in_RCX + 0x120) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    *(undefined8 **)(in_RCX + 0x120) = extraout_RAX;
    return;
  }
                    /* WARNING: Subroutine does not return */
  alloc::alloc::handle_alloc_error();
}

