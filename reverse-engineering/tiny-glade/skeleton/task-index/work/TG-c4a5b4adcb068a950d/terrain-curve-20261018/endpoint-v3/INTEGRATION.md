# Parent integration

Apply after continuation29 plus remove-before-color order-v3 and the independently owned Delete restored-height schema. This patch changes only ui_continue_wall.rs and draw_wall.rs. It preserves full-width color constructors. Do not replace shared registration or publisher sources with old snapshots.

Required shared adapters prepared by root owners:

- existing WidgetVolumeShape::sphere(position:Vec3,radius:f32), retained center:Vec3; sphere enum byte1 is padding.
- PublicWallState::is_being_bulldozed() from publication-time AnimateWallBulldozeBuffer membership. Its actual publisher owner must provide this flag; reading the newer buffer at UI time changes behavior.

Initialize WallContinueState, retain installed SuppressContinueWall / ContinueWallOriginalState / PendingWallHistory and events. Install suppress_continue_wall before ui_continue_wall; run the existing endpoint UI before draw_wall, then finish_wall, history_record_wall_changed, record_wall_history and actual input-recording completion. UI automatic history start must see the endpoint widget's record_history_when_used flag in the same interaction frame; verify root's actual history lifecycle order rather than relying only on these tests' prestarted transaction. No new event identities are needed.

Run the new endpoint tests and existing draw, finish, deferred/color/replay regressions. Native target execution and Rust test execution have not been claimed by this candidate author. Full snap merge and the UI signifier renderer remain explicit next work.
