
void __cdecl utils::geometry::aabb::Aabb2::intersects(void)

{
  float *in_RCX;
  float *in_RDX;
  
  if (in_RCX[2] < *in_RDX) {
    return;
  }
  if (in_RDX[2] < *in_RCX) {
    return;
  }
  return;
}

