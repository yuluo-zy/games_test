// name=ui_color_gates
// sig=void __cdecl ui_color_gates(void)
// size=2600

void __cdecl system_color::ui_color_gates::ui_color_gates(void)

{
  ulonglong *puVar1;
  undefined8 uVar2;
  undefined1 auVar3 [16];
  uint uVar4;
  longlong lVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  ushort uVar8;
  int iVar9;
  char extraout_AL;
  char extraout_AL_00;
  uint uVar10;
  longlong extraout_RAX;
  ulonglong extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined1 *_Size;
  undefined1 *extraout_RAX_02;
  undefined1 *_Dst;
  undefined8 *extraout_RAX_03;
  undefined8 *in_RCX;
  longlong lVar11;
  ulonglong uVar12;
  undefined8 *in_RDX;
  ulonglong uVar13;
  undefined ****ppppuVar14;
  void *_Src;
  undefined8 ***pppuVar15;
  ulonglong *puVar16;
  undefined8 *in_R8;
  longlong lVar17;
  undefined8 *in_R9;
  longlong lVar18;
  undefined1 (*pauVar19) [16];
  undefined1 (*pauVar20) [16];
  bool bVar21;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  undefined8 *in_stack_00000038;
  longlong *in_stack_00000040;
  longlong *in_stack_00000048;
  undefined8 *in_stack_00000050;
  undefined ***local_288;
  undefined8 uStack_280;
  ulonglong local_278;
  undefined8 **local_270;
  undefined *puStack_268;
  undefined8 ***local_260;
  undefined *puStack_258;
  undefined8 local_250;
  void *local_1c0;
  undefined8 local_1b8;
  undefined8 local_1b0;
  ulonglong local_1a8;
  ulonglong local_1a0;
  longlong local_190;
  undefined8 local_188;
  undefined8 local_180;
  undefined8 local_178;
  undefined4 *local_170;
  undefined4 *local_168;
  longlong local_160;
  longlong local_158;
  ulonglong local_150;
  undefined ****local_148;
  undefined8 local_140;
  undefined8 local_138;
  undefined8 *local_130;
  longlong local_128;
  longlong local_120;
  undefined ***local_118;
  undefined8 uStack_110;
  ulonglong local_108;
  undefined4 local_f8;
  undefined4 local_f4;
  longlong local_f0;
  longlong local_e8;
  longlong local_e0;
  undefined *local_d8;
  undefined8 ***pppuStack_d0;
  undefined8 *local_c8;
  undefined8 *local_c0;
  undefined8 **local_b8;
  code *pcStack_b0;
  undefined ****ppppuStack_a8;
  code *pcStack_a0;
  undefined8 local_98;
  uint local_8c;
  undefined1 *local_88;
  void *local_80;
  ulonglong local_78;
  ulonglong local_70;
  undefined1 local_63;
  undefined1 local_62;
  undefined1 local_61;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  if (*(int *)*in_RCX == 0xc) {
    pauVar19 = *(undefined1 (**) [16])*in_R9;
    local_190 = ((undefined8 *)*in_R9)[3];
    auVar3 = *pauVar19;
    local_8c = ~(uint)(ushort)((ushort)(SUB161(auVar3 >> 7,0) & 1) |
                               (ushort)(SUB161(auVar3 >> 0xf,0) & 1) << 1 |
                               (ushort)(SUB161(auVar3 >> 0x17,0) & 1) << 2 |
                               (ushort)(SUB161(auVar3 >> 0x1f,0) & 1) << 3 |
                               (ushort)(SUB161(auVar3 >> 0x27,0) & 1) << 4 |
                               (ushort)(SUB161(auVar3 >> 0x2f,0) & 1) << 5 |
                               (ushort)(SUB161(auVar3 >> 0x37,0) & 1) << 6 |
                               (ushort)(SUB161(auVar3 >> 0x3f,0) & 1) << 7 |
                               (ushort)(SUB161(auVar3 >> 0x47,0) & 1) << 8 |
                               (ushort)(SUB161(auVar3 >> 0x4f,0) & 1) << 9 |
                               (ushort)(SUB161(auVar3 >> 0x57,0) & 1) << 10 |
                               (ushort)(SUB161(auVar3 >> 0x5f,0) & 1) << 0xb |
                               (ushort)(SUB161(auVar3 >> 0x67,0) & 1) << 0xc |
                               (ushort)(SUB161(auVar3 >> 0x6f,0) & 1) << 0xd |
                               (ushort)(SUB161(auVar3 >> 0x77,0) & 1) << 0xe |
                              (ushort)(byte)(auVar3[0xf] >> 7) << 0xf);
    local_e8 = *in_stack_00000028;
    lVar5 = in_stack_00000028[1];
    local_188 = *in_stack_00000038;
    local_178 = *in_RDX;
    local_130 = (undefined8 *)*in_stack_00000050;
    local_140 = *in_R8;
    local_f0 = *in_stack_00000030;
    local_e0 = in_stack_00000030[1];
    local_f8 = *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
    local_128 = *in_stack_00000048;
    local_170 = (undefined4 *)in_stack_00000048[2];
    local_f4 = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
    local_120 = *in_stack_00000040;
    local_168 = (undefined4 *)in_stack_00000040[2];
    pauVar20 = pauVar19 + 1;
    local_160 = local_128 + 0x20;
    local_158 = local_120 + 0x20;
    while (bVar21 = local_190 != 0, local_190 = local_190 + -1, bVar21) {
      if ((short)local_8c == 0) {
        do {
          auVar3 = *pauVar20;
          uVar8 = (ushort)(SUB161(auVar3 >> 7,0) & 1) | (ushort)(SUB161(auVar3 >> 0xf,0) & 1) << 1 |
                  (ushort)(SUB161(auVar3 >> 0x17,0) & 1) << 2 |
                  (ushort)(SUB161(auVar3 >> 0x1f,0) & 1) << 3 |
                  (ushort)(SUB161(auVar3 >> 0x27,0) & 1) << 4 |
                  (ushort)(SUB161(auVar3 >> 0x2f,0) & 1) << 5 |
                  (ushort)(SUB161(auVar3 >> 0x37,0) & 1) << 6 |
                  (ushort)(SUB161(auVar3 >> 0x3f,0) & 1) << 7 |
                  (ushort)(SUB161(auVar3 >> 0x47,0) & 1) << 8 |
                  (ushort)(SUB161(auVar3 >> 0x4f,0) & 1) << 9 |
                  (ushort)(SUB161(auVar3 >> 0x57,0) & 1) << 10 |
                  (ushort)(SUB161(auVar3 >> 0x5f,0) & 1) << 0xb |
                  (ushort)(SUB161(auVar3 >> 0x67,0) & 1) << 0xc |
                  (ushort)(SUB161(auVar3 >> 0x6f,0) & 1) << 0xd |
                  (ushort)(SUB161(auVar3 >> 0x77,0) & 1) << 0xe |
                  (ushort)(byte)(auVar3[0xf] >> 7) << 0xf;
          pauVar19 = pauVar19 + -0x20;
          pauVar20 = pauVar20 + 1;
        } while (uVar8 == 0xffff);
        uVar10 = ~(uint)uVar8;
        local_8c = uVar10 & -(uint)uVar8 - 2;
      }
      else {
        uVar10 = local_8c;
        local_8c = local_8c - 1 & local_8c;
      }
      iVar9 = 0;
      for (; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
        iVar9 = iVar9 + 1;
      }
      uVar10 = iVar9 << 5;
      local_c0 = (undefined8 *)((longlong)pauVar19 + (-0x20 - (ulonglong)uVar10));
      local_d8 = (undefined *)0x0;
      pppuStack_d0 = (undefined8 ***)&DAT_00000004;
      local_c8 = (undefined8 *)0x0;
      puVar16 = *(ulonglong **)((longlong)pauVar19 + (-0x10 - (ulonglong)uVar10));
      puVar1 = puVar16 + *(longlong *)((longlong)pauVar19 + (-8 - (ulonglong)uVar10));
      lVar18 = local_e8;
      while (puVar16 != puVar1) {
        uVar13 = *puVar16;
        puVar16 = puVar16 + 1;
        uVar12 = uVar13 & 0xffffffff;
        if ((uVar12 < *(ulonglong *)(lVar5 + 0x10)) &&
           (lVar17 = *(longlong *)(lVar5 + 8),
           *(int *)(lVar17 + uVar12 * 0x14) == (int)(uVar13 >> 0x20))) {
          uVar4 = *(uint *)(lVar17 + uVar12 * 0x14 + 4);
          uVar13 = (ulonglong)uVar4;
          if ((uVar13 != 0xffffffff) &&
             ((uVar13 < *(ulonglong *)(lVar18 + 0x38) &&
              ((*(ulonglong *)(*(longlong *)(lVar18 + 0x28) + (ulonglong)(uVar4 >> 6) * 8) >>
                (uVar13 & 0x3f) & 1) != 0)))) {
            local_61 = 1;
            country_core::asset_libraries::mesh_library::AssetMeshLibrary::get();
            if (extraout_RAX == 0) {
              local_61 = 1;
                    // WARNING: Subroutine does not return
              core::option::unwrap_failed();
            }
            uVar13 = *(ulonglong *)(extraout_RAX + 0x10);
            local_61 = 1;
            country_core::render::mesh::Mesh::positions();
            uVar13 = uVar13 / 3;
            if ((ulonglong)((longlong)local_d8 - (longlong)local_c8) < uVar13) {
              local_61 = 1;
              alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
            }
            local_118 = (undefined ***)&local_270;
            uStack_110 = 0;
            local_b8 = &local_c8;
            pcStack_b0 = (code *)local_c8;
            ppppuStack_a8 = (undefined ****)pppuStack_d0;
            local_61 = 1;
            local_108 = uVar13;
            core::iter::adapters::map::impl$2::
            fold<utils::geometry::intersect::Triangle,core::ops::range::Range<usize>,system_color::ui_color_gates::ui_color_gates::closure_env_0,tuple_<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env_0<utils::geometry::intersect::Triangle,alloc::vec::impl_20::extend_trusted::closure_env_0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_color::ui_color_gates::ui_color_gates::closure_env_0>_>_>_>
                      ();
            lVar18 = local_e8;
          }
        }
      }
      local_b8 = &local_c0;
      pcStack_b0 = core::fmt::impl$74::fmt<country_core::resources::walls::WallId>;
      local_270 = (undefined8 **)&PTR_s_color_gates__14303cca0;
      puStack_268 = (undefined *)0x1;
      local_260 = &local_b8;
      puStack_258 = (undefined *)0x1;
      local_250 = 0;
      local_61 = 1;
      alloc::fmt::format::format_inner();
      local_118 = local_288;
      uStack_110 = uStack_280;
      local_108 = local_278;
      pcStack_a0 = (code *)local_c8;
      pcStack_b0 = (code *)local_d8;
      ppppuStack_a8 = (undefined ****)pppuStack_d0;
      local_b8 = (undefined8 **)CONCAT71(local_b8._1_7_,2);
      local_150 = local_150 & 0xffffffffffffff00;
      local_62 = 1;
      country_core::systems::ui::resources::widget_params::WidgetParams::new();
      local_62 = 0;
      ppppuVar14 = &local_118;
      country_core::systems::ui::resources::ui_state::UiState::add<alloc::string::String>();
      local_61 = 0;
      local_150 = extraout_RAX_00;
      local_148 = ppppuVar14;
      country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active();
      if (extraout_AL != '\0') {
        uVar2 = *local_130;
        local_b8 = &local_c0;
        pcStack_b0 = core::fmt::impl$74::fmt<country_core::resources::walls::WallId>;
        local_270 = (undefined8 **)&DAT_14303ccc8;
        puStack_268 = (undefined *)0x1;
        local_260 = &local_b8;
        puStack_258 = (undefined *)0x1;
        local_250 = 0;
        local_61 = 0;
        alloc::fmt::format::format_inner();
        uVar13 = local_1a0;
        local_78 = local_1a8;
        std::alloc::__default_lib_allocator::__rust_alloc();
        local_70 = uVar13;
        if (extraout_RAX_01 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
          alloc::alloc::handle_alloc_error();
        }
        *extraout_RAX_01 = 1;
        extraout_RAX_01[1] = 1;
        extraout_RAX_01[2] = uVar2;
        country_core::systems::ui::signifiers::SignifierFactory::new();
        local_98 = local_250;
        ppppuStack_a8 = (undefined ****)local_260;
        pcStack_a0 = (code *)puStack_258;
        local_b8 = local_270;
        pcStack_b0 = (code *)puStack_268;
        country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write();
        if (local_78 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        puVar16 = *(ulonglong **)((longlong)pauVar19 + (-0x10 - (ulonglong)uVar10));
        puVar1 = puVar16 + *(longlong *)((longlong)pauVar19 + (-8 - (ulonglong)uVar10));
        pppuVar15 = (undefined8 ***)0x0;
        lVar18 = local_e0;
        for (; puVar16 != puVar1; puVar16 = puVar16 + 1) {
          uVar13 = *puVar16 & 0xffffffff;
          local_118 = (undefined ***)pppuVar15;
          if ((uVar13 < *(ulonglong *)(lVar18 + 0x10)) &&
             (*(int *)(*(longlong *)(lVar18 + 8) + uVar13 * 0x14) == (int)(*puVar16 >> 0x20))) {
            lVar17 = *(longlong *)(lVar18 + 8) + uVar13 * 0x14;
            uVar13 = (ulonglong)*(uint *)(lVar17 + 4);
            if ((uVar13 != 0xffffffff) &&
               ((uVar13 < *(ulonglong *)(local_f0 + 0x38) &&
                ((*(ulonglong *)
                   (*(longlong *)(local_f0 + 0x28) + (ulonglong)(*(uint *)(lVar17 + 4) >> 6) * 8) >>
                  (uVar13 & 0x3f) & 1) != 0)))) {
              lVar18 = *(longlong *)
                        (*(longlong *)
                          (*(longlong *)(lVar18 + 0x1a8) + 0x18 +
                          (ulonglong)*(uint *)(lVar17 + 0xc) * 0x48) + 0x10 +
                        ~*(ulonglong *)
                          (*(longlong *)
                            (*(longlong *)(lVar18 + 0x1a8) + 0x38 +
                            (ulonglong)*(uint *)(lVar17 + 0xc) * 0x48) +
                          *(longlong *)(local_f0 + 0x110) * 8) * 0x30);
              lVar17 = (ulonglong)*(uint *)(lVar17 + 0x10) * 0x38;
              local_78 = *(ulonglong *)(lVar18 + 0x10 + lVar17);
              auVar3._8_8_ = 0;
              auVar3._0_8_ = local_78;
              _Size = SUB168(auVar3 * ZEXT816(0xc),0);
              if ((SUB168(auVar3 * ZEXT816(0xc),8) != 0) ||
                 ((undefined1 *)0x7ffffffffffffffc < _Size)) {
LAB_1422b08a0:
                local_61 = 0;
                    // WARNING: Subroutine does not return
                alloc::raw_vec::handle_error();
              }
              _Src = *(void **)(lVar18 + 8 + lVar17);
              if (_Size == (undefined1 *)0x0) {
                _Dst = &DAT_00000004;
                local_70 = 0;
              }
              else {
                local_88 = _Size;
                local_80 = _Src;
                std::alloc::__default_lib_allocator::__rust_alloc();
                if (extraout_RAX_02 == (undefined1 *)0x0) goto LAB_1422b08a0;
                local_70 = local_78;
                _Dst = extraout_RAX_02;
                _Src = local_80;
                _Size = local_88;
              }
              local_88 = _Dst;
              memcpy(_Dst,_Src,(size_t)_Size);
              local_b8 = &local_c0;
              pcStack_b0 = core::fmt::impl$74::fmt<country_core::resources::walls::WallId>;
              ppppuStack_a8 = &local_118;
              pcStack_a0 = core::fmt::num::impl$89::fmt;
              local_270 = (undefined8 **)&DAT_14303cd40;
              puStack_268 = (undefined *)0x2;
              local_260 = &local_b8;
              puStack_258 = (undefined *)0x2;
              local_250 = 0;
              local_63 = 1;
              alloc::fmt::format::format_inner();
              uVar2 = local_1b8;
              local_80 = local_1c0;
              local_180 = local_1b0;
              std::alloc::__default_lib_allocator::__rust_alloc();
              local_138 = uVar2;
              if (extraout_RAX_03 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
                alloc::alloc::handle_alloc_error();
              }
              *extraout_RAX_03 = 1;
              extraout_RAX_03[1] = 1;
              extraout_RAX_03[2] = local_70;
              extraout_RAX_03[3] = local_88;
              extraout_RAX_03[4] = local_78;
              country_core::systems::ui::signifiers::SignifierFactory::new();
              local_98 = local_250;
              ppppuStack_a8 = (undefined ****)local_260;
              pcStack_a0 = (code *)puStack_258;
              local_b8 = local_270;
              pcStack_b0 = (code *)puStack_268;
              country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write();
              lVar18 = local_e0;
              if (local_80 != (void *)0x0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
                lVar18 = local_e0;
              }
            }
          }
          pppuVar15 = (undefined8 ***)((longlong)pppuVar15 + 1);
        }
      }
      local_61 = 0;
      country_core::systems::ui::resources::ui_state::UiWidgetContext::clicked();
      if (extraout_AL_00 != '\0') {
        *local_170 = local_f8;
        lVar18 = *(longlong *)(local_128 + 0x30);
        uVar2 = *(undefined8 *)(local_128 + 0x40);
        if (lVar18 == *(longlong *)(local_128 + 0x20)) {
          local_61 = 0;
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        lVar17 = *(longlong *)(local_128 + 0x28);
        *(undefined8 *)(lVar17 + lVar18 * 0x10) = uVar2;
        *(undefined1 *)(lVar17 + 8 + lVar18 * 0x10) = 0;
        *(longlong *)(local_128 + 0x30) = lVar18 + 1;
        *(longlong *)(local_128 + 0x40) = *(longlong *)(local_128 + 0x40) + 1;
        uVar2 = *local_130;
        uVar6 = *local_c0;
        *local_168 = local_f4;
        lVar18 = *(longlong *)(local_120 + 0x30);
        uVar7 = *(undefined8 *)(local_120 + 0x40);
        if (lVar18 == *(longlong *)(local_120 + 0x20)) {
          local_61 = 0;
          alloc::raw_vec::RawVec::
          grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                    ();
        }
        lVar17 = *(longlong *)(local_120 + 0x28);
        lVar11 = lVar18 * 0x20;
        *(undefined1 *)(lVar17 + lVar11) = 5;
        *(undefined8 *)(lVar17 + 8 + lVar11) = uVar6;
        *(undefined8 *)(lVar17 + 0x10 + lVar11) = uVar2;
        *(undefined8 *)(lVar17 + 0x18 + lVar11) = uVar7;
        *(longlong *)(local_120 + 0x30) = lVar18 + 1;
        *(longlong *)(local_120 + 0x40) = *(longlong *)(local_120 + 0x40) + 1;
      }
    }
  }
  return;
}

