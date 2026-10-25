
void __cdecl
system_wall_constructor::construct_lintels::place_support_beams::
place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env_2>
          (void)

{
  char cVar1;
  bool bVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  float fVar14;
  ulonglong uVar15;
  ulonglong uVar16;
  ulonglong uVar17;
  bool bVar18;
  ulonglong uVar19;
  longlong lVar20;
  ulonglong uVar21;
  ulonglong uVar22;
  bool bVar23;
  ulonglong in_stack_00000038;
  ulonglong in_stack_00000040;
  ulonglong in_stack_00000050;
  ulonglong in_stack_00000058;
  ulonglong in_stack_00000068;
  ulonglong local_218;
  float fStack_210;
  float fStack_200;
  char local_1f0;
  undefined4 local_1e8;
  undefined4 uStack_1e4;
  float fStack_1e0;
  float fStack_1d8;
  float fStack_1d0;
  float fStack_1c4;
  char local_1c0;
  ulonglong local_178;
  float local_164;
  char local_160;
  ulonglong local_148;
  float local_134;
  char local_130;
  ulonglong local_118;
  float fStack_110;
  float fStack_100;
  char local_f0;
  
  uVar12 = in_stack_00000038;
  uVar13 = in_stack_00000040;
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  fVar5 = __real_3fb570a3;
  fVar4 = __real_40499999;
  fVar3 = __real_3f800000;
  bVar23 = true;
  lVar20 = 2;
  uVar16 = 0;
  bVar18 = false;
  bVar2 = false;
  uVar11 = 0xffffffffffffffff;
  uVar22 = in_stack_00000050;
  do {
    uVar17 = uVar22;
    uVar15 = uVar11;
    uVar8 = uStack_1e4;
    uVar6 = local_1e8;
    local_1e8 = (undefined4)local_218;
    uVar7 = local_1e8;
    uStack_1e4 = (undefined4)(local_218 >> 0x20);
    uVar9 = uStack_1e4;
    if (lVar20 != 1) {
      if ((lVar20 != 2) || (uVar13 <= uVar16)) {
LAB_141243131:
        if (!bVar18) goto LAB_14124339a;
        if (in_stack_00000040 <= uVar12) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_bounds_check();
        }
        fStack_1d8 = (float)uVar12;
        fStack_1c4 = 0.0;
        local_1c0 = '\0';
        uVar16 = uVar12;
        if (uVar12 != 0) break;
        goto LAB_14124339a;
      }
      while( true ) {
        if (in_stack_00000058 <= uVar16) goto code_r0x000141243662;
        if ((*(byte *)(in_stack_00000050 + uVar16) & 0xf) != 0) break;
        uVar16 = uVar16 + 1;
        if (uVar13 == uVar16) goto LAB_141243131;
      }
      uVar17 = uVar16;
      uVar16 = uVar16 + 1;
    }
    if (bVar23) {
      uVar12 = uVar17;
    }
    uVar22 = uVar16;
    if (uVar16 < uVar13) {
      do {
        if (in_stack_00000058 <= uVar22) {
code_r0x000141243662:
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_bounds_check();
        }
        if ((*(byte *)(in_stack_00000050 + uVar22) & 0xf) != 0) {
          uVar16 = uVar22 + 1;
          lVar20 = 1;
          goto LAB_1412427c2;
        }
        uVar22 = uVar22 + 1;
      } while (uVar13 != uVar22);
      lVar20 = 0;
      uVar16 = uVar13;
      uVar22 = uVar13;
    }
    else {
      lVar20 = 0;
    }
LAB_1412427c2:
    if (lVar20 == 0) {
      bVar2 = true;
      in_stack_00000038 = uVar17;
    }
    bVar18 = true;
    bVar23 = false;
    uVar11 = uVar17;
    local_1e8 = uVar6;
    uStack_1e4 = uVar8;
    if (uVar15 < uVar17) {
      if (in_stack_00000040 <= uVar15) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_bounds_check();
      }
      local_164 = 0.0;
      local_160 = '\0';
      if (in_stack_00000040 <= uVar17) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_bounds_check();
      }
      local_134 = 0.0;
      local_130 = '\0';
      SupportBeamGrower::propose();
      uVar19 = uVar17 - 1;
      SupportBeamGrower::propose();
      local_178 = uVar15;
      local_148 = uVar17;
      if (uVar15 < uVar17) {
        bVar23 = uVar17 <= uVar15 + 1;
        uVar21 = uVar15 + 2;
        if (bVar23) {
          uVar21 = uVar15 + 1;
        }
LAB_141242a31:
        do {
          uVar10 = local_118;
          cVar1 = local_1c0;
          if (local_f0 == '\x02') {
            if (local_1c0 == '\x02') break;
          }
          else if ((local_1c0 == '\x02') || (fStack_100 < fStack_1d0)) {
            if (local_160 == '\x01') goto LAB_141243643;
            local_178 = local_118;
            local_164 = fStack_110 + local_164;
            local_160 = local_f0;
            if ((bVar23) || (uVar17 < uVar21)) {
              local_1f0 = '\x02';
            }
            else {
              if (uVar17 <= uVar21) {
                bVar23 = true;
              }
              SupportBeamGrower::propose();
              uVar21 = uVar21 + (uVar21 < uVar17);
            }
            local_118 = local_218;
            local_f0 = local_1f0;
            fStack_100 = fStack_200;
            fStack_110 = fStack_210;
            if (local_148 <= uVar10) break;
            goto LAB_141242a31;
          }
          if (local_130 == '\x01') goto LAB_141243643;
          local_148 = CONCAT44(uStack_1e4,local_1e8);
          local_134 = fStack_1e0 + local_134;
          if (uVar15 < uVar19) {
            uVar19 = uVar19 - 1;
            SupportBeamGrower::propose();
          }
          else {
            local_1f0 = '\x02';
          }
          local_1c0 = local_1f0;
          fStack_1d0 = fStack_200;
          fStack_1e0 = fStack_210;
          local_1e8 = uVar7;
          uStack_1e4 = uVar9;
          local_130 = cVar1;
        } while (local_178 < local_148);
      }
      fVar14 = (float)uVar17;
      if ((((local_178 != local_148) || (fVar4 < local_164 + local_134)) ||
          (local_164 + local_134 < fVar3)) ||
         (((uVar19 = uVar15, local_178 == uVar15 ||
           (SupportBeamGrower::propose(), uVar19 = local_178, local_1f0 == '\x02')) &&
          ((uVar19 == uVar17 || (SupportBeamGrower::propose(), local_1f0 == '\x02')))))) {
        if (fVar3 <= local_164) {
          fStack_210 = (float)local_178;
          SupportBeamBuilder::
          emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env_2>
                    ();
          bVar23 = true;
          local_218 = uVar15;
          if (local_134 < fVar3) goto LAB_141242f38;
LAB_141242dae:
          local_218 = local_148;
          SupportBeamBuilder::
          emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env_2>
                    ();
          bVar18 = true;
          fStack_210 = fVar14;
          if (local_178 != local_148) goto LAB_141242e13;
LAB_141242f48:
          bVar23 = (bool)(bVar18 | bVar23);
          bVar18 = false;
          if (bVar23) goto LAB_141242f59;
        }
        else {
          bVar23 = false;
          if (fVar3 <= local_134) goto LAB_141242dae;
LAB_141242f38:
          bVar18 = false;
          if (local_178 == local_148) goto LAB_141242f48;
LAB_141242e13:
          if (bVar23) goto LAB_141242e24;
        }
LAB_1412430a0:
        if (bVar18) {
          if (in_stack_00000068 <= uVar17) {
                    /* WARNING: Subroutine does not return */
            core::panicking::panic_bounds_check();
          }
          add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env_2>
                    ();
        }
      }
      else {
        SupportBeamBuilder::
        emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env_2>
                  ();
        bVar18 = true;
        local_218 = uVar15;
        fStack_210 = fVar14;
        if (local_178 != local_148) {
LAB_141242e24:
          if (in_stack_00000068 <= uVar15) {
                    /* WARNING: Subroutine does not return */
            core::panicking::panic_bounds_check();
          }
          add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env_2>
                    ();
          goto LAB_1412430a0;
        }
LAB_141242f59:
        if (fVar5 <= local_164 + local_134) {
          if (in_stack_00000068 <= uVar15) {
                    /* WARNING: Subroutine does not return */
            core::panicking::panic_bounds_check();
          }
          add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env_2>
                    ();
          if (in_stack_00000068 <= uVar17) {
                    /* WARNING: Subroutine does not return */
            core::panicking::panic_bounds_check();
          }
          add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env_2>
                    ();
        }
      }
      bVar23 = false;
      bVar18 = true;
    }
  } while( true );
  while( true ) {
    if (local_1c0 == '\x01') goto LAB_141243643;
    fStack_1c4 = fStack_210 + fStack_1c4;
    local_1c0 = local_1f0;
    if (uVar16 == 0) break;
    uVar16 = uVar16 - 1;
    SupportBeamGrower::propose();
    if (local_1f0 == '\x02') break;
  }
  if (__real_3f800000 <= fStack_1c4) {
    SupportBeamBuilder::
    emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env_2>
              ();
    if (in_stack_00000068 <= uVar12) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
    add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env_2>
              ();
    fStack_210 = fStack_1d8;
  }
LAB_14124339a:
  if (bVar2) {
    if (in_stack_00000040 <= in_stack_00000038) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
    fStack_1c4 = 0.0;
    local_1c0 = '\0';
    uVar16 = in_stack_00000038 + 1;
    if (uVar16 < uVar13) {
      do {
        SupportBeamGrower::propose();
        if (local_1f0 == '\x02') {
          if (fStack_1c4 < __real_3f800000) {
            return;
          }
          goto LAB_141243523;
        }
        if (local_1c0 == '\x01') {
LAB_141243643:
                    /* WARNING: Subroutine does not return */
          core::panicking::panic();
        }
        fStack_1c4 = fStack_210 + fStack_1c4;
        uVar16 = uVar16 + 1;
        local_1c0 = local_1f0;
      } while (uVar13 != uVar16);
      if (__real_3f800000 <= fStack_1c4) {
LAB_141243523:
        SupportBeamBuilder::
        emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env_2>
                  ();
        if (in_stack_00000068 <= in_stack_00000038) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_bounds_check();
        }
        add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env_2>
                  ();
      }
    }
  }
  return;
}

