//! 游戏私有 `utils` crate 的 glue 占位（原锁 utils 0.1.0 本地包，无 registry 源码；不还原实现）。
//! 仅提供本草稿编译所需的 `utils::rng` 边界类型。

pub mod rng {
    use glam::Vec2;

    /// utils::rng::Rng（fastrand 包装；不还原流实现）。
    pub struct Rng;
    impl Rng {
        pub fn new() -> Rng {
            Rng
        }
        pub fn f32(&mut self) -> f32 {
            0.0 // TODO(evidence): fastrand 流不在本 crate 还原。
        }
    }

    /// utils::rng::Circle2d（单位圆；不还原参数化实现）。
    pub struct Circle2d;
    impl Circle2d {
        pub fn new(_center: Vec2, _radius: f32) -> Circle2d {
            Circle2d
        }
        pub fn get_pos_at_u(&self, _u: f32) -> Vec2 {
            Vec2::ZERO // TODO(evidence): 圆周采样不在本 crate 还原。
        }
    }
}
