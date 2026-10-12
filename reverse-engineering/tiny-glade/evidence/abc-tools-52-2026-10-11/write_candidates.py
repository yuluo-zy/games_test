"""本轮候选源码生成记录；只写当前合法 claim 的工作目录，不编译、不运行原码。"""
import gzip
import hashlib
import json
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
INDEX = PROJECT / 'skeleton/task-index'
SELECTION = json.loads((OUT / 'selection.json').read_text())
SOURCES = {}

def source(path, text):
    SOURCES['crates/' + path] = text.strip() + '\n'

source('utils/src/loop_range.rs', r'''
/// 原生 +0/+4 是有符号逻辑起止索引，+8 随 LoopIdx 一起返回。
/// 第三字段的作者名称未知；不把它猜作已取模的长度。
#[derive(Clone, Copy, Debug)]
pub struct LoopRange { pub start_idx: i32, pub end_idx: i32, pub context: u32 }
#[derive(Clone, Copy, Debug)]
pub struct LoopIdx { pub idx: i32, pub context: u32 }
impl LoopRange {
    // ORIGINAL: utils::loop_range::LoopRange::start
    /// 同时返回起点和上下文；不是只返回一个整数。
    pub fn start(&self) -> LoopIdx { LoopIdx { idx: self.start_idx, context: self.context } }
    // ORIGINAL: utils::loop_range::LoopRange::end
    /// 终点读取 +4，上下文读取 +8。
    pub fn end(&self) -> LoopIdx { LoopIdx { idx: self.end_idx, context: self.context } }
    // ORIGINAL: utils::loop_range::LoopRange::is_empty
    /// 原码仅判断相等；逆序区间不在此处判空。
    pub fn is_empty(&self) -> bool { self.end_idx == self.start_idx }
    // ORIGINAL: utils::loop_range::impl$3::next_back
    /// 原码使用有符号比较；成功时先减 end，再返回索引和上下文。
    /// None 不写原生 Option 的未初始化 payload，Rust 使用安全 Option 表达。
    pub fn next_back(&mut self) -> Option<LoopIdx> {
        if self.start_idx >= self.end_idx { return None; }
        self.end_idx = self.end_idx.wrapping_sub(1);
        Some(LoopIdx { idx: self.end_idx, context: self.context })
    }
}
impl LoopIdx {
    // ORIGINAL: utils::loop_range::LoopIdx::raw_idx
    /// 原码直接返回低 32 位；保留有符号索引语义。
    pub fn raw_idx(&self) -> i32 { self.idx }
    // ORIGINAL: utils::loop_range::impl$1::sub
    /// 先做 32 位回绕减法，再符号扩展成 64 位，不是 64 位直接相减。
    pub fn sub(self, rhs: Self) -> i64 { self.idx.wrapping_sub(rhs.idx) as i64 }
}
''')
source('utils/src/lib.rs', r'''
// ORIGINAL: utils::inverse_lerp
/// 顺序为 (value - start) / (end - start)，不钳制，不替换零分母。
pub fn inverse_lerp(start: f32, end: f32, value: f32) -> f32 {
    (value - start) / (end - start)
}
// ORIGINAL: utils::fit
/// 按原 SUB/DIV/SUB/MUL/ADD 顺序映射，保留 NaN、无穷大和区间外值。
/// 不使用 mul_add，避免将原本分开的乘法和加法融合。
pub fn fit(value: f32, src_start: f32, src_end: f32, dst_start: f32, dst_end: f32) -> f32 {
    let t = (value - src_start) / (src_end - src_start);
    let span = dst_end - dst_start;
    t * span + dst_start
}
// ORIGINAL: utils::impl$5::intersects
/// 两个 f32 端点记录的闭区间相交；端点相等算相交，NaN 返回 false。
/// impl$5 的作者类型名尚未知，此处只表达已观察的两个端点。
pub fn intersects(a: [f32; 2], b: [f32; 2]) -> bool {
    a[1] >= b[0] && b[1] >= a[0]
}
''')
source('utils/src/geometry/aabb.rs', r'''
use glam::{Vec2, Vec3};
/// 原生 Aabb2 的 min 位于 +0，max 位于 +8；使用原依赖向量，不重写 glam。
pub struct Aabb2 { pub min: Vec2, pub max: Vec2 }
pub struct Aabb3 { pub min: Vec3, pub max: Vec3 }
impl Aabb2 {
    // ORIGINAL: utils::geometry::aabb::Aabb2::area
    /// 不排序端点、不取绝对值；原乘法的左操作数是 y 差，右操作数是 x 差。
    pub fn area(&self) -> f32 {
        let x = self.max.x - self.min.x;
        let y = self.max.y - self.min.y;
        y * x
    }
}
impl Aabb3 {
    // ORIGINAL: utils::geometry::aabb::Aabb3::extent
    /// 返回 max-min，不除以二；这里 extent 指完整边长。
    pub fn extent(&self) -> Vec3 { self.max - self.min }
}
''')
source('utils/src/geometry/circle.rs', r'''
use glam::Vec2;
pub struct Circle2d { pub center: Vec2, pub radius: f32 }
/// 只恢复构造器，不增加未取证的三角采样 Iterator::next。
pub struct CirclePoints2<'a> {
    pub circle: &'a Circle2d,
    pub point_count: usize,
    pub front: usize,
    pub back: usize,
}
impl Circle2d {
    // ORIGINAL: utils::geometry::circle::Circle2d::signed_distance
    /// 原指令按 y²+x² 求平方根后减半径，不改用 hypot 或平方距离近似。
    pub fn signed_distance(&self, point: Vec2) -> f32 {
        let d = point - self.center;
        let squares = d * d;
        (squares.y + squares.x).sqrt() - self.radius
    }
    // ORIGINAL: utils::geometry::circle::Circle2d::is_point_inside
    /// 严格小于半径；圆周上返回 false，任一比较操作数为 NaN 也返回 false。
    pub fn is_point_inside(&self, point: Vec2) -> bool {
        let d = point - self.center;
        let squares = d * d;
        self.radius > (squares.y + squares.x).sqrt()
    }
    // ORIGINAL: utils::geometry::circle::Circle2d::as_points2_ptcount
    /// 原构造器仅写 circle 指针、count、front=0、back=count；不分配点数组。
    pub fn as_points2_ptcount(&self, point_count: usize) -> CirclePoints2<'_> {
        CirclePoints2 { circle: self, point_count, front: 0, back: point_count }
    }
}
''')
source('utils/src/geometry/rectangle.rs', r'''
use glam::Vec2;
/// 原 +0..16 几何数据未在本函数解释；原样保存，不猜旋转/轴的字段名。
pub struct Rectangle2d { pub geometry: [u8; 16], pub size: Vec2 }
impl Rectangle2d {
    // ORIGINAL: utils::geometry::rectangle::Rectangle2d::with_expanded_size
    /// 原码直接 size+amount；不乘二，也不钳制负尺寸。
    pub fn with_expanded_size(&self, amount: Vec2) -> Self {
        Self { geometry: self.geometry, size: amount + self.size }
    }
}
''')
source('utils/src/geometry/intersect.rs', r'''
pub struct SphereIntersection { pub first: f32, pub second: f32 }
impl SphereIntersection {
    // ORIGINAL: utils::geometry::intersect::SphereIntersection::closest_intersection_distance
    /// 原 cmpunord/minss 组合忽略单边 NaN；相等时选 first，保留其零符号。
    /// 两边都是 NaN 时返回 second 的位模式，不增加正距离筛选。
    pub fn closest_intersection_distance(&self) -> f32 {
        if self.first.is_nan() { self.second }
        else if self.second < self.first { self.second }
        else { self.first }
    }
}
/// points_mut 的安全投影；原四字节指令只证指针恒等。
/// 实际点存储类型由调用方提供，不根据 Triangle 名字补造 Vec3 数组或完整布局。
pub struct Triangle<T> { pub points: T }
impl<T> Triangle<T> {
    // ORIGINAL: utils::geometry::intersect::Triangle::points_mut
    /// 借用原有顶点存储，不分配或复制。
    pub fn points_mut(&mut self) -> &mut T { &mut self.points }
}
''')
source('utils/src/wall_space.rs', r'''
pub struct WallSpaceCoord { pub u: f32, pub y: f32 }
impl WallSpaceCoord {
    // ORIGINAL: utils::wall_space::WallSpaceCoord::with_y_offset
    /// 原值传递 ABI 保留 xmm0 的 u，仅对 xmm1 的 y 加 offset。
    pub fn with_y_offset(self, offset: f32) -> Self { Self { u: self.u, y: self.y + offset } }
    // ORIGINAL: utils::wall_space::WallSpaceCoord::with_y
    /// 只替换 y，不归一化 u。
    pub fn with_y(self, y: f32) -> Self { Self { u: self.u, y } }
}
''')
source('utils/src/rng.rs', r'''
// ORIGINAL: utils::rng::init_rng_u64
/// 游戏自己的 seed 变换，先 wrapping_mul，再 wrapping_add，最后 XOR。
/// 返回的是观察到的 64 位初始状态；RNG 后续生成算法继续复用原 fastrand。
/// 不擅自调用 Rng::with_seed，后者可能再次变换 seed。
pub fn init_rng_u64(seed: u64) -> u64 {
    seed.wrapping_mul(0x7598_39da_217f)
        .wrapping_add(0x0001_3c7c_a929_3765) ^ 0x4d59_5df4_d0f3_3173
}
''')
source('utils/src/subtractive_1d_boolean.rs', r'''
/// 已解析结果切片的投影；元素类型由真实 producer 提供，不猜原 Vec 元素布局。
pub struct SubtractiveBoolean1D<'a, T> { pub resolved_result: &'a [T] }
impl<'a, T> SubtractiveBoolean1D<'a, T> {
    // ORIGINAL: utils::subtractive_1d_boolean::SubtractiveBoolean1D::result
    /// 原生 +8 返回数据指针、+16 返回长度；保留借用，不复制或实现容器。
    pub fn result(&self) -> &'a [T] { self.resolved_result }
}
''')
source('utils/src/timer.rs', r'''
// ORIGINAL: utils::timer::_::impl$0::apply
/// 该具体生成方法的完整观察体只有 ret，没有计时更新或资源写入。
/// 这里只恢复空操作；作者 trait 和参数 ABI 未知，不冒充完整 Timer::apply。
pub fn apply() {}
// ORIGINAL: utils::timer::_::impl$0::queue
/// 与 apply 共用原生空操作入口；没有证据支持排队、tick 或创建计时器。
pub use self::apply as queue;
''')
source('country-core/src/utils/debug_viz.rs', r'''
/// 颜色 16 字节的具体作者类型仍未知；构造器只写默认 tag=0，
/// 未初始化的枚举 payload 不补零，不从中构造 RGBA 值。
#[derive(Clone, Copy)]
pub enum DebugColor { DefaultTagZero, Encoded([u8; 16]) }
pub struct DebugRectangle3d {
    pub color_value: DebugColor,
    pub width: f32,
    pub geometry: [u8; 32],
    pub solid: bool,
}
impl DebugRectangle3d {
    // ORIGINAL: country_core::utils::debug_viz::DebugRectangle3d::new
    /// 原写入 +0 tag=0、+16 width=1、+20..52 原样几何、+52 solid=0。
    /// 此处是语义字段投影，不声称 Rust enum/struct 与原 56B ABI 相同。
    pub fn new(geometry: [u8; 32]) -> Self {
        Self { color_value: DebugColor::DefaultTagZero, width: 1.0, geometry, solid: false }
    }
    // ORIGINAL: country_core::utils::debug_viz::DebugRectangle3d::color
    /// 原 builder 按值写 color 后返回整体，其他字段保持。
    pub fn color(mut self, encoded: [u8; 16]) -> Self {
        self.color_value = DebugColor::Encoded(encoded); self
    }
    // ORIGINAL: country_core::utils::debug_viz::DebugRectangle3d::line_width
    /// 无条件写宽度，不检查是否大于零；NaN 也原样保留。
    pub fn line_width(mut self, width: f32) -> Self { self.width = width; self }
}
/// Sphere 的构造器不在本批；使用完整已初始化的原 40B 记录，保留未知字段和 padding。
pub struct DebugSphere { pub observed_record: [u8; 40] }
impl DebugSphere {
    // ORIGINAL: country_core::utils::debug_viz::DebugSphere::solid
    /// 只写 +36 的 solid=1，之后按值返回记录，不创建球体 mesh。
    pub fn solid(mut self) -> Self { self.observed_record[36] = 1; self }
    // ORIGINAL: country_core::utils::debug_viz::DebugSphere::color
    /// 只覆盖前 16B 颜色记录，余下 24B 原样保存。
    pub fn color(mut self, encoded: [u8; 16]) -> Self {
        self.observed_record[..16].copy_from_slice(&encoded); self
    }
}
''')
source('country-core/src/resources/walls/wall_attachment.rs', r'''
/// 原生值传递的首字节 tag；其他 attachment payload 不在本批定义。
pub struct WallCornerAttachment { pub observed_tag: u8 }
impl WallCornerAttachment {
    // ORIGINAL: country_core::resources::walls::wall_attachment::WallCornerAttachment::is_attached
    /// test cl;setne，仅 tag 非零即 attached。
    pub fn is_attached(self) -> bool { self.observed_tag != 0 }
}
''')
source('country-core/src/resources/session.rs', r'''
/// 只命名已证谓词对应的 tag；不补造完整版本枚举。
pub struct GladeType { pub observed_tag: u8 }
impl GladeType {
    // ORIGINAL: country_core::resources::session::GladeType::is_flowery
    /// 花草场景 tag=2。
    pub fn is_flowery(&self) -> bool { self.observed_tag == 2 }
    // ORIGINAL: country_core::resources::session::GladeType::is_autumn
    /// 秋季场景 tag=3。
    pub fn is_autumn(&self) -> bool { self.observed_tag == 3 }
    // ORIGINAL: country_core::resources::session::GladeType::is_olden
    /// 古旧场景 tag=1。
    pub fn is_olden(&self) -> bool { self.observed_tag == 1 }
}
''')
source('country-core/src/resources/walls/decorator.rs', r'''
pub struct ClutterVersion { pub observed_tag: u8 }
impl ClutterVersion {
    // ORIGINAL: country_core::resources::walls::decorator::ClutterVersion::is_initial
    /// 原首字节为零时才是初始版本；不是任意字段全零判断。
    pub fn is_initial(&self) -> bool { self.observed_tag == 0 }
}
''')
source('country-core/src/resources/walls/mod.rs', r'''
pub struct WallId(pub u64);
impl WallId {
    // ORIGINAL: country_core::resources::walls::WallId::is_valid
    /// 原码只判断 ID 是否非零，不查询墙表，也不保证对象尚未删除。
    pub fn is_valid(&self) -> bool { self.0 != 0 }
}
''')
source('country-core/src/resources/onboarding.rs', r'''
pub struct WelcomeScreenState { pub observed_tag: u32 }
impl WelcomeScreenState {
    // ORIGINAL: country_core::resources::onboarding::WelcomeScreenState::is_closed
    /// 注意这里比较 32 位 tag=2，而非 UI 的 8 位 tag。
    pub fn is_closed(&self) -> bool { self.observed_tag == 2 }
}
''')
source('country-core/src/resources/walls/inner_wall_state.rs', r'''
/// 借用实际已初始化原记录；不重造完整 WallState、shape 或 ECS 布局。
pub struct WallState<'a> { pub observed_record: &'a [u8] }
impl<'a> WallState<'a> {
    // ORIGINAL: country_core::resources::walls::inner_wall_state::WallState::get_enclosure_params
    /// 输入至少 60B。原 u32 tag 位于 +0x38，tag<2 返回 +0x38 本身，
    /// 包含 tag，并非 +0x3c 的 payload；否则 None。
    /// payload 长度/作者类型未知，借用尾部字节，不解读未取证字段。
    pub fn get_enclosure_params(&self) -> Option<&'a [u8]> {
        let tag = u32::from_le_bytes(self.observed_record[0x38..0x3c].try_into().unwrap());
        if tag < 2 { Some(&self.observed_record[0x38..]) } else { None }
    }
}
''')
source('country-core/src/systems/ui/resources/ui_state.rs', r'''
pub struct DragState { pub observed_tag: u8 }
impl DragState {
    // ORIGINAL: country_core::systems::ui::resources::ui_state::DragState::is_finished
    /// 原码是无符号 tag<2，不能按名字写成 tag==Finished。
    pub fn is_finished(&self) -> bool { self.observed_tag < 2 }
    // ORIGINAL: country_core::systems::ui::resources::ui_state::DragState::is_started
    /// 开始状态的原 tag=3。
    pub fn is_started(&self) -> bool { self.observed_tag == 3 }
}
pub struct HoverState { pub observed_tag: u8 }
impl HoverState {
    // ORIGINAL: country_core::systems::ui::resources::ui_state::HoverState::is_some
    /// 原体 movzx 直接返回 byte，没有非零归一化。
    /// 在已证明输入只有 0/1 前保留 u8 返回投影；完整 Rust bool ABI 待调用方核对。
    pub fn is_some(&self) -> u8 { self.observed_tag }
}
''')
source('country-core/src/resources/history/mod.rs', r'''
/// 原 diff+24 长度的已解析投影；不恢复 Vec 或序列化器的内部布局。
pub struct DecoratorHistoryDiffView { pub observed_len: usize }
// ORIGINAL: country_core::resources::history::is_decorator_history_diff_empty
/// 原码只看这一集合长度；不逐字段比较，也不查询历史游标。
pub fn is_decorator_history_diff_empty(diff: &DecoratorHistoryDiffView) -> bool {
    diff.observed_len == 0
}
''')
source('asset/src/prefab.rs', r'''
/// 原名对象的已解析借用；allocation 包含 16B header，byte_len 来自原 +8。
/// 这是借用视图，不实现 Arc，也不猜字符串编码/原所有权。
pub struct PrefabName<'a> { pub allocation: &'a [u8], pub byte_len: usize }
impl<'a> PrefabName<'a> {
    // ORIGINAL: asset::prefab::PrefabName::as_name_ref
    /// 原首指针加 16、长度不变。有效输入须满足 allocation.len()>=16+byte_len。
    /// 返回借用字节；转 UTF-8 字符串的类型证明留给调用方。
    pub fn as_name_ref(&self) -> &'a [u8] { &self.allocation[16..][..self.byte_len] }
    // ORIGINAL: asset::prefab::impl$5::as_ref
    /// 与 as_name_ref 三版均共用同一地址，复用一次已恢复实现。
    pub fn as_ref(&self) -> &'a [u8] { self.as_name_ref() }
}
// ORIGINAL: asset::prefab::impl$4::as_ref
/// 原生 rcx->rax 指针恒等；具体 AsRef 作者目标类型未证，保留泛型借用边界。
pub fn as_ref_identity<T: ?Sized>(value: &T) -> &T { value }
''')
source('country-core/src/input/panic_repro_state.rs', r'''
pub struct PanicReproState { pub observed_tag: u8 }
impl PanicReproState {
    // ORIGINAL: country_core::input::panic_repro_state::PanicReproState::is_inactive
    /// 原首字节等于零为 inactive，不推断其他复现状态的名称。
    pub fn is_inactive(&self) -> bool { self.observed_tag == 0 }
}
''')
source('country-core/src/lib.rs', r'''
pub struct LazyTile(pub u32);
impl LazyTile {
    // ORIGINAL: country_core::LazyTile::from_index
    /// 原 ecx->eax 恒等封装，不做 tile 解码、范围限制或资源查找。
    pub fn from_index(index: u32) -> Self { Self(index) }
}
''')
source('country-core/src/resources/roofs/roof_ridge.rs', r'''
/// 此访问器只证明 8 字节载荷；其作者语义类型未证，保留 bits 投影。
pub struct RoofRidgeDims { pub inner_bits: u64 }
impl RoofRidgeDims {
    // ORIGINAL: country_core::resources::roofs::roof_ridge::RoofRidgeDims::get_inner
    /// 读取 +0 的 64 位值，不猜 Vec2、指针或拆为两种屋脊尺寸。
    pub fn get_inner(&self) -> u64 { self.inner_bits }
}
''')
source('country-core/src/resources/roofs/roof_shape.rs', r'''
pub struct GableRoofStyle { pub observed_tag: u8 }
impl GableRoofStyle {
    // ORIGINAL: country_core::resources::roofs::roof_shape::GableRoofStyle::is_overhang
    /// 原码直接读取 byte，没有 compare；保留 u8，合法 bool tag 范围待消费者核对。
    pub fn is_overhang(&self) -> u8 { self.observed_tag }
}
''')
source('country-core/src/systems/timelapse_replay.rs', r'''
pub struct TimelapseState { pub observed_tag: u8 }
impl TimelapseState {
    // ORIGINAL: country_core::systems::timelapse_replay::TimelapseState::is_active
    /// 原 tag!=2 即 active，不把 tag=0 默认解释为 inactive。
    pub fn is_active(&self) -> bool { self.observed_tag != 2 }
}
''')
source('motomoto/src/lib.rs', r'''
/// impl$0 作者类型与 tag 语义未证，此投影只保存原 +0 的 64 位载荷。
pub struct TagView { pub observed_word: u64 }
impl TagView {
    // ORIGINAL: motomoto::impl$0::as_tag
    /// 返回原 word，不生成/转换 tag，不把可能的指针强行解引用。
    pub fn as_tag(&self) -> u64 { self.observed_word }
}
''')
source('systems/toolbar/src/mei/core.rs', r'''
/// impl$2 的原类型未知，此投影只表达首 64 位载荷。
pub struct IdView { pub observed_word: u64 }
impl IdView {
    // ORIGINAL: toolbar::mei::core::impl$2::as_id
    /// 原 +0 qword 直接返回；不猜 ID 生成规则或对象有效性。
    pub fn as_id(&self) -> u64 { self.observed_word }
}
''')
source('tg-text/src/shape_stream.rs', r'''
/// 原 +16 的长度由真实 ShapeStream owner 提供，其余 text/shape 字段尚未恢复。
pub struct ShapeStream { pub observed_len: usize }
impl ShapeStream {
    // ORIGINAL: tg_text::shape_stream::ShapeStream::is_empty
    /// 只比较长度是否为零，不重写文字 shaping 或 Vec 存储。
    pub fn is_empty(&self) -> bool { self.observed_len == 0 }
}
''')
source('country-core/src/render/sparse_instance_buffer.rs', r'''
/// 本轮用户明确包含此后置文件中的工具函数；仅恢复访问器，不恢复 GPU。
pub struct AllocationHandle { pub observed_word: u64 }
impl AllocationHandle {
    // ORIGINAL: country_core::render::sparse_instance_buffer::AllocationHandle::is_null
    /// 原首 64 位为零即 null，不查询 allocation 表或 GPU 状态。
    pub fn is_null(&self) -> bool { self.observed_word == 0 }
}
''')

HEADER = '''//! A/B/C 三版共享工具候选，只恢复本文件列出的观察函数。
//! 状态：implemented_unverified；按用户此前要求，构建/测试/原码执行继续暂停。
//! 此文件是原指令语义的 Rust 候选，不是取回的作者源码。
//! 非完整类型标为投影；未声称原 Rust 签名、ABI 或完整系统已还原。
//! 完整身份、指令、版本对应及消费者见本任务 work 根目录 mapping.json / CONTRACT.md。
'''

def main():
    grouped = {}
    for entry in SELECTION:
        grouped.setdefault(entry['source_path'], []).append(entry)
    assert set(grouped) == set(SOURCES), (set(grouped) - set(SOURCES), set(SOURCES) - set(grouped))
    full_mapping = []
    for path, entries in grouped.items():
        task = entries[0]['file_task_id']
        claim = json.loads((INDEX / 'claims' / (task + '.json')).read_text())
        assert claim['owner'] == 'abc-tools-52'
        work = Path(claim['work_dir'])
        dst = work / path
        dst.parent.mkdir(parents=True, exist_ok=True)
        code = HEADER + SOURCES[path]
        file_mapping = []
        for entry in entries:
            name = entry['function_name']
            marker = '// ORIGINAL: ' + name
            assert code.count(marker + '\n') == 1, name
            variants = {}
            for version in 'ABC':
                original = entry['variants'][version]
                evidence_paths = []
                native = []
                for n, instance in enumerate(original['native_instances']):
                    ep = OUT / ('pack-' + version) / (entry['logical_group_id'] + '-' + str(n) + '.json')
                    evidence = json.loads(ep.read_text())
                    assert evidence['status'] == 'ok'
                    assert not evidence['range']['instruction_overhang_bytes']
                    body = bytes.fromhex(''.join(i['bytes'] for i in evidence['instructions']))
                    digest = hashlib.sha256(body).hexdigest()
                    assert digest in original['body_hashes'], (name, version, digest)
                    native.append(dict(instance, va=hex(0x140000000 + instance['rva']), body_sha256=digest))
                    evidence_paths.append(str(ep))
                variants[version] = {
                    'function_id': original['id'], 'native_instances': native,
                    'source_metadata': entry['file_metadata'][version]['metadata_path'],
                    'ownership': original['ownership'],
                    'library_dependencies': original['library_dependencies'],
                    'indexed_callers': original['distinct_callers'],
                    'physical_reference_callers_rva': original['physical_reference_callers'],
                    'private_callees': original['private_callees'], 'evidence': evidence_paths,
                }
            addresses = '；'.join(v + '=' + ','.join(i['va'] for i in variants[v]['native_instances']) for v in 'ABC')
            replacement = marker + '\n    // 三版对应：' + addresses + '\n    // 逻辑组：' + entry['logical_group_id']
            code = code.replace(marker + '\n', replacement + '\n')
            item = {'logical_group_id': entry['logical_group_id'], 'original_symbol': name,
                    'original_source_path': path, 'candidate_path': str(dst),
                    'task_id': task, 'target_version': 'C', 'status': 'implemented_unverified',
                    'same_observed_native_byte_sets_ABC': True,
                    'original_rust_signature_verified': False, 'cross_version_abi_verified': False,
                    'runtime_equivalence_verified': False, 'variants': variants}
            file_mapping.append(item)
        dst.write_text(code)
        for item in file_mapping:
            item['candidate_sha256'] = hashlib.sha256(dst.read_bytes()).hexdigest()
            item['source_marker_line'] = code[:code.index('// ORIGINAL: ' + item['original_symbol'] + '\n')].count('\n') + 1
        (work / 'mapping.json').write_text(json.dumps(file_mapping, ensure_ascii=False, indent=2) + '\n')
        contract = f'''# {path} 工具恢复合同

本文件 {len(entries)} 个逻辑组共用一份 Rust 候选，不分别复制 A/B/C。
实际原始 binary/PDB SHA、RVA/VA、范围/字节 SHA、caller/callee 与源元数据见 mapping.json；
全批次原证据位于 {OUT}。原始 PE/PDB CodeView 身份经只读取证确认。

## 已观察事实

逐个函数读取 A/B/C 的 PDB 独立过程范围，完整指令原字节检查，
各版函数观察字节 SHA 与原索引一致，均无指令越界。
每个实现的中文注释记录运算顺序、字段宽度、tag 条件或借用/写入范围。
原路径位于游戏私有项目，索引 ownership=private_game_scope_candidate；
本批不重写 std/Vec/Arc/Bevy/Serde 或 RNG 内部。glam 模块使用原依赖 API。

## 推断及未知

Rust API 是语义候选；完整作者结构、返回类型、trait ABI、padding 与枚举全集
不由相同短机器码证明。明确命名为 observed/resolved/view/bits 的输入是调用方
必须提供的已解析边界，不是已恢复的 producer。原始 caller ID 与物理调用地址
已记录，但尚未逐个接入它们；不可宣称此文件构成完整业务链。
输入须满足各访问器有效记录/长度/初始化条件。安全借用投影的检查只用于防止
无效宿主访问，不宣称复现原 UB。无调用的 leaf 仍保留零调用事实。

## 副作用与消费者

getter 借用/读取现有输入；builder 按值返回；LoopRange::next_back 原地推进 end；
其它具体副作用以候选中文注释为准。不得自动为 getter 增加 history、ECS 通知、
删除清理、分配或 GPU 行为；这些由真实业务 owner 负责，不在本批范围。

## 交付与待验收

状态 implemented_unverified。本轮没有编译、测试、原码执行或独立 reviewer。
后续需核对真实调用参数/ABI、绑定 producer/consumer、用原始 A/B/C 做差分，
特别检查 NaN/±0、32 位回绕、无效区间、tag 与空集合。
现有生产工程未改，旧已验收生产范围不因这些独立候选而扩大。
'''
        (work / 'CONTRACT.md').write_text(contract)
        (work / 'obligations.md').write_text('''# 后续义务

- 原始 Rust 签名、完整类型与 trait 绑定需补调用点证据。
- 对齐原始依赖版本，不重写库内部；本批无新 Cargo.lock。
- 数值结果和真实调用链按 A/B/C 分开验收，不从同码推定系统等价。
- 验证暂停；未运行构建、测试、oracle；无独立 review 或共享生产整合。
''')
        full_mapping.extend(file_mapping)
    assert len(full_mapping) == 52
    (OUT / 'mapping.json').write_text(json.dumps(full_mapping, ensure_ascii=False, indent=2) + '\n')
    lines = ['# 三版同码 52 个工具函数的 Rust 候选', '',
             '52 个逻辑函数组、29 个原文件组、156 个版本映射，仅保存一套候选。', '',
             '状态 **implemented_unverified**：按此前验证暂停要求，没有构建、测试、原码执行或独立审查。',
             '短函数的观察语义已写成 Rust；原始签名/ABI 和完整 producer/consumer 仍有待补证项，详见每组合同。', '',
             '源码保存在各合法 claim 的 work 目录，并保持原 crates/… 路径。',
             '每个函数含中文注释、A/B/C VA 与逻辑组 ID。JSON 保存原函数 ID、字节 SHA、证据和消费者。', '',
             '| 原模块 / 候选文件 | 函数组数 |', '|---|---|']
    for path, entries in grouped.items():
        dst = Path(next(x for x in full_mapping if x['original_source_path'] == path)['candidate_path'])
        lines.append(f'| [{path}]({dst}) | {len(entries)} |')
    lines += ['', '完整对应：[mapping.json](mapping.json)。原证据：[A](pack-A/report.json)、[B 原 RNE](pack-B/report.json)、[C](pack-C/report.json)。', '',
              'PrefabName::as_name_ref 与 impl$5::as_ref 各版共享地址，后者委托前者；timer apply/queue 共享 ret，以导出别名复用。',
              '其他地址相同但类型/业务语义不同的访问器保留各自模块，不据短指令合并作者类型。', '',
              'glam 继续使用原依赖。`init_rng_u64` 只恢复游戏 seed 变换，不复写 fastrand。',
              'CirclePoints2 只恢复构造器；未擅自补 Iterator::next。',
              'get_inner/as_tag/as_id 保留 64 位观察值；is_overhang/is_some 保留 byte 返回投影，尚未证明原 Rust 返回类型。',
              'DebugRectangle 构造器不将未初始化默认颜色 payload 补零；DebugSphere 保留已初始化原记录未知字节。',
              '本批未接共享生产 lib/Cargo；不宣称 52 个完整作者类型或完整游戏系统已经还原。', '']
    (OUT / 'README.md').write_text('\n'.join(lines))
    print(json.dumps({'functions': len(full_mapping), 'files': len(grouped), 'overview': str(OUT / 'README.md')}, ensure_ascii=False))

if __name__ == '__main__':
    main()
