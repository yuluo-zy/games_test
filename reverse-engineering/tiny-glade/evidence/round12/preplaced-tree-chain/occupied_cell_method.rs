    /// Original B1425defd0 specialization used by converted preplaced trees.
    /// Its callback rejects any occupied cell, without the different public
    /// segment-distance/sort/dedup policy of query_with_circle.
    pub fn has_occupied_cells_in_circle(&self, center: glam::Vec2, radius: f32) -> bool {
        use crate::resources::walls::spatial_index::world_space_to_grid_coord_unclamped;
        let half = WORLD_SIZE * 0.5;
        let lo = [center.x - radius, center.y - radius].map(|v| v.max(-half));
        let hi = [center.x + radius, center.y + radius].map(|v| v.min(half));
        assert!(lo[0] >= -half && lo[1] >= -half, "original circle lower bounds");
        assert!(hi[0] <= half && hi[1] <= half, "original circle upper bounds");
        let cell_size = WORLD_SIZE / GRID_SIDE as f32;
        assert!(cell_size > 0.5, "original wall spatial cell size");
        let lo = world_space_to_grid_coord_unclamped(lo).map(|v| v.max(0));
        let hi = world_space_to_grid_coord_unclamped(hi).map(|v| v.min(GRID_SIDE as i32 - 1));
        let expansion = (cell_size * 0.5) * f32::from_bits(0x3fb5_04f3);
        for z in lo[1]..=hi[1] {
            for x in lo[0]..=hi[0] {
                let middle = |i: i32| {
                    let t = i as f32 / GRID_SIDE as f32;
                    (t * half - (1.0 - t) * half) + cell_size * 0.5
                };
                let dx = middle(x) - center.x;
                let dz = middle(z) - center.y;
                let distance = (dz * dz + dx * dx).sqrt();
                // Native UCOMISS/JBE excludes equality and unordered input.
                if radius + expansion > distance {
                    let index = z as usize * GRID_SIDE + x as usize;
                    if !self.cells[index].is_empty() { return true; }
                }
            }
        }
        false
    }
