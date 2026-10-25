// parse_rgb  0x140b01cc0
// signature: void __cdecl parse_rgb(void)
// callees: [{"name": "format_err", "address": "1428a8020"}, {"name": "from<core::num::dec2flt::ParseFloatError>", "address": "1428a9620"}, {"name": "from_str", "address": "140d8ba40"}, {"name": "next", "address": "140b00920"}]


void __cdecl country_core::resources::grading_lut::impl$2::load_cube::parse_rgb(void)

{
  longlong extraout_RAX;
  ulonglong extraout_RAX_00;
  longlong extraout_RAX_01;
  ulonglong extraout_RAX_02;
  longlong extraout_RAX_03;
  ulonglong extraout_RAX_04;
  undefined8 extraout_RAX_05;
  undefined8 extraout_RAX_06;
  undefined8 uVar1;
  undefined4 *in_RCX;
  
  core::str::iter::impl$37::next();
  if (extraout_RAX == 0) {
LAB_140b01d84:
    anyhow::__private::format_err();
    uVar1 = extraout_RAX_05;
  }
  else {
    core::num::dec2flt::impl$2::from_str();
    if ((extraout_RAX_00 & 1) == 0) {
      core::str::iter::impl$37::next();
      if (extraout_RAX_01 == 0) goto LAB_140b01d84;
      core::num::dec2flt::impl$2::from_str();
      if ((extraout_RAX_02 & 1) == 0) {
        core::str::iter::impl$37::next();
        if (extraout_RAX_03 == 0) goto LAB_140b01d84;
        core::num::dec2flt::impl$2::from_str();
        if ((extraout_RAX_04 & 1) == 0) {
          in_RCX[1] = (int)(extraout_RAX_00 >> 0x20);
          in_RCX[2] = (int)(extraout_RAX_02 >> 0x20);
          in_RCX[3] = (int)(extraout_RAX_04 >> 0x20);
          *in_RCX = 0;
          return;
        }
      }
    }
    anyhow::error::impl$1::from<core::num::dec2flt::ParseFloatError>();
    uVar1 = extraout_RAX_06;
  }
  *(undefined8 *)(in_RCX + 2) = uVar1;
  *in_RCX = 1;
  return;
}

