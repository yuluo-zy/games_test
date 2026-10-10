
float tg_country_core_resources_roofs_roof_ridge_calc_roof_tip_center_ls
                (longlong param_1,float param_2)

{
  float fVar1;
  
  if (*(char *)(param_1 + 8) == '\0') {
    fVar1 = *(float *)(param_1 + 0x14) + *(float *)(param_1 + 0x14);
  }
  else {
    fVar1 = *(float *)(param_1 + 0x1c);
  }
  return fVar1 * *(float *)(param_1 + 0x24) * param_2;
}

