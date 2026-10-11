
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
country_core::systems::wall::shapes::rectangle::create_rectangle::create_rectangle(void)

{
  char *pcVar1;
  size_t sVar2;
  undefined8 *puVar3;
  int iVar4;
  float fVar5;
  float fVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  undefined4 uVar14;
  undefined4 uVar15;
  undefined4 uVar16;
  ulonglong *puVar17;
  longlong lVar18;
  longlong *plVar19;
  undefined4 *puVar20;
  longlong lVar21;
  undefined4 *puVar22;
  longlong lVar23;
  undefined4 *puVar24;
  longlong lVar25;
  undefined4 *puVar26;
  longlong lVar27;
  undefined4 *puVar28;
  longlong lVar29;
  undefined4 *puVar30;
  longlong lVar31;
  undefined4 *puVar32;
  longlong lVar33;
  undefined4 *puVar34;
  longlong lVar35;
  undefined4 *puVar36;
  longlong lVar37;
  longlong lVar38;
  undefined8 uVar39;
  undefined8 uVar40;
  void *pvVar41;
  uint uVar42;
  uint uVar43;
  undefined8 uVar44;
  undefined8 uVar45;
  float fVar46;
  float fVar47;
  float fVar48;
  float fVar49;
  float fVar50;
  float fVar51;
  float fVar52;
  longlong extraout_RAX;
  ulonglong extraout_RAX_00;
  longlong lVar53;
  ulonglong extraout_RAX_01;
  undefined1 *extraout_RAX_02;
  undefined1 *extraout_RAX_03;
  undefined8 *extraout_RAX_04;
  undefined8 *extraout_RAX_05;
  undefined8 *in_RCX;
  int *piVar54;
  int *piVar55;
  longlong lVar56;
  longlong *in_RDX;
  undefined1 uVar57;
  ulonglong uVar58;
  int *piVar59;
  longlong *in_R8;
  ulonglong uVar60;
  longlong lVar61;
  uint uVar62;
  longlong in_R9;
  ushort uVar63;
  ulonglong uVar64;
  ulonglong uVar65;
  undefined1 *puVar66;
  byte bVar67;
  char cVar68;
  char cVar71;
  char cVar72;
  float extraout_XMM0_Da;
  float fVar69;
  char cVar73;
  undefined1 auVar70 [16];
  float fVar74;
  undefined1 auVar75 [16];
  undefined1 auVar76 [16];
  float fVar77;
  undefined1 auVar78 [16];
  undefined1 auVar79 [16];
  float fVar80;
  float fVar81;
  float fVar82;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  longlong *in_stack_00000038;
  longlong *in_stack_00000040;
  longlong *in_stack_00000048;
  longlong *in_stack_00000060;
  longlong *in_stack_00000068;
  longlong *in_stack_00000070;
  longlong *in_stack_00000078;
  undefined4 uStack_244;
  undefined4 uStack_240;
  undefined4 local_230;
  int *local_190;
  int *local_188;
  
  puVar17 = (ulonglong *)*in_RCX;
  lVar18 = in_RCX[1];
  uVar60 = *puVar17;
  uVar64 = 0;
  uVar65 = uVar60 - *(ulonglong *)(lVar18 + 0x18);
  if (uVar60 < *(ulonglong *)(lVar18 + 0x18)) {
    uVar65 = uVar64;
  }
  uVar58 = uVar60 - *(ulonglong *)(lVar18 + 0x38);
  if (uVar60 < *(ulonglong *)(lVar18 + 0x38)) {
    uVar58 = uVar64;
  }
  local_188 = (int *)(uVar65 * 0x38 + *(longlong *)(lVar18 + 8));
  uVar60 = *(ulonglong *)(lVar18 + 0x10) - uVar65;
  if (*(ulonglong *)(lVar18 + 0x10) < uVar65) {
    local_188 = (int *)&DAT_00000008;
    uVar60 = uVar64;
  }
  local_190 = (int *)(uVar58 * 0x38 + *(longlong *)(lVar18 + 0x28));
  uVar65 = *(ulonglong *)(lVar18 + 0x30) - uVar58;
  if (*(ulonglong *)(lVar18 + 0x30) < uVar58) {
    local_190 = (int *)&DAT_00000008;
    uVar65 = uVar64;
  }
  *puVar17 = (*(longlong *)(lVar18 + 0x40) - uVar65) - uVar60;
  fVar52 = __real_40a80000;
  fVar51 = __real_bfc00000;
  fVar50 = __real_41400000;
  fVar49 = __real_40400000;
  fVar48 = __real_40c00000;
  fVar47 = __real_3f800000;
  fVar46 = __real_3fc00000;
  piVar54 = local_188 + uVar60 * 0xe;
  piVar55 = local_190 + uVar65 * 0xe;
  uVar7 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  plVar19 = (longlong *)*in_RDX;
  puVar20 = (undefined4 *)in_RDX[2];
  lVar18 = *in_stack_00000068;
  lVar21 = *in_stack_00000060;
  uVar8 = *(undefined4 *)(in_R9 + 0x1c);
  puVar22 = *(undefined4 **)(in_R9 + 0x10);
  uVar9 = *(undefined4 *)((longlong)in_stack_00000078 + 0x1c);
  lVar23 = *in_stack_00000078;
  puVar24 = (undefined4 *)in_stack_00000078[2];
  uVar10 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
  lVar25 = *in_stack_00000038;
  puVar26 = (undefined4 *)in_stack_00000038[2];
  uVar11 = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
  lVar27 = *in_stack_00000040;
  puVar28 = (undefined4 *)in_stack_00000040[2];
  uVar12 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  lVar29 = *in_R8;
  puVar30 = (undefined4 *)in_R8[2];
  uVar13 = *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
  lVar31 = *in_stack_00000048;
  puVar32 = (undefined4 *)in_stack_00000048[2];
  uVar14 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  lVar33 = *in_stack_00000028;
  puVar34 = (undefined4 *)in_stack_00000028[2];
  uVar15 = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
  lVar35 = *in_stack_00000030;
  puVar36 = (undefined4 *)in_stack_00000030[2];
  lVar37 = *in_stack_00000070;
  lVar38 = in_stack_00000070[1];
LAB_140a93ad4:
  if ((local_188 == (int *)0x0) || (local_188 == piVar54)) {
    if ((local_190 == (int *)0x0) || (local_190 == piVar55)) {
      return;
    }
    local_188 = (int *)0x0;
    piVar59 = local_190;
    local_190 = local_190 + 0xe;
  }
  else {
    piVar59 = local_188;
    local_188 = local_188 + 0xe;
  }
  *puVar17 = *puVar17 + 1;
  iVar4 = piVar59[0xb];
  uVar44 = *(undefined8 *)(piVar59 + 4);
  uVar45 = *(undefined8 *)(piVar59 + 6);
  uStack_244 = (undefined4)((ulonglong)uVar44 >> 0x20);
  uStack_240 = (undefined4)uVar45;
  uVar39 = *(undefined8 *)(piVar59 + 8);
  *puVar24 = uVar9;
  lVar53 = *(longlong *)(lVar23 + 0x30);
  uVar40 = *(undefined8 *)(lVar23 + 0x40);
  if (lVar53 == *(longlong *)(lVar23 + 0x20)) {
    alloc::raw_vec::RawVec::
    grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>();
  }
  lVar61 = *(longlong *)(lVar23 + 0x28);
  lVar56 = lVar53 * 0x30;
  *(undefined4 *)(lVar61 + lVar56) = 1;
  puVar3 = (undefined8 *)(lVar61 + 4 + lVar56);
  *puVar3 = uVar44;
  puVar3[1] = uVar45;
  *(undefined8 *)(lVar61 + 0x14 + lVar56) = uVar39;
  *(int *)(lVar61 + 0x1c + lVar56) = iVar4;
  *(undefined4 *)(lVar61 + 0x20 + lVar56) = 0x40800000;
  *(undefined1 *)(lVar61 + 0x24 + lVar56) = 1;
  *(undefined8 *)(lVar61 + 0x28 + lVar56) = uVar40;
  *(longlong *)(lVar23 + 0x30) = lVar53 + 1;
  *(longlong *)(lVar23 + 0x40) = *(longlong *)(lVar23 + 0x40) + 1;
  *puVar20 = uVar7;
  ::utils::geometry::rectangle::Rectangle2d::as_curve2();
  resources::walls::inner_walls::InnerWalls::new_wall_w_params();
  *puVar20 = uVar7;
  iVar4 = piVar59[0xb];
  resources::walls::inner_walls::InnerWalls::
  _try_update_state<country_core::resources::walls::inner_walls::impl_1::update_elevation::closure_env_0>
            ();
  get_building_and_roof_colors();
  *puVar20 = uVar7;
  resources::walls::inner_walls::InnerWalls::
  _try_update_state<country_core::resources::walls::inner_walls::impl_1::update_style::closure_env_0>
            ();
  *puVar26 = uVar10;
  lVar53 = *(longlong *)(lVar25 + 0x30);
  uVar40 = *(undefined8 *)(lVar25 + 0x40);
  if (lVar53 == *(longlong *)(lVar25 + 0x20)) {
    alloc::raw_vec::RawVec::
    grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
              ();
  }
  lVar61 = *(longlong *)(lVar25 + 0x28);
  *(undefined8 *)(lVar61 + lVar53 * 0x18) = uVar40;
  *(longlong *)(lVar61 + 8 + lVar53 * 0x18) = extraout_RAX;
  *(int *)(lVar61 + 0x10 + lVar53 * 0x18) = iVar4;
  *(undefined1 *)(lVar61 + 0x14 + lVar53 * 0x18) = 0;
  *(longlong *)(lVar25 + 0x30) = lVar53 + 1;
  *(longlong *)(lVar25 + 0x40) = *(longlong *)(lVar25 + 0x40) + 1;
  *puVar28 = uVar11;
  lVar53 = *(longlong *)(lVar27 + 0x30);
  uVar40 = *(undefined8 *)(lVar27 + 0x40);
  if (lVar53 == *(longlong *)(lVar27 + 0x20)) {
    alloc::raw_vec::RawVec::
    grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
              ();
  }
  lVar61 = *(longlong *)(lVar27 + 0x28);
  *(undefined8 *)(lVar61 + lVar53 * 0x18) = uVar40;
  *(longlong *)(lVar61 + 8 + lVar53 * 0x18) = extraout_RAX;
  *(undefined4 *)(lVar61 + 0x10 + lVar53 * 0x18) = uStack_240;
  *(undefined1 *)(lVar61 + 0x14 + lVar53 * 0x18) = 0;
  *(longlong *)(lVar27 + 0x30) = lVar53 + 1;
  *(longlong *)(lVar27 + 0x40) = *(longlong *)(lVar27 + 0x40) + 1;
  *puVar20 = uVar7;
  resources::walls::inner_walls::InnerWalls::
  _try_update_state<country_core::resources::walls::inner_walls::impl_1::update_wall_state_enclosure::closure_env_0>
            ();
  fVar5 = (float)piVar59[8];
  fVar6 = (float)piVar59[9];
  fVar74 = fVar5 / fVar6;
  fVar81 = fVar6 / fVar5;
  fVar82 = fVar81;
  if (fVar81 <= fVar74) {
    fVar82 = fVar74;
  }
  fVar80 = fVar6;
  if (fVar6 <= fVar5) {
    fVar80 = fVar5;
  }
  fVar77 = ((float)(~-(uint)NAN(fVar5) & (uint)fVar80 | -(uint)NAN(fVar5) & (uint)fVar6) +
           ___real_c0100000) / fVar52;
  fVar80 = 0.0;
  if (0.0 <= fVar77) {
    fVar80 = fVar77;
  }
  fVar77 = fVar47;
  if (fVar80 <= fVar47) {
    fVar77 = fVar80;
  }
  ::utils::easing_functions::EasingFunction::apply();
  fVar69 = (extraout_XMM0_Da * fVar48 + (fVar47 - extraout_XMM0_Da) * fVar49) / fVar50;
  fVar80 = 0.0;
  if (0.0 <= fVar69) {
    fVar80 = fVar69;
  }
  fVar69 = fVar47;
  if (fVar80 <= fVar47) {
    fVar69 = fVar80;
  }
  uVar57 = *(undefined1 *)(lVar21 + 0x41);
  if ((*piVar59 == 1) && (lVar53 = *(longlong *)(piVar59 + 2), *(longlong *)(lVar18 + 0x48) != 0)) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar61 = *(longlong *)(lVar18 + 0x30);
    bVar67 = (byte)(extraout_RAX_00 >> 0x39);
    auVar70 = ZEXT216(CONCAT11(bVar67,bVar67));
    auVar70 = pshuflw(auVar70,auVar70,0);
    lVar56 = 0;
    uVar60 = extraout_RAX_00;
    while( true ) {
      uVar60 = *(ulonglong *)(lVar18 + 0x38) & uVar60;
      pcVar1 = (char *)(lVar61 + uVar60);
      cVar68 = auVar70[0];
      auVar78[0] = -(*pcVar1 == cVar68);
      cVar71 = auVar70[1];
      auVar78[1] = -(pcVar1[1] == cVar71);
      cVar72 = auVar70[2];
      auVar78[2] = -(pcVar1[2] == cVar72);
      cVar73 = auVar70[3];
      auVar78[3] = -(pcVar1[3] == cVar73);
      auVar78[4] = -(pcVar1[4] == cVar68);
      auVar78[5] = -(pcVar1[5] == cVar71);
      auVar78[6] = -(pcVar1[6] == cVar72);
      auVar78[7] = -(pcVar1[7] == cVar73);
      auVar78[8] = -(pcVar1[8] == cVar68);
      auVar78[9] = -(pcVar1[9] == cVar71);
      auVar78[10] = -(pcVar1[10] == cVar72);
      auVar78[0xb] = -(pcVar1[0xb] == cVar73);
      auVar78[0xc] = -(pcVar1[0xc] == cVar68);
      auVar78[0xd] = -(pcVar1[0xd] == cVar71);
      auVar78[0xe] = -(pcVar1[0xe] == cVar72);
      auVar78[0xf] = -(pcVar1[0xf] == cVar73);
      uVar63 = (ushort)(SUB161(auVar78 >> 7,0) & 1) | (ushort)(SUB161(auVar78 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar78 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar78 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar78 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar78 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar78 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar78 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar78 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar78 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar78 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar78 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar78 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar78 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar78 >> 0x77,0) & 1) << 0xe | (ushort)(auVar78[0xf] >> 7) << 0xf;
      uVar62 = (uint)uVar63;
      while (uVar63 != 0) {
        uVar42 = 0;
        for (uVar43 = uVar62; (uVar43 & 1) == 0; uVar43 = uVar43 >> 1 | 0x80000000) {
          uVar42 = uVar42 + 1;
        }
        uVar65 = uVar42 + uVar60 & *(ulonglong *)(lVar18 + 0x38);
        if (lVar53 == *(longlong *)(lVar61 + -0x10 + uVar65 * -0x10)) {
          uVar60 = *(ulonglong *)(lVar61 + -8 + uVar65 * -0x10);
          if (((uVar60 >> 0x20 != 0) &&
              (uVar65 = uVar60 & 0xffffffff, uVar65 < *(ulonglong *)(lVar38 + 0x10))) &&
             (*(int *)(*(longlong *)(lVar38 + 8) + uVar65 * 0x14) == (int)(uVar60 >> 0x20))) {
            lVar53 = *(longlong *)(lVar38 + 8) + uVar65 * 0x14;
            uVar60 = (ulonglong)*(uint *)(lVar53 + 4);
            if (((uVar60 != 0xffffffff) && (uVar60 < *(ulonglong *)(lVar37 + 0x38))) &&
               ((*(ulonglong *)
                  (*(longlong *)(lVar37 + 0x28) + (ulonglong)(*(uint *)(lVar53 + 4) >> 6) * 8) >>
                 (uVar60 & 0x3f) & 1) != 0)) {
              lVar61 = *(longlong *)
                        (*(longlong *)
                          (*(longlong *)(lVar38 + 0x1a8) + 0x18 +
                          (ulonglong)*(uint *)(lVar53 + 0xc) * 0x48) + 0x10 +
                        ~*(ulonglong *)
                          (*(longlong *)
                            (*(longlong *)(lVar38 + 0x1a8) + 0x38 +
                            (ulonglong)*(uint *)(lVar53 + 0xc) * 0x48) +
                          *(longlong *)(lVar37 + 0x110) * 8) * 0x30);
              lVar53 = (ulonglong)*(uint *)(lVar53 + 0x10) * 0x58;
              uVar57 = *(undefined1 *)(lVar61 + 0x3d + lVar53);
              fVar80 = *(float *)(lVar61 + 0x30 + lVar53);
              if (fVar80 <= __real_3dcccccd) {
                fVar69 = fVar80;
              }
            }
          }
          goto LAB_140a940a0;
        }
        uVar63 = (ushort)(uVar62 - 1) & (ushort)uVar62;
        uVar62 = CONCAT22((short)(uVar62 - 1 >> 0x10),uVar63);
      }
      auVar75[0] = -(*pcVar1 == __xmm_ffffffffffffffffffffffffffffffff);
      auVar75[1] = -(pcVar1[1] == UNK_142925991);
      auVar75[2] = -(pcVar1[2] == UNK_142925992);
      auVar75[3] = -(pcVar1[3] == UNK_142925993);
      auVar75[4] = -(pcVar1[4] == UNK_142925994);
      auVar75[5] = -(pcVar1[5] == UNK_142925995);
      auVar75[6] = -(pcVar1[6] == UNK_142925996);
      auVar75[7] = -(pcVar1[7] == UNK_142925997);
      auVar75[8] = -(pcVar1[8] == UNK_142925998);
      auVar75[9] = -(pcVar1[9] == UNK_142925999);
      auVar75[10] = -(pcVar1[10] == UNK_14292599a);
      auVar75[0xb] = -(pcVar1[0xb] == UNK_14292599b);
      auVar75[0xc] = -(pcVar1[0xc] == UNK_14292599c);
      auVar75[0xd] = -(pcVar1[0xd] == UNK_14292599d);
      auVar75[0xe] = -(pcVar1[0xe] == UNK_14292599e);
      auVar75[0xf] = -(pcVar1[0xf] == UNK_14292599f);
      if ((((((((((((((((SUB161(auVar75 >> 7,0) & 1) != 0 || (SUB161(auVar75 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar75 >> 0x17,0) & 1) != 0) || (SUB161(auVar75 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar75 >> 0x27,0) & 1) != 0) || (SUB161(auVar75 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar75 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar75 >> 0x3f,0) & 1) != 0) || (SUB161(auVar75 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar75 >> 0x4f,0) & 1) != 0) || (SUB161(auVar75 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar75 >> 0x5f,0) & 1) != 0) || (SUB161(auVar75 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar75 >> 0x6f,0) & 1) != 0) || (SUB161(auVar75 >> 0x77,0) & 1) != 0) ||
          auVar75[0xf] < '\0') break;
      uVar60 = uVar60 + lVar56 + 0x10;
      lVar56 = lVar56 + 0x10;
    }
  }
LAB_140a940a0:
  *puVar30 = uVar12;
  lVar53 = *(longlong *)(lVar29 + 0x30);
  uVar40 = *(undefined8 *)(lVar29 + 0x40);
  if (lVar53 == *(longlong *)(lVar29 + 0x20)) {
    alloc::raw_vec::RawVec::
    grow_one<country_core::resources::window::AvailableMonitor,alloc::alloc::Global>();
  }
  fVar74 = ((fVar49 - (fVar77 + fVar77)) * fVar77 * fVar77 * fVar46 +
            (float)(~-(uint)NAN(fVar74) & (uint)fVar82 | -(uint)NAN(fVar74) & (uint)fVar81) + fVar51
           ) / fVar46;
  fVar82 = 0.0;
  if (0.0 <= fVar74) {
    fVar82 = fVar74;
  }
  fVar74 = fVar47;
  if (fVar82 <= fVar47) {
    fVar74 = fVar82;
  }
  fVar82 = (fVar49 - (fVar74 + fVar74)) * fVar74 * fVar74;
  lVar61 = *(longlong *)(lVar29 + 0x28);
  lVar56 = lVar53 * 0x40;
  *(undefined8 *)(lVar61 + lVar56) = uVar40;
  *(longlong *)(lVar61 + 8 + lVar56) = extraout_RAX;
  *(undefined4 *)(lVar61 + 0x10 + lVar56) = uStack_244;
  *(undefined8 *)(lVar61 + 0x14 + lVar56) = uVar39;
  *(undefined4 *)(lVar61 + 0x1c + lVar56) = local_230;
  *(undefined8 *)(lVar61 + 0x20 + lVar56) = 0;
  *(undefined4 *)(lVar61 + 0x28 + lVar56) = 0x3f000000;
  *(float *)(lVar61 + 0x2c + lVar56) = fVar69;
  *(float *)(lVar61 + 0x30 + lVar56) = fVar82;
  *(undefined4 *)(lVar61 + 0x34 + lVar56) = 0x3e4ccccd;
  *(bool *)(lVar61 + 0x38 + lVar56) = fVar5 < fVar6;
  *(undefined1 *)(lVar61 + 0x39 + lVar56) = uVar57;
  *(undefined1 *)(lVar61 + 0x3c + lVar56) = 0;
  *(longlong *)(lVar29 + 0x30) = lVar53 + 1;
  *(longlong *)(lVar29 + 0x40) = *(longlong *)(lVar29 + 0x40) + 1;
  *puVar22 = uVar8;
  if (plVar19[3] == 0) {
LAB_140a946c9:
                    /* WARNING: Subroutine does not return */
    core::option::unwrap_failed();
  }
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  lVar53 = *plVar19;
  bVar67 = (byte)(extraout_RAX_01 >> 0x39);
  auVar70 = ZEXT216(CONCAT11(bVar67,bVar67));
  auVar70 = pshuflw(auVar70,auVar70,0);
  lVar61 = 0;
  uVar60 = extraout_RAX_01;
  do {
    uVar60 = plVar19[1] & uVar60;
    pcVar1 = (char *)(lVar53 + uVar60);
    cVar68 = auVar70[0];
    auVar79[0] = -(*pcVar1 == cVar68);
    cVar71 = auVar70[1];
    auVar79[1] = -(pcVar1[1] == cVar71);
    cVar72 = auVar70[2];
    auVar79[2] = -(pcVar1[2] == cVar72);
    cVar73 = auVar70[3];
    auVar79[3] = -(pcVar1[3] == cVar73);
    auVar79[4] = -(pcVar1[4] == cVar68);
    auVar79[5] = -(pcVar1[5] == cVar71);
    auVar79[6] = -(pcVar1[6] == cVar72);
    auVar79[7] = -(pcVar1[7] == cVar73);
    auVar79[8] = -(pcVar1[8] == cVar68);
    auVar79[9] = -(pcVar1[9] == cVar71);
    auVar79[10] = -(pcVar1[10] == cVar72);
    auVar79[0xb] = -(pcVar1[0xb] == cVar73);
    auVar79[0xc] = -(pcVar1[0xc] == cVar68);
    auVar79[0xd] = -(pcVar1[0xd] == cVar71);
    auVar79[0xe] = -(pcVar1[0xe] == cVar72);
    auVar79[0xf] = -(pcVar1[0xf] == cVar73);
    uVar63 = (ushort)(SUB161(auVar79 >> 7,0) & 1) | (ushort)(SUB161(auVar79 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar79 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar79 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar79 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar79 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar79 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar79 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar79 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar79 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar79 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar79 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar79 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar79 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar79 >> 0x77,0) & 1) << 0xe | (ushort)(auVar79[0xf] >> 7) << 0xf;
    uVar62 = (uint)uVar63;
    while (uVar63 != 0) {
      uVar42 = 0;
      for (uVar43 = uVar62; (uVar43 & 1) == 0; uVar43 = uVar43 >> 1 | 0x80000000) {
        uVar42 = uVar42 + 1;
      }
      lVar56 = (uVar42 + uVar60 & plVar19[1]) * -0x78;
      if (*(longlong *)(lVar53 + -0x78 + lVar56) == extraout_RAX) {
        uVar60 = *(ulonglong *)(lVar53 + -0x60 + lVar56);
        sVar2 = uVar60 * 8;
        if ((uVar60 >> 0x3d != 0) || (0x7ffffffffffffffc < sVar2)) {
LAB_140a9476d:
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error();
        }
        pvVar41 = *(void **)(lVar53 + -0x68 + lVar56);
        if (sVar2 == 0) {
          puVar66 = &DAT_00000004;
        }
        else {
          std::alloc::__default_lib_allocator::__rust_alloc();
          puVar66 = extraout_RAX_02;
          if (extraout_RAX_02 == (undefined1 *)0x0) goto LAB_140a9476d;
        }
        memcpy(puVar66,pvVar41,sVar2);
        uVar60 = *(ulonglong *)(lVar53 + lVar56 + -0x48);
        sVar2 = uVar60 * 4;
        if ((uVar60 >> 0x3e != 0) || (0x7ffffffffffffffc < sVar2)) {
LAB_140a94757:
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error();
        }
        pvVar41 = *(void **)(lVar53 + lVar56 + -0x50);
        if (sVar2 == 0) {
          puVar66 = &DAT_00000004;
          uVar60 = 0;
        }
        else {
          std::alloc::__default_lib_allocator::__rust_alloc();
          puVar66 = extraout_RAX_03;
          if (extraout_RAX_03 == (undefined1 *)0x0) goto LAB_140a94757;
        }
        memcpy(puVar66,pvVar41,sVar2);
        local_230 = (undefined4)uVar60;
        resources::history::wall_history::impl$0::ev_new_wall();
        *puVar22 = uVar8;
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_04 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::alloc::handle_alloc_error();
        }
        *extraout_RAX_04 = 1;
        extraout_RAX_04[1] = 1;
        *(undefined1 *)(extraout_RAX_04 + 2) = 0;
        *(undefined4 *)((longlong)extraout_RAX_04 + 0x14) = uStack_244;
        *(undefined1 *)(extraout_RAX_04 + 3) = 0;
        *(undefined8 *)((longlong)extraout_RAX_04 + 0x24) = 0;
        *(undefined4 *)((longlong)extraout_RAX_04 + 0x2c) = 0x3f000000;
        *(float *)(extraout_RAX_04 + 6) = fVar69;
        *(float *)((longlong)extraout_RAX_04 + 0x34) = fVar82;
        *(undefined4 *)(extraout_RAX_04 + 7) = 0x3e4ccccd;
        *(bool *)((longlong)extraout_RAX_04 + 0x3c) = fVar5 < fVar6;
        *(undefined1 *)((longlong)extraout_RAX_04 + 0x3d) = uVar57;
        extraout_RAX_04[8] = extraout_RAX;
        resources::history::History::add();
        iVar4 = *(int *)(lVar21 + 0x38);
        if (iVar4 != 0) {
          *puVar32 = uVar13;
          lVar53 = *(longlong *)(lVar31 + 0x30);
          uVar39 = *(undefined8 *)(lVar31 + 0x40);
          if (lVar53 == *(longlong *)(lVar31 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                      ();
          }
          lVar61 = *(longlong *)(lVar31 + 0x28);
          *(undefined8 *)(lVar61 + lVar53 * 0x18) = uVar39;
          *(longlong *)(lVar61 + 8 + lVar53 * 0x18) = extraout_RAX;
          *(int *)(lVar61 + 0x10 + lVar53 * 0x18) = iVar4;
          *(undefined1 *)(lVar61 + 0x14 + lVar53 * 0x18) = 0;
          *(longlong *)(lVar31 + 0x30) = lVar53 + 1;
          *(longlong *)(lVar31 + 0x40) = *(longlong *)(lVar31 + 0x40) + 1;
          *puVar22 = uVar8;
          uVar16 = *(undefined4 *)(lVar21 + 0x38);
          std::alloc::__default_lib_allocator::__rust_alloc();
          if (extraout_RAX_05 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
            alloc::alloc::handle_alloc_error();
          }
          *extraout_RAX_05 = 1;
          extraout_RAX_05[1] = 1;
          *(undefined1 *)(extraout_RAX_05 + 3) = 4;
          extraout_RAX_05[4] = extraout_RAX;
          *(undefined4 *)(extraout_RAX_05 + 5) = 0;
          *(undefined4 *)((longlong)extraout_RAX_05 + 0x2c) = uVar16;
          resources::history::History::add();
        }
        *puVar34 = uVar14;
        lVar53 = *(longlong *)(lVar33 + 0x30);
        uVar39 = *(undefined8 *)(lVar33 + 0x40);
        if (lVar53 == *(longlong *)(lVar33 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        lVar61 = *(longlong *)(lVar33 + 0x28);
        *(undefined8 *)(lVar61 + lVar53 * 0x10) = uVar39;
        *(undefined1 *)(lVar61 + 8 + lVar53 * 0x10) = 7;
        *(longlong *)(lVar33 + 0x30) = lVar53 + 1;
        *(longlong *)(lVar33 + 0x40) = *(longlong *)(lVar33 + 0x40) + 1;
        *puVar36 = uVar15;
        lVar53 = *(longlong *)(lVar35 + 0x30);
        uVar39 = *(undefined8 *)(lVar35 + 0x40);
        if (lVar53 == *(longlong *)(lVar35 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        lVar61 = *(longlong *)(lVar35 + 0x28);
        *(undefined8 *)(lVar61 + lVar53 * 0x10) = uVar39;
        *(longlong *)(lVar61 + 8 + lVar53 * 0x10) = extraout_RAX;
        *(longlong *)(lVar35 + 0x30) = lVar53 + 1;
        *(longlong *)(lVar35 + 0x40) = *(longlong *)(lVar35 + 0x40) + 1;
        goto LAB_140a93ad4;
      }
      uVar63 = (ushort)(uVar62 - 1) & (ushort)uVar62;
      uVar62 = CONCAT22((short)(uVar62 - 1 >> 0x10),uVar63);
    }
    auVar76[0] = -(*pcVar1 == __xmm_ffffffffffffffffffffffffffffffff);
    auVar76[1] = -(pcVar1[1] == UNK_142925991);
    auVar76[2] = -(pcVar1[2] == UNK_142925992);
    auVar76[3] = -(pcVar1[3] == UNK_142925993);
    auVar76[4] = -(pcVar1[4] == UNK_142925994);
    auVar76[5] = -(pcVar1[5] == UNK_142925995);
    auVar76[6] = -(pcVar1[6] == UNK_142925996);
    auVar76[7] = -(pcVar1[7] == UNK_142925997);
    auVar76[8] = -(pcVar1[8] == UNK_142925998);
    auVar76[9] = -(pcVar1[9] == UNK_142925999);
    auVar76[10] = -(pcVar1[10] == UNK_14292599a);
    auVar76[0xb] = -(pcVar1[0xb] == UNK_14292599b);
    auVar76[0xc] = -(pcVar1[0xc] == UNK_14292599c);
    auVar76[0xd] = -(pcVar1[0xd] == UNK_14292599d);
    auVar76[0xe] = -(pcVar1[0xe] == UNK_14292599e);
    auVar76[0xf] = -(pcVar1[0xf] == UNK_14292599f);
    if ((((((((((((((((SUB161(auVar76 >> 7,0) & 1) != 0 || (SUB161(auVar76 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar76 >> 0x17,0) & 1) != 0) || (SUB161(auVar76 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar76 >> 0x27,0) & 1) != 0) || (SUB161(auVar76 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar76 >> 0x37,0) & 1) != 0) || (SUB161(auVar76 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar76 >> 0x47,0) & 1) != 0) || (SUB161(auVar76 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar76 >> 0x57,0) & 1) != 0) || (SUB161(auVar76 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar76 >> 0x67,0) & 1) != 0) || (SUB161(auVar76 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar76 >> 0x77,0) & 1) != 0) || auVar76[0xf] < '\0') goto LAB_140a946c9;
    uVar60 = uVar60 + lVar61 + 0x10;
    lVar61 = lVar61 + 0x10;
  } while( true );
}

