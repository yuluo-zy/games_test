//! B ShaderRefactoringState, shared by the renderer and game-time gate.
//!
//! The resource TypeId is confirmed by named insert_resource @0x14152fac0
//! and register_resource_with @0x1402157b0. resources::insert_resources
//! @0x1409410ea zeros EDX before tail-calling the insertion: disc=0.
//! The existing render consumer already owns the two observed u32 fields;
//! re-export that same type rather than creating a second ECS resource.
pub use crate::systems::render_loop::compute_passes::shader_refactoring::RefactoringState as ShaderRefactoringState;

impl bevy_ecs::resource::Resource for ShaderRefactoringState {}

impl ShaderRefactoringState {
    /// B @0x140b1f210: cmp dword [rcx],0; sete al; ret.
    pub fn disabled(&self) -> bool {
        self.disc == 0
    }
}
