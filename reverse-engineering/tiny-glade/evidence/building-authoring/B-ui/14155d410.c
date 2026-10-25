
/* WARNING: Control flow encountered bad instruction data */
/* WARNING: Instruction at (ram,0x000141560dfd) overlaps instruction at (ram,0x000141560dfa)
    */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
country_core::systems::wall::shapes::rectangle::ui_create_rectangle::ui_create_rectangle(void)

{
  undefined8 *puVar1;
  undefined4 uVar2;
  undefined4 *puVar3;
  undefined8 uVar4;
  uint uVar5;
  uint uVar6;
  byte bVar7;
  char extraout_AL;
  char extraout_AL_00;
  uint extraout_EAX;
  longlong extraout_RAX;
  ulonglong uVar8;
  undefined *puVar9;
  longlong lVar10;
  ulonglong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong lVar11;
  undefined8 *in_RCX;
  longlong lVar12;
  longlong lVar13;
  undefined *puVar14;
  byte bVar15;
  char *pcVar16;
  undefined *puVar17;
  longlong *plVar18;
  char cVar19;
  longlong unaff_RBX;
  undefined7 uVar21;
  undefined *puVar20;
  undefined **ppuVar22;
  byte *pbVar23;
  byte *pbVar24;
  longlong lVar25;
  undefined8 uVar26;
  undefined8 *in_R8;
  ulonglong uVar27;
  byte *pbVar28;
  byte *pbVar29;
  undefined4 *puVar30;
  ulonglong uVar31;
  ulonglong uVar32;
  float *pfVar33;
  byte *pbVar34;
  bool bVar35;
  bool bVar36;
  bool bVar37;
  bool bVar38;
  float fVar39;
  undefined4 extraout_XMM0_Da;
  undefined4 extraout_XMM0_Da_00;
  undefined4 extraout_XMM0_Da_01;
  undefined4 extraout_XMM0_Da_02;
  undefined4 extraout_XMM0_Da_03;
  float fVar40;
  undefined1 auVar41 [16];
  undefined1 auVar42 [16];
  undefined4 uVar43;
  float fVar44;
  float fVar45;
  float fVar48;
  undefined1 auVar46 [16];
  undefined1 auVar47 [16];
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
  
  ppuVar22 = (undefined **)auStack_208;
  pbVar23 = (byte *)in_R8[4];
  local_180 = (uint *)in_R8[6];
  local_1ac = *(uint *)((longlong)in_R8 + 0x3c);
  puVar30 = (undefined4 *)(ulonglong)local_1ac;
  if ((*(int *)*in_RCX != 6) || ((char)((int *)*in_RCX)[1] != '\x02')) {
    *local_180 = local_1ac;
    pbVar23[0] = 0;
    pbVar23[1] = 0;
    pbVar23[2] = 0;
    pbVar23[3] = 0;
    return;
  }
  pbVar24 = (byte *)*in_R8;
  local_188 = (float *)in_stack_00000060[5];
  local_90 = in_stack_00000068[4];
  local_98 = (undefined4 *)in_stack_00000068[6];
  local_168 = *(undefined4 *)((longlong)in_stack_00000068 + 0x3c);
  lVar25 = *in_stack_00000078;
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
  lVar11 = in_stack_00000068[8];
  pbVar34 = (byte *)in_stack_00000068[10];
  local_1e4 = *(undefined4 *)((longlong)in_stack_00000068 + 0x5c);
  pcVar16 = "wall_rectangle_canvascursor_rectangle_";
  puStack_210 = (undefined *)0x14155d59e;
  systems::ui::resources::ui_state::UiState::add<ref_<str_>_>();
  local_198 = pcVar16;
  local_190 = extraout_RAX;
  if (__real_42700000 <= *(float *)(pbVar24 + 0x3c)) {
    local_148 = (undefined4)*(undefined8 *)(pbVar23 + 0x20);
    uStack_144 = (undefined4)((ulonglong)*(undefined8 *)(pbVar23 + 0x20) >> 0x20);
    local_158 = (float)*(undefined8 *)(pbVar23 + 0x10);
    uStack_154 = (undefined4)((ulonglong)*(undefined8 *)(pbVar23 + 0x10) >> 0x20);
    fStack_150 = (float)*(undefined8 *)(pbVar23 + 0x18);
    uStack_14c = (undefined4)((ulonglong)*(undefined8 *)(pbVar23 + 0x18) >> 0x20);
    *(undefined4 *)in_stack_00000080[2] = *(undefined4 *)((longlong)in_stack_00000080 + 0x1c);
    unaff_RBX = *in_stack_00000080;
    lVar10 = *(longlong *)(unaff_RBX + 0x30);
    uVar26 = *(undefined8 *)(unaff_RBX + 0x40);
    local_1e0 = pbVar24;
    if (lVar10 == *(longlong *)(unaff_RBX + 0x20)) {
      puStack_210 = (undefined *)0x14155d619;
      alloc::raw_vec::RawVec::
      grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                ();
    }
    lVar12 = *(longlong *)(unaff_RBX + 0x28);
    *(undefined8 *)(lVar12 + lVar10 * 0x28) = uVar26;
    *(undefined4 *)(lVar12 + 8 + lVar10 * 0x28) = 1;
    *(ulonglong *)(lVar12 + 0x1c + lVar10 * 0x28) = CONCAT44(uStack_144,local_148);
    puVar1 = (undefined8 *)(lVar12 + 0xc + lVar10 * 0x28);
    *puVar1 = CONCAT44(uStack_154,local_158);
    puVar1[1] = CONCAT44(uStack_14c,fStack_150);
    *(longlong *)(unaff_RBX + 0x30) = lVar10 + 1;
    *(longlong *)(unaff_RBX + 0x40) = *(longlong *)(unaff_RBX + 0x40) + 1;
    pbVar24 = local_1e0;
  }
  uVar21 = (undefined7)((ulonglong)unaff_RBX >> 8);
  cVar19 = '\x02';
  if (1 < *(byte *)(local_190 + 0x18)) {
    if ((*(char **)(local_190 + 0x38) == local_198) && (*(char *)(local_190 + 0x19) == '\x03')) {
      uVar43 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
      puVar30 = (undefined4 *)in_stack_00000038[2];
      *puVar30 = uVar43;
      puVar3 = (undefined4 *)*in_stack_00000038;
      *(undefined1 *)(puVar3 + 3) = 0;
      *(undefined8 *)(puVar3 + 1) = *(undefined8 *)*in_stack_00000050;
      *puVar30 = uVar43;
      *puVar3 = 0;
      *(undefined4 *)in_stack_00000058[2] = *(undefined4 *)((longlong)in_stack_00000058 + 0x1c);
      lVar10 = *in_stack_00000058;
      lVar12 = *(longlong *)(lVar10 + 0x30);
      uVar26 = *(undefined8 *)(lVar10 + 0x40);
      local_1e0 = pbVar24;
      local_1a8 = lVar25;
      if (lVar12 == *(longlong *)(lVar10 + 0x20)) {
        puStack_210 = (undefined *)0x14155d70c;
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                  ();
      }
      *(undefined8 *)(*(longlong *)(lVar10 + 0x28) + lVar12 * 8) = uVar26;
      *(longlong *)(lVar10 + 0x30) = lVar12 + 1;
      *(longlong *)(lVar10 + 0x40) = *(longlong *)(lVar10 + 0x40) + 1;
      pbVar24 = local_1e0;
      lVar25 = local_1a8;
      if (*(byte *)(local_190 + 0x18) < 2) goto LAB_14155d8f8;
    }
    if ((*(char **)(local_190 + 0x38) == local_198) && (*(char *)(local_190 + 0x19) == '\x04')) {
      fVar44 = (float)*(undefined8 *)(*in_stack_00000040 + 0x140);
      fVar39 = (float)((ulonglong)*(undefined8 *)(*in_stack_00000040 + 0x140) >> 0x20);
      uVar43 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
      puVar3 = (undefined4 *)in_stack_00000038[2];
      *puVar3 = uVar43;
      pfVar33 = (float *)*in_stack_00000038;
      fVar39 = SQRT(fVar39 * fVar39 + fVar44 * fVar44) + *pfVar33;
      *pfVar33 = fVar39;
      if (__real_40800000 < fVar39) {
        *puVar3 = uVar43;
        *(undefined1 *)(pfVar33 + 3) = 1;
      }
      *(undefined4 *)in_stack_00000058[2] = *(undefined4 *)((longlong)in_stack_00000058 + 0x1c);
      lVar10 = *in_stack_00000058;
      lVar12 = *(longlong *)(lVar10 + 0x30);
      uVar26 = *(undefined8 *)(lVar10 + 0x40);
      local_1e0 = pbVar24;
      local_1b8 = pbVar34;
      local_1a8 = lVar25;
      if (lVar12 == *(longlong *)(lVar10 + 0x20)) {
        puStack_210 = (undefined *)0x14155d7f5;
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                  ();
      }
      *(undefined8 *)(*(longlong *)(lVar10 + 0x28) + lVar12 * 8) = uVar26;
      *(longlong *)(lVar10 + 0x30) = lVar12 + 1;
      *(longlong *)(lVar10 + 0x40) = *(longlong *)(lVar10 + 0x40) + 1;
      pbVar24 = local_1e0;
      pbVar34 = local_1b8;
      if (*(byte *)(local_190 + 0x18) < 2) goto LAB_14155d8f8;
    }
    if ((*(char **)(local_190 + 0x38) == local_198) && (*(byte *)(local_190 + 0x19) < 2)) {
      uVar26 = *(undefined8 *)(*in_stack_00000038 + 4);
      *(undefined4 *)in_stack_00000048[2] = *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
      lVar25 = *in_stack_00000048;
      lVar10 = *(longlong *)(lVar25 + 0x30);
      uVar4 = *(undefined8 *)(lVar25 + 0x40);
      local_1e0 = pbVar24;
      local_1b8 = pbVar23;
      local_1a8 = lVar11;
      if (lVar10 == *(longlong *)(lVar25 + 0x20)) {
        puStack_210 = (undefined *)0x14155d89d;
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      lVar11 = *(longlong *)(lVar25 + 0x28);
      *(undefined8 *)(lVar11 + lVar10 * 0x18) = uVar4;
      *(undefined4 *)(lVar11 + 8 + lVar10 * 0x18) = 2;
      *(undefined8 *)(lVar11 + 0xc + lVar10 * 0x18) = uVar26;
      *(longlong *)(lVar25 + 0x30) = lVar10 + 1;
      *(longlong *)(lVar25 + 0x40) = *(longlong *)(lVar25 + 0x40) + 1;
      pbVar23 = local_1b8;
      pbVar24 = local_1e0;
      lVar11 = local_1a8;
      if (*(byte *)(local_190 + 0x18) < 2) goto LAB_14155d8f8;
    }
    if (*(char **)(local_190 + 0x38) == local_198) {
      cVar19 = *(char *)(local_190 + 0x19);
      uVar21 = 0;
    }
  }
LAB_14155d8f8:
  uVar6 = _UNK_14292e984;
  uVar5 = ___xmm_7fffffff7fffffff7fffffff7fffffff;
  bVar7 = 3;
  if ((byte)(cVar19 - 2U) < 3) {
    bVar7 = cVar19 - 2U;
  }
  if (bVar7 < 2) {
    if (pbVar24[0x2c] == 2) {
      puStack_210 = (undefined *)0x14155d92f;
      ::utils::camera::CameraMatrices::eye_right();
      fVar39 = __real_3f800000 / SQRT(fStack_150 * fStack_150 + local_158 * local_158);
      uVar26 = CONCAT44(fVar39 * fStack_150,fVar39 * local_158);
    }
    else {
      fVar39 = *(float *)(pbVar24 + 0x18);
      fVar44 = *(float *)(pbVar24 + 0x1c);
      fVar40 = __real_3f800000 / SQRT(fVar39 * fVar39 + fVar44 * fVar44);
      uVar26 = CONCAT44(fVar40 * fVar39,
                        fVar40 * (float)(___xmm_80000000800000008000000080000000 ^ (uint)fVar44));
    }
    *local_180 = local_1ac;
    *(undefined8 *)(pbVar23 + 0x10) = uVar26;
    uVar43 = *(undefined4 *)(pbVar24 + 0x30);
    uVar2 = *(undefined4 *)(pbVar24 + 0x38);
    *local_180 = local_1ac;
    *(undefined4 *)(pbVar23 + 0x18) = uVar43;
    *(undefined4 *)(pbVar23 + 0x1c) = uVar2;
    uVar43 = *(undefined4 *)(pbVar24 + 0x34);
    *local_180 = local_1ac;
    *(undefined4 *)(pbVar23 + 0x28) = uVar43;
    uStack_1c8 = (undefined3)*(undefined8 *)(pbVar23 + 0x20);
    uStack_1c5 = (undefined5)((ulonglong)*(undefined8 *)(pbVar23 + 0x20) >> 0x18);
    local_1d8 = (undefined7)*(undefined8 *)(pbVar23 + 0x10);
    uStack_1d1 = (undefined1)((ulonglong)*(undefined8 *)(pbVar23 + 0x10) >> 0x38);
    uStack_1d0 = (undefined4)*(undefined8 *)(pbVar23 + 0x18);
    uStack_1cc = (undefined4)((ulonglong)*(undefined8 *)(pbVar23 + 0x18) >> 0x20);
    puVar20 = (undefined *)CONCAT71(uVar21,1);
    puStack_210 = (undefined *)0x14155db4c;
    rectangle_signifier();
    puStack_210 = (undefined *)0x14155db57;
    append_only_vec::AppendOnlyVec::
    push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>();
    uVar26 = 0;
  }
  else if (bVar7 == 2) {
    uVar26 = 0;
    puVar20 = (undefined *)CONCAT71(uVar21,1);
    if (*(char *)(*in_stack_00000038 + 0xc) != '\0') {
      fVar39 = (*(float *)(pbVar24 + 0x34) - local_188[1]) / local_188[4];
      fVar44 = (local_188[3] * fVar39 + *local_188) - *(float *)(pbVar23 + 0x18);
      fVar39 = (fVar39 * local_188[5] + local_188[2]) - *(float *)(pbVar23 + 0x1c);
      uVar26 = *(undefined8 *)(pbVar23 + 0x20);
      fVar40 = (float)*(undefined8 *)(pbVar23 + 0x10);
      fVar48 = (float)((ulonglong)*(undefined8 *)(pbVar23 + 0x10) >> 0x20);
      fVar45 = (float)((uint)(fVar48 * fVar39 + fVar44 * fVar40) &
                      ___xmm_7fffffff7fffffff7fffffff7fffffff);
      fVar39 = (float)((uint)(fVar40 * fVar39 - fVar44 * fVar48) & _UNK_14292e984);
      auVar46._0_4_ = fVar45 + fVar45;
      auVar46._4_4_ = fVar39 + fVar39;
      auVar46._8_8_ = 0;
      auVar41 = maxps(___xmm_00000000000000004010000040100000,auVar46);
      auVar42._0_4_ = (uint)__xmm_00000000000000004010000040100000 & -(uint)NAN(auVar46._0_4_);
      auVar42._4_4_ = __xmm_00000000000000004010000040100000._4_4_ & -(uint)NAN(auVar46._4_4_);
      auVar42._8_8_ = 0;
      auVar47._0_4_ = ~-(uint)NAN(auVar46._0_4_) & auVar41._0_4_;
      auVar47._4_4_ = ~-(uint)NAN(auVar46._4_4_) & auVar41._4_4_;
      auVar47._8_4_ = auVar41._8_4_;
      auVar47._12_4_ = auVar41._12_4_;
      auVar47 = auVar47 | auVar42;
      auVar42 = minps(___xmm_00000000000000004170000041700000,auVar47);
      fVar39 = (float)(~-(uint)NAN(auVar47._0_4_) & auVar42._0_4_ |
                      (uint)__xmm_00000000000000004170000041700000 & -(uint)NAN(auVar47._0_4_));
      fVar44 = (float)(~-(uint)NAN(auVar47._4_4_) & auVar42._4_4_ |
                      __xmm_00000000000000004170000041700000._4_4_ & -(uint)NAN(auVar47._4_4_));
      *(float *)(pbVar23 + 0x20) = fVar39;
      uVar26 = CONCAT44((uint)(fVar44 - (float)((ulonglong)uVar26 >> 0x20)) & uVar6,
                        (uint)(fVar39 - (float)uVar26) & uVar5);
      *local_180 = local_1ac;
      *(float *)(pbVar23 + 0x24) = fVar44;
    }
    uStack_1c8 = (undefined3)*(undefined8 *)(pbVar23 + 0x20);
    uStack_1c5 = (undefined5)((ulonglong)*(undefined8 *)(pbVar23 + 0x20) >> 0x18);
    local_1d8 = (undefined7)*(undefined8 *)(pbVar23 + 0x10);
    uStack_1d1 = (undefined1)((ulonglong)*(undefined8 *)(pbVar23 + 0x10) >> 0x38);
    uStack_1d0 = (undefined4)*(undefined8 *)(pbVar23 + 0x18);
    uStack_1cc = (undefined4)((ulonglong)*(undefined8 *)(pbVar23 + 0x18) >> 0x20);
    puStack_210 = (undefined *)0x14155da9f;
    rectangle_signifier();
    puStack_210 = (undefined *)0x14155daaa;
    append_only_vec::AppendOnlyVec::
    push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>();
  }
  else {
    uVar26 = 0;
    puVar20 = (undefined *)CONCAT71(uVar21,1);
    if (*(char *)(*in_stack_00000038 + 0xc) == '\0') {
      uStack_144 = (undefined4)*(undefined8 *)(pbVar23 + 0x20);
      local_140 = (undefined4)((ulonglong)*(undefined8 *)(pbVar23 + 0x20) >> 0x20);
      uStack_154 = (undefined4)*(undefined8 *)(pbVar23 + 0x10);
      fStack_150 = (float)((ulonglong)*(undefined8 *)(pbVar23 + 0x10) >> 0x20);
      uStack_14c = (undefined4)*(undefined8 *)(pbVar23 + 0x18);
      local_148 = (undefined4)((ulonglong)*(undefined8 *)(pbVar23 + 0x18) >> 0x20);
      local_158 = 1.4013e-45;
      puStack_210 = (undefined *)0x14155dde7;
      resources::glade_border::GladeBorder::is_shape_inside();
      if (extraout_AL_00 == '\0') {
        *local_a8 = local_170;
        lVar25 = *(longlong *)(local_1a0 + 0xc);
        uVar4 = *(undefined8 *)(local_1a0 + 0x10);
        if (lVar25 == *(longlong *)(local_1a0 + 8)) {
          puStack_210 = (undefined *)0x14155e12c;
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                    ();
        }
        *(undefined8 *)(*(longlong *)(local_1a0 + 10) + lVar25 * 8) = uVar4;
        *(longlong *)(local_1a0 + 0xc) = lVar25 + 1;
        *(longlong *)(local_1a0 + 0x10) = *(longlong *)(local_1a0 + 0x10) + 1;
        puVar20 = (undefined *)0x0;
      }
      else {
        pbVar29 = pbVar23 + 0x10;
        if (pbVar24[0x2c] != 2) {
          puVar30 = *(undefined4 **)(pbVar24 + 0x20);
        }
        pbVar28 = (byte *)(ulonglong)(pbVar24[0x2c] != 2);
        uVar43 = *(undefined4 *)(pbVar24 + 0x40);
        uVar2 = *(undefined4 *)(pbVar24 + 0x3c);
        local_148 = (undefined4)*(undefined8 *)(pbVar23 + 0x20);
        uStack_144 = (undefined4)((ulonglong)*(undefined8 *)(pbVar23 + 0x20) >> 0x20);
        local_158 = (float)*(undefined8 *)pbVar29;
        uStack_154 = (undefined4)((ulonglong)*(undefined8 *)pbVar29 >> 0x20);
        fStack_150 = (float)*(undefined8 *)(pbVar23 + 0x18);
        uStack_14c = (undefined4)((ulonglong)*(undefined8 *)(pbVar23 + 0x18) >> 0x20);
        *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
        lVar25 = *in_stack_00000028;
        lVar10 = *(longlong *)(lVar25 + 0x30);
        pfVar33 = *(float **)(lVar25 + 0x40);
        if (lVar10 == *(longlong *)(lVar25 + 0x20)) {
          puStack_210 = (undefined *)0x14155e332;
          local_1e0 = pbVar29;
          local_1b8 = pbVar28;
          local_1a0 = puVar30;
          local_188 = *(float **)(lVar25 + 0x40);
          alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>();
          pbVar29 = local_1e0;
          pbVar28 = local_1b8;
          puVar30 = local_1a0;
          pfVar33 = local_188;
        }
        lVar12 = *(longlong *)(lVar25 + 0x28);
        lVar13 = lVar10 * 0x38;
        *(byte **)(lVar12 + lVar13) = pbVar28;
        *(undefined4 **)(lVar12 + 8 + lVar13) = puVar30;
        puVar1 = (undefined8 *)(lVar12 + 0x10 + lVar13);
        *puVar1 = CONCAT44(uStack_154,local_158);
        puVar1[1] = CONCAT44(uStack_14c,fStack_150);
        *(ulonglong *)(lVar12 + 0x20 + lVar13) = CONCAT44(uStack_144,local_148);
        *(undefined4 *)(lVar12 + 0x28 + lVar13) = uVar43;
        *(undefined4 *)(lVar12 + 0x2c + lVar13) = uVar2;
        *(float **)(lVar12 + 0x30 + lVar13) = pfVar33;
        *(longlong *)(lVar25 + 0x30) = lVar10 + 1;
        *(longlong *)(lVar25 + 0x40) = *(longlong *)(lVar25 + 0x40) + 1;
        uStack_1c8 = (undefined3)((uint)*(undefined4 *)(pbVar24 + 0x38) >> 8);
        uVar4 = *(undefined8 *)(pbVar24 + 0x30);
        uStack_1d1 = (undefined1)uVar4;
        uStack_1d0 = (undefined4)((ulonglong)uVar4 >> 8);
        uStack_1cc._0_3_ = (undefined3)((ulonglong)uVar4 >> 0x28);
        uStack_1cc = CONCAT13((char)*(undefined4 *)(pbVar24 + 0x38),(undefined3)uStack_1cc);
        uStack_144 = (undefined4)*(undefined8 *)(pbVar29 + 0x10);
        local_140 = (undefined4)((ulonglong)*(undefined8 *)(pbVar29 + 0x10) >> 0x20);
        uStack_154 = (undefined4)*(undefined8 *)pbVar29;
        fStack_150 = (float)((ulonglong)*(undefined8 *)pbVar29 >> 0x20);
        uStack_14c = (undefined4)*(undefined8 *)(pbVar29 + 8);
        local_148 = (undefined4)((ulonglong)*(undefined8 *)(pbVar29 + 8) >> 0x20);
        local_158 = 1.4013e-45;
        puStack_210 = (undefined *)0x14155e3d3;
        ::utils::geometry::rectangle::Rectangle2d::area();
        puStack_210 = (undefined *)0x14155e3ec;
        ::utils::inverse_lerp();
        *(undefined4 *)pbVar34 = local_1e4;
        lVar25 = *(longlong *)(lVar11 + 0x30);
        uVar4 = *(undefined8 *)(lVar11 + 0x40);
        if (lVar25 == *(longlong *)(lVar11 + 0x20)) {
          puStack_210 = (undefined *)0x14155e419;
          alloc::raw_vec::RawVec::
          grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                    ();
        }
        lVar10 = *(longlong *)(lVar11 + 0x28);
        lVar12 = lVar25 * 0x20;
        *(undefined1 *)(lVar10 + lVar12) = 7;
        puVar1 = (undefined8 *)(lVar10 + 1 + lVar12);
        *puVar1 = CONCAT17(uStack_1d1,local_1d8);
        puVar1[1] = CONCAT44(uStack_1cc,uStack_1d0);
        *(uint *)(lVar10 + 0x10 + lVar12) = CONCAT31(uStack_1c8,uStack_1cc._3_1_);
        *(undefined4 *)(lVar10 + 0x14 + lVar12) = extraout_XMM0_Da_03;
        *(undefined8 *)(lVar10 + 0x18 + lVar12) = uVar4;
        *(longlong *)(lVar11 + 0x30) = lVar25 + 1;
        *(longlong *)(lVar11 + 0x40) = *(longlong *)(lVar11 + 0x40) + 1;
      }
    }
  }
  uStack_144 = (undefined4)*(undefined8 *)(pbVar23 + 0x20);
  local_140 = (undefined4)((ulonglong)*(undefined8 *)(pbVar23 + 0x20) >> 0x20);
  uStack_154 = (undefined4)*(undefined8 *)(pbVar23 + 0x10);
  fStack_150 = (float)((ulonglong)*(undefined8 *)(pbVar23 + 0x10) >> 0x20);
  uStack_14c = (undefined4)*(undefined8 *)(pbVar23 + 0x18);
  local_148 = (undefined4)((ulonglong)*(undefined8 *)(pbVar23 + 0x18) >> 0x20);
  local_158 = 1.4013e-45;
  puStack_210 = (undefined *)0x14155db8d;
  resources::glade_border::GladeBorder::is_shape_inside();
  if (extraout_AL == '\0') {
    *local_a0 = local_16c;
    lVar25 = *(longlong *)(local_178 + 0x30);
    uVar4 = *(undefined8 *)(local_178 + 0x40);
    if (lVar25 == *(longlong *)(local_178 + 0x20)) {
      puStack_210 = (undefined *)0x14155dbd0;
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                ();
    }
    *(undefined8 *)(*(longlong *)(local_178 + 0x28) + lVar25 * 8) = uVar4;
    *(longlong *)(local_178 + 0x30) = lVar25 + 1;
    *(longlong *)(local_178 + 0x40) = *(longlong *)(local_178 + 0x40) + 1;
  }
  pbVar29 = pbVar23 + 0x10;
  if ((char)puVar20 != '\0') {
    local_88 = *(undefined8 *)pbVar29;
    uStack_80 = *(undefined8 *)(pbVar23 + 0x18);
    fVar39 = (float)*(undefined8 *)(pbVar23 + 0x20) + ___xmm_00000000000000003f0f5c293f0f5c29;
    fVar44 = (float)((ulonglong)*(undefined8 *)(pbVar23 + 0x20) >> 0x20) + _UNK_1429df1d4;
    cVar19 = '\0';
    if ((char)*(undefined4 *)(pbVar24 + 0x2c) != '\x02') {
      cVar19 = (char)*(undefined4 *)(pbVar24 + 0x2c);
    }
    local_158 = (float)*(undefined8 *)pbVar29;
    uStack_154 = (undefined4)((ulonglong)*(undefined8 *)pbVar29 >> 0x20);
    fStack_150 = (float)*(undefined8 *)(pbVar23 + 0x18);
    uStack_14c = (undefined4)((ulonglong)*(undefined8 *)(pbVar23 + 0x18) >> 0x20);
    *(undefined4 *)in_stack_00000068[2] = *(undefined4 *)((longlong)in_stack_00000068 + 0x1c);
    lVar25 = *in_stack_00000068;
    lVar10 = *(longlong *)(lVar25 + 0x30);
    uVar4 = *(undefined8 *)(lVar25 + 0x40);
    local_1b8 = pbVar34;
    if (lVar10 == *(longlong *)(lVar25 + 0x20)) {
      puStack_210 = (undefined *)0x14155dc62;
      alloc::raw_vec::RawVec::
      grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                ();
    }
    lVar13 = local_90;
    lVar12 = *(longlong *)(lVar25 + 0x28);
    *(undefined4 *)(lVar12 + lVar10 * 0x28) = 1;
    puVar1 = (undefined8 *)(lVar12 + 4 + lVar10 * 0x28);
    *puVar1 = CONCAT44(uStack_154,local_158);
    puVar1[1] = CONCAT44(uStack_14c,fStack_150);
    *(ulonglong *)(lVar12 + 0x14 + lVar10 * 0x28) = CONCAT44(fVar44,fVar39);
    *(char *)(lVar12 + 0x1c + lVar10 * 0x28) = cVar19;
    *(undefined8 *)(lVar12 + 0x20 + lVar10 * 0x28) = uVar4;
    *(longlong *)(lVar25 + 0x30) = lVar10 + 1;
    *(longlong *)(lVar25 + 0x40) = *(longlong *)(lVar25 + 0x40) + 1;
    uVar43 = *(undefined4 *)(pbVar24 + 0x3c);
    *local_98 = local_168;
    lVar25 = *(longlong *)(local_90 + 0x30);
    puVar20 = *(undefined **)(local_90 + 0x40);
    if (lVar25 == *(longlong *)(local_90 + 0x20)) {
      puStack_210 = (undefined *)0x14155dcd4;
      alloc::raw_vec::RawVec::
      grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>();
    }
    lVar10 = *(longlong *)(lVar13 + 0x28);
    lVar12 = lVar25 * 0x30;
    *(undefined4 *)(lVar10 + lVar12) = 1;
    puVar1 = (undefined8 *)(lVar10 + 4 + lVar12);
    *puVar1 = local_88;
    puVar1[1] = uStack_80;
    *(ulonglong *)(lVar10 + 0x14 + lVar12) = CONCAT44(fVar44,fVar39);
    *(undefined4 *)(lVar10 + 0x1c + lVar12) = uVar43;
    *(undefined4 *)(lVar10 + 0x20 + lVar12) = 0x40800000;
    *(undefined1 *)(lVar10 + 0x24 + lVar12) = 0;
    *(undefined **)(lVar10 + 0x28 + lVar12) = puVar20;
    *(longlong *)(lVar13 + 0x30) = lVar25 + 1;
    *(longlong *)(lVar13 + 0x40) = *(longlong *)(lVar13 + 0x40) + 1;
    pbVar34 = local_1b8;
  }
  lVar25 = in_stack_00000060[1];
  uVar31 = *(ulonglong *)*in_stack_00000060;
  uVar8 = 0;
  uVar32 = uVar31 - *(ulonglong *)(lVar25 + 0x18);
  if (uVar31 < *(ulonglong *)(lVar25 + 0x18)) {
    uVar32 = uVar8;
  }
  uVar27 = uVar31 - *(ulonglong *)(lVar25 + 0x38);
  if (uVar31 < *(ulonglong *)(lVar25 + 0x38)) {
    uVar27 = uVar8;
  }
  bVar35 = *(ulonglong *)(lVar25 + 0x30) < uVar27;
  uVar31 = *(ulonglong *)(lVar25 + 0x30) - uVar27;
  bVar37 = uVar31 == 0;
  if (bVar35) {
    uVar31 = uVar8;
  }
  pbVar28 = (byte *)CONCAT71((int7)(uVar27 >> 8),bVar35 || bVar37);
  bVar36 = *(ulonglong *)(lVar25 + 0x10) < uVar32;
  uVar32 = *(ulonglong *)(lVar25 + 0x10) - uVar32;
  bVar38 = uVar32 == 0;
  if (bVar36) {
    uVar32 = uVar8;
  }
  puVar14 = (undefined *)
            CONCAT71((int7)((ulonglong)lVar25 >> 8),(bVar36 || bVar38) && (bVar35 || bVar37));
  uVar31 = uVar32 + uVar31;
  if (!bVar36 && !bVar38 || !bVar35 && !bVar37) {
    uVar31 = uVar8;
  }
  uVar31 = *(longlong *)(lVar25 + 0x40) - uVar31;
  *(ulonglong *)*in_stack_00000060 = uVar31;
  bVar7 = UNK_142c6e865;
  if (1 < *(byte *)(local_190 + 0x18)) {
    if (*(char **)(local_190 + 0x38) == local_198) {
      uVar8 = (ulonglong)(byte)(*(char *)(local_190 + 0x19) - 2);
    }
    else {
      uVar8 = 0;
    }
  }
  bVar15 = 3;
  if ((byte)uVar8 < 3) {
    bVar15 = (byte)uVar8;
  }
  puVar17 = &DAT_142c6e864;
  puVar9 = &DAT_142c6e864 + *(int *)(&DAT_142c6e864 + (ulonglong)bVar15 * 4);
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
  switch(bVar15) {
  case 0:
    if ((bVar36 || bVar38) && (bVar35 || bVar37)) {
      return;
    }
    puStack_210 = &UNK_14155de5b;
    ::utils::swizzlers::impl$1::xvy();
    fVar39 = fStack_160;
    fVar44 = fStack_15c;
    fVar40 = fStack_164;
    if ((*pbVar23 & 1) != 0) {
      fVar39 = *(float *)(pbVar23 + 8);
      fVar44 = *(float *)(pbVar23 + 0xc);
      fVar40 = *(float *)(pbVar23 + 4);
    }
    fVar40 = fStack_164 - fVar40;
    fVar39 = fStack_160 - fVar39;
    fVar44 = fStack_15c - fVar44;
    fVar45 = *(float *)(*in_stack_00000070 + 0x28);
    fVar48 = *(float *)(pbVar24 + 0x3c);
    local_1d8 = (undefined7)*(undefined8 *)(pbVar24 + 0x30);
    uStack_1d1 = (undefined1)((ulonglong)*(undefined8 *)(pbVar24 + 0x30) >> 0x38);
    uStack_1d0 = *(undefined4 *)(pbVar24 + 0x38);
    uStack_144 = (undefined4)*(undefined8 *)(pbVar23 + 0x20);
    local_140 = (undefined4)((ulonglong)*(undefined8 *)(pbVar23 + 0x20) >> 0x20);
    uStack_154 = (undefined4)*(undefined8 *)pbVar29;
    fStack_150 = (float)((ulonglong)*(undefined8 *)pbVar29 >> 0x20);
    uStack_14c = (undefined4)*(undefined8 *)(pbVar23 + 0x18);
    local_148 = (undefined4)((ulonglong)*(undefined8 *)(pbVar23 + 0x18) >> 0x20);
    local_158 = 1.4013e-45;
    puStack_210 = &UNK_14155e1e2;
    ::utils::geometry::rectangle::Rectangle2d::area();
    puStack_210 = &UNK_14155e1fb;
    ::utils::inverse_lerp();
    *(undefined4 *)pbVar34 = local_1e4;
    lVar25 = *(longlong *)(lVar11 + 0x30);
    uVar26 = *(undefined8 *)(lVar11 + 0x40);
    if (lVar25 == *(longlong *)(lVar11 + 0x20)) {
      puStack_210 = &UNK_14155e228;
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    lVar10 = *(longlong *)(lVar11 + 0x28);
    lVar12 = lVar25 * 0x20;
    *(undefined1 *)(lVar10 + lVar12) = 3;
    *(byte *)(lVar10 + 1 + lVar12) = -(fVar48 == 0.0) & 1;
    *(float *)(lVar10 + 4 + lVar12) =
         SQRT(fVar44 * fVar44 + fVar39 * fVar39 + fVar40 * fVar40) / fVar45;
    *(ulonglong *)(lVar10 + 8 + lVar12) = CONCAT17(uStack_1d1,local_1d8);
    *(undefined4 *)(lVar10 + 0x10 + lVar12) = uStack_1d0;
    *(undefined4 *)(lVar10 + 0x14 + lVar12) = extraout_XMM0_Da_02;
    *(undefined8 *)(lVar10 + 0x18 + lVar12) = uVar26;
    *(longlong *)(lVar11 + 0x30) = lVar25 + 1;
    *(longlong *)(lVar11 + 0x40) = *(longlong *)(lVar11 + 0x40) + 1;
    *local_180 = local_1ac;
    pbVar23[0] = 1;
    pbVar23[1] = 0;
    pbVar23[2] = 0;
    pbVar23[3] = 0;
    *(float *)(pbVar23 + 0xc) = fStack_15c;
    *(ulonglong *)(pbVar23 + 4) = CONCAT44(fStack_160,fStack_164);
    return;
  case 1:
    uStack_1c8 = (undefined3)((uint)*(undefined4 *)(pbVar24 + 0x38) >> 8);
    uVar26 = *(undefined8 *)(pbVar24 + 0x30);
    uStack_1d1 = (undefined1)uVar26;
    uStack_1d0 = (undefined4)((ulonglong)uVar26 >> 8);
    uStack_1cc._0_3_ = (undefined3)((ulonglong)uVar26 >> 0x28);
    uStack_1cc = CONCAT13((char)*(undefined4 *)(pbVar24 + 0x38),(undefined3)uStack_1cc);
    uStack_144 = (undefined4)*(undefined8 *)(pbVar23 + 0x20);
    local_140 = (undefined4)((ulonglong)*(undefined8 *)(pbVar23 + 0x20) >> 0x20);
    uStack_154 = (undefined4)*(undefined8 *)pbVar29;
    fStack_150 = (float)((ulonglong)*(undefined8 *)pbVar29 >> 0x20);
    uStack_14c = (undefined4)*(undefined8 *)(pbVar23 + 0x18);
    local_148 = (undefined4)((ulonglong)*(undefined8 *)(pbVar23 + 0x18) >> 0x20);
    local_158 = 1.4013e-45;
    puStack_210 = &UNK_14155ded0;
    ::utils::geometry::rectangle::Rectangle2d::area();
    puStack_210 = &UNK_14155dee9;
    ::utils::inverse_lerp();
    *(undefined4 *)pbVar34 = local_1e4;
    lVar25 = *(longlong *)(lVar11 + 0x30);
    uVar26 = *(undefined8 *)(lVar11 + 0x40);
    if (lVar25 == *(longlong *)(lVar11 + 0x20)) {
      puStack_210 = &UNK_14155df16;
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    lVar10 = *(longlong *)(lVar11 + 0x28);
    lVar12 = lVar25 * 0x20;
    *(undefined1 *)(lVar10 + lVar12) = 2;
    puVar1 = (undefined8 *)(lVar10 + 1 + lVar12);
    *puVar1 = CONCAT17(uStack_1d1,local_1d8);
    puVar1[1] = CONCAT44(uStack_1cc,uStack_1d0);
    uStack_1d0 = CONCAT31(uStack_1c8,uStack_1cc._3_1_);
    uVar43 = extraout_XMM0_Da;
    goto code_r0x00014155e0cf;
  case 2:
    if (0.0 < (float)uVar26) {
      uStack_1d0 = *(undefined4 *)(pbVar24 + 0x38);
      local_1d8 = (undefined7)*(undefined8 *)(pbVar24 + 0x30);
      uStack_1d1 = (undefined1)((ulonglong)*(undefined8 *)(pbVar24 + 0x30) >> 0x38);
      uStack_144 = (undefined4)*(undefined8 *)(pbVar23 + 0x20);
      local_140 = (undefined4)((ulonglong)*(undefined8 *)(pbVar23 + 0x20) >> 0x20);
      uStack_154 = (undefined4)*(undefined8 *)pbVar29;
      fStack_150 = (float)((ulonglong)*(undefined8 *)pbVar29 >> 0x20);
      uStack_14c = (undefined4)*(undefined8 *)(pbVar23 + 0x18);
      local_148 = (undefined4)((ulonglong)*(undefined8 *)(pbVar23 + 0x18) >> 0x20);
      local_158 = 1.4013e-45;
      puStack_210 = &UNK_14155df8c;
      ::utils::geometry::rectangle::Rectangle2d::area();
      puStack_210 = &UNK_14155dfa5;
      ::utils::inverse_lerp();
      *(undefined4 *)pbVar34 = local_1e4;
      lVar25 = *(longlong *)(lVar11 + 0x30);
      uVar4 = *(undefined8 *)(lVar11 + 0x40);
      if (lVar25 == *(longlong *)(lVar11 + 0x20)) {
        puStack_210 = &UNK_14155dfd2;
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                  ();
      }
      lVar10 = *(longlong *)(lVar11 + 0x28);
      lVar12 = lVar25 * 0x20;
      *(undefined1 *)(lVar10 + lVar12) = 1;
      *(float *)(lVar10 + 4 + lVar12) = (float)uVar26;
      *(ulonglong *)(lVar10 + 8 + lVar12) = CONCAT17(uStack_1d1,local_1d8);
      *(undefined4 *)(lVar10 + 0x10 + lVar12) = uStack_1d0;
      *(undefined4 *)(lVar10 + 0x14 + lVar12) = extraout_XMM0_Da_00;
      *(undefined8 *)(lVar10 + 0x18 + lVar12) = uVar4;
      *(longlong *)(lVar11 + 0x30) = lVar25 + 1;
      *(longlong *)(lVar11 + 0x40) = *(longlong *)(lVar11 + 0x40) + 1;
    }
    fVar39 = (float)((ulonglong)uVar26 >> 0x20);
    if (fVar39 <= 0.0) {
      return;
    }
    uStack_1d0 = *(undefined4 *)(pbVar24 + 0x38);
    local_1d8 = (undefined7)*(undefined8 *)(pbVar24 + 0x30);
    uStack_1d1 = (undefined1)((ulonglong)*(undefined8 *)(pbVar24 + 0x30) >> 0x38);
    uStack_144 = (undefined4)*(undefined8 *)(pbVar23 + 0x20);
    local_140 = (undefined4)((ulonglong)*(undefined8 *)(pbVar23 + 0x20) >> 0x20);
    uStack_154 = (undefined4)*(undefined8 *)pbVar29;
    fStack_150 = (float)((ulonglong)*(undefined8 *)pbVar29 >> 0x20);
    uStack_14c = (undefined4)*(undefined8 *)(pbVar23 + 0x18);
    local_148 = (undefined4)((ulonglong)*(undefined8 *)(pbVar23 + 0x18) >> 0x20);
    local_158 = 1.4013e-45;
    puStack_210 = &UNK_14155e065;
    ::utils::geometry::rectangle::Rectangle2d::area();
    puStack_210 = &UNK_14155e07e;
    ::utils::inverse_lerp();
    *(undefined4 *)pbVar34 = local_1e4;
    lVar25 = *(longlong *)(lVar11 + 0x30);
    uVar26 = *(undefined8 *)(lVar11 + 0x40);
    if (lVar25 == *(longlong *)(lVar11 + 0x20)) {
      puStack_210 = &UNK_14155e0ab;
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    lVar10 = *(longlong *)(lVar11 + 0x28);
    lVar12 = lVar25 * 0x20;
    *(undefined1 *)(lVar10 + lVar12) = 0;
    *(float *)(lVar10 + 4 + lVar12) = fVar39;
    *(ulonglong *)(lVar10 + 8 + lVar12) = CONCAT17(uStack_1d1,local_1d8);
    uVar43 = extraout_XMM0_Da_01;
code_r0x00014155e0cf:
    *(undefined4 *)(lVar10 + 0x10 + lVar12) = uStack_1d0;
    *(undefined4 *)(lVar10 + 0x14 + lVar12) = uVar43;
    *(undefined8 *)(lVar10 + 0x18 + lVar12) = uVar26;
    *(longlong *)(lVar11 + 0x30) = lVar25 + 1;
    *(longlong *)(lVar11 + 0x40) = *(longlong *)(lVar11 + 0x40) + 1;
LAB_14155e297:
    return;
  case 3:
    goto LAB_14155e297;
  case 4:
    puVar14 = &DAT_142c6e874;
    puVar9 = (undefined *)(longlong)*(int *)(&DAT_142c6e874 + (longlong)puVar9 * 4);
  case 8:
                    /* WARNING: Could not recover jumptable at 0x000141560951. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)(puVar14 + (longlong)puVar9))();
    return;
  case 5:
    *(undefined **)(pbVar23 + 0x40) = puVar9;
    *(undefined8 *)(pbVar23 + 0x48) = *(undefined8 *)(pbVar23 + 8);
    puVar9 = (undefined *)0xc;
    ppuVar22 = (undefined **)auStack_208;
    break;
  case 6:
    pbVar34 = pbVar24 + 0x142c64564;
    *pbVar34 = *pbVar34 ^ (byte)((ulonglong)puVar20 >> 8);
    if (*pbVar34 != 0) break;
    plVar18 = *(longlong **)(pbVar23 + 0x38);
    goto code_r0x000141560ba4;
  case 7:
    *pbVar28 = *pbVar28 ^ (byte)uVar31;
    *(undefined **)(pbVar23 + 0x38) = puVar9;
    puVar9 = (undefined *)0x8;
    ppuVar22 = (undefined **)auStack_208;
    break;
  case 9:
    pbVar23[0x30] = 0;
    pbVar23[0x31] = bVar7;
    puVar9 = (undefined *)0x0;
    ppuVar22 = &puStack_210;
    puStack_210 = puVar20;
    break;
  case 10:
                    /* WARNING: Bad instruction - Truncating control flow here */
    halt_baddata();
  case 0xb:
    goto code_r0x000141560dfa;
  case 0xc:
                    /* WARNING: Could not emulate address calculation at 0x000141560de4 */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)(&DAT_142c6e864 + *(uint *)(&DAT_142c6e864 + (longlong)puVar9 * 4)))();
    return;
  case 0xd:
code_r0x000141560dfa:
    puStack_210 = &UNK_141560dff;
    resources::walls::_::impl$0::deserialize::impl$0::
    visit_newtype_struct<serde::__private::de::content::ContentRefDeserializer<serde_json::error::Error>_>
              ();
    *(undefined **)(pbVar24 + 8) = puVar17;
    *(ulonglong *)pbVar24 = (ulonglong)(extraout_EAX & 1) * 3;
    return;
  case 0xe:
  case 0x10:
    return;
  default:
                    /* WARNING: Bad instruction - Truncating control flow here */
    halt_baddata();
  }
  *(undefined4 *)(pbVar23 + 0x14) = *(undefined4 *)(pbVar23 + 0x34);
  *(undefined4 *)(pbVar23 + 0x11) = *(undefined4 *)(pbVar23 + 0x31);
  *(undefined8 *)(pbVar23 + 0x20) = *(undefined8 *)(pbVar23 + 0x40);
  *(undefined8 *)(pbVar23 + 0x28) = *(undefined8 *)(pbVar23 + 0x48);
  pbVar23[0x10] = (byte)puVar9;
  *(undefined8 *)(pbVar23 + 0x18) = *(undefined8 *)(pbVar23 + 0x38);
  *(undefined ***)((longlong)ppuVar22 + 0x20) =
       &anon_36890fc008bdb4b97258d46f0ec99221_129_llvm_10723592867021123337;
  *(undefined8 *)((longlong)ppuVar22 + 0x28) = 3;
  *(undefined **)((longlong)ppuVar22 + -8) = &UNK_141560a92;
  serde::__private::de::content::impl$21::
  deserialize_enum<serde_json::error::Error,country_core::resources::walls::decorator_storage::_::impl_0::deserialize::__Visitor>
            ();
  lVar25 = *(longlong *)(pbVar23 + 0x30);
  plVar18 = *(longlong **)(pbVar23 + 0x38);
  if (lVar25 == 3) {
    lVar25 = *plVar18;
    *(longlong **)(pbVar23 + 0x50) = plVar18;
    if (lVar25 == 1) {
      *(undefined **)((longlong)ppuVar22 + -8) = &UNK_141560af9;
      core::ptr::drop_in_place<std::io::error::Error>();
    }
    else if ((lVar25 == 0) && (*(longlong *)(*(longlong *)(pbVar23 + 0x50) + 0x10) != 0)) {
      *(undefined **)((longlong)ppuVar22 + -8) = &UNK_141560ad2;
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    plVar18 = (longlong *)&DAT_00000028;
    *(undefined **)((longlong)ppuVar22 + -8) = &UNK_141560b0d;
    std::alloc::__default_lib_allocator::__rust_dealloc();
    *(undefined **)((longlong)ppuVar22 + -8) = &UNK_141560b1f;
    resources::walls::_::impl$0::deserialize::impl$0::
    visit_newtype_struct<serde::__private::de::content::ContentRefDeserializer<serde_json::error::Error>_>
              ();
    if ((extraout_RAX_00 & 1) == 0) {
      *(undefined **)((longlong)ppuVar22 + -8) = &UNK_141560b63;
      core::ptr::drop_in_place<enum2_<serde::__private::de::content::Content>_>();
      lVar11 = 0;
    }
    else {
      lVar25 = *plVar18;
      *(longlong **)(pbVar23 + 0x50) = plVar18;
      if (lVar25 == 1) {
        *(undefined **)((longlong)ppuVar22 + -8) = &UNK_141560b73;
        core::ptr::drop_in_place<std::io::error::Error>();
      }
      else if ((lVar25 == 0) && (*(longlong *)(*(longlong *)(pbVar23 + 0x50) + 0x10) != 0)) {
        *(undefined **)((longlong)ppuVar22 + -8) = &UNK_141560b58;
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      *(undefined **)((longlong)ppuVar22 + -8) = &UNK_141560b87;
      std::alloc::__default_lib_allocator::__rust_dealloc();
      *(undefined **)((longlong)ppuVar22 + -8) = &UNK_141560b98;
      serde_json::error::impl$8::custom<ref_<str_>_>();
      *(undefined **)((longlong)ppuVar22 + -8) = &UNK_141560ba4;
      core::ptr::drop_in_place<enum2_<serde::__private::de::content::Content>_>();
      plVar18 = extraout_RAX_01;
code_r0x000141560ba4:
      lVar11 = 3;
    }
  }
  else {
    *(undefined **)((longlong)ppuVar22 + -8) = &UNK_141560add;
    core::ptr::drop_in_place<enum2_<serde::__private::de::content::Content>_>();
    lVar11 = 0;
    if (lVar25 - 1U < 2) {
      lVar11 = lVar25;
    }
  }
  *(longlong **)(pbVar24 + 8) = plVar18;
  *(longlong *)pbVar24 = lVar11;
  return;
}

