//! What the render thread reads (reports/render-thread-2026-10-22).
//!
//! Declaration runs on the render thread against its own `World`. Every module
//! that declares from the World lists the components and resources it reads in
//! a `render_extract` function, using the same type names as its accessors; the
//! host's extractor mirrors exactly those into the render World each frame.
use bevy_ecs::{component::Component, event::Event, resource::Resource};
use bevy_state::state::States;

/// In the render world: the game world has an `R` (declaration checks that a
/// producer is installed without reading it).
#[derive(Resource)]
pub struct Present<R: Send + Sync + 'static>(core::marker::PhantomData<fn() -> R>);
impl<R: Send + Sync + 'static> Default for Present<R> {
    fn default() -> Self {
        Self(core::marker::PhantomData)
    }
}

/// What a partial write-back applies to the game-side resource.
pub type WriteBack<R> = Box<dyn FnOnce(&mut R) + Send>;

pub trait RenderExtract {
    /// Mirror component `C` of every game entity onto the same entity id.
    fn component<C: Component + Clone>(&mut self);
    /// Copy resource `R` whenever it changed in the game world.
    fn resource<R: Resource + Clone>(&mut self);
    /// Copy this frame's pending events of `E`.
    fn events<E: Event + Clone>(&mut self);
    /// Copy the current value of state `S`.
    fn state<S: States>(&mut self);
    /// Mirror only whether `R` exists, as `Present<R>`.
    fn presence<R: Resource>(&mut self);
    /// Declaration writes `R` and the game reads it: whenever the render copy
    /// changes during a frame it is written back to the game world at the
    /// next hand-over, unless the game changed `R` itself since the extract.
    /// A render-side removal is written back too.
    fn write_back<R: Resource + Clone>(&mut self);
    /// A partial write-back: `take` reads the render copy after the frame
    /// and returns what to apply to the game's `R` (game-side changes since the
    /// extract win when `guarded`).
    fn write_back_with<R: Resource>(&mut self, guarded: bool, take: fn(&R) -> Option<WriteBack<R>>);
    /// Events of `E` that declaration sends are forwarded to the game world.
    fn forward_events<E: Event>(&mut self);
    /// `R` belongs to the render side: whenever the game world (re)creates it,
    /// it moves to the render world.
    fn render_owned<R: Resource>(&mut self);
}
