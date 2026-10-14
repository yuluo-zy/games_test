//! image_formation.rs — 画面形成配置（WP21；B 阶段 i_infra 按机器码修正布局）。
//!
//! # 布局（O，修正旧文档）
//! 旧文档写成“三档 0xc0 B config + kind u8”。机器码：
//! * `current_mut`（0x1408aea50）：`movzx eax, byte [rcx+0xc0]; shl eax, 6; add rax, rcx` → **每档 config 占 0x40 B**
//!   （下标左移 6），三档共 0xc0 B，当前档号 `kind: u8` 在 `+0xc0`；
//! * `get`（0x1408aea80）：`movzx eax, dl; shl eax, 6; add rax, rcx`（同一步长，**无边界检查**）；
//! * `iter_non_default_mut`（0x1408aea60）：写 `{begin = self, end = self + 0xc0, idx = 0}`（三档，步长 0x40）；
//! * `switch_kind`（0x1408aea90）：`mov [rcx+0xc0], dl`；
//! * `impl$0::default`（0x1408ae7f0，607 B）：逐档写入下表，末尾 `[+0xc0] = 1`（**缺省档是 1，不是 0**）。
//!
//! 单档 0x40 B（`default` 写入的字段；语义 U，按偏移命名）：
//!
//! | 偏移 | 内容 | 缺省（档 0 / 1 / 2） |
//! |---|---|---|
//! | +0x00 | `String` 名称 `{cap, ptr, len}`（0x18 B） | 三档都是 `"default"`（cap 7、len 7；两次 `alloc(7)` 拷入 `0x61666564`/`0x746c7561`） |
//! | +0x18 | 4×f32 | `(1.0, 0.0, 1.0, 1.0)`（`.rdata 0x142a9be30`） |
//! | +0x28 | u8 | 0 |
//! | +0x30 | 8 B（2×f32） | 档 0 / 1：0；档 2：`(0.0, 0.015)`（`.rdata 0x142a9be40`） |
//! | +0x38 | u32 | 0 |
//! | +0x3c | u8 | 0 |
//! | +0x3d | u8 | 档 0 / 1：1；档 2：进程级布尔 `.data 0x143634d90`（与 `water_planes_draw` 的守卫同一个字节） |
//!
//! 整个资源 `0xc8` B（三档 0xc0 + `kind` + 填充）。`ArtisticConfig`/`CameraLensConfig` 子类型（serde）。

use bevy_ecs::resource::Resource;
use serde::{Deserialize, Serialize};

/// `ImageFormationConfig`（O：0x40 B；见模块文档的偏移表，字段语义 U）。
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageFormationConfig {
    /// `+0x00`：名称（O：`String`，缺省 "default"）。
    pub name: String,
    /// `+0x18`：4×f32（O：缺省 `(1, 0, 1, 1)`）。
    pub params_18: [f32; 4],
    /// `+0x28`：u8（O：缺省 0）。
    pub flag_28: u8,
    /// `+0x29..+0x30`：未写入（对齐填充，U）。
    pub _pad_29: [u8; 7],
    /// `+0x30`：2×f32（O：档 2 缺省 `(0.0, 0.015)`，其余 0）。
    pub params_30: [f32; 2],
    /// `+0x38`：u32（O：缺省 0）。
    pub word_38: u32,
    /// `+0x3c`：u8（O：缺省 0）。
    pub flag_3c: u8,
    /// `+0x3d`：u8（O：档 0 / 1 缺省 1；档 2 = 进程级布尔）。
    pub flag_3d: u8,
    /// `+0x3e..+0x40`：尾部填充。
    pub _pad_3e: [u8; 2],
}

impl ImageFormationConfig {
    /// `impl$0::default`（0x1408ae7f0）里档 0 / 档 1 的初值。
    pub fn default_plain() -> Self {
        ImageFormationConfig {
            name: "default".to_owned(),
            params_18: [1.0, 0.0, 1.0, 1.0],
            flag_28: 0,
            _pad_29: [0; 7],
            params_30: [0.0, 0.0],
            word_38: 0,
            flag_3c: 0,
            flag_3d: 1,
            _pad_3e: [0; 2],
        }
    }

    /// 档 2 的初值（O：`+0x30 = (0.0, 0.015)`，`+0x3d` = 进程级布尔 `process_flag`）。
    pub fn default_third(process_flag: bool) -> Self {
        ImageFormationConfig {
            params_30: [0.0, 0.015],
            flag_3d: u8::from(process_flag),
            ..Self::default_plain()
        }
    }
}

impl Default for ImageFormationConfig {
    fn default() -> Self {
        Self::default_plain()
    }
}

/// 缺省当前档（O：`default` 末尾 `mov byte [rsi+0xc0], 1`）。
pub const DEFAULT_KIND: u8 = 1;
/// 单档大小 / 档数 / `kind` 偏移（O：`shl eax, 6`、`+0xc0`）。
pub const CONFIG_STRIDE: usize = 0x40;
pub const CONFIG_COUNT: usize = 3;
pub const KIND_OFFSET: usize = 0xc0;

/// `ImageFormationConfigs`（O：三档 × 0x40 B + 当前 kind @0xc0）。
#[derive(Resource, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageFormationConfigs {
    pub configs: [ImageFormationConfig; CONFIG_COUNT],
    /// 当前档（O：`+0xc0` 字节，`switch_kind` 写）。
    pub kind: u8,
}

impl ImageFormationConfigs {
    /// `impl$0::default`（0x1408ae7f0）：`process_flag` = `.data 0x143634d90` 的值（档 2 的 `+0x3d`）。
    pub fn with_process_flag(process_flag: bool) -> Self {
        ImageFormationConfigs {
            configs: [
                ImageFormationConfig::default_plain(),
                ImageFormationConfig::default_plain(),
                ImageFormationConfig::default_third(process_flag),
            ],
            kind: DEFAULT_KIND,
        }
    }
}

impl Default for ImageFormationConfigs {
    /// 进程级布尔的 .bss 初值为 0（U：写入者未找到）。
    fn default() -> Self {
        Self::with_process_flag(false)
    }
}

impl ImageFormationConfigs {
    /// `initial`（O 0x67dd80 折叠）。
    pub fn initial() -> Self {
        Self::default()
    }
    /// `current`（O 名，内联）：`configs[kind]`。
    pub fn current(&self) -> &ImageFormationConfig {
        &self.configs[self.kind as usize]
    }
    /// `current_mut`（O 0x1408aea50）：`self + (kind << 6)`。原版无边界检查，这里越界即 panic。
    pub fn current_mut(&mut self) -> &mut ImageFormationConfig {
        &mut self.configs[self.kind as usize]
    }
    /// `get`（O 0x1408aea80）：`self + (i << 6)`，原版无边界检查（i 是 u8），这里越界返回 `None`。
    pub fn get(&self, i: usize) -> Option<&ImageFormationConfig> {
        self.configs.get(i)
    }
    /// `iter_non_default_mut`（O 0x1408aea60：`{begin, begin + 0xc0, idx = 0}`，三档遍历；“non_default”的过滤条件 U）。
    pub fn iter_non_default_mut(&mut self) -> impl Iterator<Item = &mut ImageFormationConfig> {
        self.configs.iter_mut()
    }
    /// `switch_kind`（O 0x1408aea90）。
    pub fn switch_kind(&mut self, kind: u8) {
        self.kind = kind;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    /// 布局（O：`shl eax, 6`、`+0xc0`）：单档 0x40 B，三档 0xc0 B，字段偏移与 `default` 的写入偏移一致。
    #[test]
    fn each_config_is_0x40_bytes_and_the_array_is_0xc0() {
        assert_eq!(size_of::<ImageFormationConfig>(), CONFIG_STRIDE);
        assert_eq!(
            size_of::<[ImageFormationConfig; CONFIG_COUNT]>(),
            KIND_OFFSET
        );
        assert_eq!(offset_of!(ImageFormationConfigs, kind), KIND_OFFSET);
        assert_eq!(offset_of!(ImageFormationConfig, params_18), 0x18);
        assert_eq!(offset_of!(ImageFormationConfig, flag_28), 0x28);
        assert_eq!(offset_of!(ImageFormationConfig, params_30), 0x30);
        assert_eq!(offset_of!(ImageFormationConfig, word_38), 0x38);
        assert_eq!(offset_of!(ImageFormationConfig, flag_3c), 0x3c);
        assert_eq!(offset_of!(ImageFormationConfig, flag_3d), 0x3d);
        assert_eq!(size_of::<ImageFormationConfigs>(), 0xc8);
    }

    /// O 0x1408ae7f0：缺省档是 1，三档名称都是 "default"，档 2 的 `+0x30` 是 `(0, 0.015)`，`+0x3d` 取进程级布尔。
    #[test]
    fn defaults_follow_the_machine_code() {
        let c = ImageFormationConfigs::default();
        assert_eq!(c.kind, 1, "`mov byte [rsi+0xc0], 1`");
        assert!(
            c.configs
                .iter()
                .all(|x| x.name == "default" && x.name.len() == 7)
        );
        assert!(
            c.configs
                .iter()
                .all(|x| x.params_18 == [1.0, 0.0, 1.0, 1.0])
        );
        assert_eq!(c.configs[0], ImageFormationConfig::default_plain());
        assert_eq!(c.configs[1], ImageFormationConfig::default_plain());
        assert_eq!(c.configs[0].flag_3d, 1);
        assert_eq!(c.configs[2].params_30, [0.0, 0.015]);
        assert_eq!(c.configs[2].flag_3d, 0, "进程级布尔 .bss 初值 0");
        assert_eq!(
            ImageFormationConfigs::with_process_flag(true).configs[2].flag_3d,
            1
        );
        assert_eq!(c.current(), &c.configs[1]);
    }

    /// O：`switch_kind` 写 `+0xc0`；`current_mut` / `get` 以 `kind << 6` 取档。
    #[test]
    fn switch_kind_selects_the_config_by_index() {
        let mut c = ImageFormationConfigs::default();
        c.switch_kind(2);
        assert_eq!(c.kind, 2);
        c.current_mut().word_38 = 7;
        assert_eq!(c.configs[2].word_38, 7);
        assert_eq!(c.get(2).map(|x| x.word_38), Some(7));
        assert!(c.get(3).is_none());
        assert_eq!(c.iter_non_default_mut().count(), 3);
    }
}
