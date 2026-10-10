//! 分块高度场的权威数据与笔刷事务；采样和显示共用固定三角划分。
//! 每块只保存一个当前快照，克隆地形不会深拷贝所有采样点。
use crate::{DomainError, strokes::Point};
use std::{collections::BTreeMap, sync::Arc};
pub const CELL: f64 = 0.25;
pub const TILE_CELLS: usize = 16;
pub const TILE_SIZE: f64 = CELL * TILE_CELLS as f64;
const SIDE: usize = TILE_CELLS + 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TileId(pub i32, pub i32);
#[derive(Debug, Clone, PartialEq)]
pub struct TerrainTile {
    pub heights: Vec<f64>,
}
impl Default for TerrainTile {
    fn default() -> Self {
        Self {
            heights: vec![0.; SIDE * SIDE],
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TerrainDocument {
    pub tiles: BTreeMap<TileId, Arc<TerrainTile>>,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BrushKind {
    Raise,
    Lower,
    Smooth,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BrushSample {
    pub center: Point,
    pub radius: f64,
    pub amount: f64,
    pub kind: BrushKind,
}
impl TerrainDocument {
    /// 只接受有限、有限范围的高度，避免无界分配和不可见的非法几何。
    pub fn validate(&self) -> Result<(), DomainError> {
        if self.tiles.len() > 140
            || self.tiles.iter().any(|(id, t)| {
                !(-7..=6).contains(&id.0)
                    || !(-4..=3).contains(&id.1)
                    || t.heights.len() != SIDE * SIDE
                    || t.heights
                        .iter()
                        .any(|h| !h.is_finite() || !(-1.5..=3.).contains(h))
            })
        {
            return Err(DomainError("invalid terrain"));
        }
        Ok(())
    }
    pub fn vertex(&self, x: i32, z: i32) -> f64 {
        let tx = x.div_euclid(TILE_CELLS as i32);
        let tz = z.div_euclid(TILE_CELLS as i32);
        let local = x.rem_euclid(TILE_CELLS as i32) as usize
            + z.rem_euclid(TILE_CELLS as i32) as usize * SIDE;
        self.tiles
            .get(&TileId(tx, tz))
            .map_or(0., |t| t.heights[local])
    }
    /// 同显示网格一样按左上到右下的对角线划分，不能另用双线性曲面。
    pub fn height(&self, p: Point) -> f64 {
        let x = p.x / CELL;
        let z = p.z / CELL;
        let ix = x.floor() as i32;
        let iz = z.floor() as i32;
        let u = x - ix as f64;
        let v = z - iz as f64;
        let a = self.vertex(ix, iz);
        let b = self.vertex(ix + 1, iz);
        let c = self.vertex(ix, iz + 1);
        let d = self.vertex(ix + 1, iz + 1);
        if u >= v {
            a + u * (b - a) + v * (d - b)
        } else {
            a + v * (c - a) + u * (d - c)
        }
    }
    pub fn normal(&self, p: Point) -> [f64; 3] {
        let d = CELL * 0.1;
        let dx = (self.height(Point { x: p.x + d, ..p }) - self.height(Point { x: p.x - d, ..p }))
            / (2. * d);
        let dz = (self.height(Point { z: p.z + d, ..p }) - self.height(Point { z: p.z - d, ..p }))
            / (2. * d);
        let l = (1. + dx * dx + dz * dz).sqrt();
        [-dx / l, 1. / l, -dz / l]
    }
    pub fn tiles_in(bounds: [f64; 4]) -> Vec<TileId> {
        let mut out = Vec::new();
        for z in (bounds[1] / TILE_SIZE).floor() as i32..=(bounds[3] / TILE_SIZE).floor() as i32 {
            for x in (bounds[0] / TILE_SIZE).floor() as i32..=(bounds[2] / TILE_SIZE).floor() as i32
            {
                if (-7..=6).contains(&x) && (-4..=3).contains(&z) {
                    out.push(TileId(x, z));
                }
            }
        }
        out
    }
    /// 共享边界的顶点同时写入相邻块；采样入口与网格边缘不会形成裂缝。
    fn set_vertex(&mut self, x: i32, z: i32, height: f64) {
        let tx = x.div_euclid(16);
        let tz = z.div_euclid(16);
        for a in if x.rem_euclid(16) == 0 {
            vec![tx - 1, tx]
        } else {
            vec![tx]
        } {
            for b in if z.rem_euclid(16) == 0 {
                vec![tz - 1, tz]
            } else {
                vec![tz]
            } {
                if !(-7..=6).contains(&a) || !(-4..=3).contains(&b) {
                    continue;
                }
                let tile = self.tiles.entry(TileId(a, b)).or_default();
                Arc::make_mut(tile).heights[(x - a * 16) as usize + (z - b * 16) as usize * SIDE] =
                    height;
            }
        }
    }
    /// 笔刷按不可变旧快照求平滑，顺序无关；调用者将整次手势合并为一笔历史。
    pub fn brushed(&self, sample: BrushSample) -> Result<Self, DomainError> {
        if !sample.center.x.is_finite()
            || !sample.center.z.is_finite()
            || !sample.radius.is_finite()
            || !(0.25..=5.).contains(&sample.radius)
            || !sample.amount.is_finite()
            || !(0.0..=0.25).contains(&sample.amount)
        {
            return Err(DomainError("invalid terrain brush"));
        }
        let mut next = self.clone();
        let x0 = ((sample.center.x - sample.radius) / CELL).floor() as i32;
        let x1 = ((sample.center.x + sample.radius) / CELL).ceil() as i32;
        let z0 = ((sample.center.z - sample.radius) / CELL).floor() as i32;
        let z1 = ((sample.center.z + sample.radius) / CELL).ceil() as i32;
        for z in z0.max(-62)..=z1.min(62) {
            for x in x0.max(-100)..=x1.min(100) {
                let distance = sample.center.distance(Point {
                    x: x as f64 * CELL,
                    z: z as f64 * CELL,
                });
                if distance > sample.radius {
                    continue;
                }
                let weight = (1. - distance / sample.radius).powi(2);
                let old = self.vertex(x, z);
                let height = match sample.kind {
                    BrushKind::Raise => old + sample.amount * weight,
                    BrushKind::Lower => old - sample.amount * weight,
                    BrushKind::Smooth => {
                        let mut mean = 0.;
                        for b in -1..=1 {
                            for a in -1..=1 {
                                mean += self.vertex(x + a, z + b);
                            }
                        }
                        old + (mean / 9. - old) * (sample.amount * 4.).min(1.) * weight
                    }
                }
                .clamp(-1.5, 3.);
                if height != old {
                    next.set_vertex(x, z, height);
                }
            }
        }
        next.validate()?;
        Ok(next)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shared_edges_sampling_and_undo_snapshots_agree() {
        let original = TerrainDocument::default();
        let next = original
            .brushed(BrushSample {
                center: Point { x: 0., z: 0. },
                radius: 1.5,
                amount: 0.2,
                kind: BrushKind::Raise,
            })
            .unwrap();
        assert_eq!(original.height(Point { x: 0., z: 0. }), 0.);
        assert_eq!(next.vertex(0, 0), 0.2);
        assert_eq!(
            next.tiles[&TileId(-1, -1)].heights[16 + 16 * SIDE],
            next.vertex(0, 0)
        );
        assert!(next.height(Point { x: 0.1, z: 0.1 }) > 0.);
        assert!(next.validate().is_ok());
    }
}
