//! 显示器 ICC 的原版 LUT 生成及当前 App 的纹理所有权。
//!
//! B：update_window_icc_profile@0x14093d830、网格 collect@0x1413036c0。
//! Little CMS 使用原锁定的公开依赖，不以简化矩阵替代完整 ICC 变换。

use std::{path::{Path, PathBuf}, sync::Arc};

use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;

use crate::render::load_texture::boundary::{Texture, TextureDesc, TextureDevice};

pub const ICC_LUT_EDGE: u32 = 32;
pub const ICC_LUT_FORMAT: u32 = 0x5b;

#[derive(Event, Clone, Debug)]
pub struct IccProfileChanged(pub PathBuf);

/// 原设备调用的工程边界；必须注入本 App 已有的 TextureHost。
#[derive(Resource, Clone)]
pub struct IccTextureDevice(pub Arc<dyn TextureDevice + Send + Sync>);

struct OwnedIccLut {
    name: String,
    texture: Texture,
    device: Arc<dyn TextureDevice + Send + Sync>,
}

impl Drop for OwnedIccLut {
    fn drop(&mut self) {
        // 只注销本 App 的 CPU 身份；图和描述符持有的实际 GPU owner 继续延迟回收。
        self.device.release_texture(&self.texture);
    }
}

/// 原资源的默认值是 None；Some 同时拥有显示名称、纹理及其原设备命名空间。
#[derive(Clone, Resource, Default)]
pub struct ActiveIccProfile {
    /// Shared with render-thread snapshots; the last owner unregisters it.
    active: Option<Arc<OwnedIccLut>>,
}

impl ActiveIccProfile {
    pub fn name(&self) -> Option<&str> {
        self.active.as_ref().map(|active| active.name.as_str())
    }

    /// 同时返回 owner，避免不同 App 的数字句柄相同而误绑纹理。
    pub fn texture_and_device(
        &self,
    ) -> Option<(&Texture, &Arc<dyn TextureDevice + Send + Sync>)> {
        self.active.as_ref().map(|active| (&active.texture, &active.device))
    }
}

pub struct IccProfilePlugin;

impl Plugin for IccProfilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveIccProfile>()
            .add_event::<IccProfileChanged>()
            .add_systems(Update, update_window_icc_profile);
    }
}

/// R 最快、G 次之、B 最慢。原整数除法截断，alpha 为 255，不能改成 65535。
fn input_grid() -> Vec<[u16; 4]> {
    let mut values = Vec::with_capacity(ICC_LUT_EDGE.pow(3) as usize);
    let channel = |value: u32| (value * u32::from(u16::MAX) / (ICC_LUT_EDGE - 1)) as u16;
    for b in 0..ICC_LUT_EDGE {
        for g in 0..ICC_LUT_EDGE {
            for r in 0..ICC_LUT_EDGE {
                values.push([channel(r), channel(g), channel(b), 255]);
            }
        }
    }
    values
}

fn generate_icc_lut(path: &Path) -> anyhow::Result<Vec<[u16; 4]>> {
    let output = lcms2::Profile::new_file(path)?;
    let input = lcms2::Profile::new_srgb();
    let transform = lcms2::Transform::<[u16; 4], [u16; 4]>::new(
        &input,
        lcms2::PixelFormat::RGBA_16,
        &output,
        lcms2::PixelFormat::RGBA_16,
        lcms2::Intent::Perceptual,
    )?;
    let mut values = input_grid();
    transform.transform_in_place(&mut values);
    Ok(values)
}

fn update_window_icc_profile(
    mut events: EventReader<IccProfileChanged>,
    mut active: ResMut<ActiveIccProfile>,
    device: Option<Res<IccTextureDevice>>,
) {
    let Some(event) = events.read().last() else { return; };
    // 原版在文件读取和解析前释放旧 Some；坏的新路径也必须回到 None。
    drop(active.active.take());
    let device = device.expect("ICC profile event requires this App's texture device");
    match generate_icc_lut(&event.0) {
        Ok(values) => {
            let texture = device.0.create_texture(
                "ICC",
                &TextureDesc::new_3d([ICC_LUT_EDGE; 3], ICC_LUT_FORMAT).sampled(),
                bytemuck::cast_slice(&values),
            ).expect("ICC LUT texture");
            let mut next = OwnedIccLut {
                name: String::new(),
                texture,
                device: device.0.clone(),
            };
            let shown = event.0.file_name().unwrap_or_else(|| event.0.as_os_str());
            next.name = shown.to_string_lossy().into_owned();
            active.active = Some(Arc::new(next));
        }
        Err(error) => log::warn!(target: "country_core::systems::backend::icc",
            "Failed creating an ICC LUT: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parking_lot::Mutex;
    use crate::render::load_texture::boundary::BackendError;

    #[derive(Default)]
    struct Device {
        calls: Mutex<Vec<Call>>,
        next_id: Mutex<u64>,
        fail_create: Mutex<bool>,
    }
    enum Call {
        Create { desc: TextureDesc, pixels: Vec<[u16; 4]> },
        Release(u64),
    }
    impl TextureDevice for Device {
        fn release_texture(&self, texture: &Texture) {
            self.calls.lock().push(Call::Release(texture.id));
        }
        fn create_texture(&self, name: &str, desc: &TextureDesc, data: &[u8]) -> Result<Texture, BackendError> {
            assert_eq!(name, "ICC");
            self.calls.lock().push(Call::Create {
                desc: *desc,
                pixels: data.chunks_exact(8).map(|pixel| std::array::from_fn(|i| {
                    u16::from_ne_bytes([pixel[2 * i], pixel[2 * i + 1]])
                })).collect(),
            });
            if *self.fail_create.lock() {
                return Err(BackendError::OutOfDeviceMemory);
            }
            let mut id = self.next_id.lock();
            *id += 1;
            Ok(Texture { id: *id, width: desc.extent[0], height: desc.extent[1], format: desc.format })
        }
    }
    fn app(device: Arc<Device>) -> App {
        let mut app = App::new();
        app.insert_resource(IccTextureDevice(device));
        app.add_plugins(IccProfilePlugin);
        app
    }
    fn profile_file() -> tempfile::NamedTempFile {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        std::io::Write::write_all(&mut file, &lcms2::Profile::new_srgb().icc().unwrap()).unwrap();
        file
    }
    fn send(app: &mut App, path: &Path) {
        app.world_mut().send_event(IccProfileChanged(path.to_path_buf()));
    }
    fn creates(device: &Device) -> usize {
        device.calls.lock().iter().filter(|call| matches!(call, Call::Create { .. })).count()
    }

    #[test]
    fn icc_grid_preserves_native_order_alpha_and_integer_quotients() {
        let values = input_grid();
        assert_eq!(values.len(), 32768);
        for (i, value) in values.iter().enumerate() {
            let convert = |channel: usize| (channel as u32 * 65535 / 31) as u16;
            assert_eq!(*value, [convert(i % 32), convert(i / 32 % 32), convert(i / 1024), 255]);
        }
    }

    fn grid_fingerprint_le(values: &[[u16; 4]]) -> u64 {
        values.iter().flatten().flat_map(|value| value.to_le_bytes()).fold(
            0xcbf2_9ce4_8422_2325,
            |hash, byte| (hash ^ u64::from(byte)).wrapping_mul(0x100_0000_01b3),
        )
    }

    #[test]
    fn icc_reuses_exact_lcms_215_and_serialized_srgb_quantization_oracle() {
        assert_eq!(lcms2::version(), 2150);
        let expected = input_grid();
        let memory = lcms2::Profile::new_srgb();
        let transform = lcms2::Transform::<[u16; 4], [u16; 4]>::new(
            &memory, lcms2::PixelFormat::RGBA_16,
            &memory, lcms2::PixelFormat::RGBA_16,
            lcms2::Intent::Perceptual,
        ).unwrap();
        let mut identity = expected.clone();
        transform.transform_in_place(&mut identity);
        assert_eq!(identity, expected);
        assert_eq!(grid_fingerprint_le(&identity), 0x7129_9975_8885_5b45);

        let file = profile_file();
        let actual = generate_icc_lut(file.path()).unwrap();
        // 独立 LCMS 2.15 C API 全网格基准：写盘把 XYZ/TRC 量化为 s15Fixed16。
        // 因而内存 sRGB → 重读文件并非恒等，不能按恒等预期放宽误差。
        assert_eq!(grid_fingerprint_le(&actual), 0xa436_8867_549a_25f2);
        let mut per_channel_max = [0_u16; 3];
        let mut changed_channels = 0;
        for (actual, expected) in actual.iter().zip(&expected) {
            for channel in 0..3 {
                let difference = actual[channel].abs_diff(expected[channel]);
                per_channel_max[channel] = per_channel_max[channel].max(difference);
                changed_channels += usize::from(difference != 0);
            }
            assert_eq!(actual[3], 255);
        }
        assert_eq!(per_channel_max, [31, 11, 13]);
        assert_eq!(changed_channels, 34819);
        assert_eq!(actual[31], [65535, 0, 6, 255]);
    }

    #[test]
    fn icc_linear_output_profile_applies_non_identity_transfer_curve() {
        let xy = |x, y| lcms2::CIExyY { x, y, Y: 1.0 };
        let primaries = lcms2::CIExyYTRIPLE {
            Red: xy(0.64, 0.33), Green: xy(0.30, 0.60), Blue: xy(0.15, 0.06),
        };
        let curve = lcms2::ToneCurve::new(1.0);
        let output = lcms2::Profile::new_rgb(&xy(0.3127, 0.3290), &primaries, &[&curve; 3]).unwrap();
        let mut file = tempfile::NamedTempFile::new().unwrap();
        std::io::Write::write_all(&mut file, &output.icc().unwrap()).unwrap();
        let actual = generate_icc_lut(file.path()).unwrap();
        let middle = actual[16 * 1024 + 16 * 32 + 16];
        // sRGB 码值16/31转线性约0.229；不能返回原输入约0.516或默认LUT。
        for channel in &middle[..3] { assert!((14000..16000).contains(channel)); }
        assert_eq!(middle[3], 255);
        assert_eq!(actual[0], [0, 0, 0, 255]);
    }

    #[test]
    fn icc_last_event_only_and_real_3d_texture_upload() {
        let device = Arc::new(Device::default());
        let mut app = app(device.clone());
        let file = profile_file();
        send(&mut app, &file.path().with_extension("missing"));
        send(&mut app, file.path());
        app.update();
        assert_eq!(creates(&device), 1);
        let calls = device.calls.lock();
        let Call::Create { desc, pixels } = &calls[0] else { panic!("expected texture upload") };
        assert_eq!(desc.extent, [32; 3]);
        assert_eq!(desc.kind, 4);
        assert_eq!(desc.format, 0x5b);
        assert!(desc.sampled);
        assert_eq!(pixels.len(), 32768);
        assert_eq!(pixels[31], [65535, 0, 6, 255]);
        assert_eq!(app.world().resource::<ActiveIccProfile>().name(), file.path().file_name().unwrap().to_str());
        drop(calls);
        app.update();
        assert_eq!(creates(&device), 1);
    }

    #[test]
    fn icc_bad_last_profile_clears_old_lut_before_parse_and_does_not_upload() {
        let device = Arc::new(Device::default());
        let mut app = app(device.clone());
        let good = profile_file();
        send(&mut app, good.path());
        app.update();
        let mut bad = tempfile::NamedTempFile::new().unwrap();
        std::io::Write::write_all(&mut bad, b"invalid ICC").unwrap();
        send(&mut app, good.path());
        send(&mut app, bad.path());
        app.update();
        assert!(app.world().resource::<ActiveIccProfile>().texture_and_device().is_none());
        assert_eq!(creates(&device), 1);
        assert!(matches!(device.calls.lock().last(), Some(Call::Release(1))));
        drop(app);
        assert_eq!(device.calls.lock().iter().filter(|call| matches!(call, Call::Release(_))).count(), 1);
    }

    #[test]
    fn icc_gpu_create_failure_is_fatal_and_old_owner_is_released_once() {
        let device = Arc::new(Device::default());
        let mut app = app(device.clone());
        let file = profile_file();
        send(&mut app, file.path());
        app.update();
        *device.fail_create.lock() = true;
        send(&mut app, file.path());
        let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| app.update()));
        assert!(failed.is_err());
        assert!(app.world().resource::<ActiveIccProfile>().texture_and_device().is_none());
        assert_eq!(creates(&device), 2);
        assert_eq!(device.calls.lock().iter().filter(|call| matches!(call, Call::Release(1))).count(), 1);
    }

    #[test]
    fn icc_apps_keep_distinct_owners_even_with_equal_numeric_texture_ids() {
        let first = Arc::new(Device::default());
        let second = Arc::new(Device::default());
        let mut a = app(first.clone());
        let mut b = app(second.clone());
        let file = profile_file();
        send(&mut a, file.path());
        send(&mut b, file.path());
        a.update();
        b.update();
        let aa = a.world().resource::<ActiveIccProfile>().texture_and_device().unwrap();
        let bb = b.world().resource::<ActiveIccProfile>().texture_and_device().unwrap();
        assert_eq!(aa.0.id, bb.0.id);
        assert!(!Arc::ptr_eq(aa.1, bb.1));
        drop(a);
        assert!(matches!(first.calls.lock().last(), Some(Call::Release(1))));
        assert_eq!(second.calls.lock().len(), 1);
        b.world_mut().remove_resource::<IccTextureDevice>();
        drop(b);
        assert!(matches!(second.calls.lock().last(), Some(Call::Release(1))));
    }

    #[test]
    fn icc_no_event_without_gpu_keeps_true_default_none() {
        let mut app = App::new();
        app.add_plugins(IccProfilePlugin);
        app.update();
        assert!(app.world().resource::<ActiveIccProfile>().texture_and_device().is_none());
    }
}
