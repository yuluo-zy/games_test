//! `tiny_glade::vram` — render-resolution budget heuristics.
//!
//! Recovered from B `crates/tiny-glade/src/vram.rs`, VAs
//! `0x1404f7f20` (`calculate_max_resolution_scale`) and
//! `0x1404f8240` (`calculate_resolution_scale_mem_estimate_text`).
//!
//! Both functions consume the physical device's Vulkan device type
//! (`VkPhysicalDeviceType::DISCRETE_GPU == 2`, checked at `+0xd8`) and the
//! memory-budget extension values. All numeric constants below are read from
//! the original `.rdata` at `0x1429ecb78..0x1429ecbcc`:
//! `-1365057216.0`, `-47.7399274329577`, `146.1091661136655`, `10912.0`,
//! `-992811760.0`, `-372245456.0`, `1/2^30`, and their positive twins.
//!
//! restorer-tinyglade-E fresh 复核（2026-10-17）：
//! - `calculate_max_resolution_scale` 0x1404f7f20、`..._text` 0x1404f8240
//!   逐指令对照；f32/f64 常量、`/0.0` 宿主分支、`floorf(x*10.0)` 与
//!   `f64::min`/`f64::max` 的 NaN 选择语义（`xmm` select）均一致。
//! - 文本函数先 `as u64`（饱和截断）再乘 `2^-30`（0x1404f8279 区段：
//!   `<0→0`、`>u64::MAX→MAX` 的 CMOVS 序列）；精度说明符由
//!   `format_inner` 的 Placeholder 常量解出为 `Count::Is(2)` → `{:.2}`。
//! - 预算字节在 B 中直接由 `PhysicalDevice::..._budget_bytes` 返回裸 u64；
//!   本 workspace 的 rhapsody 合同为 `Result<Option<u64>>`，缺省按 0 处理
//!   （B 未支持扩展时求和结果同样为 0）。合同缝，非行为分歧。

use ash::vk;
use rhapsody::raw::device::Device;

const BYTES_PER_UNIT: f64 = 47.7399274329577;
const COST_PER_SCALED_UNIT: f64 = 146.1091661136655;
const MAX_AXIS: f64 = 10912.0;
const DISCRETE_RESERVED: f64 = 992_811_760.0;
const SYSTEM_RESERVED: f64 = 372_245_456.0;
const UNIFIED_RESERVED: f64 = 1_365_057_216.0;
const GIB: f64 = 9.313_225_746_154_785e-10;

/// B `tiny_glade::vram::calculate_max_resolution_scale`.
///
/// Reconstructed CFG (per fresh decompilation):
/// * `budget = device_local_memory_budget_bytes()`
/// * discrete GPU (`device_type == 2`):
///   - `a = scale_from(budget, w, h, DISCRETE_RESERVED)`
///   - `b = scale_from(host_memory_budget_bytes(), w, h, SYSTEM_RESERVED)`
///   - result = `f32::min(a, b)`
/// * otherwise (integrated/unified):
///   - `budget = device_local + host`
///   - result = `scale_from(budget, w, h, UNIFIED_RESERVED)`
///
/// `scale_from` divides by `0.0` in the host-budget path in the original
/// machine code (`divsd xmm0, xmm11` where `xmm11 == 0.0`); this is preserved
/// verbatim because it is observable: a host budget below the reserve yields a
/// negative value, the `0.0 < value` guard skips it and `min` returns the first
/// scale, while a positive budget yields `+inf` and does not constrain.
///
/// Final quantisation is `(scale * 10.0).floor() * 0.1`.
pub fn calculate_max_resolution_scale(
    device: &Device,
    render_resolution: [u32; 2],
) -> f32 {
    let physical = device
        .physical_device_info()
        .expect("physical device info must be initialised");
    let width = render_resolution[0] as f64;
    let height = render_resolution[1] as f64;

    let discrete = physical.properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU;
    let device_local = physical
        .device_local_memory_budget_bytes()
        .ok()
        .flatten()
        .unwrap_or(0);
    let host = physical
        .host_memory_budget_bytes()
        .ok()
        .flatten()
        .unwrap_or(0);

    let scale = if discrete {
        let gpu = scale_from_budget(device_local as f64, width, height, DISCRETE_RESERVED);
        // Original: `(host + -372245456.0) / 0.0`.
        let host_quotient = (host as f64 + -SYSTEM_RESERVED) / 0.0;
        let sys = if 0.0 < host_quotient {
            scale_from_quotient(host_quotient, width, height)
        } else {
            0.0
        };
        f32::min(gpu, sys)
    } else {
        let unified = (device_local + host) as f64;
        scale_from_budget(unified, width, height, UNIFIED_RESERVED)
    };

    (scale * 10.0).floor() * 0.1
}

/// Shared arithmetic for `calculate_max_resolution_scale` (discrete branch).
fn scale_from_budget(budget: f64, width: f64, height: f64, reserved: f64) -> f32 {
    let quotient = (width * -BYTES_PER_UNIT * height + budget + -reserved) / COST_PER_SCALED_UNIT;
    scale_from_quotient(quotient, width, height)
}

/// Second half of the original computation, factored only for readability.
/// It is inlined into `calculate_max_resolution_scale` in the original body.
fn scale_from_quotient(quotient: f64, width: f64, height: f64) -> f32 {
    if 0.0 < quotient {
        let area = width * height;
        let per_pixel = f64::max(quotient / area, 0.0);
        let smallest_axis = f64::max(f64::min(width, height), 1.0);
        let axis_limit = MAX_AXIS / smallest_axis;
        let scale = f64::min(per_pixel.sqrt(), axis_limit);
        scale as f32
    } else {
        0.0
    }
}

/// B `tiny_glade::vram::calculate_resolution_scale_mem_estimate_text`.
///
/// Returns the panel label observed in `.rdata`:
/// * discrete: `"GPU: ~{usage} GiB / {budget} GiB\nSYS: ~{sys_usage} GiB / {host_budget} GiB"`
/// * unified:  `"iGPU: ~{usage} GiB / {budget} GiB"`
///
/// Usage is `(w*h*scale^2*146.1091661136655 + w*h*47.7399274329577 + reserve)`
/// bytes converted to GiB; budgets are converted from the budget extension.
pub fn calculate_resolution_scale_mem_estimate_text(
    device: &Device,
    render_resolution: [u32; 2],
    resolution_scale: f32,
) -> String {
    let physical = device
        .physical_device_info()
        .expect("physical device info must be initialised");
    let width = render_resolution[0] as f64;
    let height = render_resolution[1] as f64;
    let scale = resolution_scale as f64;

    let pixel_bytes = width * height * BYTES_PER_UNIT;
    let scaled_bytes = width * scale * height * scale * COST_PER_SCALED_UNIT;

    let device_local = physical
        .device_local_memory_budget_bytes()
        .ok()
        .flatten()
        .unwrap_or(0);
    let host = physical
        .host_memory_budget_bytes()
        .ok()
        .flatten()
        .unwrap_or(0);

    if physical.properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
        // 原版先把字节和 `as u64`（饱和截断）再乘 2^-30；`0.0 * x` 保留，
        // 使 inf/NaN 时与原机器码同为 0。
        let usage_gib = ((scaled_bytes + pixel_bytes + DISCRETE_RESERVED) as u64) as f64 * GIB;
        let sys_usage_gib =
            ((scaled_bytes * 0.0 + pixel_bytes * 0.0 + SYSTEM_RESERVED) as u64) as f64 * GIB;
        let device_gib = device_local as f64 * GIB;
        let host_gib = host as f64 * GIB;
        format!(
            "GPU: ~{usage_gib:.2} GiB / {device_gib:.2} GiB\nSYS: ~{sys_usage_gib:.2} GiB / {host_gib:.2} GiB"
        )
    } else {
        let usage_gib = ((scaled_bytes + pixel_bytes + UNIFIED_RESERVED) as u64) as f64 * GIB;
        let budget_gib = (device_local + host) as f64 * GIB;
        format!("iGPU: ~{usage_gib:.2} GiB / {budget_gib:.2} GiB")
    }
}
