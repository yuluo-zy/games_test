
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::systems::ui_move_freehand_walls::ui_move_freehand_walls(void)

{
  char *pcVar1;
  undefined8 *puVar2;
  size_t sVar3;
  float *pfVar4;
  float *pfVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  longlong *plVar10;
  void *pvVar11;
  ulonglong uVar12;
  undefined1 auVar13 [16];
  float fVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  bool bVar19;
  undefined2 uVar20;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  uint extraout_EAX;
  uint uVar21;
  uint uVar22;
  ulonglong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  ulonglong extraout_RAX_02;
  byte *extraout_RAX_03;
  undefined1 *extraout_RAX_04;
  undefined8 *_Dst;
  undefined4 *extraout_RAX_05;
  byte *pbVar23;
  longlong *extraout_RAX_06;
  undefined1 *extraout_RAX_07;
  undefined4 *puVar24;
  undefined8 *puVar25;
  longlong *plVar26;
  undefined1 *extraout_RAX_08;
  undefined8 *extraout_RAX_09;
  undefined1 *extraout_RAX_10;
  undefined1 *extraout_RAX_11;
  undefined1 *extraout_RAX_12;
  ulonglong uVar27;
  longlong *in_RCX;
  undefined8 *in_RDX;
  ulonglong uVar28;
  undefined4 *puVar29;
  undefined1 *puVar30;
  undefined1 (*pauVar31) [16];
  longlong lVar32;
  longlong *in_R8;
  ulonglong uVar33;
  longlong lVar34;
  ulonglong uVar35;
  ulonglong uVar36;
  longlong *in_R9;
  longlong lVar37;
  ulonglong uVar38;
  ushort uVar39;
  longlong lVar40;
  undefined4 *puVar41;
  int iVar42;
  undefined1 (*pauVar43) [16];
  byte bVar44;
  char cVar45;
  char cVar49;
  float extraout_XMM0_Da;
  char cVar47;
  char cVar48;
  undefined1 auVar46 [16];
  uint uVar50;
  uint uVar51;
  float fVar52;
  float fVar55;
  undefined1 auVar53 [16];
  undefined1 auVar54 [16];
  float fVar58;
  undefined1 auVar56 [16];
  undefined1 auVar57 [16];
  float fVar59;
  float fVar60;
  float fVar61;
  undefined8 *in_stack_00000028;
  longlong *in_stack_00000030;
  longlong in_stack_00000040;
  longlong *in_stack_00000048;
  undefined8 *in_stack_00000078;
  longlong *in_stack_00000080;
  undefined1 local_2f8 [16];
  undefined8 uStack_2e8;
  undefined8 uStack_2e0;
  undefined8 uStack_2d8;
  undefined8 local_2d0;
  longlong local_2c8;
  int *local_2c0;
  undefined4 *local_2b8;
  undefined4 *local_2b0;
  undefined4 *local_2a8;
  longlong *plStack_2a0;
  undefined4 *local_298;
  undefined8 *local_290;
  undefined4 *puStack_288;
  undefined4 *puStack_280;
  undefined4 *puStack_278;
  undefined4 *puStack_270;
  undefined4 *puStack_268;
  int iStack_260;
  undefined4 uStack_25c;
  undefined4 uStack_258;
  undefined4 uStack_254;
  undefined4 uStack_250;
  undefined4 uStack_24c;
  undefined4 uStack_248;
  undefined4 uStack_244;
  ulonglong uStack_240;
  undefined1 *puStack_238;
  ulonglong uStack_230;
  undefined4 uStack_228;
  undefined4 *puStack_220;
  undefined8 uStack_1a0;
  undefined4 *puStack_198;
  longlong lStack_190;
  float local_188;
  float fStack_184;
  ulonglong *local_180;
  undefined4 *local_178;
  undefined4 *local_170;
  undefined4 local_168;
  undefined4 local_164;
  undefined4 local_160;
  undefined4 local_15c;
  longlong local_158;
  longlong local_150;
  undefined4 *local_148;
  undefined4 *local_140;
  undefined4 *local_138;
  size_t local_130;
  undefined4 *puStack_128;
  undefined8 *puStack_120;
  undefined4 *puStack_118;
  undefined4 *puStack_110;
  undefined4 *puStack_108;
  undefined4 *puStack_100;
  longlong lStack_f8;
  undefined4 local_e4;
  undefined1 *local_e0;
  undefined4 *local_d8;
  byte *local_d0;
  undefined1 *local_c8;
  undefined8 *local_c0;
  longlong *local_b8;
  longlong *local_b0;
  undefined1 uStack_a3;
  undefined1 uStack_a2;
  char cStack_a1;
  undefined8 local_a0;
  
  local_a0 = 0xfffffffffffffffe;
  if (*(int *)*in_RDX != 1) {
override_jmp_140978638_case_0:
    return;
  }
  local_d0 = (byte *)*in_stack_00000028;
  local_180 = (ulonglong *)in_stack_00000028[4];
  local_2c8 = in_stack_00000028[5];
  lVar32 = *in_stack_00000030;
  local_148 = (undefined4 *)in_stack_00000030[2];
  local_140 = (undefined4 *)
              CONCAT44(local_140._4_4_,*(undefined4 *)((longlong)in_stack_00000030 + 0x1c));
  local_138 = (undefined4 *)in_stack_00000030[6];
  local_130 = CONCAT44(local_130._4_4_,*(undefined4 *)((longlong)in_stack_00000030 + 0x3c));
  local_2c0 = (int *)*in_stack_00000078;
  puVar29 = *(undefined4 **)((int *)*in_RDX + 2);
  plVar10 = (longlong *)*in_R8;
  local_150 = in_stack_00000078[4];
  local_2a8 = (undefined4 *)in_stack_00000078[6];
  local_15c = *(undefined4 *)((longlong)in_stack_00000078 + 0x3c);
  local_c8 = (undefined1 *)in_stack_00000078[8];
  local_2b0 = (undefined4 *)in_stack_00000078[10];
  local_160 = *(undefined4 *)((longlong)in_stack_00000078 + 0x5c);
  local_e0 = (undefined1 *)in_stack_00000080[4];
  local_178 = (undefined4 *)in_stack_00000080[6];
  local_e4 = *(undefined4 *)((longlong)in_stack_00000080 + 0x3c);
  local_158 = in_stack_00000080[8];
  local_2b8 = (undefined4 *)in_stack_00000080[10];
  local_164 = *(undefined4 *)((longlong)in_stack_00000080 + 0x5c);
  local_298 = puVar29;
  if (plVar10[3] != 0) {
    local_b8 = in_R9;
    local_b0 = in_RCX;
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar34 = *plVar10;
    bVar44 = (byte)(extraout_RAX >> 0x39);
    auVar46 = ZEXT216(CONCAT11(bVar44,bVar44));
    auVar46 = pshuflw(auVar46,auVar46,0);
    lVar37 = 0;
    uVar33 = extraout_RAX;
    do {
      uVar33 = plVar10[1] & uVar33;
      pcVar1 = (char *)(lVar34 + uVar33);
      cVar45 = auVar46[0];
      auVar56[0] = -(*pcVar1 == cVar45);
      cVar47 = auVar46[1];
      auVar56[1] = -(pcVar1[1] == cVar47);
      cVar48 = auVar46[2];
      auVar56[2] = -(pcVar1[2] == cVar48);
      cVar49 = auVar46[3];
      auVar56[3] = -(pcVar1[3] == cVar49);
      auVar56[4] = -(pcVar1[4] == cVar45);
      auVar56[5] = -(pcVar1[5] == cVar47);
      auVar56[6] = -(pcVar1[6] == cVar48);
      auVar56[7] = -(pcVar1[7] == cVar49);
      auVar56[8] = -(pcVar1[8] == cVar45);
      auVar56[9] = -(pcVar1[9] == cVar47);
      auVar56[10] = -(pcVar1[10] == cVar48);
      auVar56[0xb] = -(pcVar1[0xb] == cVar49);
      auVar56[0xc] = -(pcVar1[0xc] == cVar45);
      auVar56[0xd] = -(pcVar1[0xd] == cVar47);
      auVar56[0xe] = -(pcVar1[0xe] == cVar48);
      auVar56[0xf] = -(pcVar1[0xf] == cVar49);
      uVar39 = (ushort)(SUB161(auVar56 >> 7,0) & 1) | (ushort)(SUB161(auVar56 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar56 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar56 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar56 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar56 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar56 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar56 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar56 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar56 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar56 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar56 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar56 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar56 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar56 >> 0x77,0) & 1) << 0xe | (ushort)(auVar56[0xf] >> 7) << 0xf;
      uVar21 = (uint)uVar39;
      while (uVar39 != 0) {
        uVar22 = 0;
        for (uVar51 = uVar21; (uVar51 & 1) == 0; uVar51 = uVar51 >> 1 | 0x80000000) {
          uVar22 = uVar22 + 1;
        }
        lVar40 = (uVar22 + uVar33 & plVar10[1]) * -0x2a8;
        if (*(undefined4 **)(lVar34 + -0x2a8 + lVar40) == puVar29) {
          if ((*(ulonglong *)(lVar34 + -0x290 + lVar40) < 2) ||
             (*(float *)(lVar34 + -0x270 + lVar40) <= 0.0)) {
                    /* WARNING: Subroutine does not return */
            core::panicking::panic();
          }
          puVar25 = (undefined8 *)(lVar34 + lVar40);
          if (*(uint *)(puVar25 + -9) < 2) {
            return;
          }
          local_d8 = puVar29;
          ::utils::curve::Curve::
          get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>
                    ();
          fVar61 = __real_3f800000;
          uVar22 = _UNK_1429258f4;
          uVar21 = ___xmm_80000000800000008000000080000000;
          uVar33 = (ulonglong)extraout_EAX;
          if ((ulonglong)puVar25[-0x52] <= uVar33) {
                    /* WARNING: Subroutine does not return */
            core::panicking::panic_bounds_check();
          }
          if ((ulonglong)puVar25[-0x52] <= uVar33 + 1) {
                    /* WARNING: Subroutine does not return */
            core::panicking::panic_bounds_check();
          }
          uVar8 = *(undefined8 *)(puVar25[-0x53] + uVar33 * 8);
          uVar9 = *(undefined8 *)(puVar25[-0x53] + 8 + uVar33 * 8);
          fVar52 = (float)uVar8;
          fVar55 = (float)((ulonglong)uVar8 >> 0x20);
          fStack_184 = (float)uVar9;
          fVar58 = (float)((ulonglong)uVar9 >> 0x20);
          fVar59 = extraout_XMM0_Da * fStack_184 + (__real_3f800000 - extraout_XMM0_Da) * fVar52;
          fVar60 = extraout_XMM0_Da * fVar58 + (__real_3f800000 - extraout_XMM0_Da) * fVar55;
          fStack_184 = fStack_184 - fVar52;
          fVar58 = fVar58 - fVar55;
          fVar52 = __real_3f800000 / SQRT(fVar58 * fVar58 + fStack_184 * fStack_184);
          local_188 = (float)((uint)fVar58 ^ ___xmm_80000000800000008000000080000000) * fVar52;
          fStack_184 = fStack_184 * fVar52;
          ::utils::camera::CameraMatrices::eye_direction();
          fVar61 = fVar61 / SQRT(local_290._0_4_ * local_290._0_4_ +
                                 local_298._0_4_ * local_298._0_4_);
          uVar51 = (uint)(fVar61 * local_290._0_4_ * fStack_184 +
                          fVar61 * local_298._0_4_ * local_188 < 0.0);
          uVar50 = (int)(uVar51 << 0x1f) >> 0x1f;
          uVar51 = (int)(uVar51 << 0x1f) >> 0x1f;
          local_2d0 = CONCAT44((float)(~uVar51 & (uVar22 ^ (uint)fStack_184) |
                                      (uint)fStack_184 & uVar51) + fVar60,
                               (float)(~uVar50 & (uVar21 ^ (uint)local_188) |
                                      (uint)local_188 & uVar50) + fVar59);
          resources::terrain_height::TerrainHeightsData::
          sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
          local_c0 = puVar25;
          ::utils::swizzlers::impl$1::xvy();
          ::utils::swizzlers::impl$0::x0y();
          puVar30 = local_2f8;
          ::utils::xz_projective_rotation_bad();
          puVar24 = local_d8;
          local_290 = (undefined8 *)CONCAT44(local_290._4_4_,local_168);
          local_298 = local_170;
          ui_move_shapes::move_gizmo_and_trigger_vol();
          puVar29 = local_138;
          cVar45 = '\x02';
          if ((1 < *(byte *)(extraout_RAX_00 + 0x18)) &&
             (*(undefined1 **)(extraout_RAX_00 + 0x38) == puVar30)) {
            cVar45 = *(char *)(extraout_RAX_00 + 0x19);
          }
          bVar44 = 3;
          if ((byte)(cVar45 - 2U) < 3) {
            bVar44 = cVar45 - 2U;
          }
                    /* WARNING: Switch is manually overridden */
          switch(bVar44) {
          case 0:
            goto override_jmp_140978638_case_0;
          case 1:
            uVar6 = (undefined4)local_130;
            *local_138 = (undefined4)local_130;
            local_298 = (undefined4 *)0x0;
            local_290 = (undefined8 *)puVar24;
            puStack_288 = (undefined4 *)0x1;
            resources::walls::decorator_archivist::DecoratorArchivist::
            archive_current_decorators_in_storages<ref_<str_>_>();
            *puVar29 = uVar6;
            resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_wall();
            *puVar29 = uVar6;
            local_290 = (undefined8 *)(lVar32 + 0x90);
            if (*(longlong *)(lVar32 + 0x90) == 0) {
              local_290 = (undefined8 *)0x0;
            }
            local_298 = (undefined4 *)0x1;
            puStack_288 = (undefined4 *)0x0;
            iStack_260 = 0;
            uStack_25c = 0;
            _ZN102__LT_core__iter__adapters__map__Map_LT_I_C_F_GT__u20_as_u20_core__iter__traits__iterator__Iterator_GT_4fold17h1014fed281d97499E
                      ();
            puStack_128 = (undefined4 *)0x0;
            puStack_120 = (undefined8 *)&DAT_00000004;
            puStack_118 = (undefined4 *)0x0;
            *local_148 = local_140._0_4_;
            uStack_a2 = 1;
            resources::walls::decorator_storage::DecoratorStorage::wall_decorators_mut();
            puVar25 = local_c0;
            if (extraout_RAX_01 != 0) {
              pauVar31 = *(undefined1 (**) [16])(extraout_RAX_01 + 0x30);
              lVar32 = *(longlong *)(extraout_RAX_01 + 0x48);
              auVar46 = *pauVar31;
              uVar21 = ~(uint)(ushort)((ushort)(SUB161(auVar46 >> 7,0) & 1) |
                                       (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
                                       (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
                                       (ushort)(SUB161(auVar46 >> 0x1f,0) & 1) << 3 |
                                       (ushort)(SUB161(auVar46 >> 0x27,0) & 1) << 4 |
                                       (ushort)(SUB161(auVar46 >> 0x2f,0) & 1) << 5 |
                                       (ushort)(SUB161(auVar46 >> 0x37,0) & 1) << 6 |
                                       (ushort)(SUB161(auVar46 >> 0x3f,0) & 1) << 7 |
                                       (ushort)(SUB161(auVar46 >> 0x47,0) & 1) << 8 |
                                       (ushort)(SUB161(auVar46 >> 0x4f,0) & 1) << 9 |
                                       (ushort)(SUB161(auVar46 >> 0x57,0) & 1) << 10 |
                                       (ushort)(SUB161(auVar46 >> 0x5f,0) & 1) << 0xb |
                                       (ushort)(SUB161(auVar46 >> 0x67,0) & 1) << 0xc |
                                       (ushort)(SUB161(auVar46 >> 0x6f,0) & 1) << 0xd |
                                       (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                                      (ushort)(byte)(auVar46[0xf] >> 7) << 0xf);
              pauVar43 = pauVar31 + 1;
              while (puVar29 = puStack_118, lVar32 != 0) {
                if ((short)uVar21 == 0) {
                  do {
                    auVar46 = *pauVar43;
                    uVar39 = (ushort)(SUB161(auVar46 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar46 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar46 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar46 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar46 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar46 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar46 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar46 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar46 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar46 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar46 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar46 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                             (ushort)(byte)(auVar46[0xf] >> 7) << 0xf;
                    pauVar31 = pauVar31 + -0x50;
                    pauVar43 = pauVar43 + 1;
                  } while (uVar39 == 0xffff);
                  uVar22 = ~(uint)uVar39;
                  uVar21 = uVar22 & -(uint)uVar39 - 2;
                }
                else {
                  uVar22 = uVar21;
                  uVar21 = uVar21 - 1 & uVar21;
                }
                lVar32 = lVar32 + -1;
                uVar51 = 0;
                for (; (uVar22 & 1) == 0; uVar22 = uVar22 >> 1 | 0x80000000) {
                  uVar51 = uVar51 + 1;
                }
                uVar33 = (ulonglong)uVar51;
                lVar34 = (ulonglong)(byte)pauVar31[uVar33 * -5 + -3][10] - 1;
                if (2 < (byte)(pauVar31[uVar33 * -5 + -3][10] - 2)) {
                  lVar34 = 0;
                }
                if (lVar34 == 0) {
                  uVar6 = *(undefined4 *)pauVar31[uVar33 * -5 + -5];
                  fVar61 = *(float *)(pauVar31[uVar33 * -5 + -3] + 4);
                  fVar52 = *(float *)(pauVar31[uVar33 * -5 + -4] + 0xc);
                  if (puStack_118 == puStack_128) {
                    uStack_a2 = 1;
                    alloc::raw_vec::RawVec::
                    grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                              ();
                  }
                  *(undefined4 *)((longlong)puStack_120 + (longlong)puVar29 * 0xc) = uVar6;
                  pfVar4 = (float *)((longlong)puStack_120 + (longlong)puVar29 * 0xc + 4);
                  *pfVar4 = fVar52 - fVar59;
                  pfVar4[1] = fVar61 - fVar60;
                  puStack_118 = (undefined4 *)((longlong)puVar29 + 1);
                  puVar25 = local_c0;
                }
                else if (lVar34 != 1) {
                  uStack_a2 = 1;
                    /* WARNING: Subroutine does not return */
                  core::panicking::panic();
                }
              }
            }
            plVar10 = local_b0;
            puStack_288 = puStack_118;
            local_298 = puStack_128;
            local_290 = puStack_120;
            if (*local_b0 != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            plVar10[2] = (longlong)puStack_288;
            *plVar10 = (longlong)local_298;
            plVar10[1] = (longlong)local_290;
            plVar10 = (longlong *)puVar25[-0x52];
            sVar3 = (longlong)plVar10 * 8;
            if (((ulonglong)plVar10 >> 0x3d != 0) || (0x7ffffffffffffffc < sVar3)) {
code_r0x0001409798e1:
              uStack_a2 = 0;
                    /* WARNING: Subroutine does not return */
              alloc::raw_vec::handle_error();
            }
            pvVar11 = (void *)puVar25[-0x53];
            if (sVar3 == 0) {
              puVar30 = &DAT_00000004;
              local_b8 = (longlong *)0x0;
            }
            else {
              std::alloc::__default_lib_allocator::__rust_alloc();
              puVar30 = extraout_RAX_04;
              local_b8 = plVar10;
              if (extraout_RAX_04 == (undefined1 *)0x0) goto code_r0x0001409798e1;
            }
            local_e0 = puVar30;
            memcpy(puVar30,pvVar11,sVar3);
            uVar33 = puVar25[-0x4f];
            sVar3 = uVar33 * 4;
            if ((uVar33 >> 0x3e != 0) || (0x7ffffffffffffffc < sVar3)) {
code_r0x0001409798ff:
                    /* WARNING: Subroutine does not return */
              alloc::raw_vec::handle_error();
            }
            pvVar11 = (void *)puVar25[-0x50];
            if (sVar3 == 0) {
              puVar30 = &DAT_00000004;
              uVar28 = 0;
            }
            else {
              std::alloc::__default_lib_allocator::__rust_alloc();
              puVar30 = extraout_RAX_07;
              uVar28 = uVar33;
              if (extraout_RAX_07 == (undefined1 *)0x0) goto code_r0x0001409798ff;
            }
            memcpy(puVar30,pvVar11,sVar3);
            plVar26 = local_b0;
            uVar6 = *(undefined4 *)(puVar25 + -0x4e);
            if (!SBORROW8(0,local_b0[3])) {
              if (local_b0[3] != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              if (plVar26[6] != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
            }
            plVar26[3] = (longlong)local_b8;
            plVar26[4] = (longlong)local_e0;
            plVar26[5] = (longlong)plVar10;
            plVar26[6] = uVar28;
            plVar26[7] = (longlong)puVar30;
            plVar26[8] = uVar33;
            *(undefined4 *)(plVar26 + 9) = uVar6;
            fVar61 = 0.0;
            fVar52 = 0.0;
            if ((*local_d0 & 1) != 0) {
              fVar61 = *(float *)(local_d0 + 4);
              fVar52 = *(float *)(local_d0 + 0xc);
            }
            if (puVar25[-0x52] == 0) {
              uStack_a2 = 0;
                    /* WARNING: Subroutine does not return */
              core::option::unwrap_failed();
            }
            plVar26[10] = CONCAT44(fVar52 - (float)((ulonglong)*(undefined8 *)puVar25[-0x53] >> 0x20
                                                   ),fVar61 - (float)*(undefined8 *)puVar25[-0x53]);
            puVar24 = local_d8;
            break;
          case 2:
            plVar10 = (longlong *)*local_b8;
            local_298 = puVar24;
            if (plVar10[3] == 0) goto code_r0x000140978e47;
            core::hash::BuildHasher::
            hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
            lVar32 = *plVar10;
            bVar44 = (byte)(extraout_RAX_02 >> 0x39);
            auVar46 = ZEXT216(CONCAT11(bVar44,bVar44));
            auVar46 = pshuflw(auVar46,auVar46,0);
            lVar34 = 0;
            uVar33 = extraout_RAX_02;
            goto code_r0x0001409788b5;
          case 3:
            plVar10 = (longlong *)local_b0[2];
            auVar46._8_8_ = 0;
            auVar46._0_8_ = plVar10;
            uVar33 = SUB168(auVar46 * ZEXT816(0xc),0);
            if ((SUB168(auVar46 * ZEXT816(0xc),8) != 0) || (0x7ffffffffffffffc < uVar33)) {
LAB_1409798cf:
                    /* WARNING: Subroutine does not return */
              alloc::raw_vec::handle_error();
            }
            pvVar11 = (void *)local_b0[1];
            if (uVar33 == 0) {
              pbVar23 = &DAT_00000004;
              local_b8 = (longlong *)0x0;
            }
            else {
              std::alloc::__default_lib_allocator::__rust_alloc();
              pbVar23 = extraout_RAX_03;
              local_b8 = plVar10;
              if (extraout_RAX_03 == (byte *)0x0) goto LAB_1409798cf;
            }
            local_d0 = pbVar23;
            memcpy(pbVar23,pvVar11,uVar33);
            *(undefined4 *)in_stack_00000080[2] =
                 *(undefined4 *)((longlong)in_stack_00000080 + 0x1c);
            lVar32 = *in_stack_00000080;
            lVar34 = *(longlong *)(lVar32 + 0x30);
            uVar8 = *(undefined8 *)(lVar32 + 0x40);
            if (lVar34 == *(longlong *)(lVar32 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>
                        ();
            }
            puVar25 = local_c0;
            lVar37 = *(longlong *)(lVar32 + 0x28);
            lVar40 = lVar34 * 0x30;
            *(longlong **)(lVar37 + lVar40) = local_b8;
            *(byte **)(lVar37 + 8 + lVar40) = local_d0;
            *(longlong **)(lVar37 + 0x10 + lVar40) = plVar10;
            *(undefined4 **)(lVar37 + 0x18 + lVar40) = puVar24;
            *(ulonglong *)(lVar37 + 0x20 + lVar40) = CONCAT44(fVar60,fVar59);
            *(undefined8 *)(lVar37 + 0x28 + lVar40) = uVar8;
            *(longlong *)(lVar32 + 0x30) = lVar34 + 1;
            *(longlong *)(lVar32 + 0x40) = *(longlong *)(lVar32 + 0x40) + 1;
            lVar32 = local_b0[3];
            local_b0[3] = -0x8000000000000000;
            if (!SBORROW8(0,lVar32)) {
              puStack_120 = (undefined8 *)local_b0[4];
              puStack_118 = (undefined4 *)local_b0[5];
              puStack_110 = (undefined4 *)local_b0[6];
              puStack_108 = (undefined4 *)local_b0[7];
              puStack_100 = (undefined4 *)local_b0[8];
              lStack_f8 = local_b0[9];
              uStack_1a0 = 0;
              puStack_198 = (undefined4 *)&DAT_00000008;
              lStack_190 = 0;
              plVar10 = (longlong *)local_c0[-0x52];
              sVar3 = (longlong)plVar10 * 8;
              puStack_128 = (undefined4 *)lVar32;
              if (((ulonglong)plVar10 >> 0x3d != 0) || (0x7ffffffffffffffc < sVar3)) {
LAB_140979951:
                    /* WARNING: Subroutine does not return */
                alloc::raw_vec::handle_error();
              }
              pvVar11 = (void *)local_c0[-0x53];
              if (sVar3 == 0) {
                plVar26 = (longlong *)&DAT_00000004;
                local_b0 = (longlong *)0x0;
              }
              else {
                std::alloc::__default_lib_allocator::__rust_alloc();
                plVar26 = extraout_RAX_06;
                local_b0 = plVar10;
                if (extraout_RAX_06 == (longlong *)0x0) goto LAB_140979951;
              }
              local_b8 = plVar26;
              memcpy(plVar26,pvVar11,sVar3);
              uVar33 = puVar25[-0x4f];
              sVar3 = uVar33 * 4;
              if ((uVar33 >> 0x3e != 0) || (0x7ffffffffffffffc < sVar3)) {
LAB_140979965:
                    /* WARNING: Subroutine does not return */
                alloc::raw_vec::handle_error();
              }
              pvVar11 = (void *)puVar25[-0x50];
              if (sVar3 == 0) {
                puVar30 = &DAT_00000004;
                uVar28 = 0;
              }
              else {
                std::alloc::__default_lib_allocator::__rust_alloc();
                puVar30 = extraout_RAX_08;
                uVar28 = uVar33;
                if (extraout_RAX_08 == (undefined1 *)0x0) goto LAB_140979965;
              }
              memcpy(puVar30,pvVar11,sVar3);
              uStack_228 = *(undefined4 *)(puVar25 + -0x4e);
              puStack_220 = local_d8;
              iStack_260 = (int)lStack_f8;
              uStack_25c = (undefined4)((ulonglong)lStack_f8 >> 0x20);
              puStack_270 = puStack_108;
              puStack_268 = puStack_100;
              puStack_280 = puStack_118;
              puStack_278 = puStack_110;
              local_290 = (undefined8 *)puStack_128;
              puStack_288 = (undefined4 *)puStack_120;
              uStack_258 = SUB84(local_b0,0);
              uStack_254 = (undefined4)((ulonglong)local_b0 >> 0x20);
              uStack_250 = SUB84(local_b8,0);
              uStack_24c = (undefined4)((ulonglong)local_b8 >> 0x20);
              uStack_248 = SUB84(plVar10,0);
              uStack_244 = (undefined4)((ulonglong)plVar10 >> 0x20);
              local_298 = (undefined4 *)0x800000000000000c;
              uStack_a3 = 1;
              uStack_240 = uVar28;
              puStack_238 = puVar30;
              uStack_230 = uVar33;
              resources::history::wall_compound_edit::CompoundEditBuilder::next();
              **(undefined4 **)(in_stack_00000040 + 0x10) =
                   *(undefined4 *)(in_stack_00000040 + 0x1c);
              puStack_128 = (undefined4 *)0x0;
              puStack_120 = (undefined8 *)&DAT_00000008;
              puStack_118 = (undefined4 *)0x0;
              puStack_280 = puStack_198 + lStack_190 * 6;
              local_298 = puStack_198;
              local_290 = (undefined8 *)puStack_198;
              puStack_288 = (undefined4 *)uStack_1a0;
              puVar29 = puStack_198;
              alloc::boxed::iter::impl$13::
              from_iter<country_core::resources::history::wall_compound_edit::ConcurrentEdits,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2_<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl_0::new::closure_env_0>_>
                        ();
              local_298 = (undefined4 *)&DAT_00000007;
              uStack_a3 = 0;
              local_290 = extraout_RAX_09;
              puStack_288 = puVar29;
              resources::history::History::add();
              puVar24 = local_d8;
            }
            puVar30 = local_e0;
            *local_178 = local_e4;
            lVar32 = *(longlong *)(local_e0 + 0x30);
            uVar8 = *(undefined8 *)(local_e0 + 0x40);
            if (lVar32 == *(longlong *)(local_e0 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                        ();
            }
            lVar34 = *(longlong *)(puVar30 + 0x28);
            *(undefined8 *)(lVar34 + lVar32 * 0x18) = uVar8;
            *(undefined4 **)(lVar34 + 8 + lVar32 * 0x18) = puVar24;
            *(undefined1 *)(lVar34 + 0x10 + lVar32 * 0x18) = 1;
            *(longlong *)(puVar30 + 0x30) = lVar32 + 1;
            *(longlong *)(puVar30 + 0x40) = *(longlong *)(puVar30 + 0x40) + 1;
          }
          goto LAB_140979772;
        }
        uVar39 = (ushort)(uVar21 - 1) & (ushort)uVar21;
        uVar21 = CONCAT22((short)(uVar21 - 1 >> 0x10),uVar39);
      }
      auVar53[0] = -(*pcVar1 == -1);
      auVar53[1] = -(pcVar1[1] == -1);
      auVar53[2] = -(pcVar1[2] == -1);
      auVar53[3] = -(pcVar1[3] == -1);
      auVar53[4] = -(pcVar1[4] == -1);
      auVar53[5] = -(pcVar1[5] == -1);
      auVar53[6] = -(pcVar1[6] == -1);
      auVar53[7] = -(pcVar1[7] == -1);
      auVar53[8] = -(pcVar1[8] == -1);
      auVar53[9] = -(pcVar1[9] == -1);
      auVar53[10] = -(pcVar1[10] == -1);
      auVar53[0xb] = -(pcVar1[0xb] == -1);
      auVar53[0xc] = -(pcVar1[0xc] == -1);
      auVar53[0xd] = -(pcVar1[0xd] == -1);
      auVar53[0xe] = -(pcVar1[0xe] == -1);
      auVar53[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar53 >> 7,0) & 1) != 0 || (SUB161(auVar53 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar53 >> 0x17,0) & 1) != 0) || (SUB161(auVar53 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar53 >> 0x27,0) & 1) != 0) || (SUB161(auVar53 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar53 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar53 >> 0x3f,0) & 1) != 0) || (SUB161(auVar53 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar53 >> 0x4f,0) & 1) != 0) || (SUB161(auVar53 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar53 >> 0x5f,0) & 1) != 0) || (SUB161(auVar53 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar53 >> 0x6f,0) & 1) != 0) || (SUB161(auVar53 >> 0x77,0) & 1) != 0) ||
          auVar53[0xf] < '\0') break;
      uVar33 = uVar33 + lVar37 + 0x10;
      lVar37 = lVar37 + 0x10;
    } while( true );
  }
                    /* WARNING: Subroutine does not return */
  core::option::unwrap_failed();
code_r0x0001409788b5:
  uVar33 = plVar10[1] & uVar33;
  pcVar1 = (char *)(lVar32 + uVar33);
  cVar45 = auVar46[0];
  auVar57[0] = -(*pcVar1 == cVar45);
  cVar47 = auVar46[1];
  auVar57[1] = -(pcVar1[1] == cVar47);
  cVar48 = auVar46[2];
  auVar57[2] = -(pcVar1[2] == cVar48);
  cVar49 = auVar46[3];
  auVar57[3] = -(pcVar1[3] == cVar49);
  auVar57[4] = -(pcVar1[4] == cVar45);
  auVar57[5] = -(pcVar1[5] == cVar47);
  auVar57[6] = -(pcVar1[6] == cVar48);
  auVar57[7] = -(pcVar1[7] == cVar49);
  auVar57[8] = -(pcVar1[8] == cVar45);
  auVar57[9] = -(pcVar1[9] == cVar47);
  auVar57[10] = -(pcVar1[10] == cVar48);
  auVar57[0xb] = -(pcVar1[0xb] == cVar49);
  auVar57[0xc] = -(pcVar1[0xc] == cVar45);
  auVar57[0xd] = -(pcVar1[0xd] == cVar47);
  auVar57[0xe] = -(pcVar1[0xe] == cVar48);
  auVar57[0xf] = -(pcVar1[0xf] == cVar49);
  uVar39 = (ushort)(SUB161(auVar57 >> 7,0) & 1) | (ushort)(SUB161(auVar57 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar57 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar57 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar57 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar57 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar57 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar57 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar57 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar57 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar57 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar57 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar57 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar57 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar57 >> 0x77,0) & 1) << 0xe | (ushort)(auVar57[0xf] >> 7) << 0xf;
  uVar21 = (uint)uVar39;
  while (uVar39 != 0) {
    uVar22 = 0;
    for (uVar51 = uVar21; (uVar51 & 1) == 0; uVar51 = uVar51 >> 1 | 0x80000000) {
      uVar22 = uVar22 + 1;
    }
    lVar37 = (uVar22 + uVar33 & plVar10[1]) * -0x78;
    if (*(undefined4 **)(lVar32 + -0x78 + lVar37) == puVar24) {
      puVar29 = *(undefined4 **)(lVar32 + -0x60 + lVar37);
      if (puVar29 == (undefined4 *)0x0) {
                    /* WARNING: Subroutine does not return */
        core::option::unwrap_failed();
      }
      fVar61 = 0.0;
      fVar52 = 0.0;
      if ((*local_d0 & 1) != 0) {
        fVar61 = *(float *)(local_d0 + 4);
        fVar52 = *(float *)(local_d0 + 0xc);
      }
      sVar3 = (longlong)puVar29 * 8;
      plStack_2a0 = plVar10;
      local_148 = puVar29;
      if (((ulonglong)puVar29 >> 0x3d != 0) || (0x7ffffffffffffffc < sVar3)) {
code_r0x000140979916:
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error();
      }
      pbVar23 = (byte *)(lVar32 + lVar37);
      puVar25 = *(undefined8 **)(pbVar23 + -0x68);
      uVar8 = *puVar25;
      lVar32 = local_b0[10];
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (_Dst == (undefined8 *)0x0) goto code_r0x000140979916;
      local_130 = sVar3;
      local_c0 = _Dst;
      memcpy(_Dst,puVar25,sVar3);
      puVar29 = *(undefined4 **)(pbVar23 + -0x48);
      sVar3 = (longlong)puVar29 * 4;
      if (((ulonglong)puVar29 >> 0x3e != 0) || (0x7ffffffffffffffc < sVar3)) {
code_r0x000140979928:
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error();
      }
      pvVar11 = *(void **)(pbVar23 + -0x50);
      local_d0 = pbVar23;
      if (sVar3 == 0) {
        puVar24 = (undefined4 *)&DAT_00000004;
        puVar41 = (undefined4 *)0x0;
      }
      else {
        std::alloc::__default_lib_allocator::__rust_alloc();
        puVar24 = extraout_RAX_05;
        puVar41 = puVar29;
        if (extraout_RAX_05 == (undefined4 *)0x0) goto code_r0x000140979928;
      }
      memcpy(puVar24,pvVar11,sVar3);
      fVar61 = (fVar61 - (float)lVar32) - (float)uVar8;
      fVar52 = (fVar52 - (float)((ulonglong)lVar32 >> 0x20)) - (float)((ulonglong)uVar8 >> 0x20);
      uVar6 = *(undefined4 *)(local_d0 + -0x40);
      puStack_128 = local_148;
      puStack_120 = local_c0;
      puStack_118 = local_148;
      lStack_f8 = CONCAT44(lStack_f8._4_4_,uVar6);
      puVar2 = local_c0 + (longlong)local_148;
      uVar33 = (ulonglong)((longlong)local_148 + -1) & 0x1fffffffffffffff;
      puVar25 = local_c0;
      if (uVar33 < 3) goto code_r0x000140978f50;
      uVar33 = uVar33 + 1;
      uVar28 = uVar33 & 0xfffffffffffffffc;
      puVar25 = local_c0 + uVar28;
      uVar35 = 0;
      do {
        pfVar4 = (float *)(local_c0 + uVar35);
        fVar55 = pfVar4[1];
        fVar58 = pfVar4[2];
        fVar14 = pfVar4[3];
        pfVar5 = (float *)(local_c0 + uVar35 + 2);
        fVar15 = *pfVar5;
        fVar16 = pfVar5[1];
        fVar17 = pfVar5[2];
        fVar18 = pfVar5[3];
        pfVar5 = (float *)(local_c0 + uVar35);
        *pfVar5 = *pfVar4 + fVar61;
        pfVar5[1] = fVar55 + fVar52;
        pfVar5[2] = fVar58 + fVar61;
        pfVar5[3] = fVar14 + fVar52;
        pfVar4 = (float *)(local_c0 + uVar35 + 2);
        *pfVar4 = fVar15 + fVar61;
        pfVar4[1] = fVar16 + fVar52;
        pfVar4[2] = fVar17 + fVar61;
        pfVar4[3] = fVar18 + fVar52;
        uVar35 = uVar35 + 4;
      } while (uVar28 != uVar35);
      if (uVar33 != uVar28) {
code_r0x000140978f50:
        do {
          *puVar25 = CONCAT44((float)((ulonglong)*puVar25 >> 0x20) + fVar52,(float)*puVar25 + fVar61
                             );
          puVar25 = puVar25 + 1;
        } while (puVar25 != puVar2);
      }
      local_140 = puVar41;
      local_138 = puVar24;
      puStack_110 = puVar41;
      puStack_108 = puVar24;
      puStack_100 = puVar29;
      if (*local_2c0 == 1) {
        cStack_a1 = '\x01';
        ::utils::geometry::rectangle::Rectangle2d::contains_point();
        pbVar23 = local_d0;
        puVar25 = local_c0;
        cVar45 = extraout_AL;
        while (cVar45 != '\0') {
          puVar25 = puVar25 + 1;
          if (puVar25 == puVar2) {
code_r0x00014097931c:
            uVar33 = local_b0[2];
            auVar13._8_8_ = 0;
            auVar13._0_8_ = uVar33;
            uVar28 = SUB168(auVar13 * ZEXT816(0xc),0);
            if ((SUB168(auVar13 * ZEXT816(0xc),8) != 0) || (0x7ffffffffffffffc < uVar28)) {
code_r0x000140979985:
              cStack_a1 = 1;
                    /* WARNING: Subroutine does not return */
              alloc::raw_vec::handle_error();
            }
            pvVar11 = (void *)local_b0[1];
            if (uVar28 == 0) {
              puVar30 = &DAT_00000004;
              local_b0 = (longlong *)0x0;
            }
            else {
              std::alloc::__default_lib_allocator::__rust_alloc();
              puVar30 = extraout_RAX_10;
              local_b0 = (longlong *)uVar33;
              if (extraout_RAX_10 == (undefined1 *)0x0) goto code_r0x000140979985;
            }
            local_c8 = puVar30;
            memcpy(puVar30,pvVar11,uVar28);
            *(undefined4 *)in_stack_00000080[2] =
                 *(undefined4 *)((longlong)in_stack_00000080 + 0x1c);
            lVar32 = *in_stack_00000080;
            lVar34 = *(longlong *)(lVar32 + 0x30);
            uVar8 = *(undefined8 *)(lVar32 + 0x40);
            if (lVar34 == *(longlong *)(lVar32 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>
                        ();
            }
            lVar37 = *(longlong *)(lVar32 + 0x28);
            lVar40 = lVar34 * 0x30;
            *(longlong **)(lVar37 + lVar40) = local_b0;
            *(undefined1 **)(lVar37 + 8 + lVar40) = local_c8;
            *(ulonglong *)(lVar37 + 0x10 + lVar40) = uVar33;
            *(undefined4 **)(lVar37 + 0x18 + lVar40) = local_d8;
            *(ulonglong *)(lVar37 + 0x20 + lVar40) = CONCAT44(fVar60,fVar59);
            *(undefined8 *)(lVar37 + 0x28 + lVar40) = uVar8;
            *(longlong *)(lVar32 + 0x30) = lVar34 + 1;
            *(longlong *)(lVar32 + 0x40) = *(longlong *)(lVar32 + 0x40) + 1;
            local_b0 = *(longlong **)(pbVar23 + -0x60);
            sVar3 = (longlong)local_b0 * 8;
            if (((ulonglong)local_b0 >> 0x3d != 0) || (0x7ffffffffffffffc < sVar3)) {
code_r0x00014097945b:
              cStack_a1 = 1;
                    /* WARNING: Subroutine does not return */
              alloc::raw_vec::handle_error();
            }
            pvVar11 = *(void **)(pbVar23 + -0x68);
            if (sVar3 == 0) {
              puVar30 = &DAT_00000004;
              local_b0 = (longlong *)0x0;
            }
            else {
              std::alloc::__default_lib_allocator::__rust_alloc();
              puVar30 = extraout_RAX_11;
              if (extraout_RAX_11 == (undefined1 *)0x0) goto code_r0x00014097945b;
            }
            local_c8 = puVar30;
            memcpy(puVar30,pvVar11,sVar3);
            uVar33 = *(ulonglong *)(pbVar23 + -0x48);
            sVar3 = uVar33 * 4;
            if ((uVar33 >> 0x3e != 0) || (0x7ffffffffffffffc < sVar3)) {
code_r0x00014097999d:
                    /* WARNING: Subroutine does not return */
              alloc::raw_vec::handle_error();
            }
            pvVar11 = *(void **)(pbVar23 + -0x50);
            if (sVar3 == 0) {
              puVar30 = &DAT_00000004;
              uVar33 = 0;
            }
            else {
              std::alloc::__default_lib_allocator::__rust_alloc();
              puVar30 = extraout_RAX_12;
              if (extraout_RAX_12 == (undefined1 *)0x0) goto code_r0x00014097999d;
            }
            memcpy(puVar30,pvVar11,sVar3);
            uVar21 = *(int *)(pbVar23 + -0x38) - 2;
            if (1 < uVar21) {
              uVar21 = 2;
            }
            iVar42 = 2;
            if (uVar21 != 0) {
              if (uVar21 == 2) {
                uStack_2d8 = *(undefined8 *)(pbVar23 + -0x24);
                uStack_2e8 = *(undefined8 *)(pbVar23 + -0x34);
                uStack_2e0 = *(undefined8 *)(pbVar23 + -0x2c);
                iVar42 = *(int *)(pbVar23 + -0x38);
              }
              else {
                iVar42 = 3;
              }
            }
            uVar7 = *(undefined4 *)(pbVar23 + -0x1c);
            uVar28 = *(ulonglong *)(pbVar23 + -0x18);
            uVar20 = *(undefined2 *)(pbVar23 + -8);
            puVar30 = *(undefined1 **)(pbVar23 + -0x10);
            if (local_b0 != (longlong *)0x0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            if (uVar33 != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            *(undefined4 *)local_b8[2] = *(undefined4 *)((longlong)local_b8 + 0x1c);
            local_298 = local_148;
            local_290 = local_c0;
            puStack_288 = local_148;
            puStack_280 = local_140;
            puStack_278 = local_138;
            puStack_268 = (undefined4 *)CONCAT44(puStack_268._4_4_,uVar6);
            uStack_25c = (undefined4)uStack_2e8;
            uStack_258 = (undefined4)((ulonglong)uStack_2e8 >> 0x20);
            uStack_254 = (undefined4)uStack_2e0;
            uStack_250 = (undefined4)((ulonglong)uStack_2e0 >> 0x20);
            uStack_24c = (undefined4)uStack_2d8;
            uStack_248 = (undefined4)((ulonglong)uStack_2d8 >> 0x20);
            uStack_230 = CONCAT62(uStack_230._2_6_,uVar20);
            cStack_a1 = '\0';
            puStack_270 = puVar29;
            iStack_260 = iVar42;
            uStack_244 = uVar7;
            uStack_240 = uVar28;
            puStack_238 = puVar30;
            resources::walls::inner_walls::InnerWalls::
            _try_update_state<country_core::resources::walls::inner_walls::impl_1::update_wall_state::closure_env_0>
                      ();
            bVar19 = true;
            cVar45 = '\0';
            goto code_r0x000140979644;
          }
          cStack_a1 = '\x01';
          ::utils::geometry::rectangle::Rectangle2d::contains_point();
          cVar45 = extraout_AL_00;
        }
      }
      else {
        cStack_a1 = '\x01';
        ::utils::geometry::circle::Circle2d::contains_point();
        pbVar23 = local_d0;
        puVar25 = local_c0;
        cVar45 = extraout_AL_01;
        while (cVar45 != '\0') {
          puVar25 = puVar25 + 1;
          if (puVar25 == puVar2) goto code_r0x00014097931c;
          cStack_a1 = '\x01';
          ::utils::geometry::circle::Circle2d::contains_point();
          cVar45 = extraout_AL_02;
        }
      }
      *local_2a8 = local_15c;
      lVar32 = *(longlong *)(local_150 + 0x30);
      uVar8 = *(undefined8 *)(local_150 + 0x40);
      if (lVar32 == *(longlong *)(local_150 + 0x20)) {
        cStack_a1 = '\x01';
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                  ();
      }
      *(undefined8 *)(*(longlong *)(local_150 + 0x28) + lVar32 * 8) = uVar8;
      *(longlong *)(local_150 + 0x30) = lVar32 + 1;
      *(longlong *)(local_150 + 0x40) = *(longlong *)(local_150 + 0x40) + 1;
      *local_2b0 = local_160;
      lVar32 = *(longlong *)(local_c8 + 0x30);
      uVar8 = *(undefined8 *)(local_c8 + 0x40);
      if (lVar32 == *(longlong *)(local_c8 + 0x20)) {
        cStack_a1 = '\x01';
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                  ();
      }
      *(undefined8 *)(*(longlong *)(local_c8 + 0x28) + lVar32 * 8) = uVar8;
      *(longlong *)(local_c8 + 0x30) = lVar32 + 1;
      *(longlong *)(local_c8 + 0x40) = *(longlong *)(local_c8 + 0x40) + 1;
      cVar45 = '\x01';
      bVar19 = false;
code_r0x000140979644:
      *local_178 = local_e4;
      lVar32 = *(longlong *)(local_e0 + 0x30);
      uVar8 = *(undefined8 *)(local_e0 + 0x40);
      if (lVar32 == *(longlong *)(local_e0 + 0x20)) {
        cStack_a1 = cVar45;
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      lVar34 = *(longlong *)(local_e0 + 0x28);
      *(undefined8 *)(lVar34 + lVar32 * 0x18) = uVar8;
      *(undefined4 **)(lVar34 + 8 + lVar32 * 0x18) = local_d8;
      *(undefined1 *)(lVar34 + 0x10 + lVar32 * 0x18) = 0;
      *(longlong *)(local_e0 + 0x30) = lVar32 + 1;
      *(longlong *)(local_e0 + 0x40) = *(longlong *)(local_e0 + 0x40) + 1;
      *local_2b8 = local_164;
      lVar32 = *(longlong *)(local_158 + 0x30);
      uVar8 = *(undefined8 *)(local_158 + 0x40);
      if (lVar32 == *(longlong *)(local_158 + 0x20)) {
        cStack_a1 = cVar45;
        alloc::raw_vec::RawVec::
        grow_one<country_core::resources::window::AvailableMonitor,alloc::alloc::Global>();
      }
      puVar24 = local_d8;
      puVar29 = local_140;
      lVar34 = *(longlong *)(local_158 + 0x28);
      lVar37 = lVar32 * 0x40;
      *(undefined1 *)(lVar34 + lVar37) = 2;
      *(undefined4 **)(lVar34 + 8 + lVar37) = local_d8;
      *(undefined8 *)(lVar34 + 0x38 + lVar37) = uVar8;
      *(longlong *)(local_158 + 0x30) = lVar32 + 1;
      *(longlong *)(local_158 + 0x40) = *(longlong *)(local_158 + 0x40) + 1;
      if ((cVar45 != '\0') &&
         (std::alloc::__default_lib_allocator::__rust_dealloc(), puVar29 != (undefined4 *)0x0)) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (!bVar19) {
        return;
      }
LAB_140979772:
      uVar33 = *local_180;
      uVar35 = 0;
      uVar28 = uVar33 - *(ulonglong *)(local_2c8 + 0x18);
      if (uVar33 < *(ulonglong *)(local_2c8 + 0x18)) {
        uVar28 = uVar35;
      }
      uVar27 = uVar33 - *(ulonglong *)(local_2c8 + 0x38);
      if (uVar33 < *(ulonglong *)(local_2c8 + 0x38)) {
        uVar27 = uVar35;
      }
      uVar33 = *(ulonglong *)(local_2c8 + 0x30);
      uVar36 = uVar33 - uVar27;
      if (uVar33 < uVar27) {
        uVar36 = uVar35;
      }
      uVar12 = *(ulonglong *)(local_2c8 + 0x10);
      uVar38 = uVar12 - uVar28;
      if (uVar12 < uVar28) {
        uVar38 = uVar35;
      }
      uVar35 = *(ulonglong *)(local_2c8 + 0x40);
      *local_180 = uVar35 - (uVar38 + uVar36);
      if ((uVar12 < uVar28 || uVar12 - uVar28 == 0) && (uVar33 < uVar27 || uVar33 - uVar27 == 0)) {
        return;
      }
      *local_180 = uVar35;
      *(undefined4 *)in_stack_00000048[2] = *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
      lVar32 = *in_stack_00000048;
      lVar34 = *(longlong *)(lVar32 + 0x30);
      uVar8 = *(undefined8 *)(lVar32 + 0x40);
      if (lVar34 == *(longlong *)(lVar32 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                  ();
      }
      lVar37 = *(longlong *)(lVar32 + 0x28);
      lVar40 = lVar34 * 0x20;
      *(undefined8 *)(lVar37 + lVar40) = uVar8;
      *(undefined4 **)(lVar37 + 8 + lVar40) = puVar24;
      *(undefined4 **)(lVar37 + 0x10 + lVar40) = local_170;
      *(undefined4 *)(lVar37 + 0x18 + lVar40) = local_168;
      *(undefined2 *)(lVar37 + 0x1c + lVar40) = 1;
      *(longlong *)(lVar32 + 0x30) = lVar34 + 1;
      *(longlong *)(lVar32 + 0x40) = *(longlong *)(lVar32 + 0x40) + 1;
      return;
    }
    uVar39 = (ushort)(uVar21 - 1) & (ushort)uVar21;
    uVar21 = CONCAT22((short)(uVar21 - 1 >> 0x10),uVar39);
  }
  auVar54[0] = -(*pcVar1 == -1);
  auVar54[1] = -(pcVar1[1] == -1);
  auVar54[2] = -(pcVar1[2] == -1);
  auVar54[3] = -(pcVar1[3] == -1);
  auVar54[4] = -(pcVar1[4] == -1);
  auVar54[5] = -(pcVar1[5] == -1);
  auVar54[6] = -(pcVar1[6] == -1);
  auVar54[7] = -(pcVar1[7] == -1);
  auVar54[8] = -(pcVar1[8] == -1);
  auVar54[9] = -(pcVar1[9] == -1);
  auVar54[10] = -(pcVar1[10] == -1);
  auVar54[0xb] = -(pcVar1[0xb] == -1);
  auVar54[0xc] = -(pcVar1[0xc] == -1);
  auVar54[0xd] = -(pcVar1[0xd] == -1);
  auVar54[0xe] = -(pcVar1[0xe] == -1);
  auVar54[0xf] = -(pcVar1[0xf] == -1);
  if ((((((((((((((((SUB161(auVar54 >> 7,0) & 1) != 0 || (SUB161(auVar54 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar54 >> 0x17,0) & 1) != 0) || (SUB161(auVar54 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar54 >> 0x27,0) & 1) != 0) || (SUB161(auVar54 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar54 >> 0x37,0) & 1) != 0) || (SUB161(auVar54 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar54 >> 0x47,0) & 1) != 0) || (SUB161(auVar54 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar54 >> 0x57,0) & 1) != 0) || (SUB161(auVar54 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar54 >> 0x67,0) & 1) != 0) || (SUB161(auVar54 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar54 >> 0x77,0) & 1) != 0) || auVar54[0xf] < '\0') {
code_r0x000140978e47:
                    /* WARNING: Subroutine does not return */
    core::option::unwrap_failed();
  }
  uVar33 = uVar33 + lVar34 + 0x10;
  lVar34 = lVar34 + 0x10;
  goto code_r0x0001409788b5;
}

