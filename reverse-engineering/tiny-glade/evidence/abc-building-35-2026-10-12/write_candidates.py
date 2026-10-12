"""35组优先候选：保留上一批唯一类型定义，在新claim目录添加方法与已证字段。"""
import hashlib
import json
from pathlib import Path

OUT = Path(__file__).resolve().parent
PROJECT = OUT.parents[1]
INDEX = PROJECT / 'skeleton/task-index'
OWNER = 'abc-building-35'
SELECTION = json.loads((OUT / 'selection.json').read_text())
SOURCES = {}
BASE = {}
BASE_NAMES = {
    'base_math': 'crates/utils/src/lib.rs',
    'base_rng': 'crates/utils/src/rng.rs',
    'base_loop_range': 'crates/utils/src/loop_range.rs',
    'base_aabb': 'crates/utils/src/geometry/aabb.rs',
    'base_circle': 'crates/utils/src/geometry/circle.rs',
    'base_rectangle': 'crates/utils/src/geometry/rectangle.rs',
    'base_intersect': 'crates/utils/src/geometry/intersect.rs',
    'base_attachment': 'crates/country-core/src/resources/walls/wall_attachment.rs',
    'base_wall_id': 'crates/country-core/src/resources/walls/mod.rs',
    'base_history': 'crates/country-core/src/resources/history/mod.rs',
}
PREVIOUS = json.loads((PROJECT / 'evidence/abc-tools-52-2026-10-11/mapping.json').read_text())
for module, source_path in BASE_NAMES.items():
    entry = next(x for x in PREVIOUS if x['original_source_path'] == source_path)
    BASE[module] = Path(entry['candidate_path'])
    assert hashlib.sha256(BASE[module].read_bytes()).hexdigest() == entry['candidate_sha256']

def source(path, code):
    SOURCES['crates/' + path] = code.strip() + '\n'

source('utils/src/lib.rs', r'''
pub use crate::base_math::{fit, intersects, inverse_lerp};
// ORIGINAL: utils::lerp
/// 原 SUB/MUL/ADD 为 start+(end-start)*t；不融合乘加、不钳制 t。
/// 现有camera函数写 product+start，正常域可对应，但NaN操作数顺序尚无等价证明。
pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    let difference = end - start;
    let weighted = difference * t;
    start + weighted
}
''')
source('utils/src/swizzlers.rs', r'''
use glam::{Vec2, Vec3};
// ORIGINAL: utils::swizzlers::impl$1::xvy
/// 将原 Vec2 的两个分量放到三维 x/z，中间 y 使用调用方值。
/// 原指令只有三次32位复制，不归一化，也不做坐标缩放。
pub fn xvy(xz: Vec2, y: f32) -> Vec3 { Vec3::new(xz.x, y, xz.y) }
''')
source('utils/src/rng.rs', r'''
pub use crate::base_rng::init_rng_u64;
// ORIGINAL: utils::rng::init_rng
/// 原ECX低32位先零扩展，再做与init_rng_u64相同的游戏seed变换。
/// 两入口共用一份算法；输出是原64位状态，不再额外调用with_seed二次变换。
pub fn init_rng(seed: u32) -> u64 { init_rng_u64(u64::from(seed)) }
''')
source('utils/src/loop_range.rs', r'''
pub use crate::base_loop_range::{LoopIdx, LoopRange};
impl LoopRange {
    // ORIGINAL: utils::loop_range::LoopRange::len
    /// 原end-start在32位回绕后符号扩展；逆序可返回负数，不转换成usize或abs。
    pub fn len(&self) -> i64 { self.end_idx.wrapping_sub(self.start_idx) as i64 }
}
''')
source('utils/src/geometry/aabb.rs', r'''
use glam::Vec2;
pub use crate::base_aabb::{Aabb2, Aabb3};
impl Aabb2 {
    // ORIGINAL: utils::geometry::aabb::Aabb2::width
    /// max.x-min.x，不取绝对值。
    pub fn width(&self) -> f32 { self.max.x - self.min.x }
    // ORIGINAL: utils::geometry::aabb::Aabb2::height
    /// max.y-min.y；不能复用其他模块中仅同名的height。
    pub fn height(&self) -> f32 { self.max.y - self.min.y }
    // ORIGINAL: utils::geometry::aabb::Aabb2::contains
    /// 按原x下界/x上界/y下界/y上界顺序作ordered比较；包含边界，NaN为false。
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.min.x && self.max.x >= point.x
            && point.y >= self.min.y && self.max.y >= point.y
    }
}
''')
source('utils/src/geometry/circle.rs', r'''
use glam::Vec2;
pub use crate::base_circle::{Circle2d, CirclePoints2};
impl Circle2d {
    // ORIGINAL: utils::geometry::circle::Circle2d::contains_point
    /// 严格比较radius² > y²+x²，不开平方；不能委托is_point_inside。
    /// 负半径被平方，超大半径可溢出；保留与距离版本不同的边界行为。
    pub fn contains_point(&self, point: Vec2) -> bool {
        let d = point - self.center;
        let squares = d * d;
        let distance_squared = squares.y + squares.x;
        let radius_squared = self.radius * self.radius;
        radius_squared > distance_squared
    }
}
''')
source('utils/src/geometry/rectangle.rs', r'''
use glam::{Vec2, Vec3};
pub use crate::base_rectangle::Rectangle2d;
impl Rectangle2d {
    // ORIGINAL: utils::geometry::rectangle::Rectangle2d::area
    /// 原size.x*size.y，不取abs、不补正数约束。
    pub fn area(&self) -> f32 { self.size.x * self.size.y }
    // ORIGINAL: utils::geometry::rectangle::Rectangle2d::perimeter
    /// 原先加x/y，随后该和与自身相加，不换成乘法/FMA。
    pub fn perimeter(&self) -> f32 { let sum = self.size.x + self.size.y; sum + sum }
}
/// 三个短体交叉确认32B字段：orientation@0、center_xz@8、size_xz@16、min/max_y@24/28。
/// 作者字段名仍是语义命名；orientation存原两浮点，未增加单位向量约束。
pub struct Rectangle3d {
    pub orientation_xz: Vec2,
    pub center_xz: Vec2,
    pub size_xz: Vec2,
    pub min_y: f32,
    pub max_y: f32,
}
impl Rectangle3d {
    // ORIGINAL: utils::geometry::rectangle::Rectangle3d::extent
    /// 返回(width, max_y-min_y, depth)；xz size已是完整尺寸，不乘二。
    pub fn extent(&self) -> Vec3 {
        Vec3::new(self.size_xz.x, self.max_y - self.min_y, self.size_xz.y)
    }
    // ORIGINAL: utils::geometry::rectangle::Rectangle3d::translate
    /// 原地平移center及两条y边界，size/orientation不变。
    /// xz是old+delta，y两项是delta+old，保留原操作数顺序。
    pub fn translate(&mut self, delta: Vec3) {
        self.center_xz = self.center_xz + Vec2::new(delta.x, delta.z);
        self.min_y = delta.y + self.min_y;
        self.max_y = delta.y + self.max_y;
    }
    // ORIGINAL: utils::geometry::rectangle::Rectangle3d::set_orientation_xz
    /// 无条件写两个浮点；未调用normalize，不转角度，也不改变尺寸。
    pub fn set_orientation_xz(&mut self, orientation: Vec2) { self.orientation_xz = orientation; }
}
''')
source('utils/src/geometry/intersect.rs', r'''
use glam::Vec3;
pub use crate::base_intersect::{SphereIntersection, Triangle};
impl Triangle<[Vec3; 3]> {
    // ORIGINAL: utils::geometry::intersect::Triangle::new
    /// 三个输入各12B，按顺序复制到0/12/24，共36B。
    /// 该构造证据补齐上批Triangle<T>点存储，不改points_mut的借用实现。
    /// 使用Vec3，不使用16B的Vec3A；不计算法线或重排顶点。
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self { Self { points: [a, b, c] } }
}
''')
source('country-core/src/resources/walls/wall_attachment.rs', r'''
pub use crate::base_attachment::WallCornerAttachment;
impl Default for WallCornerAttachment {
    // ORIGINAL: country_core::resources::walls::wall_attachment::impl$1::default
    /// 原返回0，默认无附着；其他tag的完整枚举未扩充。
    fn default() -> Self { Self { observed_tag: 0 } }
}
impl WallCornerAttachment {
    // ORIGINAL: country_core::resources::walls::wall_attachment::WallCornerAttachment::to_rectangle_corner_id
    /// tag=0返回None；非零返回零扩展tag-1的u64索引。
    /// 原None仍算出rdx=u64::MAX，但该payload无语义，安全Option不制造它。
    pub fn to_rectangle_corner_id(self) -> Option<u64> {
        if self.observed_tag == 0 { None } else { Some(u64::from(self.observed_tag) - 1) }
    }
}
''')
source('country-core/src/resources/walls/mod.rs', r'''
pub use crate::base_wall_id::WallId;
impl WallId {
    // ORIGINAL: country_core::resources::walls::WallId::get_inner
    /// 原64位ID原样返回；不做valid判断或实体表查询。
    pub fn get_inner(&self) -> u64 { self.0 }
}
''')
source('country-core/src/systems/wall/valid_enclosure/mod.rs', r'''
use glam::Vec2;
/// 读取原tag及其选中payload的借用投影；不复造完整作者ShapeParameters枚举。
pub struct ShapeParameters<'a> { pub observed_record: &'a [u8] }
impl ShapeParameters<'_> {
    // ORIGINAL: country_core::systems::wall::valid_enclosure::ShapeParameters::center
    /// 原索引为4+u32(tag)*8，读取两f32；tag0用+4/+8，tag1用+12/+16。
    /// 调用方必须提供合法已初始化记录；不将未知tag当默认圆或矩形。
    pub fn center(&self) -> Vec2 {
        let tag = u32::from_le_bytes(self.observed_record[..4].try_into().unwrap());
        let offset = 4 + tag as usize * 8;
        let x = f32::from_le_bytes(self.observed_record[offset..offset + 4].try_into().unwrap());
        let y = f32::from_le_bytes(self.observed_record[offset + 4..offset + 8].try_into().unwrap());
        Vec2::new(x, y)
    }
}
''')
source('country-core/src/systems/wall/wall_state/wall_spatial_hash.rs', r'''
use glam::Vec2;
/// 原对象+72/+76是signed32网格宽/高，+80/+84是世界宽/高。
/// 不重写HashMap/空间桶，此结构只表达这两个游戏几何接口的已解析输入。
pub struct WallSpatialHash { pub grid_size: [i32; 2], pub world_size: Vec2 }
impl WallSpatialHash {
    // ORIGINAL: country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_size
    /// 各世界尺寸除以对应有符号网格尺寸；零/负值不替换，按浮点规则保留。
    pub fn cell_size(&self) -> Vec2 {
        Vec2::new(self.world_size.x / self.grid_size[0] as f32,
                  self.world_size.y / self.grid_size[1] as f32)
    }
    // ORIGINAL: country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::grid_coord_to_cell_index
    /// 原y*width+x在i32回绕后扩展i64；不固定width=64、不夹取grid坐标。
    pub fn grid_coord_to_cell_index(&self, grid: [i32; 2]) -> i64 {
        grid[1].wrapping_mul(self.grid_size[0]).wrapping_add(grid[0]) as i64
    }
}
''')
source('country-core/src/resources/walls/wall_holes.rs', r'''
use std::slice;
/// 20B原record的语义字段。padding不补默认；实际owner应提供已初始化记录。
pub struct FinalizedWallHole { pub aabb: [f32; 4], pub kind: u8, pub origin: u8 }
/// Vec/std内部不重写；借用owner的已最终化数组和原+24的wall_length。
pub struct FinalizedWallHoles<'a> { pub records: &'a [FinalizedWallHole], pub length: f32 }
impl<'a> FinalizedWallHoles<'a> {
    // ORIGINAL: country_core::resources::walls::wall_holes::FinalizedWallHoles::wall_length
    /// 原 getter直接读+24；不增加phase检查，phase由真实owner保障。
    pub fn wall_length(&self) -> f32 { self.length }
    // ORIGINAL: country_core::resources::walls::wall_holes::FinalizedWallHoles::holes_no_duplicates
    /// 短体构造ptr..ptr+20*len；closure消费证据补自三版iter_holes_with_padding的closed分支。
    /// 仅保留origin==0的主记录，不按AABB/ID HashSet去重，不重新排序。
    /// 这里只还原无重复迭代器，不实现padding/周期复制的完整消费者。
    pub fn holes_no_duplicates(&self) -> impl Iterator<Item = &'a FinalizedWallHole> {
        self.records.iter().filter(|hole| hole.origin == 0)
    }
    /// holes和holes_no_duplicates构造体共用原地址；未过滤别名须保持全部记录。
    pub fn holes(&self) -> slice::Iter<'a, FinalizedWallHole> { self.records.iter() }
}
/// 原newtype getter指针恒等；不猜内部存储全集。
#[repr(transparent)]
pub struct FinalizedHoleStorage<T>(pub T);
impl<T> FinalizedHoleStorage<T> {
    // ORIGINAL: country_core::resources::walls::wall_holes::FinalizedHoleStorage::inner
    /// 返回内部原有对象引用，不分配或克隆。
    pub fn inner(&self) -> &T { &self.0 }
}
pub struct InProgressHoleStorage<'a, T> { pub records: &'a [T] }
impl<'a, T> InProgressHoleStorage<'a, T> {
    // ORIGINAL: country_core::resources::walls::wall_holes::InProgressHoleStorage::holes
    /// 原ptr/len借用，元素型由raw owner提供，不把它当Finalized record。
    pub fn holes(&self) -> &'a [T] { self.records }
}
''')
source('country-core/src/resources/walls/decorator_subtype.rs', r'''
/// 原返回EAX/EDX两槽投影；不能仅凭movl判断作者字段/enum的完整布局。
pub struct DecoratorLibKey { pub return_eax: u32, pub return_edx: u32 }
impl DecoratorLibKey {
    // ORIGINAL: country_core::resources::walls::decorator_subtype::DecoratorLibKey::new
    /// EAX=第二输入，EDX=第一输入。C真实caller将AL/DL写到连续两byte，
    /// 可对应现有游戏key的subtype/rank低byte，但完整输入类型/ABI仍待更多caller核对。
    /// 保留32位原观察，不直接改为已有u8结构，也不将字段顺序按名字猜反。
    pub fn new(arg_ecx: u32, arg_edx: u32) -> Self {
        Self { return_eax: arg_edx, return_edx: arg_ecx }
    }
}
''')
source('country-core/src/resources/walls/decorator.rs', r'''
pub struct TransientDecoratorIdGen { pub source_word: u64, pub counter: u64, pub tag: u8 }
impl TransientDecoratorIdGen {
    // ORIGINAL: country_core::resources::walls::decorator::TransientDecoratorIdGen::new
    /// 原+0保存输入word、+8counter=0、+16保存tag；不自行产生第一条ID或推断递增规则。
    pub fn new(source_word: u64, tag: u8) -> Self { Self { source_word, counter: 0, tag } }
}
/// 原构造短体只证64位寄存器恒等；保留word，不宣称和Info72的u32 ID同型。
pub struct DecoratorId(pub u64);
impl DecoratorId {
    // ORIGINAL: country_core::resources::walls::decorator::DecoratorId::new
    /// 不新增全局ID表，也不缩窄原输入。
    pub fn new(word: u64) -> Self { Self(word) }
}
pub struct DecoratorRank(pub u32);
impl DecoratorRank {
    // ORIGINAL: country_core::resources::walls::decorator::DecoratorRank::new
    /// ECX->EAX原32位恒等封装，不猜rank上限。
    pub fn new(rank: u32) -> Self { Self(rank) }
}
''')
source('country-core/src/resources/walls/decorator_archivist.rs', r'''
/// 两字段的借用视图。真实容器沿用现有archivist owner，不重写HashMap或collision句柄。
/// 原offset为+96/+144，具体作者容器类型仍由真实producer/consumer提供。
pub struct DecoratorArchivist<'a, C, I> { pub colliders: &'a C, pub interactions: &'a I }
impl<'a, C, I> DecoratorArchivist<'a, C, I> {
    // ORIGINAL: country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_colliders
    /// getter借用+96对象；不清空、克隆或移动，原只有LEA。
    pub fn decorator_colliders(&self) -> &'a C { self.colliders }
    // ORIGINAL: country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_interactions
    /// getter借用+144对象；两种容器独立，不猜作同一个碰撞表。
    pub fn decorator_interactions(&self) -> &'a I { self.interactions }
}
''')
source('country-core/src/resources/roofs/roof_animation.rs', r'''
/// 现有roof_components已将动画保存为8B，两f32解释由其advance/删除消费者支持。
pub struct RoofAnimation { pub first: f32, pub second: f32 }
impl RoofAnimation {
    // ORIGINAL: country_core::resources::roofs::roof_animation::RoofAnimation::new
    /// 两返回通道均为正零；不设为1、不附加动画事件或history。
    pub fn new() -> Self { Self { first: 0.0, second: 0.0 } }
}
''')
source('country-core/src/resources/history/wall_compound_edit.rs', r'''
pub struct CompoundEditBuilder;
impl CompoundEditBuilder {
    // ORIGINAL: country_core::resources::history::wall_compound_edit::CompoundEditBuilder::new_edit_sequence
    /// 原24B返回是empty Vec：capacity=0、dangling align=8、len=0。
    /// 直接复用std Vec，不实现原分配/指针/容量布局；T由真实command owner确定。
    /// 创建空序列不等于start/commit/cursor推进。
    pub fn new_edit_sequence<T>() -> Vec<T> { Vec::new() }
}
''')
source('country-core/src/resources/history/mod.rs', r'''
pub use crate::base_history::{is_decorator_history_diff_empty, DecoratorHistoryDiffView};
pub struct HistoryChainIndex(pub u32);
impl HistoryChainIndex {
    // ORIGINAL: country_core::resources::history::history_chain::HistoryChainIndex::new
    /// 原32位索引恒等封装，不猜当前cursor、不检查历史链长度。
    pub fn new(index: u32) -> Self { Self(index) }
}
''')
source('country-core/src/resources/history/replay_cmd_buffer.rs', r'''
/// 原+24的长度输入投影；ReplayBuffer中其它数组/时间/owner仍未由此getter恢复。
pub struct ReplayBuffer { pub observed_len: usize }
impl ReplayBuffer {
    // ORIGINAL: country_core::resources::history::replay_cmd_buffer::ReplayBuffer::len
    /// 返回原word长度，不实现容器或把同名len当LoopRange长度。
    pub fn len(&self) -> usize { self.observed_len }
}
''')
source('country-core/src/systems/history/history_process.rs', r'''
/// 原32位命令编码投影；完整版本枚举和owner分发不由单个常量入口恢复。
pub struct HistoryCmd { pub encoded_word: u32 }
impl HistoryCmd {
    // ORIGINAL: country_core::systems::history::history_process::HistoryCmd::interrupt
    /// 返回0x02000200；这是构造命令，不直接清空队列或强制commit事务。
    pub fn interrupt() -> Self { Self { encoded_word: 0x0200_0200 } }
}
''')

HEADER = '''//! 三版共享的优先建筑候选；implemented_unverified，验证暂停。
//! 原始函数/A/B/C地址见逐函数注释；完整身份与剩余边界见任务根mapping/CONTRACT。
//! 使用同一assembly crate的上批类型，原函数只保留一次实现；完整作者ABI未自动证明。
'''

def main():
    assert (OUT / 'contract-review.md').exists(), '先完成独立合同审查'
    groups = {}
    for item in SELECTION: groups.setdefault(item['source_path'], []).append(item)
    assert set(groups) == set(SOURCES)
    entries = []
    modules = {}
    for path, functions in groups.items():
        tid = functions[0]['file_task_id']
        claim = json.loads((INDEX / 'claims' / (tid + '.json')).read_text())
        assert claim['owner'] == OWNER
        work = Path(claim['work_dir'])
        file = work / path
        file.parent.mkdir(parents=True, exist_ok=True)
        code = HEADER + SOURCES[path]
        mapping = []
        for item in functions:
            name = item['function_name']; marker = '// ORIGINAL: ' + name + '\n'
            assert code.count(marker) == 1, name
            variants = {}
            for version in 'ABC':
                native = []
                for n, instance in enumerate(item['variants'][version]['native_instances']):
                    raw_path = OUT / ('pack-' + version) / (item['logical_group_id'] + '-' + str(n) + '.json')
                    raw = json.loads(raw_path.read_text())
                    body = bytes.fromhex(''.join(i['bytes'] for i in raw['instructions']))
                    sha = hashlib.sha256(body).hexdigest()
                    assert raw['status'] == 'ok' and sha in item['variants'][version]['body_hashes']
                    native.append(dict(instance, va=hex(0x140000000 + instance['rva']), body_sha256=sha, evidence=str(raw_path)))
                variants[version] = {'function_id': item['function_ids'][version], 'native_instances': native,
                    'source_metadata': item['file_metadata'][version]['metadata_path'],
                    'indexed_callers': item['variants'][version]['distinct_callers'],
                    'physical_reference_callers_rva': item['variants'][version]['physical_reference_callers'],
                    'private_callees': item['variants'][version]['private_callees']}
            addresses = '；'.join(v + '=' + ','.join(i['va'] for i in variants[v]['native_instances']) for v in 'ABC')
            code = code.replace(marker, marker + '    // 三版对应：' + addresses + '\n    // 逻辑组：' + item['logical_group_id'] + '\n')
            mapping.append({'original_symbol': name, 'logical_group_id': item['logical_group_id'],
                'source_path': path, 'candidate_path': str(file), 'task_id': tid,
                'status': 'implemented_unverified', 'ABI_equivalence_verified': False,
                'runtime_equivalence_verified': False, 'variants': variants,
                'existing_rust_name_candidates': item['variants']['C']['existing_rust_name_candidates'],
                'dependency_reuse': ['std::Vec::new'] if 'new_edit_sequence' in name else []})
        file.write_text(code)
        (work / 'mapping.json').write_text(json.dumps(mapping, ensure_ascii=False, indent=2) + '\n')
        modules['candidate_' + tid.removeprefix('TG-')] = file
        entries.extend(mapping)
    assembly_task = next(x['file_task_id'] for x in SELECTION if x['source_path'] == 'crates/utils/src/lib.rs')
    assembly_work = INDEX / 'work' / assembly_task / OWNER
    root = assembly_work / 'assembly/lib.rs'; root.parent.mkdir(exist_ok=True)
    code = '//! 独立候选装配入口；不改生产Cargo/lib。glam依赖须由整合绑定目标C原锁版本，未构建。\n'
    for module, file in {**BASE, **modules}.items():
        code += '#[path = ' + json.dumps(str(file)) + ']\npub mod ' + module + ';\n'
    root.write_text(code)
    closure = {str(file): hashlib.sha256(file.read_bytes()).hexdigest() for file in BASE.values()}
    closure[str(PROJECT / 'reconstruction/building/roof_components.rs')] = hashlib.sha256((PROJECT / 'reconstruction/building/roof_components.rs').read_bytes()).hexdigest()
    (assembly_work / 'base-dependencies.json').write_text(json.dumps(closure, ensure_ascii=False, indent=2) + '\n')
    (OUT / 'mapping.json').write_text(json.dumps(entries, ensure_ascii=False, indent=2) + '\n')
    (OUT / 'assembly.json').write_text(json.dumps({'root': str(root), 'base_dependencies': closure, 'new_modules': {k: str(v) for k,v in modules.items()}}, ensure_ascii=False, indent=2) + '\n')
    print(json.dumps({'new_candidates':len(entries), 'files':len(groups), 'assembly':str(root)}))

if __name__ == '__main__': main()
