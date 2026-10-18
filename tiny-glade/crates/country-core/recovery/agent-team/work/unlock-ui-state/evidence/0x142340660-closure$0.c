
void __cdecl toolbar::ui_interaction::ui_nav_update::closure_0(void)

{
  longlong *plVar1;
  longlong lVar2;
  longlong in_RCX;
  undefined4 local_48;
  int iStack_44;
  undefined4 local_3c;
  undefined8 local_38;
  undefined8 uStack_30;
  undefined8 local_28;
  undefined8 uStack_20;
  undefined4 *local_18;
  undefined4 *local_10;
  
  iStack_44 = 0;
  local_3c = 0x7f7fffff;
  local_38 = *(undefined8 *)(in_RCX + 8);
  uStack_30 = *(undefined8 *)(in_RCX + 0x10);
  local_28 = *(undefined8 *)(in_RCX + 0x18);
  uStack_20 = *(undefined8 *)(in_RCX + 0x20);
  local_18 = &local_3c;
  local_10 = &local_48;
  iter_subtree<tuple_<>,toolbar::ui_interaction::ui_nav_update::closure_0::closure_env_0>();
  if (iStack_44 != 0) {
    plVar1 = *(longlong **)(in_RCX + 0x28);
    *(undefined4 *)plVar1[2] = *(undefined4 *)((longlong)plVar1 + 0x1c);
    lVar2 = *plVar1;
    *(undefined8 *)(lVar2 + 0x10) = 1;
    *(ulonglong *)(lVar2 + 0x18) = CONCAT44(iStack_44,local_48);
  }
  return;
}

