
void __cdecl
country_core::systems::mode_manager::
check_if_can_switch<country_core::systems::mode_manager::mode_manager::closure_env_13>(void)

{
  int iVar1;
  int iVar2;
  longlong lVar3;
  undefined8 uVar4;
  longlong lVar5;
  ulonglong uVar6;
  int *piVar7;
  ulonglong uVar8;
  char in_CL;
  longlong *in_RDX;
  longlong *in_R8;
  longlong lVar9;
  undefined8 *in_R9;
  
  if (in_CL == '\0') {
    *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
    lVar3 = *in_R8;
    if (8 < *(ulonglong *)(lVar3 + 0x10)) {
      lVar9 = *(longlong *)(lVar3 + 8);
      uVar6 = *(ulonglong *)(lVar9 + 0x110);
      if (uVar6 != 0) {
        if (*(char *)(lVar3 + 0x18) == '\b') {
          uVar8 = *(longlong *)(lVar9 + 0x118) + 1;
          if ((uVar8 | uVar6) >> 0x20 == 0) {
            uVar8 = (uVar8 & 0xffffffff) % (uVar6 & 0xffffffff);
          }
          else {
            uVar8 = uVar8 % uVar6;
          }
          *(ulonglong *)(lVar9 + 0x118) = uVar8;
        }
        else {
          uVar8 = *(ulonglong *)(lVar9 + 0x118);
          if (uVar6 <= uVar8) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
        }
        lVar9 = *(longlong *)(lVar9 + 0x108);
        iVar1 = *(int *)(lVar9 + uVar8 * 0x10);
        *(undefined1 *)(lVar3 + 0x18) = 8;
        if (iVar1 != 0x15) {
          lVar9 = lVar9 + uVar8 * 0x10;
          iVar2 = *(int *)(lVar9 + 0xc);
          uVar4 = *(undefined8 *)(lVar9 + 4);
          *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
          piVar7 = (int *)*in_R9;
          *piVar7 = iVar1;
          *(undefined8 *)(piVar7 + 1) = uVar4;
          piVar7[3] = iVar2;
        }
      }
    }
  }
  else {
    *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
    lVar3 = *in_RDX;
    lVar9 = *(longlong *)(lVar3 + 0x30);
    uVar4 = *(undefined8 *)(lVar3 + 0x40);
    if (lVar9 == *(longlong *)(lVar3 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    lVar5 = *(longlong *)(lVar3 + 0x28);
    *(undefined4 *)(lVar5 + lVar9 * 0x20) = 4;
    *(undefined8 *)(lVar5 + 0x18 + lVar9 * 0x20) = uVar4;
    *(longlong *)(lVar3 + 0x30) = lVar9 + 1;
    *(longlong *)(lVar3 + 0x40) = *(longlong *)(lVar3 + 0x40) + 1;
  }
  return;
}

