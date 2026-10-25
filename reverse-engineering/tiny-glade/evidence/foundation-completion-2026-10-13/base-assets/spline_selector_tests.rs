use super::*;

#[test]
fn shared_scalar_knots_select_right_segment_and_clamp_endpoints() {
    let spline=Spline2D::from_control_points_with_tangents(
        &[Vec2::ZERO,Vec2::new(3.0,4.0),Vec2::new(6.0,8.0)],None,None);
    assert_eq!(spline.find_segment_and_dt(-1.0).1,0.0);
    let (segment,dt)=spline.find_segment_and_dt(5.0);
    assert_eq!(segment.start_t,5.0);
    assert_eq!(dt,0.0);
    assert_eq!(spline.position_at_t(2.5),Vec2::new(1.5,2.0));
    assert_eq!(spline.position_at_t(f32::INFINITY),Vec2::new(6.0,8.0));
    assert_eq!(spline.position_at_t(f32::NEG_INFINITY),Vec2::ZERO);
    assert_eq!(spline.tangent_at_t(5.0),Vec2::new(0.6,0.8));
    assert!(spline.find_segment_and_dt(f32::NAN).1.is_nan());
}

#[test]
fn planar_2d_selection_agrees_with_native_validated_3d_policy() {
    let points=[Vec2::new(-2.0,1.0),Vec2::new(0.0,4.0),Vec2::new(3.0,-1.0),Vec2::new(6.0,2.0)];
    let spline2=Spline2D::from_control_points_with_tangents(&points,None,None);
    let points3:Vec<_>=points.iter().map(|p|Vec3::new(p.x,p.y,0.0)).collect();
    let spline3=Spline3D::from_control_points_with_tangents(&points3,None,None);
    for t in [-5.0,0.0,1.0,spline2.segments[1].start_t,spline2.segments[2].start_t,spline2.max_t,spline2.max_t+5.0] {
        let (segment2,dt2)=spline2.find_segment_and_dt(t);
        let (segment3,dt3)=spline3.find_segment_and_dt(t);
        assert_eq!((segment2.start_t,dt2),(segment3.start_t,dt3));
        let p=spline3.position_at_t(t);
        assert_eq!(spline2.position_at_t(t),Vec2::new(p.x,p.y));
        let tangent=spline3.tangent_at_t(t);
        assert_eq!(spline2.tangent_at_t(t),Vec2::new(tangent.x,tangent.y));
    }
    // Periodic coefficients close the curve, while scalar query selection still
    // uses the bounded span. This is a shared-policy check, not a 2D native oracle.
    let periodic=Spline2D::from_periodic_loop(&points);
    assert_eq!(periodic.find_segment_and_dt(periodic.max_t+1.0).1,
               periodic.max_t-periodic.segments.last().unwrap().start_t);
}
