
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::systems::wall::shapes::circle::ui_create_circle::ui_create_circle(void)

{
  byte *pbVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  int iVar9;
  byte *pbVar10;
  undefined4 *puVar11;
  undefined4 *puVar12;
  undefined4 *puVar13;
  undefined4 *puVar14;
  undefined4 *puVar15;
  undefined4 *puVar16;
  longlong lVar17;
  longlong lVar18;
  undefined8 uVar19;
  longlong lVar20;
  undefined4 *puVar21;
  undefined4 *puVar22;
  float *pfVar23;
  void *_Src;
  size_t _Size;
  uint uVar24;
  byte bVar25;
  char extraout_AL;
  char extraout_AL_00;
  char cVar26;
  uint uVar27;
  longlong extraout_RAX;
  ulonglong uVar28;
  undefined *puVar29;
  longlong lVar30;
  longlong extraout_RAX_00;
  undefined8 extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  void *extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  bool bVar31;
  undefined8 *in_RCX;
  longlong lVar32;
  longlong lVar33;
  longlong lVar34;
  longlong lVar35;
  longlong lVar36;
  longlong *plVar37;
  undefined4 uVar38;
  longlong *in_RDX;
  char *pcVar39;
  undefined4 *puVar40;
  longlong lVar41;
  undefined8 *puVar42;
  void *_Dst;
  undefined8 uVar43;
  undefined8 *in_R8;
  longlong lVar44;
  ulonglong uVar45;
  undefined4 *puVar46;
  undefined8 *in_R9;
  ulonglong uVar47;
  ushort uVar48;
  ulonglong uVar49;
  undefined1 (*pauVar50) [16];
  undefined8 *puVar51;
  char cVar52;
  undefined1 (*pauVar53) [16];
  bool bVar54;
  bool bVar55;
  bool bVar56;
  bool bVar57;
  float extraout_XMM0_Da;
  undefined4 extraout_XMM0_Da_00;
  undefined4 extraout_XMM0_Da_01;
  undefined4 extraout_XMM0_Da_02;
  undefined4 extraout_XMM0_Da_03;
  float fVar58;
  float extraout_XMM0_Da_04;
  float fVar60;
  undefined1 extraout_XMM0 [16];
  undefined1 extraout_XMM0_00 [16];
  undefined1 extraout_XMM0_01 [16];
  undefined1 extraout_XMM0_02 [16];
  undefined1 auVar59 [16];
  float fVar61;
  float fVar62;
  undefined1 in_XMM1 [16];
  float fVar63;
  float fVar64;
  float fVar65;
  float fVar67;
  undefined1 auVar66 [16];
  float fVar68;
  undefined1 auVar69 [16];
  float unaff_XMM6_Da;
  undefined4 uVar70;
  uint uVar71;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  longlong *in_stack_00000040;
  undefined8 *in_stack_00000048;
  longlong *in_stack_00000050;
  longlong in_stack_00000058;
  longlong *in_stack_00000060;
  undefined7 local_208;
  undefined1 uStack_201;
  undefined4 uStack_200;
  undefined3 uStack_1fc;
  float fStack_184;
  float fStack_180;
  float fStack_17c;
  undefined4 local_128;
  undefined4 uStack_124;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined4 local_118;
  undefined4 uStack_114;
  undefined4 local_110;
  undefined4 local_f8;
  undefined4 local_e0;
  undefined4 local_c8;
  undefined1 local_b0;
  undefined1 local_a0;
  undefined4 local_9c;
  undefined2 local_80;
  
  pbVar10 = (byte *)in_stack_00000030[4];
  puVar46 = (undefined4 *)in_stack_00000030[6];
  uVar38 = *(undefined4 *)((longlong)in_stack_00000030 + 0x3c);
  if ((*(int *)*in_RCX != 6) || ((char)((int *)*in_RCX)[1] != '\x01')) {
    *puVar46 = uVar38;
    pbVar10[0] = 0;
    pbVar10[1] = 0;
    pbVar10[2] = 0;
    pbVar10[3] = 0;
    return;
  }
  lVar44 = *in_stack_00000030;
  lVar30 = in_stack_00000050[4];
  puVar11 = (undefined4 *)in_stack_00000050[6];
  uVar70 = *(undefined4 *)((longlong)in_stack_00000050 + 0x3c);
  lVar41 = in_stack_00000050[8];
  lVar36 = *(longlong *)(in_stack_00000058 + 0x20);
  puVar12 = *(undefined4 **)(in_stack_00000058 + 0x30);
  uVar2 = *(undefined4 *)(in_stack_00000058 + 0x3c);
  lVar35 = *(longlong *)(in_stack_00000058 + 0x40);
  puVar13 = *(undefined4 **)(in_stack_00000058 + 0x50);
  uVar3 = *(undefined4 *)(in_stack_00000058 + 0x5c);
  local_128 = 0;
  uStack_124 = 0;
  uStack_120 = 8;
  uStack_11c = 0;
  local_118 = 0;
  uStack_114 = 0;
  local_110 = 0;
  local_f8 = 0;
  local_e0 = 0;
  local_c8 = 0;
  local_b0 = 0;
  local_a0 = 5;
  local_9c = 0x7f7fffff;
  local_80 = 0x100;
  puVar14 = (undefined4 *)in_stack_00000050[10];
  uVar4 = *(undefined4 *)((longlong)in_stack_00000050 + 0x5c);
  lVar34 = in_stack_00000050[0xc];
  puVar15 = (undefined4 *)in_stack_00000050[0xe];
  uVar5 = *(undefined4 *)((longlong)in_stack_00000050 + 0x7c);
  lVar33 = in_stack_00000050[0x10];
  puVar40 = (undefined4 *)in_stack_00000050[0x12];
  uVar6 = *(undefined4 *)((longlong)in_stack_00000050 + 0x9c);
  lVar32 = in_stack_00000050[0x14];
  puVar16 = (undefined4 *)in_stack_00000050[0x16];
  uVar7 = *(undefined4 *)((longlong)in_stack_00000050 + 0xbc);
  pcVar39 = "wall_circle_canvascursor_circle_";
  systems::ui::resources::ui_state::UiState::add<ref_<str_>_>();
  if (__real_42700000 <= *(float *)(lVar44 + 0x3c)) {
    local_128 = (undefined4)*(undefined8 *)(pbVar10 + 0x10);
    uStack_124 = (undefined4)((ulonglong)*(undefined8 *)(pbVar10 + 0x10) >> 0x20);
    uStack_120 = (undefined4)*(undefined8 *)(pbVar10 + 0x18);
    uStack_11c = (undefined4)((ulonglong)*(undefined8 *)(pbVar10 + 0x18) >> 0x20);
    *(undefined4 *)in_stack_00000060[2] = *(undefined4 *)((longlong)in_stack_00000060 + 0x1c);
    lVar17 = *in_stack_00000060;
    lVar18 = *(longlong *)(lVar17 + 0x30);
    uVar19 = *(undefined8 *)(lVar17 + 0x40);
    if (lVar18 == *(longlong *)(lVar17 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                ();
    }
    lVar20 = *(longlong *)(lVar17 + 0x28);
    *(undefined8 *)(lVar20 + lVar18 * 0x28) = uVar19;
    *(undefined4 *)(lVar20 + 8 + lVar18 * 0x28) = 0;
    puVar42 = (undefined8 *)(lVar20 + 0xc + lVar18 * 0x28);
    *puVar42 = CONCAT44(uStack_124,local_128);
    puVar42[1] = CONCAT44(uStack_11c,uStack_120);
    *(longlong *)(lVar17 + 0x30) = lVar18 + 1;
    *(longlong *)(lVar17 + 0x40) = *(longlong *)(lVar17 + 0x40) + 1;
  }
  cVar52 = '\x02';
  if (1 < *(byte *)(extraout_RAX + 0x18)) {
    if ((*(char **)(extraout_RAX + 0x38) == pcVar39) && (*(char *)(extraout_RAX + 0x19) == '\x03'))
    {
      uVar8 = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
      puVar21 = (undefined4 *)in_stack_00000040[2];
      *puVar21 = uVar8;
      puVar22 = (undefined4 *)*in_stack_00000040;
      *(undefined1 *)(puVar22 + 3) = 0;
      *(undefined8 *)(puVar22 + 1) = *(undefined8 *)*in_R8;
      *puVar21 = uVar8;
      *puVar22 = 0;
      *puVar16 = uVar7;
      lVar17 = *(longlong *)(lVar32 + 0x30);
      uVar19 = *(undefined8 *)(lVar32 + 0x40);
      if (lVar17 == *(longlong *)(lVar32 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                  ();
      }
      *(undefined8 *)(*(longlong *)(lVar32 + 0x28) + lVar17 * 8) = uVar19;
      *(longlong *)(lVar32 + 0x30) = lVar17 + 1;
      *(longlong *)(lVar32 + 0x40) = *(longlong *)(lVar32 + 0x40) + 1;
      if (*(byte *)(extraout_RAX + 0x18) < 2) goto LAB_14165c423;
    }
    if ((*(char **)(extraout_RAX + 0x38) == pcVar39) && (*(char *)(extraout_RAX + 0x19) == '\x04'))
    {
      fVar61 = (float)*(undefined8 *)(*in_RDX + 0x140);
      fVar60 = (float)((ulonglong)*(undefined8 *)(*in_RDX + 0x140) >> 0x20);
      fVar60 = fVar60 * fVar60;
      in_XMM1._0_4_ = fVar60 + fVar61 * fVar61;
      in_XMM1._4_4_ = fVar60;
      in_XMM1._8_8_ = 0;
      uVar8 = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
      puVar21 = (undefined4 *)in_stack_00000040[2];
      *puVar21 = uVar8;
      pfVar23 = (float *)*in_stack_00000040;
      fVar60 = *pfVar23;
      *pfVar23 = SQRT(in_XMM1._0_4_) + fVar60;
      if (__real_40800000 < SQRT(in_XMM1._0_4_) + fVar60) {
        *puVar21 = uVar8;
        *(undefined1 *)(pfVar23 + 3) = 1;
      }
      *puVar16 = uVar7;
      lVar17 = *(longlong *)(lVar32 + 0x30);
      uVar19 = *(undefined8 *)(lVar32 + 0x40);
      if (lVar17 == *(longlong *)(lVar32 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                  ();
      }
      *(undefined8 *)(*(longlong *)(lVar32 + 0x28) + lVar17 * 8) = uVar19;
      *(longlong *)(lVar32 + 0x30) = lVar17 + 1;
      *(longlong *)(lVar32 + 0x40) = *(longlong *)(lVar32 + 0x40) + 1;
      if (*(byte *)(extraout_RAX + 0x18) < 2) goto LAB_14165c423;
    }
    if ((*(char **)(extraout_RAX + 0x38) == pcVar39) && (*(byte *)(extraout_RAX + 0x19) < 2)) {
      unaff_XMM6_Da = *(float *)(*in_stack_00000040 + 4);
      fVar60 = *(float *)(*in_stack_00000040 + 8);
      *puVar40 = uVar6;
      lVar32 = *(longlong *)(lVar33 + 0x30);
      uVar19 = *(undefined8 *)(lVar33 + 0x40);
      if (lVar32 == *(longlong *)(lVar33 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      lVar17 = *(longlong *)(lVar33 + 0x28);
      *(undefined8 *)(lVar17 + lVar32 * 0x18) = uVar19;
      *(undefined4 *)(lVar17 + 8 + lVar32 * 0x18) = 2;
      pfVar23 = (float *)(lVar17 + 0xc + lVar32 * 0x18);
      *pfVar23 = unaff_XMM6_Da;
      pfVar23[1] = fVar60;
      *(longlong *)(lVar33 + 0x30) = lVar32 + 1;
      *(longlong *)(lVar33 + 0x40) = *(longlong *)(lVar33 + 0x40) + 1;
      if (*(byte *)(extraout_RAX + 0x18) < 2) goto LAB_14165c423;
    }
    if (*(char **)(extraout_RAX + 0x38) == pcVar39) {
      cVar52 = *(char *)(extraout_RAX + 0x19);
    }
  }
LAB_14165c423:
  bVar25 = 3;
  if ((byte)(cVar52 - 2U) < 3) {
    bVar25 = cVar52 - 2U;
  }
  if (bVar25 < 2) {
    uVar3 = *(undefined4 *)(lVar44 + 0x30);
    uVar71 = *(uint *)(lVar44 + 0x38);
    in_XMM1 = ZEXT416(uVar71);
    *puVar46 = uVar38;
    *(undefined4 *)(pbVar10 + 0x10) = uVar3;
    *(uint *)(pbVar10 + 0x14) = uVar71;
    uVar3 = *(undefined4 *)(lVar44 + 0x34);
    *puVar46 = uVar38;
    *(undefined4 *)(pbVar10 + 0x20) = uVar3;
    local_208 = (undefined7)*(undefined8 *)(pbVar10 + 0x10);
    bVar54 = true;
    circle_signifier();
    append_only_vec::AppendOnlyVec::
    push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>();
    uVar71 = 0;
  }
  else if (bVar25 == 2) {
    uVar71 = 0;
    bVar54 = true;
    if (*(char *)(*in_stack_00000040 + 0xc) != '\0') {
      pfVar23 = (float *)*in_R9;
      fVar60 = (*(float *)(lVar44 + 0x34) - pfVar23[1]) / pfVar23[4];
      fVar61 = (float)*(undefined8 *)(pbVar10 + 0x10) - (fVar60 * pfVar23[3] + *pfVar23);
      fVar60 = (float)((ulonglong)*(undefined8 *)(pbVar10 + 0x10) >> 0x20) -
               (fVar60 * pfVar23[5] + pfVar23[2]);
      fVar61 = SQRT(fVar60 * fVar60 + fVar61 * fVar61);
      fVar60 = __real_3f666667;
      if (__real_3f666667 <= fVar61) {
        fVar60 = fVar61;
      }
      in_XMM1 = ZEXT416((uint)fVar60);
      fVar61 = __real_41000000;
      if (fVar60 <= __real_41000000) {
        fVar61 = fVar60;
      }
      fVar60 = *(float *)(pbVar10 + 0x18);
      *puVar46 = uVar38;
      *(float *)(pbVar10 + 0x18) = fVar61;
      uVar71 = (uint)(fVar61 - fVar60) & ___xmm_7fffffff7fffffff7fffffff7fffffff;
    }
    local_208 = (undefined7)*(undefined8 *)(pbVar10 + 0x10);
    circle_signifier();
    append_only_vec::AppendOnlyVec::
    push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>();
  }
  else {
    bVar54 = true;
    uVar71 = 0;
    if (*(char *)(*in_stack_00000040 + 0xc) != '\x01') {
      uStack_124 = (undefined4)*(undefined8 *)(pbVar10 + 0x10);
      uStack_120 = (undefined4)((ulonglong)*(undefined8 *)(pbVar10 + 0x10) >> 0x20);
      uStack_11c = (undefined4)*(undefined8 *)(pbVar10 + 0x18);
      local_118 = (undefined4)((ulonglong)*(undefined8 *)(pbVar10 + 0x18) >> 0x20);
      local_128 = 0;
      puVar40 = &local_128;
      resources::glade_border::GladeBorder::is_shape_inside();
      if (extraout_AL == '\0') {
        *puVar13 = uVar3;
        lVar34 = *(longlong *)(lVar35 + 0x30);
        uVar19 = *(undefined8 *)(lVar35 + 0x40);
        if (lVar34 == *(longlong *)(lVar35 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                    ();
        }
        *(undefined8 *)(*(longlong *)(lVar35 + 0x28) + lVar34 * 8) = uVar19;
        *(longlong *)(lVar35 + 0x30) = lVar34 + 1;
        *(longlong *)(lVar35 + 0x40) = *(longlong *)(lVar35 + 0x40) + 1;
        bVar54 = false;
      }
      else {
        bVar56 = *(char *)(lVar44 + 0x2c) != '\x02';
        if (bVar56) {
          puVar40 = *(undefined4 **)(lVar44 + 0x20);
        }
        uVar3 = *(undefined4 *)(lVar44 + 0x40);
        uVar6 = *(undefined4 *)(lVar44 + 0x3c);
        uVar19 = *(undefined8 *)(pbVar10 + 0x10);
        local_128 = (undefined4)uVar19;
        uStack_124 = (undefined4)((ulonglong)uVar19 >> 0x20);
        uStack_120 = (undefined4)*(undefined8 *)(pbVar10 + 0x18);
        uStack_11c = (undefined4)((ulonglong)*(undefined8 *)(pbVar10 + 0x18) >> 0x20);
        *puVar15 = uVar5;
        lVar35 = *(longlong *)(lVar34 + 0x30);
        uVar19 = *(undefined8 *)(lVar34 + 0x40);
        if (lVar35 == *(longlong *)(lVar34 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>
                    ();
        }
        lVar33 = *(longlong *)(lVar34 + 0x28);
        lVar32 = lVar35 * 0x30;
        *(ulonglong *)(lVar33 + lVar32) = (ulonglong)bVar56;
        *(undefined4 **)(lVar33 + 8 + lVar32) = puVar40;
        puVar42 = (undefined8 *)(lVar33 + 0x10 + lVar32);
        *puVar42 = CONCAT44(uStack_124,local_128);
        puVar42[1] = CONCAT44(uStack_11c,uStack_120);
        *(undefined4 *)(lVar33 + 0x20 + lVar32) = uVar3;
        *(undefined4 *)(lVar33 + 0x24 + lVar32) = uVar6;
        *(undefined8 *)(lVar33 + 0x28 + lVar32) = uVar19;
        *(longlong *)(lVar34 + 0x30) = lVar35 + 1;
        *(longlong *)(lVar34 + 0x40) = *(longlong *)(lVar34 + 0x40) + 1;
        uVar3 = *(undefined4 *)(lVar44 + 0x38);
        uVar19 = *(undefined8 *)(lVar44 + 0x30);
        uStack_201 = (undefined1)uVar19;
        uStack_200 = (undefined4)((ulonglong)uVar19 >> 8);
        uStack_1fc = (undefined3)((ulonglong)uVar19 >> 0x28);
        uVar19 = *(undefined8 *)(pbVar10 + 0x10);
        uStack_124 = (undefined4)uVar19;
        uStack_120 = (undefined4)((ulonglong)uVar19 >> 0x20);
        uStack_11c = (undefined4)*(undefined8 *)(pbVar10 + 0x18);
        local_118 = (undefined4)((ulonglong)*(undefined8 *)(pbVar10 + 0x18) >> 0x20);
        local_128 = 0;
        ::utils::geometry::circle::Circle2d::area();
        in_XMM1 = ZEXT416(__real_43490fdb);
        ::utils::inverse_lerp();
        *puVar14 = uVar4;
        lVar35 = *(longlong *)(lVar41 + 0x30);
        uVar19 = *(undefined8 *)(lVar41 + 0x40);
        if (lVar35 == *(longlong *)(lVar41 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                    ();
        }
        lVar34 = *(longlong *)(lVar41 + 0x28);
        lVar33 = lVar35 * 0x20;
        *(undefined1 *)(lVar34 + lVar33) = 7;
        puVar42 = (undefined8 *)(lVar34 + 1 + lVar33);
        *puVar42 = CONCAT17(uStack_201,local_208);
        puVar42[1] = CONCAT44(CONCAT13((char)uVar3,uStack_1fc),uStack_200);
        *(undefined4 *)(lVar34 + 0x10 + lVar33) = uVar3;
        *(float *)(lVar34 + 0x14 + lVar33) = extraout_XMM0_Da;
        *(undefined8 *)(lVar34 + 0x18 + lVar33) = uVar19;
        *(longlong *)(lVar41 + 0x30) = lVar35 + 1;
        *(longlong *)(lVar41 + 0x40) = *(longlong *)(lVar41 + 0x40) + 1;
        unaff_XMM6_Da = extraout_XMM0_Da;
      }
    }
  }
  uStack_124 = (undefined4)*(undefined8 *)(pbVar10 + 0x10);
  uStack_120 = (undefined4)((ulonglong)*(undefined8 *)(pbVar10 + 0x10) >> 0x20);
  uStack_11c = (undefined4)*(undefined8 *)(pbVar10 + 0x18);
  local_118 = (undefined4)((ulonglong)*(undefined8 *)(pbVar10 + 0x18) >> 0x20);
  local_128 = 0;
  resources::glade_border::GladeBorder::is_shape_inside();
  auVar59 = extraout_XMM0;
  if (extraout_AL_00 == '\0') {
    *puVar12 = uVar2;
    lVar35 = *(longlong *)(lVar36 + 0x30);
    uVar19 = *(undefined8 *)(lVar36 + 0x40);
    if (lVar35 == *(longlong *)(lVar36 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                ();
      auVar59 = extraout_XMM0_00;
    }
    *(undefined8 *)(*(longlong *)(lVar36 + 0x28) + lVar35 * 8) = uVar19;
    *(longlong *)(lVar36 + 0x30) = lVar35 + 1;
    *(longlong *)(lVar36 + 0x40) = *(longlong *)(lVar36 + 0x40) + 1;
  }
  if (bVar54) {
    uVar19 = *(undefined8 *)(pbVar10 + 0x10);
    uVar2 = *(undefined4 *)(pbVar10 + 0x1c);
    unaff_XMM6_Da = *(float *)(pbVar10 + 0x18) + __real_3e8f5c29;
    cVar26 = (char)*(undefined4 *)(lVar44 + 0x2c);
    cVar52 = '\0';
    if (cVar26 != '\x02') {
      cVar52 = cVar26;
    }
    *(undefined4 *)in_stack_00000050[2] = *(undefined4 *)((longlong)in_stack_00000050 + 0x1c);
    lVar36 = *in_stack_00000050;
    lVar35 = *(longlong *)(lVar36 + 0x30);
    uVar43 = *(undefined8 *)(lVar36 + 0x40);
    if (lVar35 == *(longlong *)(lVar36 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                ();
      auVar59 = extraout_XMM0_01;
    }
    lVar34 = *(longlong *)(lVar36 + 0x28);
    *(undefined4 *)(lVar34 + lVar35 * 0x28) = 0;
    *(undefined8 *)(lVar34 + 4 + lVar35 * 0x28) = uVar19;
    *(float *)(lVar34 + 0xc + lVar35 * 0x28) = unaff_XMM6_Da;
    *(undefined4 *)(lVar34 + 0x10 + lVar35 * 0x28) = uVar2;
    *(char *)(lVar34 + 0x1c + lVar35 * 0x28) = cVar52;
    *(undefined8 *)(lVar34 + 0x20 + lVar35 * 0x28) = uVar43;
    *(longlong *)(lVar36 + 0x30) = lVar35 + 1;
    *(longlong *)(lVar36 + 0x40) = *(longlong *)(lVar36 + 0x40) + 1;
    uVar3 = *(undefined4 *)(lVar44 + 0x3c);
    *puVar11 = uVar70;
    lVar36 = *(longlong *)(lVar30 + 0x30);
    uVar43 = *(undefined8 *)(lVar30 + 0x40);
    if (lVar36 == *(longlong *)(lVar30 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>();
      auVar59 = extraout_XMM0_02;
    }
    lVar35 = *(longlong *)(lVar30 + 0x28);
    lVar34 = lVar36 * 0x30;
    *(undefined4 *)(lVar35 + lVar34) = 0;
    *(undefined8 *)(lVar35 + 4 + lVar34) = uVar19;
    *(float *)(lVar35 + 0xc + lVar34) = unaff_XMM6_Da;
    *(undefined4 *)(lVar35 + 0x10 + lVar34) = uVar2;
    *(undefined4 *)(lVar35 + 0x1c + lVar34) = uVar3;
    *(undefined4 *)(lVar35 + 0x20 + lVar34) = 0x40800000;
    *(undefined1 *)(lVar35 + 0x24 + lVar34) = 0;
    *(undefined8 *)(lVar35 + 0x28 + lVar34) = uVar43;
    in_stack_00000050 = (longlong *)(lVar36 + 1);
    *(longlong **)(lVar30 + 0x30) = in_stack_00000050;
    *(longlong *)(lVar30 + 0x40) = *(longlong *)(lVar30 + 0x40) + 1;
  }
  lVar30 = in_stack_00000048[1];
  uVar47 = *(ulonglong *)*in_stack_00000048;
  uVar28 = 0;
  uVar49 = uVar47 - *(ulonglong *)(lVar30 + 0x18);
  if (uVar47 < *(ulonglong *)(lVar30 + 0x18)) {
    uVar49 = uVar28;
  }
  uVar45 = uVar47 - *(ulonglong *)(lVar30 + 0x38);
  if (uVar47 < *(ulonglong *)(lVar30 + 0x38)) {
    uVar45 = uVar28;
  }
  bVar54 = *(ulonglong *)(lVar30 + 0x30) < uVar45;
  uVar47 = *(ulonglong *)(lVar30 + 0x30) - uVar45;
  bVar56 = uVar47 == 0;
  if (bVar54) {
    uVar47 = uVar28;
  }
  uVar45 = CONCAT71((int7)(uVar45 >> 8),bVar54 || bVar56);
  bVar55 = *(ulonglong *)(lVar30 + 0x10) < uVar49;
  uVar49 = *(ulonglong *)(lVar30 + 0x10) - uVar49;
  bVar57 = uVar49 == 0;
  if (bVar55) {
    uVar49 = uVar28;
  }
  bVar31 = (bVar55 || bVar57) && (bVar54 || bVar56);
  lVar36 = CONCAT71((int7)((ulonglong)lVar30 >> 8),bVar31);
  uVar47 = uVar49 + uVar47;
  if (!bVar55 && !bVar57 || !bVar54 && !bVar56) {
    uVar47 = uVar28;
  }
  uVar47 = *(longlong *)(lVar30 + 0x40) - uVar47;
  *(ulonglong *)*in_stack_00000048 = uVar47;
  if (1 < *(byte *)(extraout_RAX + 0x18)) {
    if (*(char **)(extraout_RAX + 0x38) == pcVar39) {
      uVar28 = (ulonglong)(byte)(*(char *)(extraout_RAX + 0x19) - 2);
    }
    else {
      uVar28 = 0;
    }
  }
  pbVar1 = pbVar10 + 0x10;
  bVar25 = 3;
  if ((byte)uVar28 < 3) {
    bVar25 = (byte)uVar28;
  }
  puVar29 = &DAT_142c826bc + *(int *)(&DAT_142c826bc + (ulonglong)bVar25 * 4);
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
  switch(bVar25) {
  case 0:
    if ((bVar55 || bVar57) && (bVar54 || bVar56)) {
      return;
    }
    ::utils::swizzlers::impl$1::xvy();
    fVar60 = fStack_180;
    fVar61 = fStack_17c;
    fVar67 = fStack_184;
    if ((*pbVar10 & 1) != 0) {
      fVar60 = *(float *)(pbVar10 + 8);
      fVar61 = *(float *)(pbVar10 + 0xc);
      fVar67 = *(float *)(pbVar10 + 4);
    }
    fVar68 = *(float *)(*in_stack_00000028 + 0x28);
    fVar58 = *(float *)(lVar44 + 0x3c);
    uVar19 = *(undefined8 *)(lVar44 + 0x30);
    uVar70 = *(undefined4 *)(lVar44 + 0x38);
    uStack_124 = (undefined4)*(undefined8 *)pbVar1;
    uStack_120 = (undefined4)((ulonglong)*(undefined8 *)pbVar1 >> 0x20);
    uStack_11c = (undefined4)*(undefined8 *)(pbVar10 + 0x18);
    local_118 = (undefined4)((ulonglong)*(undefined8 *)(pbVar10 + 0x18) >> 0x20);
    local_128 = 0;
    ::utils::geometry::circle::Circle2d::area();
    ::utils::inverse_lerp();
    *puVar14 = uVar4;
    lVar44 = *(longlong *)(lVar41 + 0x30);
    uVar43 = *(undefined8 *)(lVar41 + 0x40);
    if (lVar44 == *(longlong *)(lVar41 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    lVar30 = *(longlong *)(lVar41 + 0x28);
    lVar36 = lVar44 * 0x20;
    *(undefined1 *)(lVar30 + lVar36) = 3;
    *(byte *)(lVar30 + 1 + lVar36) = -(fVar58 == 0.0) & 1;
    *(float *)(lVar30 + 4 + lVar36) =
         SQRT((fStack_17c - fVar61) * (fStack_17c - fVar61) +
              (fStack_180 - fVar60) * (fStack_180 - fVar60) +
              (fStack_184 - fVar67) * (fStack_184 - fVar67)) / fVar68;
    *(undefined8 *)(lVar30 + 8 + lVar36) = uVar19;
    *(undefined4 *)(lVar30 + 0x10 + lVar36) = uVar70;
    *(undefined4 *)(lVar30 + 0x14 + lVar36) = extraout_XMM0_Da_02;
    *(undefined8 *)(lVar30 + 0x18 + lVar36) = uVar43;
    *(longlong *)(lVar41 + 0x30) = lVar44 + 1;
    *(longlong *)(lVar41 + 0x40) = *(longlong *)(lVar41 + 0x40) + 1;
    *puVar46 = uVar38;
    pbVar10[0] = 1;
    pbVar10[1] = 0;
    pbVar10[2] = 0;
    pbVar10[3] = 0;
    *(float *)(pbVar10 + 0xc) = fStack_17c;
    *(ulonglong *)(pbVar10 + 4) = CONCAT44(fStack_180,fStack_184);
    return;
  case 1:
    uVar38 = *(undefined4 *)(lVar44 + 0x38);
    uVar19 = *(undefined8 *)(lVar44 + 0x30);
    uStack_201 = (undefined1)uVar19;
    uStack_200 = (undefined4)((ulonglong)uVar19 >> 8);
    uStack_1fc = (undefined3)((ulonglong)uVar19 >> 0x28);
    uStack_124 = (undefined4)*(undefined8 *)pbVar1;
    uStack_120 = (undefined4)((ulonglong)*(undefined8 *)pbVar1 >> 0x20);
    uStack_11c = (undefined4)*(undefined8 *)(pbVar10 + 0x18);
    local_118 = (undefined4)((ulonglong)*(undefined8 *)(pbVar10 + 0x18) >> 0x20);
    local_128 = 0;
    ::utils::geometry::circle::Circle2d::area();
    ::utils::inverse_lerp();
    *puVar14 = uVar4;
    lVar44 = *(longlong *)(lVar41 + 0x30);
    uVar43 = *(undefined8 *)(lVar41 + 0x40);
    if (lVar44 == *(longlong *)(lVar41 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    lVar30 = *(longlong *)(lVar41 + 0x28);
    lVar36 = lVar44 * 0x20;
    *(undefined1 *)(lVar30 + lVar36) = 2;
    puVar42 = (undefined8 *)(lVar30 + 1 + lVar36);
    *puVar42 = CONCAT17(uStack_201,local_208);
    puVar42[1] = CONCAT44(CONCAT13((char)uVar38,uStack_1fc),uStack_200);
    uVar70 = extraout_XMM0_Da_00;
    break;
  case 2:
    if (bVar31) {
      return;
    }
    uVar38 = *(undefined4 *)(lVar44 + 0x38);
    uVar19 = *(undefined8 *)(lVar44 + 0x30);
    uStack_124 = (undefined4)*(undefined8 *)pbVar1;
    uStack_120 = (undefined4)((ulonglong)*(undefined8 *)pbVar1 >> 0x20);
    uStack_11c = (undefined4)*(undefined8 *)(pbVar10 + 0x18);
    local_118 = (undefined4)((ulonglong)*(undefined8 *)(pbVar10 + 0x18) >> 0x20);
    local_128 = 0;
    ::utils::geometry::circle::Circle2d::area();
    ::utils::inverse_lerp();
    *puVar14 = uVar4;
    lVar44 = *(longlong *)(lVar41 + 0x30);
    uVar43 = *(undefined8 *)(lVar41 + 0x40);
    if (lVar44 == *(longlong *)(lVar41 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    lVar30 = *(longlong *)(lVar41 + 0x28);
    lVar36 = lVar44 * 0x20;
    *(undefined1 *)(lVar30 + lVar36) = 0;
    *(uint *)(lVar30 + 4 + lVar36) = uVar71;
    *(undefined8 *)(lVar30 + 8 + lVar36) = uVar19;
    uVar70 = extraout_XMM0_Da_01;
    break;
  case 3:
    goto LAB_14165cd12;
  case 4:
    iVar9 = *(int *)(lVar36 + (longlong)puVar29 * 4);
    *(undefined **)(pbVar10 + 0x390) = &DAT_142c826bc;
                    /* WARNING: Could not recover jumptable at 0x00014165e3cb. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)(iVar9 + lVar36))();
    return;
  case 5:
    do {
      lVar44 = uVar45 + uVar47;
      uVar47 = uVar47 + 0x10;
      uVar45 = lVar44 + 0x10U & 0x142c826bc;
      pcVar39 = (char *)(lVar36 + uVar45);
      auVar69[0] = -(*pcVar39 == auVar59[0]);
      auVar69[1] = -(pcVar39[1] == auVar59[1]);
      auVar69[2] = -(pcVar39[2] == auVar59[2]);
      auVar69[3] = -(pcVar39[3] == auVar59[3]);
      auVar69[4] = -(pcVar39[4] == auVar59[4]);
      auVar69[5] = -(pcVar39[5] == auVar59[5]);
      auVar69[6] = -(pcVar39[6] == auVar59[6]);
      auVar69[7] = -(pcVar39[7] == auVar59[7]);
      auVar69[8] = -(pcVar39[8] == auVar59[8]);
      auVar69[9] = -(pcVar39[9] == auVar59[9]);
      auVar69[10] = -(pcVar39[10] == auVar59[10]);
      auVar69[0xb] = -(pcVar39[0xb] == auVar59[0xb]);
      auVar69[0xc] = -(pcVar39[0xc] == auVar59[0xc]);
      auVar69[0xd] = -(pcVar39[0xd] == auVar59[0xd]);
      auVar69[0xe] = -(pcVar39[0xe] == auVar59[0xe]);
      auVar69[0xf] = -(pcVar39[0xf] == auVar59[0xf]);
      uVar48 = (ushort)(SUB161(auVar69 >> 7,0) & 1) | (ushort)(SUB161(auVar69 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar69 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar69 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar69 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar69 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar69 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar69 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar69 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar69 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar69 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar69 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar69 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar69 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar69 >> 0x77,0) & 1) << 0xe | (ushort)(auVar69[0xf] >> 7) << 0xf;
      uVar71 = (uint)uVar48;
      while (uVar48 != 0) {
        uVar27 = 0;
        for (uVar24 = uVar71; (uVar24 & 1) == 0; uVar24 = uVar24 >> 1 | 0x80000000) {
          uVar27 = uVar27 + 1;
        }
        lVar44 = (uVar27 + uVar45 & 0x142c826bc) * -0x78;
        if (*(byte **)(puVar29 + lVar44) == pbVar1) {
          uVar71 = *(uint *)(lVar36 + -0x38 + lVar44);
          if (1 < uVar71) {
            *(undefined ***)(pbVar10 + 0x1b0) = &PTR_s_FinishCircleCmd_received_a_wall_t_142c82630;
            pbVar10[0x1b8] = 1;
            pbVar10[0x1b9] = 0;
            pbVar10[0x1ba] = 0;
            pbVar10[0x1bb] = 0;
            pbVar10[0x1bc] = 0;
            pbVar10[0x1bd] = 0;
            pbVar10[0x1be] = 0;
            pbVar10[0x1bf] = 0;
            pbVar10[0x1c0] = 8;
            pbVar10[0x1c1] = 0;
            pbVar10[0x1c2] = 0;
            pbVar10[0x1c3] = 0;
            pbVar10[0x1c4] = 0;
            pbVar10[0x1c5] = 0;
            pbVar10[0x1c6] = 0;
            pbVar10[0x1c7] = 0;
            *(undefined1 (*) [16])(pbVar10 + 0x1c8) = (undefined1  [16])0x0;
                    /* WARNING: Subroutine does not return */
            core::panicking::panic_fmt();
          }
          *(undefined8 **)(pbVar10 + 0x338) = in_stack_00000048;
          lVar36 = lVar36 + lVar44;
          *(undefined8 *)(pbVar10 + 0x2c0) = *(undefined8 *)(lVar36 + -0x24);
          uVar19 = *(undefined8 *)(lVar36 + -0x2c);
          *(undefined8 *)(pbVar10 + 0x2b0) = *(undefined8 *)(lVar36 + -0x34);
          *(undefined8 *)(pbVar10 + 0x2b8) = uVar19;
          *(uint *)(pbVar10 + 0x2d0) = uVar71;
          uVar19 = *(undefined8 *)(lVar36 + -0x2c);
          *(undefined8 *)(pbVar10 + 0x2d4) = *(undefined8 *)(lVar36 + -0x34);
          *(undefined8 *)(pbVar10 + 0x2dc) = uVar19;
          *(undefined8 *)(pbVar10 + 0x2e4) = *(undefined8 *)(lVar36 + -0x24);
          *(uint *)(pbVar10 + 800) = uVar71;
          if ((uVar71 & 1) != 0) {
                    /* WARNING: Subroutine does not return */
            core::panicking::panic();
          }
          **(undefined4 **)(pbVar10 + 0x100) = *(undefined4 *)(pbVar10 + 0x194);
          resources::walls::decorator_storage::DecoratorStorage::wall_decorators_mut();
          if (extraout_RAX_00 == 0) goto LAB_14165ee8d;
          pauVar53 = *(undefined1 (**) [16])(extraout_RAX_00 + 0x30);
          lVar44 = *(longlong *)(extraout_RAX_00 + 0x48);
          auVar59 = *pauVar53;
          uVar71 = ~(uint)(ushort)((ushort)(SUB161(auVar59 >> 7,0) & 1) |
                                   (ushort)(SUB161(auVar59 >> 0xf,0) & 1) << 1 |
                                   (ushort)(SUB161(auVar59 >> 0x17,0) & 1) << 2 |
                                   (ushort)(SUB161(auVar59 >> 0x1f,0) & 1) << 3 |
                                   (ushort)(SUB161(auVar59 >> 0x27,0) & 1) << 4 |
                                   (ushort)(SUB161(auVar59 >> 0x2f,0) & 1) << 5 |
                                   (ushort)(SUB161(auVar59 >> 0x37,0) & 1) << 6 |
                                   (ushort)(SUB161(auVar59 >> 0x3f,0) & 1) << 7 |
                                   (ushort)(SUB161(auVar59 >> 0x47,0) & 1) << 8 |
                                   (ushort)(SUB161(auVar59 >> 0x4f,0) & 1) << 9 |
                                   (ushort)(SUB161(auVar59 >> 0x57,0) & 1) << 10 |
                                   (ushort)(SUB161(auVar59 >> 0x5f,0) & 1) << 0xb |
                                   (ushort)(SUB161(auVar59 >> 0x67,0) & 1) << 0xc |
                                   (ushort)(SUB161(auVar59 >> 0x6f,0) & 1) << 0xd |
                                   (ushort)(SUB161(auVar59 >> 0x77,0) & 1) << 0xe |
                                  (ushort)(byte)(auVar59[0xf] >> 7) << 0xf);
          pauVar50 = pauVar53 + 1;
          goto LAB_14165ec8d;
        }
        uVar48 = (ushort)(uVar71 - 1) & (ushort)uVar71;
        uVar71 = CONCAT22((short)(uVar71 - 1 >> 0x10),uVar48);
      }
      auVar66[0] = -(*pcVar39 == in_XMM1[0]);
      auVar66[1] = -(pcVar39[1] == in_XMM1[1]);
      auVar66[2] = -(pcVar39[2] == in_XMM1[2]);
      auVar66[3] = -(pcVar39[3] == in_XMM1[3]);
      auVar66[4] = -(pcVar39[4] == in_XMM1[4]);
      auVar66[5] = -(pcVar39[5] == in_XMM1[5]);
      auVar66[6] = -(pcVar39[6] == in_XMM1[6]);
      auVar66[7] = -(pcVar39[7] == in_XMM1[7]);
      auVar66[8] = -(pcVar39[8] == in_XMM1[8]);
      auVar66[9] = -(pcVar39[9] == in_XMM1[9]);
      auVar66[10] = -(pcVar39[10] == in_XMM1[10]);
      auVar66[0xb] = -(pcVar39[0xb] == in_XMM1[0xb]);
      auVar66[0xc] = -(pcVar39[0xc] == in_XMM1[0xc]);
      auVar66[0xd] = -(pcVar39[0xd] == in_XMM1[0xd]);
      auVar66[0xe] = -(pcVar39[0xe] == in_XMM1[0xe]);
      auVar66[0xf] = -(pcVar39[0xf] == in_XMM1[0xf]);
      if ((((((((((((((((SUB161(auVar66 >> 7,0) & 1) != 0 || (SUB161(auVar66 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar66 >> 0x17,0) & 1) != 0) || (SUB161(auVar66 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar66 >> 0x27,0) & 1) != 0) || (SUB161(auVar66 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar66 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar66 >> 0x3f,0) & 1) != 0) || (SUB161(auVar66 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar66 >> 0x4f,0) & 1) != 0) || (SUB161(auVar66 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar66 >> 0x5f,0) & 1) != 0) || (SUB161(auVar66 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar66 >> 0x6f,0) & 1) != 0) || (SUB161(auVar66 >> 0x77,0) & 1) != 0) ||
          auVar66[0xf] < '\0') {
                    /* WARNING: Subroutine does not return */
        core::option::unwrap_failed();
      }
    } while( true );
  case 6:
    _UNK_142c826d2 = _UNK_142c826d2 + (int)in_stack_00000048;
                    /* WARNING: Subroutine does not return */
    core::option::expect_failed();
  case 7:
    pbVar10[0x350] = pbVar10[0x350] + (char)puVar29 + (0xf00cffff < (uint)puVar29);
    if (*(float *)(*(longlong *)(pbVar10 + 0xf0) + 0x28) * __real_3f000000 <
        (float)((uint)(auVar59._0_4_ - unaff_XMM6_Da) & ___xmm_7fffffff7fffffff7fffffff7fffffff)) {
      uVar38 = *(undefined4 *)(*(longlong *)(pbVar10 + 0xf8) + 0x1c);
      puVar46 = *(undefined4 **)(*(longlong *)(pbVar10 + 0xf8) + 0x10);
      *puVar46 = uVar38;
      ::utils::geometry::circle::Circle2d::as_curve2();
      *(undefined8 *)(pbVar10 + 0x1e0) = *(undefined8 *)(pbVar10 + 0x300);
      *(undefined8 *)(pbVar10 + 0x1d0) = *(undefined8 *)(pbVar10 + 0x2f0);
      *(undefined8 *)(pbVar10 + 0x1d8) = *(undefined8 *)(pbVar10 + 0x2f8);
      *(undefined8 *)(pbVar10 + 0x1c0) = *(undefined8 *)(pbVar10 + 0x2e0);
      *(undefined8 *)(pbVar10 + 0x1c8) = *(undefined8 *)(pbVar10 + 0x2e8);
      *(undefined8 *)(pbVar10 + 0x1b0) = *(undefined8 *)(pbVar10 + 0x2d0);
      *(undefined8 *)(pbVar10 + 0x1b8) = *(undefined8 *)(pbVar10 + 0x2d8);
      resources::walls::inner_walls::InnerWalls::
      _try_update_state<country_core::resources::walls::inner_walls::impl_1::update_wall_state_curve::closure_env_0>
                ();
      *puVar46 = uVar38;
      *(undefined8 *)(pbVar10 + 0x1b4) = *(undefined8 *)(pbVar10 + 0x348);
      *(undefined8 *)(pbVar10 + 0x1bc) = *(undefined8 *)(pbVar10 + 0x350);
      pbVar10[0x1b0] = 0;
      pbVar10[0x1b1] = 0;
      pbVar10[0x1b2] = 0;
      pbVar10[0x1b3] = 0;
      resources::walls::inner_walls::InnerWalls::
      _try_update_state<country_core::resources::walls::inner_walls::impl_1::update_wall_state_enclosure::closure_env_0>
                ();
      ::utils::swizzlers::impl$1::xvy();
      **(undefined4 **)(pbVar10 + 0x78) = *(undefined4 *)(pbVar10 + 0x140);
      lVar44 = *(longlong *)(pbVar10 + 0x168);
      lVar30 = *(longlong *)(lVar44 + 0x30);
      uVar19 = *(undefined8 *)(lVar44 + 0x40);
      if (lVar30 == *(longlong *)(lVar44 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                  ();
      }
      lVar44 = *(longlong *)(pbVar10 + 0x168);
      lVar41 = *(longlong *)(lVar44 + 0x28);
      lVar36 = lVar30 * 0x20;
      *(undefined8 *)(lVar41 + lVar36) = uVar19;
      *(undefined8 *)(lVar41 + 8 + lVar36) = *(undefined8 *)(pbVar10 + 0x3a8);
      *(undefined8 *)(lVar41 + 0x10 + lVar36) = *(undefined8 *)(pbVar10 + 0x110);
      *(undefined4 *)(lVar41 + 0x18 + lVar36) = *(undefined4 *)(pbVar10 + 0x118);
      *(undefined2 *)(lVar41 + 0x1c + lVar36) = 4;
      *(longlong *)(lVar44 + 0x30) = lVar30 + 1;
      *(longlong *)(lVar44 + 0x40) = *(longlong *)(lVar44 + 0x40) + 1;
      fVar60 = 0.0;
      ::utils::inverse_lerp();
      **(undefined4 **)(pbVar10 + 0x70) = *(undefined4 *)(pbVar10 + 0x13c);
      lVar44 = *(longlong *)(pbVar10 + 0x160);
      lVar30 = *(longlong *)(lVar44 + 0x30);
      uVar19 = *(undefined8 *)(lVar44 + 0x40);
      if (lVar30 == *(longlong *)(lVar44 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                  ();
      }
      if (0.0 <= extraout_XMM0_Da_04) {
        fVar60 = extraout_XMM0_Da_04;
      }
      fVar61 = __real_3f800000;
      if (fVar60 <= __real_3f800000) {
        fVar61 = fVar60;
      }
      lVar44 = *(longlong *)(pbVar10 + 0x160);
      lVar41 = *(longlong *)(lVar44 + 0x28);
      *(undefined8 *)(lVar41 + lVar30 * 0x28) = uVar19;
      *(undefined8 *)(lVar41 + 8 + lVar30 * 0x28) = *(undefined8 *)(pbVar10 + 0x3a8);
      *(undefined8 *)(lVar41 + 0x10 + lVar30 * 0x28) = *(undefined8 *)(pbVar10 + 0x110);
      *(undefined4 *)(lVar41 + 0x18 + lVar30 * 0x28) = *(undefined4 *)(pbVar10 + 0x118);
      *(float *)(lVar41 + 0x1c + lVar30 * 0x28) = fVar61;
      *(undefined1 *)(lVar41 + 0x20 + lVar30 * 0x28) = 0;
      *(longlong *)(lVar44 + 0x30) = lVar30 + 1;
      *(longlong *)(lVar44 + 0x40) = *(longlong *)(lVar44 + 0x40) + 1;
    }
    puVar42 = *(undefined8 **)(pbVar10 + 0x390);
    bVar25 = pbVar10[0x388];
    goto joined_r0x00014165f7ab;
  }
  *(undefined4 *)(lVar30 + 0x10 + lVar36) = uVar38;
  *(undefined4 *)(lVar30 + 0x14 + lVar36) = uVar70;
  *(undefined8 *)(lVar30 + 0x18 + lVar36) = uVar43;
  *(longlong *)(lVar41 + 0x30) = lVar44 + 1;
  *(longlong *)(lVar41 + 0x40) = *(longlong *)(lVar41 + 0x40) + 1;
LAB_14165cd12:
  return;
LAB_14165ec8d:
  while( true ) {
    if (lVar44 == 0) goto LAB_14165ee8d;
    if ((short)uVar71 == 0) {
      do {
        auVar59 = *pauVar50;
        uVar48 = (ushort)(SUB161(auVar59 >> 7,0) & 1) | (ushort)(SUB161(auVar59 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar59 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar59 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar59 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar59 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar59 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar59 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar59 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar59 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar59 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar59 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar59 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar59 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar59 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar59[0xf] >> 7) << 0xf;
        pauVar53 = pauVar53 + -0x50;
        pauVar50 = pauVar50 + 1;
      } while (uVar48 == 0xffff);
      uVar27 = ~(uint)uVar48;
      uVar71 = uVar27 & -(uint)uVar48 - 2;
    }
    else {
      uVar27 = uVar71;
      uVar71 = uVar71 - 1 & uVar71;
    }
    lVar44 = lVar44 + -1;
    uVar24 = 0;
    for (; (uVar27 & 1) == 0; uVar27 = uVar27 >> 1 | 0x80000000) {
      uVar24 = uVar24 + 1;
    }
    uVar47 = (ulonglong)uVar24;
    lVar30 = (ulonglong)(byte)pauVar53[uVar47 * -5 + -3][10] - 1;
    if (2 < (byte)(pauVar53[uVar47 * -5 + -3][10] - 2)) {
      lVar30 = 0;
    }
    if (lVar30 == 0) break;
    if (lVar30 != 1) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic();
    }
  }
  hashbrown::map::HashMap::
  insert<country_core::resources::walls::decorator::DecoratorId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
            ();
  if (1 < (byte)pauVar53[uVar47 * -5 + -3][10]) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  uVar38 = *(undefined4 *)(pauVar53[uVar47 * -5 + -4] + 0xc);
  ::utils::geometry::circle::Circle2d::closest_point_on_perimeter();
  *(undefined4 *)(pbVar10 + 0x360) = extraout_XMM0_Da_03;
  *(undefined4 *)(pbVar10 + 0x364) = uVar38;
  ::utils::swizzlers::impl$1::xvy();
  *(undefined4 *)(pauVar53[uVar47 * -5 + -3] + 4) = *(undefined4 *)(pbVar10 + 0x1b8);
  *(undefined8 *)(pauVar53[uVar47 * -5 + -4] + 0xc) = *(undefined8 *)(pbVar10 + 0x1b0);
  goto LAB_14165ec8d;
LAB_14165ee8d:
  pbVar10[0x348] = 0;
  pbVar10[0x349] = 0;
  pbVar10[0x34a] = 0;
  pbVar10[0x34b] = 0;
  pbVar10[0x34c] = 0;
  pbVar10[0x34d] = 0;
  pbVar10[0x34e] = 0;
  pbVar10[0x34f] = 0;
  pbVar10[0x350] = 8;
  pbVar10[0x351] = 0;
  pbVar10[0x352] = 0;
  pbVar10[0x353] = 0;
  pbVar10[0x354] = 0;
  pbVar10[0x355] = 0;
  pbVar10[0x356] = 0;
  pbVar10[0x357] = 0;
  pbVar10[0x358] = 0;
  pbVar10[0x359] = 0;
  pbVar10[0x35a] = 0;
  pbVar10[0x35b] = 0;
  pbVar10[0x35c] = 0;
  pbVar10[0x35d] = 0;
  pbVar10[0x35e] = 0;
  pbVar10[0x35f] = 0;
  uVar19 = (*(undefined8 **)(pbVar10 + 0x338))[1];
  *(undefined8 *)(pbVar10 + 0x1c4) = **(undefined8 **)(pbVar10 + 0x338);
  *(undefined8 *)(pbVar10 + 0x1cc) = uVar19;
  *(undefined8 *)(pbVar10 + 0x1b8) = *(undefined8 *)(pbVar10 + 0x3a8);
  pbVar10[0x1c0] = 0;
  pbVar10[0x1c1] = 0;
  pbVar10[0x1c2] = 0;
  pbVar10[0x1c3] = 0;
  *(undefined4 *)(pbVar10 + 0x1dc) = *(undefined4 *)(pbVar10 + 800);
  *(undefined8 *)(pbVar10 + 0x1e0) = *(undefined8 *)(pbVar10 + 0x2b0);
  *(undefined8 *)(pbVar10 + 0x1e8) = *(undefined8 *)(pbVar10 + 0x2b8);
  *(undefined8 *)(pbVar10 + 0x1f0) = *(undefined8 *)(pbVar10 + 0x2c0);
  pbVar10[0x1b0] = 4;
  pbVar10[0x1b1] = 0;
  pbVar10[0x1b2] = 0;
  pbVar10[0x1b3] = 0;
  pbVar10[0x1b4] = 0;
  pbVar10[0x1b5] = 0;
  pbVar10[0x1b6] = 0;
  pbVar10[0x1b7] = 0x80;
  pbVar10[0x3b6] = 1;
  resources::history::wall_compound_edit::CompoundEditBuilder::next();
  lVar44 = *(longlong *)(pbVar10 + 0x88);
  lVar30 = *(longlong *)(pbVar10 + 0x90);
  **(undefined4 **)(*(longlong *)(pbVar10 + 0x4d0) + 0x10) =
       *(undefined4 *)(*(longlong *)(pbVar10 + 0x4d0) + 0x1c);
  *(undefined8 *)(pbVar10 + 0x360) = *(undefined8 *)(pbVar10 + 0x348);
  *(undefined8 *)(pbVar10 + 0x368) = *(undefined8 *)(pbVar10 + 0x350);
  uVar19 = *(undefined8 *)(pbVar10 + 0x360);
  lVar41 = *(longlong *)(pbVar10 + 0x368);
  pbVar10[0x360] = 0;
  pbVar10[0x361] = 0;
  pbVar10[0x362] = 0;
  pbVar10[0x363] = 0;
  pbVar10[0x364] = 0;
  pbVar10[0x365] = 0;
  pbVar10[0x366] = 0;
  pbVar10[0x367] = 0;
  pbVar10[0x368] = 8;
  pbVar10[0x369] = 0;
  pbVar10[0x36a] = 0;
  pbVar10[0x36b] = 0;
  pbVar10[0x36c] = 0;
  pbVar10[0x36d] = 0;
  pbVar10[0x36e] = 0;
  pbVar10[0x36f] = 0;
  pbVar10[0x370] = 0;
  pbVar10[0x371] = 0;
  pbVar10[0x372] = 0;
  pbVar10[0x373] = 0;
  pbVar10[0x374] = 0;
  pbVar10[0x375] = 0;
  pbVar10[0x376] = 0;
  pbVar10[0x377] = 0;
  *(longlong *)(pbVar10 + 0x1b0) = lVar41;
  *(longlong *)(pbVar10 + 0x1b8) = lVar41;
  *(undefined8 *)(pbVar10 + 0x1c0) = uVar19;
  *(longlong *)(pbVar10 + 0x1c8) = lVar41 + *(longlong *)(pbVar10 + 0x358) * 0x18;
  alloc::boxed::iter::impl$13::
  from_iter<country_core::resources::history::wall_compound_edit::ConcurrentEdits,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2_<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl_0::new::closure_env_0>_>
            ();
  *(undefined8 *)(pbVar10 + 0x1b8) = extraout_RAX_01;
  *(longlong *)(pbVar10 + 0x1c0) = lVar41;
  pbVar10[0x1b0] = 7;
  pbVar10[0x1b1] = 0;
  pbVar10[0x1b2] = 0;
  pbVar10[0x1b3] = 0;
  pbVar10[0x1b4] = 0;
  pbVar10[0x1b5] = 0;
  pbVar10[0x1b6] = 0;
  pbVar10[0x1b7] = 0;
  pbVar10[0x3b6] = 0;
  resources::history::History::add();
  **(undefined4 **)(pbVar10 + 0xa0) = *(undefined4 *)(pbVar10 + 0x14c);
  lVar41 = *(longlong *)(lVar30 + 0x30);
  uVar19 = *(undefined8 *)(lVar30 + 0x40);
  if (lVar41 == *(longlong *)(lVar30 + 0x20)) {
    pbVar10[0x3b6] = 0;
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
              ();
  }
  *(undefined8 *)(*(longlong *)(lVar30 + 0x28) + lVar41 * 8) = uVar19;
  *(longlong *)(lVar30 + 0x30) = lVar41 + 1;
  *(longlong *)(lVar30 + 0x40) = *(longlong *)(lVar30 + 0x40) + 1;
  **(undefined4 **)(pbVar10 + 0x98) = *(undefined4 *)(pbVar10 + 0x148);
  lVar30 = *(longlong *)(lVar44 + 0x30);
  uVar19 = *(undefined8 *)(lVar44 + 0x40);
  if (lVar30 == *(longlong *)(lVar44 + 0x20)) {
    pbVar10[0x3b6] = 0;
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
              ();
  }
  lVar41 = *(longlong *)(lVar44 + 0x28);
  *(undefined8 *)(lVar41 + lVar30 * 0x10) = uVar19;
  *(undefined8 *)(lVar41 + 8 + lVar30 * 0x10) = *(undefined8 *)(pbVar10 + 0x3a8);
  *(longlong *)(lVar44 + 0x30) = lVar30 + 1;
  *(longlong *)(lVar44 + 0x40) = *(longlong *)(lVar44 + 0x40) + 1;
  uVar19 = (*(undefined8 **)(pbVar10 + 0x390))[1];
  *(undefined8 *)(pbVar10 + 0x1b0) = **(undefined8 **)(pbVar10 + 0x390);
  *(undefined8 *)(pbVar10 + 0x1b8) = uVar19;
  lVar41 = in_stack_00000050[-4];
  **(undefined4 **)(pbVar10 + 0xe8) = *(undefined4 *)(pbVar10 + 400);
  lVar44 = *(longlong *)(pbVar10 + 0x318);
  lVar30 = *(longlong *)(lVar44 + 0x30);
  uVar19 = *(undefined8 *)(lVar44 + 0x40);
  if (lVar30 == *(longlong *)(lVar44 + 0x20)) {
    alloc::raw_vec::RawVec::
    grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>();
  }
  lVar44 = *(longlong *)(pbVar10 + 0x318);
  lVar36 = *(longlong *)(lVar44 + 0x28);
  lVar35 = lVar30 * 0x30;
  *(undefined4 *)(lVar36 + lVar35) = 0;
  uVar43 = *(undefined8 *)(pbVar10 + 0x1b8);
  puVar42 = (undefined8 *)(lVar36 + 4 + lVar35);
  *puVar42 = *(undefined8 *)(pbVar10 + 0x1b0);
  puVar42[1] = uVar43;
  *(int *)(lVar36 + 0x1c + lVar35) = (int)lVar41;
  *(undefined4 *)(lVar36 + 0x20 + lVar35) = 0x3fc00000;
  *(undefined1 *)(lVar36 + 0x24 + lVar35) = 1;
  *(undefined8 *)(lVar36 + 0x28 + lVar35) = uVar19;
  *(longlong *)(lVar44 + 0x30) = lVar30 + 1;
  *(longlong *)(lVar44 + 0x40) = *(longlong *)(lVar44 + 0x40) + 1;
  puVar42 = *(undefined8 **)(pbVar10 + 0x390);
  bVar25 = pbVar10[0x388];
joined_r0x00014165f7ab:
  if ((bVar25 & 1) == 0) {
    uVar19 = puVar42[1];
    *(undefined8 *)(pbVar10 + 0x1b0) = *puVar42;
    *(undefined8 *)(pbVar10 + 0x1b8) = uVar19;
    *(ulonglong *)(pbVar10 + 0x40) =
         CONCAT44(*(float *)((longlong)in_stack_00000050 + -0x3c) *
                  (float)((ulonglong)**(undefined8 **)(pbVar10 + 0x3a0) >> 0x20) +
                  (float)((ulonglong)*(undefined8 *)((longlong)in_stack_00000050 + -0x44) >> 0x20),
                  *(float *)((longlong)in_stack_00000050 + -0x3c) *
                  (float)**(undefined8 **)(pbVar10 + 0x3a0) +
                  (float)*(undefined8 *)((longlong)in_stack_00000050 + -0x44));
    ::utils::swizzlers::impl$1::xvy();
    ::utils::swizzlers::impl$0::x0y();
    cVar52 = '\x02';
    lVar44 = *(longlong *)(pbVar10 + 0x1a0);
    if (*(byte *)(lVar44 + 0x18) < 2) {
      puVar42 = *(undefined8 **)(pbVar10 + 0x330);
      puVar46 = *(undefined4 **)(pbVar10 + 0x308);
      uVar71 = *(uint *)(pbVar10 + 0x388);
    }
    else {
      puVar42 = *(undefined8 **)(pbVar10 + 0x330);
      puVar46 = *(undefined4 **)(pbVar10 + 0x308);
      uVar71 = *(uint *)(pbVar10 + 0x388);
      if (*(longlong *)(lVar44 + 0x38) == *(longlong *)(pbVar10 + 0x328)) {
        cVar52 = *(char *)(lVar44 + 0x19);
      }
    }
    *(undefined8 *)(pbVar10 + 0x2d0) = *(undefined8 *)(pbVar10 + 0x1b0);
    *(undefined8 *)(pbVar10 + 0x2d8) = *(undefined8 *)(pbVar10 + 0x1b8);
    *(undefined8 *)(pbVar10 + 0x2b0) = *(undefined8 *)(pbVar10 + 0x348);
    *(undefined4 *)(pbVar10 + 0x2b8) = *(undefined4 *)(pbVar10 + 0x350);
    *(undefined8 *)(pbVar10 + 0xb8) = *(undefined8 *)(pbVar10 + 0x360);
    *(undefined4 *)(pbVar10 + 0xc0) = *(undefined4 *)(pbVar10 + 0x368);
    *puVar46 = *(undefined4 *)(pbVar10 + 0x310);
    *puVar42 = *(undefined8 *)(pbVar10 + 0x328);
    puVar42[1] = *(undefined8 *)(pbVar10 + 0x3a8);
    uVar19 = *(undefined8 *)(pbVar10 + 0x2d8);
    puVar42[2] = *(undefined8 *)(pbVar10 + 0x2d0);
    puVar42[3] = uVar19;
    puVar42[4] = *(undefined8 *)(pbVar10 + 0x2b0);
    *(undefined4 *)(puVar42 + 5) = *(undefined4 *)(pbVar10 + 0x2b8);
    *(undefined8 *)((longlong)puVar42 + 0x2c) = *(undefined8 *)(pbVar10 + 0xb8);
    *(undefined4 *)((longlong)puVar42 + 0x34) = *(undefined4 *)(pbVar10 + 0xc0);
    *(bool *)(puVar42 + 7) = cVar52 != '\x02';
    puVar42 = *(undefined8 **)(pbVar10 + 0x390);
  }
  else {
    ::utils::geometry::circle::Circle2d::new_rotated();
    *(undefined8 *)(pbVar10 + 0x1b0) = *(undefined8 *)(pbVar10 + 0x30);
    *(undefined8 *)(pbVar10 + 0x1b8) = *(undefined8 *)(pbVar10 + 0x38);
    ui_edit_circle::show_full_circle_signifier();
    ::utils::xz_rotation_bad();
    fVar60 = *(float *)(pbVar10 + 0x1b0);
    fVar61 = *(float *)(pbVar10 + 0x1b4);
    fVar67 = *(float *)(pbVar10 + 0x1b8);
    fVar68 = *(float *)(pbVar10 + 0x1bc);
    *(ulonglong *)(pbVar10 + 0x2d0) =
         CONCAT44(*(float *)(pbVar10 + 0x38) *
                  (float)((ulonglong)**(undefined8 **)(pbVar10 + 0x3a0) >> 0x20) +
                  (float)((ulonglong)**(undefined8 **)(pbVar10 + 0x390) >> 0x20),
                  *(float *)(pbVar10 + 0x38) * (float)**(undefined8 **)(pbVar10 + 0x3a0) +
                  (float)**(undefined8 **)(pbVar10 + 0x390));
    ::utils::swizzlers::impl$1::xvy();
    fVar63 = fVar68 * fVar68;
    fVar65 = fVar67 * fVar67 + fVar61 * fVar61 + fVar60 * fVar60;
    fVar62 = (fVar63 - fVar65) * _UNK_142a891e8;
    fVar58 = _UNK_142a891e8 * fVar67 +
             _UNK_142a891e4 * fVar61 + ___xmm_000000003e4ccccd0000000000000000 * fVar60;
    fVar58 = fVar58 + fVar58;
    fVar64 = fVar68 + fVar68;
    *(ulonglong *)(pbVar10 + 0x360) =
         CONCAT44((float)((ulonglong)*(undefined8 *)(pbVar10 + 0x1b0) >> 0x20) +
                  fVar64 * (fVar67 * 0.0 - fVar60 * __real_3e4ccccd) +
                  fVar58 * fVar61 + (fVar63 - fVar65) * _UNK_142a891e4,
                  (float)*(undefined8 *)(pbVar10 + 0x1b0) +
                  fVar64 * (fVar61 * __real_3e4ccccd - fVar67 * 0.0) +
                  fVar58 * fVar60 + (fVar63 - fVar65) * ___xmm_000000003e4ccccd0000000000000000);
    *(float *)(pbVar10 + 0x368) =
         fVar64 * (fVar60 * 0.0 - fVar61 * 0.0) + fVar58 * fVar67 + fVar62 +
         *(float *)(pbVar10 + 0x1b8);
    *(byte **)(pbVar10 + 0x2d0) = pbVar10 + 0x48;
    *(code **)(pbVar10 + 0x2d8) = resources::walls::impl$7::fmt;
    *(byte **)(pbVar10 + 0x2e0) = pbVar10 + 0x30;
    *(code **)(pbVar10 + 0x2e8) = ::utils::geometry::circle::impl$3::fmt;
    *(byte **)(pbVar10 + 0x2f0) = pbVar10 + 0x360;
    *(code **)(pbVar10 + 0x2f8) = glam::f32::vec3::impl$93::fmt;
    *(undefined ***)(pbVar10 + 0x1b0) = &PTR_s_circle_edit__arrow_142c82428;
    pbVar10[0x1b8] = 4;
    pbVar10[0x1b9] = 0;
    pbVar10[0x1ba] = 0;
    pbVar10[0x1bb] = 0;
    pbVar10[0x1bc] = 0;
    pbVar10[0x1bd] = 0;
    pbVar10[0x1be] = 0;
    pbVar10[0x1bf] = 0;
    *(byte **)(pbVar10 + 0x1c0) = pbVar10 + 0x2d0;
    pbVar10[0x1c8] = 3;
    pbVar10[0x1c9] = 0;
    pbVar10[0x1ca] = 0;
    pbVar10[0x1cb] = 0;
    pbVar10[0x1cc] = 0;
    pbVar10[0x1cd] = 0;
    pbVar10[0x1ce] = 0;
    pbVar10[0x1cf] = 0;
    pbVar10[0x1d0] = 0;
    pbVar10[0x1d1] = 0;
    pbVar10[0x1d2] = 0;
    pbVar10[0x1d3] = 0;
    pbVar10[0x1d4] = 0;
    pbVar10[0x1d5] = 0;
    pbVar10[0x1d6] = 0;
    pbVar10[0x1d7] = 0;
    alloc::fmt::format::format_inner();
    *(undefined8 *)(pbVar10 + 0x330) = *(undefined8 *)(pbVar10 + 0x18);
    _Src = *(void **)(pbVar10 + 0x20);
    _Size = *(size_t *)(pbVar10 + 0x28);
    *(undefined4 *)(pbVar10 + 0x1b8) = *(undefined4 *)(pbVar10 + 0x368);
    *(undefined8 *)(pbVar10 + 0x1b0) = *(undefined8 *)(pbVar10 + 0x360);
    std::alloc::__default_lib_allocator::__rust_alloc();
    *(void **)(pbVar10 + 0x308) = _Src;
    if (extraout_RAX_02 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX_02 = 1;
    extraout_RAX_02[1] = 1;
    *(float *)(extraout_RAX_02 + 2) = fVar60;
    *(float *)((longlong)extraout_RAX_02 + 0x14) = fVar61;
    *(float *)(extraout_RAX_02 + 3) = fVar67;
    *(float *)((longlong)extraout_RAX_02 + 0x1c) = fVar68;
    extraout_RAX_02[4] = *(undefined8 *)(pbVar10 + 0x1b0);
    *(undefined4 *)(extraout_RAX_02 + 5) = *(undefined4 *)(pbVar10 + 0x1b8);
    *(undefined8 **)(pbVar10 + 0x1b0) = extraout_RAX_02;
    *(undefined **)(pbVar10 + 0x1b8) = &DAT_142c82468;
    if ((longlong)_Size < 0) {
      *(undefined8 **)(pbVar10 + 0x310) = extraout_RAX_02;
LAB_14165fd6e:
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error();
    }
    lVar44 = *(longlong *)(pbVar10 + 0x1a0);
    puVar42 = *(undefined8 **)(pbVar10 + 0x390);
    if (_Size == 0) {
      _Dst = (void *)0x1;
      puVar51 = extraout_RAX_02;
    }
    else {
      *(undefined8 **)(pbVar10 + 0x310) = extraout_RAX_02;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_03 == (void *)0x0) goto LAB_14165fd6e;
      puVar51 = *(undefined8 **)(pbVar10 + 0x310);
      _Dst = extraout_RAX_03;
    }
    memcpy(_Dst,_Src,_Size);
    *(size_t *)(pbVar10 + 0x1b0) = _Size;
    *(void **)(pbVar10 + 0x1b8) = _Dst;
    *(size_t *)(pbVar10 + 0x1c0) = _Size;
    *(undefined8 **)(pbVar10 + 0x1c8) = puVar51;
    *(undefined **)(pbVar10 + 0x1d0) = &DAT_142c82468;
    append_only_vec::AppendOnlyVec::
    push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>();
    uVar71 = *(uint *)(pbVar10 + 0x388);
    if (*(longlong *)(pbVar10 + 0x330) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  cVar52 = '\x02';
  if ((1 < *(byte *)(lVar44 + 0x18)) &&
     (*(longlong *)(lVar44 + 0x38) == *(longlong *)(pbVar10 + 0x328))) {
    cVar52 = *(char *)(lVar44 + 0x19);
  }
  bVar25 = 3;
  if ((byte)(cVar52 - 2U) < 3) {
    bVar25 = cVar52 - 2U;
  }
  if ((byte)(bVar25 - 1) < 2) {
    fVar60 = __real_3f000000;
    if ((uVar71 & 1) == 0) {
      fVar60 = 0.0;
    }
    fVar61 = (float)**(undefined8 **)(pbVar10 + 0x3a0);
    fVar67 = (float)((ulonglong)**(undefined8 **)(pbVar10 + 0x3a0) >> 0x20);
    fVar68 = (float)*puVar42;
    fVar58 = (float)((ulonglong)*puVar42 >> 0x20);
    *(ulonglong *)(pbVar10 + 0x348) =
         CONCAT44((__real_3f666667 + fVar60) * fVar67 + fVar58,
                  (__real_3f666667 + fVar60) * fVar61 + fVar68);
    *(ulonglong *)(pbVar10 + 0x360) =
         CONCAT44((fVar60 + __real_41000000) * fVar67 + fVar58,
                  (fVar60 + __real_41000000) * fVar61 + fVar68);
    std::alloc::__default_lib_allocator::__rust_alloc();
    *(longlong *)(pbVar10 + 0x3a0) = extraout_RAX_04;
    if (extraout_RAX_04 == 0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error();
    }
    ::utils::swizzlers::impl$1::xvy();
    ::utils::swizzlers::impl$1::xvy();
    puVar42 = *(undefined8 **)(pbVar10 + 0x3a0);
    *(undefined4 *)(puVar42 + 1) = *(undefined4 *)(pbVar10 + 0x2d8);
    *puVar42 = *(undefined8 *)(pbVar10 + 0x2d0);
    *(undefined4 *)((longlong)puVar42 + 0x14) = *(undefined4 *)(pbVar10 + 0x1b8);
    *(undefined8 *)((longlong)puVar42 + 0xc) = *(undefined8 *)(pbVar10 + 0x1b0);
    pbVar10[0x1b0] = 2;
    pbVar10[0x1b1] = 0;
    pbVar10[0x1b2] = 0;
    pbVar10[0x1b3] = 0;
    pbVar10[0x1b4] = 0;
    pbVar10[0x1b5] = 0;
    pbVar10[0x1b6] = 0;
    pbVar10[0x1b7] = 0;
    *(undefined8 **)(pbVar10 + 0x1b8) = puVar42;
    pbVar10[0x1c0] = 2;
    pbVar10[0x1c1] = 0;
    pbVar10[0x1c2] = 0;
    pbVar10[0x1c3] = 0;
    pbVar10[0x1c4] = 0;
    pbVar10[0x1c5] = 0;
    pbVar10[0x1c6] = 0;
    pbVar10[0x1c7] = 0;
    pbVar10[0x1c8] = 0;
    pbVar10[0x1c9] = 0;
    pbVar10[0x1ca] = 0;
    pbVar10[0x1cb] = 0;
    pbVar10[0x1cc] = 0;
    pbVar10[0x1cd] = 0;
    pbVar10[0x1ce] = 0;
    pbVar10[0x1cf] = 0x80;
    pbVar10[0x1e0] = 0;
    pbVar10[0x1e1] = 0;
    pbVar10[0x1e2] = 0;
    pbVar10[0x1e3] = 0;
    pbVar10[0x1f0] = 0;
    pbVar10[0x1f1] = 0;
    pbVar10[0x1f2] = 0;
    pbVar10[499] = 0x40;
    pbVar10[500] = 1;
    pbVar10[0x1f5] = 0;
    country_core::utils::debug_viz::DebugLine::show();
  }
  if (pbVar10[0x198] != 0) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_05 == 1) {
      plVar37 = extraout_RAX_05 + 1;
    }
    else if ((*extraout_RAX_05 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar37 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0)) {
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    if (*plVar37 != 0) {
                    /* WARNING: Subroutine does not return */
      core::cell::panic_already_borrowed();
    }
    *plVar37 = -1;
    *(longlong **)(pbVar10 + 0x198) = plVar37;
    puffin::ThreadProfiler::end_scope();
    **(longlong **)(pbVar10 + 0x198) = **(longlong **)(pbVar10 + 0x198) + 1;
  }
  return;
}

