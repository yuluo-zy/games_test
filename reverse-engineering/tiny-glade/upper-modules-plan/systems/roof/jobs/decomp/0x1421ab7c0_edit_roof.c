// name=edit_roof
// sig=void __cdecl edit_roof(void)
// size=3092

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
  undefined1 uVar22;
  byte extraout_AL;
  byte extraout_AL_00;
  byte extraout_AL_01;
  undefined1 uVar23;
  ulonglong uVar24;
  int *piVar25;
  ulonglong extraout_RAX;
  longlong lVar26;
  longlong lVar27;
  longlong *in_RDX;
  undefined1 uVar28;
  int *piVar29;
  byte bVar30;
  ulonglong uVar31;
  byte bVar32;
  ulonglong uVar33;
  int *piVar34;
  undefined8 uVar35;
  undefined8 *in_R8;
  int *piVar36;
  int *piVar37;
  longlong *in_R9;
  int *piVar38;
  undefined8 *puVar39;
  undefined8 uVar40;
  float fVar41;
  float fVar42;
  float fVar43;
  float fVar44;
  float fVar45;
  uint uVar46;
  uint uVar48;
  undefined1 auVar47 [16];
  float fVar49;
  undefined1 auVar50 [16];
  undefined1 auVar51 [16];
  undefined1 auVar52 [16];
  float fVar53;
  float fVar54;
  float fVar55;
  float fVar56;
  float fVar57;
  float fVar58;
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
  uVar31 = 0;
  uVar24 = uVar11 - *(ulonglong *)(lVar10 + 0x18);
  if (uVar11 < *(ulonglong *)(lVar10 + 0x18)) {
    uVar24 = uVar31;
  }
  uVar33 = uVar11 - *(ulonglong *)(lVar10 + 0x38);
  if (uVar11 < *(ulonglong *)(lVar10 + 0x38)) {
    uVar33 = uVar31;
  }
  piVar36 = (int *)(uVar24 * 0x40 + *(longlong *)(lVar10 + 8));
  uVar11 = *(ulonglong *)(lVar10 + 0x10) - uVar24;
  if (*(ulonglong *)(lVar10 + 0x10) < uVar24) {
    piVar36 = (int *)&DAT_00000008;
    uVar11 = uVar31;
  }
  piVar25 = (int *)(uVar33 * 0x40 + *(longlong *)(lVar10 + 0x28));
  uVar24 = *(ulonglong *)(lVar10 + 0x30) - uVar33;
  if (*(ulonglong *)(lVar10 + 0x30) < uVar33) {
    piVar25 = (int *)&DAT_00000008;
    uVar24 = uVar31;
  }
  *puVar9 = (*(longlong *)(lVar10 + 0x40) - uVar24) - uVar11;
  piVar38 = piVar25 + uVar24 * 0x10;
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
  fVar58 = 1.0;
  piVar37 = piVar36;
LAB_1421ab9fc:
  if ((piVar37 == (int *)0x0) || (piVar37 == piVar36 + uVar11 * 0x10)) {
    if ((piVar25 == (int *)0x0) || (piVar25 == piVar38)) {
      return;
    }
    piVar34 = (int *)0x0;
    piVar29 = piVar25;
    piVar25 = piVar25 + 0x10;
  }
  else {
    piVar34 = piVar37 + 0x10;
    piVar29 = piVar37;
  }
  *puVar9 = *puVar9 + 1;
  country_core::resources::walls::public_walls::PublicWalls::roof_entity();
  if (extraout_RAX >> 0x20 == 0) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  uVar24 = extraout_RAX & 0xffffffff;
  if ((uVar24 < *(ulonglong *)(lVar12 + 0x10)) &&
     (*(int *)(*(longlong *)(lVar12 + 8) + uVar24 * 0x14) == (int)(extraout_RAX >> 0x20))) {
    lVar26 = *(longlong *)(lVar12 + 8) + uVar24 * 0x14;
    uVar24 = (ulonglong)*(uint *)(lVar26 + 4);
    if ((uVar24 != 0xffffffff) &&
       ((uVar24 < *(ulonglong *)(lVar10 + 0x38) &&
        ((*(ulonglong *)(*(longlong *)(lVar10 + 0x28) + (ulonglong)(*(uint *)(lVar26 + 4) >> 6) * 8)
          >> (uVar24 & 0x3f) & 1) != 0)))) {
      uVar24 = (ulonglong)*(uint *)(lVar26 + 0x10);
      lVar21 = *(longlong *)
                (*(longlong *)(lVar12 + 0x1a8) + 0x18 + (ulonglong)*(uint *)(lVar26 + 0xc) * 0x48);
      lVar26 = ~*(ulonglong *)
                 (*(longlong *)
                   (*(longlong *)(lVar12 + 0x1a8) + 0x38 + (ulonglong)*(uint *)(lVar26 + 0xc) * 0x48
                   ) + *(longlong *)(lVar10 + 0x110) * 8) * 0x30;
      puVar39 = (undefined8 *)(uVar24 * 0x58 + *(longlong *)(lVar21 + 0x10 + lVar26));
      lVar26 = *(longlong *)(lVar21 + 0x28 + lVar26);
      fVar53 = 0.0;
      fVar54 = 0.0;
      if (*piVar29 != 0) {
        if (*piVar29 == 1) {
          fVar54 = (float)piVar29[1];
          fVar41 = 0.0;
          if (0.0 <= fVar54) {
            fVar41 = fVar54;
          }
          fVar42 = fVar58;
          if (fVar41 <= 1.0) {
            fVar42 = fVar41;
          }
          fVar54 = fVar54 - *(float *)((longlong)puVar39 + 0x2c);
          *(undefined4 *)(lVar26 + uVar24 * 4) = uVar4;
          *(float *)((longlong)puVar39 + 0x2c) = fVar42;
        }
        else {
          fVar54 = (float)piVar29[1];
          *(undefined4 *)(lVar26 + uVar24 * 4) = uVar4;
          fVar42 = *(float *)((longlong)puVar39 + 0x2c) + fVar54;
          fVar41 = 0.0;
          if (0.0 <= fVar42) {
            fVar41 = fVar42;
          }
          fVar42 = fVar58;
          if (fVar41 <= 1.0) {
            fVar42 = fVar41;
          }
          *(float *)((longlong)puVar39 + 0x2c) = fVar42;
        }
      }
      fVar41 = *(float *)(puVar39 + 6);
      if (piVar29[2] == 0) {
        bVar30 = 0;
        fVar42 = fVar41;
      }
      else {
        fVar42 = fVar58;
        if (piVar29[2] == 2) {
          fVar53 = (float)piVar29[3];
          *(undefined4 *)(lVar26 + uVar24 * 4) = uVar4;
          fVar55 = 0.1;
          if (0.1 <= *(float *)(puVar39 + 6) + fVar53) {
            fVar55 = *(float *)(puVar39 + 6) + fVar53;
          }
          if (fVar55 <= 1.0) {
            fVar42 = fVar55;
          }
          *(float *)(puVar39 + 6) = fVar42;
        }
        else {
          fVar53 = (float)piVar29[3];
          fVar55 = 0.0;
          if (0.0 <= fVar53) {
            fVar55 = fVar53;
          }
          if (fVar55 <= 1.0) {
            fVar42 = fVar55;
          }
          *(undefined4 *)(lVar26 + uVar24 * 4) = uVar4;
          *(float *)(puVar39 + 6) = fVar42;
          fVar53 = fVar53 - fVar41;
        }
        bVar30 = 1;
      }
      fVar55 = 0.0;
      fVar57 = 0.0;
      if (piVar29[10] != 0) {
        if (piVar29[10] == 2) {
          fVar57 = (float)*(undefined8 *)(piVar29 + 0xb);
          fVar56 = (float)((ulonglong)*(undefined8 *)(piVar29 + 0xb) >> 0x20);
          *(undefined4 *)(lVar26 + uVar24 * 4) = uVar4;
          fVar44 = (float)*(undefined8 *)((longlong)puVar39 + 0x24) + fVar57;
          fVar45 = (float)((ulonglong)*(undefined8 *)((longlong)puVar39 + 0x24) >> 0x20) + fVar56;
          fVar49 = fVar45 * fVar45 + fVar44 * fVar44;
          uVar48 = (uint)(1.0 < fVar49);
          auVar47._0_4_ = SQRT(fVar49);
          auVar47._4_4_ = auVar47._0_4_;
          auVar47._8_8_ = 0;
          auVar50._4_4_ = fVar45;
          auVar50._0_4_ = fVar44;
          auVar50._8_8_ = 0;
          auVar51 = divps(auVar50,auVar47);
          uVar46 = (int)(uVar48 << 0x1f) >> 0x1f;
          uVar48 = (int)(uVar48 << 0x1f) >> 0x1f;
          *(ulonglong *)((longlong)puVar39 + 0x24) =
               CONCAT44(~uVar48 & (uint)fVar45,~uVar46 & (uint)fVar44) |
               CONCAT44(auVar51._4_4_ & uVar48,auVar51._0_4_ & uVar46);
        }
        else {
          uVar35 = *(undefined8 *)(piVar29 + 0xb);
          uVar31 = *(ulonglong *)((longlong)puVar39 + 0x24);
          fVar57 = (float)uVar31;
          fVar56 = (float)(uVar31 >> 0x20);
          fVar44 = fVar56 * fVar56 + fVar57 * fVar57;
          uVar48 = (uint)(1.0 < fVar44);
          auVar51._0_4_ = SQRT(fVar44);
          auVar51._4_4_ = auVar51._0_4_;
          auVar51._8_8_ = 0;
          auVar52._8_8_ = 0;
          auVar52._0_8_ = uVar31;
          auVar51 = divps(auVar52,auVar51);
          uVar46 = (int)(uVar48 << 0x1f) >> 0x1f;
          uVar48 = (int)(uVar48 << 0x1f) >> 0x1f;
          uVar31 = CONCAT44(~uVar48 & (uint)fVar56,~uVar46 & (uint)fVar57) |
                   CONCAT44(auVar51._4_4_ & uVar48,auVar51._0_4_ & uVar46);
          *(undefined4 *)(lVar26 + uVar24 * 4) = uVar4;
          *(undefined8 *)((longlong)puVar39 + 0x24) = uVar35;
          fVar57 = (float)uVar35 - (float)uVar31;
          fVar56 = (float)((ulonglong)uVar35 >> 0x20) - (float)(uVar31 >> 0x20);
        }
        fVar57 = SQRT(fVar56 * fVar56 + fVar57 * fVar57);
      }
      if (piVar29[6] != 0) {
        if (piVar29[6] == 2) {
          fVar55 = (float)piVar29[7];
          *(undefined4 *)(lVar26 + uVar24 * 4) = uVar4;
          fVar56 = 0.0;
          if (0.0 <= *(float *)(puVar39 + 7) + fVar55) {
            fVar56 = *(float *)(puVar39 + 7) + fVar55;
          }
          fVar44 = fVar58;
          if (fVar56 <= 1.0) {
            fVar44 = fVar56;
          }
          *(float *)(puVar39 + 7) = fVar44;
        }
        else {
          fVar55 = (float)piVar29[7];
          fVar56 = 0.0;
          if (0.0 <= *(float *)(puVar39 + 7)) {
            fVar56 = *(float *)(puVar39 + 7);
          }
          fVar44 = fVar58;
          if (fVar56 <= 1.0) {
            fVar44 = fVar56;
          }
          *(undefined4 *)(lVar26 + uVar24 * 4) = uVar4;
          *(float *)(puVar39 + 7) = fVar55;
          fVar55 = fVar55 - fVar44;
        }
      }
      country_core::resources::roofs::roof_shape::Roof::is_gable();
      if (piVar29[4] == 0) {
        fVar56 = 0.0;
        bVar32 = extraout_AL;
      }
      else if (piVar29[4] == 2) {
        fVar56 = (float)piVar29[5];
        *(undefined4 *)(lVar26 + uVar24 * 4) = uVar4;
        fVar45 = *(float *)((longlong)puVar39 + 0x34) + fVar56;
        fVar44 = 0.0;
        if (0.0 <= fVar45) {
          fVar44 = fVar45;
        }
        fVar45 = fVar58;
        if (fVar44 <= 1.0) {
          fVar45 = fVar44;
        }
        *(float *)((longlong)puVar39 + 0x34) = fVar45;
        country_core::resources::roofs::roof_shape::Roof::is_gable();
        bVar32 = extraout_AL_00;
      }
      else {
        fVar56 = (float)piVar29[5];
        fVar44 = 0.0;
        if (0.0 <= *(float *)((longlong)puVar39 + 0x34)) {
          fVar44 = *(float *)((longlong)puVar39 + 0x34);
        }
        fVar45 = fVar58;
        if (fVar44 <= 1.0) {
          fVar45 = fVar44;
        }
        *(undefined4 *)(lVar26 + uVar24 * 4) = uVar4;
        *(float *)((longlong)puVar39 + 0x34) = fVar56;
        country_core::resources::roofs::roof_shape::Roof::is_gable();
        fVar56 = fVar56 - fVar45;
        bVar32 = extraout_AL_01;
      }
      if (((extraout_AL == bVar32) && (((fVar54 != 0.0 | bVar30) & bVar32) == 0)) &&
         ((fVar55 != 0.0 & bVar32) == 0)) {
        if (extraout_AL != 0) goto LAB_1421abe71;
LAB_1421abf1e:
        if (bVar32 != 0) {
          country_core::resources::roofs::roof_shape::Roof::pivot_3d();
          local_1e8 = CONCAT71(local_1e8._1_7_,10);
          goto LAB_1421abf48;
        }
      }
      else {
        uVar35 = *puVar39;
        *puVar14 = uVar5;
        lVar26 = *(longlong *)(lVar13 + 0x30);
        uVar40 = *(undefined8 *)(lVar13 + 0x40);
        if (lVar26 == *(longlong *)(lVar13 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        lVar21 = *(longlong *)(lVar13 + 0x28);
        *(undefined8 *)(lVar21 + lVar26 * 0x10) = uVar40;
        *(undefined8 *)(lVar21 + 8 + lVar26 * 0x10) = uVar35;
        *(longlong *)(lVar13 + 0x30) = lVar26 + 1;
        *(longlong *)(lVar13 + 0x40) = *(longlong *)(lVar13 + 0x40) + 1;
        if (extraout_AL == 0) goto LAB_1421abf1e;
LAB_1421abe71:
        if (bVar32 == 0) {
          country_core::resources::roofs::roof_shape::Roof::pivot_3d();
          local_1e8 = CONCAT71(local_1e8._1_7_,0xb);
LAB_1421abf48:
          *puVar16 = uVar6;
          lVar26 = *(longlong *)(lVar15 + 0x30);
          uVar35 = *(undefined8 *)(lVar15 + 0x40);
          if (lVar26 == *(longlong *)(lVar15 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                      ();
          }
          lVar21 = *(longlong *)(lVar15 + 0x28);
          *(undefined8 *)(lVar21 + lVar26 * 0x28) = uVar35;
          puVar1 = (undefined4 *)(lVar21 + 0x14 + lVar26 * 0x28);
          *puVar1 = uStack_1e0._4_4_;
          puVar1[1] = local_1d8;
          puVar1[2] = uStack_1d4;
          puVar1[3] = local_1d0;
          puVar2 = (undefined8 *)(lVar21 + 8 + lVar26 * 0x28);
          *puVar2 = local_1e8;
          puVar2[1] = uStack_1e0;
          *(longlong *)(lVar15 + 0x30) = lVar26 + 1;
          *(longlong *)(lVar15 + 0x40) = *(longlong *)(lVar15 + 0x40) + 1;
        }
      }
      if (((fVar55 != 0.0) || (fVar54 != 0.0)) || ((fVar53 != 0.0 || (fVar56 != 0.0)))) {
        fVar44 = ceilf(ABS(fVar53));
        fVar45 = ceilf(ABS(fVar54));
        fVar49 = ceilf(ABS(fVar56));
        fVar43 = ceilf(ABS(fVar55));
        uVar48 = ~-(uint)(0.0 < fVar44) & (uint)(ABS(fVar56) + ABS(fVar54) + ABS(fVar55)) |
                 (uint)ABS(fVar53) & -(uint)(0.0 < fVar44);
        country_core::resources::roofs::roof_shape::Roof::pivot_3d();
        local_1e8._0_2_ = CONCAT11(bVar32,2);
        local_1e8 = CONCAT44(fVar44,(undefined4)local_1e8);
        uStack_1e0 = CONCAT44(uVar48,fVar43 + fVar49 + fVar45);
        *puVar16 = uVar6;
        lVar26 = *(longlong *)(lVar15 + 0x30);
        uVar35 = *(undefined8 *)(lVar15 + 0x40);
        if (lVar26 == *(longlong *)(lVar15 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                    ();
        }
        lVar21 = *(longlong *)(lVar15 + 0x28);
        *(undefined8 *)(lVar21 + lVar26 * 0x28) = uVar35;
        puVar3 = (uint *)(lVar21 + 0x14 + lVar26 * 0x28);
        *puVar3 = uVar48;
        puVar3[1] = local_1d8;
        puVar3[2] = uStack_1d4;
        puVar3[3] = local_1d0;
        puVar2 = (undefined8 *)(lVar21 + 8 + lVar26 * 0x28);
        *puVar2 = local_1e8;
        puVar2[1] = uStack_1e0;
        *(longlong *)(lVar15 + 0x30) = lVar26 + 1;
        *(longlong *)(lVar15 + 0x40) = *(longlong *)(lVar15 + 0x40) + 1;
      }
      if (0.0 < fVar57) {
        fVar54 = *(float *)((longlong)puVar39 + 0x24);
        fVar53 = *(float *)(puVar39 + 5);
        country_core::resources::roofs::roof_shape::Roof::pivot_3d();
        local_1e8._1_7_ = (undefined7)((ulonglong)local_1e8 >> 8);
        local_1e8 = CONCAT71(local_1e8._1_7_,3);
        local_1e8 = CONCAT44(SQRT(fVar53 * fVar53 + fVar54 * fVar54),(undefined4)local_1e8);
        uStack_1e0._4_4_ = (undefined4)((ulonglong)uStack_1e0 >> 0x20);
        uStack_1e0 = CONCAT44(uStack_1e0._4_4_,fVar57);
        *puVar16 = uVar6;
        lVar26 = *(longlong *)(lVar15 + 0x30);
        uVar35 = *(undefined8 *)(lVar15 + 0x40);
        if (lVar26 == *(longlong *)(lVar15 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                    ();
        }
        lVar21 = *(longlong *)(lVar15 + 0x28);
        *(undefined8 *)(lVar21 + lVar26 * 0x28) = uVar35;
        puVar1 = (undefined4 *)(lVar21 + 0x14 + lVar26 * 0x28);
        *puVar1 = uStack_1e0._4_4_;
        puVar1[1] = local_1d8;
        puVar1[2] = uStack_1d4;
        puVar1[3] = local_1d0;
        puVar39 = (undefined8 *)(lVar21 + 8 + lVar26 * 0x28);
        *puVar39 = local_1e8;
        puVar39[1] = uStack_1e0;
        *(longlong *)(lVar15 + 0x30) = lVar26 + 1;
        *(longlong *)(lVar15 + 0x40) = *(longlong *)(lVar15 + 0x40) + 1;
      }
      uVar35 = *(undefined8 *)(piVar29 + 8);
      *puVar18 = uVar7;
      lVar26 = *(longlong *)(lVar17 + 0x30);
      uVar40 = *(undefined8 *)(lVar17 + 0x40);
      if (lVar26 == *(longlong *)(lVar17 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                  ();
      }
      lVar21 = *(longlong *)(lVar17 + 0x28);
      lVar27 = lVar26 * 0x20;
      *(undefined8 *)(lVar21 + lVar27) = uVar40;
      *(ulonglong *)(lVar21 + 8 + lVar27) = extraout_RAX;
      *(undefined8 *)(lVar21 + 0x10 + lVar27) = uVar35;
      *(undefined1 *)(lVar21 + 0x18 + lVar27) = 1;
      *(longlong *)(lVar17 + 0x30) = lVar26 + 1;
      *(longlong *)(lVar17 + 0x40) = *(longlong *)(lVar17 + 0x40) + 1;
      piVar37 = piVar34;
      if ((fVar41 <= 0.1) || (0.1 < fVar42)) goto LAB_1421ac280;
      uVar35 = *(undefined8 *)(piVar29 + 8);
      uVar28 = (undefined1)piVar29[0xd];
      *puVar20 = uVar8;
      lVar26 = *(longlong *)(lVar19 + 0x30);
      uVar40 = *(undefined8 *)(lVar19 + 0x40);
      uVar23 = 0;
      uVar22 = 0;
      if (lVar26 == *(longlong *)(lVar19 + 0x20)) goto LAB_1421ac2e5;
      goto LAB_1421ab9b0;
    }
  }
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
LAB_1421ac280:
  if ((fVar41 <= 0.1) && (0.1 < fVar42)) {
    uVar35 = *(undefined8 *)(piVar29 + 8);
    uVar28 = (undefined1)piVar29[0xd];
    *puVar20 = uVar8;
    lVar26 = *(longlong *)(lVar19 + 0x30);
    uVar40 = *(undefined8 *)(lVar19 + 0x40);
    uVar23 = 1;
    uVar22 = 1;
    if (lVar26 == *(longlong *)(lVar19 + 0x20)) {
LAB_1421ac2e5:
      uVar23 = uVar22;
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
LAB_1421ab9b0:
    lVar21 = *(longlong *)(lVar19 + 0x28);
    lVar27 = lVar26 * 0x20;
    *(undefined8 *)(lVar21 + lVar27) = uVar40;
    *(undefined8 *)(lVar21 + 8 + lVar27) = uVar35;
    *(ulonglong *)(lVar21 + 0x10 + lVar27) = extraout_RAX;
    *(undefined1 *)(lVar21 + 0x18 + lVar27) = uVar28;
    *(undefined1 *)(lVar21 + 0x19 + lVar27) = uVar23;
    *(longlong *)(lVar19 + 0x30) = lVar26 + 1;
    *(longlong *)(lVar19 + 0x40) = *(longlong *)(lVar19 + 0x40) + 1;
  }
  goto LAB_1421ab9fc;
}

