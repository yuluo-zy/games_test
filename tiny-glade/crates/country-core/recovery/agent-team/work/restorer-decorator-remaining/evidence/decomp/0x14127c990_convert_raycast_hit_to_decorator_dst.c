
void __cdecl
system_decorator::ui_systems::calculate_decorator_dst::convert_raycast_hit_to_decorator_dst(void)

{
  undefined8 uVar1;
  undefined1 uVar2;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  undefined1 *in_RCX;
  undefined1 *in_R9;
  undefined1 auStack_60 [8];
  undefined *puStack_58;
  undefined8 uStack_50;
  undefined1 *puStack_48;
  undefined8 uStack_40;
  undefined8 uStack_38;
  
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(*in_R9) {
  case 0:
    uVar1 = *(undefined8 *)(in_R9 + 8);
    *in_RCX = 0;
    *(undefined8 *)(in_RCX + 8) = uVar1;
    return;
  case 2:
    uVar1 = *(undefined8 *)(in_R9 + 8);
    uVar2 = in_R9[2];
    if ((in_R9[1] == '\0') ||
       (country_core::resources::walls::decorator::DecoratorType::can_place_on_roof(),
       extraout_AL_01 != '\0')) {
      *in_RCX = 1;
      in_RCX[1] = uVar2;
      *(undefined8 *)(in_RCX + 8) = uVar1;
      return;
    }
  default:
    *in_RCX = 4;
    return;
  case 3:
    puStack_58 = &anon_ba42121df8d3f082639b4969edec4a0b_1_llvm_205399390136779301;
    uStack_50 = 1;
    puStack_48 = auStack_60;
    uStack_40 = 0;
    uStack_38 = 0;
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  case 5:
  case 9:
  case 0xc:
  case 0xf:
  case 0x10:
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  case 0xb:
    country_core::resources::walls::decorator::DecoratorType::can_place_on_terrain();
    if (extraout_AL_00 != '\0') {
      *in_RCX = 2;
      return;
    }
    break;
  case 0xd:
    uVar1 = *(undefined8 *)(in_R9 + 4);
    country_core::resources::walls::decorator::DecoratorType::can_place_on_stair_top();
    if (extraout_AL != '\0') {
      *in_RCX = 3;
      *(undefined8 *)(in_RCX + 4) = uVar1;
      return;
    }
  }
  *in_RCX = 4;
  return;
}

