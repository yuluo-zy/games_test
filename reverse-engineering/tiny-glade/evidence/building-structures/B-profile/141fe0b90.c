
/* WARNING: Removing unreachable block (ram,0x000141fe13d9) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>(void)

{
  longlong lVar1;
  float fVar2;
  float fVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  undefined1 auVar6 [16];
  ulonglong uVar7;
  float fVar8;
  ulonglong uVar9;
  undefined8 *extraout_RAX;
  undefined1 *extraout_RAX_00;
  undefined1 *extraout_RAX_01;
  ulonglong uVar10;
  ulonglong *puVar11;
  ulonglong *in_RCX;
  float *in_RDX;
  ulonglong uVar12;
  ulonglong in_R8;
  float *pfVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  undefined8 *puVar16;
  size_t sVar17;
  undefined1 *_Dst;
  bool bVar18;
  float fVar19;
  float fVar21;
  undefined1 auVar20 [16];
  float in_XMM3_Da;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  float fVar30;
  float fVar31;
  float fVar32;
  float fVar33;
  float fVar34;
  float fVar35;
  float fVar36;
  float fVar37;
  float fVar38;
  float fVar39;
  float fVar40;
  float in_stack_00000028;
  ulonglong local_178;
  ulonglong uStack_170;
  ulonglong local_168;
  ulonglong local_120;
  ulonglong local_118;
  undefined8 *local_110;
  ulonglong local_108;
  undefined1 *local_100;
  ulonglong local_f8;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  if (in_R8 < 3) {
    if (in_R8 == 0) {
      _Dst = &DAT_00000004;
      puVar16 = (undefined8 *)&DAT_00000008;
      sVar17 = 0;
      uVar15 = 0;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error();
      }
      lVar1 = in_R8 - 1;
      auVar20._8_4_ = (int)lVar1;
      auVar20._0_8_ = lVar1;
      auVar20._12_4_ = (int)((ulonglong)lVar1 >> 0x20);
      *extraout_RAX = 0;
      auVar6._8_4_ = _UNK_1429258f8;
      auVar6._0_8_ = ___xmm_80000000800000008000000080000000;
      auVar6._12_4_ = _UNK_1429258fc;
      if (SUB164(auVar20 ^ auVar6,0xc) != _UNK_1429258f8 ||
          _UNK_142ae0ce8 <= SUB164(auVar20 ^ auVar6,8)) {
        extraout_RAX[1] = 1;
      }
      sVar17 = in_R8 * 0xc;
      std::alloc::__default_lib_allocator::__rust_alloc();
      uVar15 = in_R8;
      puVar16 = extraout_RAX;
      _Dst = extraout_RAX_00;
      if (extraout_RAX_00 == (undefined1 *)0x0) {
        local_100 = (undefined1 *)(in_R8 * 8);
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error();
      }
    }
    memcpy(_Dst,in_RDX,sVar17);
    *in_RCX = uVar15;
    in_RCX[1] = (ulonglong)puVar16;
    in_RCX[2] = uVar15;
    in_RCX[3] = in_R8;
    in_RCX[4] = (ulonglong)_Dst;
    in_RCX[5] = in_R8;
  }
  else {
    local_120 = 0;
    local_118 = 0;
    local_110 = (undefined8 *)&DAT_00000008;
    local_108 = 0;
    alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
    uVar15 = local_120;
    *local_110 = 0;
    local_108 = 1;
    if (local_120 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                ();
    }
    *(undefined8 *)(uVar15 * 0x18 + 8) = 0;
    *(ulonglong *)(uVar15 * 0x18 + 0x10) = in_R8 - 1;
    *(undefined1 *)(uVar15 * 0x18 + 0x18) = 0;
    local_120 = uVar15 + 1;
    pfVar13 = in_RDX + in_R8 * 3;
    if (pfVar13 == (float *)&DAT_0000000c) {
                    /* WARNING: Subroutine does not return */
      core::option::unwrap_failed();
    }
    fVar27 = SQRT((in_RDX[2] - pfVar13[-1]) * (in_RDX[2] - pfVar13[-1]) +
                  (in_RDX[1] - pfVar13[-2]) * (in_RDX[1] - pfVar13[-2]) +
                  (*in_RDX - pfVar13[-3]) * (*in_RDX - pfVar13[-3]));
    local_100 = (undefined1 *)
                CONCAT44(local_100._4_4_,
                         (int)CONCAT71((int7)((ulonglong)pfVar13 >> 8),fVar27 < __real_4cbebc20));
    local_f8 = CONCAT44(local_f8._4_4_,in_stack_00000028 * in_stack_00000028);
    fVar35 = in_stack_00000028 * in_stack_00000028;
    fVar26 = __real_1e3ce508;
    while (uVar15 = local_120, local_120 != 0) {
      while( true ) {
        uVar7 = local_108;
        fVar32 = __real_1e3ce508;
        local_120 = uVar15 - 1;
        if (*(char *)(local_120 * 0x18 + 0x18) == '\x02') goto LAB_141fe0df0;
        puVar11 = (ulonglong *)(local_120 * 0x18 + 8);
        uVar4 = *(ulonglong *)(local_120 * 0x18 + 0x10);
        uVar5 = *puVar11;
        if (2 < (uVar4 - uVar5) + 1) break;
        if (local_108 == local_118) {
          alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
        }
        local_110[uVar7] = uVar4;
        local_108 = uVar7 + 1;
        uVar15 = local_120;
        if (local_120 == 0) goto LAB_141fe0df0;
      }
      if (in_R8 <= uVar5) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_bounds_check();
      }
      if (in_R8 <= uVar4) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_bounds_check();
      }
      if (uVar4 < uVar5) {
                    /* WARNING: Subroutine does not return */
        core::slice::index::slice_index_order_fail();
      }
      fVar8 = (float)local_f8;
      if (in_RDX + uVar5 * 3 + 3 == in_RDX + uVar4 * 3 || uVar4 == uVar5) {
        fVar35 = 0.0;
        uVar10 = 0;
      }
      else {
        fVar2 = in_RDX[uVar5 * 3 + 2];
        fVar33 = (float)*(undefined8 *)(in_RDX + uVar4 * 3);
        fVar34 = (float)((ulonglong)*(undefined8 *)(in_RDX + uVar4 * 3) >> 0x20);
        fVar3 = in_RDX[uVar4 * 3 + 2];
        uVar12 = (ulonglong)((longlong)(in_RDX + uVar4 * 3) - (longlong)(in_RDX + uVar5 * 3 + 3)) /
                 0xc;
        fVar19 = (float)*(undefined8 *)(in_RDX + uVar5 * 3);
        fVar22 = fVar33 - fVar19;
        fVar21 = (float)((ulonglong)*(undefined8 *)(in_RDX + uVar5 * 3) >> 0x20);
        fVar23 = fVar34 - fVar21;
        fVar24 = fVar3 - fVar2;
        fVar25 = fVar24 * fVar24 + fVar23 * fVar23 + fVar22 * fVar22;
        if (fVar25 <= fVar26) {
          fVar25 = fVar26;
        }
        if ((fVar35 != 0.0) || (NAN(fVar35))) {
          pfVar13 = in_RDX + uVar5 * 3 + 5;
          fVar35 = 0.0;
          uVar10 = 0;
          uVar14 = 1;
          do {
            fVar30 = (float)*(undefined8 *)(pfVar13 + -2);
            fVar31 = (float)((ulonglong)*(undefined8 *)(pfVar13 + -2) >> 0x20);
            fVar26 = *pfVar13;
            fVar38 = fVar30 - fVar19;
            fVar39 = fVar31 - fVar21;
            fVar40 = fVar26 - fVar2;
            fVar37 = (fVar24 * fVar40 + fVar23 * fVar39 + fVar22 * fVar38) / fVar25;
            fVar28 = __real_3f800000 - fVar37;
            fVar36 = fVar26 - (fVar3 * fVar37 + fVar2 * fVar28);
            fVar29 = fVar30 - (fVar37 * fVar33 + fVar28 * fVar19);
            fVar37 = fVar31 - (fVar37 * fVar34 + fVar28 * fVar21);
            fVar37 = SQRT(fVar36 * fVar36 + fVar37 * fVar37 + fVar29 * fVar29);
            if (0x7f7fffff < (uint)ABS(fVar37)) goto LAB_141fe1446;
            if (((fVar35 < fVar37) &&
                ((float)local_f8 <= fVar40 * fVar40 + fVar39 * fVar39 + fVar38 * fVar38)) &&
               ((float)local_f8 <=
                (fVar26 - fVar3) * (fVar26 - fVar3) +
                (fVar31 - fVar34) * (fVar31 - fVar34) + (fVar30 - fVar33) * (fVar30 - fVar33))) {
              uVar10 = uVar14;
              fVar35 = fVar37;
            }
            pfVar13 = pfVar13 + 3;
            bVar18 = uVar14 != uVar12;
            uVar14 = uVar14 + 1;
          } while (bVar18);
        }
        else {
          pfVar13 = in_RDX + uVar5 * 3 + 5;
          fVar35 = 0.0;
          uVar10 = 0;
          uVar14 = 1;
          do {
            fVar37 = (float)*(undefined8 *)(pfVar13 + -2);
            fVar26 = (float)((ulonglong)*(undefined8 *)(pfVar13 + -2) >> 0x20);
            fVar29 = ((*pfVar13 - fVar2) * fVar24 +
                     (fVar26 - fVar21) * fVar23 + (fVar37 - fVar19) * fVar22) / fVar25;
            fVar28 = __real_3f800000 - fVar29;
            fVar26 = fVar26 - (fVar34 * fVar29 + fVar21 * fVar28);
            fVar36 = *pfVar13 - (fVar29 * fVar3 + fVar28 * fVar2);
            fVar37 = fVar37 - (fVar29 * fVar33 + fVar28 * fVar19);
            fVar26 = SQRT(fVar37 * fVar37 + fVar26 * fVar26 + fVar36 * fVar36);
            if (0x7f7fffff < (uint)ABS(fVar26)) {
LAB_141fe1446:
                    /* WARNING: Subroutine does not return */
              core::panicking::panic();
            }
            uVar9 = uVar14;
            if (fVar26 <= fVar35) {
              fVar26 = fVar35;
              uVar9 = uVar10;
            }
            uVar10 = uVar9;
            fVar35 = fVar26;
            pfVar13 = pfVar13 + 3;
            bVar18 = uVar14 != uVar12;
            uVar14 = uVar14 + 1;
          } while (bVar18);
        }
      }
      if ((in_XMM3_Da < fVar35) || ((fVar27 < fVar35 & (byte)local_100) != 0)) {
        uVar4 = uVar10 + uVar5;
        *puVar11 = uVar4;
        *(undefined1 *)(local_120 * 0x18 + 0x18) = 0;
        fVar32 = __real_1e3ce508;
        local_120 = uVar15;
        if (uVar10 < 3) {
          if (local_108 == local_118) {
            alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
          }
          local_110[uVar7] = uVar4;
          local_100 = (undefined1 *)((ulonglong)local_100 & 0xffffffff00000000);
          goto LAB_141fe1193;
        }
        if (uVar15 == 0) {
          alloc::raw_vec::RawVec::
          grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                    ();
        }
        *(ulonglong *)(uVar15 * 0x18 + 8) = uVar5;
        *(ulonglong *)(uVar15 * 0x18 + 0x10) = uVar4;
        *(undefined1 *)(uVar15 * 0x18 + 0x18) = 0;
        local_100 = (undefined1 *)((ulonglong)local_100 & 0xffffffff00000000);
        puVar11 = &local_120;
      }
      else {
        if (local_108 == local_118) {
          alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
        }
        local_110[uVar7] = uVar4;
LAB_141fe1193:
        puVar11 = &local_108;
        uVar15 = uVar7;
      }
      *puVar11 = uVar15 + 1;
      fVar35 = fVar8;
      fVar26 = fVar32;
    }
LAB_141fe0df0:
    uVar15 = local_108;
    puVar16 = local_110;
    sVar17 = local_108 * 8;
    if ((local_108 >> 0x3d != 0) || (0x7ffffffffffffff8 < sVar17)) {
LAB_141fe1477:
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error();
    }
    if (sVar17 == 0) {
      local_100 = &DAT_00000008;
      local_f8 = 0;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_01 == (undefined1 *)0x0) goto LAB_141fe1477;
      local_f8 = uVar15;
      local_100 = extraout_RAX_01;
    }
    memcpy(local_100,puVar16,sVar17);
    alloc::vec::spec_from_iter::impl$0::
    from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<usize>,utils::rdp::impl_0::new_with_opts::closure_env_2<glam::f32::vec3::Vec3>_>_>
              ();
    *in_RCX = local_f8;
    in_RCX[1] = (ulonglong)local_100;
    in_RCX[2] = uVar15;
    in_RCX[5] = local_168;
    in_RCX[3] = local_178;
    in_RCX[4] = uStack_170;
    if (local_118 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  return;
}

