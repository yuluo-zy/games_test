// 0x1421b67a0 construct_timberframe_circle
// sig: void __cdecl construct_timberframe_circle(void)
// size: 4542


void __cdecl timberframe::timberframe_circle::construct_timberframe_circle(void)

{
  byte *pbVar1;
  byte *pbVar2;
  longlong lVar3;
  ulonglong *puVar4;
  ulonglong uVar5;
  longlong lVar6;
  undefined4 *puVar7;
  byte *pbVar8;
  byte *pbVar9;
  byte extraout_AL;
  byte extraout_AL_00;
  byte extraout_AL_01;
  int iVar10;
  undefined8 *extraout_RAX;
  longlong lVar11;
  undefined8 *extraout_RAX_00;
  undefined8 *in_RCX;
  ulonglong uVar12;
  longlong *in_RDX;
  ulonglong uVar13;
  ulonglong uVar14;
  longlong lVar15;
  longlong *in_R9;
  byte *pbVar16;
  byte *pbVar17;
  byte *pbVar18;
  float **ppfVar19;
  float **ppfVar20;
  bool bVar21;
  float extraout_XMM0_Da;
  float fVar22;
  float extraout_XMM0_Da_00;
  undefined4 extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float fVar23;
  float extraout_XMM0_Da_03;
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
  longlong *in_stack_00000028;
  longlong local_2e0;
  float **local_2d8;
  longlong local_2d0;
  undefined8 local_2c8;
  float local_2c0;
  float local_24c;
  float fStack_248;
  float local_244;
  float local_240;
  float local_23c;
  float local_230;
  undefined4 local_22c;
  float local_228;
  float fStack_224;
  float fStack_220;
  float fStack_21c;
  undefined8 uStack_218;
  undefined8 uStack_210;
  undefined8 local_208;
  undefined8 uStack_200;
  undefined8 local_1f8;
  float fStack_1f0;
  float fStack_1ec;
  undefined4 uStack_1e8;
  float fStack_1e4;
  float *local_1d8;
  code *pcStack_1d0;
  float **local_1c8;
  float **ppfStack_1c0;
  int local_1b8;
  undefined4 uStack_1b4;
  float fStack_1b0;
  float fStack_1ac;
  undefined8 local_1a8;
  float local_1a0;
  byte *local_198;
  float *local_190;
  undefined8 local_188;
  float fStack_180;
  float fStack_17c;
  undefined8 local_178;
  float fStack_170;
  float fStack_16c;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined4 local_140;
  undefined4 local_13c;
  undefined **local_138;
  undefined8 local_130;
  undefined8 local_128;
  undefined8 local_120;
  undefined8 uStack_118;
  float **local_108;
  float **local_100;
  float local_f4;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX != (undefined8 *)0x0) {
    if ((extraout_RAX[3] != 0x3d) ||
       (iVar10 = memcmp("country_core::resources::roofs::roof_buffers::WoodenPlankSsbo/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs"
                        ,(void *)extraout_RAX[2],0x3d), iVar10 != 0)) {
      local_188 = (undefined **)0x0;
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    lVar11 = (**(code **)(extraout_RAX[1] + 0x18))(*extraout_RAX);
    if (lVar11 != 0) {
      if (*(longlong *)*in_RDX == 0) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      lVar3 = ((longlong *)*in_RDX)[1];
      if (lVar3 == 0) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      LOCK();
      bVar21 = *(char *)(lVar3 + 0x18) == '\0';
      if (bVar21) {
        *(char *)(lVar3 + 0x18) = '\x01';
      }
      UNLOCK();
      if (!bVar21) {
        parking_lot::raw_mutex::RawMutex::lock_slow();
      }
      parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
      if ((*(longlong *)(lVar3 + 0x10) * 4 + *(longlong *)(lVar3 + 8) == 4 ||
           *(longlong *)(lVar3 + 0x10) == 0) && (*(longlong *)(lVar11 + 0x28) == 0)) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      puVar4 = (ulonglong *)*in_RCX;
      lVar11 = in_RCX[1];
      uVar5 = *puVar4;
      uVar14 = 0;
      uVar13 = uVar5 - *(ulonglong *)(lVar11 + 0x18);
      if (uVar5 < *(ulonglong *)(lVar11 + 0x18)) {
        uVar13 = uVar14;
      }
      uVar12 = uVar5 - *(ulonglong *)(lVar11 + 0x38);
      if (uVar5 < *(ulonglong *)(lVar11 + 0x38)) {
        uVar12 = uVar14;
      }
      pbVar16 = (byte *)(uVar13 * 0x28 + *(longlong *)(lVar11 + 8));
      uVar5 = *(ulonglong *)(lVar11 + 0x10) - uVar13;
      if (*(ulonglong *)(lVar11 + 0x10) < uVar13) {
        pbVar16 = &DAT_00000008;
        uVar5 = uVar14;
      }
      pbVar18 = (byte *)(uVar12 * 0x28 + *(longlong *)(lVar11 + 0x28));
      uVar13 = *(ulonglong *)(lVar11 + 0x30) - uVar12;
      if (*(ulonglong *)(lVar11 + 0x30) < uVar12) {
        pbVar18 = &DAT_00000008;
        uVar13 = uVar14;
      }
      *puVar4 = (*(longlong *)(lVar11 + 0x40) - uVar13) - uVar5;
      pbVar1 = pbVar16 + uVar5 * 0x28;
      pbVar2 = pbVar18 + uVar13 * 0x28;
      lVar11 = *in_R9;
      local_22c = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
      lVar6 = *in_stack_00000028;
      puVar7 = (undefined4 *)in_stack_00000028[2];
LAB_1421b6a1a:
      do {
        if ((pbVar16 == (byte *)0x0) || (pbVar16 == pbVar1)) {
          if ((pbVar18 == (byte *)0x0) || (pbVar18 == pbVar2)) {
            if (((uVar14 & 1) != 0) &&
               (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0)) {
              toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
              local_168 = *extraout_RAX_00;
              uStack_160 = extraout_RAX_00[1];
              local_13c = *(undefined4 *)(extraout_RAX_00 + 2);
              local_158 = (float *)0x1;
              uStack_150 = 
              "timberframe::timberframe_circle/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.7.1/src/map/core.rs"
              ;
              local_148 = 0x1f;
              local_138 = &PTR_s_Timberframe_constructor_ran_out_o_142ff8f40;
              local_130 = 1;
              local_128 = 8;
              local_188 = (undefined **)0x0;
              local_120 = 0;
              uStack_118 = 0;
              fStack_180 = 127.78009;
              fStack_17c = 1.4013e-45;
              local_178._0_4_ = 4.34403e-44;
              local_178._4_4_ = 0.0;
              fStack_170 = 0.0;
              fStack_16c = 0.0;
              local_140 = 1;
              ::log::__private_api::impl$2::log();
            }
            parking_lot_core::parking_lot::deadlock_impl::release_resource();
            LOCK();
            bVar21 = *(char *)(lVar3 + 0x18) == '\x01';
            if (bVar21) {
              *(char *)(lVar3 + 0x18) = '\0';
            }
            UNLOCK();
            if (!bVar21) {
              parking_lot::raw_mutex::RawMutex::unlock_slow();
            }
            return;
          }
          pbVar8 = (byte *)0x0;
          pbVar9 = pbVar18 + 0x28;
          pbVar17 = pbVar18;
        }
        else {
          pbVar8 = pbVar16 + 0x28;
          pbVar9 = pbVar18;
          pbVar17 = pbVar16;
        }
        pbVar18 = pbVar9;
        pbVar16 = pbVar8;
        *puVar4 = *puVar4 + 1;
      } while ((*pbVar17 & 1) != 0);
      local_198 = pbVar17 + 4;
      fVar23 = *(float *)(lVar11 + 8);
      local_f4 = *(float *)(lVar11 + 0xc);
      ::utils::geometry::circle::Circle2d::perimeter();
      fVar22 = extraout_XMM0_Da / 1.5;
      local_190 = (float *)0x0;
      if (0.0 <= fVar22) {
        local_190 = (float *)((longlong)(fVar22 - 9.223372e+18) & (longlong)fVar22 >> 0x3f |
                             (longlong)fVar22);
      }
      if (1.8446743e+19 < fVar22) {
        local_190 = (float *)0xffffffffffffffff;
      }
      if (local_190 < (float *)0x7) {
        local_190 = (float *)0x6;
      }
      ::utils::geometry::circle::Circle2d::as_points2_ptcount();
      uVar5 = CONCAT44(fStack_17c,fStack_180);
      local_108 = (float **)local_188;
      local_100 = (float **)CONCAT44(fStack_16c,fStack_170);
      lVar15 = 0;
      ppfVar19 = local_178;
      if (local_178 < local_100) {
        do {
          fVar22 = (float)ppfVar19 / ((float)uVar5 + -1.0);
          local_228 = fVar22;
          if ((fVar22 < 0.0) || (1.0 < fVar22)) {
            pcStack_1d0 = core::fmt::float::impl$5::fmt;
            local_188 = &anon_678c08fa9c7482e394956b33ca71e3dd_1_llvm_1714830819680390664;
            fStack_180 = 1.4013e-45;
            fStack_17c = 0.0;
            local_168 = 0;
            local_178 = &local_1d8;
            fStack_170 = 1.4013e-45;
            fStack_16c = 0.0;
            local_1d8 = &local_228;
                    // WARNING: Subroutine does not return
            core::panicking::panic_fmt();
          }
          if (NAN(fVar22)) {
                    // WARNING: Subroutine does not return
            core::panicking::panic();
          }
          ::utils::geometry::circle::Circle2d::get_pos_at_u();
          local_1f8._0_4_ = extraout_XMM0_Da_00;
          local_1f8._4_4_ = fVar22;
          ::utils::swizzlers::impl$1::xvy();
          local_158 = (float *)local_188;
          fStack_220 = fStack_180;
          local_228 = SUB84(local_188,0);
          fStack_224 = (float)((ulonglong)local_188 >> 0x20);
          fStack_21c = 0.2;
          uStack_218._0_4_ = local_f4;
          uStack_218._4_4_ = 0.2;
          fStack_17c = 0.0;
          local_178._0_4_ = local_f4 * 0.0;
          local_178._4_4_ = local_f4 * 1.0;
          fStack_170 = local_f4 * 0.0;
          fStack_16c = local_f4 * 0.0;
          local_188 = (undefined **)0x3e4ccccd;
          local_168 = 0;
          uStack_160 = 0x3e4ccccd3e4ccccd;
          uStack_150 = (char *)CONCAT44(fStack_180,fStack_180);
          fStack_180 = (float)0;
          country_core::resources::render::impl$1::from();
          local_168 = CONCAT44(uStack_1b4,local_1b8);
          uStack_160 = CONCAT44(fStack_1ac,fStack_1b0);
          local_188 = (undefined **)local_1d8;
          fStack_180 = SUB84(pcStack_1d0,0);
          fStack_17c = (float)((ulonglong)pcStack_1d0 >> 0x20);
          local_178._0_4_ = SUB84(local_1c8,0);
          local_178._4_4_ = (float)((ulonglong)local_1c8 >> 0x20);
          fStack_170 = SUB84(ppfStack_1c0,0);
          fStack_16c = (float)((ulonglong)ppfStack_1c0 >> 0x20);
          local_158 = (float *)0x0;
          uStack_150 = (char *)0x200000000;
          country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
          push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
          ppfVar19 = (float **)((longlong)ppfVar19 + 1);
          uVar14 = (ulonglong)(byte)((byte)uVar14 | extraout_AL);
          lVar15 = lVar15 + 1;
        } while (ppfVar19 < local_100);
      }
      country_core::resources::walls::split_height_into_skirts();
      local_240 = 0.0;
      local_23c = local_f4;
      local_108 = (float **)0x0;
      ppfVar19 = local_2d8;
LAB_1421b6d57:
      if ((ppfVar19 == (float **)0x0) ||
         (ppfVar19 == (float **)((longlong)local_2d8 + local_2d0 * 4))) {
        if (local_108 == (float **)&DAT_00000008) goto LAB_1421b7220;
        ppfVar19 = (float **)((longlong)&local_240 + (longlong)local_108);
        local_108 = (float **)((longlong)local_108 + 4);
        local_100 = (float **)0x0;
      }
      else {
        local_100 = (float **)((longlong)ppfVar19 + 4);
      }
      fVar22 = *(float *)ppfVar19;
      ::utils::geometry::circle::Circle2d::as_points2_ptcount();
      local_1c8 = local_178;
      ppfStack_1c0 = (float **)CONCAT44(fStack_16c,fStack_170);
      local_1d8 = (float *)local_188;
      pcStack_1d0 = (code *)CONCAT44(fStack_17c,fStack_180);
      local_1b8 = 0;
      fVar22 = fVar22 + fVar23;
      fVar30 = fVar22 * 0.0;
      ppfVar19 = local_100;
      if (local_178 < (float **)CONCAT44(fStack_16c,fStack_170)) {
        do {
          ppfVar19 = (float **)((longlong)local_1c8 + 1);
          fVar24 = (float)local_1c8 / ((float)pcStack_1d0 + -1.0);
          local_1c8 = ppfVar19;
          local_230 = fVar24;
          if ((fVar24 < 0.0) || (1.0 < fVar24)) {
            local_1f8 = &local_230;
            fStack_1f0 = 6.774544;
            fStack_1ec = 1.4013e-45;
            local_188 = &anon_678c08fa9c7482e394956b33ca71e3dd_1_llvm_1714830819680390664;
            fStack_180 = 1.4013e-45;
            fStack_17c = 0.0;
            local_168 = 0;
            local_178 = (float **)&local_1f8;
            fStack_170 = 1.4013e-45;
            fStack_16c = 0.0;
                    // WARNING: Subroutine does not return
            core::panicking::panic_fmt();
          }
          if (NAN(fVar24)) {
                    // WARNING: Subroutine does not return
            core::panicking::panic();
          }
          ::utils::geometry::circle::Circle2d::get_pos_at_u();
          local_188 = (undefined **)CONCAT44(fVar24,extraout_XMM0_Da_01);
          ::utils::swizzlers::impl$0::x0y();
          if (local_1b8 == 1) {
            uStack_1b4 = (float)local_1a8;
            fVar24 = local_24c;
            fVar26 = fStack_248;
            fVar27 = local_1a0;
            fVar28 = local_244;
            ppfVar20 = local_1a8;
          }
          else {
            fStack_21c = local_244;
            fStack_224 = local_24c;
            fStack_220 = fStack_248;
            local_228 = 1.4013e-45;
            uStack_218 = &local_1d8;
            itertools::tuple_impl::impl$31::
            collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,ref_mut_<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl_0::as_points2_ptcount::closure_env_0>,timberframe::timberframe_circle::construct_timberframe_circle::closure_env_1>_>_>_>
                      ();
            local_1a8 = local_178;
            local_1a0 = fStack_170;
            local_1b8 = (int)local_188;
            uStack_1b4 = (undefined4)((ulonglong)local_188 >> 0x20);
            ppfVar20 = (float **)CONCAT44(fStack_180,uStack_1b4);
            fStack_1b0 = fStack_180;
            fStack_1ac = fStack_17c;
            ppfVar19 = local_100;
            fVar24 = (float)local_178;
            fVar26 = local_178._4_4_;
            fVar27 = fStack_17c;
            fVar28 = fStack_170;
            if (local_1b8 != 1) break;
          }
          fVar31 = (float)((ulonglong)ppfVar20 >> 0x20);
          fVar29 = SUB84(ppfVar20,0);
          fVar25 = fVar28 - fVar27;
          local_24c = (1.0 / SQRT(fVar25 * fVar25 + (fVar24 - fVar29) * (fVar24 - fVar29))) * fVar25
          ;
          fStack_1b0 = fVar31;
          fStack_1ac = fVar27;
          local_1a8._0_4_ = fVar24;
          local_1a8._4_4_ = fVar26;
          local_1a0 = fVar28;
          ::utils::swizzlers::impl$1::xvy();
          ::utils::rotation_from_normal();
          local_1f8._0_4_ = (fVar29 + fVar24) * 0.5 + fVar30;
          local_1f8._4_4_ = (fVar31 + fVar26) * 0.5 + fVar22;
          fStack_1f0 = (fVar27 + fVar28) * 0.5 + fVar30;
          fStack_1e4 = SQRT((fVar27 - fVar28) * (fVar27 - fVar28) +
                            (fVar31 - fVar26) * (fVar31 - fVar26) +
                            (fVar29 - fVar24) * (fVar29 - fVar24));
          fStack_1ec = 0.2;
          uStack_1e8 = 0x3e4ccccd;
          fVar31 = fStack_180 + fStack_180;
          fVar24 = fStack_180 * fVar31;
          fVar28 = local_188._4_4_ + local_188._4_4_;
          fVar32 = fVar31 * (float)local_188;
          fVar33 = fVar31 * local_188._4_4_;
          fVar29 = (float)local_188 * ((float)local_188 + (float)local_188);
          fVar27 = ((float)local_188 + (float)local_188) * fStack_17c;
          fVar25 = fVar28 * fStack_17c;
          fVar31 = fStack_17c * fVar31;
          fStack_17c = fVar32 - fVar25;
          fStack_16c = fVar27 + fVar33;
          fVar26 = 1.0 - (fVar29 + local_188._4_4_ * fVar28);
          fStack_180 = fStack_17c * 0.2;
          fStack_17c = fStack_17c * 0.2;
          local_178._0_4_ = (fVar28 * (float)local_188 - fVar31) * 0.2;
          local_178._4_4_ = (1.0 - (fVar24 + fVar29)) * 0.2;
          fStack_170 = fStack_16c * 0.2;
          fStack_16c = fStack_16c * 0.2;
          local_188 = (undefined **)
                      CONCAT44((fVar28 * (float)local_188 + fVar31) * 0.2,
                               (1.0 - (local_188._4_4_ * fVar28 + fVar24)) * 0.2);
          local_168 = CONCAT44(fStack_1e4 * (fVar33 - fVar27),fStack_1e4 * (fVar25 + fVar32));
          uStack_160 = CONCAT44(fStack_1e4 * fVar26,fStack_1e4 * fVar26);
          local_158 = (float *)CONCAT44(local_1f8._4_4_,(float)local_1f8);
          uStack_150 = (char *)CONCAT44(fStack_1f0,fStack_1f0);
          country_core::resources::render::impl$1::from();
          local_188 = (undefined **)CONCAT44(fStack_224,local_228);
          fStack_180 = fStack_220;
          fStack_17c = fStack_21c;
          fStack_170 = (float)uStack_210;
          fStack_16c = (float)((ulonglong)uStack_210 >> 0x20);
          local_168 = local_208;
          uStack_160 = uStack_200;
          local_158 = (float *)0x0;
          uStack_150 = (char *)0x200000000;
          local_178 = uStack_218;
          country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
          push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
          uVar14 = (ulonglong)(byte)((byte)uVar14 | extraout_AL_00);
          lVar15 = lVar15 + 1;
          ppfVar19 = local_100;
        } while (local_1c8 < ppfStack_1c0);
      }
      goto LAB_1421b6d57;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
LAB_1421b7220:
  if (local_2e0 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (pbVar17[0x1c] == 0) {
    country_core::resources::roofs::roof_shape::roof_height_ws_from_wall_shape();
    local_100._0_4_ = extraout_XMM0_Da_02;
    ::utils::geometry::circle::Circle2d::as_points2_ptcount();
    uVar5 = CONCAT44(fStack_17c,fStack_180);
    local_190 = (float *)local_188;
    local_108 = (float **)CONCAT44(fStack_16c,fStack_170);
    fVar23 = fVar23 + local_f4;
    local_100 = (float **)CONCAT44(local_100._4_4_,local_100._0_4_ + fVar23);
    ppfVar20 = local_178;
    ppfVar19 = local_178;
    if (local_178 < local_108) {
      do {
        local_178 = &local_1d8;
        fVar22 = (float)ppfVar20 / ((float)uVar5 + -1.0);
        local_228 = fVar22;
        if ((fVar22 < 0.0) || (1.0 < fVar22)) {
          pcStack_1d0 = core::fmt::float::impl$5::fmt;
          local_188 = &anon_678c08fa9c7482e394956b33ca71e3dd_1_llvm_1714830819680390664;
          fStack_180 = 1.4013e-45;
          fStack_17c = 0.0;
          local_168 = 0;
          fStack_170 = 1.4013e-45;
          fStack_16c = 0.0;
          local_1d8 = &local_228;
                    // WARNING: Subroutine does not return
          core::panicking::panic_fmt();
        }
        local_178 = ppfVar19;
        if (NAN(fVar22)) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
        ::utils::geometry::circle::Circle2d::get_pos_at_u();
        local_188._0_4_ = extraout_XMM0_Da_03;
        local_188._4_4_ = fVar22;
        ::utils::swizzlers::impl$0::x0y();
        fVar30 = fStack_1f0;
        fVar22 = local_1f8._4_4_;
        local_1f8._0_4_ = 1.4013e-45;
        fVar25 = fStack_1ec + fVar23 * 0.0;
        ::utils::swizzlers::impl$1::xvy();
        fVar22 = fVar22 + fVar23 * 0.0;
        fVar30 = fVar30 + fVar23;
        fVar29 = (float)local_2c8;
        fVar31 = (float)((ulonglong)local_2c8 >> 0x20);
        fVar24 = local_2c0 - fVar25;
        fVar26 = fVar29 - fVar22;
        fVar27 = fVar31 - fVar30;
        fVar28 = 1.0 / SQRT(fVar27 * fVar27 + fVar26 * fVar26 + fVar24 * fVar24);
        local_1d8 = (float *)CONCAT44(fVar28 * fVar27,fVar28 * fVar26);
        pcStack_1d0 = (code *)CONCAT44(pcStack_1d0._4_4_,fVar28 * fVar24);
        ::utils::rotation_from_normal();
        fStack_220 = (fVar25 + local_2c0) * 0.5;
        uStack_218._4_4_ =
             SQRT((fVar25 - local_2c0) * (fVar25 - local_2c0) +
                  (fVar30 - fVar31) * (fVar30 - fVar31) + (fVar22 - fVar29) * (fVar22 - fVar29));
        local_228 = (fVar22 + fVar29) * 0.5;
        fStack_224 = (fVar30 + fVar31) * 0.5;
        fStack_21c = 0.2;
        uStack_218._0_4_ = 0.2;
        fVar25 = fStack_180 + fStack_180;
        fVar22 = fStack_180 * fVar25;
        fVar26 = local_188._4_4_ + local_188._4_4_;
        fVar29 = fVar25 * (float)local_188;
        fVar31 = fVar25 * local_188._4_4_;
        fVar28 = (float)local_188 * ((float)local_188 + (float)local_188);
        fVar24 = ((float)local_188 + (float)local_188) * fStack_17c;
        fVar27 = fVar26 * fStack_17c;
        fVar25 = fStack_17c * fVar25;
        fStack_17c = fVar29 - fVar27;
        fStack_16c = fVar24 + fVar31;
        fVar30 = 1.0 - (fVar28 + local_188._4_4_ * fVar26);
        fStack_180 = fStack_17c * 0.2;
        fStack_17c = fStack_17c * 0.2;
        local_178._0_4_ = (fVar26 * (float)local_188 - fVar25) * 0.2;
        local_178._4_4_ = (1.0 - (fVar22 + fVar28)) * 0.2;
        fStack_170 = fStack_16c * 0.2;
        fStack_16c = fStack_16c * 0.2;
        local_188 = (undefined **)
                    CONCAT44((fVar26 * (float)local_188 + fVar25) * 0.2,
                             (1.0 - (local_188._4_4_ * fVar26 + fVar22)) * 0.2);
        local_168 = CONCAT44(uStack_218._4_4_ * (fVar31 - fVar24),
                             uStack_218._4_4_ * (fVar27 + fVar29));
        uStack_160 = CONCAT44(uStack_218._4_4_ * fVar30,uStack_218._4_4_ * fVar30);
        local_158 = (float *)CONCAT44(fStack_224,local_228);
        uStack_150 = (char *)CONCAT44(fStack_220,fStack_220);
        country_core::resources::render::impl$1::from();
        local_168 = CONCAT44(uStack_1b4,local_1b8);
        uStack_160 = CONCAT44(fStack_1ac,fStack_1b0);
        local_188 = (undefined **)local_1d8;
        fStack_180 = SUB84(pcStack_1d0,0);
        fStack_17c = (float)((ulonglong)pcStack_1d0 >> 0x20);
        local_178._0_4_ = SUB84(local_1c8,0);
        local_178._4_4_ = (float)((ulonglong)local_1c8 >> 0x20);
        fStack_170 = SUB84(ppfStack_1c0,0);
        fStack_16c = (float)((ulonglong)ppfStack_1c0 >> 0x20);
        local_158 = (float *)0x0;
        uStack_150 = (char *)0x200000000;
        country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
        push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
        ppfVar20 = (float **)((longlong)ppfVar20 + 1);
        uVar14 = (ulonglong)(byte)((byte)uVar14 | extraout_AL_01);
        lVar15 = lVar15 + 1;
        ppfVar19 = local_178;
      } while (ppfVar20 < local_108);
    }
  }
  *puVar7 = local_22c;
  *(longlong *)(lVar6 + 8) = lVar15;
  goto LAB_1421b6a1a;
}

