// 0x14231ac20 sheep_pet_animation
// SIG: void __cdecl sheep_pet_animation(void)

void __cdecl system_critters::sheep::sheep_pet_animation::sheep_pet_animation(void)

{
  uint *puVar1;
  longlong lVar2;
  longlong *in_RCX;
  longlong lVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  undefined8 *in_R8;
  uint *puVar6;
  ulonglong uVar7;
  longlong lVar8;
  ulonglong uVar9;
  uint uVar10;
  longlong local_88;
  longlong local_80;
  ulonglong local_78;
  
  lVar8 = in_R8[1];
  uVar4 = *(ulonglong *)*in_R8;
  uVar9 = 0;
  uVar5 = uVar4 - *(ulonglong *)(lVar8 + 0x18);
  if (uVar4 < *(ulonglong *)(lVar8 + 0x18)) {
    uVar5 = uVar9;
  }
  uVar7 = uVar4 - *(ulonglong *)(lVar8 + 0x38);
  if (uVar4 < *(ulonglong *)(lVar8 + 0x38)) {
    uVar7 = uVar9;
  }
  uVar4 = *(ulonglong *)(lVar8 + 0x10) - uVar5;
  if (*(ulonglong *)(lVar8 + 0x10) < uVar5) {
    uVar4 = uVar9;
  }
  uVar5 = *(ulonglong *)(lVar8 + 0x30) - uVar7;
  if (*(ulonglong *)(lVar8 + 0x30) < uVar7) {
    uVar5 = uVar9;
  }
  *(ulonglong *)*in_R8 = *(longlong *)(lVar8 + 0x40) - (uVar5 + uVar4);
  alloc::vec::spec_from_iter::impl$0::
  from_iter<usize,core::iter::adapters::map::Map<bevy_ecs::event::iterators::EventIterator<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>,system_critters::sheep::sheep_pet_animation::sheep_pet_animation::closure_env_0>_>
            ();
  lVar8 = *in_RCX;
  lVar2 = in_RCX[1];
  puVar6 = *(uint **)(lVar8 + 0xf8);
  puVar1 = puVar6 + *(longlong *)(lVar8 + 0x100);
  if (*(char *)(lVar8 + 0x108) == '\0') {
    if (puVar6 != puVar1) {
      do {
        if (*(longlong *)(*(longlong *)(lVar2 + 0x100) + 0x58 + (ulonglong)*puVar6 * 0xa0) != 0) {
          lVar3 = *(longlong *)(lVar2 + 0x100) + (ulonglong)*puVar6 * 0xa0;
          uVar4 = (ulonglong)*(uint *)(lVar3 + 0x94);
          lVar8 = *(longlong *)
                   (*(longlong *)(*(longlong *)(lVar2 + 0x1a8) + 0x18 + uVar4 * 0x48) + 0x10 +
                   ~*(ulonglong *)
                     (*(longlong *)(*(longlong *)(lVar2 + 0x1a8) + 0x38 + uVar4 * 0x48) +
                     *(longlong *)(lVar8 + 0x110) * 8) * 0x30);
          uVar4 = (ulonglong)*(uint *)(*(longlong *)(lVar3 + 0x50) + 8);
          goto LAB_14231ae78;
        }
        puVar6 = puVar6 + 1;
      } while (puVar6 != puVar1);
    }
  }
  else if (puVar6 != puVar1) {
    while (*(longlong *)(*(longlong *)(lVar2 + 0x1a8) + 0x10 + (ulonglong)*puVar6 * 0x48) == 0) {
      puVar6 = puVar6 + 1;
      if (puVar6 == puVar1) goto LAB_14231b0f7;
    }
    lVar2 = *(longlong *)(lVar2 + 0x1a8) + (ulonglong)*puVar6 * 0x48;
    lVar8 = *(longlong *)
             (*(longlong *)(lVar2 + 0x18) + 0x10 +
             ~*(ulonglong *)(*(longlong *)(lVar2 + 0x38) + *(longlong *)(lVar8 + 0x110) * 8) * 0x30)
    ;
    uVar4 = 0;
LAB_14231ae78:
    lVar8 = uVar4 * 0x68 + lVar8;
    if (lVar8 == 0) goto LAB_14231b0f7;
    lVar2 = *(longlong *)(lVar8 + 0x20);
    uVar5 = local_78 & 0xfffffffffffffff8;
    uVar10 = (uint)local_78 & 7;
    uVar4 = 0;
    while (uVar5 != uVar4) {
      if (((((*(longlong *)(local_80 + uVar4 * 8) == lVar2) ||
            (*(longlong *)(local_80 + 8 + uVar4 * 8) == lVar2)) ||
           ((*(longlong *)(local_80 + 0x10 + uVar4 * 8) == lVar2 ||
            ((*(longlong *)(local_80 + 0x18 + uVar4 * 8) == lVar2 ||
             (*(longlong *)(local_80 + 0x20 + uVar4 * 8) == lVar2)))))) ||
          (*(longlong *)(local_80 + 0x28 + uVar4 * 8) == lVar2)) ||
         ((*(longlong *)(local_80 + 0x30 + uVar4 * 8) == lVar2 ||
          (lVar3 = uVar4 * 8, uVar4 = uVar4 + 8, *(longlong *)(local_80 + 0x38 + lVar3) == lVar2))))
      goto LAB_14231afd0;
    }
    if (((((((local_78 & 7) != 0) && (*(longlong *)(local_80 + uVar5 * 8) != lVar2)) &&
          (uVar10 != 1)) && ((*(longlong *)(local_80 + 8 + uVar5 * 8) != lVar2 && (uVar10 != 2))))
        && ((*(longlong *)(local_80 + 0x10 + uVar5 * 8) != lVar2 &&
            ((uVar10 != 3 && (*(longlong *)(local_80 + 0x18 + uVar5 * 8) != lVar2)))))) &&
       ((uVar10 != 4 &&
        ((((*(longlong *)(local_80 + 0x20 + uVar5 * 8) != lVar2 && (uVar10 != 5)) &&
          (*(longlong *)(local_80 + 0x28 + uVar5 * 8) != lVar2)) && (uVar10 != 6)))))) {
                    // WARNING: Could not recover jumptable at 0x00014231afc3. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_14304b178 + *(int *)(&DAT_14304b178 + (ulonglong)*(uint *)(lVar8 + 0x18) * 4))
      )(0x3e99999a,0x3f800000,0x3e4ccccd);
      return;
    }
LAB_14231afd0:
                    // WARNING: Could not recover jumptable at 0x00014231afe2. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&DAT_14304b178 + *(int *)(&DAT_14304b178 + (ulonglong)*(uint *)(lVar8 + 0x18) * 4)))
              (&DAT_14304b178 + *(int *)(&DAT_14304b178 + (ulonglong)*(uint *)(lVar8 + 0x18) * 4));
    return;
  }
LAB_14231b0f7:
  if (local_88 == 0) {
    return;
  }
  std::alloc::__default_lib_allocator::__rust_dealloc();
  return;
}

