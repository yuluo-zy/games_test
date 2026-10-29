//! B 版抹灰构造与生命周期。
//!
//! `maintain_plaster@14075b810` 和 `create_plaster@14075d460` 的事件、
//! 剥落、卸载及网格调用顺序为当前候选；原完整调用链和跨平台数值尚未验收。
//! `generate_plaster_mesh_and_entities@1412269b0` 的采样、顶点、条带、设备
//! 所有者和实体输出为候选，尚未完成真实设备及原完整调用链验收。原证与
//! 验证边界见 `reports/business/2026-10-23-module-recovery/m6-plaster-recovery-2026-10-29.md`。

use bevy_ecs::prelude::*;
use country_core::render::device::GraphicsDeviceResource;
use country_core::render::ssbo::rhapsody_boundary::{BackendError, Buffer, BufferDesc, GraphicsDevice};
use country_core::resources::{
    roofs::Roof,
    walls::{
        plaster::{CreatePlasterCmd, DeletePlasterCmd, RecalculatePlasterCmd},
        public_walls::{PublicWalls, WallPlasters},
        wall_holes::WallHoles,
    },
};

/// 原抹灰缓冲拥有者随实体卸载释放；当前设备把物理退役交给帧租约和 fence。
#[derive(Component)]
pub struct PlasterBufferOwner {
    device: std::sync::Arc<dyn GraphicsDevice + Send + Sync>,
    buffers: Vec<Buffer>,
}

impl PlasterBufferOwner {
    fn new(device: &GraphicsDeviceResource, buffers: Vec<Buffer>) -> Self {
        Self { device: std::sync::Arc::new(device.device().clone()), buffers }
    }

    /// 原 `141228...` 将48B顶点数组上传为 GPU-only `plaster_side`。
    /// 成功后立刻归属本 owner，后续步骤失败时 Drop 仍释放已获缓冲。
    fn upload_side(&mut self, bytes: &[u8]) -> Result<(Buffer, u64), BackendError> {
        assert_eq!(bytes.len() % 48, 0, "原抹灰顶点记录为48字节");
        let desc = BufferDesc::new_gpu_only(bytes.len(), 0x20020);
        let buffer = self.device.create_buffer(&desc, "plaster_side", Some(bytes))?;
        self.buffers.push(buffer);
        let address = self.device.device_address(&buffer);
        Ok((buffer, address))
    }
}

impl Drop for PlasterBufferOwner {
    fn drop(&mut self) {
        for buffer in self.buffers.drain(..) { self.device.release_buffer(&buffer); }
    }
}

/// 原 `14075c130..14075c305` 的单墙剥落更新。返回新值、是否发音、
/// 是否加入重建队列；首次观察直接采用墙值。实际音频与重建由维护系统负责。
fn advance_plaster_peel(previous: Option<f32>, target: f32, delta: f32) -> (f32, bool, bool) {
    let Some(previous) = previous else { return (target, false, false); };
    // 原调用中 XMM2=delta、XMM3=7；保留该乘法操作数顺序。
    let decay = (-delta * 7.0).exp();
    let interpolated = (target - previous) * (1.0 - decay) + previous;
    let audible = !((interpolated - target).abs() < f32::from_bits(0x3c23d70a));
    let value = if audible { interpolated } else { target };
    (value, audible, value != target)
}

fn plaster_audio_position(wall: &country_core::resources::walls::public_walls::PublicWallState) -> glam::Vec3 {
    let center = wall.try_get_wall_shape().map(|shape|
        country_core::systems::wall::valid_enclosure::ShapeParameters::from_snapshot(shape).center())
        .unwrap_or(glam::Vec2::ZERO);
    glam::Vec3::new(center.x, (wall.authority.elevation() + wall.max_y()) * 0.5, center.y)
}

/// 原生成器 141226cc0..e40：先只取 origin=0，再为闭合曲线近缝洞追加平移副本。
/// 顺序及每个原洞最多一个副本与原顺序一致；输出保留原中心/半尺寸。
fn plaster_hole_boxes(
    holes: &[country_core::resources::walls::wall_holes::FinalizedWallHole],
    wall_length: f32,
    shape_tag: u32,
) -> Vec<[f32; 4]> {
    let mut boxes: Vec<_> = holes.iter().filter(|hole| hole.origin == 0).map(|hole| {
        let a = glam::Vec2::new(hole.aabb[0], hole.aabb[1]);
        let b = glam::Vec2::new(hole.aabb[2], hole.aabb[3]);
        let center = (a + b) * 0.5;
        let half = (b - a) * 0.5;
        [center.x, center.y, half.x, half.y]
    }).collect();
    if shape_tag != 2 {
        let primary_count = boxes.len();
        for i in 0..primary_count {
            let mut duplicate = boxes[i];
            let right = duplicate[0] + duplicate[2] + 1.0;
            // 原 UCOMISS/JBE 对 unordered 也进入左侧检查。
            if right <= wall_length || right.is_nan() || wall_length.is_nan() {
                if duplicate[0] - duplicate[2] - 1.0 < 0.0 {
                    duplicate[0] += wall_length;
                } else { continue; }
            } else {
                duplicate[0] -= wall_length;
            }
            boxes.push(duplicate);
        }
    }
    boxes
}

/// 原 141227... 洞影响的半开网格范围。转换采用 Rust 的饱和 float→i32
/// 合同（NaN 为0），与原 ceilf 后 cvttss2si 饱和修正一致。
fn plaster_hole_grid_bounds(
    hole: [f32; 4], origin: glam::Vec2, inverse_step: glam::Vec2, counts: [u16; 2],
) -> Option<[u32; 4]> {
    let start_x = ((((hole[0] - hole[2]) - 1.0) - origin.x) * inverse_step.x).ceil() as i32;
    let start_y = ((((hole[1] - hole[3]) - 1.0) - origin.y) * inverse_step.y).ceil() as i32;
    let end_x = ((((hole[2] + hole[0]) + 1.0) - origin.x) * inverse_step.x).ceil() as i32;
    let end_y = ((((hole[3] + hole[1]) + 1.0) - origin.y) * inverse_step.y).ceil() as i32;
    let start_x = start_x.max(0) as u32;
    let start_y = start_y.max(0) as u32;
    let end_x = end_x.min(i32::from(counts[0]));
    let end_y = end_y.min(i32::from(counts[1]));
    if end_x <= start_x as i32 || end_y <= start_y as i32 { return None; }
    Some([start_x, start_y, end_x as u32, end_y as u32])
}

/// B 141227fb4..14122802f：矩形孔对单个网格距离的合并。
/// 调用者先按原 fmaf 生成相对孔中心的 x/y 坐标；此处保留原
/// 有序 max/min 和 NaN 选择，不以 Rust 的 clamp 替代。
fn plaster_hole_distance_update(delta: glam::Vec2, half: glam::Vec2, old: f32) -> f32 {
    let x = delta.x.abs() - half.x;
    let y = delta.y.abs() - half.y;
    let positive_x = if x.is_nan() { 0.0 } else if x > 0.0 { x } else { 0.0 };
    let positive_y = if y.is_nan() { 0.0 } else if y > 0.0 { y } else { 0.0 };
    let maximum = if x.is_nan() { y } else if y.is_nan() || x >= y { x } else { y };
    // MINSS 的源零在相等或 unordered 时胜出。
    let inside = if maximum < 0.0 { maximum } else { 0.0 };
    let distance = inside + (positive_y * positive_y + positive_x * positive_x).sqrt();
    if old.is_nan() { distance } else if distance < old { distance } else { old }
}

/// 原生成器每点临时记录；末尾三字节为布局填充，未被下游读取。
#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct PlasterGridSample {
    u: f32, y: f32, hole_distance: f32, distance: f32,
    normal: [f32; 3], outside: u8, padding: [u8; 3],
}

/// 141227952..14122794f：已由原指令准备好的列值生成一行样本。
/// 列级曲线插值与屋顶高度由调用者按原算法提供。
fn plaster_grid_sample_row(
    u: f32, y: f32, top: f32, cutoff: f32, normal: [f32; 3],
    flat_roof: Option<f32>, ridge: Option<f32>, freeform_edge: Option<f32>,
) -> PlasterGridSample {
    // 原 MINSS 及 unordered mask：NaN 取另一操作数，相等保留左值。
    fn min_native(a: f32, b: f32) -> f32 {
        if a.is_nan() { b } else if b.is_nan() || a <= b { a } else { b }
    }
    let mut distance = y - top;
    if let Some(side_top) = flat_roof { distance = min_native(distance, (side_top - y) + -0.3); }
    if let Some(ridge_y) = ridge {
        distance = min_native(distance, (ridge_y - 0.46) - y);
        if let Some(edge) = freeform_edge { distance = min_native(distance, edge); }
    }
    PlasterGridSample { u, y, hole_distance: 1.0, distance, normal,
        outside: u8::from(y < cutoff || ridge.is_some_and(|ridge_y| ridge_y < y)), padding: [0; 3] }
}

/// 采样单侧网格。调用侧保留原 prepared 曲线和两组高度的相同索引。
fn sample_plaster_side_into(
    wall: &country_core::resources::walls::public_walls::PublicWallState,
    roof: Option<&Roof>, side: &WallSide, holes: &[[f32; 4]],
    samples: &mut Vec<PlasterGridSample>,
) {
    use utils::curve::{curve_slice::CurveSlice, CurveCoordSemanticThresholded3D};
    let path = wall.prepared_path.as_ref().expect("原抹灰采样要求 prepared Curve3");
    let points: Vec<glam::Vec3> = path.positions.iter().copied().map(glam::Vec3::from_array).collect();
    let count_x = (side.grid & 0xffff) as usize;
    let count_y = (side.grid >> 16) as usize;
    let count = count_x.checked_mul(count_y).expect("抹灰网格数量溢出");
    assert!(samples.capacity() >= count, "原抹灰临时网格容量不足");
    samples.clear();
    samples.resize(count, PlasterGridSample { u: 0., y: 0., hole_distance: 1., distance: 0., normal: [0.; 3], outside: 0, padding: [0; 3] });
    let length = wall.wall_space_length();
    let elevation = wall.authority.elevation();
    let height = if roof.is_some_and(Roof::is_flat_w_no_rim) { (side.y_offset - 0.79) + 0.15 } else { side.y_offset };
    let flat = roof.is_some_and(Roof::is_flat);
    let tag = wall.shape().tag();
    let use_ridge = tag >= 2 || side.roof_flag & 1 != 0;
    for x in 0..count_x {
        let t = x as f32 / (count_x as f32 - 1.0);
        let u = (t * side.u_end + (1.0 - t) * side.u_begin) * length;
        let curve: CurveSlice<glam::Vec3, &[glam::Vec3], CurveCoordSemanticThresholded3D> = CurveSlice::from_point_range(points.as_slice(), 0..=0);
        let mut curve = curve.remap_2d();
        curve.extend_right_until(if u > 0.0 { u } else { 0.0 }, |_| false);
        let coord = curve.end(); let i = coord.segment; let k = coord.t;
        let a = points[i]; let b = points[i + 1];
        let base = (b.y - a.y) * k + a.y;
        let segment = (wall.segment_heights[i + 1] - wall.segment_heights[i]) * k + wall.segment_heights[i];
        let top = if segment > 0.0 { base + segment } else { -f32::MAX };
        let cutoff = base - if elevation > 0.0 { 0.5 } else { 0.0 };
        let ridge_y = if use_ridge {
            let heights = wall.arch_segment_heights().expect("原抹灰采样要求 arch 高度");
            Some((heights[i + 1] - heights[i]) * k + heights[i])
        } else { None };
        let normal = if tag < 2 {
            let normal_u = u / length;
            assert!((0.0..=1.0).contains(&normal_u), "原形状法线参数越界或 NaN");
            let n = if tag == 0 { wall.is_circle().expect("原圆形数据").get_normal_at_u(normal_u) }
                else { wall.is_rectangle().expect("原矩形数据").get_normal_at_u(normal_u) };
            [n.x, 0.0, n.y]
        } else {
            let d = b - a;
            let scale = 1.0 / (d.z * d.z + (d.y * d.y + d.x * d.x)).sqrt();
            let dx = d.x * scale; let dz = d.z * scale;
            let scale = 1.0 / (dz * dz + dx * dx).sqrt();
            [-dz * scale, 0.0, dx * scale]
        };
        let edge = if tag == 2 && use_ridge {
            let other = length - u;
            Some(if u.is_nan() { other } else if other.is_nan() || u <= other { u } else { other })
        } else { None };
        for y in 0..count_y {
            let yf = y as f32 / (count_y as f32 - 1.0);
            samples[y * count_x + x] = plaster_grid_sample_row(u, elevation + yf * height,
                top, cutoff, normal, flat.then_some(side.height), ridge_y, edge);
        }
    }
    let origin = glam::Vec2::new(length * side.u_begin, elevation);
    let span_x = length * side.u_end - origin.x;
    let span_y = (height + elevation) - elevation;
    let inv = glam::Vec2::new((count_x as f32 - 1.0) / span_x, (count_y as f32 - 1.0) / span_y);
    let step_x = ((side.u_end - side.u_begin) * length) / (count_x as f32 - 1.0);
    let step_y = height / (count_y as f32 - 1.0);
    for &hole in holes {
        if let Some([x0, y0, x1, y1]) = plaster_hole_grid_bounds(hole, origin, inv, [count_x as u16, count_y as u16]) {
            for x in x0..x1 { for y in y0..y1 {
                let dx = step_x.mul_add(x as f32, origin.x - hole[0]);
                let dy = step_y.mul_add(y as f32, elevation - hole[1]);
                let p = &mut samples[y as usize * count_x + x as usize];
                p.hole_distance = plaster_hole_distance_update(glam::Vec2::new(dx, dy), glam::Vec2::new(hole[2], hole[3]), p.hole_distance);
            }}
        }
    }
}

#[cfg(test)]
fn sample_plaster_side(
    wall: &country_core::resources::walls::public_walls::PublicWallState,
    roof: Option<&Roof>, side: &WallSide, holes: &[[f32; 4]],
) -> Vec<PlasterGridSample> {
    let count = (side.grid & 0xffff) as usize * (side.grid >> 16) as usize;
    let mut samples = Vec::with_capacity(count);
    sample_plaster_side_into(wall, roof, side, holes, &mut samples);
    samples
}

/// 原 14122839e..1412284a0：已转换世界坐标的样本编码成 48B 顶点。
/// 输出为逐位 u32，避免 GPU 上传时改变 NaN payload 或填充。
fn encode_plaster_vertex(sample: PlasterGridSample, world: glam::Vec3, face: f32) -> [u32; 12] {
    let d = sample.distance;
    let positive = if d.is_nan() || d >= 0.0 { d } else { 0.0 };
    let t = if positive.is_nan() || positive <= 1.0 { positive } else { 1.0 };
    let t2 = t * t;
    let smooth = 1.0 - (3.0 - (t + t)) * t2;
    let amount = smooth * 5.0 + (1.0 - smooth);
    let merged = if sample.hole_distance.is_nan() { d }
        else if d.is_nan() || sample.hole_distance <= d { sample.hole_distance } else { d };
    let n = sample.normal;
    let offset = f32::from_bits(0x3e9851ec);
    [
        (n[0] * offset * face + world.x).to_bits(),
        (n[1] * offset * face + world.y).to_bits(),
        (n[2] * offset * face + world.z).to_bits(), merged.to_bits(),
        (face * n[0]).to_bits(), (face * n[1]).to_bits(), (face * n[2]).to_bits(), amount.to_bits(),
        sample.u.to_bits(), sample.y.to_bits(), 0, 0,
    ]
}

/// 原 32B 条带记录。设备地址定位首顶点，stride 以顶点数计。
#[derive(Clone, Copy, Debug)]
struct PlasterStrip { base: u64, cells: u32, stride: i32, bounds: [f32; 4] }

/// B 141228603..7d0：全 outside 的四角格跳过；有效格按行连续分组，最多32格。
fn plaster_strips(grid: &[PlasterGridSample], nx: usize, ny: usize, address: u64, reverse: bool) -> Vec<PlasterStrip> {
    let mut out: Vec<PlasterStrip> = Vec::new();
    for y in 0..ny.saturating_sub(1) {
        let mut fresh = true;
        for x in 0..nx.saturating_sub(1) {
            let a = y * nx + x; let b = a + nx;
            if grid[a].outside == 1 && grid[a + 1].outside == 1 && grid[b].outside == 1 && grid[b + 1].outside != 0 {
                fresh = true; continue;
            }
            if fresh {
                let base = if reverse { b } else { a };
                out.push(PlasterStrip { base: address.wrapping_add((base as u64).wrapping_mul(48)), cells: 0,
                    stride: if reverse { -(nx as i32) } else { nx as i32 }, bounds: [0.; 4] });
            }
            let last = out.last_mut().expect("原抹灰条带记录缺失");
            last.cells += 1;
            fresh = last.cells >= 32;
        }
    }
    out
}

/// B 141228815..9e7：对条带两行各 cells+1 个顶点求中心与包围半径。
fn plaster_strip_bounds(strip: &mut PlasterStrip, vertices: &[[u32; 12]], address: u64) {
    let start = (strip.base.wrapping_sub(address) / 48) as i32;
    let mut sum = [0.0f32; 3]; let mut count = 0.0f32;
    for i in 0..=strip.cells {
        let a = vertices[(start + i as i32) as usize];
        let b = vertices[(start + i as i32 + strip.stride) as usize];
        sum[0] = f32::from_bits(b[0]) + (sum[0] + f32::from_bits(a[0]));
        sum[1] = f32::from_bits(b[1]) + (sum[1] + f32::from_bits(a[1]));
        sum[2] = (sum[2] + f32::from_bits(a[2])) + f32::from_bits(b[2]);
        count += 2.0;
    }
    let inverse = 1.0 / count;
    let center = [inverse * sum[0], inverse * sum[1], inverse * sum[2]];
    let mut max_sq = 0.0f32;
    for i in 0..=strip.cells {
        for index in [start + i as i32, start + i as i32 + strip.stride] {
            let p = vertices[index as usize];
            let x = center[0] - f32::from_bits(p[0]);
            let y = center[1] - f32::from_bits(p[1]);
            let z = center[2] - f32::from_bits(p[2]);
            let sq = z * z + (x * x + y * y);
            max_sq = if max_sq.is_nan() { sq } else if sq > max_sq { sq } else { max_sq };
        }
    }
    strip.bounds = [center[0], center[1], center[2], max_sq.sqrt()];
}

/// B `maintain_plaster`（0x14075b810）：石膏命令消费/实体生命周期维护。
/// 候选维护控制流；非空网格生成仍显式阻断，尚未完成原侧整链差分。
pub fn maintain_plaster(
    mut plasters: ResMut<WallPlasters>,
    graphics: Res<GraphicsDeviceResource>,
    walls: Res<PublicWalls>,
    holes: Res<WallHoles>,
    roofs: Query<&Roof>,
    mut commands: Commands,
    mut recalc: EventReader<RecalculatePlasterCmd>,
    mut delete: EventReader<DeletePlasterCmd>,
    mut audio: EventWriter<country_core::systems::audio::PlayBuildingAudio>,
    mut peel: ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,
    time: Res<country_core::resources::time::GameTime>,
) {
    // 原先消费删除事件并卸载已存在实体，再处理重算与剥落。
    for event in delete.read() {
        if let Some(entities) = plasters.0.remove(&event.0) {
            for entity in entities.into_iter().flatten() {
                commands.entity(entity).despawn();
            }
        }
    }
    let mut rebuild: Vec<_> = recalc.read().map(|event| event.0).collect();
    for (id, wall) in walls.all() {
        let target = wall.authority.plaster_peel_amount();
        let (value, audible, changed) = advance_plaster_peel(peel.get(id), target, time.delta);
        if audible {
            audio.write(country_core::systems::audio::PlayBuildingAudio {
                id: id.0, position: plaster_audio_position(wall), editing_speed: 1.0, kind: 9,
            });
        }
        peel.0.insert(id, country_core::resources::walls::plaster::PlasterPeelAmount(value));
        if changed { rebuild.push(id); }
    }
    peel.0.retain(|id, _| walls.get_unchecked(*id).is_some());
    rebuild.sort_unstable_by_key(|id| id.0);
    rebuild.dedup();
    for id in rebuild {
        // B 14075c6b8..c6c2：无旧记录的 niche 直接跳过，甚至不查询墙。
        let Some(entities) = plasters.0.remove(&id) else { continue; };
        for entity in entities.into_iter().flatten() {
            commands.entity(entity).despawn();
        }
        let wall = walls.get(id).expect("原 maintain_plaster: wall missing from PublicWalls");
        if wall.authority.is_special() { continue; }
        let finalized = holes.storage.get(&id).map(|storage|
            storage.finalized_holes().expect("原 maintain_plaster: WallHoles 未完成"))
            .unwrap_or(&[]);
        let roof = walls.roof_entity(id).and_then(|entity| roofs.get(entity).ok());
        generate_plaster_mesh_and_entities(&mut plasters, &graphics, &mut commands,
            id, wall, roof, finalized);
    }
}

/// B `create_plaster`（0x14075d460）的候选事件消费；调用的网格生成器仍未完成。
pub fn create_plaster(
    mut plasters: ResMut<WallPlasters>,
    graphics: Res<GraphicsDeviceResource>,
    walls: Res<PublicWalls>,
    holes: Res<WallHoles>,
    roofs: Query<&Roof>,
    mut commands: Commands,
    mut create: EventReader<CreatePlasterCmd>,
) {
    for event in create.read() {
        let wall = walls.get(event.0).expect("原 create_plaster: wall missing from PublicWalls");
        let finalized = holes.storage.get(&event.0).map(|storage|
            storage.finalized_holes().expect("原 create_plaster: WallHoles 未完成"))
            .unwrap_or(&[]);
        let roof = walls.roof_entity(event.0).and_then(|entity| roofs.get(entity).ok());
        generate_plaster_mesh_and_entities(&mut plasters, &graphics, &mut commands,
            event.0, wall, roof, finalized);
    }
}

/// B `WallSide`（24B=0x18）：`{u_begin@0, u_end@4, height@8, y_offset@0xc,
/// grid u32@0x10（低 u16=水平格数，高 u16=高度格数）, roof_flag u8@0x14}`。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WallSide {
    pub u_begin: f32,
    pub u_end: f32,
    pub height: f32,
    pub y_offset: f32,
    pub grid: u32,
    pub roof_flag: u8,
    pub _pad: [u8; 3],
}

/// x86 `ROUNDSS`（四舍五入远离零），随后按原 `MAXSS 0` / `MINSS 65535`
/// 与 `PSUBUSW 0x00020002` 组合把格数钳到 `[2, 65535]`；NaN → 2。
fn side_grid_cells(value: f32) -> u16 {
    let rounded = value.round();
    let lowered = if 0.0 <= rounded { rounded } else { 0.0 };
    let clamped = if lowered <= 65535.0 { lowered } else { 65535.0 };
    let cells = clamped as i32 as u16;
    // psubusw(2, cells) + cells == max(cells, 2)。
    if cells < 2 { 2 } else { cells }
}

/// B `split_wall_into_sides`（0x14075dc60，3130B / 1228 行 fresh:
/// `decomp-0x14075dc60.txt`）。
///
/// - `(shape_tag & 1) == 0`（圆/自由曲线）：单条 `[0.0, 1.0]` 侧；
/// - 矩形：四条边 `get_point_u(i)..get_point_u(i+1)`（i=0..3），
///   `u_begin += 1e-7`、`u_end -= 1e-7`；
/// - `height`：无屋顶/非 gable 时全部 `max_y`；gable 方向 0 抬升边 0/2，
///   方向非 0 抬升边 1/3，抬升值 `max_y + Roof::height()`；
/// - `y_offset = height − wall_elevation`（公开记录 +0x280）；
/// - `grid = |max(round((u_end−u_begin)·curve_length/0.4), 2) 低 u16
///   | max(round(y_offset/0.4), 2) « 16|`；
/// - `roof_flag = 1` 当该边抬升（与 gable 方向一致）。
///
/// 原函数以 `SmallVec<WallSide,4>` 返回并写 `+0x60` 长度；Rust 侧用 `Vec`
/// 保持同一元素字节序（`WallSide` 布局与 24B 一致）。
pub fn split_wall_into_sides(
    rectangle: Option<utils::geometry::Rectangle2d>,
    max_y: f32,
    wall_elevation: f32,
    curve_length: f32,
    roof: Option<&country_core::resources::roofs::Roof>,
    out: &mut Vec<WallSide>,
) {
    let gable = roof.is_some_and(|roof| roof.is_gable());
    let roof_height = roof.map(|roof| roof.height()).unwrap_or(0.0);
    let direction = roof.map(|roof| roof.observed()[0x3c]).unwrap_or(0);
    let make = |u_begin: f32, u_end: f32, height: f32, flag: u8| {
        let y_offset = height - wall_elevation;
        let span = (u_end - u_begin) * curve_length;
        let horizontal = side_grid_cells(span / f32::from_bits(0x3ecccccd));
        let vertical = side_grid_cells(y_offset / f32::from_bits(0x3ecccccd));
        WallSide {
            u_begin,
            u_end,
            height,
            y_offset,
            grid: u32::from(horizontal) | (u32::from(vertical) << 16),
            roof_flag: flag,
            _pad: [0; 3],
        }
    };
    let Some(rectangle) = rectangle else {
        out.push(make(0.0, 1.0, max_y, 0));
        return;
    };
    let epsilon = f32::from_bits(0x33d6bf95); // 1e-7
    let raised = max_y + roof_height;
    for side in 0..4 {
        let u_begin = rectangle.get_point_u(side) + epsilon;
        let u_end = rectangle.get_point_u(side + 1) - epsilon;
        let use_raised = gable && (direction == 0) == (side % 2 == 0);
        let (height, flag) = if use_raised {
            (raised, 1)
        } else {
            (max_y, 0)
        };
        out.push(make(u_begin, u_end, height, flag));
    }
}

/// 从真实 PublicWall owner 读取原 +0x280 高程；供生成器调用。
fn split_public_wall_into_sides(
    wall: &country_core::resources::walls::public_walls::PublicWallState,
    roof: Option<&Roof>, out: &mut Vec<WallSide>,
) {
    use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};
    let rectangle = match ShapeParameters::from_snapshot(wall.shape()).get_shape() {
        Some(Shape::Rectangle(rectangle)) => Some(rectangle),
        _ => None,
    };
    split_wall_into_sides(rectangle, wall.max_y(), wall.authority.elevation(),
        wall.wall_space_length(), roof, out);
}

#[cfg(test)]
mod split_sides_tests {
    use super::*;
    use glam::Vec2;
    use utils::geometry::Rectangle2d;

    #[test]
    fn original_generator_device_failure_keeps_owner_and_successful_prefix() {
        use std::sync::{Arc, Mutex};
        use country_core::{resources::{roofs::geometry::surface::Curve2, walls::{WallId, WallState,
            public_walls::PublicWallState, height_cache::RegularWallHeightCache,
            publication_source::TerrainHeightGrid512}}, systems::wall::wall_state::path_preparation::PreparedWallPath};
        struct Device { first_succeeds: bool, log: Arc<Mutex<Vec<String>>> }
        impl GraphicsDevice for Device {
            fn create_buffer(&self, desc: &BufferDesc, _: &str, data: Option<&[u8]>) -> Result<Buffer, BackendError> {
                assert_eq!(desc.size, 2400); assert_eq!(data.unwrap().len(), 2400);
                let mut log = self.log.lock().unwrap(); let attempt = log.iter().filter(|s| s.starts_with("create")).count();
                log.push(format!("create {attempt}"));
                if self.first_succeeds && attempt == 0 { Ok(Buffer { id: 7, ..Default::default() }) }
                else { Err(BackendError::invalid_usage("原探针对应的设备创建失败")) }
            }
            fn update_buffer_bytes(&self, _: &Buffer, _: &[u8], _: &[std::ops::Range<usize>], _: usize) -> Result<(), BackendError> { unreachable!() }
            fn device_address(&self, _: &Buffer) -> u64 { 0x60000000 }
            fn release_buffer(&self, b: &Buffer) { self.log.lock().unwrap().push(format!("release {}", b.id)); }
        }
        let state = WallState::new(Curve2::try_new(vec![[0.,0.],[4.,0.]], false).unwrap(), 2., 0, 0, false);
        let mut snapshot = state.into_snapshot().0; snapshot.source.terrain_floor = 1.;
        let state = WallState::from_snapshot(snapshot, 0);
        let terrain = TerrainHeightGrid512 { encoded: vec![0;512*512].into(), minimum_y: 0., maximum_y: 0., sampling_fix: true };
        let mut wall = PublicWallState::collect(&state, &terrain).unwrap();
        wall.prepared_path = Some(PreparedWallPath { positions: vec![[0.,0.,0.],[4.,0.,0.]], points_u: vec![0.,1.], length: 4., segment_values: vec![0.;2] });
        wall.regular_heights = Some(RegularWallHeightCache { maximum_y: 3., arch_segment_heights: vec![3.;2] });
        wall.segment_heights = vec![2.;2];
        for first_succeeds in [false, true] {
            let log = Arc::new(Mutex::new(Vec::new()));
            let owner = PlasterBufferOwner { device: Arc::new(Device { first_succeeds, log: log.clone() }), buffers: Vec::new() };
            let mut world = World::new(); let mut registry = WallPlasters::default();
            { let mut commands = world.commands(); generate_plaster_with_owner(&mut registry, owner, &mut commands, WallId(42), &wall, None, &[]); }
            world.flush();
            let entities = registry.0.remove(&WallId(42)).unwrap();
            assert_eq!(entities.len(), if first_succeeds { 5 } else { 1 });
            assert!(world.get::<PlasterBufferOwner>(entities[0].unwrap()).is_some());
            assert_eq!(log.lock().unwrap().as_slice(), ["create 0", "create 1"]);
            for entity in entities.into_iter().flatten() { world.despawn(entity); }
            assert_eq!(log.lock().unwrap().as_slice(), if first_succeeds { &["create 0", "create 1", "release 7"][..] } else { &["create 0", "create 1"][..] });
        }
    }

    #[test]
    fn plaster_owner_releases_each_buffer_on_entity_despawn() {
        use country_core::render::ssbo::rhapsody_boundary::{BackendError, BufferDesc};
        use std::sync::{Arc, Mutex};
        struct Device(Arc<Mutex<Vec<u64>>>);
        impl GraphicsDevice for Device {
            fn create_buffer(&self, _: &BufferDesc, _: &str, _: Option<&[u8]>) -> Result<Buffer, BackendError> { unreachable!() }
            fn update_buffer_bytes(&self, _: &Buffer, _: &[u8], _: &[std::ops::Range<usize>], _: usize) -> Result<(), BackendError> { unreachable!() }
            fn release_buffer(&self, buffer: &Buffer) { self.0.lock().unwrap().push(buffer.id); }
        }
        let released = Arc::new(Mutex::new(Vec::new()));
        let mut world = World::new();
        let entity = world.spawn(PlasterBufferOwner {
            device: Arc::new(Device(released.clone())),
            buffers: vec![Buffer { id: 3, ..Default::default() }, Buffer { id: 8, ..Default::default() }],
        }).id();
        assert!(released.lock().unwrap().is_empty());
        world.despawn(entity);
        assert_eq!(*released.lock().unwrap(), [3, 8]);
    }

    #[test]
    fn plaster_upload_owns_successful_prefix_before_later_failure() {
        use std::sync::{Arc, Mutex};
        struct Device(Arc<Mutex<Vec<String>>>);
        impl GraphicsDevice for Device {
            fn create_buffer(&self, desc: &BufferDesc, name: &str, data: Option<&[u8]>) -> Result<Buffer, BackendError> {
                assert_eq!((desc.size, desc.usage, desc.gpu_only, name), (48, 0x20020, true, "plaster_side"));
                assert_eq!(data.unwrap().len(), 48);
                let mut calls = self.0.lock().unwrap();
                if !calls.is_empty() { return Err(BackendError::invalid_usage("forced second upload failure")); }
                calls.push("create".into());
                Ok(Buffer { id: 9, ..Default::default() })
            }
            fn update_buffer_bytes(&self, _: &Buffer, _: &[u8], _: &[std::ops::Range<usize>], _: usize) -> Result<(), BackendError> { unreachable!() }
            fn device_address(&self, _: &Buffer) -> u64 { 0x1000 }
            fn release_buffer(&self, buffer: &Buffer) { self.0.lock().unwrap().push(format!("release {}", buffer.id)); }
        }
        let calls = Arc::new(Mutex::new(Vec::new()));
        {
            let mut owner = PlasterBufferOwner { device: Arc::new(Device(calls.clone())), buffers: Vec::new() };
            assert_eq!(owner.upload_side(&[0; 48]).unwrap().1, 0x1000);
            assert!(owner.upload_side(&[0; 48]).is_err());
        }
        assert_eq!(*calls.lock().unwrap(), ["create", "release 9"]);
    }

    #[test]
    fn plaster_address_failure_unwinds_owned_buffer() {
        use std::sync::{Arc, Mutex};
        struct Device(Arc<Mutex<Vec<u64>>>);
        impl GraphicsDevice for Device {
            fn create_buffer(&self, _: &BufferDesc, _: &str, _: Option<&[u8]>) -> Result<Buffer, BackendError> {
                Ok(Buffer { id: 13, ..Default::default() })
            }
            fn update_buffer_bytes(&self, _: &Buffer, _: &[u8], _: &[std::ops::Range<usize>], _: usize) -> Result<(), BackendError> { unreachable!() }
            fn device_address(&self, _: &Buffer) -> u64 { panic!("address failure") }
            fn release_buffer(&self, buffer: &Buffer) { self.0.lock().unwrap().push(buffer.id); }
        }
        let released = Arc::new(Mutex::new(Vec::new()));
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut owner = PlasterBufferOwner { device: Arc::new(Device(released.clone())), buffers: Vec::new() };
            let _ = owner.upload_side(&[0; 48]);
        }));
        assert!(result.is_err());
        assert_eq!(*released.lock().unwrap(), [13]);
    }

    #[test]
    fn peel_first_sample_snap_and_active_animation_are_distinct() {
        assert_eq!(advance_plaster_peel(None, 0.8, 0.1), (0.8, false, false));
        // 原维护入口：old=0,target=0.5,delta=0.1，Darwin expf 桥接。
        let native = advance_plaster_peel(Some(0.0), 0.5, 0.1);
        assert_eq!((native.0.to_bits(), native.1, native.2), (0x3e80dfc9, true, true));
        assert_eq!(advance_plaster_peel(Some(0.795), 0.8, 0.0), (0.8, false, false));
        let (value, audible, rebuild) = advance_plaster_peel(Some(0.0), 1.0, 0.1);
        assert!(value > 0.0 && value < 1.0 && audible && rebuild);
        let (value, audible, rebuild) = advance_plaster_peel(Some(f32::NAN), 1.0, 0.1);
        assert!(value.is_nan() && audible && rebuild);
    }

    #[test]
    fn plaster_audio_uses_elevation_not_wall_height_and_freehand_origin() {
        use country_core::resources::{roofs::geometry::surface::Curve2, walls::{WallState,
            public_walls::PublicWallState, height_cache::RegularWallHeightCache,
            publication_source::TerrainHeightGrid512}};
        let state = WallState::new(Curve2::try_new(vec![[0., 0.], [1., 0.]], false).unwrap(), 7., 2, 0, false);
        let mut snapshot = state.into_snapshot().0;
        snapshot.source.terrain_floor = 4.;
        let state = WallState::from_snapshot(snapshot, 0);
        let terrain = TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(), minimum_y: 0., maximum_y: 0., sampling_fix: true };
        let mut wall = PublicWallState::collect(&state, &terrain).unwrap();
        wall.regular_heights = Some(RegularWallHeightCache { maximum_y: 10., arch_segment_heights: Vec::new() });
        assert_eq!(plaster_audio_position(&wall), glam::Vec3::new(0., 7., 0.));
        let mut sides = Vec::new();
        split_public_wall_into_sides(&wall, None, &mut sides);
        assert_eq!(sides[0].y_offset, 6.);
    }

    #[test]
    fn plaster_holes_keep_primary_and_append_only_original_seam_copies() {
        use country_core::resources::walls::wall_holes::FinalizedWallHole;
        let holes = [
            FinalizedWallHole { aabb: [0., 1., 1., 3.], kind: 0, origin: 0 },
            FinalizedWallHole { aabb: [9., 1., 10., 3.], kind: 0, origin: 0 },
            FinalizedWallHole { aabb: [10., 1., 11., 3.], kind: 0, origin: 1 },
        ];
        assert_eq!(plaster_hole_boxes(&holes, 10., 0), vec![
            [0.5, 2., 0.5, 1.], [9.5, 2., 0.5, 1.],
            [10.5, 2., 0.5, 1.], [-0.5, 2., 0.5, 1.],
        ]);
        assert_eq!(plaster_hole_boxes(&holes, 10., 2).len(), 2);
        let nan = [FinalizedWallHole { aabb: [f32::NAN, 0., 1., 1.], kind: 0, origin: 0 }];
        assert_eq!(plaster_hole_boxes(&nan, 10., 0).len(), 1);
    }

    #[test]
    fn original_plaster_strips_group_and_reverse() {
        let mut grid = vec![PlasterGridSample { u: 0., y: 0., hole_distance: 1., distance: 0., normal: [0.;3], outside: 0, padding: [0;3] }; 12];
        let inspect = |grid: &[PlasterGridSample], nx, ny, reverse| plaster_strips(grid,nx,ny,0x60000000,reverse)
            .iter().map(|s| (s.base,s.cells,s.stride)).collect::<Vec<_>>();
        assert_eq!(inspect(&grid,4,3,false), [(0x60000000,3,4),(0x600000c0,3,4)]);
        assert_eq!(inspect(&grid,4,3,true), [(0x600000c0,3,-4),(0x60000180,3,-4)]);
        for p in &mut grid { p.outside = 1; }
        assert!(inspect(&grid,4,3,false).is_empty());
        for p in &mut grid[..2] { p.outside = 0; }
        for p in &mut grid[4..6] { p.outside = 0; }
        assert_eq!(inspect(&grid,4,3,false), [(0x60000000,2,4),(0x600000c0,2,4)]);
        let mut odd = vec![grid[0]; 4];
        for p in &mut odd { p.outside = 2; }
        assert_eq!(inspect(&odd,2,2,false), [(0x60000000,1,2)]);
        for p in &mut odd[..3] { p.outside = 1; }
        assert!(inspect(&odd,2,2,false).is_empty());
        odd[3].outside = 0;
        assert_eq!(inspect(&odd,2,2,false), [(0x60000000,1,2)]);
        let wide = vec![grid[0]; 70];
        assert_eq!(inspect(&wide,35,2,false), [(0x60000000,32,35),(0x60000600,2,35)]);
        let vertices: Vec<[u32; 12]> = (0..8).map(|i| {
            let mut p = [0; 12]; p[0] = ((i % 4) as f32).to_bits();
            p[1] = ((i / 4) as f32 * 2.).to_bits(); p
        }).collect();
        for reverse in [false, true] {
            let mut strip = PlasterStrip { base: 0x60000000 + if reverse { 192 } else { 0 },
                cells: 3, stride: if reverse { -4 } else { 4 }, bounds: [0.; 4] };
            plaster_strip_bounds(&mut strip, &vertices, 0x60000000);
            assert_eq!(strip.bounds.map(f32::to_bits), [1069547520,1065353216,0,1072087386]);
        }
    }

    #[test]
    fn original_plaster_bent_freeform_buffers_and_strips() {
        use country_core::resources::{roofs::geometry::surface::Curve2, walls::{WallState,
            public_walls::PublicWallState, height_cache::RegularWallHeightCache,
            publication_source::TerrainHeightGrid512}};
        use country_core::systems::wall::wall_state::path_preparation::PreparedWallPath;
        let points = vec![[0.,0.],[2.,0.],[2.,2.]];
        let state = WallState::new(Curve2::try_new(points.clone(),false).unwrap(),2.,0,0,false);
        let mut snapshot = state.into_snapshot().0; snapshot.source.terrain_floor = 1.;
        let state = WallState::from_snapshot(snapshot,0);
        let terrain = TerrainHeightGrid512 { encoded: vec![0;512*512].into(), minimum_y: 0., maximum_y: 0., sampling_fix: true };
        let mut wall = PublicWallState::collect(&state,&terrain).unwrap();
        wall.prepared_path = Some(PreparedWallPath { positions: points.iter().map(|p| [p[0],0.,p[1]]).collect(),
            points_u: vec![0.,0.5,1.], length: 4., segment_values: vec![0.;3] });
        wall.regular_heights = Some(RegularWallHeightCache { maximum_y: 3., arch_segment_heights: vec![3.;3] });
        wall.segment_heights = vec![2.;3];
        let mut sides=Vec::new(); split_public_wall_into_sides(&wall,None,&mut sides);
        let side=sides[0]; let grid=sample_plaster_side(&wall,None,&side,&[]); let mut all=Vec::new();
        for (i,face) in [1.0f32,-1.0].into_iter().enumerate() {
            let vertices: Vec<_> = grid.iter().copied().map(|sample| {
                let world=utils::wall_space::WallSpaceCoord::new_unchecked(sample.u,sample.y).to_world_space(wall.simplified_curve_2d());
                encode_plaster_vertex(sample,world,face)
            }).collect();
            let bytes: Vec<u8> = vertices.iter().flat_map(|v|v.iter().flat_map(|w|w.to_le_bytes())).collect();
            let expected: &[u8] = if i==0 { include_bytes!("../../../../reports/business/2026-10-23-module-recovery/m6-plaster-bent-buffer-0.bin") }
                else { include_bytes!("../../../../reports/business/2026-10-23-module-recovery/m6-plaster-bent-buffer-1.bin") };
            assert_eq!(bytes.as_slice(),expected,"弯墙面 {i}");
            let address=0x60000000+i as u64*0x100000;
            let mut strips=plaster_strips(&grid,(side.grid&0xffff) as usize,(side.grid>>16) as usize,address,face<0.0);
            for strip in &mut strips { plaster_strip_bounds(strip,&vertices,address);
                for word in strip.bounds.map(f32::to_bits) { all.extend(word.to_le_bytes()); }
                all.extend(strip.base.to_le_bytes()); all.extend(strip.cells.to_le_bytes()); all.extend(strip.stride.to_le_bytes());
            }
        }
        assert_eq!(all.as_slice(),include_bytes!("../../../../reports/business/2026-10-23-module-recovery/m6-plaster-bent-strips.bin"));
        wall.prepared_path.as_mut().unwrap().positions[1][1] = 1.0;
        wall.regular_heights.as_mut().unwrap().arch_segment_heights = vec![3.,4.,3.];
        wall.segment_heights = vec![2.,1.,2.];
        let grid=sample_plaster_side(&wall,None,&side,&[]); let mut all=Vec::new();
        for (i,face) in [1.0f32,-1.0].into_iter().enumerate() {
            let vertices: Vec<_> = grid.iter().copied().map(|sample| {
                let world=utils::wall_space::WallSpaceCoord::new_unchecked(sample.u,sample.y).to_world_space(wall.simplified_curve_2d());
                encode_plaster_vertex(sample,world,face)
            }).collect();
            let bytes: Vec<u8> = vertices.iter().flat_map(|v|v.iter().flat_map(|w|w.to_le_bytes())).collect();
            let expected: &[u8] = if i==0 { include_bytes!("../../../../reports/business/2026-10-23-module-recovery/m6-plaster-slope-buffer-0.bin") }
                else { include_bytes!("../../../../reports/business/2026-10-23-module-recovery/m6-plaster-slope-buffer-1.bin") };
            assert_eq!(bytes.as_slice(),expected,"坡地弯墙面 {i}");
            let address=0x60000000+i as u64*0x100000;
            let mut strips=plaster_strips(&grid,(side.grid&0xffff) as usize,(side.grid>>16) as usize,address,face<0.0);
            for strip in &mut strips { plaster_strip_bounds(strip,&vertices,address);
                for word in strip.bounds.map(f32::to_bits) { all.extend(word.to_le_bytes()); }
                all.extend(strip.base.to_le_bytes()); all.extend(strip.cells.to_le_bytes()); all.extend(strip.stride.to_le_bytes());
            }
        }
        assert_eq!(all.as_slice(),include_bytes!("../../../../reports/business/2026-10-23-module-recovery/m6-plaster-slope-strips.bin"));
    }

    #[test]
    fn original_plaster_rectangle_four_sides_complete_buffers_and_strips() {
        use country_core::resources::{roofs::geometry::surface::Curve2, walls::{WallState,
            public_walls::PublicWallState, height_cache::RegularWallHeightCache,
            publication_source::TerrainHeightGrid512, publication_fields::WallShapeSnapshot}};
        use country_core::systems::wall::wall_state::path_preparation::PreparedWallPath;
        let points = vec![[-2.,-1.],[-2.,1.],[2.,1.],[2.,-1.],[-2.,-1.]];
        let state = WallState::new(Curve2::try_new(points.clone(), false).unwrap(), 2., 0, 0, false);
        let mut snapshot = state.into_snapshot().0; snapshot.source.terrain_floor = 1.;
        let mut shape = [0u8; 28]; shape[..4].copy_from_slice(&1u32.to_le_bytes());
        for (i,v) in [1.0f32,0.,0.,0.,4.,2.].into_iter().enumerate() { shape[4+i*4..8+i*4].copy_from_slice(&v.to_le_bytes()); }
        snapshot.source.shape = WallShapeSnapshot::from_observed(shape);
        let state = WallState::from_snapshot(snapshot, 0);
        let terrain = TerrainHeightGrid512 { encoded: vec![0;512*512].into(), minimum_y: 0., maximum_y: 0., sampling_fix: true };
        let mut wall = PublicWallState::collect(&state, &terrain).unwrap();
        wall.prepared_path = Some(PreparedWallPath { positions: points.iter().map(|p| [p[0],0.,p[1]]).collect(),
            points_u: vec![0.,1./6.,0.5,2./3.,1.], length: 12., segment_values: vec![0.;5] });
        wall.regular_heights = Some(RegularWallHeightCache { maximum_y: 3., arch_segment_heights: vec![3.;5] });
        wall.segment_heights = vec![2.;5];
        let expected: [&[u8];4] = [
            include_bytes!("../../../../reports/business/2026-10-23-module-recovery/m6-plaster-rect-buffer-0.bin"),
            include_bytes!("../../../../reports/business/2026-10-23-module-recovery/m6-plaster-rect-buffer-1.bin"),
            include_bytes!("../../../../reports/business/2026-10-23-module-recovery/m6-plaster-rect-buffer-2.bin"),
            include_bytes!("../../../../reports/business/2026-10-23-module-recovery/m6-plaster-rect-buffer-3.bin"),
        ];
        let mut sides = Vec::new(); split_public_wall_into_sides(&wall, None, &mut sides);
        assert_eq!(sides.len(),4); let mut all = Vec::new();
        for (i,side) in sides.iter().enumerate() {
            let grid = sample_plaster_side(&wall, None, side, &[]);
            let vertices: Vec<_> = grid.iter().copied().map(|sample| {
                let world = utils::wall_space::WallSpaceCoord::new_unchecked(sample.u, sample.y).to_world_space(wall.simplified_curve_2d());
                encode_plaster_vertex(sample, world, 1.0)
            }).collect();
            let bytes: Vec<u8> = vertices.iter().flat_map(|v| v.iter().flat_map(|w| w.to_le_bytes())).collect();
            assert_eq!(bytes.as_slice(), expected[i], "侧 {i}");
            let address = 0x60000000 + i as u64 * 0x100000;
            let mut strips = plaster_strips(&grid, (side.grid & 0xffff) as usize, (side.grid >> 16) as usize, address, false);
            for strip in &mut strips { plaster_strip_bounds(strip, &vertices, address);
                for word in strip.bounds.map(f32::to_bits) { all.extend(word.to_le_bytes()); }
                all.extend(strip.base.to_le_bytes()); all.extend(strip.cells.to_le_bytes()); all.extend(strip.stride.to_le_bytes());
            }
        }
        assert_eq!(all.as_slice(), include_bytes!("../../../../reports/business/2026-10-23-module-recovery/m6-plaster-rect-strips.bin"));
        let holes = [
            country_core::resources::walls::wall_holes::FinalizedWallHole { aabb: [0.,0.,5.,5.], kind: 0, origin: 0 },
            country_core::resources::walls::wall_holes::FinalizedWallHole { aabb: [10.,0.,13.,5.], kind: 0, origin: 0 },
        ];
        let hole_boxes = plaster_hole_boxes(&holes, 12., 1);
        let hole_expected: [&[u8];4] = [
            include_bytes!("../../../../reports/business/2026-10-23-module-recovery/m6-plaster-rect-holes-buffer-0.bin"),
            include_bytes!("../../../../reports/business/2026-10-23-module-recovery/m6-plaster-rect-holes-buffer-1.bin"),
            include_bytes!("../../../../reports/business/2026-10-23-module-recovery/m6-plaster-rect-holes-buffer-2.bin"),
            include_bytes!("../../../../reports/business/2026-10-23-module-recovery/m6-plaster-rect-holes-buffer-3.bin"),
        ];
        let mut hole_strips = Vec::new();
        for (i,side) in sides.iter().enumerate() {
            let grid = sample_plaster_side(&wall, None, side, &hole_boxes);
            let vertices: Vec<_> = grid.iter().copied().map(|sample| {
                let world = utils::wall_space::WallSpaceCoord::new_unchecked(sample.u, sample.y).to_world_space(wall.simplified_curve_2d());
                encode_plaster_vertex(sample, world, 1.0)
            }).collect();
            let bytes: Vec<u8> = vertices.iter().flat_map(|v| v.iter().flat_map(|w| w.to_le_bytes())).collect();
            assert_eq!(bytes.as_slice(), hole_expected[i], "带孔侧 {i}");
            let address = 0x60000000 + i as u64 * 0x100000;
            let mut strips = plaster_strips(&grid, (side.grid & 0xffff) as usize, (side.grid >> 16) as usize, address, false);
            for strip in &mut strips { plaster_strip_bounds(strip, &vertices, address);
                for word in strip.bounds.map(f32::to_bits) { hole_strips.extend(word.to_le_bytes()); }
                hole_strips.extend(strip.base.to_le_bytes()); hole_strips.extend(strip.cells.to_le_bytes()); hole_strips.extend(strip.stride.to_le_bytes());
            }
        }
        assert_eq!(hole_strips.as_slice(), include_bytes!("../../../../reports/business/2026-10-23-module-recovery/m6-plaster-rect-holes-strips.bin"));
        let mut roof_bytes = [0u8;88]; roof_bytes[8..36].copy_from_slice(&shape);
        roof_bytes[0x30..0x34].copy_from_slice(&1.0f32.to_le_bytes());
        roof_bytes[0x34..0x38].copy_from_slice(&1.0f32.to_le_bytes());
        roof_bytes[0x4c..0x50].copy_from_slice(&3.0f32.to_le_bytes());
        let roof = Roof::from_observed(roof_bytes); assert!(roof.is_gable());
        let gable_expected: [&[u8];4] = [
            include_bytes!("../../../../reports/business/2026-10-23-module-recovery/m6-plaster-gable-buffer-0.bin"),
            include_bytes!("../../../../reports/business/2026-10-23-module-recovery/m6-plaster-gable-buffer-1.bin"),
            include_bytes!("../../../../reports/business/2026-10-23-module-recovery/m6-plaster-gable-buffer-2.bin"),
            include_bytes!("../../../../reports/business/2026-10-23-module-recovery/m6-plaster-gable-buffer-3.bin"),
        ];
        let mut sides = Vec::new(); split_public_wall_into_sides(&wall, Some(&roof), &mut sides);
        let mut gable_strips = Vec::new();
        for (i,side) in sides.iter().enumerate() {
            let grid = sample_plaster_side(&wall, Some(&roof), side, &hole_boxes);
            let vertices: Vec<_> = grid.iter().copied().map(|sample| {
                let world = utils::wall_space::WallSpaceCoord::new_unchecked(sample.u, sample.y).to_world_space(wall.simplified_curve_2d());
                encode_plaster_vertex(sample, world, 1.0)
            }).collect();
            let bytes: Vec<u8> = vertices.iter().flat_map(|v| v.iter().flat_map(|w| w.to_le_bytes())).collect();
            assert_eq!(bytes.as_slice(), gable_expected[i], "山墙带孔侧 {i}");
            let address = 0x60000000 + i as u64 * 0x100000;
            let mut strips = plaster_strips(&grid, (side.grid & 0xffff) as usize, (side.grid >> 16) as usize, address, false);
            for strip in &mut strips { plaster_strip_bounds(strip, &vertices, address);
                for word in strip.bounds.map(f32::to_bits) { gable_strips.extend(word.to_le_bytes()); }
                gable_strips.extend(strip.base.to_le_bytes()); gable_strips.extend(strip.cells.to_le_bytes()); gable_strips.extend(strip.stride.to_le_bytes());
            }
        }
        assert_eq!(gable_strips.as_slice(), include_bytes!("../../../../reports/business/2026-10-23-module-recovery/m6-plaster-gable-strips.bin"));
    }

    #[test]
    fn original_plaster_vertex_encoding_faces_and_nan() {
        let cases = [
            (1., 1., -0.4, [0x40a5b646,0x40400000,0x40e79db2,0xbecccccd,0x3f19999a,0,0x3f4ccccd,0x40a00000,0x40000000,0x40400000,0,0]),
            (-1., -0.5, 0.5, [0x409a49ba,0x40400000,0x40d8624e,0xbf000000,0xbf19999a,0x80000000,0xbf4ccccd,0x40400000,0x40000000,0x40400000,0,0]),
            (1., f32::NAN, 0.25, [0x40a5b646,0x40400000,0x40e79db2,0x3e800000,0x3f19999a,0,0x3f4ccccd,0x408c0000,0x40000000,0x40400000,0,0]),
            (-1., 0.5, f32::NAN, [0x409a49ba,0x40400000,0x40d8624e,0x3f000000,0xbf19999a,0x80000000,0xbf4ccccd,0x7fc00000,0x40000000,0x40400000,0,0]),
        ];
        for (face, hole_distance, distance, expected) in cases {
            let p = PlasterGridSample { u: 2., y: 3., hole_distance, distance,
                normal: [0.6, 0., 0.8], outside: 0, padding: [0; 3] };
            assert_eq!(encode_plaster_vertex(p, glam::Vec3::new(5., 3., 7.), face), expected);
        }
    }

    #[test]
    fn original_plaster_side_sampling_and_holes_match_bounded_native() {
        use country_core::resources::{roofs::geometry::surface::Curve2, walls::{WallState,
            public_walls::PublicWallState, height_cache::RegularWallHeightCache,
            publication_source::TerrainHeightGrid512}};
        use country_core::systems::wall::wall_state::path_preparation::PreparedWallPath;
        let state = WallState::new(Curve2::try_new(vec![[0., 0.], [4., 0.]], false).unwrap(), 2., 2, 0, false);
        let mut snapshot = state.into_snapshot().0; snapshot.source.terrain_floor = 1.;
        let state = WallState::from_snapshot(snapshot, 0);
        let terrain = TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(), minimum_y: 0., maximum_y: 0., sampling_fix: true };
        let mut wall = PublicWallState::collect(&state, &terrain).unwrap();
        wall.prepared_path = Some(PreparedWallPath { positions: vec![[0., 0., 0.], [4., 0., 0.]], points_u: vec![0., 1.], length: 4., segment_values: vec![0.; 2] });
        wall.regular_heights = Some(RegularWallHeightCache { maximum_y: 3., arch_segment_heights: vec![3.; 2] });
        wall.segment_heights = vec![2.; 2];
        let side = WallSide { u_begin: 0., u_end: 1., height: 3., y_offset: 2., grid: 2 | (2 << 16), roof_flag: 0, _pad: [0; 3] };
        let samples = sample_plaster_side(&wall, None, &side, &[]);
        let rows: Vec<_> = samples.iter().map(|p| (p.u.to_bits(), p.y.to_bits(), p.distance.to_bits(), p.normal.map(f32::to_bits), p.outside)).collect();
        assert_eq!(rows, vec![
            (0x00000000, 0x3f800000, 0xbf800000, [0x80000000, 0, 0x3f800000], 0),
            (0x40800000, 0x3f800000, 0xbf800000, [0x80000000, 0, 0x3f800000], 0),
            (0x00000000, 0x40400000, 0xbeeb8520, [0x80000000, 0, 0x3f800000], 0),
            (0x40800000, 0x40400000, 0xbeeb8520, [0x80000000, 0, 0x3f800000], 0),
        ]);
        let with_hole = sample_plaster_side(&wall, None, &side, &[[0., 1., 2., 2.]]);
        assert_eq!(with_hole.iter().map(|p| p.hole_distance.to_bits()).collect::<Vec<_>>(),
            [0xc0000000, 0x3f800000, 0, 0x3f800000]);
        assert_eq!(with_hole.iter().map(|p| p.distance.to_bits()).collect::<Vec<_>>(),
            rows.iter().map(|row| row.2).collect::<Vec<_>>());
        let dense_side = WallSide { grid: 6 | (6 << 16), ..side };
        // 原完整 generator 从入口执行到首次 create_buffer，捕获 10×5 顶点。
        let mut generated_sides = Vec::new();
        split_public_wall_into_sides(&wall, None, &mut generated_sides);
        assert_eq!(generated_sides[0].grid, 10 | (5 << 16));
        let original_grid = sample_plaster_side(&wall, None, &generated_sides[0], &[]);
        let bytes: Vec<u8> = original_grid.iter().copied().flat_map(|sample| {
            let world = utils::wall_space::WallSpaceCoord::new_unchecked(sample.u, sample.y)
                .to_world_space(wall.simplified_curve_2d());
            encode_plaster_vertex(sample, world, 1.0).into_iter().flat_map(u32::to_le_bytes)
        }).collect();
        assert_eq!(bytes.as_slice(), include_bytes!("../../../../reports/business/2026-10-23-module-recovery/m6-plaster-entry-first-buffer.bin"));
        let mut all_strips = Vec::new();
        for (face_index, face) in [1.0f32, -1.0].into_iter().enumerate() {
            let vertices: Vec<_> = original_grid.iter().copied().map(|sample| {
                let world = utils::wall_space::WallSpaceCoord::new_unchecked(sample.u, sample.y)
                    .to_world_space(wall.simplified_curve_2d());
                encode_plaster_vertex(sample, world, face)
            }).collect();
            let raw: Vec<u8> = vertices.iter().flat_map(|v| v.iter().flat_map(|word| word.to_le_bytes())).collect();
            let expected: &[u8] = if face_index == 0 {
                include_bytes!("../../../../reports/business/2026-10-23-module-recovery/m6-plaster-entry-buffer-0.bin")
            } else {
                include_bytes!("../../../../reports/business/2026-10-23-module-recovery/m6-plaster-entry-buffer-1.bin")
            };
            assert_eq!(raw.as_slice(), expected);
            let address = 0x60000000 + face_index as u64 * 0x100000;
            let mut strips = plaster_strips(&original_grid, 10, 5, address, face < 0.0);
            for strip in &mut strips {
                plaster_strip_bounds(strip, &vertices, address);
                for word in strip.bounds.map(f32::to_bits) { all_strips.extend(word.to_le_bytes()); }
                all_strips.extend(strip.base.to_le_bytes());
                all_strips.extend(strip.cells.to_le_bytes());
                all_strips.extend(strip.stride.to_le_bytes());
            }
        }
        assert_eq!(all_strips.as_slice(), include_bytes!("../../../../reports/business/2026-10-23-module-recovery/m6-plaster-entry-strips.bin"));
        let mut shared_grid = Vec::with_capacity(36);
        let initial_ptr = shared_grid.as_ptr();
        sample_plaster_side_into(&wall, None, &side, &[], &mut shared_grid);
        sample_plaster_side_into(&wall, None, &dense_side, &[], &mut shared_grid);
        assert_eq!(shared_grid.as_ptr(), initial_ptr);
        assert_eq!(shared_grid.len(), 36);
        let dense = sample_plaster_side(&wall, None, &dense_side, &[[0., 1., 2., 2.]]);
        assert_eq!(dense.iter().map(|p| p.hole_distance.to_bits()).collect::<Vec<_>>(), [
            0xc0000000,0xbf99999a,0xbecccccc,0x3eccccd0,0x3f800000,0x3f800000,
            0xbfcccccd,0xbf99999a,0xbecccccc,0x3eccccd0,0x3f800000,0x3f800000,
            0xbf99999a,0xbf99999a,0xbecccccc,0x3eccccd0,0x3f800000,0x3f800000,
            0xbf4ccccc,0xbf4ccccc,0xbecccccc,0x3eccccd0,0x3f800000,0x3f800000,
            0xbecccccc,0xbecccccc,0xbecccccc,0x3eccccd0,0x3f800000,0x3f800000,
            0,0,0,0x3eccccd0,0x3f800000,0x3f800000,
        ]);
        let expected_u = [0,0x3f4ccccd,0x3fcccccd,0x4019999a,0x404ccccd,0x40800000];
        let expected_y = [0x3f800000,0x3fb33333,0x3fe66666,0x400ccccd,0x40266666,0x40400000];
        let expected_distance = [
            [0xbf800000; 6], [0xbf19999a; 6], [0xbe4cccd0; 6],
            [0,0x3e4cccd0,0x3e4cccd0,0x3e4cccd0,0x3e4cccd0,0],
            [0xbd75c280; 6], [0xbeeb8520; 6],
        ];
        for y in 0..6 { for x in 0..6 {
            let sample = dense[y * 6 + x];
            assert_eq!((sample.u.to_bits(), sample.y.to_bits(), sample.distance.to_bits(), sample.outside),
                (expected_u[x], expected_y[y], expected_distance[y][x], 0));
        }}
        let seam_hole = [country_core::resources::walls::wall_holes::FinalizedWallHole {
            aabb: [-2., -1., 2., 3.], kind: 0, origin: 0,
        }];
        let seam_boxes = plaster_hole_boxes(&seam_hole, 4.0, 0);
        assert_eq!(seam_boxes, [[0., 1., 2., 2.], [4., 1., 2., 2.]]);
        let seam_grid = sample_plaster_side(&wall, None, &side, &seam_boxes);
        assert_eq!(seam_grid.iter().map(|p| p.hole_distance.to_bits()).collect::<Vec<_>>(),
            [0xc0000000, 0xc0000000, 0, 0]);
        // 原 is_flat_w_no_rim 的 +0x3d 非零分支，按原输入区分两种屋顶。
        let flat_roof = Roof::from_observed([0u8; 88]);
        assert!(flat_roof.is_flat() && !flat_roof.is_flat_w_no_rim());
        let flat_side = WallSide { height: 2.0, ..side };
        let flat_grid = sample_plaster_side(&wall, Some(&flat_roof), &flat_side, &[]);
        assert_eq!(flat_grid.iter().map(|p| p.distance.to_bits()).collect::<Vec<_>>(),
            [0xbf800000, 0xbf800000, 0xbfa66666, 0xbfa66666]);
        let mut no_rim_bytes = [0u8; 88]; no_rim_bytes[0x3d] = 1;
        let no_rim = Roof::from_observed(no_rim_bytes);
        assert!(no_rim.is_flat_w_no_rim());
        let no_rim_grid = sample_plaster_side(&wall, Some(&no_rim), &flat_side, &[]);
        assert_eq!(no_rim_grid.iter().map(|p| (p.y.to_bits(), p.distance.to_bits())).collect::<Vec<_>>(),
            [(0x3f800000, 0xbf800000), (0x3f800000, 0xbf800000),
             (0x40170a3e, 0xbf28f5c5), (0x40170a3e, 0xbf28f5c5)]);
        wall.segment_heights.pop();
        assert!(std::panic::catch_unwind(|| sample_plaster_side(&wall, None, &side, &[])).is_err());
        wall.segment_heights.push(2.0);
        let mut circle_snapshot = wall.authority.clone().into_snapshot().0;
        let mut circle_bytes = [0u8; 28];
        circle_bytes[12..16].copy_from_slice(&1.0f32.to_le_bytes());
        circle_snapshot.source.shape = country_core::resources::walls::publication_fields::WallShapeSnapshot::from_observed(circle_bytes);
        wall.authority = WallState::from_snapshot(circle_snapshot, 0);
        let circle_grid = sample_plaster_side(&wall, None, &side, &[]);
        assert_eq!(circle_grid.iter().map(|p| p.normal.map(f32::to_bits)).collect::<Vec<_>>(),
            [[0x3f800000, 0, 0x343bbd2e], [0x3f800000, 0, 0],
             [0x3f800000, 0, 0x343bbd2e], [0x3f800000, 0, 0]]);
        let mut rect_snapshot = wall.authority.clone().into_snapshot().0;
        let mut rect_bytes = [0u8; 28];
        rect_bytes[..4].copy_from_slice(&1u32.to_le_bytes());
        for (index, value) in [1.0f32, 0., 0., 0., 4., 2.].into_iter().enumerate() {
            rect_bytes[4 + index * 4..8 + index * 4].copy_from_slice(&value.to_le_bytes());
        }
        rect_snapshot.source.shape = country_core::resources::walls::publication_fields::WallShapeSnapshot::from_observed(rect_bytes);
        wall.authority = WallState::from_snapshot(rect_snapshot, 0);
        let rect_grid = sample_plaster_side(&wall, None, &side, &[]);
        assert!(rect_grid.iter().all(|p| p.normal.map(f32::to_bits) == [0xbf3504f3, 0, 0xbf3504f3]));
        assert_eq!(rect_grid.iter().map(|p| p.distance.to_bits()).collect::<Vec<_>>(),
            [0xbf800000, 0xbf800000, 0x3f800000, 0x3f800000]);
        let mut snapshot = wall.authority.clone().into_snapshot().0;
        let mut freeform = [0u8; 28]; freeform[..4].copy_from_slice(&2u32.to_le_bytes());
        snapshot.source.shape = country_core::resources::walls::publication_fields::WallShapeSnapshot::from_observed(freeform);
        snapshot.source.terrain_floor = f32::NAN;
        wall.authority = WallState::from_snapshot(snapshot, 0);
        let nan_grid = sample_plaster_side(&wall, None, &side, &[]);
        assert!(nan_grid.iter().all(|p| p.y.is_nan() && p.distance.to_bits() == 0 && p.outside == 0));
    }

    #[test]
    fn original_plaster_grid_row_roof_ridge_and_nan() {
        let cases = [
            (1.0, 4.0, 2.0, None, None, None, 0xc0400000, 1),
            (4.0, 3.0, 2.0, Some(4.0), None, None, 0xbe99999a, 0),
            (4.0, 3.0, 2.0, None, Some(3.0), Some(1.0), 0xbfbae148, 1),
            (f32::NAN, 4.0, 2.0, None, None, None, 0x7fc00000, 0),
        ];
        assert_eq!(std::mem::size_of::<PlasterGridSample>(), 32);
        for (y, top, cutoff, flat, ridge, edge, distance, outside) in cases {
            let p = plaster_grid_sample_row(2.0, y, top, cutoff, [1., 2., 3.], flat, ridge, edge);
            assert_eq!((p.u, p.y.to_bits(), p.hole_distance, p.normal, p.outside), (2.0, y.to_bits(), 1.0, [1., 2., 3.], outside));
            assert_eq!(p.distance.to_bits(), distance);
        }
    }

    #[test]
    fn original_plaster_hole_scalar_sdf_cases() {
        let cases = [
            (glam::Vec2::ZERO, 5.0, 3212836864u32),
            (glam::Vec2::new(2.0, 3.0), 5.0, 1068827891),
            (glam::Vec2::new(1.0, 0.0), 5.0, 0),
            (glam::Vec2::ZERO, f32::NAN, 3212836864),
            (glam::Vec2::new(f32::NAN, 0.0), 5.0, 3221225472),
            (glam::Vec2::new(0.0, f32::NAN), 5.0, 3212836864),
            (glam::Vec2::splat(f32::NAN), 5.0, 0),
            (glam::Vec2::new(1.0, 0.0), -0.0, 2147483648),
        ];
        for (delta, old, expected) in cases {
            assert_eq!(plaster_hole_distance_update(delta, glam::Vec2::new(1.0, 2.0), old).to_bits(), expected);
        }
    }

    #[test]
    fn plaster_hole_grid_bounds_use_margin_ceil_and_clip() {
        assert_eq!(plaster_hole_grid_bounds([2., 3., 0.5, 1.], glam::Vec2::ZERO,
            glam::Vec2::splat(2.), [20, 20]), Some([1, 2, 7, 10]));
        assert_eq!(plaster_hole_grid_bounds([-20., 0., 1., 1.], glam::Vec2::ZERO,
            glam::Vec2::ONE, [10, 10]), None);
        assert_eq!(plaster_hole_grid_bounds([f32::NAN, 3., 1., 1.], glam::Vec2::ZERO,
            glam::Vec2::ONE, [10, 10]), None);
    }

    #[test]
    fn non_rectangle_is_one_full_side() {
        let mut out = Vec::new();
        split_wall_into_sides(None, 3.0, 2.0, 10.0, None, &mut out);
        assert_eq!(out.len(), 1);
        let side = out[0];
        assert_eq!((side.u_begin, side.u_end), (0.0, 1.0));
        assert_eq!((side.height, side.y_offset), (3.0, 1.0));
        // 原 10 m / 0.4 = 25；高度差 1 m / 0.4 四舍五入为 3。
        assert_eq!(side.grid & 0xffff, 25);
        assert_eq!(side.grid >> 16, 3);
        assert_eq!(side.roof_flag, 0);
    }

    #[test]
    fn rectangle_four_sides_and_grid_floor() {
        let rect = Rectangle2d::new(Vec2::ZERO, Vec2::new(4.0, 2.0), Vec2::X);
        let mut out = Vec::new();
        // max_y == wall_elevation → 高度格 0 → 下限 2。
        split_wall_into_sides(Some(rect), 2.0, 2.0, 12.0, None, &mut out);
        assert_eq!(out.len(), 4);
        assert_eq!(std::mem::size_of::<WallSide>(), 24);
        for side in &out {
            assert_eq!(side.height, 2.0);
            assert_eq!(side.y_offset, 0.0);
            assert_eq!(side.grid >> 16, 2, "psubusw 下限 0x00020002");
            assert!(side.u_begin < side.u_end);
            assert_eq!(side.roof_flag, 0);
        }
        // 四条边总长 = 2*(4+2) = 12 → 12/0.4 = 30 格总量。
        let total: u32 = out.iter().map(|s| s.grid & 0xffff).sum();
        assert_eq!(total, 30);
    }

    #[test]
    fn gable_direction_raises_alternating_sides() {
        let rect = Rectangle2d::new(Vec2::ZERO, Vec2::new(4.0, 4.0), Vec2::X);
        let mut bytes = [0u8; 88];
        bytes[0x30..0x34].copy_from_slice(&1.5f32.to_le_bytes());
        bytes[0x34..0x38].copy_from_slice(&1.0f32.to_le_bytes());
        bytes[0x3c] = 0;
        let roof = country_core::resources::roofs::Roof::from_observed(bytes);
        assert!(roof.is_gable());
        let mut out = Vec::new();
        split_wall_into_sides(Some(rect), 2.0, 2.0, 16.0, Some(&roof), &mut out);
        assert_eq!(out[0].roof_flag, 1);
        assert_eq!(out[1].roof_flag, 0);
        assert_eq!(out[2].roof_flag, 1);
        assert_eq!(out[3].roof_flag, 0);
        let raised = 2.0 + roof.height();
        assert_eq!(out[0].height, raised);
        assert_eq!(out[1].height, 2.0);
    }
}

/// B `generate_plaster_mesh_and_entities`（0x1412269b0）：石膏网格与实体生成
/// （泛型单态 `Map<Filter<Iter<(Aabb2, HoleType, HoleOrigin)>>>`）。
/// 候选实现；原完整生成器差分、真实设备和生产链仍待验收。
fn generate_plaster_mesh_and_entities(
    plasters: &mut WallPlasters,
    graphics: &GraphicsDeviceResource,
    commands: &mut Commands,
    id: country_core::resources::walls::WallId,
    wall: &country_core::resources::walls::public_walls::PublicWallState,
    roof: Option<&Roof>,
    holes: &[country_core::resources::walls::wall_holes::FinalizedWallHole],
) {
    let owner = PlasterBufferOwner::new(graphics, Vec::new());
    generate_plaster_with_owner(plasters, owner, commands, id, wall, roof, holes);
}

/// 共享原生成控制流；设备拥有者由调用点在首个成功分配前交入。
fn generate_plaster_with_owner(
    plasters: &mut WallPlasters,
    mut owner: PlasterBufferOwner,
    commands: &mut Commands,
    id: country_core::resources::walls::WallId,
    wall: &country_core::resources::walls::public_walls::PublicWallState,
    roof: Option<&Roof>,
    holes: &[country_core::resources::walls::wall_holes::FinalizedWallHole],
) {
    use country_core::{components::drawable::{NaniBundle, NaniInstanceMeshDesc},
        render::nani::{gpu_world::{GpuWorldSubsetKind, InstanceDataMarker}, instance_data::PlasterInstanceData, systems::NaniInstanceDesc},
        resources::walls::plaster::Plaster, systems::render_loop::draw_passes::draw_lists::DontCastShadow,
        utils::components::transform::Transform};
    let boxes = plaster_hole_boxes(holes, wall.wall_space_length(), wall.shape().tag());
    let mut sides = Vec::new();
    split_public_wall_into_sides(wall, roof, &mut sides);
    assert!(!sides.is_empty(), "原抹灰分侧为空");
    // 原 141226f.. 取所有侧最大的点数，只分配一次并跨侧复用。
    let max_points = sides.iter().map(|side| {
        ((side.grid & 0xffff) as usize) * ((side.grid >> 16) as usize)
    }).max().expect("原抹灰分侧为空");
    let mut grid = Vec::with_capacity(max_points);
    let mut descriptors = Vec::new();
    for side in &sides {
        if side.roof_flag == 2 { break; }
        sample_plaster_side_into(wall, roof, side, &boxes, &mut grid);
        let nx = (side.grid & 0xffff) as usize;
        let ny = (side.grid >> 16) as usize;
        for face in [Some(1.0), (wall.shape().tag() >= 2).then_some(-1.0)].into_iter().flatten() {
            let vertices: Vec<[u32; 12]> = grid.iter().copied().map(|sample| {
                let world = utils::wall_space::WallSpaceCoord::new_unchecked(sample.u, sample.y)
                    .to_world_space(wall.simplified_curve_2d());
                encode_plaster_vertex(sample, world, face)
            }).collect();
            let bytes: Vec<u8> = vertices.iter().flat_map(|v| v.iter().flat_map(|word| word.to_le_bytes())).collect();
            // 原创建失败时跳过本面，已创建的缓冲仍由 owner 持有。
            let Ok((_buffer, address)) = owner.upload_side(&bytes) else { continue; };
            let mut strips = plaster_strips(&grid, nx, ny, address, face < 0.0);
            for strip in &mut strips { plaster_strip_bounds(strip, &vertices, address); }
            descriptors.extend(strips);
        }
    }
    let owner_entity = commands.spawn(owner).id();
    let mut entities = vec![Some(owner_entity)];
    for strip in descriptors {
        let mut words = [0u32; 8];
        for (dst, value) in words[..4].iter_mut().zip(strip.bounds) { *dst = value.to_bits(); }
        words[4] = strip.base as u32; words[5] = (strip.base >> 32) as u32;
        words[6] = strip.cells; words[7] = strip.stride as u32;
        let entity = commands.spawn((NaniBundle::<PlasterInstanceData> {
            desc: NaniInstanceDesc { mesh: NaniInstanceMeshDesc::AtlasPart(0), subset_kind: GpuWorldSubsetKind::Plaster },
            transform: Transform::default(), marker: InstanceDataMarker::default(),
        }, Plaster { words, wall: id }, DontCastShadow)).id();
        entities.push(Some(entity));
    }
    plasters.0.insert(id, entities);
}

/// B `register_required_components`（0x14075e9e0）：Plaster 的 required 组件。
pub fn register_required_components(_registry: &mut bevy_ecs::component::Components) {}
