
void __cdecl utils::swizzlers::impl$0::x0y(void)

{
  undefined4 uVar1;
  undefined4 *in_RCX;
  undefined4 *in_RDX;
  
  uVar1 = in_RDX[1];
  *in_RCX = *in_RDX;
  in_RCX[1] = 0;
  in_RCX[2] = uVar1;
  return;
}

