/// B DecoratorStorageAddr：装饰目的存储地址（d2s 索引 24B 条目判别 +0/1/2 =
/// Wall/Terrain/Stair；payload = WallId）。repr u64 16B：{disc@0, payload@8}。
/// WallId 未实现 serde（walls/mod.rs），故本枚举不派生序列化；wire 格式 U。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DecoratorStorageAddr {
    /// disc 0：墙（payload = WallId）。
    Wall(WallId),
    /// disc 1：地形。
    Terrain,
    /// disc 2：楼梯。
    Stair,
}

impl Default for DecoratorStorageAddr {
    fn default() -> Self {
        DecoratorStorageAddr::Wall(WallId::default())
    }
}

impl DecoratorStorageAddr {
    /// 0x140a1c240（O）：disc==0 → WallId，否则 fmt panic。
    pub fn as_wall(self) -> WallId {
        match self {
            DecoratorStorageAddr::Wall(w) => w,
            _ => panic!("DecoratorStorageAddr::as_wall: not a wall storage"),
        }
    }

    /// 0x140a1c290（O）：disc==0 → Some(WallId)，否则 None。
    pub fn try_as_wall(self) -> Option<WallId> {
        match self {
            DecoratorStorageAddr::Wall(w) => Some(w),
            _ => None,
        }
    }
}

