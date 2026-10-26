//! 原 B Tweaker：输入阶段、步长取整/限幅、布尔/枚举/列表边界、提示确认和真实控件建树。
//! 字体、容器、枚举值和格式化复用现有 Rust owner，不复制原私有ABI。
use super::{DefaultNodeContent, LabelContent, MenuButtonContent};
use crate::mei::{
    core::{GlobalId, NodeContent, NodeContext, NodePrepareContext, calculate_hash},
    layout::{LengthPercentage, horizontal},
};
use country_core::input::action_set::ActionSetAction;

#[derive(Default)]
pub struct TweakerContent {
    pub delta: i32,
}
impl DefaultNodeContent for TweakerContent {
    fn from_font_table(_: std::sync::Arc<tg_text::FontTable>) -> Self {
        Self::default()
    }
}
impl NodeContent for TweakerContent {
    fn type_name(&self) -> &'static str {
        "toolbar::mei::widgets::tweaker::TweakerContent"
    }
    fn interactive(&self) -> bool {
        true
    }
    fn on_input_phase(&mut self, action: &ActionSetAction, phase: [bool; 2]) -> bool {
        // 原67d9c0：仅回程处理；不额外以phase[1]屏蔽原可达分支。
        if !phase[0] {
            return false;
        }
        let delta = if action.just_pressed(20) {
            -1
        } else if action.just_pressed(21) {
            1
        } else {
            return false;
        };
        self.delta = self.delta.wrapping_add(delta);
        true
    }
    fn prepare_layout(&mut self, _: &mut NodePrepareContext<'_>) -> Option<[LengthPercentage; 2]> {
        self.delta = 0;
        Some([LengthPercentage::absolute(0.); 2])
    }
}

pub trait Tweakable {
    fn decrease(&mut self);
    fn increase(&mut self);
    fn format(&self) -> String;
    fn apply_delta(&mut self, delta: i32) {
        if delta < 0 {
            for _ in delta..0 {
                self.decrease();
            }
        } else {
            for _ in 0..delta {
                self.increase();
            }
        }
    }
}

pub struct TweakableFloatWithStep<'a> {
    pub value: &'a mut f32,
    pub step: f32,
    pub min: f32,
    pub max: f32,
}
impl Tweakable for TweakableFloatWithStep<'_> {
    fn decrease(&mut self) {
        // 必须先减step再除；原版先写取整结果，非法上下限随后panic。
        *self.value = ((*self.value - self.step) / self.step).round() * self.step;
        *self.value = self.value.clamp(self.min, self.max);
    }
    fn increase(&mut self) {
        *self.value = ((*self.value + self.step) / self.step).round() * self.step;
        *self.value = self.value.clamp(self.min, self.max);
    }
    fn format(&self) -> String {
        format!("{:.*}", if self.step >= 0.1 { 1 } else { 2 }, *self.value)
    }
}

pub struct TweakableBool<'a>(pub &'a mut bool);
impl Tweakable for TweakableBool<'_> {
    fn decrease(&mut self) {
        *self.0 = false;
    }
    fn increase(&mut self) {
        *self.0 = true;
    }
    fn format(&self) -> String {
        lang_lib::lookup(if *self.0 { "Tweaker.Yes" } else { "Tweaker.No" }).to_string()
    }
    fn apply_delta(&mut self, delta: i32) {
        if delta != 0 {
            *self.0 = delta > 0;
        }
    }
}

/// 原Enum的previous/next在首尾返回None；由当前枚举的原声明顺序提供variants。
/// formatter复用该类型现有的Display/本地化实现，避免另造配置枚举。
pub struct TweakableEnum<'a, T> {
    pub value: &'a mut T,
    pub variants: &'a [T],
    pub formatter: fn(&T) -> String,
}
impl<T: Copy + PartialEq> Tweakable for TweakableEnum<'_, T> {
    fn decrease(&mut self) {
        let index = self
            .variants
            .iter()
            .position(|x| x == self.value)
            .expect("枚举值不在原变体表中");
        if index > 0 {
            *self.value = self.variants[index - 1];
        }
    }
    fn increase(&mut self) {
        let index = self
            .variants
            .iter()
            .position(|x| x == self.value)
            .expect("枚举值不在原变体表中");
        if let Some(value) = self.variants.get(index + 1) {
            *self.value = *value;
        }
    }
    fn format(&self) -> String {
        (self.formatter)(self.value)
    }
    fn apply_delta(&mut self, delta: i32) {
        if delta == 0 {
            return;
        }
        let index = self
            .variants
            .iter()
            .position(|x| x == self.value)
            .expect("枚举值不在原变体表中");
        let next = if delta < 0 {
            index.saturating_sub(delta.unsigned_abs() as usize)
        } else {
            index
                .saturating_add(delta as usize)
                .min(self.variants.len() - 1)
        };
        *self.value = self.variants[next];
    }
}

pub struct TweakableList<'a, T> {
    pub index: &'a mut usize,
    pub values: &'a [T],
    pub formatter: fn(&T) -> String,
}
impl<T> Tweakable for TweakableList<'_, T> {
    fn decrease(&mut self) {
        *self.index = self.index.saturating_sub(1);
    }
    fn increase(&mut self) {
        *self.index = self
            .index
            .wrapping_add(1)
            .min(self.values.len().wrapping_sub(1));
    }
    fn format(&self) -> String {
        (self.formatter)(&self.values[*self.index])
    }
    fn apply_delta(&mut self, delta: i32) {
        if delta < 0 {
            *self.index = self.index.saturating_sub(delta.unsigned_abs() as usize);
        } else if delta > 0 {
            // 原单态将重复饱和更新展开；用等价闭式保留空表/溢出边界，避免巨大delta线性循环。
            let max = self.values.len().wrapping_sub(1);
            *self.index = if self.values.is_empty() {
                self.index.wrapping_add(delta as usize)
            } else {
                self.index
                    .wrapping_add(1)
                    .min(max)
                    .saturating_add(delta as usize - 1)
                    .min(max)
            };
        }
    }
}

#[derive(Default)]
pub struct HintTracker {
    pub current: String,
}
impl HintTracker {
    pub fn text<'a>(&'a mut self, text: &'a str) -> MaybeHint<'a> {
        MaybeHint {
            target: &mut self.current,
            text,
        }
    }
}
pub struct MaybeHint<'a> {
    target: &'a mut String,
    text: &'a str,
}
impl MaybeHint<'_> {
    /// 原67dd00：只在确认hover的分支清空并复制；无hover时保留上次说明。
    pub fn confirm(&mut self) {
        self.target.clear();
        self.target.push_str(self.text);
    }
}

impl NodeContext {
    pub fn tweaker_delta(&self) -> i32 {
        self.content_mut::<TweakerContent>().delta
    }
    fn tweaker_impl(&self, name: &str) -> Self {
        let local = calculate_hash(&(calculate_hash(&(name, ())), 0x175517bc5c12b9f6u64));
        let row = self.child_with_global_id(GlobalId(calculate_hash(&(self.id.0, local))));
        row.get_content_mut_impl::<TweakerContent>(false);
        row.node_mut().with_nav(true);
        row.layout(horizontal().space_between());
        row
    }
    /// 原默认字号48，宽度调用方默认260（67dd70），两者在UI里乘0.6。
    pub fn tweaker<T: Tweakable>(
        &self,
        name: &str,
        hint: MaybeHint<'_>,
        width: f32,
        value: T,
    ) -> bool {
        self.tweaker_w_font_size(name, hint, width, 48., value)
    }
    pub fn tweaker_w_font_size<T: Tweakable>(
        &self,
        name: &str,
        mut hint: MaybeHint<'_>,
        width: f32,
        font_size: f32,
        mut value: T,
    ) -> bool {
        let row = self.tweaker_impl(name);
        let delta = row.tweaker_delta();
        let hovered = row.node().hover();
        if hovered {
            hint.confirm();
        }
        let color = if hovered { u32::MAX } else { 0xffffffb0 };
        let size = font_size * 0.6;
        let label = row.label(name);
        let content = label.content_mut::<LabelContent>();
        content.text.set_metrics(size).expect("原Tweaker字号");
        content
            .text
            .set_family(tg_text::FontFamily::MenuButton)
            .expect("原Tweaker字体");
        content
            .text
            .set_max_width(700. * 0.6)
            .expect("原Tweaker标签限宽");
        content.text.set_weight(tg_text::Weight(700));
        content.color = color;
        let local = calculate_hash(&(calculate_hash(&0u64), 0xa79b7268a2a968d9u64));
        let controls = row.child_with_global_id(GlobalId(calculate_hash(&(row.id.0, local))));
        controls.layout(horizontal());
        let mut changed = delta != 0;
        value.apply_delta(delta);
        let left = controls.menu_button_no_nav("  <  ");
        left.node_mut().sound_click = 5;
        left.content_mut::<MenuButtonContent>()
            .text
            .set_metrics(size)
            .expect("原减小按钮字号");
        if left.node().used() {
            value.decrease();
            changed = true;
        }
        // 原版在右箭头的副作用之前创建value标签，不能在increase后重排刷新。
        let label = controls.label(&value.format());
        let content = label.content_mut::<LabelContent>();
        content.text.set_metrics(size).expect("原Tweaker值字号");
        content
            .text
            .set_family(tg_text::FontFamily::MenuButton)
            .expect("原Tweaker值字体");
        content.text.set_align(tg_text::Align::Center);
        content.color = color;
        // 原NodeContext::layout把style复制到Node+20；Node+48即style.width。
        label.node_mut().style.size = LengthPercentage::absolute(width * 0.6);
        let right = controls.menu_button_no_nav("  >  ");
        right.node_mut().sound_click = 5;
        right
            .content_mut::<MenuButtonContent>()
            .text
            .set_metrics(size)
            .expect("原增大按钮字号");
        if right.node().used() {
            value.increase();
            changed = true;
        }
        changed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore = "需TG_INSTALL_ROOT原字体表"]
    fn original_control_tree_retains_delta_and_click_order_without_navigation_arrows() {
        use crate::mei::core::{MeiContext, NodeFlags};
        let root_path =
            std::path::PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").expect("原资产路径"));
        let mut mei = MeiContext::default();
        mei.load_font_table(&root_path.join("assets/fonts/table.ron"))
            .unwrap();
        let root = mei.fullscreen("settings test");
        let mut hint = HintTracker::default();
        let mut value = 0.5;
        assert!(!root.tweaker(
            "Volume",
            hint.text("音量说明"),
            260.,
            TweakableFloatWithStep {
                value: &mut value,
                step: 0.1,
                min: 0.,
                max: 2.
            }
        ));
        let row = *mei
            .nodes
            .iter()
            .find(|(_, n)| {
                n.content.type_name() == "toolbar::mei::widgets::tweaker::TweakerContent"
            })
            .unwrap()
            .0;
        let controls = mei.nodes[&row].children[1];
        let children = mei.nodes[&controls].children.clone();
        assert_eq!(children.len(), 3);
        for id in [children[0], children[2]] {
            assert_eq!(mei.nodes[&id].flags & NodeFlags::WITH_NAV, 0);
            assert_eq!(mei.nodes[&id].sound_click, 5);
        }
        assert_eq!(
            mei.nodes[&children[1]].style.size,
            LengthPercentage::absolute(260. * 0.6)
        );
        let any: &mut dyn std::any::Any = mei.nodes.get_mut(&row).unwrap().content.as_mut();
        any.downcast_mut::<TweakerContent>().unwrap().delta = 1;
        mei.nodes.get_mut(&row).unwrap().state_bits |= 1;
        mei.nodes.get_mut(&children[2]).unwrap().state_bits |= 0x10;
        mei.recycle_frame_nodes();
        let root = mei.fullscreen("settings test");
        assert!(root.tweaker(
            "Volume",
            hint.text("音量说明"),
            260.,
            TweakableFloatWithStep {
                value: &mut value,
                step: 0.1,
                min: 0.,
                max: 2.
            }
        ));
        assert_eq!(hint.current, "音量说明");
        assert_eq!(value.to_bits(), 0.7f32.to_bits());
        // 原右箭头在值标签格式化之后执行：本帧文本0.6，权威值已经0.7。
        let label = mei
            .nodes
            .values_mut()
            .filter_map(|n| {
                let any: &mut dyn std::any::Any = n.content.as_mut();
                any.downcast_mut::<LabelContent>()
            })
            .find(|n| n.pending_text.as_deref() == Some("0.6"))
            .unwrap();
        assert_eq!(label.color, u32::MAX);
    }
    #[test]
    fn float_matches_original_instructions_and_panic_write_order() {
        #[derive(serde::Deserialize)]
        struct Case {
            increase: bool,
            before: (u32, u32, u32, u32),
            after: u32,
            panicked: bool,
        }
        let cases: Vec<Case> = ron::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../tests/fixtures/native/toolbar-tweaker-float.ron"
        )))
        .unwrap();
        assert_eq!(cases.len(), 216);
        for (index, case) in cases.into_iter().enumerate() {
            let mut value = f32::from_bits(case.before.0);
            let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut tweak = TweakableFloatWithStep {
                    value: &mut value,
                    step: f32::from_bits(case.before.1),
                    min: f32::from_bits(case.before.2),
                    max: f32::from_bits(case.before.3),
                };
                if case.increase {
                    tweak.increase();
                } else {
                    tweak.decrease();
                }
            }))
            .is_err();
            assert_eq!(panicked, case.panicked, "原样本{index}");
            if f32::from_bits(case.after).is_nan() {
                assert!(value.is_nan(), "原样本{index}");
            } else {
                assert_eq!(value.to_bits(), case.after, "原样本{index}");
            }
        }
    }
    #[test]
    fn list_closed_form_preserves_repeated_native_updates_and_invalid_index_edges() {
        for count in 0..8 {
            let values = vec![0; count];
            for before in [0, 1, 3, 10, usize::MAX - 1, usize::MAX] {
                for delta in -20i32..20 {
                    let mut expected = before;
                    if delta < 0 {
                        for _ in delta..0 {
                            expected = expected.saturating_sub(1);
                        }
                    } else {
                        for _ in 0..delta {
                            expected = expected.wrapping_add(1).min(count.wrapping_sub(1));
                        }
                    }
                    let mut actual = before;
                    TweakableList {
                        index: &mut actual,
                        values: &values,
                        formatter: |_| String::new(),
                    }
                    .apply_delta(delta);
                    assert_eq!(
                        actual, expected,
                        "count={count},before={before},delta={delta}"
                    );
                }
            }
        }
        let mut index = 1;
        TweakableList {
            index: &mut index,
            values: &[1, 2, 3],
            formatter: |x| x.to_string(),
        }
        .apply_delta(i32::MIN);
        assert_eq!(index, 0);
    }
    #[test]
    fn enum_bool_hint_and_input_phase_preserve_original_boundaries() {
        let mut content = TweakerContent::default();
        let action = ActionSetAction::Digital {
            slot: 21,
            state: country_core::input::action_set::DigitalActionState {
                phase: country_core::input::action_set::DigitalActionPhase::JustPressed,
                timer_seconds: 0.,
            },
        };
        assert!(!content.on_input_phase(&action, [false, false]));
        assert!(content.on_input_phase(&action, [true, true]));
        assert_eq!(content.delta, 1);
        let mut value = 1;
        let mut tweak = TweakableEnum {
            value: &mut value,
            variants: &[1, 2, 3],
            formatter: |x| x.to_string(),
        };
        tweak.apply_delta(i32::MAX);
        assert_eq!(tweak.format(), "3");
        tweak.apply_delta(i32::MIN);
        assert_eq!(tweak.format(), "1");
        let mut value = true;
        let mut boolean = TweakableBool(&mut value);
        boolean.decrease();
        assert_eq!(boolean.format(), lang_lib::lookup("Tweaker.No").to_string());
        boolean.increase();
        assert_eq!(
            boolean.format(),
            lang_lib::lookup("Tweaker.Yes").to_string()
        );
        let mut hint = HintTracker {
            current: "old".into(),
        };
        {
            let _ = hint.text("new");
        }
        assert_eq!(hint.current, "old");
        hint.text("new").confirm();
        assert_eq!(hint.current, "new");
    }
}
