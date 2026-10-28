// Directly includes current source; no copied algorithm or cached core artifact.
#[path = "../../crates/country-core/src/utils/conservative_line_raster.rs"]
mod raster;

#[test]
fn saturated_major_coordinate_wraps_instead_of_finishing_the_point() {
    let mut cells = Vec::new();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        raster::conservative_plot_line((0.0, 3.0e9), (0.0, 3.0e9), 0.0, |x, y| {
            cells.push((x, y));
            if cells.len() == 4 {
                panic!("bounded review probe: stop after four callbacks");
            }
        });
    }));
    assert!(result.is_err());
    assert_eq!(cells, [(0, i32::MAX), (0, i32::MIN), (0, i32::MIN + 1), (0, i32::MIN + 2)]);
}

#[test]
fn saturated_minor_coordinate_drops_the_closed_interval() {
    let mut cells = Vec::new();
    // y is the chosen major axis for this point, x is the minor axis.
    raster::conservative_plot_line((3.0e9, 0.0), (3.0e9, 0.0), 0.0, |x, y| cells.push((x, y)));
    assert!(cells.is_empty(), "hi == i32::MAX makes hi.wrapping_add(1) negative");
}
