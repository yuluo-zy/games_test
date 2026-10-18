//! B build_gpu_bvh@140b0da50. Reuse the original locked obvhs fork; only
//! game-specific parameter selection, primitive reordering and packing live here.
use super::{InputTriangle, RtBvhSsbo, RtTriangleMetaSsbo, RtTrianglesSsbo, RtUploadData};
use glam::Vec3A;
use obvhs::{
    BvhBuildParams,
    ploc::{PlocSearchDistance, SortPrecision},
    triangle::Triangle,
};

pub(crate) fn original_build_params() -> BvhBuildParams {
    BvhBuildParams {
        pre_split: true,
        ploc_search_distance: PlocSearchDistance::High,
        search_depth_threshold: 1,
        reinsertion_batch_ratio: 0.2,
        post_collapse_reinsertion_batch_ratio_multiplier: 1.0,
        sort_precision: SortPrecision::U128,
        max_prims_per_leaf: 3,
        collapse_traversal_cost: 3.0,
    }
}

pub fn build_gpu_bvh(input: &[InputTriangle]) -> RtUploadData {
    if input.is_empty() {
        return RtUploadData::default();
    }
    let triangles = input
        .iter()
        .map(|tri| Triangle {
            v0: Vec3A::from(tri.a),
            v1: Vec3A::from(tri.b),
            v2: Vec3A::from(tri.c),
        })
        .collect::<Vec<_>>();
    let started = std::time::Instant::now();
    let mut build_time = std::time::Duration::ZERO;
    let bvh = obvhs::cwbvh::builder::build_cwbvh_from_tris(
        &triangles,
        original_build_params(),
        &mut build_time,
    );
    log::trace!("Griffin has built us a GPU BVH in {:?}", started.elapsed());
    let mut output = RtUploadData {
        triangles: Vec::with_capacity(bvh.primitive_indices.len()),
        metadata: Vec::with_capacity(bvh.primitive_indices.len()),
        bvh: bvh
            .nodes
            .into_iter()
            .map(|record| RtBvhSsbo { record })
            .collect(),
    };
    // Spatial splitting may duplicate an original primitive. Reorder both
    // arrays from the same index; never zip with the unsorted input sequence.
    for original_index in bvh.primitive_indices {
        let tri = &input[original_index as usize];
        output
            .triangles
            .push(RtTrianglesSsbo::from_vertices(tri.a, tri.b, tri.c));
        output.metadata.push(RtTriangleMetaSsbo {
            packed_color: tri.user_data,
        });
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec3;

    #[test]
    fn native_triangle_packing_places_e2_in_low_half_and_e1_in_high_half() {
        let tri = RtTrianglesSsbo::from_vertices(
            Vec3::new(10., 20., 30.),
            Vec3::new(11., 22., 33.),
            Vec3::new(14., 25., 36.),
        );
        assert_eq!(tri.base_vertex, [10., 20., 30.]);
        assert_eq!(tri.edge_pairs, [0x3c00_4400, 0x4000_4500, 0x4200_4600]);
        assert_eq!(std::mem::size_of::<super::super::GpuRtTriangle>(), 24);
    }

    #[test]
    fn original_bvh_reorders_geometry_and_metadata_together() {
        let input = (0..48)
            .map(|i| {
                let x = ((i * 17) % 48) as f32;
                InputTriangle {
                    a: Vec3::new(x, 0., 0.),
                    b: Vec3::new(x + 0.25, 1., 0.),
                    c: Vec3::new(x, 0., 1.),
                    user_data: i + 100,
                }
            })
            .collect::<Vec<_>>();
        let output = build_gpu_bvh(&input);
        assert!(!output.bvh.is_empty());
        assert_eq!(output.triangles.len(), output.metadata.len());
        assert!(output.triangles.len() >= input.len());
        for (triangle, metadata) in output.triangles.iter().zip(&output.metadata) {
            let original = &input[(metadata.packed_color - 100) as usize];
            assert_eq!(
                *triangle,
                RtTrianglesSsbo::from_vertices(original.a, original.b, original.c)
            );
        }
        let empty = build_gpu_bvh(&[]);
        assert!(empty.bvh.is_empty() && empty.triangles.is_empty() && empty.metadata.is_empty());
    }
}
