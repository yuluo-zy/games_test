// 0x1421b48e0 void __cdecl process(void)

void __cdecl system_roof::replay_roof::impl$0::process(void)

{
  undefined8 *puVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  undefined1 uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  longlong lVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  byte *in_RCX;
  longlong lVar19;
  longlong lVar20;
  longlong lVar21;
  longlong *in_R8;
  longlong *in_R9;
  float fVar22;
  float in_XMM1_Da;
  float fVar23;
  float fVar24;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  longlong *in_stack_00000038;
  undefined4 uStack_9c;
  undefined4 uStack_98;
  
  lVar21 = *(longlong *)(in_RCX + 8);
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(*(undefined1 *)(lVar21 + 0x10)) {
  case 0:
    if (in_RCX[0x14] == 0) {
      uVar12 = *(undefined8 *)(lVar21 + 0x40);
      uVar16 = *(undefined8 *)(lVar21 + 0x24);
      uVar17 = *(undefined8 *)(lVar21 + 0x2c);
      uVar18 = *(undefined8 *)(lVar21 + 0x38);
      uStack_98 = (undefined4)((ulonglong)*(undefined8 *)(lVar21 + 0x30) >> 0x20);
      uStack_9c = (undefined4)((ulonglong)uVar17 >> 0x20);
      uVar10 = *(undefined4 *)(lVar21 + 0x14);
      uVar11 = *(undefined4 *)(lVar21 + 0x20);
      uVar13 = *(undefined8 *)(lVar21 + 0x18);
      *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
      lVar21 = *in_R8;
      lVar20 = *(longlong *)(lVar21 + 0x30);
      uVar15 = *(undefined8 *)(lVar21 + 0x40);
      if (lVar20 == *(longlong *)(lVar21 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<country_core::resources::window::AvailableMonitor,alloc::alloc::Global>();
      }
      lVar14 = *(longlong *)(lVar21 + 0x28);
      lVar19 = lVar20 * 0x40;
      *(undefined8 *)(lVar14 + lVar19) = uVar15;
      *(undefined8 *)(lVar14 + 8 + lVar19) = uVar12;
      *(undefined4 *)(lVar14 + 0x10 + lVar19) = uVar10;
      *(undefined8 *)(lVar14 + 0x14 + lVar19) = uVar13;
      *(undefined4 *)(lVar14 + 0x1c + lVar19) = uVar11;
      puVar1 = (undefined8 *)(lVar14 + 0x20 + lVar19);
      *puVar1 = uVar16;
      puVar1[1] = uVar17;
      puVar1 = (undefined8 *)(lVar14 + 0x2c + lVar19);
      *puVar1 = CONCAT44(uStack_98,uStack_9c);
      puVar1[1] = uVar18;
      *(undefined1 *)(lVar14 + 0x3c + lVar19) = 0;
    }
    else {
code_r0x0001421b4d4e:
      uVar12 = *(undefined8 *)(lVar21 + 0x40);
      *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
      lVar21 = *in_R9;
      lVar20 = *(longlong *)(lVar21 + 0x30);
      uVar13 = *(undefined8 *)(lVar21 + 0x40);
      if (lVar20 == *(longlong *)(lVar21 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      lVar14 = *(longlong *)(lVar21 + 0x28);
      *(undefined8 *)(lVar14 + lVar20 * 0x18) = uVar13;
      *(undefined8 *)(lVar14 + 8 + lVar20 * 0x18) = uVar12;
      *(undefined1 *)(lVar14 + 0x10 + lVar20 * 0x18) = 1;
    }
    break;
  case 1:
    if (in_RCX[0x14] == 0) goto code_r0x0001421b4d4e;
    uVar12 = *(undefined8 *)(lVar21 + 0x40);
    uVar16 = *(undefined8 *)(lVar21 + 0x24);
    uVar17 = *(undefined8 *)(lVar21 + 0x2c);
    uVar18 = *(undefined8 *)(lVar21 + 0x38);
    uStack_98 = (undefined4)((ulonglong)*(undefined8 *)(lVar21 + 0x30) >> 0x20);
    uStack_9c = (undefined4)((ulonglong)uVar17 >> 0x20);
    uVar10 = *(undefined4 *)(lVar21 + 0x14);
    uVar11 = *(undefined4 *)(lVar21 + 0x20);
    uVar13 = *(undefined8 *)(lVar21 + 0x18);
    *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
    lVar21 = *in_R8;
    lVar20 = *(longlong *)(lVar21 + 0x30);
    uVar15 = *(undefined8 *)(lVar21 + 0x40);
    if (lVar20 == *(longlong *)(lVar21 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<country_core::resources::window::AvailableMonitor,alloc::alloc::Global>();
    }
    lVar14 = *(longlong *)(lVar21 + 0x28);
    lVar19 = lVar20 * 0x40;
    *(undefined8 *)(lVar14 + lVar19) = uVar15;
    *(undefined8 *)(lVar14 + 8 + lVar19) = uVar12;
    *(undefined4 *)(lVar14 + 0x10 + lVar19) = uVar10;
    *(undefined8 *)(lVar14 + 0x14 + lVar19) = uVar13;
    *(undefined4 *)(lVar14 + 0x1c + lVar19) = uVar11;
    puVar1 = (undefined8 *)(lVar14 + 0x20 + lVar19);
    *puVar1 = uVar16;
    puVar1[1] = uVar17;
    puVar1 = (undefined8 *)(lVar14 + 0x2c + lVar19);
    *puVar1 = CONCAT44(uStack_98,uStack_9c);
    puVar1[1] = uVar18;
    *(undefined1 *)(lVar14 + 0x3c + lVar19) = 1;
    break;
  case 2:
    if ((*in_RCX & 1) != 0) {
      fVar23 = *(float *)(in_RCX + 0x10);
      *(float *)(in_RCX + 0x10) = in_XMM1_Da + fVar23;
      fVar24 = (in_XMM1_Da + fVar23) / *(float *)(in_RCX + 4);
      fVar23 = 0.0;
      if (0.0 <= fVar24) {
        fVar23 = fVar24;
      }
      if (fVar23 < 1.0) goto code_r0x0001421b4aa3;
    }
    in_RCX[0x15] = 1;
    fVar23 = 1.0;
code_r0x0001421b4aa3:
    if (in_RCX[0x14] != 0) {
      fVar23 = 1.0 - fVar23;
    }
    uVar15 = *(undefined8 *)(lVar21 + 0x38);
    fVar22 = 1.0 - fVar23;
    fVar24 = *(float *)(lVar21 + 0x18);
    fVar2 = *(float *)(lVar21 + 0x28);
    fVar3 = *(float *)(lVar21 + 0x14);
    fVar4 = *(float *)(lVar21 + 0x24);
    uVar12 = *(undefined8 *)(lVar21 + 0x40);
    uVar13 = *(undefined8 *)(lVar21 + 0x48);
    fVar5 = *(float *)(lVar21 + 0x1c);
    fVar6 = *(float *)(lVar21 + 0x2c);
    fVar7 = *(float *)(lVar21 + 0x20);
    fVar8 = *(float *)(lVar21 + 0x30);
    *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
    lVar21 = *in_stack_00000028;
    lVar20 = *(longlong *)(lVar21 + 0x30);
    uVar16 = *(undefined8 *)(lVar21 + 0x40);
    if (lVar20 == *(longlong *)(lVar21 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<country_core::resources::window::AvailableMonitor,alloc::alloc::Global>();
    }
    lVar14 = *(longlong *)(lVar21 + 0x28);
    lVar19 = lVar20 * 0x40;
    *(undefined4 *)(lVar14 + lVar19) = 1;
    *(float *)(lVar14 + 4 + lVar19) = fVar2 * fVar23 + fVar24 * fVar22;
    *(undefined4 *)(lVar14 + 8 + lVar19) = 1;
    *(float *)(lVar14 + 0xc + lVar19) = fVar4 * fVar23 + fVar3 * fVar22;
    *(undefined4 *)(lVar14 + 0x10 + lVar19) = 1;
    *(float *)(lVar14 + 0x14 + lVar19) = fVar6 * fVar23 + fVar5 * fVar22;
    *(undefined4 *)(lVar14 + 0x18 + lVar19) = 1;
    *(float *)(lVar14 + 0x1c + lVar19) = fVar23 * fVar8 + fVar22 * fVar7;
    *(undefined8 *)(lVar14 + 0x20 + lVar19) = uVar15;
    *(undefined4 *)(lVar14 + 0x28 + lVar19) = 1;
    *(ulonglong *)(lVar14 + 0x2c + lVar19) =
         CONCAT44(fVar23 * (float)((ulonglong)uVar13 >> 0x20) +
                  fVar22 * (float)((ulonglong)uVar12 >> 0x20),
                  fVar23 * (float)uVar13 + fVar22 * (float)uVar12);
    *(undefined1 *)(lVar14 + 0x34 + lVar19) = 1;
    *(undefined8 *)(lVar14 + 0x38 + lVar19) = uVar16;
    *(longlong *)(lVar21 + 0x30) = lVar20 + 1;
    *(longlong *)(lVar21 + 0x40) = *(longlong *)(lVar21 + 0x40) + 1;
    return;
  case 3:
    fVar23 = *(float *)(in_RCX + 0x10);
    if ((fVar23 == 0.0) && (!NAN(fVar23))) {
      uVar9 = *(undefined1 *)((lVar21 - (ulonglong)in_RCX[0x14]) + 0x12);
      uVar12 = *(undefined8 *)(lVar21 + 0x18);
      *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
      lVar21 = *in_stack_00000030;
      lVar20 = *(longlong *)(lVar21 + 0x30);
      uVar13 = *(undefined8 *)(lVar21 + 0x40);
      if (lVar20 == *(longlong *)(lVar21 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      lVar14 = *(longlong *)(lVar21 + 0x28);
      *(undefined8 *)(lVar14 + lVar20 * 0x18) = uVar13;
      *(undefined8 *)(lVar14 + 8 + lVar20 * 0x18) = uVar12;
      *(undefined1 *)(lVar14 + 0x10 + lVar20 * 0x18) = uVar9;
      *(longlong *)(lVar21 + 0x30) = lVar20 + 1;
code_r0x0001421b4c5b:
      *(longlong *)(lVar21 + 0x40) = *(longlong *)(lVar21 + 0x40) + 1;
    }
    goto code_r0x0001421b4c5f;
  case 4:
    fVar23 = *(float *)(in_RCX + 0x10);
    if ((fVar23 == 0.0) && (!NAN(fVar23))) {
      uVar9 = *(undefined1 *)((lVar21 - (ulonglong)in_RCX[0x14]) + 0x12);
      uVar12 = *(undefined8 *)(lVar21 + 0x18);
      *(undefined4 *)in_stack_00000038[2] = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
      lVar21 = *in_stack_00000038;
      lVar20 = *(longlong *)(lVar21 + 0x30);
      uVar13 = *(undefined8 *)(lVar21 + 0x40);
      if (lVar20 == *(longlong *)(lVar21 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      lVar14 = *(longlong *)(lVar21 + 0x28);
      *(undefined8 *)(lVar14 + lVar20 * 0x18) = uVar13;
      *(undefined8 *)(lVar14 + 8 + lVar20 * 0x18) = uVar12;
      *(undefined1 *)(lVar14 + 0x10 + lVar20 * 0x18) = uVar9;
      *(undefined1 *)(lVar14 + 0x11 + lVar20 * 0x18) = uVar9;
      *(undefined1 *)(lVar14 + 0x12 + lVar20 * 0x18) = 0;
      *(longlong *)(lVar21 + 0x30) = lVar20 + 1;
      goto code_r0x0001421b4c5b;
    }
code_r0x0001421b4c5f:
    if (*(int *)in_RCX == 1) {
      *(float *)(in_RCX + 0x10) = fVar23 + in_XMM1_Da;
      fVar24 = (fVar23 + in_XMM1_Da) / *(float *)(in_RCX + 4);
      fVar23 = 0.0;
      if (0.0 <= fVar24) {
        fVar23 = fVar24;
      }
      fVar24 = 1.0;
      if (fVar23 <= 1.0) {
        fVar24 = fVar23;
      }
      if (fVar24 != 1.0) {
        return;
      }
      if (NAN(fVar24)) {
        return;
      }
    }
    goto code_r0x0001421b4da7;
  }
  *(longlong *)(lVar21 + 0x30) = lVar20 + 1;
  *(longlong *)(lVar21 + 0x40) = *(longlong *)(lVar21 + 0x40) + 1;
code_r0x0001421b4da7:
  in_RCX[0x15] = 1;
  return;
}


