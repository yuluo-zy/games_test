// edit_rectangle addr=0x14097f620 size=834 sig=void __cdecl edit_rectangle(void)

void __cdecl country_core::systems::wall::shapes::rectangle::edit_rectangle::edit_rectangle(void)

{
  undefined4 uVar1;
  ulonglong *puVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  undefined8 uVar5;
  undefined4 *puVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  longlong lVar9;
  undefined8 *in_RCX;
  ulonglong uVar10;
  longlong lVar11;
  longlong in_RDX;
  ulonglong uVar12;
  longlong lVar13;
  longlong *in_R8;
  ulonglong uVar14;
  longlong *in_R9;
  ulonglong uVar15;
  longlong lVar16;
  ulonglong uVar17;
  longlong lVar18;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar19;
  float fVar20;
  
  puVar2 = (ulonglong *)*in_RCX;
  lVar18 = in_RCX[1];
  uVar3 = *puVar2;
  uVar12 = 0;
  uVar14 = uVar3 - *(ulonglong *)(lVar18 + 0x18);
  if (uVar3 < *(ulonglong *)(lVar18 + 0x18)) {
    uVar14 = uVar12;
  }
  uVar10 = uVar3 - *(ulonglong *)(lVar18 + 0x38);
  if (uVar3 < *(ulonglong *)(lVar18 + 0x38)) {
    uVar10 = uVar12;
  }
  uVar3 = *(ulonglong *)(lVar18 + 0x10);
  uVar4 = *(ulonglong *)(lVar18 + 0x30);
  uVar15 = uVar4 - uVar10;
  if (uVar4 < uVar10) {
    uVar15 = uVar12;
  }
  uVar17 = uVar3 - uVar14;
  if (uVar3 < uVar14) {
    uVar17 = uVar12;
  }
  uVar12 = *(ulonglong *)(lVar18 + 0x40);
  lVar16 = *(longlong *)(lVar18 + 8);
  lVar18 = *(longlong *)(lVar18 + 0x28);
  *puVar2 = (uVar12 - uVar15) - uVar17;
  if (uVar3 >= uVar14 && uVar3 - uVar14 != 0 || uVar4 >= uVar10 && uVar4 - uVar10 != 0) {
    lVar18 = lVar18 + uVar10 * 0x40;
    lVar16 = lVar16 + uVar14 * 0x40;
    if (uVar3 < uVar14) {
      lVar16 = 8;
    }
    lVar13 = 0;
    if (uVar14 < uVar3) {
      lVar13 = lVar16 + -0x40 + uVar17 * 0x40;
    }
    if (uVar4 < uVar10) {
      lVar18 = 8;
    }
    lVar18 = lVar18 + -0x40 + uVar15 * 0x40;
    if (uVar4 <= uVar10) {
      lVar18 = lVar13;
    }
    *puVar2 = uVar12;
    uVar5 = *(undefined8 *)(lVar18 + 8);
    uVar1 = *(undefined4 *)(in_RDX + 0x1c);
    puVar6 = *(undefined4 **)(in_RDX + 0x10);
    *puVar6 = uVar1;
    ::utils::geometry::rectangle::Rectangle2d::as_curve2();
    resources::walls::inner_walls::InnerWalls::
    _try_update_state<country_core::resources::walls::inner_walls::impl_1::update_wall_state_curve::closure_env_0>
              ();
    *puVar6 = uVar1;
    resources::walls::inner_walls::InnerWalls::
    _try_update_state<country_core::resources::walls::inner_walls::impl_1::update_wall_state_enclosure::closure_env_0>
              ();
    uVar1 = *(undefined4 *)(lVar18 + 0x30);
    uVar7 = *(undefined8 *)(lVar18 + 0x28);
    *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
    lVar16 = *in_R8;
    lVar13 = *(longlong *)(lVar16 + 0x30);
    uVar8 = *(undefined8 *)(lVar16 + 0x40);
    if (lVar13 == *(longlong *)(lVar16 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    lVar9 = *(longlong *)(lVar16 + 0x28);
    lVar11 = lVar13 * 0x20;
    *(undefined8 *)(lVar9 + lVar11) = uVar8;
    *(undefined8 *)(lVar9 + 8 + lVar11) = uVar5;
    *(undefined8 *)(lVar9 + 0x10 + lVar11) = uVar7;
    *(undefined4 *)(lVar9 + 0x18 + lVar11) = uVar1;
    *(undefined2 *)(lVar9 + 0x1c + lVar11) = 4;
    *(longlong *)(lVar16 + 0x30) = lVar13 + 1;
    *(longlong *)(lVar16 + 0x40) = *(longlong *)(lVar16 + 0x40) + 1;
    if (*(char *)(lVar18 + 0x38) == '\0') {
      uVar1 = *(undefined4 *)(lVar18 + 0x30);
      uVar7 = *(undefined8 *)(lVar18 + 0x28);
      fVar20 = 0.0;
      ::utils::inverse_lerp();
      if (0.0 <= extraout_XMM0_Da_00) {
        fVar20 = extraout_XMM0_Da_00;
      }
      fVar19 = 1.0;
      if (fVar20 <= 1.0) {
        fVar19 = fVar20;
      }
      *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
      lVar18 = *in_R9;
      lVar16 = *(longlong *)(lVar18 + 0x30);
      uVar8 = *(undefined8 *)(lVar18 + 0x40);
      if (lVar16 == *(longlong *)(lVar18 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                  ();
      }
      lVar13 = *(longlong *)(lVar18 + 0x28);
      *(undefined8 *)(lVar13 + lVar16 * 0x28) = uVar8;
      *(undefined8 *)(lVar13 + 8 + lVar16 * 0x28) = uVar5;
      *(undefined8 *)(lVar13 + 0x10 + lVar16 * 0x28) = uVar7;
      *(undefined4 *)(lVar13 + 0x18 + lVar16 * 0x28) = uVar1;
      *(float *)(lVar13 + 0x1c + lVar16 * 0x28) = fVar19;
      *(undefined1 *)(lVar13 + 0x20 + lVar16 * 0x28) = 0;
    }
    else {
      uVar1 = *(undefined4 *)(lVar18 + 0x30);
      uVar7 = *(undefined8 *)(lVar18 + 0x28);
      fVar20 = 0.0;
      ::utils::inverse_lerp();
      if (0.0 <= extraout_XMM0_Da) {
        fVar20 = extraout_XMM0_Da;
      }
      fVar19 = 1.0;
      if (fVar20 <= 1.0) {
        fVar19 = fVar20;
      }
      *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
      lVar18 = *in_R9;
      lVar16 = *(longlong *)(lVar18 + 0x30);
      uVar8 = *(undefined8 *)(lVar18 + 0x40);
      if (lVar16 == *(longlong *)(lVar18 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                  ();
      }
      lVar13 = *(longlong *)(lVar18 + 0x28);
      *(undefined8 *)(lVar13 + lVar16 * 0x28) = uVar8;
      *(undefined8 *)(lVar13 + 8 + lVar16 * 0x28) = uVar5;
      *(undefined8 *)(lVar13 + 0x10 + lVar16 * 0x28) = uVar7;
      *(undefined4 *)(lVar13 + 0x18 + lVar16 * 0x28) = uVar1;
      *(float *)(lVar13 + 0x1c + lVar16 * 0x28) = fVar19;
      *(undefined1 *)(lVar13 + 0x20 + lVar16 * 0x28) = 1;
    }
    *(longlong *)(lVar18 + 0x30) = lVar16 + 1;
    *(longlong *)(lVar18 + 0x40) = *(longlong *)(lVar18 + 0x40) + 1;
  }
  return;
}

