
void __cdecl system_decorator::ui_systems::ui_stairs_misc::stairs_reset_for_new_frame(void)

{
  uint uVar1;
  undefined8 *in_RCX;
  longlong *in_RDX;
  longlong lVar2;
  
  *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
  uVar1 = *(uint *)*in_RCX;
  lVar2 = 0;
  if (uVar1 - 6 < 0xf) {
    lVar2 = (ulonglong)uVar1 - 5;
  }
  if (lVar2 == 0) {
    if (uVar1 != 4) goto country_core_resources_stairs_stairs_state_StairsState_unset_endpoint;
  }
  else if ((lVar2 != 6) || ((char)((uint *)*in_RCX)[1] != '?')) {
country_core_resources_stairs_stairs_state_StairsState_unset_endpoint:
    *(undefined8 *)(*in_RDX + 0x10) = 3;
    return;
  }
  return;
}

