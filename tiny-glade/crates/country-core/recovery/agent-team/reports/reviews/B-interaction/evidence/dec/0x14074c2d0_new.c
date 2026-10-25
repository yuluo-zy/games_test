
void __cdecl system_wall_constructor::arch_walker::ArchWalker::new(void)

{
  undefined4 *in_RCX;
  undefined4 in_R8D;
  undefined4 in_XMM1_Da;
  undefined4 in_XMM3_Da;
  
  in_RCX[2] = in_XMM1_Da;
  in_RCX[3] = 0x3dcccccd;
  *in_RCX = in_R8D;
  in_RCX[1] = in_XMM3_Da;
  return;
}

