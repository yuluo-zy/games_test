//! Original component descriptor140b1d90e: size8,alignment8.
//! Startup emits index1/2; no replacement empty replay consumer is registered.
#[derive(bevy_ecs::component::Component,Debug,PartialEq,Eq)]
pub struct SignifierReplayIndex(pub usize);
