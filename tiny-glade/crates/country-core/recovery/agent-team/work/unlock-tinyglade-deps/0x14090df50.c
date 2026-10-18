
void __cdecl country_core::game_config::impl$1::fmt(void)

{
  code *UNRECOVERED_JUMPTABLE_00;
  char *in_RCX;
  longlong in_RDX;
  
  UNRECOVERED_JUMPTABLE_00 = *(code **)(*(longlong *)(in_RDX + 0x38) + 0x18);
  if (*in_RCX != '\0') {
                    // WARNING: Could not recover jumptable at 0x00014090df71. Too many branches
                    // WARNING: Treating indirect jump as call
    (*UNRECOVERED_JUMPTABLE_00)
              (*(undefined8 *)(in_RDX + 0x30),"PNGcrates/country-core/src/resources/gpu_lut.rs",3);
    return;
  }
                    // WARNING: Could not recover jumptable at 0x00014090df81. Too many branches
                    // WARNING: Treating indirect jump as call
  (*UNRECOVERED_JUMPTABLE_00)(*(undefined8 *)(in_RDX + 0x30),&DAT_142aab678,4);
  return;
}

