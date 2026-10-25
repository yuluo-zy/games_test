// 0x141a41ba0 construct_timberframe_rectangle :: void __cdecl construct_timberframe_rectangle(void)

void __cdecl timberframe::timberframe_rectangle::construct_timberframe_rectangle(void)

{
  byte *pbVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  ulonglong *puVar4;
  ulonglong uVar5;
  longlong lVar6;
  undefined4 *puVar7;
  longlong lVar8;
  byte extraout_AL;
  byte extraout_AL_00;
  byte extraout_AL_01;
  byte extraout_AL_02;
  byte extraout_AL_03;
  byte extraout_AL_04;
  int iVar9;
  undefined8 *extraout_RAX;
  longlong lVar10;
  float *extraout_RAX_00;
  longlong lVar11;
  undefined8 *extraout_RAX_01;
  float *pfVar12;
  float *pfVar13;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  undefined8 *extraout_RAX_04;
  undefined8 *in_RCX;
  float *pfVar14;
  float *pfVar15;
  undefined8 *in_RDX;
  byte *pbVar16;
  float *pfVar17;
  byte *pbVar18;
  longlong lVar19;
  longlong *in_R9;
  byte *pbVar20;
  ulonglong uVar21;
  float *pfVar22;
  longlong lVar23;
  ulonglong uVar24;
  ulonglong uVar25;
  bool bVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  float extraout_XMM0_Da;
  undefined4 extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
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
  longlong *in_stack_00000028;
  undefined8 local_380;
  float local_378;
  undefined8 local_370;
  float local_368;
  undefined8 local_360;
  float local_358;
  undefined8 local_350;
  float local_348;
  float local_2b0 [2];
  float *local_2a8;
  undefined4 local_29c;
  float *local_298;
  byte *local_290;
  longlong local_288;
  float *pfStack_280;
  longlong local_278;
  byte *local_270;
  longlong local_268;
  float fStack_260;
  undefined4 uStack_25c;
  float fStack_258;
  float fStack_254;
  longlong lStack_250;
  undefined8 local_248;
  undefined8 uStack_240;
  undefined8 local_228;
  float fStack_220;
  undefined8 uStack_21c;
  float fStack_214;
  longlong local_208;
  float *pfStack_200;
  float local_1f8;
  float fStack_1f4;
  float fStack_1f0;
  float fStack_1ec;
  undefined8 local_1e8;
  undefined8 uStack_1e0;
  undefined8 local_1d8;
  undefined8 uStack_1d0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  float local_1b8;
  float fStack_1b4;
  float fStack_1b0;
  float fStack_1ac;
  undefined8 local_1a8;
  undefined8 uStack_1a0;
  undefined8 local_198;
  undefined8 uStack_190;
  undefined8 local_188;
  undefined4 local_180;
  undefined4 local_17c;
  undefined **local_178;
  undefined8 local_170;
  undefined8 local_168;
  undefined8 local_160;
  undefined8 uStack_158;
  undefined8 local_148;
  float fStack_140;
  undefined4 uStack_13c;
  float fStack_138;
  float fStack_134;
  float fStack_130;
  float fStack_12c;
  undefined8 local_128;
  undefined8 uStack_120;
  float *local_118;
  float local_10c;
  float *local_108;
  byte *local_100;
  float *local_f8;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  pbVar18 = (byte *)*in_RDX;
  if (*(longlong *)pbVar18 == 0) {
    frozen::FrozenMap::
    get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
              ();
    if (extraout_RAX != (undefined8 *)0x0) {
      if ((extraout_RAX[3] != 0x3d) ||
         (iVar9 = memcmp("country_core::resources::roofs::roof_buffers::WoodenPlankSsbo/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs"
                         ,(void *)extraout_RAX[2],0x3d), iVar9 != 0)) {
        local_1c8 = 0;
                    // WARNING: Subroutine does not return
        core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
      }
      lVar10 = (**(code **)(extraout_RAX[1] + 0x18))(*extraout_RAX);
      if (lVar10 != 0) {
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_00 == (float *)0x0) {
                    // WARNING: Subroutine does not return
          alloc::alloc::handle_alloc_error();
        }
        extraout_RAX_00[0] = 0.0;
        extraout_RAX_00[1] = 0.0;
        extraout_RAX_00[2] = 5.60519e-45;
        extraout_RAX_00[3] = 0.0;
        extraout_RAX_00[4] = 0.0;
        extraout_RAX_00[5] = 0.0;
        *(undefined1 *)(extraout_RAX_00 + 6) = 0;
        local_f8 = *(float **)(lVar10 + 0x60);
        do {
          lVar10 = *(longlong *)(local_f8 + 2);
          while (lVar10 != -1) {
            if (lVar10 < 0) {
                    // WARNING: Subroutine does not return
              alloc::sync::impl$21::downgrade::panic_cold_display<ref_<str_>_>();
            }
            LOCK();
            lVar11 = *(longlong *)(local_f8 + 2);
            bVar26 = lVar10 == lVar11;
            if (bVar26) {
              *(longlong *)(local_f8 + 2) = lVar10 + 1;
              lVar11 = lVar10;
            }
            UNLOCK();
            lVar10 = lVar11;
            if (bVar26) {
              *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
              local_108 = extraout_RAX_00;
              local_100 = pbVar18;
              if (*(longlong *)pbVar18 != 0) {
                core::ptr::
                drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>_>
                          ();
              }
              *(float **)local_100 = local_f8;
              *(float **)(local_100 + 8) = local_108;
              pbVar18 = local_100;
              goto LAB_141a41d5b;
            }
          }
        } while( true );
      }
    }
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
LAB_141a41d5b:
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX_01 != (undefined8 *)0x0) {
    if ((extraout_RAX_01[3] != 0x3d) ||
       (iVar9 = memcmp("country_core::resources::roofs::roof_buffers::WoodenPlankSsbo/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs"
                       ,(void *)extraout_RAX_01[2],0x3d), iVar9 != 0)) {
      local_1c8 = 0;
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    lVar10 = (**(code **)(extraout_RAX_01[1] + 0x18))(*extraout_RAX_01);
    if (lVar10 != 0) {
      if (*(longlong *)pbVar18 == 0) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      lVar11 = *(longlong *)(pbVar18 + 8);
      if (lVar11 == 0) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      LOCK();
      bVar26 = *(char *)(lVar11 + 0x18) == '\0';
      if (bVar26) {
        *(char *)(lVar11 + 0x18) = '\x01';
      }
      UNLOCK();
      if (!bVar26) {
        parking_lot::raw_mutex::RawMutex::lock_slow();
      }
      parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
      if ((*(longlong *)(lVar11 + 0x10) * 4 + *(longlong *)(lVar11 + 8) == 4 ||
           *(longlong *)(lVar11 + 0x10) == 0) && (*(longlong *)(lVar10 + 0x28) == 0)) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
      clear<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
      puVar4 = (ulonglong *)*in_RCX;
      lVar10 = in_RCX[1];
      uVar5 = *puVar4;
      uVar24 = 0;
      uVar25 = uVar5 - *(ulonglong *)(lVar10 + 0x18);
      if (uVar5 < *(ulonglong *)(lVar10 + 0x18)) {
        uVar25 = uVar24;
      }
      uVar21 = uVar5 - *(ulonglong *)(lVar10 + 0x38);
      if (uVar5 < *(ulonglong *)(lVar10 + 0x38)) {
        uVar21 = uVar24;
      }
      pbVar18 = (byte *)(uVar25 * 0x28 + *(longlong *)(lVar10 + 8));
      uVar5 = *(ulonglong *)(lVar10 + 0x10) - uVar25;
      if (*(ulonglong *)(lVar10 + 0x10) < uVar25) {
        pbVar18 = &DAT_00000008;
        uVar5 = uVar24;
      }
      pbVar20 = (byte *)(uVar21 * 0x28 + *(longlong *)(lVar10 + 0x28));
      uVar25 = *(ulonglong *)(lVar10 + 0x30) - uVar21;
      if (*(ulonglong *)(lVar10 + 0x30) < uVar21) {
        pbVar20 = &DAT_00000008;
        uVar25 = uVar24;
      }
      *puVar4 = (*(longlong *)(lVar10 + 0x40) - uVar25) - uVar5;
      pbVar1 = pbVar20 + uVar25 * 0x28;
      lVar10 = *in_stack_00000028;
      local_29c = *(undefined4 *)((longlong)in_R9 + 0x1c);
      lVar6 = *in_R9;
      puVar7 = (undefined4 *)in_R9[2];
      pfVar13 = (float *)pbVar18;
LAB_141a41f69:
      do {
        pbVar16 = (byte *)pfVar13;
        if ((pbVar16 == (byte *)0x0) || (pbVar16 == pbVar18 + uVar5 * 0x28)) {
          if ((pbVar20 == (byte *)0x0) || (pbVar20 == pbVar1)) {
            if (((uVar24 & 1) != 0) &&
               (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0)) {
              toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
              local_1a8 = *extraout_RAX_04;
              uStack_1a0 = extraout_RAX_04[1];
              local_17c = *(undefined4 *)(extraout_RAX_04 + 2);
              local_198 = 1;
              uStack_190 = &DAT_142ebc3e8;
              local_188 = 0x22;
              local_178 = &PTR_s_Timberframe_constructor_ran_out_o_142ebc3c0;
              local_170 = 1;
              local_168 = 8;
              local_1c8 = 0;
              local_160 = 0;
              uStack_158 = 0;
              uStack_1c0 = (float *)&DAT_142ebc3e8;
              local_1b8 = 4.76441e-44;
              fStack_1b4 = 0.0;
              fStack_1b0 = 0.0;
              fStack_1ac = 0.0;
              local_180 = 1;
              ::log::__private_api::impl$2::log();
            }
            parking_lot_core::parking_lot::deadlock_impl::release_resource();
            LOCK();
            bVar26 = *(char *)(lVar11 + 0x18) == '\x01';
            if (bVar26) {
              *(char *)(lVar11 + 0x18) = '\0';
            }
            UNLOCK();
            if (!bVar26) {
              parking_lot::raw_mutex::RawMutex::unlock_slow();
            }
            return;
          }
          pfVar13 = (float *)0x0;
          pbVar16 = pbVar20;
          pbVar20 = pbVar20 + 0x28;
        }
        else {
          pfVar13 = (float *)(pbVar16 + 0x28);
        }
        *puVar4 = *puVar4 + 1;
      } while ((*pbVar16 & 1) == 0);
      local_100 = pbVar16 + 4;
      local_10c = *(float *)(lVar10 + 8);
      fVar33 = *(float *)(lVar10 + 0xc);
      local_290 = pbVar16;
      ::utils::geometry::rectangle::Rectangle2d::as_curve3();
      pfVar14 = pfStack_200;
      lVar8 = CONCAT44(fStack_1f4,local_1f8);
      local_270 = pbVar20;
      if (lVar8 == 0) {
        lVar19 = 0;
      }
      else {
        lVar23 = 0;
        lVar19 = 0;
        do {
          fStack_260 = *(float *)((longlong)pfVar14 + lVar23 + 8);
          local_268 = *(longlong *)((longlong)pfVar14 + lVar23);
          ::utils::geometry::rectangle::Rectangle2d::orientation_mat3();
          if ((float)local_1a8 <= 0.0) {
            local_1b8 = local_1b8 - (float)local_1c8;
            if (local_1b8 <= 0.0) {
              local_1c8._0_4_ = fStack_1b0 + (float)uStack_1c0;
              fVar31 = uStack_1c0._4_4_ + local_1c8._4_4_;
              fVar30 = fStack_1b4 - fStack_1ac;
              fStack_1b0 = (1.0 - (float)local_1a8) - local_1b8;
              fVar27 = fStack_1b0;
            }
            else {
              fVar30 = fStack_1b0 - (float)uStack_1c0;
              local_1c8._0_4_ = fStack_1ac + fStack_1b4;
              fVar31 = (1.0 - (float)local_1a8) + local_1b8;
              fStack_1b0 = uStack_1c0._4_4_ + local_1c8._4_4_;
              fVar27 = fVar31;
            }
LAB_141a4217a:
            fVar27 = 0.5 / SQRT(fVar27);
            fVar28 = fVar27 * (float)local_1c8;
            fVar30 = fVar27 * fVar30;
            fVar31 = fVar27 * fVar31;
            fVar27 = fVar27 * fStack_1b0;
          }
          else {
            local_1c8._0_4_ = (float)local_1c8 + local_1b8;
            if ((float)local_1c8 <= 0.0) {
              fVar31 = fStack_1b4 + fStack_1ac;
              fStack_1b0 = fStack_1b0 + (float)uStack_1c0;
              local_1c8._0_4_ = ((float)local_1a8 + 1.0) - (float)local_1c8;
              fVar30 = local_1c8._4_4_ - uStack_1c0._4_4_;
              fVar27 = (float)local_1c8;
              goto LAB_141a4217a;
            }
            local_1c8._0_4_ = (float)local_1a8 + 1.0 + (float)local_1c8;
            fVar27 = 0.5 / SQRT((float)local_1c8);
            fVar28 = fVar27 * (local_1c8._4_4_ - uStack_1c0._4_4_);
            fVar30 = fVar27 * (float)local_1c8;
            fVar31 = fVar27 * (fStack_1b0 - (float)uStack_1c0);
            fVar27 = fVar27 * (fStack_1b4 - fStack_1ac);
          }
          fStack_140 = fStack_260;
          local_148 = local_268;
          uStack_13c = 0x3e4ccccd;
          fStack_134 = 0.2;
          fVar35 = fVar28 + fVar28;
          fVar36 = fVar31 + fVar31;
          fVar39 = fVar27 * (fVar27 + fVar27);
          fVar32 = (fVar27 + fVar27) * fVar30;
          fVar34 = fVar27 * fVar35 - fVar36 * fVar30;
          fStack_1ac = fVar31 * fVar35 + fVar32;
          fVar29 = 1.0 - (fVar39 + fVar31 * fVar36);
          local_1b8 = fVar33 * (fVar27 * fVar36 - fVar35 * fVar30);
          fStack_1b4 = fVar33 * (1.0 - (fVar28 * fVar35 + fVar39));
          fStack_1b0 = fVar33 * fStack_1ac;
          fStack_1ac = fVar33 * fStack_1ac;
          local_1c8 = CONCAT44((fVar27 * fVar36 + fVar35 * fVar30) * 0.2,
                               (1.0 - (fVar31 * fVar36 + fVar28 * fVar35)) * 0.2);
          uStack_1c0 = (float *)CONCAT44(fVar34 * 0.2,fVar34 * 0.2);
          local_1a8 = CONCAT44((fVar31 * fVar35 - fVar32) * 0.2,
                               (fVar36 * fVar30 + fVar27 * fVar35) * 0.2);
          uStack_1a0 = CONCAT44(fVar29 * 0.2,fVar29 * 0.2);
          local_198 = local_268;
          uStack_190 = (undefined *)CONCAT44(fStack_260,fStack_260);
          fStack_138 = fVar33;
          country_core::resources::render::impl$1::from();
          uStack_1c0 = (float *)CONCAT44(uStack_25c,fStack_260);
          local_1c8 = local_268;
          local_1b8 = fStack_258;
          fStack_1b4 = fStack_254;
          fStack_1b0 = (float)lStack_250;
          fStack_1ac = (float)((ulonglong)lStack_250 >> 0x20);
          local_1a8 = local_248;
          uStack_1a0 = uStack_240;
          local_198 = 0;
          uStack_190 = (undefined *)0x200000000;
          country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
          push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
          uVar24 = (ulonglong)(byte)((byte)uVar24 | extraout_AL);
          lVar19 = lVar19 + 1;
          lVar23 = lVar23 + 0xc;
        } while (lVar8 * 0xc != lVar23);
      }
      if (local_208 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (CONCAT44(fStack_1ec,fStack_1f0) != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      ::utils::geometry::rectangle::Rectangle2d::as_curve3();
      local_298 = pfStack_200;
      pbVar20 = local_270;
      if (CONCAT44(fStack_1f4,local_1f8) != 0) {
        local_118 = pfStack_200 + CONCAT44(fStack_1f4,local_1f8) * 3;
        bVar26 = true;
        pfVar14 = pfStack_200;
        do {
          pfVar12 = pfVar14 + 3;
          pfVar15 = pfVar14;
          pfVar22 = local_108;
          if (bVar26) {
            if (pfVar12 == local_118) {
              local_108 = local_118;
              break;
            }
            pfVar15 = pfVar12;
            pfVar22 = pfVar14;
            pfVar12 = pfVar14 + 6;
          }
          local_f8 = pfVar12;
          local_108 = pfVar15;
          fVar27 = floorf(SQRT((pfVar22[2] - pfVar15[2]) * (pfVar22[2] - pfVar15[2]) +
                               (pfVar22[1] - pfVar15[1]) * (pfVar22[1] - pfVar15[1]) +
                               (*pfVar22 - *pfVar15) * (*pfVar22 - *pfVar15)) / 1.51);
          uVar25 = 0;
          if (0.0 <= fVar27) {
            uVar25 = (longlong)(fVar27 - 9.223372e+18) & (longlong)fVar27 >> 0x3f | (longlong)fVar27
            ;
          }
          if (1.8446743e+19 < fVar27) {
            uVar25 = 0xffffffffffffffff;
          }
          if (1 < uVar25) {
            lVar19 = lVar19 + uVar25 + -1;
            uVar21 = 1;
            do {
              fVar29 = (float)uVar21 / (float)uVar25;
              fVar32 = 1.0 - fVar29;
              fVar27 = pfVar22[2];
              uVar2 = *(undefined8 *)pfVar22;
              uVar3 = *(undefined8 *)local_108;
              fVar30 = local_108[2];
              ::utils::geometry::rectangle::Rectangle2d::orientation_mat3();
              fVar31 = fVar29 * (float)uVar3 + fVar32 * (float)uVar2;
              fVar28 = fVar29 * (float)((ulonglong)uVar3 >> 0x20) +
                       fVar32 * (float)((ulonglong)uVar2 >> 0x20);
              fStack_140 = fVar27 * fVar32 + fVar30 * fVar29;
              if ((float)local_1a8 <= 0.0) {
                local_1b8 = local_1b8 - (float)local_1c8;
                if (local_1b8 <= 0.0) {
                  local_1c8._0_4_ = fStack_1b0 + (float)uStack_1c0;
                  fVar29 = uStack_1c0._4_4_ + local_1c8._4_4_;
                  fVar30 = fStack_1b4 - fStack_1ac;
                  fStack_1b0 = (1.0 - (float)local_1a8) - local_1b8;
                  fVar27 = fStack_1b0;
                }
                else {
                  fVar30 = fStack_1b0 - (float)uStack_1c0;
                  local_1c8._0_4_ = fStack_1ac + fStack_1b4;
                  fVar29 = (1.0 - (float)local_1a8) + local_1b8;
                  fStack_1b0 = uStack_1c0._4_4_ + local_1c8._4_4_;
                  fVar27 = fVar29;
                }
LAB_141a4271c:
                fVar27 = 0.5 / SQRT(fVar27);
                fVar32 = fVar27 * (float)local_1c8;
                fVar30 = fVar27 * fVar30;
                fVar29 = fVar27 * fVar29;
                fVar27 = fVar27 * fStack_1b0;
              }
              else {
                local_1c8._0_4_ = (float)local_1c8 + local_1b8;
                if ((float)local_1c8 <= 0.0) {
                  fVar29 = fStack_1b4 + fStack_1ac;
                  fStack_1b0 = fStack_1b0 + (float)uStack_1c0;
                  local_1c8._0_4_ = ((float)local_1a8 + 1.0) - (float)local_1c8;
                  fVar30 = local_1c8._4_4_ - uStack_1c0._4_4_;
                  fVar27 = (float)local_1c8;
                  goto LAB_141a4271c;
                }
                local_1c8._0_4_ = (float)local_1a8 + 1.0 + (float)local_1c8;
                fVar27 = 0.5 / SQRT((float)local_1c8);
                fVar32 = fVar27 * (local_1c8._4_4_ - uStack_1c0._4_4_);
                fVar30 = fVar27 * (float)local_1c8;
                fVar29 = fVar27 * (fStack_1b0 - (float)uStack_1c0);
                fVar27 = fVar27 * (fStack_1b4 - fStack_1ac);
              }
              local_148 = CONCAT44(fVar28,fVar31);
              uStack_13c = 0x3e3851eb;
              fStack_134 = 0.17999999;
              fVar37 = fVar32 + fVar32;
              fVar38 = fVar29 + fVar29;
              fVar39 = fVar27 * (fVar27 + fVar27);
              fVar36 = (fVar27 + fVar27) * fVar30;
              fVar35 = fVar27 * fVar37 - fVar38 * fVar30;
              fStack_1ac = fVar29 * fVar37 + fVar36;
              fVar34 = 1.0 - (fVar39 + fVar29 * fVar38);
              local_1b8 = fVar33 * (fVar27 * fVar38 - fVar37 * fVar30);
              fStack_1b4 = fVar33 * (1.0 - (fVar32 * fVar37 + fVar39));
              fStack_1b0 = fVar33 * fStack_1ac;
              fStack_1ac = fVar33 * fStack_1ac;
              local_1c8 = CONCAT44((fVar27 * fVar38 + fVar37 * fVar30) * 0.17999999,
                                   (1.0 - (fVar29 * fVar38 + fVar32 * fVar37)) * 0.17999999);
              uStack_1c0 = (float *)CONCAT44(fVar35 * 0.17999999,fVar35 * 0.17999999);
              local_1a8 = CONCAT44((fVar29 * fVar37 - fVar36) * 0.17999999,
                                   (fVar38 * fVar30 + fVar27 * fVar37) * 0.17999999);
              uStack_1a0 = CONCAT44(fVar34 * 0.17999999,fVar34 * 0.17999999);
              local_198 = CONCAT44(fVar28,fVar31);
              uStack_190 = (undefined *)CONCAT44(fStack_140,fStack_140);
              fStack_138 = fVar33;
              country_core::resources::render::impl$1::from();
              uStack_1c0 = (float *)CONCAT44(uStack_25c,fStack_260);
              local_1c8 = local_268;
              local_1b8 = fStack_258;
              fStack_1b4 = fStack_254;
              fStack_1b0 = (float)lStack_250;
              fStack_1ac = (float)((ulonglong)lStack_250 >> 0x20);
              local_1a8 = local_248;
              uStack_1a0 = uStack_240;
              local_198 = 0;
              uStack_190 = (undefined *)0x200000000;
              country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
              push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
              uVar21 = uVar21 + 1;
              uVar24 = (ulonglong)(byte)((byte)uVar24 | extraout_AL_00);
              pbVar20 = local_270;
            } while (uVar25 != uVar21);
          }
          bVar26 = false;
          pfVar14 = local_f8;
        } while (local_f8 != local_118);
      }
      if (local_208 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (CONCAT44(fStack_1ec,fStack_1f0) != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      country_core::resources::walls::split_height_into_skirts();
      pfVar14 = pfStack_280 + local_278;
      local_2b0[0] = 0.0;
      local_2b0[1] = fVar33;
      local_2a8 = pfStack_280;
      local_118 = (float *)0x0;
      local_298 = pfVar13;
      pfVar12 = pfStack_280;
LAB_141a429b3:
      if ((pfVar12 == (float *)0x0) || (pfVar12 == pfVar14)) {
        if (local_118 == (float *)&DAT_00000008) goto LAB_141a42fd0;
        pfVar13 = (float *)((longlong)local_2b0 + (longlong)local_118);
        local_118 = local_118 + 1;
        pfVar12 = (float *)0x0;
      }
      else {
        pfVar13 = pfVar12;
        pfVar12 = pfVar12 + 1;
      }
      fVar33 = *pfVar13;
      ::utils::geometry::rectangle::Rectangle2d::as_curve3();
      local_f8 = (float *)CONCAT44(uStack_25c,fStack_260);
      lVar8 = CONCAT44(fStack_254,fStack_258);
      if ((lVar8 != 0) && (lVar8 != 1)) {
        fVar27 = local_f8[2];
        fVar30 = local_f8[5];
        uVar2 = *(undefined8 *)local_f8;
        fVar28 = (float)uVar2;
        uVar3 = *(undefined8 *)(local_f8 + 3);
        fVar32 = (float)uVar3;
        fVar34 = fVar32 - fVar28;
        fVar31 = fVar30 - fVar27;
        fVar29 = 1.0 / SQRT(fVar31 * fVar31 + fVar34 * fVar34);
        local_148 = CONCAT44(fVar29 * fVar31,fVar34 * fVar29);
        ::utils::swizzlers::impl$1::xvy();
        ::utils::rotation_from_normal();
        fVar33 = fVar33 + local_10c;
        fVar38 = fVar33 * 0.0;
        fVar39 = (fVar28 + fVar32) * 0.5 + fVar38;
        fVar37 = ((float)((ulonglong)uVar2 >> 0x20) + (float)((ulonglong)uVar3 >> 0x20)) * 0.5 +
                 fVar33;
        fStack_220 = (fVar27 + fVar30) * 0.5 + fVar38;
        fStack_214 = SQRT((local_f8[2] - local_f8[5]) * (local_f8[2] - local_f8[5]) +
                          (local_f8[1] - local_f8[4]) * (local_f8[1] - local_f8[4]) +
                          (*local_f8 - local_f8[3]) * (*local_f8 - local_f8[3]));
        local_228 = CONCAT44(fVar37,fVar39);
        uStack_21c = 0x3e4ccccd3e4ccccd;
        fVar36 = (float)uStack_1c0 + (float)uStack_1c0;
        fVar28 = local_1c8._4_4_ + local_1c8._4_4_;
        fVar34 = fVar36 * (float)local_1c8;
        fVar35 = fVar36 * local_1c8._4_4_;
        fVar32 = (float)local_1c8 * ((float)local_1c8 + (float)local_1c8);
        fVar30 = ((float)local_1c8 + (float)local_1c8) * uStack_1c0._4_4_;
        fVar29 = fVar28 * uStack_1c0._4_4_;
        fVar27 = fVar34 - fVar29;
        fStack_1ac = fVar30 + fVar35;
        fVar31 = 1.0 - (fVar32 + local_1c8._4_4_ * fVar28);
        local_1b8 = (fVar28 * (float)local_1c8 - uStack_1c0._4_4_ * fVar36) * 0.2;
        fStack_1b4 = (1.0 - ((float)uStack_1c0 * fVar36 + fVar32)) * 0.2;
        fStack_1b0 = fStack_1ac * 0.2;
        fStack_1ac = fStack_1ac * 0.2;
        local_1c8 = CONCAT44((fVar28 * (float)local_1c8 + uStack_1c0._4_4_ * fVar36) * 0.2,
                             (1.0 - (local_1c8._4_4_ * fVar28 + (float)uStack_1c0 * fVar36)) * 0.2);
        uStack_1c0 = (float *)CONCAT44(fVar27 * 0.2,fVar27 * 0.2);
        local_1a8 = CONCAT44(fStack_214 * (fVar35 - fVar30),fStack_214 * (fVar29 + fVar34));
        uStack_1a0 = CONCAT44(fStack_214 * fVar31,fStack_214 * fVar31);
        local_198 = CONCAT44(fVar37,fVar39);
        uStack_190 = (undefined *)CONCAT44(fStack_220,fStack_220);
        country_core::resources::render::impl$1::from();
        pfStack_200 = (float *)CONCAT44(uStack_13c,fStack_140);
        local_208 = local_148;
        local_1f8 = fStack_138;
        fStack_1f4 = fStack_134;
        fStack_1f0 = fStack_130;
        fStack_1ec = fStack_12c;
        local_1e8 = local_128;
        uStack_1e0 = uStack_120;
        local_1d8 = 0;
        uStack_1d0 = 0x200000000;
        country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
        push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
        uVar24 = (ulonglong)(byte)((byte)uVar24 | extraout_AL_01);
        lVar19 = lVar19 + 1;
        if (lVar8 != 2) {
          pfVar13 = local_f8 + lVar8 * 3;
          pfVar15 = local_f8 + 6;
          pfVar22 = local_f8 + 3;
          do {
            pfVar17 = pfVar15;
            fVar27 = pfVar22[2];
            fVar30 = pfVar17[2];
            uVar2 = *(undefined8 *)pfVar22;
            fVar28 = (float)uVar2;
            uVar3 = *(undefined8 *)pfVar17;
            fVar32 = (float)uVar3;
            fVar34 = fVar32 - fVar28;
            fVar31 = fVar30 - fVar27;
            fVar29 = 1.0 / SQRT(fVar31 * fVar31 + fVar34 * fVar34);
            local_148 = CONCAT44(fVar29 * fVar31,fVar34 * fVar29);
            ::utils::swizzlers::impl$1::xvy();
            ::utils::rotation_from_normal();
            fVar32 = (fVar28 + fVar32) * 0.5 + fVar38;
            fVar35 = ((float)((ulonglong)uVar2 >> 0x20) + (float)((ulonglong)uVar3 >> 0x20)) * 0.5 +
                     fVar33;
            fStack_220 = (fVar27 + fVar30) * 0.5 + fVar38;
            fStack_214 = SQRT((pfVar22[2] - pfVar17[2]) * (pfVar22[2] - pfVar17[2]) +
                              (pfVar22[1] - pfVar17[1]) * (pfVar22[1] - pfVar17[1]) +
                              (*pfVar22 - *pfVar17) * (*pfVar22 - *pfVar17));
            local_228 = CONCAT44(fVar35,fVar32);
            uStack_21c = 0x3e4ccccd3e4ccccd;
            fVar37 = (float)uStack_1c0 + (float)uStack_1c0;
            fVar28 = local_1c8._4_4_ + local_1c8._4_4_;
            fVar36 = fVar37 * (float)local_1c8;
            fVar39 = fVar37 * local_1c8._4_4_;
            fVar34 = (float)local_1c8 * ((float)local_1c8 + (float)local_1c8);
            fVar31 = ((float)local_1c8 + (float)local_1c8) * uStack_1c0._4_4_;
            fVar29 = fVar28 * uStack_1c0._4_4_;
            fVar30 = fVar36 - fVar29;
            fStack_1ac = fVar31 + fVar39;
            fVar27 = 1.0 - (fVar34 + local_1c8._4_4_ * fVar28);
            local_1b8 = (fVar28 * (float)local_1c8 - uStack_1c0._4_4_ * fVar37) * 0.2;
            fStack_1b4 = (1.0 - ((float)uStack_1c0 * fVar37 + fVar34)) * 0.2;
            fStack_1b0 = fStack_1ac * 0.2;
            fStack_1ac = fStack_1ac * 0.2;
            local_1c8 = CONCAT44((fVar28 * (float)local_1c8 + uStack_1c0._4_4_ * fVar37) * 0.2,
                                 (1.0 - (local_1c8._4_4_ * fVar28 + (float)uStack_1c0 * fVar37)) *
                                 0.2);
            uStack_1c0 = (float *)CONCAT44(fVar30 * 0.2,fVar30 * 0.2);
            local_1a8 = CONCAT44(fStack_214 * (fVar39 - fVar31),fStack_214 * (fVar29 + fVar36));
            uStack_1a0 = CONCAT44(fStack_214 * fVar27,fStack_214 * fVar27);
            local_198 = CONCAT44(fVar35,fVar32);
            uStack_190 = (undefined *)CONCAT44(fStack_220,fStack_220);
            country_core::resources::render::impl$1::from();
            pfStack_200 = (float *)CONCAT44(uStack_13c,fStack_140);
            local_208 = local_148;
            local_1f8 = fStack_138;
            fStack_1f4 = fStack_134;
            fStack_1f0 = fStack_130;
            fStack_1ec = fStack_12c;
            local_1e8 = local_128;
            uStack_1e0 = uStack_120;
            local_1d8 = 0;
            uStack_1d0 = 0x200000000;
            country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
            push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
            uVar24 = (ulonglong)(byte)((byte)uVar24 | extraout_AL_02);
            lVar19 = lVar19 + 1;
            pfVar15 = pfVar17 + 3;
            pfVar22 = pfVar17;
          } while (pfVar17 + 3 != pfVar13);
        }
      }
      if (local_268 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      pbVar20 = local_270;
      pfVar13 = local_298;
      if (lStack_250 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      goto LAB_141a429b3;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
LAB_141a42fd0:
  if (local_288 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_290[0x1c] == 0) {
    country_core::resources::roofs::roof_shape::roof_height_ws_from_wall_shape();
    fVar33 = *(float *)(local_290 + 0x18);
    country_core::resources::roofs::roof_shape::ridge_length_01_from_rectangle_ratio();
    country_core::resources::roofs::roof_ridge::RoofRidgeDims::calc();
    local_1c8 = extraout_RAX_02;
    minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
    local_288 = *(longlong *)local_100;
    pfStack_280 = *(float **)(local_100 + 8);
    local_278 = extraout_RAX_03;
    ::utils::geometry::rectangle::Rectangle2d::as_curve2();
    pbVar20 = (byte *)CONCAT44(uStack_25c,fStack_260);
    uVar25 = CONCAT44(fStack_254,fStack_258);
    ::utils::geometry::rectangle::Rectangle2d::as_curve2();
    if (CONCAT44(fStack_1f4,local_1f8) < uVar25) {
      uVar25 = CONCAT44(fStack_1f4,local_1f8);
    }
    local_100 = pbVar20;
    if (uVar25 != 0) {
      lVar19 = lVar19 + uVar25;
      local_f8 = pfStack_200;
      do {
        ::utils::swizzlers::impl$1::xvy();
        ::utils::swizzlers::impl$1::xvy();
        fVar31 = (float)local_350;
        fVar28 = (float)((ulonglong)local_350 >> 0x20);
        fVar29 = (float)local_360;
        fVar32 = (float)((ulonglong)local_360 >> 0x20);
        fStack_140 = local_358 - local_348;
        fVar33 = fVar29 - fVar31;
        fVar27 = fVar32 - fVar28;
        fVar30 = 1.0 / SQRT(fVar27 * fVar27 + fVar33 * fVar33 + fStack_140 * fStack_140);
        fStack_140 = fVar30 * fStack_140;
        local_148 = CONCAT44(fVar30 * fVar27,fVar30 * fVar33);
        ::utils::rotation_from_normal();
        fStack_220 = (local_348 + local_358) * 0.5;
        fStack_214 = SQRT((local_348 - local_358) * (local_348 - local_358) +
                          (fVar28 - fVar32) * (fVar28 - fVar32) +
                          (fVar31 - fVar29) * (fVar31 - fVar29));
        fVar33 = (fVar31 + fVar29) * 0.5;
        fVar27 = (fVar28 + fVar32) * 0.5;
        local_228 = CONCAT44(fVar27,fVar33);
        uStack_21c = 0x3e4ccccd3e4ccccd;
        fVar34 = (float)uStack_1c0 + (float)uStack_1c0;
        fVar28 = local_1c8._4_4_ + local_1c8._4_4_;
        fVar36 = fVar34 * (float)local_1c8;
        fVar39 = fVar34 * local_1c8._4_4_;
        fVar32 = (float)local_1c8 * ((float)local_1c8 + (float)local_1c8);
        fVar31 = ((float)local_1c8 + (float)local_1c8) * uStack_1c0._4_4_;
        fVar29 = fVar28 * uStack_1c0._4_4_;
        fVar35 = fVar36 - fVar29;
        fStack_1ac = fVar31 + fVar39;
        fVar30 = 1.0 - (fVar32 + local_1c8._4_4_ * fVar28);
        local_1b8 = (fVar28 * (float)local_1c8 - uStack_1c0._4_4_ * fVar34) * 0.2;
        fStack_1b4 = (1.0 - ((float)uStack_1c0 * fVar34 + fVar32)) * 0.2;
        fStack_1b0 = fStack_1ac * 0.2;
        fStack_1ac = fStack_1ac * 0.2;
        local_1c8 = CONCAT44((fVar28 * (float)local_1c8 + uStack_1c0._4_4_ * fVar34) * 0.2,
                             (1.0 - (local_1c8._4_4_ * fVar28 + (float)uStack_1c0 * fVar34)) * 0.2);
        uStack_1c0 = (float *)CONCAT44(fVar35 * 0.2,fVar35 * 0.2);
        local_1a8 = CONCAT44(fStack_214 * (fVar39 - fVar31),fStack_214 * (fVar29 + fVar36));
        uStack_1a0 = CONCAT44(fStack_214 * fVar30,fStack_214 * fVar30);
        local_198 = CONCAT44(fVar27,fVar33);
        uStack_190 = (undefined *)CONCAT44(fStack_220,fStack_220);
        country_core::resources::render::impl$1::from();
        uStack_1c0 = (float *)CONCAT44(uStack_13c,fStack_140);
        local_1c8 = local_148;
        local_1b8 = fStack_138;
        fStack_1b4 = fStack_134;
        fStack_1b0 = fStack_130;
        fStack_1ac = fStack_12c;
        local_1a8 = local_128;
        uStack_1a0 = uStack_120;
        local_198 = 0;
        uStack_190 = (undefined *)0x200000000;
        fVar33 = fStack_138;
        country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
        push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
        uVar24 = (ulonglong)(byte)((byte)uVar24 | extraout_AL_03);
        uVar25 = uVar25 - 1;
      } while (uVar25 != 0);
    }
    if (local_208 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    pbVar20 = local_270;
    if (CONCAT44(fStack_1ec,fStack_1f0) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (local_268 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (lStack_250 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    ::utils::geometry::rectangle::Rectangle2d::perimeter();
    if (0.5 < extraout_XMM0_Da) {
      ::utils::geometry::rectangle::Rectangle2d::get_point();
      local_1c8 = CONCAT44(fVar33,extraout_XMM0_Da_00);
      ::utils::swizzlers::impl$1::xvy();
      ::utils::geometry::rectangle::Rectangle2d::get_point();
      local_1c8._0_4_ = extraout_XMM0_Da_01;
      local_1c8._4_4_ = fVar33;
      ::utils::swizzlers::impl$1::xvy();
      fVar28 = (float)local_370;
      fVar29 = (float)((ulonglong)local_370 >> 0x20);
      fVar32 = (float)local_380;
      fVar34 = (float)((ulonglong)local_380 >> 0x20);
      fVar33 = local_378 - local_368;
      fVar27 = fVar32 - fVar28;
      fVar30 = fVar34 - fVar29;
      fVar31 = 1.0 / SQRT(fVar30 * fVar30 + fVar27 * fVar27 + fVar33 * fVar33);
      local_208 = CONCAT44(fVar31 * fVar30,fVar31 * fVar27);
      pfStack_200 = (float *)CONCAT44(pfStack_200._4_4_,fVar31 * fVar33);
      ::utils::rotation_from_normal();
      fStack_260 = (local_368 + local_378) * 0.5;
      fStack_254 = SQRT((local_368 - local_378) * (local_368 - local_378) +
                        (fVar29 - fVar34) * (fVar29 - fVar34) +
                        (fVar28 - fVar32) * (fVar28 - fVar32));
      fVar33 = (fVar28 + fVar32) * 0.5;
      fVar27 = (fVar29 + fVar34) * 0.5;
      local_268 = CONCAT44(fVar27,fVar33);
      uStack_25c = 0x3e4ccccd;
      fStack_258 = 0.2;
      fVar34 = (float)uStack_1c0 + (float)uStack_1c0;
      fVar28 = local_1c8._4_4_ + local_1c8._4_4_;
      fVar36 = fVar34 * (float)local_1c8;
      fVar39 = fVar34 * local_1c8._4_4_;
      fVar32 = (float)local_1c8 * ((float)local_1c8 + (float)local_1c8);
      fVar31 = ((float)local_1c8 + (float)local_1c8) * uStack_1c0._4_4_;
      fVar29 = fVar28 * uStack_1c0._4_4_;
      fVar35 = fVar36 - fVar29;
      fStack_1ac = fVar31 + fVar39;
      fVar30 = 1.0 - (fVar32 + local_1c8._4_4_ * fVar28);
      local_1b8 = (fVar28 * (float)local_1c8 - uStack_1c0._4_4_ * fVar34) * 0.2;
      fStack_1b4 = (1.0 - ((float)uStack_1c0 * fVar34 + fVar32)) * 0.2;
      fStack_1b0 = fStack_1ac * 0.2;
      fStack_1ac = fStack_1ac * 0.2;
      local_1c8 = CONCAT44((fVar28 * (float)local_1c8 + uStack_1c0._4_4_ * fVar34) * 0.2,
                           (1.0 - (local_1c8._4_4_ * fVar28 + (float)uStack_1c0 * fVar34)) * 0.2);
      uStack_1c0 = (float *)CONCAT44(fVar35 * 0.2,fVar35 * 0.2);
      local_1a8 = CONCAT44(fStack_254 * (fVar39 - fVar31),fStack_254 * (fVar29 + fVar36));
      uStack_1a0 = CONCAT44(fStack_254 * fVar30,fStack_254 * fVar30);
      local_198 = CONCAT44(fVar27,fVar33);
      uStack_190 = (undefined *)CONCAT44(fStack_260,fStack_260);
      country_core::resources::render::impl$1::from();
      local_1c8 = local_208;
      uStack_1c0 = pfStack_200;
      local_1b8 = local_1f8;
      fStack_1b4 = fStack_1f4;
      fStack_1b0 = fStack_1f0;
      fStack_1ac = fStack_1ec;
      local_1a8 = local_1e8;
      uStack_1a0 = uStack_1e0;
      local_198 = 0;
      uStack_190 = (undefined *)0x200000000;
      country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
      push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
      uVar24 = (ulonglong)(byte)((byte)uVar24 | extraout_AL_04);
      lVar19 = lVar19 + 1;
    }
  }
  *puVar7 = local_29c;
  *(longlong *)(lVar6 + 8) = lVar19;
  goto LAB_141a41f69;
}


