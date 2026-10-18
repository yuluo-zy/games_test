// 0x14227c910 sig void __cdecl hint_camera_display_condition(void)

// WARNING: Type propagation algorithm not settling

void __cdecl system_onboarding::hint_camera_controls::hint_camera_display_condition(void)

{
  undefined8 *puVar1;
  int iVar2;
  int *piVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  char cVar7;
  undefined4 uVar8;
  char extraout_AL;
  char extraout_AL_00;
  char cVar9;
  ulonglong uVar10;
  undefined8 *in_RCX;
  longlong lVar11;
  float *pfVar12;
  longlong *in_RDX;
  longlong lVar13;
  longlong lVar14;
  longlong *in_R8;
  undefined8 uVar15;
  undefined8 *in_R9;
  longlong lVar16;
  code *pcVar17;
  float *pfVar18;
  bool bVar19;
  bool bVar20;
  float fVar21;
  float fVar22;
  longlong *in_stack_00000028;
  undefined8 *in_stack_00000038;
  float *in_stack_00000040;
  longlong *in_stack_00000048;
  longlong *in_stack_00000050;
  undefined8 local_1d4;
  undefined8 uStack_1cc;
  undefined4 uStack_1c4;
  char local_190 [8];
  code *local_188;
  undefined1 local_180;
  code *local_178;
  undefined1 local_170;
  code *local_168;
  longlong local_160;
  undefined8 local_158;
  undefined4 local_150;
  undefined4 uStack_14c;
  undefined4 uStack_148;
  undefined4 uStack_144;
  undefined4 local_140;
  undefined4 uStack_13c;
  undefined4 uStack_138;
  undefined4 uStack_134;
  undefined4 local_130;
  undefined4 uStack_12c;
  undefined4 uStack_128;
  undefined4 uStack_124;
  undefined4 uStack_120;
  undefined4 local_11c;
  undefined4 uStack_118;
  undefined4 uStack_114;
  undefined4 uStack_110;
  undefined4 local_10c;
  undefined1 local_108;
  char local_107;
  undefined6 uStack_106;
  undefined8 local_100;
  undefined4 *local_f8;
  longlong local_f0;
  longlong local_e8;
  undefined4 *local_e0;
  undefined8 local_d8;
  longlong local_d0;
  float *local_c8;
  longlong local_c0;
  undefined4 local_b8;
  undefined4 local_b4;
  float *local_b0;
  longlong local_a8;
  float *local_a0;
  undefined8 local_98;
  undefined8 local_90;
  
  local_90 = 0xfffffffffffffffe;
  piVar3 = (int *)*in_R9;
  if (*piVar3 != 1) {
    return;
  }
  local_a8 = *in_RDX;
  if (((*(char *)(local_a8 + 0x1f6) == '\x01') && (*(char *)(local_a8 + 0x1f7) != '\0')) &&
     (*(char *)(local_a8 + 0x1f8) == '\x01')) {
    *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
    *piVar3 = 2;
  }
  piVar3 = (int *)*in_RCX;
  iVar2 = *piVar3;
  bVar19 = true;
  if (iVar2 != 6 || (char)piVar3[1] != '\x01') {
    if (iVar2 == 6) {
      bVar19 = (char)piVar3[1] == '\x02';
    }
    else {
      bVar20 = true;
      bVar19 = false;
      if (iVar2 == 2) {
        country_core::resources::walls::inner_wall_state::StructureType::is_fence();
        cVar9 = extraout_AL;
        goto joined_r0x00014227ca10;
      }
    }
  }
  bVar20 = iVar2 == 1;
  country_core::resources::walls::inner_wall_state::StructureType::is_fence();
  cVar9 = extraout_AL_00;
joined_r0x00014227ca10:
  if (cVar9 == '\0') {
    if ((bool)(bVar19 | bVar20)) {
      return;
    }
  }
  else if ((bool)(bVar19 | bVar20 | *piVar3 == 10)) {
    return;
  }
  local_190[0] = '\n';
  local_188 = camera_pan_hint;
  local_180 = 0xb;
  local_178 = camera_rotate_hint;
  local_170 = 0xc;
  local_168 = camera_zoom_hint;
  local_158 = 3;
  lVar4 = *in_stack_00000028;
  lVar5 = in_stack_00000028[1];
  pfVar18 = (float *)*in_stack_00000038;
  local_e8 = *in_stack_00000050;
  local_98 = (code *)CONCAT44(local_98._4_4_,(uint)*(byte *)(in_stack_00000040 + 1));
  fVar22 = *in_stack_00000040;
  local_a0 = pfVar18 + 2;
  local_b4 = *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
  local_c0 = *in_stack_00000048;
  local_e0 = (undefined4 *)in_stack_00000048[2];
  local_b8 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  lVar6 = *in_R8;
  local_f8 = (undefined4 *)in_R8[2];
  local_b0 = pfVar18 + 1;
  local_d0 = local_c0 + 0x20;
  local_f0 = lVar6 + 0x20;
  lVar13 = 0;
  local_c8 = pfVar18;
  do {
    lVar14 = lVar13 + 1;
    local_160 = lVar14;
    cVar9 = local_190[lVar13 * 0x10];
    pcVar17 = (&local_188)[lVar13 * 2];
    if (*(char *)(lVar4 + 0x108) == '\0') {
      lVar13 = 0;
      do {
        if (*(longlong *)(lVar4 + 0x100) << 2 == lVar13) goto LAB_14227ce70;
        lVar16 = (ulonglong)*(uint *)(*(longlong *)(lVar4 + 0xf8) + lVar13) * 0xa0;
        lVar13 = lVar13 + 4;
      } while (*(longlong *)(*(longlong *)(lVar5 + 0x100) + 0x58 + lVar16) == 0);
      lVar16 = *(longlong *)(lVar5 + 0x100) + lVar16;
      uVar10 = (ulonglong)*(uint *)(lVar16 + 0x94);
      if (*(ulonglong *)(lVar4 + 0x110) <
          *(ulonglong *)(*(longlong *)(lVar5 + 0x1a8) + 0x40 + uVar10 * 0x48)) {
        lVar13 = *(longlong *)(lVar5 + 0x1a8) + uVar10 * 0x48;
        uVar10 = *(ulonglong *)(*(longlong *)(lVar13 + 0x38) + *(ulonglong *)(lVar4 + 0x110) * 8);
        if (uVar10 == 0) goto LAB_14227cced;
        lVar13 = *(longlong *)(*(longlong *)(lVar13 + 0x18) + 0x10 + ~uVar10 * 0x30);
      }
      else {
LAB_14227cced:
        lVar13 = 0;
      }
      if (*(char *)((ulonglong)*(uint *)(*(longlong *)(lVar16 + 0x50) + 8) + lVar13) == cVar9)
      goto LAB_14227cd02;
LAB_14227ce70:
      bVar19 = false;
    }
    else {
      lVar13 = 0;
      do {
        if (*(longlong *)(lVar4 + 0x100) << 2 == lVar13) goto LAB_14227ce70;
        uVar10 = (ulonglong)*(uint *)(*(longlong *)(lVar4 + 0xf8) + lVar13);
        lVar13 = lVar13 + 4;
      } while (*(longlong *)(*(longlong *)(lVar5 + 0x1a8) + 0x10 + uVar10 * 0x48) == 0);
      lVar13 = *(longlong *)(lVar5 + 0x1a8) + uVar10 * 0x48;
      cVar7 = cRam0000000000000000;
      if ((*(ulonglong *)(lVar4 + 0x110) < *(ulonglong *)(lVar13 + 0x40)) &&
         (uVar10 = *(ulonglong *)(*(longlong *)(lVar13 + 0x38) + *(ulonglong *)(lVar4 + 0x110) * 8),
         uVar10 != 0)) {
        cVar7 = **(char **)(*(longlong *)(lVar13 + 0x18) + 0x10 + ~uVar10 * 0x30);
      }
      if (cVar7 != cVar9) goto LAB_14227ce70;
LAB_14227cd02:
      uVar10 = (ulonglong)local_98 & 0xffffffff;
      if (cVar9 == '\n') {
        if ((1.0 < pfVar18[3] || pfVar18[3] == 1.0) || (fVar21 = *pfVar18, fVar21 < 1.0))
        goto LAB_14227cda4;
LAB_14227cd66:
        if ((fVar21 < 1.0) || (*local_b0 < 1.0)) {
          uVar10 = 0;
        }
        else {
          uVar10 = (ulonglong)CONCAT31((int3)((ulonglong)local_98 >> 8),1.0 <= *local_a0);
        }
        *in_stack_00000040 = 0.4;
        *(char *)(in_stack_00000040 + 1) = (char)uVar10;
        fVar22 = 0.4;
      }
      else if (cVar9 == '\v') {
        if (pfVar18[4] <= 1.0 && pfVar18[4] != 1.0) {
          fVar21 = *local_b0;
          goto joined_r0x00014227cd4c;
        }
      }
      else {
        if (cVar9 != '\f') {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
        if (pfVar18[5] <= 1.0 && pfVar18[5] != 1.0) {
          fVar21 = *local_a0;
joined_r0x00014227cd4c:
          if (1.0 <= fVar21) {
            fVar21 = *pfVar18;
            goto LAB_14227cd66;
          }
        }
      }
LAB_14227cda4:
      fVar22 = fVar22 - *(float *)(local_e8 + 0x28);
      *in_stack_00000040 = fVar22;
      bVar19 = 0.0 < fVar22;
      if ((fVar22 <= 0.0) && ((uVar10 & 1) != 0)) {
        *local_e0 = local_b4;
        lVar13 = *(longlong *)(local_c0 + 0x30);
        uVar15 = *(undefined8 *)(local_c0 + 0x40);
        if (lVar13 == *(longlong *)(local_c0 + 0x20)) {
          local_d8 = *(undefined8 *)(local_c0 + 0x40);
          local_98 = pcVar17;
          alloc::raw_vec::RawVec::
          grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                    ();
          uVar15 = local_d8;
          pcVar17 = local_98;
          pfVar18 = local_c8;
        }
        lVar16 = *(longlong *)(local_c0 + 0x28);
        *(undefined4 *)(lVar16 + lVar13 * 0x20) = 0x13;
        *(undefined8 *)(lVar16 + 0x18 + lVar13 * 0x20) = uVar15;
        *(longlong *)(local_c0 + 0x30) = lVar13 + 1;
        *(longlong *)(local_c0 + 0x40) = *(longlong *)(local_c0 + 0x40) + 1;
        *(undefined1 *)(in_stack_00000040 + 1) = 0;
        local_98 = (code *)((ulonglong)local_98 & 0xffffffff00000000);
        goto LAB_14227ce70;
      }
      local_98 = (code *)CONCAT44(local_98._4_4_,(int)uVar10);
    }
    pfVar12 = pfVar18;
    if (((cVar9 != '\n') && (pfVar12 = local_b0, cVar9 != '\v')) &&
       (pfVar12 = local_a0, cVar9 != '\f')) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    if (bVar19 || *pfVar12 <= 1.0 && *pfVar12 != 1.0) {
      (*pcVar17)(&local_150,local_a8);
      uVar8 = local_150;
      *local_f8 = local_b8;
      local_100 = *(undefined8 *)(lVar6 + 0x40);
      local_150 = 6;
      local_10c = uStack_124;
      local_11c = uStack_134;
      uStack_118 = local_130;
      uStack_114 = uStack_12c;
      uStack_110 = uStack_128;
      uStack_12c = uStack_144;
      uStack_128 = local_140;
      uStack_124 = uStack_13c;
      uStack_120 = uStack_138;
      uStack_13c = uStack_1c4;
      uStack_138 = uVar8;
      uStack_134 = uStack_14c;
      local_130 = uStack_148;
      uStack_14c = (undefined4)local_1d4;
      uStack_148 = (undefined4)((ulonglong)local_1d4 >> 0x20);
      uStack_144 = (undefined4)uStack_1cc;
      local_140 = (undefined4)((ulonglong)uStack_1cc >> 0x20);
      local_108 = 1;
      lVar13 = *(longlong *)(lVar6 + 0x30);
      local_107 = cVar9;
      if (lVar13 == *(longlong *)(lVar6 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer>,alloc::alloc::Global>
                  ();
      }
      lVar16 = *(longlong *)(lVar6 + 0x28);
      lVar11 = lVar13 * 0x58;
      *(undefined8 *)(lVar16 + 0x50 + lVar11) = local_100;
      puVar1 = (undefined8 *)(lVar16 + 0x40 + lVar11);
      *puVar1 = CONCAT44(local_10c,uStack_110);
      puVar1[1] = CONCAT62(uStack_106,CONCAT11(local_107,local_108));
      puVar1 = (undefined8 *)(lVar16 + 0x30 + lVar11);
      *puVar1 = CONCAT44(local_11c,uStack_120);
      puVar1[1] = CONCAT44(uStack_114,uStack_118);
      puVar1 = (undefined8 *)(lVar16 + 0x20 + lVar11);
      *puVar1 = CONCAT44(uStack_12c,local_130);
      puVar1[1] = CONCAT44(uStack_124,uStack_128);
      puVar1 = (undefined8 *)(lVar16 + 0x10 + lVar11);
      *puVar1 = CONCAT44(uStack_13c,local_140);
      puVar1[1] = CONCAT44(uStack_134,uStack_138);
      *(undefined8 *)(lVar16 + lVar11) = CONCAT44(uStack_14c,local_150);
      ((undefined8 *)(lVar16 + lVar11))[1] = CONCAT44(uStack_144,uStack_148);
      *(longlong *)(lVar6 + 0x30) = lVar13 + 1;
      *(longlong *)(lVar6 + 0x40) = *(longlong *)(lVar6 + 0x40) + 1;
      pfVar18 = local_c8;
    }
    lVar13 = lVar14;
    if (lVar14 == 3) {
      return;
    }
  } while( true );
}

