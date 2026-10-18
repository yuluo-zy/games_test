// 0x142284220 sig void __cdecl hint_undo(void)

void __cdecl system_onboarding::hint_undo::hint_undo(void)

{
  undefined8 *puVar1;
  undefined4 uVar2;
  longlong lVar3;
  undefined8 uVar4;
  ulonglong *puVar5;
  ulonglong uVar6;
  char cVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  char extraout_AL;
  char extraout_AL_00;
  undefined2 extraout_AX;
  longlong *in_RCX;
  longlong lVar10;
  longlong lVar11;
  undefined8 *in_RDX;
  ulonglong uVar12;
  ulonglong uVar13;
  longlong *in_R8;
  ulonglong uVar14;
  longlong *in_R9;
  ulonglong uVar15;
  longlong lVar16;
  ulonglong uVar17;
  ulonglong uVar18;
  float fVar19;
  longlong *in_stack_00000028;
  longlong *in_stack_00000038;
  longlong *in_stack_00000040;
  undefined2 uStack_102;
  undefined8 uStack_100;
  undefined4 uStack_f8;
  undefined4 uStack_f0;
  undefined2 uStack_ac;
  undefined2 uStack_aa;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 local_a0;
  undefined4 uStack_9c;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 local_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined6 uStack_66;
  
  uVar9 = local_a0;
  country_core::input::action_set::DigitalActionState::just_pressed();
  if (extraout_AL != '\0') {
    *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
    lVar10 = *in_R8;
    lVar3 = *(longlong *)(lVar10 + 0x30);
    uVar4 = *(undefined8 *)(lVar10 + 0x40);
    if (lVar3 == *(longlong *)(lVar10 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                ();
    }
    lVar16 = *(longlong *)(lVar10 + 0x28);
    *(undefined8 *)(lVar16 + lVar3 * 0x10) = uVar4;
    *(undefined1 *)(lVar16 + 8 + lVar3 * 0x10) = 2;
    *(longlong *)(lVar10 + 0x30) = lVar3 + 1;
    *(longlong *)(lVar10 + 0x40) = *(longlong *)(lVar10 + 0x40) + 1;
  }
  puVar5 = (ulonglong *)*in_RDX;
  lVar10 = in_RDX[1];
  uVar12 = *puVar5;
  uVar15 = 0;
  uVar17 = uVar12 - *(ulonglong *)(lVar10 + 0x18);
  if (uVar12 < *(ulonglong *)(lVar10 + 0x18)) {
    uVar17 = uVar15;
  }
  uVar14 = uVar12 - *(ulonglong *)(lVar10 + 0x38);
  if (uVar12 < *(ulonglong *)(lVar10 + 0x38)) {
    uVar14 = uVar15;
  }
  uVar12 = *(ulonglong *)(lVar10 + 0x30);
  uVar18 = uVar12 - uVar14;
  if (uVar12 < uVar14) {
    uVar18 = uVar15;
  }
  uVar6 = *(ulonglong *)(lVar10 + 0x10);
  uVar13 = uVar6 - uVar17;
  if (uVar6 < uVar17) {
    uVar13 = uVar15;
  }
  uVar15 = *(ulonglong *)(lVar10 + 0x40);
  *puVar5 = uVar15 - (uVar13 + uVar18);
  if (uVar6 >= uVar17 && uVar6 - uVar17 != 0 || uVar12 >= uVar14 && uVar12 - uVar14 != 0) {
    *puVar5 = uVar15;
    *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
    *(undefined1 *)(*in_stack_00000028 + 4) = 1;
  }
  lVar10 = *in_stack_00000038;
  lVar3 = in_stack_00000038[1];
  if (*(char *)(lVar10 + 0x108) != '\0') {
    lVar16 = 0;
    do {
      if (*(longlong *)(lVar10 + 0x100) << 2 == lVar16) goto LAB_142284589;
      uVar12 = (ulonglong)*(uint *)(*(longlong *)(lVar10 + 0xf8) + lVar16);
      lVar16 = lVar16 + 4;
    } while (*(longlong *)(*(longlong *)(lVar3 + 0x1a8) + 0x10 + uVar12 * 0x48) == 0);
    lVar3 = *(longlong *)(lVar3 + 0x1a8) + uVar12 * 0x48;
    cVar7 = cRam0000000000000000;
    if ((*(ulonglong *)(lVar10 + 0x110) < *(ulonglong *)(lVar3 + 0x40)) &&
       (uVar12 = *(ulonglong *)(*(longlong *)(lVar3 + 0x38) + *(ulonglong *)(lVar10 + 0x110) * 8),
       uVar12 != 0)) {
      cVar7 = **(char **)(*(longlong *)(lVar3 + 0x18) + 0x10 + ~uVar12 * 0x30);
    }
    goto joined_r0x00014228448e;
  }
  lVar16 = 0;
  do {
    if (*(longlong *)(lVar10 + 0x100) << 2 == lVar16) goto LAB_142284589;
    lVar11 = (ulonglong)*(uint *)(*(longlong *)(lVar10 + 0xf8) + lVar16) * 0xa0;
    lVar16 = lVar16 + 4;
  } while (*(longlong *)(*(longlong *)(lVar3 + 0x100) + 0x58 + lVar11) == 0);
  lVar11 = *(longlong *)(lVar3 + 0x100) + lVar11;
  uVar12 = (ulonglong)*(uint *)(lVar11 + 0x94);
  if (*(ulonglong *)(lVar10 + 0x110) <
      *(ulonglong *)(*(longlong *)(lVar3 + 0x1a8) + 0x40 + uVar12 * 0x48)) {
    lVar3 = *(longlong *)(lVar3 + 0x1a8) + uVar12 * 0x48;
    uVar12 = *(ulonglong *)(*(longlong *)(lVar3 + 0x38) + *(ulonglong *)(lVar10 + 0x110) * 8);
    if (uVar12 == 0) goto LAB_14228447f;
    lVar10 = *(longlong *)(*(longlong *)(lVar3 + 0x18) + 0x10 + ~uVar12 * 0x30);
  }
  else {
LAB_14228447f:
    lVar10 = 0;
  }
  cVar7 = *(char *)((ulonglong)*(uint *)(*(longlong *)(lVar11 + 0x50) + 8) + lVar10);
joined_r0x00014228448e:
  if (cVar7 == '\x02') {
    fVar19 = *(float *)(*in_stack_00000040 + 0x28);
    *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
    fVar19 = fVar19 + *(float *)*in_stack_00000028;
    *(float *)*in_stack_00000028 = fVar19;
    if (10.0 < fVar19) {
      *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
      lVar10 = *in_R8;
      lVar3 = *(longlong *)(lVar10 + 0x30);
      uVar4 = *(undefined8 *)(lVar10 + 0x40);
      if (lVar3 == *(longlong *)(lVar10 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar16 = *(longlong *)(lVar10 + 0x28);
      *(undefined8 *)(lVar16 + lVar3 * 0x10) = uVar4;
      *(undefined1 *)(lVar16 + 8 + lVar3 * 0x10) = 2;
      *(longlong *)(lVar10 + 0x30) = lVar3 + 1;
      *(longlong *)(lVar10 + 0x40) = *(longlong *)(lVar10 + 0x40) + 1;
    }
    country_core::input::action_set::DigitalActionState::just_pressed();
    if (extraout_AL_00 != '\0') {
      *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
      lVar10 = *in_R8;
      lVar3 = *(longlong *)(lVar10 + 0x30);
      uVar4 = *(undefined8 *)(lVar10 + 0x40);
      if (lVar3 == *(longlong *)(lVar10 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar16 = *(longlong *)(lVar10 + 0x28);
      *(undefined8 *)(lVar16 + lVar3 * 0x10) = uVar4;
      *(undefined1 *)(lVar16 + 8 + lVar3 * 0x10) = 2;
      *(longlong *)(lVar10 + 0x30) = lVar3 + 1;
      *(longlong *)(lVar10 + 0x40) = *(longlong *)(lVar10 + 0x40) + 1;
    }
  }
LAB_142284589:
  if ((*(char *)(*in_stack_00000028 + 4) == '\x01') &&
     (lVar10 = *in_RCX, *(char *)(lVar10 + 0x1fe) != '\x01')) {
    country_core::resources::steam::impl$31::from();
    country_core::resources::steam::impl$31::from();
    hint_orchestrator::BindingHintDisplay::
    adjacent<enum2_<system_onboarding::hint_orchestrator::BindingHintDisplay>_>();
    uVar2 = *(undefined4 *)(lVar10 + 0x104);
    uVar4 = *(undefined8 *)(lVar10 + 0xfc);
    uStack_ac = (undefined2)uVar4;
    uStack_aa = (undefined2)((ulonglong)uVar4 >> 0x10);
    uStack_a8 = (undefined4)((ulonglong)uVar4 >> 0x20);
    uVar8 = uStack_a8;
    hint_orchestrator::BindingHintDisplay::
    adjacent<enum2_<core::option::Option<enum2_<bevy_input::keyboard::KeyCode>_>_>_>();
    hint_orchestrator::BindingHintDisplay::
    alternate<enum2_<system_onboarding::hint_orchestrator::BindingHintDisplay>_>();
    uStack_f0 = (undefined4)(CONCAT26(uStack_aa,CONCAT24(uStack_ac,0xc6)) >> 0x20);
    *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
    lVar10 = *in_R9;
    uVar4 = *(undefined8 *)(lVar10 + 0x40);
    uStack_a8 = (undefined4)(CONCAT26(uStack_102,CONCAT24(extraout_AX,0xc9)) >> 0x20);
    uStack_a4 = (undefined4)uStack_100;
    local_a0 = (undefined4)((ulonglong)uStack_100 >> 0x20);
    lVar3 = *(longlong *)(lVar10 + 0x30);
    if (lVar3 == *(longlong *)(lVar10 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer>,alloc::alloc::Global>
                ();
    }
    lVar16 = *(longlong *)(lVar10 + 0x28);
    lVar11 = lVar3 * 0x58;
    *(undefined8 *)(lVar16 + 0x50 + lVar11) = uVar4;
    puVar1 = (undefined8 *)(lVar16 + 0x40 + lVar11);
    *puVar1 = CONCAT44(uStack_84,uStack_88);
    puVar1[1] = CONCAT62(uStack_66,0x200);
    puVar1 = (undefined8 *)(lVar16 + 0x30 + lVar11);
    *puVar1 = CONCAT44(uStack_94,uStack_98);
    puVar1[1] = CONCAT44(uStack_8c,local_90);
    puVar1 = (undefined8 *)(lVar16 + 0x20 + lVar11);
    *puVar1 = CONCAT44(uVar2,uVar8);
    puVar1[1] = CONCAT44(uStack_9c,uVar9);
    puVar1 = (undefined8 *)(lVar16 + 0x10 + lVar11);
    *puVar1 = CONCAT44(uStack_f8,local_a0);
    puVar1[1] = CONCAT44(uStack_f0,0xc6);
    *(undefined8 *)(lVar16 + lVar11) = 0xc900000006;
    ((undefined8 *)(lVar16 + lVar11))[1] = CONCAT44(uStack_a4,uStack_a8);
    *(longlong *)(lVar10 + 0x30) = lVar3 + 1;
    *(longlong *)(lVar10 + 0x40) = *(longlong *)(lVar10 + 0x40) + 1;
  }
  return;
}

