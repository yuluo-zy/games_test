// name=set_window_light
// sig=void __cdecl set_window_light(void)
// size=582

void __cdecl system_color::set_window_light::set_window_light(void)

{
  undefined8 *puVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  ulonglong *puVar5;
  longlong lVar6;
  ulonglong uVar7;
  undefined4 *puVar8;
  longlong lVar9;
  undefined4 *puVar10;
  undefined8 uVar11;
  byte bVar12;
  longlong extraout_RAX;
  undefined4 *extraout_RAX_00;
  longlong in_RCX;
  longlong lVar13;
  undefined8 *in_RDX;
  ulonglong uVar14;
  longlong lVar15;
  longlong lVar16;
  longlong lVar17;
  longlong *in_R8;
  ulonglong uVar18;
  ulonglong uVar19;
  longlong lVar20;
  undefined1 uVar21;
  longlong lVar22;
  undefined4 local_98;
  undefined4 local_54;
  undefined8 uStack_4c;
  undefined4 local_44;
  
  puVar5 = (ulonglong *)*in_RDX;
  lVar6 = in_RDX[1];
  uVar7 = *puVar5;
  uVar14 = 0;
  uVar19 = uVar7 - *(ulonglong *)(lVar6 + 0x18);
  if (uVar7 < *(ulonglong *)(lVar6 + 0x18)) {
    uVar19 = uVar14;
  }
  uVar18 = uVar7 - *(ulonglong *)(lVar6 + 0x38);
  if (uVar7 < *(ulonglong *)(lVar6 + 0x38)) {
    uVar18 = uVar14;
  }
  lVar15 = uVar19 * 0x18 + *(longlong *)(lVar6 + 8);
  uVar7 = *(ulonglong *)(lVar6 + 0x10) - uVar19;
  if (*(ulonglong *)(lVar6 + 0x10) < uVar19) {
    lVar15 = 8;
    uVar7 = uVar14;
  }
  lVar22 = uVar18 * 0x18 + *(longlong *)(lVar6 + 0x28);
  uVar19 = *(ulonglong *)(lVar6 + 0x30) - uVar18;
  if (*(ulonglong *)(lVar6 + 0x30) < uVar18) {
    lVar22 = 8;
    uVar19 = uVar14;
  }
  *puVar5 = (*(longlong *)(lVar6 + 0x40) - uVar19) - uVar7;
  lVar6 = lVar22 + uVar19 * 0x18;
  uVar2 = *(undefined4 *)(in_RCX + 0x1c);
  puVar8 = *(undefined4 **)(in_RCX + 0x10);
  uVar3 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  lVar9 = *in_R8;
  puVar10 = (undefined4 *)in_R8[2];
  lVar16 = lVar15;
  do {
    if ((lVar16 == 0) || (lVar16 == lVar15 + uVar7 * 0x18)) {
      if ((lVar22 == 0) || (lVar22 == lVar6)) {
        return;
      }
      lVar17 = 0;
      lVar20 = lVar22;
      lVar22 = lVar22 + 0x18;
    }
    else {
      lVar17 = lVar16 + 0x18;
      lVar20 = lVar16;
    }
    *puVar5 = *puVar5 + 1;
    *puVar8 = uVar2;
    country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_decorator_info_mut()
    ;
    if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    uVar4 = *(undefined4 *)(lVar20 + 0xc);
    country_core::resources::walls::decorator::DecoratorExtraInfo::as_window();
    *extraout_RAX_00 = uVar4;
    lVar16 = lVar17;
    if (*(char *)(lVar20 + 0x10) == '\x01') {
      bVar12 = *(char *)(extraout_RAX + 0x46) - 0x38;
      if (7 < bVar12) {
        bVar12 = 6;
      }
      uVar21 = 10;
      if ((0xcfU >> (bVar12 & 0x1f) & 1) == 0) {
        if (bVar12 == 4) {
          local_98 = *(undefined4 *)(extraout_RAX + 0x28);
          uVar21 = 0xd;
        }
        else {
          uVar21 = 8;
        }
      }
      country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::
      position();
      local_54 = CONCAT31(local_54._1_3_,uVar21);
      *puVar10 = uVar3;
      lVar17 = *(longlong *)(lVar9 + 0x30);
      uVar11 = *(undefined8 *)(lVar9 + 0x40);
      if (lVar17 == *(longlong *)(lVar9 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                  ();
      }
      lVar20 = *(longlong *)(lVar9 + 0x28);
      lVar13 = lVar17 * 0x20;
      *(undefined8 *)(lVar20 + lVar13) = uVar11;
      *(undefined4 *)(lVar20 + 0x18 + lVar13) = local_44;
      puVar1 = (undefined8 *)(lVar20 + 8 + lVar13);
      *puVar1 = CONCAT44(local_98,local_54);
      puVar1[1] = uStack_4c;
      *(longlong *)(lVar9 + 0x30) = lVar17 + 1;
      *(longlong *)(lVar9 + 0x40) = *(longlong *)(lVar9 + 0x40) + 1;
    }
  } while( true );
}

