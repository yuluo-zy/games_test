utils::geometry::rectangle::Rectangle3d::translate

void tg_math_140c984c0(longlong param_1,float *param_2)

{
  *(ulonglong *)(param_1 + 8) =
       CONCAT44((float)((ulonglong)*(undefined8 *)(param_1 + 8) >> 0x20) + param_2[2],
                (float)*(undefined8 *)(param_1 + 8) + *param_2);
  *(ulonglong *)(param_1 + 0x18) =
       CONCAT44(param_2[1] + (float)((ulonglong)*(undefined8 *)(param_1 + 0x18) >> 0x20),
                param_2[1] + (float)*(undefined8 *)(param_1 + 0x18));
  return;
}

