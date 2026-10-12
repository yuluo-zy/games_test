
void __cdecl country_core::systems::wall::eraser::ui_bulldozer::wall_eraser_bulldoze(void)

{
  undefined1 auVar1 [16];
  undefined8 uVar2;
  undefined8 *puVar3;
  undefined8 *puVar4;
  ushort uVar5;
  int iVar6;
  undefined *puVar7;
  ulonglong uVar8;
  uint uVar9;
  uint uVar10;
  undefined *extraout_RAX;
  longlong lVar11;
  longlong lVar12;
  longlong extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  void *extraout_RAX_02;
  longlong lVar13;
  longlong *in_RDX;
  ulonglong uVar14;
  undefined ***pppuVar15;
  undefined1 (*pauVar16) [16];
  void *_Dst;
  ulonglong uVar17;
  longlong lVar18;
  longlong in_R9;
  undefined1 (*pauVar19) [16];
  longlong lVar20;
  float fVar21;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  undefined8 *in_stack_00000038;
  undefined8 *in_stack_00000040;
  undefined8 *in_stack_00000048;
  longlong in_stack_00000050;
  undefined8 *in_stack_00000068;
  undefined8 *local_4e0;
  undefined *local_438;
  undefined8 local_430;
  undefined4 local_414;
  undefined4 local_410;
  undefined4 local_40c;
  undefined8 local_408;
  undefined8 local_400;
  undefined8 *local_3f0;
  undefined8 local_3e8;
  byte local_3e0;
  uint uStack_3dc;
  uint uStack_3d8;
  undefined4 uStack_3d4;
  ulonglong uStack_3d0;
  float local_3c8;
  float fStack_3c4;
  float fStack_3c0;
  undefined8 *local_3b8;
  code *pcStack_3b0;
  undefined8 local_348 [6];
  undefined8 local_318;
  undefined8 uStack_310;
  undefined8 local_308;
  undefined **local_2f8;
  code *local_2f0;
  size_t local_2e8;
  undefined8 local_280;
  undefined ***local_278;
  undefined8 local_270;
  undefined4 local_268;
  uint local_264;
  undefined8 local_260;
  undefined8 local_258;
  undefined8 local_248;
  uint uStack_23c;
  undefined8 local_230;
  longlong local_228;
  undefined8 local_220;
  undefined **local_218;
  undefined *local_210;
  undefined ***local_208;
  ulonglong local_200;
  undefined *local_1f8;
  undefined4 local_1e8;
  undefined4 local_1d0;
  undefined4 local_1b8;
  ushort local_1a0;
  undefined1 local_190;
  undefined4 local_18c;
  undefined8 local_188;
  float local_180;
  undefined2 local_170;
  undefined4 local_118;
  longlong local_110;
  undefined8 local_108;
  uint local_100;
  undefined8 local_f8;
  ulonglong local_e8;
  undefined *local_e0;
  longlong local_d8;
  undefined8 local_d0;
  undefined8 local_c0;
  
  local_c0 = 0xfffffffffffffffe;
  local_108 = *in_stack_00000038;
  local_280 = in_stack_00000038[4];
  uVar2 = *in_stack_00000040;
  local_410 = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
  local_408 = in_stack_00000040[4];
  local_400 = in_stack_00000040[6];
  local_268 = *(undefined4 *)((longlong)in_stack_00000040 + 0x3c);
  local_230 = in_stack_00000040[8];
  local_270 = in_stack_00000040[0xc];
  local_40c = *(undefined4 *)((longlong)in_stack_00000040 + 0x7c);
  puVar3 = (undefined8 *)*in_stack_00000048;
  puVar4 = (undefined8 *)in_stack_00000048[4];
  local_3f0 = (undefined8 *)*in_stack_00000068;
  lVar12 = in_stack_00000068[4];
  local_110 = in_stack_00000068[5];
  local_430 = in_stack_00000068[7];
  local_414 = *(undefined4 *)((longlong)in_stack_00000068 + 0x54);
  local_218 = (undefined **)0x0;
  local_210 = (undefined *)0x8;
  local_208 = (undefined ***)0x0;
  local_200 = local_200 & 0xffffffff00000000;
  local_1e8 = 0;
  local_1d0 = 0;
  local_1b8 = 0;
  local_1a0 = 1;
  local_190 = 5;
  local_18c = 0x7f7fffff;
  local_170 = 0x100;
  local_260 = *(undefined8 *)(in_R9 + 0x40);
  local_3e8 = *(undefined8 *)(in_R9 + 0x60);
  local_248 = *(undefined8 *)(in_stack_00000050 + 0xe0);
  local_220 = *(undefined8 *)(in_stack_00000050 + 0xf0);
  local_118 = *(undefined4 *)(in_stack_00000050 + 0xfc);
  systems::ui::resources::ui_state::UiState::add<ref_<str_>_>();
  local_318 = *puVar4;
  uStack_310 = puVar4[1];
  local_308 = puVar4[2];
  collision::world::Rapier::raycast_simple();
  uVar17 = 0;
  if (local_3e0 == 0x11) {
    if (_DEV_OVERRIDE_IS_DEMO_MODE != '\0') {
      local_d0 = (undefined *)((ulonglong)local_d0._4_4_ << 0x20);
      local_348[0] = *puVar3;
      local_3b8 = local_348;
      pcStack_3b0 = glam::f32::vec2::impl$93::fmt;
      local_218 = &PTR_s_cursor_hammer__142ac4080;
      local_210 = (undefined *)0x1;
      local_208 = (undefined ***)&local_3b8;
      local_200 = 1;
      local_1f8 = (undefined *)0x0;
      alloc::fmt::format::format_inner();
      uVar2 = local_348[0];
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_01 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::alloc::handle_alloc_error();
      }
      *extraout_RAX_01 = 1;
      extraout_RAX_01[1] = 1;
      extraout_RAX_01[2] = uVar2;
      *(char *)(extraout_RAX_01 + 3) = (char)local_d0;
      local_210 = &DAT_142ac4090;
      if ((longlong)local_2e8 < 0) {
LAB_1409e21fe:
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error();
      }
      if (local_2e8 == 0) {
        _Dst = (void *)0x1;
      }
      else {
        std::alloc::__default_lib_allocator::__rust_alloc();
        _Dst = extraout_RAX_02;
        if (extraout_RAX_02 == (void *)0x0) goto LAB_1409e21fe;
      }
      memcpy(_Dst,local_2f0,local_2e8);
      local_218 = (undefined **)local_2e8;
      local_208 = (undefined ***)local_2e8;
      local_1f8 = &DAT_142ac4090;
      local_210 = _Dst;
      append_only_vec::AppendOnlyVec::
      push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>();
      if (local_2f8 != (undefined **)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      return;
    }
    local_100 = 5;
    fVar21 = __real_7f7fffff;
    local_e0 = extraout_RAX;
    local_d0 = extraout_RAX;
    goto LAB_1409de5d1;
  }
  local_100 = 5;
  local_e0 = (undefined *)0x0;
  local_d0 = &DAT_142ac4508;
  puVar7 = extraout_RAX;
  switch((uint)local_3e0) {
  case 0:
  case 1:
  case 2:
  case 6:
  case 7:
  case 8:
  case 9:
    puVar7 = (undefined *)CONCAT44(uStack_3d4,uStack_3d8);
    lVar11 = *(longlong *)(*in_RDX + 0x10) * 0x98;
    local_e0 = (undefined *)0x0;
    lVar20 = *(longlong *)(*in_RDX + 8);
    do {
      if (lVar11 == 0) {
        local_100 = 0;
        local_e0 = puVar7;
        goto LAB_1409de55f;
      }
      lVar11 = lVar11 + -0x98;
      puVar3 = (undefined8 *)(lVar20 + 0x70);
      lVar20 = lVar20 + 0x98;
    } while ((undefined *)*puVar3 != puVar7);
    local_100 = 5;
    break;
  case 4:
    local_f8 = CONCAT44(uStack_3d4,uStack_3d8);
    local_100 = 1;
    local_e8 = uStack_3d0;
    local_e0 = (undefined *)0x0;
    puVar7 = (undefined *)(ulonglong)uStack_3dc;
    break;
  case 5:
    local_100 = 3;
    local_d0 = (undefined *)(ulonglong)uStack_3dc;
    uStack_23c = uStack_3dc;
  case 3:
  case 10:
  case 0xb:
  case 0xc:
    local_e0 = (undefined *)0x0;
    goto joined_r0x0001409deb18;
  case 0xd:
    local_100 = 3;
    goto code_r0x0001409deab2;
  case 0xe:
    local_100 = 4;
code_r0x0001409deab2:
    local_d0 = (undefined *)(ulonglong)uStack_3d8;
    uStack_23c = uStack_3dc;
    local_e0 = (undefined *)0x0;
    puVar7 = (undefined *)0x0;
    if (_DEV_OVERRIDE_IS_DEMO_MODE != '\0') goto LAB_1409deb25;
    goto code_r0x0001409de57b;
  }
LAB_1409de55f:
  local_d0 = puVar7;
  local_e0 = (undefined *)((ulonglong)local_e0 >> 0x20);
joined_r0x0001409deb18:
  puVar7 = local_e0;
  if (_DEV_OVERRIDE_IS_DEMO_MODE == '\0') {
code_r0x0001409de57b:
    local_e0 = puVar7;
    fVar21 = SQRT((fStack_3c0 - (float)uStack_310) * (fStack_3c0 - (float)uStack_310) +
                  (fStack_3c4 - local_318._4_4_) * (fStack_3c4 - local_318._4_4_) +
                  (local_3c8 - (float)local_318) * (local_3c8 - (float)local_318));
LAB_1409de5d1:
    lVar20 = local_3f0[3];
    if (lVar20 != 0) {
      pauVar19 = (undefined1 (*) [16])*local_3f0;
      pauVar16 = pauVar19 + 1;
      auVar1 = *pauVar19;
      uVar9 = ~(uint)(ushort)((ushort)(SUB161(auVar1 >> 7,0) & 1) |
                              (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                              (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                              (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                              (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                              (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                              (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                              (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                              (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                              (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                              (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                              (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                              (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                              (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                              (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                             (ushort)(byte)(auVar1[0xf] >> 7) << 0xf);
      fVar22 = (float)__xmm_00000000000000003f00000000000000;
      uVar8 = (ulonglong)__xmm_00000000000000003f00000000000000 >> 0x20;
      local_264 = (uint)local_3e0;
      local_258 = uVar2;
      local_228 = lVar12;
      do {
        if ((short)uVar9 == 0) {
          do {
            auVar1 = *pauVar16;
            uVar5 = (ushort)(SUB161(auVar1 >> 7,0) & 1) | (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1
                    | (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                    (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                    (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                    (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                    (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                    (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                    (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                    (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                    (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                    (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                    (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                    (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                    (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                    (ushort)(byte)(auVar1[0xf] >> 7) << 0xf;
            pauVar19 = pauVar19 + -0x10;
            pauVar16 = pauVar16 + 1;
          } while (uVar5 == 0xffff);
          uVar10 = ~(uint)uVar5;
          uVar9 = uVar10 & -(uint)uVar5 - 2;
        }
        else {
          uVar10 = uVar9;
          uVar9 = uVar9 - 1 & uVar9;
        }
        iVar6 = 0;
        for (; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
          iVar6 = iVar6 + 1;
        }
        local_438 = *(undefined **)((longlong)pauVar19 + (-0x10 - (ulonglong)(uint)(iVar6 << 4)));
        local_210 = *(undefined **)((longlong)pauVar19 + (-8 - (ulonglong)(uint)(iVar6 << 4)));
        uVar14 = (ulonglong)local_210 & 0xffffffff;
        if ((*(ulonglong *)(local_110 + 0x10) <= uVar14) ||
           (*(int *)(*(longlong *)(local_110 + 8) + uVar14 * 0x14) !=
            (int)((ulonglong)local_210 >> 0x20))) {
LAB_1409e214c:
          local_218 = (undefined **)((uVar17 & 0xffffffff00000000) + 1);
LAB_1409e2166:
                    /* WARNING: Subroutine does not return */
          core::result::unwrap_failed();
        }
        lVar11 = *(longlong *)(local_110 + 8) + uVar14 * 0x14;
        uVar14 = (ulonglong)*(uint *)(lVar11 + 4);
        if (uVar14 == 0xffffffff) goto LAB_1409e214c;
        if ((*(ulonglong *)(lVar12 + 0x38) <= uVar14) ||
           ((*(ulonglong *)
              (*(longlong *)(lVar12 + 0x28) + (ulonglong)(*(uint *)(lVar11 + 4) >> 6) * 8) >>
             (uVar14 & 0x3f) & 1) == 0)) {
          local_218 = (undefined **)(uVar14 << 0x20);
          goto LAB_1409e2166;
        }
        lVar18 = *(longlong *)(local_110 + 0x1a8) + (ulonglong)*(uint *)(lVar11 + 0xc) * 0x48;
        uVar17 = *(ulonglong *)
                  (*(longlong *)(local_110 + 0x1a8) + 0x40 +
                  (ulonglong)*(uint *)(lVar11 + 0xc) * 0x48);
        if ((*(ulonglong *)(lVar12 + 0x110) < uVar17) &&
           (uVar14 = *(ulonglong *)
                      (*(longlong *)(lVar18 + 0x38) + *(ulonglong *)(lVar12 + 0x110) * 8),
           uVar14 != 0)) {
          lVar13 = *(longlong *)(*(longlong *)(lVar18 + 0x18) + 0x10 + ~uVar14 * 0x30);
          uVar14 = *(ulonglong *)(lVar12 + 0x118);
          if (uVar17 <= uVar14) goto LAB_1409de7a0;
LAB_1409de775:
          uVar17 = *(ulonglong *)(*(longlong *)(lVar18 + 0x38) + uVar14 * 8);
          if (uVar17 == 0) goto LAB_1409de7a0;
          lVar18 = *(longlong *)(*(longlong *)(lVar18 + 0x18) + 0x10 + ~uVar17 * 0x30);
        }
        else {
          lVar13 = 0;
          uVar14 = *(ulonglong *)(lVar12 + 0x118);
          if (uVar14 < uVar17) goto LAB_1409de775;
LAB_1409de7a0:
          lVar18 = 0;
        }
        uVar10 = *(uint *)(lVar11 + 0x10);
        lVar12 = (ulonglong)uVar10 * 0x30;
        uVar2 = *(undefined8 *)(lVar13 + lVar12);
        fVar23 = (float)uVar2 + fVar22;
        fVar24 = (float)((ulonglong)uVar2 >> 0x20) + (float)uVar8;
        fVar25 = *(float *)(lVar13 + 8 + lVar12) + 0.0;
        local_2f8 = &local_438;
        local_2f0 = components::impl$13::fmt;
        local_218 = &PTR_s_delete_tree__142ac3c18;
        local_210 = (undefined *)0x1;
        pppuVar15 = &local_2f8;
        local_200 = 1;
        local_1f8 = (undefined *)0x0;
        local_208 = pppuVar15;
        local_d8 = lVar20;
        alloc::fmt::format::format_inner();
        local_3b8 = local_4e0;
        alloc::string::impl$6::clone();
        local_218 = (undefined **)0x0;
        local_210 = (undefined *)0x8;
        local_208 = (undefined ***)0x0;
        local_200 = CONCAT44(local_200._4_4_,0x17);
        local_1e8 = 0x18;
        local_1d0 = 0;
        local_1b8 = 0;
        local_1a0 = local_1a0 & 0xff00;
        local_190 = 0;
        local_18c = 0x3f800000;
        local_188 = CONCAT44(fVar24,fVar23);
        local_170 = 0x101;
        local_180 = fVar25;
        systems::ui::resources::ui_state::UiState::add<alloc::string::String>();
        lVar20 = local_d8;
        lVar12 = local_228;
        if ((((*(char *)(extraout_RAX_00 + 0x18) != '\0') &&
             (*(undefined ****)(extraout_RAX_00 + 0x38) == pppuVar15)) &&
            (*(int *)(extraout_RAX_00 + 0x28) != 2)) &&
           (*(float *)(extraout_RAX_00 + 0x48) <= fVar21 &&
            fVar21 != *(float *)(extraout_RAX_00 + 0x48))) {
          local_f8 = CONCAT44(fVar24,fVar23);
          local_e8 = local_e8 & 0xffffffff00000000 | (ulonglong)(uint)fVar25;
          local_e0 = (undefined *)((ulonglong)local_438 >> 0x20);
          local_100 = 2;
          local_278 = pppuVar15;
          local_d0 = local_438;
        }
        if (local_3b8 != (undefined8 *)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        lVar20 = lVar20 + -1;
        uVar17 = lVar18 + (ulonglong)uVar10 * 0x58;
      } while (lVar20 != 0);
    }
  }
LAB_1409deb25:
                    /* WARNING: Could not recover jumptable at 0x0001409deb47. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (*(code *)(&DAT_142ac4544 + *(int *)(&DAT_142ac4544 + (ulonglong)local_100 * 4)))();
  return;
}

