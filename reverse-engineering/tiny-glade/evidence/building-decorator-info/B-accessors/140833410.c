
void __cdecl country_core::resources::walls::decorator::DecoratorInfo::dst_as_terrain(void)

{
  longlong in_RCX;
  undefined1 local_40 [8];
  code *local_38;
  undefined **local_30;
  undefined8 local_28;
  undefined1 *local_20;
  undefined8 local_18;
  undefined8 local_10;
  
  if (*(char *)(in_RCX + 0x22) == '\x03') {
    return;
  }
  local_38 = impl$99::fmt;
  local_30 = &PTR_s_internal_error__entered_unreacha_142a92798;
  local_28 = 1;
  local_10 = 0;
  local_20 = local_40;
  local_18 = 1;
                    /* WARNING: Subroutine does not return */
  core::panicking::panic_fmt();
}

