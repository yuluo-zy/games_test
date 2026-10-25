
// WARNING: Instruction at (ram,0x000141c80077) overlaps instruction at (ram,0x000141c80076)
// 

void __cdecl system_color::replay_color::replay_color(void)

{
  uint *puVar1;
  float fVar2;
  float fVar3;
  undefined1 auVar4 [16];
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
  undefined4 uVar15;
  undefined4 uVar16;
  undefined4 uVar17;
  int iVar18;
  uint uVar19;
  char *pcVar20;
  undefined4 *puVar21;
  undefined *puVar22;
  undefined4 *puVar23;
  undefined *puVar24;
  undefined4 *puVar25;
  undefined *puVar26;
  undefined4 *puVar27;
  undefined *puVar28;
  undefined4 *puVar29;
  undefined *puVar30;
  undefined4 *puVar31;
  undefined *puVar32;
  undefined4 *puVar33;
  undefined *puVar34;
  undefined4 *puVar35;
  undefined *puVar36;
  undefined4 *puVar37;
  longlong lVar38;
  undefined4 *puVar39;
  undefined *puVar40;
  undefined4 *puVar41;
  undefined *puVar42;
  undefined4 *puVar43;
  undefined *puVar44;
  undefined4 *puVar45;
  undefined8 uVar46;
  char cVar47;
  longlong extraout_RAX;
  undefined7 uVar50;
  longlong lVar48;
  char *_Dst;
  longlong *plVar49;
  byte bVar51;
  longlong *in_RCX;
  undefined *puVar52;
  longlong lVar53;
  ulonglong uVar54;
  undefined8 *in_RDX;
  undefined *_Src;
  ulonglong uVar55;
  longlong lVar56;
  longlong lVar57;
  undefined8 *in_R8;
  undefined1 (*pauVar58) [16];
  undefined8 *puVar59;
  longlong lVar60;
  longlong *in_R9;
  ulonglong uVar61;
  undefined *puVar62;
  char *pcVar63;
  undefined *puVar64;
  char *pcVar65;
  char *pcVar66;
  undefined1 auVar67 [16];
  undefined1 auVar68 [16];
  float fVar69;
  undefined1 auVar70 [16];
  undefined1 auVar71 [16];
  undefined4 uVar72;
  undefined8 *in_stack_00000028;
  undefined8 *in_stack_00000030;
  undefined8 *in_stack_00000038;
  undefined8 *in_stack_00000040;
  undefined8 *in_stack_00000048;
  undefined8 *in_stack_00000050;
  undefined8 *in_stack_00000058;
  undefined8 *in_stack_00000060;
  undefined8 *in_stack_00000068;
  undefined8 *in_stack_00000070;
  longlong *in_stack_00000078;
  ulonglong local_158;
  longlong *plStack_150;
  longlong *local_148;
  ulonglong local_b8;
  char *local_b0;
  undefined *local_a8;
  undefined8 uStack_98;
  
  lVar56 = *in_RCX;
  country_core::input::input_stream::InputStream::sync_point_count();
  fVar2 = *(float *)(lVar56 + 0x28);
  *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
  country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket();
  if (extraout_RAX == 0) {
    return;
  }
  local_b8 = 0;
  pcVar20 = *(char **)(extraout_RAX + 8);
  lVar56 = *(longlong *)(extraout_RAX + 0x10);
  lVar60 = *in_stack_00000078;
  uVar5 = *(undefined4 *)((longlong)in_stack_00000070 + 0x1c);
  local_a8 = (undefined *)*in_stack_00000070;
  puVar21 = (undefined4 *)in_stack_00000070[2];
  uVar6 = *(undefined4 *)((longlong)in_stack_00000068 + 0x1c);
  puVar22 = (undefined *)*in_stack_00000068;
  puVar23 = (undefined4 *)in_stack_00000068[2];
  uVar7 = *(undefined4 *)((longlong)in_stack_00000060 + 0x1c);
  puVar24 = (undefined *)*in_stack_00000060;
  puVar25 = (undefined4 *)in_stack_00000060[2];
  uVar8 = *(undefined4 *)((longlong)in_stack_00000050 + 0x1c);
  puVar26 = (undefined *)*in_stack_00000050;
  puVar27 = (undefined4 *)in_stack_00000050[2];
  uVar9 = *(undefined4 *)((longlong)in_stack_00000058 + 0x1c);
  puVar28 = (undefined *)*in_stack_00000058;
  puVar29 = (undefined4 *)in_stack_00000058[2];
  uVar10 = *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
  puVar30 = (undefined *)*in_stack_00000048;
  puVar31 = (undefined4 *)in_stack_00000048[2];
  uVar11 = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
  puVar32 = (undefined *)*in_stack_00000040;
  puVar33 = (undefined4 *)in_stack_00000040[2];
  uVar12 = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
  puVar34 = (undefined *)*in_stack_00000030;
  puVar35 = (undefined4 *)in_stack_00000030[2];
  uVar13 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  puVar36 = (undefined *)*in_stack_00000028;
  puVar37 = (undefined4 *)in_stack_00000028[2];
  uVar14 = *(undefined4 *)((longlong)in_R9 + 0x1c);
  lVar38 = *in_R9;
  puVar39 = (undefined4 *)in_R9[2];
  uVar15 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  puVar40 = (undefined *)*in_R8;
  puVar41 = (undefined4 *)in_R8[2];
  uVar16 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
  puVar42 = (undefined *)*in_stack_00000038;
  puVar43 = (undefined4 *)in_stack_00000038[2];
  uVar17 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  puVar44 = (undefined *)*in_RDX;
  puVar45 = (undefined4 *)in_RDX[2];
  puVar62 = puVar32;
  local_b0 = pcVar20;
  puVar64 = puVar40;
  pcVar66 = (char *)0x0;
  while (pcVar65 = pcVar66, pcVar63 = local_b0, pcVar63 != pcVar20 + lVar56 * 0x30) {
    local_b0 = pcVar63 + 0x30;
    pcVar66 = pcVar65 + 1;
    if (*pcVar63 == '\x06') {
      _Dst = *(char **)(pcVar63 + 0x10);
      bVar51 = _Dst[0x18] - 3;
      if (0xd < bVar51) {
        bVar51 = 3;
      }
      _Src = &DAT_142f20388;
      iVar18 = *(int *)(&DAT_142f20388 + (ulonglong)bVar51 * 4);
      puVar52 = &DAT_142f20388 + iVar18;
      uVar50 = (undefined7)((ulonglong)_Dst >> 8);
      cVar47 = (char)_Dst;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
      switch(bVar51) {
      case 0:
        if ((*(float *)(pcVar63 + 0x18) != 0.0) || (NAN(*(float *)(pcVar63 + 0x18)))) break;
        puVar62 = *(undefined **)(_Dst + 0x20);
        puVar52 = puVar44;
        if (pcVar63[0x1c] == '\0') {
          uVar72 = *(undefined4 *)(_Dst + 0x2c);
          *puVar45 = uVar17;
          lVar57 = *(longlong *)(puVar44 + 0x30);
          puVar64 = *(undefined **)(puVar44 + 0x40);
          if (lVar57 == *(longlong *)(puVar44 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                      ();
          }
        }
        else {
          uVar72 = *(undefined4 *)(_Dst + 0x28);
          *puVar45 = uVar17;
          lVar57 = *(longlong *)(puVar44 + 0x30);
          puVar64 = *(undefined **)(puVar44 + 0x40);
          if (lVar57 == *(longlong *)(puVar44 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                      ();
          }
        }
code_r0x000141c7fe0d:
        lVar48 = *(longlong *)(puVar52 + 0x28);
        *(undefined **)(lVar48 + lVar57 * 0x18) = puVar64;
        *(undefined **)(lVar48 + 8 + lVar57 * 0x18) = puVar62;
        *(undefined4 *)(lVar48 + 0x10 + lVar57 * 0x18) = uVar72;
        *(undefined1 *)(lVar48 + 0x14 + lVar57 * 0x18) = 1;
code_r0x000141c7ff10:
        *(longlong *)(puVar52 + 0x30) = lVar57 + 1;
        *(longlong *)(puVar52 + 0x40) = *(longlong *)(puVar52 + 0x40) + 1;
        if ((pcVar63[8] & 1U) != 0) goto code_r0x000141c7fc5a;
        goto LAB_141c7ff2c;
      case 1:
        if ((*(float *)(pcVar63 + 0x18) == 0.0) && (!NAN(*(float *)(pcVar63 + 0x18)))) {
          puVar62 = *(undefined **)(_Dst + 0x20);
          puVar52 = puVar42;
          if (pcVar63[0x1c] == '\0') {
            uVar72 = *(undefined4 *)(_Dst + 0x2c);
            *puVar43 = uVar16;
            lVar57 = *(longlong *)(puVar42 + 0x30);
            puVar64 = *(undefined **)(puVar42 + 0x40);
            if (lVar57 == *(longlong *)(puVar42 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                        ();
            }
          }
          else {
            uVar72 = *(undefined4 *)(_Dst + 0x28);
            *puVar43 = uVar16;
            lVar57 = *(longlong *)(puVar42 + 0x30);
            puVar64 = *(undefined **)(puVar42 + 0x40);
            if (lVar57 == *(longlong *)(puVar42 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                        ();
            }
          }
          goto code_r0x000141c7fe0d;
        }
        break;
      case 2:
        if ((*(float *)(pcVar63 + 0x18) == 0.0) && (!NAN(*(float *)(pcVar63 + 0x18)))) {
          puVar62 = *(undefined **)(_Dst + 0x20);
          puVar52 = puVar40;
          if (pcVar63[0x1c] == '\0') {
            uVar72 = *(undefined4 *)(_Dst + 0x2c);
            *puVar41 = uVar15;
            lVar57 = *(longlong *)(puVar40 + 0x30);
            puVar64 = *(undefined **)(puVar40 + 0x40);
            if (lVar57 == *(longlong *)(puVar40 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                        ();
            }
          }
          else {
            uVar72 = *(undefined4 *)(_Dst + 0x28);
            *puVar41 = uVar15;
            lVar57 = *(longlong *)(puVar40 + 0x30);
            puVar64 = *(undefined **)(puVar40 + 0x40);
            if (lVar57 == *(longlong *)(puVar40 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                        ();
            }
          }
          goto code_r0x000141c7fe0d;
        }
        break;
      case 3:
        if ((*(float *)(pcVar63 + 0x18) == 0.0) && (!NAN(*(float *)(pcVar63 + 0x18)))) {
          if (pcVar63[0x1c] == '\0') {
            puVar64 = *(undefined **)(_Dst + 0x10);
            local_158 = *(ulonglong *)(_Dst + 0x24);
            uVar72 = *(undefined4 *)(_Dst + 0x2c);
            *puVar39 = uVar14;
            lVar57 = *(longlong *)(lVar38 + 0x30);
            puVar62 = *(undefined **)(lVar38 + 0x40);
            if (lVar57 == *(longlong *)(lVar38 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                        ();
            }
            lVar48 = *(longlong *)(lVar38 + 0x28);
            lVar53 = lVar57 * 0x20;
            *(undefined **)(lVar48 + lVar53) = puVar62;
            *(undefined **)(lVar48 + 8 + lVar53) = puVar64;
            *(ulonglong *)(lVar48 + 0x10 + lVar53) = local_158;
            *(undefined4 *)(lVar48 + 0x18 + lVar53) = uVar72;
            *(undefined1 *)(lVar48 + 0x1c + lVar53) = 1;
            *(longlong *)(lVar38 + 0x30) = lVar57 + 1;
          }
          else {
            uVar46 = *(undefined8 *)(_Dst + 0x10);
            uVar72 = *(undefined4 *)(_Dst + 0x20);
            local_158 = *(ulonglong *)(_Dst + 0x18);
            *puVar39 = uVar14;
            lVar57 = *(longlong *)(lVar38 + 0x30);
            puVar62 = *(undefined **)(lVar38 + 0x40);
            if (lVar57 == *(longlong *)(lVar38 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                        ();
            }
            lVar48 = *(longlong *)(lVar38 + 0x28);
            lVar53 = lVar57 * 0x20;
            *(undefined **)(lVar48 + lVar53) = puVar62;
            *(undefined8 *)(lVar48 + 8 + lVar53) = uVar46;
            *(ulonglong *)(lVar48 + 0x10 + lVar53) = local_158;
            *(undefined4 *)(lVar48 + 0x18 + lVar53) = uVar72;
            *(undefined1 *)(lVar48 + 0x1c + lVar53) = 1;
            puVar64 = (undefined *)(lVar57 + 1);
            *(undefined **)(lVar38 + 0x30) = puVar64;
          }
          plStack_150 = (longlong *)CONCAT44(plStack_150._4_4_,uVar72);
          *(longlong *)(lVar38 + 0x40) = *(longlong *)(lVar38 + 0x40) + 1;
        }
        break;
      case 4:
        if ((*(float *)(pcVar63 + 0x18) == 0.0) && (!NAN(*(float *)(pcVar63 + 0x18)))) {
          uVar72 = *(undefined4 *)(_Dst + (ulonglong)((byte)pcVar63[0x1c] ^ 1) * 4 + 0x20);
          puVar62 = (undefined *)(ulonglong)*(uint *)(_Dst + 0x1c);
          *puVar37 = uVar13;
          lVar57 = *(longlong *)(puVar36 + 0x30);
          puVar64 = *(undefined **)(puVar36 + 0x40);
          puVar52 = puVar36;
          if (lVar57 == *(longlong *)(puVar36 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                      ();
          }
code_r0x000141c7fabd:
          lVar48 = *(longlong *)(puVar52 + 0x28);
          *(undefined **)(lVar48 + lVar57 * 0x18) = puVar64;
          *(int *)(lVar48 + 8 + lVar57 * 0x18) = (int)puVar62;
          *(undefined4 *)(lVar48 + 0xc + lVar57 * 0x18) = uVar72;
code_r0x000141c7fada:
          *(undefined1 *)(lVar48 + 0x10 + lVar57 * 0x18) = 1;
          goto code_r0x000141c7ff10;
        }
        break;
      case 5:
        if ((*(float *)(pcVar63 + 0x18) == 0.0) && (!NAN(*(float *)(pcVar63 + 0x18)))) {
          uVar72 = *(undefined4 *)(_Dst + (ulonglong)((byte)pcVar63[0x1c] ^ 1) * 4 + 0x20);
          puVar62 = (undefined *)(ulonglong)*(uint *)(_Dst + 0x1c);
          *puVar35 = uVar12;
          lVar57 = *(longlong *)(puVar34 + 0x30);
          puVar64 = *(undefined **)(puVar34 + 0x40);
          puVar52 = puVar34;
          if (lVar57 == *(longlong *)(puVar34 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                      ();
          }
          goto code_r0x000141c7fabd;
        }
        break;
      case 6:
        if ((*(float *)(pcVar63 + 0x18) == 0.0) && (!NAN(*(float *)(pcVar63 + 0x18)))) {
          puVar62 = *(undefined **)(_Dst + 0x20);
          puVar52 = puVar32;
          if (pcVar63[0x1c] == '\0') {
            uVar72 = *(undefined4 *)(_Dst + 0x2c);
            *puVar33 = uVar11;
            lVar57 = *(longlong *)(puVar32 + 0x30);
            puVar64 = *(undefined **)(puVar32 + 0x40);
            if (lVar57 == *(longlong *)(puVar32 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                        ();
            }
          }
          else {
            uVar72 = *(undefined4 *)(_Dst + 0x28);
            *puVar33 = uVar11;
            lVar57 = *(longlong *)(puVar32 + 0x30);
            puVar64 = *(undefined **)(puVar32 + 0x40);
            if (lVar57 == *(longlong *)(puVar32 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                        ();
            }
          }
          goto code_r0x000141c7fe0d;
        }
        break;
      case 7:
        if ((*(float *)(pcVar63 + 0x18) == 0.0) && (!NAN(*(float *)(pcVar63 + 0x18)))) {
          puVar62 = (undefined *)(ulonglong)*(uint *)(_Dst + 0x10);
          if (pcVar63[0x1c] == '\0') {
            uVar54 = (ulonglong)(byte)_Dst[0x15];
            *puVar31 = uVar10;
            lVar57 = *(longlong *)(puVar30 + 0x30);
            uStack_98 = *(undefined8 *)(puVar30 + 0x40);
            if (lVar57 == *(longlong *)(puVar30 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                        ();
            }
          }
          else {
            uVar54 = (ulonglong)(byte)_Dst[0x14];
            *puVar31 = uVar10;
            lVar57 = *(longlong *)(puVar30 + 0x30);
            uStack_98 = *(undefined8 *)(puVar30 + 0x40);
            if (lVar57 == *(longlong *)(puVar30 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                        ();
            }
          }
          uVar54 = uVar54 << 0x20 | (ulonglong)puVar62;
          puVar52 = puVar30;
code_r0x000141c7fee6:
          puVar64 = (undefined *)(uVar54 | 0x10000000000);
          lVar48 = *(longlong *)(puVar52 + 0x28);
          *(undefined8 *)(lVar48 + lVar57 * 0x10) = uStack_98;
          *(undefined **)(lVar48 + 8 + lVar57 * 0x10) = puVar64;
          goto code_r0x000141c7ff10;
        }
        break;
      case 8:
        if ((*(float *)(pcVar63 + 0x18) == 0.0) && (!NAN(*(float *)(pcVar63 + 0x18)))) {
          uVar19 = *(uint *)(_Dst + 0x1c);
          puVar62 = (undefined *)(ulonglong)uVar19;
          if (pcVar63[0x1c] == '\0') {
            uVar72 = *(undefined4 *)(_Dst + 0x24);
            *puVar29 = uVar9;
            lVar57 = *(longlong *)(puVar28 + 0x30);
            puVar64 = *(undefined **)(puVar28 + 0x40);
            if (lVar57 == *(longlong *)(puVar28 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                        ();
            }
          }
          else {
            uVar72 = *(undefined4 *)(_Dst + 0x20);
            *puVar29 = uVar9;
            lVar57 = *(longlong *)(puVar28 + 0x30);
            puVar64 = *(undefined **)(puVar28 + 0x40);
            if (lVar57 == *(longlong *)(puVar28 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                        ();
            }
          }
          lVar48 = *(longlong *)(puVar28 + 0x28);
          *(undefined **)(lVar48 + lVar57 * 0x18) = puVar64;
          *(uint *)(lVar48 + 8 + lVar57 * 0x18) = uVar19;
          *(undefined4 *)(lVar48 + 0xc + lVar57 * 0x18) = uVar72;
          puVar52 = puVar28;
          goto code_r0x000141c7fada;
        }
        break;
      case 9:
        if ((*(float *)(pcVar63 + 0x18) == 0.0) && (!NAN(*(float *)(pcVar63 + 0x18)))) {
          puVar62 = (undefined *)(ulonglong)*(uint *)(_Dst + 0x10);
          if (pcVar63[0x1c] == '\0') {
            uVar54 = (ulonglong)(byte)_Dst[0x15];
            *puVar27 = uVar8;
            lVar57 = *(longlong *)(puVar26 + 0x30);
            uStack_98 = *(undefined8 *)(puVar26 + 0x40);
            if (lVar57 == *(longlong *)(puVar26 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                        ();
            }
          }
          else {
            uVar54 = (ulonglong)(byte)_Dst[0x14];
            *puVar27 = uVar8;
            lVar57 = *(longlong *)(puVar26 + 0x30);
            uStack_98 = *(undefined8 *)(puVar26 + 0x40);
            if (lVar57 == *(longlong *)(puVar26 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                        ();
            }
          }
          uVar54 = uVar54 << 0x20 | (ulonglong)puVar62;
          puVar52 = puVar26;
          goto code_r0x000141c7fee6;
        }
        break;
      case 10:
        if ((*(float *)(pcVar63 + 0x18) == 0.0) && (!NAN(*(float *)(pcVar63 + 0x18)))) {
          uVar72 = *(undefined4 *)(_Dst + (ulonglong)((byte)pcVar63[0x1c] ^ 1) * 4 + 0x20);
          puVar62 = (undefined *)(ulonglong)*(uint *)(_Dst + 0x1c);
          *puVar25 = uVar7;
          lVar57 = *(longlong *)(puVar24 + 0x30);
          puVar64 = *(undefined **)(puVar24 + 0x40);
          puVar52 = puVar24;
          if (lVar57 == *(longlong *)(puVar24 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                      ();
          }
          goto code_r0x000141c7fabd;
        }
        break;
      case 0xb:
        if ((*(float *)(pcVar63 + 0x18) == 0.0) && (!NAN(*(float *)(pcVar63 + 0x18)))) {
          cVar47 = _Dst[(ulonglong)(byte)pcVar63[0x1c] ^ 0x15];
          uVar19 = *(uint *)(_Dst + 0x10);
          puVar64 = (undefined *)(ulonglong)uVar19;
          *puVar23 = uVar6;
          lVar57 = *(longlong *)(puVar22 + 0x30);
          uVar46 = *(undefined8 *)(puVar22 + 0x40);
          if (lVar57 == *(longlong *)(puVar22 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                      ();
          }
          puVar62 = (undefined *)((ulonglong)CONCAT14(cVar47,uVar19) | 0x10000000000);
          lVar48 = *(longlong *)(puVar22 + 0x28);
          *(undefined8 *)(lVar48 + lVar57 * 0x10) = uVar46;
          *(undefined **)(lVar48 + 8 + lVar57 * 0x10) = puVar62;
          puVar52 = puVar22;
          goto code_r0x000141c7ff10;
        }
        break;
      case 0xc:
        if ((*(float *)(pcVar63 + 0x18) == 0.0) && (!NAN(*(float *)(pcVar63 + 0x18)))) {
          uVar72 = *(undefined4 *)(_Dst + (ulonglong)((byte)pcVar63[0x1c] ^ 1) * 4 + 0x20);
          puVar62 = (undefined *)(ulonglong)*(uint *)(_Dst + 0x1c);
          *puVar21 = uVar5;
          lVar57 = *(longlong *)(local_a8 + 0x30);
          puVar64 = *(undefined **)(local_a8 + 0x40);
          puVar52 = local_a8;
          if (lVar57 == *(longlong *)(local_a8 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                      ();
          }
          goto code_r0x000141c7fabd;
        }
        break;
      case 0xe:
        _Dst[-9] = _Dst[-9] ^ (byte)puVar52;
        puVar1 = (uint *)(_Dst + (longlong)(&UNK_142f20379 + iVar18));
        *puVar1 = *puVar1 >> 1 | (uint)((*puVar1 & 1) != 0) << 0x1f;
        goto code_r0x000141c80123;
      case 0xf:
        *_Dst = *_Dst + cVar47;
        plVar49 = (longlong *)CONCAT71(uVar50,cVar47 + '\x03');
        *plVar49 = *plVar49 + -1;
        lVar56 = *plVar49;
        goto code_r0x000141c801b9;
      case 0x10:
        *_Dst = *_Dst + cVar47;
        plVar49 = (longlong *)CONCAT71(uVar50,cVar47 + '\x03');
        *plVar49 = *plVar49 + -1;
        lVar56 = *plVar49;
        goto code_r0x000141c8015f;
      case 0x11:
        *_Dst = *_Dst + cVar47;
        plVar49 = (longlong *)CONCAT71(uVar50,cVar47 + '\x03');
        *plVar49 = *plVar49 + -1;
        lVar56 = *plVar49;
        goto code_r0x000141c8017d;
      case 0x12:
        goto joined_r0x000141c80080;
      case 0x13:
        *_Dst = *_Dst + cVar47;
        plVar49 = (longlong *)CONCAT71(uVar50,cVar47 + '\x03');
        *plVar49 = *plVar49 + -1;
        lVar56 = *plVar49;
        goto code_r0x000141c801d7;
      case 0x14:
        *_Dst = *_Dst + cVar47;
        plVar49 = (longlong *)CONCAT71(uVar50,cVar47 + '\x03');
        *plVar49 = *plVar49 + -1;
        lVar56 = *plVar49;
        goto code_r0x000141c801f5;
      case 0x15:
        *_Dst = *_Dst + cVar47;
        plVar49 = (longlong *)CONCAT71(uVar50,cVar47 + '\x03');
        *plVar49 = *plVar49 + -1;
        lVar56 = *plVar49;
        goto code_r0x000141c8019b;
      case 0x16:
        *_Dst = *_Dst + cVar47;
        plVar49 = (longlong *)CONCAT71(uVar50,cVar47 + '\x03');
        *plVar49 = *plVar49 + -1;
        lVar56 = *plVar49;
        goto code_r0x000141c80231;
      case 0x17:
        goto code_r0x000141c8013a;
      }
      if ((pcVar63[8] & 1U) == 0) {
LAB_141c7ff2c:
        if (local_b8 == 0) {
          alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
        }
        *(char **)(local_b8 * 8 + 8) = pcVar65;
        local_b8 = local_b8 + 1;
      }
      else {
code_r0x000141c7fc5a:
        fVar69 = *(float *)(pcVar63 + 0xc) / fVar2;
        fVar3 = *(float *)(pcVar63 + 0x18);
        if (fVar69 <= 0.1) {
          fVar69 = 0.1;
        }
        if (fVar69 < fVar3) {
          if (local_b8 == 0) {
            alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
          }
          *(char **)(local_b8 * 8 + 8) = pcVar65;
          local_b8 = local_b8 + 1;
          fVar3 = *(float *)(pcVar63 + 0x18);
        }
        *(float *)(pcVar63 + 0x18) = fVar3 + *(float *)(lVar60 + 0x28);
      }
    }
  }
  if (1 < local_b8) {
    uVar54 = local_b8 >> 1;
    if (local_b8 < 8) {
      uVar55 = 0;
    }
    else {
      uVar55 = uVar54 & 0xfffffffffffffffc;
      pauVar58 = (undefined1 (*) [16])(local_b8 * 8 + -8);
      uVar61 = 0;
      do {
        auVar68 = *(undefined1 (*) [16])(uVar61 * 8 + 8);
        auVar4 = *(undefined1 (*) [16])(uVar61 * 8 + 0x18);
        auVar67 = pauVar58[-1];
        auVar70 = *pauVar58;
        auVar71._0_8_ = auVar70._8_8_;
        auVar71._8_4_ = auVar70._0_4_;
        auVar71._12_4_ = auVar70._4_4_;
        auVar70._0_8_ = auVar67._8_8_;
        auVar70._8_4_ = auVar67._0_4_;
        auVar70._12_4_ = auVar67._4_4_;
        *(undefined1 (*) [16])(uVar61 * 8 + 8) = auVar71;
        *(undefined1 (*) [16])(uVar61 * 8 + 0x18) = auVar70;
        auVar67._0_8_ = auVar68._8_8_;
        auVar67._8_4_ = auVar68._0_4_;
        auVar67._12_4_ = auVar68._4_4_;
        *pauVar58 = auVar67;
        auVar68._0_8_ = auVar4._8_8_;
        auVar68._8_4_ = auVar4._0_4_;
        auVar68._12_4_ = auVar4._4_4_;
        pauVar58[-1] = auVar68;
        uVar61 = uVar61 + 4;
        pauVar58 = pauVar58 + -2;
      } while (uVar55 != uVar61);
      if (uVar54 == uVar55) goto joined_r0x000141c80080;
    }
    puVar59 = (undefined8 *)(uVar55 * 8);
    lVar56 = -uVar55;
    do {
      puVar59 = puVar59 + 1;
      uVar46 = *puVar59;
      *puVar59 = *(undefined8 *)(local_b8 * 8 + lVar56 * 8);
      *(undefined8 *)(local_b8 * 8 + lVar56 * 8) = uVar46;
      lVar56 = lVar56 + -1;
    } while (-lVar56 != uVar54);
  }
joined_r0x000141c80080:
  local_a8 = (undefined *)0x0;
  if (local_b8 != 0) {
    local_a8 = (undefined *)0x0;
    local_b0 = &DAT_00000008;
    pcVar66 = (char *)(local_b8 * 8 + 8);
    puVar62 = &DAT_142f203c0;
    do {
      puVar22 = *(undefined **)local_b0;
      puVar64 = *(undefined **)(extraout_RAX + 0x10);
      if (puVar64 <= puVar22) {
                    // WARNING: Subroutine does not return
        alloc::vec::impl$1::remove::assert_failed();
      }
      lVar56 = *(longlong *)(extraout_RAX + 8);
      lVar60 = (longlong)puVar22 * 0x30;
      _Dst = (char *)(lVar56 + lVar60);
      local_158 = *(ulonglong *)(lVar56 + lVar60);
      plStack_150 = (longlong *)((ulonglong *)(lVar56 + lVar60))[1];
      local_148 = *(longlong **)(lVar56 + 0x10 + lVar60);
      _Src = (undefined *)(lVar56 + lVar60 + 0x30);
      puVar52 = puVar64 + ~(ulonglong)puVar22;
code_r0x000141c80123:
      memmove(_Dst,_Src,(longlong)puVar52 * 0x30);
      *(undefined **)(extraout_RAX + 0x10) = puVar64 + -1;
code_r0x000141c8013a:
      if ((local_158 & 0xff) < 10) {
        switch(puVar62 + *(int *)(puVar62 + (local_158 & 0xff) * 4)) {
        case (undefined *)0x141c800af:
          core::ptr::drop_in_place<country_core::systems::history::replay_wall::WallReplayCmd>();
          break;
        case (undefined *)0x141c80154:
          LOCK();
          *local_148 = *local_148 + -1;
          lVar56 = *local_148;
          UNLOCK();
code_r0x000141c8015f:
          if (lVar56 == 0) {
            alloc::sync::Arc::
            drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>
                      ();
          }
          break;
        case (undefined *)0x141c80172:
          LOCK();
          *local_148 = *local_148 + -1;
          lVar56 = *local_148;
          UNLOCK();
code_r0x000141c8017d:
          if (lVar56 == 0) {
            alloc::sync::Arc::
            drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo>,alloc::alloc::Global>
                      ();
          }
          break;
        case (undefined *)0x141c80190:
          LOCK();
          *local_148 = *local_148 + -1;
          lVar56 = *local_148;
          UNLOCK();
code_r0x000141c8019b:
          if (lVar56 == 0) {
            alloc::sync::Arc::
            drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>
                      ();
          }
          break;
        case (undefined *)0x141c801ae:
          LOCK();
          *plStack_150 = *plStack_150 + -1;
          lVar56 = *plStack_150;
          UNLOCK();
code_r0x000141c801b9:
          if (lVar56 == 0) {
            alloc::sync::Arc::
            drop_slow<enum2_<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global>
                      ();
          }
          break;
        case (undefined *)0x141c801cc:
          LOCK();
          *local_148 = *local_148 + -1;
          lVar56 = *local_148;
          UNLOCK();
code_r0x000141c801d7:
          if (lVar56 == 0) {
            alloc::sync::Arc::
            drop_slow<enum2_<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>
                      ();
          }
          break;
        case (undefined *)0x141c801ea:
          LOCK();
          *local_148 = *local_148 + -1;
          lVar56 = *local_148;
          UNLOCK();
code_r0x000141c801f5:
          if (lVar56 == 0) {
            alloc::sync::Arc::
            drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>
                      ();
          }
          break;
        case (undefined *)0x141c80208:
          LOCK();
          *local_148 = *local_148 + -1;
          UNLOCK();
          if (*local_148 == 0) {
            alloc::sync::Arc::
            drop_slow<enum2_<country_core::resources::history::roof_edit::RoofEdit>,alloc::alloc::Global>
                      ();
          }
          break;
        case (undefined *)0x141c80226:
          LOCK();
          *local_148 = *local_148 + -1;
          lVar56 = *local_148;
          UNLOCK();
code_r0x000141c80231:
          if (lVar56 == 0) {
            alloc::sync::Arc::
            drop_slow<enum2_<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>
                      ();
          }
          break;
        case (undefined *)0x141c80262:
          LOCK();
          *local_148 = *local_148 + -1;
          UNLOCK();
          if (*local_148 == 0) {
            alloc::sync::Arc::
            drop_slow<enum2_<country_core::resources::history::tree_edit::TreeEdit>,alloc::alloc::Global>
                      ();
          }
        }
      }
      else {
        LOCK();
        *local_148 = *local_148 + -1;
        UNLOCK();
        if (*local_148 == 0) {
          alloc::sync::Arc::
          drop_slow<std::sync::poison::mutex::Mutex<winit::dpi::PhysicalSize<u32>_>,alloc::alloc::Global>
                    ();
        }
      }
      local_b0 = local_b0 + 8;
    } while (local_b0 != pcVar66);
  }
  if (local_a8 != (undefined *)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

