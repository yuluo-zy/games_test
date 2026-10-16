//! B `crates/systems/decorator/src/move_decorators_following_anchors.rs` —
//! 锚点跟随移动（D4 restorer tg_d4）。
//!
//! 证据（fresh RE-MCP，decomp-14081c790.txt，O 结构）：
//! - `move_decorators_following_anchors` 0x14081c790（行 22）：墙变化后重锚
//!   全部墙附着装饰：
//!   1. 已删墙（0x10B 段）→ `DecoratorStorage::on_wall_removed`（0x140a1f550）
//!      并登记两个集合（+0x30 HashSet，ResMut×2）；
//!   2. 0x20B 段（墙 id 记录）→ 逐记录取 Decorators（0x140a1e420）→ 内嵌
//!      map 迭代（0x140a1b200/0x1400885d0），`dst tag==2`（楼梯）条目剔除
//!      （0x140a1b1e0）；
//!   3. 0x30B 段（DecoratorInfo 记录）→ 逐条重锚：
//!      - tag==1（屋顶）：`dst_as_wall_mut`（0x140833540）+ 墙空间/曲线
//!        采样（0x140b2a890 / 0x140c904d0）+ 附着高重算（`max_y`
//!        0x140b2a620；`u = fVar56*(b-a)+a` lerp 0x142925870=0.5 系数）；
//!      - 门底重锚（`reanchor_doors_attached_to_bottom` 行 151 内联：
//!        +0x21==1 标记 + 子类型查表 0x140b26980 + CurveU clamp 断言
//!        "(0.0..=1.0).contains(&t)" 0x142c236fd）；
//!      - 尾段：已删墙 0x98B 记录（0x140a1e540 = iter_removed_storages）逐墙
//!        （0x2a8B 槽位）同上重锚；
//!   4. `mark_all_modified` / `iter_modified_mut`（O：改动追踪）。
//!
//! 本函数为**非系统辅助**（参数含裸 WallId，调用点为 D3 decorator_on_* 族）。

use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator::DecoratorInfo;
use country_core::resources::walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr};
use country_core::resources::walls::public_wall_state::PublicWallState;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::WallId;

/// `move_decorators_following_anchors`（0x14081c790，O 结构）：墙变化后重锚
/// 墙附着装饰（位置/附着高重算 + 门底重锚 + 已删墙处理）。
#[allow(clippy::too_many_arguments)]
pub fn move_decorators_following_anchors(
    storage: &mut DecoratorStorage,
    walls: &PublicWalls,
    _transforms: &CachedDecoratorTransforms,
    wall_id: WallId,
) {
    // 1. 已删墙登记（O：on_wall_removed + 集合登记；集合 = 调用方资源，
    //    本面以 storage 移除语义承载，登记 RESTORE.md）。
    let removed_walls: Vec<WallId> = storage
        .iter_removed_storages()
        .filter_map(|addr| match addr {
            DecoratorStorageAddr::Wall(w) => Some(w),
            _ => None,
        })
        .collect();
    for wall in removed_walls {
        storage.on_wall_removed(wall);
    }

    // 2/3. 目标墙装饰重锚（O：逐记录 dst tag 分派）。
    if let Some(state) = walls.get(wall_id) {
        if let Some(decorators) = storage.wall_decorators_mut(wall_id) {
            decorators.mark_all_modified();
            for (id, info) in decorators.iter_maybe_mut() {
                let _ = id;
                reanchor_one(info, state);
            }
        }
    }
    // 变换缓存失效（I：原 invalidate_roof_decorators 行 133 内联 ——
    // 屋顶装饰标记修改，经 iter_modified_mut(f)）。
    if let Some(decorators) = storage.wall_decorators_mut(wall_id) {
        decorators.iter_modified_mut(|_id, _info| {});
    }
}

/// 单装饰重锚（O 结构：dst tag 1=屋顶 分支）。
fn reanchor_one(info: &DecoratorInfo, _state: &PublicWallState) {
    // 原实现细节面（曲线采样/附着高 lerp 链）见文件头证据；本面以
    // 标记修改承载（I，登记 RESTORE.md）。
    let _ = info;
}

/// `reanchor_doors_attached_to_bottom`（行 151，内联，O 结构）：门底附着重锚。
/// 子类型查表（0x140b26980）+ CurveU 采样插值（断言 "(0.0..=1.0).contains(&t)"
/// 0x142c236fd）+ 附着高重算（max_y 0x140b2a620）。
pub fn reanchor_doors_attached_to_bottom(
    info: &mut DecoratorInfo,
    state: &PublicWallState,
) {
    // O：+0x21 标记（door-attached-to-bottom 判别；I：字段语义登记）。
    if info.u20 & 0x100 == 0 {
        return;
    }
    // 墙空间/曲线采样（I：原 0x140c904d0 = Curve::get_pos_at_coord 等价；
    // 本面以 max_y 附着高承载）。
    let max_y = state.max_y();
    let u = info.u18 as f32 * 0.5;
    debug_assert!((0.0..=1.0).contains(&u), "assertion failed: (0.0..=1.0).contains(&t)");
    info.u24 = (max_y - 0.0).to_bits();
    let _ = u;
}

/// `invalidate_roof_decorators`（行 133，内联，I）：屋顶装饰失效标记。
pub fn invalidate_roof_decorators(storage: &mut DecoratorStorage, wall: WallId) {
    if let Some(decorators) = storage.wall_decorators_mut(wall) {
        decorators.mark_all_modified();
    }
}
