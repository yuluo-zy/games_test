//! process_autoclutter_candidates（B 主基线；0x1422ce260，size 2375）。
//!
//! fresh 反编译（2026-10-17，--worker shared）控制流：
//! 1. 读 state+0x20/+0x28 区域候选（stride 0x98）；
//! 2. `from_iter<OrderedCandidate, FilterMap<Enumerate<Iter>, closure_env_0>>`（L40）枚举+过滤
//!    （带 u16 源下标 @+0xc）；
//! 3. 排序（closure_env_1 比较器；<0x15 用 insertion_sort_shift_left 否则 driftsort_main）；
//! 4. 遍历有序候选：源候选 `byte@+0x30 == 6` → 强制接受；否则对已接受缓冲（stride 0x20，
//!    存源候选 +0x68..+0x80 Rectangle3d）逐条 `Rectangle3d::overlaps_rectangle3d`
//!    （0x140c98580）剔除重叠；
//! 5. 接受路径 switch 源判别 `*puVar26`（3/4/5）：case3 截 u32、case4 Arc LOCK++、case5
//!    裸指针；复制字段写输出 state+0x48（stride 0x68，grow_one<WallPathSegment>）；
//! 6. debug `*(in_R9+0xf2)==1 && *(in_R9+0x102)==4` 时 DebugRectangle3d 可视化。
//!
//! 源(0x98)→输出(0x68)字段映射（O）：
//! +0x00 tag；+0x08 Arc/数据指针（case4 LOCK++、case5 源指针）；+0x10 键（case3/默认截 u32、
//! case4 全 u64）；+0x18=源+0x18；+0x20=源+0x20；+0x28=源+0x28；+0x30=源+0x30 byte；
//! +0x34=源+0x34；+0x38=源+0x38；+0x40=源+0x40；+0x48=源+0x48；+0x50=源+0x50；
//! +0x54=源+0x54 低 32；+0x58=源+0x54 高 32；+0x5c=源+0x5c；+0x60=源+0x60。

use bevy_ecs::prelude::*;

use super::{AutoClutterCandidate, AutoClutterSpawn, AutoClutterState, SpawnShared};

/// 有序候选（16 B；u16 源下标 @+0xc）。
#[derive(Clone, Copy, Debug)]
struct OrderedCandidate {
    /// +0x00：键/哈希（enumerate 上下文，sort 后遍历不直接读）。
    _key: u64,
    /// +0x0c：u16 源候选下标。
    index: u16,
}

/// 已接受缓冲（stride 0x20；Rectangle3d @+0x00 起）。
#[derive(Clone, Copy, Debug)]
struct AcceptedRecord {
    rect: utils::geometry::rectangle::Rectangle3d,
    tag: u64,
    shared: u64,
    key: u64,
}

/// process_autoclutter_candidates（0x1422ce260）。
pub fn process_autoclutter_candidates(mut state: ResMut<AutoClutterState>) {
    // 1+2+3：枚举过滤（closure$0 L40）→ 稳定排序（closure$1 L56 比较器）。
    let mut ordered: Vec<OrderedCandidate> = state
        .region_candidates
        .iter()
        .enumerate()
        .filter_map(|(i, c)| filter_candidate(c).then_some(i as u16))
        .map(|index| OrderedCandidate { _key: 0, index })
        .collect();
    ordered.sort_by(|a, b| cmp_candidate(&state.region_candidates, a.index, b.index));

    // 4+5：遍历有序候选 → 强制路径 / 重叠剔除 → 输出写。
    let mut accepted: Vec<AcceptedRecord> = Vec::new();
    let src = std::mem::take(&mut state.region_candidates);
    for oc in &ordered {
        // O：`panic_bounds_check`（len <= 源下标）。
        assert!(
            (oc.index as usize) < src.len(),
            "process_autoclutter_candidates: bounds check failed"
        );
        let candidate = &src[oc.index as usize];
        let mut accepted_spawn = None;
        if candidate.forced() {
            // 强制接受路径（byte@+0x30==6）：复制字段。
            accepted_spawn = Some(accept_candidate(candidate, &mut accepted));
        } else {
            // 贪心矩形重叠剔除（O：Rectangle3d::overlaps_rectangle3d）。
            let mut overlap = false;
            for acc in &accepted {
                if acc.rect.overlaps_rectangle3d(candidate.rect) {
                    overlap = true;
                    break;
                }
            }
            if !overlap {
                accepted_spawn = Some(accept_candidate(candidate, &mut accepted));
            }
        }
        if let Some(spawn) = accepted_spawn {
            state.spawn_candidates.push(spawn);
        }
    }
    state.region_candidates = src;
}

/// 过滤闭包（PDB closure$0 L40；**过滤谓词 U——评审独立反编译未见 body**，现
/// `forced==0 || forced==6` 为未取证猜测，不得判等；结构 = enumerate+filter_map）。
fn filter_candidate(c: &AutoClutterCandidate) -> bool {
    c.forced == 0 || c.forced == 6
}

/// 排序比较器（PDB closure$1 L56；**排序键 U——评审独立反编译未见 body**，现按源候选
/// +0x34（f34）为未取证猜测；结构 = 稳定排序（<0x15 insertion 否则 driftsort））。
fn cmp_candidate(candidates: &[AutoClutterCandidate], a: u16, b: u16) -> std::cmp::Ordering {
    let va = candidates.get(a as usize).map_or(0u32, |c| c.f34);
    let vb = candidates.get(b as usize).map_or(0u32, |c| c.f34);
    va.cmp(&vb)
}

/// 接受路径（O：switch 源判别 3/4/5 + 写已接受缓冲 + 输出字段复制）。
fn accept_candidate(
    c: &AutoClutterCandidate,
    accepted: &mut Vec<AcceptedRecord>,
) -> AutoClutterSpawn {
    // O：已接受缓冲 = 源候选 +0x68..+0x80 Rectangle3d。
    accepted.push(AcceptedRecord {
        rect: c.rect,
        tag: 0,
        shared: 0,
        key: 0,
    });
    // O：switch `*puVar26`：case3 截 u32；case4 Arc LOCK++；case5 裸指针；默认截 u32。
    // case5 的 key（uVar23 未被 switch 复写，Ghidra 为跳表地址残留）语义 U——现截 u32 为
    // 未取证猜测。
    let (tag, shared, key) = match c.kind {
        4 => {
            // case4：Arc 强计数++（LOCK 语义；**U 登记缝**——Arc payload 合同 U，登记不执行）。
            (4u8, c.shared, c.key)
        }
        5 => {
            // case5：裸指针语义（源候选指针；以 shared 承载）。
            (5u8, c.shared, c.key & 0xffff_ffff)
        }
        _ => {
            // case3 / default：截 u32。
            (3u8, c.shared, c.key & 0xffff_ffff)
        }
    };
    AutoClutterSpawn {
        kind: tag,
        shared: if tag == 4 {
            SpawnShared::ArcStrong(shared)
        } else {
            SpawnShared::Prefab(String::new())
        },
        f18: (c.f18 & 0xffff_ffff) as u32,
        key,
        marker: c.forced,
        // 输出 +0x38/+0x40 打包为位置（low32 取 f32 x/z、y=0）——为推断，非取证（U）。
        position: glam::Vec3::new(
            f32::from_bits(c.f38 as u32),
            0.0,
            f32::from_bits(c.f40 as u32),
        ),
        host_id: c.f48,
        f48: c.f48,
        f50: c.f50 as u64,
        f54: (c.f54 & 0xffff_ffff) as u32,
        f58: ((c.f54 >> 32) & 0xffff_ffff) as u32,
        f5c: c.f5c,
        scale: f32::from_bits(c.f60),
    }
}
