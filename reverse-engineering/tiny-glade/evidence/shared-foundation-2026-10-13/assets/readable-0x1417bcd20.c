
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
float asset_pipe__texture__srgb_oetf(float param_1)
{
  float fVar1;
  if (_DAT_142a10e28 < param_1) {
    fVar1 = (float)func_0x000142923f70(param_1,_DAT_142a10e2c);
    return fVar1 * _DAT_142a10e30 + _DAT_142a10e34;
  }
  return param_1 * _DAT_142a10e38;
}
