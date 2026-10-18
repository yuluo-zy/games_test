// name=hide_color_menu
// sig=void __cdecl hide_color_menu(void)
// size=2408
// address=0x1422adcf0

void __cdecl system_color::show_color_menu::hide_color_menu(void)

{
  uint *puVar1;
  char cVar2;
  undefined1 uVar3;
  undefined4 uVar4;
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
  undefined4 uVar18;
  undefined4 uVar19;
  uint uVar20;
  ulonglong uVar21;
  longlong lVar22;
  undefined4 *puVar23;
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
  undefined4 *puVar38;
  longlong lVar39;
  undefined4 *puVar40;
  longlong lVar41;
  undefined4 *puVar42;
  longlong lVar43;
  undefined4 *puVar44;
  longlong *plVar45;
  undefined4 *puVar46;
  longlong lVar47;
  undefined4 *puVar48;
  longlong lVar49;
  undefined4 *puVar50;
  longlong lVar51;
  undefined4 *puVar52;
  longlong lVar53;
  undefined8 uVar54;
  uint5 uVar55;
  char extraout_AL;
  longlong lVar56;
  ulonglong *puVar57;
  undefined8 *in_RCX;
  ulonglong uVar58;
  ulonglong uVar59;
  longlong lVar60;
  longlong *in_RDX;
  longlong lVar61;
  longlong lVar62;
  ulonglong uVar63;
  char cVar64;
  ulonglong uVar65;
  longlong *in_R8;
  undefined8 uVar66;
  undefined4 uVar67;
  undefined4 uVar68;
  longlong *in_R9;
  ulonglong uVar69;
  longlong *plVar70;
  longlong lVar71;
  longlong lVar72;
  ulonglong uVar73;
  uint *puVar74;
  longlong lVar75;
  undefined8 uVar76;
  longlong in_stack_00000030;
  longlong *in_stack_00000038;
  longlong *in_stack_00000040;
  longlong *in_stack_00000048;
  longlong local_280;
  longlong local_168;
  longlong local_160;
  
  puVar57 = (ulonglong *)*in_RCX;
  lVar75 = in_RCX[1];
  uVar63 = *puVar57;
  uVar59 = 0;
  uVar65 = uVar63 - *(ulonglong *)(lVar75 + 0x18);
  if (uVar63 < *(ulonglong *)(lVar75 + 0x18)) {
    uVar65 = uVar59;
  }
  uVar58 = uVar63 - *(ulonglong *)(lVar75 + 0x38);
  if (uVar63 < *(ulonglong *)(lVar75 + 0x38)) {
    uVar58 = uVar59;
  }
  uVar63 = *(ulonglong *)(lVar75 + 0x10);
  uVar21 = *(ulonglong *)(lVar75 + 0x30);
  uVar69 = uVar21 - uVar58;
  if (uVar58 > uVar21) {
    uVar69 = uVar59;
  }
  uVar73 = uVar63 - uVar65;
  if (uVar65 > uVar63) {
    uVar73 = uVar59;
  }
  uVar59 = *(ulonglong *)(lVar75 + 0x40);
  lVar71 = *(longlong *)(lVar75 + 8);
  lVar75 = *(longlong *)(lVar75 + 0x28);
  lVar22 = in_stack_00000040[4];
  puVar23 = (undefined4 *)in_stack_00000040[6];
  uVar4 = *(undefined4 *)((longlong)in_stack_00000040 + 0x3c);
  lVar72 = in_stack_00000040[8];
  puVar24 = (undefined4 *)in_stack_00000040[10];
  uVar5 = *(undefined4 *)((longlong)in_stack_00000040 + 0x5c);
  lVar25 = in_stack_00000040[0xc];
  puVar26 = (undefined4 *)in_stack_00000040[0xe];
  uVar6 = *(undefined4 *)((longlong)in_stack_00000040 + 0x7c);
  lVar27 = in_stack_00000040[0x10];
  puVar28 = (undefined4 *)in_stack_00000040[0x12];
  uVar7 = *(undefined4 *)((longlong)in_stack_00000040 + 0x9c);
  lVar29 = in_stack_00000040[0x14];
  puVar30 = (undefined4 *)in_stack_00000040[0x16];
  uVar8 = *(undefined4 *)((longlong)in_stack_00000040 + 0xbc);
  lVar31 = in_stack_00000040[0x18];
  puVar32 = (undefined4 *)in_stack_00000040[0x1a];
  uVar9 = *(undefined4 *)((longlong)in_stack_00000040 + 0xdc);
  lVar33 = in_stack_00000040[0x1c];
  puVar34 = (undefined4 *)in_stack_00000040[0x1e];
  uVar10 = *(undefined4 *)((longlong)in_stack_00000040 + 0xfc);
  lVar35 = in_stack_00000040[0x20];
  puVar36 = (undefined4 *)in_stack_00000040[0x22];
  uVar11 = *(undefined4 *)((longlong)in_stack_00000040 + 0x11c);
  lVar37 = in_stack_00000040[0x24];
  puVar38 = (undefined4 *)in_stack_00000040[0x26];
  uVar12 = *(undefined4 *)((longlong)in_stack_00000040 + 0x13c);
  lVar39 = in_stack_00000040[0x28];
  puVar40 = (undefined4 *)in_stack_00000040[0x2a];
  uVar13 = *(undefined4 *)((longlong)in_stack_00000040 + 0x15c);
  lVar41 = in_stack_00000040[0x2c];
  puVar42 = (undefined4 *)in_stack_00000040[0x2e];
  uVar14 = *(undefined4 *)((longlong)in_stack_00000040 + 0x17c);
  lVar43 = in_stack_00000040[0x30];
  puVar44 = (undefined4 *)in_stack_00000040[0x32];
  uVar15 = *(undefined4 *)((longlong)in_stack_00000040 + 0x19c);
  *puVar57 = (uVar59 - uVar69) - uVar73;
  if (uVar65 <= uVar63 && uVar63 - uVar65 != 0 || uVar58 <= uVar21 && uVar21 - uVar58 != 0) {
    lVar75 = lVar75 + uVar58 * 0x10;
    lVar71 = lVar71 + uVar65 * 0x10;
    if (uVar63 < uVar65) {
      lVar71 = 8;
    }
    lVar61 = 0;
    if (uVar65 < uVar63) {
      lVar61 = lVar71 + -0x10 + uVar73 * 0x10;
    }
    if (uVar21 < uVar58) {
      lVar75 = 8;
    }
    lVar75 = lVar75 + -0x10 + uVar69 * 0x10;
    if (uVar21 <= uVar58) {
      lVar75 = lVar61;
    }
    *puVar57 = uVar59;
    cVar64 = *(char *)(lVar75 + 8);
  }
  else {
    country_core::input::action_set::DigitalActionState::just_pressed();
    cVar64 = '\x01';
    if (extraout_AL == '\0') {
      return;
    }
  }
  lVar75 = *in_R8;
  lVar71 = in_R8[1];
  puVar74 = *(uint **)(lVar75 + 0xf8);
  puVar1 = puVar74 + *(longlong *)(lVar75 + 0x100);
  plVar45 = *(longlong **)(in_stack_00000030 + 8);
  plVar70 = *(longlong **)(in_stack_00000030 + 0x10);
  uVar16 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
  lVar61 = *in_stack_00000038;
  puVar46 = (undefined4 *)in_stack_00000038[2];
  uVar17 = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
  lVar47 = *in_stack_00000040;
  puVar48 = (undefined4 *)in_stack_00000040[2];
  uVar18 = *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
  lVar49 = *in_stack_00000048;
  puVar50 = (undefined4 *)in_stack_00000048[2];
  uVar19 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  lVar51 = *in_RDX;
  puVar52 = (undefined4 *)in_RDX[2];
  cVar2 = *(char *)(lVar75 + 0x108);
  local_160 = 8;
  local_280 = 0;
  local_168 = 8;
  uVar63 = 0;
  uVar65 = 0;
joined_r0x0001422ae245:
  do {
    if (cVar2 == '\0') {
      if (uVar65 == uVar63) {
        do {
          if (puVar74 == puVar1) goto LAB_1422aeeac;
          uVar20 = *puVar74;
          puVar74 = puVar74 + 1;
          uVar63 = *(ulonglong *)(*(longlong *)(lVar71 + 0x100) + 0x58 + (ulonglong)uVar20 * 0xa0);
        } while (uVar63 == 0);
        lVar56 = *(longlong *)(lVar71 + 0x100) + (ulonglong)uVar20 * 0xa0;
        uVar65 = (ulonglong)*(uint *)(lVar56 + 0x94);
        if (*(ulonglong *)(lVar75 + 0x110) <
            *(ulonglong *)(*(longlong *)(lVar71 + 0x1a8) + 0x40 + uVar65 * 0x48)) {
          lVar62 = *(longlong *)(lVar71 + 0x1a8) + uVar65 * 0x48;
          uVar65 = *(ulonglong *)(*(longlong *)(lVar62 + 0x38) + *(ulonglong *)(lVar75 + 0x110) * 8)
          ;
          if (uVar65 == 0) goto LAB_1422ae2df;
          local_280 = *(longlong *)(*(longlong *)(lVar62 + 0x18) + 0x10 + ~uVar65 * 0x30);
        }
        else {
LAB_1422ae2df:
          local_280 = 0;
        }
        local_168 = *(longlong *)(lVar56 + 0x50);
        uVar65 = 0;
      }
      puVar57 = (ulonglong *)(local_168 + uVar65 * 0x10);
      uVar59 = (ulonglong)*(uint *)(local_168 + 8 + uVar65 * 0x10);
    }
    else {
      if (uVar65 == uVar63) {
        do {
          if (puVar74 == puVar1) goto LAB_1422aeeac;
          uVar20 = *puVar74;
          puVar74 = puVar74 + 1;
          uVar63 = *(ulonglong *)(*(longlong *)(lVar71 + 0x1a8) + 0x10 + (ulonglong)uVar20 * 0x48);
        } while (uVar63 == 0);
        lVar56 = *(longlong *)(lVar71 + 0x1a8) + (ulonglong)uVar20 * 0x48;
        if ((*(ulonglong *)(lVar75 + 0x110) < *(ulonglong *)(lVar56 + 0x40)) &&
           (uVar65 = *(ulonglong *)
                      (*(longlong *)(lVar56 + 0x38) + *(ulonglong *)(lVar75 + 0x110) * 8),
           uVar65 != 0)) {
          local_280 = *(longlong *)(*(longlong *)(lVar56 + 0x18) + 0x10 + ~uVar65 * 0x30);
        }
        else {
          local_280 = 0;
        }
        local_160 = *(longlong *)(lVar56 + 8);
        uVar65 = 0;
      }
      puVar57 = (ulonglong *)(local_160 + uVar65 * 8);
      uVar59 = uVar65 & 0xffffffff;
    }
    lVar56 = uVar59 * 0xa0 + local_280;
    if (*puVar57 >> 0x20 == 0) {
LAB_1422aeeac:
      lVar75 = *in_R9;
      lVar71 = in_R9[1];
      puVar74 = *(uint **)(lVar75 + 0xf8);
      puVar1 = puVar74 + *(longlong *)(lVar75 + 0x100);
      if (plVar45 != (longlong *)0x0) {
        plVar70 = plVar45;
      }
      if (*(char *)(lVar75 + 0x108) == '\0') {
        while (puVar74 != puVar1) {
          uVar20 = *puVar74;
          puVar74 = puVar74 + 1;
          lVar75 = *(longlong *)(lVar71 + 0x100);
          lVar22 = *(longlong *)(lVar75 + 0x58 + (ulonglong)uVar20 * 0xa0);
          if (lVar22 != 0) {
            lVar75 = *(longlong *)(lVar75 + (ulonglong)uVar20 * 0xa0 + 0x50);
            lVar72 = 0;
            do {
              uVar63 = *(ulonglong *)(lVar75 + lVar72 * 0x10);
              if (uVar63 >> 0x20 == 0) {
                return;
              }
              lVar25 = plVar70[2];
              if ((ulonglong)(*plVar70 - lVar25) < 0x20) {
                alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
              }
              lVar72 = lVar72 + 1;
              lVar27 = plVar70[1];
              *(code **)(lVar27 + lVar25) =
                   core::ops::function::FnOnce::
                   call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::color_icon::CustomizationIconMarkedForDeletion>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
              ;
              *(code **)(lVar27 + 8 + lVar25) = bevy_ecs::error::handler::panic;
              *(ulonglong *)(lVar27 + 0x10 + lVar25) = uVar63;
              *(undefined1 *)(lVar27 + 0x18 + lVar25) = 0;
              plVar70[2] = lVar25 + 0x20;
            } while (lVar72 != lVar22);
          }
        }
      }
      else {
        while (puVar74 != puVar1) {
          uVar20 = *puVar74;
          puVar74 = puVar74 + 1;
          lVar75 = *(longlong *)(lVar71 + 0x1a8);
          lVar22 = *(longlong *)(lVar75 + 0x10 + (ulonglong)uVar20 * 0x48);
          if (lVar22 != 0) {
            lVar75 = *(longlong *)(lVar75 + (ulonglong)uVar20 * 0x48 + 8);
            lVar72 = 0;
            do {
              uVar63 = *(ulonglong *)(lVar75 + lVar72 * 8);
              if (uVar63 >> 0x20 == 0) {
                return;
              }
              lVar25 = plVar70[2];
              if ((ulonglong)(*plVar70 - lVar25) < 0x20) {
                alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
              }
              lVar72 = lVar72 + 1;
              lVar27 = plVar70[1];
              *(code **)(lVar27 + lVar25) =
                   core::ops::function::FnOnce::
                   call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::color_icon::CustomizationIconMarkedForDeletion>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
              ;
              *(code **)(lVar27 + 8 + lVar25) = bevy_ecs::error::handler::panic;
              *(ulonglong *)(lVar27 + 0x10 + lVar25) = uVar63;
              *(undefined1 *)(lVar27 + 0x18 + lVar25) = 0;
              plVar70[2] = lVar25 + 0x20;
            } while (lVar72 != lVar22);
          }
        }
      }
      return;
    }
    uVar65 = uVar65 + 1;
    if (cVar64 == '\0') break;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
    switch(*(undefined1 *)(lVar56 + 0x30)) {
    case 0:
      uVar66 = *(undefined8 *)(lVar56 + 0x38);
      uVar68 = *(undefined4 *)(lVar56 + 0x68);
      *puVar48 = uVar17;
      lVar62 = *(longlong *)(lVar47 + 0x30);
      uVar76 = *(undefined8 *)(lVar47 + 0x40);
      if (lVar62 == *(longlong *)(lVar47 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      lVar60 = *(longlong *)(lVar47 + 0x28);
      *(undefined8 *)(lVar60 + lVar62 * 0x18) = uVar76;
      *(undefined8 *)(lVar60 + 8 + lVar62 * 0x18) = uVar66;
      *(undefined4 *)(lVar60 + 0x10 + lVar62 * 0x18) = uVar68;
      *(undefined1 *)(lVar60 + 0x14 + lVar62 * 0x18) = 0;
      *(longlong *)(lVar47 + 0x30) = lVar62 + 1;
      *(longlong *)(lVar47 + 0x40) = *(longlong *)(lVar47 + 0x40) + 1;
      uVar3 = *(undefined1 *)(lVar56 + 0x85);
      *puVar34 = uVar10;
      lVar62 = *(longlong *)(lVar33 + 0x30);
      uVar76 = *(undefined8 *)(lVar33 + 0x40);
      if (lVar62 == *(longlong *)(lVar33 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      lVar60 = *(longlong *)(lVar33 + 0x28);
      *(undefined8 *)(lVar60 + lVar62 * 0x18) = uVar76;
      *(undefined8 *)(lVar60 + 8 + lVar62 * 0x18) = uVar66;
      *(undefined1 *)(lVar60 + 0x10 + lVar62 * 0x18) = uVar3;
      *(undefined1 *)(lVar60 + 0x11 + lVar62 * 0x18) = uVar3;
      *(undefined2 *)(lVar60 + 0x12 + lVar62 * 0x18) = 0;
      *(longlong *)(lVar33 + 0x30) = lVar62 + 1;
      *(longlong *)(lVar33 + 0x40) = *(longlong *)(lVar33 + 0x40) + 1;
      uVar3 = *(undefined1 *)(lVar56 + 0x88);
      *puVar40 = uVar13;
      lVar62 = *(longlong *)(lVar39 + 0x30);
      uVar76 = *(undefined8 *)(lVar39 + 0x40);
      if (lVar62 == *(longlong *)(lVar39 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      lVar60 = *(longlong *)(lVar39 + 0x28);
      *(undefined8 *)(lVar60 + lVar62 * 0x18) = uVar76;
      *(undefined8 *)(lVar60 + 8 + lVar62 * 0x18) = uVar66;
      *(undefined1 *)(lVar60 + 0x10 + lVar62 * 0x18) = uVar3;
      *(undefined1 *)(lVar60 + 0x11 + lVar62 * 0x18) = uVar3;
      *(undefined1 *)(lVar60 + 0x12 + lVar62 * 0x18) = 0;
      *(longlong *)(lVar39 + 0x30) = lVar62 + 1;
      *(longlong *)(lVar39 + 0x40) = *(longlong *)(lVar39 + 0x40) + 1;
      uVar68 = *(undefined4 *)(lVar56 + 0x70);
      *puVar50 = uVar18;
      lVar56 = *(longlong *)(lVar49 + 0x30);
      uVar76 = *(undefined8 *)(lVar49 + 0x40);
      if (lVar56 == *(longlong *)(lVar49 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                  ();
      }
      lVar62 = *(longlong *)(lVar49 + 0x28);
      lVar60 = lVar56 * 0x20;
      *(undefined8 *)(lVar62 + lVar60) = uVar76;
      *(undefined8 *)(lVar62 + 8 + lVar60) = uVar66;
      *(undefined4 *)(lVar62 + 0x10 + lVar60) = uVar68;
      *(undefined4 *)(lVar62 + 0x14 + lVar60) = uVar68;
      *(undefined1 *)(lVar62 + 0x18 + lVar60) = 0;
      lVar62 = lVar49;
      goto code_r0x0001422aedd0;
    case 1:
      uVar66 = *(undefined8 *)(lVar56 + 0x38);
      uVar68 = *(undefined4 *)(lVar56 + 0x68);
      *puVar23 = uVar4;
      lVar56 = *(longlong *)(lVar22 + 0x30);
      uVar76 = *(undefined8 *)(lVar22 + 0x40);
      lVar62 = lVar22;
      if (lVar56 == *(longlong *)(lVar22 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      break;
    case 2:
      uVar66 = *(undefined8 *)(lVar56 + 0x38);
      uVar68 = *(undefined4 *)(lVar56 + 0x80);
      uVar76 = *(undefined8 *)(lVar56 + 0x78);
      *puVar46 = uVar16;
      lVar56 = *(longlong *)(lVar61 + 0x30);
      uVar54 = *(undefined8 *)(lVar61 + 0x40);
      if (lVar56 == *(longlong *)(lVar61 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                  ();
      }
      lVar62 = *(longlong *)(lVar61 + 0x28);
      lVar60 = lVar56 * 0x20;
      *(undefined8 *)(lVar62 + lVar60) = uVar54;
      *(undefined8 *)(lVar62 + 8 + lVar60) = uVar66;
      *(undefined8 *)(lVar62 + 0x10 + lVar60) = uVar76;
      *(undefined4 *)(lVar62 + 0x18 + lVar60) = uVar68;
      *(undefined1 *)(lVar62 + 0x1c + lVar60) = 0;
      *(longlong *)(lVar61 + 0x30) = lVar56 + 1;
      *(longlong *)(lVar61 + 0x40) = *(longlong *)(lVar61 + 0x40) + 1;
      goto code_r0x0001422aeddb;
    case 3:
      uVar68 = *(undefined4 *)(lVar56 + 0x34);
      uVar67 = *(undefined4 *)(lVar56 + 0x68);
      *puVar24 = uVar5;
      lVar56 = *(longlong *)(lVar72 + 0x30);
      uVar66 = *(undefined8 *)(lVar72 + 0x40);
      lVar62 = lVar72;
      if (lVar56 == *(longlong *)(lVar72 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      goto code_r0x0001422ae892;
    case 4:
      uVar68 = *(undefined4 *)(lVar56 + 0x34);
      uVar67 = *(undefined4 *)(lVar56 + 0x68);
      *puVar38 = uVar12;
      lVar56 = *(longlong *)(lVar37 + 0x30);
      uVar66 = *(undefined8 *)(lVar37 + 0x40);
      lVar62 = lVar37;
      if (lVar56 == *(longlong *)(lVar37 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      goto code_r0x0001422ae892;
    case 5:
      uVar66 = *(undefined8 *)(lVar56 + 0x38);
      uVar68 = *(undefined4 *)(lVar56 + 0x68);
      *puVar30 = uVar8;
      lVar56 = *(longlong *)(lVar29 + 0x30);
      uVar76 = *(undefined8 *)(lVar29 + 0x40);
      lVar62 = lVar29;
      if (lVar56 == *(longlong *)(lVar29 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      break;
    case 6:
      uVar68 = *(undefined4 *)(lVar56 + 0x34);
      uVar67 = *(undefined4 *)(lVar56 + 0x68);
      *puVar24 = uVar5;
      lVar62 = *(longlong *)(lVar72 + 0x30);
      uVar66 = *(undefined8 *)(lVar72 + 0x40);
      if (lVar62 == *(longlong *)(lVar72 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      lVar60 = *(longlong *)(lVar72 + 0x28);
      *(undefined8 *)(lVar60 + lVar62 * 0x18) = uVar66;
      *(undefined4 *)(lVar60 + 8 + lVar62 * 0x18) = uVar68;
      *(undefined4 *)(lVar60 + 0xc + lVar62 * 0x18) = uVar67;
      *(undefined1 *)(lVar60 + 0x10 + lVar62 * 0x18) = 0;
      *(longlong *)(lVar72 + 0x30) = lVar62 + 1;
      *(longlong *)(lVar72 + 0x40) = *(longlong *)(lVar72 + 0x40) + 1;
      uVar3 = *(undefined1 *)(lVar56 + 0x86);
      *puVar36 = uVar11;
      lVar56 = *(longlong *)(lVar35 + 0x30);
      uVar66 = *(undefined8 *)(lVar35 + 0x40);
      if (lVar56 == *(longlong *)(lVar35 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      uVar55 = CONCAT14(uVar3,uVar68);
      lVar62 = lVar35;
      goto code_r0x0001422aedb4;
    default:
      uVar66 = *(undefined8 *)(lVar56 + 0x38);
      uVar68 = *(undefined4 *)(lVar56 + 0x68);
      *puVar32 = uVar9;
      lVar56 = *(longlong *)(lVar31 + 0x30);
      uVar76 = *(undefined8 *)(lVar31 + 0x40);
      lVar62 = lVar31;
      if (lVar56 == *(longlong *)(lVar31 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      break;
    case 9:
      uVar68 = *(undefined4 *)(lVar56 + 0x34);
      uVar67 = *(undefined4 *)(lVar56 + 0x68);
      *puVar24 = uVar5;
      lVar62 = *(longlong *)(lVar72 + 0x30);
      uVar66 = *(undefined8 *)(lVar72 + 0x40);
      if (lVar62 == *(longlong *)(lVar72 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      lVar60 = *(longlong *)(lVar72 + 0x28);
      *(undefined8 *)(lVar60 + lVar62 * 0x18) = uVar66;
      *(undefined4 *)(lVar60 + 8 + lVar62 * 0x18) = uVar68;
      *(undefined4 *)(lVar60 + 0xc + lVar62 * 0x18) = uVar67;
      *(undefined1 *)(lVar60 + 0x10 + lVar62 * 0x18) = 0;
      *(longlong *)(lVar72 + 0x30) = lVar62 + 1;
      *(longlong *)(lVar72 + 0x40) = *(longlong *)(lVar72 + 0x40) + 1;
      uVar67 = *(undefined4 *)(lVar56 + 0x6c);
      *puVar28 = uVar7;
      lVar56 = *(longlong *)(lVar27 + 0x30);
      uVar66 = *(undefined8 *)(lVar27 + 0x40);
      if (lVar56 == *(longlong *)(lVar27 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      lVar60 = *(longlong *)(lVar27 + 0x28);
      *(undefined8 *)(lVar60 + lVar56 * 0x18) = uVar66;
      *(undefined4 *)(lVar60 + 8 + lVar56 * 0x18) = uVar68;
      *(undefined4 *)(lVar60 + 0xc + lVar56 * 0x18) = uVar67;
      lVar62 = lVar27;
      goto code_r0x0001422ae8a8;
    case 10:
      uVar68 = *(undefined4 *)(lVar56 + 0x34);
      uVar67 = *(undefined4 *)(lVar56 + 0x68);
      *puVar24 = uVar5;
      lVar62 = *(longlong *)(lVar72 + 0x30);
      uVar66 = *(undefined8 *)(lVar72 + 0x40);
      if (lVar62 == *(longlong *)(lVar72 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      lVar60 = *(longlong *)(lVar72 + 0x28);
      *(undefined8 *)(lVar60 + lVar62 * 0x18) = uVar66;
      *(undefined4 *)(lVar60 + 8 + lVar62 * 0x18) = uVar68;
      *(undefined4 *)(lVar60 + 0xc + lVar62 * 0x18) = uVar67;
      *(undefined1 *)(lVar60 + 0x10 + lVar62 * 0x18) = 0;
      *(longlong *)(lVar72 + 0x30) = lVar62 + 1;
      *(longlong *)(lVar72 + 0x40) = *(longlong *)(lVar72 + 0x40) + 1;
      uVar3 = *(undefined1 *)(lVar56 + 0x89);
      *puVar42 = uVar14;
      lVar56 = *(longlong *)(lVar41 + 0x30);
      uVar66 = *(undefined8 *)(lVar41 + 0x40);
      if (lVar56 == *(longlong *)(lVar41 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      uVar55 = CONCAT14(uVar3,uVar68);
      lVar62 = lVar41;
code_r0x0001422aedb4:
      lVar60 = *(longlong *)(lVar62 + 0x28);
      *(undefined8 *)(lVar60 + lVar56 * 0x10) = uVar66;
      *(ulonglong *)(lVar60 + 8 + lVar56 * 0x10) = (ulonglong)uVar55;
      goto code_r0x0001422aedd0;
    case 0xb:
      uVar68 = *(undefined4 *)(lVar56 + 0x34);
      uVar67 = *(undefined4 *)(lVar56 + 0x68);
      *puVar24 = uVar5;
      lVar62 = *(longlong *)(lVar72 + 0x30);
      uVar66 = *(undefined8 *)(lVar72 + 0x40);
      lVar60 = lVar72;
      if (lVar62 == *(longlong *)(lVar72 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      goto code_r0x0001422ae612;
    case 0xc:
      uVar68 = *(undefined4 *)(lVar56 + 0x34);
      uVar67 = *(undefined4 *)(lVar56 + 0x68);
      *puVar26 = uVar6;
      lVar62 = *(longlong *)(lVar25 + 0x30);
      uVar66 = *(undefined8 *)(lVar25 + 0x40);
      lVar60 = lVar25;
      if (lVar62 == *(longlong *)(lVar25 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
code_r0x0001422ae612:
      lVar53 = *(longlong *)(lVar60 + 0x28);
      *(undefined8 *)(lVar53 + lVar62 * 0x18) = uVar66;
      *(undefined4 *)(lVar53 + 8 + lVar62 * 0x18) = uVar68;
      *(undefined4 *)(lVar53 + 0xc + lVar62 * 0x18) = uVar67;
      *(undefined1 *)(lVar53 + 0x10 + lVar62 * 0x18) = 0;
      *(longlong *)(lVar60 + 0x30) = lVar62 + 1;
      *(longlong *)(lVar60 + 0x40) = *(longlong *)(lVar60 + 0x40) + 1;
      uVar67 = *(undefined4 *)(lVar56 + 0x74);
      *puVar44 = uVar15;
      lVar56 = *(longlong *)(lVar43 + 0x30);
      uVar66 = *(undefined8 *)(lVar43 + 0x40);
      lVar62 = lVar43;
      if (lVar56 == *(longlong *)(lVar43 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
code_r0x0001422ae892:
      lVar60 = *(longlong *)(lVar62 + 0x28);
      *(undefined8 *)(lVar60 + lVar56 * 0x18) = uVar66;
      *(undefined4 *)(lVar60 + 8 + lVar56 * 0x18) = uVar68;
      *(undefined4 *)(lVar60 + 0xc + lVar56 * 0x18) = uVar67;
code_r0x0001422ae8a8:
      *(undefined1 *)(lVar60 + 0x10 + lVar56 * 0x18) = 0;
      goto code_r0x0001422aedd0;
    }
    lVar60 = *(longlong *)(lVar62 + 0x28);
    *(undefined8 *)(lVar60 + lVar56 * 0x18) = uVar76;
    *(undefined8 *)(lVar60 + 8 + lVar56 * 0x18) = uVar66;
    *(undefined4 *)(lVar60 + 0x10 + lVar56 * 0x18) = uVar68;
    *(undefined1 *)(lVar60 + 0x14 + lVar56 * 0x18) = 0;
code_r0x0001422aedd0:
    *(longlong *)(lVar62 + 0x30) = lVar56 + 1;
    *(longlong *)(lVar62 + 0x40) = *(longlong *)(lVar62 + 0x40) + 1;
code_r0x0001422aeddb:
    bevy_ecs::system::commands::EntityCommands::despawn();
    *puVar52 = uVar19;
    lVar56 = *(longlong *)(lVar51 + 0x30);
    uVar66 = *(undefined8 *)(lVar51 + 0x40);
    if (lVar56 == *(longlong *)(lVar51 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    lVar62 = *(longlong *)(lVar51 + 0x28);
    *(undefined4 *)(lVar62 + lVar56 * 0x20) = 9;
    *(undefined8 *)(lVar62 + 0x18 + lVar56 * 0x20) = uVar66;
    *(longlong *)(lVar51 + 0x30) = lVar56 + 1;
    *(longlong *)(lVar51 + 0x40) = *(longlong *)(lVar51 + 0x40) + 1;
  } while( true );
  bevy_ecs::system::commands::EntityCommands::despawn();
  goto joined_r0x0001422ae245;
}


