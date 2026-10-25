
void __cdecl country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall_mut(void)

{
  longlong lVar1;
  longlong in_RCX;
  undefined1 local_48 [8];
  undefined1 *local_40;
  code *local_38;
  undefined **local_30;
  undefined8 local_28;
  undefined1 **local_20;
  undefined8 local_18;
  undefined8 local_10;
  
  lVar1 = 0;
  if ((byte)(*(byte *)(in_RCX + 0x22) - 2) < 3) {
    lVar1 = (ulonglong)*(byte *)(in_RCX + 0x22) - 1;
  }
  if (lVar1 == 0) {
    return;
  }
  if (lVar1 == 1) {
    local_38 = core::fmt::impl$74::fmt<country_core::resources::walls::decorator::RoofAttachment>;
    local_30 = &PTR_s_internal_error__entered_unreacha_142a927e0;
  }
  else if (lVar1 == 2) {
    local_38 = core::fmt::impl$74::fmt<country_core::resources::walls::decorator::TerrainAttachment>
    ;
    local_30 = &PTR_s_internal_error__entered_unreacha_142a92830;
  }
  else {
    local_38 = core::fmt::impl$74::fmt<country_core::resources::walls::decorator::StairAttachment>;
    local_30 = &PTR_s_internal_error__entered_unreacha_142a92878;
  }
  local_40 = local_48;
  local_28 = 1;
  local_10 = 0;
  local_20 = &local_40;
  local_18 = 1;
                    /* WARNING: Subroutine does not return */
  core::panicking::panic_fmt();
}

