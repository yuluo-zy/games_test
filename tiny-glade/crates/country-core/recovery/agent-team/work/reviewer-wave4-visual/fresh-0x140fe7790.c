
void __cdecl taffy::node::Taffy::add_child(void)

{
  int *piVar1;
  ulonglong uVar2;
  longlong lVar3;
  longlong lVar4;
  ulonglong uVar5;
  undefined4 *in_RCX;
  ulonglong uVar6;
  longlong in_RDX;
  longlong lVar7;
  int in_R8D;
  uint in_R9D;
  longlong *plVar8;
  int in_stack_00000028;
  uint in_stack_00000030;
  
  uVar6 = (ulonglong)in_stack_00000030;
  uVar2 = *(ulonglong *)(in_RDX + 0x50);
  if (uVar6 < uVar2) {
    lVar3 = *(longlong *)(in_RDX + 0x48);
    if (*(int *)(lVar3 + 8 + uVar6 * 0xc) == in_stack_00000028) {
      piVar1 = (int *)(lVar3 + uVar6 * 0xc);
      *piVar1 = in_R8D;
      piVar1[1] = in_R9D;
      uVar6 = (ulonglong)in_R9D;
      if (uVar6 < *(ulonglong *)(in_RDX + 0x30)) {
        if (*(int *)(*(longlong *)(in_RDX + 0x28) + 0x18 + uVar6 * 0x20) == in_R8D) {
          plVar8 = (longlong *)(*(longlong *)(in_RDX + 0x28) + uVar6 * 0x20);
          lVar4 = plVar8[2];
          if (lVar4 == *plVar8) {
            alloc::raw_vec::RawVec::grow_one<tuple_<u32,u32>,alloc::alloc::Global>();
          }
          lVar7 = plVar8[1];
          *(int *)(lVar7 + lVar4 * 8) = in_stack_00000028;
          *(uint *)(lVar7 + 4 + lVar4 * 8) = in_stack_00000030;
          plVar8[2] = lVar4 + 1;
          uVar5 = *(ulonglong *)(in_RDX + 0x10);
          if (uVar6 < uVar5) {
            lVar4 = *(longlong *)(in_RDX + 8);
            do {
              if (*(int *)(lVar4 + 0x328 + uVar6 * 0x330) != in_R8D) break;
              lVar7 = uVar6 * 0x330 + lVar4;
              *(undefined4 *)(lVar7 + 0x158) = 2;
              *(undefined4 *)(lVar7 + 0x194) = 2;
              *(undefined4 *)(lVar7 + 0x1d0) = 2;
              *(undefined4 *)(lVar7 + 0x20c) = 2;
              *(undefined4 *)(lVar7 + 0x248) = 2;
              *(undefined4 *)(lVar7 + 0x284) = 2;
              *(undefined4 *)(lVar7 + 0x2c0) = 2;
              if ((uVar2 <= uVar6) || (*(int *)(lVar3 + 8 + uVar6 * 0xc) != in_R8D)) {
LAB_140fe78ff:
                *in_RCX = 4;
                return;
              }
              piVar1 = (int *)(lVar3 + uVar6 * 0xc);
              in_R8D = *piVar1;
              if (in_R8D == 0) goto LAB_140fe78ff;
              uVar6 = (ulonglong)(uint)piVar1[1];
            } while (uVar6 < uVar5);
          }
                    // WARNING: Subroutine does not return
          std::panicking::begin_panic<ref_<str_>_>();
        }
      }
                    // WARNING: Subroutine does not return
      std::panicking::begin_panic<ref_<str_>_>();
    }
  }
                    // WARNING: Subroutine does not return
  std::panicking::begin_panic<ref_<str_>_>();
}

