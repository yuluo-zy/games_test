
void __cdecl system_onboarding::reset_hints::reset_hints(void)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  ulonglong *puVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  undefined8 *puVar7;
  undefined4 *puVar8;
  undefined2 *puVar9;
  undefined4 *puVar10;
  undefined1 *puVar11;
  undefined4 *puVar12;
  undefined8 uVar13;
  ulonglong uVar14;
  undefined8 *in_RCX;
  ulonglong uVar15;
  undefined8 *in_RDX;
  undefined4 uVar16;
  undefined8 *in_R8;
  ulonglong uVar17;
  longlong lVar18;
  undefined8 *in_R9;
  ulonglong uVar19;
  longlong lVar20;
  ulonglong uVar21;
  longlong lVar22;
  undefined8 *in_stack_00000028;
  undefined8 *in_stack_00000030;
  undefined8 *in_stack_00000038;
  undefined8 *in_stack_00000040;
  undefined8 *in_stack_00000048;
  undefined8 *in_stack_00000050;
  undefined8 *in_stack_00000058;
  longlong *in_stack_00000060;
  longlong *in_stack_00000068;
  undefined8 *in_stack_00000070;
  
  puVar4 = (ulonglong *)*in_RCX;
  lVar22 = in_RCX[1];
  uVar5 = *puVar4;
  uVar14 = 0;
  uVar15 = uVar5 - *(ulonglong *)(lVar22 + 0x18);
  if (uVar5 < *(ulonglong *)(lVar22 + 0x18)) {
    uVar15 = uVar14;
  }
  uVar19 = uVar5 - *(ulonglong *)(lVar22 + 0x38);
  if (uVar5 < *(ulonglong *)(lVar22 + 0x38)) {
    uVar19 = uVar14;
  }
  uVar5 = *(ulonglong *)(lVar22 + 0x10);
  uVar6 = *(ulonglong *)(lVar22 + 0x30);
  uVar21 = uVar6 - uVar19;
  if (uVar6 < uVar19) {
    uVar21 = uVar14;
  }
  uVar17 = uVar5 - uVar15;
  if (uVar5 < uVar15) {
    uVar17 = uVar14;
  }
  uVar14 = *(ulonglong *)(lVar22 + 0x40);
  puVar7 = (undefined8 *)*in_RDX;
  puVar8 = (undefined4 *)in_RDX[2];
  uVar1 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  puVar9 = (undefined2 *)in_RDX[4];
  puVar10 = (undefined4 *)in_RDX[6];
  uVar2 = *(undefined4 *)((longlong)in_RDX + 0x3c);
  puVar11 = (undefined1 *)in_RDX[8];
  puVar12 = (undefined4 *)in_RDX[10];
  uVar3 = *(undefined4 *)((longlong)in_RDX + 0x5c);
  lVar20 = *(longlong *)(lVar22 + 8);
  lVar22 = *(longlong *)(lVar22 + 0x28);
  *puVar4 = (uVar14 - uVar21) - uVar17;
  if (uVar5 >= uVar15 && uVar5 - uVar15 != 0 || uVar6 >= uVar19 && uVar6 - uVar19 != 0) {
    lVar22 = lVar22 + uVar19 * 0x10;
    lVar20 = lVar20 + uVar15 * 0x10;
    uVar16 = 0;
    if (uVar5 < uVar15) {
      lVar20 = 8;
    }
    lVar18 = 0;
    if (uVar15 < uVar5) {
      lVar18 = lVar20 + -0x10 + uVar17 * 0x10;
    }
    if (uVar6 < uVar19) {
      lVar22 = 8;
    }
    lVar22 = lVar22 + -0x10 + uVar21 * 0x10;
    if (uVar6 <= uVar19) {
      lVar22 = lVar18;
    }
    *puVar4 = uVar14;
    if (*(char *)(lVar22 + 8) == '\0') {
      *(undefined4 *)in_stack_00000060[2] = *(undefined4 *)((longlong)in_stack_00000060 + 0x1c);
      lVar22 = *in_stack_00000060;
      *(undefined8 *)(lVar22 + 500) = 0x101010101010101;
      *(undefined8 *)(lVar22 + 0x1fc) = 0x101010101010101;
      *(undefined4 *)(lVar22 + 0x203) = 0x1010101;
      uVar16 = 2;
    }
    else {
      *puVar8 = uVar1;
      *puVar7 = 0;
      *(undefined1 *)(puVar7 + 1) = 0;
      *puVar10 = uVar2;
      *puVar9 = 0;
      *puVar12 = uVar3;
      *puVar11 = 0;
      *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
      puVar7 = (undefined8 *)*in_R8;
      *puVar7 = 0;
      *(undefined2 *)(puVar7 + 1) = 0;
      *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
      *(undefined1 *)*in_R9 = 0;
      *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
      puVar7 = (undefined8 *)*in_stack_00000028;
      *puVar7 = 0;
      *(undefined2 *)(puVar7 + 1) = 0;
      *(undefined4 *)in_stack_00000050[2] = *(undefined4 *)((longlong)in_stack_00000050 + 0x1c);
      puVar7 = (undefined8 *)*in_stack_00000050;
      *puVar7 = 0;
      puVar7[1] = 0;
      puVar7[2] = 0;
      builtin_strncpy((char *)(puVar7 + 3),"333?333?",8);
      *(undefined4 *)(puVar7 + 4) = 0x42700000;
      *(undefined4 *)in_stack_00000058[2] = *(undefined4 *)((longlong)in_stack_00000058 + 0x1c);
      puVar7 = (undefined8 *)*in_stack_00000058;
      *puVar7 = 0;
      puVar7[1] = 0;
      *(undefined4 *)(puVar7 + 2) = 0;
      *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
      puVar8 = (undefined4 *)*in_stack_00000030;
      *puVar8 = 0;
      *(undefined1 *)(puVar8 + 1) = 0;
      *(undefined4 *)in_stack_00000038[2] = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
      *(undefined2 *)*in_stack_00000038 = 0;
      *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
      *(undefined2 *)*in_stack_00000040 = 0;
      *(undefined4 *)in_stack_00000048[2] = *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
      puVar7 = (undefined8 *)*in_stack_00000048;
      *puVar7 = 0;
      *(undefined4 *)(puVar7 + 1) = 0;
      uVar1 = *(undefined4 *)((longlong)in_stack_00000060 + 0x1c);
      lVar22 = *in_stack_00000060;
      puVar8 = (undefined4 *)in_stack_00000060[2];
      *(undefined8 *)(lVar22 + 500) = 0;
      *(undefined8 *)(lVar22 + 0x1fc) = 0;
      *(undefined4 *)(lVar22 + 0x203) = 0;
      *(undefined8 *)(lVar22 + 0x1e0) = 0;
      *(undefined8 *)(lVar22 + 0x1e8) = 0;
      *puVar8 = uVar1;
      *(undefined1 *)(lVar22 + 0x206) = 0;
    }
    *(undefined4 *)in_stack_00000070[2] = *(undefined4 *)((longlong)in_stack_00000070 + 0x1c);
    *(undefined4 *)*in_stack_00000070 = uVar16;
    *(undefined4 *)in_stack_00000068[2] = *(undefined4 *)((longlong)in_stack_00000068 + 0x1c);
    lVar22 = *in_stack_00000068;
    lVar20 = *(longlong *)(lVar22 + 0x30);
    uVar13 = *(undefined8 *)(lVar22 + 0x40);
    if (lVar20 == *(longlong *)(lVar22 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                ();
    }
    *(undefined8 *)(*(longlong *)(lVar22 + 0x28) + lVar20 * 8) = uVar13;
    *(longlong *)(lVar22 + 0x30) = lVar20 + 1;
    *(longlong *)(lVar22 + 0x40) = *(longlong *)(lVar22 + 0x40) + 1;
  }
  return;
}

