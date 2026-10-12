
void __cdecl system_roof::ui_edit_roof_ridge_dir::switch_roof_ridge_direction(void)

{
  longlong lVar1;
  undefined1 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  uint uVar7;
  ulonglong *puVar8;
  longlong lVar9;
  ulonglong uVar10;
  longlong lVar11;
  longlong lVar12;
  longlong lVar13;
  undefined4 *puVar14;
  longlong lVar15;
  undefined4 *puVar16;
  longlong lVar17;
  undefined4 *puVar18;
  longlong lVar19;
  undefined8 uVar20;
  undefined8 uVar21;
  ulonglong uVar22;
  longlong lVar23;
  ulonglong extraout_RAX;
  longlong lVar24;
  longlong *in_RDX;
  longlong lVar25;
  undefined8 *in_R8;
  longlong *in_R9;
  ulonglong uVar26;
  ulonglong uVar27;
  longlong lVar28;
  longlong lVar29;
  float extraout_XMM0_Da;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  undefined8 local_128;
  float local_120;
  
  puVar8 = (ulonglong *)*in_R8;
  lVar9 = in_R8[1];
  uVar10 = *puVar8;
  uVar26 = 0;
  uVar22 = uVar10 - *(ulonglong *)(lVar9 + 0x18);
  if (uVar10 < *(ulonglong *)(lVar9 + 0x18)) {
    uVar22 = uVar26;
  }
  uVar27 = uVar10 - *(ulonglong *)(lVar9 + 0x38);
  if (uVar10 < *(ulonglong *)(lVar9 + 0x38)) {
    uVar27 = uVar26;
  }
  lVar28 = uVar22 * 0x18 + *(longlong *)(lVar9 + 8);
  uVar10 = *(ulonglong *)(lVar9 + 0x10) - uVar22;
  if (*(ulonglong *)(lVar9 + 0x10) < uVar22) {
    lVar28 = 8;
    uVar10 = uVar26;
  }
  lVar23 = uVar27 * 0x18 + *(longlong *)(lVar9 + 0x28);
  uVar22 = *(ulonglong *)(lVar9 + 0x30) - uVar27;
  if (*(ulonglong *)(lVar9 + 0x30) < uVar27) {
    lVar23 = 8;
    uVar22 = uVar26;
  }
  *puVar8 = (*(longlong *)(lVar9 + 0x40) - uVar22) - uVar10;
  lVar9 = lVar23 + uVar22 * 0x18;
  lVar11 = *in_RDX;
  lVar12 = in_RDX[1];
  uVar3 = *(undefined4 *)((longlong)in_RDX + 0x14);
  uVar4 = *(undefined4 *)((longlong)in_R9 + 0x1c);
  lVar13 = *in_R9;
  puVar14 = (undefined4 *)in_R9[2];
  uVar5 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  lVar15 = *in_stack_00000028;
  puVar16 = (undefined4 *)in_stack_00000028[2];
  uVar6 = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
  lVar17 = *in_stack_00000030;
  puVar18 = (undefined4 *)in_stack_00000030[2];
  lVar29 = lVar28;
  while( true ) {
    if ((lVar29 == 0) || (lVar29 == lVar28 + uVar10 * 0x18)) {
      if ((lVar23 == 0) || (lVar23 == lVar9)) {
        return;
      }
      lVar25 = 0;
      lVar29 = lVar23;
      lVar23 = lVar23 + 0x18;
    }
    else {
      lVar25 = lVar29 + 0x18;
    }
    *puVar8 = *puVar8 + 1;
    country_core::resources::walls::public_walls::PublicWalls::roof_entity();
    if (extraout_RAX >> 0x20 == 0) {
                    /* WARNING: Subroutine does not return */
      core::option::unwrap_failed();
    }
    uVar22 = extraout_RAX & 0xffffffff;
    if ((*(ulonglong *)(lVar12 + 0x10) <= uVar22) ||
       (*(int *)(*(longlong *)(lVar12 + 8) + uVar22 * 0x14) != (int)(extraout_RAX >> 0x20))) break;
    lVar1 = *(longlong *)(lVar12 + 8) + uVar22 * 0x14;
    uVar22 = (ulonglong)*(uint *)(lVar1 + 4);
    if ((uVar22 == 0xffffffff) ||
       ((*(ulonglong *)(lVar11 + 0x38) <= uVar22 ||
        ((*(ulonglong *)(*(longlong *)(lVar11 + 0x28) + (ulonglong)(*(uint *)(lVar1 + 4) >> 6) * 8)
          >> (uVar22 & 0x3f) & 1) == 0)))) break;
    uVar7 = *(uint *)(lVar1 + 0x10);
    lVar19 = *(longlong *)
              (*(longlong *)(lVar12 + 0x1a8) + 0x18 + (ulonglong)*(uint *)(lVar1 + 0xc) * 0x48);
    lVar24 = ~*(ulonglong *)
               (*(longlong *)
                 (*(longlong *)(lVar12 + 0x1a8) + 0x38 + (ulonglong)*(uint *)(lVar1 + 0xc) * 0x48) +
               *(longlong *)(lVar11 + 0x110) * 8) * 0x30;
    lVar1 = *(longlong *)(lVar19 + 0x10 + lVar24);
    uVar2 = *(undefined1 *)(lVar29 + 0x10);
    *(undefined4 *)(*(longlong *)(lVar19 + 0x28 + lVar24) + (ulonglong)uVar7 * 4) = uVar3;
    *(undefined1 *)(lVar1 + 0x3c + (ulonglong)uVar7 * 0x58) = uVar2;
    uVar20 = *(undefined8 *)(lVar29 + 8);
    *puVar14 = uVar4;
    lVar1 = *(longlong *)(lVar13 + 0x30);
    uVar21 = *(undefined8 *)(lVar13 + 0x40);
    if (lVar1 == *(longlong *)(lVar13 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    lVar19 = *(longlong *)(lVar13 + 0x28);
    lVar24 = lVar1 * 0x20;
    *(undefined8 *)(lVar19 + lVar24) = uVar21;
    *(ulonglong *)(lVar19 + 8 + lVar24) = extraout_RAX;
    *(undefined8 *)(lVar19 + 0x10 + lVar24) = uVar20;
    *(undefined1 *)(lVar19 + 0x18 + lVar24) = 1;
    *(longlong *)(lVar13 + 0x30) = lVar1 + 1;
    *(longlong *)(lVar13 + 0x40) = *(longlong *)(lVar13 + 0x40) + 1;
    uVar20 = *(undefined8 *)(lVar29 + 8);
    *puVar16 = uVar5;
    lVar29 = *(longlong *)(lVar15 + 0x30);
    uVar21 = *(undefined8 *)(lVar15 + 0x40);
    if (lVar29 == *(longlong *)(lVar15 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                ();
    }
    lVar1 = *(longlong *)(lVar15 + 0x28);
    *(undefined8 *)(lVar1 + lVar29 * 0x10) = uVar21;
    *(undefined8 *)(lVar1 + 8 + lVar29 * 0x10) = uVar20;
    *(longlong *)(lVar15 + 0x30) = lVar29 + 1;
    *(longlong *)(lVar15 + 0x40) = *(longlong *)(lVar15 + 0x40) + 1;
    country_core::resources::roofs::roof_shape::Roof::pivot_3d();
    country_core::resources::roofs::roof_shape::Roof::height();
    *puVar18 = uVar6;
    lVar29 = *(longlong *)(lVar17 + 0x30);
    uVar20 = *(undefined8 *)(lVar17 + 0x40);
    if (lVar29 == *(longlong *)(lVar17 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                ();
    }
    lVar1 = *(longlong *)(lVar17 + 0x28);
    *(undefined8 *)(lVar1 + lVar29 * 0x28) = uVar20;
    *(undefined1 *)(lVar1 + 8 + lVar29 * 0x28) = 7;
    *(ulonglong *)(lVar1 + 0x18 + lVar29 * 0x28) =
         CONCAT44((float)((ulonglong)local_128 >> 0x20) + extraout_XMM0_Da,
                  (float)local_128 + extraout_XMM0_Da * 0.0);
    *(float *)(lVar1 + 0x20 + lVar29 * 0x28) = local_120 + extraout_XMM0_Da * 0.0;
    *(longlong *)(lVar17 + 0x30) = lVar29 + 1;
    *(longlong *)(lVar17 + 0x40) = *(longlong *)(lVar17 + 0x40) + 1;
    lVar29 = lVar25;
  }
                    /* WARNING: Subroutine does not return */
  core::result::unwrap_failed();
}

