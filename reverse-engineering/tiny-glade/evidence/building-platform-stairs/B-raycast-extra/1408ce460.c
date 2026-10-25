
void __cdecl country_core::systems::collision::world::Rapier::on_collider_upserted(void)

{
  tabloid::Publisher::
  publish<country_core::systems::collision::world::ColliderDiff,enum2_<country_core::systems::collision::world::ColliderDelta>_>
            ();
  return;
}

