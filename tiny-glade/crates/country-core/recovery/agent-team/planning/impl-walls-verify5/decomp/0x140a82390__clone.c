// clone  size=496
// void __cdecl clone(void)

void __cdecl country_core::resources::walls::inner_wall_state::impl$25::clone(void)

{
  size_t sVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined2 uVar5;
  ulonglong uVar6;
  void *pvVar7;
  ulonglong uVar8;
  int iVar9;
  undefined1 *extraout_RAX;
  undefined1 *_Dst;
  undefined1 *extraout_RAX_00;
  ulonglong *in_RCX;
  uint uVar10;
  longlong in_RDX;
  undefined1 *_Dst_00;
  ulonglong uVar11;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  ulonglong local_50;
  
  uVar6 = *(ulonglong *)(in_RDX + 0x10);
  sVar1 = uVar6 * 8;
  if ((uVar6 >> 0x3d != 0) || (0x7ffffffffffffffc < sVar1)) {
LAB_140a8255c:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  pvVar7 = *(void **)(in_RDX + 8);
  if (sVar1 == 0) {
    _Dst = &DAT_00000004;
    local_50 = 0;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    _Dst = extraout_RAX;
    local_50 = uVar6;
    if (extraout_RAX == (undefined1 *)0x0) goto LAB_140a8255c;
  }
  memcpy(_Dst,pvVar7,sVar1);
  uVar8 = *(ulonglong *)(in_RDX + 0x28);
  sVar1 = uVar8 * 4;
  if ((uVar8 >> 0x3e == 0) && (sVar1 < 0x7ffffffffffffffd)) {
    pvVar7 = *(void **)(in_RDX + 0x20);
    if (sVar1 == 0) {
      _Dst_00 = &DAT_00000004;
      uVar11 = 0;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      _Dst_00 = extraout_RAX_00;
      uVar11 = uVar8;
      if (extraout_RAX_00 == (undefined1 *)0x0) goto LAB_140a8256e;
    }
    memcpy(_Dst_00,pvVar7,sVar1);
    uVar2 = *(undefined4 *)(in_RDX + 0x30);
    uVar3 = *(undefined4 *)(in_RDX + 0x54);
    uVar10 = *(int *)(in_RDX + 0x38) - 2;
    if (1 < uVar10) {
      uVar10 = 2;
    }
    iVar9 = 2;
    if (uVar10 != 0) {
      if (uVar10 == 1) {
        iVar9 = 3;
      }
      else {
        local_68 = *(undefined8 *)(in_RDX + 0x4c);
        local_78 = *(undefined8 *)(in_RDX + 0x3c);
        uStack_70 = *(undefined8 *)(in_RDX + 0x44);
        iVar9 = *(int *)(in_RDX + 0x38);
      }
    }
    uVar5 = *(undefined2 *)(in_RDX + 0x68);
    uVar4 = *(undefined4 *)(in_RDX + 0x60);
    *in_RCX = local_50;
    in_RCX[1] = (ulonglong)_Dst;
    in_RCX[2] = uVar6;
    in_RCX[3] = uVar11;
    in_RCX[4] = (ulonglong)_Dst_00;
    in_RCX[5] = uVar8;
    *(undefined4 *)(in_RCX + 6) = uVar2;
    *(undefined4 *)((longlong)in_RCX + 0x54) = uVar3;
    *(int *)(in_RCX + 7) = iVar9;
    *(undefined8 *)((longlong)in_RCX + 0x3c) = local_78;
    *(undefined8 *)((longlong)in_RCX + 0x44) = uStack_70;
    *(undefined8 *)((longlong)in_RCX + 0x4c) = local_68;
    in_RCX[0xb] = *(ulonglong *)(in_RDX + 0x58);
    *(undefined2 *)(in_RCX + 0xd) = uVar5;
    *(undefined4 *)(in_RCX + 0xc) = uVar4;
    *(undefined4 *)((longlong)in_RCX + 100) = *(undefined4 *)(in_RDX + 100);
    return;
  }
LAB_140a8256e:
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}



// CALLEES: [{'name': '__rust_alloc', 'address': '140613c10'}, {'name': 'handle_error', 'address': '1428d8fe3'}, {'name': 'memcpy', 'address': '142923c00'}]