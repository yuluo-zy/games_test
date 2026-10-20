//! Original roof UI private geometry adapters. General vector/matrix algorithms
//! remain in the original glam dependency, rather than being reimplemented.
use glam::{Mat3, Quat, Vec2, Vec3};

/// utils::xz_rotation_bad140c90fd0: normalize_or(X), Y-cross-direction,
/// normalize, then the original glam Mat3-to-quaternion branch implementation.
pub(super) fn xz_rotation_bad(x: f32, z: f32) -> Quat {
    let direction = Vec2::new(x, z).normalize_or(Vec2::X);
    let forward = Vec3::new(direction.x, 0.0, direction.y);
    let right = Vec3::Y.cross(forward).normalize();
    Quat::from_mat3(&Mat3::from_cols(right, Vec3::Y, forward))
}

/// First24 original dimension-to-corner records from140ada0e0..140ada463.
/// Keep individual signed half-products, vertex duplicates and face order.
pub(super) fn box_positions(width: f32, height: f32, depth: f32) -> [Vec3; 24] {
    let xn = width * -0.5;
    let xp = width * 0.5;
    let yn = height * -0.5;
    let yp = height * 0.5;
    let zn = depth * -0.5;
    let zp = depth * 0.5;
    [
        Vec3::new(xn, yn, zp),
        Vec3::new(xp, yn, zp),
        Vec3::new(xp, yp, zp),
        Vec3::new(xn, yp, zp),
        Vec3::new(xn, yp, zn),
        Vec3::new(xp, yp, zn),
        Vec3::new(xp, yn, zn),
        Vec3::new(xn, yn, zn),
        Vec3::new(xp, yn, zn),
        Vec3::new(xp, yp, zn),
        Vec3::new(xp, yp, zp),
        Vec3::new(xp, yn, zp),
        Vec3::new(xn, yn, zp),
        Vec3::new(xn, yp, zp),
        Vec3::new(xn, yp, zn),
        Vec3::new(xn, yn, zn),
        Vec3::new(xp, yp, zn),
        Vec3::new(xn, yp, zn),
        Vec3::new(xn, yp, zp),
        Vec3::new(xp, yp, zp),
        Vec3::new(xp, yn, zp),
        Vec3::new(xn, yn, zp),
        Vec3::new(xn, yn, zn),
        Vec3::new(xp, yn, zn),
    ]
}

pub(super) const BOX_INDICES: [usize; 36] = [
    0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4, 8, 9, 10, 10, 11, 8, 12, 13, 14, 14, 15, 12, 16, 17, 18,
    18, 19, 16, 20, 21, 22, 22, 23, 20,
];

pub(super) fn squared_distance(a: Vec3, b: Vec3) -> f32 {
    let delta = a - b;
    (delta.y * delta.y + delta.x * delta.x) + delta.z * delta.z
}

pub(super) fn height_from_projection(projected: Vec3, origin: Vec3, offset: f32) -> f32 {
    let value = (squared_distance(projected, origin).sqrt() - offset - 0.4) / (12.0 - 0.4);
    let bounded = if 0.0 > value { 0.0 } else { value };
    if 1.0 < bounded { 1.0 } else { bounded }
}

#[cfg(test)]
mod original_native_geometry_tests {
    use super::*;
    fn same_bits(actual: f32, expected: u32) {
        let value = f32::from_bits(expected);
        if value.is_nan() {
            assert!(actual.is_nan());
        } else {
            assert_eq!(actual.to_bits(), expected);
        }
    }
    #[test]
    fn all_twenty_four_box_records_match_original() {
        {
            let [x, y, z] = [1073741824, 1082130432, 1086324736].map(f32::from_bits);
            let actual = box_positions(x, y, z);
            let expected = [
                3212836864, 3221225472, 1077936128, 1065353216, 3221225472, 1077936128, 1065353216,
                1073741824, 1077936128, 3212836864, 1073741824, 1077936128, 3212836864, 1073741824,
                3225419776, 1065353216, 1073741824, 3225419776, 1065353216, 3221225472, 3225419776,
                3212836864, 3221225472, 3225419776, 1065353216, 3221225472, 3225419776, 1065353216,
                1073741824, 3225419776, 1065353216, 1073741824, 1077936128, 1065353216, 3221225472,
                1077936128, 3212836864, 3221225472, 1077936128, 3212836864, 1073741824, 1077936128,
                3212836864, 1073741824, 3225419776, 3212836864, 3221225472, 3225419776, 1065353216,
                1073741824, 3225419776, 3212836864, 1073741824, 3225419776, 3212836864, 1073741824,
                1077936128, 1065353216, 1073741824, 1077936128, 1065353216, 3221225472, 1077936128,
                3212836864, 3221225472, 1077936128, 3212836864, 3221225472, 3225419776, 1065353216,
                3221225472, 3225419776,
            ];
            for (i, p) in actual.into_iter().enumerate() {
                for (j, v) in p.to_array().into_iter().enumerate() {
                    same_bits(v, expected[i * 3 + j]);
                }
            }
        }
        {
            let [x, y, z] = [1061158912, 1065353216, 1056964608].map(f32::from_bits);
            let actual = box_positions(x, y, z);
            let expected = [
                3200253952, 3204448256, 1048576000, 1052770304, 3204448256, 1048576000, 1052770304,
                1056964608, 1048576000, 3200253952, 1056964608, 1048576000, 3200253952, 1056964608,
                3196059648, 1052770304, 1056964608, 3196059648, 1052770304, 3204448256, 3196059648,
                3200253952, 3204448256, 3196059648, 1052770304, 3204448256, 3196059648, 1052770304,
                1056964608, 3196059648, 1052770304, 1056964608, 1048576000, 1052770304, 3204448256,
                1048576000, 3200253952, 3204448256, 1048576000, 3200253952, 1056964608, 1048576000,
                3200253952, 1056964608, 3196059648, 3200253952, 3204448256, 3196059648, 1052770304,
                1056964608, 3196059648, 3200253952, 1056964608, 3196059648, 3200253952, 1056964608,
                1048576000, 1052770304, 1056964608, 1048576000, 1052770304, 3204448256, 1048576000,
                3200253952, 3204448256, 1048576000, 3200253952, 3204448256, 3196059648, 1052770304,
                3204448256, 3196059648,
            ];
            for (i, p) in actual.into_iter().enumerate() {
                for (j, v) in p.to_array().into_iter().enumerate() {
                    same_bits(v, expected[i * 3 + j]);
                }
            }
        }
        {
            let [x, y, z] = [3221225472, 1082130432, 3233808384].map(f32::from_bits);
            let actual = box_positions(x, y, z);
            let expected = [
                1065353216, 3221225472, 3225419776, 3212836864, 3221225472, 3225419776, 3212836864,
                1073741824, 3225419776, 1065353216, 1073741824, 3225419776, 1065353216, 1073741824,
                1077936128, 3212836864, 1073741824, 1077936128, 3212836864, 3221225472, 1077936128,
                1065353216, 3221225472, 1077936128, 3212836864, 3221225472, 1077936128, 3212836864,
                1073741824, 1077936128, 3212836864, 1073741824, 3225419776, 3212836864, 3221225472,
                3225419776, 1065353216, 3221225472, 3225419776, 1065353216, 1073741824, 3225419776,
                1065353216, 1073741824, 1077936128, 1065353216, 3221225472, 1077936128, 3212836864,
                1073741824, 1077936128, 1065353216, 1073741824, 1077936128, 1065353216, 1073741824,
                3225419776, 3212836864, 1073741824, 3225419776, 3212836864, 3221225472, 3225419776,
                1065353216, 3221225472, 3225419776, 1065353216, 3221225472, 1077936128, 3212836864,
                3221225472, 1077936128,
            ];
            for (i, p) in actual.into_iter().enumerate() {
                for (j, v) in p.to_array().into_iter().enumerate() {
                    same_bits(v, expected[i * 3 + j]);
                }
            }
        }
        {
            let [x, y, z] = [0, 2147483648, 0].map(f32::from_bits);
            let actual = box_positions(x, y, z);
            let expected = [
                2147483648, 0, 0, 0, 0, 0, 0, 2147483648, 0, 2147483648, 2147483648, 0, 2147483648,
                2147483648, 2147483648, 0, 2147483648, 2147483648, 0, 0, 2147483648, 2147483648, 0,
                2147483648, 0, 0, 2147483648, 0, 2147483648, 2147483648, 0, 2147483648, 0, 0, 0, 0,
                2147483648, 0, 0, 2147483648, 2147483648, 0, 2147483648, 2147483648, 2147483648,
                2147483648, 0, 2147483648, 0, 2147483648, 2147483648, 2147483648, 2147483648,
                2147483648, 2147483648, 2147483648, 0, 0, 2147483648, 0, 0, 0, 0, 2147483648, 0, 0,
                2147483648, 0, 2147483648, 0, 0, 2147483648,
            ];
            for (i, p) in actual.into_iter().enumerate() {
                for (j, v) in p.to_array().into_iter().enumerate() {
                    same_bits(v, expected[i * 3 + j]);
                }
            }
        }
        {
            let [x, y, z] = [2143289344, 1065353216, 2139095040].map(f32::from_bits);
            let actual = box_positions(x, y, z);
            let expected = [
                2143289344, 3204448256, 2139095040, 2143289344, 3204448256, 2139095040, 2143289344,
                1056964608, 2139095040, 2143289344, 1056964608, 2139095040, 2143289344, 1056964608,
                4286578688, 2143289344, 1056964608, 4286578688, 2143289344, 3204448256, 4286578688,
                2143289344, 3204448256, 4286578688, 2143289344, 3204448256, 4286578688, 2143289344,
                1056964608, 4286578688, 2143289344, 1056964608, 2139095040, 2143289344, 3204448256,
                2139095040, 2143289344, 3204448256, 2139095040, 2143289344, 1056964608, 2139095040,
                2143289344, 1056964608, 4286578688, 2143289344, 3204448256, 4286578688, 2143289344,
                1056964608, 4286578688, 2143289344, 1056964608, 4286578688, 2143289344, 1056964608,
                2139095040, 2143289344, 1056964608, 2139095040, 2143289344, 3204448256, 2139095040,
                2143289344, 3204448256, 2139095040, 2143289344, 3204448256, 4286578688, 2143289344,
                3204448256, 4286578688,
            ];
            for (i, p) in actual.into_iter().enumerate() {
                for (j, v) in p.to_array().into_iter().enumerate() {
                    same_bits(v, expected[i * 3 + j]);
                }
            }
        }
    }
    #[test]
    fn rotation_matches_complete_original_private_function() {
        {
            let [x, z] = [0, 0].map(f32::from_bits);
            let expected = [0, 1060439283, 0, 1060439283];
            for (actual, expected) in xz_rotation_bad(x, z).to_array().into_iter().zip(expected) {
                same_bits(actual, expected);
            }
        }
        {
            let [x, z] = [1065353216, 0].map(f32::from_bits);
            let expected = [0, 1060439283, 0, 1060439283];
            for (actual, expected) in xz_rotation_bad(x, z).to_array().into_iter().zip(expected) {
                same_bits(actual, expected);
            }
        }
        {
            let [x, z] = [0, 1065353216].map(f32::from_bits);
            let expected = [0, 0, 0, 1065353216];
            for (actual, expected) in xz_rotation_bad(x, z).to_array().into_iter().zip(expected) {
                same_bits(actual, expected);
            }
        }
        {
            let [x, z] = [0, 3212836864].map(f32::from_bits);
            let expected = [0, 1065353216, 0, 0];
            for (actual, expected) in xz_rotation_bad(x, z).to_array().into_iter().zip(expected) {
                same_bits(actual, expected);
            }
        }
        {
            let [x, z] = [3212836864, 0].map(f32::from_bits);
            let expected = [0, 1060439283, 0, 3207922931];
            for (actual, expected) in xz_rotation_bad(x, z).to_array().into_iter().zip(expected) {
                same_bits(actual, expected);
            }
        }
        {
            let [x, z] = [1077936128, 1082130432].map(f32::from_bits);
            let expected = [0, 1050798236, 0, 1064492266];
            for (actual, expected) in xz_rotation_bad(x, z).to_array().into_iter().zip(expected) {
                same_bits(actual, expected);
            }
        }
        {
            let [x, z] = [3225419776, 1082130432].map(f32::from_bits);
            let expected = [0, 3198281884, 2147483648, 1064492266];
            for (actual, expected) in xz_rotation_bad(x, z).to_array().into_iter().zip(expected) {
                same_bits(actual, expected);
            }
        }
        {
            let [x, z] = [1077936128, 3229614080].map(f32::from_bits);
            let expected = [0, 1064492266, 0, 1050798236];
            for (actual, expected) in xz_rotation_bad(x, z).to_array().into_iter().zip(expected) {
                same_bits(actual, expected);
            }
        }
        {
            let [x, z] = [3225419776, 3229614080].map(f32::from_bits);
            let expected = [0, 1064492266, 0, 3198281884];
            for (actual, expected) in xz_rotation_bad(x, z).to_array().into_iter().zip(expected) {
                same_bits(actual, expected);
            }
        }
        {
            let [x, z] = [1900671690, 1900671690].map(f32::from_bits);
            let expected = [0, 1060439283, 0, 1060439283];
            for (actual, expected) in xz_rotation_bad(x, z).to_array().into_iter().zip(expected) {
                same_bits(actual, expected);
            }
        }
        {
            let [x, z] = [2143289344, 1065353216].map(f32::from_bits);
            let expected = [0, 1060439283, 0, 1060439283];
            for (actual, expected) in xz_rotation_bad(x, z).to_array().into_iter().zip(expected) {
                same_bits(actual, expected);
            }
        }
        {
            let [x, z] = [2139095040, 0].map(f32::from_bits);
            let expected = [0, 1060439283, 0, 1060439283];
            for (actual, expected) in xz_rotation_bad(x, z).to_array().into_iter().zip(expected) {
                same_bits(actual, expected);
            }
        }
        {
            let [x, z] = [71362, 71362].map(f32::from_bits);
            let expected = [0, 1060439283, 0, 1060439283];
            for (actual, expected) in xz_rotation_bad(x, z).to_array().into_iter().zip(expected) {
                same_bits(actual, expected);
            }
        }
    }
}
