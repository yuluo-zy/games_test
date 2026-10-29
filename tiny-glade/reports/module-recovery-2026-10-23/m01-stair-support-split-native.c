
void __cdecl
system_decorator::decorator_visual::stairs_assemble::
split_segment_into_supported_and_unsupported_subsegments(void)

{
  float fVar1;
  float fVar2;
  float fVar3;
  float *pfVar4;
  longlong lVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  bool bVar8;
  longlong lVar9;
  longlong lVar10;
  undefined8 *extraout_RAX;
  float *extraout_RAX_00;
  float *extraout_RAX_01;
  longlong *in_RCX;
  longlong lVar11;
  longlong lVar12;
  longlong *plVar13;
  longlong in_R9;
  bool bVar14;
  ulonglong uVar15;
  float extraout_XMM0_Da;
  float fVar16;
  float in_XMM1_Da;
  float fVar17;
  float fVar18;
  char local_1c0;
  undefined7 uStack_1bf;
  float local_1b8;
  float local_1a8;
  float local_1a0;
  longlong local_160;
  ulonglong local_158;
  longlong local_140;
  longlong local_138;
  longlong lStack_130;
  longlong local_128;
  longlong local_120;
  longlong lStack_118;
  longlong local_110;
  ulonglong local_108;
  ulonglong local_100;
  float *local_f8;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  local_120 = 0;
  lStack_118 = 8;
  local_110 = 0;
  local_138 = 0;
  lStack_130 = 8;
  local_128 = 0;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 == (float *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  local_108 = *(ulonglong *)(in_R9 + 0x60);
  if (local_108 == 0) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  pfVar4 = *(float **)(in_R9 + 0x58);
  *extraout_RAX_00 = *pfVar4 * 0.5;
  *extraout_RAX = 1;
  extraout_RAX[1] = extraout_RAX_00;
  extraout_RAX[2] = 1;
  local_140 = 1;
  lVar5 = *(longlong *)(in_R9 + 8);
  uVar6 = *(ulonglong *)(in_R9 + 0x10);
  local_100 = uVar6 - 1;
  uVar7 = *(ulonglong *)(in_R9 + 0x48);
  lVar11 = *(longlong *)(in_R9 + 0x40);
  uVar15 = 1;
  bVar14 = false;
  local_158 = 0;
  local_f8 = (float *)0x1;
  if (1 < local_100) {
    do {
      while( true ) {
        country_core::resources::stairs::stairs_assembly_params::StairWallAnchor::wall_normal();
        if (uVar7 <= uVar15) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        fVar18 = *(float *)(lVar11 + uVar15 * 8);
        fVar16 = *(float *)(lVar11 + 4 + uVar15 * 8);
        ::utils::swizzlers::impl$0::x0y();
        fVar18 = *(float *)(&__real_bf8000003f800000 +
                           (ulonglong)(extraout_XMM0_Da * fVar16 < in_XMM1_Da * fVar18) * 4);
        fVar16 = local_1b8 * fVar18;
        fVar17 = fVar18 * (float)CONCAT71(uStack_1bf,local_1c0);
        fVar18 = fVar18 * (float)((uint7)uStack_1bf >> 0x18);
        fVar18 = 1.0 / SQRT(fVar18 * fVar18 + fVar17 * fVar17 + fVar16 * fVar16);
        bVar8 = (int)fVar18 - 1U < 0x7fffff ||
                (int)ABS(fVar18) - 0x800000U < 0x7f000000 && -1 < (int)fVar18;
        if (bVar8) {
          fVar16 = fVar16 * fVar18;
        }
        else {
          fVar16 = 0.0;
        }
        if (uVar6 <= uVar15) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        if (local_108 <= uVar15) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        in_XMM1_Da = *(float *)(lVar5 + 4 + uVar15 * 0xc) + -1.17;
        fVar1 = pfVar4[uVar15];
        fVar2 = *(float *)(lVar5 + 8 + uVar15 * 0xc);
        fVar3 = *(float *)(lVar5 + uVar15 * 0xc);
        country_core::systems::collision::world::RaycastWorld::raycast_simple();
        lVar10 = local_110;
        lVar12 = local_128;
        lVar9 = local_140;
        if (local_1c0 == '\x11') break;
        fVar17 = (fVar3 + fVar1 * (float)((int)((uint)bVar8 << 0x1f) >> 0x1f &
                                         (uint)(fVar17 * fVar18)) * -0.5) - local_1a8;
        fVar18 = (fVar16 * fVar1 * -0.5 + fVar2) - local_1a0;
        fVar18 = pfVar4[uVar15] * -0.5 + SQRT(fVar18 * fVar18 + fVar17 * fVar17);
        if (bVar14) {
          if (local_110 == local_120) {
            alloc::raw_vec::RawVec::
            grow_one<alloc::boxed::Box<dyn_<minidump_unwind::symbols::SymbolProvider,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>,alloc::alloc::Global>
                      ();
          }
          lVar12 = lVar10 * 0x10;
          *(longlong *)(lStack_118 + lVar12) = local_160;
          *(ulonglong *)(lStack_118 + 8 + lVar12) = uVar15;
          local_110 = lVar10 + 1;
          std::alloc::__default_lib_allocator::__rust_alloc();
          lVar12 = local_140;
          if (extraout_RAX_01 == (float *)0x0) {
                    // WARNING: Subroutine does not return
            alloc::alloc::handle_alloc_error();
          }
          *extraout_RAX_01 = fVar18;
          local_f8 = extraout_RAX_01;
          if (local_140 == 1) {
            alloc::raw_vec::RawVec::
            grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                      ();
          }
          extraout_RAX[lVar12 * 3] = 1;
          extraout_RAX[lVar12 * 3 + 1] = local_f8;
          extraout_RAX[lVar12 * 3 + 2] = 1;
          local_f8 = (float *)0x1;
          plVar13 = &local_140;
          bVar14 = false;
          local_158 = uVar15;
        }
        else {
          if ((local_140 == 0) || (extraout_RAX + local_140 * 3 == (undefined8 *)&DAT_00000018)) {
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
          lVar12 = extraout_RAX[local_140 * 3 + -1];
          if (lVar12 == extraout_RAX[local_140 * 3 + -3]) {
            alloc::raw_vec::RawVec::grow_one<f32,alloc::alloc::Global>();
          }
          plVar13 = extraout_RAX + lVar9 * 3 + -1;
          *(float *)(extraout_RAX[lVar9 * 3 + -2] + lVar12 * 4) = fVar18;
        }
LAB_1407d97fe:
        *plVar13 = lVar12 + 1;
        uVar15 = uVar15 + 1;
        if (local_100 <= uVar15) goto LAB_1407d965b;
      }
      if (!bVar14) {
        if (((ulonglong)local_f8 & 1) == 0) {
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        if (local_128 == local_138) {
          alloc::raw_vec::RawVec::
          grow_one<alloc::boxed::Box<dyn_<minidump_unwind::symbols::SymbolProvider,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>,alloc::alloc::Global>
                    ();
        }
        local_160 = uVar15 - 1;
        *(ulonglong *)(lStack_130 + lVar12 * 0x10) = local_158;
        *(longlong *)(lStack_130 + 8 + lVar12 * 0x10) = local_160;
        bVar14 = true;
        plVar13 = &local_128;
        local_f8 = (float *)0x0;
        goto LAB_1407d97fe;
      }
      bVar14 = true;
      uVar15 = uVar15 + 1;
    } while (uVar15 < local_100);
  }
LAB_1407d965b:
  lVar5 = local_110;
  if (bVar14) {
    if (local_110 == local_120) {
      alloc::raw_vec::RawVec::
      grow_one<alloc::boxed::Box<dyn_<minidump_unwind::symbols::SymbolProvider,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>,alloc::alloc::Global>
                ();
    }
    lVar11 = lVar5 * 0x10;
    *(longlong *)(lStack_118 + lVar11) = local_160;
    *(ulonglong *)(lStack_118 + 8 + lVar11) = local_100;
    local_110 = lVar5 + 1;
  }
  lVar5 = local_128;
  if (local_f8 == (float *)0x1) {
    if (local_128 == local_138) {
      alloc::raw_vec::RawVec::
      grow_one<alloc::boxed::Box<dyn_<minidump_unwind::symbols::SymbolProvider,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>,alloc::alloc::Global>
                ();
    }
    lVar11 = local_140;
    lVar12 = lVar5 * 0x10;
    *(ulonglong *)(lStack_130 + lVar12) = local_158;
    *(ulonglong *)(lStack_130 + 8 + lVar12) = local_100;
    local_128 = lVar5 + 1;
    if ((local_140 == 0) || (extraout_RAX + local_140 * 3 == (undefined8 *)&DAT_00000018)) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    if (pfVar4 + (local_108 - 1) == (float *)0x0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    fVar18 = pfVar4[local_108 - 1];
    lVar5 = extraout_RAX[local_140 * 3 + -1];
    if (lVar5 == extraout_RAX[local_140 * 3 + -3]) {
      alloc::raw_vec::RawVec::grow_one<f32,alloc::alloc::Global>();
    }
    *(float *)(extraout_RAX[lVar11 * 3 + -2] + lVar5 * 4) = fVar18 * 0.5;
    extraout_RAX[lVar11 * 3 + -1] = lVar5 + 1;
  }
  in_RCX[2] = local_110;
  *in_RCX = local_120;
  in_RCX[1] = lStack_118;
  in_RCX[3] = local_138;
  in_RCX[4] = lStack_130;
  in_RCX[5] = local_128;
  in_RCX[6] = 1;
  in_RCX[7] = (longlong)extraout_RAX;
  in_RCX[8] = local_140;
  return;
}

