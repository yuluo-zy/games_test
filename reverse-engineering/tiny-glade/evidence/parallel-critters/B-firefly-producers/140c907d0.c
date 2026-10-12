
void __cdecl utils::uniform_sample_sphere(void)

{
  undefined8 *in_RCX;
  undefined8 in_RDX;
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  
  fVar4 = __real_3f800000 - ((float)in_RDX + (float)in_RDX);
  fVar2 = __real_3f800000 - fVar4 * fVar4;
  if (fVar2 <= 0.0) {
    fVar2 = 0.0;
  }
  fVar3 = (float)((ulonglong)in_RDX >> 0x20) * __real_40c90fdb;
  fVar1 = sinf(fVar3);
  fVar3 = cosf(fVar3);
  *in_RCX = CONCAT44(SQRT(fVar2) * fVar1,SQRT(fVar2) * fVar3);
  *(float *)(in_RCX + 1) = fVar4;
  return;
}

