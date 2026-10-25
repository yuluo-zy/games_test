utils::geometry::aabb::is_inside_bbx

byte tg_math_14123ffc0(float param_1,float param_2,float param_3,float param_4,float param_5,
                      float param_6)

{
  return (param_6 < param_4 && param_5 < param_3) & -(param_1 < param_5) & -(param_2 < param_6);
}

