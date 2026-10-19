//! The executable main-wall producer lives with the authoritative wall/cache,
//! hole, trigger and RT owners in country-core. Reexport that one system here;
//! a second wrapper that guessed height from a 2D point coordinate and dropped
//! its records would corrupt the producer/consumer contract.
//!
//! Original B owner140757800 also emits corners/skirts/signifiers. Those
//! independent geometry consumers remain explicit until their source wrappers
//! are complete; reexporting the main producer does not claim that extra work.
pub use country_core::systems::wall::construct_wall_bricks::construct_wall_bricks as construct_walls;
