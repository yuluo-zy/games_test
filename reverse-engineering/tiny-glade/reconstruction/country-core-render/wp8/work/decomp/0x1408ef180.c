
void __cdecl
country_core::resources::mouse_terrain_raycast::CameraCursorRay::intersect_with_xz_plane(void)

{
  float fVar1;
  float fVar2;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  float fVar3;
  float fVar4;
  float in_XMM2_Da;
  float fVar5;
  
  fVar3 = (float)((ulonglong)*in_RDX >> 0x20);
  fVar4 = (float)((ulonglong)*(undefined8 *)((longlong)in_RDX + 0xc) >> 0x20);
  fVar5 = (in_XMM2_Da - fVar3) / fVar4;
  fVar1 = *(float *)((longlong)in_RDX + 0x14);
  fVar2 = *(float *)(in_RDX + 1);
  *in_RCX = CONCAT44(fVar5 * fVar4 + fVar3,
                     fVar5 * (float)*(undefined8 *)((longlong)in_RDX + 0xc) + (float)*in_RDX);
  *(float *)(in_RCX + 1) = fVar5 * fVar1 + fVar2;
  return;
}


