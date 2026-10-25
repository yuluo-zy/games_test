
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl system_color::set_halftimber_color_id::set_halftimber_color_id(void)

{
  longlong lVar1;
  uint *puVar2;
  longlong *plVar3;
  uint uVar4;
  uint uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  ulonglong *puVar9;
  longlong lVar10;
  longlong lVar11;
  undefined4 *puVar12;
  longlong lVar13;
  undefined4 *puVar14;
  longlong lVar15;
  undefined8 uVar16;
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  int iVar19;
  ulonglong uVar20;
  undefined8 *extraout_RAX;
  longlong lVar21;
  longlong lVar22;
  ulonglong extraout_RAX_00;
  longlong extraout_RAX_01;
  uint *puVar23;
  longlong lVar24;
  longlong lVar25;
  longlong lVar26;
  undefined8 *in_R8;
  ulonglong uVar27;
  ulonglong uVar28;
  ulonglong uVar29;
  undefined1 auVar30 [16];
  undefined1 auVar31 [16];
  float fVar32;
  float fVar33;
  float fVar34;
  float fVar35;
  undefined1 auVar36 [16];
  undefined1 auVar37 [16];
  undefined1 auVar38 [16];
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  longlong local_f0;
  longlong lStack_e8;
  undefined4 uStack_e0;
  
  puVar9 = (ulonglong *)*in_R8;
  lVar10 = in_R8[1];
  uVar27 = 0;
  if (*puVar9 <= *(ulonglong *)(lVar10 + 0x40)) {
    uVar27 = *(ulonglong *)(lVar10 + 0x40) - *puVar9;
  }
  uVar20 = *(longlong *)(lVar10 + 0x30) + *(longlong *)(lVar10 + 0x10);
  if (uVar27 <= uVar20) {
    uVar20 = uVar27;
  }
  if (uVar20 == 0) {
                    // WARNING: Read-only address (ram,0x000142ac2e30) is written
    return;
  }
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX != (undefined8 *)0x0) {
    if ((extraout_RAX[3] != 0x38) ||
       (iVar19 = memcmp(&anon_a49387b4b2f61cd3615a99b166e92ad9_1_llvm_6175655424687704485,
                        (void *)extraout_RAX[2],0x38), iVar19 != 0)) {
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    lVar21 = (**(code **)(extraout_RAX[1] + 0x18))(*extraout_RAX);
    if (lVar21 != 0) {
      uVar27 = *puVar9;
      uVar28 = 0;
      uVar20 = uVar27 - *(ulonglong *)(lVar10 + 0x18);
      if (uVar27 < *(ulonglong *)(lVar10 + 0x18)) {
        uVar20 = uVar28;
      }
      uVar29 = uVar27 - *(ulonglong *)(lVar10 + 0x38);
      if (uVar27 < *(ulonglong *)(lVar10 + 0x38)) {
        uVar29 = uVar28;
      }
      lVar25 = uVar20 * 0x18 + *(longlong *)(lVar10 + 8);
      uVar27 = *(ulonglong *)(lVar10 + 0x10) - uVar20;
      if (*(ulonglong *)(lVar10 + 0x10) < uVar20) {
        lVar25 = 8;
        uVar27 = uVar28;
      }
      lVar22 = uVar29 * 0x18 + *(longlong *)(lVar10 + 0x28);
      uVar20 = *(ulonglong *)(lVar10 + 0x30) - uVar29;
      if (*(ulonglong *)(lVar10 + 0x30) < uVar29) {
        lVar22 = 8;
        uVar20 = uVar28;
      }
      *puVar9 = (*(longlong *)(lVar10 + 0x40) - uVar20) - uVar27;
      auVar18 = ___xmm_00000000000000007f7fffff7f7fffff;
      lVar10 = lVar25 + uVar27 * 0x18;
      lVar1 = lVar22 + uVar20 * 0x18;
      uVar6 = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
      lVar11 = *in_stack_00000030;
      puVar12 = (undefined4 *)in_stack_00000030[2];
      uVar7 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
      lVar13 = *in_stack_00000028;
      puVar14 = (undefined4 *)in_stack_00000028[2];
      while( true ) {
        if ((lVar25 == 0) || (lVar25 == lVar10)) {
          if ((lVar22 == 0) || (lVar22 == lVar1)) {
            country_core::render::impl$0::deref();
            if (*(longlong *)(lVar11 + 0x10) == 0) {
              return;
            }
            if (((ulonglong)(*(longlong *)(lVar11 + 0x10) << 2) <= *(ulonglong *)(lVar21 + 0x18)) &&
               (rhapsody::raw::device::GraphicsDevice::update_buffer_bytes(), local_f0 == 0xb)) {
              return;
            }
                    // WARNING: Subroutine does not return
            core::result::unwrap_failed();
          }
          lVar25 = 0;
          lVar15 = lVar22 + 0x18;
          lVar26 = lVar22;
        }
        else {
          lVar15 = lVar22;
          lVar26 = lVar25;
          lVar25 = lVar25 + 0x18;
        }
        lVar22 = lVar15;
        *puVar9 = *puVar9 + 1;
        minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
        if (0x4fff < extraout_RAX_00) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
        uVar8 = *(undefined4 *)(lVar26 + 0x10);
        *puVar12 = uVar6;
        if (*(ulonglong *)(lVar11 + 0x10) <= extraout_RAX_00) break;
        *(undefined4 *)(*(longlong *)(lVar11 + 8) + extraout_RAX_00 * 4) = uVar8;
        if ((*(char *)(lVar26 + 0x14) == '\x01') &&
           (country_core::resources::walls::public_walls::PublicWalls::get(), extraout_RAX_01 != 0))
        {
          if (*(longlong *)(extraout_RAX_01 + 0x48) != 0) {
            puVar23 = *(uint **)(extraout_RAX_01 + 0x40);
            puVar2 = puVar23 + *(longlong *)(extraout_RAX_01 + 0x48) * 3;
            auVar31 = auVar18;
            fVar32 = -3.4028235e+38;
            fVar33 = -3.4028235e+38;
            fVar34 = 0.0;
            fVar35 = 0.0;
            do {
              uVar4 = *puVar23;
              uVar5 = puVar23[2];
              auVar36._4_4_ = uVar5;
              auVar36._0_4_ = uVar4;
              auVar36._8_8_ = 0;
              auVar37._4_4_ = uVar5;
              auVar37._0_4_ = uVar4;
              auVar37._8_8_ = 0;
              auVar38 = minps(auVar37,auVar31);
              auVar30._0_4_ = ~-(uint)NAN(auVar31._0_4_) & auVar38._0_4_;
              auVar30._4_4_ = ~-(uint)NAN(auVar31._4_4_) & auVar38._4_4_;
              auVar30._8_4_ = ~-(uint)NAN(auVar31._8_4_) & auVar38._8_4_;
              auVar30._12_4_ = ~-(uint)NAN(auVar31._12_4_) & auVar38._12_4_;
              auVar17._4_4_ = uVar5 & -(uint)NAN(auVar31._4_4_);
              auVar17._0_4_ = uVar4 & -(uint)NAN(auVar31._0_4_);
              auVar17._8_8_ = 0;
              auVar31 = auVar30 | auVar17;
              auVar38._4_4_ = fVar33;
              auVar38._0_4_ = fVar32;
              auVar38._8_4_ = fVar34;
              auVar38._12_4_ = fVar35;
              auVar38 = maxps(auVar36,auVar38);
              fVar34 = (float)(~-(uint)NAN(fVar34) & auVar38._8_4_);
              fVar35 = (float)(~-(uint)NAN(fVar35) & auVar38._12_4_);
              fVar32 = (float)(~-(uint)NAN(fVar32) & auVar38._0_4_ | uVar4 & -(uint)NAN(fVar32));
              fVar33 = (float)(~-(uint)NAN(fVar33) & auVar38._4_4_ | uVar5 & -(uint)NAN(fVar33));
              puVar23 = puVar23 + 3;
            } while (puVar23 != puVar2);
          }
          ::utils::geometry::aabb::Aabb2::center();
          country_core::resources::walls::public_wall_state::PublicWallState::max_y();
          ::utils::swizzlers::impl$1::xvy();
          local_f0 = CONCAT71(local_f0._1_7_,0xc);
          *puVar14 = uVar7;
          lVar15 = *(longlong *)(lVar13 + 0x30);
          uVar16 = *(undefined8 *)(lVar13 + 0x40);
          if (lVar15 == *(longlong *)(lVar13 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                      ();
          }
          lVar26 = *(longlong *)(lVar13 + 0x28);
          lVar24 = lVar15 * 0x20;
          *(undefined8 *)(lVar26 + lVar24) = uVar16;
          *(undefined4 *)(lVar26 + 0x18 + lVar24) = uStack_e0;
          plVar3 = (longlong *)(lVar26 + 8 + lVar24);
          *plVar3 = local_f0;
          plVar3[1] = lStack_e8;
          *(longlong *)(lVar13 + 0x30) = lVar15 + 1;
          *(longlong *)(lVar13 + 0x40) = *(longlong *)(lVar13 + 0x40) + 1;
        }
      }
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

