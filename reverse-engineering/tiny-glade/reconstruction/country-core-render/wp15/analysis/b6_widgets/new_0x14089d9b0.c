// addr=0x14089d9b0 name=new
// signature=void __cdecl new(void)


void __cdecl country_core::systems::ui::signifiers::SignifierEntityBuilder::new(void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 *extraout_RAX;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX != (undefined8 *)0x0) {
    *extraout_RAX = 0x635f786574726576;
    extraout_RAX[1] = 0x6168735f726f6c6f;
    *(undefined4 *)((longlong)extraout_RAX + 0xf) = 0x72656461;
    uVar1 = in_RDX[1];
    uVar2 = in_RDX[2];
    uVar3 = in_RDX[3];
    uVar4 = in_RDX[4];
    uVar5 = in_RDX[5];
    in_RCX[2] = *in_RDX;
    in_RCX[3] = uVar1;
    in_RCX[4] = uVar2;
    in_RCX[5] = uVar3;
    in_RCX[6] = uVar4;
    in_RCX[7] = uVar5;
    *(undefined4 *)(in_RCX + 0xf) = 0x3e4ccccd;
    *(undefined2 *)(in_RCX + 8) = 0x100;
    *(undefined8 *)((longlong)in_RCX + 0x44) = 0x13f800000;
    in_RCX[0xc] = 0x13;
    in_RCX[0xd] = extraout_RAX;
    in_RCX[0xe] = 0x13;
    *(undefined8 *)((longlong)in_RCX + 0x81) = 0;
    *(undefined8 *)((longlong)in_RCX + 0x7c) = 0;
    *in_RCX = 0;
    in_RCX[1] = 0x3f80000000000000;
    *(undefined4 *)(in_RCX + 10) = 0;
    *(undefined2 *)((longlong)in_RCX + 0x89) = 0;
    *(undefined4 *)(in_RCX + 0xb) = 0;
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}


