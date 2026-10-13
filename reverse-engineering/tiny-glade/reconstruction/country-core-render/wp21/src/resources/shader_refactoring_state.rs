//! shader_refactoring_state.rs — 着色器重构状态（WP21）。
//!
//! O：`ShaderRefactoringState::disabled`（0xb1f210）。

use bevy_ecs::resource::Resource;

/// `ShaderRefactoringState`（O：重构开关状态）。
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct ShaderRefactoringState {
    pub enabled: bool,
}

impl Default for ShaderRefactoringState {
    fn default() -> Self {
        Self { enabled: false }
    }
}

impl ShaderRefactoringState {
    /// `disabled`（O 0xb1f210）。
    pub fn disabled() -> Self {
        Self { enabled: false }
    }
}
