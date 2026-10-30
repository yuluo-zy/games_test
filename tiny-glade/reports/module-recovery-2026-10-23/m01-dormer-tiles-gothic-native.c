
void __cdecl
system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::
assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env_6>
          (void)

{
  ulonglong uVar1;
  longlong lVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  longlong lVar5;
  undefined1 auVar6 [16];
  undefined1 auVar7 [16];
  undefined1 auVar8 [16];
  undefined1 auVar9 [16];
  undefined1 auVar10 [16];
  undefined1 auVar11 [16];
  undefined1 auVar12 [16];
  undefined1 auVar13 [16];
  uint extraout_EAX;
  uint extraout_EAX_00;
  ulonglong extraout_RAX;
  float *pfVar14;
  float *pfVar15;
  ulonglong extraout_RAX_00;
  bool bVar16;
  longlong in_RCX;
  float *in_RDX;
  ulonglong uVar17;
  float *pfVar18;
  float *pfVar19;
  ulonglong uVar20;
  longlong lVar21;
  float *pfVar22;
  float *pfVar23;
  longlong lVar24;
  undefined4 extraout_XMM0_Da;
  undefined4 extraout_XMM0_Da_00;
  undefined4 uVar25;
  float fVar26;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
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
  float fVar41;
  float fVar42;
  uint in_stack_00000028;
  float local_358;
  float fStack_354;
  float fStack_350;
  float fStack_34c;
  float local_348;
  float fStack_344;
  float fStack_340;
  int local_324;
  ulonglong uStack_318;
  undefined8 local_310;
  longlong local_1f0;
  float *local_1e8;
  longlong local_1e0;
  undefined8 local_1d8;
  float local_1d0;
  float *local_1b0;
  code *local_1a8;
  float *local_1a0;
  float *local_198;
  float *local_190;
  longlong local_188;
  float *local_180;
  float *local_178;
  float local_16c;
  float *local_168;
  float *local_160;
  float *local_158;
  float local_14c;
  undefined8 local_148;
  ulonglong uStack_140;
  float **local_138;
  ulonglong local_130;
  undefined8 local_128;
  float local_120;
  undefined8 local_11c;
  int local_114;
  undefined8 local_110;
  float *local_108;
  float local_100;
  float local_fc;
  ulonglong local_f8;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  if (*(longlong *)(in_RCX + 0x10) == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  uVar3 = **(undefined8 **)(in_RCX + 8);
  fVar31 = (float)uVar3;
  fVar32 = (float)((ulonglong)uVar3 >> 0x20);
  fVar27 = *(float *)(*(undefined8 **)(in_RCX + 8) + 1);
  country_core::resources::roofs::roof_shape::Roof::roof_shape_at_intersection_y();
  if (local_324 != 2) {
    pfVar19 = (float *)(ulonglong)in_stack_00000028;
    fVar26 = fVar31;
    if (local_324 == 0) {
      uStack_140 = uStack_318;
      ::utils::geometry::circle::Circle2d::closest_point_on_perimeter();
      uVar25 = extraout_XMM0_Da_00;
    }
    else {
      local_138 = (float **)local_310;
      uStack_140 = uStack_318;
      ::utils::geometry::rectangle::Rectangle2d::closest_point_on_perimeter();
      uVar25 = extraout_XMM0_Da;
    }
    local_148 = (undefined **)CONCAT44(fVar26,uVar25);
    ::utils::swizzlers::impl$1::xvy();
    country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::
    transform();
    fVar26 = SQRT(fStack_340 * 10.0 * fStack_340 * 10.0 +
                  fStack_344 * 10.0 * fStack_344 * 10.0 + local_348 * 10.0 * local_348 * 10.0);
    local_f8 = (longlong)(fVar26 - 9.223372e+18) & (longlong)fVar26 >> 0x3f | (longlong)fVar26;
    if (fVar26 < 0.0) {
      local_f8 = 0;
    }
    if (1.8446743e+19 < fVar26) {
      local_f8 = 0xffffffffffffffff;
    }
    fVar26 = fVar27 - local_1d0;
    fVar29 = (float)((ulonglong)local_1d8 >> 0x20);
    fVar38 = fVar31 - (float)local_1d8;
    fVar39 = fVar32 - fVar29;
    fVar42 = SQRT(fVar39 * fVar39 + fVar38 * fVar38 + fVar26 * fVar26);
    roundf(fVar42 / 0.46875);
    local_fc = fVar42;
    ::utils::random_splits();
    local_108 = local_180;
    local_14c = *in_RDX;
    if (local_178 != (float *)0x0) {
      pfVar14 = local_180 + (longlong)local_178;
      bVar16 = true;
      pfVar19 = (float *)&local_148;
      pfVar15 = local_178;
      pfVar18 = local_180;
      do {
        pfVar23 = pfVar18 + 1;
        if (bVar16) {
          if (pfVar23 == pfVar14) break;
          pfVar22 = pfVar18 + 2;
          pfVar15 = pfVar18;
          pfVar18 = pfVar23;
          pfVar23 = pfVar22;
        }
        fVar42 = *pfVar15;
        fVar28 = *pfVar18;
        fVar40 = (fVar42 + fVar28) * 0.5;
        ::utils::halver::impl$7::to_f16();
        local_148 = (undefined **)CONCAT44((fVar28 - fVar42) * local_fc * 1.2,0x3dcccccd);
        uStack_140 = CONCAT44(uStack_140._4_4_,0x3e4ccccd);
        ::utils::halver::impl$2::to_f16();
        uVar20 = local_f8 + 0xa0761d6478bd642f;
        auVar6._8_8_ = 0;
        auVar6._0_8_ = uVar20 ^ 0xe7037ed1a0b428db;
        auVar10._8_8_ = 0;
        auVar10._0_8_ = uVar20;
        uVar17 = SUB168(auVar6 * auVar10,8) ^ SUB168(auVar6 * auVar10,0);
        if ((uint)((int)uVar17 * 200) < 0x60) {
          local_f8 = local_f8 + 0x40ec3ac8f17ac85e;
          do {
            uVar20 = uVar20 + 0xa0761d6478bd642f;
            auVar7._8_8_ = 0;
            auVar7._0_8_ = local_f8 ^ 0xe7037ed1a0b428db;
            auVar11._8_8_ = 0;
            auVar11._0_8_ = local_f8;
            uVar17 = SUB168(auVar7 * auVar11,8) ^ SUB168(auVar7 * auVar11,0);
            local_f8 = local_f8 + 0xa0761d6478bd642f;
          } while ((uint)((int)uVar17 * 200) < 0x60);
        }
        local_f8 = uVar20;
        fVar42 = 1.0 - fVar40;
        local_148 = (undefined **)
                    CONCAT44(fVar40 * fVar29 + fVar42 * (fVar39 * 0.05 + fVar32),
                             fVar40 * (float)local_1d8 + fVar42 * (fVar38 * 0.05 + fVar31));
        local_114 = (int)((uVar17 & 0xffffffff) * 200 >> 0x20) + -100;
        uStack_140 = (ulonglong)(uint)(fVar40 * local_1d0 + (fVar26 * 0.05 + fVar27) * fVar42);
        local_138 = (float **)0x7bff00003c000000;
        local_130 = extraout_RAX & 0xffffffffffff;
        local_120 = local_14c;
        local_11c = 0x3f80000000000000;
        local_110 = 0;
        country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
        push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
        bVar16 = false;
        pfVar15 = pfVar18;
        in_RDX = pfVar18;
        pfVar18 = pfVar23;
      } while (pfVar23 != pfVar14);
    }
    if (local_188 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    ceilf(local_fc / 0.3);
    lVar21 = 0;
    ::utils::random_splits();
    fVar31 = fStack_34c * fStack_34c;
    fVar27 = fStack_350 * fStack_350 + fStack_354 * fStack_354 + local_358 * local_358;
    fVar32 = fStack_34c + fStack_34c;
    fVar26 = fStack_350 * 1.0 + fStack_354 * 0.0 + local_358 * 0.0;
    fVar26 = fVar26 + fVar26;
    local_158 = local_1e8;
    local_190 = local_1e8 + local_1e0;
    lVar24 = 0;
    do {
      lVar2 = in_RCX + lVar21;
      ceilf(*(float *)(lVar2 + 0x30) / 0.32000002);
      ::utils::random_splits();
      lVar21 = lVar21 + 0x38;
      fVar29 = *(float *)(&__real_3f800000bf800000 + (ulonglong)(lVar24 == 0) * 4);
      fVar39 = (float)((uint)fVar29 & 0x80000000);
      fVar38 = fStack_350 * fVar29 + fStack_354 * fVar39 + local_358 * fVar39;
      fVar38 = fVar38 + fVar38;
      fVar42 = (fStack_354 * fVar29 - fVar39 * fStack_350) * fVar32 +
               fVar38 * local_358 + fVar39 * (fVar31 - fVar27);
      fVar28 = (fStack_350 * fVar39 - fVar29 * local_358) * fVar32 +
               fVar38 * fStack_354 + fVar39 * (fVar31 - fVar27);
      fVar29 = (local_358 * fVar39 - fVar39 * fStack_354) * fVar32 +
               fVar38 * fStack_350 + fVar29 * (fVar31 - fVar27);
      local_160 = local_180;
      local_1a0 = local_180 + (longlong)local_178;
      bVar16 = true;
      pfVar14 = local_158;
      while (pfVar14 != local_190) {
        pfVar15 = pfVar14 + 1;
        if (bVar16) {
          if (pfVar15 == local_190) break;
          pfVar18 = pfVar14 + 2;
          in_RDX = pfVar14;
          pfVar14 = pfVar15;
          pfVar15 = pfVar18;
        }
        local_168 = pfVar15;
        fVar38 = (*in_RDX + *pfVar14) * 0.5;
        local_16c = (*pfVar14 - *in_RDX) * local_fc;
        fVar38 = (1.0 - fVar38) * 0.0 - fVar38 * local_fc;
        bVar16 = true;
        pfVar15 = local_160;
        pfVar18 = pfVar19;
        local_198 = pfVar14;
        if (local_160 != local_1a0) {
          do {
            pfVar14 = pfVar15 + 1;
            pfVar19 = pfVar15;
            if (bVar16) {
              pfVar19 = pfVar18;
              if (pfVar14 == local_1a0) break;
              pfVar19 = pfVar14;
              pfVar14 = pfVar15 + 2;
              pfVar18 = pfVar15;
            }
            pfVar15 = pfVar14;
            local_100 = *pfVar18;
            fVar39 = *pfVar19;
            if ((local_100 < 0.0) || (1.0 < local_100)) {
              local_1b0 = &local_100;
              local_1a8 = core::fmt::float::impl$5::fmt;
              local_148 = &anon_b35f6c505c1858eb9906e98b629952b4_3_llvm_7383313424677244125;
              uStack_140 = 1;
              local_128 = 0;
              local_138 = &local_1b0;
              local_130 = 1;
                    // WARNING: Subroutine does not return
              core::panicking::panic_fmt();
            }
            if (NAN(local_100)) {
                    // WARNING: Subroutine does not return
              core::panicking::panic();
            }
            ::utils::curve::Curve::
            get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                      ();
            uVar17 = (ulonglong)extraout_EAX;
            uVar20 = *(ulonglong *)(lVar2 + 0x10);
            if (uVar20 <= uVar17) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            uVar1 = uVar17 + 1;
            if (uVar20 <= uVar1) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            lVar5 = *(longlong *)(lVar2 + 8);
            fVar37 = 1.0 - extraout_XMM0_Da_01;
            fVar40 = *(float *)(lVar5 + 8 + uVar17 * 0xc);
            uVar3 = *(undefined8 *)(lVar5 + uVar17 * 0xc);
            uVar4 = *(undefined8 *)(lVar5 + uVar1 * 0xc);
            fVar30 = *(float *)(lVar5 + 8 + uVar1 * 0xc);
            local_100 = fVar39;
            if ((fVar39 < 0.0) || (1.0 < fVar39)) {
              local_1b0 = &local_100;
              local_1a8 = core::fmt::float::impl$5::fmt;
              local_148 = &anon_b35f6c505c1858eb9906e98b629952b4_3_llvm_7383313424677244125;
              uStack_140 = 1;
              local_128 = 0;
              local_138 = &local_1b0;
              local_130 = 1;
                    // WARNING: Subroutine does not return
              core::panicking::panic_fmt();
            }
            if (NAN(fVar39)) {
                    // WARNING: Subroutine does not return
              core::panicking::panic();
            }
            ::utils::curve::Curve::
            get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                      ();
            uVar17 = (ulonglong)extraout_EAX_00;
            if (uVar20 <= uVar17) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            uVar1 = uVar17 + 1;
            if (uVar20 <= uVar1) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            fVar33 = extraout_XMM0_Da_01 * (float)uVar4 + fVar37 * (float)uVar3;
            fVar34 = extraout_XMM0_Da_01 * (float)((ulonglong)uVar4 >> 0x20) +
                     fVar37 * (float)((ulonglong)uVar3 >> 0x20);
            fVar39 = fVar40 * fVar37 + fVar30 * extraout_XMM0_Da_01;
            fVar40 = 1.0 - extraout_XMM0_Da_02;
            fVar41 = *(float *)(lVar5 + 8 + uVar1 * 0xc) * extraout_XMM0_Da_02 +
                     fVar40 * *(float *)(lVar5 + 8 + uVar17 * 0xc);
            local_108 = (float *)CONCAT44(local_108._4_4_,fVar39);
            fVar39 = fVar41 - fVar39;
            uVar3 = *(undefined8 *)(lVar5 + uVar17 * 0xc);
            uVar4 = *(undefined8 *)(lVar5 + uVar1 * 0xc);
            fVar35 = (float)uVar4 * extraout_XMM0_Da_02 + (float)uVar3 * fVar40;
            fVar36 = (float)((ulonglong)uVar4 >> 0x20) * extraout_XMM0_Da_02 +
                     (float)((ulonglong)uVar3 >> 0x20) * fVar40;
            fVar40 = fVar35 - fVar33;
            fVar30 = fVar36 - fVar34;
            fVar37 = 1.0 / SQRT(fVar30 * fVar30 + fVar40 * fVar40 + fVar39 * fVar39);
            fVar40 = fVar37 * fVar40;
            fVar30 = fVar37 * fVar30;
            fVar37 = fVar37 * fVar39;
            fastrand::Rng::f32();
            fastrand::Rng::f32();
            fVar39 = ((1.0 - extraout_XMM0_Da_03) + (1.0 - extraout_XMM0_Da_03) +
                     extraout_XMM0_Da_03 * 5.0) * 0.017453292 * 0.5;
            sinf(fVar39);
            cosf(fVar39);
            ::utils::halver::impl$7::to_f16();
            fVar39 = extraout_XMM0_Da_04 * 0.2 + (1.0 - extraout_XMM0_Da_04) * 0.1;
            local_148 = (undefined **)
                        CONCAT44(SQRT((local_108._0_4_ - fVar41) * (local_108._0_4_ - fVar41) +
                                      (fVar34 - fVar36) * (fVar34 - fVar36) +
                                      (fVar33 - fVar35) * (fVar33 - fVar35)) + fVar39,local_16c);
            uStack_140 = CONCAT44(uStack_140._4_4_,0x3da3d70b);
            ::utils::halver::impl$2::to_f16();
            uVar20 = local_f8 + 0xa0761d6478bd642f;
            auVar8._8_8_ = 0;
            auVar8._0_8_ = uVar20 ^ 0xe7037ed1a0b428db;
            auVar12._8_8_ = 0;
            auVar12._0_8_ = uVar20;
            uVar17 = SUB168(auVar8 * auVar12,8) ^ SUB168(auVar8 * auVar12,0);
            if ((uint)((int)uVar17 * 200) < 0x60) {
              local_f8 = local_f8 + 0x40ec3ac8f17ac85e;
              do {
                uVar20 = uVar20 + 0xa0761d6478bd642f;
                auVar9._8_8_ = 0;
                auVar9._0_8_ = local_f8 ^ 0xe7037ed1a0b428db;
                auVar13._8_8_ = 0;
                auVar13._0_8_ = local_f8;
                uVar17 = SUB168(auVar9 * auVar13,8) ^ SUB168(auVar9 * auVar13,0);
                local_f8 = local_f8 + 0xa0761d6478bd642f;
              } while ((uint)((int)uVar17 * 200) < 0x60);
            }
            local_f8 = uVar20;
            local_114 = (int)((uVar17 & 0xffffffff) * 200 >> 0x20) + -100;
            local_148 = (undefined **)
                        CONCAT44((fVar37 * fVar42 - fVar40 * fVar29) * -0.080000006 * 0.5 +
                                 fVar30 * fVar39 * 0.5 + fVar36 * 0.5 + fVar34 * 0.5 +
                                 fVar38 * (fVar26 * fStack_354 + (fVar31 - fVar27) * 0.0 +
                                          (local_358 * -1.0 + fStack_350 * 0.0) * fVar32),
                                 (fVar30 * fVar29 - fVar37 * fVar28) * -0.080000006 * 0.5 +
                                 fVar40 * fVar39 * 0.5 + fVar35 * 0.5 + fVar33 * 0.5 +
                                 fVar38 * (fVar26 * local_358 + (fVar31 - fVar27) * 0.0 +
                                          (fStack_350 * -0.0 + fStack_354 * 1.0) * fVar32));
            uStack_140 = (ulonglong)
                         (uint)((fVar28 * fVar40 - fVar30 * fVar42) * -0.080000006 * 0.5 +
                                fVar37 * fVar39 * 0.5 + fVar41 * 0.5 + local_108._0_4_ * 0.5 +
                               fVar38 * (fVar26 * fStack_350 + (fVar31 - fVar27) * 1.0 +
                                        (fStack_354 * -0.0 + local_358 * 0.0) * fVar32));
            local_138 = (float **)0x7bff00003c000000;
            local_130 = extraout_RAX_00 & 0xffffffffffff;
            local_120 = local_14c;
            local_11c = 0x3f80000000000000;
            local_110 = 0;
            country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
            push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
            bVar16 = false;
            pfVar18 = pfVar19;
          } while (pfVar15 != local_1a0);
        }
        bVar16 = false;
        pfVar14 = local_168;
        in_RDX = local_198;
      }
      if (local_188 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      lVar24 = lVar24 + 1;
    } while (lVar21 != 0x70);
    if (local_1f0 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  return;
}

