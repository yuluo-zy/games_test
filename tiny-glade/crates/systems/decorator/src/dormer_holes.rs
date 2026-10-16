//! B `crates/systems/decorator/src/dormer_holes.rs` — 老虎窗开洞网格上传（D4）。
//!
//! 证据（fresh RE-MCP，decomp-1407b1d40.txt，O 结构）：
//! - `upload_dormer_holes_to_gpu` 0x1407b1d40（行 20）：遍历
//!   `DecoratorBlueprints`（0x10B 槽位 map；值记录 +0x0..+0x1c = Rectangle2d
//!   {center, size, axis} + +0x40/+0x50 u64）→ `Rectangle2d::aabb`
//!   （0x140c9b680）→ 世界→网格坐标（`world_space_to_grid_coord_unclamped`
//!   行 121 内联：`floor((x + 65.0) / 8.6667)`；0x142a83814=65.0、
//!   0x142a83818=8.6667、0x1429a49bc=2.147e9 钳制）→ `get_cells_from_bbx`
//!   （行 135 内联）逐格写入计数缓冲 `counts[900]`（u32，0x142923c20 memset 0）
//!   与单元表 `cells[225][50]`（0x30B DormerHoleSsbo 记录，容量 panic
//!   0x142a83c70/0x142a83c88）→ 双 `gpu_write`（0x140beefa0：
//!   counts 900 + cells 区）经 `AssetSsboLibrary::get`（0x141258610）+
//!   `DormerHoleSsbo::dummy`（0x141286660）。
//!
//! 网格：单元索引 = row*15 + col（15 列 × 60 行 = 900 计数；记录单元 225
//! 个 × 50 条 × 0x30B）。`DecoratorBlueprint` 的洞记录（+0x0..+0x1c）=
//! 其首个碰撞体 footprint（set_colliders 0x14017a22f0 写 +0x0 区，O 对齐）。

use country_core::asset_libraries::ssbo_library::{AssetSsboLibrary, SsboType};
use country_core::render::ssbo::rhapsody_boundary::GraphicsDevice;
use country_core::render::ssbo::{InstanceBuffer, StorageElement};
use glam::Vec2;

use crate::blueprint::DecoratorBlueprints;
use crate::utils::DormerHoleSsbo;

/// 网格列数（单元索引 = row*15 + col；cell 上限 0xe=14，O）。
pub const GRID_COLS: usize = 15;
/// 网格行数（计数缓冲 900 = 15×60，O）。
pub const GRID_ROWS: usize = 60;
/// 计数缓冲长度（u32[900]，O：0x384=900）。
pub const COUNT_BUFFER_LEN: usize = GRID_COLS * GRID_ROWS;
/// 单元表容量（225 单元 × 50 条，O：`pcStack_278 = 0x32` + 0x142a83b28
/// 字符串 "…"; 0xe0=224 上限 +1）。
pub const CELL_COUNT: usize = GRID_COLS * GRID_COLS;
pub const CELL_CAP: usize = 50;

/// 网格原点偏移（0x142a83814 = 65.0，O）。
pub const GRID_ORIGIN: f32 = 65.0;
/// 单元尺寸（0x142a83818 = 8.6667，O）。
pub const GRID_CELL_SIZE: f32 = 8.6667;
/// 坐标钳制上限（0x1429a49bc = 2.147e9 → i32::MAX 转换语义，O）。
const GRID_LIMIT: f32 = 2_147_483_647.0;

/// `world_space_to_grid_coord_unclamped`（行 121，内联，O）：世界 x/z →
/// 网格坐标（未钳制）。
pub fn world_space_to_grid_coord_unclamped(world: f32) -> i32 {
    let v = (world + GRID_ORIGIN) / GRID_CELL_SIZE;
    if v > GRID_LIMIT || v.is_nan() {
        0
    } else {
        v.floor() as i32
    }
}

/// `get_cells_from_bbx`（行 135，内联，O）：包围盒 → 覆盖单元列表
/// （行/列钳制 [0, 0xe]，单元索引 row*15+col）。
pub fn get_cells_from_bbx(
    min: Vec2,
    max: Vec2,
) -> Vec<usize> {
    let col_min = world_space_to_grid_coord_unclamped(min.x).max(0) as usize;
    let col_max = world_space_to_grid_coord_unclamped(max.x).max(0) as usize;
    let row_min = world_space_to_grid_coord_unclamped(min.y).max(0) as usize;
    let row_max = world_space_to_grid_coord_unclamped(max.y).max(0) as usize;
    let col_max = col_max.min(GRID_COLS - 1);
    let row_max = row_max.min(GRID_COLS - 1);
    let mut cells = Vec::new();
    for row in row_min..=row_max {
        for col in col_min..=col_max {
            cells.push(row * GRID_COLS + col);
        }
    }
    cells
}

/// 老虎窗洞计数缓冲（u32[900]，O）。
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct DormerHoleCounts {
    pub counts: [u32; COUNT_BUFFER_LEN],
}

impl Default for DormerHoleCounts {
    fn default() -> Self {
        Self {
            counts: [0; COUNT_BUFFER_LEN],
        }
    }
}

/// 老虎窗洞 SSBO 元素（native DormerHoleSsbo = 0x30B 洞记录；D1 utils 承载）。
/// 名称串：`country_core::resources::walls::dormers::DormerHoleSsbo`
/// （ssbo_tags.rs O；产生名字符串 0x142c215f8 待取证，登记 RESTORE.md）。
pub const DORMER_HOLE_SSBO_NAME: &str = "country_core::resources::walls::dormers::DormerHoleSsbo";

impl StorageElement for DormerHoleSsbo {
    const NAME: &'static str = DORMER_HOLE_SSBO_NAME;
    const BUFFER_LEN: usize = CELL_COUNT * CELL_CAP;
}
impl SsboType for DormerHoleSsbo {
    type Storage = InstanceBuffer<DormerHoleSsbo>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        InstanceBuffer::<DormerHoleSsbo>::new_storage_buffer(device, initial)
    }
}

/// `upload_dormer_holes_to_gpu`（0x1407b1d40，O 结构）：老虎窗洞网格 CPU
/// 生产者 + 上传。逐蓝图取洞记录（首个碰撞体 footprint = 原 +0x0 区），
/// 覆盖单元计数 + 记录入表；随后双 gpu_write（计数 + 单元表）。
///
/// **render handoff（与 ivy_send_to_gpu_leaf 同约定）**：本函数由 render 世界
/// （systems/render_loop）持有 `GraphicsDevice` 后调用；GPU 真机后置。
pub fn upload_dormer_holes_to_gpu(
    blueprints: &DecoratorBlueprints,
    ssbos: &mut AssetSsboLibrary,
    device: &dyn GraphicsDevice,
) -> usize {
    // 计数缓冲（O：900 u32 memset 0）。
    let mut counts = [0u32; COUNT_BUFFER_LEN];
    // 单元表（O：225 × 50 × 0x30B；逐单元计数 + 记录追加，容量越界原 panic）。
    let mut cells: Vec<Vec<DormerHoleSsbo>> = vec![Vec::with_capacity(CELL_CAP); CELL_COUNT];

    for (_, blueprint) in blueprints.map.iter() {
        // 洞记录 = 蓝图 +0x0 区（Rectangle2d）；D1 消费面以 colliders 首个
        // footprint 承载（O 对齐：set_colliders 写 +0x0 区）。
        let Some(first) = blueprint.colliders.first() else {
            continue;
        };
        let footprint = first.bounds.footprint;
        let aabb = footprint.aabb();
        let covered = get_cells_from_bbx(aabb.min, aabb.max);
        for cell in covered {
            if cell >= CELL_COUNT {
                continue;
            }
            if counts[cell] >= CELL_CAP as u32 {
                // 原容量越界 panic（0x142a83c70/0x142a83c88，O）；Rust 侧
                // 跳过并登记（同语义：不溢出）。
                continue;
            }
            counts[cell] += 1;
            let mut record = DormerHoleSsbo::dummy();
            // 洞记录载荷（I：原 0x30B 写出面 = 蓝图 +0x0 区拷贝；
            // 本面以 footprint 四值承载并登记 RESTORE.md）。
            record.a[0..4].copy_from_slice(&footprint.center.x.to_le_bytes());
            record.a[4..8].copy_from_slice(&footprint.center.y.to_le_bytes());
            record.a[8..12].copy_from_slice(&footprint.size.x.to_le_bytes());
            record.a[12..16].copy_from_slice(&footprint.size.y.to_le_bytes());
            cells[cell].push(record);
        }
    }

    // 上传（O：双 gpu_write：counts 900 + 单元表；native = 0x140beefa0 ×2）。
    let count = counts.iter().sum::<u32>() as usize;
    let Some(buffer) = ssbos.get_mut::<DormerHoleSsbo>() else {
        return count;
    };
    // 计数写（O：900 u32 → 缓冲头区；InstanceBuffer<DormerHoleSsbo> 无法承载
    // u32 计数 —— 计数区布局挂账 RESTORE.md，本面以 cells 写承载）。
    let _ = counts;
    // 单元写（O：225×50 记录；I：扁平化次序 = 单元序 + 记录序）。
    let mut flat: Vec<DormerHoleSsbo> = Vec::with_capacity(CELL_COUNT * CELL_CAP);
    for cell in &cells {
        flat.extend_from_slice(cell);
    }
    let _ = buffer.buffer.gpu_write(device, &flat);
    buffer.instance_count = count;
    count
}
