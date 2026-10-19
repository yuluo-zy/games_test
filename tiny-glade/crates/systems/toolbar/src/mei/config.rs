//! B RON template schema and recursive instantiation, original14068eb50.
//! Schema: native field visitors140fc9a40/140fc9b40; NodeChild visitor140fb7960
//! and140fb7c30. See original-template-layout.json and original RNE asm in the
//! TG-1ab2df0ca206ff9918/meihost-20261018 evidence directory.
use crate::mei::{
    core::{GlobalId, NodeContext, calculate_hash},
    layout::{Align, Justify, LayoutStyle, LengthPercentage, XAlign},
    widgets::{FrameContent, LabelContent},
};
use serde::{
    Deserialize, Deserializer,
    de::{self, MapAccess, SeqAccess, Visitor},
};
use std::{fmt, path::Path};
use tg_text::{FontFamily, ShapeStream, font_table::FontTable};

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct NodeOwnConfig {
    pub draw: ShapeStream,
    pub layout: Vec<NodeLayoutConfigAttrib>,
    pub slot: Option<String>,
}
#[derive(Debug, Clone, Default)]
pub struct NodeConfig {
    pub own: NodeOwnConfig,
    pub children: Vec<NodeChild>,
}
#[derive(Debug, Clone)]
pub enum NodeChild {
    Node(NodeConfig),
    Label(LabelConfig),
}

/// Original LabelConfig leaves absent color/metrics/width/family unchanged.
/// Text is required; alignment/weight are optional (native None tags5/9).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LabelConfig {
    pub text: String,
    #[serde(default, deserialize_with = "deserialize_optional")]
    pub color: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_optional")]
    pub align: Option<TextAlign>,
    #[serde(default, deserialize_with = "deserialize_optional")]
    pub font: Option<FontFamily>,
    #[serde(default, deserialize_with = "deserialize_optional")]
    pub size: Option<f32>,
    #[serde(default, deserialize_with = "deserialize_optional")]
    pub max_width: Option<f32>,
    #[serde(default, deserialize_with = "deserialize_optional")]
    pub weight: Option<TextWeight>,
}
#[derive(Debug, Clone, Copy, Deserialize)]
pub enum TextAlign {
    Left,
    Right,
    Center,
    Justified,
    End,
}
impl TextAlign {
    fn cosmic(self) -> tg_text::Align {
        match self {
            Self::Left => tg_text::Align::Left,
            Self::Right => tg_text::Align::Right,
            Self::Center => tg_text::Align::Center,
            Self::Justified => tg_text::Align::Justified,
            Self::End => tg_text::Align::End,
        }
    }
}
#[derive(Debug, Clone, Copy, Deserialize)]
#[allow(non_camel_case_types)]
pub enum TextWeight {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    Semibold,
    Bold,
    ExtraBold,
    Black,
}
impl TextWeight {
    fn cosmic(self) -> tg_text::Weight {
        tg_text::Weight(match self {
            Self::Thin => 100,
            Self::ExtraLight => 200,
            Self::Light => 300,
            Self::Normal => 400,
            Self::Medium => 500,
            Self::Semibold => 600,
            Self::Bold => 700,
            Self::ExtraBold => 800,
            Self::Black => 900,
        })
    }
}

/// Padding accepts one scalar/percentage or a left/top/right/bottom tuple,
/// matching the native untagged Rect<LengthPercentage> deserializer140fe2560.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Padding {
    All(LengthPercentage),
    Four(
        (
            LengthPercentage,
            LengthPercentage,
            LengthPercentage,
            LengthPercentage,
        ),
    ),
}
impl Padding {
    fn values(&self) -> [LengthPercentage; 4] {
        match *self {
            Self::All(v) => [v; 4],
            Self::Four((l, t, r, b)) => [l, t, r, b],
        }
    }
}
#[derive(Debug, Clone, Deserialize)]
pub enum NodeLayoutConfigAttrib {
    Horizontal,
    Vertical,
    Expand,
    Align(Align),
    XAlign(XAlign),
    Size(LengthPercentage, LengthPercentage),
    Gap(LengthPercentage),
    Padding(Padding),
}
impl NodeOwnConfig {
    pub fn layout_style(&self) -> LayoutStyle {
        let mut style = LayoutStyle::default();
        for attr in &self.layout {
            match attr {
                NodeLayoutConfigAttrib::Horizontal => style.swizzle = 0,
                NodeLayoutConfigAttrib::Vertical => style.swizzle = 1,
                NodeLayoutConfigAttrib::Expand => style.expand = 1.,
                NodeLayoutConfigAttrib::Align(value) => style.justify = Justify(*value as u8),
                NodeLayoutConfigAttrib::XAlign(value) => style.align = *value as u8,
                NodeLayoutConfigAttrib::Size(width, height) => {
                    style.size = *width;
                    style.min = *height;
                }
                NodeLayoutConfigAttrib::Gap(value) => style.gap = *value,
                NodeLayoutConfigAttrib::Padding(value) => style.padding = value.values(),
            }
        }
        style
    }
}
struct NodeVisitor;
impl<'de> Visitor<'de> for NodeVisitor {
    type Value = NodeChild;
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a (node config, optional children) tuple or (Label: config)")
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let own = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let children = seq.next_element()?.unwrap_or_default();
        Ok(NodeChild::Node(NodeConfig { own, children }))
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        #[derive(Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            Label,
        }
        match map.next_key::<Field>()? {
            Some(Field::Label) => Ok(NodeChild::Label(map.next_value()?)),
            None => Err(de::Error::custom("missing Label configuration")),
        }
    }
}
impl<'de> Deserialize<'de> for NodeChild {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        de.deserialize_any(NodeVisitor)
    }
}
impl<'de> Deserialize<'de> for NodeConfig {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        match NodeChild::deserialize(de)? {
            NodeChild::Node(node) => Ok(node),
            NodeChild::Label(_) => Err(de::Error::custom("UI template root must be a node")),
        }
    }
}

pub fn read_node_config_from_file(path: &Path) -> Result<NodeConfig, ron::error::SpannedError> {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{e}"));
    // B original Options extension word1 / recursion_limit Some128.
    ron::Options::default()
        .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
        .from_str(&text)
}
pub fn load_font_table(path: &Path) -> tg_text::TextResult<FontTable> {
    FontTable::from_ron(&std::fs::read_to_string(path)?)
}

/// Original TypeId::hash hashes only the high u64.1405bdf20 loads[rcx+10],
/// hashes value then this word; using today's TypeId would change persisted IDs.
const NODE_TYPE_HASH: u64 = 0xa79b7268a2a968d9;
const LABEL_TYPE_HASH: u64 = 0xce69dbf0aee13da2;
fn child_id(parent: GlobalId, value: u64, type_hash: u64) -> GlobalId {
    GlobalId(calculate_hash(&(
        parent.0,
        calculate_hash(&(value, type_hash)),
    )))
}
fn slot_id(namespace: GlobalId, name: &str) -> GlobalId {
    child_id(namespace, calculate_hash(&(name, ())), NODE_TYPE_HASH)
}

/// Replace every t!(key) segment, keeping surrounding UTF-8 text. Missing ')'
/// produces the original visible Invalid t! format diagnostic as the label.
///
/// Original name `localize_ui_text_from_config` (inline, 2 instances; deps are
/// `format!`, `str::find`, slicing and `push_str`). Missing keys fall back to
/// the lang-lib lookup value.
pub fn localize_ui_text_from_config(text: &str) -> String {
    localize_label(text)
}

/// Original `deserialize_optional` (inline, 19 instances): the transparent
/// `Option<T>` field seed used by the config/template RON structures.
pub fn deserialize_optional<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    Option::<T>::deserialize(deserializer)
}

fn localize_label(text: &str) -> String {
    let mut rest = text;
    let mut result = String::new();
    while let Some(start) = rest.find("t!(") {
        result.push_str(&rest[..start]);
        let key = &rest[start + 3..];
        let Some(end) = key.find(')') else {
            return format!("Invalid t! format: {text}");
        };
        result.push_str(&lang_lib::lookup(&key[..end]));
        rest = &key[end + 1..];
    }
    result.push_str(rest);
    result
}

/// Original four-argument helper: parent node context, stable template namespace,
/// fallback child id and NodeConfig. Slots use the template namespace globally;
/// unnamed children use parent-node/index IDs. Label IDs use localized text.
pub fn instantiate_ui_template_impl(
    parent: &NodeContext,
    namespace: GlobalId,
    fallback: GlobalId,
    config: &NodeConfig,
) -> NodeContext {
    let id = config
        .own
        .slot
        .as_deref()
        .map(|name| slot_id(namespace, name))
        .unwrap_or(fallback);
    let frame = parent.child_with_global_id(id);
    frame.get_content_mut_impl::<FrameContent>(false).shapes = config.own.draw.clone();
    frame.layout(config.own.layout_style());
    for (index, child) in config.children.iter().enumerate() {
        match child {
            NodeChild::Node(node) => {
                instantiate_ui_template_impl(
                    &frame,
                    namespace,
                    child_id(id, index as u64, NODE_TYPE_HASH),
                    node,
                );
            }
            NodeChild::Label(config) => {
                let text = localize_label(&config.text);
                let id = child_id(id, calculate_hash(&(text.as_str(), ())), LABEL_TYPE_HASH);
                let label = frame.child_with_global_id(id);
                let content = label.get_content_mut_impl::<LabelContent>(false);
                content.set_text(text);
                if let Some(color) = config.color {
                    content.color = color;
                }
                if let Some(size) = config.size {
                    content
                        .text
                        .set_metrics(size)
                        .expect("template label metrics");
                }
                if let Some(width) = config.max_width {
                    content
                        .text
                        .set_max_width(width)
                        .expect("template label maximum width");
                }
                if let Some(family) = config.font {
                    content
                        .text
                        .set_family(family)
                        .expect("template label font role");
                }
                if let Some(align) = config.align {
                    content.text.set_align(align.cosmic());
                } else {
                    content.text.clear_align();
                }
                content
                    .text
                    .set_weight(config.weight.unwrap_or(TextWeight::Normal).cosmic());
            }
        }
    }
    frame
}

/// Original stores template name and caller's node context, not a detached tree.
#[derive(Clone, Debug)]
pub struct TemplateView {
    pub name: String,
    pub context: NodeContext,
}
impl TemplateView {
    pub fn node_mut(&self) -> &mut crate::mei::core::Node {
        self.context.node_mut()
    }
    pub fn slot(&self, name: &str) -> NodeContext {
        let id = slot_id(self.context.id, name);
        if self.context.has_global_id(id) {
            self.context.expect_child_with_global_id(id)
        } else {
            log::warn!("UI template {} has no slot {}", self.name, name);
            self.context.child_with_global_id(id)
        }
    }
    pub fn into_slot(self, name: &str) -> NodeContext {
        self.slot(name)
    }
    pub fn with_slot<R>(&self, name: &str, f: impl FnOnce(NodeContext) -> R) -> R {
        f(self.slot(name))
    }
}
impl NodeContext {
    pub fn template(&self, name: &str) -> TemplateView {
        self.template_with_id(name, 0)
    }
    pub fn template_with_id(&self, name: &str, id: u64) -> TemplateView {
        let config = self
            .mei_ref()
            .configs
            .get(name)
            .unwrap_or_else(|| {
                panic!(
                    "UI template {name} is missing; available: {:?}",
                    self.mei_ref().configs.keys()
                )
            })
            .clone();
        instantiate_ui_template_impl(
            self,
            self.id,
            child_id(self.id, id, NODE_TYPE_HASH),
            &config,
        );
        TemplateView {
            name: name.into(),
            context: *self,
        }
    }
}

#[cfg(test)]
mod native_template_tests {
    use super::*;
    #[test]
    fn shipped_tuple_template_builds_real_frames_and_labels() {
        let text = "((layout:[Vertical,Gap(3),Padding((2,4,6,8))]), [((slot:\"body\",draw:[FillColor(0x123456ff)],layout:[Size(40,20)])), (Label:(text:\"Title\",size:20,align:Center))])";
        let config: NodeConfig = ron::Options::default()
            .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
            .from_str(text)
            .unwrap();
        let mut mei = crate::mei::core::MeiContext::default();
        mei.font_table = std::sync::Arc::new(
            FontTable::from_ron("(fonts:{Label:(family:\"Baloo2\")})").unwrap(),
        );
        mei.configs
            .insert("test".into(), std::sync::Arc::new(config));
        let root = mei.safe_frame("frame", false);
        let view = root.template("test");
        let body = view.slot("body");
        assert_eq!(body.node().style.size.value, 40.);
        assert!(
            !body
                .get_content_mut_impl::<FrameContent>(true)
                .shapes
                .is_empty()
        );
        let frame_id = root.node().children[0];
        let frame = mei.nodes.get(&frame_id).unwrap();
        assert_eq!(frame.children.len(), 2);
        assert_eq!(frame.style.padding[0].value, 2.);
        assert_eq!(frame.style.padding[3].value, 8.);
        let label = root.expect_child_with_global_id(frame.children[1]);
        assert_eq!(
            label
                .get_content_mut_impl::<LabelContent>(true)
                .pending_text
                .as_deref(),
            Some("Title")
        );
    }

    /// O: localize_ui_text_from_config replaces t!(key) and keeps text around
    /// it; an unterminated marker returns the native diagnostic string.
    #[test]
    fn localize_ui_text_substitutes_every_t_marker() {
        assert_eq!(localize_ui_text_from_config("plain"), "plain");
        let localized = localize_ui_text_from_config("a t!(Some_Key) b t!(Other_Key) c");
        assert!(localized.starts_with("a "));
        assert_eq!(localized.matches(" b ").count(), 1);
        assert!(localized.ends_with(" c"));
        assert_eq!(
            localize_ui_text_from_config("t!(unterminated"),
            "Invalid t! format: t!(unterminated"
        );
    }
}
