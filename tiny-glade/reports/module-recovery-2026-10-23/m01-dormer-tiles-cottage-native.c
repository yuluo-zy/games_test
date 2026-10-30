
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl
system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::
assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env_7>
          (void)

{
  byte *pbVar1;
  longlong lVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  undefined1 auVar6 [16];
  undefined1 auVar7 [16];
  undefined1 auVar8 [16];
  undefined1 auVar9 [16];
  undefined1 auVar10 [16];
  undefined1 auVar11 [16];
  undefined1 auVar12 [16];
  undefined1 auVar13 [16];
  undefined1 auVar14 [16];
  undefined1 auVar15 [16];
  undefined1 auVar16 [16];
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  char extraout_AL;
  uint uVar21;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong lVar22;
  ulonglong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  undefined8 extraout_RAX_04;
  ulonglong extraout_RAX_05;
  undefined8 extraout_RAX_06;
  ulonglong extraout_RAX_07;
  longlong *extraout_RAX_08;
  longlong *extraout_RAX_09;
  longlong *extraout_RAX_10;
  longlong in_RCX;
  longlong *plVar23;
  ulonglong in_RDX;
  ulonglong uVar24;
  longlong lVar25;
  float *pfVar26;
  bool bVar27;
  undefined *puVar28;
  ulonglong uVar29;
  longlong in_R9;
  longlong lVar30;
  float extraout_XMM0_Da;
  undefined4 extraout_XMM0_Da_00;
  undefined4 extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  float fVar31;
  float extraout_XMM0_Da_05;
  float extraout_XMM0_Da_06;
  float fVar32;
  float fVar33;
  float fVar34;
  undefined4 extraout_XMM0_Da_07;
  undefined4 extraout_XMM0_Da_08;
  undefined4 extraout_XMM0_Da_09;
  undefined4 extraout_XMM0_Da_10;
  float extraout_XMM0_Da_11;
  float extraout_XMM0_Da_12;
  float extraout_XMM0_Da_13;
  undefined4 extraout_XMM0_Da_14;
  undefined4 extraout_XMM0_Da_15;
  float fVar35;
  float extraout_XMM0_Da_16;
  float fVar36;
  float fVar37;
  float fVar38;
  float in_XMM1_Da;
  undefined4 uVar39;
  float fVar40;
  float fVar41;
  float fVar42;
  float fVar43;
  float in_XMM2_Da;
  float fVar44;
  float fVar47;
  undefined1 auVar45 [16];
  undefined1 auVar46 [16];
  float fVar48;
  undefined1 auVar49 [16];
  float fVar50;
  float unaff_XMM10_Da;
  float unaff_XMM11_Da;
  float fVar51;
  float fVar52;
  float fVar53;
  float fVar54;
  float fVar55;
  undefined4 *in_stack_00000028;
  char in_stack_00000048;
  longlong local_3d0;
  float *local_3c8;
  longlong local_3c0;
  float local_398;
  float local_390;
  float local_38c;
  float local_384;
  undefined8 local_370;
  float local_368;
  float local_328;
  float fStack_324;
  longlong local_2f0;
  longlong local_2e8;
  longlong local_2e0;
  float local_2c8;
  float fStack_2c4;
  float local_268;
  float local_264;
  float local_260;
  float local_25c;
  float local_258;
  float local_254;
  longlong *local_250;
  float local_248;
  undefined8 local_240;
  float local_238;
  undefined8 local_230;
  float local_228;
  undefined8 local_220;
  float local_218;
  float local_210;
  float local_20c;
  uint local_208;
  uint local_204;
  uint local_200;
  float local_1fc;
  float local_1f8;
  float local_1f4;
  longlong local_1f0;
  undefined8 local_1e8;
  undefined8 uStack_1e0;
  undefined8 local_1d8;
  float *local_1d0;
  ulonglong *local_1c0;
  float local_1b0;
  float local_1ac;
  undefined8 local_1a8;
  float fStack_1a0;
  float fStack_19c;
  float fStack_198;
  undefined4 uStack_194;
  ulonglong uStack_190;
  undefined8 local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 uStack_170;
  float local_168;
  float local_164;
  float local_160;
  float local_15c;
  float local_158;
  undefined4 local_154;
  float local_150;
  float local_14c;
  float local_148;
  float local_144;
  longlong local_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 *local_128;
  ulonglong uStack_120;
  undefined8 local_118;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined4 uStack_108;
  int iStack_104;
  undefined8 uStack_100;
  ulonglong local_f8;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  local_1ac = in_XMM1_Da;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    lVar25 = 0;
  }
  else {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (in_RDX != 0) {
        if (in_RDX < 0x6b) {
          if ((char)(&DAT_142c291f0)[in_RDX] < -0x40) {
LAB_14128dbea:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX != 0x6b) goto LAB_14128dbea;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (in_RDX != 0xfffffffffffffffe)) {
        if (in_RDX + 2 < 0x6b) {
          if ("stem_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles::f"
              [in_RDX] < -0x40) {
LAB_14128dc3e:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX + 2 != 0x6b) goto LAB_14128dc3e;
      }
    }
    puVar28 = &DAT_142c29383;
    do {
      bVar3 = puVar28[-1];
      uVar21 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = puVar28[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar28[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar28 + -4;
            puVar28 = puVar28 + -4;
            uVar21 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar28 = puVar28 + -3;
            uVar21 = bVar5 & 0xf;
          }
          uVar21 = bVar4 & 0x3f | uVar21 << 6;
        }
        else {
          puVar28 = puVar28 + -2;
          uVar21 = bVar4 & 0x1f;
        }
        uVar21 = (uint)(bVar3 & 0x3f) | uVar21 << 6;
        if (uVar21 != 0x5c) goto LAB_14128dd04;
LAB_14128dd16:
        if (puVar28 == &UNK_142c29327) break;
        if (puVar28 + -0x142c29327 < (undefined *)0x5b) {
          if (-0x41 < (char)puVar28[1]) break;
        }
        else if (puVar28 + -0x142c29327 == (undefined *)0x5b) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar28 = puVar28 + -1;
      if (uVar21 == 0x5c) goto LAB_14128dd16;
LAB_14128dd04:
      if (uVar21 == 0x2f) goto LAB_14128dd16;
    } while (puVar28 != &DAT_142c29328);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar23 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar23 = extraout_RAX_09, extraout_RAX_09 == (longlong *)0x0))
    goto LAB_141290100;
    if (*plVar23 != 0) goto LAB_14129010c;
    *plVar23 = -1;
    local_1e8 = plVar23;
    puffin::ThreadProfiler::begin_scope();
    *local_1e8 = *local_1e8 + 1;
    lVar25 = 1;
  }
  local_140 = lVar25;
  country_core::resources::roofs::roof_shape::Roof::height();
  if (2 < in_R9 - 1U) {
    local_140 = lVar25;
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  fVar40 = *(float *)(&DAT_142c293b4 + in_R9 * 4);
  fVar32 = *(float *)(in_RCX + 0x10);
  local_158 = *(float *)(in_RCX + 0x14);
  local_138 = *(undefined ***)(in_RCX + 0x10);
  fVar33 = *(float *)(in_RCX + 0x18);
  uStack_130 = CONCAT44(uStack_130._4_4_,fVar33);
  local_140 = lVar25;
  country_core::resources::roofs::roof_shape::Roof::get_roof_shape_from_pos_ws_clamped();
  local_160 = fVar32;
  local_15c = fVar33;
  local_140 = lVar25;
  country_core::systems::wall::valid_enclosure::ShapeParameters::get_closest_u_from_pos();
  local_140 = lVar25;
  country_core::systems::wall::valid_enclosure::ShapeParameters::get_pos_at_u();
  local_148 = in_XMM2_Da + local_158;
  local_140 = lVar25;
  ::utils::swizzlers::impl$1::xvy();
  local_14c = extraout_XMM0_Da * fVar40 + local_148;
  local_1a8 = (undefined8 *)CONCAT44(fVar33,fVar32);
  local_140 = lVar25;
  ::utils::swizzlers::impl$1::xvy();
  local_140 = lVar25;
  country_core::resources::roofs::roof_shape::Roof::get_roof_shape_from_pos_ws_clamped();
  local_140 = lVar25;
  country_core::systems::wall::valid_enclosure::ShapeParameters::get_closest_u_from_pos();
  uVar39 = extraout_XMM0_Da_00;
  local_140 = lVar25;
  country_core::systems::wall::valid_enclosure::ShapeParameters::get_pos_at_u();
  local_138 = (undefined **)CONCAT44(uVar39,extraout_XMM0_Da_01);
  local_140 = lVar25;
  ::utils::swizzlers::impl$1::xvy();
  fVar40 = SQRT(fVar33 * 10.0 * fVar33 * 10.0 +
                local_158 * 10.0 * local_158 * 10.0 + fVar32 * 10.0 * fVar32 * 10.0);
  local_f8 = (longlong)(fVar40 - 9.223372e+18) & (longlong)fVar40 >> 0x3f | (longlong)fVar40;
  if (fVar40 < 0.0) {
    local_f8 = 0;
  }
  fVar50 = 1.8446743e+19;
  if (1.8446743e+19 < fVar40) {
    local_f8 = 0xffffffffffffffff;
  }
  ceilf(SQRT((local_260 - local_254) * (local_260 - local_254) +
             (local_264 - local_258) * (local_264 - local_258) +
             (local_268 - local_25c) * (local_268 - local_25c)) / 0.4);
  local_140 = lVar25;
  ::utils::random_splits();
  country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter();
  lVar2 = local_2e8 + local_2e0 * 4;
  local_154 = *in_stack_00000028;
  fVar40 = (local_1ac / extraout_XMM0_Da_02) * -0.5;
  fVar36 = (local_1ac / extraout_XMM0_Da_02) * 0.5;
  bVar27 = false;
  local_1f0 = 0;
  local_1f8 = fVar32;
  local_1f4 = fVar33;
  while (fVar33 = local_1f4, fVar32 = local_1f8, local_2e8 != lVar2) {
    ::utils::inverse_lerp();
    lVar22 = local_2e8 + 4;
    fVar31 = extraout_XMM0_Da_03;
    if (!bVar27) {
      if (local_2e8 + 4 == lVar2) break;
      ::utils::inverse_lerp();
      lVar22 = local_2e8 + 8;
      unaff_XMM11_Da = extraout_XMM0_Da_03;
      fVar31 = extraout_XMM0_Da_04;
    }
    local_2e8 = lVar22;
    local_144 = local_14c * unaff_XMM11_Da;
    local_150 = local_14c * fVar31;
    local_210 = local_158 * (1.0 - unaff_XMM11_Da) + local_144;
    local_1b0 = local_158 * (1.0 - fVar31) + local_150;
    local_1d8 = (float *)CONCAT44(fVar33,fVar32);
    ::utils::swizzlers::impl$1::xvy();
    country_core::resources::roofs::roof_shape::Roof::get_roof_shape_from_pos_ws_clamped();
    country_core::systems::wall::valid_enclosure::ShapeParameters::expand();
    local_1d8 = (float *)CONCAT44(fVar33,fVar32);
    ::utils::swizzlers::impl$1::xvy();
    country_core::resources::roofs::roof_shape::Roof::get_roof_shape_from_pos_ws_clamped();
    country_core::systems::wall::valid_enclosure::ShapeParameters::expand();
    country_core::systems::wall::valid_enclosure::ShapeParameters::get_closest_u_from_pos();
    country_core::systems::wall::valid_enclosure::ShapeParameters::get_closest_u_from_pos();
    country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter();
    country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter();
    ::utils::random_splits();
    lVar22 = local_1f0 + 1;
    local_144 = local_144 + (1.0 - unaff_XMM11_Da) * local_148;
    local_150 = local_150 + (1.0 - fVar31) * local_148;
    local_1fc = fVar31;
    fVar32 = fmodf(extraout_XMM0_Da_05 + fVar40,1.0);
    fVar33 = fmodf(extraout_XMM0_Da_05 + fVar36,1.0);
    fVar53 = (float)((uint)(fVar32 + 1.0) & -(uint)(fVar32 < 0.0) |
                    ~-(uint)(fVar32 < 0.0) & (uint)fVar32);
    fVar54 = (float)((uint)(fVar33 + 1.0) & -(uint)(fVar33 < 0.0) |
                    ~-(uint)(fVar33 < 0.0) & (uint)fVar33);
    fVar32 = fmodf(extraout_XMM0_Da_06 + fVar40,1.0);
    fVar33 = fmodf(extraout_XMM0_Da_06 + fVar36,1.0);
    fVar31 = (float)((uint)(fVar32 + 1.0) & -(uint)(fVar32 < 0.0) |
                    ~-(uint)(fVar32 < 0.0) & (uint)fVar32);
    fVar33 = (float)((uint)(fVar33 + 1.0) & -(uint)(fVar33 < 0.0) |
                    ~-(uint)(fVar33 < 0.0) & (uint)fVar33);
    fVar32 = fmodf(extraout_XMM0_Da_05 - fVar36,1.0);
    fVar32 = (float)(~-(uint)(fVar32 < 0.0) & (uint)fVar32 |
                    (uint)(fVar32 + 1.0) & -(uint)(fVar32 < 0.0));
    local_208 = -(uint)(fVar32 <= 1.0 && 0.0 <= fVar32);
    local_204 = -(uint)(0.0 <= fVar54 && fVar54 <= 1.0);
    local_200 = -(uint)(0.0 <= extraout_XMM0_Da_05 && extraout_XMM0_Da_05 <= 1.0);
    bVar27 = false;
    lVar30 = 0;
    pfVar26 = local_3c8;
    while (pfVar26 != local_3c8 + local_3c0) {
      fVar34 = *pfVar26;
      fVar44 = fVar53 * (1.0 - fVar34);
      if (fVar53 <= fVar54) {
        fVar44 = fVar54 * fVar34 + fVar44;
        if (NAN(fVar44)) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
      }
      else {
        fVar44 = (fVar54 + 1.0) * fVar34 + fVar44;
        if (NAN(fVar44)) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
      }
      fVar41 = (1.0 - fVar34) * fVar31;
      if (fVar31 <= fVar33) {
        fVar41 = fVar34 * fVar33 + fVar41;
        if (NAN(fVar41)) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
      }
      else {
        fVar41 = fVar34 * (fVar33 + 1.0) + fVar41;
        if (NAN(fVar41)) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
      }
      fVar34 = fmodf(fVar44,1.0);
      fVar44 = (float)(~-(uint)(fVar34 < 0.0) & (uint)fVar34 |
                      (uint)(fVar34 + 1.0) & -(uint)(fVar34 < 0.0));
      fVar34 = fmodf(fVar41,1.0);
      fVar34 = (float)(~-(uint)(fVar34 < 0.0) & (uint)fVar34 |
                      (uint)(fVar34 + 1.0) & -(uint)(fVar34 < 0.0));
      if (bVar27) {
        pfVar26 = pfVar26 + 1;
        fVar41 = fVar50;
        fVar35 = unaff_XMM10_Da;
        unaff_XMM10_Da = fVar34;
        fVar50 = fVar44;
      }
      else {
        if (pfVar26 + 1 == local_3c8 + local_3c0) break;
        fVar50 = pfVar26[1];
        fVar41 = fVar53 * (1.0 - fVar50);
        if (fVar53 <= fVar54) {
          fVar41 = fVar54 * fVar50 + fVar41;
          if (NAN(fVar41)) {
                    // WARNING: Subroutine does not return
            core::panicking::panic();
          }
        }
        else {
          fVar41 = (fVar54 + 1.0) * fVar50 + fVar41;
          if (NAN(fVar41)) {
                    // WARNING: Subroutine does not return
            core::panicking::panic();
          }
        }
        fVar35 = (1.0 - fVar50) * fVar31;
        if (fVar31 <= fVar33) {
          fVar35 = fVar50 * fVar33 + fVar35;
          if (NAN(fVar35)) {
                    // WARNING: Subroutine does not return
            core::panicking::panic();
          }
        }
        else {
          fVar35 = fVar50 * (fVar33 + 1.0) + fVar35;
          if (NAN(fVar35)) {
                    // WARNING: Subroutine does not return
            core::panicking::panic();
          }
        }
        fVar50 = fmodf(fVar41,1.0);
        fVar51 = fmodf(fVar35,1.0);
        pfVar26 = pfVar26 + 2;
        fVar41 = fVar44;
        fVar35 = fVar34;
        unaff_XMM10_Da =
             (float)(~-(uint)(fVar51 < 0.0) & (uint)fVar51 |
                    (uint)(fVar51 + 1.0) & -(uint)(fVar51 < 0.0));
        fVar50 = (float)(~-(uint)(fVar50 < 0.0) & (uint)fVar50 |
                        (uint)(fVar50 + 1.0) & -(uint)(fVar50 < 0.0));
      }
      local_1d8._0_4_ = fVar41;
      if ((fVar41 < 0.0) || (1.0 < fVar41)) {
        local_128 = &local_1a8;
        local_1a8 = &local_1d8;
        fStack_1a0 = 6.774544;
        fStack_19c = 1.4013e-45;
        local_138 = &anon_b35f6c505c1858eb9906e98b629952b4_3_llvm_7383313424677244125;
        uStack_130 = 1;
        local_118 = 0;
        uStack_120 = 1;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
      if (NAN(fVar41)) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      country_core::systems::wall::valid_enclosure::ShapeParameters::get_pos_at_u();
      local_138 = (undefined **)CONCAT44(fVar41,extraout_XMM0_Da_07);
      ::utils::swizzlers::impl$1::xvy();
      local_1d8._0_4_ = fVar50;
      if ((fVar50 < 0.0) || (1.0 < fVar50)) {
        local_1a8 = &local_1d8;
        fStack_1a0 = 6.774544;
        fStack_19c = 1.4013e-45;
        local_138 = &anon_b35f6c505c1858eb9906e98b629952b4_3_llvm_7383313424677244125;
        uStack_130 = 1;
        local_118 = 0;
        uStack_120 = 1;
        local_128 = &local_1a8;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
      if (NAN(fVar50)) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      fVar34 = fVar50;
      country_core::systems::wall::valid_enclosure::ShapeParameters::get_pos_at_u();
      local_138 = (undefined **)CONCAT44(fVar34,extraout_XMM0_Da_08);
      ::utils::swizzlers::impl$1::xvy();
      fVar34 = (float)local_220;
      fVar44 = (float)((ulonglong)local_220 >> 0x20);
      local_1d8._0_4_ = fVar35;
      if ((fVar35 < 0.0) || (1.0 < fVar35)) {
        local_1a8 = &local_1d8;
        fStack_1a0 = 6.774544;
        fStack_19c = 1.4013e-45;
        local_138 = &anon_b35f6c505c1858eb9906e98b629952b4_3_llvm_7383313424677244125;
        uStack_130 = 1;
        local_118 = 0;
        uStack_120 = 1;
        local_128 = &local_1a8;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
      if (NAN(fVar35)) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      fVar41 = fVar35;
      country_core::systems::wall::valid_enclosure::ShapeParameters::get_pos_at_u();
      local_138 = (undefined **)CONCAT44(fVar41,extraout_XMM0_Da_09);
      ::utils::swizzlers::impl$1::xvy();
      local_1d8._0_4_ = unaff_XMM10_Da;
      if ((unaff_XMM10_Da < 0.0) || (1.0 < unaff_XMM10_Da)) {
        local_1a8 = &local_1d8;
        fStack_1a0 = 6.774544;
        fStack_19c = 1.4013e-45;
        local_138 = &anon_b35f6c505c1858eb9906e98b629952b4_3_llvm_7383313424677244125;
        uStack_130 = 1;
        local_118 = 0;
        uStack_120 = 1;
        local_128 = &local_1a8;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
      if (NAN(unaff_XMM10_Da)) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      fVar41 = unaff_XMM10_Da;
      country_core::systems::wall::valid_enclosure::ShapeParameters::get_pos_at_u();
      local_138 = (undefined **)CONCAT44(fVar41,extraout_XMM0_Da_10);
      ::utils::swizzlers::impl$1::xvy();
      local_2c8 = (float)local_230;
      fStack_2c4 = (float)((ulonglong)local_230 >> 0x20);
      fVar41 = fVar34 * 0.5 + local_2c8 * 0.5;
      fVar55 = fVar44 * 0.5 + fStack_2c4 * 0.5;
      fVar51 = local_218 * 0.5 + local_228 * 0.5;
      local_164 = local_238;
      local_168 = local_248;
      local_1e8 = local_250;
      uStack_1e0 = 0;
      if (local_1f0 == 0) {
        local_1d8._0_4_ = fVar32;
        if ((local_208 & 1) == 0) {
          local_1a8 = &local_1d8;
          fStack_1a0 = 6.774544;
          fStack_19c = 1.4013e-45;
          local_138 = &anon_b35f6c505c1858eb9906e98b629952b4_3_llvm_7383313424677244125;
          uStack_130 = 1;
          local_118 = 0;
          uStack_120 = 1;
          local_128 = &local_1a8;
                    // WARNING: Subroutine does not return
          core::panicking::panic_fmt();
        }
        if (NAN(fVar32)) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
        fVar37 = fVar32;
        country_core::systems::wall::valid_enclosure::ShapeParameters::get_pos_at_u();
        local_1d8._0_4_ = fVar54;
        if ((local_204 & 1) == 0) {
          local_1a8 = &local_1d8;
          fStack_1a0 = 6.774544;
          fStack_19c = 1.4013e-45;
          local_138 = &anon_b35f6c505c1858eb9906e98b629952b4_3_llvm_7383313424677244125;
          uStack_130 = 1;
          local_118 = 0;
          uStack_120 = 1;
          local_128 = &local_1a8;
                    // WARNING: Subroutine does not return
          core::panicking::panic_fmt();
        }
        if (NAN(fVar54)) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
        fVar38 = fVar54;
        country_core::systems::wall::valid_enclosure::ShapeParameters::get_pos_at_u();
        if ((local_200 & 1) == 0) {
          local_1a8 = &local_1d8;
          fStack_1a0 = 6.774544;
          fStack_19c = 1.4013e-45;
          local_138 = &anon_b35f6c505c1858eb9906e98b629952b4_3_llvm_7383313424677244125;
          uStack_130 = 1;
          local_118 = 0;
          uStack_120 = 1;
          local_128 = &local_1a8;
                    // WARNING: Subroutine does not return
          core::panicking::panic_fmt();
        }
        if (NAN(extraout_XMM0_Da_05)) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
        fVar42 = extraout_XMM0_Da_05;
        local_1d8._0_4_ = extraout_XMM0_Da_05;
        country_core::systems::wall::valid_enclosure::ShapeParameters::get_pos_at_u();
        fVar43 = (extraout_XMM0_Da_12 * 0.5 + extraout_XMM0_Da_11 * 0.5) - extraout_XMM0_Da_13;
        fVar42 = (fVar38 * 0.5 + fVar37 * 0.5) - fVar42;
        fVar37 = SQRT(fVar42 * fVar42 + fVar43 * fVar43) + 0.1;
        if (fVar37 <= 0.3) {
          fVar37 = 0.3;
        }
        country_core::systems::wall::valid_enclosure::ShapeParameters::get_pos_at_u();
        local_138 = (undefined **)CONCAT44(fVar35,extraout_XMM0_Da_14);
        ::utils::swizzlers::impl$1::xvy();
        fVar35 = unaff_XMM10_Da;
        country_core::systems::wall::valid_enclosure::ShapeParameters::get_pos_at_u();
        local_138 = (undefined **)CONCAT44(fVar35,extraout_XMM0_Da_15);
        ::utils::swizzlers::impl$1::xvy();
        fVar35 = (local_398 * 0.5 + local_38c * 0.5) - fVar41;
        fVar38 = (local_390 * 0.5 + local_384 * 0.5) - fVar51;
        fVar42 = 1.0 / SQRT(fVar38 * fVar38 + fVar35 * fVar35);
        local_138 = (undefined **)CONCAT44(fVar42 * fVar38,fVar42 * fVar35);
        ::utils::swizzlers::impl$0::x0y();
        ::utils::halver::impl$7::to_f16();
        ::utils::halver::impl$5::to_f16();
        uVar29 = local_f8 + 0xa0761d6478bd642f;
        auVar46._8_8_ = 0;
        auVar46._0_8_ = uVar29 ^ 0xe7037ed1a0b428db;
        auVar13._8_8_ = 0;
        auVar13._0_8_ = uVar29;
        uVar24 = SUB168(auVar46 * auVar13,8) ^ SUB168(auVar46 * auVar13,0);
        if ((uint)((int)uVar24 * 200) < 0x60) {
          local_f8 = local_f8 + 0x40ec3ac8f17ac85e;
          do {
            uVar29 = uVar29 + 0xa0761d6478bd642f;
            auVar6._8_8_ = 0;
            auVar6._0_8_ = local_f8 ^ 0xe7037ed1a0b428db;
            auVar14._8_8_ = 0;
            auVar14._0_8_ = local_f8;
            uVar24 = SUB168(auVar6 * auVar14,8) ^ SUB168(auVar6 * auVar14,0);
            local_f8 = local_f8 + 0xa0761d6478bd642f;
          } while ((uint)((int)uVar24 * 200) < 0x60);
        }
        local_f8 = uVar29;
        iStack_104 = (int)((uVar24 & 0xffffffff) * 200 >> 0x20) + -100;
        local_138 = (undefined **)
                    CONCAT44(fVar37 * (float)((ulonglong)local_370 >> 0x20) * 0.5 + fVar55,
                             fVar37 * (float)local_370 * 0.5 + fVar41);
        uStack_130 = (ulonglong)(uint)(fVar37 * local_368 * 0.5 + fVar51);
        local_128 = (undefined8 *)0x7bff00003c000000;
        uStack_120 = extraout_RAX_02 & 0xffffffffffff;
        uStack_110 = local_154;
        uStack_10c = 0;
        uStack_108 = 0x3f800000;
        uStack_100 = 0;
        country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
        push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
      }
      local_328 = (float)local_240;
      fStack_324 = (float)((ulonglong)local_240 >> 0x20);
      local_20c = local_164 * 0.5;
      fVar42 = (float)local_1e8 * 0.5;
      fVar43 = local_1e8._4_4_ * 0.5;
      fVar47 = fStack_324 * 0.5 + fVar43;
      fVar35 = fVar34 - local_328;
      fVar37 = fVar44 - fStack_324;
      fVar48 = local_218 - local_164;
      fVar52 = SQRT(fVar48 * fVar48 + fVar37 * fVar37 + fVar35 * fVar35);
      fVar41 = (local_328 * 0.5 + fVar42) - fVar41;
      fVar38 = fVar47 - fVar55;
      fVar51 = (local_20c + local_168 * 0.5) - fVar51;
      fVar47 = fVar47 - fVar55;
      auVar45._0_4_ =
           (fStack_2c4 - fVar44) * (fStack_2c4 - fVar44) +
           (local_2c8 - fVar34) * (local_2c8 - fVar34);
      auVar45._4_4_ = fVar47 * fVar47 + fVar41 * fVar41;
      auVar45._8_8_ = 0;
      auVar49._0_4_ = (local_228 - local_218) * (local_228 - local_218) + auVar45._0_4_;
      auVar49._4_4_ = fVar51 * fVar51 + auVar45._4_4_;
      auVar49._8_4_ = 0;
      auVar49._12_4_ = fVar38 * fVar38 + 0.0;
      auVar46 = sqrtps(auVar45,auVar49);
      divps(___xmm_00000000000000003f8000003f800000,auVar46);
      fVar51 = fVar34 + local_2c8 + local_328 + (float)local_1e8;
      fVar55 = fVar44 + fStack_2c4 + fStack_324 + local_1e8._4_4_;
      fVar38 = local_218 + local_228 + local_164 + local_168;
      local_164 = local_168 * 0.5;
      fastrand::Rng::f32();
      fVar41 = (extraout_XMM0_Da_16 * -10.0 -
               ((1.0 - extraout_XMM0_Da_16) + (1.0 - extraout_XMM0_Da_16))) * 0.017453292 * 0.5;
      sinf(fVar41);
      cosf(fVar41);
      ::utils::halver::impl$7::to_f16();
      ::utils::halver::impl$5::to_f16();
      uVar29 = local_f8 + 0xa0761d6478bd642f;
      auVar7._8_8_ = 0;
      auVar7._0_8_ = uVar29 ^ 0xe7037ed1a0b428db;
      auVar15._8_8_ = 0;
      auVar15._0_8_ = uVar29;
      uVar24 = SUB168(auVar7 * auVar15,8) ^ SUB168(auVar7 * auVar15,0);
      if ((uint)((int)uVar24 * 200) < 0x60) {
        local_f8 = local_f8 + 0x40ec3ac8f17ac85e;
        do {
          uVar29 = uVar29 + 0xa0761d6478bd642f;
          auVar8._8_8_ = 0;
          auVar8._0_8_ = local_f8 ^ 0xe7037ed1a0b428db;
          auVar16._8_8_ = 0;
          auVar16._0_8_ = local_f8;
          uVar24 = SUB168(auVar8 * auVar16,8) ^ SUB168(auVar8 * auVar16,0);
          local_f8 = local_f8 + 0xa0761d6478bd642f;
        } while ((uint)((int)uVar24 * 200) < 0x60);
      }
      local_f8 = uVar29;
      iStack_104 = (int)((uVar24 & 0xffffffff) * 200 >> 0x20) + -100;
      local_138 = (undefined **)CONCAT44(fVar55 * 0.25,fVar51 * 0.25);
      uStack_130 = (ulonglong)(uint)(fVar38 * 0.25);
      local_128 = (undefined8 *)0x7bff00003c000000;
      uStack_120 = extraout_RAX_03 & 0xffffffffffff;
      uStack_110 = local_154;
      uStack_10c = 0;
      uStack_108 = 0x3f800000;
      uStack_100 = 0;
      country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
      push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
      local_1d8 = &local_210;
      local_1d0 = &local_1b0;
      local_1c0 = &local_f8;
      if (lVar30 == 0) {
        fVar41 = 1.0 / fVar52;
        fStack_1a0 = fVar48 * fVar41;
        local_1a8 = (undefined8 *)CONCAT44(fVar37 * fVar41,fVar35 * fVar41);
        ::utils::rotation_from_normal();
        fVar41 = (float)local_138 * 0.0;
        fVar35 = (float)local_138 * 0.70710677;
        fVar51 = local_138._4_4_ * 0.70710677;
        fVar55 = local_138._4_4_ * 0.0;
        local_138 = (undefined **)
                    CONCAT44((float)uStack_130 * 0.70710677 * 1.0 +
                             local_138._4_4_ * 0.70710677 * 1.0 +
                             (float)local_138 * 0.0 * -1.0 + uStack_130._4_4_ * 0.0,
                             (float)uStack_130 * 0.0 * -1.0 + local_138._4_4_ * 0.0 * 1.0 +
                             (float)local_138 * 0.70710677 * 1.0 + uStack_130._4_4_ * 0.70710677);
        uStack_130 = CONCAT44((float)uStack_130 * 0.0 * -1.0 + fVar55 * -1.0 +
                              fVar35 * -1.0 + uStack_130._4_4_ * 0.70710677,
                              (float)uStack_130 * 0.70710677 * 1.0 + fVar51 * -1.0 +
                              fVar41 * 1.0 + uStack_130._4_4_ * 0.0);
        ::utils::halver::impl$7::to_f16();
        local_1a8 = (undefined8 *)CONCAT44(fVar52 + 0.16,0x3e19999a);
        fStack_1a0 = 0.2;
        ::utils::halver::impl$5::to_f16();
        uVar29 = local_f8 + 0xa0761d6478bd642f;
        auVar9._8_8_ = 0;
        auVar9._0_8_ = uVar29 ^ 0xe7037ed1a0b428db;
        auVar17._8_8_ = 0;
        auVar17._0_8_ = uVar29;
        uVar24 = SUB168(auVar9 * auVar17,8) ^ SUB168(auVar9 * auVar17,0);
        if ((uint)((int)uVar24 * 200) < 0x60) {
          local_f8 = local_f8 + 0x40ec3ac8f17ac85e;
          do {
            uVar29 = uVar29 + 0xa0761d6478bd642f;
            auVar10._8_8_ = 0;
            auVar10._0_8_ = local_f8 ^ 0xe7037ed1a0b428db;
            auVar18._8_8_ = 0;
            auVar18._0_8_ = local_f8;
            uVar24 = SUB168(auVar10 * auVar18,8) ^ SUB168(auVar10 * auVar18,0);
            local_f8 = local_f8 + 0xa0761d6478bd642f;
          } while ((uint)((int)uVar24 * 200) < 0x60);
        }
        local_f8 = uVar29;
        local_138 = (undefined **)
                    CONCAT44(fVar44 * 0.5 + fStack_324 * 0.5,fVar34 * 0.5 + local_328 * 0.5);
        iStack_104 = (int)((uVar24 & 0xffffffff) * 200 >> 0x20) + -100;
        uStack_130 = (ulonglong)(uint)(local_218 * 0.5 + local_20c);
        local_128 = (undefined8 *)0x7bff00003c000000;
        uStack_120 = extraout_RAX_05 & 0xffffffffffff;
        uStack_110 = local_154;
        uStack_10c = 0;
        uStack_108 = 0x3f800000;
        uStack_100 = 0;
        local_118 = extraout_RAX_04;
        country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
        push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
        if (in_stack_00000048 != '\0') {
          uStack_190 = uStack_190 & 0xffffffffffffff00;
          fStack_19c = local_328;
          fStack_198 = fStack_324;
          assemble_dormer_roof_tiles::
          closure_3<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env_7>
                    ();
          uStack_180 = CONCAT44(uStack_10c,uStack_110);
          local_178 = CONCAT44(iStack_104,uStack_108);
          local_1a8 = local_138;
          fStack_1a0 = (float)uStack_130;
          fStack_19c = (float)(uStack_130 >> 0x20);
          fStack_198 = SUB84(local_128,0);
          uStack_194 = (undefined4)((ulonglong)local_128 >> 0x20);
          uStack_190 = uStack_120;
          local_188 = local_118;
          uStack_170 = uStack_100;
          country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
          push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
        }
      }
      if (lVar30 == local_3c0 + -2) {
        local_168 = local_228 - local_168;
        fVar34 = local_2c8 - (float)local_1e8;
        fVar44 = fStack_2c4 - local_1e8._4_4_;
        fVar51 = SQRT(fVar44 * fVar44 + fVar34 * fVar34 + local_168 * local_168);
        fStack_1a0 = 1.0 / fVar51;
        local_1a8 = (undefined8 *)CONCAT44(fStack_1a0 * fVar44,fStack_1a0 * fVar34);
        fStack_1a0 = fStack_1a0 * local_168;
        ::utils::rotation_from_normal();
        fVar34 = (float)local_138 * 0.0;
        fVar44 = (float)local_138 * 0.70710677;
        fVar41 = local_138._4_4_ * 0.70710677;
        fVar35 = local_138._4_4_ * 0.0;
        local_138 = (undefined **)
                    CONCAT44((float)uStack_130 * 0.70710677 * 1.0 +
                             local_138._4_4_ * 0.70710677 * 1.0 +
                             (float)local_138 * 0.0 * -1.0 + uStack_130._4_4_ * 0.0,
                             (float)uStack_130 * 0.0 * -1.0 + local_138._4_4_ * 0.0 * 1.0 +
                             (float)local_138 * 0.70710677 * 1.0 + uStack_130._4_4_ * 0.70710677);
        uStack_130 = CONCAT44((float)uStack_130 * 0.0 * -1.0 + fVar35 * -1.0 +
                              fVar44 * -1.0 + uStack_130._4_4_ * 0.70710677,
                              (float)uStack_130 * 0.70710677 * 1.0 + fVar41 * -1.0 +
                              fVar34 * 1.0 + uStack_130._4_4_ * 0.0);
        ::utils::halver::impl$7::to_f16();
        local_1a8 = (undefined8 *)CONCAT44(fVar51 + 0.16,0x3e19999a);
        fStack_1a0 = 0.2;
        ::utils::halver::impl$5::to_f16();
        uVar29 = local_f8 + 0xa0761d6478bd642f;
        auVar11._8_8_ = 0;
        auVar11._0_8_ = uVar29 ^ 0xe7037ed1a0b428db;
        auVar19._8_8_ = 0;
        auVar19._0_8_ = uVar29;
        uVar24 = SUB168(auVar11 * auVar19,8) ^ SUB168(auVar11 * auVar19,0);
        if ((uint)((int)uVar24 * 200) < 0x60) {
          local_f8 = local_f8 + 0x40ec3ac8f17ac85e;
          do {
            uVar29 = uVar29 + 0xa0761d6478bd642f;
            auVar12._8_8_ = 0;
            auVar12._0_8_ = local_f8 ^ 0xe7037ed1a0b428db;
            auVar20._8_8_ = 0;
            auVar20._0_8_ = local_f8;
            uVar24 = SUB168(auVar12 * auVar20,8) ^ SUB168(auVar12 * auVar20,0);
            local_f8 = local_f8 + 0xa0761d6478bd642f;
          } while ((uint)((int)uVar24 * 200) < 0x60);
        }
        local_f8 = uVar29;
        local_138 = (undefined **)CONCAT44(fStack_2c4 * 0.5 + fVar43,local_2c8 * 0.5 + fVar42);
        iStack_104 = (int)((uVar24 & 0xffffffff) * 200 >> 0x20) + -100;
        uStack_130 = (ulonglong)(uint)(local_228 * 0.5 + local_164);
        local_128 = (undefined8 *)0x7bff00003c000000;
        uStack_120 = extraout_RAX_07 & 0xffffffffffff;
        uStack_110 = local_154;
        uStack_10c = 0;
        uStack_108 = 0x3f800000;
        uStack_100 = 0;
        local_118 = extraout_RAX_06;
        country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
        push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
        if (in_stack_00000048 != '\0') {
          fStack_19c = SUB84(local_250,0);
          fStack_198 = (float)((ulonglong)local_250 >> 0x20);
          uStack_190 = CONCAT71(uStack_190._1_7_,1);
          assemble_dormer_roof_tiles::
          closure_3<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env_7>
                    ();
          uStack_180 = CONCAT44(uStack_10c,uStack_110);
          local_178 = CONCAT44(iStack_104,uStack_108);
          local_1a8 = local_138;
          fStack_1a0 = (float)uStack_130;
          fStack_19c = (float)(uStack_130 >> 0x20);
          fStack_198 = SUB84(local_128,0);
          uStack_194 = (undefined4)((ulonglong)local_128 >> 0x20);
          uStack_190 = uStack_120;
          local_188 = local_118;
          uStack_170 = uStack_100;
          country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
          push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
        }
      }
      lVar30 = lVar30 + 1;
      bVar27 = true;
    }
    if (local_3d0 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    bVar27 = true;
    unaff_XMM11_Da = local_1fc;
    local_1f0 = lVar22;
  }
  if (local_2f0 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (lVar25 != 0) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_08 == 1) {
      plVar23 = extraout_RAX_08 + 1;
    }
    else if ((*extraout_RAX_08 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar23 = extraout_RAX_10, extraout_RAX_10 == (longlong *)0x0)) {
LAB_141290100:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar23 != 0) {
LAB_14129010c:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar23 = -1;
    local_1e8 = plVar23;
    puffin::ThreadProfiler::end_scope();
    *local_1e8 = *local_1e8 + 1;
  }
  return;
}

