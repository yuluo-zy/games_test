//! feedback_report_resources.rs — 反馈报告资源（WP21）。
//!
//! O：`FeedbackReportResources`（+0x0 Arc<Mutex<Option<Vec<u8>>>>、+0x8
//! Arc<Mutex<Option<Texture>>>、+0x3e 状态字节 7=空、+0x40..0x46 选项）；
//! `reset_choices` 0x909910、`clear_screenshot` 0x909930、`feedback_report_texture_receive`
//! 0x909ab0、`FeedbackReportTags::to_list` 0x909870、`FeedbackReportBgTexture::is_available`
//! 0x909860、plugin build 0x909a70。

use std::sync::Arc;

use bevy_ecs::resource::Resource;
use parking_lot::Mutex;

use crate::typed_gap::Gap;

/// `FeedbackReportResources`（O：反馈报告状态 + 截图 + 选择）。
#[derive(Resource, Clone, Debug)]
pub struct FeedbackReportResources {
    /// +0x0 截图字节（O：Arc<Mutex<Option<Vec<u8>>>>）。
    pub screenshot: Arc<Mutex<Option<Vec<u8>>>>,
    /// +0x8 GPU 纹理（O：Arc<Mutex<Option<Texture>>>；rhapsody 复用边界）。
    pub texture: Arc<Mutex<Option<Gap>>>,
    /// +0x3e 状态字节（O：7=空）。
    pub state: u8,
    /// +0x40..0x46 选择字节（O）。
    pub choices: [u8; 6],
    /// 未映射区（U）。
    pub _gap: Gap,
}

impl Default for FeedbackReportResources {
    fn default() -> Self {
        Self {
            screenshot: Arc::new(Mutex::new(None)),
            texture: Arc::new(Mutex::new(None)),
            state: 7,
            choices: [0; 6],
            _gap: Gap::default(),
        }
    }
}

impl FeedbackReportResources {
    /// `reset_choices`（O 0x909910）。
    pub fn reset_choices(&mut self) {
        self.choices[0] = 0;
        self.choices[1] = 1;
        self.choices[2] = 1;
        self.choices[3..].fill(0);
    }
    /// `clear_screenshot`（O 0x909930）。
    pub fn clear_screenshot(&mut self) {
        *self.screenshot.lock() = None;
        *self.texture.lock() = None;
        self.state = 7;
    }
}

/// `FeedbackReportTags`（O：标签表；to_list → Vec<String>）。
#[derive(Resource, Clone, Debug, Default)]
pub struct FeedbackReportTags {
    pub tags: &'static [&'static str],
}

impl FeedbackReportTags {
    /// `to_list`（O 0x909870）。
    pub fn to_list(&self) -> Vec<String> {
        self.tags.iter().filter_map(|s| Some((*s).to_string())).collect()
    }
}

/// `FeedbackReportBgTexture`（O：背景纹理就绪标志）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct FeedbackReportBgTexture(pub bool);

impl FeedbackReportBgTexture {
    /// `is_available`（O 0x909860）。
    pub fn is_available(&self) -> bool {
        self.0
    }
}

/// `FeedbackReportBackgroundPlugin`（O：Bevy 插件；build 0x909a70）。
#[derive(Clone, Copy, Debug, Default)]
pub struct FeedbackReportBackgroundPlugin;

/// `feedback_report_texture_receive`（O 0x909ab0）：接收截图纹理。
pub fn feedback_report_texture_receive(_res: &mut FeedbackReportResources, _texture: Gap) {
    // 业务逻辑 O（存纹理、状态置 7 前处理）；rhapsody 纹理边界。
}
