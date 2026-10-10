
float tg_country_core_resources_roofs_roof_shape_Roof_shape_dims(longlong param_1)

{
  if (*(char *)(param_1 + 8) == '\0') {
    return *(float *)(param_1 + 0x14) + *(float *)(param_1 + 0x14);
  }
  return *(float *)(param_1 + 0x1c);
}

