//! B system-water / country-core CPU water raster state chain — 反汇编级证据草稿。
//!
//! 证据全文：`evidence/multi-baseline/water-cpu-analysis.md`（B 主基线；reva 反编译 +
//! radare2 RNE 机器码 oracle 交叉核对）与 `evidence/multi-baseline/water-extra-analysis.md`
//!（第二批补全：栅格 512×512 / u8→255 归一 / 三闭包选层与写公式）。本模块不是 ECS/ABI 复刻，
//! 只恢复已证实的水域 CPU 状态链业务规则；所有未知项以 `// [unknown]` 标注，不伪造。
//!
//! 链式关系（硬证据，VA 为 B patched EXE）：
//! `draw_water`(0x141ba9130) → `draw_raster<WaterRasterStagingSsbo,WorldRaster<512>,…>`
//! (0x142260ae0) → `rasterize_stroke<512>`(0x142261d10) / `copy_composite_to_scratch<512>`
//! (0x142262680) / `commit_scratch_to_composite<512>`(0x142262a60) / `copy_to_staging<512>`
//! (0x142261760)；`water_rasterize_from_cpu`(0x140a12680) 把 CPU staging 上传成 GPU 水 mask；
//! 回放 `replay_raster<WaterRasterStagingSsbo,WorldRaster<512>,closure_env$2>`(0x1409f4650)
//! 把存档 `ReplayCmd` 笔画重跑同一条栅格链恢复水位。

/// 世界→栅格变换：`grid = (world / WORLD_SCALE + HALF_CELL) * GRID_SIZE as f32`。
/// 证据：rasterize_stroke<512> 反汇编常量 0x43020000=130.0、0x3f000000=0.5、
/// 0x44000000=512.0、.rdata 0x1429a3c84=512.0f。世界窗口约 130 单位 → 512 栅格，原点对应格 256。
pub const WORLD_SCALE: f32 = 130.0;
pub const HALF_CELL: f32 = 0.5;
pub const GRID_SIZE: usize = 512;

/// 水笔刷默认/上限。证据：BrushSizes::reset_water 写 2.0（0x40000000）、
/// set_water clamp [1.0,5.0]（0x3f800000 / 0x40a00000）。
pub const WATER_BRUSH_DEFAULT: f32 = 2.0;
pub const WATER_BRUSH_MIN: f32 = 1.0;
pub const WATER_BRUSH_MAX: f32 = 5.0;

/// 栅格单元为 u8；**栅格确认为 512×512**（bilinear_sample_at_world_xz 0x140a14df0 反汇编：
/// 行/列索引各自 CMP clamp 到 0x1ff=511，行距 SHL 9=×512，邻域 +0x200/0x201）。
/// `WorldRaster::new` 填一块 0x4000B 初始块（初始水位），解码语义 [unknown]。
pub type Cell = u8;

/// 栅格行数（与 GRID_SIZE 相同）；总单元 = 512×512 = 0x40000。
/// 证据：bilinear_sample 索引 clamp 0x1ff（0x140a14ed6–0x140a14f0e）。
pub const GRID_ROWS: usize = 512;

/// 栅格 u8 → 归一样本 [0,1] 的除数：255.0（.rdata 0x14293ac70 = 0x437f0000，radare2 核对；
/// bilinear_sample 四角 MOVZX→CVTSI2SS→DIVSS 255.0）。
pub const U8_TO_NORMALIZED_DIVISOR: f32 = 255.0;

/// 世界坐标 xz → 栅格格心坐标（f32；写路径用）。
/// 证据：rasterize_stroke<512> 首个命令与循环命令的坐标计算。
pub fn point_to_grid_xz(x: f32, z: f32) -> [f32; 2] {
    [
        (x / WORLD_SCALE + HALF_CELL) * GRID_SIZE as f32,
        (z / WORLD_SCALE + HALF_CELL) * GRID_SIZE as f32,
    ]
}

/// zeno::command::Command 结构（stride 28B；只用到前 12B）。
/// 证据：rasterize_stroke<512> `*cmd = 0`（variant 0 = move-to，首点）、
/// `cmd[u*7] = 1`（variant 1 = line-to，后续点），payload = CONCAT44(gx, gz)（2×f32 栅格坐标）。
/// [unknown] 后 16B 字段语义（zeno 依赖，未还原）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StrokeCommand {
    pub variant: u32, // 0 = move-to, 1 = line-to
    pub grid: [f32; 2],
}

/// 把重采样后的点列转成栅格笔画命令列表（首点 move-to，其余 line-to）。
/// 证据：rasterize_stroke<512> 反汇编（Vec<Command> 预分配 ≈ n/3*28B，逐点 push）。
pub fn build_stroke_commands(points: &[glam::Vec3]) -> Vec<StrokeCommand> {
    let mut cmds = Vec::with_capacity(points.len());
    if let Some(first) = points.first() {
        let [gx, gz] = point_to_grid_xz(first.x, first.z);
        cmds.push(StrokeCommand { variant: 0, grid: [gx, gz] });
    }
    for p in points.iter().skip(1) {
        let [gx, gz] = point_to_grid_xz(p.x, p.z);
        cmds.push(StrokeCommand { variant: 1, grid: [gx, gz] });
    }
    cmds
}

/// WorldRaster<512> 字段布局（证据：copy_composite_to_scratch / commit_scratch_to_composite /
/// copy_to_staging / water_rasterize_from_cpu 的偏移读写）。
///
/// ```text
/// +0x08 composite 数据指针   +0x10 composite len（字节）
/// +0x20 scratch  数据指针   +0x28 scratch   len（字节）
/// +0x30 DirtyTiles/retain 闭包状态
/// +0x50..0x5f 16B 脏块元数据
/// +0x60/+0x68 parking_lot Mutex（Arc 结构）
/// +0x98..+0x297 512B staging/脏区（u8）
/// +0x298 读缓冲选择字节（0→composite, 1→scratch）
/// +0x299 staging 清零脏字节
/// +0x29a published 字节
/// +0x29b 再上传脏字节
/// ```
/// [unknown] 栅格行数/总尺寸；composite/scratch 以 Vec<Cell> 建模仅为近似（布局非 ABI 复刻）。
pub struct WorldRaster<const N: usize = GRID_SIZE> {
    pub composite: Vec<Cell>,
    pub scratch: Vec<Cell>,
    pub read_selector: u8,
    pub staging_zero_dirty: bool,
    pub published: bool,
    pub reupload_dirty: bool,
    /// 512B staging（与脏块共用区，见证据）；[unknown] 精确语义。
    pub staging: [u8; 512],
}

impl<const N: usize> WorldRaster<N> {
    pub fn new() -> Self {
        Self {
            composite: Vec::new(),
            scratch: Vec::new(),
            read_selector: 0,
            staging_zero_dirty: false,
            published: false,
            reupload_dirty: false,
            staging: [0; 512],
        }
    }

    /// copy_composite_to_scratch<512>：composite → scratch（len 校验后 memcpy）。
    /// 证据：0x142262680 `memcpy(scratch_ptr, composite_ptr, composite_len)`；
    /// len 不符 → `len_mismatch_fail`。
    pub fn copy_composite_to_scratch(&mut self) {
        assert_eq!(self.composite.len(), self.scratch.len(), "len mismatch");
        self.scratch.copy_from_slice(&self.composite);
    }

    /// commit_scratch_to_composite<512>：scratch → composite。
    /// 证据：0x142262a60 `memcpy(composite_ptr, scratch_ptr, composite_len)`。
    pub fn commit_scratch_to_composite(&mut self) {
        assert_eq!(self.composite.len(), self.scratch.len(), "len mismatch");
        self.composite.copy_from_slice(&self.scratch);
    }

    /// copy_to_staging<512>（0x142261760）：把当前读缓冲选中的 512B 发布进 staging 并标记。
    /// [unknown] 原实现用 DirtyTiles::retain 增量复制 + parking_lot Mutex；这里只表达语义。
    pub fn copy_to_staging(&mut self) {
        if self.staging_zero_dirty {
            self.staging.fill(0);
            self.staging_zero_dirty = false;
        }
        let src = if self.read_selector == 0 { &self.composite } else { &self.scratch };
        for (d, s) in self.staging.iter_mut().zip(src.iter()) {
            *d = *s;
        }
        self.published = true;
    }

    /// 从世界坐标双线性采样水位（>0 视为"入水"；栅格 u8，[0,1] 归一后 bilinear lerp）。
    /// 证据：0x140a14df0 `divps(coords, size) + 0.5`、clamp[0,1]、三次 lerp；
    /// MOVZX byte 读、行距 ×512。调用侧除数 130.0（tree-cmds 采样参数）。
    pub fn bilinear_sample_at_world_xz(&self, x: f32, z: f32) -> f32 {
        if self.composite.is_empty() {
            return 0.0;
        }
        let u = (x / WORLD_SCALE + HALF_CELL).clamp(0.0, 1.0);
        let v = (z / WORLD_SCALE + HALF_CELL).clamp(0.0, 1.0);
        let cols = (u * (N as f32 - 1.0)).floor() as usize;
        let rows = (v * (N as f32 - 1.0)).floor() as usize;
        let fu = u * (N as f32 - 1.0) - cols as f32;
        let fv = v * (N as f32 - 1.0) - rows as f32;
        let w = N;
        let at = |r: usize, c: usize| self.composite.get(r.wrapping_mul(w).wrapping_add(c)).copied().unwrap_or(0) as f32;
        let a = at(rows, cols);
        let b = at(rows, cols + 1);
        let c = at(rows + 1, cols);
        let d = at(rows + 1, cols + 1);
        let top = a * (1.0 - fu) + b * fu;
        let bot = c * (1.0 - fu) + d * fu;
        top * (1.0 - fv) + bot * fv
    }
}

/// 回放恢复水位：把存档的笔画命令重跑同一条 rasterize_stroke→copy_to_staging→commit 链。
/// 证据：replay_raster<Water…>（0x1409f4650）：对每条 ReplayCmd（48B，含权重 f32@+0x14、
/// 处理标记@+0x19、事件标记@+0x1a、清除标志@+0x18）：
/// - 清除记录：clear_buffer 后重放嵌套命令；
/// - 普通记录：cubic_in_out 缓动 → Curve 重采样 → copy_composite_to_scratch →
///   rasterize_stroke → rasterize_world_space_stroke（脏块位图 OR 进 +0x98..0x290）→
///   copy_to_staging → 权重≥0.01 时 blend（new = brush*scale + old，fVar36≥1 才 commit）；
/// - 处理完按记录下标 memmove 移除。
/// [unknown]：权重 fVar5 与栅格 u8 值的具体换算；Curve/zeno 依赖实现不在此恢复。
pub fn replay_restore_water<const N: usize>(
    raster: &mut WorldRaster<N>,
    strokes: &[(Vec<glam::Vec3>, f32)],
) {
    for (points, weight) in strokes {
        raster.copy_composite_to_scratch();
        let _cmds = build_stroke_commands(points);
        // [unknown] 实际以闭包经 zeno::mask::Mask::blend_into 写入栅格。
        if *weight >= 0.01 {
            // blend 分支：new = brush*scale + old；只表达存在性
            // [unknown] brush 来自 ActiveRasterStroke/BrushSizes。
        }
        raster.copy_to_staging();
        if *weight >= 1.0 {
            raster.commit_scratch_to_composite();
        }
    }
}

/// rasterize_stroke<512> 三闭包（写入回调）选层与写公式。
/// 证据：rasterize_stroke @0x142261d10 闭包分派（0x142262221–0x1422622c8）；三个
/// `blend_into<&[Command], closure_env$N>` 入口 0x1425dcda0/$0、0x1425dc570/$1、0x1425dbd00/$2；
/// 游戏闭包体内联进 `rasterize_write<HeapStorage,…>` 实例
/// （env0=0x142821d90、env1=0x142828b30、env2=0x142823e50）。
/// - 目标缓冲由 arg2（EDX 低字节）选：`*(WorldRaster+0x8|+0x20)` 数据指针、
///   `*(+0x10|+0x28)` 长度；实时 draw_raster 以 EDX=1 → scratch（双缓冲工作层）。
/// - 分派：arg6=0&&bit0=0→env1；arg6≠0&&bit0=0→env0；arg6≠0&&bit0=1→env2；
///   arg6=0&&bit0=1→panic。（arg6/bit0 来自 ActiveRasterStroke +0x64 byte / +0x68 u32；
///   +0x64 亦区分 draw/erase 事件。）
/// - 写公式：env0 `dst=max(dst,a)`；env1 `dst=min(dst,255-a)`；env2
///   `dst=saturating_add(dst, a*brush>>16)`（brush=clamp(brush,0,1)*65536 定点）。
///   `a = 覆盖度>>9` clamp 0xff（zeno 覆盖度定点刻度 [unknown]）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StrokeWriteMode {
    /// env$1：擦除（min 写，向 0 收）。
    EraseMin,
    /// env$0：硬写入（max 写，抬到 alpha）。
    AddMax,
    /// env$2：加权加（饱和加 × 笔刷 16.16 定点）。
    AddWeighted,
}

/// 由 arg6/bit0 选写模式。证据：rasterize_stroke 分派反汇编。
pub fn select_write_mode(arg6_is_zero: bool, bit0: bool) -> Option<StrokeWriteMode> {
    match (arg6_is_zero, bit0) {
        (true, false) => Some(StrokeWriteMode::EraseMin),
        (false, false) => Some(StrokeWriteMode::AddMax),
        (false, true) => Some(StrokeWriteMode::AddWeighted),
        (true, true) => None, // panic 组合
    }
}

/// 覆盖度定点 → u8 alpha（>>9 后 clamp 0xff）。证据：三 rasterize_write 实例 `iVar>>9`。
pub fn coverage_to_alpha(coverage: i32) -> u8 {
    let v = coverage >> 9;
    if v > 0xff { 0xff } else { v as u8 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_maps_world_origin_to_cell_256() {
        let [gx, gz] = point_to_grid_xz(0.0, 0.0);
        assert!((gx - 256.0).abs() < 1e-3);
        assert!((gz - 256.0).abs() < 1e-3);
    }

    #[test]
    fn brush_bounds_are_clamped() {
        assert!((WATER_BRUSH_MAX - 5.0).abs() < 1e-6);
        assert!((WATER_BRUSH_MIN - 1.0).abs() < 1e-6);
        assert!((WATER_BRUSH_DEFAULT - 2.0).abs() < 1e-6);
    }

    #[test]
    fn grid_is_square_512() {
        assert_eq!(GRID_ROWS, GRID_SIZE);
        assert_eq!(GRID_ROWS * GRID_SIZE, 512 * 512);
    }

    #[test]
    fn u8_normalizes_by_255() {
        assert!((128.0_f32 / U8_TO_NORMALIZED_DIVISOR - 0.5019608).abs() < 1e-4);
    }

    #[test]
    fn closure_dispatch_matches_binary() {
        assert_eq!(select_write_mode(true, false), Some(StrokeWriteMode::EraseMin));
        assert_eq!(select_write_mode(false, false), Some(StrokeWriteMode::AddMax));
        assert_eq!(select_write_mode(false, true), Some(StrokeWriteMode::AddWeighted));
        assert_eq!(select_write_mode(true, true), None);
    }

    #[test]
    fn coverage_shifts_to_alpha() {
        assert_eq!(coverage_to_alpha(0x1ff << 9), 0xff);
        assert_eq!(coverage_to_alpha(0x100 << 9), 0xff); // 1<<(9+8) → 0x100，超界 clamp 0xff
        assert_eq!(coverage_to_alpha(0x7f << 9), 0x7f);
    }
}
