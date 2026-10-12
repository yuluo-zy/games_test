
void __cdecl country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2(void)

{
  byte *in_RDX;
  
  if ((*in_RDX & 1) == 0) {
    ::utils::geometry::circle::Circle2d::as_curve2();
  }
  else {
    ::utils::geometry::rectangle::Rectangle2d::as_curve2();
  }
  return;
}

