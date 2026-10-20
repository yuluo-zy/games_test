// name: split_wall_into_sides
// addr: 0x14075dc60
// size: 3130
// signature: void __cdecl split_wall_into_sides(void)


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl system_wall_constructor::plaster_systems::split_wall_into_sides(void)

{
  char cVar1;
  float fVar2;
  ulonglong uVar3;
  char extraout_AL;
  undefined8 *******pppppppuVar4;
  ulonglong *puVar5;
  uint uVar6;
  longlong *in_RCX;
  ulonglong uVar7;
  longlong in_RDX;
  ulonglong *puVar8;
  undefined1 uVar9;
  longlong in_R8;
  undefined1 uVar10;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  float extraout_XMM0_Da_05;
  float extraout_XMM0_Da_06;
  float extraout_XMM0_Da_07;
  float fVar11;
  float fVar12;
  float extraout_XMM0_Da_08;
  float extraout_XMM0_Da_09;
  float extraout_XMM0_Da_10;
  float extraout_XMM0_Da_11;
  float extraout_XMM0_Da_12;
  float extraout_XMM0_Da_13;
  float extraout_XMM0_Da_14;
  float extraout_XMM0_Da_15;
  float extraout_XMM0_Da_16;
  float extraout_XMM0_Da_17;
  float extraout_XMM0_Da_18;
  undefined4 extraout_XMM0_Db;
  undefined4 extraout_XMM0_Db_00;
  undefined4 extraout_XMM0_Db_01;
  undefined4 extraout_XMM0_Db_07;
  undefined4 extraout_XMM0_Db_08;
  undefined4 extraout_XMM0_Dc;
  undefined1 extraout_XMM0 [16];
  undefined1 auVar13 [16];
  undefined4 extraout_XMM0_Db_02;
  undefined4 extraout_XMM0_Db_03;
  undefined4 extraout_XMM0_Dd;
  undefined1 auVar14 [16];
  undefined4 extraout_XMM0_Db_04;
  undefined4 extraout_XMM0_Dc_00;
  undefined4 extraout_XMM0_Dd_00;
  undefined1 auVar15 [16];
  undefined4 extraout_XMM0_Db_05;
  undefined1 auVar16 [16];
  undefined4 extraout_XMM0_Db_06;
  undefined4 extraout_XMM0_Dc_01;
  undefined4 extraout_XMM0_Dd_01;
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  float fVar19;
  undefined4 uVar20;
  float fVar21;
  undefined4 uVar22;
  undefined4 uVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  undefined1 auVar27 [16];
  undefined1 auVar28 [16];
  undefined1 auVar29 [16];
  float fVar31;
  undefined1 auVar32 [16];
  undefined1 auVar33 [16];
  undefined1 auVar34 [16];
  undefined1 auVar35 [16];
  undefined1 auVar37 [16];
  undefined1 auVar38 [16];
  undefined1 auVar39 [16];
  undefined1 auVar41 [16];
  undefined8 ******local_1c0;
  ulonglong uStack_1b8;
  longlong local_1b0;
  longlong lStack_1a8;
  longlong local_1a0;
  longlong lStack_198;
  longlong local_190;
  longlong lStack_188;
  longlong local_180;
  longlong lStack_178;
  longlong local_170;
  longlong lStack_168;
  ulonglong local_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined **local_140;
  undefined8 local_138;
  float **local_130;
  undefined8 local_128;
  undefined8 local_120;
  float local_10c;
  float *local_108;
  code *local_100;
  float local_f4 [45];
  undefined1 auVar30 [16];
  undefined1 auVar36 [16];
  undefined1 auVar40 [16];
  
  local_f4[1] = -NAN;
  local_f4[2] = -NAN;
  uVar6 = 0;
  if (*(uint *)(in_RDX + 600) < 2) {
    uVar6 = *(uint *)(in_RDX + 600);
  }
  if ((uVar6 & 1) == 0) {
    country_core::resources::walls::public_wall_state::PublicWallState::max_y();
    fVar25 = extraout_XMM0._0_4_ - *(float *)(in_RDX + 0x280);
    uVar6 = *(uint *)(in_RDX + 0x30);
    *in_RCX = 0x3f80000000000000;
    *(float *)(in_RCX + 1) = extraout_XMM0._0_4_;
    *(float *)((longlong)in_RCX + 0xc) = fVar25;
    auVar18._4_8_ = 0;
    auVar18._0_4_ = uVar6;
    auVar18._12_4_ = extraout_XMM0._4_4_;
    auVar41._8_8_ = auVar18._8_8_;
    auVar41._4_4_ = fVar25;
    auVar41._0_4_ = uVar6;
    divps(auVar41,___xmm_00000000000000003ecccccd3ecccccd);
    fVar19 = (float)roundf();
    fVar21 = (float)roundf();
    fVar25 = 0.0;
    if (0.0 <= fVar21) {
      fVar25 = fVar21;
    }
    fVar21 = 65535.0;
    if (fVar25 <= 65535.0) {
      fVar21 = fVar25;
    }
    fVar25 = 0.0;
    if (0.0 <= fVar19) {
      fVar25 = fVar19;
    }
    fVar19 = 65535.0;
    if (fVar25 <= 65535.0) {
      fVar19 = fVar25;
    }
    auVar13._8_8_ = 0;
    auVar13._0_8_ =
         (ulonglong)(uint)(int)fVar19 & 0xffffffff0000ffff | (ulonglong)(ushort)(int)fVar21 << 0x10;
    auVar41 = psubusw(___xmm_00000000000000000000000000020002,auVar13);
    *(uint *)(in_RCX + 2) =
         CONCAT22(auVar41._2_2_ + (ushort)(int)fVar21,
                  auVar41._0_2_ + (short)((ulonglong)(uint)(int)fVar19 & 0xffffffff0000ffff));
    *(undefined1 *)((longlong)in_RCX + 0x14) = 0;
    in_RCX[0xc] = 1;
    return;
  }
  local_148 = *(undefined8 *)(in_RDX + 0x26c);
  local_158 = *(undefined8 *)(in_RDX + 0x25c);
  uStack_150 = *(undefined8 *)(in_RDX + 0x264);
  if ((in_R8 == 0) ||
     (country_core::resources::roofs::roof_shape::Roof::is_gable(), extraout_AL == '\0')) {
    country_core::resources::walls::public_wall_state::PublicWallState::max_y();
    uVar10 = 0;
    fVar25 = extraout_XMM0_Da_05;
    uVar23 = extraout_XMM0_Db_02;
    fVar19 = extraout_XMM0_Da_05;
    uVar20 = extraout_XMM0_Db_02;
    fVar21 = extraout_XMM0_Da_05;
    uVar22 = extraout_XMM0_Db_02;
    local_10c = extraout_XMM0_Da_05;
  }
  else {
    cVar1 = *(char *)(in_R8 + 0x3c);
    country_core::resources::walls::public_wall_state::PublicWallState::max_y();
    uVar23 = extraout_XMM0_Db;
    if (cVar1 != '\0') {
      country_core::resources::walls::public_wall_state::PublicWallState::max_y();
      country_core::resources::roofs::roof_shape::Roof::height();
      fVar21 = extraout_XMM0_Da_01 + extraout_XMM0_Da_00;
      country_core::resources::walls::public_wall_state::PublicWallState::max_y();
      local_10c = extraout_XMM0_Da_02;
      country_core::resources::walls::public_wall_state::PublicWallState::max_y();
      country_core::resources::roofs::roof_shape::Roof::height();
      fVar19 = extraout_XMM0_Da_04 + extraout_XMM0_Da_03;
      uVar9 = 1;
      uVar10 = 0;
      fVar25 = extraout_XMM0_Da;
      uVar20 = extraout_XMM0_Db_01;
      uVar22 = extraout_XMM0_Db_00;
      goto LAB_14075de4c;
    }
    country_core::resources::roofs::roof_shape::Roof::height();
    fVar25 = extraout_XMM0_Da + extraout_XMM0_Da_14;
    country_core::resources::walls::public_wall_state::PublicWallState::max_y();
    country_core::resources::walls::public_wall_state::PublicWallState::max_y();
    country_core::resources::roofs::roof_shape::Roof::height();
    local_10c = extraout_XMM0_Da_17 + extraout_XMM0_Da_16;
    country_core::resources::walls::public_wall_state::PublicWallState::max_y();
    uVar10 = 1;
    fVar19 = extraout_XMM0_Da_18;
    uVar20 = extraout_XMM0_Db_08;
    fVar21 = extraout_XMM0_Da_15;
    uVar22 = extraout_XMM0_Db_07;
  }
  uVar9 = 0;
LAB_14075de4c:
  local_160 = 0;
  ::utils::geometry::rectangle::Rectangle2d::get_point_u();
  fVar26 = extraout_XMM0_Da_06 + 1e-07;
  local_f4[0] = fVar26;
  if ((fVar26 < 0.0) || (1.0 < fVar26)) {
    local_108 = local_f4;
    local_100 = core::fmt::float::impl$5::fmt;
    local_140 = &anon_95101f4d2e0328d894274322bc39dc37_3_llvm_14090887955549727255;
    local_138 = 1;
    local_120 = 0;
    local_130 = &local_108;
    local_128 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  if (NAN(fVar26)) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  ::utils::geometry::rectangle::Rectangle2d::get_point_u();
  fVar31 = extraout_XMM0_Da_07 + -1e-07;
  local_f4[0] = fVar31;
  if ((fVar31 < 0.0) || (1.0 < fVar31)) {
    local_108 = local_f4;
    local_100 = core::fmt::float::impl$5::fmt;
    local_140 = &anon_95101f4d2e0328d894274322bc39dc37_3_llvm_14090887955549727255;
    local_138 = 1;
    local_120 = 0;
    local_130 = &local_108;
    local_128 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  if (NAN(fVar31)) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  fVar24 = fVar25 - *(float *)(in_RDX + 0x280);
  auVar37._4_4_ = extraout_XMM0_Db_03;
  auVar37._0_4_ = fVar31;
  auVar37._8_4_ = extraout_XMM0_Dc;
  auVar37._12_4_ = extraout_XMM0_Dd;
  auVar38._4_12_ = auVar37._4_12_;
  auVar38._0_4_ = (fVar31 - fVar26) * *(float *)(in_RDX + 0x30);
  auVar40._0_8_ = auVar38._0_8_;
  auVar40._8_4_ = extraout_XMM0_Db_03;
  auVar40._12_4_ = uVar23;
  auVar39._8_8_ = auVar40._8_8_;
  auVar39._4_4_ = fVar24;
  auVar39._0_4_ = auVar38._0_4_;
  divps(auVar39,___xmm_00000000000000003ecccccd3ecccccd);
  fVar11 = (float)roundf();
  fVar12 = (float)roundf();
  puVar5 = &local_160;
  pppppppuVar4 = &local_1c0;
  if (4 < local_160) {
    puVar5 = &uStack_1b8;
    pppppppuVar4 = (undefined8 *******)local_1c0;
  }
  puVar8 = &local_160;
  uVar3 = 4;
  if (4 < local_160) {
    puVar8 = &uStack_1b8;
    uVar3 = local_160;
  }
  uVar7 = *puVar8;
  if (*puVar8 == uVar3) {
    smallvec::SmallVec::
    reserve_one_unchecked<array_<system_wall_constructor::plaster_systems::WallSide,4>_>();
    pppppppuVar4 = (undefined8 *******)local_1c0;
    puVar5 = &uStack_1b8;
    uVar7 = uStack_1b8;
  }
  auVar41 = ___xmm_00000000000000000000000000020002;
  fVar2 = 0.0;
  if (0.0 <= fVar12) {
    fVar2 = fVar12;
  }
  fVar12 = 65535.0;
  if (fVar2 <= 65535.0) {
    fVar12 = fVar2;
  }
  fVar2 = 0.0;
  if (0.0 <= fVar11) {
    fVar2 = fVar11;
  }
  fVar11 = 65535.0;
  if (fVar2 <= 65535.0) {
    fVar11 = fVar2;
  }
  auVar14._8_8_ = 0;
  auVar14._0_8_ =
       (ulonglong)(uint)(int)fVar11 & 0xffffffff0000ffff | (ulonglong)(ushort)(int)fVar12 << 0x10;
  auVar18 = psubusw(___xmm_00000000000000000000000000020002,auVar14);
  *(float *)(pppppppuVar4 + uVar7 * 3) = fVar26;
  *(float *)((longlong)pppppppuVar4 + uVar7 * 0x18 + 4) = fVar31;
  *(float *)(pppppppuVar4 + uVar7 * 3 + 1) = fVar25;
  *(float *)((longlong)pppppppuVar4 + uVar7 * 0x18 + 0xc) = fVar24;
  *(uint *)(pppppppuVar4 + uVar7 * 3 + 2) =
       CONCAT22(auVar18._2_2_ + (ushort)(int)fVar12,
                auVar18._0_2_ + (short)((ulonglong)(uint)(int)fVar11 & 0xffffffff0000ffff));
  *(undefined1 *)((longlong)pppppppuVar4 + uVar7 * 0x18 + 0x14) = uVar10;
  *puVar5 = *puVar5 + 1;
  ::utils::geometry::rectangle::Rectangle2d::get_point_u();
  fVar25 = extraout_XMM0_Da_08 + 1e-07;
  local_f4[0] = fVar25;
  if ((fVar25 < 0.0) || (1.0 < fVar25)) {
    local_108 = local_f4;
    local_100 = core::fmt::float::impl$5::fmt;
    local_140 = &anon_95101f4d2e0328d894274322bc39dc37_3_llvm_14090887955549727255;
    local_138 = 1;
    local_120 = 0;
    local_130 = &local_108;
    local_128 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  if (NAN(fVar25)) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  ::utils::geometry::rectangle::Rectangle2d::get_point_u();
  fVar26 = extraout_XMM0_Da_09 + -1e-07;
  local_f4[0] = fVar26;
  if ((fVar26 < 0.0) || (1.0 < fVar26)) {
    local_108 = local_f4;
    local_100 = core::fmt::float::impl$5::fmt;
    local_140 = &anon_95101f4d2e0328d894274322bc39dc37_3_llvm_14090887955549727255;
    local_138 = 1;
    local_120 = 0;
    local_130 = &local_108;
    local_128 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  if (NAN(fVar26)) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  fVar12 = fVar21 - *(float *)(in_RDX + 0x280);
  auVar33._4_4_ = extraout_XMM0_Db_04;
  auVar33._0_4_ = fVar26;
  auVar33._8_4_ = extraout_XMM0_Dc_00;
  auVar33._12_4_ = extraout_XMM0_Dd_00;
  auVar34._4_12_ = auVar33._4_12_;
  auVar34._0_4_ = (fVar26 - fVar25) * *(float *)(in_RDX + 0x30);
  auVar36._0_8_ = auVar34._0_8_;
  auVar36._8_4_ = extraout_XMM0_Db_04;
  auVar36._12_4_ = uVar22;
  auVar35._8_8_ = auVar36._8_8_;
  auVar35._4_4_ = fVar12;
  auVar35._0_4_ = auVar34._0_4_;
  divps(auVar35,___xmm_00000000000000003ecccccd3ecccccd);
  fVar31 = (float)roundf();
  fVar11 = (float)roundf();
  puVar5 = &local_160;
  pppppppuVar4 = &local_1c0;
  if (4 < local_160) {
    puVar5 = &uStack_1b8;
    pppppppuVar4 = (undefined8 *******)local_1c0;
  }
  puVar8 = &local_160;
  uVar3 = 4;
  if (4 < local_160) {
    puVar8 = &uStack_1b8;
    uVar3 = local_160;
  }
  uVar7 = *puVar8;
  if (*puVar8 == uVar3) {
    smallvec::SmallVec::
    reserve_one_unchecked<array_<system_wall_constructor::plaster_systems::WallSide,4>_>();
    pppppppuVar4 = (undefined8 *******)local_1c0;
    puVar5 = &uStack_1b8;
    uVar7 = uStack_1b8;
  }
  fVar24 = 0.0;
  if (0.0 <= fVar11) {
    fVar24 = fVar11;
  }
  fVar11 = 65535.0;
  if (fVar24 <= 65535.0) {
    fVar11 = fVar24;
  }
  fVar24 = 0.0;
  if (0.0 <= fVar31) {
    fVar24 = fVar31;
  }
  fVar31 = 65535.0;
  if (fVar24 <= 65535.0) {
    fVar31 = fVar24;
  }
  auVar15._8_8_ = 0;
  auVar15._0_8_ =
       (ulonglong)(uint)(int)fVar31 & 0xffffffff0000ffff | (ulonglong)(ushort)(int)fVar11 << 0x10;
  auVar18 = psubusw(auVar41,auVar15);
  *(float *)(pppppppuVar4 + uVar7 * 3) = fVar25;
  *(float *)((longlong)pppppppuVar4 + uVar7 * 0x18 + 4) = fVar26;
  *(float *)(pppppppuVar4 + uVar7 * 3 + 1) = fVar21;
  *(float *)((longlong)pppppppuVar4 + uVar7 * 0x18 + 0xc) = fVar12;
  *(uint *)(pppppppuVar4 + uVar7 * 3 + 2) =
       CONCAT22(auVar18._2_2_ + (ushort)(int)fVar11,
                auVar18._0_2_ + (short)((ulonglong)(uint)(int)fVar31 & 0xffffffff0000ffff));
  *(undefined1 *)((longlong)pppppppuVar4 + uVar7 * 0x18 + 0x14) = uVar9;
  *puVar5 = *puVar5 + 1;
  ::utils::geometry::rectangle::Rectangle2d::get_point_u();
  fVar25 = extraout_XMM0_Da_10 + 1e-07;
  local_f4[0] = fVar25;
  if ((fVar25 < 0.0) || (1.0 < fVar25)) {
    local_108 = local_f4;
    local_100 = core::fmt::float::impl$5::fmt;
    local_140 = &anon_95101f4d2e0328d894274322bc39dc37_3_llvm_14090887955549727255;
    local_138 = 1;
    local_120 = 0;
    local_130 = &local_108;
    local_128 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  if (NAN(fVar25)) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  ::utils::geometry::rectangle::Rectangle2d::get_point_u();
  fVar21 = local_10c;
  fVar26 = extraout_XMM0_Da_11 + -1e-07;
  local_f4[0] = fVar26;
  if ((fVar26 < 0.0) || (1.0 < fVar26)) {
    local_108 = local_f4;
    local_100 = core::fmt::float::impl$5::fmt;
    local_140 = &anon_95101f4d2e0328d894274322bc39dc37_3_llvm_14090887955549727255;
    local_138 = 1;
    local_120 = 0;
    local_130 = &local_108;
    local_128 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  if (NAN(fVar26)) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  fVar12 = local_10c - *(float *)(in_RDX + 0x280);
  auVar32._0_4_ = (fVar26 - fVar25) * *(float *)(in_RDX + 0x30);
  auVar32._4_4_ = fVar12;
  auVar32._8_4_ = extraout_XMM0_Db_05;
  auVar32._12_4_ = 0;
  divps(auVar32,___xmm_00000000000000003ecccccd3ecccccd);
  fVar31 = (float)roundf();
  fVar11 = (float)roundf();
  puVar5 = &local_160;
  pppppppuVar4 = &local_1c0;
  if (4 < local_160) {
    puVar5 = &uStack_1b8;
    pppppppuVar4 = (undefined8 *******)local_1c0;
  }
  puVar8 = &local_160;
  uVar3 = 4;
  if (4 < local_160) {
    puVar8 = &uStack_1b8;
    uVar3 = local_160;
  }
  uVar7 = *puVar8;
  if (*puVar8 == uVar3) {
    smallvec::SmallVec::
    reserve_one_unchecked<array_<system_wall_constructor::plaster_systems::WallSide,4>_>();
    pppppppuVar4 = (undefined8 *******)local_1c0;
    puVar5 = &uStack_1b8;
    uVar7 = uStack_1b8;
    fVar21 = local_10c;
  }
  fVar24 = 0.0;
  if (0.0 <= fVar11) {
    fVar24 = fVar11;
  }
  fVar11 = 65535.0;
  if (fVar24 <= 65535.0) {
    fVar11 = fVar24;
  }
  fVar24 = 0.0;
  if (0.0 <= fVar31) {
    fVar24 = fVar31;
  }
  fVar31 = 65535.0;
  if (fVar24 <= 65535.0) {
    fVar31 = fVar24;
  }
  auVar16._8_8_ = 0;
  auVar16._0_8_ =
       (ulonglong)(uint)(int)fVar31 & 0xffffffff0000ffff | (ulonglong)(ushort)(int)fVar11 << 0x10;
  auVar18 = psubusw(auVar41,auVar16);
  *(float *)(pppppppuVar4 + uVar7 * 3) = fVar25;
  *(float *)((longlong)pppppppuVar4 + uVar7 * 0x18 + 4) = fVar26;
  *(float *)(pppppppuVar4 + uVar7 * 3 + 1) = fVar21;
  *(float *)((longlong)pppppppuVar4 + uVar7 * 0x18 + 0xc) = fVar12;
  *(uint *)(pppppppuVar4 + uVar7 * 3 + 2) =
       CONCAT22(auVar18._2_2_ + (ushort)(int)fVar11,
                auVar18._0_2_ + (short)((ulonglong)(uint)(int)fVar31 & 0xffffffff0000ffff));
  *(undefined1 *)((longlong)pppppppuVar4 + uVar7 * 0x18 + 0x14) = uVar10;
  *puVar5 = *puVar5 + 1;
  ::utils::geometry::rectangle::Rectangle2d::get_point_u();
  fVar25 = extraout_XMM0_Da_12 + 1e-07;
  local_f4[0] = fVar25;
  if ((fVar25 < 0.0) || (1.0 < fVar25)) {
    local_108 = local_f4;
    local_100 = core::fmt::float::impl$5::fmt;
    local_140 = &anon_95101f4d2e0328d894274322bc39dc37_3_llvm_14090887955549727255;
    local_138 = 1;
    local_120 = 0;
    local_130 = &local_108;
    local_128 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  if (NAN(fVar25)) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  ::utils::geometry::rectangle::Rectangle2d::get_point_u();
  fVar21 = extraout_XMM0_Da_13 + -1e-07;
  local_f4[0] = fVar21;
  if ((0.0 <= fVar21) && (fVar21 <= 1.0)) {
    if (!NAN(fVar21)) {
      fVar11 = fVar19 - *(float *)(in_RDX + 0x280);
      auVar27._4_4_ = extraout_XMM0_Db_06;
      auVar27._0_4_ = fVar21;
      auVar27._8_4_ = extraout_XMM0_Dc_01;
      auVar27._12_4_ = extraout_XMM0_Dd_01;
      auVar28._4_12_ = auVar27._4_12_;
      auVar28._0_4_ = (fVar21 - fVar25) * *(float *)(in_RDX + 0x30);
      auVar30._0_8_ = auVar28._0_8_;
      auVar30._8_4_ = extraout_XMM0_Db_06;
      auVar30._12_4_ = uVar20;
      auVar29._8_8_ = auVar30._8_8_;
      auVar29._4_4_ = fVar11;
      auVar29._0_4_ = auVar28._0_4_;
      divps(auVar29,___xmm_00000000000000003ecccccd3ecccccd);
      fVar26 = (float)roundf();
      fVar31 = (float)roundf();
      puVar5 = &local_160;
      pppppppuVar4 = &local_1c0;
      if (4 < local_160) {
        puVar5 = &uStack_1b8;
        pppppppuVar4 = (undefined8 *******)local_1c0;
      }
      puVar8 = &local_160;
      uVar3 = 4;
      if (4 < local_160) {
        puVar8 = &uStack_1b8;
        uVar3 = local_160;
      }
      uVar7 = *puVar8;
      if (*puVar8 == uVar3) {
        smallvec::SmallVec::
        reserve_one_unchecked<array_<system_wall_constructor::plaster_systems::WallSide,4>_>();
        puVar5 = &uStack_1b8;
        uVar7 = uStack_1b8;
        pppppppuVar4 = (undefined8 *******)local_1c0;
      }
      fVar12 = 0.0;
      if (0.0 <= fVar31) {
        fVar12 = fVar31;
      }
      fVar31 = 65535.0;
      if (fVar12 <= 65535.0) {
        fVar31 = fVar12;
      }
      fVar12 = 0.0;
      if (0.0 <= fVar26) {
        fVar12 = fVar26;
      }
      fVar26 = 65535.0;
      if (fVar12 <= 65535.0) {
        fVar26 = fVar12;
      }
      auVar17._8_8_ = 0;
      auVar17._0_8_ =
           (ulonglong)(uint)(int)fVar26 & 0xffffffff0000ffff |
           (ulonglong)(ushort)(int)fVar31 << 0x10;
      auVar41 = psubusw(auVar41,auVar17);
      *(float *)(pppppppuVar4 + uVar7 * 3) = fVar25;
      *(float *)((longlong)pppppppuVar4 + uVar7 * 0x18 + 4) = fVar21;
      *(float *)(pppppppuVar4 + uVar7 * 3 + 1) = fVar19;
      *(float *)((longlong)pppppppuVar4 + uVar7 * 0x18 + 0xc) = fVar11;
      *(uint *)(pppppppuVar4 + uVar7 * 3 + 2) =
           CONCAT22(auVar41._2_2_ + (ushort)(int)fVar31,
                    auVar41._0_2_ + (short)((ulonglong)(uint)(int)fVar26 & 0xffffffff0000ffff));
      *(undefined1 *)((longlong)pppppppuVar4 + uVar7 * 0x18 + 0x14) = uVar9;
      *puVar5 = *puVar5 + 1;
      in_RCX[0xc] = local_160;
      in_RCX[10] = local_170;
      in_RCX[0xb] = lStack_168;
      in_RCX[8] = local_180;
      in_RCX[9] = lStack_178;
      in_RCX[6] = local_190;
      in_RCX[7] = lStack_188;
      in_RCX[4] = local_1a0;
      in_RCX[5] = lStack_198;
      in_RCX[2] = local_1b0;
      in_RCX[3] = lStack_1a8;
      *in_RCX = (longlong)local_1c0;
      in_RCX[1] = uStack_1b8;
                    // WARNING: Read-only address (ram,0x0001429cd2c0) is written
                    // WARNING: Read-only address (ram,0x000142a7cd50) is written
      return;
    }
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  local_108 = local_f4;
  local_100 = core::fmt::float::impl$5::fmt;
  local_140 = &anon_95101f4d2e0328d894274322bc39dc37_3_llvm_14090887955549727255;
  local_138 = 1;
  local_120 = 0;
  local_130 = &local_108;
  local_128 = 1;
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}

