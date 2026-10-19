//! B `decorator_visual/lantern.rs` — 灯笼装饰视觉与灯光维护（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x14078d320（generate_lanterns，91KB）/
//! 0x140790400（maintain_lantern_lights，24KB）/ 0x141257fa0
//! （compare_light_lists closure_env<3>，7.3KB）/ 0x1412583d0
//! （closure_env<5>，2.9KB）+ 骨架 callees（O）。
//!
//! 业务（O 结构）：generate_lanterns 逐存储逐 modified 灯笼 → 六键
//! （collision/interaction/outline × rank 变化）→ as_lantern 载荷 → 蓝图
//! （set_colliders）+ 视觉实体（lantern_light 标记）→ 灯光记录。
//!
//! `maintain_lantern_lights`（0x140790400）：
//! - 灯光记录 **0x1cB = 28B**（O：fresh compare 闭包读取面）：
//!   `{key u32@0x00, u16@0x0c, color_id u64@0x10, value f32@0x18}`；
//! - 生成门（O，fresh 0x140790400）：`fVar60 = rng.f32()*0.14 + 0.01`
//!   （常量 0x142a80d40=0.14、0x142960674=0.01）；olden/加载门真时
//!   `0.45 + 0.2*fVar60 <= t && t < 0.72`（0x142a7fca4=0.45、
//!   0x1429cd068=0.2、0x142a80d44=0.72）；否则
//!   `fVar6 < 1.0 && fVar60 <= fVar6`（`fVar6 = 资源+0x24`，字段身份 U）。
//!   记录上限 **0x8000**（`if (len < 0x8000)`）。
//! - 新旧列表比较（0x141257fa0/0x1412583d0）：归并扫描，逐条分类
//!   `0=仅旧、1=仅新、2=相同`；差异输出 0x28B 记录（`tag 0xe @+8`、
//!   载荷 0x1bB @+9、尾字节 0）。GPU `update_buffer_bytes` 0x140beefa0 为
//!   渲染后置面（rhapsody 设备 owner），本模块产出真实 CPU 记录/差异，
//!   GPU 上传登记跨 crate。
//!
//! U 登记：① 灯载荷（LanternExtra u32）语义 ② 记录其余字段（+4..+0xc）
//! ③ 非 olden 分支 `资源+0x24` 字段身份 ④ GPU 上传调用面。
use bevy_ecs::prelude::*;
use country_core::resources::session::ActiveSession;
use country_core::resources::time::GameTime;
use country_core::resources::walls::decorator::DecoratorExtraInfo;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary;
use glam::{Quat, Vec3};

use crate::blueprint::{DecoratorBlueprint, DecoratorBlueprints};
use crate::chimney_smoke_anim_thickness::TimeOfDayCycle;
use crate::decorator_visual_state::DecoratorTransientVisualState;
use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;

/// 灯光记录步长（O：0x1cB = 28B）。
pub const LIGHT_RECORD_SIZE: usize = 0x1c;
/// 灯光记录上限（O：`if (len < 0x8000)`）。
pub const LIGHT_RECORD_CAP: usize = 0x8000;

/// 原生 0x1cB 灯光记录（字段语义部分 U，保留原字节）。
pub type LanternLightRecord = [u8; LIGHT_RECORD_SIZE];

/// 记录键（O：比较闭包读取 `*record` u32）。
#[inline]
pub fn light_record_key(record: &LanternLightRecord) -> u32 {
    u32::from_le_bytes(record[0..4].try_into().unwrap())
}

/// 记录颜色 id（O：compare 读 `+0x10` u64）。
#[inline]
pub fn light_record_color_id(record: &LanternLightRecord) -> u64 {
    u64::from_le_bytes(record[0x10..0x18].try_into().unwrap())
}

/// 记录值（O：compare 读 `+0x18` f32）。
#[inline]
pub fn light_record_value(record: &LanternLightRecord) -> f32 {
    f32::from_le_bytes(record[0x18..0x1c].try_into().unwrap())
}

/// 灯光列表差异类别（O：closure_env$5 状态 0/1/2）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LightListDiffKind {
    /// 仅旧列表（0）。
    OldOnly,
    /// 仅新列表（1）。
    NewOnly,
    /// 两列表相同键（2）。
    Both,
}

/// 单条比较结果（O：closure_env$5 `FUN_141257c50(ctx, {kind, record})`）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LightListDiff {
    pub kind: LightListDiffKind,
    pub record: LanternLightRecord,
}

/// `compare_light_lists`（0x141257fa0 / 0x1412583d0，O）：归并扫描两条
/// 按 `key` 升序的 0x1cB 记录列表，逐条分类：
/// - `old.key == new.key` → `Both`（双方前进）；
/// - `old.key < new.key` → `OldOnly`（旧前进）；
/// - `old.key > new.key` → `NewOnly`（新前进）；
/// - 尾部剩余逐条按所属列表分类。
///
/// 差异输出记录（0x141257fa0）：仅 `OldOnly`/`NewOnly` 生成 0x28B 记录
/// `{seq u64@0, tag=0xe @8, 载荷 0x1bB @9, 0@0x24}`——调用方按需构造。
pub fn compare_light_lists(
    old: &[LanternLightRecord],
    new: &[LanternLightRecord],
) -> Vec<LightListDiff> {
    let mut out = Vec::with_capacity(old.len().abs_diff(new.len()) + 1);
    let (mut i, mut j) = (0usize, 0usize);
    while i < old.len() && j < new.len() {
        let ok = light_record_key(&old[i]);
        let nk = light_record_key(&new[j]);
        match ok.cmp(&nk) {
            core::cmp::Ordering::Equal => {
                out.push(LightListDiff {
                    kind: LightListDiffKind::Both,
                    record: old[i],
                });
                i += 1;
                j += 1;
            }
            core::cmp::Ordering::Less => {
                out.push(LightListDiff {
                    kind: LightListDiffKind::OldOnly,
                    record: old[i],
                });
                i += 1;
            }
            core::cmp::Ordering::Greater => {
                out.push(LightListDiff {
                    kind: LightListDiffKind::NewOnly,
                    record: new[j],
                });
                j += 1;
            }
        }
    }
    while i < old.len() {
        out.push(LightListDiff {
            kind: LightListDiffKind::OldOnly,
            record: old[i],
        });
        i += 1;
    }
    while j < new.len() {
        out.push(LightListDiff {
            kind: LightListDiffKind::NewOnly,
            record: new[j],
        });
        j += 1;
    }
    out
}

/// 差异输出记录（0x28B，O：`tag 0xe @+8`、载荷 0x1bB @+9、`0 @+0x24`）。
pub fn light_diff_output_record(seq: u64, record: &LanternLightRecord) -> [u8; 0x28] {
    let mut out = [0u8; 0x28];
    out[0..8].copy_from_slice(&seq.to_le_bytes());
    out[8] = 0x0e;
    // O：载荷 0x1bB（27B）@+9..+0x24；记录第 28 字节不进入输出。
    out[9..9 + LIGHT_RECORD_SIZE - 1].copy_from_slice(&record[..LIGHT_RECORD_SIZE - 1]);
    out[0x24] = 0;
    out
}

/// 灯光双缓冲 + 差异记录（替代原生内联双 Vec，O 结构）。
#[derive(Resource, Clone, Debug, Default)]
pub struct LanternLightLists {
    /// 旧列表（原生 +0x08/+0x10）。
    pub old: Vec<LanternLightRecord>,
    /// 新列表（原生 +0x20/+0x28）。
    pub new: Vec<LanternLightRecord>,
    /// 上帧比较结果（0x141257fa0/0x1412583d0 输出）。
    pub diffs: Vec<LightListDiff>,
    /// 差异记录序号（原生输出流计数器 +0x40）。
    pub seq: u64,
    /// 差异输出记录（0x28B；GPU 上传后置）。
    pub output: Vec<[u8; 0x28]>,
}

/// 灯笼点亮门（O，fresh 0x140790400；非 olden 分支 +0x24 身份 U）。
///
/// `fVar60 = rng*0.14 + 0.01`；olden 门真时
/// `0.45 + 0.2*fVar60 <= t < 0.72`；否则 `fVar6 < 1.0 && fVar60 <= fVar6`
/// ——`fVar6` 为外部资源字段（U），本实现以 `t` 承载并显式登记。
pub fn lantern_lit(olden: bool, t: f32, fvar60: f32) -> bool {
    if olden {
        0.45 + 0.2 * fvar60 <= t && t < 0.72
    } else {
        t < 1.0 && fvar60 <= t
    }
}

/// 0x14078d320（O 结构）：生成灯笼装饰视觉。
pub fn generate_lanterns(
    decorator_storage: Res<DecoratorStorage>,
    mesh_lib: Res<DecoratorMeshLibrary>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    mut transient: ResMut<DecoratorTransientVisualState>,
    _glade: Res<ActiveSession>,
) {
    for (_wall, decorators) in decorator_storage.iter() {
        for (_id, info) in decorators.iter_modified() {
            // O 判别：DecoratorExtraInfo::as_lantern（非灯笼 panic）。
            let lantern = info.extra().as_lantern();
            let _payload = lantern.0; // U：灯载荷语义。
            // O：六键（DecoratorLibKey::new ×6 + rank ×6 = 碰撞/交互/描边 ×
            // 2 侧；I 键面）。
            let key = country_core::resources::walls::decorator_subtype::DecoratorLibKey::new(
                info.decorator_type().0,
                info.rank(),
            );
            let mut blueprint = DecoratorBlueprint::new();
            if let Some(collision) = mesh_lib.collision_mesh(key) {
                blueprint.set_colliders(vec![collision]);
            }
            blueprints.insert(info.decorator_id, blueprint);
            // O：视觉实体（lantern_light 标记）。位置由缓存变换消费面提供
            // （U：本函数无 transforms 参数；实体 builder 记录保留）。
            let builder = WindowVisualEntityBuilder::new(
                Vec3::ZERO, // U：位置 = 缓存变换消费面待取证。
                Quat::IDENTITY,
                info.decorator_id,
            );
            let _ = &mut transient;
            let _ = &builder;
        }
    }
}

/// 0x140790400（O 结构）：维护灯笼灯光（新旧 0x1cB 列表比较 + 差异记录）。
///
/// 真实输出 = [`LanternLightLists`]（旧/新列表、0/1/2 分类差异、0x28B 输出
/// 记录）。GPU `update_buffer_bytes`（0x140beefa0）为渲染后置面，设备 owner
/// 在 rhapsody，登记跨 crate 缺口。
pub fn maintain_lantern_lights(
    storage: Res<DecoratorStorage>,
    cycle: Res<TimeOfDayCycle>,
    session: Res<ActiveSession>,
    _time: Res<GameTime>,
    mut lists: ResMut<LanternLightLists>,
) {
    let olden = session.is_olden();
    let t = cycle.t;
    let mut new: Vec<LanternLightRecord> = Vec::new();
    // 收集灯笼装饰（I：原版经墙/屋顶窗桶表收集；core 密集表未暴露——
    // 以 DecoratorStorage 全桶 modified 条目承载，语义 = 每灯笼一条记录）。
    let mut collect = |id: u32, info: &country_core::resources::walls::decorator::DecoratorInfo| {
        if !matches!(info.extra(), DecoratorExtraInfo::Lantern(_)) {
            return;
        }
        if new.len() >= LIGHT_RECORD_CAP {
            return;
        }
        // O：`fVar60 = rng*0.14 + 0.01`（全局 fastrand，无 init_rng）。
        let fvar60 = fastrand::f32() * 0.14 + 0.01;
        let lit = lantern_lit(olden, t, fvar60);
        let mut record = [0u8; LIGHT_RECORD_SIZE];
        record[0..4].copy_from_slice(&id.to_le_bytes());
        // U：记录 +4..+0xc 字段（原为窗桶实体数据）；保留 0。
        record[0x10..0x18].copy_from_slice(&u64::from(info.decorator_id).to_le_bytes());
        record[0x18..0x1c].copy_from_slice(&(if lit { 1.0f32 } else { 0.0 }).to_le_bytes());
        new.push(record);
    };
    for (_, bucket) in storage.iter() {
        for (id, info) in bucket.iter_modified() {
            collect(id, info);
        }
    }
    for bucket in storage.iter_terrain_decorators() {
        for (id, info) in bucket.iter_modified() {
            collect(id, info);
        }
    }
    for bucket in storage.iter_stair_decorators() {
        for (id, info) in bucket.iter_modified() {
            collect(id, info);
        }
    }
    // O：比较前按 key 升序（归并扫描前提）。
    new.sort_by_key(|r| light_record_key(r));
    lists.diffs = compare_light_lists(&lists.old, &new);
    // O：差异输出记录（tag 0xe，0x28B）。先取走 diffs 再写 seq/output，
    // 避免同一资源字段的不可变/可变借用重叠。
    let diffs = std::mem::take(&mut lists.diffs);
    lists.output.clear();
    for diff in &diffs {
        if diff.kind != LightListDiffKind::Both {
            let record = light_diff_output_record(lists.seq, &diff.record);
            lists.seq = lists.seq.wrapping_add(1);
            lists.output.push(record);
        }
    }
    lists.diffs = diffs;
    // 双缓冲推进。
    lists.old = std::mem::take(&mut lists.new);
    lists.new = new;
}

/// rhapsody 设备句柄契约（U：更新缓冲调用面待取证）。
pub mod rhapsody_contract {
    use bevy_ecs::resource::Resource;

    /// GraphicsDevice 句柄（D1 占位契约：maintain_lantern_lights 消费
    /// update_buffer_bytes 0x140beefa0；真实设备资源归 rhapsody 集成）。
    #[derive(Resource, Clone, Debug, Default)]
    pub struct GraphicsDeviceHandle {
        /// U。
        pub _gap: u64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rec(key: u32, color: u64, value: f32) -> LanternLightRecord {
        let mut r = [0u8; LIGHT_RECORD_SIZE];
        r[0..4].copy_from_slice(&key.to_le_bytes());
        r[0x10..0x18].copy_from_slice(&color.to_le_bytes());
        r[0x18..0x1c].copy_from_slice(&value.to_le_bytes());
        r
    }

    #[test]
    fn compare_classifies_old_new_and_equal_keys() {
        let old = vec![rec(1, 10, 1.0), rec(3, 30, 1.0), rec(5, 50, 1.0)];
        let new = vec![rec(2, 20, 1.0), rec(3, 31, 0.0), rec(6, 60, 1.0)];
        let diffs = compare_light_lists(&old, &new);
        let kinds: Vec<_> = diffs.iter().map(|d| (light_record_key(&d.record), d.kind)).collect();
        assert_eq!(
            kinds,
            vec![
                (1, LightListDiffKind::OldOnly),
                (2, LightListDiffKind::NewOnly),
                (3, LightListDiffKind::Both),
                (5, LightListDiffKind::OldOnly),
                (6, LightListDiffKind::NewOnly),
            ]
        );
        // 相同键记录取旧侧（closure$5 对 Both 只报告键）。
        assert_eq!(light_record_color_id(&diffs[2].record), 30);
    }

    #[test]
    fn compare_empty_and_tail_cases() {
        assert!(compare_light_lists(&[], &[]).is_empty());
        let new = vec![rec(1, 1, 0.0), rec(2, 2, 0.0)];
        let d = compare_light_lists(&[], &new);
        assert_eq!(d.len(), 2);
        assert!(d.iter().all(|x| x.kind == LightListDiffKind::NewOnly));
        let d = compare_light_lists(&new, &[]);
        assert!(d.iter().all(|x| x.kind == LightListDiffKind::OldOnly));
    }

    #[test]
    fn diff_output_record_layout_matches_native_0x28b() {
        let r = rec(9, 0xdead_beef, 1.0);
        let out = light_diff_output_record(7, &r);
        assert_eq!(out.len(), 0x28);
        assert_eq!(u64::from_le_bytes(out[0..8].try_into().unwrap()), 7);
        assert_eq!(out[8], 0x0e);
        assert_eq!(&out[9..9 + LIGHT_RECORD_SIZE - 1], &r[..LIGHT_RECORD_SIZE - 1]);
        assert_eq!(out[0x24], 0);
    }

    #[test]
    fn lit_window_uses_native_constants() {
        // O：0.45 + 0.2*fVar60 <= t < 0.72（olden 门）。
        assert!(lantern_lit(true, 0.46, 0.01));
        assert!(!lantern_lit(true, 0.44, 0.01));
        assert!(!lantern_lit(true, 0.72, 0.01));
        // 非 olden 分支（U：字段 +0x24 以 t 承载）。
        assert!(lantern_lit(false, 0.5, 0.1));
        assert!(!lantern_lit(false, 0.05, 0.1));
    }
}
