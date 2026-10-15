// name=set_flag_connection_and_chimney_smoke
// sig=void __cdecl set_flag_connection_and_chimney_smoke(void)
// size=929

void __cdecl
system_color::set_flag_connection_n_chimney_smoke::set_flag_connection_and_chimney_smoke(void)

{
  undefined8 *puVar1;
  undefined1 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  ulonglong *puVar5;
  longlong lVar6;
  ulonglong uVar7;
  undefined4 *puVar8;
  undefined4 *puVar9;
  longlong lVar10;
  undefined8 uVar11;
  ulonglong uVar12;
  longlong lVar13;
  longlong extraout_RAX;
  undefined1 *extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong in_RCX;
  longlong lVar14;
  undefined8 *in_RDX;
  longlong lVar15;
  ulonglong uVar16;
  longlong lVar17;
  undefined8 *in_R8;
  longlong *in_R9;
  ulonglong uVar18;
  longlong lVar19;
  longlong lVar20;
  undefined6 uStack_72;
  undefined8 uStack_6c;
  undefined4 local_64;
  
  puVar5 = (ulonglong *)*in_RDX;
  lVar6 = in_RDX[1];
  uVar7 = *puVar5;
  uVar18 = 0;
  uVar12 = uVar7 - *(ulonglong *)(lVar6 + 0x18);
  if (uVar7 < *(ulonglong *)(lVar6 + 0x18)) {
    uVar12 = uVar18;
  }
  uVar16 = uVar7 - *(ulonglong *)(lVar6 + 0x38);
  if (uVar7 < *(ulonglong *)(lVar6 + 0x38)) {
    uVar16 = uVar18;
  }
  lVar15 = uVar12 * 0x10 + *(longlong *)(lVar6 + 8);
  uVar7 = *(ulonglong *)(lVar6 + 0x10) - uVar12;
  if (*(ulonglong *)(lVar6 + 0x10) < uVar12) {
    lVar15 = 8;
    uVar7 = uVar18;
  }
  lVar13 = uVar16 * 0x10 + *(longlong *)(lVar6 + 0x28);
  uVar12 = *(ulonglong *)(lVar6 + 0x30) - uVar16;
  if (*(ulonglong *)(lVar6 + 0x30) < uVar16) {
    lVar13 = 8;
    uVar12 = uVar18;
  }
  *puVar5 = (*(longlong *)(lVar6 + 0x40) - uVar12) - uVar7;
  lVar19 = uVar7 * 0x10 + lVar15;
  lVar20 = uVar12 * 0x10 + lVar13;
  uVar3 = *(undefined4 *)(in_RCX + 0x1c);
  puVar8 = *(undefined4 **)(in_RCX + 0x10);
  uVar4 = *(undefined4 *)((longlong)in_R9 + 0x1c);
  lVar6 = *in_R9;
  puVar9 = (undefined4 *)in_R9[2];
  do {
    if ((lVar15 == 0) || (lVar15 == lVar19)) {
      if ((lVar13 == 0) || (lVar13 == lVar20)) {
        puVar5 = (ulonglong *)*in_R8;
        lVar15 = in_R8[1];
        uVar7 = *puVar5;
        uVar18 = 0;
        uVar12 = uVar7 - *(ulonglong *)(lVar15 + 0x18);
        if (uVar7 < *(ulonglong *)(lVar15 + 0x18)) {
          uVar12 = uVar18;
        }
        uVar16 = uVar7 - *(ulonglong *)(lVar15 + 0x38);
        if (uVar7 < *(ulonglong *)(lVar15 + 0x38)) {
          uVar16 = uVar18;
        }
        lVar13 = uVar12 * 0x10 + *(longlong *)(lVar15 + 8);
        uVar7 = *(ulonglong *)(lVar15 + 0x10) - uVar12;
        if (*(ulonglong *)(lVar15 + 0x10) < uVar12) {
          lVar13 = 8;
          uVar7 = uVar18;
        }
        lVar19 = uVar16 * 0x10 + *(longlong *)(lVar15 + 0x28);
        uVar12 = *(ulonglong *)(lVar15 + 0x30) - uVar16;
        if (*(ulonglong *)(lVar15 + 0x30) < uVar16) {
          lVar19 = 8;
          uVar12 = uVar18;
        }
        *puVar5 = (*(longlong *)(lVar15 + 0x40) - uVar12) - uVar7;
        lVar20 = uVar7 * 0x10 + lVar13;
        lVar15 = uVar12 * 0x10 + lVar19;
        do {
          if ((lVar13 == 0) || (lVar13 == lVar20)) {
            if ((lVar19 == 0) || (lVar19 == lVar15)) {
              return;
            }
            lVar13 = 0;
            lVar17 = lVar19;
            lVar19 = lVar19 + 0x10;
          }
          else {
            lVar17 = lVar13;
            lVar13 = lVar13 + 0x10;
          }
          *puVar5 = *puVar5 + 1;
          *puVar8 = uVar3;
          country_core::resources::walls::decorator_storage::DecoratorStorage::
          lookup_decorator_info_mut();
          if (extraout_RAX_01 == 0) {
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
          uVar2 = *(undefined1 *)(lVar17 + 0xc);
          country_core::resources::walls::decorator::DecoratorExtraInfo::as_chimney();
          *(undefined1 *)(extraout_RAX_02 + 4) = uVar2;
          if (*(char *)(lVar17 + 0xd) == '\x01') {
            uVar2 = *(undefined1 *)(lVar17 + 0xc);
            country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::
            position();
            *puVar9 = uVar4;
            lVar17 = *(longlong *)(lVar6 + 0x30);
            uVar11 = *(undefined8 *)(lVar6 + 0x40);
            if (lVar17 == *(longlong *)(lVar6 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                        ();
            }
            lVar10 = *(longlong *)(lVar6 + 0x28);
            lVar14 = lVar17 * 0x20;
            *(undefined8 *)(lVar10 + lVar14) = uVar11;
            *(undefined4 *)(lVar10 + 0x18 + lVar14) = local_64;
            puVar1 = (undefined8 *)(lVar10 + 8 + lVar14);
            *puVar1 = CONCAT62(uStack_72,CONCAT11(uVar2,0xf));
            puVar1[1] = uStack_6c;
            *(longlong *)(lVar6 + 0x30) = lVar17 + 1;
            *(longlong *)(lVar6 + 0x40) = *(longlong *)(lVar6 + 0x40) + 1;
          }
        } while( true );
      }
      lVar15 = 0;
      lVar17 = lVar13;
      lVar13 = lVar13 + 0x10;
    }
    else {
      lVar17 = lVar15;
      lVar15 = lVar15 + 0x10;
    }
    *puVar5 = *puVar5 + 1;
    *puVar8 = uVar3;
    country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_decorator_info_mut()
    ;
    if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    uVar2 = *(undefined1 *)(lVar17 + 0xc);
    country_core::resources::walls::decorator::DecoratorExtraInfo::as_flag();
    *extraout_RAX_00 = uVar2;
    if (*(char *)(lVar17 + 0xd) != '\0') {
      uVar2 = *(undefined1 *)(lVar17 + 0xc);
      country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::
      position();
      *puVar9 = uVar4;
      lVar17 = *(longlong *)(lVar6 + 0x30);
      uVar11 = *(undefined8 *)(lVar6 + 0x40);
      if (lVar17 == *(longlong *)(lVar6 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                  ();
      }
      lVar10 = *(longlong *)(lVar6 + 0x28);
      lVar14 = lVar17 * 0x20;
      *(undefined8 *)(lVar10 + lVar14) = uVar11;
      *(undefined4 *)(lVar10 + 0x18 + lVar14) = local_64;
      puVar1 = (undefined8 *)(lVar10 + 8 + lVar14);
      *puVar1 = CONCAT62(uStack_72,CONCAT11(uVar2,9));
      puVar1[1] = uStack_6c;
      *(longlong *)(lVar6 + 0x30) = lVar17 + 1;
      *(longlong *)(lVar6 + 0x40) = *(longlong *)(lVar6 + 0x40) + 1;
    }
  } while( true );
}

