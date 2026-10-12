
void __cdecl utils::geometry::circle::Circle2d::get_normal_at_u(void)

{
  float fVar1;
  longlong in_RCX;
  float fVar2;
  float in_XMM1_Da;
  float fVar3;
  
  fVar1 = __real_40c90fdb;
  fVar3 = (__real_3f800000 - in_XMM1_Da) * __real_40c90fdb;
  fVar2 = fmodf(*(float *)(in_RCX + 0xc),__real_40c90fdb);
  fVar3 = (float)(~-(uint)(fVar2 < 0.0) & (uint)fVar2 | (uint)(fVar1 + fVar2) & -(uint)(fVar2 < 0.0)
                 ) + fVar3;
  cosf(fVar3);
  sinf(fVar3);
  return;
}

