
void __cdecl
country_core::systems::mode_manager::
check_if_can_switch<country_core::systems::mode_manager::mode_manager::closure_env_3>(void)

{
  longlong lVar1;
  undefined8 uVar2;
  longlong lVar3;
  longlong *plVar4;
  ulonglong *puVar5;
  longlong *plVar6;
  ulonglong *puVar7;
  ulonglong uVar8;
  longlong lVar9;
  ulonglong uVar10;
  char in_CL;
  longlong *in_RDX;
  longlong *in_R8;
  int local_28;
  
  if (in_CL == '\0') {
    plVar4 = (longlong *)*in_R8;
    puVar5 = (ulonglong *)in_R8[1];
    plVar6 = (longlong *)in_R8[2];
    *(undefined4 *)plVar4[2] = *(undefined4 *)((longlong)plVar4 + 0x1c);
    puVar7 = (ulonglong *)*plVar4;
    uVar8 = *puVar5;
    uVar10 = puVar5[1];
    *(undefined4 *)plVar6[2] = *(undefined4 *)((longlong)plVar6 + 0x1c);
    lVar1 = *plVar6;
    *puVar7 = uVar8;
    puVar7[1] = uVar10;
    local_28 = (int)uVar8;
    lVar9 = 0;
    if (local_28 - 6U < 0xf) {
      lVar9 = (uVar8 & 0xffffffff) - 5;
    }
    if ((lVar9 == 0xf) || (lVar9 == 0)) {
      if (7 < *(ulonglong *)(lVar1 + 0x10)) {
        lVar9 = *(longlong *)(lVar1 + 8);
        uVar8 = *(ulonglong *)(lVar9 + 0xf0);
        if (uVar8 != 0) {
          if (*(char *)(lVar1 + 0x18) == '\a') {
            uVar10 = *(longlong *)(lVar9 + 0xf8) + 1;
            if ((uVar10 | uVar8) >> 0x20 == 0) {
              uVar10 = (uVar10 & 0xffffffff) % (uVar8 & 0xffffffff);
            }
            else {
              uVar10 = uVar10 % uVar8;
            }
            *(ulonglong *)(lVar9 + 0xf8) = uVar10;
          }
          else if (uVar8 <= *(ulonglong *)(lVar9 + 0xf8)) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          *(undefined1 *)(lVar1 + 0x18) = 7;
        }
      }
    }
    else {
      resources::toolbar::Toolbar::switch_mode();
    }
  }
  else {
    *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
    lVar1 = *in_RDX;
    lVar9 = *(longlong *)(lVar1 + 0x30);
    uVar2 = *(undefined8 *)(lVar1 + 0x40);
    if (lVar9 == *(longlong *)(lVar1 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    lVar3 = *(longlong *)(lVar1 + 0x28);
    *(undefined4 *)(lVar3 + lVar9 * 0x20) = 4;
    *(undefined8 *)(lVar3 + 0x18 + lVar9 * 0x20) = uVar2;
    *(longlong *)(lVar1 + 0x30) = lVar9 + 1;
    *(longlong *)(lVar1 + 0x40) = *(longlong *)(lVar1 + 0x40) + 1;
  }
  return;
}

