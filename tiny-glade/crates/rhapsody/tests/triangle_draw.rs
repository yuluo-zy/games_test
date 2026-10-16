//! 三角形绘制 + 像素读回 demo（rhapsody 内部、无窗口，lavapipe 可跑）。
//!
//! 走的全是 rhapsody 自己的接口：`GraphicsDevice::create_render_pass`（`RenderPassDesc` 缓存）、
//! `create_framebuffer`、`VulkanPipeline::create_raster_pipeline`（`RasterPipelineOpts` 位域给固定状态）、
//! `create_buffer`（顶点上传）、`VulkanCommandPool`/`PendingCommand`（录制与提交），最后把附件拷回缓冲逐像素检查。
//! `cargo test -p rhapsody --test triangle_draw -- --nocapture` 会打印一张 ASCII 预览。
//! 没有 Vulkan 设备时跳过。
mod draw_support;
use draw_support::*;

const CLEAR: [u8; 4] = [0, 0, 255, 255];

#[test]
fn a_triangle_is_rasterized_and_the_pixels_read_back() {
    let Some(rig) = Rig::new() else { return };
    let target = rig.new_target("triangle target");
    // NDC，覆盖画面中央，面积 0.5 / 4 = 12.5% = 512 像素；A、B 在上（y 向下），C 在下。
    let triangle: Triangle = [
        [-0.5, -0.5, 0.0, 1.0],
        [0.5, -0.5, 0.0, 1.0],
        [0.0, 0.5, 0.0, 1.0],
    ];
    let pending = rig.draw(&target, &triangle, CLEAR);
    pending.wait(u64::MAX).unwrap();

    let pixels = read_pixels(&target);
    // 预览：# 为三角形（红），. 为清除色（蓝）。
    for y in (0..SIZE).step_by(2) {
        let row: String = (0..SIZE)
            .map(|x| {
                if pixel(&pixels, x, y) == FILL {
                    '#'
                } else {
                    '.'
                }
            })
            .collect();
        println!("{row}");
    }
    let count = |color| {
        (0..SIZE)
            .flat_map(|y| (0..SIZE).map(move |x| (x, y)))
            .filter(|&(x, y)| pixel(&pixels, x, y) == color)
            .count()
    };
    let (filled, cleared) = (count(FILL), count(CLEAR));
    assert_eq!(
        filled + cleared,
        (SIZE * SIZE) as usize,
        "只应出现填充色或清除色（不混合、无抗锯齿）"
    );
    assert!(
        (480..=545).contains(&filled),
        "三角形像素数 {filled} 偏离 512"
    );
    assert_eq!(pixel(&pixels, 32, 32), FILL, "画面中心在三角形内");
    assert_eq!(
        pixel(&pixels, 32, 24),
        FILL,
        "NDC(0,-0.25) 在三角形内（宽 0.75）"
    );
    assert_eq!(pixel(&pixels, 2, 2), CLEAR, "左上角在三角形外");
    assert_eq!(pixel(&pixels, 61, 61), CLEAR, "右下角在三角形外");
    assert_eq!(pixel(&pixels, 32, 8), CLEAR, "NDC y=-0.75 在三角形上方");
    assert_eq!(pixel(&pixels, 32, 58), CLEAR, "NDC y=0.81 在顶点 C 之下");
    let width = |y: u32| (0..SIZE).filter(|&x| pixel(&pixels, x, y) == FILL).count();
    assert!(
        width(20) > width(40),
        "越靠近顶点 C（y 大）越窄：{} vs {}",
        width(20),
        width(40)
    );
}
