
void __cdecl country_core::systems::collision::world::impl$18::eq(void)

{
  undefined4 uVar1;
  char *in_RCX;
  char *in_RDX;
  undefined1 auVar2 [16];
  undefined1 auVar3 [16];
  int iVar4;
  int iVar5;
  
  if ((*(longlong *)(*(longlong *)(in_RCX + 0x18) + 0x80) !=
       *(longlong *)(*(longlong *)(in_RDX + 0x18) + 0x80)) || (*in_RCX != *in_RDX)) {
    return;
  }
  uVar1 = (undefined4)
          CONCAT71((int7)((ulonglong)*(longlong *)(*(longlong *)(in_RCX + 0x18) + 0x80) >> 8),1);
  switch(*in_RCX) {
  case '\0':
  case '\x01':
  case '\x06':
  case '\a':
  case '\b':
  case '\t':
  case '\f':
    break;
  case '\x02':
    if (*(longlong *)(in_RCX + 8) != *(longlong *)(in_RDX + 8)) {
      return;
    }
    if (in_RCX[1] != in_RDX[1]) {
      return;
    }
    break;
  case '\x03':
  case '\x04':
  case '\x05':
    if (*(int *)(in_RCX + 4) != *(int *)(in_RDX + 4)) {
      return;
    }
    if (*(longlong *)(in_RCX + 8) != *(longlong *)(in_RDX + 8)) {
      return;
    }
    break;
  case '\r':
  case '\x0e':
    iVar4 = -(uint)((int)*(undefined8 *)(in_RDX + 4) == (int)*(undefined8 *)(in_RCX + 4));
    iVar5 = -(uint)((int)((ulonglong)*(undefined8 *)(in_RDX + 4) >> 0x20) ==
                   (int)((ulonglong)*(undefined8 *)(in_RCX + 4) >> 0x20));
    auVar3._4_4_ = iVar4;
    auVar3._0_4_ = iVar4;
    auVar3._8_4_ = iVar5;
    auVar3._12_4_ = iVar5;
    movmskpd(uVar1,auVar3);
    break;
  case '\x0f':
  case '\x10':
    iVar4 = -(uint)((int)*(undefined8 *)(in_RDX + 8) == (int)*(undefined8 *)(in_RCX + 8));
    iVar5 = -(uint)((int)((ulonglong)*(undefined8 *)(in_RDX + 8) >> 0x20) ==
                   (int)((ulonglong)*(undefined8 *)(in_RCX + 8) >> 0x20));
    auVar2._4_4_ = iVar4;
    auVar2._0_4_ = iVar4;
    auVar2._8_4_ = iVar5;
    auVar2._12_4_ = iVar5;
    iVar4 = movmskpd(uVar1,auVar2);
    if (iVar4 != 3) {
      return;
    }
  }
  return;
}

