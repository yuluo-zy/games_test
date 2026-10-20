// name: resolve_hole_overlap
// addr: 0x1412411c0
// size: 869
// signature: void __cdecl resolve_hole_overlap(void)


void __cdecl system_wall_constructor::utils::resolve_hole_overlap::resolve_hole_overlap(void)

{
  undefined8 *puVar1;
  float *pfVar2;
  undefined8 *puVar3;
  undefined4 *puVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined1 auVar11 [16];
  undefined1 auVar12 [16];
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  undefined8 uVar19;
  undefined8 uVar20;
  undefined8 uVar21;
  bool bVar22;
  char extraout_AL;
  byte extraout_AL_00;
  uint extraout_EAX;
  uint extraout_EAX_00;
  longlong lVar23;
  float *in_RCX;
  ulonglong uVar24;
  ulonglong *in_RDX;
  ulonglong uVar25;
  longlong lVar26;
  ulonglong uVar27;
  uint uVar28;
  ulonglong uVar29;
  ulonglong uVar30;
  uint uVar31;
  byte bVar32;
  ulonglong uVar33;
  int iVar34;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  float fVar35;
  undefined8 uStack_1f8;
  undefined8 uStack_1f0;
  undefined4 uStack_1e8;
  undefined4 uStack_1e4;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  float local_1b8;
  float fStack_1b4;
  float fStack_1b0;
  float fStack_1ac;
  uint local_198;
  char cStack_194;
  char cStack_193;
  float local_188;
  float fStack_184;
  undefined8 uStack_168;
  undefined8 uStack_160;
  float local_148;
  float fStack_144;
  float fStack_128;
  float fStack_124;
  float fStack_120;
  float fStack_11c;
  float fStack_118;
  float fStack_114;
  float fStack_110;
  float fStack_10c;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  
  ::utils::geometry::aabb::Aabb2::width();
  if (((0.0 < extraout_XMM0_Da) &&
      (::utils::geometry::aabb::Aabb2::height(), 0.0 < extraout_XMM0_Da_00)) && (in_RDX[3] != 0)) {
    fVar5 = in_RCX[2];
    fVar6 = in_RCX[3];
    fVar7 = *in_RCX;
    fVar8 = in_RCX[1];
    uVar33 = 0;
    do {
      uVar25 = in_RDX[1];
      uVar24 = 0;
      if (*in_RDX <= in_RDX[2] + uVar33) {
        uVar24 = *in_RDX;
      }
      lVar23 = (in_RDX[2] + uVar33) - uVar24;
      puVar1 = (undefined8 *)(uVar25 + lVar23 * 0x28);
      uVar10 = *puVar1;
      uVar17 = puVar1[1];
      pfVar2 = (float *)(uVar25 + 0x10 + lVar23 * 0x28);
      fVar13 = *pfVar2;
      fVar14 = pfVar2[1];
      uVar18 = *(undefined8 *)pfVar2;
      fVar15 = pfVar2[2];
      fVar16 = pfVar2[3];
      uVar19 = *(undefined8 *)(pfVar2 + 2);
      uVar9 = *(undefined8 *)(uVar25 + 0x20 + lVar23 * 0x28);
      cStack_194 = (char)((ulonglong)uVar9 >> 0x20);
      cStack_193 = (char)((ulonglong)uVar9 >> 0x28);
      ::utils::geometry::aabb::Aabb2::center();
      fStack_1b0 = (float)uVar17;
      fStack_1ac = (float)((ulonglong)uVar17 >> 0x20);
      local_1b8 = (float)uVar10;
      fStack_1b4 = (float)((ulonglong)uVar10 >> 0x20);
      ::utils::geometry::aabb::Aabb2::center();
      ::utils::geometry::aabb::Aabb2::contains();
      ::utils::geometry::aabb::Aabb2::contains();
      uVar28 = (extraout_EAX & 0xff) + 1;
      if (extraout_AL == '\0') {
        uVar28 = extraout_EAX & 0xff;
      }
      ::utils::geometry::aabb::Aabb2::contains();
      uVar31 = extraout_EAX_00 & 0xff;
      ::utils::geometry::aabb::Aabb2::contains();
      bVar32 = (byte)extraout_EAX_00;
      local_188 = (float)uVar19;
      fStack_184 = (float)((ulonglong)uVar19 >> 0x20);
      local_148 = (float)uVar18;
      fStack_144 = (float)((ulonglong)uVar18 >> 0x20);
      local_198 = (uint)uVar9;
      switch(uVar28 + uVar31 + (uint)extraout_AL_00) {
      case 0:
        if (cStack_193 == '\0') {
          if ((fStack_1b0 <= fVar5) || (fVar7 <= local_1b8)) {
            if ((fStack_1ac <= fVar6) || ((fVar8 <= fStack_1b4 || (cStack_194 != '\0'))))
            goto code_r0x0001412414dd;
            goto code_r0x000141241d56;
          }
          ::utils::geometry::aabb::Aabb2::width();
          ::utils::geometry::aabb::Aabb2::center();
          ::utils::geometry::aabb::Aabb2::height();
          ::utils::geometry::aabb::Aabb2::from_center_dims();
          ::utils::geometry::aabb::Aabb2::center();
          ::utils::geometry::aabb::Aabb2::height();
          ::utils::geometry::aabb::Aabb2::from_center_dims();
          if (in_RDX[3] <= uVar33) {
                    // WARNING: Subroutine does not return
            core::option::expect_failed();
          }
          auVar11._4_4_ = fStack_184 * 0.5;
          auVar11._0_4_ = local_188 * 0.5;
          uVar25 = in_RDX[1];
          uVar24 = 0;
          if (*in_RDX <= in_RDX[2] + uVar33) {
            uVar24 = *in_RDX;
          }
          lVar23 = (in_RDX[2] + uVar33) - uVar24;
          puVar1 = (undefined8 *)(uVar25 + lVar23 * 0x28);
          *puVar1 = uStack_1f8;
          puVar1[1] = uStack_1f0;
          auVar11._8_8_ = 0;
          *(undefined1 (*) [16])(uVar25 + 0x10 + lVar23 * 0x28) = auVar11 << 0x40;
          *(uint *)(uVar25 + 0x20 + lVar23 * 0x28) = local_198;
          *(char *)(uVar25 + 0x24 + lVar23 * 0x28) = cStack_194;
          *(undefined1 *)(uVar25 + 0x25 + lVar23 * 0x28) = 1;
          uVar25 = *in_RDX;
          uVar24 = in_RDX[3];
          if (uVar24 == uVar25) {
            alloc::collections::vec_deque::VecDeque::
            grow<system_wall_constructor::utils::MyBrick,alloc::alloc::Global>();
            uVar25 = *in_RDX;
            uVar24 = in_RDX[3];
          }
          uVar29 = 0;
          if (uVar25 <= uVar24 + in_RDX[2]) {
            uVar29 = uVar25;
          }
          lVar23 = (uVar24 + in_RDX[2]) - uVar29;
          uVar25 = in_RDX[1];
          puVar1 = (undefined8 *)(uVar25 + lVar23 * 0x28);
          *puVar1 = uStack_108;
          puVar1[1] = uStack_100;
          *(undefined1 (*) [16])(uVar25 + 0x10 + lVar23 * 0x28) = auVar11 << 0x40;
          *(uint *)(uVar25 + 0x20 + lVar23 * 0x28) = local_198 + 100;
          *(char *)(uVar25 + 0x24 + lVar23 * 0x28) = cStack_194;
          *(undefined1 *)(uVar25 + 0x25 + lVar23 * 0x28) = 1;
          in_RDX[3] = in_RDX[3] + 1;
        }
        else if (((cStack_194 != '\0') || (fStack_1ac <= fVar6)) || (fVar8 <= fStack_1b4)) {
code_r0x0001412414dd:
          uVar33 = uVar33 + 1;
        }
        else {
code_r0x000141241d56:
          ::utils::geometry::aabb::Aabb2::height();
          ::utils::geometry::aabb::Aabb2::center();
          ::utils::geometry::aabb::Aabb2::width();
          ::utils::geometry::aabb::Aabb2::from_center_dims();
          ::utils::geometry::aabb::Aabb2::center();
          ::utils::geometry::aabb::Aabb2::width();
          ::utils::geometry::aabb::Aabb2::from_center_dims();
          if (in_RDX[3] <= uVar33) {
                    // WARNING: Subroutine does not return
            core::option::expect_failed();
          }
          auVar12._4_4_ = fStack_184 * 0.5;
          auVar12._0_4_ = local_188 * 0.5;
          uVar25 = in_RDX[1];
          uVar24 = 0;
          if (*in_RDX <= in_RDX[2] + uVar33) {
            uVar24 = *in_RDX;
          }
          lVar23 = (in_RDX[2] + uVar33) - uVar24;
          puVar1 = (undefined8 *)(uVar25 + lVar23 * 0x28);
          *puVar1 = uStack_1f8;
          puVar1[1] = uStack_1f0;
          auVar12._8_8_ = 0;
          *(undefined1 (*) [16])(uVar25 + 0x10 + lVar23 * 0x28) = auVar12 << 0x40;
          *(uint *)(uVar25 + 0x20 + lVar23 * 0x28) = local_198;
          *(undefined1 *)(uVar25 + 0x24 + lVar23 * 0x28) = 1;
          *(char *)(uVar25 + 0x25 + lVar23 * 0x28) = cStack_193;
          uVar25 = *in_RDX;
          uVar24 = in_RDX[3];
          if (uVar24 == uVar25) {
            alloc::collections::vec_deque::VecDeque::
            grow<system_wall_constructor::utils::MyBrick,alloc::alloc::Global>();
            uVar25 = *in_RDX;
            uVar24 = in_RDX[3];
          }
          uVar29 = 0;
          if (uVar25 <= uVar24 + in_RDX[2]) {
            uVar29 = uVar25;
          }
          lVar23 = (uVar24 + in_RDX[2]) - uVar29;
          uVar25 = in_RDX[1];
          puVar1 = (undefined8 *)(uVar25 + lVar23 * 0x28);
          *puVar1 = uStack_f8;
          puVar1[1] = uStack_f0;
          *(undefined1 (*) [16])(uVar25 + 0x10 + lVar23 * 0x28) = auVar12 << 0x40;
          *(uint *)(uVar25 + 0x20 + lVar23 * 0x28) = local_198 + 100;
          *(undefined1 *)(uVar25 + 0x24 + lVar23 * 0x28) = 1;
          *(char *)(uVar25 + 0x25 + lVar23 * 0x28) = cStack_193;
          in_RDX[3] = in_RDX[3] + 1;
        }
        break;
      case 1:
        if ((char)extraout_EAX == '\0') {
          if (extraout_AL == '\0') {
            if (bVar32 == 0) {
              if (extraout_AL_00 == 0) goto code_r0x000141242480;
              ::utils::inverse_lerp();
              fVar35 = 0.0;
              if (0.0 <= extraout_XMM0_Da_04) {
                fVar35 = extraout_XMM0_Da_04;
              }
              fStack_118 = 1.0;
              if (fVar35 <= 1.0) {
                fStack_118 = fVar35;
              }
              fStack_11c = 1.0 - fStack_118;
              fStack_128 = fStack_11c * fVar13;
              fStack_124 = fStack_11c * fVar14;
              fStack_120 = fStack_11c * fVar15;
              fStack_11c = fStack_11c * fVar16;
              fStack_110 = 0.0;
              fStack_10c = 0.0;
              iVar34 = 1;
              bVar22 = false;
            }
            else {
              if (extraout_AL_00 != 0) {
code_r0x000141242480:
                    // WARNING: Subroutine does not return
                core::panicking::panic();
              }
              ::utils::inverse_lerp();
              fVar35 = 0.0;
              if (0.0 <= extraout_XMM0_Da_03) {
                fVar35 = extraout_XMM0_Da_03;
              }
              fStack_118 = 1.0;
              if (fVar35 <= 1.0) {
                fStack_118 = fVar35;
              }
              fStack_128 = fStack_118 * local_148;
              fStack_124 = fStack_118 * fStack_144;
              fStack_118 = 1.0 - fStack_118;
              fStack_110 = fStack_118 * fVar15;
              fStack_10c = fStack_118 * fVar16;
              fStack_120 = 0.0;
              fStack_11c = 0.0;
              iVar34 = 0;
              bVar22 = true;
            }
            fStack_114 = fStack_118 * fVar14;
            fStack_118 = fStack_118 * fVar13;
          }
          else {
            if (extraout_AL_00 != 0 || uVar31 != 0) goto code_r0x000141242480;
            ::utils::inverse_lerp();
            fVar35 = 0.0;
            if (0.0 <= extraout_XMM0_Da_02) {
              fVar35 = extraout_XMM0_Da_02;
            }
            fStack_10c = 1.0;
            if (fVar35 <= 1.0) {
              fStack_10c = fVar35;
            }
            fStack_11c = 1.0 - fStack_10c;
            fStack_128 = fStack_11c * fVar13;
            fStack_124 = fStack_11c * fVar14;
            fStack_120 = fStack_11c * fVar15;
            fStack_11c = fStack_11c * fVar16;
            fStack_110 = local_188 * fStack_10c;
            fStack_10c = fStack_184 * fStack_10c;
            fStack_118 = 0.0;
            fStack_114 = 0.0;
            iVar34 = 1;
            bVar22 = false;
          }
        }
        else {
          if ((extraout_AL != '\0' || bVar32 != 0) || extraout_AL_00 != 0)
          goto code_r0x000141242480;
          ::utils::inverse_lerp();
          fVar35 = 0.0;
          if (0.0 <= extraout_XMM0_Da_01) {
            fVar35 = extraout_XMM0_Da_01;
          }
          fStack_10c = 1.0;
          if (fVar35 <= 1.0) {
            fStack_10c = fVar35;
          }
          fStack_120 = local_188 * fStack_10c;
          fStack_11c = fStack_184 * fStack_10c;
          fStack_10c = 1.0 - fStack_10c;
          fStack_118 = fStack_10c * fVar13;
          fStack_114 = fStack_10c * fVar14;
          fStack_110 = fStack_10c * fVar15;
          fStack_10c = fStack_10c * fVar16;
          fStack_128 = 0.0;
          fStack_124 = 0.0;
          iVar34 = 0;
          bVar22 = true;
        }
        ::utils::geometry::aabb::Aabb2::width();
        ::utils::geometry::aabb::Aabb2::width();
        uVar25 = (ulonglong)(uint)(iVar34 << 4);
        if (bVar22) {
          ::utils::geometry::aabb::Aabb2::from_center_dims();
          uVar9 = *(undefined8 *)((longlong)&fStack_128 + uVar25);
          uVar10 = *(undefined8 *)((longlong)&fStack_120 + uVar25);
          uVar25 = *in_RDX;
          uVar24 = in_RDX[3];
          if (uVar24 == uVar25) {
            alloc::collections::vec_deque::VecDeque::
            grow<system_wall_constructor::utils::MyBrick,alloc::alloc::Global>();
            uVar25 = *in_RDX;
            uVar24 = in_RDX[3];
          }
          uVar27 = in_RDX[2] - 1;
          uVar29 = uVar25 + uVar27;
          if (!CARRY8(uVar25,uVar27)) {
            uVar29 = uVar27;
          }
          uVar25 = in_RDX[1];
          in_RDX[2] = uVar29;
          in_RDX[3] = uVar24 + 1;
          uStack_1e8 = (undefined4)uVar9;
          uStack_1e4 = (undefined4)((ulonglong)uVar9 >> 0x20);
          uStack_1e0 = (undefined4)uVar10;
          uStack_1dc = (undefined4)((ulonglong)uVar10 >> 0x20);
          puVar1 = (undefined8 *)(uVar25 + uVar29 * 0x28);
          *puVar1 = uStack_168;
          puVar1[1] = uStack_160;
          puVar4 = (undefined4 *)(uVar25 + 0x10 + uVar29 * 0x28);
          *puVar4 = uStack_1e8;
          puVar4[1] = uStack_1e4;
          puVar4[2] = uStack_1e0;
          puVar4[3] = uStack_1dc;
          *(uint *)(uVar25 + 0x20 + uVar29 * 0x28) = (iVar34 + local_198) % 10;
          *(char *)(uVar25 + 0x24 + uVar29 * 0x28) = cStack_194;
          *(char *)(uVar25 + 0x25 + uVar29 * 0x28) = cStack_193;
          ::utils::geometry::aabb::Aabb2::center();
          ::utils::geometry::aabb::Aabb2::height();
          ::utils::geometry::aabb::Aabb2::from_center_dims();
          uVar9 = CONCAT44(fStack_114,fStack_118);
          uVar10 = CONCAT44(fStack_10c,fStack_110);
          uVar25 = *in_RDX;
          uVar24 = in_RDX[3];
          if (uVar24 == uVar25) {
            alloc::collections::vec_deque::VecDeque::
            grow<system_wall_constructor::utils::MyBrick,alloc::alloc::Global>();
            uVar25 = *in_RDX;
            uVar24 = in_RDX[3];
          }
          uVar27 = in_RDX[2] - 1;
          uVar29 = uVar25 + uVar27;
          if (!CARRY8(uVar25,uVar27)) {
            uVar29 = uVar27;
          }
          uVar25 = in_RDX[1];
          in_RDX[2] = uVar29;
          in_RDX[3] = uVar24 + 1;
          puVar1 = (undefined8 *)(uVar25 + uVar29 * 0x28);
          *puVar1 = uStack_168;
          puVar1[1] = uStack_160;
          puVar1 = (undefined8 *)(uVar25 + 0x10 + uVar29 * 0x28);
          *puVar1 = uVar9;
          puVar1[1] = uVar10;
          *(uint *)(uVar25 + 0x20 + uVar29 * 0x28) = ((local_198 + 1) / 10) * -10 + local_198 + 1;
          *(char *)(uVar25 + 0x24 + uVar29 * 0x28) = cStack_194;
          *(char *)(uVar25 + 0x25 + uVar29 * 0x28) = cStack_193;
          uVar25 = in_RDX[3];
        }
        else {
          ::utils::geometry::aabb::Aabb2::center();
          ::utils::geometry::aabb::Aabb2::height();
          ::utils::geometry::aabb::Aabb2::from_center_dims();
          uVar9 = CONCAT44(fStack_124,fStack_128);
          uVar10 = CONCAT44(fStack_11c,fStack_120);
          uVar24 = *in_RDX;
          uVar29 = in_RDX[3];
          if (uVar29 == uVar24) {
            alloc::collections::vec_deque::VecDeque::
            grow<system_wall_constructor::utils::MyBrick,alloc::alloc::Global>();
            uVar24 = *in_RDX;
            uVar29 = in_RDX[3];
          }
          uVar30 = in_RDX[2] - 1;
          uVar27 = uVar24 + uVar30;
          if (!CARRY8(uVar24,uVar30)) {
            uVar27 = uVar30;
          }
          uVar24 = in_RDX[1];
          in_RDX[2] = uVar27;
          in_RDX[3] = uVar29 + 1;
          puVar1 = (undefined8 *)(uVar24 + uVar27 * 0x28);
          *puVar1 = uStack_168;
          puVar1[1] = uStack_160;
          puVar1 = (undefined8 *)(uVar24 + 0x10 + uVar27 * 0x28);
          *puVar1 = uVar9;
          puVar1[1] = uVar10;
          *(uint *)(uVar24 + 0x20 + uVar27 * 0x28) = local_198 % 10;
          *(char *)(uVar24 + 0x24 + uVar27 * 0x28) = cStack_194;
          *(char *)(uVar24 + 0x25 + uVar27 * 0x28) = cStack_193;
          ::utils::geometry::aabb::Aabb2::from_center_dims();
          uVar9 = *(undefined8 *)((longlong)&fStack_128 + uVar25);
          uVar10 = *(undefined8 *)((longlong)&fStack_120 + uVar25);
          uVar25 = *in_RDX;
          uVar24 = in_RDX[3];
          if (uVar24 == uVar25) {
            alloc::collections::vec_deque::VecDeque::
            grow<system_wall_constructor::utils::MyBrick,alloc::alloc::Global>();
            uVar25 = *in_RDX;
            uVar24 = in_RDX[3];
          }
          uVar27 = in_RDX[2] - 1;
          uVar29 = uVar25 + uVar27;
          if (!CARRY8(uVar25,uVar27)) {
            uVar29 = uVar27;
          }
          uVar25 = in_RDX[1];
          in_RDX[2] = uVar29;
          in_RDX[3] = uVar24 + 1;
          puVar1 = (undefined8 *)(uVar25 + uVar29 * 0x28);
          *puVar1 = uStack_168;
          puVar1[1] = uStack_160;
          puVar1 = (undefined8 *)(uVar25 + 0x10 + uVar29 * 0x28);
          *puVar1 = uVar9;
          puVar1[1] = uVar10;
          *(uint *)(uVar25 + 0x20 + uVar29 * 0x28) = (iVar34 + local_198) % 10;
          *(char *)(uVar25 + 0x24 + uVar29 * 0x28) = cStack_194;
          *(char *)(uVar25 + 0x25 + uVar29 * 0x28) = cStack_193;
          uVar25 = in_RDX[3];
        }
        if (uVar25 != 0) {
          uVar29 = uVar33 + 2;
          uVar24 = uVar25 - 1;
          if (uVar29 < uVar24) {
            uVar33 = in_RDX[2] + uVar29;
            uVar25 = *in_RDX;
            uVar27 = in_RDX[1];
            uVar30 = uVar25;
            if (uVar33 < uVar25) {
              uVar30 = 0;
            }
            lVar23 = uVar33 - uVar30;
            uVar33 = in_RDX[2] + uVar24;
            if (uVar33 < uVar25) {
              uVar25 = 0;
            }
            lVar26 = uVar33 - uVar25;
            uVar9 = *(undefined8 *)(uVar27 + 0x20 + lVar23 * 0x28);
            puVar1 = (undefined8 *)(uVar27 + lVar23 * 0x28);
            uStack_1f8 = *puVar1;
            uStack_1f0 = puVar1[1];
            puVar1 = (undefined8 *)(uVar27 + 0x10 + lVar23 * 0x28);
            uVar17 = *puVar1;
            uVar18 = puVar1[1];
            uVar10 = *(undefined8 *)(uVar27 + 0x20 + lVar26 * 0x28);
            puVar1 = (undefined8 *)(uVar27 + lVar26 * 0x28);
            uVar19 = puVar1[1];
            puVar3 = (undefined8 *)(uVar27 + 0x10 + lVar26 * 0x28);
            uVar20 = *puVar3;
            uVar21 = puVar3[1];
            puVar3 = (undefined8 *)(uVar27 + lVar23 * 0x28);
            *puVar3 = *puVar1;
            puVar3[1] = uVar19;
            puVar1 = (undefined8 *)(uVar27 + 0x10 + lVar23 * 0x28);
            *puVar1 = uVar20;
            puVar1[1] = uVar21;
            *(undefined8 *)(uVar27 + 0x20 + lVar23 * 0x28) = uVar10;
            *(undefined8 *)(uVar27 + 0x20 + lVar26 * 0x28) = uVar9;
            puVar1 = (undefined8 *)(uVar27 + 0x10 + lVar26 * 0x28);
            *puVar1 = uVar17;
            puVar1[1] = uVar18;
            puVar1 = (undefined8 *)(uVar27 + lVar26 * 0x28);
            *puVar1 = uStack_1f8;
            puVar1[1] = uStack_1f0;
            uVar33 = uVar29;
            goto code_r0x0001412412e3;
          }
        }
        uStack_1f0 = uStack_160;
        uStack_1f8 = uStack_168;
        uVar33 = uVar33 + 2;
        if (uVar25 <= uVar33) break;
code_r0x0001412412dd:
        uVar24 = uVar25 - 1;
        goto code_r0x0001412412e3;
      case 2:
        if ((char)extraout_EAX == '\0') {
          if (extraout_AL != '\0') {
            if ((extraout_AL_00 & (bVar32 ^ 1)) == 0) goto code_r0x000141242498;
            fStack_144 = 0.0;
            fStack_184 = 0.0;
            goto code_r0x000141241a6d;
          }
          if ((bVar32 & extraout_AL_00) == 0) goto code_r0x000141242498;
          local_188 = 0.0;
          fStack_184 = 0.0;
        }
        else {
          if (extraout_AL == '\0') {
            if ((bVar32 & (extraout_AL_00 ^ 1)) == 0) goto code_r0x000141242498;
            local_148 = 0.0;
            local_188 = 0.0;
code_r0x000141241a6d:
            ::utils::geometry::aabb::Aabb2::center();
            ::utils::geometry::aabb::Aabb2::height();
            ::utils::geometry::aabb::Aabb2::from_center_dims();
            if (in_RDX[3] <= uVar33) {
                    // WARNING: Subroutine does not return
              core::option::expect_failed();
            }
            uVar25 = in_RDX[1];
            uVar24 = 0;
            if (*in_RDX <= in_RDX[2] + uVar33) {
              uVar24 = *in_RDX;
            }
            lVar23 = (in_RDX[2] + uVar33) - uVar24;
            puVar1 = (undefined8 *)(uVar25 + lVar23 * 0x28);
            *puVar1 = uStack_1f8;
            puVar1[1] = uStack_1f0;
            *(float *)(uVar25 + 0x10 + lVar23 * 0x28) = local_148;
            *(float *)(uVar25 + 0x14 + lVar23 * 0x28) = fStack_144;
            *(float *)(uVar25 + 0x18 + lVar23 * 0x28) = local_188;
            *(float *)(uVar25 + 0x1c + lVar23 * 0x28) = fStack_184;
            *(uint *)(uVar25 + 0x20 + lVar23 * 0x28) = local_198 % 10;
            *(char *)(uVar25 + 0x24 + lVar23 * 0x28) = cStack_194;
            *(char *)(uVar25 + 0x25 + lVar23 * 0x28) = cStack_193;
            uVar33 = uVar33 + 1;
            break;
          }
          if (extraout_AL_00 != 0 || uVar31 != 0) {
code_r0x000141242498:
                    // WARNING: Subroutine does not return
            core::panicking::panic();
          }
          local_148 = 0.0;
          fStack_144 = 0.0;
        }
        ::utils::geometry::aabb::Aabb2::center();
        ::utils::geometry::aabb::Aabb2::width();
        ::utils::geometry::aabb::Aabb2::from_center_dims();
        if (in_RDX[3] <= uVar33) {
                    // WARNING: Subroutine does not return
          core::option::expect_failed();
        }
        uVar25 = in_RDX[1];
        uVar24 = 0;
        if (*in_RDX <= in_RDX[2] + uVar33) {
          uVar24 = *in_RDX;
        }
        lVar23 = (in_RDX[2] + uVar33) - uVar24;
        puVar1 = (undefined8 *)(uVar25 + lVar23 * 0x28);
        *puVar1 = uStack_1f8;
        puVar1[1] = uStack_1f0;
        *(float *)(uVar25 + 0x10 + lVar23 * 0x28) = local_148;
        *(float *)(uVar25 + 0x14 + lVar23 * 0x28) = fStack_144;
        *(float *)(uVar25 + 0x18 + lVar23 * 0x28) = local_188;
        *(float *)(uVar25 + 0x1c + lVar23 * 0x28) = fStack_184;
        *(uint *)(uVar25 + 0x20 + lVar23 * 0x28) = local_198 % 10;
        *(char *)(uVar25 + 0x24 + lVar23 * 0x28) = cStack_194;
        *(char *)(uVar25 + 0x25 + lVar23 * 0x28) = cStack_193;
        uVar33 = uVar33 + 1;
        break;
      default:
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      case 4:
        uVar25 = in_RDX[3];
        if ((uVar25 == 0) || (uVar24 = uVar25 - 1, uVar24 <= uVar33)) {
          if (uVar25 <= uVar33) break;
          goto code_r0x0001412412dd;
        }
        uVar25 = in_RDX[2] + uVar33;
        uVar29 = *in_RDX;
        uVar27 = in_RDX[1];
        uVar30 = uVar29;
        if (uVar25 < uVar29) {
          uVar30 = 0;
        }
        lVar23 = uVar25 - uVar30;
        uVar25 = in_RDX[2] + uVar24;
        if (uVar25 < uVar29) {
          uVar29 = 0;
        }
        lVar26 = uVar25 - uVar29;
        uVar9 = *(undefined8 *)(uVar27 + 0x20 + lVar23 * 0x28);
        puVar1 = (undefined8 *)(uVar27 + lVar23 * 0x28);
        uStack_1f8 = *puVar1;
        uStack_1f0 = puVar1[1];
        puVar1 = (undefined8 *)(uVar27 + 0x10 + lVar23 * 0x28);
        uVar17 = *puVar1;
        uVar18 = puVar1[1];
        uVar10 = *(undefined8 *)(uVar27 + 0x20 + lVar26 * 0x28);
        puVar1 = (undefined8 *)(uVar27 + lVar26 * 0x28);
        uVar19 = puVar1[1];
        puVar3 = (undefined8 *)(uVar27 + 0x10 + lVar26 * 0x28);
        uVar20 = *puVar3;
        uVar21 = puVar3[1];
        puVar3 = (undefined8 *)(uVar27 + lVar23 * 0x28);
        *puVar3 = *puVar1;
        puVar3[1] = uVar19;
        puVar1 = (undefined8 *)(uVar27 + 0x10 + lVar23 * 0x28);
        *puVar1 = uVar20;
        puVar1[1] = uVar21;
        *(undefined8 *)(uVar27 + 0x20 + lVar23 * 0x28) = uVar10;
        *(undefined8 *)(uVar27 + 0x20 + lVar26 * 0x28) = uVar9;
        puVar1 = (undefined8 *)(uVar27 + 0x10 + lVar26 * 0x28);
        *puVar1 = uVar17;
        puVar1[1] = uVar18;
        puVar1 = (undefined8 *)(uVar27 + lVar26 * 0x28);
        *puVar1 = uStack_1f8;
        puVar1[1] = uStack_1f0;
code_r0x0001412412e3:
        in_RDX[3] = uVar24;
      }
    } while (uVar33 < in_RDX[3]);
  }
  return;
}

