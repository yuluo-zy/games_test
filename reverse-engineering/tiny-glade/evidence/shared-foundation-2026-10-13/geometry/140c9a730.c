utils::geometry::rectangle::Rectangle2d::with_expanded_size

undefined8 * tg_math_140c9a730(undefined8 *param_1,undefined8 *param_2,float param_3,float param_4)

{
  undefined8 uVar1;
  
  param_1[2] = CONCAT44(param_4 + (float)((ulonglong)param_2[2] >> 0x20),param_3 + (float)param_2[2]
                       );
  uVar1 = param_2[1];
  *param_1 = *param_2;
  param_1[1] = uVar1;
  return param_1;
}

