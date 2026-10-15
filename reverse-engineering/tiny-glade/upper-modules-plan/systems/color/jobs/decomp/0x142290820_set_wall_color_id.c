// name=set_wall_color_id
// sig=void __cdecl set_wall_color_id(void)
// size=1817

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl system_color::set_wall_color_id::set_wall_color_id(void)

{
  longlong *plVar1;
  longlong *plVar2;
  longlong *plVar3;
  longlong *plVar4;
  uint *puVar5;
  uint uVar6;
  uint uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  int iVar13;
  ulonglong *puVar14;
  longlong lVar15;
  longlong lVar16;
  undefined4 *puVar17;
  longlong lVar18;
  undefined4 *puVar19;
  longlong lVar20;
  undefined4 *puVar21;
  longlong lVar22;
  undefined4 *puVar23;
  longlong lVar24;
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  undefined1 extraout_AL;
  int iVar27;
  ulonglong uVar28;
  undefined8 *extraout_RAX;
  longlong lVar29;
  longlong *plVar30;
  longlong lVar31;
  ulonglong extraout_RAX_00;
  longlong extraout_RAX_01;
  uint *puVar32;
  longlong *plVar33;
  longlong lVar34;
  ulonglong uVar35;
  longlong lVar36;
  longlong lVar37;
  undefined8 uVar38;
  undefined8 *in_R8;
  ulonglong uVar39;
  ulonglong uVar40;
  longlong *in_R9;
  longlong *plVar41;
  longlong lVar42;
  undefined1 auVar43 [16];
  undefined1 auVar44 [16];
  float fVar45;
  float fVar46;
  float fVar47;
  float fVar48;
  undefined1 auVar49 [16];
  undefined1 auVar50 [16];
  undefined1 auVar51 [16];
  longlong *in_stack_00000028;
  longlong *in_stack_00000038;
  longlong *in_stack_00000040;
  undefined8 local_128;
  longlong lStack_120;
  undefined4 uStack_118;
  longlong local_c8;
  
  puVar14 = (ulonglong *)*in_R8;
  lVar15 = in_R8[1];
  uVar39 = 0;
  if (*puVar14 <= *(ulonglong *)(lVar15 + 0x40)) {
    uVar39 = *(ulonglong *)(lVar15 + 0x40) - *puVar14;
  }
  uVar28 = *(longlong *)(lVar15 + 0x30) + *(longlong *)(lVar15 + 0x10);
  if (uVar39 <= uVar28) {
    uVar28 = uVar39;
  }
  if (uVar28 == 0) {
                    // WARNING: Read-only address (ram,0x000142ac2e30) is written
    return;
  }
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX != (undefined8 *)0x0) {
    if ((extraout_RAX[3] != 0x32) ||
       (iVar27 = memcmp(&anon_a49387b4b2f61cd3615a99b166e92ad9_0_llvm_6175655424687704485,
                        (void *)extraout_RAX[2],0x32), iVar27 != 0)) {
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    lVar29 = (**(code **)(extraout_RAX[1] + 0x18))(*extraout_RAX);
    if (lVar29 != 0) {
      uVar39 = *puVar14;
      uVar35 = 0;
      uVar28 = uVar39 - *(ulonglong *)(lVar15 + 0x18);
      if (uVar39 < *(ulonglong *)(lVar15 + 0x18)) {
        uVar28 = uVar35;
      }
      uVar40 = uVar39 - *(ulonglong *)(lVar15 + 0x38);
      if (uVar39 < *(ulonglong *)(lVar15 + 0x38)) {
        uVar40 = uVar35;
      }
      lVar36 = uVar28 * 0x18 + *(longlong *)(lVar15 + 8);
      uVar39 = *(ulonglong *)(lVar15 + 0x10) - uVar28;
      if (*(ulonglong *)(lVar15 + 0x10) < uVar28) {
        lVar36 = 8;
        uVar39 = uVar35;
      }
      local_c8 = uVar40 * 0x18 + *(longlong *)(lVar15 + 0x28);
      uVar28 = *(ulonglong *)(lVar15 + 0x30) - uVar40;
      if (*(ulonglong *)(lVar15 + 0x30) < uVar40) {
        local_c8 = 8;
        uVar28 = uVar35;
      }
      *puVar14 = (*(longlong *)(lVar15 + 0x40) - uVar28) - uVar39;
      auVar26 = ___xmm_00000000000000007f7fffff7f7fffff;
      lVar15 = local_c8 + uVar28 * 0x18;
      lVar16 = *in_stack_00000028;
      puVar17 = (undefined4 *)in_stack_00000028[2];
      uVar8 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
      uVar9 = *(undefined4 *)((longlong)in_R9 + 0x1c);
      lVar18 = *in_R9;
      puVar19 = (undefined4 *)in_R9[2];
      uVar10 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
      lVar20 = *in_stack_00000038;
      puVar21 = (undefined4 *)in_stack_00000038[2];
      plVar1 = (longlong *)(lVar20 + 0x40);
      plVar2 = (longlong *)(lVar20 + 0x30);
      uVar11 = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
      lVar22 = *in_stack_00000040;
      puVar23 = (undefined4 *)in_stack_00000040[2];
      plVar3 = (longlong *)(lVar22 + 0x40);
      plVar4 = (longlong *)(lVar22 + 0x30);
      lVar37 = lVar36;
LAB_142290ab7:
      do {
        if ((lVar37 == 0) || (lVar37 == lVar36 + uVar39 * 0x18)) {
          if ((local_c8 == 0) || (local_c8 == lVar15)) {
            country_core::render::impl$0::deref();
            if (*(longlong *)(lVar16 + 0x10) == 0) {
              return;
            }
            if (((ulonglong)(*(longlong *)(lVar16 + 0x10) << 2) <= *(ulonglong *)(lVar29 + 0x18)) &&
               (rhapsody::raw::device::GraphicsDevice::update_buffer_bytes(), local_128 == 0xb)) {
              return;
            }
                    // WARNING: Subroutine does not return
            core::result::unwrap_failed();
          }
          lVar31 = 0;
          lVar42 = local_c8;
          local_c8 = local_c8 + 0x18;
        }
        else {
          lVar31 = lVar37 + 0x18;
          lVar42 = lVar37;
        }
        *puVar14 = *puVar14 + 1;
        minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
        if (0x4fff < extraout_RAX_00) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
        if (*(ulonglong *)(lVar16 + 0x10) <= extraout_RAX_00) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        iVar27 = *(int *)(*(longlong *)(lVar16 + 8) + extraout_RAX_00 * 4);
        uVar12 = *(undefined4 *)(lVar42 + 0x10);
        *puVar17 = uVar8;
        if (*(ulonglong *)(lVar16 + 0x10) <= extraout_RAX_00) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        *(undefined4 *)(*(longlong *)(lVar16 + 8) + extraout_RAX_00 * 4) = uVar12;
        country_core::resources::walls::public_walls::PublicWalls::get();
        lVar37 = lVar31;
      } while (extraout_RAX_01 == 0);
      if (*(char *)(lVar42 + 0x14) != '\0') {
        country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered();
        if (*(longlong *)(extraout_RAX_01 + 0x48) != 0) {
          puVar32 = *(uint **)(extraout_RAX_01 + 0x40);
          puVar5 = puVar32 + *(longlong *)(extraout_RAX_01 + 0x48) * 3;
          auVar44 = auVar26;
          fVar45 = -3.4028235e+38;
          fVar46 = -3.4028235e+38;
          fVar47 = 0.0;
          fVar48 = 0.0;
          do {
            uVar6 = *puVar32;
            uVar7 = puVar32[2];
            auVar49._4_4_ = uVar7;
            auVar49._0_4_ = uVar6;
            auVar49._8_8_ = 0;
            auVar50._4_4_ = uVar7;
            auVar50._0_4_ = uVar6;
            auVar50._8_8_ = 0;
            auVar51 = minps(auVar50,auVar44);
            auVar43._0_4_ = ~-(uint)NAN(auVar44._0_4_) & auVar51._0_4_;
            auVar43._4_4_ = ~-(uint)NAN(auVar44._4_4_) & auVar51._4_4_;
            auVar43._8_4_ = ~-(uint)NAN(auVar44._8_4_) & auVar51._8_4_;
            auVar43._12_4_ = ~-(uint)NAN(auVar44._12_4_) & auVar51._12_4_;
            auVar25._4_4_ = uVar7 & -(uint)NAN(auVar44._4_4_);
            auVar25._0_4_ = uVar6 & -(uint)NAN(auVar44._0_4_);
            auVar25._8_8_ = 0;
            auVar44 = auVar43 | auVar25;
            auVar51._4_4_ = fVar46;
            auVar51._0_4_ = fVar45;
            auVar51._8_4_ = fVar47;
            auVar51._12_4_ = fVar48;
            auVar51 = maxps(auVar49,auVar51);
            fVar47 = (float)(~-(uint)NAN(fVar47) & auVar51._8_4_);
            fVar48 = (float)(~-(uint)NAN(fVar48) & auVar51._12_4_);
            fVar45 = (float)(~-(uint)NAN(fVar45) & auVar51._0_4_ | uVar6 & -(uint)NAN(fVar45));
            fVar46 = (float)(~-(uint)NAN(fVar46) & auVar51._4_4_ | uVar7 & -(uint)NAN(fVar46));
            puVar32 = puVar32 + 3;
          } while (puVar32 != puVar5);
        }
        ::utils::geometry::aabb::Aabb2::center();
        country_core::resources::walls::public_wall_state::PublicWallState::max_y();
        ::utils::swizzlers::impl$1::xvy();
        local_128._0_2_ = CONCAT11(extraout_AL,6);
        *puVar19 = uVar9;
        lVar31 = *(longlong *)(lVar18 + 0x30);
        uVar38 = *(undefined8 *)(lVar18 + 0x40);
        if (lVar31 == *(longlong *)(lVar18 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                    ();
        }
        lVar24 = *(longlong *)(lVar18 + 0x28);
        lVar34 = lVar31 * 0x20;
        *(undefined8 *)(lVar24 + lVar34) = uVar38;
        *(undefined4 *)(lVar24 + 0x18 + lVar34) = uStack_118;
        plVar30 = (longlong *)(lVar24 + 8 + lVar34);
        *plVar30 = local_128;
        plVar30[1] = lStack_120;
        *(longlong *)(lVar18 + 0x30) = lVar31 + 1;
        *(longlong *)(lVar18 + 0x40) = *(longlong *)(lVar18 + 0x40) + 1;
      }
      iVar13 = *(int *)(lVar42 + 0x10);
      if ((6 < iVar13 - 4U) && (iVar13 != 0xc)) goto code_r0x000142290d10;
      uVar38 = *(undefined8 *)(lVar42 + 8);
      *puVar21 = uVar10;
      lVar31 = *plVar1;
      lVar42 = *plVar2;
      plVar30 = (longlong *)(lVar20 + 0x28);
      plVar33 = plVar2;
      plVar41 = plVar1;
      if (lVar42 == *(longlong *)(lVar20 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      goto LAB_142290a9a;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
code_r0x000142290d10:
  if (((iVar27 - 4U < 7) || (iVar27 == 0xc)) && (iVar13 != 0xc)) {
    uVar38 = *(undefined8 *)(lVar42 + 8);
    *puVar23 = uVar11;
    lVar31 = *plVar3;
    lVar42 = *plVar4;
    plVar30 = (longlong *)(lVar22 + 0x28);
    plVar33 = plVar4;
    plVar41 = plVar3;
    if (lVar42 == *(longlong *)(lVar22 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                ();
    }
LAB_142290a9a:
    lVar24 = *plVar30;
    *(longlong *)(lVar24 + lVar42 * 0x10) = lVar31;
    *(undefined8 *)(lVar24 + 8 + lVar42 * 0x10) = uVar38;
    *plVar33 = lVar42 + 1;
    *plVar41 = *plVar41 + 1;
  }
  goto LAB_142290ab7;
}

