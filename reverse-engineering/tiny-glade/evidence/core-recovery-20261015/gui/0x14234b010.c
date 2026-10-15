
void __cdecl toolbar::photomode::do_photomode(void)

{
  char *pcVar1;
  byte *pbVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  int *piVar8;
  float *pfVar9;
  uint *size;
  longlong lVar10;
  undefined4 *puVar11;
  ulonglong *puVar12;
  ulonglong *puVar13;
  longlong lVar14;
  undefined4 *puVar15;
  undefined4 *puVar16;
  ulonglong *puVar17;
  longlong lVar18;
  longlong lVar19;
  byte *pbVar20;
  float *pfVar21;
  undefined4 *puVar22;
  undefined4 *puVar23;
  longlong lVar24;
  undefined8 uVar25;
  ulonglong uVar26;
  undefined8 *puVar27;
  uint uVar28;
  uint uVar29;
  char extraout_AL;
  char extraout_AL_00;
  undefined1 extraout_AL_01;
  undefined1 extraout_AL_02;
  char extraout_AL_03;
  char extraout_AL_04;
  char extraout_AL_05;
  char extraout_AL_06;
  char extraout_AL_07;
  char extraout_AL_08;
  char extraout_AL_09;
  char extraout_AL_10;
  byte bVar30;
  uint extraout_EAX;
  int iVar31;
  int *piVar32;
  ulonglong uVar33;
  longlong lVar34;
  ulonglong uVar35;
  longlong extraout_RAX;
  undefined8 *extraout_RAX_00;
  longlong *plVar36;
  uint *extraout_RAX_01;
  byte *extraout_RAX_02;
  longlong extraout_RAX_03;
  ulonglong extraout_RAX_04;
  longlong extraout_RAX_05;
  ulonglong extraout_RAX_06;
  ulonglong extraout_RAX_07;
  ulonglong extraout_RAX_08;
  longlong extraout_RAX_09;
  longlong *in_RCX;
  ulonglong uVar37;
  undefined8 *in_RDX;
  int *piVar38;
  ulonglong uVar39;
  int *piVar40;
  longlong lVar41;
  byte bVar42;
  ushort uVar43;
  longlong lVar44;
  ulonglong uVar45;
  longlong *in_R8;
  longlong lVar46;
  int *piVar47;
  ulonglong uVar48;
  longlong *in_R9;
  longlong lVar49;
  longlong lVar50;
  longlong lVar51;
  int *piVar52;
  uint uVar53;
  longlong lVar54;
  longlong lVar55;
  bool bVar56;
  bool bVar57;
  bool bVar58;
  bool bVar59;
  char cVar60;
  char cVar63;
  char cVar64;
  float fVar61;
  float extraout_XMM0_Da;
  char cVar65;
  undefined1 auVar62 [16];
  float fVar66;
  undefined1 auVar67 [16];
  undefined1 auVar68 [16];
  undefined1 auVar69 [16];
  undefined1 auVar70 [16];
  float fVar72;
  undefined1 auVar71 [16];
  float fVar73;
  float fVar74;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  longlong in_stack_00000038;
  longlong in_stack_00000040;
  longlong *in_stack_00000050;
  longlong *in_stack_00000060;
  longlong local_238;
  longlong local_230;
  
  piVar8 = (int *)*in_RDX;
  pfVar9 = (float *)in_RDX[0xc];
  size = (uint *)in_RDX[0x10];
  lVar10 = in_RDX[0x14];
  lVar51 = in_stack_00000050[4];
  puVar11 = (undefined4 *)in_stack_00000050[6];
  uVar3 = *(undefined4 *)((longlong)in_stack_00000050 + 0x3c);
  puVar12 = (ulonglong *)in_stack_00000050[8];
  lVar50 = in_stack_00000050[9];
  puVar13 = (ulonglong *)in_stack_00000050[0xd];
  lVar54 = in_stack_00000050[0xe];
  lVar14 = in_stack_00000050[0x12];
  puVar15 = (undefined4 *)in_stack_00000050[0x14];
  uVar4 = *(undefined4 *)((longlong)in_stack_00000050 + 0xac);
  lVar55 = in_stack_00000050[0x16];
  puVar16 = (undefined4 *)in_stack_00000050[0x18];
  uVar5 = *(undefined4 *)((longlong)in_stack_00000050 + 0xcc);
  puVar17 = (ulonglong *)in_stack_00000050[0x1e];
  lVar18 = in_stack_00000050[0x1f];
  *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
  lVar19 = *in_stack_00000030;
  country_core::resources::gui::EscMenuState::is_open();
  if (extraout_AL != '\0') {
    *(undefined1 *)(lVar19 + 10) = 0;
    *(undefined1 *)(lVar19 + 0xd) = 1;
  }
  iVar31 = *piVar8;
  if (iVar31 != 0xd) {
    if (iVar31 == 0x13) {
      country_core::input::action_set::DigitalActionState::just_pressed();
      if (extraout_AL_00 == '\0') goto LAB_14234b1f7;
      bVar42 = *(byte *)(lVar19 + 10);
      *(byte *)(lVar19 + 10) = bVar42 ^ 1;
    }
    else {
      *(undefined1 *)(lVar19 + 10) = 0;
      bVar42 = 1;
    }
    *(byte *)(lVar19 + 0xd) = bVar42;
  }
LAB_14234b1f7:
  uVar48 = *puVar13;
  piVar32 = (int *)0x0;
  piVar52 = (int *)(uVar48 - *(ulonglong *)(lVar54 + 0x18));
  if (uVar48 < *(ulonglong *)(lVar54 + 0x18)) {
    piVar52 = piVar32;
  }
  piVar38 = (int *)(uVar48 - *(ulonglong *)(lVar54 + 0x38));
  if (uVar48 < *(ulonglong *)(lVar54 + 0x38)) {
    piVar38 = piVar32;
  }
  lVar46 = (longlong)piVar52 * 0x28 + *(longlong *)(lVar54 + 8);
  lVar49 = (longlong)piVar38 * 0x28 + *(longlong *)(lVar54 + 0x28);
  bVar56 = *(int **)(lVar54 + 0x10) < piVar52;
  piVar52 = (int *)((longlong)*(int **)(lVar54 + 0x10) - (longlong)piVar52);
  bVar58 = piVar52 == (int *)0x0;
  if (bVar56) {
    piVar52 = piVar32;
  }
  if (bVar56) {
    lVar46 = 8;
  }
  piVar47 = (int *)(lVar46 + -0x28 + (longlong)piVar52 * 0x28);
  if (bVar56 || bVar58) {
    piVar47 = piVar32;
  }
  bVar57 = *(int **)(lVar54 + 0x30) < piVar38;
  piVar38 = (int *)((longlong)*(int **)(lVar54 + 0x30) - (longlong)piVar38);
  bVar59 = piVar38 == (int *)0x0;
  if (bVar57) {
    lVar49 = 8;
    piVar38 = piVar32;
  }
  piVar40 = (int *)(lVar49 + -0x28 + (longlong)piVar38 * 0x28);
  if (bVar57 || bVar59) {
    piVar40 = piVar47;
  }
  piVar52 = (int *)((longlong)piVar38 + (longlong)piVar52);
  if (!bVar56 && !bVar58 || !bVar57 && !bVar59) {
    piVar52 = piVar32;
  }
  *puVar13 = *(longlong *)(lVar54 + 0x40) - (longlong)piVar52;
  if (!bVar56 && !bVar58 || !bVar57 && !bVar59) {
    if ((*piVar40 == 0x13) && (piVar40[4] != 0xd)) {
      *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
      *(undefined1 *)(*in_RCX + 1) = 1;
      country_core::startup::init_photomode_state::PhotomodeState::on_enable_photomode();
      country_core::resources::photo_mode::impl$1::from();
      *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
      *(undefined1 *)(*in_R9 + 0x103) = extraout_AL_01;
      **(undefined4 **)(in_stack_00000038 + 0x10) = *(undefined4 *)(in_stack_00000038 + 0x1c);
      country_core::resources::image_formation::ImageFormationConfigs::switch_kind();
      if (*(char *)(lVar19 + 0xb) == '\0') {
        *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
        country_core::systems::camera_rig::MainCamera::match_free_camera_to_normal();
        *(undefined1 *)(lVar19 + 0xb) = 1;
      }
    }
    if ((piVar40[4] == 0x13) && (*piVar40 != 0xd)) {
      *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
      *(undefined1 *)(*in_R9 + 0x103) = 0;
      **(undefined4 **)(in_stack_00000038 + 0x10) = *(undefined4 *)(in_stack_00000038 + 0x1c);
      country_core::resources::image_formation::ImageFormationConfigs::switch_kind();
      **(undefined4 **)(in_stack_00000040 + 0x10) = *(undefined4 *)(in_stack_00000040 + 0x1c);
      country_core::systems::render_loop::vermeer::VermeerState::deactivate();
      *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
      *(undefined1 *)(*in_RCX + 1) = 0;
    }
  }
  pbVar20 = (byte *)*in_RCX;
  lVar54 = *in_R9;
  country_core::resources::audio_resources::AudioCore::set_parameter();
  uVar48 = *puVar12;
  uVar33 = 0;
  uVar37 = uVar48 - *(ulonglong *)(lVar50 + 0x18);
  if (uVar48 < *(ulonglong *)(lVar50 + 0x18)) {
    uVar37 = uVar33;
  }
  uVar35 = uVar48 - *(ulonglong *)(lVar50 + 0x38);
  if (uVar48 < *(ulonglong *)(lVar50 + 0x38)) {
    uVar35 = uVar33;
  }
  lVar46 = uVar37 * 0x10 + *(longlong *)(lVar50 + 8);
  uVar48 = *(ulonglong *)(lVar50 + 0x10) - uVar37;
  if (*(ulonglong *)(lVar50 + 0x10) < uVar37) {
    lVar46 = 8;
    uVar48 = uVar33;
  }
  lVar49 = uVar35 * 0x10 + *(longlong *)(lVar50 + 0x28);
  uVar37 = *(ulonglong *)(lVar50 + 0x30) - uVar35;
  if (*(ulonglong *)(lVar50 + 0x30) < uVar35) {
    lVar49 = 8;
    uVar37 = uVar33;
  }
  *puVar12 = (*(longlong *)(lVar50 + 0x40) - uVar37) - uVar48;
  uVar6 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  pfVar21 = (float *)*in_R8;
  puVar22 = (undefined4 *)in_R8[2];
  uVar7 = *(undefined4 *)((longlong)in_R9 + 0x1c);
  puVar23 = (undefined4 *)in_R9[2];
  lVar50 = lVar49;
  lVar44 = lVar46;
  do {
    if ((lVar44 == 0) || (lVar44 == uVar48 * 0x10 + lVar46)) {
      if ((lVar50 == 0) || (lVar50 == uVar37 * 0x10 + lVar49)) goto LAB_14234b653;
      lVar44 = lVar50 + 0x10;
      lVar34 = 0;
      lVar41 = lVar50;
    }
    else {
      lVar34 = lVar44 + 0x10;
      lVar41 = lVar44;
      lVar44 = lVar50;
    }
    *puVar12 = *puVar12 + 1;
    lVar50 = lVar44;
    lVar44 = lVar34;
    if (*(char *)(lVar41 + 8) != '\x04') {
      if (*(char *)(lVar54 + 0x103) == '\x03') {
        *puVar16 = uVar5;
        lVar34 = *(longlong *)(lVar55 + 0x30);
        uVar25 = *(undefined8 *)(lVar55 + 0x40);
        if (lVar34 == *(longlong *)(lVar55 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        lVar24 = *(longlong *)(lVar55 + 0x28);
        *(undefined8 *)(lVar24 + lVar34 * 0x10) = uVar25;
        *(undefined1 *)(lVar24 + 8 + lVar34 * 0x10) = 1;
        *(longlong *)(lVar55 + 0x30) = lVar34 + 1;
        *(longlong *)(lVar55 + 0x40) = *(longlong *)(lVar55 + 0x40) + 1;
      }
      else if (*(char *)(lVar41 + 8) == '\x02') {
        *puVar16 = uVar5;
        lVar34 = *(longlong *)(lVar55 + 0x30);
        uVar25 = *(undefined8 *)(lVar55 + 0x40);
        if (lVar34 == *(longlong *)(lVar55 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        lVar24 = *(longlong *)(lVar55 + 0x28);
        *(undefined8 *)(lVar24 + lVar34 * 0x10) = uVar25;
        *(undefined1 *)(lVar24 + 8 + lVar34 * 0x10) = 0;
        *(longlong *)(lVar55 + 0x30) = lVar34 + 1;
        *(longlong *)(lVar55 + 0x40) = *(longlong *)(lVar55 + 0x40) + 1;
        if (*(char *)(lVar19 + 0xc) == '\0') {
          *puVar22 = uVar6;
          country_core::systems::camera_rig::MainCamera::init_walk_camera();
          *(undefined1 *)(lVar19 + 0xc) = 1;
        }
      }
      country_core::resources::photo_mode::impl$1::from();
      *puVar23 = uVar7;
      *(undefined1 *)(lVar54 + 0x103) = extraout_AL_02;
      *(undefined1 *)(lVar19 + 9) = *(undefined1 *)(lVar41 + 8);
    }
  } while( true );
LAB_14234bf3f:
  uVar48 = *(ulonglong *)(local_238 + 0x1a8) & uVar48;
  pcVar1 = (char *)(lVar10 + uVar48);
  cVar60 = auVar62[0];
  auVar70[0] = -(*pcVar1 == cVar60);
  cVar63 = auVar62[1];
  auVar70[1] = -(pcVar1[1] == cVar63);
  cVar64 = auVar62[2];
  auVar70[2] = -(pcVar1[2] == cVar64);
  cVar65 = auVar62[3];
  auVar70[3] = -(pcVar1[3] == cVar65);
  auVar70[4] = -(pcVar1[4] == cVar60);
  auVar70[5] = -(pcVar1[5] == cVar63);
  auVar70[6] = -(pcVar1[6] == cVar64);
  auVar70[7] = -(pcVar1[7] == cVar65);
  auVar70[8] = -(pcVar1[8] == cVar60);
  auVar70[9] = -(pcVar1[9] == cVar63);
  auVar70[10] = -(pcVar1[10] == cVar64);
  auVar70[0xb] = -(pcVar1[0xb] == cVar65);
  auVar70[0xc] = -(pcVar1[0xc] == cVar60);
  auVar70[0xd] = -(pcVar1[0xd] == cVar63);
  auVar70[0xe] = -(pcVar1[0xe] == cVar64);
  auVar70[0xf] = -(pcVar1[0xf] == cVar65);
  uVar43 = (ushort)(SUB161(auVar70 >> 7,0) & 1) | (ushort)(SUB161(auVar70 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar70 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar70 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar70 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar70 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar70 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar70 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar70 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar70 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar70 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar70 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar70 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar70 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar70 >> 0x77,0) & 1) << 0xe | (ushort)(auVar70[0xf] >> 7) << 0xf;
  uVar53 = (uint)uVar43;
  while (uVar43 != 0) {
    uVar28 = 0;
    for (uVar29 = uVar53; (uVar29 & 1) == 0; uVar29 = uVar29 >> 1 | 0x80000000) {
      uVar28 = uVar28 + 1;
    }
    lVar50 = (uVar28 + uVar48 & *(ulonglong *)(local_238 + 0x1a8)) * -0xc0;
    if (local_230 == *(longlong *)(lVar10 + -0xc0 + lVar50)) {
      *(undefined4 *)(lVar10 + -8 + lVar50) = 0x3f800000;
      goto LAB_14234bfb2;
    }
    uVar43 = (ushort)(uVar53 - 1) & (ushort)uVar53;
    uVar53 = CONCAT22((short)(uVar53 - 1 >> 0x10),uVar43);
  }
  auVar68[0] = -(*pcVar1 == -1);
  auVar68[1] = -(pcVar1[1] == -1);
  auVar68[2] = -(pcVar1[2] == -1);
  auVar68[3] = -(pcVar1[3] == -1);
  auVar68[4] = -(pcVar1[4] == -1);
  auVar68[5] = -(pcVar1[5] == -1);
  auVar68[6] = -(pcVar1[6] == -1);
  auVar68[7] = -(pcVar1[7] == -1);
  auVar68[8] = -(pcVar1[8] == -1);
  auVar68[9] = -(pcVar1[9] == -1);
  auVar68[10] = -(pcVar1[10] == -1);
  auVar68[0xb] = -(pcVar1[0xb] == -1);
  auVar68[0xc] = -(pcVar1[0xc] == -1);
  auVar68[0xd] = -(pcVar1[0xd] == -1);
  auVar68[0xe] = -(pcVar1[0xe] == -1);
  auVar68[0xf] = -(pcVar1[0xf] == -1);
  if ((((((((((((((((SUB161(auVar68 >> 7,0) & 1) != 0 || (SUB161(auVar68 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar68 >> 0x17,0) & 1) != 0) || (SUB161(auVar68 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar68 >> 0x27,0) & 1) != 0) || (SUB161(auVar68 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar68 >> 0x37,0) & 1) != 0) || (SUB161(auVar68 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar68 >> 0x47,0) & 1) != 0) || (SUB161(auVar68 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar68 >> 0x57,0) & 1) != 0) || (SUB161(auVar68 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar68 >> 0x67,0) & 1) != 0) || (SUB161(auVar68 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar68 >> 0x77,0) & 1) != 0) || auVar68[0xf] < '\0') goto LAB_14234bf9e;
  uVar48 = uVar48 + lVar51 + 0x10;
  lVar51 = lVar51 + 0x10;
  goto LAB_14234bf3f;
LAB_14234b653:
  if (pbVar20[1] == 0) {
    bVar42 = 0;
  }
  else {
    bVar42 = *pbVar20 ^ 1;
  }
  uVar48 = *puVar17;
  uVar33 = 0;
  uVar37 = uVar48 - *(ulonglong *)(lVar18 + 0x18);
  if (uVar48 < *(ulonglong *)(lVar18 + 0x18)) {
    uVar37 = uVar33;
  }
  uVar35 = uVar48 - *(ulonglong *)(lVar18 + 0x38);
  if (uVar48 < *(ulonglong *)(lVar18 + 0x38)) {
    uVar35 = uVar33;
  }
  lVar50 = *(longlong *)(lVar18 + 8);
  uVar48 = *(ulonglong *)(lVar18 + 0x30);
  uVar39 = uVar48 - uVar35;
  if (uVar48 < uVar35) {
    uVar39 = uVar33;
  }
  uVar26 = *(ulonglong *)(lVar18 + 0x10);
  uVar45 = uVar26 - uVar37;
  if (uVar26 < uVar37) {
    uVar45 = uVar33;
  }
  lVar54 = *(longlong *)(lVar18 + 0x28);
  uVar33 = *(ulonglong *)(lVar18 + 0x40);
  *puVar17 = (uVar33 - uVar39) - uVar45;
  if (uVar26 >= uVar37 && uVar26 - uVar37 != 0 || uVar48 >= uVar35 && uVar48 - uVar35 != 0) {
    lVar54 = lVar54 + uVar35 * 0x10;
    lVar50 = lVar50 + uVar37 * 0x10;
    if (uVar26 < uVar37) {
      lVar50 = 8;
    }
    lVar55 = 0;
    if (uVar37 < uVar26) {
      lVar55 = lVar50 + -0x10 + uVar45 * 0x10;
    }
    if (uVar48 < uVar35) {
      lVar54 = 8;
    }
    lVar50 = lVar54 + -0x10 + uVar39 * 0x10;
    if (uVar48 <= uVar35) {
      lVar50 = lVar55;
    }
    *puVar17 = uVar33;
    if (*(char *)(lVar50 + 8) == '\x01') {
      *(undefined1 *)(lVar19 + 10) = 0;
      *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
      pbVar20[1] = 1;
      *puVar15 = uVar4;
      lVar50 = *(longlong *)(lVar14 + 0x30);
      uVar25 = *(undefined8 *)(lVar14 + 0x40);
      if (lVar50 == *(longlong *)(lVar14 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      lVar54 = *(longlong *)(lVar14 + 0x28);
      *(undefined4 *)(lVar54 + lVar50 * 0x18) = 0x13;
      *(undefined8 *)(lVar54 + 0x10 + lVar50 * 0x18) = uVar25;
      *(longlong *)(lVar14 + 0x30) = lVar50 + 1;
      *(longlong *)(lVar14 + 0x40) = *(longlong *)(lVar14 + 0x40) + 1;
    }
  }
  bVar30 = pbVar20[1];
  if (bVar30 != 1) {
LAB_14234c104:
    *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
    *pbVar20 = bVar30;
    return;
  }
  if (bVar42 != 0) {
    if (*piVar8 == 0xd) {
      *puVar11 = uVar3;
      lVar50 = *(longlong *)(lVar51 + 0x30);
      uVar25 = *(undefined8 *)(lVar51 + 0x40);
      if (lVar50 == *(longlong *)(lVar51 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar54 = *(longlong *)(lVar51 + 0x28);
      *(undefined8 *)(lVar54 + lVar50 * 0x10) = uVar25;
      *(undefined4 *)(lVar54 + 8 + lVar50 * 0x10) = 0x3000300;
      *(longlong *)(lVar51 + 0x30) = lVar50 + 1;
      *(longlong *)(lVar51 + 0x40) = *(longlong *)(lVar51 + 0x40) + 1;
    }
    *(undefined4 *)in_stack_00000050[2] = *(undefined4 *)((longlong)in_stack_00000050 + 0x1c);
    lVar51 = *in_stack_00000050;
    lVar50 = *(longlong *)(lVar51 + 0x30);
    uVar25 = *(undefined8 *)(lVar51 + 0x40);
    if (lVar50 == *(longlong *)(lVar51 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                ();
    }
    *(undefined8 *)(*(longlong *)(lVar51 + 0x28) + lVar50 * 8) = uVar25;
    *(longlong *)(lVar51 + 0x30) = lVar50 + 1;
    *(longlong *)(lVar51 + 0x40) = *(longlong *)(lVar51 + 0x40) + 1;
  }
  fVar66 = 0.0;
  if ((((*(float *)(lVar10 + 0x160) != 0.0) || (NAN(*(float *)(lVar10 + 0x160)))) &&
      (country_core::systems::camera_rig::MainCamera::mode(), extraout_AL_03 != '\x04')) &&
     (extraout_AL_03 != '\x01')) {
    *puVar22 = uVar6;
    country_core::systems::camera_rig::MainCamera::lens();
    fVar74 = *(float *)(extraout_RAX + 0xc);
    fVar73 = *(float *)(extraout_RAX + 0x10);
    if (fVar73 < fVar74) {
                    // WARNING: Subroutine does not return
      core::f32::impl$0::clamp::do_panic::runtime();
    }
    fVar61 = (float)exp2f();
    fVar61 = fVar61 * *(float *)(extraout_RAX + 8);
    if (fVar74 <= fVar61) {
      fVar74 = fVar61;
    }
    if (fVar74 <= fVar73) {
      fVar73 = fVar74;
    }
    *(float *)(extraout_RAX + 8) = fVar73;
  }
  country_core::systems::ui::resources::widget_params::WidgetParams::new();
  country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world();
  country_core::systems::ui::resources::widget_params::UiSounds::empty();
  country_core::systems::ui::resources::widget_params::WidgetParams::sounds();
  country_core::systems::ui::resources::ui_state::UiState::add<ref_<str_>_>();
  country_core::systems::camera_rig::MainCamera::mode();
  *(undefined4 *)(lVar19 + 4) = 0xffffffff;
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX_00 != (undefined8 *)0x0) {
    if ((extraout_RAX_00[3] != 0x57) ||
       (iVar31 = memcmp(&anon_35c0e18e72144e77e5fdf2ecf42a564d_0_llvm_9588986728419368905,
                        (void *)extraout_RAX_00[2],0x57), iVar31 != 0)) {
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    plVar36 = (longlong *)(**(code **)(extraout_RAX_00[1] + 0x18))(*extraout_RAX_00);
    if (plVar36 != (longlong *)0x0) {
      if (plVar36[1] == 0) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      puVar27 = (undefined8 *)*plVar36;
      uVar53 = *(uint *)((longlong)puVar27 + 0xc);
      *puVar22 = uVar6;
      country_core::systems::camera_rig::MainCamera::focus();
      if (*extraout_RAX_01 < 2) {
        if (uVar53 == 0 || uVar53 != (extraout_EAX & 0xff)) {
          if (*extraout_RAX_01 == 0) {
            *(uint *)(lVar19 + 4) = extraout_EAX & 0xff;
          }
        }
        else {
          uVar53 = *(uint *)(puVar27 + 1);
          uVar25 = *puVar27;
          *extraout_RAX_01 = 1;
          *(undefined8 *)(extraout_RAX_01 + 1) = uVar25;
          extraout_RAX_01[3] = uVar53;
        }
      }
      fVar74 = *pfVar9;
      fVar73 = pfVar9[1];
      country_core::resources::window::WindowSize::as_vec2();
      fVar61 = country_core::resources::window::WindowSize::aspect_ratio(size);
      *puVar22 = uVar6;
      country_core::systems::camera_rig::MainCamera::focus();
      uVar53 = 2;
      if (*(int *)extraout_RAX_02 - 2U < 2) {
        uVar53 = *(int *)extraout_RAX_02 - 2U;
      }
      if (uVar53 == 0) {
        fVar61 = 0.0;
        fVar66 = 0.0;
        cVar60 = *(char *)(lVar19 + 0xd);
      }
      else {
        fVar74 = fVar74 / extraout_XMM0_Da;
        fVar73 = fVar73 / fVar66;
        fVar61 = fVar61 * (fVar74 + fVar74 + -1.0);
        fVar66 = -(fVar73 + fVar73 + -1.0);
        if (uVar53 == 1) {
          country_core::systems::ui::resources::slider3d::impl$1::into_drag_state();
          bVar42 = 3;
          if ((byte)(extraout_AL_04 - 2U) < 3) {
            bVar42 = extraout_AL_04 - 2U;
          }
          if ((byte)(bVar42 - 1) < 2) {
            *(float *)(extraout_RAX_02 + 4) = fVar61;
            *(float *)(extraout_RAX_02 + 8) = fVar66;
            cVar60 = *(char *)(lVar19 + 0xd);
          }
          else {
            fVar61 = *(float *)(extraout_RAX_02 + 4);
            fVar66 = *(float *)(extraout_RAX_02 + 8);
            cVar60 = *(char *)(lVar19 + 0xd);
          }
        }
        else {
          country_core::systems::ui::resources::slider3d::impl$1::into_drag_state();
          bVar42 = 3;
          if ((byte)(extraout_AL_05 - 2U) < 3) {
            bVar42 = extraout_AL_05 - 2U;
          }
          if ((byte)(bVar42 - 1) < 2) {
            *(uint *)(lVar19 + 4) = extraout_EAX & 0xff;
            *(float *)(extraout_RAX_02 + 0x10) = fVar61;
            *(float *)(extraout_RAX_02 + 0x14) = fVar66;
            extraout_RAX_02[0] = 0;
            extraout_RAX_02[1] = 0;
            extraout_RAX_02[2] = 0;
            extraout_RAX_02[3] = 0;
            cVar60 = *(char *)(lVar19 + 0xd);
          }
          else if ((*extraout_RAX_02 & 1) == 0) {
            fVar61 = *(float *)(extraout_RAX_02 + 0x10);
            fVar66 = *(float *)(extraout_RAX_02 + 0x14);
            cVar60 = *(char *)(lVar19 + 0xd);
          }
          else {
            fVar66 = *(float *)(extraout_RAX_02 + 4);
            fVar74 = *(float *)(extraout_RAX_02 + 8);
            fVar73 = *(float *)(extraout_RAX_02 + 0xc);
            fVar61 = fVar73 * pfVar21[0x28] + fVar74 * pfVar21[0x24] + fVar66 * pfVar21[0x20] +
                     pfVar21[0x2c];
            fVar72 = fVar73 * pfVar21[0x29] + fVar74 * pfVar21[0x25] + fVar66 * pfVar21[0x21] +
                     pfVar21[0x2d];
            fVar66 = fVar73 * pfVar21[0x2a] + fVar74 * pfVar21[0x26] + fVar66 * pfVar21[0x22] +
                     pfVar21[0x2e];
            auVar71._0_4_ =
                 fVar66 * pfVar21[8] + fVar72 * pfVar21[4] + fVar61 * *pfVar21 + pfVar21[0xc];
            auVar71._4_4_ =
                 fVar66 * pfVar21[9] + fVar72 * pfVar21[5] + fVar61 * pfVar21[1] + pfVar21[0xd];
            auVar71._8_4_ =
                 fVar66 * pfVar21[10] + fVar72 * pfVar21[6] + fVar61 * pfVar21[2] + pfVar21[0xe];
            auVar71._12_4_ =
                 fVar66 * pfVar21[0xb] + fVar72 * pfVar21[7] + fVar61 * pfVar21[3] + pfVar21[0xf];
            auVar62._4_4_ = auVar71._12_4_;
            auVar62._0_4_ = auVar71._12_4_;
            auVar62._8_4_ = auVar71._12_4_;
            auVar62._12_4_ = auVar71._12_4_;
            auVar62 = divps(auVar71,auVar62);
            fVar66 = auVar62._4_4_;
            fVar61 = country_core::resources::window::WindowSize::aspect_ratio(size);
            fVar61 = fVar61 * auVar62._0_4_;
            cVar60 = *(char *)(lVar19 + 0xd);
          }
        }
      }
      if (((cVar60 != '\0') &&
          (country_core::resources::gui::EscMenuState::is_closed(), extraout_AL_06 != '\0')) &&
         (country_core::systems::camera_rig::MainCamera::mode(), 1 < (byte)(extraout_AL_07 - 3U))) {
        fVar73 = country_core::resources::window::WindowSize::aspect_ratio(size);
        country_core::systems::camera_rig::MainCamera::focus();
        fVar74 = *(float *)(extraout_RAX_03 + 0x18);
        *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
        plVar36 = (longlong *)*in_stack_00000028;
        ::utils::calculate_hash<tuple_<ref_<str_>,ref_<tuple_<>_>_>_>();
        uVar48 = extraout_RAX_04 ^ 0xb1695320004fb443;
        mei::core::MeiContext::reuse_or_create_node();
        *(undefined1 *)(extraout_RAX_05 + 0x5d) = 0;
        if ((uVar48 & 1) == 0) {
          fVar74 = fVar74 * 1000.0;
          lVar10 = plVar36[2];
          if (lVar10 == *plVar36) {
            alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>()
            ;
          }
          lVar51 = plVar36[1];
          lVar50 = lVar10 * 0x38;
          *(undefined4 *)(lVar51 + lVar50) = 4;
          puVar27 = (undefined8 *)(lVar51 + 4 + lVar50);
          *puVar27 = CONCAT44((1.0 - fVar66) * 0.5 * 1000.0,(fVar61 + fVar73) * 0.5 * 1000.0);
          puVar27[1] = 0;
          *(undefined4 *)(lVar51 + 0x14 + lVar50) = 1;
          *(float *)(lVar51 + 0x18 + lVar50) = fVar74;
          *(float *)(lVar51 + 0x1c + lVar50) = fVar74;
          *(ulonglong *)(lVar51 + 0x30 + lVar50) = extraout_RAX_04 ^ 0xb1695320004fb443;
          plVar36[2] = lVar10 + 1;
        }
        country_core::systems::camera_rig::MainCamera::focus();
        mei::config::impl$18::template_with_id<u64>();
        if (*(longlong *)(local_238 + 0x1b8) != 0) {
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
          lVar10 = *(longlong *)(local_238 + 0x1a0);
          bVar42 = (byte)(extraout_RAX_06 >> 0x39);
          auVar62 = ZEXT216(CONCAT11(bVar42,bVar42));
          auVar62 = pshuflw(auVar62,auVar62,0);
          lVar51 = 0;
          uVar48 = extraout_RAX_06;
          while( true ) {
            uVar48 = *(ulonglong *)(local_238 + 0x1a8) & uVar48;
            pcVar1 = (char *)(lVar10 + uVar48);
            cVar60 = auVar62[0];
            auVar69[0] = -(*pcVar1 == cVar60);
            cVar63 = auVar62[1];
            auVar69[1] = -(pcVar1[1] == cVar63);
            cVar64 = auVar62[2];
            auVar69[2] = -(pcVar1[2] == cVar64);
            cVar65 = auVar62[3];
            auVar69[3] = -(pcVar1[3] == cVar65);
            auVar69[4] = -(pcVar1[4] == cVar60);
            auVar69[5] = -(pcVar1[5] == cVar63);
            auVar69[6] = -(pcVar1[6] == cVar64);
            auVar69[7] = -(pcVar1[7] == cVar65);
            auVar69[8] = -(pcVar1[8] == cVar60);
            auVar69[9] = -(pcVar1[9] == cVar63);
            auVar69[10] = -(pcVar1[10] == cVar64);
            auVar69[0xb] = -(pcVar1[0xb] == cVar65);
            auVar69[0xc] = -(pcVar1[0xc] == cVar60);
            auVar69[0xd] = -(pcVar1[0xd] == cVar63);
            auVar69[0xe] = -(pcVar1[0xe] == cVar64);
            auVar69[0xf] = -(pcVar1[0xf] == cVar65);
            uVar43 = (ushort)(SUB161(auVar69 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar69 >> 0xf,0) & 1) << 1 |
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
                     (ushort)(SUB161(auVar69 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar69[0xf] >> 7) << 0xf;
            uVar53 = (uint)uVar43;
            while (uVar43 != 0) {
              uVar28 = 0;
              for (uVar29 = uVar53; (uVar29 & 1) == 0; uVar29 = uVar29 >> 1 | 0x80000000) {
                uVar28 = uVar28 + 1;
              }
              lVar50 = (uVar28 + uVar48 & *(ulonglong *)(local_238 + 0x1a8)) * -0xc0;
              if (local_230 == *(longlong *)(lVar10 + -0xc0 + lVar50)) {
                pbVar2 = (byte *)(lVar10 + -1 + lVar50);
                *pbVar2 = *pbVar2 | 2;
                if (*(longlong *)(local_238 + 0x1b8) == 0) goto LAB_14234bf9e;
                core::hash::BuildHasher::
                hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>
                          ();
                lVar10 = *(longlong *)(local_238 + 0x1a0);
                bVar42 = (byte)(extraout_RAX_07 >> 0x39);
                auVar62 = ZEXT216(CONCAT11(bVar42,bVar42));
                auVar62 = pshuflw(auVar62,auVar62,0);
                lVar51 = 0;
                uVar48 = extraout_RAX_07;
                goto LAB_14234bf3f;
              }
              uVar43 = (ushort)(uVar53 - 1) & (ushort)uVar53;
              uVar53 = CONCAT22((short)(uVar53 - 1 >> 0x10),uVar43);
            }
            auVar67[0] = -(*pcVar1 == -1);
            auVar67[1] = -(pcVar1[1] == -1);
            auVar67[2] = -(pcVar1[2] == -1);
            auVar67[3] = -(pcVar1[3] == -1);
            auVar67[4] = -(pcVar1[4] == -1);
            auVar67[5] = -(pcVar1[5] == -1);
            auVar67[6] = -(pcVar1[6] == -1);
            auVar67[7] = -(pcVar1[7] == -1);
            auVar67[8] = -(pcVar1[8] == -1);
            auVar67[9] = -(pcVar1[9] == -1);
            auVar67[10] = -(pcVar1[10] == -1);
            auVar67[0xb] = -(pcVar1[0xb] == -1);
            auVar67[0xc] = -(pcVar1[0xc] == -1);
            auVar67[0xd] = -(pcVar1[0xd] == -1);
            auVar67[0xe] = -(pcVar1[0xe] == -1);
            auVar67[0xf] = -(pcVar1[0xf] == -1);
            if ((((((((((((((((SUB161(auVar67 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar67 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar67 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar67 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar67 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar67 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar67 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar67 >> 0x3f,0) & 1) != 0) || (SUB161(auVar67 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar67 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar67 >> 0x57,0) & 1) != 0) || (SUB161(auVar67 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar67 >> 0x67,0) & 1) != 0) || (SUB161(auVar67 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar67 >> 0x77,0) & 1) != 0) || auVar67[0xf] < '\0') break;
            uVar48 = uVar48 + lVar51 + 0x10;
            lVar51 = lVar51 + 0x10;
          }
        }
LAB_14234bf9e:
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
LAB_14234bfb2:
      country_core::systems::camera_rig::MainCamera::mode();
      if ((extraout_AL_08 == '\x03') &&
         (country_core::resources::walls::inner_wall_state::StructureType::is_fence(),
         extraout_AL_09 != '\0')) {
        *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
        plVar36 = (longlong *)*in_stack_00000028;
        ::utils::calculate_hash<tuple_<ref_<str_>,ref_<tuple_<>_>_>_>();
        uVar48 = extraout_RAX_08 ^ 0xb1695320004fb443;
        mei::core::MeiContext::reuse_or_create_node();
        *(undefined1 *)(extraout_RAX_09 + 0x5d) = 0;
        if ((uVar48 & 1) == 0) {
          lVar10 = plVar36[2];
          if (lVar10 == *plVar36) {
            alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>()
            ;
          }
          lVar51 = plVar36[1];
          *(undefined4 *)(lVar51 + lVar10 * 0x38) = 1;
          *(ulonglong *)(lVar51 + 0x30 + lVar10 * 0x38) = extraout_RAX_08 ^ 0xb1695320004fb443;
          plVar36[2] = lVar10 + 1;
        }
        mei::config::impl$18::template_with_id<u64>();
      }
      else {
        country_core::systems::camera_rig::MainCamera::mode();
        if ((extraout_AL_10 == '\x03') &&
           (*__imp__ZN12country_core7systems11main_camera20PROMPT_ROLLERCOASTER17hb5c298e904570b37E
            == '\x01')) {
          *(undefined4 *)in_stack_00000060[2] = *(undefined4 *)((longlong)in_stack_00000060 + 0x1c);
          lVar10 = *in_stack_00000060;
          lVar51 = *(longlong *)(lVar10 + 0x30);
          uVar25 = *(undefined8 *)(lVar10 + 0x40);
          if (lVar51 == *(longlong *)(lVar10 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                      ();
          }
          *(undefined8 *)(*(longlong *)(lVar10 + 0x28) + lVar51 * 8) = uVar25;
          *(longlong *)(lVar10 + 0x30) = lVar51 + 1;
          *(longlong *)(lVar10 + 0x40) = *(longlong *)(lVar10 + 0x40) + 1;
        }
      }
      bVar30 = pbVar20[1];
      goto LAB_14234c104;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}


