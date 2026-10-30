//! B triangle, normal and UV ownership; generic Vec storage is reused.
use bevy_ecs::component::Component;
use glam::{Vec2, Vec3};

#[derive(Component, Clone, Debug, Default)]
pub struct RoofTriggerVolume {
    triangles: Vec<[Vec3; 3]>,
    normals: Vec<[Vec3; 3]>,
    uvs: Vec<[Vec2; 3]>,
}

impl RoofTriggerVolume {
    /// Original getter asserts the geometry producer has populated triangles.
    pub fn triangles(&self) -> &[[Vec3; 3]] {
        assert!(
            !self.triangles.is_empty(),
            "assertion failed: !self.triangles.is_empty()"
        );
        &self.triangles
    }
    /// 原1408e4560/140dce1d0：借用原局部三角形，逐点加世界floor偏移。
    /// 原碰撞注册使用单位变换，floor必须进入点位；XZ零乘不能删掉。
    /// 保留raw访问以供已有几何缓存使用，碰撞/世界空间消费者使用此迭代器。
    pub fn triangles_at_floor_y(
        &self,
        floor_y: f32,
    ) -> impl ExactSizeIterator<Item = utils::geometry::intersect::Triangle> + '_ {
        let triangles = self.triangles();
        let offset = Vec3::new(floor_y * 0.0, floor_y, floor_y * 0.0);
        triangles.iter().map(move |points| {
            utils::geometry::intersect::Triangle {
                points: points.map(|point| point + offset),
            }
        })
    }
    pub fn normals(&self) -> &[[Vec3; 3]] {
        &self.normals
    }
    pub fn uvs(&self) -> &[[Vec2; 3]] {
        &self.uvs
    }
    pub fn set_triangles(&mut self, values: Vec<[Vec3; 3]>) {
        self.triangles = values;
    }
    pub fn set_normals(&mut self, values: Vec<[Vec3; 3]>) {
        self.normals = values;
        assert_eq!(
            self.triangles.len(),
            self.normals.len(),
            "original normal count guard"
        );
    }
    pub fn set_uvs(&mut self, values: Vec<[Vec2; 3]>) {
        self.uvs = values;
        assert_eq!(
            self.triangles.len(),
            self.uvs.len(),
            "original UV count guard"
        );
    }
    /// B 0x1408e46d0 delegates triangle UV lookup to the recovered game utility
    /// then adds the supplied floor offset, retaining the zero products on X/Z.
    pub fn get_position_from_mesh_uv(&self, uv: Vec2, floor_y: f32) -> Option<Vec3> {
        utils::geometry::position_from_mesh_uvs::get_position_from_mesh_uv(
            &self.triangles,
            &self.uvs,
            uv,
        )
        .map(|point| {
            Vec3::new(
                floor_y * 0.0 + point.x,
                floor_y + point.y,
                point.z + floor_y * 0.0,
            )
        })
    }
}

#[cfg(test)]
mod original_floor_projection_tests {
    use super::*;
    #[test]
    fn original_roof_floor_projection_preserves_each_point_and_required_empty_failure() {
        type Case=(u32,&'static [u32],Option<&'static [u32]>);
        let cases:&[Case]=include!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../reports/module-recovery-2026-10-23/m01-roof-trigger-floor-native-fixtures.rs"));
        for (index,(floor,input,expected)) in cases.iter().enumerate() {
            let triangles=input.chunks_exact(9).map(|t|std::array::from_fn(|i|Vec3::new(f32::from_bits(t[i*3]),f32::from_bits(t[i*3+1]),f32::from_bits(t[i*3+2])))).collect();
            let mut volume=RoofTriggerVolume::default();volume.set_triangles(triangles);
            let actual=std::panic::catch_unwind(||volume.triangles_at_floor_y(f32::from_bits(*floor)).flat_map(|triangle|triangle.points.into_iter().flat_map(|p|p.to_array().map(f32::to_bits))).collect::<Vec<_>>());
            match (actual,expected) {
                (Err(_),None)=>{},
                (Ok(actual),Some(expected))=>{
                    assert_eq!(actual.len(),expected.len());
                    for (point,(&a,&b)) in actual.iter().zip(*expected).enumerate() {
                        // 跨架构不承诺NaN载荷位型；原零乘仍须保持相同NaN分类。
                        if f32::from_bits(b).is_nan() {assert!(f32::from_bits(a).is_nan(),"case {index} word {point}");}
                        else {assert_eq!(a,b,"case {index} word {point}");}
                    }
                },
                _=>panic!("case {index} original failure boundary"),
            }
        }
    }
}
