// ADDR 0x140822cf0
// SIG void __cdecl place_sound(void)
// ORIG system::place_sound


// WARNING: Instruction at (ram,0x000140822dbc) overlaps instruction at (ram,0x000140822dbb)
// 

void __cdecl system_decorator::audio::op_confirm_audio::place_sound(void)

{
  char extraout_AL;
  ulonglong uVar1;
  ulonglong uVar2;
  undefined1 *in_RCX;
  char in_DL;
  undefined4 unaff_EBX;
  undefined4 unaff_0000001c;
  int *unaff_RDI;
  undefined1 in_R8B;
  ulonglong uVar3;
  char in_R9B;
  
  uVar1 = 6;
  if ((byte)(in_DL - 0x38U) < 8) {
    uVar1 = (ulonglong)(byte)(in_DL - 0x38U);
  }
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(uVar1) {
  case 0:
  case 1:
  case 2:
    uVar2 = 1;
    if ((char)uVar1 != '\x01') {
      uVar2 = (ulonglong)((uint)((char)uVar1 == '\x02') * 2);
    }
    in_RCX[2] = in_R9B == '\x02';
    *(ulonglong *)(in_RCX + 8) = uVar2;
    in_RCX[1] = in_R8B;
    *in_RCX = 1;
    break;
  case 3:
    country_core::resources::walls::decorator_subtype::impl$8::from();
    country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype();
    if (2 < (byte)(extraout_AL - 0x13U)) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    in_RCX[1] = (char)(0x100 >> ((extraout_AL - 0x13U & 3) << 3));
    *in_RCX = 0x11;
    break;
  case 4:
    in_RCX[1] = in_R9B == '\x03';
    *in_RCX = 0xb;
    break;
  case 5:
    uVar1 = (ulonglong)(byte)(in_R9B - 2);
    uVar2 = uVar1 + 1;
    goto code_r0x000140822d96;
  case 6:
    in_RCX[1] = in_DL;
    *in_RCX = 0x23;
                    // WARNING: This code block may not be properly labeled as switch case
    break;
  case 7:
    break;
  default:
                    // WARNING: This code block may not be properly labeled as switch case
    uVar2 = (ulonglong)((int)uVar1 + 1);
code_r0x000140822d96:
    uVar3 = 0;
    if ((byte)uVar1 < 3) {
      uVar3 = uVar2;
    }
                    // WARNING: Could not recover jumptable at 0x000140822dad. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&UNK_142a910c0 + *(int *)(&UNK_142a910c0 + uVar3 * 4)))
              (in_RCX,&UNK_142a910c0 + *(int *)(&UNK_142a910c0 + uVar3 * 4));
    return;
  case 9:
    *(int *)CONCAT44(unaff_0000001c,unaff_EBX) =
         *(int *)CONCAT44(unaff_0000001c,unaff_EBX) + (int)in_RCX + *unaff_RDI;
  }
  return;
}


