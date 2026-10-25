
void __cdecl system_decorator::decorator_visual::stairs_assemble::determine_ladder_support(void)

{
  float *pfVar1;
  float fVar2;
  char cVar3;
  bool bVar4;
  longlong lVar5;
  undefined1 in_CL;
  undefined7 in_register_00000009;
  longlong in_RDX;
  longlong lVar6;
  longlong in_R8;
  undefined1 in_R9B;
  undefined7 in_register_00000089;
  char *pcVar7;
  bool bVar8;
  
  fVar2 = *(float *)(in_RDX + 4);
  pfVar1 = (float *)(CONCAT71(in_register_00000009,in_CL) + 4);
  bVar8 = fVar2 == *pfVar1;
  bVar4 = fVar2 < *pfVar1;
  pcVar7 = (char *)(CONCAT71(in_register_00000089,in_R9B) + 0x14);
  if (!bVar4 && !bVar8) {
    pcVar7 = (char *)CONCAT71(in_register_00000089,in_R9B);
  }
  lVar6 = (ulonglong)(bVar4 || bVar8) * 0x20;
  lVar5 = (ulonglong)((!NAN(fVar2) && !NAN(*pfVar1)) && (!bVar4 && !bVar8)) * 0x20;
  bVar4 = true;
  bVar8 = true;
  if (*(char *)(in_R8 + 0x18 + lVar6) == '\x01') {
    if (1 < *(uint *)(in_R8 + lVar6)) {
      bVar8 = *pcVar7 == '\0';
      cVar3 = *(char *)(in_R8 + 0x18 + lVar5);
      goto joined_r0x0001407de877;
    }
    bVar8 = *(float *)(in_R8 + 4 + lVar6) <= 0.13;
  }
  cVar3 = *(char *)(in_R8 + 0x18 + lVar5);
joined_r0x0001407de877:
  if ((cVar3 == '\x01') && (*(uint *)(in_R8 + lVar5) < 2)) {
    bVar4 = *(float *)(in_R8 + 4 + lVar5) <= 0.13;
  }
  if (!bVar8) {
    return;
  }
  if (bVar4) {
    return;
  }
  if ((*(uint *)(in_R8 + lVar6) < 2) && (*(float *)(in_R8 + 4 + lVar6) <= 0.13)) {
    return;
  }
  return;
}

