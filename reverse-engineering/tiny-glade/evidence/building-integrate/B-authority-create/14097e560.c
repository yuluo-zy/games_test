
void __cdecl country_core::systems::wall::shapes::circle::create_circle::create_circle(void)

{
  char *pcVar1;
  size_t sVar2;
  undefined4 uVar3;
  float fVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  undefined4 uVar14;
  int iVar15;
  ulonglong *puVar16;
  longlong lVar17;
  longlong *plVar18;
  undefined4 *puVar19;
  longlong lVar20;
  undefined4 *puVar21;
  longlong lVar22;
  undefined4 *puVar23;
  longlong lVar24;
  undefined4 *puVar25;
  longlong lVar26;
  undefined4 *puVar27;
  longlong lVar28;
  undefined4 *puVar29;
  longlong lVar30;
  undefined4 *puVar31;
  longlong lVar32;
  undefined4 *puVar33;
  longlong lVar34;
  undefined4 *puVar35;
  longlong lVar36;
  longlong lVar37;
  undefined8 uVar38;
  longlong lVar39;
  void *pvVar40;
  uint uVar41;
  uint uVar42;
  longlong lVar43;
  float fVar44;
  float fVar45;
  float fVar46;
  float fVar47;
  undefined8 uVar48;
  longlong *plVar49;
  longlong extraout_RAX;
  ulonglong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  undefined1 *extraout_RAX_02;
  undefined1 *extraout_RAX_03;
  undefined8 *extraout_RAX_04;
  undefined8 *extraout_RAX_05;
  undefined8 *in_RCX;
  longlong lVar50;
  longlong lVar51;
  longlong *in_RDX;
  undefined1 *puVar52;
  ulonglong uVar53;
  longlong *in_R8;
  ulonglong uVar54;
  longlong lVar55;
  uint uVar56;
  longlong in_R9;
  longlong lVar57;
  ushort uVar58;
  longlong *plVar59;
  longlong *plVar60;
  ulonglong uVar61;
  undefined1 uVar62;
  ulonglong uVar63;
  byte bVar64;
  char cVar65;
  char cVar68;
  char cVar69;
  float extraout_XMM0_Da;
  float fVar66;
  char cVar70;
  undefined1 auVar67 [16];
  undefined1 auVar71 [16];
  undefined1 auVar72 [16];
  undefined1 auVar73 [16];
  undefined1 auVar74 [16];
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  longlong *in_stack_00000038;
  longlong *in_stack_00000040;
  longlong *in_stack_00000048;
  longlong *in_stack_00000060;
  longlong *in_stack_00000068;
  longlong *in_stack_00000070;
  longlong *in_stack_00000078;
  longlong *local_1f8;
  longlong *local_1f0;
  undefined4 uStack_1c4;
  undefined4 uStack_1c0;
  ulonglong local_1b8;
  ulonglong uStack_1b0;
  
  puVar16 = (ulonglong *)*in_RCX;
  lVar17 = in_RCX[1];
  uVar54 = *puVar16;
  uVar63 = 0;
  uVar61 = uVar54 - *(ulonglong *)(lVar17 + 0x18);
  if (uVar54 < *(ulonglong *)(lVar17 + 0x18)) {
    uVar61 = uVar63;
  }
  uVar53 = uVar54 - *(ulonglong *)(lVar17 + 0x38);
  if (uVar54 < *(ulonglong *)(lVar17 + 0x38)) {
    uVar53 = uVar63;
  }
  local_1f0 = (longlong *)(uVar61 * 0x30 + *(longlong *)(lVar17 + 8));
  uVar54 = *(ulonglong *)(lVar17 + 0x10) - uVar61;
  if (*(ulonglong *)(lVar17 + 0x10) < uVar61) {
    local_1f0 = (longlong *)&DAT_00000008;
    uVar54 = uVar63;
  }
  local_1f8 = (longlong *)(uVar53 * 0x30 + *(longlong *)(lVar17 + 0x28));
  uVar61 = *(ulonglong *)(lVar17 + 0x30) - uVar53;
  if (*(ulonglong *)(lVar17 + 0x30) < uVar53) {
    local_1f8 = (longlong *)&DAT_00000008;
    uVar61 = uVar63;
  }
  *puVar16 = (*(longlong *)(lVar17 + 0x40) - uVar61) - uVar54;
  uVar48 = __xmm_00000000000000003e4ccccd3ecccccd;
  fVar47 = __real_41400000;
  fVar46 = __real_40800000;
  fVar45 = __real_41200000;
  fVar44 = __real_3f800000;
  plVar59 = local_1f0 + uVar54 * 6;
  plVar60 = local_1f8 + uVar61 * 6;
  uVar5 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  plVar18 = (longlong *)*in_RDX;
  puVar19 = (undefined4 *)in_RDX[2];
  lVar17 = *in_stack_00000068;
  lVar20 = *in_stack_00000060;
  uVar6 = *(undefined4 *)(in_R9 + 0x1c);
  puVar21 = *(undefined4 **)(in_R9 + 0x10);
  uVar7 = *(undefined4 *)((longlong)in_stack_00000078 + 0x1c);
  lVar22 = *in_stack_00000078;
  puVar23 = (undefined4 *)in_stack_00000078[2];
  uVar8 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
  lVar24 = *in_stack_00000038;
  puVar25 = (undefined4 *)in_stack_00000038[2];
  uVar9 = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
  lVar26 = *in_stack_00000040;
  puVar27 = (undefined4 *)in_stack_00000040[2];
  uVar10 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  lVar28 = *in_R8;
  puVar29 = (undefined4 *)in_R8[2];
  uVar11 = *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
  lVar30 = *in_stack_00000048;
  puVar31 = (undefined4 *)in_stack_00000048[2];
  uVar12 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  lVar32 = *in_stack_00000028;
  puVar33 = (undefined4 *)in_stack_00000028[2];
  uVar13 = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
  lVar34 = *in_stack_00000030;
  puVar35 = (undefined4 *)in_stack_00000030[2];
LAB_14097e8ae:
  if ((local_1f0 == (longlong *)0x0) || (local_1f0 == plVar59)) {
    if ((local_1f8 == (longlong *)0x0) || (local_1f8 == plVar60)) {
      return;
    }
    local_1f0 = (longlong *)0x0;
    plVar49 = local_1f8;
    local_1f8 = local_1f8 + 6;
  }
  else {
    plVar49 = local_1f0;
    local_1f0 = local_1f0 + 6;
  }
  *puVar16 = *puVar16 + 1;
  lVar36 = *plVar49;
  lVar55 = plVar49[1];
  uVar3 = *(undefined4 *)((longlong)plVar49 + 0x24);
  lVar51 = plVar49[2];
  uVar14 = *(undefined4 *)((longlong)plVar49 + 0x1c);
  lVar57 = plVar49[2];
  lVar43 = plVar49[3];
  *puVar23 = uVar7;
  lVar37 = *(longlong *)(lVar22 + 0x30);
  uVar38 = *(undefined8 *)(lVar22 + 0x40);
  if (lVar37 == *(longlong *)(lVar22 + 0x20)) {
    alloc::raw_vec::RawVec::
    grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>();
  }
  lVar39 = *(longlong *)(lVar22 + 0x28);
  lVar50 = lVar37 * 0x30;
  *(undefined4 *)(lVar39 + lVar50) = 0;
  plVar49 = (longlong *)(lVar39 + 4 + lVar50);
  *plVar49 = lVar57;
  plVar49[1] = lVar43;
  *(undefined4 *)(lVar39 + 0x1c + lVar50) = uVar3;
  *(undefined4 *)(lVar39 + 0x20 + lVar50) = 0x40800000;
  *(undefined1 *)(lVar39 + 0x24 + lVar50) = 1;
  *(undefined8 *)(lVar39 + 0x28 + lVar50) = uVar38;
  *(longlong *)(lVar22 + 0x30) = lVar37 + 1;
  *(longlong *)(lVar22 + 0x40) = *(longlong *)(lVar22 + 0x40) + 1;
  *puVar19 = uVar5;
  ::utils::geometry::circle::Circle2d::as_points2();
  alloc::vec::spec_from_iter::impl$0::
  from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl_0::as_points2_ptcount::closure_env_0>_>
            ();
  ::utils::curve::Curve::
  try_new_from_points<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
  if (SBORROW8(0,lVar57)) {
                    /* WARNING: Subroutine does not return */
    core::result::unwrap_failed();
  }
  resources::walls::inner_walls::InnerWalls::new_wall_w_params();
  *puVar19 = uVar5;
  resources::walls::inner_walls::InnerWalls::
  _try_update_state<country_core::resources::walls::inner_walls::impl_1::update_elevation::closure_env_0>
            ();
  *puVar19 = uVar5;
  uStack_1c4 = (undefined4)lVar51;
  uStack_1c0 = (undefined4)((ulonglong)lVar51 >> 0x20);
  local_1b8 = CONCAT44(local_1b8._4_4_,uVar14);
  resources::walls::inner_walls::InnerWalls::
  _try_update_state<country_core::resources::walls::inner_walls::impl_1::update_wall_state_enclosure::closure_env_0>
            ();
  rectangle::create_rectangle::get_building_and_roof_colors();
  *puVar19 = uVar5;
  resources::walls::inner_walls::InnerWalls::
  _try_update_state<country_core::resources::walls::inner_walls::impl_1::update_style::closure_env_0>
            ();
  *puVar25 = uVar8;
  lVar51 = *(longlong *)(lVar24 + 0x30);
  uVar38 = *(undefined8 *)(lVar24 + 0x40);
  if (lVar51 == *(longlong *)(lVar24 + 0x20)) {
    alloc::raw_vec::RawVec::
    grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
              ();
  }
  lVar37 = *(longlong *)(lVar24 + 0x28);
  *(undefined8 *)(lVar37 + lVar51 * 0x18) = uVar38;
  *(longlong *)(lVar37 + 8 + lVar51 * 0x18) = extraout_RAX;
  *(undefined4 *)(lVar37 + 0x10 + lVar51 * 0x18) = 0;
  *(undefined1 *)(lVar37 + 0x14 + lVar51 * 0x18) = 0;
  *(longlong *)(lVar24 + 0x30) = lVar51 + 1;
  *(longlong *)(lVar24 + 0x40) = *(longlong *)(lVar24 + 0x40) + 1;
  *puVar27 = uVar9;
  lVar51 = *(longlong *)(lVar26 + 0x30);
  uVar38 = *(undefined8 *)(lVar26 + 0x40);
  if (lVar51 == *(longlong *)(lVar26 + 0x20)) {
    alloc::raw_vec::RawVec::
    grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
              ();
  }
  lVar37 = *(longlong *)(lVar26 + 0x28);
  *(undefined8 *)(lVar37 + lVar51 * 0x18) = uVar38;
  *(longlong *)(lVar37 + 8 + lVar51 * 0x18) = extraout_RAX;
  *(undefined4 *)(lVar37 + 0x10 + lVar51 * 0x18) = uStack_1c0;
  *(undefined1 *)(lVar37 + 0x14 + lVar51 * 0x18) = 0;
  *(longlong *)(lVar26 + 0x30) = lVar51 + 1;
  *(longlong *)(lVar26 + 0x40) = *(longlong *)(lVar26 + 0x40) + 1;
  ::utils::easing_functions::EasingFunction::apply();
  fVar66 = (extraout_XMM0_Da * fVar45 + (fVar44 - extraout_XMM0_Da) * fVar46) / fVar47;
  fVar4 = 0.0;
  if (0.0 <= fVar66) {
    fVar4 = fVar66;
  }
  fVar66 = fVar44;
  if (fVar4 <= fVar44) {
    fVar66 = fVar4;
  }
  uVar62 = *(undefined1 *)(lVar20 + 0x41);
  if (lVar36 != 0) {
    lVar36 = *in_stack_00000070;
    lVar51 = in_stack_00000070[1];
    if (*(longlong *)(lVar17 + 0x48) != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      lVar37 = *(longlong *)(lVar17 + 0x30);
      bVar64 = (byte)(extraout_RAX_00 >> 0x39);
      auVar67 = ZEXT216(CONCAT11(bVar64,bVar64));
      auVar67 = pshuflw(auVar67,auVar67,0);
      lVar57 = 0;
      uVar54 = extraout_RAX_00;
      while( true ) {
        uVar54 = *(ulonglong *)(lVar17 + 0x38) & uVar54;
        pcVar1 = (char *)(lVar37 + uVar54);
        cVar65 = auVar67[0];
        auVar73[0] = -(*pcVar1 == cVar65);
        cVar68 = auVar67[1];
        auVar73[1] = -(pcVar1[1] == cVar68);
        cVar69 = auVar67[2];
        auVar73[2] = -(pcVar1[2] == cVar69);
        cVar70 = auVar67[3];
        auVar73[3] = -(pcVar1[3] == cVar70);
        auVar73[4] = -(pcVar1[4] == cVar65);
        auVar73[5] = -(pcVar1[5] == cVar68);
        auVar73[6] = -(pcVar1[6] == cVar69);
        auVar73[7] = -(pcVar1[7] == cVar70);
        auVar73[8] = -(pcVar1[8] == cVar65);
        auVar73[9] = -(pcVar1[9] == cVar68);
        auVar73[10] = -(pcVar1[10] == cVar69);
        auVar73[0xb] = -(pcVar1[0xb] == cVar70);
        auVar73[0xc] = -(pcVar1[0xc] == cVar65);
        auVar73[0xd] = -(pcVar1[0xd] == cVar68);
        auVar73[0xe] = -(pcVar1[0xe] == cVar69);
        auVar73[0xf] = -(pcVar1[0xf] == cVar70);
        uVar58 = (ushort)(SUB161(auVar73 >> 7,0) & 1) | (ushort)(SUB161(auVar73 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar73 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar73 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar73 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar73 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar73 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar73 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar73 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar73 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar73 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar73 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar73 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar73 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar73 >> 0x77,0) & 1) << 0xe | (ushort)(auVar73[0xf] >> 7) << 0xf
        ;
        uVar56 = (uint)uVar58;
        while (uVar58 != 0) {
          uVar41 = 0;
          for (uVar42 = uVar56; (uVar42 & 1) == 0; uVar42 = uVar42 >> 1 | 0x80000000) {
            uVar41 = uVar41 + 1;
          }
          uVar61 = uVar41 + uVar54 & *(ulonglong *)(lVar17 + 0x38);
          if (*(longlong *)(lVar37 + -0x10 + uVar61 * -0x10) == lVar55) {
            uVar54 = *(ulonglong *)(lVar37 + -8 + uVar61 * -0x10);
            if (((uVar54 >> 0x20 != 0) &&
                (uVar61 = uVar54 & 0xffffffff, uVar61 < *(ulonglong *)(lVar51 + 0x10))) &&
               (*(int *)(*(longlong *)(lVar51 + 8) + uVar61 * 0x14) == (int)(uVar54 >> 0x20))) {
              lVar55 = *(longlong *)(lVar51 + 8) + uVar61 * 0x14;
              uVar54 = (ulonglong)*(uint *)(lVar55 + 4);
              if (((uVar54 != 0xffffffff) && (uVar54 < *(ulonglong *)(lVar36 + 0x38))) &&
                 ((*(ulonglong *)
                    (*(longlong *)(lVar36 + 0x28) + (ulonglong)(*(uint *)(lVar55 + 4) >> 6) * 8) >>
                   (uVar54 & 0x3f) & 1) != 0)) {
                lVar36 = *(longlong *)
                          (*(longlong *)
                            (*(longlong *)(lVar51 + 0x1a8) + 0x18 +
                            (ulonglong)*(uint *)(lVar55 + 0xc) * 0x48) + 0x10 +
                          ~*(ulonglong *)
                            (*(longlong *)
                              (*(longlong *)(lVar51 + 0x1a8) + 0x38 +
                              (ulonglong)*(uint *)(lVar55 + 0xc) * 0x48) +
                            *(longlong *)(lVar36 + 0x110) * 8) * 0x30);
                lVar55 = (ulonglong)*(uint *)(lVar55 + 0x10) * 0x58;
                uVar62 = *(undefined1 *)(lVar36 + 0x3d + lVar55);
                fVar4 = *(float *)(lVar36 + 0x30 + lVar55);
                if (fVar4 <= __real_3dcccccd) {
                  fVar66 = fVar4;
                }
              }
            }
            goto LAB_14097ed97;
          }
          uVar58 = (ushort)(uVar56 - 1) & (ushort)uVar56;
          uVar56 = CONCAT22((short)(uVar56 - 1 >> 0x10),uVar58);
        }
        auVar71[0] = -(*pcVar1 == __xmm_ffffffffffffffffffffffffffffffff);
        auVar71[1] = -(pcVar1[1] == UNK_142925991);
        auVar71[2] = -(pcVar1[2] == UNK_142925992);
        auVar71[3] = -(pcVar1[3] == UNK_142925993);
        auVar71[4] = -(pcVar1[4] == UNK_142925994);
        auVar71[5] = -(pcVar1[5] == UNK_142925995);
        auVar71[6] = -(pcVar1[6] == UNK_142925996);
        auVar71[7] = -(pcVar1[7] == UNK_142925997);
        auVar71[8] = -(pcVar1[8] == UNK_142925998);
        auVar71[9] = -(pcVar1[9] == UNK_142925999);
        auVar71[10] = -(pcVar1[10] == UNK_14292599a);
        auVar71[0xb] = -(pcVar1[0xb] == UNK_14292599b);
        auVar71[0xc] = -(pcVar1[0xc] == UNK_14292599c);
        auVar71[0xd] = -(pcVar1[0xd] == UNK_14292599d);
        auVar71[0xe] = -(pcVar1[0xe] == UNK_14292599e);
        auVar71[0xf] = -(pcVar1[0xf] == UNK_14292599f);
        if ((((((((((((((((SUB161(auVar71 >> 7,0) & 1) != 0 || (SUB161(auVar71 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar71 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar71 >> 0x1f,0) & 1) != 0) || (SUB161(auVar71 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar71 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar71 >> 0x37,0) & 1) != 0) || (SUB161(auVar71 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar71 >> 0x47,0) & 1) != 0) || (SUB161(auVar71 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar71 >> 0x57,0) & 1) != 0) || (SUB161(auVar71 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar71 >> 0x67,0) & 1) != 0) || (SUB161(auVar71 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar71 >> 0x77,0) & 1) != 0) || auVar71[0xf] < '\0') break;
        uVar54 = uVar54 + lVar57 + 0x10;
        lVar57 = lVar57 + 0x10;
      }
    }
  }
LAB_14097ed97:
  *puVar29 = uVar10;
  lVar36 = *(longlong *)(lVar28 + 0x30);
  uVar38 = *(undefined8 *)(lVar28 + 0x40);
  if (lVar36 == *(longlong *)(lVar28 + 0x20)) {
    alloc::raw_vec::RawVec::
    grow_one<country_core::resources::window::AvailableMonitor,alloc::alloc::Global>();
  }
  lVar55 = *(longlong *)(lVar28 + 0x28);
  lVar51 = lVar36 * 0x40;
  *(undefined8 *)(lVar55 + lVar51) = uVar38;
  *(longlong *)(lVar55 + 8 + lVar51) = extraout_RAX;
  *(undefined4 *)(lVar55 + 0x10 + lVar51) = uStack_1c4;
  *(ulonglong *)(lVar55 + 0x14 + lVar51) = local_1b8;
  *(undefined4 *)(lVar55 + 0x1c + lVar51) = (undefined4)uStack_1b0;
  *(undefined8 *)(lVar55 + 0x20 + lVar51) = 0;
  *(undefined4 *)(lVar55 + 0x28 + lVar51) = 0x3f000000;
  *(float *)(lVar55 + 0x2c + lVar51) = fVar66;
  *(undefined8 *)(lVar55 + 0x30 + lVar51) = uVar48;
  *(undefined1 *)(lVar55 + 0x38 + lVar51) = 0;
  *(undefined1 *)(lVar55 + 0x39 + lVar51) = uVar62;
  *(undefined1 *)(lVar55 + 0x3c + lVar51) = 0;
  *(longlong *)(lVar28 + 0x30) = lVar36 + 1;
  *(longlong *)(lVar28 + 0x40) = *(longlong *)(lVar28 + 0x40) + 1;
  *puVar21 = uVar6;
  if (plVar18[3] == 0) {
LAB_14097f426:
                    /* WARNING: Subroutine does not return */
    core::option::unwrap_failed();
  }
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  lVar36 = *plVar18;
  bVar64 = (byte)(extraout_RAX_01 >> 0x39);
  auVar67 = ZEXT216(CONCAT11(bVar64,bVar64));
  auVar67 = pshuflw(auVar67,auVar67,0);
  lVar55 = 0;
  uVar54 = extraout_RAX_01;
  do {
    uVar54 = plVar18[1] & uVar54;
    pcVar1 = (char *)(lVar36 + uVar54);
    cVar65 = auVar67[0];
    auVar74[0] = -(*pcVar1 == cVar65);
    cVar68 = auVar67[1];
    auVar74[1] = -(pcVar1[1] == cVar68);
    cVar69 = auVar67[2];
    auVar74[2] = -(pcVar1[2] == cVar69);
    cVar70 = auVar67[3];
    auVar74[3] = -(pcVar1[3] == cVar70);
    auVar74[4] = -(pcVar1[4] == cVar65);
    auVar74[5] = -(pcVar1[5] == cVar68);
    auVar74[6] = -(pcVar1[6] == cVar69);
    auVar74[7] = -(pcVar1[7] == cVar70);
    auVar74[8] = -(pcVar1[8] == cVar65);
    auVar74[9] = -(pcVar1[9] == cVar68);
    auVar74[10] = -(pcVar1[10] == cVar69);
    auVar74[0xb] = -(pcVar1[0xb] == cVar70);
    auVar74[0xc] = -(pcVar1[0xc] == cVar65);
    auVar74[0xd] = -(pcVar1[0xd] == cVar68);
    auVar74[0xe] = -(pcVar1[0xe] == cVar69);
    auVar74[0xf] = -(pcVar1[0xf] == cVar70);
    uVar58 = (ushort)(SUB161(auVar74 >> 7,0) & 1) | (ushort)(SUB161(auVar74 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar74 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar74 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar74 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar74 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar74 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar74 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar74 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar74 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar74 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar74 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar74 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar74 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar74 >> 0x77,0) & 1) << 0xe | (ushort)(auVar74[0xf] >> 7) << 0xf;
    uVar56 = (uint)uVar58;
    while (uVar58 != 0) {
      uVar41 = 0;
      for (uVar42 = uVar56; (uVar42 & 1) == 0; uVar42 = uVar42 >> 1 | 0x80000000) {
        uVar41 = uVar41 + 1;
      }
      lVar51 = (uVar41 + uVar54 & plVar18[1]) * -0x78;
      if (*(longlong *)(lVar36 + -0x78 + lVar51) == extraout_RAX) {
        local_1b8 = *(ulonglong *)(lVar36 + -0x60 + lVar51);
        sVar2 = local_1b8 * 8;
        if ((local_1b8 >> 0x3d != 0) || (0x7ffffffffffffffc < sVar2)) {
LAB_14097f4ff:
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error();
        }
        pvVar40 = *(void **)(lVar36 + -0x68 + lVar51);
        if (sVar2 == 0) {
          puVar52 = &DAT_00000004;
        }
        else {
          std::alloc::__default_lib_allocator::__rust_alloc();
          puVar52 = extraout_RAX_02;
          if (extraout_RAX_02 == (undefined1 *)0x0) goto LAB_14097f4ff;
        }
        memcpy(puVar52,pvVar40,sVar2);
        uStack_1b0 = *(ulonglong *)(lVar36 + lVar51 + -0x48);
        sVar2 = uStack_1b0 * 4;
        if ((uStack_1b0 >> 0x3e != 0) || (0x7ffffffffffffffc < sVar2)) {
LAB_14097f4e8:
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error();
        }
        pvVar40 = *(void **)(lVar36 + lVar51 + -0x50);
        if (sVar2 == 0) {
          puVar52 = &DAT_00000004;
          uStack_1b0 = 0;
        }
        else {
          std::alloc::__default_lib_allocator::__rust_alloc();
          puVar52 = extraout_RAX_03;
          if (extraout_RAX_03 == (undefined1 *)0x0) goto LAB_14097f4e8;
        }
        memcpy(puVar52,pvVar40,sVar2);
        resources::history::wall_history::impl$0::ev_new_wall();
        *puVar21 = uVar6;
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_04 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::alloc::handle_alloc_error();
        }
        *extraout_RAX_04 = 1;
        extraout_RAX_04[1] = 1;
        *(undefined1 *)(extraout_RAX_04 + 2) = 0;
        *(undefined4 *)((longlong)extraout_RAX_04 + 0x14) = uStack_1c4;
        *(undefined1 *)(extraout_RAX_04 + 3) = 0;
        *(undefined8 *)((longlong)extraout_RAX_04 + 0x24) = 0;
        *(undefined4 *)((longlong)extraout_RAX_04 + 0x2c) = 0x3f000000;
        *(float *)(extraout_RAX_04 + 6) = fVar66;
        *(undefined8 *)((longlong)extraout_RAX_04 + 0x34) = uVar48;
        *(undefined1 *)((longlong)extraout_RAX_04 + 0x3c) = 0;
        *(undefined1 *)((longlong)extraout_RAX_04 + 0x3d) = uVar62;
        extraout_RAX_04[8] = extraout_RAX;
        resources::history::History::add();
        iVar15 = *(int *)(lVar20 + 0x38);
        if (iVar15 != 0) {
          *puVar31 = uVar11;
          lVar36 = *(longlong *)(lVar30 + 0x30);
          uVar38 = *(undefined8 *)(lVar30 + 0x40);
          if (lVar36 == *(longlong *)(lVar30 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                      ();
          }
          lVar55 = *(longlong *)(lVar30 + 0x28);
          *(undefined8 *)(lVar55 + lVar36 * 0x18) = uVar38;
          *(longlong *)(lVar55 + 8 + lVar36 * 0x18) = extraout_RAX;
          *(int *)(lVar55 + 0x10 + lVar36 * 0x18) = iVar15;
          *(undefined1 *)(lVar55 + 0x14 + lVar36 * 0x18) = 0;
          *(longlong *)(lVar30 + 0x30) = lVar36 + 1;
          *(longlong *)(lVar30 + 0x40) = *(longlong *)(lVar30 + 0x40) + 1;
          *puVar21 = uVar6;
          uVar3 = *(undefined4 *)(lVar20 + 0x38);
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
          *(undefined4 *)((longlong)extraout_RAX_05 + 0x2c) = uVar3;
          resources::history::History::add();
        }
        *puVar33 = uVar12;
        lVar36 = *(longlong *)(lVar32 + 0x30);
        uVar38 = *(undefined8 *)(lVar32 + 0x40);
        if (lVar36 == *(longlong *)(lVar32 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        lVar55 = *(longlong *)(lVar32 + 0x28);
        *(undefined8 *)(lVar55 + lVar36 * 0x10) = uVar38;
        *(undefined1 *)(lVar55 + 8 + lVar36 * 0x10) = 7;
        *(longlong *)(lVar32 + 0x30) = lVar36 + 1;
        *(longlong *)(lVar32 + 0x40) = *(longlong *)(lVar32 + 0x40) + 1;
        *puVar35 = uVar13;
        lVar36 = *(longlong *)(lVar34 + 0x30);
        uVar38 = *(undefined8 *)(lVar34 + 0x40);
        if (lVar36 == *(longlong *)(lVar34 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        lVar55 = *(longlong *)(lVar34 + 0x28);
        *(undefined8 *)(lVar55 + lVar36 * 0x10) = uVar38;
        *(longlong *)(lVar55 + 8 + lVar36 * 0x10) = extraout_RAX;
        *(longlong *)(lVar34 + 0x30) = lVar36 + 1;
        *(longlong *)(lVar34 + 0x40) = *(longlong *)(lVar34 + 0x40) + 1;
        goto LAB_14097e8ae;
      }
      uVar58 = (ushort)(uVar56 - 1) & (ushort)uVar56;
      uVar56 = CONCAT22((short)(uVar56 - 1 >> 0x10),uVar58);
    }
    auVar72[0] = -(*pcVar1 == __xmm_ffffffffffffffffffffffffffffffff);
    auVar72[1] = -(pcVar1[1] == UNK_142925991);
    auVar72[2] = -(pcVar1[2] == UNK_142925992);
    auVar72[3] = -(pcVar1[3] == UNK_142925993);
    auVar72[4] = -(pcVar1[4] == UNK_142925994);
    auVar72[5] = -(pcVar1[5] == UNK_142925995);
    auVar72[6] = -(pcVar1[6] == UNK_142925996);
    auVar72[7] = -(pcVar1[7] == UNK_142925997);
    auVar72[8] = -(pcVar1[8] == UNK_142925998);
    auVar72[9] = -(pcVar1[9] == UNK_142925999);
    auVar72[10] = -(pcVar1[10] == UNK_14292599a);
    auVar72[0xb] = -(pcVar1[0xb] == UNK_14292599b);
    auVar72[0xc] = -(pcVar1[0xc] == UNK_14292599c);
    auVar72[0xd] = -(pcVar1[0xd] == UNK_14292599d);
    auVar72[0xe] = -(pcVar1[0xe] == UNK_14292599e);
    auVar72[0xf] = -(pcVar1[0xf] == UNK_14292599f);
    if ((((((((((((((((SUB161(auVar72 >> 7,0) & 1) != 0 || (SUB161(auVar72 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar72 >> 0x17,0) & 1) != 0) || (SUB161(auVar72 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar72 >> 0x27,0) & 1) != 0) || (SUB161(auVar72 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar72 >> 0x37,0) & 1) != 0) || (SUB161(auVar72 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar72 >> 0x47,0) & 1) != 0) || (SUB161(auVar72 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar72 >> 0x57,0) & 1) != 0) || (SUB161(auVar72 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar72 >> 0x67,0) & 1) != 0) || (SUB161(auVar72 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar72 >> 0x77,0) & 1) != 0) || auVar72[0xf] < '\0') goto LAB_14097f426;
    uVar54 = uVar54 + lVar55 + 0x10;
    lVar55 = lVar55 + 0x10;
  } while( true );
}

