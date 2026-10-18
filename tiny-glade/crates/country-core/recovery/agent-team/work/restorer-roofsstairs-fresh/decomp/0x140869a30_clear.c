
void __cdecl country_core::resources::stairs::structural_graph::StairsStructuralGraph::clear(void)

{
  ulonglong uVar1;
  ulonglong uVar2;
  longlong in_RCX;
  longlong in_RDX;
  longlong lVar3;
  longlong lVar4;
  
  if (*(longlong *)(in_RCX + 0x30) != 0) {
    uVar1 = *(ulonglong *)(in_RCX + 0x20);
    if (uVar1 == 0) {
      uVar2 = 0;
    }
    else {
      in_RDX = CONCAT71((int7)((ulonglong)in_RDX >> 8),0xff);
      memset(*(void **)(in_RCX + 0x18),(int)in_RDX,uVar1 + 0x11);
      uVar2 = (uVar1 + 1 & 0xfffffffffffffff8) - (uVar1 + 1 >> 3);
      if (uVar1 < 8) {
        uVar2 = uVar1;
      }
    }
    *(undefined8 *)(in_RCX + 0x30) = 0;
    *(ulonglong *)(in_RCX + 0x28) = uVar2;
  }
  lVar3 = *(longlong *)(in_RCX + 0x10);
  *(undefined8 *)(in_RCX + 0x10) = 0;
  if (lVar3 != 0) {
    lVar4 = *(longlong *)(in_RCX + 8) + 8;
    do {
      in_RDX = 0;
      if (*(longlong *)(lVar4 + -8) != 0) {
        in_RDX = *(longlong *)(lVar4 + -8) << 4;
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      lVar4 = lVar4 + 0x30;
      lVar3 = lVar3 + -1;
    } while (lVar3 != 0);
  }
  if (*(longlong *)(in_RCX + 0x78) != 0) {
    uVar1 = *(ulonglong *)(in_RCX + 0x68);
    if (uVar1 == 0) {
      uVar2 = 0;
    }
    else {
      in_RDX = CONCAT71((int7)((ulonglong)in_RDX >> 8),0xff);
      memset(*(void **)(in_RCX + 0x60),(int)in_RDX,uVar1 + 0x11);
      uVar2 = (uVar1 + 1 & 0xfffffffffffffff8) - (uVar1 + 1 >> 3);
      if (uVar1 < 8) {
        uVar2 = uVar1;
      }
    }
    *(undefined8 *)(in_RCX + 0x78) = 0;
    *(ulonglong *)(in_RCX + 0x70) = uVar2;
  }
  *(undefined8 *)(in_RCX + 0x58) = 0;
  if (*(longlong *)(in_RCX + 0xa8) != 0) {
    uVar1 = *(ulonglong *)(in_RCX + 0x98);
    if (uVar1 != 0) {
      in_RDX = CONCAT71((int7)((ulonglong)in_RDX >> 8),0xff);
      memset(*(void **)(in_RCX + 0x90),(int)in_RDX,uVar1 + 0x11);
    }
    *(undefined8 *)(in_RCX + 0xa8) = 0;
    uVar2 = (uVar1 + 1 & 0xfffffffffffffff8) - (uVar1 + 1 >> 3);
    if (uVar1 < 8) {
      uVar2 = uVar1;
    }
    *(ulonglong *)(in_RCX + 0xa0) = uVar2;
  }
  if (*(longlong *)(in_RCX + 0xd8) != 0) {
    uVar1 = *(ulonglong *)(in_RCX + 200);
    if (uVar1 != 0) {
      in_RDX = CONCAT71((int7)((ulonglong)in_RDX >> 8),0xff);
      memset(*(void **)(in_RCX + 0xc0),(int)in_RDX,uVar1 + 0x11);
    }
    *(undefined8 *)(in_RCX + 0xd8) = 0;
    uVar2 = (uVar1 + 1 & 0xfffffffffffffff8) - (uVar1 + 1 >> 3);
    if (uVar1 < 8) {
      uVar2 = uVar1;
    }
    *(ulonglong *)(in_RCX + 0xd0) = uVar2;
  }
  if (*(longlong *)(in_RCX + 0x108) != 0) {
    uVar1 = *(ulonglong *)(in_RCX + 0xf8);
    if (uVar1 != 0) {
      in_RDX = CONCAT71((int7)((ulonglong)in_RDX >> 8),0xff);
      memset(*(void **)(in_RCX + 0xf0),(int)in_RDX,uVar1 + 0x11);
    }
    *(undefined8 *)(in_RCX + 0x108) = 0;
    uVar2 = (uVar1 + 1 & 0xfffffffffffffff8) - (uVar1 + 1 >> 3);
    if (uVar1 < 8) {
      uVar2 = uVar1;
    }
    *(ulonglong *)(in_RCX + 0x100) = uVar2;
  }
  if (*(longlong *)(in_RCX + 0x138) != 0) {
    uVar1 = *(ulonglong *)(in_RCX + 0x128);
    if (uVar1 != 0) {
      in_RDX = CONCAT71((int7)((ulonglong)in_RDX >> 8),0xff);
      memset(*(void **)(in_RCX + 0x120),(int)in_RDX,uVar1 + 0x11);
    }
    *(undefined8 *)(in_RCX + 0x138) = 0;
    uVar2 = (uVar1 + 1 & 0xfffffffffffffff8) - (uVar1 + 1 >> 3);
    if (uVar1 < 8) {
      uVar2 = uVar1;
    }
    *(ulonglong *)(in_RCX + 0x130) = uVar2;
  }
  if (*(longlong *)(in_RCX + 0x168) != 0) {
    uVar1 = *(ulonglong *)(in_RCX + 0x158);
    if (uVar1 != 0) {
      in_RDX = CONCAT71((int7)((ulonglong)in_RDX >> 8),0xff);
      memset(*(void **)(in_RCX + 0x150),(int)in_RDX,uVar1 + 0x11);
    }
    *(undefined8 *)(in_RCX + 0x168) = 0;
    uVar2 = (uVar1 + 1 & 0xfffffffffffffff8) - (uVar1 + 1 >> 3);
    if (uVar1 < 8) {
      uVar2 = uVar1;
    }
    *(ulonglong *)(in_RCX + 0x160) = uVar2;
  }
  if (*(longlong *)(in_RCX + 0x198) != 0) {
    uVar1 = *(ulonglong *)(in_RCX + 0x188);
    if (uVar1 != 0) {
      memset(*(void **)(in_RCX + 0x180),(int)CONCAT71((int7)((ulonglong)in_RDX >> 8),0xff),
             uVar1 + 0x11);
    }
    *(undefined8 *)(in_RCX + 0x198) = 0;
    uVar2 = (uVar1 + 1 & 0xfffffffffffffff8) - (uVar1 + 1 >> 3);
    if (uVar1 < 8) {
      uVar2 = uVar1;
    }
    *(ulonglong *)(in_RCX + 400) = uVar2;
  }
  return;
}

