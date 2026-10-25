// hide_trees 0x14226a260 size=3349
// signature: void __cdecl hide_trees(void)


void __cdecl system_tree::hide_trees::hide_trees(void)

{
  longlong lVar1;
  undefined8 *puVar2;
  undefined1 auVar3 [16];
  undefined1 uVar4;
  longlong lVar5;
  undefined8 uVar6;
  ushort uVar7;
  int iVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  byte *pbVar12;
  byte *pbVar13;
  char extraout_AL;
  char extraout_AL_00;
  char cVar14;
  uint uVar15;
  longlong lVar16;
  longlong lVar17;
  void *_Src;
  void *extraout_RAX;
  void *pvVar18;
  undefined8 *extraout_RAX_00;
  ulonglong uVar19;
  undefined8 *extraout_RAX_01;
  void *extraout_RAX_02;
  void *extraout_RAX_03;
  longlong *in_RCX;
  undefined8 *in_RDX;
  ulonglong uVar20;
  ulonglong uVar21;
  size_t sVar22;
  longlong lVar23;
  uint uVar24;
  longlong lVar25;
  undefined1 (*pauVar26) [16];
  undefined8 *in_R8;
  ulonglong uVar27;
  size_t sVar28;
  undefined8 *in_R9;
  longlong lVar29;
  longlong lVar30;
  void *pvVar31;
  longlong lVar32;
  undefined8 *in_stack_00000028;
  size_t *in_stack_00000030;
  undefined8 *in_stack_00000038;
  longlong local_1f0;
  undefined1 (*local_1e8) [16];
  ushort local_1d8;
  longlong local_1d0;
  undefined4 local_1a8;
  undefined4 uStack_1a4;
  undefined8 local_184;
  undefined8 local_178;
  undefined4 *local_170;
  undefined4 *local_168;
  undefined8 local_160;
  longlong local_158;
  float local_148;
  longlong local_140;
  ulonglong *local_138;
  byte *local_130;
  byte *local_128;
  void *local_120;
  longlong local_118;
  undefined8 *local_110;
  char *local_108;
  byte *local_100;
  ulonglong *local_f8;
  size_t local_f0;
  undefined4 local_e8;
  undefined4 local_e4;
  byte *local_e0;
  undefined8 local_d8;
  longlong local_d0;
  void *local_c8;
  undefined4 local_bc;
  void *local_b8;
  byte *local_b0;
  longlong local_a8;
  size_t local_a0;
  char local_91;
  undefined8 local_90;
  
  local_90 = 0xfffffffffffffffe;
  local_f8 = (ulonglong *)*in_R9;
  lVar25 = in_R9[1];
  uVar20 = *local_f8;
  uVar21 = 0;
  uVar19 = uVar20 - *(ulonglong *)(lVar25 + 0x18);
  if (uVar20 < *(ulonglong *)(lVar25 + 0x18)) {
    uVar19 = uVar21;
  }
  uVar27 = uVar20 - *(ulonglong *)(lVar25 + 0x38);
  if (uVar20 < *(ulonglong *)(lVar25 + 0x38)) {
    uVar27 = uVar21;
  }
  local_e0 = (byte *)(uVar19 * 0x18 + *(longlong *)(lVar25 + 8));
  uVar20 = *(ulonglong *)(lVar25 + 0x10) - uVar19;
  if (*(ulonglong *)(lVar25 + 0x10) < uVar19) {
    local_e0 = (byte *)0x8;
    uVar20 = uVar21;
  }
  local_b0 = (byte *)(uVar27 * 0x18 + *(longlong *)(lVar25 + 0x28));
  uVar19 = *(ulonglong *)(lVar25 + 0x30) - uVar27;
  if (*(ulonglong *)(lVar25 + 0x30) < uVar27) {
    local_b0 = (byte *)0x8;
    uVar19 = uVar21;
  }
  *local_f8 = (*(longlong *)(lVar25 + 0x40) - uVar19) - uVar20;
  local_130 = (byte *)((longlong)local_e0 + uVar20 * 0x18);
  lVar25 = (longlong)local_b0 + uVar19 * 0x18;
  local_178 = *in_RDX;
  local_128 = (byte *)*in_stack_00000028;
  local_e8 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
  local_160 = *in_stack_00000038;
  local_170 = (undefined4 *)in_stack_00000038[2];
  lVar17 = *in_RCX;
  pvVar31 = (void *)in_RCX[1];
  local_bc = *(undefined4 *)((longlong)in_RCX + 0x14);
  local_e4 = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
  local_f0 = *in_stack_00000030;
  local_168 = (undefined4 *)in_stack_00000030[2];
  local_158 = local_f0 + 0x20;
  local_140 = lVar17;
  local_c8 = pvVar31;
  do {
    do {
      if ((local_e0 == (byte *)0x0) || (local_e0 == local_130)) {
        if ((local_b0 == (byte *)0x0) || (local_b0 == (byte *)lVar25)) {
          local_138 = (ulonglong *)*in_R8;
          lVar25 = in_R8[1];
          uVar20 = *local_138;
          uVar21 = 0;
          uVar19 = uVar20 - *(ulonglong *)(lVar25 + 0x18);
          if (uVar20 < *(ulonglong *)(lVar25 + 0x18)) {
            uVar19 = uVar21;
          }
          uVar27 = uVar20 - *(ulonglong *)(lVar25 + 0x38);
          if (uVar20 < *(ulonglong *)(lVar25 + 0x38)) {
            uVar27 = uVar21;
          }
          local_100 = (byte *)(uVar19 * 0x30 + *(longlong *)(lVar25 + 8));
          uVar20 = *(ulonglong *)(lVar25 + 0x10) - uVar19;
          if (*(ulonglong *)(lVar25 + 0x10) < uVar19) {
            local_100 = &DAT_00000008;
            uVar20 = uVar21;
          }
          local_e0 = (byte *)(uVar27 * 0x30 + *(longlong *)(lVar25 + 0x28));
          uVar19 = *(ulonglong *)(lVar25 + 0x30) - uVar27;
          if (*(ulonglong *)(lVar25 + 0x30) < uVar27) {
            local_e0 = &DAT_00000008;
            uVar19 = uVar21;
          }
          *local_138 = (*(longlong *)(lVar25 + 0x40) - uVar19) - uVar20;
          local_130 = local_100 + uVar20 * 0x30;
          local_128 = local_e0 + uVar19 * 0x30;
          do {
            do {
              if ((local_100 == (byte *)0x0) || (local_100 == local_130)) {
                if ((local_e0 == (byte *)0x0) || (local_e0 == local_128)) {
                  return;
                }
                local_100 = (byte *)0x0;
                pbVar12 = local_100;
                pbVar13 = local_e0 + 0x30;
                local_b0 = local_e0;
              }
              else {
                pbVar12 = local_100 + 0x30;
                pbVar13 = local_e0;
                local_b0 = local_100;
              }
              local_e0 = pbVar13;
              local_100 = pbVar12;
              *local_138 = *local_138 + 1;
              country_core::resources::walls::decorator_storage::Decorators::iter_removed_ids();
            } while (local_1d0 == 0);
            local_f8 = (ulonglong *)(local_b0 + 4);
            uVar15 = (uint)local_1d8;
            lVar25 = local_1f0;
            lVar16 = local_1d0;
            pauVar26 = local_1e8;
            do {
              if ((short)uVar15 == 0) {
                do {
                  auVar3 = *pauVar26;
                  uVar7 = (ushort)(SUB161(auVar3 >> 7,0) & 1) |
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
                          (ushort)(byte)(auVar3[0xf] >> 7) << 0xf;
                  lVar25 = lVar25 + -0x100;
                  pauVar26 = pauVar26 + 1;
                } while (uVar7 == 0xffff);
                uVar24 = ~(uint)uVar7;
                uVar15 = uVar24 & -(uint)uVar7 - 2;
              }
              else {
                if (lVar25 == 0) break;
                uVar24 = uVar15;
                uVar15 = uVar15 - 1 & uVar15;
              }
              iVar8 = 0;
              for (; (uVar24 & 1) == 0; uVar24 = uVar24 >> 1 | 0x80000000) {
                iVar8 = iVar8 + 1;
              }
              lVar23 = lVar25 - (ulonglong)(uint)(iVar8 << 4);
              uVar20 = *(ulonglong *)(lVar23 + -8);
              uVar19 = uVar20 & 0xffffffff;
              if ((uVar19 < *(ulonglong *)((longlong)pvVar31 + 0x10)) &&
                 (*(int *)(*(longlong *)((longlong)pvVar31 + 8) + uVar19 * 0x14) ==
                  (int)(uVar20 >> 0x20))) {
                lVar32 = *(longlong *)((longlong)pvVar31 + 8) + uVar19 * 0x14;
                uVar20 = (ulonglong)*(uint *)(lVar32 + 4);
                if ((uVar20 != 0xffffffff) &&
                   ((uVar20 < *(ulonglong *)(lVar17 + 0x38) &&
                    ((*(ulonglong *)
                       (*(longlong *)(lVar17 + 0x28) + (ulonglong)(*(uint *)(lVar32 + 4) >> 6) * 8)
                      >> (uVar20 & 0x3f) & 1) != 0)))) {
                  uVar19 = (ulonglong)*(uint *)(lVar32 + 0xc);
                  lVar1 = *(longlong *)((longlong)pvVar31 + 0x1a8);
                  uVar20 = *(ulonglong *)(lVar1 + 0x40 + uVar19 * 0x48);
                  lVar5 = *(longlong *)(lVar1 + 0x18 + uVar19 * 0x48);
                  lVar1 = *(longlong *)(lVar1 + 0x38 + uVar19 * 0x48);
                  if ((*(ulonglong *)(lVar17 + 0x118) < uVar20) &&
                     (uVar19 = *(ulonglong *)(lVar1 + *(ulonglong *)(lVar17 + 0x118) * 8),
                     uVar19 != 0)) {
                    lVar30 = *(longlong *)(lVar5 + 0x10 + ~uVar19 * 0x30);
                    uVar19 = *(ulonglong *)(lVar17 + 0x120);
                    if (uVar20 <= uVar19) goto LAB_14226ac28;
LAB_14226abfa:
                    uVar19 = *(ulonglong *)(lVar1 + uVar19 * 8);
                    if (uVar19 == 0) goto LAB_14226ac28;
                    local_b8 = *(void **)(lVar5 + 0x10 + ~uVar19 * 0x30);
                    uVar19 = *(ulonglong *)(lVar17 + 0x128);
                    if (uVar19 < uVar20) goto LAB_14226ac3f;
LAB_14226ac61:
                    local_d0 = 0;
                  }
                  else {
                    lVar30 = 0;
                    uVar19 = *(ulonglong *)(lVar17 + 0x120);
                    if (uVar19 < uVar20) goto LAB_14226abfa;
LAB_14226ac28:
                    local_b8 = (void *)0x0;
                    uVar19 = *(ulonglong *)(lVar17 + 0x128);
                    if (uVar20 <= uVar19) goto LAB_14226ac61;
LAB_14226ac3f:
                    uVar20 = *(ulonglong *)(lVar1 + uVar19 * 8);
                    if (uVar20 == 0) goto LAB_14226ac61;
                    local_d0 = *(longlong *)(lVar5 + 0x10 + ~uVar20 * 0x30);
                  }
                  local_d8 = *(undefined8 *)(lVar23 + -0x10);
                  uVar20 = (ulonglong)*(uint *)(lVar32 + 0x10);
                  lVar17 = ~*(ulonglong *)(lVar1 + *(longlong *)(lVar17 + 0x110) * 8) * 0x30;
                  local_a8 = *(longlong *)(lVar5 + 0x10 + lVar17);
                  local_a0 = *(size_t *)(lVar5 + 0x28 + lVar17);
                  *(undefined4 *)(local_a0 + uVar20 * 4) = local_bc;
                  lVar23 = uVar20 * 0x10;
                  *(undefined1 *)(local_a8 + 0xc + lVar23) = 0;
                  puVar2 = (undefined8 *)(lVar30 + uVar20 * 0x30);
                  uVar9 = *puVar2;
                  uVar10 = puVar2[1];
                  uVar11 = *(undefined8 *)(lVar30 + 0x10 + uVar20 * 0x30);
                  country_core::resources::placement_data::PlacementDataSsbo::position();
                  if ((*(float *)(local_b0 + 0x1c) <= 0.0) ||
                     (lVar17 = local_140, *(float *)(local_b0 + 0x1c) - local_148 <= 4.0)) {
                    if ((*local_b0 & 1) == 0) {
                      ::utils::geometry::circle::Circle2d::with_expanded_diameter();
                      ::utils::geometry::circle::Circle2d::contains_point();
                      lVar32 = local_a8;
                      sVar22 = local_a0;
                      cVar14 = extraout_AL_00;
                    }
                    else {
                      ::utils::geometry::rectangle::Rectangle2d::with_expanded_size();
                      ::utils::geometry::rectangle::Rectangle2d::contains_point();
                      lVar32 = local_a8;
                      sVar22 = local_a0;
                      cVar14 = extraout_AL;
                    }
                    lVar17 = local_140;
                    local_a8 = lVar32;
                    local_a0 = sVar22;
                    if (cVar14 != '\0') {
                      *(undefined4 *)(sVar22 + uVar20 * 4) = local_bc;
                      *(undefined1 *)(lVar32 + lVar23 + 0xc) = 1;
                      if (local_b0[0x24] == 1) {
                        *local_168 = local_e4;
                        local_a8 = *(longlong *)(local_f0 + 0x30);
                        local_a0 = *(size_t *)(local_f0 + 0x40);
                        sVar28 = local_a0;
                        lVar17 = local_a8;
                        if (local_a8 == *(longlong *)(local_f0 + 0x20)) {
                          alloc::raw_vec::RawVec::
                          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                                    ();
                          sVar28 = local_a0;
                          lVar17 = local_a8;
                          lVar32 = local_a8;
                          sVar22 = local_a0;
                        }
                        local_a0 = sVar22;
                        local_a8 = lVar32;
                        lVar32 = *(longlong *)(local_f0 + 0x28);
                        *(size_t *)(lVar32 + lVar17 * 0x10) = sVar28;
                        *(undefined8 *)(lVar32 + 8 + lVar17 * 0x10) = local_d8;
                        *(longlong *)(local_f0 + 0x30) = lVar17 + 1;
                        *(longlong *)(local_f0 + 0x40) = *(longlong *)(local_f0 + 0x40) + 1;
                        *local_170 = local_e8;
                        local_1a8 = (undefined4)uVar11;
                        uStack_1a4 = (undefined4)((ulonglong)uVar11 >> 0x20);
                        local_a8 = CONCAT44(local_a8._4_4_,uStack_1a4);
                        sVar22 = local_f0;
                        asset::prefab::PrefabName::as_name_ref();
                        if ((longlong)sVar22 < 0) {
                    // WARNING: Subroutine does not return
                          alloc::raw_vec::handle_error();
                        }
                        if (sVar22 == 0) {
                          local_c8 = (void *)0x1;
                          pvVar18 = extraout_RAX_02;
                        }
                        else {
                          local_b8 = extraout_RAX_02;
                          local_a0 = sVar22;
                          std::alloc::__default_lib_allocator::__rust_alloc();
                          local_c8 = extraout_RAX_03;
                          pvVar18 = local_b8;
                          sVar22 = local_a0;
                          if (extraout_RAX_03 == (void *)0x0) {
                            local_c8 = (void *)local_a0;
                    // WARNING: Subroutine does not return
                            alloc::raw_vec::handle_error();
                          }
                        }
                        memcpy(local_c8,pvVar18,sVar22);
                        uVar4 = *(undefined1 *)(local_d0 + 8 + lVar23);
                        std::alloc::__default_lib_allocator::__rust_alloc();
                        if (extraout_RAX_01 == (undefined8 *)0x0) {
                          local_a0 = sVar22;
                    // WARNING: Subroutine does not return
                          alloc::alloc::handle_alloc_error();
                        }
                        *extraout_RAX_01 = 1;
                        extraout_RAX_01[1] = 1;
                        *(undefined4 *)(extraout_RAX_01 + 2) = 2;
                        extraout_RAX_01[3] = local_d8;
                        extraout_RAX_01[4] = sVar22;
                        extraout_RAX_01[5] = local_c8;
                        extraout_RAX_01[6] = sVar22;
                        extraout_RAX_01[7] = uVar9;
                        extraout_RAX_01[8] = uVar10;
                        *(undefined4 *)(extraout_RAX_01 + 9) = local_1a8;
                        *(undefined4 *)((longlong)extraout_RAX_01 + 0x4c) = (undefined4)local_a8;
                        *(undefined1 *)(extraout_RAX_01 + 10) = uVar4;
                        local_118 = 0xc;
                        local_110 = extraout_RAX_01;
                        country_core::resources::history::History::add();
                        lVar17 = local_140;
                      }
                    }
                  }
                }
              }
              lVar16 = lVar16 + -1;
            } while (lVar16 != 0);
          } while( true );
        }
        local_e0 = (byte *)0x0;
        lVar16 = (longlong)local_b0;
        local_b0 = (byte *)((longlong)local_b0 + 0x18);
      }
      else {
        lVar16 = (longlong)local_e0;
        local_e0 = (byte *)((longlong)local_e0 + 0x18);
      }
      *local_f8 = *local_f8 + 1;
      country_core::resources::walls::decorator_storage::Decorators::iter_removed_ids();
    } while (local_1d0 == 0);
    local_100 = (byte *)(lVar16 + 8);
    uVar15 = (uint)local_1d8;
    lVar23 = local_1f0;
    pauVar26 = local_1e8;
    lVar32 = local_1d0;
    local_138 = (ulonglong *)lVar16;
    do {
      if ((short)uVar15 == 0) {
        do {
          auVar3 = *pauVar26;
          uVar7 = (ushort)(SUB161(auVar3 >> 7,0) & 1) | (ushort)(SUB161(auVar3 >> 0xf,0) & 1) << 1 |
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
          lVar23 = lVar23 + -0x100;
          pauVar26 = pauVar26 + 1;
        } while (uVar7 == 0xffff);
        uVar24 = ~(uint)uVar7;
        uVar15 = uVar24 & -(uint)uVar7 - 2;
      }
      else {
        if (lVar23 == 0) break;
        uVar24 = uVar15;
        uVar15 = uVar15 - 1 & uVar15;
      }
      iVar8 = 0;
      for (; (uVar24 & 1) == 0; uVar24 = uVar24 >> 1 | 0x80000000) {
        iVar8 = iVar8 + 1;
      }
      lVar16 = lVar23 - (ulonglong)(uint)(iVar8 << 4);
      uVar20 = *(ulonglong *)(lVar16 + -8);
      uVar19 = uVar20 & 0xffffffff;
      if ((uVar19 < *(ulonglong *)((longlong)pvVar31 + 0x10)) &&
         (*(int *)(*(longlong *)((longlong)pvVar31 + 8) + uVar19 * 0x14) == (int)(uVar20 >> 0x20)))
      {
        lVar1 = *(longlong *)((longlong)pvVar31 + 8) + uVar19 * 0x14;
        uVar20 = (ulonglong)*(uint *)(lVar1 + 4);
        if ((uVar20 != 0xffffffff) &&
           ((uVar20 < *(ulonglong *)(lVar17 + 0x38) &&
            ((*(ulonglong *)
               (*(longlong *)(lVar17 + 0x28) + (ulonglong)(*(uint *)(lVar1 + 4) >> 6) * 8) >>
              (uVar20 & 0x3f) & 1) != 0)))) {
          uVar19 = (ulonglong)*(uint *)(lVar1 + 0xc);
          lVar5 = *(longlong *)((longlong)pvVar31 + 0x1a8);
          uVar20 = *(ulonglong *)(lVar5 + 0x40 + uVar19 * 0x48);
          lVar30 = *(longlong *)(lVar5 + 0x18 + uVar19 * 0x48);
          lVar5 = *(longlong *)(lVar5 + 0x38 + uVar19 * 0x48);
          if ((*(ulonglong *)(lVar17 + 0x118) < uVar20) &&
             (uVar19 = *(ulonglong *)(lVar5 + *(ulonglong *)(lVar17 + 0x118) * 8), uVar19 != 0)) {
            lVar29 = *(longlong *)(lVar30 + 0x10 + ~uVar19 * 0x30);
            uVar19 = *(ulonglong *)(lVar17 + 0x120);
            if (uVar20 <= uVar19) goto LAB_14226a5a7;
LAB_14226a579:
            uVar19 = *(ulonglong *)(lVar5 + uVar19 * 8);
            if (uVar19 == 0) goto LAB_14226a5a7;
            local_b8 = *(void **)(lVar30 + 0x10 + ~uVar19 * 0x30);
            uVar19 = *(ulonglong *)(lVar17 + 0x128);
            if (uVar19 < uVar20) goto LAB_14226a5be;
LAB_14226a5e0:
            local_d0 = 0;
          }
          else {
            lVar29 = 0;
            uVar19 = *(ulonglong *)(lVar17 + 0x120);
            if (uVar19 < uVar20) goto LAB_14226a579;
LAB_14226a5a7:
            local_b8 = (void *)0x0;
            uVar19 = *(ulonglong *)(lVar17 + 0x128);
            if (uVar20 <= uVar19) goto LAB_14226a5e0;
LAB_14226a5be:
            uVar20 = *(ulonglong *)(lVar5 + uVar19 * 8);
            if (uVar20 == 0) goto LAB_14226a5e0;
            local_d0 = *(longlong *)(lVar30 + 0x10 + ~uVar20 * 0x30);
          }
          local_d8 = *(undefined8 *)(lVar16 + -0x10);
          uVar20 = (ulonglong)*(uint *)(lVar1 + 0x10);
          lVar17 = ~*(ulonglong *)(lVar5 + *(longlong *)(lVar17 + 0x110) * 8) * 0x30;
          local_a8 = *(longlong *)(lVar30 + 0x10 + lVar17);
          local_a0 = *(size_t *)(lVar30 + 0x28 + lVar17);
          *(undefined4 *)(local_a0 + uVar20 * 4) = local_bc;
          lVar16 = uVar20 * 0x10;
          *(undefined1 *)(local_a8 + 0xc + lVar16) = 0;
          puVar2 = (undefined8 *)(lVar29 + uVar20 * 0x30);
          uVar9 = *puVar2;
          uVar10 = puVar2[1];
          uVar11 = *(undefined8 *)(lVar29 + 0x10 + uVar20 * 0x30);
          country_core::resources::placement_data::PlacementDataSsbo::position();
          local_91 = '\x01';
          ::utils::geometry::circle::Circle2d::new();
          local_118 = (longlong)local_100;
          local_110 = &local_184;
          local_108 = &local_91;
          country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::
          query_with_circle_callback<system_tree::hide_trees::hide_trees::closure_env_0>();
          pvVar31 = local_c8;
          lVar17 = local_140;
          if (local_91 == '\0') {
            *(undefined4 *)(local_a0 + uVar20 * 4) = local_bc;
            *(undefined1 *)(local_a8 + lVar16 + 0xc) = 1;
            if (*(char *)((longlong)local_138 + 0x10) != '\0') {
              *local_168 = local_e4;
              lVar17 = *(longlong *)(local_f0 + 0x30);
              uVar6 = *(undefined8 *)(local_f0 + 0x40);
              local_a8 = lVar16;
              if (lVar17 == *(longlong *)(local_f0 + 0x20)) {
                alloc::raw_vec::RawVec::
                grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                          ();
              }
              lVar16 = *(longlong *)(local_f0 + 0x28);
              *(undefined8 *)(lVar16 + lVar17 * 0x10) = uVar6;
              *(undefined8 *)(lVar16 + 8 + lVar17 * 0x10) = local_d8;
              *(longlong *)(local_f0 + 0x30) = lVar17 + 1;
              *(longlong *)(local_f0 + 0x40) = *(longlong *)(local_f0 + 0x40) + 1;
              *local_170 = local_e8;
              local_1a8 = (undefined4)uVar11;
              uStack_1a4 = (undefined4)((ulonglong)uVar11 >> 0x20);
              sVar22 = local_f0;
              asset::prefab::PrefabName::as_name_ref();
              pvVar31 = local_c8;
              if ((longlong)sVar22 < 0) {
                    // WARNING: Subroutine does not return
                alloc::raw_vec::handle_error();
              }
              local_a0 = CONCAT44(local_a0._4_4_,uStack_1a4);
              if (sVar22 == 0) {
                pvVar18 = (void *)0x1;
              }
              else {
                std::alloc::__default_lib_allocator::__rust_alloc();
                pvVar18 = extraout_RAX;
                if (extraout_RAX == (void *)0x0) {
                  local_120 = (void *)sVar22;
                    // WARNING: Subroutine does not return
                  alloc::raw_vec::handle_error();
                }
              }
              memcpy(pvVar18,_Src,sVar22);
              local_a8 = CONCAT71(local_a8._1_7_,*(undefined1 *)(local_d0 + 8 + local_a8));
              std::alloc::__default_lib_allocator::__rust_alloc();
              local_120 = pvVar18;
              if (extraout_RAX_00 == (undefined8 *)0x0) {
                local_b0 = (byte *)sVar22;
                    // WARNING: Subroutine does not return
                alloc::alloc::handle_alloc_error();
              }
              *extraout_RAX_00 = 1;
              extraout_RAX_00[1] = 1;
              *(undefined4 *)(extraout_RAX_00 + 2) = 2;
              extraout_RAX_00[3] = local_d8;
              extraout_RAX_00[4] = sVar22;
              extraout_RAX_00[5] = pvVar18;
              extraout_RAX_00[6] = sVar22;
              extraout_RAX_00[7] = uVar9;
              extraout_RAX_00[8] = uVar10;
              *(undefined4 *)(extraout_RAX_00 + 9) = local_1a8;
              *(undefined4 *)((longlong)extraout_RAX_00 + 0x4c) = (undefined4)local_a0;
              *(undefined1 *)(extraout_RAX_00 + 10) = (undefined1)local_a8;
              local_118 = 0xc;
              local_110 = extraout_RAX_00;
              country_core::resources::history::History::add();
              lVar17 = local_140;
            }
          }
        }
      }
      lVar32 = lVar32 + -1;
    } while (lVar32 != 0);
  } while( true );
}

