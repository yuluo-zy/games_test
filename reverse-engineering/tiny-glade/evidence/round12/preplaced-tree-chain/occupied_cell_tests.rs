
#[cfg(test)]
mod original_occupied_circle_fixtures {
    use super::*;
    #[test]
    fn original_b_837_byte_callback_boundary_vectors() {
        // Obtained by running untouched original B1425defd0, including its
        // original geometry/grid helpers, against independently built cells.
        // Full168-case oracle and binary SHA: round12/preplaced-tree-chain/
        // original-wall-fixtures.json. These are oracle outputs, not roundtrips.
        for (center, radius, occupied, reject) in [
            ([0., 0.], 7., None, false),
            ([0., 0.], 7., Some([32, 32]), true),
            ([0., 0.], 7., Some([36, 32]), false),
            ([65., 65.], 7., Some([63, 63]), true),
            ([-65., -65.], 7., Some([0, 0]), true),
            ([72., 72.], 7., Some([63, 63]), false),
            ([80., 80.], 7., Some([63, 63]), false),
            ([0., 0.], 0., Some([32, 32]), false),
        ] {
            let mut hash = WallSpatialHash::default();
            if let Some([x, z]) = occupied {
                // The callback reads cell occupancy only, even with no
                // geometry record associated with the stored key.
                hash.cells[z * GRID_SIDE + x].push(7);
            }
            assert_eq!(hash.has_occupied_cells_in_circle(glam::Vec2::from_array(center), radius), reject,
                "center={center:?}, radius={radius}, occupied={occupied:?}");
        }
    }
}
