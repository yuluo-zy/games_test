
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong tg_country_core_resources_roofs_roof_ridge_RoofRidgeDims_calc
                    (float param_1,char param_2,float param_3,float param_4)

{
  float fVar1;
  
  fVar1 = (_DAT_142925904 - param_1) * _DAT_142925984;
  if (param_2 != '\0') {
    return (ulonglong)(uint)(fVar1 + param_1 * param_4) << 0x20 | 0x3dcccccd;
  }
  return (ulonglong)(uint)(fVar1 + param_1 * param_3) | 0x3dcccccd00000000;
}

