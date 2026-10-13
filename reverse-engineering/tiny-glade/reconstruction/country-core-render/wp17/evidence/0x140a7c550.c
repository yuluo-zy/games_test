// 0x140a7c550 replay_mode

void __cdecl country_core::systems::history::replay_mode::replay_mode(void)

{
  int iVar1;
  ulonglong uVar2;
  uint *puVar3;
  longlong lVar4;
  undefined4 uVar5;
  undefined8 *in_RCX;
  longlong *in_RDX;
  int *piVar6;
  undefined4 *puVar7;
  longlong lVar8;
  undefined4 uVar9;
  longlong *in_R8;
  int *piVar10;
  int iVar11;
  undefined8 *in_R9;
  int *piVar12;
  longlong lVar13;
  int *piVar14;
  undefined8 uVar15;
  int *piVar16;
  bool bVar17;
  bool bVar18;
  bool bVar19;
  bool bVar20;
  uint extraout_XMM0_Da;
  uint uVar21;
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
  
  lVar4 = in_R9[1];
  uVar2 = *(ulonglong *)*in_R9;
  piVar6 = (int *)0x0;
  piVar16 = (int *)(uVar2 - *(ulonglong *)(lVar4 + 0x18));
  if (uVar2 < *(ulonglong *)(lVar4 + 0x18)) {
    piVar16 = piVar6;
  }
  piVar12 = (int *)(uVar2 - *(ulonglong *)(lVar4 + 0x38));
  if (uVar2 < *(ulonglong *)(lVar4 + 0x38)) {
    piVar12 = piVar6;
  }
  lVar13 = (longlong)piVar16 * 0x28 + *(longlong *)(lVar4 + 8);
  lVar8 = (longlong)piVar12 * 0x28 + *(longlong *)(lVar4 + 0x28);
  bVar17 = *(int **)(lVar4 + 0x10) < piVar16;
  piVar16 = (int *)((longlong)*(int **)(lVar4 + 0x10) - (longlong)piVar16);
  bVar19 = piVar16 == (int *)0x0;
  if (bVar17) {
    piVar16 = piVar6;
  }
  if (bVar17) {
    lVar13 = 8;
  }
  piVar14 = (int *)(lVar13 + -0x28 + (longlong)piVar16 * 0x28);
  if (bVar17 || bVar19) {
    piVar14 = piVar6;
  }
  bVar18 = *(int **)(lVar4 + 0x30) < piVar12;
  piVar12 = (int *)((longlong)*(int **)(lVar4 + 0x30) - (longlong)piVar12);
  bVar20 = piVar12 == (int *)0x0;
  if (bVar18) {
    lVar8 = 8;
    piVar12 = piVar6;
  }
  piVar10 = (int *)(lVar8 + -0x28 + (longlong)piVar12 * 0x28);
  if (bVar18 || bVar20) {
    piVar10 = piVar14;
  }
  piVar16 = (int *)((longlong)piVar12 + (longlong)piVar16);
  if (!bVar17 && !bVar19 || !bVar18 && !bVar20) {
    piVar16 = piVar6;
  }
  *(ulonglong *)*in_R9 = *(longlong *)(lVar4 + 0x40) - (longlong)piVar16;
  if ((!bVar17 && !bVar19 || !bVar18 && !bVar20) && (*piVar10 == 0xd)) {
    iVar1 = piVar10[4];
    iVar11 = 0;
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
      local_d0 = piVar10[7];
      local_d8 = *(undefined8 *)(piVar10 + 5);
      iVar11 = iVar1;
      break;
    case 0xd:
    case 0xe:
      piVar16 = (int *)*in_stack_00000030;
      iVar11 = *piVar16;
      local_d0 = piVar16[3];
      local_d8 = *(undefined8 *)(piVar16 + 1);
      break;
    case 0x14:
      break;
    default:
      iVar11 = 10;
      if (iVar1 != 5) {
        iVar11 = 0;
      }
    }
    *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
    piVar16 = (int *)*in_stack_00000030;
    *piVar16 = iVar11;
    *(undefined8 *)(piVar16 + 1) = local_d8;
    piVar16[3] = local_d0;
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
  if ((*(byte *)*in_stack_00000060 & 1) != 0) {
    uVar5 = *(undefined4 *)((byte *)*in_stack_00000060 + 4);
    *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
    *(undefined4 *)(*in_R8 + 0x28) = uVar5;
    return;
  }
  puVar3 = (uint *)*in_stack_00000028;
  lVar4 = *in_stack_00000048;
  if ((*puVar3 & 1) == 0) {
    if ((*(uint *)(lVar4 + 0x80) < 3) || (fVar24 = 0.0, *(uint *)(lVar4 + 0x88) < 3)) {
      fVar24 = (float)puVar3[1] + *(float *)(*in_stack_00000050 + 0x28);
    }
    uVar21 = 0x3f800000;
    fVar23 = 1.0;
    if (fVar24 <= 0.2) {
      fVar23 = fVar24;
    }
    *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
    *puVar3 = (uint)(0.2 < fVar24);
    puVar3[1] = (uint)fVar23;
    lVar4 = *in_R8;
    if (fVar24 <= 0.2) {
      uVar5 = *(undefined4 *)((longlong)in_R8 + 0x1c);
      puVar7 = (undefined4 *)in_R8[2];
LAB_140a7cafc:
      lVar8 = (*(longlong *)(*in_RDX + 0x18) + 1) - (ulonglong)(*(longlong *)(lVar4 + 0x18) == 0);
      fVar22 = (float)(ulonglong)(lVar8 * lVar8) * 0.5;
      fVar24 = *(float *)(lVar4 + 0x28);
      fVar23 = fVar22;
      if (fVar22 <= fVar24) {
        fVar23 = fVar24;
      }
      *puVar7 = uVar5;
      *(uint *)(lVar4 + 0x28) =
           ~-(uint)NAN(fVar24) & (uint)fVar23 | -(uint)NAN(fVar24) & (uint)fVar22;
      return;
    }
    goto LAB_140a7ca1e;
  }
  if (*(uint *)(lVar4 + 0x80) < 3) {
    *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
    lVar4 = *in_stack_00000040;
    lVar8 = *(longlong *)(lVar4 + 0x30);
    uVar15 = *(undefined8 *)(lVar4 + 0x40);
    uVar9 = 0x3000000;
    uVar5 = 0x3000000;
    if (lVar8 == *(longlong *)(lVar4 + 0x20)) {
LAB_140a7c95a:
      uVar9 = uVar5;
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                ();
    }
  }
  else {
    if (2 < *(uint *)(lVar4 + 0x88)) {
      *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
      lVar4 = *in_stack_00000040;
      lVar8 = *(longlong *)(lVar4 + 0x30);
      uVar15 = *(undefined8 *)(lVar4 + 0x40);
      if (lVar8 == *(longlong *)(lVar4 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar13 = *(longlong *)(lVar4 + 0x28);
      *(undefined8 *)(lVar13 + lVar8 * 0x10) = uVar15;
      *(undefined4 *)(lVar13 + 8 + lVar8 * 0x10) = 0x3000200;
      *(longlong *)(lVar4 + 0x30) = lVar8 + 1;
      *(longlong *)(lVar4 + 0x40) = *(longlong *)(lVar4 + 0x40) + 1;
      uVar5 = *(undefined4 *)((longlong)in_R8 + 0x1c);
      puVar7 = (undefined4 *)in_R8[2];
      *puVar7 = uVar5;
      lVar4 = *in_R8;
      *(undefined4 *)(lVar4 + 0x28) = 0x3f800000;
      *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
      puVar3[0] = 0;
      puVar3[1] = 0;
      goto LAB_140a7cafc;
    }
    *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
    lVar4 = *in_stack_00000040;
    lVar8 = *(longlong *)(lVar4 + 0x30);
    uVar15 = *(undefined8 *)(lVar4 + 0x40);
    uVar9 = 0x3000100;
    uVar5 = 0x3000100;
    if (lVar8 == *(longlong *)(lVar4 + 0x20)) goto LAB_140a7c95a;
  }
  lVar13 = *(longlong *)(lVar4 + 0x28);
  *(undefined8 *)(lVar13 + lVar8 * 0x10) = uVar15;
  *(undefined4 *)(lVar13 + 8 + lVar8 * 0x10) = uVar9;
  *(longlong *)(lVar4 + 0x30) = lVar8 + 1;
  *(longlong *)(lVar4 + 0x40) = *(longlong *)(lVar4 + 0x40) + 1;
  ::utils::time_continuous_lerp();
  *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  *puVar3 = 1;
  puVar3[1] = extraout_XMM0_Da;
  lVar4 = *in_R8;
  uVar21 = extraout_XMM0_Da;
LAB_140a7ca1e:
  *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
  *(uint *)(lVar4 + 0x28) = uVar21;
  return;
}

