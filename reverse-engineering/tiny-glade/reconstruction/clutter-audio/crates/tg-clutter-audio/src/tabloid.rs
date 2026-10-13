//! 游戏私有 `tabloid` crate 的 glue 占位（原锁 tabloid 0.1.0，依赖 parking_lot；不还原实现）。
//! 证据：populate_autoclutter_regions 用 `Subscriber::take<DirtyTiles>` 订阅脏格发布。

/// tabloid::Subscriber（发布/订阅句柄；不还原）。
pub struct Subscriber;
