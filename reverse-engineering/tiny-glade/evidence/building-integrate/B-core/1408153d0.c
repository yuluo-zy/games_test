
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl system_decorator::cache_decorator_transforms::cache_decorator_transforms(void)

{
  byte *pbVar1;
  float fVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  byte bVar5;
  byte bVar6;
  ulonglong uVar7;
  ushort uVar8;
  uint uVar9;
  undefined1 *puVar10;
  code *pcVar11;
  float *pfVar12;
  float fVar13;
  float fVar14;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  byte bVar15;
  char extraout_AL_02;
  char extraout_AL_03;
  uint uVar16;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong lVar17;
  ulonglong extraout_RAX_02;
  longlong extraout_RAX_03;
  ulonglong extraout_RAX_04;
  float *extraout_RAX_05;
  longlong extraout_RAX_06;
  undefined8 *extraout_RAX_07;
  longlong extraout_RAX_08;
  float *extraout_RAX_09;
  longlong extraout_RAX_10;
  undefined8 *extraout_RAX_11;
  float *extraout_RAX_12;
  longlong extraout_RAX_13;
  float *pfVar18;
  longlong extraout_RAX_14;
  undefined8 *extraout_RAX_15;
  longlong extraout_RAX_16;
  undefined8 *extraout_RAX_17;
  ulonglong extraout_RAX_18;
  ulonglong uVar19;
  undefined8 *extraout_RAX_19;
  longlong *extraout_RAX_20;
  longlong *extraout_RAX_21;
  longlong *extraout_RAX_22;
  undefined8 *in_RCX;
  longlong lVar20;
  float *pfVar21;
  ulonglong uVar22;
  longlong *plVar23;
  undefined8 *in_RDX;
  undefined8 *puVar24;
  undefined1 (*pauVar25) [16];
  ulonglong uVar26;
  ulonglong uVar27;
  undefined *puVar28;
  longlong *in_R9;
  longlong lVar29;
  longlong lVar30;
  undefined1 (*pauVar31) [16];
  longlong lVar32;
  ulonglong uVar33;
  undefined1 (*pauVar34) [16];
  longlong *plVar35;
  float extraout_XMM0_Da;
  float fVar36;
  float fVar37;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  uint uVar38;
  undefined4 extraout_XMM0_Da_05;
  float fVar39;
  float fVar40;
  float fVar41;
  int iVar43;
  undefined1 auVar42 [16];
  float fVar44;
  float fVar45;
  float fVar46;
  float fVar47;
  float fVar48;
  float fVar49;
  float fVar50;
  float fVar51;
  float fVar52;
  float fVar53;
  float unaff_XMM11_Da;
  float fVar54;
  float unaff_XMM11_Db;
  float unaff_XMM11_Dc;
  float unaff_XMM11_Dd;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  longlong *in_stack_00000038;
  undefined8 *in_stack_00000040;
  undefined8 *in_stack_00000048;
  ulonglong uStack_2d0;
  undefined8 local_280;
  undefined1 (*local_278) [16];
  undefined1 *puStack_270;
  ushort local_268;
  undefined6 uStack_266;
  undefined8 local_260;
  longlong local_258;
  longlong local_250;
  undefined1 (*local_248) [16];
  longlong local_240;
  longlong local_238;
  longlong local_230;
  undefined8 *local_228;
  undefined8 local_220;
  undefined1 (*local_218) [16];
  undefined1 *puStack_210;
  undefined8 local_208;
  undefined8 local_1f8;
  longlong local_1f0;
  longlong local_1e8;
  longlong local_1d8;
  uint local_1cc;
  undefined4 *local_1c8;
  undefined8 local_1c0;
  undefined8 local_1b8;
  undefined8 uStack_1b0;
  undefined1 *local_1a8;
  undefined4 uStack_1a0;
  float fStack_19c;
  undefined4 uStack_198;
  undefined4 uStack_194;
  ulonglong uStack_190;
  undefined4 *local_180;
  undefined8 local_178;
  undefined1 *local_170;
  undefined1 (*local_168) [16];
  undefined1 *puStack_160;
  undefined8 local_158;
  undefined8 local_150;
  float local_148;
  undefined8 local_140;
  code *local_138;
  undefined4 local_130;
  char local_129;
  float local_128;
  float fStack_124;
  undefined8 uStack_120;
  undefined8 local_118;
  ushort uStack_110;
  undefined2 uStack_10e;
  float fStack_10c;
  undefined4 uStack_108;
  undefined4 uStack_104;
  ulonglong uStack_100;
  undefined8 local_f8;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  puVar24 = in_RDX;
  local_228 = in_RCX;
  puffin::are_scopes_on();
  local_129 = extraout_AL;
  if (extraout_AL != '\0') {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (puVar24 != (undefined8 *)0x0) {
        if (&DAT_00000047 < puVar24) {
          if (puVar24 != (undefined8 *)&DAT_00000048) goto LAB_1408154a5;
        }
        else if (*(char *)((longlong)puVar24 + 0x142a8faeb) < -0x40) {
LAB_1408154a5:
                    /* WARNING: Subroutine does not return */
          core::str::slice_error_fail();
        }
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (puVar24 != (undefined8 *)0xfffffffffffffffe)) {
        if ((undefined1 *)((longlong)puVar24 + 2) < (undefined1 *)0x48) {
          if (*(char *)((longlong)puVar24 + 0x142a8faed) < -0x40) {
LAB_1408154f9:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if ((undefined1 *)((longlong)puVar24 + 2) != (undefined1 *)0x48) goto LAB_1408154f9;
      }
    }
    puVar28 = &DAT_142a8fd02;
    do {
      bVar15 = puVar28[-1];
      uVar16 = (uint)(char)bVar15;
      if ((char)bVar15 < '\0') {
        bVar5 = puVar28[-2];
        if ((char)bVar5 < -0x40) {
          bVar6 = puVar28[-3];
          if ((char)bVar6 < -0x40) {
            pbVar1 = puVar28 + -4;
            puVar28 = puVar28 + -4;
            uVar16 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar28 = puVar28 + -3;
            uVar16 = bVar6 & 0xf;
          }
          uVar16 = bVar5 & 0x3f | uVar16 << 6;
        }
        else {
          puVar28 = puVar28 + -2;
          uVar16 = bVar5 & 0x1f;
        }
        uVar16 = (uint)(bVar15 & 0x3f) | uVar16 << 6;
        if (uVar16 != 0x5c) goto LAB_1408155b4;
LAB_1408155c6:
        if (puVar28 == &UNK_142a8fcc7) break;
        if (puVar28 + -0x142a8fcc7 < (undefined *)0x3a) {
          if (-0x41 < (char)puVar28[1]) break;
        }
        else if (puVar28 + -0x142a8fcc7 == (undefined *)0x3a) break;
                    /* WARNING: Subroutine does not return */
        core::str::slice_error_fail();
      }
      puVar28 = puVar28 + -1;
      if (uVar16 == 0x5c) goto LAB_1408155c6;
LAB_1408155b4:
      if (uVar16 == 0x2f) goto LAB_1408155c6;
    } while (puVar28 != &DAT_142a8fcc8);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar23 = extraout_RAX_01 + 1;
    }
    else {
      if (*extraout_RAX_01 != 0) goto LAB_140817284;
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      plVar23 = extraout_RAX_21;
      if (extraout_RAX_21 == (longlong *)0x0) goto LAB_140817284;
    }
    if (*plVar23 != 0) goto LAB_140817417;
    *plVar23 = -1;
    local_f8 = plVar23;
    puffin::ThreadProfiler::begin_scope();
    *local_f8 = *local_f8 + 1;
  }
  local_130 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  local_1c8 = (undefined4 *)in_RDX[2];
  *local_1c8 = local_130;
  local_1c0 = *in_RDX;
  country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::
  swap_buffers_and_clear();
  local_220 = *local_228;
  country_core::resources::walls::decorator_storage::Decorators::iter_removed_ids();
  fVar54 = _UNK_1429cd19c;
  fVar45 = _UNK_1429cd198;
  fVar40 = _UNK_1429cd194;
  fVar46 = ___xmm_bf8000003f8000003f800000bf800000;
  fVar47 = __real_c2c80000;
  fVar37 = DAT_14293e0c8._4_4_;
  fVar36 = (float)DAT_14293e0c8;
  fVar49 = __xmm_3f800000000000000000000000000000._4_4_;
  fVar44 = (float)__xmm_3f800000000000000000000000000000;
  lVar30 = CONCAT44(fStack_124,local_128);
  lVar32 = CONCAT44(uStack_104,uStack_108);
  local_1f0 = *in_R9;
  local_238 = *in_stack_00000038;
  local_240 = local_238 + 0xa8;
  local_1e8 = *in_stack_00000030;
  local_250 = in_stack_00000030[1];
  local_230 = local_238 + 0xf0;
  uVar16 = (uint)uStack_110;
  pauVar31 = (undefined1 (*) [16])CONCAT44(uStack_120._4_4_,(float)uStack_120);
  while (lVar32 != 0) {
    lVar32 = lVar32 + -1;
    if ((short)uVar16 == 0) {
      do {
        auVar42 = *pauVar31;
        uVar8 = (ushort)(SUB161(auVar42 >> 7,0) & 1) | (ushort)(SUB161(auVar42 >> 0xf,0) & 1) << 1 |
                (ushort)(SUB161(auVar42 >> 0x17,0) & 1) << 2 |
                (ushort)(SUB161(auVar42 >> 0x1f,0) & 1) << 3 |
                (ushort)(SUB161(auVar42 >> 0x27,0) & 1) << 4 |
                (ushort)(SUB161(auVar42 >> 0x2f,0) & 1) << 5 |
                (ushort)(SUB161(auVar42 >> 0x37,0) & 1) << 6 |
                (ushort)(SUB161(auVar42 >> 0x3f,0) & 1) << 7 |
                (ushort)(SUB161(auVar42 >> 0x47,0) & 1) << 8 |
                (ushort)(SUB161(auVar42 >> 0x4f,0) & 1) << 9 |
                (ushort)(SUB161(auVar42 >> 0x57,0) & 1) << 10 |
                (ushort)(SUB161(auVar42 >> 0x5f,0) & 1) << 0xb |
                (ushort)(SUB161(auVar42 >> 0x67,0) & 1) << 0xc |
                (ushort)(SUB161(auVar42 >> 0x6f,0) & 1) << 0xd |
                (ushort)(SUB161(auVar42 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar42[0xf] >> 7) << 0xf;
        lVar30 = lVar30 + -0x980;
        pauVar31 = pauVar31 + 1;
      } while (uVar8 == 0xffff);
      uVar16 = ~(uint)uVar8;
      local_1cc = uVar16 & -(uint)uVar8 - 2;
      local_248 = pauVar31;
    }
    else {
      if (lVar30 == 0) break;
      local_1cc = uVar16 - 1 & uVar16;
      local_248 = pauVar31;
    }
    uVar38 = 0;
    for (; (uVar16 & 1) == 0; uVar16 = uVar16 >> 1 | 0x80000000) {
      uVar38 = uVar38 + 1;
    }
    lVar20 = (ulonglong)uVar38 * -0x98;
    local_180 = *(undefined4 **)(lVar30 + -0x98 + lVar20);
    pauVar25 = *(undefined1 (**) [16])(lVar30 + -0x60 + lVar20);
    auVar42 = *pauVar25;
    pauVar34 = pauVar25 + 1;
    lVar20 = *(longlong *)(lVar30 + -0x48 + lVar20);
    uVar38 = ~(uint)(ushort)((ushort)(SUB161(auVar42 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar42 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar42 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar42 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar42 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar42 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar42 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar42 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar42 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar42 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar42 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar42 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar42 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar42 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar42 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar42[0xf] >> 7) << 0xf);
    while (uVar16 = local_1cc, pauVar31 = local_248, lVar20 != 0) {
      lVar20 = lVar20 + -1;
      if ((short)uVar38 == 0) {
        do {
          auVar42 = *pauVar34;
          uVar8 = (ushort)(SUB161(auVar42 >> 7,0) & 1) | (ushort)(SUB161(auVar42 >> 0xf,0) & 1) << 1
                  | (ushort)(SUB161(auVar42 >> 0x17,0) & 1) << 2 |
                  (ushort)(SUB161(auVar42 >> 0x1f,0) & 1) << 3 |
                  (ushort)(SUB161(auVar42 >> 0x27,0) & 1) << 4 |
                  (ushort)(SUB161(auVar42 >> 0x2f,0) & 1) << 5 |
                  (ushort)(SUB161(auVar42 >> 0x37,0) & 1) << 6 |
                  (ushort)(SUB161(auVar42 >> 0x3f,0) & 1) << 7 |
                  (ushort)(SUB161(auVar42 >> 0x47,0) & 1) << 8 |
                  (ushort)(SUB161(auVar42 >> 0x4f,0) & 1) << 9 |
                  (ushort)(SUB161(auVar42 >> 0x57,0) & 1) << 10 |
                  (ushort)(SUB161(auVar42 >> 0x5f,0) & 1) << 0xb |
                  (ushort)(SUB161(auVar42 >> 0x67,0) & 1) << 0xc |
                  (ushort)(SUB161(auVar42 >> 0x6f,0) & 1) << 0xd |
                  (ushort)(SUB161(auVar42 >> 0x77,0) & 1) << 0xe |
                  (ushort)(byte)(auVar42[0xf] >> 7) << 0xf;
          pauVar25 = pauVar25 + -0x50;
          pauVar34 = pauVar34 + 1;
        } while (uVar8 == 0xffff);
        uVar38 = ~(uint)uVar8;
        uVar16 = uVar38 & -(uint)uVar8 - 2;
      }
      else {
        uVar16 = uVar38 - 1 & uVar38;
      }
      uVar9 = 0;
      for (; (uVar38 & 1) == 0; uVar38 = uVar38 >> 1 | 0x80000000) {
        uVar9 = uVar9 + 1;
      }
      uVar19 = (ulonglong)uVar9;
      local_f8._4_4_ = (undefined4)((ulonglong)local_f8 >> 0x20);
      local_f8 = (longlong *)CONCAT44(local_f8._4_4_,*(undefined4 *)pauVar25[uVar19 * -5 + -5]);
      lVar17 = (ulonglong)(byte)pauVar25[uVar19 * -5 + -3][10] - 1;
      if (2 < (byte)(pauVar25[uVar19 * -5 + -3][10] - 2)) {
        lVar17 = 0;
      }
      if (lVar17 == 0) {
        local_140 = *(undefined8 **)(pauVar25[uVar19 * -5 + -5] + 8);
        country_core::resources::walls::public_walls::PublicWalls::get();
        if (extraout_RAX_03 == 0) {
                    /* WARNING: Subroutine does not return */
          core::option::unwrap_failed();
        }
        country_core::resources::walls::wall_attachment::WallCornerAttachment::
        to_rectangle_corner_id();
        if (((extraout_RAX_04 & 1) == 0) ||
           (country_core::resources::walls::public_wall_state::PublicWallState::
            corner_pos_and_normal_xz(), ((uint)local_128 & 1) == 0)) {
          ::utils::wall_space::WallSpaceCoord::get_tangent();
          ::utils::wall_space::WallSpaceCoord::to_world_space();
          ::utils::xz_rotation_bad();
          fVar39 = (float)local_1b8;
          fVar41 = (float)uStack_1b0;
          fVar48 = local_1b8._4_4_;
        }
        else {
          local_218 = (undefined1 (*) [16])CONCAT44((float)uStack_120,fStack_124);
          ::utils::swizzlers::impl$1::xvy();
          ::utils::xz_rotation_bad();
          fVar39 = (float)local_280;
          fVar41 = local_278._0_4_;
          local_128 = (float)local_1b8;
          fStack_124 = local_1b8._4_4_;
          uStack_120._0_4_ = (float)uStack_1b0;
          uStack_120._4_4_ = uStack_1b0._4_4_;
          fVar48 = local_280._4_4_;
        }
LAB_140815f30:
        local_170 = (undefined1 *)0x3f8000003f800000;
        local_168 = (undefined1 (*) [16])CONCAT44(local_168._4_4_,0x3f800000);
      }
      else {
        if (lVar17 != 1) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic();
        }
        fVar2 = *(float *)pauVar25[uVar19 * -5 + -4];
        country_core::resources::walls::public_walls::PublicWalls::roof_entity();
        if (extraout_RAX_02 >> 0x20 != 0) {
          uVar22 = extraout_RAX_02 & 0xffffffff;
          iVar43 = (int)(extraout_RAX_02 >> 0x20);
          if ((uVar22 < *(ulonglong *)(local_250 + 0x10)) &&
             (*(int *)(*(longlong *)(local_250 + 8) + uVar22 * 0x14) == iVar43)) {
            lVar17 = *(longlong *)(local_250 + 8) + uVar22 * 0x14;
            uVar22 = (ulonglong)*(uint *)(lVar17 + 4);
            if (uVar22 != 0xffffffff) {
              if ((*(ulonglong *)(local_1e8 + 0x38) <= uVar22) ||
                 ((*(ulonglong *)
                    (*(longlong *)(local_1e8 + 0x28) + (ulonglong)(*(uint *)(lVar17 + 4) >> 6) * 8)
                   >> (uVar22 & 0x3f) & 1) == 0)) {
                lVar30 = uVar22 << 0x20;
                goto LAB_140817439;
              }
              uVar22 = (ulonglong)*(uint *)(lVar17 + 0xc);
              lVar29 = *(longlong *)(local_250 + 0x1a8);
              if (*(ulonglong *)(local_1e8 + 0x110) < *(ulonglong *)(lVar29 + 0x40 + uVar22 * 0x48))
              {
                uVar27 = *(ulonglong *)
                          (*(longlong *)(lVar29 + 0x38 + uVar22 * 0x48) +
                          *(ulonglong *)(local_1e8 + 0x110) * 8);
                if (uVar27 == 0) goto LAB_140815aeb;
                lVar29 = *(longlong *)
                          (*(longlong *)(lVar29 + uVar22 * 0x48 + 0x18) + 0x10 + ~uVar27 * 0x30);
              }
              else {
LAB_140815aeb:
                lVar29 = 0;
              }
              uVar38 = *(uint *)(lVar17 + 0x10);
              country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume::
              get_position_from_mesh_uv();
              fVar41 = uStack_120._4_4_;
              fVar48 = (float)uStack_120;
              fVar39 = fStack_124;
              if (local_128 == 1.4013e-45) {
                local_1d8 = lVar29 + (ulonglong)uVar38 * 0x58;
                fVar50 = fVar44;
                fVar51 = fVar49;
                fVar52 = fVar36;
                fVar53 = fVar37;
                if ((~pauVar25[uVar19 * -5 + -1][0xe] & 0x3e) == 0 ||
                    (~pauVar25[uVar19 * -5 + -1][0xe] & 0x38) != 0) {
                  country_core::resources::roofs::roof_shape::Roof::rotation_yaw_radians();
                  local_1b8 = (undefined1 (*) [16])
                              CONCAT44(local_1b8._4_4_,fVar2 + extraout_XMM0_Da);
                  country_core::resources::walls::decorator::DecoratorRotationXZ::to_quat();
                  fVar50 = local_128;
                  fVar51 = fStack_124;
                  fVar52 = (float)uStack_120;
                  fVar53 = uStack_120._4_4_;
                }
                country_core::resources::roofs::roof_shape::Roof::is_flat();
                fVar2 = fVar50;
                fVar13 = fVar51;
                fVar14 = fVar52;
                if ((extraout_AL_00 == '\0') ||
                   (bVar15 = pauVar25[uVar19 * -5 + -1][0xe] - 0x38, bVar15 < 8 && bVar15 != 6)) {
                  if (pauVar25[uVar19 * -5 + -1][0xe] == '?') {
                    local_128 = (float)local_f8;
                    indexmap::map::IndexMap::
                    get<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::resources::walls::decorator::DecoratorId>
                              ();
                    if (extraout_RAX_06 == 0) {
                      pfVar18 = (float *)&DAT_00000004;
                      pfVar12 = (float *)&DAT_00000004;
                    }
                    else {
                      pfVar18 = *(float **)(extraout_RAX_06 + 8) +
                                *(longlong *)(extraout_RAX_06 + 0x10) * 2;
                      pfVar12 = *(float **)(extraout_RAX_06 + 8);
                    }
                    do {
                      pfVar21 = pfVar12;
                      if (pfVar21 == pfVar18) goto LAB_140815eed;
                      pfVar12 = pfVar21 + 2;
                    } while (*(char *)(pfVar21 + 1) != '\0');
                    fStack_124 = *pfVar21;
                    local_128 = (float)local_f8;
                    indexmap::map::IndexMap::
                    get<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,country_core::resources::stairs::stairs_assembly_params::AdjustedStairEdge,std::hash::random::RandomState,tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>_>
                              ();
                    if (extraout_RAX_08 == 0) {
                    /* WARNING: Subroutine does not return */
                      core::panicking::panic();
                    }
                    ::utils::xz_rotation_bad();
                    fVar2 = local_128;
                    fVar13 = fStack_124;
                    fVar14 = (float)uStack_120;
                    fVar53 = uStack_120._4_4_;
                  }
                  else {
                    if ((*(byte *)(local_1d8 + 8) & 1) != 0) {
                      if (2 < (byte)(pauVar25[uVar19 * -5 + -1][0xc] - 1)) {
                    /* WARNING: Subroutine does not return */
                        core::panicking::panic();
                      }
                      local_1b8 = (undefined1 (*) [16])
                                  CONCAT44(fVar48 + __xmm_00000000000000003f80000000000000._4_4_,
                                           fVar39 + (float)__xmm_00000000000000003f80000000000000);
                      uStack_1b0 = (undefined1 (*) [16])CONCAT44(uStack_1b0._4_4_,fVar41 + 0.0);
                      country_core::resources::roofs::roof_shape::Roof::
                      get_roof_shape_from_pos_ws_clamped();
                      country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle();
                      local_1a8 = (undefined1 *)extraout_RAX_07[2];
                      local_1b8 = (undefined1 (*) [16])*extraout_RAX_07;
                      uStack_1b0 = (undefined1 (*) [16])extraout_RAX_07[1];
                      ::utils::geometry::rectangle::Rectangle2d::closest_edge_normal();
                    }
                    ::utils::xz_rotation_bad();
                    fVar2 = fStack_124 * fVar52 * fVar46 +
                            (float)uStack_120 * fVar51 * ___xmm_bf800000bf8000003f8000003f800000 +
                            fVar50 * uStack_120._4_4_ * ___xmm_bf8000003f800000bf8000003f800000 +
                            fVar53 * local_128;
                    fVar13 = local_128 * fVar52 * fVar40 +
                             uStack_120._4_4_ * fVar51 * _UNK_1429cd184 +
                             fVar50 * (float)uStack_120 * _UNK_1429cd174 + fVar53 * fStack_124;
                    fVar14 = uStack_120._4_4_ * fVar52 * fVar45 +
                             local_128 * fVar51 * _UNK_1429cd188 +
                             fVar50 * fStack_124 * _UNK_1429cd178 + fVar53 * (float)uStack_120;
                    fVar53 = (float)uStack_120 * fVar52 * fVar54 +
                             fStack_124 * fVar51 * _UNK_1429cd18c +
                             fVar50 * local_128 * _UNK_1429cd17c + fVar53 * uStack_120._4_4_;
                  }
                }
LAB_140815eed:
                uStack_120._4_4_ = fVar53;
                uStack_120._0_4_ = fVar14;
                fStack_124 = fVar13;
                local_128 = fVar2;
                country_core::resources::roofs::roof_shape::Roof::is_flat();
                if ((extraout_AL_01 != '\0') && (pauVar25[uVar19 * -5 + -1][0xe] == '?')) {
                  country_core::resources::walls::decorator::DecoratorExtraInfo::as_stair();
                  fVar48 = fVar48 + *extraout_RAX_09;
                }
                goto LAB_140815f30;
              }
              goto LAB_140815c00;
            }
          }
          lVar30 = 1;
LAB_140817439:
          local_128 = (float)lVar30;
          fStack_124 = (float)((ulonglong)lVar30 >> 0x20);
          uStack_120._0_4_ = (float)extraout_RAX_02;
          uStack_120._4_4_ = (float)iVar43;
                    /* WARNING: Subroutine does not return */
          core::result::unwrap_failed();
        }
LAB_140815c00:
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransformStorage
        ::get();
        if (extraout_RAX_05 == (float *)0x0) {
          fVar39 = -100.0;
          fVar41 = -100.0;
          fVar48 = fVar47;
        }
        else {
          unaff_XMM11_Da = *extraout_RAX_05;
          unaff_XMM11_Db = extraout_RAX_05[1];
          unaff_XMM11_Dc = extraout_RAX_05[2];
          unaff_XMM11_Dd = extraout_RAX_05[3];
          fVar39 = extraout_RAX_05[4];
          fVar41 = extraout_RAX_05[6];
          local_278 = (undefined1 (*) [16])CONCAT44(local_278._4_4_,extraout_RAX_05[9]);
          local_280 = *(undefined1 (**) [16])(extraout_RAX_05 + 7);
          local_178 = *(ulonglong *)(extraout_RAX_05 + 10);
          fVar48 = extraout_RAX_05[5];
        }
        ::utils::components::transform::impl$1::default();
        local_1b8 = (undefined1 (*) [16])CONCAT44(uStack_108,fStack_10c);
        uStack_1b0 = (undefined1 (*) [16])CONCAT44(uStack_1b0._4_4_,uStack_104);
        puVar24 = &local_1b8;
        if (extraout_RAX_05 != (float *)0x0) {
          local_128 = unaff_XMM11_Da;
          fStack_124 = unaff_XMM11_Db;
          uStack_120._0_4_ = unaff_XMM11_Dc;
          uStack_120._4_4_ = unaff_XMM11_Dd;
          puVar24 = &local_280;
        }
        local_168 = (undefined1 (*) [16])CONCAT44(local_168._4_4_,*(undefined4 *)(puVar24 + 1));
        local_170 = (undefined1 *)*puVar24;
      }
      *local_1c8 = local_130;
      local_118 = (undefined8 *)CONCAT44(fVar48,fVar39);
      uStack_110 = SUB42(fVar41,0);
      uStack_10e = (undefined2)((uint)fVar41 >> 0x10);
      uStack_104 = local_168._0_4_;
      fStack_10c = SUB84(local_170,0);
      uStack_108 = (undefined4)((ulonglong)local_170 >> 0x20);
      uStack_100 = local_178;
      country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::insert
                ();
      uVar38 = uVar16;
    }
  }
  local_180 = (undefined4 *)local_228[2];
  uVar19 = local_178 >> 0x20;
  local_178 = CONCAT44((int)uVar19,*(undefined4 *)((longlong)local_228 + 0x1c));
  *local_180 = *(undefined4 *)((longlong)local_228 + 0x1c);
  country_core::resources::walls::decorator_storage::DecoratorStorage::get_mut();
  if (extraout_RAX_10 != 0) {
    local_1b8 = *(undefined1 (**) [16])(extraout_RAX_10 + 0x30);
    local_1a8 = *local_1b8 + *(longlong *)(extraout_RAX_10 + 0x38) + 1;
    auVar42 = *local_1b8;
    uStack_1a0._0_2_ =
         ~((ushort)(SUB161(auVar42 >> 7,0) & 1) | (ushort)(SUB161(auVar42 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar42 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar42 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar42 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar42 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar42 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar42 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar42 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar42 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar42 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar42 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar42 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar42 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar42 >> 0x77,0) & 1) << 0xe | (ushort)(byte)(auVar42[0xf] >> 7) << 0xf
          );
    uStack_190 = extraout_RAX_10 + 0x60;
    uStack_1b0 = local_1b8 + 1;
    uStack_198 = (undefined4)*(undefined8 *)(extraout_RAX_10 + 0x48);
    uStack_194 = (undefined4)((ulonglong)*(undefined8 *)(extraout_RAX_10 + 0x48) >> 0x20);
    lVar30 = *in_stack_00000028;
    local_f8 = (longlong *)*in_stack_00000040;
    while( true ) {
      country_core::resources::walls::decorator_storage::impl$3::next();
      if (local_278 == (undefined1 (*) [16])0x0) break;
      fVar44 = (float)local_280;
      local_208 = CONCAT62(uStack_266,local_268);
      local_218 = local_278;
      puStack_210 = puStack_270;
      minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
      if (*(char *)((longlong)extraout_RAX_11 + 0x22) != '\x03') {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic();
      }
      local_148 = *(float *)(extraout_RAX_11 + 1);
      local_150 = *extraout_RAX_11;
      uStack_120 = (undefined1 (*) [16])(*(longlong *)(lVar30 + 0x10) * 2);
      local_128 = (float)*(undefined8 *)(lVar30 + 8);
      fStack_124 = (float)((ulonglong)*(undefined8 *)(lVar30 + 8) >> 0x20);
      local_118 = *(undefined8 **)(lVar30 + 0x50);
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>()
      ;
      ::utils::swizzlers::impl$1::xvy();
      fVar49 = local_148 * __real_3f000000;
      fVar36 = sinf(fVar49);
      fVar37 = cosf(fVar49);
      fVar49 = __xmm_00000000000000000000000000000000;
      pauVar31 = (undefined1 (*) [16])((ulonglong)(uint)fVar37 << 0x20);
      bVar15 = *(char *)((longlong)extraout_RAX_11 + 0x46) - 0x38;
      if (7 < bVar15) {
        bVar15 = 6;
      }
      if (bVar15 == 6) {
        country_core::resources::clutter_meshes::clutter_mesh_ty_data();
        puVar24 = (undefined8 *)CONCAT44(fStack_124,local_128);
        local_118 = (undefined8 *)puVar24[2];
        uVar3 = puVar24[3];
        uStack_110 = (ushort)uVar3;
        uStack_10e = (undefined2)((ulonglong)uVar3 >> 0x10);
        fStack_10c = (float)((ulonglong)uVar3 >> 0x20);
        local_128 = (float)*puVar24;
        fStack_124 = (float)((ulonglong)*puVar24 >> 0x20);
        uStack_120._0_4_ = (float)puVar24[1];
        uStack_120._4_4_ = (float)((ulonglong)puVar24[1] >> 0x20);
        ::utils::geometry::rectangle::Rectangle3d::set_orientation_xz();
        local_168 = (undefined1 (*) [16])CONCAT44(local_168._4_4_,local_138._0_4_);
        local_170 = (undefined1 *)local_140;
        ::utils::geometry::rectangle::Rectangle3d::translate();
        local_170 = *(undefined1 **)(lVar30 + 8);
        local_168 = (undefined1 (*) [16])(*(longlong *)(lVar30 + 0x10) * 2);
        puStack_160 = *(undefined1 **)(lVar30 + 0x50);
        ::utils::float_grid::impl$0::
        bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>
                  ();
        local_170 = *(undefined1 **)(lVar30 + 8);
        local_168 = (undefined1 (*) [16])(*(longlong *)(lVar30 + 0x10) * 2);
        puStack_160 = *(undefined1 **)(lVar30 + 0x50);
        ::utils::float_grid::impl$0::
        bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>
                  ();
        local_170 = *(undefined1 **)(lVar30 + 8);
        local_168 = (undefined1 (*) [16])(*(longlong *)(lVar30 + 0x10) * 2);
        puStack_160 = *(undefined1 **)(lVar30 + 0x50);
        ::utils::float_grid::impl$0::
        bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>
                  ();
        local_170 = *(undefined1 **)(lVar30 + 8);
        local_168 = (undefined1 (*) [16])(*(longlong *)(lVar30 + 0x10) * 2);
        puStack_160 = *(undefined1 **)(lVar30 + 0x50);
        ::utils::float_grid::impl$0::
        bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>
                  ();
        fVar44 = extraout_XMM0_Da_01;
        if (extraout_XMM0_Da_01 <= extraout_XMM0_Da_00) {
          fVar44 = extraout_XMM0_Da_00;
        }
        fVar37 = extraout_XMM0_Da_01;
        if (extraout_XMM0_Da_00 <= extraout_XMM0_Da_01) {
          fVar37 = extraout_XMM0_Da_00;
        }
        fVar47 = (float)(~-(uint)NAN(extraout_XMM0_Da_00) & (uint)fVar37 |
                        -(uint)NAN(extraout_XMM0_Da_00) & (uint)extraout_XMM0_Da_01);
        fVar37 = extraout_XMM0_Da_02;
        if (fVar47 <= extraout_XMM0_Da_02) {
          fVar37 = fVar47;
        }
        fVar47 = (float)(-(uint)NAN(fVar47) & (uint)extraout_XMM0_Da_02 |
                        ~-(uint)NAN(fVar47) & (uint)fVar37);
        fVar37 = extraout_XMM0_Da_03;
        if (fVar47 <= extraout_XMM0_Da_03) {
          fVar37 = fVar47;
        }
        fVar47 = (float)(-(uint)NAN(fVar47) & (uint)extraout_XMM0_Da_03 |
                        ~-(uint)NAN(fVar47) & (uint)fVar37);
        fVar37 = 0.0;
        if (0.0 <= fVar47 + fVar47) {
          fVar37 = fVar47 + fVar47;
        }
        fVar46 = __real_3f800000;
        if (fVar37 <= __real_3f800000) {
          fVar46 = fVar37;
        }
        fVar37 = __real_3f800000 - (__real_40400000 - (fVar46 + fVar46)) * fVar46 * fVar46;
        fVar47 = (extraout_XMM0_Da_00 + extraout_XMM0_Da_01 + extraout_XMM0_Da_02 +
                 extraout_XMM0_Da_03) * __real_3e800000 * fVar37 +
                 (__real_3f800000 - fVar37) * fVar47;
        fVar37 = extraout_XMM0_Da_02;
        if (extraout_XMM0_Da_02 <= fVar44) {
          fVar37 = fVar44;
        }
        fVar37 = (float)(-(uint)NAN(fVar44) & (uint)extraout_XMM0_Da_02 |
                        ~-(uint)NAN(fVar44) & (uint)fVar37);
        fVar44 = extraout_XMM0_Da_03;
        if (extraout_XMM0_Da_03 <= fVar37) {
          fVar44 = fVar37;
        }
        fVar37 = (fStack_10c - (float)CONCAT22(uStack_10e,uStack_110)) * __real_bf000000 +
                 (float)(-(uint)NAN(fVar37) & (uint)extraout_XMM0_Da_03 |
                        ~-(uint)NAN(fVar37) & (uint)fVar44);
        fVar44 = fVar47;
        if (fVar47 <= fVar37) {
          fVar44 = fVar37;
        }
        local_140 = (undefined8 *)
                    CONCAT44(~-(uint)NAN(fVar37) & (uint)fVar44 | -(uint)NAN(fVar37) & (uint)fVar47,
                             (undefined4)local_140);
      }
      else {
        pauVar31 = (undefined1 (*) [16])((ulonglong)(uint)fVar37 << 0x20);
        if (bVar15 == 7) {
          country_core::resources::walls::decorator::DecoratorExtraInfo::as_stair();
          local_140 = (undefined8 *)
                      CONCAT44(*extraout_RAX_12 + local_140._4_4_,(undefined4)local_140);
          local_128 = fVar44;
          indexmap::map::IndexMap::
          get<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::resources::walls::decorator::DecoratorId>
                    ();
          if (extraout_RAX_13 == 0) {
            pfVar18 = (float *)&DAT_00000004;
            pfVar12 = (float *)&DAT_00000004;
          }
          else {
            pfVar18 = *(float **)(extraout_RAX_13 + 8) + *(longlong *)(extraout_RAX_13 + 0x10) * 2;
            pfVar12 = *(float **)(extraout_RAX_13 + 8);
          }
          do {
            pfVar21 = pfVar12;
            pauVar31 = (undefined1 (*) [16])((ulonglong)(uint)fVar37 << 0x20);
            if (pfVar21 == pfVar18) goto LAB_1408166c0;
            pfVar12 = pfVar21 + 2;
          } while (*(char *)(pfVar21 + 1) != '\0');
          fStack_124 = *pfVar21;
          local_128 = fVar44;
          indexmap::map::IndexMap::
          get<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,country_core::resources::stairs::stairs_assembly_params::AdjustedStairEdge,std::hash::random::RandomState,tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>_>
                    ();
          if (extraout_RAX_14 == 0) {
                    /* WARNING: Subroutine does not return */
            core::panicking::panic();
          }
          ::utils::xz_rotation_from_forward();
          fVar49 = local_128;
          fVar36 = fStack_124;
          pauVar31 = uStack_120;
        }
      }
LAB_1408166c0:
      pcVar11 = local_138;
      puVar10 = (undefined1 *)local_140;
      country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransformStorage::
      get();
      if (extraout_RAX_15 == (undefined8 *)0x0) {
LAB_140816721:
        *local_1c8 = local_130;
        uStack_108 = 0x3f800000;
        uStack_104 = 0x3f800000;
        uStack_100 = uStack_2d0;
        local_118 = (undefined8 *)puVar10;
        uStack_110 = (ushort)pcVar11;
        uStack_10e = (undefined2)((ulonglong)pcVar11 >> 0x10);
        fStack_10c = 1.0;
        local_128 = fVar49;
        fStack_124 = fVar36;
        uStack_120 = pauVar31;
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::
        insert();
        country_core::resources::walls::decorator_storage::DecoratorMaybeMut::get_mut();
      }
      else {
        ::utils::components::transform::Transform::abs_diff_eq();
        if (extraout_AL_02 == '\0') goto LAB_140816721;
        *local_1c8 = local_130;
        local_118 = (undefined8 *)extraout_RAX_15[2];
        uVar3 = extraout_RAX_15[3];
        uStack_100 = extraout_RAX_15[5];
        uStack_108 = (undefined4)extraout_RAX_15[4];
        uStack_104 = (undefined4)((ulonglong)extraout_RAX_15[4] >> 0x20);
        uStack_110 = (ushort)uVar3;
        uStack_10e = (undefined2)((ulonglong)uVar3 >> 0x10);
        fStack_10c = (float)((ulonglong)uVar3 >> 0x20);
        local_128 = (float)*extraout_RAX_15;
        fStack_124 = (float)((ulonglong)*extraout_RAX_15 >> 0x20);
        uStack_120._0_4_ = (float)extraout_RAX_15[1];
        uStack_120._4_4_ = (float)((ulonglong)extraout_RAX_15[1] >> 0x20);
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::
        insert();
      }
    }
  }
  *local_180 = (undefined4)local_178;
  country_core::resources::walls::decorator_storage::DecoratorStorage::get_mut();
  fVar44 = __real_3f800000;
  if (extraout_RAX_16 != 0) {
    local_280 = *(undefined1 (**) [16])(extraout_RAX_16 + 0x30);
    puStack_270 = *local_280 + *(longlong *)(extraout_RAX_16 + 0x38) + 1;
    auVar42 = *local_280;
    local_268 = ~((ushort)(SUB161(auVar42 >> 7,0) & 1) | (ushort)(SUB161(auVar42 >> 0xf,0) & 1) << 1
                  | (ushort)(SUB161(auVar42 >> 0x17,0) & 1) << 2 |
                  (ushort)(SUB161(auVar42 >> 0x1f,0) & 1) << 3 |
                  (ushort)(SUB161(auVar42 >> 0x27,0) & 1) << 4 |
                  (ushort)(SUB161(auVar42 >> 0x2f,0) & 1) << 5 |
                  (ushort)(SUB161(auVar42 >> 0x37,0) & 1) << 6 |
                  (ushort)(SUB161(auVar42 >> 0x3f,0) & 1) << 7 |
                  (ushort)(SUB161(auVar42 >> 0x47,0) & 1) << 8 |
                  (ushort)(SUB161(auVar42 >> 0x4f,0) & 1) << 9 |
                  (ushort)(SUB161(auVar42 >> 0x57,0) & 1) << 10 |
                  (ushort)(SUB161(auVar42 >> 0x5f,0) & 1) << 0xb |
                  (ushort)(SUB161(auVar42 >> 0x67,0) & 1) << 0xc |
                  (ushort)(SUB161(auVar42 >> 0x6f,0) & 1) << 0xd |
                  (ushort)(SUB161(auVar42 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar42[0xf] >> 7) << 0xf);
    local_260 = *(undefined8 *)(extraout_RAX_16 + 0x48);
    local_258 = extraout_RAX_16 + 0x60;
    local_278 = local_280 + 1;
    local_180 = (undefined4 *)*in_stack_00000048;
    local_1f0 = local_238 + 0x198;
    while( true ) {
      puVar24 = &local_280;
      country_core::resources::walls::decorator_storage::impl$3::next();
      if (local_168 == (undefined1 (*) [16])0x0) break;
      local_f8 = (longlong *)CONCAT44(local_f8._4_4_,local_170._0_4_);
      local_208 = local_158;
      local_218 = local_168;
      puStack_210 = puStack_160;
      minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
      if (*(char *)((longlong)extraout_RAX_17 + 0x22) != '\x04') {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic();
      }
      fVar49 = *(float *)(extraout_RAX_17 + 1);
      fVar36 = *(float *)((longlong)extraout_RAX_17 + 0xc);
      lVar30 = *(longlong *)(local_238 + 8);
      lVar32 = 0;
      plVar23 = (longlong *)(lVar30 + -0xf8);
      do {
        plVar35 = plVar23;
        if (*(longlong *)(local_238 + 0x10) * -0x150 - lVar32 == 0) {
          local_1a8 = __xmm_0000000000000000c2c8000080000000;
          uStack_1a0 = -0.0;
          local_1b8 = (undefined1 (*) [16])
                      CONCAT44(__xmm_3f800000000000000000000000000000._4_4_,
                               (float)__xmm_3f800000000000000000000000000000);
          uStack_1b0 = (undefined1 (*) [16])CONCAT44(DAT_14293e0c8._4_4_,(float)DAT_14293e0c8);
          uStack_194 = 0;
          fStack_19c = 0.0;
          uStack_198 = 0;
          goto LAB_1408170ff;
        }
        auVar42._0_4_ = -(uint)((int)plVar35[0x44] == (int)*extraout_RAX_17);
        iVar43 = -(uint)((int)((ulonglong)plVar35[0x44] >> 0x20) ==
                        (int)((ulonglong)*extraout_RAX_17 >> 0x20));
        auVar42._4_4_ = auVar42._0_4_;
        auVar42._8_4_ = iVar43;
        auVar42._12_4_ = iVar43;
        uVar16 = movmskpd((int)puVar24,auVar42);
        puVar24 = (undefined8 *)(ulonglong)uVar16;
        lVar32 = lVar32 + -0x150;
        plVar23 = plVar35 + 0x2a;
      } while (uVar16 != 3);
      local_150 = CONCAT44(local_150._4_4_,fVar36);
      if ((fVar36 < __real_00000000) || (fVar44 < fVar36)) {
        local_140 = &local_150;
        local_138 = core::fmt::float::impl$5::fmt;
        local_128 = 97.107056;
        fStack_124 = 1.4013e-45;
        uStack_120._0_4_ = 1.4013e-45;
        uStack_120._4_4_ = 0.0;
        uStack_108 = 0;
        uStack_104 = 0;
        local_118 = &local_140;
        uStack_110 = 1;
        uStack_10e = 0;
        fStack_10c = 0.0;
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_fmt();
      }
      if (NAN(fVar36)) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic();
      }
      ::utils::curve::Curve::
      get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                ();
      uVar19 = plVar35[0x47];
      if (uVar19 < 2) {
        lVar30 = (lVar30 - lVar32) + -0x20;
        if (uVar19 == 0) goto LAB_140816a20;
LAB_140816998:
        lVar32 = uVar19 * 4;
        uVar22 = 0;
        uVar27 = 0;
        do {
          uVar26 = *(ushort *)(lVar30 + 2 + uVar22 * 4) + uVar27;
          if ((extraout_RAX_18 & 0xffffffff) < uVar26) break;
          uVar22 = uVar22 + 1;
          lVar32 = lVar32 + -4;
          uVar27 = uVar26;
        } while (lVar32 != 0);
      }
      else {
        lVar30 = plVar35[0x45];
        uVar19 = plVar35[0x46];
        if (uVar19 != 0) goto LAB_140816998;
LAB_140816a20:
        uVar22 = 0;
        uVar27 = 0;
      }
      uVar26 = extraout_RAX_18 & 0xffffffff;
      if (uVar19 - 1 < uVar22) {
        uVar22 = uVar19 - 1;
      }
      if (uVar19 <= uVar22) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_bounds_check();
      }
      if ((ulonglong)plVar35[0x2e] <= uVar26) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_bounds_check();
      }
      uVar19 = uVar26 + 1;
      if ((ulonglong)plVar35[0x2e] <= uVar19) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_bounds_check();
      }
      uVar3 = *(undefined8 *)(plVar35[0x2d] + uVar26 * 8);
      uVar4 = *(undefined8 *)(plVar35[0x2d] + 8 + uVar26 * 8);
      fVar36 = (float)uVar3;
      fVar37 = (float)((ulonglong)uVar3 >> 0x20);
      fVar36 = ((float)uVar4 - fVar36) * extraout_XMM0_Da_04 + fVar36;
      fVar37 = ((float)((ulonglong)uVar4 >> 0x20) - fVar37) * extraout_XMM0_Da_04 + fVar37;
      fVar47 = fVar44 / SQRT(fVar37 * fVar37 + fVar36 * fVar36);
      uVar16 = (uint)((int)fVar47 - 1U < 0x7fffff ||
                     (int)ABS(fVar47) - 0x800000U < 0x7f000000 && -1 < (int)fVar47);
      uVar38 = uVar16 << 0x1f;
      uVar16 = uVar16 << 0x1f;
      local_1f8 = CONCAT44(~uVar38 & (uint)__xmm_00000000000000008000000000000000 |
                           (int)uVar38 >> 0x1f & (uint)(fVar47 * fVar36),
                           ~uVar16 & __xmm_00000000000000008000000000000000._4_4_ |
                           (int)uVar16 >> 0x1f &
                           (uint)((float)((uint)fVar47 ^ ___xmm_80000000800000008000000080000000) *
                                 fVar37));
      if (SBORROW8(0,plVar35[0x32])) {
                    /* WARNING: Subroutine does not return */
        core::option::unwrap_failed();
      }
      uVar7 = plVar35[0x34];
      if (uVar7 <= uVar26) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_bounds_check();
      }
      if (uVar7 <= uVar19) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_bounds_check();
      }
      uVar33 = *(ushort *)(lVar30 + 2 + uVar22 * 4) + uVar27;
      lVar32 = plVar35[0x33];
      fVar36 = fVar44 - extraout_XMM0_Da_04;
      uVar3 = *(undefined8 *)(lVar32 + uVar26 * 0xc);
      uVar4 = *(undefined8 *)(lVar32 + uVar19 * 0xc);
      fVar47 = extraout_XMM0_Da_04 * (float)uVar4 + (float)uVar3 * fVar36;
      fVar37 = extraout_XMM0_Da_04 * (float)((ulonglong)uVar4 >> 0x20) +
               (float)((ulonglong)uVar3 >> 0x20) * fVar36;
      fVar36 = *(float *)(lVar32 + 8 + uVar19 * 0xc) * extraout_XMM0_Da_04 +
               fVar36 * *(float *)(lVar32 + 8 + uVar26 * 0xc);
      switch(*(undefined1 *)(lVar30 + uVar22 * 4)) {
      case 2:
        uVar22 = uVar33 + 1;
        if (uVar22 < uVar27) {
                    /* WARNING: Subroutine does not return */
          core::slice::index::slice_index_order_fail();
        }
        if ((ulonglong)plVar35[0x37] <= uVar33) {
                    /* WARNING: Subroutine does not return */
          core::slice::index::slice_end_index_len_fail();
        }
        lVar30 = plVar35[0x36];
        if (lVar30 + uVar22 * 4 == 4) {
          local_178 = uVar19;
                    /* WARNING: Subroutine does not return */
          core::option::unwrap_failed();
        }
        fVar46 = (*(float *)(lVar30 + uVar33 * 4) - *(float *)(lVar30 + uVar27 * 4)) *
                 *(float *)(plVar35 + 0x38);
        if (fVar46 <= __real_322bcc77) {
          fVar46 = __real_322bcc77;
        }
        fVar40 = __real_3f000000 / fVar46;
        if (fVar44 <= __real_3f000000 / fVar46) {
          fVar40 = fVar44;
        }
        local_178 = uVar19;
        fVar46 = (float)roundf(fVar44 / fVar40);
        uVar19 = 0;
        if (__real_00000000 <= fVar46) {
          uVar19 = (longlong)(fVar46 - __real_5f000000) & (longlong)fVar46 >> 0x3f |
                   (longlong)fVar46;
        }
        if (__real_5f7fffff < fVar46) {
          uVar19 = 0xffffffffffffffff;
        }
        if (uVar7 <= uVar33) {
                    /* WARNING: Subroutine does not return */
          core::slice::index::slice_end_index_len_fail();
        }
        lVar30 = lVar32 + uVar22 * 0xc;
        if (lVar30 == 0xc) {
                    /* WARNING: Subroutine does not return */
          core::option::unwrap_failed();
        }
        fVar46 = *(float *)(lVar32 + 4 + uVar27 * 0xc);
        fVar40 = *(float *)(lVar30 + -8);
        fVar45 = fVar40;
        if (fVar40 <= fVar46) {
          fVar45 = fVar46;
        }
        fVar54 = fVar40;
        if (fVar46 <= fVar40) {
          fVar54 = fVar46;
        }
        fVar46 = (float)(-(uint)NAN(fVar46) & (uint)fVar40 | ~-(uint)NAN(fVar46) & (uint)fVar54);
        fVar45 = (fVar45 - fVar46) / (float)(uVar19 + (uVar19 == 0));
        fVar40 = fVar45;
        if (fVar45 <= __real_3727c5ac) {
          fVar40 = __real_3727c5ac;
        }
        fVar37 = ceilf((fVar37 - fVar46) / fVar40);
        fVar37 = fVar37 * fVar45 + fVar46;
        uVar19 = local_178;
        break;
      case 3:
        if (uVar33 + 1 < uVar27) {
                    /* WARNING: Subroutine does not return */
          core::slice::index::slice_index_order_fail();
        }
        if (uVar7 <= uVar33) {
                    /* WARNING: Subroutine does not return */
          core::slice::index::slice_end_index_len_fail();
        }
        if ((uVar33 + 1) - uVar27 != 2) {
          local_128 = 0.0;
          fStack_124 = 0.0;
                    /* WARNING: Subroutine does not return */
          core::panicking::assert_failed<usize,usize>();
        }
        local_148 = *(float *)(lVar32 + 8 + uVar27 * 0xc);
        local_150 = *(undefined8 *)(lVar32 + uVar27 * 0xc);
        local_138 = (code *)CONCAT44(local_138._4_4_,*(undefined4 *)(lVar32 + 0x14 + uVar27 * 0xc));
        local_140 = *(undefined8 **)(lVar32 + 0xc + uVar27 * 0xc);
        local_178 = uVar19;
        decorator_visual::stairs_assemble::calculate_adjusted_ladder_positions();
        fVar40 = fStack_124;
        fVar45 = (float)local_118;
        fVar54 = (float)uStack_120 - local_118._4_4_;
        fVar46 = fVar54;
        if ((float)local_118 <= fStack_124) {
          fVar46 = local_118._4_4_ - (float)uStack_120;
        }
        fVar48 = local_128 - uStack_120._4_4_;
        fVar39 = (float)(~-(uint)(fStack_124 < (float)local_118) &
                         (uint)(uStack_120._4_4_ - local_128) |
                        -(uint)(fStack_124 < (float)local_118) & (uint)fVar48);
        fVar41 = fVar44 / SQRT(fVar46 * fVar46 + fVar39 * fVar39);
        if ((int)fVar41 - 1U < 0x7fffff ||
            (int)ABS(fVar41) - 0x800000U < 0x7f000000 && -1 < (int)fVar41) {
          fVar39 = fVar39 * fVar41;
          fVar46 = fVar46 * fVar41;
        }
        else {
          fVar46 = 0.0;
          fVar39 = 0.0;
        }
        local_150 = CONCAT44(fVar46,fVar39);
        ::utils::swizzlers::impl$0::x0y();
        fVar46 = fVar40 - fVar45;
        fVar46 = ceilf(SQRT(fVar54 * fVar54 + fVar46 * fVar46 + fVar48 * fVar48) / __real_3f266666);
        uVar19 = 0;
        if (__real_00000000 <= fVar46) {
          uVar19 = (longlong)(fVar46 - __real_5f000000) & (longlong)fVar46 >> 0x3f |
                   (longlong)fVar46;
        }
        if (__real_5f7fffff < fVar46) {
          uVar19 = 0xffffffffffffffff;
        }
        if (uVar19 < 4) {
          uVar19 = 3;
        }
        fVar45 = (fVar45 - fVar40) / (float)(uVar19 - 1);
        fVar46 = fVar45;
        if (fVar45 <= __real_3727c5ac) {
          fVar46 = __real_3727c5ac;
        }
        fVar37 = ceilf((fVar37 - fVar40) / fVar46);
        fVar47 = fVar47 + SUB84(local_140,0) * (float)__xmm_00000000000000003e4ccccd3e4ccccd;
        fVar37 = fVar37 * fVar45 + fVar40 +
                 (float)((ulonglong)local_140 >> 0x20) *
                 __xmm_00000000000000003e4ccccd3e4ccccd._4_4_;
        fVar36 = fVar36 + local_138._0_4_ * __real_3e4ccccd;
        uVar19 = local_178;
        break;
      default:
      }
      if ((ulonglong)plVar35[0x2b] <= uVar26) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_bounds_check();
      }
      if ((ulonglong)plVar35[0x2b] <= uVar19) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_bounds_check();
      }
      lVar30 = plVar35[0x2a];
      fVar46 = *(float *)(lVar30 + uVar26 * 4);
      fVar40 = *(float *)(lVar30 + 4 + uVar26 * 4);
      ::utils::swizzlers::impl$0::x0y();
      fVar54 = fStack_124;
      fVar45 = local_128;
      fVar49 = fVar49 * (extraout_XMM0_Da_04 * (fVar40 - fVar46) + fVar46) * __real_3f000000;
      fVar46 = (float)uStack_120 * fVar49;
      country_core::resources::walls::decorator::DecoratorRotationXZ::from_xz_forward_vector();
      country_core::resources::walls::decorator::DecoratorRotationXZ::offset();
      local_140 = (undefined8 *)CONCAT44(local_140._4_4_,extraout_XMM0_Da_05);
      country_core::resources::walls::decorator::DecoratorRotationXZ::to_quat();
      uStack_1a0 = fVar36 + fVar46;
      local_1b8 = (undefined1 (*) [16])CONCAT44(fStack_124,local_128);
      uStack_1b0 = uStack_120;
      local_1a8 = (undefined1 *)CONCAT44(fVar37 + fVar49 * fVar54,fVar47 + fVar49 * fVar45);
      fStack_19c = 1.0;
      uStack_198 = 0x3f800000;
      uStack_194 = 0x3f800000;
LAB_1408170ff:
      country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransformStorage::
      get();
      if (extraout_RAX_19 == (undefined8 *)0x0) {
LAB_140817138:
        *local_1c8 = local_130;
        uStack_108 = uStack_198;
        uStack_104 = uStack_194;
        uStack_100 = uStack_190;
        local_118 = (undefined8 *)local_1a8;
        uStack_110 = SUB42(uStack_1a0,0);
        uStack_10e = (undefined2)((uint)uStack_1a0 >> 0x10);
        fStack_10c = fStack_19c;
        local_128 = SUB84(local_1b8,0);
        fStack_124 = (float)((ulonglong)local_1b8 >> 0x20);
        uStack_120._0_4_ = SUB84(uStack_1b0,0);
        uStack_120._4_4_ = (float)((ulonglong)uStack_1b0 >> 0x20);
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::
        insert();
        country_core::resources::walls::decorator_storage::DecoratorMaybeMut::get_mut();
      }
      else {
        ::utils::components::transform::Transform::abs_diff_eq();
        if (extraout_AL_03 == '\0') goto LAB_140817138;
        *local_1c8 = local_130;
        local_118 = (undefined8 *)extraout_RAX_19[2];
        uVar3 = extraout_RAX_19[3];
        uStack_100 = extraout_RAX_19[5];
        uStack_108 = (undefined4)extraout_RAX_19[4];
        uStack_104 = (undefined4)((ulonglong)extraout_RAX_19[4] >> 0x20);
        uStack_110 = (ushort)uVar3;
        uStack_10e = (undefined2)((ulonglong)uVar3 >> 0x10);
        fStack_10c = (float)((ulonglong)uVar3 >> 0x20);
        local_128 = (float)*extraout_RAX_19;
        fStack_124 = (float)((ulonglong)*extraout_RAX_19 >> 0x20);
        uStack_120._0_4_ = (float)extraout_RAX_19[1];
        uStack_120._4_4_ = (float)((ulonglong)extraout_RAX_19[1] >> 0x20);
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::
        insert();
      }
    }
  }
  if (local_129 != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_20 == 1) {
      plVar23 = extraout_RAX_20 + 1;
    }
    else {
      if (*extraout_RAX_20 != 0) {
LAB_140817284:
                    /* WARNING: Subroutine does not return */
        std::thread::local::panic_access_error();
      }
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      plVar23 = extraout_RAX_22;
      if (extraout_RAX_22 == (longlong *)0x0) goto LAB_140817284;
    }
    if (*plVar23 != 0) {
LAB_140817417:
                    /* WARNING: Subroutine does not return */
      core::cell::panic_already_borrowed();
    }
    *plVar23 = -1;
    local_f8 = plVar23;
    puffin::ThreadProfiler::end_scope();
    *local_f8 = *local_f8 + 1;
  }
  return;
}

