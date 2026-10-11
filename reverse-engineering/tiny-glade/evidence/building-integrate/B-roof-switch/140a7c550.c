
void __cdecl country_core::systems::history::replay_mode::replay_mode(void)

{
  int iVar1;
  ulonglong uVar2;
  uint *puVar3;
  float fVar4;
  longlong lVar5;
  undefined4 uVar6;
  undefined8 *in_RCX;
  longlong *in_RDX;
  int *piVar7;
  undefined4 *puVar8;
  longlong lVar9;
  undefined4 uVar10;
  longlong *in_R8;
  int *piVar11;
  int iVar12;
  undefined8 *in_R9;
  int *piVar13;
  longlong lVar14;
  int *piVar15;
  undefined8 uVar16;
  int *piVar17;
  bool bVar18;
  bool bVar19;
  bool bVar20;
  bool bVar21;
  float extraout_XMM0_Da;
  float fVar22;
  float fVar23;
  float fVar24;
  undefined8 *in_stack_00000028;
  undefined8 *in_stack_00000030;
  undefined8 *in_stack_00000038;
  longlong *in_stack_00000040;
  longlong *in_stack_00000048;
  longlong *in_stack_00000050;
  undefined8 *in_stack_00000060;
  undefined8 local_d8;
  int local_d0;
  
  lVar5 = in_R9[1];
  uVar2 = *(ulonglong *)*in_R9;
  piVar7 = (int *)0x0;
  piVar17 = (int *)(uVar2 - *(ulonglong *)(lVar5 + 0x18));
  if (uVar2 < *(ulonglong *)(lVar5 + 0x18)) {
    piVar17 = piVar7;
  }
  piVar13 = (int *)(uVar2 - *(ulonglong *)(lVar5 + 0x38));
  if (uVar2 < *(ulonglong *)(lVar5 + 0x38)) {
    piVar13 = piVar7;
  }
  lVar14 = (longlong)piVar17 * 0x28 + *(longlong *)(lVar5 + 8);
  lVar9 = (longlong)piVar13 * 0x28 + *(longlong *)(lVar5 + 0x28);
  bVar18 = *(int **)(lVar5 + 0x10) < piVar17;
  piVar17 = (int *)((longlong)*(int **)(lVar5 + 0x10) - (longlong)piVar17);
  bVar20 = piVar17 == (int *)0x0;
  if (bVar18) {
    piVar17 = piVar7;
  }
  if (bVar18) {
    lVar14 = 8;
  }
  piVar15 = (int *)(lVar14 + -0x28 + (longlong)piVar17 * 0x28);
  if (bVar18 || bVar20) {
    piVar15 = piVar7;
  }
  bVar19 = *(int **)(lVar5 + 0x30) < piVar13;
  piVar13 = (int *)((longlong)*(int **)(lVar5 + 0x30) - (longlong)piVar13);
  bVar21 = piVar13 == (int *)0x0;
  if (bVar19) {
    lVar9 = 8;
    piVar13 = piVar7;
  }
  piVar11 = (int *)(lVar9 + -0x28 + (longlong)piVar13 * 0x28);
  if (bVar19 || bVar21) {
    piVar11 = piVar15;
  }
  piVar17 = (int *)((longlong)piVar13 + (longlong)piVar17);
  if (!bVar18 && !bVar20 || !bVar19 && !bVar21) {
    piVar17 = piVar7;
  }
  *(ulonglong *)*in_R9 = *(longlong *)(lVar5 + 0x40) - (longlong)piVar17;
  if ((!bVar18 && !bVar20 || !bVar19 && !bVar21) && (*piVar11 == 0xd)) {
    iVar1 = piVar11[4];
    iVar12 = 0;
    switch(iVar1) {
    case 6:
    case 7:
    case 8:
    case 9:
    case 10:
    case 0xb:
    case 0xc:
    case 0xf:
    case 0x10:
    case 0x11:
    case 0x12:
    case 0x13:
      local_d0 = piVar11[7];
      local_d8 = *(undefined8 *)(piVar11 + 5);
      iVar12 = iVar1;
      break;
    case 0xd:
    case 0xe:
      piVar17 = (int *)*in_stack_00000030;
      iVar12 = *piVar17;
      local_d0 = piVar17[3];
      local_d8 = *(undefined8 *)(piVar17 + 1);
      break;
    case 0x14:
      break;
    default:
      iVar12 = 10;
      if (iVar1 != 5) {
        iVar12 = 0;
      }
    }
    *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
    piVar17 = (int *)*in_stack_00000030;
    *piVar17 = iVar12;
    *(undefined8 *)(piVar17 + 1) = local_d8;
    piVar17[3] = local_d0;
    *(undefined4 *)in_stack_00000038[2] = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
    *(undefined8 *)*in_stack_00000038 = 1;
    *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
    *(undefined4 *)(*in_R8 + 0x28) = 0x3f800000;
    *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
    *(undefined8 *)*in_stack_00000028 = 0;
  }
  if (*(int *)*in_RCX != 0xd) {
    if ((DAT_143634d88 & 1) == 0) {
      return;
    }
    if (3 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      ::log::__private_api::impl$2::log();
      DAT_143634d88 = 0;
      return;
    }
    DAT_143634d88 = 0;
    return;
  }
  if ((DAT_143634d88 & 1) == 0) {
    if (3 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      ::log::__private_api::impl$2::log();
    }
    DAT_143634d88 = 1;
  }
  systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer_and_sound();
  fVar24 = __real_3e4ccccd;
  fVar22 = __real_3f800000;
  if ((*(byte *)*in_stack_00000060 & 1) != 0) {
    uVar6 = *(undefined4 *)((byte *)*in_stack_00000060 + 4);
    *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
    *(undefined4 *)(*in_R8 + 0x28) = uVar6;
    return;
  }
  puVar3 = (uint *)*in_stack_00000028;
  lVar5 = *in_stack_00000048;
  if ((*puVar3 & 1) == 0) {
    if ((*(uint *)(lVar5 + 0x80) < 3) || (fVar23 = 0.0, *(uint *)(lVar5 + 0x88) < 3)) {
      fVar23 = (float)puVar3[1] + *(float *)(*in_stack_00000050 + 0x28);
    }
    bVar18 = __real_3e4ccccd < fVar23;
    fVar4 = __real_3f800000;
    if (fVar23 <= __real_3e4ccccd) {
      fVar4 = fVar23;
    }
    *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
    *puVar3 = (uint)bVar18;
    puVar3[1] = (uint)fVar4;
    lVar5 = *in_R8;
    if (fVar23 <= fVar24) {
      uVar6 = *(undefined4 *)((longlong)in_R8 + 0x1c);
      puVar8 = (undefined4 *)in_R8[2];
LAB_140a7cafc:
      lVar9 = (*(longlong *)(*in_RDX + 0x18) + 1) - (ulonglong)(*(longlong *)(lVar5 + 0x18) == 0);
      fVar23 = (float)(ulonglong)(lVar9 * lVar9) * __real_3f000000;
      fVar22 = *(float *)(lVar5 + 0x28);
      fVar24 = fVar23;
      if (fVar23 <= fVar22) {
        fVar24 = fVar22;
      }
      *puVar8 = uVar6;
      *(uint *)(lVar5 + 0x28) =
           ~-(uint)NAN(fVar22) & (uint)fVar24 | -(uint)NAN(fVar22) & (uint)fVar23;
      return;
    }
    goto LAB_140a7ca1e;
  }
  if (*(uint *)(lVar5 + 0x80) < 3) {
    *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
    lVar5 = *in_stack_00000040;
    lVar9 = *(longlong *)(lVar5 + 0x30);
    uVar16 = *(undefined8 *)(lVar5 + 0x40);
    uVar10 = 0x3000000;
    uVar6 = 0x3000000;
    if (lVar9 == *(longlong *)(lVar5 + 0x20)) {
LAB_140a7c95a:
      uVar10 = uVar6;
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                ();
    }
  }
  else {
    if (2 < *(uint *)(lVar5 + 0x88)) {
      *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
      lVar5 = *in_stack_00000040;
      lVar9 = *(longlong *)(lVar5 + 0x30);
      uVar16 = *(undefined8 *)(lVar5 + 0x40);
      if (lVar9 == *(longlong *)(lVar5 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar14 = *(longlong *)(lVar5 + 0x28);
      *(undefined8 *)(lVar14 + lVar9 * 0x10) = uVar16;
      *(undefined4 *)(lVar14 + 8 + lVar9 * 0x10) = 0x3000200;
      *(longlong *)(lVar5 + 0x30) = lVar9 + 1;
      *(longlong *)(lVar5 + 0x40) = *(longlong *)(lVar5 + 0x40) + 1;
      uVar6 = *(undefined4 *)((longlong)in_R8 + 0x1c);
      puVar8 = (undefined4 *)in_R8[2];
      *puVar8 = uVar6;
      lVar5 = *in_R8;
      *(undefined4 *)(lVar5 + 0x28) = 0x3f800000;
      *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
      puVar3[0] = 0;
      puVar3[1] = 0;
      goto LAB_140a7cafc;
    }
    *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
    lVar5 = *in_stack_00000040;
    lVar9 = *(longlong *)(lVar5 + 0x30);
    uVar16 = *(undefined8 *)(lVar5 + 0x40);
    uVar10 = 0x3000100;
    uVar6 = 0x3000100;
    if (lVar9 == *(longlong *)(lVar5 + 0x20)) goto LAB_140a7c95a;
  }
  lVar14 = *(longlong *)(lVar5 + 0x28);
  *(undefined8 *)(lVar14 + lVar9 * 0x10) = uVar16;
  *(undefined4 *)(lVar14 + 8 + lVar9 * 0x10) = uVar10;
  *(longlong *)(lVar5 + 0x30) = lVar9 + 1;
  *(longlong *)(lVar5 + 0x40) = *(longlong *)(lVar5 + 0x40) + 1;
  ::utils::time_continuous_lerp();
  *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  *puVar3 = 1;
  puVar3[1] = (uint)extraout_XMM0_Da;
  lVar5 = *in_R8;
  fVar22 = extraout_XMM0_Da;
LAB_140a7ca1e:
  *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
  *(float *)(lVar5 + 0x28) = fVar22;
  return;
}

