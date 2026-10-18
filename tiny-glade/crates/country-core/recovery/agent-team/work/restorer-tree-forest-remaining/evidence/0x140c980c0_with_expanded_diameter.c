// with_expanded_diameter 0x140c980c0 size=40
// signature: void __cdecl with_expanded_diameter(void)


void __cdecl utils::geometry::circle::Circle2d::with_expanded_diameter(void)

{
  float fVar1;
  undefined4 uVar2;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  float in_XMM2_Da;
  
  fVar1 = *(float *)(in_RDX + 1);
  uVar2 = *(undefined4 *)((longlong)in_RDX + 0xc);
  *in_RCX = *in_RDX;
  *(float *)(in_RCX + 1) = in_XMM2_Da * 0.5 + fVar1;
  *(undefined4 *)((longlong)in_RCX + 0xc) = uVar2;
  return;
}

