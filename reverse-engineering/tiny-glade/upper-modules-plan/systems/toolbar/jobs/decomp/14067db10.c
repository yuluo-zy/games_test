// 0x14067db10 format
// signature: void __cdecl format(void)

void __cdecl toolbar::mei::widgets::tweaker::impl$4::format(void)

{
  undefined8 *in_RDX;
  undefined8 local_80;
  code *local_78;
  undefined *local_70;
  undefined8 local_68;
  undefined8 *local_60;
  undefined8 local_58;
  undefined8 *local_50;
  undefined8 local_48;
  undefined8 local_40;
  undefined8 local_38;
  undefined8 local_30;
  undefined8 local_28;
  undefined8 local_20;
  undefined8 local_18;
  undefined1 local_10;
  
  if (0.1 <= *(float *)(in_RDX + 1)) {
    local_80 = *in_RDX;
    local_38 = 1;
    local_28 = 1;
  }
  else {
    local_80 = *in_RDX;
    local_38 = 2;
    local_28 = 2;
  }
  local_30 = 0;
  local_40 = 0;
  local_78 = ::core::fmt::float::impl$5::fmt;
  local_20 = 0;
  local_18 = 0x20;
  local_10 = 3;
  local_70 = &DAT_142a16de8;
  local_68 = 1;
  local_60 = &local_80;
  local_58 = 1;
  local_50 = &local_40;
  local_48 = 1;
  alloc::fmt::format::format_inner();
  return;
}

