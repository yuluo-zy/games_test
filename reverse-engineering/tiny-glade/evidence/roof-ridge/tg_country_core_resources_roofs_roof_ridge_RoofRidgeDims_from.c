
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong tg_country_core_resources_roofs_roof_ridge_RoofRidgeDims_from(longlong param_1)

{
  float fVar1;
  float fVar2;
  
  if (*(int *)(param_1 + 8) != 1) {
    return 0x3dcccccd3dcccccd;
  }
  fVar1 = *(float *)(param_1 + 0x34);
  fVar2 = (_DAT_142925904 - fVar1) * _DAT_142925984;
  if (*(char *)(param_1 + 0x3c) != '\0') {
    return (ulonglong)(uint)(fVar2 + fVar1 * *(float *)(param_1 + 0x20)) << 0x20 | 0x3dcccccd;
  }
  return (ulonglong)(uint)(fVar2 + fVar1 * *(float *)(param_1 + 0x1c)) | 0x3dcccccd00000000;
}

