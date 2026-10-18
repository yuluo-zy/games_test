
void __cdecl system_decorator::ui_systems::ui_inactive::ui_decorator_inactive(void)

{
  ulonglong uVar1;
  longlong *in_RCX;
  ulonglong uVar2;
  longlong in_RDX;
  
  uVar1 = *(ulonglong *)*in_RCX;
  uVar2 = uVar1 - 2;
  if (uVar2 == 4 &&
      (((ulonglong *)*in_RCX)[1] - 1) + (ulonglong)(1 < uVar1) < (ulonglong)(uVar2 < 5)) {
    **(undefined4 **)(in_RDX + 0x10) = *(undefined4 *)(in_RDX + 0x1c);
    backup::DecoratorBackup::clear();
    return;
  }
  return;
}

