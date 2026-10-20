//! `decorator_resources.rs` — 装饰域共享小资源/手柄名。
//!
//! PDB 模块 1499 / 3068（B 版）实际内容只有两部分：
//! 1. 命名实体 `system_decorator::decorator_resources::DecoratorHandleName`
//!    （IPI 引用 3；`UiState::add<DecoratorHandleName>` 泛型关系，见骨架
//!    `decorator_resources.rs.structures.json.gz`）；
//! 2. 唯一函数符号 `impl$0::to_widget_name_hash`（inline，源码行 36；宿主
//!    `UiState::add<DecoratorHandleName>` / `create_decorator_widget<DecoratorHandleName>`）。
//!
//! # 归属（SOURCE_MAP 分歧登记）
//!
//! systems SOURCE_MAP 曾把 `entities/add_decorator_visual_entity/seal_persistent_entities`
//! 归入本文件；骨架 PDB 模块 1499/3068 将视觉实体状态归属
//! `decorator_visual_state.rs`（行号一致）。本批次按 PDB 归属，视觉状态留在
//! `decorator_visual_state.rs`；本文件只承载 PDB 明确命名的手柄名类型与其哈希。
//!
//! # `DecoratorHandleName` 的类型身份
//!
//! 该类型当前定义在 `ui_systems/ui_decorator_affordance_dispatch.rs`（UI 族恢复时
//! 落地）。PDB 的权威定义模块是本文件，因此这里以 **re-export** 暴露同一类型
//! 身份（`crate::decorator_resources::DecoratorHandleName` == UI 模块同一类型），
//! 不复制定义、不产生两个不兼容的句柄名。
//!
//! # `to_widget_name_hash` 算法（O）
//!
//! 宿主为 `UiState::add<DecoratorHandleName>` / `create_decorator_widget<
//! DecoratorHandleName>`（反编译 0x14125d180 / 0x141282300）。widget 名 → id 的
//! 哈希在 core 已有权威实现：`WidgetId::from_name`（B 0x1405bdda0，零键
//! SipHasher13 + str Hash 终止符，见 `country-core/src/systems/ui/resources/ui_state.rs`）。
//! 旧实现使用 `utils::name_hash::NameHash`（无种子 xxh3）与宿主不符，现按
//! O 证据改为 `WidgetId::from_name`。
//!
//! 方法面：`impl$0` = `impl DecoratorHandleName`，故提供同名方法；
//! 自由函数保留为既有调用面（无跨文件消费者，登记于交付）。

pub use crate::ui_systems::ui_decorator_affordance_dispatch::DecoratorHandleName;
use country_core::systems::ui::resources::ui_state::WidgetId;

/// `impl$0::to_widget_name_hash`（inline，行 36）：手柄名 → widget id 哈希。
///
/// O：与 `WidgetId::from_name` 0x1405bdda0 同源（零键 SipHash13）；返回裸 u64。
impl DecoratorHandleName {
    /// 手柄名哈希（= `WidgetId::from_name(name).0`）。
    #[inline]
    pub fn to_widget_name_hash(self) -> u64 {
        to_widget_name_hash(self.0)
    }
}

/// `to_widget_name_hash`（行 36 内联的自由函数面）：widget 名 → u64。
///
/// O：宿主 `UiState::add<DecoratorHandleName>` 使用与 `WidgetId::from_name`
/// 相同的零键 SipHasher13。
#[inline]
pub fn to_widget_name_hash(name: &str) -> u64 {
    WidgetId::from_name(name).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widget_name_hash_matches_core_widget_id() {
        for name in ["ui_decide_decorator_interaction", "", "decorator-handle"] {
            assert_eq!(
                to_widget_name_hash(name),
                WidgetId::from_name(name).0,
                "name={name}"
            );
        }
    }

    #[test]
    fn handle_name_method_matches_free_function() {
        let handle = DecoratorHandleName("ui_decide_decorator_interaction");
        assert_eq!(handle.to_widget_name_hash(), to_widget_name_hash(handle.0));
        assert_eq!(handle.to_widget_id().0, handle.to_widget_name_hash());
    }

    #[test]
    fn hash_is_stable_across_calls_and_distinct_across_names() {
        let a = to_widget_name_hash("decorator_a");
        let b = to_widget_name_hash("decorator_b");
        assert_eq!(a, to_widget_name_hash("decorator_a"));
        assert_ne!(a, b);
    }
}
