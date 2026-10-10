
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8
tg_country_core_resources_roofs_roof_shape_RoofShapeParams_eave_length_ws(longlong param_1)

{
  float fVar1;
  
  fVar1 = SQRT(*(float *)(param_1 + 8)) +
          (_UNK_14292e6b4 - SQRT(*(float *)(param_1 + 8))) * _UNK_142a80ce4;
  return CONCAT44(fVar1,fVar1 * (_DAT_1429258d0 * *(float *)(param_1 + 0x14) +
                                (_DAT_14292e6b0 - *(float *)(param_1 + 0x14)) * _DAT_142a80ce0));
}

