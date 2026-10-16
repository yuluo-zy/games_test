//! 多帧渲染动画测试：三角形每帧旋转 15°，连续渲染 48 帧，双缓冲（2 帧在飞）。
//!
//! 检查的是“多帧之间有没有问题”，不是单帧正确性：
//! - 每帧像素都和 CPU 参考光栅化一致（旋转后的覆盖区域，容许边缘个别像素）；
//! - 红色像素的质心沿预期圆周轨迹移动（帧间没有串帧、没有复用到旧内容）；
//! - 每帧清除色都不同，且清除色在三角形外全部生效（上一帧内容不泄漏）；
//! - 真的出现过 2 帧同时在飞；每帧新建的顶点缓冲在命令缓冲持有下安全退役（循环里不崩、不报错）。
//!
//! `cargo test -p rhapsody --test triangle_animation -- --nocapture` 会打印第 0/6/12/18 帧的 ASCII 预览。
//! 没有 Vulkan 设备时跳过。
mod draw_support;
use draw_support::*;
use rhapsody::raw::command_buffer::PendingCommand;

const FRAMES: usize = 48;
const IN_FLIGHT: usize = 2;
const STEP_DEG: f32 = 15.0;

/// 基础三角形绕 NDC 原点旋转 `angle` 弧度。
fn rotated(angle: f32) -> Triangle {
    let base: Triangle = [[-0.5, -0.5, 0.0, 1.0], [0.5, -0.5, 0.0, 1.0], [0.0, 0.5, 0.0, 1.0]];
    let (s, c) = angle.sin_cos();
    base.map(|v| [v[0] * c - v[1] * s, v[0] * s + v[1] * c, 0.0, 1.0])
}

/// 每帧不同的清除色（蓝分量始终 ≥ 155，不会和红色三角形混淆）。
fn clear_color(frame: usize) -> [u8; 4] {
    [(frame * 5 % 100) as u8, (frame * 9 % 200) as u8, 255 - (frame * 3 % 100) as u8, 255]
}

struct Slot {
    target: Target,
    pending: Option<(usize, PendingCommand)>,
}

struct Stats {
    frames_checked: usize,
    worst_mismatch: usize,
    worst_centroid_error: f32,
}

fn verify(frame: usize, target: &Target, stats: &mut Stats) {
    let pixels = read_pixels(target);
    let triangle = rotated(frame as f32 * STEP_DEG.to_radians());
    let covered = reference_coverage(&triangle);
    let clear = clear_color(frame);

    let mut mismatches = 0;
    let (mut sum_x, mut sum_y, mut filled) = (0.0f32, 0.0f32, 0usize);
    for y in 0..SIZE {
        for x in 0..SIZE {
            let p = pixel(&pixels, x, y);
            let is_fill = p == FILL;
            assert!(is_fill || p == clear, "第 {frame} 帧 ({x},{y}) 出现既非填充色也非本帧清除色的像素 {p:?}（上一帧内容泄漏？）");
            if is_fill != covered[(y * SIZE + x) as usize] {
                mismatches += 1;
            }
            if is_fill {
                sum_x += x as f32 + 0.5;
                sum_y += y as f32 + 0.5;
                filled += 1;
            }
        }
    }
    assert!(mismatches <= 8, "第 {frame} 帧与 CPU 参考光栅化有 {mismatches} 个像素不一致");
    assert!((470..=555).contains(&filled), "第 {frame} 帧三角形像素数 {filled} 偏离 512（旋转不改面积）");

    // 解析质心：重心 (0,-1/6) 随同一旋转移动。
    let a = frame as f32 * STEP_DEG.to_radians();
    let (s, c) = a.sin_cos();
    let (cx, cy) = (s * (1.0 / 6.0), -c * (1.0 / 6.0));
    let expected = ((cx + 1.0) * 0.5 * SIZE as f32, (cy + 1.0) * 0.5 * SIZE as f32);
    let measured = (sum_x / filled as f32, sum_y / filled as f32);
    let error = ((measured.0 - expected.0).powi(2) + (measured.1 - expected.1).powi(2)).sqrt();
    assert!(error < 0.8, "第 {frame} 帧质心 {measured:?} 偏离预期 {expected:?}（{error:.2} px）");

    if frame.is_multiple_of(6) {
        println!("--- 第 {frame} 帧（旋转 {:.0}°，清除色 {clear:?}）质心 ({:.1},{:.1}) ---", frame as f32 * STEP_DEG, measured.0, measured.1);
        for y in (0..SIZE).step_by(4) {
            let row: String = (0..SIZE).step_by(2).map(|x| if pixel(&pixels, x, y) == FILL { '#' } else { '.' }).collect();
            println!("{row}");
        }
    }
    stats.frames_checked += 1;
    stats.worst_mismatch = stats.worst_mismatch.max(mismatches);
    stats.worst_centroid_error = stats.worst_centroid_error.max(error);
}

#[test]
fn a_rotating_triangle_renders_correctly_across_many_frames_in_flight() {
    let Some(rig) = Rig::new() else { return };
    let mut slots: Vec<Slot> = (0..IN_FLIGHT)
        .map(|i| Slot { target: rig.new_target(&format!("frame target {i}")), pending: None })
        .collect();
    let mut stats = Stats { frames_checked: 0, worst_mismatch: 0, worst_centroid_error: 0.0 };
    let mut max_in_flight = 0;

    for frame in 0..FRAMES {
        let slot = &mut slots[frame % IN_FLIGHT];
        // 帧节流：复用该槽前，先等它上一次的提交完成并校验。
        if let Some((old_frame, pending)) = slot.pending.take() {
            pending.wait(u64::MAX).unwrap();
            verify(old_frame, &slot.target, &mut stats);
        }
        let pending = rig.draw(&slot.target, &rotated(frame as f32 * STEP_DEG.to_radians()), clear_color(frame));
        slot.pending = Some((frame, pending));
        max_in_flight = max_in_flight.max(slots.iter().filter(|s| s.pending.is_some()).count());
    }
    // 排空尾部：按提交顺序等待并校验。
    let mut tail: Vec<usize> = (0..IN_FLIGHT).collect();
    tail.sort_by_key(|&i| slots[i].pending.as_ref().map(|(f, _)| *f));
    for i in tail {
        if let Some((frame, pending)) = slots[i].pending.take() {
            pending.wait(u64::MAX).unwrap();
            verify(frame, &slots[i].target, &mut stats);
        }
    }

    println!(
        "共校验 {} 帧；最大在飞帧数 {max_in_flight}；与参考光栅化最多 {} 个像素不一致；质心最大误差 {:.2} px",
        stats.frames_checked, stats.worst_mismatch, stats.worst_centroid_error
    );
    assert_eq!(stats.frames_checked, FRAMES, "每一帧都必须被校验到");
    assert_eq!(max_in_flight, IN_FLIGHT, "测试必须真的出现多帧同时在飞，否则没有验证到帧间重叠");
    // 全部结束后设备应可正常空闲（没有遗留的错误状态）。
    rig.device.wait_idle().unwrap();
}
