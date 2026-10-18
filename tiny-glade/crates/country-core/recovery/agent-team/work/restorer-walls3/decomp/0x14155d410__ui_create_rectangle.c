// ui_create_rectangle  size=3134
// void __cdecl ui_create_rectangle(void)

// WARNING: Control flow encountered bad instruction data
// WARNING: Instruction at (ram,0x000141560dfd) overlaps instruction at (ram,0x000141560dfa)
// 
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl
country_core::systems::wall::shapes::rectangle::ui_create_rectangle::ui_create_rectangle(void)

{
  undefined8 *puVar1;
  undefined4 uVar2;
  undefined4 *puVar3;
  undefined8 uVar4;
  byte bVar5;
  char extraout_AL;
  char extraout_AL_00;
  uint extraout_EAX;
  longlong extraout_RAX;
  ulonglong uVar6;
  undefined *puVar7;
  longlong lVar8;
  ulonglong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong lVar9;
  undefined8 *in_RCX;
  longlong lVar10;
  longlong lVar11;
  undefined *puVar12;
  char *pcVar13;
  undefined *puVar14;
  longlong *plVar15;
  char cVar16;
  longlong unaff_RBX;
  undefined7 uVar18;
  undefined *puVar17;
  undefined **ppuVar19;
  byte *pbVar20;
  byte *pbVar21;
  longlong lVar22;
  undefined8 uVar23;
  undefined8 *in_R8;
  ulonglong uVar24;
  byte *pbVar25;
  byte *pbVar26;
  undefined4 *puVar27;
  ulonglong uVar28;
  ulonglong uVar29;
  float *pfVar30;
  byte *pbVar31;
  bool bVar32;
  bool bVar33;
  bool bVar34;
  bool bVar35;
  float fVar36;
  undefined4 extraout_XMM0_Da;
  undefined4 extraout_XMM0_Da_00;
  undefined4 extraout_XMM0_Da_01;
  undefined4 extraout_XMM0_Da_02;
  undefined4 extraout_XMM0_Da_03;
  float fVar37;
  undefined1 auVar38 [16];
  undefined1 auVar39 [16];
  undefined4 uVar40;
  float fVar41;
  float fVar42;
  ulonglong uVar43;
  float fVar46;
  undefined1 auVar44 [16];
  undefined1 auVar45 [16];
  longlong *in_stack_00000028;
  longlong *in_stack_00000038;
  longlong *in_stack_00000040;
  longlong *in_stack_00000048;
  undefined8 *in_stack_00000050;
  longlong *in_stack_00000058;
  undefined8 *in_stack_00000060;
  longlong *in_stack_00000068;
  longlong *in_stack_00000070;
  longlong *in_stack_00000078;
  longlong *in_stack_00000080;
  undefined *puStack_210;
  undefined1 auStack_208 [36];
  undefined4 local_1e4;
  byte *local_1e0;
  undefined7 local_1d8;
  undefined1 uStack_1d1;
  undefined4 uStack_1d0;
  undefined4 uStack_1cc;
  undefined3 uStack_1c8;
  undefined5 uStack_1c5;
  byte *local_1b8;
  uint local_1ac;
  longlong local_1a8;
  undefined4 *local_1a0;
  char *local_198;
  longlong local_190;
  float *local_188;
  uint *local_180;
  longlong local_178;
  undefined4 local_170;
  undefined4 local_16c;
  undefined4 local_168;
  float fStack_164;
  float fStack_160;
  float fStack_15c;
  float local_158;
  undefined4 uStack_154;
  float fStack_150;
  undefined4 uStack_14c;
  undefined4 local_148;
  undefined4 uStack_144;
  undefined4 local_140;
  undefined4 local_128;
  undefined4 local_110;
  undefined4 local_f8;
  undefined1 local_e0;
  undefined1 local_d0;
  undefined4 local_cc;
  undefined2 local_b0;
  undefined4 *local_a8;
  undefined4 *local_a0;
  undefined4 *local_98;
  longlong local_90;
  undefined8 local_88;
  undefined8 uStack_80;
  
  ppuVar19 = (undefined **)auStack_208;
  pbVar20 = (byte *)in_R8[4];
  local_180 = (uint *)in_R8[6];
  local_1ac = *(uint *)((longlong)in_R8 + 0x3c);
  puVar27 = (undefined4 *)(ulonglong)local_1ac;
  if ((*(int *)*in_RCX != 6) || ((char)((int *)*in_RCX)[1] != '\x02')) {
    *local_180 = local_1ac;
    pbVar20[0] = 0;
    pbVar20[1] = 0;
    pbVar20[2] = 0;
    pbVar20[3] = 0;
    return;
  }
  pbVar21 = (byte *)*in_R8;
  local_188 = (float *)in_stack_00000060[5];
  local_90 = in_stack_00000068[4];
  local_98 = (undefined4 *)in_stack_00000068[6];
  local_168 = *(undefined4 *)((longlong)in_stack_00000068 + 0x3c);
  lVar22 = *in_stack_00000078;
  local_178 = in_stack_00000078[4];
  local_a0 = (undefined4 *)in_stack_00000078[6];
  local_16c = *(undefined4 *)((longlong)in_stack_00000078 + 0x3c);
  local_1a0 = (undefined4 *)in_stack_00000078[8];
  local_a8 = (undefined4 *)in_stack_00000078[10];
  local_170 = *(undefined4 *)((longlong)in_stack_00000078 + 0x5c);
  local_158 = 0.0;
  uStack_154 = 0;
  fStack_150 = 1.12104e-44;
  uStack_14c = 0;
  local_148 = 0;
  uStack_144 = 0;
  local_140 = 0;
  local_128 = 0;
  local_110 = 0;
  local_f8 = 0;
  local_e0 = 0;
  local_d0 = 5;
  local_cc = 0x7f7fffff;
  local_b0 = 0x100;
  lVar9 = in_stack_00000068[8];
  pbVar31 = (byte *)in_stack_00000068[10];
  local_1e4 = *(undefined4 *)((longlong)in_stack_00000068 + 0x5c);
  pcVar13 = "wall_rectangle_canvascursor_rectangle_";
  puStack_210 = (undefined *)0x14155d59e;
  systems::ui::resources::ui_state::UiState::add<ref_<str_>_>();
  local_198 = pcVar13;
  local_190 = extraout_RAX;
  if (60.0 <= *(float *)(pbVar21 + 0x3c)) {
    local_148 = (undefined4)*(undefined8 *)(pbVar20 + 0x20);
    uStack_144 = (undefined4)((ulonglong)*(undefined8 *)(pbVar20 + 0x20) >> 0x20);
    local_158 = (float)*(undefined8 *)(pbVar20 + 0x10);
    uStack_154 = (undefined4)((ulonglong)*(undefined8 *)(pbVar20 + 0x10) >> 0x20);
    fStack_150 = (float)*(undefined8 *)(pbVar20 + 0x18);
    uStack_14c = (undefined4)((ulonglong)*(undefined8 *)(pbVar20 + 0x18) >> 0x20);
    *(undefined4 *)in_stack_00000080[2] = *(undefined4 *)((longlong)in_stack_00000080 + 0x1c);
    unaff_RBX = *in_stack_00000080;
    lVar8 = *(longlong *)(unaff_RBX + 0x30);
    uVar23 = *(undefined8 *)(unaff_RBX + 0x40);
    local_1e0 = pbVar21;
    if (lVar8 == *(longlong *)(unaff_RBX + 0x20)) {
      puStack_210 = (undefined *)0x14155d619;
      alloc::raw_vec::RawVec::
      grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                ();
    }
    lVar10 = *(longlong *)(unaff_RBX + 0x28);
    *(undefined8 *)(lVar10 + lVar8 * 0x28) = uVar23;
    *(undefined4 *)(lVar10 + 8 + lVar8 * 0x28) = 1;
    *(ulonglong *)(lVar10 + 0x1c + lVar8 * 0x28) = CONCAT44(uStack_144,local_148);
    puVar1 = (undefined8 *)(lVar10 + 0xc + lVar8 * 0x28);
    *puVar1 = CONCAT44(uStack_154,local_158);
    puVar1[1] = CONCAT44(uStack_14c,fStack_150);
    *(longlong *)(unaff_RBX + 0x30) = lVar8 + 1;
    *(longlong *)(unaff_RBX + 0x40) = *(longlong *)(unaff_RBX + 0x40) + 1;
    pbVar21 = local_1e0;
  }
  uVar18 = (undefined7)((ulonglong)unaff_RBX >> 8);
  cVar16 = '\x02';
  if (1 < *(byte *)(local_190 + 0x18)) {
    if ((*(char **)(local_190 + 0x38) == local_198) && (*(char *)(local_190 + 0x19) == '\x03')) {
      uVar40 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
      puVar27 = (undefined4 *)in_stack_00000038[2];
      *puVar27 = uVar40;
      puVar3 = (undefined4 *)*in_stack_00000038;
      *(undefined1 *)(puVar3 + 3) = 0;
      *(undefined8 *)(puVar3 + 1) = *(undefined8 *)*in_stack_00000050;
      *puVar27 = uVar40;
      *puVar3 = 0;
      *(undefined4 *)in_stack_00000058[2] = *(undefined4 *)((longlong)in_stack_00000058 + 0x1c);
      lVar8 = *in_stack_00000058;
      lVar10 = *(longlong *)(lVar8 + 0x30);
      uVar23 = *(undefined8 *)(lVar8 + 0x40);
      local_1e0 = pbVar21;
      local_1a8 = lVar22;
      if (lVar10 == *(longlong *)(lVar8 + 0x20)) {
        puStack_210 = (undefined *)0x14155d70c;
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                  ();
      }
      *(undefined8 *)(*(longlong *)(lVar8 + 0x28) + lVar10 * 8) = uVar23;
      *(longlong *)(lVar8 + 0x30) = lVar10 + 1;
      *(longlong *)(lVar8 + 0x40) = *(longlong *)(lVar8 + 0x40) + 1;
      pbVar21 = local_1e0;
      lVar22 = local_1a8;
      if (*(byte *)(local_190 + 0x18) < 2) goto LAB_14155d8f8;
    }
    if ((*(char **)(local_190 + 0x38) == local_198) && (*(char *)(local_190 + 0x19) == '\x04')) {
      fVar41 = (float)*(undefined8 *)(*in_stack_00000040 + 0x140);
      fVar36 = (float)((ulonglong)*(undefined8 *)(*in_stack_00000040 + 0x140) >> 0x20);
      uVar40 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
      puVar3 = (undefined4 *)in_stack_00000038[2];
      *puVar3 = uVar40;
      pfVar30 = (float *)*in_stack_00000038;
      fVar36 = SQRT(fVar36 * fVar36 + fVar41 * fVar41) + *pfVar30;
      *pfVar30 = fVar36;
      if (4.0 < fVar36) {
        *puVar3 = uVar40;
        *(undefined1 *)(pfVar30 + 3) = 1;
      }
      *(undefined4 *)in_stack_00000058[2] = *(undefined4 *)((longlong)in_stack_00000058 + 0x1c);
      lVar8 = *in_stack_00000058;
      lVar10 = *(longlong *)(lVar8 + 0x30);
      uVar23 = *(undefined8 *)(lVar8 + 0x40);
      local_1e0 = pbVar21;
      local_1b8 = pbVar31;
      local_1a8 = lVar22;
      if (lVar10 == *(longlong *)(lVar8 + 0x20)) {
        puStack_210 = (undefined *)0x14155d7f5;
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                  ();
      }
      *(undefined8 *)(*(longlong *)(lVar8 + 0x28) + lVar10 * 8) = uVar23;
      *(longlong *)(lVar8 + 0x30) = lVar10 + 1;
      *(longlong *)(lVar8 + 0x40) = *(longlong *)(lVar8 + 0x40) + 1;
      pbVar21 = local_1e0;
      pbVar31 = local_1b8;
      if (*(byte *)(local_190 + 0x18) < 2) goto LAB_14155d8f8;
    }
    if ((*(char **)(local_190 + 0x38) == local_198) && (*(byte *)(local_190 + 0x19) < 2)) {
      uVar23 = *(undefined8 *)(*in_stack_00000038 + 4);
      *(undefined4 *)in_stack_00000048[2] = *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
      lVar22 = *in_stack_00000048;
      lVar8 = *(longlong *)(lVar22 + 0x30);
      uVar4 = *(undefined8 *)(lVar22 + 0x40);
      local_1e0 = pbVar21;
      local_1b8 = pbVar20;
      local_1a8 = lVar9;
      if (lVar8 == *(longlong *)(lVar22 + 0x20)) {
        puStack_210 = (undefined *)0x14155d89d;
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      lVar9 = *(longlong *)(lVar22 + 0x28);
      *(undefined8 *)(lVar9 + lVar8 * 0x18) = uVar4;
      *(undefined4 *)(lVar9 + 8 + lVar8 * 0x18) = 2;
      *(undefined8 *)(lVar9 + 0xc + lVar8 * 0x18) = uVar23;
      *(longlong *)(lVar22 + 0x30) = lVar8 + 1;
      *(longlong *)(lVar22 + 0x40) = *(longlong *)(lVar22 + 0x40) + 1;
      pbVar20 = local_1b8;
      pbVar21 = local_1e0;
      lVar9 = local_1a8;
      if (*(byte *)(local_190 + 0x18) < 2) goto LAB_14155d8f8;
    }
    if (*(char **)(local_190 + 0x38) == local_198) {
      cVar16 = *(char *)(local_190 + 0x19);
      uVar18 = 0;
    }
  }
LAB_14155d8f8:
  bVar5 = 3;
  if ((byte)(cVar16 - 2U) < 3) {
    bVar5 = cVar16 - 2U;
  }
  if (bVar5 < 2) {
    if (pbVar21[0x2c] == 2) {
      puStack_210 = (undefined *)0x14155d92f;
      ::utils::camera::CameraMatrices::eye_right();
      fVar36 = 1.0 / SQRT(fStack_150 * fStack_150 + local_158 * local_158);
      uVar23 = CONCAT44(fVar36 * fStack_150,fVar36 * local_158);
    }
    else {
      fVar36 = *(float *)(pbVar21 + 0x18);
      fVar41 = *(float *)(pbVar21 + 0x1c);
      fVar37 = 1.0 / SQRT(fVar36 * fVar36 + fVar41 * fVar41);
      uVar23 = CONCAT44(fVar37 * fVar36,fVar37 * -fVar41);
    }
    *local_180 = local_1ac;
    *(undefined8 *)(pbVar20 + 0x10) = uVar23;
    uVar40 = *(undefined4 *)(pbVar21 + 0x30);
    uVar2 = *(undefined4 *)(pbVar21 + 0x38);
    *local_180 = local_1ac;
    *(undefined4 *)(pbVar20 + 0x18) = uVar40;
    *(undefined4 *)(pbVar20 + 0x1c) = uVar2;
    uVar40 = *(undefined4 *)(pbVar21 + 0x34);
    *local_180 = local_1ac;
    *(undefined4 *)(pbVar20 + 0x28) = uVar40;
    uStack_1c8 = (undefined3)*(undefined8 *)(pbVar20 + 0x20);
    uStack_1c5 = (undefined5)((ulonglong)*(undefined8 *)(pbVar20 + 0x20) >> 0x18);
    local_1d8 = (undefined7)*(undefined8 *)(pbVar20 + 0x10);
    uStack_1d1 = (undefined1)((ulonglong)*(undefined8 *)(pbVar20 + 0x10) >> 0x38);
    uStack_1d0 = (undefined4)*(undefined8 *)(pbVar20 + 0x18);
    uStack_1cc = (undefined4)((ulonglong)*(undefined8 *)(pbVar20 + 0x18) >> 0x20);
    puVar17 = (undefined *)CONCAT71(uVar18,1);
    puStack_210 = (undefined *)0x14155db4c;
    rectangle_signifier();
    puStack_210 = (undefined *)0x14155db57;
    append_only_vec::AppendOnlyVec::
    push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>();
    uVar43 = 0;
  }
  else if (bVar5 == 2) {
    uVar43 = 0;
    puVar17 = (undefined *)CONCAT71(uVar18,1);
    if (*(char *)(*in_stack_00000038 + 0xc) != '\0') {
      fVar36 = (*(float *)(pbVar21 + 0x34) - local_188[1]) / local_188[4];
      fVar41 = (local_188[3] * fVar36 + *local_188) - *(float *)(pbVar20 + 0x18);
      fVar36 = (fVar36 * local_188[5] + local_188[2]) - *(float *)(pbVar20 + 0x1c);
      uVar23 = *(undefined8 *)(pbVar20 + 0x20);
      fVar37 = (float)*(undefined8 *)(pbVar20 + 0x10);
      fVar46 = (float)((ulonglong)*(undefined8 *)(pbVar20 + 0x10) >> 0x20);
      fVar42 = ABS(fVar46 * fVar36 + fVar41 * fVar37);
      fVar36 = ABS(fVar37 * fVar36 - fVar41 * fVar46);
      auVar44._0_4_ = fVar42 + fVar42;
      auVar44._4_4_ = fVar36 + fVar36;
      auVar44._8_8_ = 0;
      auVar38 = maxps(___xmm_00000000000000004010000040100000,auVar44);
      auVar39._0_4_ = (uint)__xmm_00000000000000004010000040100000 & -(uint)NAN(auVar44._0_4_);
      auVar39._4_4_ = __xmm_00000000000000004010000040100000._4_4_ & -(uint)NAN(auVar44._4_4_);
      auVar39._8_8_ = 0;
      auVar45._0_4_ = ~-(uint)NAN(auVar44._0_4_) & auVar38._0_4_;
      auVar45._4_4_ = ~-(uint)NAN(auVar44._4_4_) & auVar38._4_4_;
      auVar45._8_4_ = auVar38._8_4_;
      auVar45._12_4_ = auVar38._12_4_;
      auVar45 = auVar45 | auVar39;
      auVar39 = minps(___xmm_00000000000000004170000041700000,auVar45);
      fVar36 = (float)(~-(uint)NAN(auVar45._0_4_) & auVar39._0_4_ |
                      (uint)__xmm_00000000000000004170000041700000 & -(uint)NAN(auVar45._0_4_));
      fVar41 = (float)(~-(uint)NAN(auVar45._4_4_) & auVar39._4_4_ |
                      __xmm_00000000000000004170000041700000._4_4_ & -(uint)NAN(auVar45._4_4_));
      *(float *)(pbVar20 + 0x20) = fVar36;
      uVar43 = CONCAT44(fVar41 - (float)((ulonglong)uVar23 >> 0x20),fVar36 - (float)uVar23) &
               0x7fffffff7fffffff;
      *local_180 = local_1ac;
      *(float *)(pbVar20 + 0x24) = fVar41;
    }
    uStack_1c8 = (undefined3)*(undefined8 *)(pbVar20 + 0x20);
    uStack_1c5 = (undefined5)((ulonglong)*(undefined8 *)(pbVar20 + 0x20) >> 0x18);
    local_1d8 = (undefined7)*(undefined8 *)(pbVar20 + 0x10);
    uStack_1d1 = (undefined1)((ulonglong)*(undefined8 *)(pbVar20 + 0x10) >> 0x38);
    uStack_1d0 = (undefined4)*(undefined8 *)(pbVar20 + 0x18);
    uStack_1cc = (undefined4)((ulonglong)*(undefined8 *)(pbVar20 + 0x18) >> 0x20);
    puStack_210 = (undefined *)0x14155da9f;
    rectangle_signifier();
    puStack_210 = (undefined *)0x14155daaa;
    append_only_vec::AppendOnlyVec::
    push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>();
  }
  else {
    uVar43 = 0;
    puVar17 = (undefined *)CONCAT71(uVar18,1);
    if (*(char *)(*in_stack_00000038 + 0xc) == '\0') {
      uStack_144 = (undefined4)*(undefined8 *)(pbVar20 + 0x20);
      local_140 = (undefined4)((ulonglong)*(undefined8 *)(pbVar20 + 0x20) >> 0x20);
      uStack_154 = (undefined4)*(undefined8 *)(pbVar20 + 0x10);
      fStack_150 = (float)((ulonglong)*(undefined8 *)(pbVar20 + 0x10) >> 0x20);
      uStack_14c = (undefined4)*(undefined8 *)(pbVar20 + 0x18);
      local_148 = (undefined4)((ulonglong)*(undefined8 *)(pbVar20 + 0x18) >> 0x20);
      local_158 = 1.4013e-45;
      puStack_210 = (undefined *)0x14155dde7;
      resources::glade_border::GladeBorder::is_shape_inside();
      if (extraout_AL_00 == '\0') {
        *local_a8 = local_170;
        lVar22 = *(longlong *)(local_1a0 + 0xc);
        uVar23 = *(undefined8 *)(local_1a0 + 0x10);
        if (lVar22 == *(longlong *)(local_1a0 + 8)) {
          puStack_210 = (undefined *)0x14155e12c;
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                    ();
        }
        *(undefined8 *)(*(longlong *)(local_1a0 + 10) + lVar22 * 8) = uVar23;
        *(longlong *)(local_1a0 + 0xc) = lVar22 + 1;
        *(longlong *)(local_1a0 + 0x10) = *(longlong *)(local_1a0 + 0x10) + 1;
        puVar17 = (undefined *)0x0;
      }
      else {
        pbVar26 = pbVar20 + 0x10;
        if (pbVar21[0x2c] != 2) {
          puVar27 = *(undefined4 **)(pbVar21 + 0x20);
        }
        pbVar25 = (byte *)(ulonglong)(pbVar21[0x2c] != 2);
        uVar40 = *(undefined4 *)(pbVar21 + 0x40);
        uVar2 = *(undefined4 *)(pbVar21 + 0x3c);
        local_148 = (undefined4)*(undefined8 *)(pbVar20 + 0x20);
        uStack_144 = (undefined4)((ulonglong)*(undefined8 *)(pbVar20 + 0x20) >> 0x20);
        local_158 = (float)*(undefined8 *)pbVar26;
        uStack_154 = (undefined4)((ulonglong)*(undefined8 *)pbVar26 >> 0x20);
        fStack_150 = (float)*(undefined8 *)(pbVar20 + 0x18);
        uStack_14c = (undefined4)((ulonglong)*(undefined8 *)(pbVar20 + 0x18) >> 0x20);
        *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
        lVar22 = *in_stack_00000028;
        lVar8 = *(longlong *)(lVar22 + 0x30);
        pfVar30 = *(float **)(lVar22 + 0x40);
        if (lVar8 == *(longlong *)(lVar22 + 0x20)) {
          puStack_210 = (undefined *)0x14155e332;
          local_1e0 = pbVar26;
          local_1b8 = pbVar25;
          local_1a0 = puVar27;
          local_188 = *(float **)(lVar22 + 0x40);
          alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>();
          pbVar26 = local_1e0;
          pbVar25 = local_1b8;
          puVar27 = local_1a0;
          pfVar30 = local_188;
        }
        lVar10 = *(longlong *)(lVar22 + 0x28);
        lVar11 = lVar8 * 0x38;
        *(byte **)(lVar10 + lVar11) = pbVar25;
        *(undefined4 **)(lVar10 + 8 + lVar11) = puVar27;
        puVar1 = (undefined8 *)(lVar10 + 0x10 + lVar11);
        *puVar1 = CONCAT44(uStack_154,local_158);
        puVar1[1] = CONCAT44(uStack_14c,fStack_150);
        *(ulonglong *)(lVar10 + 0x20 + lVar11) = CONCAT44(uStack_144,local_148);
        *(undefined4 *)(lVar10 + 0x28 + lVar11) = uVar40;
        *(undefined4 *)(lVar10 + 0x2c + lVar11) = uVar2;
        *(float **)(lVar10 + 0x30 + lVar11) = pfVar30;
        *(longlong *)(lVar22 + 0x30) = lVar8 + 1;
        *(longlong *)(lVar22 + 0x40) = *(longlong *)(lVar22 + 0x40) + 1;
        uStack_1c8 = (undefined3)((uint)*(undefined4 *)(pbVar21 + 0x38) >> 8);
        uVar23 = *(undefined8 *)(pbVar21 + 0x30);
        uStack_1d1 = (undefined1)uVar23;
        uStack_1d0 = (undefined4)((ulonglong)uVar23 >> 8);
        uStack_1cc._0_3_ = (undefined3)((ulonglong)uVar23 >> 0x28);
        uStack_1cc = CONCAT13((char)*(undefined4 *)(pbVar21 + 0x38),(undefined3)uStack_1cc);
        uStack_144 = (undefined4)*(undefined8 *)(pbVar26 + 0x10);
        local_140 = (undefined4)((ulonglong)*(undefined8 *)(pbVar26 + 0x10) >> 0x20);
        uStack_154 = (undefined4)*(undefined8 *)pbVar26;
        fStack_150 = (float)((ulonglong)*(undefined8 *)pbVar26 >> 0x20);
        uStack_14c = (undefined4)*(undefined8 *)(pbVar26 + 8);
        local_148 = (undefined4)((ulonglong)*(undefined8 *)(pbVar26 + 8) >> 0x20);
        local_158 = 1.4013e-45;
        puStack_210 = (undefined *)0x14155e3d3;
        ::utils::geometry::rectangle::Rectangle2d::area();
        puStack_210 = (undefined *)0x14155e3ec;
        ::utils::inverse_lerp();
        *(undefined4 *)pbVar31 = local_1e4;
        lVar22 = *(longlong *)(lVar9 + 0x30);
        uVar23 = *(undefined8 *)(lVar9 + 0x40);
        if (lVar22 == *(longlong *)(lVar9 + 0x20)) {
          puStack_210 = (undefined *)0x14155e419;
          alloc::raw_vec::RawVec::
          grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                    ();
        }
        lVar8 = *(longlong *)(lVar9 + 0x28);
        lVar10 = lVar22 * 0x20;
        *(undefined1 *)(lVar8 + lVar10) = 7;
        puVar1 = (undefined8 *)(lVar8 + 1 + lVar10);
        *puVar1 = CONCAT17(uStack_1d1,local_1d8);
        puVar1[1] = CONCAT44(uStack_1cc,uStack_1d0);
        *(uint *)(lVar8 + 0x10 + lVar10) = CONCAT31(uStack_1c8,uStack_1cc._3_1_);
        *(undefined4 *)(lVar8 + 0x14 + lVar10) = extraout_XMM0_Da_03;
        *(undefined8 *)(lVar8 + 0x18 + lVar10) = uVar23;
        *(longlong *)(lVar9 + 0x30) = lVar22 + 1;
        *(longlong *)(lVar9 + 0x40) = *(longlong *)(lVar9 + 0x40) + 1;
      }
    }
  }
  uStack_144 = (undefined4)*(undefined8 *)(pbVar20 + 0x20);
  local_140 = (undefined4)((ulonglong)*(undefined8 *)(pbVar20 + 0x20) >> 0x20);
  uStack_154 = (undefined4)*(undefined8 *)(pbVar20 + 0x10);
  fStack_150 = (float)((ulonglong)*(undefined8 *)(pbVar20 + 0x10) >> 0x20);
  uStack_14c = (undefined4)*(undefined8 *)(pbVar20 + 0x18);
  local_148 = (undefined4)((ulonglong)*(undefined8 *)(pbVar20 + 0x18) >> 0x20);
  local_158 = 1.4013e-45;
  puStack_210 = (undefined *)0x14155db8d;
  resources::glade_border::GladeBorder::is_shape_inside();
  if (extraout_AL == '\0') {
    *local_a0 = local_16c;
    lVar22 = *(longlong *)(local_178 + 0x30);
    uVar23 = *(undefined8 *)(local_178 + 0x40);
    if (lVar22 == *(longlong *)(local_178 + 0x20)) {
      puStack_210 = (undefined *)0x14155dbd0;
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                ();
    }
    *(undefined8 *)(*(longlong *)(local_178 + 0x28) + lVar22 * 8) = uVar23;
    *(longlong *)(local_178 + 0x30) = lVar22 + 1;
    *(longlong *)(local_178 + 0x40) = *(longlong *)(local_178 + 0x40) + 1;
  }
  pbVar26 = pbVar20 + 0x10;
  if ((char)puVar17 != '\0') {
    local_88 = *(undefined8 *)pbVar26;
    uStack_80 = *(undefined8 *)(pbVar20 + 0x18);
    fVar36 = (float)*(undefined8 *)(pbVar20 + 0x20) + 0.56;
    fVar41 = (float)((ulonglong)*(undefined8 *)(pbVar20 + 0x20) >> 0x20) + 0.56;
    cVar16 = '\0';
    if ((char)*(undefined4 *)(pbVar21 + 0x2c) != '\x02') {
      cVar16 = (char)*(undefined4 *)(pbVar21 + 0x2c);
    }
    local_158 = (float)*(undefined8 *)pbVar26;
    uStack_154 = (undefined4)((ulonglong)*(undefined8 *)pbVar26 >> 0x20);
    fStack_150 = (float)*(undefined8 *)(pbVar20 + 0x18);
    uStack_14c = (undefined4)((ulonglong)*(undefined8 *)(pbVar20 + 0x18) >> 0x20);
    *(undefined4 *)in_stack_00000068[2] = *(undefined4 *)((longlong)in_stack_00000068 + 0x1c);
    lVar22 = *in_stack_00000068;
    lVar8 = *(longlong *)(lVar22 + 0x30);
    uVar23 = *(undefined8 *)(lVar22 + 0x40);
    local_1b8 = pbVar31;
    if (lVar8 == *(longlong *)(lVar22 + 0x20)) {
      puStack_210 = (undefined *)0x14155dc62;
      alloc::raw_vec::RawVec::
      grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                ();
    }
    lVar11 = local_90;
    lVar10 = *(longlong *)(lVar22 + 0x28);
    *(undefined4 *)(lVar10 + lVar8 * 0x28) = 1;
    puVar1 = (undefined8 *)(lVar10 + 4 + lVar8 * 0x28);
    *puVar1 = CONCAT44(uStack_154,local_158);
    puVar1[1] = CONCAT44(uStack_14c,fStack_150);
    *(ulonglong *)(lVar10 + 0x14 + lVar8 * 0x28) = CONCAT44(fVar41,fVar36);
    *(char *)(lVar10 + 0x1c + lVar8 * 0x28) = cVar16;
    *(undefined8 *)(lVar10 + 0x20 + lVar8 * 0x28) = uVar23;
    *(longlong *)(lVar22 + 0x30) = lVar8 + 1;
    *(longlong *)(lVar22 + 0x40) = *(longlong *)(lVar22 + 0x40) + 1;
    uVar40 = *(undefined4 *)(pbVar21 + 0x3c);
    *local_98 = local_168;
    lVar22 = *(longlong *)(local_90 + 0x30);
    puVar17 = *(undefined **)(local_90 + 0x40);
    if (lVar22 == *(longlong *)(local_90 + 0x20)) {
      puStack_210 = (undefined *)0x14155dcd4;
      alloc::raw_vec::RawVec::
      grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>();
    }
    lVar8 = *(longlong *)(lVar11 + 0x28);
    lVar10 = lVar22 * 0x30;
    *(undefined4 *)(lVar8 + lVar10) = 1;
    puVar1 = (undefined8 *)(lVar8 + 4 + lVar10);
    *puVar1 = local_88;
    puVar1[1] = uStack_80;
    *(ulonglong *)(lVar8 + 0x14 + lVar10) = CONCAT44(fVar41,fVar36);
    *(undefined4 *)(lVar8 + 0x1c + lVar10) = uVar40;
    *(undefined4 *)(lVar8 + 0x20 + lVar10) = 0x40800000;
    *(undefined1 *)(lVar8 + 0x24 + lVar10) = 0;
    *(undefined **)(lVar8 + 0x28 + lVar10) = puVar17;
    *(longlong *)(lVar11 + 0x30) = lVar22 + 1;
    *(longlong *)(lVar11 + 0x40) = *(longlong *)(lVar11 + 0x40) + 1;
    pbVar31 = local_1b8;
  }
  lVar22 = in_stack_00000060[1];
  uVar28 = *(ulonglong *)*in_stack_00000060;
  uVar6 = 0;
  uVar29 = uVar28 - *(ulonglong *)(lVar22 + 0x18);
  if (uVar28 < *(ulonglong *)(lVar22 + 0x18)) {
    uVar29 = uVar6;
  }
  uVar24 = uVar28 - *(ulonglong *)(lVar22 + 0x38);
  if (uVar28 < *(ulonglong *)(lVar22 + 0x38)) {
    uVar24 = uVar6;
  }
  bVar32 = *(ulonglong *)(lVar22 + 0x30) < uVar24;
  uVar28 = *(ulonglong *)(lVar22 + 0x30) - uVar24;
  bVar34 = uVar28 == 0;
  if (bVar32) {
    uVar28 = uVar6;
  }
  pbVar25 = (byte *)CONCAT71((int7)(uVar24 >> 8),bVar32 || bVar34);
  bVar33 = *(ulonglong *)(lVar22 + 0x10) < uVar29;
  uVar29 = *(ulonglong *)(lVar22 + 0x10) - uVar29;
  bVar35 = uVar29 == 0;
  if (bVar33) {
    uVar29 = uVar6;
  }
  puVar12 = (undefined *)
            CONCAT71((int7)((ulonglong)lVar22 >> 8),(bVar33 || bVar35) && (bVar32 || bVar34));
  uVar28 = uVar29 + uVar28;
  if (!bVar33 && !bVar35 || !bVar32 && !bVar34) {
    uVar28 = uVar6;
  }
  uVar28 = *(longlong *)(lVar22 + 0x40) - uVar28;
  *(ulonglong *)*in_stack_00000060 = uVar28;
  if (1 < *(byte *)(local_190 + 0x18)) {
    if (*(char **)(local_190 + 0x38) == local_198) {
      uVar6 = (ulonglong)(byte)(*(char *)(local_190 + 0x19) - 2);
    }
    else {
      uVar6 = 0;
    }
  }
  bVar5 = 3;
  if ((byte)uVar6 < 3) {
    bVar5 = (byte)uVar6;
  }
  puVar14 = &DAT_142c6e864;
  puVar7 = &DAT_142c6e864 + *(int *)(&DAT_142c6e864 + (ulonglong)bVar5 * 4);
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(bVar5) {
  case 0:
    if ((bVar33 || bVar35) && (bVar32 || bVar34)) {
      return;
    }
    puStack_210 = &UNK_14155de5b;
    ::utils::swizzlers::impl$1::xvy();
    fVar36 = fStack_160;
    fVar41 = fStack_15c;
    fVar37 = fStack_164;
    if ((*pbVar20 & 1) != 0) {
      fVar36 = *(float *)(pbVar20 + 8);
      fVar41 = *(float *)(pbVar20 + 0xc);
      fVar37 = *(float *)(pbVar20 + 4);
    }
    fVar37 = fStack_164 - fVar37;
    fVar36 = fStack_160 - fVar36;
    fVar41 = fStack_15c - fVar41;
    fVar42 = *(float *)(*in_stack_00000070 + 0x28);
    fVar46 = *(float *)(pbVar21 + 0x3c);
    local_1d8 = (undefined7)*(undefined8 *)(pbVar21 + 0x30);
    uStack_1d1 = (undefined1)((ulonglong)*(undefined8 *)(pbVar21 + 0x30) >> 0x38);
    uStack_1d0 = *(undefined4 *)(pbVar21 + 0x38);
    uStack_144 = (undefined4)*(undefined8 *)(pbVar20 + 0x20);
    local_140 = (undefined4)((ulonglong)*(undefined8 *)(pbVar20 + 0x20) >> 0x20);
    uStack_154 = (undefined4)*(undefined8 *)pbVar26;
    fStack_150 = (float)((ulonglong)*(undefined8 *)pbVar26 >> 0x20);
    uStack_14c = (undefined4)*(undefined8 *)(pbVar20 + 0x18);
    local_148 = (undefined4)((ulonglong)*(undefined8 *)(pbVar20 + 0x18) >> 0x20);
    local_158 = 1.4013e-45;
    puStack_210 = &UNK_14155e1e2;
    ::utils::geometry::rectangle::Rectangle2d::area();
    puStack_210 = &UNK_14155e1fb;
    ::utils::inverse_lerp();
    *(undefined4 *)pbVar31 = local_1e4;
    lVar22 = *(longlong *)(lVar9 + 0x30);
    uVar23 = *(undefined8 *)(lVar9 + 0x40);
    if (lVar22 == *(longlong *)(lVar9 + 0x20)) {
      puStack_210 = &UNK_14155e228;
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    lVar8 = *(longlong *)(lVar9 + 0x28);
    lVar10 = lVar22 * 0x20;
    *(undefined1 *)(lVar8 + lVar10) = 3;
    *(byte *)(lVar8 + 1 + lVar10) = -(fVar46 == 0.0) & 1;
    *(float *)(lVar8 + 4 + lVar10) =
         SQRT(fVar41 * fVar41 + fVar36 * fVar36 + fVar37 * fVar37) / fVar42;
    *(ulonglong *)(lVar8 + 8 + lVar10) = CONCAT17(uStack_1d1,local_1d8);
    *(undefined4 *)(lVar8 + 0x10 + lVar10) = uStack_1d0;
    *(undefined4 *)(lVar8 + 0x14 + lVar10) = extraout_XMM0_Da_02;
    *(undefined8 *)(lVar8 + 0x18 + lVar10) = uVar23;
    *(longlong *)(lVar9 + 0x30) = lVar22 + 1;
    *(longlong *)(lVar9 + 0x40) = *(longlong *)(lVar9 + 0x40) + 1;
    *local_180 = local_1ac;
    pbVar20[0] = 1;
    pbVar20[1] = 0;
    pbVar20[2] = 0;
    pbVar20[3] = 0;
    *(float *)(pbVar20 + 0xc) = fStack_15c;
    *(ulonglong *)(pbVar20 + 4) = CONCAT44(fStack_160,fStack_164);
    return;
  case 1:
    uStack_1c8 = (undefined3)((uint)*(undefined4 *)(pbVar21 + 0x38) >> 8);
    uVar23 = *(undefined8 *)(pbVar21 + 0x30);
    uStack_1d1 = (undefined1)uVar23;
    uStack_1d0 = (undefined4)((ulonglong)uVar23 >> 8);
    uStack_1cc._0_3_ = (undefined3)((ulonglong)uVar23 >> 0x28);
    uStack_1cc = CONCAT13((char)*(undefined4 *)(pbVar21 + 0x38),(undefined3)uStack_1cc);
    uStack_144 = (undefined4)*(undefined8 *)(pbVar20 + 0x20);
    local_140 = (undefined4)((ulonglong)*(undefined8 *)(pbVar20 + 0x20) >> 0x20);
    uStack_154 = (undefined4)*(undefined8 *)pbVar26;
    fStack_150 = (float)((ulonglong)*(undefined8 *)pbVar26 >> 0x20);
    uStack_14c = (undefined4)*(undefined8 *)(pbVar20 + 0x18);
    local_148 = (undefined4)((ulonglong)*(undefined8 *)(pbVar20 + 0x18) >> 0x20);
    local_158 = 1.4013e-45;
    puStack_210 = &UNK_14155ded0;
    ::utils::geometry::rectangle::Rectangle2d::area();
    puStack_210 = &UNK_14155dee9;
    ::utils::inverse_lerp();
    *(undefined4 *)pbVar31 = local_1e4;
    lVar22 = *(longlong *)(lVar9 + 0x30);
    uVar23 = *(undefined8 *)(lVar9 + 0x40);
    if (lVar22 == *(longlong *)(lVar9 + 0x20)) {
      puStack_210 = &UNK_14155df16;
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    lVar8 = *(longlong *)(lVar9 + 0x28);
    lVar10 = lVar22 * 0x20;
    *(undefined1 *)(lVar8 + lVar10) = 2;
    puVar1 = (undefined8 *)(lVar8 + 1 + lVar10);
    *puVar1 = CONCAT17(uStack_1d1,local_1d8);
    puVar1[1] = CONCAT44(uStack_1cc,uStack_1d0);
    uStack_1d0 = CONCAT31(uStack_1c8,uStack_1cc._3_1_);
    uVar40 = extraout_XMM0_Da;
    goto code_r0x00014155e0cf;
  case 2:
    if (0.0 < (float)uVar43) {
      uStack_1d0 = *(undefined4 *)(pbVar21 + 0x38);
      local_1d8 = (undefined7)*(undefined8 *)(pbVar21 + 0x30);
      uStack_1d1 = (undefined1)((ulonglong)*(undefined8 *)(pbVar21 + 0x30) >> 0x38);
      uStack_144 = (undefined4)*(undefined8 *)(pbVar20 + 0x20);
      local_140 = (undefined4)((ulonglong)*(undefined8 *)(pbVar20 + 0x20) >> 0x20);
      uStack_154 = (undefined4)*(undefined8 *)pbVar26;
      fStack_150 = (float)((ulonglong)*(undefined8 *)pbVar26 >> 0x20);
      uStack_14c = (undefined4)*(undefined8 *)(pbVar20 + 0x18);
      local_148 = (undefined4)((ulonglong)*(undefined8 *)(pbVar20 + 0x18) >> 0x20);
      local_158 = 1.4013e-45;
      puStack_210 = &UNK_14155df8c;
      ::utils::geometry::rectangle::Rectangle2d::area();
      puStack_210 = &UNK_14155dfa5;
      ::utils::inverse_lerp();
      *(undefined4 *)pbVar31 = local_1e4;
      lVar22 = *(longlong *)(lVar9 + 0x30);
      uVar23 = *(undefined8 *)(lVar9 + 0x40);
      if (lVar22 == *(longlong *)(lVar9 + 0x20)) {
        puStack_210 = &UNK_14155dfd2;
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                  ();
      }
      lVar8 = *(longlong *)(lVar9 + 0x28);
      lVar10 = lVar22 * 0x20;
      *(undefined1 *)(lVar8 + lVar10) = 1;
      *(float *)(lVar8 + 4 + lVar10) = (float)uVar43;
      *(ulonglong *)(lVar8 + 8 + lVar10) = CONCAT17(uStack_1d1,local_1d8);
      *(undefined4 *)(lVar8 + 0x10 + lVar10) = uStack_1d0;
      *(undefined4 *)(lVar8 + 0x14 + lVar10) = extraout_XMM0_Da_00;
      *(undefined8 *)(lVar8 + 0x18 + lVar10) = uVar23;
      *(longlong *)(lVar9 + 0x30) = lVar22 + 1;
      *(longlong *)(lVar9 + 0x40) = *(longlong *)(lVar9 + 0x40) + 1;
    }
    fVar36 = (float)(uVar43 >> 0x20);
    if (fVar36 <= 0.0) {
      return;
    }
    uStack_1d0 = *(undefined4 *)(pbVar21 + 0x38);
    local_1d8 = (undefined7)*(undefined8 *)(pbVar21 + 0x30);
    uStack_1d1 = (undefined1)((ulonglong)*(undefined8 *)(pbVar21 + 0x30) >> 0x38);
    uStack_144 = (undefined4)*(undefined8 *)(pbVar20 + 0x20);
    local_140 = (undefined4)((ulonglong)*(undefined8 *)(pbVar20 + 0x20) >> 0x20);
    uStack_154 = (undefined4)*(undefined8 *)pbVar26;
    fStack_150 = (float)((ulonglong)*(undefined8 *)pbVar26 >> 0x20);
    uStack_14c = (undefined4)*(undefined8 *)(pbVar20 + 0x18);
    local_148 = (undefined4)((ulonglong)*(undefined8 *)(pbVar20 + 0x18) >> 0x20);
    local_158 = 1.4013e-45;
    puStack_210 = &UNK_14155e065;
    ::utils::geometry::rectangle::Rectangle2d::area();
    puStack_210 = &UNK_14155e07e;
    ::utils::inverse_lerp();
    *(undefined4 *)pbVar31 = local_1e4;
    lVar22 = *(longlong *)(lVar9 + 0x30);
    uVar23 = *(undefined8 *)(lVar9 + 0x40);
    if (lVar22 == *(longlong *)(lVar9 + 0x20)) {
      puStack_210 = &UNK_14155e0ab;
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    lVar8 = *(longlong *)(lVar9 + 0x28);
    lVar10 = lVar22 * 0x20;
    *(undefined1 *)(lVar8 + lVar10) = 0;
    *(float *)(lVar8 + 4 + lVar10) = fVar36;
    *(ulonglong *)(lVar8 + 8 + lVar10) = CONCAT17(uStack_1d1,local_1d8);
    uVar40 = extraout_XMM0_Da_01;
code_r0x00014155e0cf:
    *(undefined4 *)(lVar8 + 0x10 + lVar10) = uStack_1d0;
    *(undefined4 *)(lVar8 + 0x14 + lVar10) = uVar40;
    *(undefined8 *)(lVar8 + 0x18 + lVar10) = uVar23;
    *(longlong *)(lVar9 + 0x30) = lVar22 + 1;
    *(longlong *)(lVar9 + 0x40) = *(longlong *)(lVar9 + 0x40) + 1;
LAB_14155e297:
                    // WARNING: Read-only address (ram,0x000142c6da00) is written
                    // WARNING: Read-only address (ram,0x000142c6da10) is written
    return;
  case 3:
    goto LAB_14155e297;
  case 4:
    puVar12 = &DAT_142c6e874;
    puVar7 = (undefined *)(longlong)*(int *)(&DAT_142c6e874 + (longlong)puVar7 * 4);
  case 8:
                    // WARNING: Could not recover jumptable at 0x000141560951. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(puVar12 + (longlong)puVar7))();
    return;
  case 5:
    *(undefined **)(pbVar20 + 0x40) = puVar7;
    *(undefined8 *)(pbVar20 + 0x48) = *(undefined8 *)(pbVar20 + 8);
    puVar7 = (undefined *)0xc;
    ppuVar19 = (undefined **)auStack_208;
    break;
  case 6:
    pbVar31 = pbVar21 + 0x142c64564;
    *pbVar31 = *pbVar31 ^ (byte)((ulonglong)puVar17 >> 8);
    if (*pbVar31 != 0) break;
    plVar15 = *(longlong **)(pbVar20 + 0x38);
    goto code_r0x000141560ba4;
  case 7:
    *pbVar25 = *pbVar25 ^ (byte)uVar28;
    *(undefined **)(pbVar20 + 0x38) = puVar7;
    puVar7 = (undefined *)0x8;
    ppuVar19 = (undefined **)auStack_208;
    break;
  case 9:
    pbVar20[0x30] = 0;
    pbVar20[0x31] = 0xf5;
    puVar7 = (undefined *)0x0;
    ppuVar19 = &puStack_210;
    puStack_210 = puVar17;
    break;
  case 10:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0xb:
    goto code_r0x000141560dfa;
  case 0xc:
                    // WARNING: Could not emulate address calculation at 0x000141560de4
                    // WARNING: Treating indirect jump as call
    (*(code *)(&DAT_142c6e864 + *(uint *)(&DAT_142c6e864 + (longlong)puVar7 * 4)))();
    return;
  case 0xd:
code_r0x000141560dfa:
    puStack_210 = &UNK_141560dff;
    resources::walls::_::impl$0::deserialize::impl$0::
    visit_newtype_struct<serde::__private::de::content::ContentRefDeserializer<serde_json::error::Error>_>
              ();
    *(undefined **)(pbVar21 + 8) = puVar14;
    *(ulonglong *)pbVar21 = (ulonglong)(extraout_EAX & 1) * 3;
    return;
  case 0xe:
  case 0x10:
    return;
  default:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  }
  *(undefined4 *)(pbVar20 + 0x14) = *(undefined4 *)(pbVar20 + 0x34);
  *(undefined4 *)(pbVar20 + 0x11) = *(undefined4 *)(pbVar20 + 0x31);
  *(undefined8 *)(pbVar20 + 0x20) = *(undefined8 *)(pbVar20 + 0x40);
  *(undefined8 *)(pbVar20 + 0x28) = *(undefined8 *)(pbVar20 + 0x48);
  pbVar20[0x10] = (byte)puVar7;
  *(undefined8 *)(pbVar20 + 0x18) = *(undefined8 *)(pbVar20 + 0x38);
  *(undefined ***)((longlong)ppuVar19 + 0x20) =
       &anon_36890fc008bdb4b97258d46f0ec99221_129_llvm_10723592867021123337;
  *(undefined8 *)((longlong)ppuVar19 + 0x28) = 3;
  *(undefined **)((longlong)ppuVar19 + -8) = &UNK_141560a92;
  serde::__private::de::content::impl$21::
  deserialize_enum<serde_json::error::Error,country_core::resources::walls::decorator_storage::_::impl_0::deserialize::__Visitor>
            ();
  lVar22 = *(longlong *)(pbVar20 + 0x30);
  plVar15 = *(longlong **)(pbVar20 + 0x38);
  if (lVar22 == 3) {
    lVar22 = *plVar15;
    *(longlong **)(pbVar20 + 0x50) = plVar15;
    if (lVar22 == 1) {
      *(undefined **)((longlong)ppuVar19 + -8) = &UNK_141560af9;
      core::ptr::drop_in_place<std::io::error::Error>();
    }
    else if ((lVar22 == 0) && (*(longlong *)(*(longlong *)(pbVar20 + 0x50) + 0x10) != 0)) {
      *(undefined **)((longlong)ppuVar19 + -8) = &UNK_141560ad2;
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    plVar15 = (longlong *)&DAT_00000028;
    *(undefined **)((longlong)ppuVar19 + -8) = &UNK_141560b0d;
    std::alloc::__default_lib_allocator::__rust_dealloc();
    *(undefined **)((longlong)ppuVar19 + -8) = &UNK_141560b1f;
    resources::walls::_::impl$0::deserialize::impl$0::
    visit_newtype_struct<serde::__private::de::content::ContentRefDeserializer<serde_json::error::Error>_>
              ();
    if ((extraout_RAX_00 & 1) == 0) {
      *(undefined **)((longlong)ppuVar19 + -8) = &UNK_141560b63;
      core::ptr::drop_in_place<enum2_<serde::__private::de::content::Content>_>();
      lVar9 = 0;
    }
    else {
      lVar22 = *plVar15;
      *(longlong **)(pbVar20 + 0x50) = plVar15;
      if (lVar22 == 1) {
        *(undefined **)((longlong)ppuVar19 + -8) = &UNK_141560b73;
        core::ptr::drop_in_place<std::io::error::Error>();
      }
      else if ((lVar22 == 0) && (*(longlong *)(*(longlong *)(pbVar20 + 0x50) + 0x10) != 0)) {
        *(undefined **)((longlong)ppuVar19 + -8) = &UNK_141560b58;
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      *(undefined **)((longlong)ppuVar19 + -8) = &UNK_141560b87;
      std::alloc::__default_lib_allocator::__rust_dealloc();
      *(undefined **)((longlong)ppuVar19 + -8) = &UNK_141560b98;
      serde_json::error::impl$8::custom<ref_<str_>_>();
      *(undefined **)((longlong)ppuVar19 + -8) = &UNK_141560ba4;
      core::ptr::drop_in_place<enum2_<serde::__private::de::content::Content>_>();
      plVar15 = extraout_RAX_01;
code_r0x000141560ba4:
      lVar9 = 3;
    }
  }
  else {
    *(undefined **)((longlong)ppuVar19 + -8) = &UNK_141560add;
    core::ptr::drop_in_place<enum2_<serde::__private::de::content::Content>_>();
    lVar9 = 0;
    if (lVar22 - 1U < 2) {
      lVar9 = lVar22;
    }
  }
  *(longlong **)(pbVar21 + 8) = plVar15;
  *(longlong *)pbVar21 = lVar9;
  return;
}



// CALLEES: [{'name': 'add<ref$<str$>_>', 'address': '1409236d0'}, {'name': 'area', 'address': '140c99520'}, {'name': 'eye_right', 'address': '140c96df0'}, {'name': 'grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>', 'address': '140562870'}, {'name': 'grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>', 'address': '1405629f0'}, {'name': 'grow_one<enum2$<iced_core::event::Event>,alloc::alloc::Global>', 'address': '1406319d0'}, {'name': 'grow_one<indexmap::Bucket<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2$<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>', 'address': '140562ab0'}, {'name': 'grow_one<tuple$<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>', 'address': '140562c30'}, {'name': 'grow_one<tuple$<ref$<str$>,enum2$<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>', 'address': '140562db0'}, {'name': 'inverse_lerp', 'address': '140c904a0'}, {'name': 'is_shape_inside', 'address': '140ad2970'}, {'name': 'push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>', 'address': '14146c880'}, {'name': 'rectangle_signifier', 'address': '14155e460'}]