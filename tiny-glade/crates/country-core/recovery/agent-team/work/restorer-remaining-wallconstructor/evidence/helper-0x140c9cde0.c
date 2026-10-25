
void __cdecl utils::swizzlers::impl$1::xvy(void)

{
  undefined4 uVar1;
  undefined4 *in_RCX;
  undefined4 *in_RDX;
  undefined4 in_XMM2_Da;
  
  uVar1 = in_RDX[1];
  *in_RCX = *in_RDX;
  in_RCX[1] = in_XMM2_Da;
  in_RCX[2] = uVar1;
  return;
}

