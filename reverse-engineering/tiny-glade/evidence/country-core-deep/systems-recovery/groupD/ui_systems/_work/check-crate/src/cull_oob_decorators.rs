//! B `cull_oob_decorators.rs` — 越界裁剪（D2 消费契约；主体业务归 D-owner）。
//!
//! `should_snap_to_corner`（lib.rs 符号；calculate_decorator_interaction_intent
//! 0x14127cf70 消费）：装饰类型是否参与四角吸附（I：与 DecoratorType
//! `should_offer_snap_to_corner` 0x1408331c0 `t != 0x3f` 语义一致，D2 以 core
//! 谓词承载）。
use country_core::resources::walls::decorator::DecoratorType;

/// 是否应吸附到角（I：core 谓词 0x1408331c0 语义）。
pub fn should_snap_to_corner(ty: DecoratorType) -> bool {
    ty.should_offer_snap_to_corner()
}
