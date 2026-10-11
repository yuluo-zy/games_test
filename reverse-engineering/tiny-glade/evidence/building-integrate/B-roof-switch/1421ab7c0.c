
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl system_roof::edit_roof::edit_roof(void)

{
  undefined4 *puVar1;
  undefined8 *puVar2;
  uint *puVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  ulonglong *puVar9;
  longlong lVar10;
  ulonglong uVar11;
  longlong lVar12;
  longlong lVar13;
  undefined4 *puVar14;
  longlong lVar15;
  undefined4 *puVar16;
  longlong lVar17;
  undefined4 *puVar18;
  longlong lVar19;
  undefined4 *puVar20;
  longlong lVar21;
  float fVar22;
  float fVar23;
  undefined1 uVar24;
  byte extraout_AL;
  byte extraout_AL_00;
  byte extraout_AL_01;
  undefined1 uVar25;
  ulonglong uVar26;
  int *piVar27;
  ulonglong extraout_RAX;
  longlong lVar28;
  longlong lVar29;
  longlong *in_RDX;
  undefined1 uVar30;
  int *piVar31;
  byte bVar32;
  ulonglong uVar33;
  byte bVar34;
  ulonglong uVar35;
  int *piVar36;
  undefined8 uVar37;
  undefined8 *in_R8;
  int *piVar38;
  int *piVar39;
  longlong *in_R9;
  int *piVar40;
  undefined8 *puVar41;
  undefined8 uVar42;
  float fVar43;
  float fVar44;
  float fVar45;
  float fVar46;
  float fVar47;
  uint uVar48;
  uint uVar50;
  undefined1 auVar49 [16];
  float fVar51;
  undefined1 auVar52 [16];
  undefined1 auVar53 [16];
  undefined1 auVar54 [16];
  float fVar55;
  float fVar56;
  float fVar57;
  float fVar58;
  float fVar59;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  longlong *in_stack_00000038;
  undefined8 local_1e8;
  undefined8 uStack_1e0;
  uint local_1d8;
  uint uStack_1d4;
  uint local_1d0;
  
  puVar9 = (ulonglong *)*in_R8;
  lVar10 = in_R8[1];
  uVar11 = *puVar9;
  uVar33 = 0;
  uVar26 = uVar11 - *(ulonglong *)(lVar10 + 0x18);
  if (uVar11 < *(ulonglong *)(lVar10 + 0x18)) {
    uVar26 = uVar33;
  }
  uVar35 = uVar11 - *(ulonglong *)(lVar10 + 0x38);
  if (uVar11 < *(ulonglong *)(lVar10 + 0x38)) {
    uVar35 = uVar33;
  }
  piVar38 = (int *)(uVar26 * 0x40 + *(longlong *)(lVar10 + 8));
  uVar11 = *(ulonglong *)(lVar10 + 0x10) - uVar26;
  if (*(ulonglong *)(lVar10 + 0x10) < uVar26) {
    piVar38 = (int *)&DAT_00000008;
    uVar11 = uVar33;
  }
  piVar27 = (int *)(uVar35 * 0x40 + *(longlong *)(lVar10 + 0x28));
  uVar26 = *(ulonglong *)(lVar10 + 0x30) - uVar35;
  if (*(ulonglong *)(lVar10 + 0x30) < uVar35) {
    piVar27 = (int *)&DAT_00000008;
    uVar26 = uVar33;
  }
  *puVar9 = (*(longlong *)(lVar10 + 0x40) - uVar26) - uVar11;
  fVar23 = __real_3dcccccd;
  fVar22 = __real_3f800000;
  piVar40 = piVar27 + uVar26 * 0x10;
  lVar10 = *in_RDX;
  lVar12 = in_RDX[1];
  uVar4 = *(undefined4 *)((longlong)in_RDX + 0x14);
  uVar5 = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
  lVar13 = *in_stack_00000030;
  puVar14 = (undefined4 *)in_stack_00000030[2];
  uVar6 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
  lVar15 = *in_stack_00000038;
  puVar16 = (undefined4 *)in_stack_00000038[2];
  uVar7 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  lVar17 = *in_stack_00000028;
  puVar18 = (undefined4 *)in_stack_00000028[2];
  uVar8 = *(undefined4 *)((longlong)in_R9 + 0x1c);
  lVar19 = *in_R9;
  puVar20 = (undefined4 *)in_R9[2];
  piVar39 = piVar38;
LAB_1421ab9fc:
  if ((piVar39 == (int *)0x0) || (piVar39 == piVar38 + uVar11 * 0x10)) {
    if ((piVar27 == (int *)0x0) || (piVar27 == piVar40)) {
      return;
    }
    piVar36 = (int *)0x0;
    piVar31 = piVar27;
    piVar27 = piVar27 + 0x10;
  }
  else {
    piVar36 = piVar39 + 0x10;
    piVar31 = piVar39;
  }
  *puVar9 = *puVar9 + 1;
  country_core::resources::walls::public_walls::PublicWalls::roof_entity();
  if (extraout_RAX >> 0x20 == 0) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_fmt();
  }
  uVar26 = extraout_RAX & 0xffffffff;
  if ((uVar26 < *(ulonglong *)(lVar12 + 0x10)) &&
     (*(int *)(*(longlong *)(lVar12 + 8) + uVar26 * 0x14) == (int)(extraout_RAX >> 0x20))) {
    lVar28 = *(longlong *)(lVar12 + 8) + uVar26 * 0x14;
    uVar26 = (ulonglong)*(uint *)(lVar28 + 4);
    if ((uVar26 != 0xffffffff) &&
       ((uVar26 < *(ulonglong *)(lVar10 + 0x38) &&
        ((*(ulonglong *)(*(longlong *)(lVar10 + 0x28) + (ulonglong)(*(uint *)(lVar28 + 4) >> 6) * 8)
          >> (uVar26 & 0x3f) & 1) != 0)))) {
      uVar26 = (ulonglong)*(uint *)(lVar28 + 0x10);
      lVar21 = *(longlong *)
                (*(longlong *)(lVar12 + 0x1a8) + 0x18 + (ulonglong)*(uint *)(lVar28 + 0xc) * 0x48);
      lVar28 = ~*(ulonglong *)
                 (*(longlong *)
                   (*(longlong *)(lVar12 + 0x1a8) + 0x38 + (ulonglong)*(uint *)(lVar28 + 0xc) * 0x48
                   ) + *(longlong *)(lVar10 + 0x110) * 8) * 0x30;
      puVar41 = (undefined8 *)(uVar26 * 0x58 + *(longlong *)(lVar21 + 0x10 + lVar28));
      lVar28 = *(longlong *)(lVar21 + 0x28 + lVar28);
      fVar55 = 0.0;
      fVar56 = 0.0;
      if (*piVar31 != 0) {
        if (*piVar31 == 1) {
          fVar56 = (float)piVar31[1];
          fVar43 = 0.0;
          if (0.0 <= fVar56) {
            fVar43 = fVar56;
          }
          fVar44 = fVar22;
          if (fVar43 <= fVar22) {
            fVar44 = fVar43;
          }
          fVar56 = fVar56 - *(float *)((longlong)puVar41 + 0x2c);
          *(undefined4 *)(lVar28 + uVar26 * 4) = uVar4;
          *(float *)((longlong)puVar41 + 0x2c) = fVar44;
        }
        else {
          fVar56 = (float)piVar31[1];
          *(undefined4 *)(lVar28 + uVar26 * 4) = uVar4;
          fVar44 = *(float *)((longlong)puVar41 + 0x2c) + fVar56;
          fVar43 = 0.0;
          if (0.0 <= fVar44) {
            fVar43 = fVar44;
          }
          fVar44 = fVar22;
          if (fVar43 <= fVar22) {
            fVar44 = fVar43;
          }
          *(float *)((longlong)puVar41 + 0x2c) = fVar44;
        }
      }
      fVar43 = *(float *)(puVar41 + 6);
      if (piVar31[2] == 0) {
        bVar32 = 0;
        fVar44 = fVar43;
      }
      else {
        fVar44 = fVar22;
        if (piVar31[2] == 2) {
          fVar55 = (float)piVar31[3];
          *(undefined4 *)(lVar28 + uVar26 * 4) = uVar4;
          fVar57 = fVar23;
          if (fVar23 <= *(float *)(puVar41 + 6) + fVar55) {
            fVar57 = *(float *)(puVar41 + 6) + fVar55;
          }
          if (fVar57 <= fVar22) {
            fVar44 = fVar57;
          }
          *(float *)(puVar41 + 6) = fVar44;
        }
        else {
          fVar55 = (float)piVar31[3];
          fVar57 = 0.0;
          if (0.0 <= fVar55) {
            fVar57 = fVar55;
          }
          if (fVar57 <= fVar22) {
            fVar44 = fVar57;
          }
          *(undefined4 *)(lVar28 + uVar26 * 4) = uVar4;
          *(float *)(puVar41 + 6) = fVar44;
          fVar55 = fVar55 - fVar43;
        }
        bVar32 = 1;
      }
      fVar57 = 0.0;
      fVar59 = 0.0;
      if (piVar31[10] != 0) {
        if (piVar31[10] == 2) {
          fVar59 = (float)*(undefined8 *)(piVar31 + 0xb);
          fVar58 = (float)((ulonglong)*(undefined8 *)(piVar31 + 0xb) >> 0x20);
          *(undefined4 *)(lVar28 + uVar26 * 4) = uVar4;
          fVar46 = (float)*(undefined8 *)((longlong)puVar41 + 0x24) + fVar59;
          fVar47 = (float)((ulonglong)*(undefined8 *)((longlong)puVar41 + 0x24) >> 0x20) + fVar58;
          fVar51 = fVar47 * fVar47 + fVar46 * fVar46;
          uVar50 = (uint)(fVar22 < fVar51);
          auVar49._0_4_ = SQRT(fVar51);
          auVar49._4_4_ = auVar49._0_4_;
          auVar49._8_8_ = 0;
          auVar52._4_4_ = fVar47;
          auVar52._0_4_ = fVar46;
          auVar52._8_8_ = 0;
          auVar53 = divps(auVar52,auVar49);
          uVar48 = (int)(uVar50 << 0x1f) >> 0x1f;
          uVar50 = (int)(uVar50 << 0x1f) >> 0x1f;
          *(ulonglong *)((longlong)puVar41 + 0x24) =
               CONCAT44(~uVar50 & (uint)fVar47,~uVar48 & (uint)fVar46) |
               CONCAT44(auVar53._4_4_ & uVar50,auVar53._0_4_ & uVar48);
        }
        else {
          uVar37 = *(undefined8 *)(piVar31 + 0xb);
          uVar33 = *(ulonglong *)((longlong)puVar41 + 0x24);
          fVar59 = (float)uVar33;
          fVar58 = (float)(uVar33 >> 0x20);
          fVar46 = fVar58 * fVar58 + fVar59 * fVar59;
          uVar50 = (uint)(fVar22 < fVar46);
          auVar53._0_4_ = SQRT(fVar46);
          auVar53._4_4_ = auVar53._0_4_;
          auVar53._8_8_ = 0;
          auVar54._8_8_ = 0;
          auVar54._0_8_ = uVar33;
          auVar53 = divps(auVar54,auVar53);
          uVar48 = (int)(uVar50 << 0x1f) >> 0x1f;
          uVar50 = (int)(uVar50 << 0x1f) >> 0x1f;
          uVar33 = CONCAT44(~uVar50 & (uint)fVar58,~uVar48 & (uint)fVar59) |
                   CONCAT44(auVar53._4_4_ & uVar50,auVar53._0_4_ & uVar48);
          *(undefined4 *)(lVar28 + uVar26 * 4) = uVar4;
          *(undefined8 *)((longlong)puVar41 + 0x24) = uVar37;
          fVar59 = (float)uVar37 - (float)uVar33;
          fVar58 = (float)((ulonglong)uVar37 >> 0x20) - (float)(uVar33 >> 0x20);
        }
        fVar59 = SQRT(fVar58 * fVar58 + fVar59 * fVar59);
      }
      if (piVar31[6] != 0) {
        if (piVar31[6] == 2) {
          fVar57 = (float)piVar31[7];
          *(undefined4 *)(lVar28 + uVar26 * 4) = uVar4;
          fVar58 = 0.0;
          if (0.0 <= *(float *)(puVar41 + 7) + fVar57) {
            fVar58 = *(float *)(puVar41 + 7) + fVar57;
          }
          fVar46 = fVar22;
          if (fVar58 <= fVar22) {
            fVar46 = fVar58;
          }
          *(float *)(puVar41 + 7) = fVar46;
        }
        else {
          fVar57 = (float)piVar31[7];
          fVar58 = 0.0;
          if (0.0 <= *(float *)(puVar41 + 7)) {
            fVar58 = *(float *)(puVar41 + 7);
          }
          fVar46 = fVar22;
          if (fVar58 <= fVar22) {
            fVar46 = fVar58;
          }
          *(undefined4 *)(lVar28 + uVar26 * 4) = uVar4;
          *(float *)(puVar41 + 7) = fVar57;
          fVar57 = fVar57 - fVar46;
        }
      }
      country_core::resources::roofs::roof_shape::Roof::is_gable();
      if (piVar31[4] == 0) {
        fVar58 = 0.0;
        bVar34 = extraout_AL;
      }
      else if (piVar31[4] == 2) {
        fVar58 = (float)piVar31[5];
        *(undefined4 *)(lVar28 + uVar26 * 4) = uVar4;
        fVar47 = *(float *)((longlong)puVar41 + 0x34) + fVar58;
        fVar46 = 0.0;
        if (0.0 <= fVar47) {
          fVar46 = fVar47;
        }
        fVar47 = fVar22;
        if (fVar46 <= fVar22) {
          fVar47 = fVar46;
        }
        *(float *)((longlong)puVar41 + 0x34) = fVar47;
        country_core::resources::roofs::roof_shape::Roof::is_gable();
        bVar34 = extraout_AL_00;
      }
      else {
        fVar58 = (float)piVar31[5];
        fVar46 = 0.0;
        if (0.0 <= *(float *)((longlong)puVar41 + 0x34)) {
          fVar46 = *(float *)((longlong)puVar41 + 0x34);
        }
        fVar47 = fVar22;
        if (fVar46 <= fVar22) {
          fVar47 = fVar46;
        }
        *(undefined4 *)(lVar28 + uVar26 * 4) = uVar4;
        *(float *)((longlong)puVar41 + 0x34) = fVar58;
        country_core::resources::roofs::roof_shape::Roof::is_gable();
        fVar58 = fVar58 - fVar47;
        bVar34 = extraout_AL_01;
      }
      if (((extraout_AL == bVar34) && (((fVar56 != 0.0 | bVar32) & bVar34) == 0)) &&
         ((fVar57 != 0.0 & bVar34) == 0)) {
        if (extraout_AL != 0) goto LAB_1421abe71;
LAB_1421abf1e:
        if (bVar34 != 0) {
          country_core::resources::roofs::roof_shape::Roof::pivot_3d();
          local_1e8 = CONCAT71(local_1e8._1_7_,10);
          goto LAB_1421abf48;
        }
      }
      else {
        uVar37 = *puVar41;
        *puVar14 = uVar5;
        lVar28 = *(longlong *)(lVar13 + 0x30);
        uVar42 = *(undefined8 *)(lVar13 + 0x40);
        if (lVar28 == *(longlong *)(lVar13 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        lVar21 = *(longlong *)(lVar13 + 0x28);
        *(undefined8 *)(lVar21 + lVar28 * 0x10) = uVar42;
        *(undefined8 *)(lVar21 + 8 + lVar28 * 0x10) = uVar37;
        *(longlong *)(lVar13 + 0x30) = lVar28 + 1;
        *(longlong *)(lVar13 + 0x40) = *(longlong *)(lVar13 + 0x40) + 1;
        if (extraout_AL == 0) goto LAB_1421abf1e;
LAB_1421abe71:
        if (bVar34 == 0) {
          country_core::resources::roofs::roof_shape::Roof::pivot_3d();
          local_1e8 = CONCAT71(local_1e8._1_7_,0xb);
LAB_1421abf48:
          *puVar16 = uVar6;
          lVar28 = *(longlong *)(lVar15 + 0x30);
          uVar37 = *(undefined8 *)(lVar15 + 0x40);
          if (lVar28 == *(longlong *)(lVar15 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                      ();
          }
          lVar21 = *(longlong *)(lVar15 + 0x28);
          *(undefined8 *)(lVar21 + lVar28 * 0x28) = uVar37;
          puVar1 = (undefined4 *)(lVar21 + 0x14 + lVar28 * 0x28);
          *puVar1 = uStack_1e0._4_4_;
          puVar1[1] = local_1d8;
          puVar1[2] = uStack_1d4;
          puVar1[3] = local_1d0;
          puVar2 = (undefined8 *)(lVar21 + 8 + lVar28 * 0x28);
          *puVar2 = local_1e8;
          puVar2[1] = uStack_1e0;
          *(longlong *)(lVar15 + 0x30) = lVar28 + 1;
          *(longlong *)(lVar15 + 0x40) = *(longlong *)(lVar15 + 0x40) + 1;
        }
      }
      uVar50 = ___xmm_7fffffff7fffffff7fffffff7fffffff;
      if (((fVar57 != 0.0) || (fVar56 != 0.0)) || ((fVar55 != 0.0 || (fVar58 != 0.0)))) {
        fVar55 = (float)((uint)fVar55 & ___xmm_7fffffff7fffffff7fffffff7fffffff);
        fVar46 = ceilf(fVar55);
        fVar47 = ceilf((float)((uint)fVar56 & uVar50));
        fVar51 = ceilf((float)((uint)fVar58 & uVar50));
        fVar45 = ceilf((float)((uint)fVar57 & uVar50));
        uVar50 = ~-(uint)(0.0 < fVar46) &
                 (uint)((float)((uint)fVar58 & uVar50) + (float)((uint)fVar56 & uVar50) +
                       (float)((uint)fVar57 & uVar50)) | (uint)fVar55 & -(uint)(0.0 < fVar46);
        country_core::resources::roofs::roof_shape::Roof::pivot_3d();
        local_1e8._0_2_ = CONCAT11(bVar34,2);
        local_1e8 = CONCAT44(fVar46,(undefined4)local_1e8);
        uStack_1e0 = CONCAT44(uVar50,fVar45 + fVar51 + fVar47);
        *puVar16 = uVar6;
        lVar28 = *(longlong *)(lVar15 + 0x30);
        uVar37 = *(undefined8 *)(lVar15 + 0x40);
        if (lVar28 == *(longlong *)(lVar15 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                    ();
        }
        lVar21 = *(longlong *)(lVar15 + 0x28);
        *(undefined8 *)(lVar21 + lVar28 * 0x28) = uVar37;
        puVar3 = (uint *)(lVar21 + 0x14 + lVar28 * 0x28);
        *puVar3 = uVar50;
        puVar3[1] = local_1d8;
        puVar3[2] = uStack_1d4;
        puVar3[3] = local_1d0;
        puVar2 = (undefined8 *)(lVar21 + 8 + lVar28 * 0x28);
        *puVar2 = local_1e8;
        puVar2[1] = uStack_1e0;
        *(longlong *)(lVar15 + 0x30) = lVar28 + 1;
        *(longlong *)(lVar15 + 0x40) = *(longlong *)(lVar15 + 0x40) + 1;
      }
      if (0.0 < fVar59) {
        fVar56 = *(float *)((longlong)puVar41 + 0x24);
        fVar55 = *(float *)(puVar41 + 5);
        country_core::resources::roofs::roof_shape::Roof::pivot_3d();
        local_1e8._1_7_ = (undefined7)((ulonglong)local_1e8 >> 8);
        local_1e8 = CONCAT71(local_1e8._1_7_,3);
        local_1e8 = CONCAT44(SQRT(fVar55 * fVar55 + fVar56 * fVar56),(undefined4)local_1e8);
        uStack_1e0._4_4_ = (undefined4)((ulonglong)uStack_1e0 >> 0x20);
        uStack_1e0 = CONCAT44(uStack_1e0._4_4_,fVar59);
        *puVar16 = uVar6;
        lVar28 = *(longlong *)(lVar15 + 0x30);
        uVar37 = *(undefined8 *)(lVar15 + 0x40);
        if (lVar28 == *(longlong *)(lVar15 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                    ();
        }
        lVar21 = *(longlong *)(lVar15 + 0x28);
        *(undefined8 *)(lVar21 + lVar28 * 0x28) = uVar37;
        puVar1 = (undefined4 *)(lVar21 + 0x14 + lVar28 * 0x28);
        *puVar1 = uStack_1e0._4_4_;
        puVar1[1] = local_1d8;
        puVar1[2] = uStack_1d4;
        puVar1[3] = local_1d0;
        puVar41 = (undefined8 *)(lVar21 + 8 + lVar28 * 0x28);
        *puVar41 = local_1e8;
        puVar41[1] = uStack_1e0;
        *(longlong *)(lVar15 + 0x30) = lVar28 + 1;
        *(longlong *)(lVar15 + 0x40) = *(longlong *)(lVar15 + 0x40) + 1;
      }
      uVar37 = *(undefined8 *)(piVar31 + 8);
      *puVar18 = uVar7;
      lVar28 = *(longlong *)(lVar17 + 0x30);
      uVar42 = *(undefined8 *)(lVar17 + 0x40);
      if (lVar28 == *(longlong *)(lVar17 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                  ();
      }
      lVar21 = *(longlong *)(lVar17 + 0x28);
      lVar29 = lVar28 * 0x20;
      *(undefined8 *)(lVar21 + lVar29) = uVar42;
      *(ulonglong *)(lVar21 + 8 + lVar29) = extraout_RAX;
      *(undefined8 *)(lVar21 + 0x10 + lVar29) = uVar37;
      *(undefined1 *)(lVar21 + 0x18 + lVar29) = 1;
      *(longlong *)(lVar17 + 0x30) = lVar28 + 1;
      *(longlong *)(lVar17 + 0x40) = *(longlong *)(lVar17 + 0x40) + 1;
      piVar39 = piVar36;
      if ((fVar43 <= fVar23) || (fVar23 < fVar44)) goto LAB_1421ac280;
      uVar37 = *(undefined8 *)(piVar31 + 8);
      uVar30 = (undefined1)piVar31[0xd];
      *puVar20 = uVar8;
      lVar28 = *(longlong *)(lVar19 + 0x30);
      uVar42 = *(undefined8 *)(lVar19 + 0x40);
      uVar25 = 0;
      uVar24 = 0;
      if (lVar28 == *(longlong *)(lVar19 + 0x20)) goto LAB_1421ac2e5;
      goto LAB_1421ab9b0;
    }
  }
                    /* WARNING: Subroutine does not return */
  core::result::unwrap_failed();
LAB_1421ac280:
  if ((fVar43 <= fVar23) && (fVar23 < fVar44)) {
    uVar37 = *(undefined8 *)(piVar31 + 8);
    uVar30 = (undefined1)piVar31[0xd];
    *puVar20 = uVar8;
    lVar28 = *(longlong *)(lVar19 + 0x30);
    uVar42 = *(undefined8 *)(lVar19 + 0x40);
    uVar25 = 1;
    uVar24 = 1;
    if (lVar28 == *(longlong *)(lVar19 + 0x20)) {
LAB_1421ac2e5:
      uVar25 = uVar24;
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
LAB_1421ab9b0:
    lVar21 = *(longlong *)(lVar19 + 0x28);
    lVar29 = lVar28 * 0x20;
    *(undefined8 *)(lVar21 + lVar29) = uVar42;
    *(undefined8 *)(lVar21 + 8 + lVar29) = uVar37;
    *(ulonglong *)(lVar21 + 0x10 + lVar29) = extraout_RAX;
    *(undefined1 *)(lVar21 + 0x18 + lVar29) = uVar30;
    *(undefined1 *)(lVar21 + 0x19 + lVar29) = uVar25;
    *(longlong *)(lVar19 + 0x30) = lVar28 + 1;
    *(longlong *)(lVar19 + 0x40) = *(longlong *)(lVar19 + 0x40) + 1;
  }
  goto LAB_1421ab9fc;
}

